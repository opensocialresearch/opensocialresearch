//! El arranque de `osr`: abre o crea el proyecto, levanta el servidor, avisa
//! la dirección y espera la señal de cierre (SRV-01, SRV-15).

use std::path::Path;
use std::process::ExitCode;
use std::sync::Arc;

use tokio::signal::unix::{SignalKind, signal};

use crate::{navegador, nuevo, servidor, token};

/// Corre `osr` hasta SIGINT o SIGTERM. Con `crear` primero crea el proyecto
/// según PRY-01.
///
/// Las señales se registran antes de crear, para que una señal que llega
/// apenas existe el bloqueo ya encuentre quién la atienda.
pub async fn correr(crear: Option<(&Path, Option<&str>)>, sin_navegador: bool) -> ExitCode {
    let mut terminar = signal(SignalKind::terminate()).expect("registrar SIGTERM");
    let mut interrumpir = signal(SignalKind::interrupt()).expect("registrar SIGINT");
    if let Some((ruta, nombre)) = crear
        && let Err(error) = nuevo::crear(ruta, nombre, &nuevo::Momento::ahora())
    {
        eprintln!("osr: {error}");
        return ExitCode::from(error.codigo() as u8);
    }
    let token: Arc<str> = token::nuevo().expect("leer /dev/urandom").into();
    let escucha = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("escuchar en 127.0.0.1");
    let puerto = escucha.local_addr().expect("leer el puerto").port();
    let direccion = format!("http://127.0.0.1:{puerto}/?token={token}");
    println!("Open Social Research está abierto en {direccion}");
    if !sin_navegador {
        navegador::abrir(&direccion);
    }
    let servidor = tokio::spawn(axum::serve(escucha, servidor::rutas(token)).into_future());
    tokio::select! {
        _ = terminar.recv() => {}
        _ = interrumpir.recv() => {}
    }
    servidor.abort();
    if let Some((ruta, _)) = crear {
        let _ = std::fs::remove_file(ruta.join(".osr/bloqueo"));
    }
    ExitCode::SUCCESS
}
