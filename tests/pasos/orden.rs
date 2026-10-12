//! Pasos que ejecutan el binario `osr` real y miran su salida.

use std::fs;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use cucumber::{given, then, when};

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

    pub(super) fn salida_estandar(&self) -> String {
        fs::read_to_string(self.salida("estandar")).expect("leer la salida estándar")
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

/// El comienzo de la línea que `osr` escribe al quedar listo (SRV-01).
pub(super) const LISTO: &str = "Open Social Research está abierto en ";

/// El puerto y el token de la dirección que `osr` escribe al quedar listo,
/// si `salida` ya tiene esa línea completa.
pub(super) fn direccion_de(salida: &str) -> Option<(u16, String)> {
    let linea = salida.lines().next().filter(|_| salida.contains('\n'))?;
    let resto = linea
        .strip_prefix(LISTO)?
        .strip_prefix("http://127.0.0.1:")?;
    let (puerto, token) = resto.split_once("/?token=")?;
    Some((puerto.parse().ok()?, token.to_string()))
}

impl Mundo {
    /// Lanza `osr` con los argumentos de `orden` y espera hasta que termine o
    /// hasta que escriba la línea de SRV-01, de la que anota `{P}` y `{token}`.
    pub(super) fn lanzar(&mut self, orden: &str) {
        let partes = dividir(orden);
        assert_eq!(partes[0], "osr", "la orden tiene que empezar con osr");
        let argumentos: Vec<String> = partes[1..].iter().map(|p| self.resolver(p)).collect();
        if let [orden, ruta, ..] = argumentos.as_slice()
            && ["nuevo", "abrir"].contains(&orden.as_str())
        {
            self.proyecto = PathBuf::from(ruta);
        }
        let salidas = tempfile::tempdir().expect("crear la carpeta de salidas");
        let estandar = fs::File::create(salidas.path().join("estandar")).unwrap();
        let error = fs::File::create(salidas.path().join("error")).unwrap();
        self.salidas = Some(salidas);
        let binario = env!("CARGO_BIN_EXE_osr");
        let mut comando = match self.espacio() {
            Some(entrar) => {
                let mut nsenter = entrar;
                nsenter.arg(binario);
                nsenter
            }
            None => Command::new(binario),
        };
        let osr = comando
            .args(&argumentos)
            .current_dir(self.tmp())
            .envs(self.entorno())
            .stdin(Stdio::null())
            .stdout(estandar)
            .stderr(error)
            .spawn()
            .expect("lanzar osr");
        if let Some(mut anterior) = self.osr.replace(osr)
            && anterior.try_wait().unwrap().is_none()
        {
            self.anteriores.push(anterior);
        }
        self.codigo = None;
        self.esperar_listo();
    }

    /// `osr` sigue corriendo cuando queda listo: se espera hasta que termine
    /// o hasta que escriba la dirección.
    fn esperar_listo(&mut self) {
        let inicio = Instant::now();
        while inicio.elapsed() < Duration::from_secs(10) {
            let osr = self.osr.as_mut().unwrap();
            if let Some(estado) = osr.try_wait().unwrap() {
                self.codigo = Some(estado.code().unwrap_or(-1));
                return;
            }
            if let Some((puerto, token)) = direccion_de(&self.salida_estandar()) {
                self.puerto = Some(puerto);
                self.token = Some(token);
                return;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        panic!("osr no terminó ni quedó listo en 10 s");
    }
}

impl Mundo {
    /// Arranca `osr`, anota su puerto y su token y lo cierra de forma ordenada.
    pub(super) fn arrancar_y_cerrar(&mut self, orden: &str) {
        self.lanzar(orden);
        let puerto = self.puerto.expect("osr no escribió su puerto");
        let token = self.token.clone().expect("osr no escribió su token");
        self.anotados.push((puerto, token));
        let codigo = terminar(self.osr.as_mut().unwrap());
        assert_eq!(codigo, Some(0), "osr no terminó con código 0 en 3 s");
    }

    /// Crea `{proyecto}` con `osr nuevo` y lo cierra de forma ordenada, así
    /// queda en el disco como lo deja la app cerrada.
    pub(super) fn crear_cerrado(&mut self) {
        let proyecto = self.proyecto.to_string_lossy().into_owned();
        self.lanzar(&format!("osr nuevo \"{proyecto}\" --sin-navegador"));
        let mut osr = self.osr.take().expect("falta osr");
        assert_eq!(terminar(&mut osr), Some(0), "osr nuevo no cerró en orden");
    }
}

// Solo las órdenes que existen, `osr`, `osr nuevo`, `osr abrir` y `osr
// <ruta>` con el proyecto del escenario; con otras el paso queda sin definir
// y sus escenarios cuentan como omitidos en vez de fallar (D-20). Quien
// implemente otra orden amplía esta captura.
#[when(
    regex = r"^se ejecuta `(osr(?: nuevo .*| abrir .*| \{proyecto\}.*| --sin-navegador| --opcion-falsa)?)`$"
)]
fn se_ejecuta(mundo: &mut Mundo, orden: String) {
    mundo.lanzar(&orden);
}

#[when("osr se cierra de forma ordenada")]
fn se_cierra(mundo: &mut Mundo) {
    let osr = mundo.osr.as_mut().expect("falta «se ejecuta»");
    let codigo = terminar(osr);
    assert_eq!(codigo, Some(0), "osr no terminó con código 0 en 3 s");
    mundo.codigo = codigo;
}

/// Abre con `osr abrir` el proyecto que ya preparó el escenario (D-27).
#[given("osr está corriendo con el proyecto abierto")]
fn corre_con_el_proyecto(mundo: &mut Mundo) {
    let proyecto = mundo.proyecto.to_string_lossy().into_owned();
    mundo.lanzar(&format!("osr abrir \"{proyecto}\" --sin-navegador"));
    esta_corriendo(mundo);
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

// Solo los códigos que `osr` ya produce (D-20).
#[then(regex = r"^el código de salida es (2|3|4|5|6)$")]
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

#[when("se abre el proyecto con `osr abrir {proyecto} --sin-navegador`")]
fn se_abre(mundo: &mut Mundo) {
    mundo.lanzar("osr abrir {proyecto} --sin-navegador");
}

/// `Child::kill` manda SIGKILL. Se espera al proceso para que no quede como
/// zombi, que seguiría figurando en `/proc` como vivo.
#[given("se envía SIGKILL al proceso osr")]
fn sigkill(mundo: &mut Mundo) {
    let osr = mundo.osr.as_mut().expect("falta «se ejecuta»");
    osr.kill().expect("mandar SIGKILL");
    osr.wait().expect("esperar a osr");
}

#[then("el primer proceso osr sigue atendiendo pedidos autenticados")]
async fn primero_atiende(mundo: &mut Mundo) {
    let primero = mundo
        .anteriores
        .first_mut()
        .expect("falta el primer proceso");
    assert!(primero.try_wait().unwrap().is_none(), "el primero terminó");
    let token = mundo.token.clone().expect("osr no escribió su token");
    let respuesta = super::http::Pedido::nuevo("GET", mundo.direccion("/"))
        .con_encabezado("Authorization", &format!("Bearer {token}"))
        .mandar()
        .await;
    assert_eq!(respuesta.estado, 200);
}
