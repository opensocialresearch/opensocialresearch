//! El arranque de `osr`: abre o crea el proyecto, levanta el servidor, avisa
//! la dirección y espera la señal de cierre (SRV-01, SRV-15).

use std::path::Path;
use std::process::ExitCode;
use std::sync::Arc;

use tokio::sync::watch;
use tokio::task::JoinHandle;

use crate::cierre::{self, Cierre, Senales};
use crate::escucha::{self, Escucha};
use crate::{navegador, nuevo, registro, servidor, token};

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
/// pedidos en curso salvo que llegue otra señal, borra el bloqueo y anota el
/// cierre. Termina con 0, o con 1 si una segunda señal cortó la espera.
async fn cerrar(
    senales: &mut Senales,
    cerrando: watch::Sender<()>,
    servidores: Vec<JoinHandle<()>>,
    bloqueo: Option<std::path::PathBuf>,
) -> ExitCode {
    let _ = cerrando.send(());
    let pendientes = async {
        for servidor in servidores {
            let _ = servidor.await;
        }
    };
    let cierre = cierre::cerrar(cierre::ESPERA, pendientes, senales.recibir()).await;
    if let Some(bloqueo) = bloqueo {
        let _ = std::fs::remove_file(bloqueo);
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

/// Corre `osr` hasta SIGINT o SIGTERM. Con `crear` primero crea el proyecto
/// según PRY-01.
///
/// Las señales se registran antes de crear, para que una señal que llega
/// apenas existe el bloqueo ya encuentre quién la atienda. El puerto se toma
/// antes de crear, así, si no hay puerto libre, no queda nada creado.
pub async fn correr(crear: Option<(&Path, Option<&str>)>, sin_navegador: bool) -> ExitCode {
    let mut senales = Senales::registrar();
    let Some(escucha) = escucha::escuchar() else {
        registro::anotar("error", "sin puerto libre en las direcciones locales");
        eprintln!("osr: no se encontró un puerto libre en las direcciones locales.");
        return ExitCode::from(5);
    };
    anotar_escucha(&escucha);
    if let Some((ruta, nombre)) = crear
        && let Err(error) = nuevo::crear(ruta, nombre, &nuevo::Momento::ahora())
    {
        eprintln!("osr: {error}");
        return ExitCode::from(error.codigo() as u8);
    }
    let token: Arc<str> = token::nuevo().expect("leer /dev/urandom").into();
    let rutas = servidor::rutas(token.clone());
    let (cerrando, aviso) = watch::channel(());
    let mut servidores = vec![atender(escucha.v4, rutas.clone(), aviso.clone())];
    servidores.extend(escucha.v6.map(|v6| atender(v6, rutas, aviso)));
    let direccion = format!("http://127.0.0.1:{}/?token={token}", escucha.puerto);
    println!("Open Social Research está abierto en {direccion}");
    if !sin_navegador {
        navegador::abrir(&direccion);
    }
    senales.recibir().await;
    let bloqueo = crear.map(|(ruta, _)| ruta.join(".osr/bloqueo"));
    cerrar(&mut senales, cerrando, servidores, bloqueo).await
}
