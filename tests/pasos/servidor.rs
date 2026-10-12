//! Pasos propios del módulo servidor (SRV).

use std::time::{Duration, Instant};

use cucumber::{given, then};

use super::Mundo;
use super::orden::direccion_de;

#[given("el sistema de prueba no tiene ningún navegador disponible")]
fn sin_navegador(mundo: &mut Mundo) {
    mundo.sin_navegador = true;
}

#[then(expr = "la salida estándar tiene exactamente una línea con la forma {string}")]
fn una_linea_con_la_forma(mundo: &mut Mundo, forma: String) {
    let salida = mundo.salida_estandar();
    assert_eq!(salida.lines().count(), 1, "{salida:?}");
    let (_, token) = direccion_de(&salida).expect("la línea no tiene la forma de SRV-01");
    assert_eq!(token.len(), 43, "{token}");
    assert_eq!(salida, format!("{}\n", mundo.resolver(&forma)));
}

/// La línea puede llegar un poco después del arranque, cuando el intento de
/// abrir el navegador termina, así que se espera hasta 5 s.
#[then(expr = "la salida de error tiene la línea {string}")]
fn error_tiene_la_linea(mundo: &mut Mundo, linea: String) {
    let inicio = Instant::now();
    loop {
        let error = std::fs::read_to_string(mundo.salidas.as_ref().unwrap().path().join("error"))
            .expect("leer la salida de error");
        if error.lines().any(|l| l == linea) {
            return;
        }
        assert!(inicio.elapsed() < Duration::from_secs(5), "{error:?}");
        std::thread::sleep(Duration::from_millis(20));
    }
}

#[then("osr sigue corriendo")]
fn sigue_corriendo(mundo: &mut Mundo) {
    let osr = mundo.osr.as_mut().expect("falta «se ejecuta»");
    assert!(osr.try_wait().unwrap().is_none(), "osr ya terminó");
}
