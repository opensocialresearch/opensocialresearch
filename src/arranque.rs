//! El arranque de `osr`: abre o crea el proyecto, levanta el servidor, avisa
//! la dirección y espera la señal de cierre (SRV-01, SRV-15).

use std::path::Path;
use std::process::ExitCode;
use std::sync::Arc;

use tokio::signal::unix::{SignalKind, signal};

use crate::escucha::{self, Escucha};
use crate::{navegador, nuevo, registro, servidor, token};

/// Atiende con `rutas` en un socket ya abierto, en segundo plano.
fn atender(socket: std::net::TcpListener, rutas: axum::Router) -> tokio::task::JoinHandle<()> {
    socket.set_nonblocking(true).expect("socket sin bloqueo");
    let socket = tokio::net::TcpListener::from_std(socket).expect("socket de tokio");
    tokio::spawn(async move {
        let _ = axum::serve(socket, rutas).await;
    })
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
    let mut terminar = signal(SignalKind::terminate()).expect("registrar SIGTERM");
    let mut interrumpir = signal(SignalKind::interrupt()).expect("registrar SIGINT");
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
    let mut servidores = vec![atender(escucha.v4, rutas.clone())];
    servidores.extend(escucha.v6.map(|v6| atender(v6, rutas)));
    let direccion = format!("http://127.0.0.1:{}/?token={token}", escucha.puerto);
    println!("Open Social Research está abierto en {direccion}");
    if !sin_navegador {
        navegador::abrir(&direccion);
    }
    tokio::select! {
        _ = terminar.recv() => {}
        _ = interrumpir.recv() => {}
    }
    servidores.iter().for_each(|servidor| servidor.abort());
    if let Some((ruta, _)) = crear {
        let _ = std::fs::remove_file(ruta.join(".osr/bloqueo"));
    }
    ExitCode::SUCCESS
}
