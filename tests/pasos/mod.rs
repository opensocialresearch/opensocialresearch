//! Definiciones de los pasos de los escenarios y el mundo que comparten.
//!
//! Las frases son las de los `.feature`, que son contrato: se reutilizan en
//! otros requisitos y no se cambian para que una prueba pase.

mod archivos;
mod orden;

use std::collections::BTreeMap;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
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
    /// Código de salida de `osr`, cuando ya terminó.
    codigo: Option<i32>,
    /// Proceso que sostiene el espacio de nombres del disco de prueba.
    disco: Option<Child>,
    /// Contenido de cada ruta tomado por «se tomó la huella».
    huellas: BTreeMap<String, Option<Vec<u8>>>,
    /// Carpetas a las que el escenario les quitó permisos.
    restaurar_permisos: Vec<PathBuf>,
    /// Rutas fuera de `{tmp}` que el escenario crea y hay que borrar al final.
    limpiar: Vec<PathBuf>,
}

impl Mundo {
    fn nuevo() -> Mundo {
        let tmp = tempfile::tempdir().expect("crear {tmp}");
        Mundo {
            proyecto: tmp.path().join("estudio"),
            tmp,
            salidas: None,
            osr: None,
            codigo: None,
            disco: None,
            huellas: BTreeMap::new(),
            restaurar_permisos: Vec::new(),
            limpiar: Vec::new(),
        }
    }

    fn tmp(&self) -> &Path {
        self.tmp.path()
    }

    /// Reemplaza los marcadores de las convenciones de `proyecto.feature`.
    fn resolver(&self, texto: &str) -> String {
        let mut texto = texto.replace("{201 veces la letra a}", &"a".repeat(201));
        texto = texto.replace("{Uno, salto de línea, Dos}", "Uno\nDos");
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
        match &self.disco {
            Some(disco) => Path::new(&format!("/proc/{}/root", disco.id()))
                .join(ruta.strip_prefix("/").expect("ruta absoluta")),
            None => ruta,
        }
    }
}

impl Drop for Mundo {
    fn drop(&mut self) {
        if let Some(mut osr) = self.osr.take() {
            orden::terminar(&mut osr);
        }
        if let Some(mut disco) = self.disco.take() {
            let _ = disco.kill();
            let _ = disco.wait();
        }
        for carpeta in &self.restaurar_permisos {
            let _ = fs::set_permissions(carpeta, fs::Permissions::from_mode(0o700));
        }
        for ruta in &self.limpiar {
            let _ = fs::remove_dir_all(ruta);
        }
    }
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
