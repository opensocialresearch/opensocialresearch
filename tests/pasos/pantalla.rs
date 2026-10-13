//! Pasos de la pantalla de inicio: crear y abrir por la API como lo hace la
//! pestaña (PRY-19 a PRY-21) y lo que muestra la página `/` (PRY-22, PRY-23).
//!
//! «se pide a la API» manda el pedido con el token válido en Authorization y
//! el `Origin` propio, como la pestaña, que pasa SRV-07.

use std::fs;
use std::path::{Path, PathBuf};

use cucumber::gherkin::Step;
use cucumber::{given, then, when};

use super::Mundo;
use super::http::{Pedido, Respuesta};

impl Mundo {
    /// Un POST a la API de la sesión, como lo manda la pestaña.
    fn post_de_la_pestana(&self, camino: &str, cuerpo: String) -> Pedido {
        let puerto = self.puerto.expect("osr no escribió su puerto");
        Pedido::nuevo("POST", self.direccion(camino))
            .con_encabezado("Authorization", &format!("Bearer {}", self.token_valido()))
            .con_encabezado("Origin", &format!("http://127.0.0.1:{puerto}"))
            .con_encabezado("Content-Type", "application/json")
            .con_cuerpo(cuerpo)
    }

    /// Manda el pedido a la API y, si abrió un proyecto, lo deja como
    /// `{proyecto}` del escenario.
    async fn pedir_a_la_api(&mut self, camino: &str, cuerpo: String) {
        let respuesta = self.post_de_la_pestana(camino, cuerpo).mandar().await;
        if respuesta.estado == 200 {
            let hecho: serde_json::Value =
                serde_json::from_str(&respuesta.cuerpo).expect("la respuesta es JSON");
            let carpeta = hecho["carpeta"]
                .as_str()
                .expect("la respuesta trae la carpeta");
            self.proyecto = PathBuf::from(carpeta);
        }
        self.respuesta = Some(respuesta);
    }

    async fn crear_por_la_api(&mut self, nombre: &str) {
        let cuerpo = serde_json::json!({ "nombre": self.resolver(nombre) });
        self.pedir_a_la_api("/api/proyecto/crear", cuerpo.to_string())
            .await;
    }

    async fn abrir_por_la_api(&mut self, carpeta: &str) {
        let cuerpo = serde_json::json!({ "carpeta": self.resolver(carpeta) });
        self.pedir_a_la_api("/api/proyecto/abrir", cuerpo.to_string())
            .await;
    }

    fn respondio_200(&self) {
        let respuesta = self.respuesta.as_ref().expect("falta el pedido");
        assert_eq!(respuesta.estado, 200, "{}", respuesta.cuerpo);
    }

    /// Una página de la sesión, con el token.
    pub(super) async fn pagina_en(&self, camino: &str) -> String {
        let respuesta = Pedido::nuevo("GET", self.direccion(camino))
            .con_encabezado("Authorization", &format!("Bearer {}", self.token_valido()))
            .mandar()
            .await;
        assert_eq!(respuesta.estado, 200, "GET {camino}");
        respuesta.cuerpo
    }
}

/// Las entidades que escribe el servidor, de vuelta a sus caracteres.
fn sin_entidades(texto: &str) -> String {
    texto
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&amp;", "&")
}

/// El texto de la página sin etiquetas, sin código y con los espacios
/// juntados, como lo leería una persona. Incluye las vistas ocultas.
pub(super) fn texto_visible(html: &str) -> String {
    let mut texto = String::new();
    let mut resto = html;
    while let Some(inicio) = resto.find('<') {
        texto.push_str(&resto[..inicio]);
        let etiqueta = &resto[inicio..];
        let salto = ["<script", "<style"]
            .iter()
            .find(|apertura| etiqueta.starts_with(*apertura))
            .map(|apertura| {
                let cierre = format!("</{}>", &apertura[1..]);
                etiqueta
                    .find(&cierre)
                    .map_or(etiqueta.len(), |fin| fin + cierre.len())
            })
            .unwrap_or_else(|| etiqueta.find('>').map_or(etiqueta.len(), |fin| fin + 1));
        // Las etiquetas de texto en línea no separan palabras; las demás sí.
        let nombre: String = etiqueta[1..]
            .trim_start_matches('/')
            .chars()
            .take_while(char::is_ascii_alphanumeric)
            .collect();
        if !["a", "em", "b", "i"].contains(&nombre.as_str()) {
            texto.push(' ');
        }
        resto = &etiqueta[salto..];
    }
    texto.push_str(resto);
    let texto = sin_entidades(&texto);
    texto.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Los nombres de la tabla de correspondencia de los datos de prueba.
fn nombres_de_prueba() -> Vec<String> {
    let tabla = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/datos/proyecto/identificable/correspondencia.csv");
    let texto = fs::read_to_string(tabla).expect("leer la correspondencia de prueba");
    let nombres: Vec<String> = texto
        .lines()
        .skip(1)
        .filter_map(|linea| linea.split(',').nth(1))
        .map(|nombre| nombre.trim_matches('"').to_string())
        .filter(|nombre| !nombre.is_empty())
        .collect();
    assert!(
        !nombres.is_empty(),
        "la correspondencia de prueba no tiene nombres"
    );
    nombres
}

#[given("osr está corriendo sin proyecto")]
#[when("osr está corriendo sin proyecto")]
fn corre_sin_proyecto(mundo: &mut Mundo) {
    mundo.lanzar("osr --sin-navegador");
    let osr = mundo.osr.as_mut().expect("osr no arrancó");
    assert!(osr.try_wait().unwrap().is_none(), "osr ya terminó");
}

#[when(expr = "se pide a la API crear la investigación {string}")]
async fn pide_crear(mundo: &mut Mundo, nombre: String) {
    mundo.crear_por_la_api(&nombre).await;
}

#[given(expr = "se pidió a la API crear la investigación {string}")]
async fn pidio_crear(mundo: &mut Mundo, nombre: String) {
    mundo.crear_por_la_api(&nombre).await;
    mundo.respondio_200();
}

#[when(expr = "se manda a la API de crear un proyecto el cuerpo {string}")]
async fn manda_cuerpo(mundo: &mut Mundo, cuerpo: String) {
    mundo.pedir_a_la_api("/api/proyecto/crear", cuerpo).await;
}

#[when(expr = "se pide a la API abrir la investigación {string}")]
async fn pide_abrir(mundo: &mut Mundo, carpeta: String) {
    mundo.abrir_por_la_api(&carpeta).await;
}

#[given(expr = "se pidió a la API abrir la investigación {string}")]
async fn pidio_abrir(mundo: &mut Mundo, carpeta: String) {
    mundo.abrir_por_la_api(&carpeta).await;
    mundo.respondio_200();
}

#[given(regex = r#"^\{proyecto\} es "([^"]+)"$"#)]
fn proyecto_es(mundo: &mut Mundo, ruta: String) {
    mundo.proyecto = PathBuf::from(mundo.resolver(&ruta));
}

/// Una copia del proyecto de prueba en una subcarpeta de la carpeta de la
/// aplicación, sin cambiar el `{proyecto}` del escenario.
#[given(expr = "la carpeta de la aplicación tiene una copia del proyecto de prueba en {string}")]
fn copia_en_la_aplicacion(mundo: &mut Mundo, subcarpeta: String) {
    let proyecto = mundo.proyecto.clone();
    fs::create_dir_all(mundo.aplicacion()).expect("crear la carpeta de la aplicación");
    mundo.proyecto = mundo.aplicacion().join(&subcarpeta);
    super::archivos::copia_del_proyecto(mundo);
    mundo.proyecto = proyecto;
}

#[given(
    expr = "la carpeta de la aplicación tiene una copia del proyecto de prueba con version_formato 2 en {string}"
)]
fn copia_version_2_en_la_aplicacion(mundo: &mut Mundo, subcarpeta: String) {
    let proyecto = mundo.proyecto.clone();
    fs::create_dir_all(mundo.aplicacion()).expect("crear la carpeta de la aplicación");
    mundo.proyecto = mundo.aplicacion().join(&subcarpeta);
    super::archivos::copia_con_otra_version(mundo);
    mundo.proyecto = proyecto;
}

/// Crea con `osr nuevo` un proyecto en la carpeta de la aplicación y lo
/// cierra, sin cambiar el `{proyecto}` del escenario.
#[given(expr = "la carpeta de la aplicación tiene un proyecto nuevo {string} en {string}")]
fn nuevo_en_la_aplicacion(mundo: &mut Mundo, nombre: String, subcarpeta: String) {
    let proyecto = mundo.proyecto.clone();
    fs::create_dir_all(mundo.aplicacion()).expect("crear la carpeta de la aplicación");
    let ruta = mundo.aplicacion().join(&subcarpeta);
    let ruta = ruta.to_string_lossy();
    mundo.lanzar(&format!(
        "osr nuevo \"{ruta}\" --nombre \"{nombre}\" --sin-navegador"
    ));
    let osr = mundo.osr.as_mut().expect("osr no arrancó");
    assert_eq!(
        super::orden::terminar(osr),
        Some(0),
        "osr nuevo no cerró bien"
    );
    mundo.osr = None;
    mundo.proyecto = proyecto;
}

#[when(
    regex = r#"^se piden a la API a la vez (\d+) investigaciones con los nombres "([^"]*?)1" a "[^"]*"$"#
)]
async fn crear_a_la_vez(mundo: &mut Mundo, cantidad: usize, prefijo: String) {
    let pedidos: Vec<Pedido> = (1..=cantidad)
        .map(|n| {
            let cuerpo = serde_json::json!({ "nombre": format!("{prefijo}{n}") });
            mundo.post_de_la_pestana("/api/proyecto/crear", cuerpo.to_string())
        })
        .collect();
    let tareas: Vec<_> = pedidos
        .into_iter()
        .map(|pedido| tokio::spawn(async move { pedido.mandar().await }))
        .collect();
    mundo.respuestas.clear();
    for (n, tarea) in tareas.into_iter().enumerate() {
        let respuesta: Respuesta = tarea.await.expect("mandar el pedido");
        mundo
            .respuestas
            .push((format!("{prefijo}{}", n + 1), respuesta));
    }
}

#[then(regex = r"^exactamente (\d+) respuesta tiene estado (\d+) y (\d+) tienen estado (\d+)$")]
fn reparto_de_estados(mundo: &mut Mundo, unos: usize, estado: u16, otros: usize, otro: u16) {
    let contar = |buscado: u16| {
        mundo
            .respuestas
            .iter()
            .filter(|(_, respuesta)| respuesta.estado == buscado)
            .count()
    };
    assert_eq!((contar(estado), contar(otro)), (unos, otros));
    assert_eq!(mundo.respuestas.len(), unos + otros);
}

#[then(regex = r"^la carpeta de la aplicación tiene exactamente (\d+) investigación(?:es)?$")]
fn cuantas_investigaciones(mundo: &mut Mundo, cantidad: usize) {
    let entradas = fs::read_dir(mundo.aplicacion()).expect("leer la carpeta de la aplicación");
    let investigaciones = entradas
        .filter_map(Result::ok)
        .filter(|entrada| entrada.path().join("proyecto.toml").is_file())
        .count();
    assert_eq!(investigaciones, cantidad);
}

#[then(expr = "la página {string} de la sesión lista estas investigaciones en este orden")]
async fn lista_en_orden(mundo: &mut Mundo, camino: String, step: &Step) {
    let visible = texto_visible(&mundo.pagina_en(&camino).await);
    let mut desde = 0;
    for fila in &step.table.as_ref().expect("falta la tabla").rows[1..] {
        let buscado = format!("{} {}", fila[0].trim(), fila[1].trim());
        let posicion = visible[desde..]
            .find(&buscado)
            .unwrap_or_else(|| panic!("falta {buscado:?}, o no va en orden, en {visible:?}"));
        desde += posicion + buscado.len();
    }
}

#[then(expr = "la página {string} de la sesión contiene {string}")]
async fn pagina_contiene(mundo: &mut Mundo, camino: String, texto: String) {
    let visible = texto_visible(&mundo.pagina_en(&camino).await);
    let texto = mundo.resolver(&texto);
    assert!(visible.contains(&texto), "falta {texto:?} en {visible:?}");
}

#[then(expr = "la página {string} de la sesión no contiene {string}")]
async fn pagina_no_contiene(mundo: &mut Mundo, camino: String, texto: String) {
    let visible = texto_visible(&mundo.pagina_en(&camino).await);
    let texto = mundo.resolver(&texto);
    assert!(!visible.contains(&texto), "sobra {texto:?} en {visible:?}");
}

#[then(expr = "la página {string} de la sesión contiene estos textos")]
async fn pagina_contiene_textos(mundo: &mut Mundo, camino: String, step: &Step) {
    let visible = texto_visible(&mundo.pagina_en(&camino).await);
    for fila in &step.table.as_ref().expect("falta la tabla").rows[1..] {
        let texto = mundo.resolver(fila[0].trim());
        assert!(visible.contains(&texto), "falta {texto:?} en {visible:?}");
    }
}

#[then(expr = "la página {string} de la sesión tiene el título {string}")]
async fn pagina_titulo(mundo: &mut Mundo, camino: String, titulo: String) {
    let pagina = mundo.pagina_en(&camino).await;
    let inicio = pagina.find("<title>").expect("la página no tiene título") + 7;
    let fin = pagina[inicio..]
        .find("</title>")
        .expect("título sin cerrar")
        + inicio;
    assert_eq!(sin_entidades(&pagina[inicio..fin]), mundo.resolver(&titulo));
}

#[then(expr = "la página {string} de la sesión no tiene ningún elemento {string}")]
async fn pagina_sin_elemento(mundo: &mut Mundo, camino: String, elemento: String) {
    let pagina = mundo.pagina_en(&camino).await.to_ascii_lowercase();
    for apertura in [">", " ", "/", "\n", "\t"] {
        let buscada = format!("<{elemento}{apertura}");
        assert!(!pagina.contains(&buscada), "la página tiene {buscada:?}");
    }
}

#[then(
    expr = "la página {string} de la sesión no contiene ninguno de los nombres de la tabla de correspondencia de prueba"
)]
async fn pagina_sin_nombres(mundo: &mut Mundo, camino: String) {
    let visible = texto_visible(&mundo.pagina_en(&camino).await).to_lowercase();
    let pagina = mundo.pagina_en(&camino).await.to_lowercase();
    for nombre in nombres_de_prueba() {
        let nombre = nombre.to_lowercase();
        assert!(
            !visible.contains(&nombre) && !pagina.contains(&nombre),
            "{nombre}"
        );
    }
}
