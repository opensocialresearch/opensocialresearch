//! El proyecto que tiene abierto este proceso, si hay uno (PRY-21).
//!
//! Un proceso abre a lo sumo un proyecto y lo mantiene hasta terminar. Lo
//! comparten el servidor, que lo muestra y lo abre desde la pantalla de
//! inicio, y el cierre, que borra su bloqueo (SRV-15).

use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};

/// Un proyecto abierto: su carpeta absoluta y su nombre.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Abierto {
    pub carpeta: PathBuf,
    pub nombre: String,
}

/// El proyecto abierto, compartido entre los pedidos y el cierre.
#[derive(Debug, Clone, Default)]
pub struct Compartido(Arc<Mutex<Option<Abierto>>>);

impl Compartido {
    pub fn con(abierto: Option<Abierto>) -> Compartido {
        Compartido(Arc::new(Mutex::new(abierto)))
    }

    /// El candado, para comprobar y abrir sin que otro pedido se cruce. Un
    /// pedido que entró en pánico con el candado no deja el estado a medias,
    /// porque solo se escribe al final, así que se sigue usando.
    pub fn tomar(&self) -> MutexGuard<'_, Option<Abierto>> {
        self.0
            .lock()
            .unwrap_or_else(|envenenado| envenenado.into_inner())
    }

    /// Una copia del proyecto abierto en este momento.
    pub fn actual(&self) -> Option<Abierto> {
        self.tomar().clone()
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    fn estudio() -> Abierto {
        Abierto {
            carpeta: PathBuf::from("/tmp/estudio"),
            nombre: "Estudio".into(),
        }
    }

    #[test]
    fn empieza_con_lo_que_se_le_da_y_las_copias_comparten_el_estado() {
        assert_eq!(Compartido::default().actual(), None);
        let compartido = Compartido::con(None);
        let otro = compartido.clone();
        *otro.tomar() = Some(estudio());
        assert_eq!(compartido.actual(), Some(estudio()));
        assert_eq!(Compartido::con(Some(estudio())).actual(), Some(estudio()));
    }

    #[test]
    fn un_panico_con_el_candado_no_lo_deja_inservible() {
        let compartido = Compartido::con(Some(estudio()));
        let otro = compartido.clone();
        let _ = std::thread::spawn(move || {
            let _candado = otro.tomar();
            panic!("pedido que falla");
        })
        .join();
        assert_eq!(compartido.actual(), Some(estudio()));
    }
}
