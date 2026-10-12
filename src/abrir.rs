//! `osr abrir`: abre un proyecto existente (PRY-04).
//!
//! Abrir es comprobar la carpeta, tomar el bloqueo (PRY-05) y anotar la
//! verificación al abrir (PRY-11, D-17). El servidor lo levanta el arranque.
//! Si algo falla después de tomar el bloqueo, se suelta.

use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::nuevo::{self, Momento};
use crate::{bloqueo, plantillas};

/// Errores de `osr abrir`, con su código de salida.
#[derive(Debug)]
pub enum Error {
    NoExiste(PathBuf),
    Abierto(u32),
    Escritura(PathBuf, io::Error),
}

impl Error {
    pub fn codigo(&self) -> i32 {
        match self {
            Error::NoExiste(_) => 3,
            Error::Abierto(_) => 4,
            Error::Escritura(..) => 6,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NoExiste(ruta) => write!(f, "no existe la carpeta {}.", ruta.display()),
            Error::Abierto(pid) => write!(
                f,
                "el proyecto ya está abierto en otro proceso (PID {pid}). Hay que usar esa pestaña o cerrar ese proceso."
            ),
            Error::Escritura(ruta, error) => write!(
                f,
                "no se pudo abrir el proyecto en {} ({}).",
                ruta.display(),
                nuevo::motivo(error)
            ),
        }
    }
}

/// Agrega una línea al registro de eventos y la lleva a disco (PRY-12).
fn anotar(raiz: &Path, linea: &str) -> io::Result<()> {
    let mut registro = fs::OpenOptions::new()
        .append(true)
        .open(raiz.join("bitacora/registro.jsonl"))?;
    registro.write_all(linea.as_bytes())?;
    registro.sync_data()
}

/// Anota el bloqueo abandonado, si lo hubo, y la verificación.
fn anotar_apertura(
    raiz: &Path,
    abandonado: Option<bloqueo::Abandonado>,
    momento: &Momento,
) -> io::Result<()> {
    if let Some(abandonado) = abandonado {
        let inicio = abandonado.inicio.as_deref();
        let linea = plantillas::evento_bloqueo_abandonado(&momento.marca, abandonado.pid, inicio);
        anotar(raiz, &linea)?;
    }
    anotar(raiz, &plantillas::evento_verificacion(&momento.marca))
}

/// Abre el proyecto de `ruta` y lo deja con su bloqueo.
pub fn abrir(ruta: &Path, momento: &Momento) -> Result<(), Error> {
    if !ruta.is_dir() {
        return Err(Error::NoExiste(ruta.to_path_buf()));
    }
    let escritura = |error| Error::Escritura(ruta.to_path_buf(), error);
    let abandonado = match bloqueo::tomar(ruta, &momento.marca) {
        Ok(abandonado) => abandonado,
        Err(bloqueo::Error::Abierto(pid)) => return Err(Error::Abierto(pid)),
        Err(bloqueo::Error::Escritura(error)) => return Err(escritura(error)),
    };
    anotar_apertura(ruta, abandonado, momento).map_err(|error| {
        let _ = fs::remove_file(ruta.join(".osr/bloqueo"));
        escritura(error)
    })
}

#[cfg(test)]
mod pruebas {
    use super::*;

    fn momento() -> Momento {
        Momento {
            marca: "2026-10-12T05:00:00-06:00".into(),
            fecha: "2026-10-12".into(),
        }
    }

    /// Un proyecto recién creado y ya cerrado, sin bloqueo.
    fn proyecto() -> (tempfile::TempDir, PathBuf) {
        let tmp = tempfile::tempdir().unwrap();
        let raiz = tmp.path().join("estudio");
        nuevo::crear(&raiz, None, &momento()).unwrap();
        fs::remove_file(raiz.join(".osr/bloqueo")).unwrap();
        (tmp, raiz)
    }

    #[test]
    fn una_ruta_que_no_existe_da_codigo_3_sin_crear_nada() {
        let tmp = tempfile::tempdir().unwrap();
        let ruta = tmp.path().join("no-existe");
        let error = abrir(&ruta, &momento()).unwrap_err();
        assert_eq!(error.codigo(), 3);
        assert_eq!(
            error.to_string(),
            format!("no existe la carpeta {}.", ruta.display())
        );
        assert!(!ruta.exists());
    }

    #[test]
    fn un_archivo_no_es_una_carpeta() {
        let tmp = tempfile::tempdir().unwrap();
        let ruta = tmp.path().join("archivo");
        fs::write(&ruta, "x").unwrap();
        assert!(matches!(abrir(&ruta, &momento()), Err(Error::NoExiste(_))));
    }

    #[test]
    fn toma_el_bloqueo_y_anota_la_verificacion() {
        let (_tmp, raiz) = proyecto();
        abrir(&raiz, &momento()).unwrap();
        assert_eq!(
            fs::read_to_string(raiz.join(".osr/bloqueo")).unwrap(),
            plantillas::bloqueo(std::process::id(), "2026-10-12T05:00:00-06:00")
        );
        let registro = fs::read_to_string(raiz.join("bitacora/registro.jsonl")).unwrap();
        let lineas: Vec<&str> = registro.lines().collect();
        assert_eq!(lineas.len(), 2);
        assert_eq!(
            format!("{}\n", lineas[1]),
            plantillas::evento_verificacion("2026-10-12T05:00:00-06:00")
        );
    }

    #[test]
    fn un_bloqueo_abandonado_queda_anotado_antes_de_la_verificacion() {
        let (_tmp, raiz) = proyecto();
        let viejo = "2026-10-11T05:00:00-06:00";
        fs::write(raiz.join(".osr/bloqueo"), plantillas::bloqueo(1, viejo)).unwrap();
        abrir(&raiz, &momento()).unwrap();
        let registro = fs::read_to_string(raiz.join("bitacora/registro.jsonl")).unwrap();
        let marca = "2026-10-12T05:00:00-06:00";
        let esperado = [
            plantillas::evento_bloqueo_abandonado(marca, Some(1), Some(viejo)),
            plantillas::evento_verificacion(marca),
        ]
        .concat();
        assert!(registro.ends_with(&esperado), "{registro}");
        assert_eq!(registro.lines().count(), 3);
    }

    #[test]
    fn con_otro_osr_abierto_da_codigo_4_con_su_pid() {
        let error = Error::Abierto(1234);
        assert_eq!(error.codigo(), 4);
        assert_eq!(
            error.to_string(),
            "el proyecto ya está abierto en otro proceso (PID 1234). Hay que usar esa pestaña o cerrar ese proceso."
        );
    }

    #[test]
    fn sin_carpeta_osr_da_codigo_6() {
        let (_tmp, raiz) = proyecto();
        fs::remove_dir_all(raiz.join(".osr")).unwrap();
        assert_eq!(abrir(&raiz, &momento()).unwrap_err().codigo(), 6);
    }

    #[test]
    fn sin_registro_de_eventos_no_se_abre_y_da_codigo_6() {
        let (_tmp, raiz) = proyecto();
        fs::remove_file(raiz.join("bitacora/registro.jsonl")).unwrap();
        let error = abrir(&raiz, &momento()).unwrap_err();
        assert!(!raiz.join(".osr/bloqueo").exists(), "soltó el bloqueo");
        assert_eq!(error.codigo(), 6);
        assert_eq!(
            error.to_string(),
            format!(
                "no se pudo abrir el proyecto en {} (error de escritura del sistema).",
                raiz.display()
            )
        );
    }
}
