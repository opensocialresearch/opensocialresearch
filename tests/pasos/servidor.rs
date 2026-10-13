//! Pasos propios del módulo servidor (SRV).

use std::collections::BTreeSet;
use std::fs;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use cucumber::gherkin::Step;
use cucumber::{given, then, when};

use super::http::{Pedido, Respuesta};
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

#[given(
    regex = r"^osr se arrancó con `(osr --sin-navegador)` y se cerró de forma ordenada, anotando su token como \{token anterior\}$"
)]
fn arranco_y_cerro(mundo: &mut Mundo, orden: String) {
    mundo.arrancar_y_cerrar(&orden);
    mundo.token_anterior = mundo.token.clone();
}

#[then(expr = "los {int} tokens anotados son distintos")]
fn tokens_distintos(mundo: &mut Mundo, total: usize) {
    let tokens: BTreeSet<&String> = mundo.anotados.iter().map(|(_, t)| t).collect();
    assert_eq!((mundo.anotados.len(), tokens.len()), (total, total));
}

#[then(expr = "cada token anotado tiene {int} caracteres")]
fn largo_de_cada_token(mundo: &mut Mundo, largo: usize) {
    for (_, token) in &mundo.anotados {
        assert_eq!(token.chars().count(), largo, "{token}");
    }
}

/// Los caracteres de una lista como "A-Z a-z 0-9 - _", con rangos y sueltos
/// separados por espacios.
fn caracteres(lista: &str) -> BTreeSet<char> {
    let mut permitidos = BTreeSet::new();
    for parte in lista.split(' ') {
        match parte.chars().collect::<Vec<_>>().as_slice() {
            [desde, '-', hasta] => permitidos.extend(*desde..=*hasta),
            [suelto] => {
                permitidos.insert(*suelto);
            }
            _ => panic!("no se entiende {parte:?}"),
        }
    }
    permitidos
}

#[then(expr = "cada token anotado tiene solo caracteres de {string}")]
fn alfabeto_de_cada_token(mundo: &mut Mundo, lista: String) {
    let permitidos = caracteres(&lista);
    assert_eq!(permitidos.len(), 64);
    for (_, token) in &mundo.anotados {
        assert!(token.chars().all(|c| permitidos.contains(&c)), "{token}");
    }
}

/// El `xdg-open` de prueba que ponen todos los escenarios ya anota cómo lo
/// llamaron, en `navegador.log` de la carpeta aparte.
#[given(
    "el navegador predeterminado del sistema de prueba es un programa que anota cómo lo llamaron"
)]
fn navegador_que_anota(mundo: &mut Mundo) {
    assert!(!mundo.sin_navegador);
    assert!(mundo.aparte.path().join("bin/xdg-open").is_file());
}

/// Lee `/proc/*/cmdline` cada 10 ms en un hilo aparte hasta que el escenario
/// lo detiene, y junta cada línea de órdenes distinta.
#[given("la prueba lee la línea de órdenes de todos los procesos del sistema cada 10 ms")]
fn lee_lineas_de_ordenes(mundo: &mut Mundo) {
    let seguir = Arc::new(AtomicBool::new(true));
    let sigue = Arc::clone(&seguir);
    let hilo = std::thread::spawn(move || {
        let mut leidas = BTreeSet::new();
        while sigue.load(Ordering::Relaxed) {
            for proceso in fs::read_dir("/proc").into_iter().flatten().flatten() {
                if let Ok(linea) = fs::read(proceso.path().join("cmdline")) {
                    leidas.insert(String::from_utf8_lossy(&linea).replace('\0', " "));
                }
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        leidas
    });
    mundo.lector_de_ordenes = Some((seguir, hilo));
}

#[when(expr = "pasan {int} s")]
fn pasan(_mundo: &mut Mundo, segundos: u64) {
    std::thread::sleep(Duration::from_secs(segundos));
}

#[then("ninguna línea de órdenes leída contiene el token de la sesión")]
fn ninguna_linea_con_el_token(mundo: &mut Mundo) {
    let (seguir, hilo) = mundo
        .lector_de_ordenes
        .take()
        .expect("falta el lector de órdenes");
    seguir.store(false, Ordering::Relaxed);
    let mut leidas = hilo.join().expect("el lector de órdenes falló");
    let token = mundo.token.clone().expect("osr no escribió su token");
    // El navegador de prueba dura muy poco para verlo cada 10 ms, así que se
    // suma su propia anotación de cómo lo llamaron, que además prueba que hubo
    // un llamado.
    let llamados = fs::read_to_string(mundo.aparte.path().join("navegador.log"))
        .expect("nadie llamó al navegador");
    leidas.insert(llamados);
    assert!(
        !leidas.iter().any(|l| l.contains(&token)),
        "el token apareció en una línea de órdenes"
    );
}

/// La definición de rutas del propio servidor, incluida tal cual (SRV-10).
#[path = "../../src/rutas.rs"]
mod rutas_del_servidor;

/// Un camino concreto para un patrón de axum, con un valor de prueba en cada
/// segmento variable como `{entrevista}`.
fn camino_concreto(patron: &str) -> String {
    patron
        .split('/')
        .map(|parte| if parte.starts_with('{') { "E01" } else { parte })
        .collect::<Vec<_>>()
        .join("/")
}

#[given("la lista completa de rutas que el servidor declara en su propia definición de rutas")]
fn lista_de_rutas(mundo: &mut Mundo) {
    mundo.lista_de_rutas = rutas_del_servidor::RUTAS
        .iter()
        .flat_map(|ruta| {
            ruta.metodos
                .iter()
                .map(|m| (m.to_string(), camino_concreto(ruta.camino)))
        })
        .collect();
    assert!(!mundo.lista_de_rutas.is_empty());
}

#[when(
    regex = r#"^se manda a cada ruta de la lista, con cada método que acepta, un pedido con Host propio, con "([^"]+)" con valor "([^"]+)" si el método no es GET ni HEAD, y sin cookie ni token$"#
)]
async fn a_cada_ruta(mundo: &mut Mundo, nombre: String, valor: String) {
    let valor = mundo.resolver(&valor);
    mundo.respuestas.clear();
    for (metodo, camino) in mundo.lista_de_rutas.clone() {
        let mut pedido = Pedido::nuevo(&metodo, mundo.direccion(&camino));
        if !["GET", "HEAD"].contains(&metodo.as_str()) {
            pedido = pedido.con_encabezado(&nombre, &valor);
        }
        let respuesta = pedido.mandar().await;
        mundo.respuestas.push((camino, respuesta));
    }
}

/// La verificación previa de CORS desde otro origen, a cada camino de la
/// lista (SRV-08.1).
#[when(
    regex = r#"^se manda a cada ruta de la lista un pedido OPTIONS con "(Origin)" con valor "([^"]+)" y "(Access-Control-Request-Method)" con valor "([^"]+)"$"#
)]
async fn verificacion_previa(
    mundo: &mut Mundo,
    origen: String,
    valor_origen: String,
    metodo: String,
    valor_metodo: String,
) {
    mundo.respuestas.clear();
    let caminos: BTreeSet<String> = mundo
        .lista_de_rutas
        .iter()
        .map(|(_, c)| c.clone())
        .collect();
    for camino in caminos {
        let respuesta = Pedido::nuevo("OPTIONS", mundo.direccion(&camino))
            .con_encabezado(&origen, &valor_origen)
            .con_encabezado(&metodo, &valor_metodo)
            .mandar()
            .await;
        mundo.respuestas.push((camino, respuesta));
    }
}

#[then(expr = "ninguna respuesta tiene un encabezado que empiece con {string}")]
fn ninguna_con_el_prefijo(mundo: &mut Mundo, prefijo: String) {
    assert!(
        !mundo.respuestas.is_empty(),
        "no hay respuestas que revisar"
    );
    let prefijo = prefijo.to_ascii_lowercase();
    for (camino, respuesta) in &mundo.respuestas {
        let con_prefijo: Vec<&String> = respuesta
            .encabezados
            .iter()
            .map(|(nombre, _)| nombre)
            .filter(|nombre| nombre.starts_with(&prefijo))
            .collect();
        assert!(con_prefijo.is_empty(), "{camino}: {con_prefijo:?}");
    }
}

#[then(expr = "cada respuesta tiene estado {int}")]
fn cada_estado(mundo: &mut Mundo, estado: u16) {
    for (camino, respuesta) in &mundo.respuestas {
        assert_eq!(respuesta.estado, estado, "{camino}");
    }
}

#[then("cada respuesta tiene estos encabezados")]
fn cada_encabezado(mundo: &mut Mundo, step: &Step) {
    let tabla = step.table.as_ref().expect("falta la tabla de encabezados");
    for (camino, respuesta) in &mundo.respuestas {
        for fila in &tabla.rows[1..] {
            assert_eq!(
                respuesta.encabezado(&fila[0]),
                Some(fila[1].as_str()),
                "{camino}"
            );
        }
    }
}

#[then(expr = "el cuerpo de cada respuesta de la familia {string} es el JSON")]
fn cuerpo_json_de_la_familia(mundo: &mut Mundo, familia: String, step: &Step) {
    let esperado: serde_json::Value =
        serde_json::from_str(step.docstring.as_ref().expect("falta el JSON")).unwrap();
    for (camino, respuesta) in mundo
        .respuestas
        .iter()
        .filter(|(c, _)| c.starts_with(&familia))
    {
        let cuerpo: serde_json::Value = serde_json::from_str(&respuesta.cuerpo)
            .unwrap_or_else(|e| panic!("{camino}: el cuerpo no es JSON ({e})"));
        assert_eq!(cuerpo, esperado, "{camino}");
    }
    mundo.familias_vistas.push(familia);
}

#[then(
    expr = "el cuerpo de cada respuesta de las demás familias es una página HTML que contiene {string}"
)]
fn cuerpo_html_de_las_demas(mundo: &mut Mundo, texto: String) {
    let demas = mundo
        .respuestas
        .iter()
        .filter(|(c, _)| !mundo.familias_vistas.iter().any(|f| c.starts_with(f)));
    for (camino, respuesta) in demas {
        es_pagina_con(respuesta, &texto, camino);
    }
}

#[then(expr = "el cuerpo de la respuesta es una página HTML que contiene {string}")]
fn cuerpo_html(mundo: &mut Mundo, texto: String) {
    let respuesta = mundo
        .respuesta
        .as_ref()
        .expect("falta «se manda el pedido»");
    es_pagina_con(respuesta, &texto, "");
}

fn es_pagina_con(respuesta: &Respuesta, texto: &str, camino: &str) {
    let tipo = respuesta.encabezado("content-type").unwrap_or("");
    assert!(tipo.starts_with("text/html"), "{camino}: {tipo}");
    assert!(
        respuesta.cuerpo.contains(texto),
        "{camino}: {}",
        respuesta.cuerpo
    );
}

/// El instante en que el proceso `pid` deja de correr, mirado cada 5 ms en
/// `/proc`, hasta 10 s. Un proceso terminado que nadie esperó queda como
/// zombi, con estado `Z`, y eso también cuenta como terminado.
fn mirar_el_fin(pid: u32) -> std::thread::JoinHandle<Option<Instant>> {
    std::thread::spawn(move || {
        let inicio = Instant::now();
        while inicio.elapsed() < Duration::from_secs(10) {
            let estado = std::fs::read_to_string(format!("/proc/{pid}/stat"));
            let terminado = match estado {
                Err(_) => true,
                Ok(linea) => linea
                    .rsplit_once(") ")
                    .is_some_and(|(_, resto)| resto.starts_with('Z') || resto.starts_with('X')),
            };
            if terminado {
                return Some(Instant::now());
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        None
    })
}

/// Anota cuándo se mandó la señal y cuántas líneas tenía el registro
/// técnico, y empieza a mirar cuándo termina el proceso, así la medida no
/// depende de cuándo le toca correr al paso siguiente.
#[when("se envía SIGTERM al proceso osr")]
fn envia_sigterm(mundo: &mut Mundo) {
    mundo.lineas_del_registro = mundo.registro_tecnico().lines().count();
    let osr = mundo.osr.as_ref().expect("falta «se ejecuta»");
    let pid = osr.id();
    super::sigterm(osr);
    mundo.senal_enviada = Some(Instant::now());
    mundo.fin_observado = Some(mirar_el_fin(pid));
}

#[then(expr = "osr termina con código de salida {int} a más tardar {int} s después")]
fn termina_a_tiempo(mundo: &mut Mundo, codigo: i32, segundos: u64) {
    let enviada = mundo.senal_enviada.expect("falta «se envía SIGTERM»");
    let fin = mundo
        .fin_observado
        .take()
        .expect("falta «se envía SIGTERM»")
        .join()
        .expect("el hilo que mira el fin");
    let fin = fin.expect("osr no terminó en 10 s");
    assert!(
        fin.duration_since(enviada) <= Duration::from_secs(segundos),
        "osr tardó {:?}",
        fin.duration_since(enviada)
    );
    let osr = mundo.osr.as_mut().expect("falta «se ejecuta»");
    let terminado = super::esperar(osr, Duration::from_secs(5));
    assert_eq!(terminado, Some(codigo), "osr no terminó con ese código");
    mundo.codigo = terminado;
}

#[then("el registro técnico tiene una línea nueva que anota el cierre")]
fn registro_anota_el_cierre(mundo: &mut Mundo) {
    let registro = mundo.registro_tecnico();
    let nuevas: Vec<&str> = registro.lines().skip(mundo.lineas_del_registro).collect();
    assert!(nuevas.iter().any(|l| l.contains("cierre")), "{nuevas:?}");
}

#[given(
    regex = r#"^a la lista se suman los caminos inexistentes "(/no-existe)", "(/api/no-existe)" y "(/recursos/no-existe\.js)"$"#
)]
fn suma_inexistentes(mundo: &mut Mundo, uno: String, dos: String, tres: String) {
    // Un camino inexistente no acepta ningún método, se le manda GET (D-34).
    for camino in [uno, dos, tres] {
        mundo.lista_de_rutas.push(("GET".to_string(), camino));
    }
}

/// Un pedido de la batería de SRV-10 y lo que respondió.
#[derive(Debug)]
pub struct Disparo {
    factor: String,
    metodo: String,
    camino: String,
    respuesta: Respuesta,
}

/// El valor de una celda de la batería para un método, o nada si la celda
/// dice que solo va cuando el método no es GET ni HEAD y lo es.
fn celda(texto: &str, metodo: &str) -> Option<String> {
    match texto.strip_suffix(" si el método no es GET ni HEAD") {
        Some(_) if ["GET", "HEAD"].contains(&metodo) => None,
        Some(valor) => Some(valor.to_string()),
        None => Some(texto.to_string()),
    }
}

#[when(
    "se manda la batería de pedidos con un solo factor inválido a cada ruta de la lista y con cada método que acepta"
)]
async fn bateria(mundo: &mut Mundo, step: &Step) {
    let tabla = step.table.as_ref().expect("falta la tabla de la batería");
    assert_eq!(
        tabla.rows[0],
        ["factor inválido", "token", "Host", "Origin"]
    );
    mundo.bateria.clear();
    for (metodo, camino) in mundo.lista_de_rutas.clone() {
        for fila in &tabla.rows[1..] {
            let mut pedido = Pedido::nuevo(&metodo, mundo.direccion(&camino))
                .con_encabezado("Host", &mundo.resolver(&fila[2]));
            match fila[1].as_str() {
                "válido" => {
                    let token = mundo.token.clone().expect("osr no escribió su token");
                    pedido = pedido.con_encabezado("Authorization", &format!("Bearer {token}"));
                }
                ninguno => assert_eq!(ninguno, "ninguno"),
            }
            if let Some(origen) = celda(&fila[3], &metodo) {
                pedido = pedido.con_encabezado("Origin", &mundo.resolver(&origen));
            }
            let respuesta = pedido.mandar().await;
            mundo.bateria.push(Disparo {
                factor: fila[0].clone(),
                metodo: metodo.clone(),
                camino: camino.clone(),
                respuesta,
            });
        }
    }
}

#[then(regex = r#"^las respuestas con factor inválido "([^"]+)" tienen estado (\d+)$"#)]
fn estado_por_factor(mundo: &mut Mundo, factor: String, estado: u16) {
    let del_factor: Vec<&Disparo> = mundo
        .bateria
        .iter()
        .filter(|d| d.factor == factor)
        .collect();
    assert!(!del_factor.is_empty(), "no hay pedidos con factor {factor}");
    for disparo in del_factor {
        assert_eq!(
            disparo.respuesta.estado, estado,
            "{} {}",
            disparo.metodo, disparo.camino
        );
    }
}

#[given("se guardó la cantidad de líneas del registro técnico")]
fn guarda_lineas_del_registro(mundo: &mut Mundo) {
    mundo.lineas_del_registro = mundo.registro_tecnico().lines().count();
}

/// Las líneas que el registro técnico recibió desde que se guardó la cantidad.
fn lineas_nuevas(mundo: &Mundo) -> Vec<String> {
    let registro = mundo.registro_tecnico();
    registro
        .lines()
        .skip(mundo.lineas_del_registro)
        .map(str::to_string)
        .collect()
}

#[then("el registro técnico tiene exactamente una línea nueva por cada pedido rechazado")]
fn una_linea_por_rechazo(mundo: &mut Mundo) {
    let rechazados = mundo
        .bateria
        .iter()
        .filter(|d| d.respuesta.estado >= 400)
        .count();
    assert!(rechazados > 0);
    let nuevas = lineas_nuevas(mundo);
    assert_eq!(nuevas.len(), rechazados, "{nuevas:#?}");
}

/// El motivo que anota el servidor según el factor inválido (D-34).
fn motivo(factor: &str) -> &'static str {
    match factor {
        "token" => "no_autorizado",
        "Host" => "host_no_permitido",
        "Origin" => "origen_no_permitido",
        otro => panic!("factor desconocido {otro}"),
    }
}

/// Los pedidos se mandan de a uno, así que la línea i es la del pedido i.
#[then(
    "cada una de esas líneas tiene el método, el camino sin parámetros, el código de estado y el motivo del rechazo"
)]
fn lineas_con_detalle(mundo: &mut Mundo) {
    let nuevas = lineas_nuevas(mundo);
    let rechazados = mundo.bateria.iter().filter(|d| d.respuesta.estado >= 400);
    for (linea, disparo) in nuevas.iter().zip(rechazados) {
        let camino = disparo.camino.split('?').next().unwrap();
        let esperado = format!(
            " {} {camino} {} {}",
            disparo.metodo,
            disparo.respuesta.estado,
            motivo(&disparo.factor)
        );
        assert!(
            linea.ends_with(&esperado),
            "{linea:?} no termina en {esperado:?}"
        );
    }
}

// Solo las variables que `osr` lee: XDG_STATE_HOME y HOME, con la que se
// interpreta `~` en la pantalla de inicio (PRY-19, D-20).
#[given(regex = r#"^la variable de entorno "(XDG_STATE_HOME|HOME)" vale "([^"]+)"$"#)]
fn variable_de_entorno(mundo: &mut Mundo, nombre: String, valor: String) {
    let valor = Some(mundo.resolver(&valor).into());
    match nombre.as_str() {
        "HOME" => mundo.home = valor,
        _ => mundo.xdg_state_home = valor,
    }
}

#[given(regex = r"^la umask del proceso osr es ([0-7]{3})$")]
fn con_umask(mundo: &mut Mundo, umask: String) {
    mundo.umask = Some(umask);
}

// Solo la carpeta de estado y el registro técnico; los permisos de lo que
// está dentro del proyecto llegan con SRV-13 (D-20).
#[then(
    regex = r#"^los permisos de "(\{tmp\}/estado/osr(?:/registro\.log)?|\{aplicación\})" son (0[0-7]{3})$"#
)]
fn permisos_de(mundo: &mut Mundo, ruta: String, modo: String) {
    use std::os::unix::fs::PermissionsExt;
    let ruta = mundo.ruta(&ruta);
    let esperado = u32::from_str_radix(&modo, 8).unwrap();
    let modo = fs::metadata(&ruta)
        .unwrap_or_else(|e| panic!("leer {}: {e}", ruta.display()))
        .permissions()
        .mode();
    assert_eq!(modo & 0o777, esperado, "{}", ruta.display());
}

/// Cuántos pedidos van a la vez, cada uno por su conexión keep-alive.
const CONEXIONES: usize = 8;

/// Un camino de 200 caracteres distinto para cada `i`.
fn camino_de_200(i: usize) -> String {
    let camino = format!("/{i:0>199}");
    assert_eq!(camino.chars().count(), 200);
    camino
}

#[when("se mandan 300.000 pedidos sin token a caminos distintos de 200 caracteres")]
async fn pedidos_sin_token(mundo: &mut Mundo) {
    const PEDIDOS: usize = 300_000;
    let base = mundo.direccion("");
    let cliente = reqwest::Client::builder().no_proxy().build().unwrap();
    let mut tareas = tokio::task::JoinSet::new();
    for primero in 0..CONEXIONES {
        let (cliente, base) = (cliente.clone(), base.clone());
        tareas.spawn(async move {
            for i in (primero..PEDIDOS).step_by(CONEXIONES) {
                let respuesta = cliente
                    .get(format!("{base}{}", camino_de_200(i)))
                    .send()
                    .await
                    .expect("mandar el pedido");
                assert_eq!(respuesta.status().as_u16(), 401);
                respuesta.bytes().await.expect("leer el cuerpo");
            }
        });
    }
    while let Some(tarea) = tareas.join_next().await {
        tarea.expect("una conexión de la prueba falló");
    }
}

/// Los archivos de la carpeta de estado que empiezan con `registro.log`.
fn archivos_del_registro(mundo: &Mundo) -> BTreeSet<String> {
    fs::read_dir(mundo.carpeta_de_estado())
        .expect("leer la carpeta de estado")
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|nombre| nombre.starts_with("registro.log"))
        .collect()
}

#[then(
    r#"en la carpeta de estado solo existen "registro.log" y "registro.log.1" como archivos del registro técnico"#
)]
fn solo_los_dos_registros(mundo: &mut Mundo) {
    let esperados = BTreeSet::from(["registro.log".to_string(), "registro.log.1".to_string()]);
    assert_eq!(archivos_del_registro(mundo), esperados);
}

#[then(r#"los tamaños de "registro.log" y "registro.log.1" suman 10.000.000 bytes o menos"#)]
fn suman_a_lo_sumo(mundo: &mut Mundo) {
    let carpeta = mundo.carpeta_de_estado();
    let suma: u64 = ["registro.log", "registro.log.1"]
        .iter()
        .map(|nombre| {
            fs::metadata(carpeta.join(nombre))
                .expect("leer el tamaño")
                .len()
        })
        .sum();
    assert!(suma <= 10_000_000, "suman {suma} bytes");
}
