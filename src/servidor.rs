//! El servidor local: escucha, rutas y token (SRV-01, SRV-05).

use std::sync::Arc;

use axum::Router;
use axum::extract::{Request, State};
use axum::http::{Method, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;

use crate::token;

/// La pantalla sin proyecto, con `Abrir proyecto` y `Crear proyecto`.
const INICIO: &str = include_str!("inicio.html");

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

/// Rechaza con 401 todo pedido sin el token, antes de llegar a la ruta.
async fn exigir_token(State(token): State<Arc<str>>, pedido: Request, siguiente: Next) -> Response {
    if autenticado(&pedido, &token) {
        siguiente.run(pedido).await
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}

async fn inicio() -> Html<&'static str> {
    Html(INICIO)
}

/// Las rutas del servidor. La capa del token va al final, así cubre también
/// las rutas inexistentes.
pub fn rutas(token: Arc<str>) -> Router {
    Router::new()
        .route("/", get(inicio))
        .layer(middleware::from_fn_with_state(token, exigir_token))
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
}
