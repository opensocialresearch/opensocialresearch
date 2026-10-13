//! El diseño de la investigación (DIS): cinco pasos con preguntas guía
//! citadas, escritos en espiral y guardados en `diseno/` del proyecto.
//!
//! Cada paso es un Markdown que se lee sin la herramienta (DIS-06), y las
//! marcas de tiempo de los guardados y las revisiones viven en
//! `diseno/estado.toml`. El estado de cada paso se calcula de esos archivos,
//! así sobrevive a borrar el índice.

use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use unicode_normalization::UnicodeNormalization;

use crate::nuevo::{self, Momento};
use crate::{escritura, indice, plantillas};

/// Un paso del diseño, con sus preguntas guía y las citas de su panel.
pub struct Paso {
    pub numero: &'static str,
    pub titulo: &'static str,
    pub archivo: &'static str,
    pub preguntas: &'static [&'static str],
    /// La fuente de cada pregunta guía, en el mismo orden (DIS-01).
    pub fuentes: &'static [&'static str],
    pub citas: &'static [(&'static str, &'static str)],
}

/// Los cinco pasos de DIS-01 y DIS-02.
pub const PASOS: [Paso; 5] = [
    Paso {
        numero: "01",
        titulo: "Tema e interés",
        archivo: "01-tema-e-interes.md",
        preguntas: &[
            "¿Qué parte del mundo social interesa estudiar?",
            "¿De dónde surge el interés: experiencia propia, lecturas, un encargo, una comunidad?",
            "¿Para qué y para quién se investiga?",
            "¿Con quiénes se define el tema y el problema, y quién decide sobre ellos: quien investiga, un equipo, las personas o comunidades involucradas?",
        ],
        fuentes: &[
            "Sautu et al. (2005, p. 30)",
            "Flick (2015, p. 34); Abarca et al. (2012, p. 32)",
            "Tutoría 1 de 05646, diap. 2; Fals Borda (2015, p. 263)",
            "Corona Berkin (2019, p. 58); Kaltmeier (2012, pp. 36 y 53); Fals Borda (2015, p. 285)",
        ],
        citas: &[
            (
                "Toda investigación o proyecto se inicia con un conjunto de consideraciones acerca de aquella parte del mundo social que se desea estudiar.",
                "Sautu et al. (2005, p. 30)",
            ),
            (
                "Se debe aclarar el por qué y el qué de la investigación antes de que se plantee la pregunta del cómo: el método.",
                "Kvale (2011, p. 77)",
            ),
            (
                "Éste no surge de la inspiración introspectiva de un sujeto–investigador sino que es producto de interacciones sociales.",
                "Kaltmeier (2012, p. 36), sobre el tema de investigación",
            ),
            (
                "resultó importante tener conciencia de “para quién” se trabajaba",
                "Fals Borda (2015, p. 263)",
            ),
        ],
    },
    Paso {
        numero: "02",
        titulo: "Posicionamiento y supuestos",
        archivo: "02-posicionamiento-y-supuestos.md",
        preguntas: &[
            "¿Desde dónde se investiga: trayectoria, experiencias, creencias, formación?",
            "¿Qué se supone que existe cuando se estudia este tema?",
            "¿Cómo se puede conocer esa realidad?",
            "¿Qué relación se establece entre quien investiga y las personas o los fenómenos que investiga?",
            "¿Qué papel tienen los valores en esta investigación?",
            "¿Con quién se comparte este posicionamiento y cómo se va a revisar a lo largo de la investigación?",
        ],
        fuentes: &[
            "Jiménez Cortés (2021, p. 183)",
            "Tutoría 1 de 05646, diap. 16; Sautu et al. (2005, p. 40)",
            "Tutoría 1 de 05646, diap. 16",
            "Sautu et al. (2005, p. 40); tutoría 1 de 05646, diap. 2; Pajares Sánchez (2020, p. 304)",
            "Villalobos Zamora (2017, pp. 7-8); Sautu et al. (2005, p. 40)",
            "Jiménez Cortés (2021, p. 183); Kaltmeier (2012, p. 52)",
        ],
        citas: &[
            (
                "la persona que investiga no lo hace al margen de sus vivencias, su formación, sus creencias y sus circunstancias vitales",
                "Jiménez Cortés (2021, p. 183)",
            ),
            (
                "toda investigación participativa comienza con una reflexión personal orientada a que emerjan prejuicios, presunciones, dudas o posturas subconscientes o invisibles",
                "Pajares Sánchez (2020, p. 304)",
            ),
            (
                "Coincidimos en que no todos los sociólogos tienen que ser filósofos y, ciertamente, gran parte de la investigación en ciencias sociales jamás se habría efectuado si tuviésemos que comenzar resolviendo las cuestiones fundamentales sobre el ser y el conocimiento. No obstante, es necesaria cierta reflexión sobre la esencia del conocimiento como premisa preliminar antes de abordar cualquier investigación.",
                "Della Porta y Keating (2013, pp. 32-33)",
            ),
        ],
    },
    Paso {
        numero: "03",
        titulo: "Problema, estado de la cuestión y marco teórico",
        archivo: "03-problema-estado-de-la-cuestion-y-marco-teorico.md",
        preguntas: &[
            "¿Cuál es el problema, todavía en forma preliminar?",
            "¿Por qué es importante estudiarlo y qué aporta, en la práctica y al conocimiento?",
            "¿Qué se sabe ya, en la bibliografía y entre quienes viven el problema, y qué falta saber?",
            "¿Con qué conceptos y teorías se va a mirar el problema, y cómo se relacionan entre sí?",
        ],
        fuentes: &[
            "Villalobos Zamora (2017, p. 261); Pajares Sánchez (2020, p. 302)",
            "Abarca et al. (2012, pp. 33 y 41); Villalobos Zamora (2017, p. 93)",
            "Villalobos Zamora (2017, p. 266); Sautu et al. (2005, p. 30); Fals Borda (2015, p. 282); Kaltmeier (2012, p. 39)",
            "Sautu et al. (2005, pp. 161-162)",
        ],
        citas: &[
            (
                "no existe problema alguno si no hay una persona –o grupo de personas– que lo defina como tal y lo padezca. Un problema es siempre problema para alguien",
                "Pajares Sánchez (2020, p. 302)",
            ),
            (
                "se plantea en forma preliminar un problema de investigación muy general",
                "Villalobos Zamora (2017, p. 261), sobre el diseño cualitativo",
            ),
            (
                "el marco teórico no es un estado del arte",
                "Sautu et al. (2005, p. 161)",
            ),
        ],
    },
    Paso {
        numero: "04",
        titulo: "Pregunta y objetivos",
        archivo: "04-pregunta-y-objetivos.md",
        preguntas: &[
            "¿Qué pregunta concreta va a responder la investigación, con su lugar, su período y las personas o casos que abarca?",
            "¿Qué objetivo general y qué objetivos específicos se desprenden de la pregunta?",
            "Si la investigación pone a prueba hipótesis, ¿cuáles son y cómo se relacionan con el diseño?",
        ],
        fuentes: &[
            "Flick (2015, p. 39); Abarca et al. (2012, pp. 39-40)",
            "Sautu et al. (2005, pp. 36-37)",
            "Abarca et al. (2012, p. 52); Villalobos Zamora (2017, pp. 106 y 273)",
        ],
        citas: &[
            (
                "Esta pregunta decide qué es importante [...] y, todavía en mayor medida, qué es menos importante y debe quedar fuera.",
                "Flick (2015, p. 39)",
            ),
            (
                "Los objetivos constituyen el pilar de una investigación y sirven de nexo entre la teoría y la metodología.",
                "Sautu et al. (2005, p. 37)",
            ),
            (
                "Las preguntas en la perspectiva horizontal no responden a las necesidades previamente construidas por un investigador.",
                "Corona Berkin (2019, p. 58)",
            ),
        ],
    },
    Paso {
        numero: "05",
        titulo: "Enfoque, metodología y métodos",
        archivo: "05-enfoque-metodologia-y-metodos.md",
        preguntas: &[
            "¿Qué enfoque permite responder mejor la pregunta y por qué?",
            "¿Con quiénes, con qué casos o con qué fuentes se va a trabajar, en qué lugar, y con qué criterios se eligen?",
            "¿Con qué técnicas se producirán los datos, quiénes participarán en producirlos y analizarlos, y cómo se analizarán?",
            "¿Qué acuerdos éticos pide este diseño: riesgos para las personas, consentimiento, anonimato o uso de sus nombres, devolución de resultados y autoría?",
        ],
        fuentes: &[
            "Della Porta y Keating (2013, p. 339); tutoría 1 de 05646, diap. 12",
            "Sautu et al. (2005, p. 162); Flick (2015, p. 58); Villalobos Zamora (2017, p. 285)",
            "Sautu et al. (2005, pp. 38 y 157); Corona Berkin (2019, p. 56)",
            "Tutoría 1 de 05646, diap. 18-20; Kvale (2011, p. 77); Flick (2015, p. 91); Kaltmeier (2012, p. 54)",
        ],
        citas: &[
            (
                "lo que deben hacer los especialistas es escoger un enfoque, una metodología y los métodos específicos que mejor se ajusten a las preguntas que plantean.",
                "Della Porta y Keating (2013, p. 339)",
            ),
            (
                "La selección de la metodología se define de acuerdo con las características del objeto de estudio y con la información que se desea obtener.",
                "Villalobos Zamora (2017, p. 57)",
            ),
        ],
    },
];

/// Las ayudas en gris de las preguntas que, sin ejemplo, quedan en blanco
/// (D-55). Dan opciones de distintas tradiciones sin elegir por la persona.
const AYUDAS: [(&str, &str); 3] = [
    (
        "¿Qué se supone que existe cuando se estudia este tema?",
        "Por ejemplo: si lo que se estudia es un hecho que se puede medir, algo que las personas viven y cuentan, algo que se construye entre personas, o varias de esas cosas.",
    ),
    (
        "¿Cómo se puede conocer esa realidad?",
        "Por ejemplo: contando y midiendo, escuchando y observando, o construyendo el conocimiento con las personas.",
    ),
    (
        "¿Qué enfoque permite responder mejor la pregunta y por qué?",
        "Cualitativo, cuantitativo, mixto o participativo, y por qué ese le sirve a la pregunta del paso 04.",
    ),
];

/// La ayuda de una pregunta guía, si tiene.
pub fn ayuda(pregunta: &str) -> Option<&'static str> {
    AYUDAS
        .iter()
        .find(|(con_ayuda, _)| *con_ayuda == pregunta)
        .map(|(_, ayuda)| *ayuda)
}

/// La carpeta del diseño dentro del proyecto.
pub const CARPETA: &str = "diseno";

/// Lo más largo que puede ser una respuesta, en caracteres.
pub const LARGO_MAXIMO: usize = 20_000;

/// El paso con ese número, si existe.
pub fn paso(numero: &str) -> Option<&'static Paso> {
    PASOS.iter().find(|paso| paso.numero == numero)
}

/// La ruta del archivo de un paso, relativa al proyecto.
pub fn relativa(paso: &Paso) -> String {
    format!("{CARPETA}/{}", paso.archivo)
}

/// Lo que dice el archivo de un paso: una respuesta por pregunta guía y las
/// secciones que no son preguntas guía, que se conservan tal cual.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Contenido {
    pub respuestas: Vec<String>,
    pub otras: String,
}

/// Si una línea de una respuesta se leería como encabezado de nivel 2,
/// contando las barras invertidas que ya lleve delante.
fn parece_encabezado(linea: &str) -> bool {
    linea.trim_start_matches('\\').starts_with("## ")
}

/// Una respuesta como va en el archivo: cada línea que empezaría un
/// encabezado de nivel 2 lleva una barra invertida delante, así se lee como
/// texto y no como otra sección (DIS-06).
fn escapar_respuesta(respuesta: &str) -> String {
    respuesta
        .lines()
        .map(|linea| {
            if parece_encabezado(linea) {
                format!("\\{linea}")
            } else {
                linea.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Una línea del archivo dentro de una respuesta, sin la barra invertida que
/// le puso `escapar_respuesta`.
fn quitar_escape(linea: &str) -> &str {
    match linea.strip_prefix('\\') {
        Some(resto) if parece_encabezado(resto) => resto,
        _ => linea,
    }
}

/// Lee el Markdown de un paso (DIS-06). La respuesta de una pregunta es el
/// texto entre su encabezado y el siguiente de nivel 2; lo que está bajo un
/// encabezado de nivel 2 que no es una pregunta guía, o antes del primero
/// salvo el título, queda en `otras`.
pub fn leer(paso: &Paso, texto: &str) -> Contenido {
    let mut respuestas = vec![String::new(); paso.preguntas.len()];
    let mut otras = String::new();
    let mut actual: Option<usize> = None;
    let mut titulo_visto = false;
    for linea in texto.lines() {
        if let Some(encabezado) = linea.strip_prefix("## ") {
            actual = paso.preguntas.iter().position(|p| *p == encabezado.trim());
            if actual.is_none() {
                otras.push_str(linea);
                otras.push('\n');
            }
            continue;
        }
        if !titulo_visto && linea.starts_with("# ") {
            titulo_visto = true;
            continue;
        }
        match actual {
            Some(indice) => {
                respuestas[indice].push_str(quitar_escape(linea));
                respuestas[indice].push('\n');
            }
            None => {
                otras.push_str(linea);
                otras.push('\n');
            }
        }
    }
    Contenido {
        respuestas: respuestas.iter().map(|r| r.trim().to_string()).collect(),
        otras: otras.trim().to_string(),
    }
}

/// El Markdown de un paso con sus respuestas, y al final las otras
/// secciones, para que nada se pierda si las preguntas cambian.
pub fn escribir(paso: &Paso, contenido: &Contenido) -> String {
    let mut texto = format!("# {}\n", paso.titulo);
    for (pregunta, respuesta) in paso.preguntas.iter().zip(&contenido.respuestas) {
        texto.push_str(&format!("\n## {pregunta}\n"));
        if !respuesta.is_empty() {
            texto.push_str(&format!("\n{}\n", escapar_respuesta(respuesta)));
        }
    }
    if !contenido.otras.is_empty() {
        texto.push_str(&format!("\n{}\n", contenido.otras));
    }
    texto
}

/// Una respuesta como la guarda la app: NFC, fines de línea LF y sin
/// espacios ni saltos en los extremos (PRY-06).
pub fn normalizar(respuesta: &str) -> String {
    let unificada = respuesta.replace("\r\n", "\n").replace('\r', "\n");
    unificada.nfc().collect::<String>().trim().to_string()
}

/// Las marcas de un paso en `estado.toml`: cuándo se guardó y se revisó,
/// y en qué orden, porque dos cambios pueden caer en el mismo segundo.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Marcas {
    pub guardado: Option<String>,
    pub orden_guardado: Option<i64>,
    /// El orden del primer guardado con texto, que no pide revisar a los
    /// demás (D-55).
    pub orden_primero: Option<i64>,
    /// La huella del archivo que escribió la app, para notar una edición
    /// hecha fuera de ella.
    pub huella: Option<String>,
    pub revisado: Option<String>,
    pub orden_revisado: Option<i64>,
}

fn ruta_estado(raiz: &Path) -> std::path::PathBuf {
    raiz.join(CARPETA).join("estado.toml")
}

/// Las marcas de cada paso; un archivo que falta o no se puede leer cuenta
/// como sin marcas.
pub fn leer_marcas(raiz: &Path) -> BTreeMap<String, Marcas> {
    let Ok(texto) = fs::read_to_string(ruta_estado(raiz)) else {
        return BTreeMap::new();
    };
    let Ok(tabla) = texto.parse::<toml::Table>() else {
        return BTreeMap::new();
    };
    let fecha = |valor: &toml::Value, clave: &str| match valor.get(clave) {
        Some(toml::Value::Datetime(fecha)) => Some(fecha.to_string()),
        _ => None,
    };
    let orden =
        |valor: &toml::Value, clave: &str| valor.get(clave).and_then(toml::Value::as_integer);
    tabla
        .iter()
        .map(|(numero, valor)| {
            let marcas = Marcas {
                guardado: fecha(valor, "guardado"),
                orden_guardado: orden(valor, "orden_guardado"),
                orden_primero: orden(valor, "orden_primero"),
                huella: valor
                    .get("huella")
                    .and_then(toml::Value::as_str)
                    .map(String::from),
                revisado: fecha(valor, "revisado"),
                orden_revisado: orden(valor, "orden_revisado"),
            };
            (numero.clone(), marcas)
        })
        .collect()
}

/// `estado.toml` con una tabla por paso, en orden, con sus marcas como
/// fechas y horas de TOML y sus números de orden.
pub fn escribir_marcas(marcas: &BTreeMap<String, Marcas>) -> String {
    let mut texto = String::new();
    for (numero, marca) in marcas {
        if marca.guardado.is_none() && marca.revisado.is_none() {
            continue;
        }
        if !texto.is_empty() {
            texto.push('\n');
        }
        texto.push_str(&format!("[\"{numero}\"]\n"));
        if let (Some(guardado), Some(orden)) = (&marca.guardado, marca.orden_guardado) {
            texto.push_str(&format!(
                "guardado = {guardado}\norden_guardado = {orden}\n"
            ));
        }
        if let Some(primero) = marca.orden_primero {
            texto.push_str(&format!("orden_primero = {primero}\n"));
        }
        if let Some(huella) = &marca.huella {
            texto.push_str(&format!("huella = \"{huella}\"\n"));
        }
        if let (Some(revisado), Some(orden)) = (&marca.revisado, marca.orden_revisado) {
            texto.push_str(&format!(
                "revisado = {revisado}\norden_revisado = {orden}\n"
            ));
        }
    }
    texto
}

/// El siguiente número de orden, uno más que el mayor anotado.
fn siguiente_orden(marcas: &BTreeMap<String, Marcas>) -> i64 {
    marcas
        .values()
        .flat_map(|m| [m.orden_guardado, m.orden_revisado])
        .flatten()
        .max()
        .unwrap_or(0)
        + 1
}

/// El estado de un paso (DIS-05).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Estado {
    SinEmpezar,
    Borrador,
    Revisado,
    /// Otro paso se guardó después; con su número y la fecha de ese guardado.
    Coherencia {
        paso: String,
        fecha: String,
    },
}

impl Estado {
    pub fn texto(&self) -> &'static str {
        match self {
            Estado::SinEmpezar => "Sin empezar",
            Estado::Borrador => "En borrador",
            Estado::Revisado => "Revisado",
            Estado::Coherencia { .. } => "Revisar coherencia",
        }
    }

    pub fn icono(&self) -> &'static str {
        match self {
            Estado::SinEmpezar => "○",
            Estado::Borrador => "◐",
            Estado::Revisado => "✓",
            Estado::Coherencia { .. } => "△",
        }
    }

    pub fn clave(&self) -> &'static str {
        match self {
            Estado::SinEmpezar => "sin",
            Estado::Borrador => "borrador",
            Estado::Revisado => "revisado",
            Estado::Coherencia { .. } => "coherencia",
        }
    }
}

/// El contenido de cada paso según el disco.
pub fn contenidos(raiz: &Path) -> Vec<Contenido> {
    PASOS
        .iter()
        .map(|paso| {
            let texto = fs::read_to_string(raiz.join(relativa(paso))).unwrap_or_default();
            leer(paso, &texto)
        })
        .collect()
}

/// El estado de un paso con texto, según sus marcas y las de los demás.
/// Solo un paso revisado pide revisar la coherencia, y no por el primer
/// guardado con texto de otro paso (DIS-05, D-55).
fn estado_de(numero: &str, marcas: &BTreeMap<String, Marcas>) -> Estado {
    let propias = marcas.get(numero).cloned().unwrap_or_default();
    let Some(revision) = propias
        .orden_revisado
        .filter(|revision| Some(*revision) > propias.orden_guardado)
    else {
        return Estado::Borrador;
    };
    let otro = marcas
        .iter()
        .filter(|(otro, _)| otro.as_str() != numero)
        .filter_map(|(otro, m)| {
            Some((
                otro,
                m.orden_guardado?,
                m.orden_primero,
                m.guardado.clone()?,
            ))
        })
        .filter(|(_, orden, primero, _)| *orden > revision && Some(*orden) != *primero)
        .max_by_key(|(_, orden, _, _)| *orden);
    match otro {
        Some((paso, _, _, guardado)) => Estado::Coherencia {
            paso: paso.clone(),
            fecha: guardado.chars().take(10).collect(),
        },
        None => Estado::Revisado,
    }
}

/// Una fecha `AAAA-MM-DD` escrita como «13 de octubre».
pub fn fecha_larga(fecha: &str) -> String {
    const MESES: [&str; 12] = [
        "enero",
        "febrero",
        "marzo",
        "abril",
        "mayo",
        "junio",
        "julio",
        "agosto",
        "septiembre",
        "octubre",
        "noviembre",
        "diciembre",
    ];
    let partes: Vec<&str> = fecha.split('-').collect();
    match partes.as_slice() {
        [_, mes, dia] => match (mes.parse::<usize>(), dia.parse::<u32>()) {
            (Ok(mes @ 1..=12), Ok(dia)) => format!("{dia} de {}", MESES[mes - 1]),
            _ => fecha.to_string(),
        },
        _ => fecha.to_string(),
    }
}

/// El estado de los cinco pasos, calculado de los archivos y las marcas.
pub fn estados(raiz: &Path) -> Vec<Estado> {
    let marcas = leer_marcas(raiz);
    PASOS
        .iter()
        .zip(contenidos(raiz))
        .map(|(paso, contenido)| {
            let editado_fuera = marcas
                .get(paso.numero)
                .and_then(|m| m.huella.as_deref())
                .is_some_and(|huella| huella != version(raiz, paso));
            if contenido.respuestas.iter().all(String::is_empty) {
                Estado::SinEmpezar
            } else if editado_fuera {
                // Lo que se revisó ya no es lo que está en el disco.
                Estado::Borrador
            } else {
                estado_de(paso.numero, &marcas)
            }
        })
        .collect()
}

/// La versión de un paso: la huella de su archivo, o vacía si no existe.
pub fn version(raiz: &Path, paso: &Paso) -> String {
    fs::read(raiz.join(relativa(paso)))
        .map(|bytes| indice::huella(&bytes))
        .unwrap_or_default()
}

/// Por qué no se pudo guardar o revisar un paso.
#[derive(Debug)]
pub enum Falla {
    PedidoInvalido,
    RespuestaLarga,
    CambioExterno(String),
    PasoVacio,
    Escritura(io::Error),
}

impl Falla {
    pub fn estado(&self) -> u16 {
        match self {
            Falla::PedidoInvalido => 400,
            Falla::RespuestaLarga | Falla::PasoVacio => 422,
            Falla::CambioExterno(_) => 409,
            Falla::Escritura(_) => 500,
        }
    }

    pub fn codigo(&self) -> &'static str {
        match self {
            Falla::PedidoInvalido => "pedido_invalido",
            Falla::RespuestaLarga => "respuesta_larga",
            Falla::CambioExterno(_) => "cambio_externo",
            Falla::PasoVacio => "paso_vacio",
            Falla::Escritura(_) => "escritura",
        }
    }
}

impl fmt::Display for Falla {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Falla::PedidoInvalido => write!(f, "El pedido no tiene la forma esperada."),
            Falla::RespuestaLarga => {
                write!(f, "Cada respuesta puede tener hasta 20 000 caracteres.")
            }
            Falla::CambioExterno(ruta) => write!(
                f,
                "El archivo {ruta} cambió fuera de la herramienta mientras estaba abierto. No se guardó el cambio para no pisarlo."
            ),
            Falla::PasoVacio => write!(f, "Un paso sin texto no se puede marcar como revisado."),
            Falla::Escritura(error) => write!(
                f,
                "No se pudo guardar el diseño ({}).",
                nuevo::motivo(error)
            ),
        }
    }
}

/// Crea `diseno/` con 0700 si falta.
fn preparar_carpeta(raiz: &Path) -> io::Result<()> {
    let carpeta = raiz.join(CARPETA);
    if !carpeta.exists() {
        nuevo::carpeta(&carpeta)?;
    }
    Ok(())
}

fn guardar_marcas(raiz: &Path, marcas: &BTreeMap<String, Marcas>) -> io::Result<()> {
    escritura::atomica(
        &ruta_estado(raiz),
        escribir_marcas(marcas).as_bytes(),
        0o600,
    )
}

/// Guarda las respuestas de un paso si la versión es la del disco (DIS-04).
/// Si no cambian, no escribe ni anota nada. Devuelve la versión nueva.
pub fn guardar(
    raiz: &Path,
    paso: &Paso,
    respuestas: &[String],
    version_vista: &str,
    momento: &Momento,
) -> Result<String, Falla> {
    if respuestas.len() != paso.preguntas.len() {
        return Err(Falla::PedidoInvalido);
    }
    let respuestas: Vec<String> = respuestas.iter().map(|r| normalizar(r)).collect();
    if respuestas.iter().any(|r| r.chars().count() > LARGO_MAXIMO) {
        return Err(Falla::RespuestaLarga);
    }
    let actual = version(raiz, paso);
    if actual != version_vista {
        return Err(Falla::CambioExterno(relativa(paso)));
    }
    let ruta = raiz.join(relativa(paso));
    let anterior = leer(paso, &fs::read_to_string(&ruta).unwrap_or_default());
    if anterior.respuestas == respuestas {
        return Ok(actual);
    }
    let nuevo = Contenido {
        respuestas,
        otras: anterior.otras,
    };
    let texto = escribir(paso, &nuevo);
    let carpeta_nueva = !raiz.join(CARPETA).exists();
    preparar_carpeta(raiz).map_err(Falla::Escritura)?;
    escritura::atomica(&ruta, texto.as_bytes(), 0o600).map_err(Falla::Escritura)?;
    let mut marcas = leer_marcas(raiz);
    let orden = siguiente_orden(&marcas);
    let tenia_texto = anterior.respuestas.iter().any(|r| !r.is_empty());
    let primero = match marcas.get(paso.numero).and_then(|m| m.orden_primero) {
        Some(primero) if tenia_texto => primero,
        _ => orden,
    };
    marcas.insert(
        paso.numero.to_string(),
        Marcas {
            guardado: Some(momento.marca.clone()),
            orden_guardado: Some(orden),
            orden_primero: Some(primero),
            huella: Some(indice::huella(texto.as_bytes())),
            revisado: None,
            orden_revisado: None,
        },
    );
    guardar_marcas(raiz, &marcas).map_err(Falla::Escritura)?;
    let linea = plantillas::evento_diseno_guardado(&momento.marca, paso.numero);
    crate::abrir::anotar(raiz, &linea).map_err(Falla::Escritura)?;
    if carpeta_nueva {
        // El README describe cada carpeta presente (PRY-16).
        if let Ok(nombre) = crate::proyecto::validar(raiz) {
            let _ = crate::readme::regenerar(raiz, &nombre, momento);
        }
    }
    Ok(indice::huella(texto.as_bytes()))
}

/// Marca un paso como revisado o lo vuelve a borrador (DIS-05). Si el paso
/// ya está así, no escribe ni anota nada.
pub fn revisar(raiz: &Path, paso: &Paso, revisado: bool, momento: &Momento) -> Result<(), Falla> {
    let posicion = PASOS
        .iter()
        .position(|p| p.numero == paso.numero)
        .expect("paso de PASOS");
    let estado = estados(raiz).swap_remove(posicion);
    if revisado && estado == Estado::SinEmpezar {
        return Err(Falla::PasoVacio);
    }
    let mut marcas = leer_marcas(raiz);
    let ya_revisado = marcas
        .get(paso.numero)
        .is_some_and(|m| m.orden_revisado.is_some());
    if (revisado && estado == Estado::Revisado) || (!revisado && !ya_revisado) {
        return Ok(());
    }
    let mut orden = siguiente_orden(&marcas);
    let huella = version(raiz, paso);
    let marca = marcas.entry(paso.numero.to_string()).or_default();
    if revisado && marca.orden_guardado.is_none() {
        // Un archivo escrito fuera de la app: cuenta como guardado antes de
        // revisarlo, con su propio número de orden.
        marca.guardado = Some(momento.marca.clone());
        marca.orden_guardado = Some(orden);
        orden += 1;
    }
    marca.revisado = revisado.then(|| momento.marca.clone());
    marca.orden_revisado = revisado.then_some(orden);
    if revisado {
        // Se revisa lo que está en el disco, aunque se haya editado fuera.
        marca.huella = Some(huella);
    }
    preparar_carpeta(raiz).map_err(Falla::Escritura)?;
    guardar_marcas(raiz, &marcas).map_err(Falla::Escritura)?;
    let linea = plantillas::evento_diseno_revisado(&momento.marca, paso.numero, revisado);
    crate::abrir::anotar(raiz, &linea).map_err(Falla::Escritura)
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    fn momento(segundo: u32) -> Momento {
        Momento {
            marca: format!("2026-10-13T06:00:{segundo:02}-06:00"),
            fecha: "2026-10-13".into(),
        }
    }

    /// Un proyecto recién creado, sin bloqueo.
    fn proyecto() -> (tempfile::TempDir, std::path::PathBuf) {
        let tmp = tempfile::tempdir().unwrap();
        let raiz = tmp.path().join("estudio");
        nuevo::crear(&raiz, None, &momento(0)).unwrap();
        (tmp, raiz)
    }

    fn respuestas(paso: &Paso, primera: &str) -> Vec<String> {
        let mut todas = vec![String::new(); paso.preguntas.len()];
        todas[0] = primera.to_string();
        todas
    }

    fn guardar_primera(raiz: &Path, numero: &str, texto: &str, segundo: u32) -> String {
        let paso = paso(numero).unwrap();
        let version = version(raiz, paso);
        guardar(
            raiz,
            paso,
            &respuestas(paso, texto),
            &version,
            &momento(segundo),
        )
        .unwrap()
    }

    #[test]
    fn los_cinco_pasos_tienen_preguntas_citas_y_archivos_distintos() {
        assert_eq!(PASOS.len(), 5);
        let numeros: Vec<&str> = PASOS.iter().map(|p| p.numero).collect();
        assert_eq!(numeros, ["01", "02", "03", "04", "05"]);
        let preguntas: Vec<usize> = PASOS.iter().map(|p| p.preguntas.len()).collect();
        assert_eq!(preguntas, [4, 6, 4, 3, 4]);
        assert!(PASOS.iter().all(|p| !p.citas.is_empty()));
        assert_eq!(
            paso("03").unwrap().titulo,
            "Problema, estado de la cuestión y marco teórico"
        );
        assert!(paso("06").is_none() && paso("").is_none());
        assert_eq!(relativa(paso("01").unwrap()), "diseno/01-tema-e-interes.md");
    }

    #[test]
    fn escribir_y_leer_conservan_las_respuestas_y_lo_demas() {
        let paso = paso("04").unwrap();
        let contenido = Contenido {
            respuestas: vec![
                "¿Cómo?".into(),
                String::new(),
                "H1: algo.\nH2: otro.".into(),
            ],
            otras: "## Una pregunta vieja\n\nUna respuesta vieja.".into(),
        };
        let texto = escribir(paso, &contenido);
        assert!(texto.starts_with("# Pregunta y objetivos\n\n## ¿Qué pregunta concreta"));
        assert!(texto.ends_with("## Una pregunta vieja\n\nUna respuesta vieja.\n"));
        assert!(texto.contains("## ¿Qué objetivo general y qué objetivos específicos se desprenden de la pregunta?\n\n## Si"));
        assert_eq!(leer(paso, &texto), contenido);
    }

    #[test]
    fn leer_guarda_aparte_lo_que_no_es_una_pregunta() {
        let paso = paso("05").unwrap();
        let texto = "# Otro título\nUna nota suelta.\n## ¿Qué enfoque permite responder mejor la pregunta y por qué?\nCualitativo.\n## Extra\nAlgo.\n";
        let contenido = leer(paso, texto);
        assert_eq!(contenido.respuestas[0], "Cualitativo.");
        assert_eq!(contenido.respuestas[1], "");
        assert_eq!(contenido.otras, "Una nota suelta.\n## Extra\nAlgo.");
        assert_eq!(
            leer(paso, ""),
            Contenido {
                respuestas: vec![String::new(); 4],
                otras: String::new()
            }
        );
    }

    #[test]
    fn normalizar_deja_nfc_lf_y_sin_espacios_en_los_extremos() {
        assert_eq!(
            normalizar("  El an\u{303}o\r\nuno\rdos \n"),
            "El año\nuno\ndos"
        );
        assert_eq!(normalizar("\n\n"), "");
    }

    #[test]
    fn las_marcas_se_escriben_y_se_leen() {
        let mut marcas = BTreeMap::new();
        marcas.insert(
            "01".to_string(),
            Marcas {
                guardado: Some("2026-10-13T06:00:01-06:00".into()),
                orden_guardado: Some(1),
                revisado: Some("2026-10-13T06:00:02-06:00".into()),
                orden_revisado: Some(2),
                ..Marcas::default()
            },
        );
        marcas.insert("02".to_string(), Marcas::default());
        let texto = escribir_marcas(&marcas);
        assert_eq!(
            texto,
            "[\"01\"]\nguardado = 2026-10-13T06:00:01-06:00\norden_guardado = 1\nrevisado = 2026-10-13T06:00:02-06:00\norden_revisado = 2\n"
        );
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir(tmp.path().join(CARPETA)).unwrap();
        fs::write(tmp.path().join("diseno/estado.toml"), &texto).unwrap();
        let leidas = leer_marcas(tmp.path());
        assert_eq!(leidas.get("01"), marcas.get("01"));
        assert_eq!(siguiente_orden(&leidas), 3);
        assert_eq!(siguiente_orden(&BTreeMap::new()), 1);
        fs::write(tmp.path().join("diseno/estado.toml"), "no = [toml").unwrap();
        assert!(leer_marcas(tmp.path()).is_empty());
    }

    #[test]
    fn los_cuatro_estados() {
        let (_tmp, raiz) = proyecto();
        assert!(estados(&raiz).iter().all(|e| *e == Estado::SinEmpezar));
        guardar_primera(&raiz, "01", "Caminar de noche.", 1);
        assert_eq!(estados(&raiz)[0], Estado::Borrador);
        revisar(&raiz, paso("01").unwrap(), true, &momento(2)).unwrap();
        assert_eq!(estados(&raiz)[0], Estado::Revisado);
        revisar(&raiz, paso("01").unwrap(), false, &momento(3)).unwrap();
        assert_eq!(estados(&raiz)[0], Estado::Borrador);
        assert_eq!(
            estados(&raiz)[2],
            Estado::SinEmpezar,
            "sin texto no pide coherencia"
        );
    }

    #[test]
    fn la_coherencia_va_hacia_adelante_y_hacia_atras_en_el_mismo_segundo() {
        let (_tmp, raiz) = proyecto();
        guardar_primera(&raiz, "01", "Uno.", 5);
        guardar_primera(&raiz, "02", "Dos.", 5);
        guardar_primera(&raiz, "04", "Cuatro.", 5);
        for numero in ["01", "02", "04"] {
            revisar(&raiz, paso(numero).unwrap(), true, &momento(5)).unwrap();
        }
        assert_eq!(estados(&raiz)[1], Estado::Revisado);
        guardar_primera(&raiz, "01", "Uno, otra vez.", 5);
        let coherencia = Estado::Coherencia {
            paso: "01".into(),
            fecha: "2026-10-13".into(),
        };
        assert_eq!(estados(&raiz)[1], coherencia);
        assert_eq!(estados(&raiz)[3], coherencia);
        revisar(&raiz, paso("02").unwrap(), true, &momento(5)).unwrap();
        assert_eq!(estados(&raiz)[1], Estado::Revisado);
        guardar_primera(&raiz, "04", "Cuatro, otra vez.", 5);
        assert_eq!(
            estados(&raiz)[1],
            Estado::Coherencia {
                paso: "04".into(),
                fecha: "2026-10-13".into()
            }
        );
        assert_eq!(
            estados(&raiz)[0],
            Estado::Borrador,
            "un paso en borrador no pide coherencia"
        );
    }

    /// Marcas armadas a mano, como las de un `estado.toml` editado fuera.
    fn marcas(pasos: &[(&str, i64, Option<i64>, Option<i64>)]) -> BTreeMap<String, Marcas> {
        pasos
            .iter()
            .map(|(numero, guardado, primero, revisado)| {
                let marca = Marcas {
                    guardado: Some("2026-10-13T06:00:00-06:00".into()),
                    orden_guardado: Some(*guardado),
                    orden_primero: *primero,
                    revisado: revisado.map(|_| "2026-10-13T06:00:00-06:00".into()),
                    orden_revisado: *revisado,
                    huella: None,
                };
                (numero.to_string(), marca)
            })
            .collect()
    }

    #[test]
    fn con_ordenes_iguales_gana_la_comparacion_estricta() {
        let iguales = marcas(&[("01", 3, Some(1), Some(3))]);
        assert_eq!(
            estado_de("01", &iguales),
            Estado::Borrador,
            "revisado no es posterior"
        );
        let empate = marcas(&[("01", 1, Some(1), Some(4)), ("02", 4, Some(2), None)]);
        assert_eq!(
            estado_de("01", &empate),
            Estado::Revisado,
            "el otro no es posterior"
        );
        let posterior = marcas(&[("01", 1, Some(1), Some(4)), ("02", 5, Some(2), None)]);
        assert!(matches!(
            estado_de("01", &posterior),
            Estado::Coherencia { .. }
        ));
        assert_eq!(estado_de("09", &posterior), Estado::Borrador, "sin marcas");
    }

    #[test]
    fn un_encabezado_de_nivel_uno_en_una_respuesta_se_conserva() {
        let paso = paso("01").unwrap();
        let texto = "# Tema e interés\n\n## ¿Qué parte del mundo social interesa estudiar?\n\n# No es título\nTexto.\n";
        assert_eq!(leer(paso, texto).respuestas[0], "# No es título\nTexto.");
    }

    #[test]
    fn revisar_un_archivo_escrito_fuera_le_da_orden_propio() {
        let (_tmp, raiz) = proyecto();
        fs::create_dir(raiz.join(CARPETA)).unwrap();
        let paso = paso("01").unwrap();
        let texto = escribir(
            paso,
            &Contenido {
                respuestas: respuestas(paso, "A mano."),
                otras: String::new(),
            },
        );
        fs::write(raiz.join(relativa(paso)), texto).unwrap();
        assert_eq!(estados(&raiz)[0], Estado::Borrador);
        revisar(&raiz, paso, true, &momento(1)).unwrap();
        let marca = &leer_marcas(&raiz)["01"];
        assert_eq!(
            (marca.orden_guardado, marca.orden_revisado),
            (Some(1), Some(2))
        );
        assert_eq!(estados(&raiz)[0], Estado::Revisado);
    }

    #[test]
    fn el_primer_guardado_con_texto_no_pide_revisar_a_los_demas() {
        let (_tmp, raiz) = proyecto();
        guardar_primera(&raiz, "01", "Uno.", 1);
        revisar(&raiz, paso("01").unwrap(), true, &momento(2)).unwrap();
        guardar_primera(&raiz, "02", "Dos.", 3);
        assert_eq!(estados(&raiz)[0], Estado::Revisado);
        guardar_primera(&raiz, "02", "Dos, corregido.", 4);
        assert!(matches!(estados(&raiz)[0], Estado::Coherencia { .. }));
        revisar(&raiz, paso("01").unwrap(), true, &momento(5)).unwrap();
        guardar_primera(&raiz, "02", "Dos, corregido.", 6);
        assert_eq!(
            estados(&raiz)[0],
            Estado::Revisado,
            "guardar lo mismo no cuenta"
        );
        let paso3 = paso("03").unwrap();
        let vacias = vec![String::new(); paso3.preguntas.len()];
        guardar(&raiz, paso3, &vacias, "", &momento(7)).unwrap();
        guardar_primera(&raiz, "03", "Tres.", 8);
        assert_eq!(
            estados(&raiz)[0],
            Estado::Revisado,
            "el primer texto del 03 no cuenta"
        );
    }

    #[test]
    fn volver_a_escribir_un_paso_vaciado_cuenta_como_primer_texto() {
        let (_tmp, raiz) = proyecto();
        let paso3 = paso("03").unwrap();
        guardar_primera(&raiz, "03", "Tres.", 1);
        let vacias = vec![String::new(); paso3.preguntas.len()];
        guardar(&raiz, paso3, &vacias, &version(&raiz, paso3), &momento(2)).unwrap();
        guardar_primera(&raiz, "01", "Uno.", 3);
        revisar(&raiz, paso("01").unwrap(), true, &momento(4)).unwrap();
        guardar_primera(&raiz, "03", "Tres, otra vez.", 5);
        assert_eq!(estados(&raiz)[0], Estado::Revisado);
    }

    #[test]
    fn un_archivo_editado_fuera_vuelve_a_borrador() {
        let (_tmp, raiz) = proyecto();
        guardar_primera(&raiz, "01", "Uno.", 1);
        revisar(&raiz, paso("01").unwrap(), true, &momento(2)).unwrap();
        assert_eq!(estados(&raiz)[0], Estado::Revisado);
        let ruta = raiz.join("diseno/01-tema-e-interes.md");
        let texto = fs::read_to_string(&ruta)
            .unwrap()
            .replace("Uno.", "Uno, a mano.");
        fs::write(&ruta, texto).unwrap();
        assert_eq!(estados(&raiz)[0], Estado::Borrador);
        revisar(&raiz, paso("01").unwrap(), true, &momento(3)).unwrap();
        assert_eq!(
            estados(&raiz)[0],
            Estado::Revisado,
            "se revisa lo que está en el disco"
        );
    }

    #[test]
    fn una_linea_que_parece_encabezado_queda_en_la_respuesta() {
        let paso = paso("04").unwrap();
        let contenido = Contenido {
            respuestas: vec![
                "Pregunta\n## Contexto\nTexto".into(),
                "\\## ya escapada".into(),
                "## ".into(),
            ],
            otras: String::new(),
        };
        let texto = escribir(paso, &contenido);
        assert!(texto.contains("Pregunta\n\\## Contexto\nTexto"));
        assert!(texto.contains("\\\\## ya escapada"));
        assert_eq!(leer(paso, &texto).respuestas[0], contenido.respuestas[0]);
        assert_eq!(leer(paso, &texto).respuestas[1], contenido.respuestas[1]);
        assert_eq!(leer(paso, &texto).otras, "");
        assert_eq!(quitar_escape("\\no es"), "\\no es");
        assert!(
            parece_encabezado("## x") && parece_encabezado("\\## x") && !parece_encabezado("##x")
        );
    }

    #[test]
    fn la_fecha_larga_y_las_ayudas() {
        assert_eq!(fecha_larga("2026-10-13"), "13 de octubre");
        assert_eq!(fecha_larga("2026-01-05"), "5 de enero");
        assert_eq!(fecha_larga("2026-12-31"), "31 de diciembre");
        assert_eq!(fecha_larga("2026-13-01"), "2026-13-01");
        assert_eq!(fecha_larga("2026-00-01"), "2026-00-01");
        assert_eq!(fecha_larga("2026-10-xx"), "2026-10-xx");
        assert_eq!(fecha_larga("ayer"), "ayer");
        assert!(
            ayuda("¿Cómo se puede conocer esa realidad?")
                .unwrap()
                .starts_with("Por ejemplo")
        );
        assert_eq!(
            ayuda("¿Qué parte del mundo social interesa estudiar?"),
            None
        );
        assert!(PASOS.iter().all(|p| p.fuentes.len() == p.preguntas.len()));
        let con_ayuda = PASOS
            .iter()
            .flat_map(|p| p.preguntas)
            .filter(|q| ayuda(q).is_some());
        assert_eq!(con_ayuda.count(), 3);
    }

    #[test]
    fn los_textos_e_iconos_de_cada_estado() {
        let coherencia = Estado::Coherencia {
            paso: "01".into(),
            fecha: "x".into(),
        };
        let casos = [
            (Estado::SinEmpezar, "Sin empezar", "○", "sin"),
            (Estado::Borrador, "En borrador", "◐", "borrador"),
            (Estado::Revisado, "Revisado", "✓", "revisado"),
            (coherencia, "Revisar coherencia", "△", "coherencia"),
        ];
        for (estado, texto, icono, clave) in casos {
            assert_eq!(
                (estado.texto(), estado.icono(), estado.clave()),
                (texto, icono, clave)
            );
        }
    }

    #[test]
    fn guardar_escribe_el_archivo_las_marcas_y_un_evento() {
        let (_tmp, raiz) = proyecto();
        let paso = paso("01").unwrap();
        assert_eq!(version(&raiz, paso), "");
        let nueva = guardar(
            &raiz,
            paso,
            &respuestas(paso, "El an\u{303}o."),
            "",
            &momento(1),
        )
        .unwrap();
        let ruta = raiz.join(relativa(paso));
        let texto = fs::read_to_string(&ruta).unwrap();
        assert!(texto.contains("\n\nEl año.\n"));
        assert_eq!(nueva, indice::huella(texto.as_bytes()));
        assert_eq!(version(&raiz, paso), nueva);
        let modo = |r: &Path| fs::metadata(r).unwrap().permissions().mode() & 0o777;
        assert_eq!(modo(&ruta), 0o600);
        assert_eq!(modo(&raiz.join(CARPETA)), 0o700);
        assert_eq!(modo(&raiz.join("diseno/estado.toml")), 0o600);
        let registro = fs::read_to_string(raiz.join("bitacora/registro.jsonl")).unwrap();
        assert!(registro.ends_with(&plantillas::evento_diseno_guardado(&momento(1).marca, "01")));
        assert_eq!(leer_marcas(&raiz)["01"].orden_guardado, Some(1));
    }

    #[test]
    fn guardar_lo_mismo_no_escribe_ni_anota() {
        let (_tmp, raiz) = proyecto();
        let primera = guardar_primera(&raiz, "02", "Igual.", 1);
        let registro = fs::read_to_string(raiz.join("bitacora/registro.jsonl")).unwrap();
        let paso = paso("02").unwrap();
        let otra = guardar(
            &raiz,
            paso,
            &respuestas(paso, " Igual. "),
            &primera,
            &momento(2),
        )
        .unwrap();
        assert_eq!(otra, primera);
        assert_eq!(
            fs::read_to_string(raiz.join("bitacora/registro.jsonl")).unwrap(),
            registro
        );
    }

    #[test]
    fn guardar_conserva_las_secciones_agregadas_a_mano() {
        let (_tmp, raiz) = proyecto();
        guardar_primera(&raiz, "01", "Uno.", 1);
        let ruta = raiz.join("diseno/01-tema-e-interes.md");
        let mut texto = fs::read_to_string(&ruta).unwrap();
        texto.push_str("\n## Una pregunta que ya no está\n\nUna respuesta vieja.\n");
        fs::write(&ruta, &texto).unwrap();
        guardar_primera(&raiz, "01", "Dos.", 2);
        let texto = fs::read_to_string(&ruta).unwrap();
        assert!(
            texto.contains("Dos.")
                && texto.contains("## Una pregunta que ya no está\n\nUna respuesta vieja.\n")
        );
    }

    #[test]
    fn los_errores_de_guardar_no_escriben_nada() {
        let (_tmp, raiz) = proyecto();
        let paso = paso("01").unwrap();
        let tres = vec![String::new(); 3];
        assert!(matches!(
            guardar(&raiz, paso, &tres, "", &momento(1)),
            Err(Falla::PedidoInvalido)
        ));
        let larga = respuestas(paso, &"a".repeat(LARGO_MAXIMO + 1));
        assert!(matches!(
            guardar(&raiz, paso, &larga, "", &momento(1)),
            Err(Falla::RespuestaLarga)
        ));
        let justa = respuestas(paso, &"ñ".repeat(LARGO_MAXIMO));
        assert!(guardar(&raiz, paso, &justa, "", &momento(1)).is_ok());
        let falla = guardar(&raiz, paso, &respuestas(paso, "x"), "", &momento(2)).unwrap_err();
        assert!(
            matches!(&falla, Falla::CambioExterno(ruta) if ruta == "diseno/01-tema-e-interes.md")
        );
        assert!(
            !raiz
                .join("diseno/02-posicionamiento-y-supuestos.md")
                .exists()
        );
    }

    #[test]
    fn revisar_un_paso_vacio_es_un_error_y_lo_que_no_cambia_no_se_anota() {
        let (_tmp, raiz) = proyecto();
        let paso = paso("03").unwrap();
        assert!(matches!(
            revisar(&raiz, paso, true, &momento(1)),
            Err(Falla::PasoVacio)
        ));
        let registro = || fs::read_to_string(raiz.join("bitacora/registro.jsonl")).unwrap();
        let antes = registro();
        assert!(revisar(&raiz, paso, false, &momento(1)).is_ok());
        assert_eq!(
            registro(),
            antes,
            "volver a borrador sin estar revisado no anota"
        );
        guardar_primera(&raiz, "03", "Un problema.", 2);
        revisar(&raiz, paso, true, &momento(3)).unwrap();
        let una = registro();
        assert!(una.ends_with(&plantillas::evento_diseno_revisado(
            &momento(3).marca,
            "03",
            true
        )));
        revisar(&raiz, paso, true, &momento(4)).unwrap();
        assert_eq!(registro(), una, "revisar dos veces anota una");
        revisar(&raiz, paso, false, &momento(5)).unwrap();
        assert!(registro().ends_with(&plantillas::evento_diseno_revisado(
            &momento(5).marca,
            "03",
            false
        )));
        assert_eq!(estados(&raiz)[2], Estado::Borrador);
    }

    #[test]
    fn si_no_se_puede_escribir_es_un_error_de_escritura() {
        let (_tmp, raiz) = proyecto();
        fs::write(raiz.join(CARPETA), "un archivo").unwrap();
        let paso = paso("01").unwrap();
        let falla = guardar(&raiz, paso, &respuestas(paso, "x"), "", &momento(1)).unwrap_err();
        assert!(matches!(falla, Falla::Escritura(_)));
        let (_otro, raiz) = proyecto();
        guardar_primera(&raiz, "01", "x", 1);
        fs::remove_file(raiz.join("diseno/estado.toml")).unwrap();
        fs::create_dir_all(raiz.join("diseno/estado.toml/ocupado")).unwrap();
        assert!(matches!(
            revisar(&raiz, paso, true, &momento(2)),
            Err(Falla::Escritura(_))
        ));
    }

    #[test]
    fn cada_falla_tiene_su_estado_codigo_y_mensaje() {
        let casos = [
            (
                Falla::PedidoInvalido,
                400,
                "pedido_invalido",
                "El pedido no tiene la forma esperada.",
            ),
            (
                Falla::RespuestaLarga,
                422,
                "respuesta_larga",
                "Cada respuesta puede tener hasta 20 000 caracteres.",
            ),
            (
                Falla::CambioExterno("diseno/x.md".into()),
                409,
                "cambio_externo",
                "El archivo diseno/x.md cambió fuera de la herramienta mientras estaba abierto. No se guardó el cambio para no pisarlo.",
            ),
            (
                Falla::PasoVacio,
                422,
                "paso_vacio",
                "Un paso sin texto no se puede marcar como revisado.",
            ),
            (
                Falla::Escritura(io::ErrorKind::StorageFull.into()),
                500,
                "escritura",
                "No se pudo guardar el diseño (el disco está lleno).",
            ),
        ];
        for (falla, estado, codigo, mensaje) in casos {
            assert_eq!(
                (falla.estado(), falla.codigo(), falla.to_string().as_str()),
                (estado, codigo, mensaje)
            );
        }
    }
}
