//! `osr`, el programa de Open Social Research.

mod abrir;
mod arranque;
mod cierre;
mod escucha;
mod estado;
mod navegador;
mod nuevo;
mod plantillas;
mod registro;
mod rutas;
mod servidor;
mod token;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::{Parser, Subcommand};

/// Herramienta libre de investigación social aplicada.
#[derive(Parser)]
#[command(name = "osr", version, args_conflicts_with_subcommands = true)]
struct Cli {
    #[command(subcommand)]
    orden: Option<Orden>,
    /// Carpeta de un proyecto para abrir, igual que `osr abrir` (PRY-04).
    ruta: Option<PathBuf>,
    /// No abre el navegador (SRV-01).
    #[arg(long, global = true)]
    sin_navegador: bool,
}

#[derive(Subcommand)]
enum Orden {
    /// Abre un proyecto existente (PRY-04).
    Abrir {
        /// Carpeta del proyecto.
        ruta: PathBuf,
    },
    /// Crea un proyecto vacío y lo abre (PRY-01).
    Nuevo {
        /// Carpeta del proyecto, nueva o vacía.
        ruta: PathBuf,
        /// Nombre visible; por defecto, el último componente de la ruta.
        #[arg(long)]
        nombre: Option<String>,
    },
}

/// El mensaje de SRV-01 para una orden u opción desconocida. Los demás
/// errores de la línea de órdenes quedan como los da clap.
fn desconocido(error: &clap::Error) -> Option<String> {
    let contexto = match error.kind() {
        ErrorKind::UnknownArgument => ContextKind::InvalidArg,
        ErrorKind::InvalidSubcommand => ContextKind::InvalidSubcommand,
        _ => return None,
    };
    match error.get(contexto) {
        Some(ContextValue::String(texto)) => Some(format!(
            "osr: no se reconoce {texto}. Las órdenes son osr, osr abrir <carpeta> y osr nuevo <carpeta>."
        )),
        _ => None,
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(error) => match desconocido(&error) {
            Some(mensaje) => {
                eprintln!("{mensaje}");
                return ExitCode::from(2);
            }
            None => error.exit(),
        },
    };
    arranque::correr(accion(&cli), cli.sin_navegador).await
}

/// Qué tiene que hacer `osr` según la línea de órdenes.
fn accion(cli: &Cli) -> arranque::Accion<'_> {
    match (&cli.orden, &cli.ruta) {
        (Some(Orden::Nuevo { ruta, nombre }), _) => {
            arranque::Accion::Nuevo(ruta, nombre.as_deref())
        }
        (Some(Orden::Abrir { ruta }), _) | (None, Some(ruta)) => arranque::Accion::Abrir(ruta),
        (None, None) => arranque::Accion::Inicio,
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    fn mensaje(argumentos: &[&str]) -> Option<String> {
        let error = Cli::try_parse_from(argumentos)
            .err()
            .expect("tenía que fallar");
        desconocido(&error)
    }

    #[test]
    fn una_opcion_o_una_orden_desconocida_da_el_mensaje_de_srv_01() {
        assert_eq!(
            mensaje(&["osr", "--opcion-falsa"]).unwrap(),
            "osr: no se reconoce --opcion-falsa. Las órdenes son osr, osr abrir <carpeta> y osr nuevo <carpeta>."
        );
        assert!(
            mensaje(&["osr", "abrir", "x", "--borrar"])
                .unwrap()
                .starts_with("osr: no se reconoce --borrar.")
        );
    }

    fn accion_de(argumentos: &[&str]) -> String {
        let cli = Cli::try_parse_from(argumentos).unwrap();
        format!("{:?}", accion(&cli))
    }

    /// Una palabra suelta es la ruta de un proyecto, como `osr abrir` (PRY-04).
    #[test]
    fn osr_con_una_ruta_es_lo_mismo_que_osr_abrir() {
        assert_eq!(accion_de(&["osr", "estudio"]), r#"Abrir("estudio")"#);
        assert_eq!(
            accion_de(&["osr", "abrir", "estudio"]),
            r#"Abrir("estudio")"#
        );
        assert_eq!(
            accion_de(&["osr", "nuevo", "x", "--nombre", "y"]),
            r#"Nuevo("x", Some("y"))"#
        );
        assert_eq!(accion_de(&["osr"]), "Inicio");
        assert!(Cli::try_parse_from(["osr", "estudio", "nuevo"]).is_err());
    }

    #[test]
    fn los_demas_errores_quedan_como_los_da_clap() {
        assert_eq!(mensaje(&["osr", "nuevo"]), None);
        assert_eq!(mensaje(&["osr", "--help"]), None);
    }

    #[test]
    fn sin_navegador_vale_antes_y_despues_de_la_orden() {
        assert!(
            Cli::try_parse_from(["osr", "--sin-navegador"])
                .unwrap()
                .sin_navegador
        );
        let cli = Cli::try_parse_from(["osr", "nuevo", "x", "--sin-navegador"]).unwrap();
        assert!(cli.sin_navegador);
        assert!(!Cli::try_parse_from(["osr"]).unwrap().sin_navegador);
    }
}
