//! Abre el navegador predeterminado en la sesión (SRV-01) sin poner el token
//! en los argumentos de ningún proceso (SRV-03).
//!
//! En Linux cualquier cuenta lee los argumentos de los procesos, así que el
//! navegador recibe la ruta de un archivo de un solo uso que lo lleva a la
//! dirección. El archivo es 0600, vive en la carpeta de estado (0700) y se
//! borra a los 30 s.

use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Lo que dura el archivo de entrada antes de borrarse (SRV-03).
const VIDA_DEL_ARCHIVO: Duration = Duration::from_secs(30);

/// Escribe la página que lleva a `direccion` y devuelve su ruta.
fn preparar(direccion: &str, carpeta: &Path) -> io::Result<PathBuf> {
    let archivo = carpeta.join(format!("abrir-{}.html", std::process::id()));
    let _ = fs::remove_file(&archivo);
    let mut abierto = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&archivo)?;
    // `mode` pasa por la umask; se fija después para no depender de ella.
    fs::set_permissions(&archivo, fs::Permissions::from_mode(0o600))?;
    writeln!(
        abierto,
        "<!doctype html><meta charset=\"utf-8\">\
         <meta http-equiv=\"refresh\" content=\"0;url={direccion}\">\
         <title>Open Social Research</title>"
    )?;
    Ok(archivo)
}

/// Lanza `programa` con el archivo y espera su resultado.
async fn lanzar(programa: &str, archivo: &Path) -> bool {
    tokio::process::Command::new(programa)
        .arg(archivo)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .await
        .is_ok_and(|estado| estado.success())
}

/// El archivo de entrada, que se borra al soltarse. Si `osr` se cierra antes
/// de los 30 s, el entorno de tokio suelta la tarea y el archivo se borra igual.
struct ArchivoDeEntrada(PathBuf);

impl Drop for ArchivoDeEntrada {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

/// Abre el navegador con `xdg-open` en segundo plano. Si no se puede, lo
/// avisa en la salida de error y `osr` sigue corriendo.
pub fn abrir(direccion: &str) {
    let archivo = crate::estado::carpeta()
        .and_then(|carpeta| preparar(direccion, &carpeta))
        .map(ArchivoDeEntrada);
    tokio::spawn(async move {
        let abierto = match &archivo {
            Ok(archivo) => lanzar("xdg-open", &archivo.0).await,
            Err(_) => false,
        };
        if !abierto {
            eprintln!(
                "osr: no se pudo abrir el navegador. Hay que abrir a mano la dirección de arriba."
            );
        }
        tokio::time::sleep(VIDA_DEL_ARCHIVO).await;
        drop(archivo);
    });
}

#[cfg(test)]
mod pruebas {
    use super::*;

    const DIRECCION: &str = "http://127.0.0.1:1/?token=secreto";

    #[test]
    fn el_archivo_lleva_a_la_direccion_y_es_0600() {
        let tmp = tempfile::tempdir().unwrap();
        let archivo = preparar(DIRECCION, tmp.path()).unwrap();
        let texto = fs::read_to_string(&archivo).unwrap();
        assert!(texto.contains("content=\"0;url=http://127.0.0.1:1/?token=secreto\""));
        let modo = fs::metadata(&archivo).unwrap().permissions().mode();
        assert_eq!(modo & 0o777, 0o600);
        assert!(!archivo.to_string_lossy().contains("secreto"));
        // Un segundo arranque del mismo proceso reemplaza el archivo viejo.
        preparar(DIRECCION, tmp.path()).unwrap();
    }

    #[tokio::test]
    async fn lanzar_distingue_exito_falla_y_programa_ausente() {
        let archivo = Path::new("/dev/null");
        assert!(lanzar("true", archivo).await);
        assert!(!lanzar("false", archivo).await);
        assert!(!lanzar("osr-programa-que-no-existe", archivo).await);
    }

    #[test]
    fn el_archivo_de_entrada_se_borra_al_soltarse() {
        let tmp = tempfile::tempdir().unwrap();
        let archivo = preparar(DIRECCION, tmp.path()).unwrap();
        drop(ArchivoDeEntrada(archivo.clone()));
        assert!(!archivo.exists());
    }
}
