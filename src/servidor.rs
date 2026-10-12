//! El servidor local: rutas, Host y token (SRV-01, SRV-05, SRV-06).

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

/// El cuerpo del 403 por un Host ajeno o ausente (SRV-06).
const HOST_NO_PERMITIDO: &str =
    r#"{"error":"host_no_permitido","mensaje":"El pedido no viene de esta computadora."}"#;

/// Lo que el servidor necesita saber de su arranque para validar un pedido.
#[derive(Clone)]
struct Sesion {
    token: Arc<str>,
    puerto: u16,
}

/// Un Host propio es `127.0.0.1:P`, `[::1]:P` o `localhost:P`, sin
/// distinguir mayúsculas (SRV-06). Se lee el encabezado `Host` tal cual, sin
/// mirar `X-Forwarded-Host` ni la autoridad del camino.
fn host_propio(host: Option<&str>, puerto: u16) -> bool {
    host.is_some_and(|host| {
        ["127.0.0.1", "[::1]", "localhost"]
            .iter()
            .any(|propio| host.eq_ignore_ascii_case(&format!("{propio}:{puerto}")))
    })
}

/// Un 403 con su cuerpo JSON, en todas las familias.
fn prohibido(cuerpo: &'static str) -> Response {
    let tipo = [(header::CONTENT_TYPE, "application/json")];
    (StatusCode::FORBIDDEN, tipo, cuerpo).into_response()
}

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

/// Rechaza el pedido antes de llegar a la ruta, sin ejecutar nada ni leer
/// ningún archivo. Primero el Host (SRV-06), después el token (SRV-05).
async fn vigilar(State(sesion): State<Sesion>, pedido: Request, siguiente: Next) -> Response {
    let host = pedido
        .headers()
        .get(header::HOST)
        .and_then(|valor| valor.to_str().ok());
    if !host_propio(host, sesion.puerto) {
        return prohibido(HOST_NO_PERMITIDO);
    }
    if !autenticado(&pedido, &sesion.token) {
        return no_autorizado(pedido.uri().path());
    }
    siguiente.run(pedido).await
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

/// Las rutas del servidor que escucha en `puerto`, armadas solo desde
/// `RUTAS`. La capa que valida el pedido va al final, así cubre también las
/// rutas inexistentes, que con token dan 404.
pub fn rutas(token: Arc<str>, puerto: u16) -> Router {
    let mut router = Router::new();
    for ruta in RUTAS {
        for metodo in ruta.metodos {
            let metodo = Method::from_bytes(metodo.as_bytes()).expect("método de RUTAS");
            let filtro = MethodFilter::try_from(metodo).expect("método de RUTAS");
            router = router.route(ruta.camino, atender(ruta.camino, filtro));
        }
    }
    let sesion = Sesion { token, puerto };
    router.layer(middleware::from_fn_with_state(sesion, vigilar))
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use axum::body::Body;
    use tower::ServiceExt;

    const TOKEN: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQ";
    const PUERTO: u16 = 40123;

    fn servidor() -> Router {
        rutas(Arc::from(TOKEN), PUERTO)
    }

    async fn estado(pedido: axum::http::request::Builder) -> StatusCode {
        servidor()
            .oneshot(pedido.body(Body::empty()).unwrap())
            .await
            .unwrap()
            .status()
    }

    /// Un pedido con el Host propio, como los que manda el navegador.
    fn get(camino: &str) -> axum::http::request::Builder {
        Request::builder()
            .uri(camino)
            .header("Host", format!("127.0.0.1:{PUERTO}"))
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
        let respuesta = servidor()
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
        let respuesta = servidor()
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
    fn los_host_propios_son_los_tres_de_loopback_con_el_puerto() {
        for host in [
            "127.0.0.1:40123",
            "[::1]:40123",
            "localhost:40123",
            "LocalHost:40123",
        ] {
            assert!(host_propio(Some(host), PUERTO), "{host}");
        }
        for host in [
            "ejemplo.invalid:40123",
            "127.0.0.1:40124",
            "127.0.0.1",
            "localhost",
            "127.0.0.1:40123.ejemplo.invalid",
            "",
        ] {
            assert!(!host_propio(Some(host), PUERTO), "{host}");
        }
        assert!(!host_propio(None, PUERTO));
    }

    /// Un pedido con token válido y el Host dado, o sin Host.
    async fn con_host(host: Option<&str>) -> Response {
        let mut pedido = Request::builder()
            .uri("/")
            .header("Authorization", format!("Bearer {TOKEN}"));
        if let Some(host) = host {
            pedido = pedido.header("Host", host);
        }
        servidor()
            .oneshot(pedido.body(Body::empty()).unwrap())
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn un_host_ajeno_o_ausente_da_403_aunque_traiga_el_token() {
        for host in [Some("ejemplo.invalid:40123"), Some("127.0.0.1:40124"), None] {
            let respuesta = con_host(host).await;
            assert_eq!(respuesta.status(), StatusCode::FORBIDDEN, "{host:?}");
            assert_eq!(
                encabezado(&respuesta, header::CONTENT_TYPE),
                "application/json"
            );
            let cuerpo: serde_json::Value = serde_json::from_str(&texto(respuesta).await).unwrap();
            assert_eq!(
                cuerpo,
                serde_json::json!({"error": "host_no_permitido", "mensaje": "El pedido no viene de esta computadora."})
            );
        }
    }

    #[tokio::test]
    async fn el_host_se_valida_antes_que_el_token() {
        let pedido = Request::builder()
            .uri("/")
            .header("Host", "ejemplo.invalid:40123");
        assert_eq!(estado(pedido).await, StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn un_host_propio_con_token_es_atendido() {
        let respuesta = con_host(Some("LOCALHOST:40123")).await;
        assert_eq!(respuesta.status(), StatusCode::OK);
    }

    #[test]
    #[should_panic(expected = "no tiene manejador")]
    fn una_ruta_sin_manejador_no_arranca() {
        let _ = atender("/sin-manejador", MethodFilter::GET);
    }
}
