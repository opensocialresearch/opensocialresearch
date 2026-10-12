//! El servidor local: rutas y token (SRV-01, SRV-05).

use std::sync::Arc;

use axum::Router;
use axum::extract::{Request, State};
use axum::http::{HeaderValue, Method, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{MethodFilter, MethodRouter, on};

use crate::rutas::RUTAS;
use crate::token;

/// La pantalla sin proyecto, con `Abrir proyecto` y `Crear proyecto`.
const INICIO: &str = include_str!("inicio.html");

/// La página del 401 fuera de `/api/` (SRV-05).
const NO_AUTORIZADO: &str = include_str!("no_autorizado.html");

/// El cuerpo del 401 en `/api/` (SRV-05).
const NO_AUTORIZADO_API: &str =
    r#"{"error":"no_autorizado","mensaje":"Falta el token de la sesión o no es válido."}"#;

/// El valor de `token` en la consulta de un camino, si lo hay.
fn token_de_la_consulta(consulta: &str) -> Option<&str> {
    consulta
        .split('&')
        .find_map(|par| par.strip_prefix("token="))
}

/// Un pedido está autenticado si trae el token en `Authorization: Bearer`,
/// o si es la entrada `GET /?token=<token>`, la única que lo lleva en el
/// camino (SRV-05). La cookie de sesión llega con SRV-04.
fn autenticado(pedido: &Request, token: &str) -> bool {
    let portador = pedido
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|valor| valor.to_str().ok())
        .and_then(|valor| valor.strip_prefix("Bearer "));
    let entrada = (pedido.method() == Method::GET && pedido.uri().path() == "/")
        .then(|| pedido.uri().query().and_then(token_de_la_consulta))
        .flatten();
    [portador, entrada]
        .into_iter()
        .flatten()
        .any(|traido| token::iguales(traido, token))
}

/// El 401 con `WWW-Authenticate` y el cuerpo de la familia del camino.
fn no_autorizado(camino: &str) -> Response {
    let mut respuesta = if camino.starts_with("/api/") {
        let tipo = [(header::CONTENT_TYPE, "application/json")];
        (StatusCode::UNAUTHORIZED, tipo, NO_AUTORIZADO_API).into_response()
    } else {
        (StatusCode::UNAUTHORIZED, Html(NO_AUTORIZADO)).into_response()
    };
    respuesta.headers_mut().insert(
        header::WWW_AUTHENTICATE,
        HeaderValue::from_static("Bearer realm=\"osr\""),
    );
    respuesta
}

/// Rechaza con 401 todo pedido sin el token antes de llegar a la ruta, sin
/// ejecutar nada ni leer ningún archivo.
async fn exigir_token(State(token): State<Arc<str>>, pedido: Request, siguiente: Next) -> Response {
    if autenticado(&pedido, &token) {
        siguiente.run(pedido).await
    } else {
        no_autorizado(pedido.uri().path())
    }
}

async fn inicio() -> Html<&'static str> {
    Html(INICIO)
}

/// El manejador de un camino de `RUTAS` para un método.
fn atender(camino: &str, filtro: MethodFilter) -> MethodRouter {
    match camino {
        "/" => on(filtro, inicio),
        otro => panic!("la ruta {otro} de RUTAS no tiene manejador"),
    }
}

/// Las rutas del servidor, armadas solo desde `RUTAS`. La capa del token va
/// al final, así cubre también las rutas inexistentes, que con token dan 404.
pub fn rutas(token: Arc<str>) -> Router {
    let mut router = Router::new();
    for ruta in RUTAS {
        for metodo in ruta.metodos {
            let metodo = Method::from_bytes(metodo.as_bytes()).expect("método de RUTAS");
            let filtro = MethodFilter::try_from(metodo).expect("método de RUTAS");
            router = router.route(ruta.camino, atender(ruta.camino, filtro));
        }
    }
    router.layer(middleware::from_fn_with_state(token, exigir_token))
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use axum::body::Body;
    use tower::ServiceExt;

    const TOKEN: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQ";

    async fn estado(pedido: axum::http::request::Builder) -> StatusCode {
        let router = rutas(Arc::from(TOKEN));
        router
            .oneshot(pedido.body(Body::empty()).unwrap())
            .await
            .unwrap()
            .status()
    }

    fn get(camino: &str) -> axum::http::request::Builder {
        Request::builder().uri(camino)
    }

    #[test]
    fn el_token_de_la_consulta_es_el_parametro_token() {
        assert_eq!(token_de_la_consulta("token=x"), Some("x"));
        assert_eq!(token_de_la_consulta("a=1&token=y"), Some("y"));
        assert_eq!(token_de_la_consulta("xtoken=z"), None);
        assert_eq!(token_de_la_consulta(""), None);
    }

    #[tokio::test]
    async fn la_entrada_con_el_token_responde_la_pantalla_de_inicio() {
        let respuesta = rutas(Arc::from(TOKEN))
            .oneshot(
                get(&format!("/?token={TOKEN}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(respuesta.status(), StatusCode::OK);
        let cuerpo = axum::body::to_bytes(respuesta.into_body(), usize::MAX)
            .await
            .unwrap();
        let cuerpo = String::from_utf8(cuerpo.to_vec()).unwrap();
        assert!(cuerpo.contains("Abrir proyecto"));
        assert!(cuerpo.contains("Crear proyecto"));
    }

    #[tokio::test]
    async fn sin_token_o_con_otro_responde_401() {
        assert_eq!(estado(get("/")).await, StatusCode::UNAUTHORIZED);
        assert_eq!(estado(get("/?token=otro")).await, StatusCode::UNAUTHORIZED);
        let otro = get("/").header("Authorization", "Bearer otro");
        assert_eq!(estado(otro).await, StatusCode::UNAUTHORIZED);
        let sin_bearer = get("/").header("Authorization", TOKEN);
        assert_eq!(estado(sin_bearer).await, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn el_token_en_la_consulta_solo_vale_en_la_entrada() {
        let otro_camino = get(&format!("/no-existe?token={TOKEN}"));
        assert_eq!(estado(otro_camino).await, StatusCode::UNAUTHORIZED);
        let otro_metodo = get(&format!("/?token={TOKEN}")).method("POST");
        assert_eq!(estado(otro_metodo).await, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn el_encabezado_authorization_basta() {
        let portador = get("/").header("Authorization", format!("Bearer {TOKEN}"));
        assert_eq!(estado(portador).await, StatusCode::OK);
    }

    async fn texto(respuesta: Response) -> String {
        let cuerpo = axum::body::to_bytes(respuesta.into_body(), usize::MAX);
        String::from_utf8(cuerpo.await.unwrap().to_vec()).unwrap()
    }

    fn encabezado(respuesta: &Response, nombre: header::HeaderName) -> &str {
        respuesta.headers()[nombre].to_str().unwrap()
    }

    #[tokio::test]
    async fn el_401_de_la_api_es_json() {
        let respuesta = no_autorizado("/api/participantes");
        assert_eq!(respuesta.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(
            encabezado(&respuesta, header::WWW_AUTHENTICATE),
            "Bearer realm=\"osr\""
        );
        assert_eq!(
            encabezado(&respuesta, header::CONTENT_TYPE),
            "application/json"
        );
        let cuerpo: serde_json::Value = serde_json::from_str(&texto(respuesta).await).unwrap();
        assert_eq!(
            cuerpo,
            serde_json::json!({"error": "no_autorizado", "mensaje": "Falta el token de la sesión o no es válido."})
        );
    }

    #[tokio::test]
    async fn el_401_de_las_demas_familias_es_una_pagina() {
        for camino in ["/", "/recursos/x.js", "/api"] {
            let respuesta = no_autorizado(camino);
            assert_eq!(respuesta.status(), StatusCode::UNAUTHORIZED);
            assert_eq!(
                encabezado(&respuesta, header::WWW_AUTHENTICATE),
                "Bearer realm=\"osr\""
            );
            assert!(encabezado(&respuesta, header::CONTENT_TYPE).starts_with("text/html"));
            let cuerpo = texto(respuesta).await;
            assert!(
                cuerpo.contains("Esta dirección necesita el enlace que mostró osr al arrancar.")
            );
        }
    }

    #[tokio::test]
    async fn sin_token_la_respuesta_es_la_de_su_familia() {
        let respuesta = rutas(Arc::from(TOKEN))
            .oneshot(get("/api/no-existe").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(
            encabezado(&respuesta, header::CONTENT_TYPE),
            "application/json"
        );
    }

    #[tokio::test]
    async fn con_token_una_ruta_inexistente_da_404() {
        let portador = get("/no-existe").header("Authorization", format!("Bearer {TOKEN}"));
        assert_eq!(estado(portador).await, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn cada_ruta_de_la_lista_atiende_sus_metodos_con_token() {
        for ruta in RUTAS {
            for metodo in ruta.metodos {
                let pedido = get(ruta.camino)
                    .method(*metodo)
                    .header("Authorization", format!("Bearer {TOKEN}"));
                let estado = estado(pedido).await;
                let no_atendida = [StatusCode::NOT_FOUND, StatusCode::METHOD_NOT_ALLOWED];
                assert!(
                    !no_atendida.contains(&estado),
                    "{metodo} {}: {estado}",
                    ruta.camino
                );
            }
        }
    }

    #[test]
    #[should_panic(expected = "no tiene manejador")]
    fn una_ruta_sin_manejador_no_arranca() {
        let _ = atender("/sin-manejador", MethodFilter::GET);
    }
}
