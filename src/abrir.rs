//! `osr abrir`: abre un proyecto existente (PRY-04).
//!
//! Abrir es comprobar la carpeta, tomar el bloqueo (PRY-05) y anotar la
//! verificación al abrir (PRY-11, D-17). El servidor lo levanta el arranque.

use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::nuevo::{self, Momento};
use crate::plantillas;

/// Errores de `osr abrir`, con su código de salida.
#[derive(Debug)]
pub enum Error {
    NoExiste(PathBuf),
    Escritura(PathBuf, io::Error),
}

impl Error {
    pub fn codigo(&self) -> i32 {
        match self {
            Error::NoExiste(_) => 3,
            Error::Escritura(..) => 6,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NoExiste(ruta) => write!(f, "no existe la carpeta {}.", ruta.display()),
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

/// Escribe el bloqueo de este proceso y anota la verificación.
fn preparar(raiz: &Path, momento: &Momento) -> io::Result<()> {
    let bloqueo = plantillas::bloqueo(std::process::id(), &momento.marca);
    nuevo::archivo(&raiz.join(".osr/bloqueo"), &bloqueo)?;
    anotar(raiz, &plantillas::evento_verificacion(&momento.marca))
}

/// Abre el proyecto de `ruta` y lo deja con su bloqueo.
pub fn abrir(ruta: &Path, momento: &Momento) -> Result<(), Error> {
    if !ruta.is_dir() {
        return Err(Error::NoExiste(ruta.to_path_buf()));
    }
    preparar(ruta, momento).map_err(|error| Error::Escritura(ruta.to_path_buf(), error))
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
    fn sin_registro_de_eventos_no_se_abre_y_da_codigo_6() {
        let (_tmp, raiz) = proyecto();
        fs::remove_file(raiz.join("bitacora/registro.jsonl")).unwrap();
        let error = abrir(&raiz, &momento()).unwrap_err();
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
