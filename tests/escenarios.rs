//! Ejecuta los escenarios de `escenarios/` como pruebas de aceptación.
//!
//! Por defecto corre todos, falla si algún paso definido falla y cuenta como
//! omitidos los pasos que todavía no tienen definición. Con `OSR_ESTRICTO=1`
//! un paso sin definir también es una falla, que es como se corren los
//! escenarios del requisito en curso:
//!
//!     OSR_ESTRICTO=1 cargo test --test escenarios -- --tags @PRY-01

use cucumber::World;

#[derive(Debug, Default, World)]
pub struct Mundo {}

#[tokio::main]
async fn main() {
    let cucumber = Mundo::cucumber();
    if std::env::var_os("OSR_ESTRICTO").is_some() {
        cucumber.fail_on_skipped().run_and_exit("escenarios").await;
    } else {
        cucumber.run_and_exit("escenarios").await;
    }
}
