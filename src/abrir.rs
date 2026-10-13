//! `osr abrir`: abre un proyecto existente (PRY-04).
//!
//! Abrir es comprobar la carpeta, validar `proyecto.toml` (PRY-03), tomar el
//! bloqueo (PRY-05), hacer las reparaciones de PRY-08 y PRY-02 y anotar la
//! verificación al abrir (PRY-11, D-17). El servidor lo levanta el arranque.
//! Si algo falla después de tomar el bloqueo, se suelta.

use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::nuevo::{self, Momento};
use crate::{bloqueo, indice, plantillas, proyecto, readme, reparar};

/// Errores de `osr abrir`, con su código de salida.
#[derive(Debug)]
pub enum Error {
    NoExiste(PathBuf),
    Proyecto(proyecto::Error),
    Abierto(u32),
    Escritura(PathBuf, io::Error),
}

impl Error {
    pub fn codigo(&self) -> i32 {
        match self {
            Error::NoExiste(_) => 3,
            Error::Proyecto(error) => error.codigo(),
            Error::Abierto(_) => 4,
            Error::Escritura(..) => 6,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NoExiste(ruta) => write!(f, "no existe la carpeta {}.", ruta.display()),
            Error::Proyecto(error) => error.fmt(f),
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

/// Repara el registro, las carpetas y el índice, anota en este orden la
/// línea descartada, las carpetas recreadas, el índice reconstruido, el
/// bloqueo abandonado y la verificación, y al final regenera el README como
/// pide el orden de PRY-04, anotando si guardó aparte uno editado. El recorte va antes que cualquier línea nueva, que si no
/// quedaría pegada a la incompleta (PRY-08).
fn anotar_apertura(
    raiz: &Path,
    nombre: &str,
    abandonado: Option<bloqueo::Abandonado>,
    momento: &Momento,
) -> io::Result<()> {
    if let Some(bytes) = reparar::recortar_registro(raiz)? {
        anotar(
            raiz,
            &plantillas::evento_linea_incompleta(&momento.marca, bytes),
        )?;
    }
    for carpeta in reparar::recrear_carpetas(raiz)? {
        anotar(
            raiz,
            &plantillas::evento_carpeta_recreada(&momento.marca, carpeta),
        )?;
    }
    if let Some(motivo) = indice::asegurar(raiz)? {
        let linea = plantillas::evento_indice_reconstruido(&momento.marca, motivo.nombre());
        anotar(raiz, &linea)?;
    }
    if let Some(abandonado) = abandonado {
        let inicio = abandonado.inicio.as_deref();
        let linea = plantillas::evento_bloqueo_abandonado(&momento.marca, abandonado.pid, inicio);
        anotar(raiz, &linea)?;
    }
    anotar(raiz, &plantillas::evento_verificacion(&momento.marca))?;
    if let Some(aparte) = readme::regenerar(raiz, nombre, momento)? {
        anotar(
            raiz,
            &plantillas::evento_readme_guardado(&momento.marca, &aparte),
        )?;
    }
    Ok(())
}

/// Abre el proyecto de `ruta`, lo deja con su bloqueo y devuelve su nombre.
pub fn abrir(ruta: &Path, momento: &Momento) -> Result<String, Error> {
    if !ruta.is_dir() {
        return Err(Error::NoExiste(ruta.to_path_buf()));
    }
    let nombre = proyecto::validar(ruta).map_err(Error::Proyecto)?;
    let escritura = |error| Error::Escritura(ruta.to_path_buf(), error);
    // Sin `.osr/` no hay bloqueo de nadie, así que crearla no pisa a otro
    // proceso; el índice que faltaba se reconstruye después (D-45).
    if !ruta.join(".osr").exists() {
        nuevo::carpeta(&ruta.join(".osr")).map_err(escritura)?;
    }
    let abandonado = match bloqueo::tomar(ruta, &momento.marca) {
        Ok(abandonado) => abandonado,
        Err(bloqueo::Error::Abierto(pid)) => return Err(Error::Abierto(pid)),
        Err(bloqueo::Error::Escritura(error)) => return Err(escritura(error)),
    };
    anotar_apertura(ruta, &nombre, abandonado, momento).map_err(|error| {
        let _ = fs::remove_file(ruta.join(".osr/bloqueo"));
        escritura(error)
    })?;
    Ok(nombre)
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

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
        assert_eq!(abrir(&raiz, &momento()).unwrap(), "estudio");
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
    fn repara_el_registro_y_las_carpetas_antes_de_anotar_lo_demas() {
        let (_tmp, raiz) = proyecto();
        let viejo = "2026-10-11T05:00:00-06:00";
        fs::write(raiz.join(".osr/bloqueo"), plantillas::bloqueo(1, viejo)).unwrap();
        fs::remove_dir(raiz.join("identificable/consentimientos")).unwrap();
        let mut registro = fs::OpenOptions::new()
            .append(true)
            .open(raiz.join("bitacora/registro.jsonl"))
            .unwrap();
        registro.write_all(b"{\"fecha\":").unwrap();
        abrir(&raiz, &momento()).unwrap();
        let marca = "2026-10-12T05:00:00-06:00";
        let esperado = [
            plantillas::evento_creado("2026-10-12T05:00:00-06:00"),
            plantillas::evento_linea_incompleta(marca, 9),
            plantillas::evento_carpeta_recreada(marca, "identificable/consentimientos/"),
            plantillas::evento_bloqueo_abandonado(marca, Some(1), Some(viejo)),
            plantillas::evento_verificacion(marca),
        ]
        .concat();
        let registro = fs::read_to_string(raiz.join("bitacora/registro.jsonl")).unwrap();
        assert_eq!(registro, esperado);
        assert!(raiz.join("identificable/consentimientos").is_dir());
    }

    #[test]
    fn un_readme_editado_se_anota_despues_de_la_verificacion() {
        let (_tmp, raiz) = proyecto();
        fs::write(raiz.join("README.md"), "a mano\n").unwrap();
        abrir(&raiz, &momento()).unwrap();
        let marca = "2026-10-12T05:00:00-06:00";
        let esperado = [
            plantillas::evento_verificacion(marca),
            plantillas::evento_readme_guardado(marca, "README.modificado-20261012T050000.md"),
        ]
        .concat();
        let registro = fs::read_to_string(raiz.join("bitacora/registro.jsonl")).unwrap();
        assert!(registro.ends_with(&esperado), "{registro}");
        let aparte = raiz.join("README.modificado-20261012T050000.md");
        assert_eq!(fs::read_to_string(aparte).unwrap(), "a mano\n");
        assert_eq!(
            fs::read_to_string(raiz.join("README.md")).unwrap(),
            plantillas::readme("estudio", "2026-10-12")
        );
    }

    #[test]
    fn un_proyecto_toml_invalido_no_abre_ni_escribe_nada() {
        let (_tmp, raiz) = proyecto();
        fs::remove_file(raiz.join("proyecto.toml")).unwrap();
        let error = abrir(&raiz, &momento()).unwrap_err();
        assert_eq!(error.codigo(), 3);
        assert!(
            error.to_string().ends_with("(falta proyecto.toml)."),
            "{error}"
        );
        assert!(!raiz.join(".osr/bloqueo").exists());
        let registro = fs::read_to_string(raiz.join("bitacora/registro.jsonl")).unwrap();
        assert_eq!(registro.lines().count(), 1);
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
    fn sin_carpeta_osr_la_recrea_y_reconstruye_el_indice() {
        let (_tmp, raiz) = proyecto();
        fs::remove_dir_all(raiz.join(".osr")).unwrap();
        abrir(&raiz, &momento()).unwrap();
        let modo = fs::metadata(raiz.join(".osr"))
            .unwrap()
            .permissions()
            .mode();
        assert_eq!(modo & 0o777, 0o700);
        assert!(raiz.join(".osr/bloqueo").is_file());
        assert_eq!(indice::revisar(&raiz), None);
        let registro = fs::read_to_string(raiz.join("bitacora/registro.jsonl")).unwrap();
        let marca = "2026-10-12T05:00:00-06:00";
        let esperado = [
            plantillas::evento_indice_reconstruido(marca, "ausente"),
            plantillas::evento_verificacion(marca),
        ]
        .concat();
        assert!(registro.ends_with(&esperado), "{registro}");
        assert_eq!(registro.lines().count(), 3);
    }

    #[test]
    fn un_indice_danado_se_anota_antes_del_bloqueo_abandonado() {
        let (_tmp, raiz) = proyecto();
        let viejo = "2026-10-11T05:00:00-06:00";
        fs::write(raiz.join(".osr/bloqueo"), plantillas::bloqueo(1, viejo)).unwrap();
        fs::write(raiz.join(indice::RUTA), [0xffu8; 1024]).unwrap();
        abrir(&raiz, &momento()).unwrap();
        let registro = fs::read_to_string(raiz.join("bitacora/registro.jsonl")).unwrap();
        let marca = "2026-10-12T05:00:00-06:00";
        let esperado = [
            plantillas::evento_indice_reconstruido(marca, "danado"),
            plantillas::evento_bloqueo_abandonado(marca, Some(1), Some(viejo)),
            plantillas::evento_verificacion(marca),
        ]
        .concat();
        assert!(registro.ends_with(&esperado), "{registro}");
    }

    #[test]
    fn si_no_puede_crear_osr_da_codigo_6_sin_bloqueo() {
        let (_tmp, raiz) = proyecto();
        fs::remove_dir_all(raiz.join(".osr")).unwrap();
        fs::set_permissions(&raiz, fs::Permissions::from_mode(0o500)).unwrap();
        let error = abrir(&raiz, &momento()).unwrap_err();
        fs::set_permissions(&raiz, fs::Permissions::from_mode(0o700)).unwrap();
        assert_eq!(error.codigo(), 6);
        assert!(!raiz.join(".osr").exists());
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
