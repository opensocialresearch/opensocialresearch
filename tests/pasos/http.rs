//! Pasos que arman pedidos HTTP al servidor de `osr` y miran la respuesta.
//!
//! Un pedido nuevo lleva el Host propio, 127.0.0.1 con el puerto {P}, sin
//! Origin, sin cookie y sin token, como dice el encabezado de servidor.feature.

use cucumber::{given, then, when};

use super::Mundo;

/// Un pedido armado por el escenario.
#[derive(Debug)]
pub struct Pedido {
    metodo: String,
    direccion: String,
    encabezados: Vec<(String, String)>,
    cuerpo: Option<String>,
    /// Si el pedido va sin `Host`, que reqwest y curl siempre agregan (SRV-06.4).
    sin_host: bool,
}

/// Lo que respondió el servidor, con los nombres de los encabezados en
/// minúscula.
#[derive(Debug)]
pub struct Respuesta {
    pub estado: u16,
    pub encabezados: Vec<(String, String)>,
    pub cuerpo: String,
}

impl Respuesta {
    /// El valor de un encabezado, sin distinguir mayúsculas en el nombre.
    pub fn encabezado(&self, nombre: &str) -> Option<&str> {
        let nombre = nombre.to_ascii_lowercase();
        self.encabezados
            .iter()
            .find(|(n, _)| *n == nombre)
            .map(|(_, v)| v.as_str())
    }
}

impl Mundo {
    fn nuevo_pedido(&mut self, metodo: &str, direccion: String) {
        self.pedido = Some(Pedido::nuevo(metodo, direccion));
    }

    /// La dirección de un camino en el servidor del escenario.
    pub(super) fn direccion(&self, camino: &str) -> String {
        let puerto = self.puerto.expect("osr no escribió su puerto");
        format!("http://127.0.0.1:{puerto}{}", self.resolver(camino))
    }

    fn encabezado(&mut self, nombre: &str, valor: String) {
        let pedido = self.pedido.as_mut().expect("falta «un pedido HTTP»");
        pedido.encabezados.push((nombre.to_string(), valor));
    }

    fn token_valido(&self) -> String {
        self.token.clone().expect("osr no escribió su token")
    }
}

// Solo los caminos de las familias que ya existen, la entrada y una ruta
// inexistente. Los de `/audio/` y `/respaldo/` quedan sin definir hasta SRV-11
// y PAR-08 (D-20).
#[given(regex = r#"^un pedido HTTP ([A-Z]+) a "(/|/\?token=[^"]*|/no-existe)"$"#)]
fn pedido_a(mundo: &mut Mundo, metodo: String, camino: String) {
    let direccion = mundo.direccion(&camino);
    mundo.nuevo_pedido(&metodo, direccion);
}

#[given("un pedido HTTP GET a la dirección de la línea de la salida estándar")]
fn pedido_a_la_linea(mundo: &mut Mundo) {
    let salida = mundo.salida_estandar();
    let linea = salida
        .lines()
        .next()
        .expect("la salida estándar está vacía");
    let direccion = linea
        .strip_prefix(super::orden::LISTO)
        .expect("no es la línea de SRV-01");
    mundo.nuevo_pedido("GET", direccion.to_string());
}

#[given("el pedido lleva el token válido en el encabezado Authorization")]
fn con_token(mundo: &mut Mundo) {
    let valor = format!("Bearer {}", mundo.token_valido());
    mundo.encabezado("Authorization", valor);
}

impl Pedido {
    pub fn nuevo(metodo: &str, direccion: String) -> Pedido {
        Pedido {
            metodo: metodo.to_string(),
            direccion,
            encabezados: Vec::new(),
            cuerpo: None,
            sin_host: false,
        }
    }

    pub fn con_encabezado(mut self, nombre: &str, valor: &str) -> Pedido {
        self.encabezados
            .push((nombre.to_string(), valor.to_string()));
        self
    }

    pub async fn mandar(&self) -> Respuesta {
        if self.sin_host {
            return self.mandar_crudo();
        }
        let cliente = reqwest::Client::builder().no_proxy().build().unwrap();
        let metodo = reqwest::Method::from_bytes(self.metodo.as_bytes()).expect("método HTTP");
        let mut armado = cliente.request(metodo, &self.direccion);
        for (nombre, valor) in &self.encabezados {
            armado = armado.header(nombre, valor);
        }
        if let Some(cuerpo) = &self.cuerpo {
            armado = armado.body(cuerpo.clone());
        }
        let respuesta = armado.send().await.expect("mandar el pedido");
        let encabezados = respuesta
            .headers()
            .iter()
            .map(|(n, v)| (n.as_str().to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();
        Respuesta {
            estado: respuesta.status().as_u16(),
            encabezados,
            cuerpo: respuesta.text().await.expect("leer el cuerpo"),
        }
    }

    /// Escribe el pedido a mano en HTTP/1.1, sin `Host`, y lee la respuesta
    /// hasta que el servidor cierra la conexión.
    fn mandar_crudo(&self) -> Respuesta {
        use std::io::{Read, Write};
        let resto = self
            .direccion
            .strip_prefix("http://")
            .expect("dirección http");
        let (autoridad, camino) = resto.split_at(resto.find('/').unwrap_or(resto.len()));
        let mut texto = format!("{} {camino} HTTP/1.1\r\nConnection: close\r\n", self.metodo);
        for (nombre, valor) in &self.encabezados {
            texto.push_str(&format!("{nombre}: {valor}\r\n"));
        }
        let cuerpo = self.cuerpo.clone().unwrap_or_default();
        texto.push_str(&format!("Content-Length: {}\r\n\r\n{cuerpo}", cuerpo.len()));
        let mut conexion = std::net::TcpStream::connect(autoridad).expect("conectar con osr");
        conexion
            .write_all(texto.as_bytes())
            .expect("escribir el pedido");
        let mut leido = String::new();
        conexion
            .read_to_string(&mut leido)
            .expect("leer la respuesta");
        let (cabecera, cuerpo) = leido.split_once("\r\n\r\n").expect("respuesta HTTP");
        let mut lineas = cabecera.lines();
        let estado = lineas.next().and_then(|l| l.split(' ').nth(1));
        Respuesta {
            estado: estado
                .and_then(|e| e.parse().ok())
                .expect("línea de estado"),
            encabezados: lineas
                .filter_map(|l| l.split_once(": "))
                .map(|(n, v)| (n.to_ascii_lowercase(), v.to_string()))
                .collect(),
            cuerpo: cuerpo.to_string(),
        }
    }

    /// Manda el pedido con curl desde la red de prueba, donde corre `osr`.
    /// Solo trae el estado, que es lo único que miran esos escenarios.
    fn mandar_con_curl(&self, mut nsenter: std::process::Command) -> Respuesta {
        nsenter.arg(super::programa("curl"));
        nsenter.args([
            "-s",
            "-o",
            "/dev/null",
            "-w",
            "%{http_code}",
            "-X",
            &self.metodo,
        ]);
        for (nombre, valor) in &self.encabezados {
            nsenter.args(["-H", &format!("{nombre}: {valor}")]);
        }
        let salida = nsenter.arg(&self.direccion).output().expect("lanzar curl");
        let estado = String::from_utf8_lossy(&salida.stdout);
        Respuesta {
            estado: estado
                .parse()
                .unwrap_or_else(|_| panic!("curl respondió {estado:?}")),
            encabezados: Vec::new(),
            cuerpo: String::new(),
        }
    }
}

// Solo los encabezados que el servidor ya mira, el token, la cookie, el Host,
// el Origin y Sec-Fetch-Site. Range queda sin definir hasta SRV-11 (D-20).
#[given(
    regex = r#"^el pedido lleva el encabezado "(Authorization|Cookie|Host|Origin|Sec-Fetch-Site)" con valor "([^"]*)"$"#
)]
fn con_encabezado(mundo: &mut Mundo, nombre: String, valor: String) {
    let valor = mundo.resolver(&valor);
    mundo.encabezado(&nombre, valor);
}

#[given(r#"el pedido no lleva el encabezado "Host""#)]
fn sin_host(mundo: &mut Mundo) {
    let pedido = mundo.pedido.as_mut().expect("falta «un pedido HTTP»");
    pedido.sin_host = true;
}

#[given("el pedido no lleva token")]
fn sin_token(_mundo: &mut Mundo) {}

/// La ruta todavía no existe, llega con PAR. El camino es el que va a tener,
/// y sin token tiene que dar 401 exista o no (SRV-05).
#[given(
    expr = "un pedido HTTP POST a la ruta de la API que crea un participante, con el cuerpo de un participante {string}"
)]
fn pedido_crea_participante(mundo: &mut Mundo, seudonimo: String) {
    let direccion = mundo.direccion("/api/participantes");
    let mut pedido =
        Pedido::nuevo("POST", direccion).con_encabezado("Content-Type", "application/json");
    pedido.cuerpo = Some(serde_json::json!({ "seudonimo": seudonimo }).to_string());
    mundo.pedido = Some(pedido);
}

#[when("se manda el pedido")]
async fn se_manda(mundo: &mut Mundo) {
    let pedido = mundo.pedido.as_ref().expect("falta «un pedido HTTP»");
    let respuesta = match mundo.red.is_some().then(|| mundo.espacio()).flatten() {
        Some(nsenter) => pedido.mandar_con_curl(nsenter),
        None => pedido.mandar().await,
    };
    mundo.respuesta = Some(respuesta);
}

// Solo los estados que el servidor ya produce. Los 206 y 416 llegan con
// SRV-11, y sus escenarios quedan omitidos (D-20).
#[then(regex = r"^la respuesta tiene estado (200|401|403|404|405)$")]
fn estado_es(mundo: &mut Mundo, estado: u16) {
    let respuesta = mundo
        .respuesta
        .as_ref()
        .expect("falta «se manda el pedido»");
    assert_eq!(respuesta.estado, estado);
}

impl Mundo {
    fn la_respuesta(&self) -> &Respuesta {
        self.respuesta.as_ref().expect("falta «se manda el pedido»")
    }
}

#[then("la respuesta tiene estos encabezados")]
fn tiene_estos_encabezados(mundo: &mut Mundo, step: &cucumber::gherkin::Step) {
    let tabla = step.table.as_ref().expect("falta la tabla de encabezados");
    let respuesta = mundo.la_respuesta();
    for fila in &tabla.rows[1..] {
        let valor = mundo.resolver(&fila[1]);
        assert_eq!(
            respuesta.encabezado(&fila[0]),
            Some(valor.as_str()),
            "{}",
            fila[0]
        );
    }
}

#[then(expr = "la respuesta tiene el encabezado {string}")]
fn tiene_el_encabezado(mundo: &mut Mundo, nombre: String) {
    let respuesta = mundo.la_respuesta();
    assert!(respuesta.encabezado(&nombre).is_some(), "falta {nombre}");
}

#[then("el cuerpo de la respuesta es el JSON")]
fn cuerpo_json(mundo: &mut Mundo, step: &cucumber::gherkin::Step) {
    let esperado: serde_json::Value =
        serde_json::from_str(step.docstring.as_ref().expect("falta el JSON")).unwrap();
    let respuesta = mundo
        .respuesta
        .as_ref()
        .expect("falta «se manda el pedido»");
    let cuerpo: serde_json::Value = serde_json::from_str(&respuesta.cuerpo)
        .unwrap_or_else(|e| panic!("el cuerpo no es JSON ({e}): {}", respuesta.cuerpo));
    assert_eq!(cuerpo, esperado);
}

/// Lo que muestra la app es, hasta la vista «Resumen del proyecto», la
/// interfaz que sirve `GET /` con el token de la sesión (D-27).
async fn interfaz(mundo: &Mundo) -> String {
    let respuesta = Pedido::nuevo("GET", mundo.direccion("/"))
        .con_encabezado("Authorization", &format!("Bearer {}", mundo.token_valido()))
        .mandar()
        .await;
    assert_eq!(respuesta.estado, 200);
    respuesta.cuerpo
}

#[when("se guarda el estado que muestra la app")]
async fn guarda_el_estado(mundo: &mut Mundo) {
    mundo.estado_guardado = Some(interfaz(mundo).await);
}

#[then("la app muestra el mismo estado que el guardado")]
async fn mismo_estado(mundo: &mut Mundo) {
    let guardado = mundo
        .estado_guardado
        .clone()
        .expect("falta guardar el estado");
    assert_eq!(interfaz(mundo).await, guardado);
}
