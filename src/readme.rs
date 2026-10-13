//! El README generado se reemplaza en cada apertura (PRY-16).
//!
//! Antes de reemplazarlo se mira si es el que escribió la app: su huella es
//! la del índice o, si el índice no la tiene porque se reconstruyó, sus bytes
//! son los de la plantilla con la fecha que dice el propio archivo. Si no lo
//! es, se guarda aparte como `README.modificado-<marca>.md` (D-46).

use std::fs;
use std::io;
use std::path::Path;

use crate::nuevo::Momento;
use crate::{escritura, indice, plantillas};

/// Lo que precede a la fecha de generación en la plantilla.
const GENERADO: &str = "se generó el ";

/// La fecha `AAAA-MM-DD` en que dice haberse generado el README.
fn fecha_de(texto: &str) -> Option<&str> {
    let inicio = texto.find(GENERADO)? + GENERADO.len();
    texto.get(inicio..inicio + 10)
}

/// Si `actual` es el README que la app escribió para el proyecto `nombre`.
fn es_propio(raiz: &Path, actual: &[u8], nombre: &str) -> bool {
    if indice::huella_de(raiz, "README.md").is_some_and(|huella| huella == indice::huella(actual)) {
        return true;
    }
    std::str::from_utf8(actual)
        .ok()
        .and_then(fecha_de)
        .is_some_and(|fecha| plantillas::readme(nombre, fecha).as_bytes() == actual)
}

/// Escribe el README de hoy y lo anota en el índice. Si el que había lo
/// cambió alguien a mano, antes lo guarda aparte y devuelve ese nombre.
pub fn regenerar(raiz: &Path, nombre: &str, momento: &Momento) -> io::Result<Option<String>> {
    let ruta = raiz.join("README.md");
    let guardado = match fs::read(&ruta) {
        Ok(actual) if !es_propio(raiz, &actual, nombre) => {
            let aparte = format!("README.modificado-{}.md", momento.compacta());
            escritura::atomica(&raiz.join(&aparte), &actual, 0o600)?;
            Some(aparte)
        }
        Ok(_) => None,
        Err(error) if error.kind() == io::ErrorKind::NotFound => None,
        Err(error) => return Err(error),
    };
    let nuevo = plantillas::readme(nombre, &momento.fecha);
    escritura::atomica(&ruta, nuevo.as_bytes(), 0o600)?;
    indice::anotar_archivo(raiz, "README.md")?;
    Ok(guardado)
}

#[cfg(test)]
mod pruebas {
    use super::*;

    fn momento() -> Momento {
        Momento {
            marca: "2026-10-12T05:06:07-06:00".into(),
            fecha: "2026-10-12".into(),
        }
    }

    /// Un proyecto recién creado con `osr nuevo` el 2026-10-11, sin bloqueo.
    fn proyecto() -> (tempfile::TempDir, std::path::PathBuf) {
        let tmp = tempfile::tempdir().unwrap();
        let raiz = tmp.path().join("estudio");
        let ayer = Momento {
            marca: "2026-10-11T05:00:00-06:00".into(),
            fecha: "2026-10-11".into(),
        };
        crate::nuevo::crear(&raiz, Some("Estudio"), &ayer).unwrap();
        (tmp, raiz)
    }

    fn leer(raiz: &Path, nombre: &str) -> String {
        fs::read_to_string(raiz.join(nombre)).unwrap()
    }

    /// Si no fuera así, cada copia del proyecto de prueba guardaría aparte
    /// su README al abrirse.
    #[test]
    fn el_readme_de_los_datos_de_prueba_es_el_de_la_plantilla() {
        let datos = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/datos/proyecto/README.md");
        assert_eq!(
            fs::read_to_string(datos).unwrap(),
            plantillas::readme("Proyecto de prueba inventado", "2026-10-05")
        );
    }

    #[test]
    fn la_fecha_sale_del_texto_de_la_plantilla() {
        let texto = plantillas::readme("x", "2026-10-05");
        assert_eq!(fecha_de(&texto), Some("2026-10-05"));
        assert_eq!(fecha_de("sin fecha"), None);
        assert_eq!(fecha_de("se generó el 2026"), None);
    }

    #[test]
    fn el_readme_de_la_app_se_reemplaza_sin_guardarlo_aparte() {
        let (_tmp, raiz) = proyecto();
        assert_eq!(regenerar(&raiz, "Estudio", &momento()).unwrap(), None);
        assert_eq!(
            leer(&raiz, "README.md"),
            plantillas::readme("Estudio", "2026-10-12")
        );
        let huella = indice::huella(leer(&raiz, "README.md").as_bytes());
        assert_eq!(indice::huella_de(&raiz, "README.md"), Some(huella));
        assert!(!raiz.join("README.modificado-20261012T050607.md").exists());
    }

    #[test]
    fn un_readme_editado_se_guarda_aparte_identico() {
        let (_tmp, raiz) = proyecto();
        let editado = format!("{}Nota escrita a mano\n", leer(&raiz, "README.md"));
        fs::write(raiz.join("README.md"), &editado).unwrap();
        let aparte = regenerar(&raiz, "Estudio", &momento()).unwrap();
        assert_eq!(
            aparte.as_deref(),
            Some("README.modificado-20261012T050607.md")
        );
        assert_eq!(leer(&raiz, "README.modificado-20261012T050607.md"), editado);
        assert_eq!(
            leer(&raiz, "README.md"),
            plantillas::readme("Estudio", "2026-10-12")
        );
    }

    #[test]
    fn la_huella_del_indice_reconoce_un_readme_propio_de_otra_plantilla() {
        let (_tmp, raiz) = proyecto();
        fs::write(raiz.join("README.md"), "otro texto de la app\n").unwrap();
        indice::anotar_archivo(&raiz, "README.md").unwrap();
        assert_eq!(regenerar(&raiz, "Estudio", &momento()).unwrap(), None);
    }

    #[test]
    fn sin_la_huella_en_el_indice_compara_con_la_plantilla_de_su_fecha() {
        let (_tmp, raiz) = proyecto();
        fs::remove_file(raiz.join(indice::RUTA)).unwrap();
        indice::construir(&raiz).unwrap();
        assert_eq!(indice::huella_de(&raiz, "README.md"), None);
        assert_eq!(regenerar(&raiz, "Estudio", &momento()).unwrap(), None);
        fs::write(raiz.join("README.md"), "a mano").unwrap();
        fs::remove_file(raiz.join(indice::RUTA)).unwrap();
        indice::construir(&raiz).unwrap();
        assert!(regenerar(&raiz, "Estudio", &momento()).unwrap().is_some());
    }

    #[test]
    fn la_plantilla_con_otro_nombre_no_es_propia() {
        let (_tmp, raiz) = proyecto();
        fs::remove_file(raiz.join(indice::RUTA)).unwrap();
        indice::construir(&raiz).unwrap();
        assert!(!es_propio(
            &raiz,
            plantillas::readme("Otro", "2026-10-11").as_bytes(),
            "Estudio"
        ));
        assert!(es_propio(
            &raiz,
            plantillas::readme("Estudio", "2026-10-11").as_bytes(),
            "Estudio"
        ));
        assert!(!es_propio(&raiz, &[0xff, 0xfe], "Estudio"));
    }

    #[test]
    fn sin_readme_lo_escribe() {
        let (_tmp, raiz) = proyecto();
        fs::remove_file(raiz.join("README.md")).unwrap();
        assert_eq!(regenerar(&raiz, "Estudio", &momento()).unwrap(), None);
        assert!(leer(&raiz, "README.md").contains("se generó el 2026-10-12."));
    }

    #[test]
    fn un_readme_sin_permiso_de_lectura_no_se_reemplaza() {
        use std::os::unix::fs::PermissionsExt;
        let (_tmp, raiz) = proyecto();
        let ruta = raiz.join("README.md");
        fs::write(&ruta, "a mano").unwrap();
        fs::set_permissions(&ruta, fs::Permissions::from_mode(0o000)).unwrap();
        let resultado = regenerar(&raiz, "Estudio", &momento());
        fs::set_permissions(&ruta, fs::Permissions::from_mode(0o600)).unwrap();
        assert!(resultado.is_err());
        assert_eq!(leer(&raiz, "README.md"), "a mano");
    }

    #[test]
    fn un_readme_que_no_se_puede_leer_es_un_error() {
        let (_tmp, raiz) = proyecto();
        fs::remove_file(raiz.join("README.md")).unwrap();
        fs::create_dir(raiz.join("README.md")).unwrap();
        assert!(regenerar(&raiz, "Estudio", &momento()).is_err());
    }
}
