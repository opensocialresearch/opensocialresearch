//! Pasos que preparan y revisan carpetas y archivos.

use std::collections::BTreeSet;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use cucumber::gherkin::Step;
use cucumber::{given, then};

use super::{DORMIR, Mundo, rutas, sostener};

fn esta_vacia(carpeta: &Path) -> bool {
    fs::read_dir(carpeta)
        .unwrap_or_else(|e| panic!("leer {}: {e}", carpeta.display()))
        .next()
        .is_none()
}

/// Los datos de prueba de PRY-17, en `tests/datos/` del repositorio.
fn datos_de_prueba() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/datos")
}

/// Las carpetas de la columna «Al crear» de PRY-02, más `.osr/`, sin la cual
/// `osr abrir` no reconoce el proyecto. Git no guarda carpetas vacías.
const CARPETAS_AL_CREAR: [&str; 5] = [
    "bitacora",
    "participantes",
    "identificable/consentimientos",
    "identificable/entrevistas",
    ".osr",
];

/// Copia `origen` en `destino`, que no existe, carpeta por carpeta.
fn copiar_arbol(origen: &Path, destino: &Path) {
    fs::create_dir(destino).unwrap_or_else(|e| panic!("crear {}: {e}", destino.display()));
    for entrada in fs::read_dir(origen).expect("leer los datos de prueba") {
        let entrada = entrada.expect("leer una entrada");
        let hacia = destino.join(entrada.file_name());
        if entrada.file_type().expect("tipo de la entrada").is_dir() {
            copiar_arbol(&entrada.path(), &hacia);
        } else {
            fs::copy(entrada.path(), &hacia).expect("copiar un archivo");
        }
    }
}

/// Si `relativa` es un original de solo lectura según SRV-13, el audio
/// `identificable/entrevistas/<id>/<id>.<ext>` o algo bajo `origen/`.
fn es_original(relativa: &Path) -> bool {
    let partes: Vec<&str> = relativa.iter().filter_map(|p| p.to_str()).collect();
    match partes.as_slice() {
        ["identificable", "entrevistas", _, "origen", ..] => true,
        ["identificable", "entrevistas", id, nombre] => nombre
            .strip_prefix(&format!("{id}."))
            .is_some_and(|ext| !ext.is_empty() && !ext.contains('.')),
        _ => false,
    }
}

/// Fija los permisos de SRV-13 bajo `carpeta`: carpetas 0700, originales
/// 0400 y los demás archivos 0600.
fn fijar_permisos(raiz: &Path, carpeta: &Path) {
    for entrada in fs::read_dir(carpeta).expect("leer el proyecto") {
        let ruta = entrada.expect("leer una entrada").path();
        if ruta.is_dir() {
            fijar_permisos(raiz, &ruta);
            continue;
        }
        let modo = if es_original(ruta.strip_prefix(raiz).unwrap()) {
            0o400
        } else {
            0o600
        };
        fs::set_permissions(&ruta, fs::Permissions::from_mode(modo)).expect("fijar permisos");
    }
    fs::set_permissions(carpeta, fs::Permissions::from_mode(0o700)).expect("fijar permisos");
}

/// Copia el proyecto de prueba de PRY-17 en `{proyecto}` y le devuelve lo
/// que git no guarda, las carpetas vacías y los permisos (D-38). No crea el
/// índice ni el bloqueo, que son de `osr`.
#[given("una copia del proyecto de prueba")]
fn copia_del_proyecto(mundo: &mut Mundo) {
    assert!(!mundo.proyecto.exists());
    copiar_arbol(&datos_de_prueba().join("proyecto"), &mundo.proyecto);
    for carpeta in CARPETAS_AL_CREAR {
        fs::create_dir_all(mundo.proyecto.join(carpeta)).expect("crear una carpeta");
    }
    fijar_permisos(&mundo.proyecto, &mundo.proyecto);
}

#[given("los datos de prueba del repositorio")]
fn datos_del_repositorio(_mundo: &mut Mundo) {
    assert!(datos_de_prueba().is_dir(), "faltan los datos de prueba");
}

#[then(regex = r#"^existe el archivo "([^"/]+)" en la raíz de los datos de prueba$"#)]
fn archivo_en_los_datos(_mundo: &mut Mundo, archivo: String) {
    let ruta = datos_de_prueba().join(&archivo);
    assert!(ruta.is_file(), "no existe el archivo {}", ruta.display());
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

// Solo carpetas bajo `{tmp}`. Las del proyecto, como `origen/` de TRA-10,
// llegan con sus requisitos, y sus escenarios quedan omitidos (D-20).
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

#[then("el registro de eventos no recibió líneas nuevas")]
fn sin_eventos_nuevos(mundo: &mut Mundo) {
    let nuevas = eventos(mundo).split_off(mundo.lineas_de_eventos);
    assert!(nuevas.is_empty(), "{nuevas:?}");
}

#[given("un proyecto nuevo creado con `osr nuevo`")]
fn proyecto_nuevo(mundo: &mut Mundo) {
    let proyecto = mundo.proyecto.to_string_lossy().into_owned();
    mundo.lanzar(&format!("osr nuevo \"{proyecto}\" --sin-navegador"));
}

/// Los bytes de un archivo, que tienen que ser UTF-8 sin BOM, y su TOML.
fn toml_de(mundo: &Mundo, archivo: &str) -> toml::Table {
    let bytes = fs::read(mundo.ruta(archivo)).expect("leer el TOML");
    assert!(!bytes.starts_with(b"\xEF\xBB\xBF"), "empieza con BOM");
    let texto = String::from_utf8(bytes).expect("UTF-8");
    texto.parse().expect("TOML 1.0 válido")
}

/// El valor de una clave con puntos, si está.
fn valor_de<'a>(tabla: &'a toml::Table, clave: &str) -> Option<&'a toml::Value> {
    let (primera, resto) = clave.split_once('.').unwrap_or((clave, ""));
    let valor = tabla.get(primera)?;
    match resto {
        "" => Some(valor),
        resto => valor_de(valor.as_table()?, resto),
    }
}

/// Una celda de la tabla del escenario, que es un valor escrito en TOML.
fn valor_toml(celda: &str) -> toml::Value {
    let tabla: toml::Table = format!("v = {celda}").parse().expect("celda en TOML");
    tabla["v"].clone()
}

#[then(regex = r#"^"([^"]+)" es TOML 1\.0 en UTF-8 sin BOM$"#)]
fn es_toml(mundo: &mut Mundo, archivo: String) {
    toml_de(mundo, &archivo);
}

#[then(regex = r#"^"([^"]+)" tiene estas claves con estos valores$"#)]
fn claves_con_valores(mundo: &mut Mundo, archivo: String, step: &Step) {
    let tabla = toml_de(mundo, &archivo);
    for fila in &step.table.as_ref().expect("falta la tabla").rows[1..] {
        let actual = valor_de(&tabla, &fila[0]);
        assert_eq!(actual, Some(&valor_toml(&fila[1])), "{}", fila[0]);
    }
}

#[then(
    regex = r#"^cada una de estas claves de "([^"]+)", si está en el archivo, tiene este valor$"#
)]
fn claves_si_estan(mundo: &mut Mundo, archivo: String, step: &Step) {
    let tabla = toml_de(mundo, &archivo);
    for fila in &step.table.as_ref().expect("falta la tabla").rows[1..] {
        if let Some(actual) = valor_de(&tabla, &fila[0]) {
            assert_eq!(actual, &valor_toml(&fila[1]), "{}", fila[0]);
        }
    }
}

#[then(
    regex = r#"^la clave "([^"]+)" de "([^"]+)" es una marca de tiempo ISO 8601 con desplazamiento y precisión de segundos$"#
)]
fn es_marca_de_tiempo(mundo: &mut Mundo, clave: String, archivo: String) {
    let tabla = toml_de(mundo, &archivo);
    let fecha = valor_de(&tabla, &clave)
        .and_then(toml::Value::as_datetime)
        .unwrap_or_else(|| panic!("{clave} no es una fecha y hora"));
    let hora = fecha.time.expect("sin hora");
    assert!(fecha.date.is_some() && fecha.offset.is_some(), "{fecha}");
    assert_eq!(hora.nanosecond, None, "sin fracción de segundo: {fecha}");
    assert_eq!(fecha.to_string().len(), 25, "{fecha}");
}

fn proyecto_toml(mundo: &Mundo) -> (std::path::PathBuf, String) {
    let ruta = mundo.ruta("proyecto.toml");
    let texto = fs::read_to_string(&ruta).expect("leer proyecto.toml");
    (ruta, texto)
}

/// Reemplaza en `proyecto.toml` las líneas que `cambiar` devuelve distintas
/// y exige que haya cambiado alguna.
fn cambiar_lineas(mundo: &Mundo, cambiar: impl Fn(usize, &str) -> Option<String>) {
    let (ruta, texto) = proyecto_toml(mundo);
    let mut cambio = false;
    let mut nuevo = String::new();
    for (indice, linea) in texto.lines().enumerate() {
        if let Some(otra) = cambiar(indice + 1, linea) {
            cambio = true;
            nuevo.push_str(&otra);
        } else {
            nuevo.push_str(linea);
            nuevo.push('\n');
        }
    }
    assert!(cambio, "no cambió ninguna línea de proyecto.toml");
    fs::write(ruta, nuevo).expect("escribir proyecto.toml");
}

// Solo `proyecto.toml` y sus claves de primer nivel, que son las que lee
// PRY-03. Las demás alteraciones llegan con sus requisitos (D-20).
#[given(
    regex = r#"^con la app cerrada se cambia en "proyecto\.toml" la clave "([a-z_]+)" a (.+)$"#
)]
fn cambia_clave(mundo: &mut Mundo, clave: String, valor: String) {
    let inicio = format!("{clave} = ");
    cambiar_lineas(mundo, |_, linea| {
        linea
            .starts_with(&inicio)
            .then(|| format!("{inicio}{valor}\n"))
    });
}

#[given(regex = r#"^con la app cerrada se quita de "proyecto\.toml" la clave "([a-z_]+)"$"#)]
fn quita_clave(mundo: &mut Mundo, clave: String) {
    let inicio = format!("{clave} = ");
    cambiar_lineas(mundo, |_, linea| {
        linea.starts_with(&inicio).then(String::new)
    });
}

#[given(
    regex = r#"^con la app cerrada se reemplaza la línea (\d+) de "proyecto\.toml" por "(.*)"$"#
)]
fn reemplaza_linea(mundo: &mut Mundo, numero: usize, texto: String) {
    cambiar_lineas(mundo, |indice, _| {
        (indice == numero).then(|| format!("{texto}\n"))
    });
}

// Solo `proyecto.toml` y los archivos de la carpeta de una entrevista (D-20).
#[given(
    regex = r#"^con la app cerrada se borra el archivo "(proyecto\.toml|identificable/entrevistas/[A-Z0-9]+/[^"/]+)"$"#
)]
fn borra_archivo(mundo: &mut Mundo, archivo: String) {
    let ruta = mundo.ruta(&archivo);
    fs::remove_file(&ruta).unwrap_or_else(|e| panic!("borrar {}: {e}", ruta.display()));
}

#[given("no hay ninguna preparación adicional")]
fn sin_preparacion(_mundo: &mut Mundo) {}
