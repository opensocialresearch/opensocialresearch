//! El servidor local: rutas, Host, origen, token y encabezados de todas las
//! respuestas (SRV-01, SRV-05 a SRV-10).

use std::sync::Arc;

use axum::Router;
use axum::extract::{Request, State};
use axum::http::{HeaderValue, Method, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{MethodFilter, MethodRouter, on};

use crate::rutas::RUTAS;
use crate::{registro, token};

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

/// El cuerpo del 403 por un pedido que viene de otra página (SRV-07).
const ORIGEN_NO_PERMITIDO: &str =
    r#"{"error":"origen_no_permitido","mensaje":"El pedido viene de otra página."}"#;

/// Los encabezados que lleva toda respuesta, también las de error (SRV-09).
const ENCABEZADOS: [(&str, &str); 7] = [
    (
        "content-security-policy",
        "default-src 'none'; script-src 'self'; style-src 'self'; img-src 'self' data: blob:; media-src 'self' blob:; font-src 'self'; connect-src 'self'; form-action 'self'; base-uri 'none'; frame-ancestors 'none'",
    ),
    ("x-content-type-options", "nosniff"),
    ("referrer-policy", "no-referrer"),
    ("cache-control", "no-store"),
    ("cross-origin-opener-policy", "same-origin"),
    ("cross-origin-resource-policy", "same-origin"),
    ("x-frame-options", "DENY"),
];

/// Pone los encabezados de SRV-09 en una respuesta, reemplazando los que ya
/// tuviera con el mismo nombre.
async fn con_encabezados(mut respuesta: Response) -> Response {
    for (nombre, valor) in ENCABEZADOS {
        respuesta
            .headers_mut()
            .insert(nombre, HeaderValue::from_static(valor));
    }
    respuesta
}

/// Lo que el servidor necesita saber de su arranque para validar un pedido.
#[derive(Clone)]
struct Sesion {
    token: Arc<str>,
    puerto: u16,
    /// Dónde se anota cada pedido rechazado, el registro técnico salvo en
    /// las pruebas unitarias, que no escriben en la carpeta de estado.
    anotar: fn(&str, &str),
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

/// Un origen propio es `http://127.0.0.1:P`, `http://[::1]:P` o
/// `http://localhost:P`, tal como lo serializa el navegador (SRV-07).
fn origen_propio(origen: &str, puerto: u16) -> bool {
    ["127.0.0.1", "[::1]", "localhost"]
        .iter()
        .any(|propio| origen == format!("http://{propio}:{puerto}"))
}

/// El valor de un encabezado como texto, vacío si no es ASCII visible.
fn valor<'a>(pedido: &'a Request, nombre: &str) -> Option<&'a str> {
    let valor = pedido.headers().get(nombre)?;
    Some(valor.to_str().unwrap_or(""))
}

/// Las tres reglas de SRV-07. Un `Origin` tiene que ser propio, un método
/// que no es GET ni HEAD tiene que traerlo, y `Sec-Fetch-Site` tiene que ser
/// `same-origin` o `none`, salvo en la entrada con `?token=` cuando el
/// navegador navega hasta ella.
fn origen_permitido(pedido: &Request, puerto: u16) -> bool {
    let lectura = matches!(*pedido.method(), Method::GET | Method::HEAD);
    let origen = match valor(pedido, "origin") {
        Some(origen) => origen_propio(origen, puerto),
        None => lectura,
    };
    let navega = entrada(pedido).is_some() && valor(pedido, "sec-fetch-mode") == Some("navigate");
    let sitio = match valor(pedido, "sec-fetch-site") {
        Some(sitio) => ["same-origin", "none"].contains(&sitio) || navega,
        None => true,
    };
    origen && sitio
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

/// El token que trae la entrada `GET /?token=<token>`, si el pedido es ella.
fn entrada(pedido: &Request) -> Option<&str> {
    (pedido.method() == Method::GET && pedido.uri().path() == "/")
        .then(|| pedido.uri().query().and_then(token_de_la_consulta))
        .flatten()
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
    [portador, entrada(pedido)]
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

/// El motivo y la respuesta del rechazo de un pedido, si no pasa las
/// validaciones. Primero el Host (SRV-06), después el origen (SRV-07) y al
/// final el token (SRV-05).
fn rechazo(pedido: &Request, sesion: &Sesion) -> Option<(&'static str, Response)> {
    if !host_propio(valor(pedido, "host"), sesion.puerto) {
        return Some(("host_no_permitido", prohibido(HOST_NO_PERMITIDO)));
    }
    if !origen_permitido(pedido, sesion.puerto) {
        return Some(("origen_no_permitido", prohibido(ORIGEN_NO_PERMITIDO)));
    }
    if !autenticado(pedido, &sesion.token) {
        return Some(("no_autorizado", no_autorizado(pedido.uri().path())));
    }
    None
}

/// La línea del registro técnico de un pedido rechazado, con el camino sin
/// la consulta, para que nunca quede el token de la entrada (SRV-10, SRV-12).
fn linea_de_rechazo(pedido: &Request, estado: StatusCode, motivo: &str) -> String {
    let camino = pedido.uri().path();
    let estado = estado.as_u16();
    format!(
        "pedido rechazado {} {camino} {estado} {motivo}",
        pedido.method()
    )
}

/// Rechaza el pedido antes de llegar a la ruta, sin ejecutar nada ni leer
/// ningún archivo, y lo anota en el registro técnico.
async fn vigilar(State(sesion): State<Sesion>, pedido: Request, siguiente: Next) -> Response {
    match rechazo(&pedido, &sesion) {
        Some((motivo, respuesta)) => {
            let linea = linea_de_rechazo(&pedido, respuesta.status(), motivo);
            (sesion.anotar)("aviso", &linea);
            respuesta
        }
        None => siguiente.run(pedido).await,
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

/// Las rutas del servidor que escucha en `puerto`, armadas solo desde
/// `RUTAS`. La capa que valida el pedido va al final, así cubre también las
/// rutas inexistentes, que con token dan 404, y la de los encabezados de
/// SRV-09 la envuelve, así llegan también a los 401, 403, 404 y 405.
pub fn rutas(token: Arc<str>, puerto: u16) -> Router {
    rutas_con(Sesion {
        token,
        puerto,
        anotar: registro::anotar,
    })
}

fn rutas_con(sesion: Sesion) -> Router {
    let mut router = Router::new();
    for ruta in RUTAS {
        for metodo in ruta.metodos {
            let metodo = Method::from_bytes(metodo.as_bytes()).expect("método de RUTAS");
            let filtro = MethodFilter::try_from(metodo).expect("método de RUTAS");
            router = router.route(ruta.camino, atender(ruta.camino, filtro));
        }
    }
    router
        .layer(middleware::from_fn_with_state(sesion, vigilar))
        .layer(middleware::map_response(con_encabezados))
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use axum::body::Body;
    use tower::ServiceExt;

    const TOKEN: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQ";
    const PUERTO: u16 = 40123;

    thread_local! {
        /// Lo que anotó el servidor de prueba en este hilo, en lugar del
        /// registro técnico.
        static ANOTADO: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) };
    }

    fn anotar_en_memoria(nivel: &str, mensaje: &str) {
        ANOTADO.with(|anotado| anotado.borrow_mut().push(format!("{nivel} {mensaje}")));
    }

    fn servidor() -> Router {
        rutas_con(Sesion {
            token: Arc::from(TOKEN),
            puerto: PUERTO,
            anotar: anotar_en_memoria,
        })
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
        let otro_metodo = get(&format!("/?token={TOKEN}"))
            .method("POST")
            .header("Origin", format!("http://127.0.0.1:{PUERTO}"));
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
    fn los_origenes_propios_son_los_tres_de_loopback_con_el_puerto() {
        for origen in [
            "http://127.0.0.1:40123",
            "http://[::1]:40123",
            "http://localhost:40123",
        ] {
            assert!(origen_propio(origen, PUERTO), "{origen}");
        }
        for origen in [
            "http://ejemplo.invalid",
            "null",
            "http://127.0.0.1:40124",
            "https://127.0.0.1:40123",
            "http://127.0.0.1",
            "",
        ] {
            assert!(!origen_propio(origen, PUERTO), "{origen}");
        }
    }

    const PROPIO: &str = "http://127.0.0.1:40123";

    /// Si un pedido con token válido pasa las reglas de SRV-07.
    fn pasa(metodo: &str, camino: &str, encabezados: &[(&str, &str)]) -> bool {
        let mut pedido = get(camino).method(metodo);
        for (nombre, valor) in encabezados {
            pedido = pedido.header(*nombre, *valor);
        }
        origen_permitido(&pedido.body(Body::empty()).unwrap(), PUERTO)
    }

    #[test]
    fn un_origin_tiene_que_ser_propio() {
        assert!(pasa("GET", "/", &[]));
        assert!(pasa("GET", "/", &[("Origin", PROPIO)]));
        assert!(!pasa("GET", "/", &[("Origin", "http://ejemplo.invalid")]));
        assert!(!pasa("GET", "/", &[("Origin", "null")]));
        assert!(!pasa("POST", "/", &[("Origin", "http://127.0.0.1:40124")]));
        assert!(!pasa("GET", "/", &[("Origin", "http://127.0.0.1:4012ñ")]));
    }

    #[test]
    fn fuera_de_get_y_head_hace_falta_origin() {
        assert!(pasa("HEAD", "/", &[]));
        for metodo in ["POST", "PUT", "DELETE", "OPTIONS"] {
            assert!(!pasa(metodo, "/api/x", &[]), "{metodo}");
            assert!(pasa(metodo, "/api/x", &[("Origin", PROPIO)]), "{metodo}");
        }
    }

    #[test]
    fn sec_fetch_site_tiene_que_ser_del_mismo_origen_o_ninguno() {
        for sitio in ["same-origin", "none"] {
            assert!(pasa("GET", "/", &[("Sec-Fetch-Site", sitio)]), "{sitio}");
        }
        for sitio in ["cross-site", "same-site", ""] {
            assert!(!pasa("GET", "/", &[("Sec-Fetch-Site", sitio)]), "{sitio}");
        }
    }

    #[test]
    fn la_entrada_navegada_se_exceptua_de_sec_fetch_site() {
        let entrada = format!("/?token={TOKEN}");
        let navega = [
            ("Sec-Fetch-Site", "cross-site"),
            ("Sec-Fetch-Mode", "navigate"),
        ];
        assert!(pasa("GET", &entrada, &navega));
        let sin_navegar = [("Sec-Fetch-Site", "cross-site"), ("Sec-Fetch-Mode", "cors")];
        assert!(!pasa("GET", &entrada, &sin_navegar));
        assert!(!pasa("GET", "/", &navega));
        assert!(!pasa("GET", &format!("/x?token={TOKEN}"), &navega));
        let con_origen_ajeno = [
            ("Sec-Fetch-Site", "cross-site"),
            ("Sec-Fetch-Mode", "navigate"),
            ("Origin", "http://ejemplo.invalid"),
        ];
        assert!(!pasa("GET", &entrada, &con_origen_ajeno));
    }

    #[tokio::test]
    async fn otro_origen_da_403_con_su_cuerpo_antes_que_el_token() {
        let pedido = get("/api/participantes")
            .method("POST")
            .header("Origin", "http://ejemplo.invalid");
        let respuesta = servidor()
            .oneshot(pedido.body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(respuesta.status(), StatusCode::FORBIDDEN);
        let cuerpo: serde_json::Value = serde_json::from_str(&texto(respuesta).await).unwrap();
        assert_eq!(
            cuerpo,
            serde_json::json!({"error": "origen_no_permitido", "mensaje": "El pedido viene de otra página."})
        );
    }

    /// Los siete encabezados exactos de SRV-09, como los escribe el escenario.
    fn tiene_los_siete(respuesta: &Response) {
        let esperados = [
            (
                "Content-Security-Policy",
                "default-src 'none'; script-src 'self'; style-src 'self'; img-src 'self' data: blob:; media-src 'self' blob:; font-src 'self'; connect-src 'self'; form-action 'self'; base-uri 'none'; frame-ancestors 'none'",
            ),
            ("X-Content-Type-Options", "nosniff"),
            ("Referrer-Policy", "no-referrer"),
            ("Cache-Control", "no-store"),
            ("Cross-Origin-Opener-Policy", "same-origin"),
            ("Cross-Origin-Resource-Policy", "same-origin"),
            ("X-Frame-Options", "DENY"),
        ];
        for (nombre, valor) in esperados {
            let encontrados: Vec<_> = respuesta.headers().get_all(nombre).iter().collect();
            assert_eq!(encontrados, [valor], "{nombre} en {}", respuesta.status());
        }
        assert!(
            !respuesta
                .headers()
                .keys()
                .any(|nombre| nombre.as_str().starts_with("access-control-")),
            "{}",
            respuesta.status()
        );
    }

    #[tokio::test]
    async fn toda_respuesta_lleva_los_siete_encabezados() {
        let con_token = format!("Bearer {TOKEN}");
        let pedidos = [
            (get("/").header("Authorization", &con_token), StatusCode::OK),
            (get("/"), StatusCode::UNAUTHORIZED),
            (
                Request::builder()
                    .uri("/")
                    .header("Host", "ejemplo.invalid:40123"),
                StatusCode::FORBIDDEN,
            ),
            (
                get("/").method("POST").header("Authorization", &con_token),
                StatusCode::FORBIDDEN,
            ),
            (
                get("/no-existe").header("Authorization", &con_token),
                StatusCode::NOT_FOUND,
            ),
            (
                get("/")
                    .method("OPTIONS")
                    .header("Authorization", &con_token)
                    .header("Origin", PROPIO),
                StatusCode::METHOD_NOT_ALLOWED,
            ),
        ];
        for (pedido, esperado) in pedidos {
            let respuesta = servidor()
                .oneshot(pedido.body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(respuesta.status(), esperado);
            tiene_los_siete(&respuesta);
        }
    }

    #[tokio::test]
    async fn los_encabezados_reemplazan_a_los_de_la_ruta() {
        let respuesta = (
            [
                (header::CACHE_CONTROL, "max-age=60"),
                (header::X_FRAME_OPTIONS, "SAMEORIGIN"),
            ],
            "x",
        )
            .into_response();
        tiene_los_siete(&con_encabezados(respuesta).await);
    }

    #[tokio::test]
    async fn cada_rechazo_deja_una_linea_con_metodo_camino_estado_y_motivo() {
        ANOTADO.with(|anotado| anotado.borrow_mut().clear());
        let con_token = format!("Bearer {TOKEN}");
        let pedidos = [
            get("/?token=otro"),
            Request::builder()
                .uri("/api/x?a=1")
                .header("Host", "ejemplo.invalid:40123"),
            get("/api/x")
                .method("DELETE")
                .header("Authorization", &con_token),
            get("/").header("Authorization", &con_token),
        ];
        for pedido in pedidos {
            servidor()
                .oneshot(pedido.body(Body::empty()).unwrap())
                .await
                .unwrap();
        }
        assert_eq!(
            ANOTADO.with(|anotado| anotado.borrow().clone()),
            [
                "aviso pedido rechazado GET / 401 no_autorizado",
                "aviso pedido rechazado GET /api/x 403 host_no_permitido",
                "aviso pedido rechazado DELETE /api/x 403 origen_no_permitido",
            ]
        );
    }

    #[test]
    #[should_panic(expected = "no tiene manejador")]
    fn una_ruta_sin_manejador_no_arranca() {
        let _ = atender("/sin-manejador", MethodFilter::GET);
    }
}
