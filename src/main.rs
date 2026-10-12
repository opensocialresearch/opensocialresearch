//! `osr`, el programa de Open Social Research.

mod nuevo;
mod plantillas;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use tokio::signal::unix::{SignalKind, signal};

/// Herramienta libre de investigación social aplicada.
#[derive(Parser)]
#[command(name = "osr", version)]
struct Cli {
    #[command(subcommand)]
    orden: Option<Orden>,
}

#[derive(Subcommand)]
enum Orden {
    /// Crea un proyecto vacío y lo abre (PRY-01).
    Nuevo {
        /// Carpeta del proyecto, nueva o vacía.
        ruta: PathBuf,
        /// Nombre visible; por defecto, el último componente de la ruta.
        #[arg(long)]
        nombre: Option<String>,
        /// No abre el navegador (SRV-01).
        #[arg(long)]
        sin_navegador: bool,
    },
}

/// Crea el proyecto y lo mantiene abierto hasta SIGINT o SIGTERM (SRV-15).
///
/// Las señales se registran antes de crear, para que una señal que llega
/// apenas existe el bloqueo ya encuentre quién la atienda.
async fn crear_y_esperar(ruta: &Path, nombre: Option<&str>) -> ExitCode {
    let mut terminar = signal(SignalKind::terminate()).expect("registrar SIGTERM");
    let mut interrumpir = signal(SignalKind::interrupt()).expect("registrar SIGINT");
    if let Err(error) = nuevo::crear(ruta, nombre, &nuevo::Momento::ahora()) {
        eprintln!("osr: {error}");
        return ExitCode::from(error.codigo() as u8);
    }
    tokio::select! {
        _ = terminar.recv() => {}
        _ = interrumpir.recv() => {}
    }
    let _ = std::fs::remove_file(ruta.join(".osr/bloqueo"));
    ExitCode::SUCCESS
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    match Cli::parse().orden {
        Some(Orden::Nuevo { ruta, nombre, .. }) => crear_y_esperar(&ruta, nombre.as_deref()).await,
        None => ExitCode::SUCCESS,
    }
}
