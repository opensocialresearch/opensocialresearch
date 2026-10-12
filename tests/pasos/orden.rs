//! Pasos que ejecutan el binario `osr` real y miran su salida.

use std::fs;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use cucumber::{then, when};

use super::{Mundo, esperar, sigterm};

/// Divide una orden como lo haría la terminal con comillas dobles, sin
/// perder los argumentos vacíos como `""`.
fn dividir(orden: &str) -> Vec<String> {
    let mut partes = Vec::new();
    let mut actual = String::new();
    let (mut en_comillas, mut en_parte) = (false, false);
    for letra in orden.chars() {
        match letra {
            '"' => (en_comillas, en_parte) = (!en_comillas, true),
            ' ' if !en_comillas => {
                if en_parte {
                    partes.push(std::mem::take(&mut actual));
                }
                en_parte = false;
            }
            _ => {
                actual.push(letra);
                en_parte = true;
            }
        }
    }
    if en_parte {
        partes.push(actual);
    }
    partes
}

/// Termina `osr` de forma ordenada y, si no responde en 3 s, a la fuerza.
pub(super) fn terminar(osr: &mut Child) -> Option<i32> {
    if osr.try_wait().ok().flatten().is_some() {
        return None;
    }
    sigterm(osr);
    let codigo = esperar(osr, Duration::from_secs(3));
    if codigo.is_none() {
        let _ = osr.kill();
        let _ = osr.wait();
    }
    codigo
}

impl Mundo {
    fn salida(&self, nombre: &str) -> PathBuf {
        self.salidas
            .as_ref()
            .expect("falta «se ejecuta»")
            .path()
            .join(nombre)
    }

    fn salida_de_error(&self) -> String {
        fs::read_to_string(self.salida("error")).expect("leer la salida de error")
    }

    /// El código de salida, esperando hasta 5 s a que `osr` termine.
    fn codigo_de_salida(&mut self) -> i32 {
        if self.codigo.is_none() {
            let osr = self.osr.as_mut().expect("falta «se ejecuta»");
            self.codigo = esperar(osr, Duration::from_secs(5));
        }
        self.codigo.expect("osr no terminó en 5 s")
    }

    fn bloqueo(&self) -> PathBuf {
        self.ruta(".osr/bloqueo")
    }
}

// Solo `osr nuevo` existe todavía; con otras órdenes el paso queda sin
// definir y sus escenarios cuentan como omitidos en vez de fallar. Quien
// implemente `osr abrir` o `osr` solo amplía esta captura.
#[when(regex = r"^se ejecuta `(osr nuevo .*)`$")]
fn se_ejecuta(mundo: &mut Mundo, orden: String) {
    let partes = dividir(&orden);
    assert_eq!(partes[0], "osr", "la orden tiene que empezar con osr");
    let argumentos: Vec<String> = partes[1..].iter().map(|p| mundo.resolver(p)).collect();
    mundo.proyecto = PathBuf::from(&argumentos[1]);
    let salidas = tempfile::tempdir().expect("crear la carpeta de salidas");
    let estandar = fs::File::create(salidas.path().join("estandar")).unwrap();
    let error = fs::File::create(salidas.path().join("error")).unwrap();
    mundo.salidas = Some(salidas);
    let binario = env!("CARGO_BIN_EXE_osr");
    let mut comando = match &mundo.disco {
        Some(disco) => {
            let mut nsenter = Command::new("nsenter");
            let pid = disco.id().to_string();
            nsenter.args(["-t", &pid, "-U", "-m", "--preserve-credentials", binario]);
            nsenter
        }
        None => Command::new(binario),
    };
    let osr = comando
        .args(&argumentos)
        .current_dir(mundo.tmp())
        .stdin(Stdio::null())
        .stdout(estandar)
        .stderr(error)
        .spawn()
        .expect("lanzar osr");
    mundo.osr = Some(osr);
    mundo.codigo = None;
    // `osr` sigue corriendo cuando abre un proyecto: se espera hasta que
    // termine o hasta que el proyecto quede abierto, con su bloqueo.
    let inicio = Instant::now();
    while inicio.elapsed() < Duration::from_secs(10) {
        let osr = mundo.osr.as_mut().unwrap();
        if let Some(estado) = osr.try_wait().unwrap() {
            mundo.codigo = Some(estado.code().unwrap_or(-1));
            return;
        }
        if mundo.bloqueo().exists() {
            return;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    panic!("osr no terminó ni abrió el proyecto en 10 s");
}

#[when("osr se cierra de forma ordenada")]
fn se_cierra(mundo: &mut Mundo) {
    let osr = mundo.osr.as_mut().expect("falta «se ejecuta»");
    let codigo = terminar(osr);
    assert_eq!(codigo, Some(0), "osr no terminó con código 0 en 3 s");
    mundo.codigo = codigo;
}

#[then("osr está corriendo con el proyecto abierto")]
fn esta_corriendo(mundo: &mut Mundo) {
    let osr = mundo.osr.as_mut().expect("falta «se ejecuta»");
    assert!(osr.try_wait().unwrap().is_none(), "osr ya terminó");
    let pid = osr.id();
    let bloqueo = fs::read_to_string(mundo.bloqueo()).expect("leer .osr/bloqueo");
    assert_eq!(
        bloqueo.lines().next(),
        Some(format!("pid = {pid}").as_str())
    );
}

#[then(expr = "el código de salida es {int}")]
fn codigo_es(mundo: &mut Mundo, codigo: i32) {
    assert_eq!(mundo.codigo_de_salida(), codigo);
}

#[then("la salida de error está vacía")]
fn error_vacio(mundo: &mut Mundo) {
    assert_eq!(mundo.salida_de_error(), "");
}

#[then(expr = "la salida de error es la línea {string}")]
fn error_es_la_linea(mundo: &mut Mundo, linea: String) {
    assert_eq!(
        mundo.salida_de_error(),
        format!("{}\n", mundo.resolver(&linea))
    );
}

#[then(expr = "la salida de error empieza con {string}")]
fn error_empieza(mundo: &mut Mundo, inicio: String) {
    let error = mundo.salida_de_error();
    let inicio = mundo.resolver(&inicio);
    assert!(
        error.starts_with(&inicio),
        "{error:?} no empieza con {inicio:?}"
    );
}

#[then(expr = "la salida de error termina con {string}")]
fn error_termina(mundo: &mut Mundo, fin: String) {
    let error = mundo.salida_de_error();
    let sin_salto = error.strip_suffix('\n').unwrap_or(&error);
    assert!(
        sin_salto.ends_with(&fin),
        "{error:?} no termina con {fin:?}"
    );
}
