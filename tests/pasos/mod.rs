//! Definiciones de los pasos de los escenarios y el mundo que comparten.
//!
//! Las frases son las de los `.feature`, que son contrato: se reutilizan en
//! otros requisitos y no se cambian para que una prueba pase.

mod archivos;
mod diseno;
mod http;
mod orden;
mod pantalla;
mod readme;
mod servidor;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{BufRead, BufReader};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use cucumber::World;
use tempfile::TempDir;

/// El estado de un escenario.
#[derive(Debug, World)]
#[world(init = Self::nuevo)]
pub struct Mundo {
    /// `{tmp}`, la carpeta temporal nueva de cada escenario.
    tmp: TempDir,
    /// `{proyecto}`, `{tmp}/estudio` salvo que una orden cree otro.
    proyecto: PathBuf,
    /// Carpeta aparte para la salida estándar y la de error de `osr`.
    salidas: Option<TempDir>,
    /// El proceso `osr` lanzado por el último «se ejecuta».
    osr: Option<Child>,
    /// Los procesos `osr` que seguían corriendo cuando se lanzó otro, del
    /// más viejo al más nuevo, para no perderlos ni dejarlos vivos.
    anteriores: Vec<Child>,
    /// El puerto y el token de cada proceso de `anteriores`, en el mismo orden.
    sesiones_anteriores: Vec<(Option<u16>, Option<String>)>,
    /// Cuántas líneas tenía el registro de eventos al guardar la cantidad.
    lineas_de_eventos: usize,
    /// El cuerpo de la interfaz que guardó «se guarda el estado que muestra la app».
    estado_guardado: Option<String>,
    /// Código de salida de `osr`, cuando ya terminó.
    codigo: Option<i32>,
    /// Proceso que sostiene el espacio de nombres del disco de prueba.
    disco: Option<Child>,
    /// Proceso que sostiene el espacio de nombres de la red de prueba.
    red: Option<Child>,
    /// Contenido de cada ruta tomado por «se tomó la huella».
    huellas: BTreeMap<String, Option<Vec<u8>>>,
    /// Carpetas a las que el escenario les quitó permisos.
    restaurar_permisos: Vec<PathBuf>,
    /// Rutas fuera de `{tmp}` que el escenario crea y hay que borrar al final.
    limpiar: Vec<PathBuf>,
    /// Carpeta fuera de `{tmp}` con la carpeta de estado de `osr` y el
    /// navegador de prueba, para no tocar los del equipo.
    aparte: TempDir,
    /// `{P}` y `{token}`, de la línea que escribió el último arranque.
    puerto: Option<u16>,
    token: Option<String>,
    /// El puerto y el token de cada arranque de «osr se arranca N veces».
    anotados: Vec<(u16, String)>,
    /// `{token anterior}`, el de un arranque previo ya cerrado.
    token_anterior: Option<String>,
    /// El hilo que lee las líneas de órdenes de todos los procesos y la
    /// bandera que lo detiene.
    lector_de_ordenes: Option<(Arc<AtomicBool>, JoinHandle<BTreeSet<String>>)>,
    /// Lo que pasó con la conexión TCP que intentó la prueba.
    conexion: Option<String>,
    /// Si `osr` corre sin ningún navegador en el `PATH` (SRV-01.3).
    sin_navegador: bool,
    /// El pedido HTTP que arma el escenario y la respuesta que recibió.
    pedido: Option<http::Pedido>,
    respuesta: Option<http::Respuesta>,
    /// Método y camino de cada ruta que declara el servidor, y la respuesta
    /// de cada camino al pedido que les mandó el escenario.
    lista_de_rutas: Vec<(String, String)>,
    respuestas: Vec<(String, http::Respuesta)>,
    /// Cuándo se mandó SIGTERM y cuántas líneas tenía antes el registro técnico.
    senal_enviada: Option<Instant>,
    /// El hilo que mira cuándo termina de verdad el proceso después de la
    /// señal, y devuelve ese instante.
    fin_observado: Option<JoinHandle<Option<Instant>>>,
    lineas_del_registro: usize,
    /// Las familias de rutas que el escenario ya revisó por su nombre.
    familias_vistas: Vec<String>,
    /// Cada pedido de la batería de SRV-10, con su factor inválido, su
    /// método, su camino y la respuesta.
    bateria: Vec<servidor::Disparo>,
    /// `XDG_STATE_HOME` de `osr` si el escenario la fija; si no, va en `aparte`.
    xdg_state_home: Option<PathBuf>,
    /// La umask con la que se lanza `osr`, en octal, si el escenario la fija.
    umask: Option<String>,
    /// `HOME` de `osr`, si el escenario la fija. Si no, una carpeta personal
    /// en `aparte`, así ningún `osr` escribe en la del equipo (PRY-24).
    home: Option<PathBuf>,
    /// La segunda huella de un escenario que compara dos carpetas.
    segunda_huella: BTreeMap<String, Option<Vec<u8>>>,
    /// La versión de un paso del diseño anotada por el escenario (DIS-04.2).
    version_anotada: Option<String>,
    /// Los bytes que guardó «se guardó una copia de … como versión editada».
    version_editada: Option<Vec<u8>>,
}

impl Mundo {
    fn nuevo() -> Mundo {
        let tmp = tempfile::tempdir().expect("crear {tmp}");
        Mundo {
            proyecto: tmp.path().join("estudio"),
            tmp,
            salidas: None,
            osr: None,
            anteriores: Vec::new(),
            sesiones_anteriores: Vec::new(),
            estado_guardado: None,
            lineas_de_eventos: 0,
            codigo: None,
            disco: None,
            red: None,
            huellas: BTreeMap::new(),
            restaurar_permisos: Vec::new(),
            limpiar: Vec::new(),
            aparte: aparte(),
            puerto: None,
            token: None,
            anotados: Vec::new(),
            conexion: None,
            token_anterior: None,
            lector_de_ordenes: None,
            sin_navegador: false,
            pedido: None,
            respuesta: None,
            lista_de_rutas: Vec::new(),
            respuestas: Vec::new(),
            familias_vistas: Vec::new(),
            bateria: Vec::new(),
            senal_enviada: None,
            fin_observado: None,
            lineas_del_registro: 0,
            xdg_state_home: None,
            umask: None,
            home: None,
            segunda_huella: BTreeMap::new(),
            version_anotada: None,
            version_editada: None,
        }
    }

    /// El entorno de cada `osr`. La carpeta de estado queda en `aparte` y el
    /// `PATH` solo tiene un `xdg-open` de prueba que anota cómo lo llamaron,
    /// o nada si el escenario pide que no haya navegador.
    fn entorno(&self) -> Vec<(&'static str, PathBuf)> {
        let carpeta = if self.sin_navegador { "vacia" } else { "bin" };
        let estado = self.xdg_state_home.clone();
        let mut entorno = vec![
            (
                "XDG_STATE_HOME",
                estado.unwrap_or_else(|| self.aparte.path().join("estado")),
            ),
            ("PATH", self.aparte.path().join(carpeta)),
        ];
        entorno.push(("HOME", self.casa()));
        entorno
    }

    /// La carpeta personal de `osr` en el escenario.
    pub(super) fn casa(&self) -> PathBuf {
        self.home
            .clone()
            .unwrap_or_else(|| self.aparte.path().join("casa"))
    }

    /// `{aplicación}`, la carpeta de la aplicación de PRY-24.
    pub(super) fn aplicacion(&self) -> PathBuf {
        self.casa().join("Open Social Research")
    }

    /// `nsenter` listo para correr un programa en el espacio de nombres del
    /// disco o de la red de prueba, si el escenario preparó alguno.
    fn espacio(&self) -> Option<Command> {
        let (proceso, espacio) = match (&self.disco, &self.red) {
            (Some(disco), _) => (disco, "-m"),
            (None, Some(red)) => (red, "-n"),
            (None, None) => return None,
        };
        let mut nsenter = Command::new(programa("nsenter"));
        let pid = proceso.id().to_string();
        nsenter.args(["-t", &pid, "-U", espacio, "--preserve-credentials"]);
        Some(nsenter)
    }

    /// La carpeta de estado de `osr`, la de las pruebas salvo que el
    /// escenario fije `XDG_STATE_HOME`.
    fn carpeta_de_estado(&self) -> PathBuf {
        self.entorno()[0].1.join("osr")
    }

    /// El registro técnico de `osr`, en su carpeta de estado.
    fn registro_tecnico(&self) -> String {
        fs::read_to_string(self.carpeta_de_estado().join("registro.log"))
            .expect("leer el registro técnico")
    }

    fn tmp(&self) -> &Path {
        self.tmp.path()
    }

    /// Reemplaza los marcadores de las convenciones de `proyecto.feature`.
    fn resolver(&self, texto: &str) -> String {
        let mut texto = texto.replace("{201 veces la letra a}", &"a".repeat(201));
        texto = texto.replace("{120 veces la letra a}", &"a".repeat(120));
        texto = texto.replace("{60 veces la letra a}", &"a".repeat(60));
        texto = texto.replace("{aplicación}", &self.aplicacion().to_string_lossy());
        texto = texto.replace("{año con la ñ descompuesta}", "an\u{303}o");
        texto = texto.replace(
            "{hoy}",
            &jiff::Zoned::now().strftime("%Y-%m-%d").to_string(),
        );
        texto = texto.replace("{salto de línea}", "\n");
        if texto.contains("{hoy en palabras}") {
            const MESES: [&str; 12] = [
                "enero",
                "febrero",
                "marzo",
                "abril",
                "mayo",
                "junio",
                "julio",
                "agosto",
                "septiembre",
                "octubre",
                "noviembre",
                "diciembre",
            ];
            let hoy = jiff::Zoned::now();
            let palabras = format!("{} de {}", hoy.day(), MESES[hoy.month() as usize - 1]);
            texto = texto.replace("{hoy en palabras}", &palabras);
        }
        texto = texto.replace("{Uno, salto de línea, Dos}", "Uno\nDos");
        if let Some(primero) = self.anteriores.first() {
            texto = texto.replace("{PID del primer proceso}", &primero.id().to_string());
        }
        if let Some(anterior) = &self.token_anterior {
            texto = texto.replace("{token anterior}", anterior);
        }
        if let (Some(puerto), Some(token)) = (self.puerto, &self.token) {
            texto = texto.replace("{P+1}", &(puerto + 1).to_string());
            texto = texto.replace("{P}", &puerto.to_string());
            texto = texto.replace("{token}", token);
        }
        if texto.contains("{tamaño de E01.ogg}") {
            let audio = self.proyecto.join("identificable/entrevistas/E01/E01.ogg");
            let tamano = fs::metadata(&audio)
                .expect("leer el tamaño de E01.ogg")
                .len();
            texto = texto.replace("{tamaño de E01.ogg}", &tamano.to_string());
        }
        texto = texto.replace("{tmp}", &self.tmp().to_string_lossy());
        texto.replace("{proyecto}", &self.proyecto.to_string_lossy())
    }

    /// La ruta de un texto del escenario, relativa a `{proyecto}` si no es
    /// absoluta, vista desde el espacio de nombres del disco de prueba si hay.
    fn ruta(&self, texto: &str) -> PathBuf {
        let ruta = PathBuf::from(self.resolver(texto));
        let ruta = if ruta.is_absolute() {
            ruta
        } else {
            self.proyecto.join(ruta)
        };
        let ruta = con_marca(ruta);
        match &self.disco {
            Some(disco) => Path::new(&format!("/proc/{}/root", disco.id()))
                .join(ruta.strip_prefix("/").expect("ruta absoluta")),
            None => ruta,
        }
    }
}

/// Si el nombre del archivo tiene `{marca}`, la ruta del único archivo de su
/// carpeta que tiene ahí una marca compacta `AAAAMMDDTHHMMSS` generada por la
/// app. Si no hay exactamente uno, la ruta queda con `{marca}` y el paso que
/// la usa falla al no encontrarla.
fn con_marca(ruta: PathBuf) -> PathBuf {
    let Some(nombre) = ruta.file_name().and_then(|n| n.to_str()) else {
        return ruta;
    };
    let Some((antes, despues)) = nombre.split_once("{marca}") else {
        return ruta;
    };
    let es_marca = |marca: &str| {
        let bytes = marca.as_bytes();
        bytes.len() == 15
            && bytes[8] == b'T'
            && bytes
                .iter()
                .enumerate()
                .all(|(i, b)| i == 8 || b.is_ascii_digit())
    };
    let carpeta = ruta.parent().unwrap_or(Path::new("."));
    let candidatos: Vec<PathBuf> = fs::read_dir(carpeta)
        .into_iter()
        .flatten()
        .filter_map(|entrada| {
            let otro = entrada.ok()?.file_name().into_string().ok()?;
            let marca = otro.strip_prefix(antes)?.strip_suffix(despues)?;
            es_marca(marca).then(|| carpeta.join(&otro))
        })
        .collect();
    match <[PathBuf; 1]>::try_from(candidatos) {
        Ok([unica]) => unica,
        Err(_) => ruta,
    }
}

impl Drop for Mundo {
    fn drop(&mut self) {
        // Un escenario que no llegó a su último paso deja el lector de
        // órdenes corriendo; se detiene para no gastar en el resto de la corrida.
        if let Some((seguir, hilo)) = self.lector_de_ordenes.take() {
            seguir.store(false, std::sync::atomic::Ordering::Relaxed);
            let _ = hilo.join();
        }
        for mut osr in self.osr.take().into_iter().chain(self.anteriores.drain(..)) {
            orden::terminar(&mut osr);
        }
        for mut espacio in [self.disco.take(), self.red.take()].into_iter().flatten() {
            let _ = espacio.kill();
            let _ = espacio.wait();
        }
        for carpeta in &self.restaurar_permisos {
            let _ = fs::set_permissions(carpeta, fs::Permissions::from_mode(0o700));
        }
        for ruta in &self.limpiar {
            let _ = fs::remove_dir_all(ruta);
        }
    }
}

/// La carpeta `aparte` de un escenario, con `bin/xdg-open` de prueba y una
/// carpeta `vacia` para el `PATH` sin navegador.
fn aparte() -> TempDir {
    let aparte = tempfile::tempdir().expect("crear la carpeta aparte");
    let bin = aparte.path().join("bin");
    fs::create_dir(&bin).unwrap();
    fs::create_dir(aparte.path().join("vacia")).unwrap();
    let xdg_open = bin.join("xdg-open");
    let registro = aparte.path().join("navegador.log");
    let guion = format!("#!/bin/sh\necho \"$0 $*\" >> '{}'\n", registro.display());
    fs::write(&xdg_open, guion).unwrap();
    fs::set_permissions(&xdg_open, fs::Permissions::from_mode(0o755)).unwrap();
    aparte
}

/// El final de casi todo guion de `sostener`, avisar y quedar dormido.
const DORMIR: &str = "echo listo && exec sleep infinity";

/// Lanza un proceso en un espacio de nombres de usuario propio, más los de
/// `espacios`, que corre `guion` para sostenerlos. Así las pruebas cambian
/// discos y redes sin privilegios. El guion escribe `listo` cuando terminó de
/// preparar todo y después queda vivo, casi siempre con `DORMIR`.
fn sostener(espacios: &[&str], guion: &str) -> Child {
    let mut proceso = Command::new("unshare")
        .args(["--user", "--map-root-user"])
        .args(espacios)
        .args(["sh", "-c", guion])
        .stdout(Stdio::piped())
        .spawn()
        .expect("lanzar unshare");
    let mut linea = String::new();
    BufReader::new(proceso.stdout.take().unwrap())
        .read_line(&mut linea)
        .expect("leer la salida de unshare");
    assert_eq!(
        linea, "listo\n",
        "no se pudo preparar el espacio de nombres"
    );
    proceso
}

/// La ruta de un programa en el `PATH` de las pruebas, que no es el de `osr`.
fn programa(nombre: &str) -> PathBuf {
    std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
        .map(|carpeta| carpeta.join(nombre))
        .find(|ruta| ruta.is_file())
        .unwrap_or_else(|| panic!("falta {nombre} en el PATH"))
}

/// Manda SIGTERM a un proceso; `Child::kill` mandaría SIGKILL.
fn sigterm(proceso: &Child) {
    let estado = Command::new("kill")
        .args(["-TERM", &proceso.id().to_string()])
        .status()
        .expect("ejecutar kill");
    assert!(estado.success(), "kill -TERM falló");
}

/// Espera hasta `limite` a que el proceso termine y devuelve su código.
fn esperar(proceso: &mut Child, limite: Duration) -> Option<i32> {
    let inicio = Instant::now();
    while inicio.elapsed() < limite {
        if let Some(estado) = proceso.try_wait().expect("consultar el proceso") {
            return Some(estado.code().unwrap_or(-1));
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    None
}

/// Todas las rutas bajo `raiz`, las carpetas con `/` final y sin contenido,
/// los archivos con sus bytes.
fn rutas(raiz: &Path) -> BTreeMap<String, Option<Vec<u8>>> {
    fn recorrer(raiz: &Path, carpeta: &Path, rutas: &mut BTreeMap<String, Option<Vec<u8>>>) {
        let entradas =
            fs::read_dir(carpeta).unwrap_or_else(|e| panic!("leer {}: {e}", carpeta.display()));
        for entrada in entradas {
            let ruta = entrada.expect("leer una entrada").path();
            let relativa = ruta
                .strip_prefix(raiz)
                .unwrap()
                .to_string_lossy()
                .into_owned();
            if ruta.is_dir() {
                rutas.insert(format!("{relativa}/"), None);
                recorrer(raiz, &ruta, rutas);
            } else {
                rutas.insert(relativa, Some(fs::read(&ruta).expect("leer un archivo")));
            }
        }
    }
    let mut todas = BTreeMap::new();
    recorrer(raiz, raiz, &mut todas);
    todas
}
