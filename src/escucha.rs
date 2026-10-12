//! Dónde escucha el servidor, solo en loopback y con puerto aleatorio (SRV-02).

use std::io;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener};

/// Intentos de encontrar un puerto libre en las dos direcciones.
const INTENTOS: usize = 10;

/// Los sockets del servidor, con el mismo puerto en las dos direcciones.
pub struct Escucha {
    pub v4: TcpListener,
    /// Falta si el sistema no tiene IPv6 de loopback.
    pub v6: Option<TcpListener>,
    pub puerto: u16,
}

/// El sistema no tiene `::1`, o no tiene IPv6 del todo (`EAFNOSUPPORT`).
fn sin_ipv6(error: &io::Error) -> bool {
    error.kind() == io::ErrorKind::AddrNotAvailable || error.raw_os_error() == Some(97)
}

/// Pide al sistema un puerto efímero en `127.0.0.1` y toma el mismo en
/// `[::1]`. Si ese está ocupado en IPv6 suelta los dos y prueba otro, hasta
/// `INTENTOS` veces. `atar` es `TcpListener::bind` salvo en las pruebas.
fn escuchar_con(atar: impl Fn(SocketAddr) -> io::Result<TcpListener>) -> Option<Escucha> {
    for _ in 0..INTENTOS {
        let Ok(v4) = atar(SocketAddr::from((Ipv4Addr::LOCALHOST, 0))) else {
            continue;
        };
        let puerto = v4.local_addr().ok()?.port();
        match atar(SocketAddr::from((Ipv6Addr::LOCALHOST, puerto))) {
            Ok(v6) => {
                return Some(Escucha {
                    v4,
                    v6: Some(v6),
                    puerto,
                });
            }
            Err(error) if sin_ipv6(&error) => {
                return Some(Escucha {
                    v4,
                    v6: None,
                    puerto,
                });
            }
            Err(_) => {}
        }
    }
    None
}

/// Los sockets del servidor, o nada si no hubo puerto libre en 10 intentos.
pub fn escuchar() -> Option<Escucha> {
    escuchar_con(TcpListener::bind)
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use std::cell::Cell;

    fn error(tipo: io::ErrorKind) -> io::Error {
        io::Error::from(tipo)
    }

    #[test]
    fn escucha_en_las_dos_direcciones_con_el_mismo_puerto() {
        let escucha = escuchar().unwrap();
        let v4 = escucha.v4.local_addr().unwrap();
        let v6 = escucha.v6.unwrap().local_addr().unwrap();
        assert_eq!(v4.ip(), Ipv4Addr::LOCALHOST);
        assert_eq!(v6.ip(), Ipv6Addr::LOCALHOST);
        assert_eq!((v4.port(), v6.port()), (escucha.puerto, escucha.puerto));
        assert_ne!(escucha.puerto, 0);
    }

    #[test]
    fn sin_ipv6_de_loopback_escucha_solo_en_ipv4() {
        for falla in [
            error(io::ErrorKind::AddrNotAvailable),
            io::Error::from_raw_os_error(97),
        ] {
            let falla = Cell::new(Some(falla));
            let escucha = escuchar_con(|direccion| match direccion {
                SocketAddr::V4(_) => TcpListener::bind(direccion),
                SocketAddr::V6(_) => Err(falla.take().unwrap()),
            })
            .unwrap();
            assert!(escucha.v6.is_none());
            assert_eq!(escucha.v4.local_addr().unwrap().port(), escucha.puerto);
        }
    }

    #[test]
    fn con_ipv6_ocupado_prueba_otro_puerto_hasta_10_veces() {
        let intentos = Cell::new(0);
        let ocupado = |direccion: SocketAddr| match direccion {
            SocketAddr::V4(_) => TcpListener::bind(direccion),
            SocketAddr::V6(_) => {
                intentos.set(intentos.get() + 1);
                Err(error(io::ErrorKind::AddrInUse))
            }
        };
        assert!(escuchar_con(ocupado).is_none());
        assert_eq!(intentos.get(), 10);
    }

    #[test]
    fn el_segundo_intento_puede_salir_bien() {
        let intentos = Cell::new(0);
        let escucha = escuchar_con(|direccion| match direccion {
            SocketAddr::V6(_) if intentos.replace(intentos.get() + 1) == 0 => {
                Err(error(io::ErrorKind::AddrInUse))
            }
            _ => TcpListener::bind(direccion),
        })
        .unwrap();
        assert!(escucha.v6.is_some());
        assert_eq!(intentos.get(), 2);
    }

    #[test]
    fn sin_ipv4_no_hay_escucha() {
        assert!(escuchar_con(|_| Err(error(io::ErrorKind::AddrInUse))).is_none());
    }

    #[test]
    fn otros_errores_no_son_falta_de_ipv6() {
        assert!(!sin_ipv6(&error(io::ErrorKind::AddrInUse)));
        assert!(!sin_ipv6(&io::Error::from_raw_os_error(98)));
    }
}
