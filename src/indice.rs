//! El índice reconstruible `.osr/indice.sqlite` (PRY-10).
//!
//! Es una copia derivada de los archivos de texto: se puede borrar sin perder
//! nada. Al abrir, si falta, no es una base SQLite válida o su versión de
//! esquema no es la que espera `osr`, se reconstruye desde los archivos.
//!
//! Hoy guarda la tabla `archivos`, con la ruta, el tamaño, la fecha de
//! modificación y la huella de los archivos de texto de «Al crear» que la
//! app reescribe enteros. Los de PAR y TRA entran con sus requisitos (D-45).

use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::time::UNIX_EPOCH;

use rusqlite::{Connection, OpenFlags};
use sha2::{Digest, Sha256};

/// La versión de esquema que espera esta versión de `osr`, en `user_version`.
pub const VERSION: i64 = 2;

/// Dónde vive el índice dentro del proyecto.
pub const RUTA: &str = ".osr/indice.sqlite";

/// Los archivos que la reconstrucción anota en la tabla `archivos`. El
/// registro de eventos queda fuera porque solo crece al final y lo cuida
/// PRY-08. `README.md` también, porque al reconstruir no se sabe si el del
/// disco es el que escribió la app; su fila la pone la regeneración (D-46).
const ARCHIVOS: [&str; 6] = [
    "proyecto.toml",
    "sensibilidad.csv",
    "bitacora/bitacora.md",
    "participantes/participantes.csv",
    "participantes/consentimientos.csv",
    "identificable/correspondencia.csv",
];

/// Por qué se reconstruyó el índice, con los nombres de PRY-10.
#[derive(Debug, PartialEq, Eq)]
pub enum Motivo {
    Ausente,
    Danado,
    Version,
}

impl Motivo {
    pub fn nombre(&self) -> &'static str {
        match self {
            Motivo::Ausente => "ausente",
            Motivo::Danado => "danado",
            Motivo::Version => "version",
        }
    }
}

/// La huella SHA-256 de unos bytes, en 64 caracteres hexadecimales en minúscula.
pub fn huella(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// Nanosegundos desde 1970 de la fecha de modificación.
fn modificado(datos: &fs::Metadata) -> io::Result<i64> {
    let desde = datos.modified()?.duration_since(UNIX_EPOCH);
    Ok(desde.map_or(0, |duracion| duracion.as_nanos() as i64))
}

/// Si el índice de `raiz` se puede usar; si no, por qué.
pub fn revisar(raiz: &Path) -> Option<Motivo> {
    let ruta = raiz.join(RUTA);
    if !ruta.exists() {
        return Some(Motivo::Ausente);
    }
    let Ok(conexion) = Connection::open(&ruta) else {
        return Some(Motivo::Danado);
    };
    let version = conexion.pragma_query_value(None, "user_version", |fila| fila.get::<_, i64>(0));
    match version {
        Err(_) => Some(Motivo::Danado),
        Ok(version) if version != VERSION => Some(Motivo::Version),
        Ok(_) => {
            let filas = conexion.query_row("SELECT count(*) FROM archivos", [], |fila| {
                fila.get::<_, i64>(0)
            });
            filas.is_err().then_some(Motivo::Danado)
        }
    }
}

/// Construye el índice de `raiz` desde los archivos, reemplazando el que haya.
///
/// Todo va en una transacción, también `user_version`, así un corte a mitad
/// deja una base con versión 0, que la próxima apertura reconstruye.
pub fn construir(raiz: &Path) -> io::Result<()> {
    let ruta = raiz.join(RUTA);
    for sobrante in ["", "-journal", "-wal", "-shm"] {
        let archivo = format!("{}{sobrante}", ruta.display());
        match fs::remove_file(&archivo) {
            Err(error) if error.kind() != io::ErrorKind::NotFound => return Err(error),
            _ => {}
        }
    }
    let mut conexion = Connection::open(&ruta).map_err(io::Error::other)?;
    let transaccion = conexion.transaction().map_err(io::Error::other)?;
    transaccion
        .execute_batch(
            "CREATE TABLE archivos (
                ruta TEXT PRIMARY KEY,
                bytes INTEGER NOT NULL,
                modificado_ns INTEGER NOT NULL,
                sha256 TEXT NOT NULL
            ) STRICT;",
        )
        .map_err(io::Error::other)?;
    for relativa in ARCHIVOS {
        if raiz.join(relativa).is_file() {
            anotar_en(&transaccion, raiz, relativa)?;
        }
    }
    transaccion
        .pragma_update(None, "user_version", VERSION)
        .map_err(io::Error::other)?;
    transaccion.commit().map_err(io::Error::other)?;
    drop(conexion);
    fs::set_permissions(&ruta, fs::Permissions::from_mode(0o600))
}

/// Anota o reemplaza la fila de `relativa` con lo que hay hoy en el disco.
fn anotar_en(conexion: &Connection, raiz: &Path, relativa: &str) -> io::Result<()> {
    let archivo = raiz.join(relativa);
    let contenido = fs::read(&archivo)?;
    let datos = fs::metadata(&archivo)?;
    conexion
        .execute(
            "INSERT OR REPLACE INTO archivos (ruta, bytes, modificado_ns, sha256) \
             VALUES (?1, ?2, ?3, ?4)",
            (
                relativa,
                contenido.len() as i64,
                modificado(&datos)?,
                huella(&contenido),
            ),
        )
        .map_err(io::Error::other)?;
    Ok(())
}

/// Anota en el índice el archivo `relativa` tal como quedó en el disco,
/// después de que la app lo escribió.
pub fn anotar_archivo(raiz: &Path, relativa: &str) -> io::Result<()> {
    let conexion = Connection::open(raiz.join(RUTA)).map_err(io::Error::other)?;
    anotar_en(&conexion, raiz, relativa)
}

/// La huella que el índice guarda para `relativa`, si la tiene.
pub fn huella_de(raiz: &Path, relativa: &str) -> Option<String> {
    let ruta = raiz.join(RUTA);
    let conexion = Connection::open_with_flags(&ruta, OpenFlags::SQLITE_OPEN_READ_ONLY).ok()?;
    conexion
        .query_row(
            "SELECT sha256 FROM archivos WHERE ruta = ?1",
            [relativa],
            |fila| fila.get(0),
        )
        .ok()
}

/// Revisa el índice y, si no se puede usar, lo reconstruye y devuelve el
/// motivo. `.osr/` ya tiene que existir, porque ahí vive el bloqueo.
pub fn asegurar(raiz: &Path) -> io::Result<Option<Motivo>> {
    let motivo = revisar(raiz);
    if motivo.is_some() {
        construir(raiz)?;
    }
    Ok(motivo)
}

#[cfg(test)]
mod pruebas {
    use super::*;

    /// Una raíz con `.osr/` y algunos de los archivos de [`ARCHIVOS`].
    fn raiz() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir(tmp.path().join(".osr")).unwrap();
        fs::create_dir(tmp.path().join("bitacora")).unwrap();
        fs::write(tmp.path().join("proyecto.toml"), "nombre = \"x\"\n").unwrap();
        fs::write(tmp.path().join("bitacora/bitacora.md"), "# Bitácora\n").unwrap();
        tmp
    }

    fn filas(raiz: &Path) -> Vec<(String, i64, i64, String)> {
        let conexion = Connection::open(raiz.join(RUTA)).unwrap();
        let mut consulta = conexion
            .prepare("SELECT ruta, bytes, modificado_ns, sha256 FROM archivos ORDER BY ruta")
            .unwrap();
        consulta
            .query_map([], |f| Ok((f.get(0)?, f.get(1)?, f.get(2)?, f.get(3)?)))
            .unwrap()
            .map(Result::unwrap)
            .collect()
    }

    #[test]
    fn la_huella_es_sha_256_en_hexadecimal_minuscula() {
        assert_eq!(
            huella(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            huella(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn los_motivos_tienen_los_nombres_de_pry_10() {
        assert_eq!(Motivo::Ausente.nombre(), "ausente");
        assert_eq!(Motivo::Danado.nombre(), "danado");
        assert_eq!(Motivo::Version.nombre(), "version");
    }

    #[test]
    fn sin_indice_el_motivo_es_ausente_y_no_lo_crea() {
        let tmp = raiz();
        assert_eq!(revisar(tmp.path()), Some(Motivo::Ausente));
        assert!(!tmp.path().join(RUTA).exists());
    }

    #[test]
    fn bytes_al_azar_son_un_indice_danado() {
        let tmp = raiz();
        let basura: Vec<u8> = (0..1024u32).map(|i| (i * 7 + 13) as u8).collect();
        fs::write(tmp.path().join(RUTA), basura).unwrap();
        assert_eq!(revisar(tmp.path()), Some(Motivo::Danado));
    }

    #[test]
    fn una_base_sin_la_tabla_de_archivos_esta_danada() {
        let tmp = raiz();
        let conexion = Connection::open(tmp.path().join(RUTA)).unwrap();
        conexion
            .pragma_update(None, "user_version", VERSION)
            .unwrap();
        drop(conexion);
        assert_eq!(revisar(tmp.path()), Some(Motivo::Danado));
    }

    #[test]
    fn otra_version_de_esquema_es_motivo_version() {
        let tmp = raiz();
        let conexion = Connection::open(tmp.path().join(RUTA)).unwrap();
        conexion.pragma_update(None, "user_version", 1).unwrap();
        drop(conexion);
        assert_eq!(revisar(tmp.path()), Some(Motivo::Version));
    }

    #[test]
    fn construir_guarda_los_archivos_presentes_con_su_huella() {
        let tmp = raiz();
        construir(tmp.path()).unwrap();
        assert_eq!(revisar(tmp.path()), None);
        let filas = filas(tmp.path());
        let rutas: Vec<&str> = filas.iter().map(|f| f.0.as_str()).collect();
        assert_eq!(rutas, ["bitacora/bitacora.md", "proyecto.toml"]);
        let (_, bytes, modificado_ns, sha256) = &filas[1];
        assert_eq!(*bytes, 13);
        assert_eq!(sha256, &huella(b"nombre = \"x\"\n"));
        let datos = fs::metadata(tmp.path().join("proyecto.toml")).unwrap();
        assert_eq!(*modificado_ns, modificado(&datos).unwrap());
        assert!(*modificado_ns > 1_700_000_000_000_000_000);
        let modo = fs::metadata(tmp.path().join(RUTA))
            .unwrap()
            .permissions()
            .mode();
        assert_eq!(modo & 0o777, 0o600);
    }

    #[test]
    fn construir_reemplaza_un_indice_danado_y_sus_sobrantes() {
        let tmp = raiz();
        fs::write(tmp.path().join(RUTA), [0xffu8; 1024]).unwrap();
        fs::write(tmp.path().join(".osr/indice.sqlite-journal"), [0u8; 10]).unwrap();
        construir(tmp.path()).unwrap();
        assert_eq!(revisar(tmp.path()), None);
        assert!(!tmp.path().join(".osr/indice.sqlite-journal").exists());
        assert_eq!(filas(tmp.path()).len(), 2);
    }

    #[test]
    fn construir_falla_si_no_puede_borrar_el_anterior() {
        let tmp = raiz();
        fs::create_dir_all(tmp.path().join(".osr/indice.sqlite-shm/x")).unwrap();
        assert!(construir(tmp.path()).is_err());
    }

    #[test]
    fn construir_no_anota_el_readme() {
        let tmp = raiz();
        fs::write(tmp.path().join("README.md"), "x").unwrap();
        construir(tmp.path()).unwrap();
        assert_eq!(huella_de(tmp.path(), "README.md"), None);
        assert_eq!(
            huella_de(tmp.path(), "proyecto.toml"),
            Some(huella(b"nombre = \"x\"\n"))
        );
    }

    #[test]
    fn anotar_archivo_agrega_y_reemplaza_su_fila() {
        let tmp = raiz();
        construir(tmp.path()).unwrap();
        fs::write(tmp.path().join("README.md"), "uno").unwrap();
        anotar_archivo(tmp.path(), "README.md").unwrap();
        assert_eq!(huella_de(tmp.path(), "README.md"), Some(huella(b"uno")));
        fs::write(tmp.path().join("README.md"), "dos!").unwrap();
        anotar_archivo(tmp.path(), "README.md").unwrap();
        assert_eq!(huella_de(tmp.path(), "README.md"), Some(huella(b"dos!")));
        let fila = filas(tmp.path())
            .into_iter()
            .find(|f| f.0 == "README.md")
            .unwrap();
        assert_eq!(fila.1, 4);
        assert_eq!(filas(tmp.path()).len(), 3);
    }

    #[test]
    fn anotar_un_archivo_que_no_existe_es_un_error() {
        let tmp = raiz();
        construir(tmp.path()).unwrap();
        assert!(anotar_archivo(tmp.path(), "README.md").is_err());
    }

    #[test]
    fn sin_indice_no_hay_huella_y_no_lo_crea() {
        let tmp = raiz();
        assert_eq!(huella_de(tmp.path(), "proyecto.toml"), None);
        assert!(!tmp.path().join(RUTA).exists());
    }

    #[test]
    fn asegurar_reconstruye_solo_si_hace_falta() {
        let tmp = raiz();
        assert_eq!(asegurar(tmp.path()).unwrap(), Some(Motivo::Ausente));
        assert_eq!(asegurar(tmp.path()).unwrap(), None);
        fs::write(tmp.path().join(RUTA), [0xffu8; 1024]).unwrap();
        assert_eq!(asegurar(tmp.path()).unwrap(), Some(Motivo::Danado));
        assert_eq!(revisar(tmp.path()), None);
    }
}
