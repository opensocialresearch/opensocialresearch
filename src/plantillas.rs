//! Contenido de los archivos que `osr nuevo` escribe en un proyecto vacío.
//!
//! Cada función devuelve los bytes exactos del archivo, de modo que el mismo
//! contenido lógico produce siempre el mismo texto (PRY-06).

use serde::Serialize;

/// Alcances del consentimiento que se escriben al crear (PAR-06).
const ALCANCES: [(&str, &str); 5] = [
    ("grabar", "Grabar la entrevista"),
    ("citar_textualmente", "Citar textualmente sus palabras"),
    ("usar_puntos_mapa", "Usar sus puntos del mapa"),
    ("archivar", "Archivar los datos al terminar"),
    (
        "reutilizar",
        "Reutilizar los datos en otras investigaciones",
    ),
];

/// Escribe `texto` como cadena básica de TOML, con comillas dobles.
///
/// El nombre ya llega sin caracteres de control (PRY-01), así que solo hace
/// falta escapar la barra invertida y la comilla doble.
fn cadena_toml(texto: &str) -> String {
    let escapado = texto.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{escapado}\"")
}

/// `proyecto.toml` con las claves obligatorias y los alcances (PRY-03, PAR-06).
pub fn proyecto_toml(nombre: &str, creado: &str) -> String {
    let mut toml = format!(
        "formato = \"osr-proyecto\"\nversion_formato = 1\nnombre = {}\ncreado = {creado}\n",
        cadena_toml(nombre)
    );
    for (id, etiqueta) in ALCANCES {
        toml.push_str(&format!(
            "\n[[consentimiento.alcances]]\nid = \"{id}\"\netiqueta = \"{etiqueta}\"\n"
        ));
    }
    toml
}

/// Encabezado de `participantes/consentimientos.csv`, con una columna por
/// alcance en el orden de `proyecto.toml` (PAR-07).
pub fn consentimientos_csv() -> String {
    let mut columnas = vec![
        "seudonimo",
        "version_formulario",
        "fecha",
        "modalidad",
        "respaldo",
        "respaldo_inicio_s",
        "respaldo_fin_s",
    ]
    .into_iter()
    .map(String::from)
    .collect::<Vec<_>>();
    columnas.extend(ALCANCES.iter().map(|(id, _)| format!("alcance_{id}")));
    format!("{}\r\n", columnas.join(","))
}

pub const SENSIBILIDAD_CSV: &str = "ruta,nivel\r\n";
pub const PARTICIPANTES_CSV: &str = "seudonimo,fecha_alta,estado\r\n";
pub const CORRESPONDENCIA_CSV: &str = "seudonimo,nombre,contacto,nota\r\n";

/// `bitacora/bitacora.md`, que después es solo de la persona (PRY-08).
pub const BITACORA_MD: &str = "\
# Bitácora

Esta bitácora es de quien investiga y Open Social Research no la vuelve a
escribir. Cada entrada empieza con un título de nivel dos con la fecha y el
tipo de entrada, que es uno de estos cuatro, decisión metodológica, reflexión,
analítica o de campo, por ejemplo `## 2026-10-11 · reflexión`. Lo que la
herramienta hace queda anotado aparte, en `registro.jsonl`.
";

/// Una línea del registro de eventos, con las claves en el orden de PRY-08.
#[derive(Serialize)]
struct Evento<'a, D: Serialize> {
    fecha: &'a str,
    tipo: &'a str,
    detalle: D,
}

/// Una línea del registro de eventos con su salto final.
fn evento(fecha: &str, tipo: &str, detalle: impl Serialize) -> String {
    let evento = Evento {
        fecha,
        tipo,
        detalle,
    };
    let linea = serde_json::to_string(&evento).expect("un evento siempre se puede escribir");
    format!("{linea}\n")
}

/// La línea `proyecto_creado` del registro de eventos, con su salto final.
pub fn evento_creado(fecha: &str) -> String {
    evento(fecha, "proyecto_creado", serde_json::Map::new())
}

/// El detalle del evento `verificacion`, con las claves en el orden de PRY-11.
#[derive(Serialize, Default)]
struct Verificacion {
    modificados_fuera: Vec<String>,
    con_error: Vec<String>,
    faltantes: Vec<String>,
    originales_alterados: Vec<String>,
    no_registrados: u64,
    temporales_eliminados: u64,
}

/// La línea `verificacion` de una apertura. Hasta PRY-11 la verificación no
/// revisa nada y el detalle sale con las listas vacías y los contadores en
/// cero (D-27).
pub fn evento_verificacion(fecha: &str) -> String {
    evento(fecha, "verificacion", Verificacion::default())
}

/// El detalle de `bloqueo_abandonado`, con lo que decía el bloqueo
/// reemplazado (D-28).
#[derive(Serialize)]
struct Abandonado<'a> {
    pid: Option<u32>,
    inicio: Option<&'a str>,
}

/// La línea `bloqueo_abandonado`, con el PID y el inicio del bloqueo
/// reemplazado, o `null` si no se pudieron leer (PRY-05).
pub fn evento_bloqueo_abandonado(fecha: &str, pid: Option<u32>, inicio: Option<&str>) -> String {
    evento(fecha, "bloqueo_abandonado", Abandonado { pid, inicio })
}

/// `.osr/bloqueo`, con el número del proceso y su inicio (PRY-05).
pub fn bloqueo(pid: u32, inicio: &str) -> String {
    format!("pid = {pid}\ninicio = {inicio}\n")
}

/// `README.md` de un proyecto recién creado (PRY-16).
pub fn readme(nombre: &str, hoy: &str) -> String {
    format!(
        "\
<!-- Generado por Open Social Research. Los cambios a mano se guardan aparte y este archivo se reemplaza. -->

# {nombre}

## Qué es esta carpeta

Es el proyecto «{nombre}» de Open Social Research, en la versión 1 del
formato de carpeta. Este README se generó el {hoy}. Todo lo que hay aquí se
puede leer sin la herramienta, con un editor de texto, una hoja de cálculo o R.

## Carpetas

| Carpeta | Qué contiene | Zona |
|---|---|---|
| `./` | La identidad del proyecto, este README y las marcas de sensibilidad | de trabajo |
| `bitacora/` | La bitácora de quien investiga y el registro de eventos de la herramienta | de trabajo |
| `participantes/` | El registro de participantes por seudónimo y sus consentimientos | de trabajo |
| `identificable/` | Todo lo que identifica a una persona | identificable |
| `identificable/consentimientos/` | Respaldos de los consentimientos con sus fichas | identificable |
| `identificable/entrevistas/` | Una carpeta por entrevista, con su audio y su transcripción | identificable |
| `.osr/` | El índice interno, que se puede borrar porque se reconstruye | interna |

## Tipos de archivo

| Archivo | Formato | Qué contiene | Cómo se abre sin la herramienta | Especificación |
|---|---|---|---|---|
| `proyecto.toml` | TOML 1.0 | Nombre, versión del formato y configuración | Editor de texto | PRY-03 |
| `README.md` | Markdown | Esta explicación | Editor de texto | PRY-16 |
| `sensibilidad.csv` | CSV | Marcas de sensibilidad asignadas a mano | Hoja de cálculo o R | PAR-14 |
| `bitacora/bitacora.md` | Markdown | Entradas fechadas de quien investiga | Editor de texto | PRY-08 |
| `bitacora/registro.jsonl` | JSON Lines | Un evento de la herramienta por línea | Editor de texto | PRY-08 |
| `participantes/participantes.csv` | CSV | Participantes por seudónimo | Hoja de cálculo o R | PAR-02 |
| `participantes/consentimientos.csv` | CSV | Consentimiento de cada participante por alcance | Hoja de cálculo o R | PAR-07 |
| `identificable/correspondencia.csv` | CSV | La tabla que liga cada seudónimo con la persona | Hoja de cálculo o R | PAR-03 |
| `.osr/indice.sqlite` | SQLite | Copia derivada de los archivos de texto | SQLite | PRY-10 |

## Zonas y sensibilidad

La zona identificable es todo lo que está bajo `identificable/` y la zona de
trabajo es todo lo demás salvo `.osr/`. La zona identificable solo se copia a
un disco cifrado.

Cada archivo tiene uno de cuatro niveles de sensibilidad. `identificable` es
todo archivo bajo `identificable/`, `sensible` son los archivos de la zona de
trabajo con datos de participantes, `no_sensible` son `proyecto.toml` y
`README.md`, y `sin_marca` son los demás archivos de la zona de trabajo hasta
que se marquen.

| Archivo | Nivel |
|---|---|
| `proyecto.toml` | no_sensible |
| `README.md` | no_sensible |
| `sensibilidad.csv` | sensible |
| `bitacora/bitacora.md` | sensible |
| `bitacora/registro.jsonl` | sensible |
| `participantes/participantes.csv` | sensible |
| `participantes/consentimientos.csv` | sensible |
| `identificable/correspondencia.csv` | identificable |

## Cómo respaldar

Para copiar la carpeta entera a un disco cifrado, conservando los permisos:

    rsync -a ./ /ruta/al/disco/cifrado/proyecto/

Para copiar todo salvo la zona identificable a un destino sin cifrar:

    rsync -a --exclude identificable/ ./ /ruta/al/respaldo/proyecto/

La carpeta `.osr/` se puede omitir en cualquier respaldo, porque la
herramienta la reconstruye.

## Qué no hacer

- No guardar esta carpeta en servicios de sincronización en la nube.
- No editar los archivos de `origen/` ni los originales.
- Borrar un archivo en un disco SSD no garantiza que sea irrecuperable, por lo
  que el disco tiene que estar cifrado.
"
    )
}

#[cfg(test)]
mod pruebas {
    use super::*;

    #[test]
    fn cadena_toml_escapa_barra_y_comilla() {
        assert_eq!(cadena_toml(r#"a\b"c"#), r#""a\\b\"c""#);
        assert_eq!(cadena_toml("año"), "\"año\"");
    }

    #[test]
    fn proyecto_toml_es_toml_con_las_claves_y_los_alcances() {
        let texto = proyecto_toml("Dice \"hola\"", "2026-10-11T21:30:05-06:00");
        let valor: toml::Table = texto.parse().unwrap();
        assert_eq!(valor["formato"].as_str(), Some("osr-proyecto"));
        assert_eq!(valor["version_formato"].as_integer(), Some(1));
        assert_eq!(valor["nombre"].as_str(), Some("Dice \"hola\""));
        assert_eq!(
            valor["creado"].as_datetime().unwrap().to_string(),
            "2026-10-11T21:30:05-06:00"
        );
        let alcances = valor["consentimiento"]["alcances"].as_array().unwrap();
        let ids: Vec<_> = alcances.iter().map(|a| a["id"].as_str().unwrap()).collect();
        assert_eq!(
            ids,
            [
                "grabar",
                "citar_textualmente",
                "usar_puntos_mapa",
                "archivar",
                "reutilizar"
            ]
        );
        assert_eq!(
            alcances[0]["etiqueta"].as_str(),
            Some("Grabar la entrevista")
        );
    }

    #[test]
    fn consentimientos_tiene_una_columna_por_alcance() {
        assert_eq!(
            consentimientos_csv(),
            "seudonimo,version_formulario,fecha,modalidad,respaldo,respaldo_inicio_s,\
             respaldo_fin_s,alcance_grabar,alcance_citar_textualmente,\
             alcance_usar_puntos_mapa,alcance_archivar,alcance_reutilizar\r\n"
        );
    }

    #[test]
    fn evento_creado_tiene_las_claves_en_orden() {
        assert_eq!(
            evento_creado("2026-10-11T21:30:05-06:00"),
            "{\"fecha\":\"2026-10-11T21:30:05-06:00\",\"tipo\":\"proyecto_creado\",\"detalle\":{}}\n"
        );
    }

    #[test]
    fn evento_verificacion_tiene_el_detalle_de_pry_11_en_orden() {
        assert_eq!(
            evento_verificacion("2026-10-12T05:00:00-06:00"),
            "{\"fecha\":\"2026-10-12T05:00:00-06:00\",\"tipo\":\"verificacion\",\"detalle\":\
             {\"modificados_fuera\":[],\"con_error\":[],\"faltantes\":[],\"originales_alterados\":[],\
             \"no_registrados\":0,\"temporales_eliminados\":0}}\n"
        );
    }

    #[test]
    fn evento_bloqueo_abandonado_tiene_pid_e_inicio() {
        assert_eq!(
            evento_bloqueo_abandonado("2026-10-12T06:00:00-06:00", Some(7), Some("x")),
            "{\"fecha\":\"2026-10-12T06:00:00-06:00\",\"tipo\":\"bloqueo_abandonado\",\
             \"detalle\":{\"pid\":7,\"inicio\":\"x\"}}\n"
        );
        assert!(
            evento_bloqueo_abandonado("f", None, None).contains("{\"pid\":null,\"inicio\":null}")
        );
    }

    #[test]
    fn bloqueo_tiene_pid_e_inicio() {
        assert_eq!(
            bloqueo(42, "2026-10-12T05:00:00-06:00"),
            "pid = 42\ninicio = 2026-10-12T05:00:00-06:00\n"
        );
    }

    #[test]
    fn readme_empieza_con_el_aviso_y_nombra_el_proyecto() {
        let texto = readme("Estudio de prueba", "2026-10-11");
        assert!(texto.starts_with(
            "<!-- Generado por Open Social Research. Los cambios a mano se guardan aparte y \
             este archivo se reemplaza. -->\n"
        ));
        assert!(texto.contains("# Estudio de prueba\n"));
        assert!(texto.contains("se generó el 2026-10-11."));
        assert!(texto.contains("rsync -a --exclude identificable/"));
    }
}
