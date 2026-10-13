//! La carpeta de la aplicación, donde viven las investigaciones que se crean
//! desde la pantalla (PRY-24).
//!
//! Es `Open Social Research` en la carpeta personal. Cada investigación va en
//! una subcarpeta con su nombre simplificado, así la persona solo escribe el
//! nombre y nunca una ruta (D-53).

use std::ffi::OsStr;
use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use unicode_normalization::UnicodeNormalization;
use unicode_normalization::char::is_combining_mark;

/// El nombre de la carpeta de la aplicación dentro de la carpeta personal.
pub const NOMBRE: &str = "Open Social Research";

/// Lo más largo que puede ser el nombre simplificado.
const LARGO_MAXIMO: usize = 60;

/// La subcarpeta de un nombre que no deja ninguna letra ni cifra.
const SIN_LETRAS: &str = "investigacion";

/// La carpeta de la aplicación para la carpeta personal `casa`, si se sabe.
pub fn carpeta_desde(casa: Option<&OsStr>) -> Option<PathBuf> {
    casa.filter(|casa| !casa.is_empty())
        .map(|casa| Path::new(casa).join(NOMBRE))
}

/// Crea la carpeta de la aplicación si falta y la deja en 0700 (SRV-13).
pub fn crear(carpeta: &Path) -> io::Result<()> {
    fs::create_dir_all(carpeta)?;
    fs::set_permissions(carpeta, fs::Permissions::from_mode(0o700))
}

/// El nombre simplificado de PRY-24: sin marcas diacríticas, en minúsculas,
/// con guiones entre las letras y cifras, de 60 caracteres o menos.
pub fn simplificar(nombre: &str) -> String {
    let mut simple = String::new();
    let mut guion = false;
    for letra in nombre.nfd().filter(|letra| !is_combining_mark(*letra)) {
        let letra = letra.to_ascii_lowercase();
        if letra.is_ascii_lowercase() || letra.is_ascii_digit() {
            if guion && !simple.is_empty() {
                simple.push('-');
            }
            guion = false;
            simple.push(letra);
        } else {
            guion = true;
        }
    }
    if simple.len() > LARGO_MAXIMO {
        let corte = simple[..=LARGO_MAXIMO].rfind('-').unwrap_or(LARGO_MAXIMO);
        simple.truncate(corte);
    }
    let simple = simple.trim_end_matches('-');
    if simple.is_empty() {
        SIN_LETRAS.to_string()
    } else {
        simple.to_string()
    }
}

/// La primera subcarpeta libre para `base` en `carpeta`: `base`, `base-2`,
/// `base-3` y así.
pub fn libre(carpeta: &Path, base: &str) -> String {
    std::iter::once(base.to_string())
        .chain((2..).map(|numero| format!("{base}-{numero}")))
        .find(|nombre| carpeta.join(nombre).symlink_metadata().is_err())
        .expect("siempre hay un número libre")
}

/// Una investigación de la carpeta de la aplicación.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Investigacion {
    /// El nombre de su subcarpeta.
    pub carpeta: String,
    /// El nombre de su `proyecto.toml`, o el de la subcarpeta si no se puede
    /// leer.
    pub nombre: String,
}

/// Las investigaciones de `carpeta`, las subcarpetas directas con un
/// `proyecto.toml`, en orden alfabético del nombre sin distinguir
/// mayúsculas.
pub fn investigaciones(carpeta: &Path) -> Vec<Investigacion> {
    let Ok(entradas) = fs::read_dir(carpeta) else {
        return Vec::new();
    };
    let mut lista: Vec<Investigacion> = entradas
        .filter_map(Result::ok)
        .filter(|entrada| entrada.path().join("proyecto.toml").is_file())
        .filter_map(|entrada| {
            let subcarpeta = entrada.file_name().into_string().ok()?;
            let nombre =
                crate::proyecto::validar(&entrada.path()).unwrap_or_else(|_| subcarpeta.clone());
            Some(Investigacion {
                carpeta: subcarpeta,
                nombre,
            })
        })
        .collect();
    lista.sort_by_key(|investigacion| {
        (
            investigacion.nombre.to_lowercase(),
            investigacion.carpeta.clone(),
        )
    });
    lista
}

/// Si `carpeta` es un solo nombre de subcarpeta, sin `/`, distinto de `.` y
/// de `..` y no vacío (PRY-20).
pub fn es_subcarpeta(carpeta: &str) -> bool {
    !carpeta.is_empty() && carpeta != "." && carpeta != ".." && !carpeta.contains('/')
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn la_carpeta_esta_en_la_carpeta_personal() {
        assert_eq!(
            carpeta_desde(Some(OsStr::new("/home/ana"))),
            Some(PathBuf::from("/home/ana/Open Social Research"))
        );
        assert_eq!(carpeta_desde(Some(OsStr::new(""))), None);
        assert_eq!(carpeta_desde(None), None);
    }

    #[test]
    fn crear_deja_la_carpeta_en_0700_aunque_ya_exista() {
        let tmp = tempfile::tempdir().unwrap();
        let carpeta = tmp.path().join(NOMBRE);
        crear(&carpeta).unwrap();
        fs::set_permissions(&carpeta, fs::Permissions::from_mode(0o755)).unwrap();
        crear(&carpeta).unwrap();
        let modo = fs::metadata(&carpeta).unwrap().permissions().mode();
        assert_eq!(modo & 0o777, 0o700);
    }

    #[test]
    fn simplifica_como_la_tabla_de_pry_24() {
        let casos = [
            (
                "Caminar en el centro de San José",
                "caminar-en-el-centro-de-san-jose",
            ),
            ("Año 2026: ¿por qué?", "ano-2026-por-que"),
            ("!!!", "investigacion"),
            ("", "investigacion"),
            ("  Ñandú  ÜBER  ", "nandu-uber"),
            ("a--b__c", "a-b-c"),
            ("Estudio 10", "estudio-10"),
            ("日本", "investigacion"),
        ];
        for (nombre, esperado) in casos {
            assert_eq!(simplificar(nombre), esperado, "{nombre}");
        }
        assert_eq!(simplificar(&"a".repeat(120)), "a".repeat(60));
        let corte = format!("{}-b", "a".repeat(59));
        assert_eq!(simplificar(&corte), "a".repeat(59), "sin guion al final");
        let largo = "Percepción de inseguridad al caminar por el centro de San José de noche";
        assert_eq!(
            simplificar(largo),
            "percepcion-de-inseguridad-al-caminar-por-el-centro-de-san",
            "corta en el último guion, sin partir palabras"
        );
        assert_eq!(
            simplificar(&format!("{} b", "a".repeat(60))),
            "a".repeat(60)
        );
        assert_eq!(
            simplificar(&format!("{} b", "a".repeat(61))),
            "a".repeat(60)
        );
        let justo = format!("{}-{}", "a".repeat(30), "b".repeat(29));
        assert_eq!(simplificar(&justo), justo, "60 caracteres no se cortan");
    }

    #[test]
    fn libre_agrega_un_numero_si_ya_existe() {
        let tmp = tempfile::tempdir().unwrap();
        assert_eq!(libre(tmp.path(), "estudio"), "estudio");
        fs::create_dir(tmp.path().join("estudio")).unwrap();
        assert_eq!(libre(tmp.path(), "estudio"), "estudio-2");
        fs::write(tmp.path().join("estudio-2"), "x").unwrap();
        assert_eq!(libre(tmp.path(), "estudio"), "estudio-3");
    }

    fn con_proyecto(carpeta: &Path, subcarpeta: &str, toml: &str) {
        fs::create_dir_all(carpeta.join(subcarpeta)).unwrap();
        fs::write(carpeta.join(subcarpeta).join("proyecto.toml"), toml).unwrap();
    }

    fn toml(nombre: &str) -> String {
        format!(
            "formato = \"osr-proyecto\"\nversion_formato = 1\nnombre = \"{nombre}\"\n\
             creado = 2026-10-12T05:00:00-06:00\n"
        )
    }

    #[test]
    fn lista_solo_las_investigaciones_en_orden_alfabetico() {
        let tmp = tempfile::tempdir().unwrap();
        con_proyecto(tmp.path(), "zeta", &toml("alfa"));
        con_proyecto(tmp.path(), "beta", &toml("Beta"));
        con_proyecto(tmp.path(), "rota", "no es toml");
        fs::create_dir(tmp.path().join("sin-proyecto")).unwrap();
        fs::write(tmp.path().join("notas.txt"), "x").unwrap();
        let lista = investigaciones(tmp.path());
        let pares: Vec<(&str, &str)> = lista
            .iter()
            .map(|i| (i.nombre.as_str(), i.carpeta.as_str()))
            .collect();
        assert_eq!(
            pares,
            [("alfa", "zeta"), ("Beta", "beta"), ("rota", "rota")]
        );
    }

    #[test]
    fn sin_carpeta_no_hay_investigaciones() {
        let tmp = tempfile::tempdir().unwrap();
        assert!(investigaciones(&tmp.path().join("no-existe")).is_empty());
    }

    #[test]
    fn una_subcarpeta_es_un_solo_nombre() {
        assert!(es_subcarpeta("estudio"));
        assert!(es_subcarpeta("estudio-2"));
        for malo in ["", ".", "..", "../otra", "a/b", "/abs"] {
            assert!(!es_subcarpeta(malo), "{malo}");
        }
    }
}
