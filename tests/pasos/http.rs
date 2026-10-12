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
}

/// Lo que respondió el servidor.
#[derive(Debug)]
pub struct Respuesta {
    pub estado: u16,
}

impl Mundo {
    fn nuevo_pedido(&mut self, metodo: &str, direccion: String) {
        self.pedido = Some(Pedido {
            metodo: metodo.to_string(),
            direccion,
            encabezados: Vec::new(),
        });
    }

    fn encabezado(&mut self, nombre: &str, valor: String) {
        let pedido = self.pedido.as_mut().expect("falta «un pedido HTTP»");
        pedido.encabezados.push((nombre.to_string(), valor));
    }

    fn token_valido(&self) -> String {
        self.token.clone().expect("osr no escribió su token")
    }
}

#[given(expr = "un pedido HTTP {word} a {string}")]
fn pedido_a(mundo: &mut Mundo, metodo: String, camino: String) {
    let puerto = mundo.puerto.expect("osr no escribió su puerto");
    let direccion = format!("http://127.0.0.1:{puerto}{}", mundo.resolver(&camino));
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
    async fn mandar(&self) -> Respuesta {
        let cliente = reqwest::Client::builder().no_proxy().build().unwrap();
        let metodo = reqwest::Method::from_bytes(self.metodo.as_bytes()).expect("método HTTP");
        let mut armado = cliente.request(metodo, &self.direccion);
        for (nombre, valor) in &self.encabezados {
            armado = armado.header(nombre, valor);
        }
        let respuesta = armado.send().await.expect("mandar el pedido");
        Respuesta {
            estado: respuesta.status().as_u16(),
        }
    }

    /// Manda el pedido con curl desde la red de prueba, donde corre `osr`.
    /// Solo trae el estado, que es lo que miran esos escenarios.
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
        }
    }
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

#[then(expr = "la respuesta tiene estado {int}")]
fn estado_es(mundo: &mut Mundo, estado: u16) {
    let respuesta = mundo
        .respuesta
        .as_ref()
        .expect("falta «se manda el pedido»");
    assert_eq!(respuesta.estado, estado);
}
