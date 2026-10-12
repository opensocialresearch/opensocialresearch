//! Pasos que preparan y revisan carpetas y archivos.

use std::collections::BTreeSet;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use cucumber::gherkin::Step;
use cucumber::{given, then};

use super::{DORMIR, Mundo, rutas, sostener};

fn esta_vacia(carpeta: &Path) -> bool {
    fs::read_dir(carpeta)
        .unwrap_or_else(|e| panic!("leer {}: {e}", carpeta.display()))
        .next()
        .is_none()
}

/// Los datos de prueba de PRY-17 todavía no existen, así que por ahora el
/// proyecto de prueba es uno vacío, creado en `{proyecto}` con `osr nuevo` y
/// cerrado de forma ordenada (D-27).
#[given("una copia del proyecto de prueba")]
fn copia_del_proyecto(mundo: &mut Mundo) {
    assert!(!mundo.proyecto.exists());
    mundo.crear_cerrado();
}

#[given("se tomó la huella de cada archivo del proyecto")]
fn huellas_del_proyecto(mundo: &mut Mundo) {
    mundo.huellas = rutas(&mundo.proyecto);
}

#[then("ningún archivo del proyecto cambió respecto de las huellas tomadas")]
fn proyecto_sin_cambios(mundo: &mut Mundo) {
    assert_eq!(rutas(&mundo.proyecto), mundo.huellas);
}

#[given(expr = "una carpeta temporal vacía {string}")]
fn carpeta_temporal(mundo: &mut Mundo, tmp: String) {
    assert!(esta_vacia(&mundo.ruta(&tmp)));
}

#[given(expr = "existe la carpeta vacía {string}")]
fn existe_carpeta_vacia(mundo: &mut Mundo, carpeta: String) {
    let carpeta = mundo.ruta(&carpeta);
    fs::create_dir_all(&carpeta).expect("crear la carpeta");
    assert!(esta_vacia(&carpeta), "{} no está vacía", carpeta.display());
}

/// Deja la ruta sin existir y la anota para borrarla al final del escenario,
/// porque el escenario la va a crear.
#[given(expr = "no existe {string}")]
fn sin_ruta(mundo: &mut Mundo, ruta: String) {
    let ruta = mundo.ruta(&ruta);
    if ruta.exists() {
        fs::remove_dir_all(&ruta).expect("borrar la ruta");
    }
    mundo.limpiar.push(ruta);
}

#[given(expr = "existe el archivo {string} con el texto {string}")]
fn archivo_con_texto(mundo: &mut Mundo, archivo: String, texto: String) {
    let archivo = mundo.ruta(&archivo);
    fs::create_dir_all(archivo.parent().unwrap()).expect("crear la carpeta del archivo");
    fs::write(&archivo, texto).expect("escribir el archivo");
}

// Solo carpetas bajo `{tmp}`. Las de los datos de prueba no existen hasta
// PRY-17, y sus escenarios quedan omitidos (D-20).
#[given(regex = r#"^se tomó la huella de cada archivo de "(\{tmp\}[^"]*)"$"#)]
fn tomar_huellas(mundo: &mut Mundo, carpeta: String) {
    mundo.huellas = rutas(&mundo.ruta(&carpeta));
}

/// Monta un tmpfs del tamaño pedido en un espacio de nombres de usuario y de
/// montaje propio, sin privilegios. `osr` corre dentro de ese espacio y los
/// pasos miran la carpeta a través de `/proc/<pid>/root`.
#[given(expr = "un disco de prueba montado en {string} con solo {int} KiB libres")]
fn disco_de_prueba(mundo: &mut Mundo, carpeta: String, kib: u32) {
    let carpeta = mundo.ruta(&carpeta);
    fs::create_dir_all(&carpeta).expect("crear el punto de montaje");
    let montar = format!(
        "mount -t tmpfs -o size={kib}k tmpfs '{}' && {DORMIR}",
        carpeta.display()
    );
    let disco = sostener(&["--mount", "--propagation", "private"], &montar);
    mundo.disco = Some(disco);
}

#[given(expr = "la carpeta {string} está vacía")]
fn carpeta_esta_vacia(mundo: &mut Mundo, carpeta: String) {
    assert!(esta_vacia(&mundo.ruta(&carpeta)));
}

#[given(expr = "la carpeta {string} queda con permisos {word}")]
fn con_permisos(mundo: &mut Mundo, carpeta: String, permisos: String) {
    let carpeta = mundo.ruta(&carpeta);
    let modo = u32::from_str_radix(&permisos, 8).expect("permisos en octal");
    fs::set_permissions(&carpeta, fs::Permissions::from_mode(modo)).expect("cambiar permisos");
    mundo.restaurar_permisos.push(carpeta);
}

#[then(expr = "la carpeta {string} tiene exactamente estas rutas")]
fn tiene_exactamente(mundo: &mut Mundo, carpeta: String, step: &Step) {
    let tabla = step.table.as_ref().expect("falta la tabla de rutas");
    let esperadas: BTreeSet<String> = tabla.rows[1..].iter().map(|f| f[0].clone()).collect();
    let presentes: BTreeSet<String> = rutas(&mundo.ruta(&carpeta)).into_keys().collect();
    assert_eq!(presentes, esperadas);
}

#[then(expr = "existe el archivo {string}")]
fn existe_archivo(mundo: &mut Mundo, archivo: String) {
    let archivo = mundo.ruta(&archivo);
    assert!(
        archivo.is_file(),
        "no existe el archivo {}",
        archivo.display()
    );
}

#[then(expr = "no existe {string}")]
fn no_existe(mundo: &mut Mundo, ruta: String) {
    let ruta = mundo.ruta(&ruta);
    assert!(!ruta.exists(), "existe {}", ruta.display());
}

#[then(expr = "la carpeta {string} sigue vacía")]
fn sigue_vacia(mundo: &mut Mundo, carpeta: String) {
    let carpeta = mundo.ruta(&carpeta);
    assert!(esta_vacia(&carpeta), "{:?}", rutas(&carpeta).keys());
}

#[then(expr = "ningún archivo de {string} cambió respecto de las huellas tomadas")]
fn sin_cambios(mundo: &mut Mundo, carpeta: String) {
    assert_eq!(rutas(&mundo.ruta(&carpeta)), mundo.huellas);
}

#[then(expr = "la clave {string} de {string} vale {string}")]
fn clave_vale(mundo: &mut Mundo, clave: String, archivo: String, valor: String) {
    let texto = fs::read_to_string(mundo.ruta(&archivo)).expect("leer el TOML");
    let tabla: toml::Table = texto.parse().expect("TOML válido");
    let mut actual = &toml::Value::Table(tabla);
    for parte in clave.split('.') {
        actual = actual
            .get(parte)
            .unwrap_or_else(|| panic!("falta la clave {clave}"));
    }
    let actual = actual
        .as_str()
        .map_or_else(|| actual.to_string(), String::from);
    assert_eq!(actual, mundo.resolver(&valor));
}

#[then(expr = "el archivo {string} tiene exactamente {int} línea(s)")]
fn cantidad_de_lineas(mundo: &mut Mundo, archivo: String, lineas: usize) {
    let texto = fs::read_to_string(mundo.ruta(&archivo)).expect("leer el archivo");
    assert_eq!(texto.lines().count(), lineas);
}

#[then(expr = "la última línea del registro de eventos es de tipo {string}")]
fn ultimo_evento(mundo: &mut Mundo, tipo: String) {
    let texto =
        fs::read_to_string(mundo.ruta("bitacora/registro.jsonl")).expect("leer el registro");
    let linea = texto.lines().last().expect("el registro está vacío");
    let evento: serde_json::Value = serde_json::from_str(linea).expect("JSON válido");
    assert_eq!(evento["tipo"], tipo.as_str());
}

fn eventos(mundo: &Mundo) -> Vec<serde_json::Value> {
    let texto =
        fs::read_to_string(mundo.ruta("bitacora/registro.jsonl")).expect("leer el registro");
    texto
        .lines()
        .map(|linea| serde_json::from_str(linea).expect("JSON válido"))
        .collect()
}

#[given("se guardó la cantidad de líneas del registro de eventos")]
fn guarda_cantidad(mundo: &mut Mundo) {
    mundo.lineas_de_eventos = eventos(mundo).len();
}

/// Cuenta solo las líneas nuevas de ese tipo, porque cada apertura anota
/// además su `verificacion` (D-17, D-28).
#[then(regex = r#"^el registro de eventos tiene exactamente una línea nueva de tipo "([a-z_]+)"$"#)]
fn una_linea_nueva(mundo: &mut Mundo, tipo: String) {
    let nuevas = eventos(mundo).split_off(mundo.lineas_de_eventos);
    let de_ese_tipo = nuevas.iter().filter(|e| e["tipo"] == tipo.as_str()).count();
    assert_eq!(de_ese_tipo, 1, "{nuevas:?}");
}
