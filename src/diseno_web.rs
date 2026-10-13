//! Las páginas y la API del diseño de la investigación (DIS-03 a DIS-05).
//!
//! Sin proyecto abierto, las páginas no existen y la API responde
//! `sin_proyecto`. Guardar y revisar corren con el candado del proyecto
//! abierto, así dos pedidos a la vez no escriben el mismo archivo.

use serde::{Deserialize, Serialize};

use crate::abierto::Compartido;
use crate::diseno::{self, Estado, Falla, PASOS, Paso};
use crate::nuevo::Momento;
use crate::resumen::{escapar, llenar};

const VISTA: &str = include_str!("paginas/diseno.html");
const PASO: &str = include_str!("paginas/diseno_paso.html");

/// Lo más largo de la primera respuesta que muestra la vista general.
const LINEA_MAXIMA: usize = 120;

/// La etiqueta de un estado, con su ícono y su texto.
fn etiqueta(estado: &Estado) -> String {
    format!(
        "<span class=\"etiqueta etiqueta--{}\"><span aria-hidden=\"true\">{}</span> {}</span>",
        estado.clave(),
        estado.icono(),
        estado.texto()
    )
}

/// La primera respuesta no vacía, recortada a 120 caracteres.
fn primera_linea(respuestas: &[String]) -> Option<String> {
    let primera = respuestas.iter().find(|r| !r.is_empty())?;
    let linea = primera.lines().next().unwrap_or_default();
    let recortada: String = linea.chars().take(LINEA_MAXIMA).collect();
    Some(if linea.chars().count() > LINEA_MAXIMA {
        format!("{}…", recortada.trim_end())
    } else {
        recortada
    })
}

/// La vista general del diseño del proyecto `nombre` en `raiz` (DIS-03).
pub fn vista(raiz: &std::path::Path, nombre: &str) -> String {
    let estados = diseno::estados(raiz);
    let contenidos = diseno::contenidos(raiz);
    let filas: String = PASOS
        .iter()
        .zip(&estados)
        .zip(&contenidos)
        .map(|((paso, estado), contenido)| {
            let linea = match primera_linea(&contenido.respuestas) {
                Some(linea) => format!("<span class=\"paso__linea\">{}</span>", escapar(&linea)),
                None => "<span class=\"paso__linea paso__linea--vacia\">Todavía sin texto</span>".into(),
            };
            format!(
                "      <li data-paso=\"{n}\"><a href=\"/diseno/{n}\"><span class=\"numero\">{n}</span><span class=\"paso__texto\"><strong>{t}</strong>{linea}</span>{e}</a></li>\n",
                n = paso.numero,
                t = escapar(paso.titulo),
                e = etiqueta(estado),
            )
        })
        .collect();
    llenar(VISTA, &[("nombre", &escapar(nombre)), ("pasos", &filas)])
}

/// La navegación entre pasos, con el actual marcado.
fn navegacion(actual: &Paso, estados: &[Estado]) -> String {
    PASOS
        .iter()
        .zip(estados)
        .map(|(paso, estado)| {
            let actual = if paso.numero == actual.numero {
                " aria-current=\"page\""
            } else {
                ""
            };
            format!(
                "      <li><a href=\"/diseno/{n}\"{actual}><span class=\"numero\">{n}</span><span>{t}</span>{e}</a></li>\n",
                n = paso.numero,
                t = escapar(paso.titulo),
                e = etiqueta(estado),
            )
        })
        .collect()
}

/// El aviso de coherencia de un paso, si lo tiene, con el paso que cambió
/// como enlace y la fecha escrita en palabras (DIS-05).
fn aviso(estado: &Estado) -> String {
    let Estado::Coherencia { paso, fecha } = estado else {
        return String::new();
    };
    let titulo = diseno::paso(paso).map_or("", |p| p.titulo);
    format!(
        "<div class=\"aviso\" role=\"status\">{} <span>El paso {paso}, <a href=\"/diseno/{paso}\">{}</a>, cambió el {}, después de la revisión de este paso. Hay que releerlo: si sigue de acuerdo con el cambio, se marca como revisado; si no, se corrige y se guarda.</span></div>",
        etiqueta(estado),
        escapar(titulo),
        diseno::fecha_larga(fecha)
    )
}

/// El enlace al paso siguiente, salvo en el último.
fn siguiente(posicion: usize) -> String {
    match PASOS.get(posicion + 1) {
        Some(paso) => format!(
            "<a class=\"siguiente\" href=\"/diseno/{}\">Siguiente paso: {}</a>",
            paso.numero,
            escapar(paso.titulo)
        ),
        None => String::new(),
    }
}

/// La página de un paso (DIS-04).
pub fn pagina(raiz: &std::path::Path, nombre: &str, paso: &Paso) -> String {
    let estados = diseno::estados(raiz);
    let posicion = PASOS
        .iter()
        .position(|p| p.numero == paso.numero)
        .expect("paso de PASOS");
    let estado = &estados[posicion];
    let contenido = &diseno::contenidos(raiz)[posicion];
    let campos: String = paso
        .preguntas
        .iter()
        .zip(&contenido.respuestas)
        .enumerate()
        .map(|(i, (pregunta, respuesta))| {
            let (ayuda, describe) = match diseno::ayuda(pregunta) {
                Some(ayuda) => (
                    format!("\n          <span id=\"a{i}\" class=\"ayuda\">{}</span>", escapar(ayuda)),
                    format!(" aria-describedby=\"a{i}\""),
                ),
                None => (String::new(), String::new()),
            };
            format!(
                "        <div class=\"campo\">\n          <label for=\"r{i}\">{p}</label>{ayuda}\n          <textarea id=\"r{i}\" class=\"campo-escritura\" name=\"respuesta\" rows=\"4\"{describe}>{r}</textarea>\n        </div>\n",
                p = escapar(pregunta),
                r = escapar(respuesta),
            )
        })
        .collect();
    let citas: String = paso
        .citas
        .iter()
        .map(|(cita, referencia)| {
            format!(
                "        <figure><blockquote class=\"biblio__cita\">{}</blockquote><figcaption class=\"biblio__ref\">{}</figcaption></figure>\n",
                escapar(cita),
                escapar(referencia)
            )
        })
        .collect();
    let fuentes: String = paso
        .preguntas
        .iter()
        .zip(paso.fuentes)
        .map(|(pregunta, fuente)| {
            format!(
                "        <li><span>{}</span><span class=\"biblio__ref\">{}</span></li>\n",
                escapar(pregunta),
                escapar(fuente)
            )
        })
        .collect();
    let (revision, revisado) = if *estado == Estado::Revisado {
        ("Volver a borrador", "false")
    } else {
        ("Marcar como revisado", "true")
    };
    llenar(
        PASO,
        &[
            ("nombre", &escapar(nombre)),
            ("numero", paso.numero),
            ("titulo", &escapar(paso.titulo)),
            ("version", &diseno::version(raiz, paso)),
            ("navegacion", &navegacion(paso, &estados)),
            ("aviso", &aviso(estado)),
            ("campos", &campos),
            ("citas", &citas),
            ("fuentes", &fuentes),
            ("revision", revision),
            ("revisado", revisado),
            ("estado", &etiqueta(estado)),
            ("siguiente", &siguiente(posicion)),
        ],
    )
}

#[derive(Deserialize)]
struct PedidoGuardar {
    respuestas: Vec<String>,
    version: String,
}

#[derive(Deserialize)]
struct PedidoRevisar {
    revisado: bool,
}

#[derive(Serialize)]
struct Datos {
    respuestas: Vec<String>,
    version: String,
    estado: &'static str,
}

#[derive(Serialize)]
struct CuerpoFalla {
    error: &'static str,
    mensaje: String,
}

/// El estado y el cuerpo JSON de una respuesta de la API del diseño.
pub type Respuesta = (u16, String);

fn falla(falla: &Falla) -> Respuesta {
    let cuerpo = CuerpoFalla {
        error: falla.codigo(),
        mensaje: falla.to_string(),
    };
    (
        falla.estado(),
        serde_json::to_string(&cuerpo).expect("JSON"),
    )
}

fn sin_proyecto() -> Respuesta {
    let cuerpo = CuerpoFalla {
        error: "sin_proyecto",
        mensaje: "No hay un proyecto abierto.".into(),
    };
    (409, serde_json::to_string(&cuerpo).expect("JSON"))
}

/// Las respuestas, la versión y el estado de un paso.
fn datos(raiz: &std::path::Path, paso: &Paso) -> Respuesta {
    let posicion = PASOS
        .iter()
        .position(|p| p.numero == paso.numero)
        .expect("paso de PASOS");
    let datos = Datos {
        respuestas: diseno::contenidos(raiz)[posicion].respuestas.clone(),
        version: diseno::version(raiz, paso),
        estado: diseno::estados(raiz)[posicion].texto(),
    };
    (200, serde_json::to_string(&datos).expect("JSON"))
}

/// `GET` de un paso.
pub fn leer(abierto: &Compartido, paso: &Paso) -> Respuesta {
    match abierto.actual() {
        Some(proyecto) => datos(&proyecto.carpeta, paso),
        None => sin_proyecto(),
    }
}

/// `POST` que guarda un paso.
pub fn guardar(abierto: &Compartido, paso: &Paso, cuerpo: &[u8], momento: &Momento) -> Respuesta {
    let candado = abierto.tomar();
    let Some(proyecto) = candado.as_ref() else {
        return sin_proyecto();
    };
    let Ok(pedido) = serde_json::from_slice::<PedidoGuardar>(cuerpo) else {
        return falla(&Falla::PedidoInvalido);
    };
    match diseno::guardar(
        &proyecto.carpeta,
        paso,
        &pedido.respuestas,
        &pedido.version,
        momento,
    ) {
        Ok(_) => datos(&proyecto.carpeta, paso),
        Err(error) => falla(&error),
    }
}

/// `POST` que marca un paso como revisado o lo vuelve a borrador.
pub fn revisar(abierto: &Compartido, paso: &Paso, cuerpo: &[u8], momento: &Momento) -> Respuesta {
    let candado = abierto.tomar();
    let Some(proyecto) = candado.as_ref() else {
        return sin_proyecto();
    };
    let Ok(pedido) = serde_json::from_slice::<PedidoRevisar>(cuerpo) else {
        return falla(&Falla::PedidoInvalido);
    };
    match diseno::revisar(&proyecto.carpeta, paso, pedido.revisado, momento) {
        Ok(()) => datos(&proyecto.carpeta, paso),
        Err(error) => falla(&error),
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use crate::abierto::Abierto;

    fn momento() -> Momento {
        Momento {
            marca: "2026-10-13T06:00:01-06:00".into(),
            fecha: "2026-10-13".into(),
        }
    }

    fn abierto() -> (tempfile::TempDir, Compartido) {
        let tmp = tempfile::tempdir().unwrap();
        let carpeta = tmp.path().join("estudio");
        crate::nuevo::crear(&carpeta, None, &momento()).unwrap();
        let abierto = Compartido::con(Some(Abierto {
            carpeta,
            nombre: "<b>Estudio</b>".into(),
        }));
        (tmp, abierto)
    }

    fn paso01() -> &'static Paso {
        diseno::paso("01").unwrap()
    }

    #[test]
    fn la_etiqueta_lleva_icono_y_texto() {
        assert_eq!(
            etiqueta(&Estado::Borrador),
            "<span class=\"etiqueta etiqueta--borrador\"><span aria-hidden=\"true\">◐</span> En borrador</span>"
        );
    }

    #[test]
    fn el_ultimo_paso_no_tiene_siguiente() {
        assert_eq!(siguiente(4), "");
        assert!(siguiente(0).contains("/diseno/02"));
        assert_eq!(aviso(&Estado::Borrador), "");
    }

    #[test]
    fn la_primera_linea_es_la_primera_respuesta_recortada() {
        let vacias = vec![String::new(), String::new()];
        assert_eq!(primera_linea(&vacias), None);
        let dos = vec![String::new(), "Hola\nmundo".into()];
        assert_eq!(primera_linea(&dos).as_deref(), Some("Hola"));
        let justa = vec!["a".repeat(LINEA_MAXIMA)];
        assert_eq!(primera_linea(&justa), Some("a".repeat(LINEA_MAXIMA)));
        let larga = vec!["a".repeat(LINEA_MAXIMA + 1)];
        assert_eq!(
            primera_linea(&larga),
            Some(format!("{}…", "a".repeat(LINEA_MAXIMA)))
        );
    }

    #[test]
    fn guardar_leer_y_revisar_por_la_api() {
        let (_tmp, abierto) = abierto();
        let (estado, cuerpo) = leer(&abierto, paso01());
        assert_eq!(estado, 200);
        let datos: serde_json::Value = serde_json::from_str(&cuerpo).unwrap();
        assert_eq!(datos["version"], "");
        assert_eq!(datos["estado"], "Sin empezar");
        assert_eq!(datos["respuestas"].as_array().unwrap().len(), 4);
        let pedido = serde_json::json!({"respuestas": ["Uno.", "", "", ""], "version": ""});
        let (estado, cuerpo) = guardar(
            &abierto,
            paso01(),
            pedido.to_string().as_bytes(),
            &momento(),
        );
        assert_eq!(estado, 200, "{cuerpo}");
        let datos: serde_json::Value = serde_json::from_str(&cuerpo).unwrap();
        assert_eq!(datos["estado"], "En borrador");
        assert_eq!(datos["respuestas"][0], "Uno.");
        let (estado, cuerpo) = revisar(&abierto, paso01(), br#"{"revisado": true}"#, &momento());
        assert_eq!(estado, 200, "{cuerpo}");
        assert!(cuerpo.contains("\"estado\":\"Revisado\""));
    }

    #[test]
    fn los_errores_de_la_api_tienen_su_json() {
        let (_tmp, abierto) = abierto();
        let (estado, cuerpo) = guardar(&abierto, paso01(), b"no es json", &momento());
        assert_eq!(
            (estado, cuerpo.as_str()),
            (
                400,
                r#"{"error":"pedido_invalido","mensaje":"El pedido no tiene la forma esperada."}"#
            )
        );
        let (estado, _) = revisar(&abierto, paso01(), b"{}", &momento());
        assert_eq!(estado, 400);
        let (estado, cuerpo) = revisar(&abierto, paso01(), br#"{"revisado": true}"#, &momento());
        assert_eq!(estado, 422);
        assert!(cuerpo.contains("paso_vacio"));
        let sin = Compartido::default();
        let esperado = (
            409,
            r#"{"error":"sin_proyecto","mensaje":"No hay un proyecto abierto."}"#.to_string(),
        );
        assert_eq!(leer(&sin, paso01()), esperado);
        assert_eq!(guardar(&sin, paso01(), b"{}", &momento()), esperado);
        assert_eq!(revisar(&sin, paso01(), b"{}", &momento()), esperado);
    }

    #[test]
    fn las_paginas_escapan_y_muestran_estado_aviso_y_citas() {
        let (_tmp, abierto) = abierto();
        let proyecto = abierto.actual().unwrap();
        let pagina_vista = vista(&proyecto.carpeta, &proyecto.nombre);
        assert!(
            pagina_vista.contains("&lt;b&gt;Estudio&lt;/b&gt;") && !pagina_vista.contains("<b>")
        );
        assert_eq!(
            pagina_vista.matches("Sin empezar").count(),
            5,
            "una etiqueta por paso"
        );
        assert_eq!(pagina_vista.matches("Todavía sin texto").count(), 5);
        assert!(pagina_vista.contains("href=\"/diseno/05\""));
        for numero in ["01", "02"] {
            let pedido = serde_json::json!({"respuestas": vec!["<i>x</i>"; diseno::paso(numero).unwrap().preguntas.len()], "version": ""});
            guardar(
                &abierto,
                diseno::paso(numero).unwrap(),
                pedido.to_string().as_bytes(),
                &momento(),
            );
        }
        revisar(
            &abierto,
            diseno::paso("02").unwrap(),
            br#"{"revisado": true}"#,
            &momento(),
        );
        let pedido = serde_json::json!({"respuestas": ["otra", "", "", ""], "version": diseno::version(&proyecto.carpeta, paso01())});
        guardar(
            &abierto,
            paso01(),
            pedido.to_string().as_bytes(),
            &momento(),
        );
        let pagina02 = pagina(
            &proyecto.carpeta,
            &proyecto.nombre,
            diseno::paso("02").unwrap(),
        );
        assert!(pagina02.contains("El paso 01, <a href=\"/diseno/01\">Tema e interés</a>, cambió el 13 de octubre, después de la revisión de este paso."));
        assert!(
            pagina02.contains("Siguiente paso: Problema, estado de la cuestión y marco teórico")
        );
        assert!(pagina02.contains("Fuentes de las preguntas"));
        assert!(
            pagina02.contains("aria-describedby=\"a1\""),
            "la pregunta con ayuda la referencia"
        );
        assert!(pagina02.contains("&lt;i&gt;x&lt;/i&gt;") && !pagina02.contains("<i>x"));
        assert_eq!(pagina02.matches("aria-current=\"page\"").count(), 1);
        assert!(pagina02.contains("<a href=\"/diseno/02\" aria-current=\"page\">"));
        assert!(pagina02.contains("Jiménez Cortés (2021, p. 183)"));
        assert!(
            pagina02.contains("Marcar como revisado")
                && pagina02.contains("data-revisado=\"true\"")
        );
        revisar(
            &abierto,
            diseno::paso("02").unwrap(),
            br#"{"revisado": true}"#,
            &momento(),
        );
        let pagina02 = pagina(
            &proyecto.carpeta,
            &proyecto.nombre,
            diseno::paso("02").unwrap(),
        );
        assert!(
            pagina02.contains("Volver a borrador") && pagina02.contains("data-revisado=\"false\"")
        );
        assert!(!pagina02.contains("class=\"aviso\""));
        assert!(!pagina02.contains("{{"));
        let vista = vista(&proyecto.carpeta, &proyecto.nombre);
        assert!(vista.contains("otra") && vista.contains("Revisado"));
    }
}
