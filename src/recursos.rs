//! Los archivos de la interfaz en `/recursos/…` (SRV-10): estilos, código y
//! fuentes, metidos en el programa al compilar desde `recursos/`.
//!
//! La política de SRV-08 no deja estilos ni código en línea, así que la
//! interfaz los pide aparte, con la cookie de la sesión (SRV-04).

use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};

/// Un archivo de `recursos/`, con su camino bajo `/recursos/` y su tipo.
struct Recurso {
    camino: &'static str,
    tipo: &'static str,
    bytes: &'static [u8],
}

macro_rules! recurso {
    ($camino:literal, $tipo:literal) => {
        Recurso {
            camino: $camino,
            tipo: $tipo,
            bytes: include_bytes!(concat!("../recursos/", $camino)),
        }
    };
}

const RECURSOS: [Recurso; 8] = [
    recurso!("estilos.css", "text/css; charset=utf-8"),
    recurso!("entrada.js", "text/javascript; charset=utf-8"),
    recurso!("fuentes/Archivo.woff2", "font/woff2"),
    recurso!("fuentes/IBMPlexMono-400.woff2", "font/woff2"),
    recurso!("fuentes/IBMPlexMono-500.woff2", "font/woff2"),
    recurso!("fuentes/IBMPlexMono-600.woff2", "font/woff2"),
    recurso!("fuentes/SourceSerif4.woff2", "font/woff2"),
    recurso!("fuentes/SourceSerif4-Italic.woff2", "font/woff2"),
];

/// El archivo de `/recursos/<camino>`, o 404 si no es uno de `RECURSOS`.
pub fn servir(camino: &str) -> Response {
    match RECURSOS.iter().find(|recurso| recurso.camino == camino) {
        Some(recurso) => ([(header::CONTENT_TYPE, recurso.tipo)], recurso.bytes).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;

    async fn cuerpo(respuesta: Response) -> Vec<u8> {
        axum::body::to_bytes(respuesta.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec()
    }

    #[tokio::test]
    async fn sirve_cada_recurso_con_su_tipo_y_sus_bytes() {
        let respuesta = servir("estilos.css");
        assert_eq!(respuesta.status(), StatusCode::OK);
        assert_eq!(
            respuesta.headers()[header::CONTENT_TYPE],
            "text/css; charset=utf-8"
        );
        let estilos = cuerpo(respuesta).await;
        assert_eq!(estilos, include_bytes!("../recursos/estilos.css"));

        let respuesta = servir("fuentes/Archivo.woff2");
        assert_eq!(respuesta.headers()[header::CONTENT_TYPE], "font/woff2");
        assert_eq!(&cuerpo(respuesta).await[..4], b"wOF2");
    }

    #[test]
    fn un_camino_que_no_es_un_recurso_es_404() {
        for camino in [
            "no-existe.js",
            "",
            "../Cargo.toml",
            "fuentes/",
            "estilos.css/",
        ] {
            assert_eq!(servir(camino).status(), StatusCode::NOT_FOUND, "{camino}");
        }
    }

    #[test]
    fn la_hoja_de_estilos_solo_pide_fuentes_que_se_sirven() {
        let estilos = std::str::from_utf8(include_bytes!("../recursos/estilos.css")).unwrap();
        for pedida in estilos.split("url(/recursos/").skip(1) {
            let camino = &pedida[..pedida.find(')').unwrap()];
            assert_eq!(servir(camino).status(), StatusCode::OK, "{camino}");
        }
    }
}
