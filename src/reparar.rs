//! Las reparaciones al abrir que no necesitan el índice: la última línea
//! incompleta del registro de eventos (PRY-08) y las carpetas de «Al crear»
//! que faltan (PRY-02).
//!
//! Corren con el bloqueo ya tomado, así un segundo `osr` no escribe nada.

use std::fs;
use std::io;
use std::path::Path;

/// Las carpetas de la columna «Al crear» de PRY-02, cada una después de la
/// que la contiene. `.osr/` queda fuera, porque sin ella no hay bloqueo y la
/// reconstruye el índice (PRY-10, D-42).
pub const CARPETAS: [&str; 5] = [
    "bitacora/",
    "participantes/",
    "identificable/",
    "identificable/consentimientos/",
    "identificable/entrevistas/",
];

/// Si el registro de eventos no termina en salto de línea, lo recorta hasta
/// después del último salto, lo lleva a disco y devuelve cuántos bytes
/// descartó. Es el único caso en que `osr` recorta el registro (PRY-08).
pub fn recortar_registro(raiz: &Path) -> io::Result<Option<u64>> {
    let ruta = raiz.join("bitacora/registro.jsonl");
    let bytes = match fs::read(&ruta) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error),
    };
    if bytes.last().is_none_or(|&ultimo| ultimo == b'\n') {
        return Ok(None);
    }
    let conservar = bytes
        .iter()
        .rposition(|&byte| byte == b'\n')
        .map_or(0, |salto| salto + 1);
    let registro = fs::OpenOptions::new().write(true).open(&ruta)?;
    registro.set_len(conservar as u64)?;
    registro.sync_data()?;
    Ok(Some((bytes.len() - conservar) as u64))
}

/// Vuelve a crear vacías, con 0700, las carpetas de [`CARPETAS`] que faltan,
/// y devuelve sus rutas en ese orden.
pub fn recrear_carpetas(raiz: &Path) -> io::Result<Vec<&'static str>> {
    let mut recreadas = Vec::new();
    for carpeta in CARPETAS {
        let ruta = raiz.join(carpeta);
        if !ruta.exists() {
            crate::nuevo::carpeta(&ruta)?;
            recreadas.push(carpeta);
        }
    }
    Ok(recreadas)
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    fn con_registro(contenido: &[u8]) -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir(tmp.path().join("bitacora")).unwrap();
        fs::write(tmp.path().join("bitacora/registro.jsonl"), contenido).unwrap();
        tmp
    }

    fn registro(tmp: &tempfile::TempDir) -> Vec<u8> {
        fs::read(tmp.path().join("bitacora/registro.jsonl")).unwrap()
    }

    #[test]
    fn un_registro_completo_no_se_toca() {
        let tmp = con_registro(b"{\"a\":1}\n{\"b\":2}\n");
        assert_eq!(recortar_registro(tmp.path()).unwrap(), None);
        assert_eq!(registro(&tmp), b"{\"a\":1}\n{\"b\":2}\n");
    }

    #[test]
    fn un_registro_vacio_no_se_toca() {
        let tmp = con_registro(b"");
        assert_eq!(recortar_registro(tmp.path()).unwrap(), None);
        assert_eq!(registro(&tmp), b"");
    }

    #[test]
    fn la_ultima_linea_sin_salto_se_descarta() {
        let tmp = con_registro(b"{\"a\":1}\n{\"fecha\":");
        assert_eq!(recortar_registro(tmp.path()).unwrap(), Some(9));
        assert_eq!(registro(&tmp), b"{\"a\":1}\n");
    }

    #[test]
    fn sin_ningun_salto_queda_vacio() {
        let tmp = con_registro(b"{\"fecha\":");
        assert_eq!(recortar_registro(tmp.path()).unwrap(), Some(9));
        assert_eq!(registro(&tmp), b"");
    }

    #[test]
    fn sin_registro_no_hay_nada_que_recortar() {
        let tmp = tempfile::tempdir().unwrap();
        assert_eq!(recortar_registro(tmp.path()).unwrap(), None);
        assert!(!tmp.path().join("bitacora/registro.jsonl").exists());
    }

    #[test]
    fn un_registro_que_no_se_puede_leer_es_un_error() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("bitacora/registro.jsonl")).unwrap();
        assert!(recortar_registro(tmp.path()).is_err());
    }

    #[test]
    fn recrea_solo_las_que_faltan_con_0700() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("identificable/entrevistas")).unwrap();
        fs::create_dir(tmp.path().join("bitacora")).unwrap();
        let recreadas = recrear_carpetas(tmp.path()).unwrap();
        assert_eq!(
            recreadas,
            ["participantes/", "identificable/consentimientos/"]
        );
        for carpeta in CARPETAS {
            assert!(tmp.path().join(carpeta).is_dir(), "{carpeta}");
        }
        let modo = |ruta: &str| {
            fs::metadata(tmp.path().join(ruta))
                .unwrap()
                .permissions()
                .mode()
                & 0o777
        };
        assert_eq!(modo("participantes"), 0o700);
        assert_eq!(modo("identificable/consentimientos"), 0o700);
    }

    #[test]
    fn sin_carpetas_recrea_todas_en_orden() {
        let tmp = tempfile::tempdir().unwrap();
        assert_eq!(recrear_carpetas(tmp.path()).unwrap(), CARPETAS);
        assert!(recrear_carpetas(tmp.path()).unwrap().is_empty());
    }
}
