//! `proyecto.toml`, la identidad del proyecto (PRY-03).
//!
//! Al abrir se comprueban las claves obligatorias y la versión del formato.
//! Las claves opcionales, con sus valores por defecto y sus avisos, llegan
//! con los requisitos que las usan (D-29).

use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Por qué `proyecto.toml` no deja abrir el proyecto.
#[derive(Debug)]
pub enum Error {
    /// No existe en la carpeta.
    Falta(PathBuf),
    /// No se pudo leer por otro motivo del sistema.
    Lectura(io::Error),
    /// No es TOML válido, en esta línea y por este motivo.
    Toml { linea: usize, motivo: &'static str },
    /// `formato` no es de Open Social Research o falta esta clave obligatoria.
    Clave(&'static str),
    /// La versión del formato no es la 1.
    Version(i64),
}

impl Error {
    pub fn codigo(&self) -> i32 {
        match self {
            Error::Lectura(_) => 6,
            _ => 3,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Falta(ruta) => write!(
                f,
                "{} no es un proyecto de Open Social Research (falta proyecto.toml).",
                ruta.display()
            ),
            Error::Lectura(error) => {
                let motivo = match error.kind() {
                    io::ErrorKind::PermissionDenied => "sin permiso de lectura",
                    _ => "error de lectura del sistema",
                };
                write!(f, "no se pudo leer proyecto.toml ({motivo}).")
            }
            Error::Toml { linea, motivo } => {
                write!(
                    f,
                    "proyecto.toml tiene un error en la línea {linea}: {motivo}."
                )
            }
            Error::Clave(clave) => write!(
                f,
                "proyecto.toml no es de Open Social Research o le falta {clave}."
            ),
            Error::Version(version) => write!(
                f,
                "este proyecto usa la versión {version} del formato y esta versión de osr solo abre la versión 1."
            ),
        }
    }
}

/// La línea, contada desde 1, del byte `posicion` de `texto`.
fn linea_de(texto: &[u8], posicion: usize) -> usize {
    texto[..posicion].iter().filter(|&&b| b == b'\n').count() + 1
}

/// Lee el texto como TOML, o el error con su línea.
fn leer(bytes: &[u8]) -> Result<toml::Table, Error> {
    let texto = std::str::from_utf8(bytes).map_err(|error| Error::Toml {
        linea: linea_de(bytes, error.valid_up_to()),
        motivo: "no está escrito en UTF-8",
    })?;
    texto.parse().map_err(|error: toml::de::Error| Error::Toml {
        linea: linea_de(bytes, error.span().map_or(0, |span| span.start)),
        motivo: "no es TOML válido",
    })
}

/// Una marca de tiempo completa, con fecha, hora y desplazamiento.
fn es_marca(valor: &toml::Value) -> bool {
    valor
        .as_datetime()
        .is_some_and(|marca| marca.date.is_some() && marca.time.is_some() && marca.offset.is_some())
}

/// Comprueba las claves obligatorias en el orden de la tabla de PRY-03, con
/// la versión antes que las demás, porque otra versión puede tener otras.
fn comprobar(tabla: &toml::Table) -> Result<(), Error> {
    if tabla.get("formato").and_then(toml::Value::as_str) != Some("osr-proyecto") {
        return Err(Error::Clave("formato"));
    }
    match tabla
        .get("version_formato")
        .and_then(toml::Value::as_integer)
    {
        Some(1) => {}
        Some(version) => return Err(Error::Version(version)),
        None => return Err(Error::Clave("version_formato")),
    }
    if !tabla.get("nombre").is_some_and(toml::Value::is_str) {
        return Err(Error::Clave("nombre"));
    }
    if !tabla.get("creado").is_some_and(es_marca) {
        return Err(Error::Clave("creado"));
    }
    Ok(())
}

/// Valida el `proyecto.toml` de `raiz` sin escribir nada.
pub fn validar(raiz: &Path) -> Result<(), Error> {
    let bytes = fs::read(raiz.join("proyecto.toml")).map_err(|error| match error.kind() {
        io::ErrorKind::NotFound => Error::Falta(raiz.to_path_buf()),
        _ => Error::Lectura(error),
    })?;
    comprobar(&leer(&bytes)?)
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use crate::plantillas;
    use std::os::unix::fs::PermissionsExt;

    const BUENO: &str = "formato = \"osr-proyecto\"\nversion_formato = 1\nnombre = \"e\"\n\
                         creado = 2026-10-12T05:00:00-06:00\n";

    fn validar_texto(texto: &str) -> Result<(), Error> {
        comprobar(&leer(texto.as_bytes())?)
    }

    fn mensaje(texto: &str) -> String {
        validar_texto(texto).unwrap_err().to_string()
    }

    #[test]
    fn el_proyecto_toml_que_escribe_osr_nuevo_es_valido() {
        let texto = plantillas::proyecto_toml("Estudio", "2026-10-12T05:00:00-06:00");
        validar_texto(&texto).unwrap();
        validar_texto(BUENO).unwrap();
    }

    #[test]
    fn el_error_de_toml_dice_su_linea() {
        let texto = BUENO.replace("nombre = \"e\"", "nombre = ");
        assert_eq!(
            mensaje(&texto),
            "proyecto.toml tiene un error en la línea 3: no es TOML válido."
        );
        assert_eq!(
            mensaje("a = 1\nb = 2\nc = 3\nd = [\n"),
            "proyecto.toml tiene un error en la línea 4: no es TOML válido."
        );
    }

    #[test]
    fn fuera_de_utf8_es_un_error_en_su_linea() {
        let mut bytes = b"a = 1\nb = \"".to_vec();
        bytes.extend([0xff, b'"', b'\n']);
        let error = leer(&bytes).unwrap_err();
        assert_eq!(
            error.to_string(),
            "proyecto.toml tiene un error en la línea 2: no está escrito en UTF-8."
        );
        assert_eq!(error.codigo(), 3);
    }

    #[test]
    fn la_linea_se_cuenta_desde_1() {
        assert_eq!(linea_de(b"abc", 0), 1);
        assert_eq!(linea_de(b"a\nb\nc", 4), 3);
        assert_eq!(linea_de(b"a\nb", 1), 1);
    }

    #[test]
    fn cada_clave_obligatoria_que_falta_se_nombra() {
        for clave in ["formato", "version_formato", "nombre", "creado"] {
            let sin_clave: String = BUENO
                .lines()
                .filter(|linea| !linea.starts_with(&format!("{clave} = ")))
                .map(|linea| format!("{linea}\n"))
                .collect();
            assert_eq!(
                mensaje(&sin_clave),
                format!("proyecto.toml no es de Open Social Research o le falta {clave}."),
            );
        }
    }

    #[test]
    fn una_clave_obligatoria_de_otro_tipo_cuenta_como_faltante() {
        let casos = [
            (
                "formato = \"osr-proyecto\"",
                "formato = \"otro-formato\"",
                "formato",
            ),
            (
                "version_formato = 1",
                "version_formato = \"1\"",
                "version_formato",
            ),
            ("nombre = \"e\"", "nombre = 3", "nombre"),
            (
                "creado = 2026-10-12T05:00:00-06:00",
                "creado = \"ayer\"",
                "creado",
            ),
            (
                "creado = 2026-10-12T05:00:00-06:00",
                "creado = 2026-10-12T05:00:00",
                "creado",
            ),
            (
                "creado = 2026-10-12T05:00:00-06:00",
                "creado = 2026-10-12",
                "creado",
            ),
            (
                "creado = 2026-10-12T05:00:00-06:00",
                "creado = 05:00:00",
                "creado",
            ),
        ];
        for (antes, despues, clave) in casos {
            assert!(
                matches!(validar_texto(&BUENO.replace(antes, despues)), Err(Error::Clave(c)) if c == clave),
                "{despues}"
            );
        }
    }

    #[test]
    fn otra_version_no_se_abre_y_se_mira_antes_que_las_demas_claves() {
        let texto = BUENO.replace("version_formato = 1", "version_formato = 2");
        assert_eq!(
            mensaje(&texto),
            "este proyecto usa la versión 2 del formato y esta versión de osr solo abre la versión 1."
        );
        let sin_nombre = texto.replace("nombre = \"e\"\n", "");
        assert!(matches!(validar_texto(&sin_nombre), Err(Error::Version(2))));
    }

    #[test]
    fn sin_proyecto_toml_no_es_un_proyecto() {
        let tmp = tempfile::tempdir().unwrap();
        let error = validar(tmp.path()).unwrap_err();
        assert_eq!(error.codigo(), 3);
        assert_eq!(
            error.to_string(),
            format!(
                "{} no es un proyecto de Open Social Research (falta proyecto.toml).",
                tmp.path().display()
            )
        );
    }

    #[test]
    fn valida_el_archivo_de_la_carpeta() {
        let tmp = tempfile::tempdir().unwrap();
        fs::write(tmp.path().join("proyecto.toml"), BUENO).unwrap();
        validar(tmp.path()).unwrap();
    }

    #[test]
    fn un_proyecto_toml_ilegible_da_codigo_6() {
        let tmp = tempfile::tempdir().unwrap();
        let ruta = tmp.path().join("proyecto.toml");
        fs::write(&ruta, BUENO).unwrap();
        fs::set_permissions(&ruta, fs::Permissions::from_mode(0o000)).unwrap();
        let error = validar(tmp.path()).unwrap_err();
        assert_eq!(error.codigo(), 6);
        assert_eq!(
            error.to_string(),
            "no se pudo leer proyecto.toml (sin permiso de lectura)."
        );
        let carpeta = tmp.path().join("carpeta");
        fs::create_dir_all(carpeta.join("proyecto.toml")).unwrap();
        assert_eq!(
            validar(&carpeta).unwrap_err().to_string(),
            "no se pudo leer proyecto.toml (error de lectura del sistema)."
        );
    }
}
