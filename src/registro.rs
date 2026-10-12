//! El registro técnico de `osr`, en la carpeta de estado (SRV-12).
//!
//! Anota el arranque, el puerto, si hay IPv6 de loopback, el cierre y cada
//! pedido rechazado. Cuando una línea haría pasar el archivo de 5.000.000
//! bytes, lo rota a `registro.log.1`, así los dos nunca suman más de
//! 10.000.000 (D-35).

use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::Path;
use std::sync::Mutex;

/// El tamaño máximo de `registro.log` y de `registro.log.1`.
const LIMITE: u64 = 5_000_000;

/// Los caracteres que se conservan de un camino rechazado.
pub const CAMINO_MAXIMO: usize = 200;

/// Una escritura a la vez, para que dos pedidos rechazados al mismo tiempo no
/// miren el mismo tamaño y pasen juntos el límite.
static ESCRITURA: Mutex<()> = Mutex::new(());

/// Los primeros `maximo` caracteres de `texto`, sin partir ninguno.
pub fn recortar(texto: &str, maximo: usize) -> &str {
    match texto.char_indices().nth(maximo) {
        Some((corte, _)) => &texto[..corte],
        None => texto,
    }
}

/// La línea completa, con su salto, recortada para que entre sola en un
/// archivo de `limite` bytes sin partir ningún carácter (D-35).
fn linea(marca: &str, nivel: &str, mensaje: &str, limite: u64) -> String {
    let linea = format!("{marca} {nivel} {mensaje}");
    let corte = linea.floor_char_boundary(limite as usize - 1);
    format!("{}\n", &linea[..corte])
}

/// Si agregar `bytes` haría pasar `registro.log` de `limite`, lo renombra a
/// `registro.log.1`, que reemplaza al anterior.
fn rotar_si_hace_falta(carpeta: &Path, bytes: usize, limite: u64) -> io::Result<()> {
    let ruta = carpeta.join("registro.log");
    let tamano = fs::metadata(&ruta).map_or(0, |datos| datos.len());
    if tamano + bytes as u64 > limite {
        fs::rename(&ruta, carpeta.join("registro.log.1"))?;
    }
    Ok(())
}

/// Agrega una línea con la marca de tiempo, el nivel y el mensaje, rotando
/// antes si hace falta.
fn anotar_en(
    carpeta: &Path,
    limite: u64,
    marca: &str,
    nivel: &str,
    mensaje: &str,
) -> io::Result<()> {
    let linea = linea(marca, nivel, mensaje, limite);
    rotar_si_hace_falta(carpeta, linea.len(), limite)?;
    let ruta = carpeta.join("registro.log");
    let mut archivo = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .mode(0o600)
        .open(&ruta)?;
    fs::set_permissions(&ruta, fs::Permissions::from_mode(0o600))?;
    archivo.write_all(linea.as_bytes())
}

/// Anota en el registro técnico. Si no se puede escribir, `osr` sigue, porque
/// el registro es de diagnóstico y no guarda nada del proyecto.
pub fn anotar(nivel: &str, mensaje: &str) {
    let marca = crate::nuevo::Momento::ahora().marca;
    let _escribiendo = ESCRITURA
        .lock()
        .unwrap_or_else(|envenenado| envenenado.into_inner());
    let _ = crate::estado::carpeta()
        .and_then(|carpeta| anotar_en(&carpeta, LIMITE, &marca, nivel, mensaje));
}

#[cfg(test)]
mod pruebas {
    use super::*;

    const MARCA: &str = "2026-10-12T05:00:00-06:00";

    #[test]
    fn agrega_lineas_con_marca_nivel_y_mensaje_en_0600() {
        let tmp = tempfile::tempdir().unwrap();
        anotar_en(tmp.path(), LIMITE, MARCA, "info", "arranque").unwrap();
        anotar_en(tmp.path(), LIMITE, MARCA, "aviso", "sin IPv6").unwrap();
        let ruta = tmp.path().join("registro.log");
        assert_eq!(
            fs::read_to_string(&ruta).unwrap(),
            format!("{MARCA} info arranque\n{MARCA} aviso sin IPv6\n")
        );
        let modo = fs::metadata(&ruta).unwrap().permissions().mode();
        assert_eq!(modo & 0o777, 0o600);
    }

    #[test]
    fn recortar_cuenta_caracteres_y_no_parte_ninguno() {
        assert_eq!(recortar("abc", 2), "ab");
        assert_eq!(recortar("abc", 3), "abc");
        assert_eq!(recortar("abc", 4), "abc");
        assert_eq!(recortar("ñañá", 3), "ñañ");
        let largo = format!("/{}", "é".repeat(250));
        let recortado = recortar(&largo, CAMINO_MAXIMO);
        assert_eq!(recortado.chars().count(), 200);
        assert!(largo.starts_with(recortado));
    }

    /// Deja `registro.log` con `tamano` bytes, disperso, sin ocupar el disco.
    fn registro_de(carpeta: &Path, tamano: u64) {
        let archivo = fs::File::create(carpeta.join("registro.log")).unwrap();
        archivo.set_len(tamano).unwrap();
    }

    fn tamano(carpeta: &Path, nombre: &str) -> Option<u64> {
        fs::metadata(carpeta.join(nombre)).ok().map(|d| d.len())
    }

    #[test]
    fn una_linea_que_llega_justo_al_limite_no_rota() {
        let tmp = tempfile::tempdir().unwrap();
        let bytes = linea(MARCA, "info", "x", LIMITE).len() as u64;
        registro_de(tmp.path(), LIMITE - bytes);
        anotar_en(tmp.path(), LIMITE, MARCA, "info", "x").unwrap();
        assert_eq!(tamano(tmp.path(), "registro.log"), Some(LIMITE));
        assert_eq!(tamano(tmp.path(), "registro.log.1"), None);
    }

    #[test]
    fn una_linea_que_pasaria_el_limite_rota_y_empieza_otro() {
        let tmp = tempfile::tempdir().unwrap();
        let bytes = linea(MARCA, "info", "x", LIMITE).len() as u64;
        registro_de(tmp.path(), LIMITE - bytes + 1);
        anotar_en(tmp.path(), LIMITE, MARCA, "info", "x").unwrap();
        assert_eq!(
            tamano(tmp.path(), "registro.log.1"),
            Some(LIMITE - bytes + 1)
        );
        assert_eq!(
            fs::read_to_string(tmp.path().join("registro.log")).unwrap(),
            format!("{MARCA} info x\n")
        );
        let modo = fs::metadata(tmp.path().join("registro.log"))
            .unwrap()
            .permissions()
            .mode();
        assert_eq!(modo & 0o777, 0o600);
    }

    #[test]
    fn la_rotacion_reemplaza_el_anterior_y_nunca_pasa_del_doble() {
        let tmp = tempfile::tempdir().unwrap();
        let limite = 100;
        for i in 0..20 {
            anotar_en(tmp.path(), limite, MARCA, "aviso", &format!("n{i:02}")).unwrap();
            let log = tamano(tmp.path(), "registro.log").unwrap();
            let anterior = tamano(tmp.path(), "registro.log.1").unwrap_or(0);
            assert!(log <= limite && anterior <= limite);
        }
        let anterior = fs::read_to_string(tmp.path().join("registro.log.1")).unwrap();
        let actual = fs::read_to_string(tmp.path().join("registro.log")).unwrap();
        assert!(anterior.ends_with(" n17\n"), "{anterior:?}");
        assert!(actual.ends_with(" n19\n"), "{actual:?}");
        let nombres: Vec<_> = fs::read_dir(tmp.path()).unwrap().collect();
        assert_eq!(nombres.len(), 2);
    }

    #[test]
    fn una_linea_mas_larga_que_el_limite_se_recorta_sin_partir_caracteres() {
        let exacta = linea("m", "i", "abcdefgh", 15);
        assert_eq!(exacta, "m i abcdefgh\n");
        assert_eq!(linea("m", "i", "abcdefghi", 15), "m i abcdefghi\n");
        assert_eq!(linea("m", "i", "abcdefghij", 15), "m i abcdefghij\n");
        assert_eq!(linea("m", "i", "abcdefghijk", 15), "m i abcdefghij\n");
        assert_eq!(linea("m", "i", "abcdefghiñ", 15), "m i abcdefghi\n");
        let tmp = tempfile::tempdir().unwrap();
        anotar_en(tmp.path(), 15, "m", "i", &"z".repeat(40)).unwrap();
        assert_eq!(tamano(tmp.path(), "registro.log"), Some(15));
        assert_eq!(tamano(tmp.path(), "registro.log.1"), None);
    }
}
