//! `osr`, el programa de Open Social Research.

use clap::Parser;

/// Herramienta libre de investigación social aplicada.
#[derive(Parser)]
#[command(name = "osr", version)]
struct Cli {}

fn main() {
    Cli::parse();
}
