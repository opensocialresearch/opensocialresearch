//! El bloqueo de proceso único, `.osr/bloqueo` (PRY-05).
//!
//! El bloqueo se escribe completo en un temporal y se pone en su lugar con un
//! enlace, que falla si ya existe, así dos procesos no lo toman a la vez y
//! nadie lee un bloqueo a medio escribir. Un bloqueo de un proceso que ya no
//! existe, o que no es un `osr`, se reemplaza con un `rename`.

use std::fs;
use std::io;
use std::path::Path;

use crate::{nuevo, plantillas, token};

/// Lo que decía un bloqueo abandonado, si se pudo leer.
#[derive(Debug, PartialEq)]
pub struct Abandonado {
    pub pid: Option<u32>,
    pub inicio: Option<String>,
}

/// Por qué no se tomó el bloqueo.
#[derive(Debug)]
pub enum Error {
    /// Otro `osr` vivo tiene el proyecto abierto, con este PID.
    Abierto(u32),
    Escritura(io::Error),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Escritura(error)
    }
}

/// Lee el PID y el inicio de un bloqueo, cada uno si está bien escrito.
fn leer(texto: &str) -> Abandonado {
    let valor = |clave: &str| {
        texto
            .lines()
            .find_map(|linea| linea.strip_prefix(clave)?.strip_prefix(" = "))
    };
    Abandonado {
        pid: valor("pid").and_then(|pid| pid.parse().ok()),
        inicio: valor("inicio").map(String::from),
    }
}

/// El nombre corto del programa de un proceso, de `/proc/<pid>/comm`.
fn programa(pid: &str) -> Option<String> {
    fs::read_to_string(format!("/proc/{pid}/comm")).ok()
}

/// Si `pid` es otro proceso vivo con el mismo programa que este. Un zombi ya
/// terminó aunque siga en `/proc`.
fn es_otro_osr_vivo(pid: u32) -> bool {
    if pid == std::process::id() {
        return false;
    }
    let Ok(estado) = fs::read_to_string(format!("/proc/{pid}/stat")) else {
        return false;
    };
    let vivo = estado
        .rsplit_once(") ")
        .is_some_and(|(_, resto)| !resto.starts_with(['Z', 'X']));
    vivo && programa(&pid.to_string()) == programa("self")
}

/// Pone `temporal` como bloqueo en `ruta`, reemplazando uno abandonado.
fn poner(temporal: &Path, ruta: &Path) -> Result<Option<Abandonado>, Error> {
    match fs::hard_link(temporal, ruta) {
        Ok(()) => return Ok(None),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
        Err(error) => return Err(error.into()),
    }
    let anterior = leer(&fs::read_to_string(ruta).unwrap_or_default());
    if let Some(pid) = anterior.pid.filter(|&pid| es_otro_osr_vivo(pid)) {
        return Err(Error::Abierto(pid));
    }
    fs::rename(temporal, ruta)?;
    Ok(Some(anterior))
}

/// Toma el bloqueo del proyecto en `raiz` para este proceso. Devuelve lo que
/// decía el bloqueo abandonado que reemplazó, si había uno.
pub fn tomar(raiz: &Path, inicio: &str) -> Result<Option<Abandonado>, Error> {
    let azar = token::nuevo()?;
    let temporal = raiz.join(format!(".osr/.bloqueo.osr-tmp-{}", &azar[..8]));
    nuevo::archivo(&temporal, &plantillas::bloqueo(std::process::id(), inicio))?;
    let resultado = poner(&temporal, &raiz.join(".osr/bloqueo"));
    let _ = fs::remove_file(&temporal);
    resultado
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use std::process::{Child, Command};
    use std::time::{Duration, Instant};

    const INICIO: &str = "2026-10-12T05:00:00-06:00";

    fn carpeta() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir(tmp.path().join(".osr")).unwrap();
        tmp
    }

    fn bloqueo(raiz: &Path) -> String {
        fs::read_to_string(raiz.join(".osr/bloqueo")).unwrap()
    }

    /// Las únicas rutas de `.osr/`, para ver que no quedó el temporal.
    fn en_osr(raiz: &Path) -> Vec<String> {
        let mut nombres: Vec<String> = fs::read_dir(raiz.join(".osr"))
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        nombres.sort();
        nombres
    }

    const HIJA: &str = "OSR_PRUEBA_BLOQUEO_HIJA";
    const LISTA: &str = "OSR_PRUEBA_BLOQUEO_LISTA";

    /// Otro proceso con el mismo programa que las pruebas, que duerme lo que
    /// diga la variable `HIJA`, en milisegundos. Antes crea el archivo de
    /// `LISTA`, para avisar que ya corre el programa y no la copia del fork,
    /// que todavía tiene el nombre del hilo que la lanzó.
    #[test]
    fn hija() {
        if let Some(ms) = std::env::var_os(HIJA) {
            if let Some(lista) = std::env::var_os(LISTA) {
                fs::write(lista, "").unwrap();
            }
            let ms: u64 = ms.to_str().unwrap().parse().unwrap();
            std::thread::sleep(Duration::from_millis(ms));
        }
    }

    fn lanzar_hija(ms: u64) -> Child {
        let tmp = carpeta();
        let lista = tmp.path().join("lista");
        let hija = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "bloqueo::pruebas::hija", "--quiet"])
            .env(HIJA, ms.to_string())
            .env(LISTA, &lista)
            .stdout(std::process::Stdio::null())
            .spawn()
            .unwrap();
        let inicio = Instant::now();
        while !lista.exists() {
            assert!(
                inicio.elapsed() < Duration::from_secs(10),
                "la hija no arrancó"
            );
            std::thread::sleep(Duration::from_millis(5));
        }
        hija
    }

    #[test]
    fn lee_pid_e_inicio_y_tolera_un_bloqueo_roto() {
        assert_eq!(
            leer(&plantillas::bloqueo(42, INICIO)),
            Abandonado {
                pid: Some(42),
                inicio: Some(INICIO.into())
            }
        );
        assert_eq!(
            leer("pid = x\n"),
            Abandonado {
                pid: None,
                inicio: None
            }
        );
        assert_eq!(leer("").pid, None);
        assert_eq!(leer("pidx = 3\n").pid, None);
    }

    #[test]
    fn otro_osr_vivo_es_solo_otro_proceso_vivo_del_mismo_programa() {
        let mut hija = lanzar_hija(10_000);
        assert!(es_otro_osr_vivo(hija.id()));
        assert!(!es_otro_osr_vivo(std::process::id()), "este proceso");
        assert!(!es_otro_osr_vivo(1), "otro programa");
        hija.kill().unwrap();
        hija.wait().unwrap();
        assert!(!es_otro_osr_vivo(hija.id()), "ya terminó");
    }

    #[test]
    fn un_zombi_no_esta_vivo() {
        let mut hija = lanzar_hija(0);
        let stat = format!("/proc/{}/stat", hija.id());
        let inicio = Instant::now();
        while !fs::read_to_string(&stat).unwrap().contains(") Z") {
            assert!(inicio.elapsed() < Duration::from_secs(10));
            std::thread::sleep(Duration::from_millis(10));
        }
        assert!(!es_otro_osr_vivo(hija.id()));
        hija.wait().unwrap();
    }

    #[test]
    fn sin_bloqueo_lo_toma_y_no_deja_el_temporal() {
        let tmp = carpeta();
        assert_eq!(tomar(tmp.path(), INICIO).unwrap(), None);
        assert_eq!(
            bloqueo(tmp.path()),
            plantillas::bloqueo(std::process::id(), INICIO)
        );
        assert_eq!(en_osr(tmp.path()), ["bloqueo"]);
    }

    #[test]
    fn con_otro_osr_vivo_no_lo_toma_ni_lo_toca() {
        let tmp = carpeta();
        let mut hija = lanzar_hija(10_000);
        let ajeno = plantillas::bloqueo(hija.id(), INICIO);
        fs::write(tmp.path().join(".osr/bloqueo"), &ajeno).unwrap();
        let resultado = tomar(tmp.path(), INICIO);
        hija.kill().unwrap();
        hija.wait().unwrap();
        assert!(matches!(resultado, Err(Error::Abierto(pid)) if pid == hija.id()));
        assert_eq!(bloqueo(tmp.path()), ajeno);
        assert_eq!(en_osr(tmp.path()), ["bloqueo"]);
    }

    #[test]
    fn un_bloqueo_abandonado_se_reemplaza() {
        let tmp = carpeta();
        fs::write(
            tmp.path().join(".osr/bloqueo"),
            plantillas::bloqueo(1, INICIO),
        )
        .unwrap();
        assert_eq!(
            tomar(tmp.path(), "2026-10-12T06:00:00-06:00").unwrap(),
            Some(Abandonado {
                pid: Some(1),
                inicio: Some(INICIO.into())
            })
        );
        assert_eq!(
            bloqueo(tmp.path()),
            plantillas::bloqueo(std::process::id(), "2026-10-12T06:00:00-06:00")
        );
        assert_eq!(en_osr(tmp.path()), ["bloqueo"]);
    }

    #[test]
    fn sin_carpeta_osr_es_un_error_de_escritura() {
        let tmp = tempfile::tempdir().unwrap();
        assert!(matches!(
            tomar(tmp.path(), INICIO),
            Err(Error::Escritura(_))
        ));
    }

    /// Un directorio no admite enlaces duros pero sí `rename`, así que solo
    /// el error del enlace impide ponerlo como bloqueo.
    #[test]
    fn un_error_del_enlace_que_no_es_ya_existe_es_de_escritura() {
        let tmp = carpeta();
        let temporal = tmp.path().join("carpeta");
        fs::create_dir(&temporal).unwrap();
        assert!(matches!(
            poner(&temporal, &tmp.path().join(".osr/bloqueo")),
            Err(Error::Escritura(_))
        ));
        assert_eq!(en_osr(tmp.path()), Vec::<String>::new());
    }
}
