//! El audio de una entrevista, con pedidos por rangos (SRV-11).
//!
//! Devuelve los bytes del original sin transformarlos y los lee por trozos,
//! sin cargar el archivo entero en memoria, así un audio de cientos de MB no
//! pesa más que un trozo por pedido en curso (D-39).

use std::io;
use std::path::{Path, PathBuf};

use axum::body::{Body, Bytes};
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use futures_util::Stream;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, SeekFrom};

/// El `<ext>` con que TRA-02 guarda cada formato y su `Content-Type`.
const FORMATOS: [(&str, &str); 5] = [
    ("ogg", "audio/ogg"),
    ("opus", "audio/ogg"),
    ("m4a", "audio/mp4"),
    ("mp3", "audio/mpeg"),
    ("wav", "audio/wav"),
];

/// Cuántos bytes se leen del disco por vez.
const TROZO: u64 = 65_536;

/// Por qué no se puede servir el audio, con su estado y su cuerpo.
#[derive(Debug, PartialEq)]
enum Fallo {
    /// La entrevista no existe, o no tiene audio importado.
    NoExiste,
    /// La ficha dice que el audio se eliminó (TRA-23).
    Eliminado,
    /// La ficha existe pero el original no está en disco.
    Faltante,
}

impl Fallo {
    fn respuesta(&self) -> Response {
        let (estado, cuerpo) = match self {
            Fallo::NoExiste => (StatusCode::NOT_FOUND, r#"{"error":"no_existe"}"#),
            Fallo::Eliminado => (StatusCode::GONE, r#"{"error":"audio_eliminado"}"#),
            Fallo::Faltante => (StatusCode::NOT_FOUND, r#"{"error":"audio_faltante"}"#),
        };
        let tipo = [(header::CONTENT_TYPE, "application/json")];
        (estado, tipo, cuerpo).into_response()
    }
}

/// Un seudónimo cumple `^[A-Z][A-Z0-9]{1,15}$` (PAR-01). Así el camino no
/// puede traer `..`, `/` ni nada que salga de la carpeta de entrevistas.
fn seudonimo_valido(id: &str) -> bool {
    let mut letras = id.chars();
    let primera = letras.next().is_some_and(|c| c.is_ascii_uppercase());
    let resto = letras.all(|c| c.is_ascii_uppercase() || c.is_ascii_digit());
    primera && resto && (2..=16).contains(&id.len())
}

/// Si la ficha dice `estado = "eliminado"` (PRY-07).
fn eliminado(ficha: &Path) -> bool {
    std::fs::read_to_string(ficha)
        .ok()
        .and_then(|texto| texto.parse::<toml::Table>().ok())
        .is_some_and(|ficha| ficha.get("estado").and_then(|v| v.as_str()) == Some("eliminado"))
}

/// El original de la entrevista `id` y su `Content-Type`. La entrevista
/// existe si tiene `entrevista.toml` (TRA-01), y su audio es el que nombra
/// su ficha `<id>.<ext>.ficha.toml`, que sigue ahí si el original falta o se
/// eliminó (PRY-07).
fn ubicar(raiz: &Path, id: &str) -> Result<(PathBuf, &'static str), Fallo> {
    if !seudonimo_valido(id) {
        return Err(Fallo::NoExiste);
    }
    let carpeta = raiz.join("identificable/entrevistas").join(id);
    if !carpeta.join("entrevista.toml").is_file() {
        return Err(Fallo::NoExiste);
    }
    let (ext, tipo) = FORMATOS
        .into_iter()
        .find(|(ext, _)| carpeta.join(format!("{id}.{ext}.ficha.toml")).is_file())
        .ok_or(Fallo::NoExiste)?;
    if eliminado(&carpeta.join(format!("{id}.{ext}.ficha.toml"))) {
        return Err(Fallo::Eliminado);
    }
    Ok((carpeta.join(format!("{id}.{ext}")), tipo))
}

/// Lo que pide el encabezado `Range` sobre un archivo de `tamano` bytes.
#[derive(Debug, PartialEq)]
enum Rango {
    /// Sin `Range`, o con uno que no se atiende: el archivo entero.
    Entero,
    /// Desde el byte `a` hasta el `b`, los dos incluidos.
    Parcial(u64, u64),
    /// Empieza después del último byte.
    Fuera,
}

/// Lee `bytes=a-b` y `bytes=a-`, con `b` recortado al último byte. Lo demás,
/// el rango final `bytes=-n`, varios rangos o algo que no se entiende, se
/// ignora y se responde el archivo entero, como permite RFC 9110 (D-39).
fn rango(valor: Option<&str>, tamano: u64) -> Rango {
    let Some((a, b)) = valor
        .and_then(|v| v.strip_prefix("bytes="))
        .and_then(|v| v.split_once('-'))
    else {
        return Rango::Entero;
    };
    let Ok(a) = a.parse::<u64>() else {
        return Rango::Entero;
    };
    let fin = match b {
        "" => None,
        b => match b.parse::<u64>() {
            Ok(b) if b >= a => Some(b),
            _ => return Rango::Entero,
        },
    };
    if a >= tamano {
        return Rango::Fuera;
    }
    let ultimo = tamano - 1;
    Rango::Parcial(a, fin.map_or(ultimo, |b| b.min(ultimo)))
}

/// Los `largo` bytes que siguen en `archivo`, por trozos. Si el archivo se
/// acaba antes, el cuerpo termina con un error y la conexión se corta.
fn trozos(archivo: File, largo: u64) -> impl Stream<Item = io::Result<Bytes>> {
    futures_util::stream::try_unfold((archivo, largo), |(mut archivo, restante)| async move {
        if restante == 0 {
            return Ok(None);
        }
        let mut trozo = vec![0; restante.min(TROZO) as usize];
        let leidos = archivo.read(&mut trozo).await?;
        if leidos == 0 {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }
        trozo.truncate(leidos);
        Ok(Some((
            Bytes::from(trozo),
            (archivo, restante - leidos as u64),
        )))
    })
}

/// Abre el original y devuelve su tamaño, si es un archivo.
async fn abrir(ruta: &Path) -> Option<(File, u64)> {
    let archivo = File::open(ruta).await.ok()?;
    let datos = archivo.metadata().await.ok()?;
    datos.is_file().then_some((archivo, datos.len()))
}

/// La respuesta con los encabezados de SRV-11 y el cuerpo dado.
fn con_encabezados(estado: StatusCode, tipo: &'static str, largo: u64, cuerpo: Body) -> Response {
    let mut respuesta = (estado, cuerpo).into_response();
    let encabezados = respuesta.headers_mut();
    encabezados.insert(header::CONTENT_TYPE, HeaderValue::from_static(tipo));
    encabezados.insert(header::ACCEPT_RANGES, HeaderValue::from_static("bytes"));
    encabezados.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_static("inline"),
    );
    encabezados.insert(header::CONTENT_LENGTH, HeaderValue::from(largo));
    respuesta
}

/// Manda el original entero o el tramo que pide `Range`.
async fn enviar(ruta: &Path, tipo: &'static str, pedido: Option<&str>) -> Response {
    let Some((mut archivo, tamano)) = abrir(ruta).await else {
        return Fallo::Faltante.respuesta();
    };
    let (estado, desde, hasta) = match rango(pedido, tamano) {
        Rango::Entero => (StatusCode::OK, 0, tamano),
        Rango::Parcial(a, b) => (StatusCode::PARTIAL_CONTENT, a, b + 1),
        Rango::Fuera => {
            let mut respuesta =
                con_encabezados(StatusCode::RANGE_NOT_SATISFIABLE, tipo, 0, Body::empty());
            let total = format!("bytes */{tamano}");
            respuesta.headers_mut().insert(
                header::CONTENT_RANGE,
                HeaderValue::try_from(total).expect("ASCII"),
            );
            return respuesta;
        }
    };
    if archivo.seek(SeekFrom::Start(desde)).await.is_err() {
        return Fallo::Faltante.respuesta();
    }
    let largo = hasta - desde;
    let cuerpo = Body::from_stream(trozos(archivo, largo));
    let mut respuesta = con_encabezados(estado, tipo, largo, cuerpo);
    if estado == StatusCode::PARTIAL_CONTENT {
        let tramo = format!("bytes {desde}-{}/{tamano}", hasta - 1);
        respuesta.headers_mut().insert(
            header::CONTENT_RANGE,
            HeaderValue::try_from(tramo).expect("ASCII"),
        );
    }
    respuesta
}

/// `GET /audio/<entrevista>` en el proyecto abierto, si hay uno. `pedido` es
/// el valor del encabezado `Range`.
pub async fn servir(proyecto: Option<&Path>, entrevista: &str, pedido: Option<&str>) -> Response {
    let Some(raiz) = proyecto else {
        return Fallo::NoExiste.respuesta();
    };
    match ubicar(raiz, entrevista) {
        Ok((ruta, tipo)) => enviar(&ruta, tipo, pedido).await,
        Err(fallo) => fallo.respuesta(),
    }
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use futures_util::TryStreamExt;

    /// El proyecto de prueba de PRY-17, que estas pruebas solo leen.
    fn datos() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/datos/proyecto")
    }

    fn original(id: &str, archivo: &str) -> Vec<u8> {
        std::fs::read(
            datos()
                .join("identificable/entrevistas")
                .join(id)
                .join(archivo),
        )
        .unwrap()
    }

    async fn bytes(respuesta: Response) -> Vec<u8> {
        axum::body::to_bytes(respuesta.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec()
    }

    fn encabezado(respuesta: &Response, nombre: header::HeaderName) -> Option<&str> {
        respuesta.headers().get(nombre).map(|v| v.to_str().unwrap())
    }

    /// Una copia de la entrevista `id` del proyecto de prueba en una carpeta
    /// temporal, para poder cambiarla.
    fn copia(id: &str) -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let destino = tmp.path().join("identificable/entrevistas").join(id);
        std::fs::create_dir_all(&destino).unwrap();
        let origen = datos().join("identificable/entrevistas").join(id);
        for entrada in std::fs::read_dir(origen).unwrap() {
            let entrada = entrada.unwrap();
            if entrada.file_type().unwrap().is_file() {
                std::fs::copy(entrada.path(), destino.join(entrada.file_name())).unwrap();
            }
        }
        tmp
    }

    #[test]
    fn un_seudonimo_valido_es_mayuscula_y_de_2_a_16_caracteres() {
        for id in ["E01", "AB", "E", "A123456789012345"] {
            assert_eq!(seudonimo_valido(id), id.len() >= 2, "{id}");
        }
        for id in [
            "",
            "e01",
            "1E",
            "E-1",
            "..",
            "E01/..",
            "E0é",
            "A1234567890123456",
            "Ñ1",
        ] {
            assert!(!seudonimo_valido(id), "{id}");
        }
    }

    #[test]
    fn los_rangos_que_se_atienden() {
        assert_eq!(rango(None, 100), Rango::Entero);
        assert_eq!(rango(Some("bytes=0-99"), 1000), Rango::Parcial(0, 99));
        assert_eq!(rango(Some("bytes=5-5"), 1000), Rango::Parcial(5, 5));
        assert_eq!(rango(Some("bytes=10-"), 100), Rango::Parcial(10, 99));
        assert_eq!(rango(Some("bytes=10-500"), 100), Rango::Parcial(10, 99));
        assert_eq!(rango(Some("bytes=99-99"), 100), Rango::Parcial(99, 99));
        assert_eq!(rango(Some("bytes=100-"), 100), Rango::Fuera);
        assert_eq!(rango(Some("bytes=100-200"), 100), Rango::Fuera);
        assert_eq!(rango(Some("bytes=0-"), 0), Rango::Fuera);
    }

    #[test]
    fn los_rangos_que_se_ignoran_dan_el_archivo_entero() {
        for valor in [
            "bytes=-10",
            "bytes=5-3",
            "bytes=0-1,5-6",
            "bytes=x-1",
            "bytes=1-x",
            "bytes=200-x",
            "bytes 0-1",
            "items=0-1",
            "bytes=10",
        ] {
            assert_eq!(rango(Some(valor), 100), Rango::Entero, "{valor}");
        }
    }

    #[test]
    fn la_entrevista_se_ubica_por_su_ficha() {
        let (ruta, tipo) = ubicar(&datos(), "E01").unwrap();
        assert_eq!(ruta, datos().join("identificable/entrevistas/E01/E01.ogg"));
        assert_eq!(tipo, "audio/ogg");
        let (ruta, tipo) = ubicar(&datos(), "E02").unwrap();
        assert_eq!(ruta, datos().join("identificable/entrevistas/E02/E02.m4a"));
        assert_eq!(tipo, "audio/mp4");
    }

    #[test]
    fn una_entrevista_inexistente_o_un_seudonimo_invalido_no_existen() {
        for id in ["E09", "..", "e01", "identificable"] {
            assert_eq!(ubicar(&datos(), id), Err(Fallo::NoExiste), "{id}");
        }
    }

    #[test]
    fn una_carpeta_sin_entrevista_toml_no_es_una_entrevista() {
        let tmp = copia("E01");
        let carpeta = tmp.path().join("identificable/entrevistas/E01");
        std::fs::remove_file(carpeta.join("entrevista.toml")).unwrap();
        assert_eq!(ubicar(tmp.path(), "E01"), Err(Fallo::NoExiste));
    }

    #[test]
    fn una_entrevista_sin_ficha_de_audio_no_tiene_audio() {
        let tmp = copia("E01");
        let carpeta = tmp.path().join("identificable/entrevistas/E01");
        std::fs::remove_file(carpeta.join("E01.ogg.ficha.toml")).unwrap();
        assert_eq!(ubicar(tmp.path(), "E01"), Err(Fallo::NoExiste));
    }

    #[test]
    fn cada_formato_tiene_su_tipo() {
        for (ext, tipo) in [
            ("ogg", "audio/ogg"),
            ("opus", "audio/ogg"),
            ("m4a", "audio/mp4"),
            ("mp3", "audio/mpeg"),
            ("wav", "audio/wav"),
        ] {
            let tmp = copia("E01");
            let carpeta = tmp.path().join("identificable/entrevistas/E01");
            std::fs::rename(
                carpeta.join("E01.ogg.ficha.toml"),
                carpeta.join(format!("E01.{ext}.ficha.toml")),
            )
            .unwrap();
            let (ruta, encontrado) = ubicar(tmp.path(), "E01").unwrap();
            assert_eq!(ruta, carpeta.join(format!("E01.{ext}")));
            assert_eq!(encontrado, tipo, "{ext}");
        }
    }

    /// Cambia `estado = "presente"` por otro valor en la ficha de E02.
    fn con_estado(tmp: &tempfile::TempDir, estado: &str) {
        let ficha = tmp
            .path()
            .join("identificable/entrevistas/E02/E02.m4a.ficha.toml");
        let texto = std::fs::read_to_string(&ficha).unwrap();
        let cambiado = texto.replace("estado = \"presente\"", estado);
        assert_ne!(texto, cambiado);
        std::fs::write(ficha, cambiado).unwrap();
    }

    #[tokio::test]
    async fn un_audio_eliminado_da_410() {
        let tmp = copia("E02");
        let carpeta = tmp.path().join("identificable/entrevistas/E02");
        std::fs::remove_file(carpeta.join("E02.m4a")).unwrap();
        con_estado(
            &tmp,
            "estado = \"eliminado\"\neliminado = 2026-10-10T09:00:00-06:00\nmotivo_eliminacion = \"retencion\"",
        );
        let respuesta = servir(Some(tmp.path()), "E02", None).await;
        assert_eq!(respuesta.status(), StatusCode::GONE);
        assert_eq!(
            encabezado(&respuesta, header::CONTENT_TYPE),
            Some("application/json")
        );
        assert_eq!(bytes(respuesta).await, br#"{"error":"audio_eliminado"}"#);
    }

    #[test]
    fn solo_el_estado_eliminado_cuenta_como_eliminado() {
        let tmp = copia("E02");
        con_estado(&tmp, "estado = \"otro\"");
        assert!(ubicar(tmp.path(), "E02").is_ok());
        let ficha = tmp
            .path()
            .join("identificable/entrevistas/E02/E02.m4a.ficha.toml");
        std::fs::write(&ficha, "estado = ").unwrap();
        assert!(ubicar(tmp.path(), "E02").is_ok());
        assert!(!eliminado(&tmp.path().join("no-existe.toml")));
    }

    #[tokio::test]
    async fn un_audio_que_falta_en_disco_da_404_audio_faltante() {
        let tmp = copia("E01");
        let carpeta = tmp.path().join("identificable/entrevistas/E01");
        std::fs::remove_file(carpeta.join("E01.ogg")).unwrap();
        let respuesta = servir(Some(tmp.path()), "E01", None).await;
        assert_eq!(respuesta.status(), StatusCode::NOT_FOUND);
        assert_eq!(bytes(respuesta).await, br#"{"error":"audio_faltante"}"#);
        std::fs::create_dir(carpeta.join("E01.ogg")).unwrap();
        let respuesta = servir(Some(tmp.path()), "E01", None).await;
        assert_eq!(bytes(respuesta).await, br#"{"error":"audio_faltante"}"#);
    }

    #[tokio::test]
    async fn sin_proyecto_o_sin_entrevista_da_404_no_existe() {
        for (proyecto, id) in [(None, "E01"), (Some(datos()), "E09")] {
            let respuesta = servir(proyecto.as_deref(), id, None).await;
            assert_eq!(respuesta.status(), StatusCode::NOT_FOUND);
            assert_eq!(bytes(respuesta).await, br#"{"error":"no_existe"}"#);
        }
    }

    #[tokio::test]
    async fn sin_range_responde_el_archivo_entero() {
        for (id, archivo, tipo) in [
            ("E01", "E01.ogg", "audio/ogg"),
            ("E02", "E02.m4a", "audio/mp4"),
        ] {
            let respuesta = servir(Some(&datos()), id, None).await;
            let esperado = original(id, archivo);
            assert_eq!(respuesta.status(), StatusCode::OK);
            assert_eq!(encabezado(&respuesta, header::CONTENT_TYPE), Some(tipo));
            assert_eq!(encabezado(&respuesta, header::ACCEPT_RANGES), Some("bytes"));
            assert_eq!(
                encabezado(&respuesta, header::CONTENT_DISPOSITION),
                Some("inline")
            );
            let largo = esperado.len().to_string();
            assert_eq!(
                encabezado(&respuesta, header::CONTENT_LENGTH),
                Some(largo.as_str())
            );
            assert_eq!(encabezado(&respuesta, header::CONTENT_RANGE), None);
            assert!(esperado.len() as u64 > TROZO);
            assert_eq!(bytes(respuesta).await, esperado);
        }
    }

    #[tokio::test]
    async fn un_rango_responde_206_con_esos_bytes() {
        let esperado = original("E01", "E01.ogg");
        let total = esperado.len();
        for (valor, a, b) in [
            ("bytes=0-99", 0, 99),
            ("bytes=1000-70000", 1000, 70000),
            (
                &format!("bytes={}-", total - 10) as &str,
                total - 10,
                total - 1,
            ),
        ] {
            let respuesta = servir(Some(&datos()), "E01", Some(valor)).await;
            assert_eq!(respuesta.status(), StatusCode::PARTIAL_CONTENT, "{valor}");
            let tramo = format!("bytes {a}-{b}/{total}");
            assert_eq!(
                encabezado(&respuesta, header::CONTENT_RANGE),
                Some(tramo.as_str())
            );
            let largo = (b - a + 1).to_string();
            assert_eq!(
                encabezado(&respuesta, header::CONTENT_LENGTH),
                Some(largo.as_str())
            );
            assert_eq!(encabezado(&respuesta, header::ACCEPT_RANGES), Some("bytes"));
            assert_eq!(bytes(respuesta).await, esperado[a..=b]);
        }
    }

    #[tokio::test]
    async fn un_rango_despues_del_ultimo_byte_da_416() {
        let total = original("E01", "E01.ogg").len();
        let valor = format!("bytes={total}-");
        let respuesta = servir(Some(&datos()), "E01", Some(&valor)).await;
        assert_eq!(respuesta.status(), StatusCode::RANGE_NOT_SATISFIABLE);
        let total = format!("bytes */{total}");
        assert_eq!(
            encabezado(&respuesta, header::CONTENT_RANGE),
            Some(total.as_str())
        );
        assert_eq!(encabezado(&respuesta, header::CONTENT_LENGTH), Some("0"));
        assert!(bytes(respuesta).await.is_empty());
    }

    #[tokio::test]
    async fn los_trozos_cortan_en_el_largo_y_fallan_si_el_archivo_se_acaba() {
        let tmp = tempfile::tempdir().unwrap();
        let ruta = tmp.path().join("diez");
        std::fs::write(&ruta, b"0123456789").unwrap();
        let leido: Vec<Bytes> = trozos(File::open(&ruta).await.unwrap(), 4)
            .try_collect()
            .await
            .unwrap();
        assert_eq!(leido.concat(), b"0123");
        let corto: io::Result<Vec<Bytes>> = trozos(File::open(&ruta).await.unwrap(), 20)
            .try_collect()
            .await;
        assert_eq!(corto.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    }
}
