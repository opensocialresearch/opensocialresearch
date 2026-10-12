//! La lista de rutas del servidor (SRV-05, SRV-10).
//!
//! El servidor arma sus rutas solo desde esta lista, y la prueba de seguridad
//! lee este mismo archivo, así una ruta nueva queda cubierta sin tocar la
//! prueba. No depende de nada del programa para que la prueba lo pueda
//! incluir tal cual.

/// Una ruta, con su camino en la sintaxis de axum y los métodos que acepta.
pub struct Ruta {
    pub camino: &'static str,
    pub metodos: &'static [&'static str],
}

/// Todas las rutas que atiende el servidor.
///
/// `/audio/{entrevista}` declara solo GET: el servidor atiende HEAD con el
/// mismo manejador, sin cuerpo (D-39).
pub const RUTAS: &[Ruta] = &[
    Ruta {
        camino: "/",
        metodos: &["GET"],
    },
    Ruta {
        camino: "/recursos/{*camino}",
        metodos: &["GET"],
    },
    Ruta {
        camino: "/audio/{entrevista}",
        metodos: &["GET"],
    },
];
