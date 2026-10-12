//! El registro técnico de `osr`, en la carpeta de estado (SRV-12).
//!
//! Por ahora anota el arranque, el puerto, si hay IPv6 de loopback y el
//! cierre. La rotación y los pedidos rechazados llegan con SRV-12.

use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::Path;

/// Agrega una línea con la marca de tiempo, el nivel y el mensaje.
fn anotar_en(carpeta: &Path, marca: &str, nivel: &str, mensaje: &str) -> io::Result<()> {
    let ruta = carpeta.join("registro.log");
    let mut archivo = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .mode(0o600)
        .open(&ruta)?;
    fs::set_permissions(&ruta, fs::Permissions::from_mode(0o600))?;
    writeln!(archivo, "{marca} {nivel} {mensaje}")
}

/// Anota en el registro técnico. Si no se puede escribir, `osr` sigue, porque
/// el registro es de diagnóstico y no guarda nada del proyecto.
pub fn anotar(nivel: &str, mensaje: &str) {
    let marca = crate::nuevo::Momento::ahora().marca;
    let _ =
        crate::estado::carpeta().and_then(|carpeta| anotar_en(&carpeta, &marca, nivel, mensaje));
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn agrega_lineas_con_marca_nivel_y_mensaje_en_0600() {
        let tmp = tempfile::tempdir().unwrap();
        anotar_en(tmp.path(), "2026-10-12T05:00:00-06:00", "info", "arranque").unwrap();
        anotar_en(tmp.path(), "2026-10-12T05:00:01-06:00", "aviso", "sin IPv6").unwrap();
        let ruta = tmp.path().join("registro.log");
        assert_eq!(
            fs::read_to_string(&ruta).unwrap(),
            "2026-10-12T05:00:00-06:00 info arranque\n2026-10-12T05:00:01-06:00 aviso sin IPv6\n"
        );
        let modo = fs::metadata(&ruta).unwrap().permissions().mode();
        assert_eq!(modo & 0o777, 0o600);
    }
}
