//! El arranque de `osr`: abre o crea el proyecto, levanta el servidor, avisa
//! la dirección y espera la señal de cierre (SRV-01, SRV-15).

use std::path::Path;
use std::process::ExitCode;
use std::sync::Arc;

use tokio::sync::watch;
use tokio::task::JoinHandle;

use crate::abierto::{Abierto, Compartido};
use crate::cierre::{self, Cierre, Senales};
use crate::escucha::{self, Escucha};
use crate::{abrir, aplicacion, navegador, nuevo, registro, servidor, token};

/// Atiende con `rutas` en un socket ya abierto, en segundo plano, hasta que
/// `cerrando` cambie. Entonces deja de aceptar y termina lo que tiene en curso.
fn atender(
    socket: std::net::TcpListener,
    rutas: axum::Router,
    mut cerrando: watch::Receiver<()>,
) -> JoinHandle<()> {
    socket.set_nonblocking(true).expect("socket sin bloqueo");
    let socket = tokio::net::TcpListener::from_std(socket).expect("socket de tokio");
    tokio::spawn(async move {
        let aviso = async move {
            let _ = cerrando.changed().await;
        };
        let _ = axum::serve(socket, rutas)
            .with_graceful_shutdown(aviso)
            .await;
    })
}

/// Cierra de forma ordenada (SRV-15). Detiene los servidores, espera los
/// pedidos en curso salvo que llegue otra señal, borra el bloqueo del
/// proyecto abierto en ese momento, también si se abrió desde la pantalla
/// (PRY-21), y anota el cierre. Termina con 0, o con 1 si una segunda señal
/// cortó la espera.
async fn cerrar(
    senales: &mut Senales,
    cerrando: watch::Sender<()>,
    servidores: Vec<JoinHandle<()>>,
    abierto: &Compartido,
) -> ExitCode {
    let _ = cerrando.send(());
    let pendientes = async {
        for servidor in servidores {
            let _ = servidor.await;
        }
    };
    let cierre = cierre::cerrar(cierre::ESPERA, pendientes, senales.recibir()).await;
    if let Some(abierto) = abierto.actual() {
        let _ = std::fs::remove_file(abierto.carpeta.join(".osr/bloqueo"));
    }
    match cierre {
        Cierre::Ordenado => {
            registro::anotar("info", "cierre ordenado");
            ExitCode::SUCCESS
        }
        Cierre::Forzado => {
            registro::anotar("aviso", "cierre forzado por una segunda señal");
            ExitCode::from(1)
        }
    }
}

/// Anota en el registro técnico el puerto y si hay IPv6 de loopback.
fn anotar_escucha(escucha: &Escucha) {
    registro::anotar("info", &format!("arranque en el puerto {}", escucha.puerto));
    match escucha.v6 {
        Some(_) => registro::anotar("info", "con IPv6 de loopback"),
        None => registro::anotar("aviso", "sin IPv6 de loopback"),
    }
}

/// Crea la carpeta de la aplicación si falta (PRY-24). Si no puede, lo
/// anota en el registro técnico y sigue: crear una investigación lo vuelve a
/// intentar.
fn preparar_la_carpeta_de_la_aplicacion() {
    let casa = std::env::var_os("HOME");
    let Some(carpeta) = aplicacion::carpeta_desde(casa.as_deref()) else {
        registro::anotar(
            "aviso",
            "sin carpeta personal para la carpeta de la aplicación",
        );
        return;
    };
    if let Err(error) = aplicacion::crear(&carpeta) {
        registro::anotar(
            "aviso",
            &format!(
                "no se pudo crear la carpeta de la aplicación ({})",
                nuevo::motivo(&error)
            ),
        );
    }
}

/// Qué hace `osr` antes de levantar el servidor.
#[derive(Debug)]
pub enum Accion<'a> {
    /// Sin proyecto, la pantalla de inicio (SRV-01).
    Inicio,
    /// Crear un proyecto con su nombre opcional (PRY-01).
    Nuevo(&'a Path, Option<&'a str>),
    /// Abrir un proyecto existente (PRY-04).
    Abrir(&'a Path),
}

/// El proyecto abierto en `ruta` con su nombre, con la carpeta absoluta
/// para el resumen (PRY-22).
fn abierto_en(ruta: &Path, nombre: String) -> Abierto {
    let carpeta = std::path::absolute(ruta).unwrap_or_else(|_| ruta.to_path_buf());
    Abierto { carpeta, nombre }
}

/// Crea o abre el proyecto de `accion` y lo devuelve, o el mensaje y el
/// código de salida del error.
fn preparar(accion: &Accion<'_>) -> Result<Option<Abierto>, (String, i32)> {
    let momento = nuevo::Momento::ahora();
    match *accion {
        Accion::Inicio => Ok(None),
        Accion::Nuevo(ruta, nombre) => nuevo::crear(ruta, nombre, &momento)
            .map(|nombre| Some(abierto_en(ruta, nombre)))
            .map_err(|error| (error.to_string(), error.codigo())),
        Accion::Abrir(ruta) => abrir::abrir(ruta, &momento)
            .map(|nombre| Some(abierto_en(ruta, nombre)))
            .map_err(|error| (error.to_string(), error.codigo())),
    }
}

/// Corre `osr` hasta SIGINT o SIGTERM, después de crear o abrir el proyecto
/// de `accion`.
///
/// Las señales se registran antes de crear, para que una señal que llega
/// apenas existe el bloqueo ya encuentre quién la atienda. El puerto se toma
/// antes de crear, así, si no hay puerto libre, no queda nada creado.
pub async fn correr(accion: Accion<'_>, sin_navegador: bool) -> ExitCode {
    let mut senales = Senales::registrar();
    let Some(escucha) = escucha::escuchar() else {
        registro::anotar("error", "sin puerto libre en las direcciones locales");
        eprintln!("osr: no se encontró un puerto libre en las direcciones locales.");
        return ExitCode::from(5);
    };
    anotar_escucha(&escucha);
    preparar_la_carpeta_de_la_aplicacion();
    let proyecto = match preparar(&accion) {
        Ok(proyecto) => proyecto,
        Err((mensaje, codigo)) => {
            eprintln!("osr: {mensaje}");
            return ExitCode::from(codigo as u8);
        }
    };
    let token: Arc<str> = token::nuevo().expect("leer /dev/urandom").into();
    let abierto = Compartido::con(proyecto);
    let rutas = servidor::rutas(token.clone(), escucha.puerto, abierto.clone());
    let (cerrando, aviso) = watch::channel(());
    let mut servidores = vec![atender(escucha.v4, rutas.clone(), aviso.clone())];
    servidores.extend(escucha.v6.map(|v6| atender(v6, rutas, aviso)));
    let direccion = format!("http://127.0.0.1:{}/?token={token}", escucha.puerto);
    println!("Open Social Research está abierto en {direccion}");
    if !sin_navegador {
        navegador::abrir(&direccion);
    }
    senales.recibir().await;
    cerrar(&mut senales, cerrando, servidores, &abierto).await
}
