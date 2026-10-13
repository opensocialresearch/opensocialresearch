//! Escritura atómica de un archivo gestionado (PRY-12).
//!
//! Se escribe un temporal `.<nombre>.osr-tmp-<8 caracteres>` en la misma
//! carpeta, se lleva a disco, se renombra sobre el destino y se lleva a disco
//! la carpeta. Ante cualquier corte, el destino queda con su contenido
//! anterior completo o con el nuevo completo.

use std::fs;
use std::io::{self, Read, Write};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::Path;

const LETRAS: &[u8; 36] = b"abcdefghijklmnopqrstuvwxyz0123456789";

/// El nombre del temporal de `nombre` con el sufijo dado.
fn temporal(nombre: &str, sufijo: &str) -> String {
    format!(".{nombre}.osr-tmp-{sufijo}")
}

/// Ocho caracteres al azar de `/dev/urandom`, en minúsculas y cifras.
fn sufijo() -> io::Result<String> {
    let mut bytes = [0u8; 8];
    fs::File::open("/dev/urandom")?.read_exact(&mut bytes)?;
    Ok(bytes
        .iter()
        .map(|byte| char::from(LETRAS[usize::from(*byte) % LETRAS.len()]))
        .collect())
}

/// Escribe `contenido` en `ruta` de forma atómica, con permisos `modo` sin
/// importar la umask (SRV-13). Si algo falla, borra el temporal.
pub fn atomica(ruta: &Path, contenido: &[u8], modo: u32) -> io::Result<()> {
    let carpeta = ruta.parent().unwrap_or(Path::new("."));
    let nombre = ruta
        .file_name()
        .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))?;
    let provisional = carpeta.join(temporal(&nombre.to_string_lossy(), &sufijo()?));
    let resultado = (|| {
        let mut archivo = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(modo)
            .open(&provisional)?;
        archivo.set_permissions(fs::Permissions::from_mode(modo))?;
        archivo.write_all(contenido)?;
        archivo.sync_all()?;
        fs::rename(&provisional, ruta)?;
        fs::File::open(carpeta)?.sync_all()
    })();
    if resultado.is_err() {
        let _ = fs::remove_file(&provisional);
    }
    resultado
}

#[cfg(test)]
mod pruebas {
    use super::*;

    fn nombres(carpeta: &Path) -> Vec<String> {
        let mut nombres: Vec<String> = fs::read_dir(carpeta)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        nombres.sort();
        nombres
    }

    #[test]
    fn el_temporal_lleva_punto_nombre_y_sufijo() {
        assert_eq!(
            temporal("README.md", "ab12cd34"),
            ".README.md.osr-tmp-ab12cd34"
        );
    }

    #[test]
    fn el_sufijo_tiene_ocho_minusculas_o_cifras_y_cambia() {
        let uno = sufijo().unwrap();
        assert_eq!(uno.len(), 8);
        assert!(uno.bytes().all(|b| LETRAS.contains(&b)), "{uno}");
        let distintos: std::collections::BTreeSet<String> =
            (0..20).map(|_| sufijo().unwrap()).collect();
        assert!(distintos.len() > 1);
        // Con 160 caracteres al azar sale alguno de la segunda mitad del alfabeto.
        let todos: String = distintos.into_iter().collect();
        assert!(
            todos.bytes().any(|b| b > b'r' || b.is_ascii_digit()),
            "{todos}"
        );
    }

    #[test]
    fn reemplaza_el_contenido_con_los_permisos_y_sin_temporales() {
        let tmp = tempfile::tempdir().unwrap();
        let ruta = tmp.path().join("README.md");
        fs::write(&ruta, "viejo").unwrap();
        fs::set_permissions(&ruta, fs::Permissions::from_mode(0o644)).unwrap();
        atomica(&ruta, b"nuevo\n", 0o600).unwrap();
        assert_eq!(fs::read(&ruta).unwrap(), b"nuevo\n");
        let modo = fs::metadata(&ruta).unwrap().permissions().mode();
        assert_eq!(modo & 0o777, 0o600);
        assert_eq!(nombres(tmp.path()), ["README.md"]);
    }

    #[test]
    fn crea_el_archivo_si_no_existe() {
        let tmp = tempfile::tempdir().unwrap();
        let ruta = tmp.path().join("nuevo.md");
        atomica(&ruta, b"x", 0o400).unwrap();
        assert_eq!(fs::read(&ruta).unwrap(), b"x");
        let modo = fs::metadata(&ruta).unwrap().permissions().mode();
        assert_eq!(modo & 0o777, 0o400);
    }

    #[test]
    fn si_no_puede_renombrar_borra_el_temporal_y_deja_el_destino() {
        let tmp = tempfile::tempdir().unwrap();
        let ruta = tmp.path().join("ocupado");
        fs::create_dir_all(ruta.join("adentro")).unwrap();
        assert!(atomica(&ruta, b"x", 0o600).is_err());
        assert_eq!(nombres(tmp.path()), ["ocupado"]);
        assert!(ruta.join("adentro").is_dir());
    }

    #[test]
    fn una_ruta_sin_nombre_no_se_escribe() {
        assert_eq!(
            atomica(Path::new("/"), b"x", 0o600).unwrap_err().kind(),
            io::ErrorKind::InvalidInput
        );
    }
}
