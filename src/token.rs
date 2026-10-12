//! El token de la sesión, nuevo en cada arranque (SRV-03).
//!
//! Son 256 bits del generador del sistema en base64url sin relleno, 43
//! caracteres. Vive solo en la memoria del proceso.

use std::io::{self, Read};

const ALFABETO: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/// Codifica en base64url sin relleno (RFC 4648, sección 5).
fn base64url(bytes: &[u8]) -> String {
    let mut texto = String::new();
    for grupo in bytes.chunks(3) {
        let segundo = *grupo.get(1).unwrap_or(&0);
        let tercero = *grupo.get(2).unwrap_or(&0);
        let n = u32::from_be_bytes([0, grupo[0], segundo, tercero]);
        // Un grupo de k bytes da k + 1 caracteres.
        for i in 0..=grupo.len() {
            texto.push(char::from(ALFABETO[(n >> (18 - 6 * i) & 63) as usize]));
        }
    }
    texto
}

/// Un token nuevo con 256 bits de `/dev/urandom`.
pub fn nuevo() -> io::Result<String> {
    let mut bytes = [0u8; 32];
    std::fs::File::open("/dev/urandom")?.read_exact(&mut bytes)?;
    Ok(base64url(&bytes))
}

/// Compara sin cortar en la primera diferencia, para no dar pistas por el
/// tiempo de respuesta.
pub fn iguales(a: &str, b: &str) -> bool {
    a.len() == b.len() && a.bytes().zip(b.bytes()).fold(0, |d, (x, y)| d | (x ^ y)) == 0
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn base64url_sigue_los_vectores_del_rfc_4648() {
        let casos = [
            ("", ""),
            ("f", "Zg"),
            ("fo", "Zm8"),
            ("foo", "Zm9v"),
            ("foob", "Zm9vYg"),
            ("fooba", "Zm9vYmE"),
            ("foobar", "Zm9vYmFy"),
        ];
        for (entrada, salida) in casos {
            assert_eq!(base64url(entrada.as_bytes()), salida, "{entrada}");
        }
    }

    #[test]
    fn base64url_usa_guion_y_guion_bajo() {
        assert_eq!(base64url(&[0xfb, 0xff]), "-_8");
        assert_eq!(base64url(&[0xff; 3]), "____");
    }

    #[test]
    fn el_token_tiene_43_caracteres_y_cambia() {
        let a = nuevo().unwrap();
        let b = nuevo().unwrap();
        assert_eq!(a.len(), 43);
        assert!(a.bytes().all(|c| ALFABETO.contains(&c)));
        assert_ne!(a, b);
    }

    #[test]
    fn iguales_compara_todo_el_texto() {
        assert!(iguales("abc", "abc"));
        assert!(!iguales("abc", "abd"));
        assert!(!iguales("abd", "abc"));
        assert!(!iguales("abc", "ab"));
        assert!(!iguales("xbc", "abc"));
        // Dos diferencias iguales no se cancelan entre sí.
        assert!(!iguales("ab", "ba"));
        assert!(iguales("", ""));
    }
}
