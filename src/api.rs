//! Crear y abrir una investigación desde la pantalla (PRY-19, PRY-20), con un
//! solo proyecto por proceso (PRY-21).
//!
//! La persona nunca escribe una ruta: crear pide el nombre y la investigación
//! va a la carpeta de la aplicación (PRY-24); abrir pide el nombre de una de
//! sus subcarpetas. Todo el trabajo es síncrono y se hace con el candado del
//! proyecto abierto tomado, desde que se comprueba que no hay proyecto hasta
//! que queda anotado, así dos pedidos a la vez no abren dos proyectos.

use std::ffi::OsStr;
use std::io;
use std::path::PathBuf;

use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::abierto::{Abierto, Compartido};
use crate::nuevo::Momento;
use crate::{abrir, aplicacion, nuevo};

/// Por qué no se pudo crear o abrir, con su estado, su código y su mensaje.
#[derive(Debug)]
pub enum Falla {
    PedidoInvalido,
    NombreVacio,
    YaAbierto,
    Crear(nuevo::Error),
    Abrir(abrir::Error),
}

impl Falla {
    pub fn estado(&self) -> StatusCode {
        match self {
            Falla::PedidoInvalido => StatusCode::BAD_REQUEST,
            Falla::YaAbierto
            | Falla::Crear(nuevo::Error::Ocupada(_))
            | Falla::Abrir(abrir::Error::Abierto(_)) => StatusCode::CONFLICT,
            Falla::Crear(nuevo::Error::Escritura(..))
            | Falla::Abrir(abrir::Error::Escritura(..)) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }

    pub fn codigo(&self) -> &'static str {
        match self {
            Falla::PedidoInvalido => "pedido_invalido",
            Falla::NombreVacio => "nombre_vacio",
            Falla::YaAbierto => "proyecto_ya_abierto",
            Falla::Crear(nuevo::Error::NombreInvalido) => "nombre_invalido",
            Falla::Crear(nuevo::Error::Ocupada(_)) => "carpeta_ocupada",
            Falla::Crear(nuevo::Error::SinPadre(_)) | Falla::Abrir(abrir::Error::NoExiste(_)) => {
                "sin_carpeta"
            }
            Falla::Abrir(abrir::Error::Proyecto(_)) => "proyecto_invalido",
            Falla::Abrir(abrir::Error::Abierto(_)) => "abierto_en_otro_proceso",
            Falla::Crear(nuevo::Error::Escritura(..))
            | Falla::Abrir(abrir::Error::Escritura(..)) => "escritura",
        }
    }

    pub fn mensaje(&self) -> String {
        match self {
            Falla::PedidoInvalido => "El pedido no tiene la forma esperada.".into(),
            Falla::NombreVacio => "Hay que escribir el nombre de la investigación.".into(),
            Falla::YaAbierto => "Ya hay un proyecto abierto en esta sesión. Para abrir otro hay que cerrar Open Social Research y volver a abrirlo.".into(),
            Falla::Crear(error) => con_mayuscula(&error.to_string()),
            Falla::Abrir(error) => con_mayuscula(&error.to_string()),
        }
    }
}

/// El mensaje de la terminal sin `osr: ` y con mayúscula inicial, salvo que
/// empiece con una ruta o un nombre de archivo, que se dejan como están.
fn con_mayuscula(mensaje: &str) -> String {
    let primera_palabra = mensaje.split(' ').next().unwrap_or_default();
    if primera_palabra.contains(['/', '.']) {
        return mensaje.to_string();
    }
    let mut letras = mensaje.chars();
    match letras.next() {
        Some(primera) => primera.to_uppercase().chain(letras).collect(),
        None => String::new(),
    }
}

/// La carpeta de la aplicación, o el error de escritura si no se sabe cuál
/// es porque falta la carpeta personal.
fn carpeta_de_la_aplicacion(casa: Option<&OsStr>) -> Result<PathBuf, io::Error> {
    aplicacion::carpeta_desde(casa).ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))
}

#[derive(Deserialize)]
struct PedidoCrear {
    nombre: String,
}

#[derive(Deserialize)]
struct PedidoAbrir {
    carpeta: String,
}

/// La respuesta de una acción que salió bien.
#[derive(Serialize)]
pub struct Hecho {
    pub nombre: String,
    pub carpeta: String,
}

/// El cuerpo de una respuesta con error.
#[derive(Serialize)]
pub struct CuerpoFalla {
    pub error: &'static str,
    pub mensaje: String,
}

/// Crea la investigación que pide `cuerpo` en la carpeta de la aplicación y
/// la deja abierta en este proceso.
pub fn crear(
    abierto: &Compartido,
    cuerpo: &[u8],
    casa: Option<&OsStr>,
    momento: &Momento,
) -> Result<Abierto, Falla> {
    let pedido: PedidoCrear = serde_json::from_slice(cuerpo).map_err(|_| Falla::PedidoInvalido)?;
    let nombre = pedido.nombre.trim_matches([' ', '\t']);
    if nombre.is_empty() {
        return Err(Falla::NombreVacio);
    }
    let mut candado = abierto.tomar();
    if candado.is_some() {
        return Err(Falla::YaAbierto);
    }
    let escritura = |ruta: PathBuf| move |error| Falla::Crear(nuevo::Error::Escritura(ruta, error));
    let carpeta =
        carpeta_de_la_aplicacion(casa).map_err(escritura(PathBuf::from(aplicacion::NOMBRE)))?;
    aplicacion::crear(&carpeta).map_err(escritura(carpeta.clone()))?;
    let subcarpeta = aplicacion::libre(&carpeta, &aplicacion::simplificar(nombre));
    let ruta = carpeta.join(subcarpeta);
    let nombre = nuevo::crear(&ruta, Some(nombre), momento).map_err(Falla::Crear)?;
    let nuevo = Abierto {
        carpeta: ruta,
        nombre,
    };
    *candado = Some(nuevo.clone());
    Ok(nuevo)
}

/// Abre en este proceso la investigación de la subcarpeta que pide `cuerpo`.
pub fn abrir(
    abierto: &Compartido,
    cuerpo: &[u8],
    casa: Option<&OsStr>,
    momento: &Momento,
) -> Result<Abierto, Falla> {
    let pedido: PedidoAbrir = serde_json::from_slice(cuerpo).map_err(|_| Falla::PedidoInvalido)?;
    if !aplicacion::es_subcarpeta(&pedido.carpeta) {
        return Err(Falla::PedidoInvalido);
    }
    let mut candado = abierto.tomar();
    if candado.is_some() {
        return Err(Falla::YaAbierto);
    }
    let carpeta = carpeta_de_la_aplicacion(casa).map_err(|error| {
        Falla::Abrir(abrir::Error::Escritura(
            PathBuf::from(aplicacion::NOMBRE),
            error,
        ))
    })?;
    let ruta = carpeta.join(&pedido.carpeta);
    let nombre = abrir::abrir(&ruta, momento).map_err(Falla::Abrir)?;
    let nuevo = Abierto {
        carpeta: ruta,
        nombre,
    };
    *candado = Some(nuevo.clone());
    Ok(nuevo)
}

/// El estado y el cuerpo JSON de la respuesta de una acción.
pub fn respuesta(resultado: &Result<Abierto, Falla>) -> (StatusCode, String) {
    match resultado {
        Ok(abierto) => {
            let hecho = Hecho {
                nombre: abierto.nombre.clone(),
                carpeta: abierto.carpeta.to_string_lossy().into_owned(),
            };
            (StatusCode::OK, serde_json::to_string(&hecho).expect("JSON"))
        }
        Err(falla) => {
            let cuerpo = CuerpoFalla {
                error: falla.codigo(),
                mensaje: falla.mensaje(),
            };
            (
                falla.estado(),
                serde_json::to_string(&cuerpo).expect("JSON"),
            )
        }
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    fn momento() -> Momento {
        Momento {
            marca: "2026-10-12T05:00:00-06:00".into(),
            fecha: "2026-10-12".into(),
        }
    }

    fn crear_con(
        abierto: &Compartido,
        casa: &std::path::Path,
        cuerpo: serde_json::Value,
    ) -> Result<Abierto, Falla> {
        crear(
            abierto,
            cuerpo.to_string().as_bytes(),
            Some(casa.as_os_str()),
            &momento(),
        )
    }

    fn abrir_con(
        abierto: &Compartido,
        casa: &std::path::Path,
        cuerpo: serde_json::Value,
    ) -> Result<Abierto, Falla> {
        abrir(
            abierto,
            cuerpo.to_string().as_bytes(),
            Some(casa.as_os_str()),
            &momento(),
        )
    }

    #[test]
    fn la_mayuscula_no_toca_rutas_ni_archivos() {
        assert_eq!(
            con_mayuscula("no existe la carpeta /x."),
            "No existe la carpeta /x."
        );
        assert_eq!(
            con_mayuscula("/x no es un proyecto"),
            "/x no es un proyecto"
        );
        assert_eq!(
            con_mayuscula("proyecto.toml tiene un error"),
            "proyecto.toml tiene un error"
        );
        assert_eq!(con_mayuscula("ñandú"), "Ñandú");
        assert_eq!(con_mayuscula(""), "");
    }

    #[test]
    fn cada_falla_tiene_su_estado_codigo_y_mensaje() {
        let x = PathBuf::from("/x");
        let casos = [
            (
                Falla::PedidoInvalido,
                400,
                "pedido_invalido",
                "El pedido no tiene la forma esperada.",
            ),
            (
                Falla::NombreVacio,
                422,
                "nombre_vacio",
                "Hay que escribir el nombre de la investigación.",
            ),
            (
                Falla::YaAbierto,
                409,
                "proyecto_ya_abierto",
                "Ya hay un proyecto abierto en esta sesión. Para abrir otro hay que cerrar Open Social Research y volver a abrirlo.",
            ),
            (
                Falla::Crear(nuevo::Error::NombreInvalido),
                422,
                "nombre_invalido",
                "El nombre del proyecto tiene que tener entre 1 y 200 caracteres, sin saltos de línea.",
            ),
            (
                Falla::Crear(nuevo::Error::Ocupada(x.clone())),
                409,
                "carpeta_ocupada",
                "La carpeta /x ya existe y no está vacía. Hay que elegir una carpeta nueva o vacía.",
            ),
            (
                Falla::Crear(nuevo::Error::SinPadre(x.clone())),
                422,
                "sin_carpeta",
                "No existe la carpeta /x.",
            ),
            (
                Falla::Crear(nuevo::Error::Escritura(
                    x.clone(),
                    io::ErrorKind::StorageFull.into(),
                )),
                500,
                "escritura",
                "No se pudo crear el proyecto en /x (el disco está lleno).",
            ),
            (
                Falla::Abrir(abrir::Error::NoExiste(x.clone())),
                422,
                "sin_carpeta",
                "No existe la carpeta /x.",
            ),
            (
                Falla::Abrir(abrir::Error::Abierto(7)),
                409,
                "abierto_en_otro_proceso",
                "El proyecto ya está abierto en otro proceso (PID 7). Hay que usar esa pestaña o cerrar ese proceso.",
            ),
            (
                Falla::Abrir(abrir::Error::Escritura(
                    x,
                    io::ErrorKind::PermissionDenied.into(),
                )),
                500,
                "escritura",
                "No se pudo abrir el proyecto en /x (sin permiso de escritura).",
            ),
        ];
        for (falla, estado, codigo, mensaje) in casos {
            assert_eq!(falla.estado().as_u16(), estado, "{falla:?}");
            assert_eq!(falla.codigo(), codigo, "{falla:?}");
            assert_eq!(falla.mensaje(), mensaje, "{falla:?}");
        }
    }

    #[test]
    fn crear_va_a_la_carpeta_de_la_aplicacion_y_un_segundo_pedido_da_409() {
        let casa = tempfile::tempdir().unwrap();
        let abierto = Compartido::default();
        let hecho = crear_con(
            &abierto,
            casa.path(),
            serde_json::json!({"nombre": "  Año uno  "}),
        )
        .unwrap();
        let aplicacion = casa.path().join("Open Social Research");
        assert_eq!(
            hecho,
            Abierto {
                carpeta: aplicacion.join("ano-uno"),
                nombre: "Año uno".into()
            }
        );
        assert_eq!(abierto.actual(), Some(hecho));
        let modo = fs::metadata(&aplicacion).unwrap().permissions().mode();
        assert_eq!(modo & 0o777, 0o700);
        let falla =
            crear_con(&abierto, casa.path(), serde_json::json!({"nombre": "Dos"})).unwrap_err();
        assert!(matches!(falla, Falla::YaAbierto));
        assert!(!aplicacion.join("dos").exists());
        let falla = abrir_con(
            &abierto,
            casa.path(),
            serde_json::json!({"carpeta": "ano-uno"}),
        )
        .unwrap_err();
        assert!(matches!(falla, Falla::YaAbierto));
    }

    #[test]
    fn un_nombre_repetido_va_a_otra_subcarpeta() {
        let casa = tempfile::tempdir().unwrap();
        let aplicacion = casa.path().join("Open Social Research");
        fs::create_dir_all(aplicacion.join("estudio")).unwrap();
        let hecho = crear_con(
            &Compartido::default(),
            casa.path(),
            serde_json::json!({"nombre": "Estudio"}),
        )
        .unwrap();
        assert_eq!(hecho.carpeta, aplicacion.join("estudio-2"));
    }

    #[test]
    fn un_nombre_vacio_o_invalido_no_crea_nada() {
        let casa = tempfile::tempdir().unwrap();
        let abierto = Compartido::default();
        for nombre in ["", " \t "] {
            let falla = crear_con(&abierto, casa.path(), serde_json::json!({"nombre": nombre}))
                .unwrap_err();
            assert!(matches!(falla, Falla::NombreVacio), "{nombre:?}");
        }
        assert!(!casa.path().join("Open Social Research").exists());
        let falla = crear_con(
            &abierto,
            casa.path(),
            serde_json::json!({"nombre": "a".repeat(201)}),
        )
        .unwrap_err();
        assert_eq!(falla.codigo(), "nombre_invalido");
        assert_eq!(abierto.actual(), None);
    }

    #[test]
    fn sin_carpeta_personal_crear_y_abrir_dan_error_de_escritura() {
        let abierto = Compartido::default();
        let falla = crear(&abierto, br#"{"nombre": "x"}"#, None, &momento()).unwrap_err();
        assert_eq!(falla.codigo(), "escritura");
        assert!(
            falla.mensaje().contains("Open Social Research"),
            "{}",
            falla.mensaje()
        );
        let falla = abrir(&abierto, br#"{"carpeta": "x"}"#, None, &momento()).unwrap_err();
        assert_eq!(falla.codigo(), "escritura");
    }

    #[test]
    fn si_no_puede_crear_la_carpeta_de_la_aplicacion_es_error_de_escritura() {
        let casa = tempfile::tempdir().unwrap();
        fs::write(casa.path().join("Open Social Research"), "un archivo").unwrap();
        let falla = crear_con(
            &Compartido::default(),
            casa.path(),
            serde_json::json!({"nombre": "x"}),
        )
        .unwrap_err();
        assert_eq!(falla.codigo(), "escritura");
    }

    #[test]
    fn un_cuerpo_que_no_es_el_esperado_da_pedido_invalido() {
        let casa = tempfile::tempdir().unwrap();
        let abierto = Compartido::default();
        for cuerpo in [
            "no es json",
            "[]",
            "{}",
            r#"{"nombre": 3}"#,
            r#"{"ruta": "/x"}"#,
        ] {
            let falla = crear(
                &abierto,
                cuerpo.as_bytes(),
                Some(casa.path().as_os_str()),
                &momento(),
            )
            .unwrap_err();
            assert!(matches!(falla, Falla::PedidoInvalido), "{cuerpo}");
        }
        for carpeta in ["", ".", "..", "../otra", "a/b"] {
            let falla = abrir_con(
                &abierto,
                casa.path(),
                serde_json::json!({"carpeta": carpeta}),
            )
            .unwrap_err();
            assert!(matches!(falla, Falla::PedidoInvalido), "{carpeta}");
        }
        let falla = abrir(&abierto, b"{}", Some(casa.path().as_os_str()), &momento()).unwrap_err();
        assert!(matches!(falla, Falla::PedidoInvalido));
        assert_eq!(abierto.actual(), None);
    }

    #[test]
    fn abrir_una_subcarpeta_de_la_aplicacion() {
        let casa = tempfile::tempdir().unwrap();
        let ruta = casa.path().join("Open Social Research/estudio");
        fs::create_dir_all(ruta.parent().unwrap()).unwrap();
        nuevo::crear(&ruta, Some("Estudio"), &momento()).unwrap();
        fs::remove_file(ruta.join(".osr/bloqueo")).unwrap();
        let abierto = Compartido::default();
        let falla = abrir_con(
            &abierto,
            casa.path(),
            serde_json::json!({"carpeta": "no-existe"}),
        )
        .unwrap_err();
        assert_eq!(falla.codigo(), "sin_carpeta");
        let hecho = abrir_con(
            &abierto,
            casa.path(),
            serde_json::json!({"carpeta": "estudio"}),
        )
        .unwrap();
        assert_eq!(
            hecho,
            Abierto {
                carpeta: ruta,
                nombre: "Estudio".into()
            }
        );
        assert_eq!(abierto.actual(), Some(hecho));
    }

    #[test]
    fn la_respuesta_lleva_el_estado_y_el_json() {
        let hecho = Ok(Abierto {
            carpeta: PathBuf::from("/tmp/estudio"),
            nombre: "Estudio".into(),
        });
        assert_eq!(
            respuesta(&hecho),
            (
                StatusCode::OK,
                r#"{"nombre":"Estudio","carpeta":"/tmp/estudio"}"#.to_string()
            )
        );
        assert_eq!(
            respuesta(&Err(Falla::NombreVacio)),
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                r#"{"error":"nombre_vacio","mensaje":"Hay que escribir el nombre de la investigación."}"#.to_string()
            )
        );
    }
}
