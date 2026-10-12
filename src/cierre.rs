//! El cierre ordenado de `osr` (SRV-15).
//!
//! Con la primera señal el servidor deja de aceptar pedidos y termina los que
//! tiene en curso. Una segunda señal durante el cierre lo corta en el acto,
//! sin riesgo para los archivos gracias a las escrituras atómicas (PRY-12).

use std::future::Future;
use std::time::Duration;

use tokio::signal::unix::{Signal, SignalKind, signal};

/// Lo más que espera el cierre a los pedidos en curso, dentro de los 3 s que
/// pide SRV-15 para todo el cierre.
pub const ESPERA: Duration = Duration::from_secs(2);

/// SIGINT y SIGTERM, registradas apenas arranca `osr`.
pub struct Senales {
    terminar: Signal,
    interrumpir: Signal,
}

impl Senales {
    pub fn registrar() -> Senales {
        Senales {
            terminar: signal(SignalKind::terminate()).expect("registrar SIGTERM"),
            interrumpir: signal(SignalKind::interrupt()).expect("registrar SIGINT"),
        }
    }

    /// Espera la próxima SIGINT o SIGTERM.
    pub async fn recibir(&mut self) {
        tokio::select! {
            _ = self.terminar.recv() => {}
            _ = self.interrumpir.recv() => {}
        }
    }
}

/// Cómo terminó el cierre.
#[derive(Debug, PartialEq)]
pub enum Cierre {
    /// Los pedidos en curso terminaron, o se acabó la espera.
    Ordenado,
    /// Llegó una segunda señal antes.
    Forzado,
}

/// Espera hasta `espera` a que `pendientes` termine, salvo que antes llegue
/// `segunda`.
pub async fn cerrar(
    espera: Duration,
    pendientes: impl Future<Output = ()>,
    segunda: impl Future<Output = ()>,
) -> Cierre {
    tokio::select! {
        _ = tokio::time::timeout(espera, pendientes) => Cierre::Ordenado,
        _ = segunda => Cierre::Forzado,
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use std::future::{pending, ready};
    use std::time::Instant;

    const LARGA: Duration = Duration::from_secs(60);

    #[tokio::test]
    async fn sin_pendientes_el_cierre_es_ordenado() {
        assert_eq!(cerrar(LARGA, ready(()), pending()).await, Cierre::Ordenado);
    }

    #[tokio::test]
    async fn una_segunda_senal_corta_la_espera() {
        let inicio = Instant::now();
        assert_eq!(cerrar(LARGA, pending(), ready(())).await, Cierre::Forzado);
        assert!(inicio.elapsed() < Duration::from_secs(1));
    }

    #[tokio::test]
    async fn la_espera_tiene_limite() {
        let inicio = Instant::now();
        let espera = Duration::from_millis(50);
        assert_eq!(cerrar(espera, pending(), pending()).await, Cierre::Ordenado);
        assert!(inicio.elapsed() >= espera);
    }
}
