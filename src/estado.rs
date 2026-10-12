//! La carpeta de estado de `osr`, fuera de todo proyecto.

use std::ffi::OsString;
use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

/// `$XDG_STATE_HOME/osr`, o `~/.local/state/osr` si la variable falta o
/// está vacía, como pide la especificación XDG.
fn carpeta_desde(xdg: Option<OsString>, casa: Option<OsString>) -> PathBuf {
    match xdg.filter(|x| !x.is_empty()) {
        Some(xdg) => PathBuf::from(xdg).join("osr"),
        None => PathBuf::from(casa.unwrap_or_default()).join(".local/state/osr"),
    }
}

/// Crea la carpeta de estado si falta y la deja en 0700 (SRV-13).
pub fn carpeta() -> io::Result<PathBuf> {
    let carpeta = carpeta_desde(std::env::var_os("XDG_STATE_HOME"), std::env::var_os("HOME"));
    fs::create_dir_all(&carpeta)?;
    fs::set_permissions(&carpeta, fs::Permissions::from_mode(0o700))?;
    Ok(carpeta)
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn usa_xdg_state_home_y_si_falta_la_casa() {
        let casa = Some(OsString::from("/home/x"));
        assert_eq!(
            carpeta_desde(Some("/e".into()), casa.clone()),
            PathBuf::from("/e/osr")
        );
        assert_eq!(
            carpeta_desde(None, casa.clone()),
            PathBuf::from("/home/x/.local/state/osr")
        );
        assert_eq!(
            carpeta_desde(Some("".into()), casa),
            PathBuf::from("/home/x/.local/state/osr")
        );
    }

    /// Cambiar el entorno del proceso de pruebas no es seguro con hilos, así
    /// que la prueba se vuelve a lanzar a sí misma con otro entorno.
    #[test]
    fn carpeta_crea_la_de_xdg_state_home_en_0700() {
        const HIJA: &str = "OSR_PRUEBA_CARPETA_HIJA";
        if std::env::var_os(HIJA).is_some() {
            carpeta().unwrap();
            return;
        }
        let tmp = tempfile::tempdir().unwrap();
        let estado = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "estado::pruebas::carpeta_crea_la_de_xdg_state_home_en_0700",
            ])
            .env(HIJA, "1")
            .env("XDG_STATE_HOME", tmp.path())
            .stdout(std::process::Stdio::null())
            .status()
            .unwrap();
        assert!(estado.success());
        let modo = fs::metadata(tmp.path().join("osr"))
            .unwrap()
            .permissions()
            .mode();
        assert_eq!(modo & 0o777, 0o700);
    }
}
