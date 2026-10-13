//! Pasos del diseño de la investigación (DIS): guardar y revisar por la API
//! como la pestaña, y mirar los archivos y las páginas del diseño.

use std::fs;

use cucumber::gherkin::Step;
use cucumber::{given, then, when};

use super::Mundo;
use super::http::Pedido;
use super::pantalla::texto_visible;

/// Cuántas preguntas guía tiene cada paso (DIS-01).
fn preguntas(paso: &str) -> usize {
    match paso {
        "01" | "03" | "05" => 4,
        "02" => 6,
        "04" => 3,
        otro => panic!("no existe el paso {otro}"),
    }
}

impl Mundo {
    /// Un POST de la pestaña a la API, con el token y el Origin propio.
    fn post_al_diseno(&self, camino: &str, cuerpo: String) -> Pedido {
        let puerto = self.puerto.expect("osr no escribió su puerto");
        Pedido::nuevo("POST", self.direccion(camino))
            .con_encabezado("Authorization", &format!("Bearer {}", self.token_valido()))
            .con_encabezado("Origin", &format!("http://127.0.0.1:{puerto}"))
            .con_encabezado("Content-Type", "application/json")
            .con_cuerpo(cuerpo)
    }

    /// La versión actual de un paso, según la API.
    async fn version_de(&self, paso: &str) -> String {
        let respuesta = Pedido::nuevo("GET", self.direccion(&format!("/api/diseno/{paso}")))
            .con_encabezado("Authorization", &format!("Bearer {}", self.token_valido()))
            .mandar()
            .await;
        assert_eq!(respuesta.estado, 200, "{}", respuesta.cuerpo);
        let datos: serde_json::Value = serde_json::from_str(&respuesta.cuerpo).expect("JSON");
        datos["version"].as_str().expect("versión").to_string()
    }

    async fn guardar_paso(&mut self, paso: &str, respuestas: Vec<String>, version: Option<String>) {
        let version = match version {
            Some(version) => version,
            None => self.version_de(paso).await,
        };
        let cuerpo = serde_json::json!({ "respuestas": respuestas, "version": version });
        let respuesta = self
            .post_al_diseno(&format!("/api/diseno/{paso}"), cuerpo.to_string())
            .mandar()
            .await;
        self.respuesta = Some(respuesta);
    }

    /// Las respuestas de un paso con la primera dada y las demás vacías.
    fn con_primera(&self, paso: &str, primera: &str) -> Vec<String> {
        let mut respuestas = vec![String::new(); preguntas(paso)];
        respuestas[0] = self.resolver(primera);
        respuestas
    }

    async fn revisar_paso(&mut self, paso: &str, revisado: bool) {
        let cuerpo = serde_json::json!({ "revisado": revisado });
        let respuesta = self
            .post_al_diseno(&format!("/api/diseno/{paso}/revision"), cuerpo.to_string())
            .mandar()
            .await;
        self.respuesta = Some(respuesta);
    }

    fn respondio_ok(&self) {
        let respuesta = self.respuesta.as_ref().expect("falta el pedido");
        assert_eq!(respuesta.estado, 200, "{}", respuesta.cuerpo);
    }
}

#[when(expr = "se guarda por la API el paso {string} con estas respuestas")]
async fn guarda_con_tabla(mundo: &mut Mundo, paso: String, step: &Step) {
    let tabla = step.table.as_ref().expect("falta la tabla de respuestas");
    let respuestas = tabla.rows[1..]
        .iter()
        .map(|fila| mundo.resolver(fila[0].trim()))
        .collect();
    mundo.guardar_paso(&paso, respuestas, None).await;
}

#[when(expr = "se guarda por la API el paso {string} con la primera respuesta {string}")]
async fn guarda_primera(mundo: &mut Mundo, paso: String, primera: String) {
    let respuestas = mundo.con_primera(&paso, &primera);
    mundo.guardar_paso(&paso, respuestas, None).await;
}

#[given(expr = "se guardó por la API el paso {string} con la primera respuesta {string}")]
async fn guardo_primera(mundo: &mut Mundo, paso: String, primera: String) {
    guarda_primera(mundo, paso, primera).await;
    mundo.respondio_ok();
}

#[given(expr = "se anotó la versión del paso {string}")]
async fn anota_version(mundo: &mut Mundo, paso: String) {
    mundo.version_anotada = Some(mundo.version_de(&paso).await);
}

#[when(
    expr = "se guarda por la API el paso {string} con la versión anotada y la primera respuesta {string}"
)]
async fn guarda_con_version(mundo: &mut Mundo, paso: String, primera: String) {
    let respuestas = mundo.con_primera(&paso, &primera);
    let version = mundo
        .version_anotada
        .clone()
        .expect("falta anotar la versión");
    mundo.guardar_paso(&paso, respuestas, Some(version)).await;
}

#[when(expr = "se manda a la API del paso {string} el cuerpo de {word} {word}")]
async fn manda_cuerpo_corto(mundo: &mut Mundo, paso: String, uno: String, dos: String) {
    manda_cuerpo_de(mundo, paso, format!("{uno} {dos}")).await;
}

#[when(
    regex = r#"^se manda a la API del paso "(\d\d)" el cuerpo de (una respuesta de 20001 caracteres|un texto que no es JSON)$"#
)]
async fn manda_cuerpo_largo(mundo: &mut Mundo, paso: String, caso: String) {
    manda_cuerpo_de(mundo, paso, caso).await;
}

async fn manda_cuerpo_de(mundo: &mut Mundo, paso: String, caso: String) {
    let cuerpo = match caso.as_str() {
        "tres respuestas" => {
            serde_json::json!({"respuestas": ["", "", ""], "version": ""}).to_string()
        }
        "cuatro respuestas" => {
            serde_json::json!({"respuestas": ["", "", "", ""], "version": ""}).to_string()
        }
        "una respuesta de 20001 caracteres" => {
            let mut respuestas = vec![String::new(); preguntas(&paso)];
            respuestas[0] = "a".repeat(20_001);
            serde_json::json!({"respuestas": respuestas, "version": ""}).to_string()
        }
        "un texto que no es JSON" => "no es json".to_string(),
        otro => panic!("caso desconocido: {otro}"),
    };
    let respuesta = mundo
        .post_al_diseno(&format!("/api/diseno/{paso}"), cuerpo)
        .mandar()
        .await;
    mundo.respuesta = Some(respuesta);
}

#[when(regex = r#"^se marca por la API el paso "(\d\d)" como (revisado|borrador)$"#)]
async fn marca_paso(mundo: &mut Mundo, paso: String, como: String) {
    mundo.revisar_paso(&paso, como == "revisado").await;
}

#[given(regex = r#"^se marcó por la API el paso "(\d\d)" como (revisado|borrador)$"#)]
async fn marco_paso(mundo: &mut Mundo, paso: String, como: String) {
    mundo.revisar_paso(&paso, como == "revisado").await;
    mundo.respondio_ok();
}

/// El tramo del texto visible de la vista general que va desde el número de
/// un paso hasta el siguiente.
fn fila_del_paso(visible: &str, paso: &str) -> String {
    let inicio = visible
        .find(&format!("{paso} "))
        .unwrap_or_else(|| panic!("falta el paso {paso} en {visible:?}"));
    let siguiente: u32 = paso.parse::<u32>().unwrap() + 1;
    let resto = &visible[inicio..];
    let fin = resto
        .find(&format!("{siguiente:02} "))
        .unwrap_or(resto.len());
    resto[..fin].to_string()
}

#[then(expr = "la página {string} de la sesión muestra el paso {string} con el estado {string}")]
async fn paso_con_estado(mundo: &mut Mundo, camino: String, paso: String, estado: String) {
    let visible = texto_visible(&mundo.pagina_en(&camino).await);
    let fila = fila_del_paso(&visible, &paso);
    assert!(
        fila.contains(&estado),
        "el paso {paso} no está en {estado:?}: {fila:?}"
    );
}

#[then(expr = "la página {string} de la sesión muestra los pasos con estos estados")]
async fn pasos_con_estados(mundo: &mut Mundo, camino: String, step: &Step) {
    let visible = texto_visible(&mundo.pagina_en(&camino).await);
    for fila in &step.table.as_ref().expect("falta la tabla").rows[1..] {
        let tramo = fila_del_paso(&visible, fila[0].trim());
        assert!(tramo.contains(fila[1].trim()), "{:?}: {tramo:?}", fila[1]);
    }
}

#[then(expr = "la página {string} de la sesión tiene un enlace {string} a {string}")]
async fn tiene_enlace(mundo: &mut Mundo, camino: String, texto: String, destino: String) {
    let pagina = mundo.pagina_en(&camino).await;
    let enlace = format!("href=\"{destino}\">{texto}</a>");
    assert!(pagina.contains(&enlace), "falta {enlace:?}");
}

#[given(regex = r#"^con la app abierta se agrega al final de "([^"]+)" la línea "(.*)"$"#)]
#[when(regex = r#"^con la app abierta se agrega al final de "([^"]+)" la línea "(.*)"$"#)]
fn agrega_linea_abierta(mundo: &mut Mundo, archivo: String, linea: String) {
    let ruta = mundo.ruta(&archivo);
    let mut texto =
        fs::read_to_string(&ruta).unwrap_or_else(|e| panic!("leer {}: {e}", ruta.display()));
    texto.push_str(&format!("{linea}\n"));
    fs::write(&ruta, texto).expect("escribir el archivo");
}

#[then(expr = "el archivo {string} es exactamente")]
fn archivo_exacto(mundo: &mut Mundo, archivo: String, step: &Step) {
    let esperado = format!(
        "{}\n",
        step.docstring
            .as_ref()
            .expect("falta el texto")
            .trim_end_matches('\n')
            .trim_start_matches('\n')
    );
    let actual = fs::read_to_string(mundo.ruta(&archivo)).expect("leer el archivo");
    assert_eq!(actual, esperado);
}

#[then(expr = "el archivo {string} contiene {string}")]
fn archivo_contiene(mundo: &mut Mundo, archivo: String, texto: String) {
    let actual = fs::read_to_string(mundo.ruta(&archivo)).expect("leer el archivo");
    let texto = mundo.resolver(&texto);
    assert!(actual.contains(&texto), "{archivo} no contiene {texto:?}");
}

#[then(expr = "el archivo {string} contiene {string} en NFC y no en NFD")]
fn archivo_nfc(mundo: &mut Mundo, archivo: String, texto: String) {
    use unicode_normalization::UnicodeNormalization;
    let actual = fs::read_to_string(mundo.ruta(&archivo)).expect("leer el archivo");
    let nfc: String = texto.nfc().collect();
    let nfd: String = texto.nfd().collect();
    assert!(actual.contains(&nfc), "{archivo} no tiene {nfc:?} en NFC");
    assert!(!actual.contains(&nfd), "{archivo} tiene {nfd:?} en NFD");
}

#[then(expr = "el archivo {string} no empieza con BOM")]
fn sin_bom(mundo: &mut Mundo, archivo: String) {
    let bytes = fs::read(mundo.ruta(&archivo)).expect("leer el archivo");
    assert!(!bytes.starts_with(b"\xEF\xBB\xBF"));
}

#[then(regex = r#"^los permisos de "(diseno/[^"]*)" son (0[0-7]{3})$"#)]
fn permisos_del_diseno(mundo: &mut Mundo, ruta: String, modo: String) {
    use std::os::unix::fs::PermissionsExt;
    let actual = fs::metadata(mundo.ruta(&ruta))
        .expect("leer los permisos")
        .permissions()
        .mode();
    assert_eq!(actual & 0o777, u32::from_str_radix(&modo, 8).unwrap());
}
