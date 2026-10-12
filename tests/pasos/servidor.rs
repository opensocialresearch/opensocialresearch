//! Pasos propios del módulo servidor (SRV).

use std::collections::BTreeSet;
use std::fs;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::time::{Duration, Instant};

use cucumber::gherkin::Step;
use cucumber::{given, then, when};

use super::orden::direccion_de;
use super::{DORMIR, Mundo, sostener};

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

/// Una dirección de `/proc/net/tcp` o `tcp6`, en hexadecimal con cada
/// palabra de 32 bits en el orden de la máquina, escrita como en el escenario.
fn direccion_de_proc(hex: &str) -> String {
    let (ip, puerto) = hex.split_once(':').expect("dirección de /proc/net");
    let puerto = u16::from_str_radix(puerto, 16).expect("puerto hexadecimal");
    let palabras: Vec<[u8; 4]> = (0..ip.len() / 8)
        .map(|i| {
            let palabra = u32::from_str_radix(&ip[8 * i..8 * i + 8], 16).unwrap();
            palabra.to_ne_bytes()
        })
        .collect();
    match palabras.as_slice() {
        [v4] => format!("{}:{puerto}", Ipv4Addr::from(*v4)),
        _ => format!(
            "[{}]:{puerto}",
            Ipv6Addr::from(<[u8; 16]>::try_from(palabras.concat()).unwrap())
        ),
    }
}

/// Los sockets en escucha de un proceso, de sus descriptores y de las tablas
/// de `/proc/<pid>/net`, que muestran su propio espacio de nombres de red.
fn sockets_en_escucha(pid: u32) -> BTreeSet<String> {
    let inodos: BTreeSet<String> = fs::read_dir(format!("/proc/{pid}/fd"))
        .expect("leer los descriptores de osr")
        .filter_map(|fd| fs::read_link(fd.ok()?.path()).ok())
        .filter_map(|destino| {
            let destino = destino.to_string_lossy().into_owned();
            Some(
                destino
                    .strip_prefix("socket:[")?
                    .strip_suffix(']')?
                    .to_string(),
            )
        })
        .collect();
    let mut sockets = BTreeSet::new();
    for tabla in ["tcp", "tcp6"] {
        let texto = fs::read_to_string(format!("/proc/{pid}/net/{tabla}")).unwrap_or_default();
        for fila in texto.lines().skip(1) {
            let campos: Vec<&str> = fila.split_whitespace().collect();
            if campos[3] == "0A" && inodos.contains(campos[9]) {
                sockets.insert(direccion_de_proc(campos[1]));
            }
        }
    }
    sockets
}

#[then("los sockets en escucha del proceso osr son exactamente estos")]
fn sockets_exactos(mundo: &mut Mundo, step: &Step) {
    let tabla = step.table.as_ref().expect("falta la tabla de direcciones");
    let esperados: BTreeSet<String> = tabla.rows[1..]
        .iter()
        .map(|f| mundo.resolver(&f[0]))
        .collect();
    let pid = mundo.osr.as_ref().expect("falta «se ejecuta»").id();
    assert_eq!(sockets_en_escucha(pid), esperados);
}

/// Una interfaz `dummy` con una sola dirección /32, sin ninguna ruta hacia
/// fuera, en una red propia donde también queda levantado el loopback.
#[given(
    expr = "el sistema de prueba tiene una interfaz de red local con la dirección {string} y sin rutas de salida"
)]
fn interfaz_local(mundo: &mut Mundo, direccion: String) {
    let guion = format!(
        "ip link set lo up && ip link add local0 type dummy && \
         ip addr add {direccion}/32 dev local0 && ip link set local0 up && {DORMIR}"
    );
    mundo.red = Some(sostener(&["--net"], &guion));
}

#[given("el sistema de prueba no tiene IPv6 de loopback")]
fn sin_ipv6(mundo: &mut Mundo) {
    let guion = format!(
        "ip link set lo up && echo 1 > /proc/sys/net/ipv6/conf/lo/disable_ipv6 && {DORMIR}"
    );
    mundo.red = Some(sostener(&["--net"], &guion));
}

/// El rango efímero queda en el puerto 40000, y el mismo proceso que sostiene
/// la red lo ocupa en `[::1]`.
#[given(
    expr = "el rango de puertos efímeros del sistema de prueba es un solo puerto, ocupado en {string}"
)]
fn un_solo_puerto(mundo: &mut Mundo, direccion: String) {
    assert_eq!(direccion, "[::1]");
    let ocupar = "import socket, time\n\
                  s = socket.socket(socket.AF_INET6)\n\
                  s.bind(('::1', 40000))\n\
                  s.listen()\n\
                  print('listo', flush=True)\n\
                  time.sleep(1e9)";
    let guion = format!(
        "ip link set lo up && echo '40000 40000' > /proc/sys/net/ipv4/ip_local_port_range && \
         exec python3 -c \"{ocupar}\""
    );
    mundo.red = Some(sostener(&["--net"], &guion));
}

/// La conexión se intenta desde dentro de la red de prueba, porque fuera de
/// ella esa dirección iría a la red real.
#[when(expr = "la prueba intenta una conexión TCP a {string}")]
fn intenta_conexion(mundo: &mut Mundo, destino: String) {
    let destino = mundo.resolver(&destino);
    let (ip, puerto) = destino.rsplit_once(':').unwrap();
    let mut comando = mundo.espacio().expect("falta la red de prueba");
    let guion = format!(
        "import socket\ntry:\n socket.create_connection(('{ip}',{puerto}),timeout=2)\n print('aceptada')\n\
         except ConnectionRefusedError:\n print('rechazada')\nexcept OSError as e:\n print(e)"
    );
    let salida = comando
        .args([super::programa("python3").to_str().unwrap(), "-c", &guion])
        .output()
        .unwrap();
    mundo.conexion = Some(String::from_utf8_lossy(&salida.stdout).trim().to_string());
}

#[then("la conexión es rechazada")]
fn conexion_rechazada(mundo: &mut Mundo) {
    assert_eq!(mundo.conexion.as_deref(), Some("rechazada"));
}

#[then(expr = "el registro técnico tiene una línea que contiene {string}")]
fn registro_contiene(mundo: &mut Mundo, texto: String) {
    let registro = mundo.registro_tecnico();
    assert!(registro.lines().any(|l| l.contains(&texto)), "{registro}");
}

/// Arranca `osr` las veces pedidas, anota el puerto y el token de cada
/// arranque y lo cierra de forma ordenada antes del siguiente.
#[when(
    regex = r"^osr se arranca (\d+) veces seguidas con `(osr --sin-navegador)` y se cierra de forma ordenada después de cada arranque$"
)]
fn arranca_varias_veces(mundo: &mut Mundo, veces: usize, orden: String) {
    for _ in 0..veces {
        mundo.arrancar_y_cerrar(&orden);
    }
}

#[then(expr = "entre los {int} puertos anotados hay al menos {int} distintos")]
fn puertos_distintos(mundo: &mut Mundo, total: usize, distintos: usize) {
    assert_eq!(mundo.anotados.len(), total);
    let puertos: BTreeSet<u16> = mundo.anotados.iter().map(|(p, _)| *p).collect();
    assert!(puertos.len() >= distintos, "{puertos:?}");
}
