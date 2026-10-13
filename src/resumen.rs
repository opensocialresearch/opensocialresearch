//! La página `/` con un proyecto abierto: el resumen del tramo 1 (PRY-22).
//!
//! Un proyecto vacío empieza por el diseño de la investigación (D-49). Uno
//! con participantes o entrevistas avisa que esta versión todavía no los
//! muestra. Solo se cuentan líneas y carpetas, sin leer su contenido, y nada
//! cambia entre dos aperturas sin cambios en los archivos.

use std::fs;
use std::path::Path;

use crate::abierto::Abierto;
use crate::aplicacion::Investigacion;

const PAGINA: &str = include_str!("paginas/resumen.html");
const INICIO: &str = include_str!("paginas/inicio.html");
const AVISO: &str = r#"<svg viewBox="0 0 12 12" aria-hidden="true"><path d="M6 1.6 L10.8 10.2 H1.2 Z" fill="none" stroke="currentColor" stroke-width="1.4" stroke-linejoin="round"/><path d="M5.4 4.6h1.2v2.8h-1.2zM5.4 8.1h1.2v1.2h-1.2z" fill="currentColor"/></svg>"#;
const VACIO: &str = include_str!("paginas/resumen_vacio.html");
const CON_DATOS: &str = include_str!("paginas/resumen_con_datos.html");

/// El texto con `&`, `<`, `>`, `"` y `'` escritos como entidades, para que
/// se muestre como texto y nunca como código de la página.
pub fn escapar(texto: &str) -> String {
    let mut escapado = String::with_capacity(texto.len());
    for letra in texto.chars() {
        match letra {
            '&' => escapado.push_str("&amp;"),
            '<' => escapado.push_str("&lt;"),
            '>' => escapado.push_str("&gt;"),
            '"' => escapado.push_str("&quot;"),
            '\'' => escapado.push_str("&#39;"),
            otra => escapado.push(otra),
        }
    }
    escapado
}

/// Si `participantes.csv` tiene alguna línea con texto después del
/// encabezado. Un archivo que no se puede leer cuenta como sin líneas.
fn hay_participantes(carpeta: &Path) -> bool {
    fs::read_to_string(carpeta.join("participantes/participantes.csv"))
        .is_ok_and(|texto| texto.lines().skip(1).any(|linea| !linea.trim().is_empty()))
}

/// Si `identificable/entrevistas/` tiene alguna carpeta.
fn hay_entrevistas(carpeta: &Path) -> bool {
    fs::read_dir(carpeta.join("identificable/entrevistas"))
        .is_ok_and(|mut entradas| entradas.any(|entrada| entrada.is_ok_and(|e| e.path().is_dir())))
}

/// Un proyecto está vacío sin participantes ni entrevistas.
pub fn vacio(carpeta: &Path) -> bool {
    !hay_participantes(carpeta) && !hay_entrevistas(carpeta)
}

/// La plantilla con cada `{{clave}}` reemplazada por su valor, en una sola
/// pasada, así un valor que contenga otra marca no se vuelve a reemplazar.
fn llenar(plantilla: &str, valores: &[(&str, &str)]) -> String {
    let mut lleno = String::with_capacity(plantilla.len());
    let mut resto = plantilla;
    while let Some(inicio) = resto.find("{{") {
        lleno.push_str(&resto[..inicio]);
        let despues = &resto[inicio + 2..];
        let fin = despues
            .find("}}")
            .expect("marca sin cerrar en la plantilla");
        let clave = &despues[..fin];
        let (_, valor) = valores
            .iter()
            .find(|(nombre, _)| *nombre == clave)
            .unwrap_or_else(|| panic!("la plantilla pide {clave}"));
        lleno.push_str(valor);
        resto = &despues[fin + 2..];
    }
    lleno.push_str(resto);
    lleno
}

/// La página del resumen del proyecto abierto.
pub fn pagina(abierto: &Abierto) -> String {
    let bloque = if vacio(&abierto.carpeta) {
        VACIO
    } else {
        CON_DATOS
    };
    let nombre = escapar(&abierto.nombre);
    let carpeta = escapar(&abierto.carpeta.to_string_lossy());
    let bloque = llenar(bloque, &[("nombre", &nombre), ("carpeta", &carpeta)]);
    llenar(PAGINA, &[("bloque", &bloque), ("nombre", &nombre)])
}

/// La fila de una investigación de la lista, con su botón y su aviso.
fn fila(numero: usize, investigacion: &Investigacion) -> String {
    let nombre = escapar(&investigacion.nombre);
    let carpeta = escapar(&investigacion.carpeta);
    format!(
        "      <li>\n        <span><strong>{nombre}</strong><span class=\"mono\">{carpeta}</span></span>\n        \
         <form data-accion=\"/api/proyecto/abrir\"><input type=\"hidden\" name=\"carpeta\" value=\"{carpeta}\">\
         <button type=\"submit\" class=\"boton\" data-esperando=\"Abriendo…\">Abrir</button>\
         <div id=\"abrir-error-{numero}\" class=\"aviso-error\" role=\"alert\" hidden>{AVISO}<span></span></div></form>\n      </li>\n"
    )
}

/// La pantalla sin proyecto con las investigaciones de la carpeta de la
/// aplicación (PRY-20) y el formulario para crear otra (PRY-19).
pub fn inicio(investigaciones: &[Investigacion]) -> String {
    let filas: String = investigaciones
        .iter()
        .enumerate()
        .map(|(numero, investigacion)| fila(numero + 1, investigacion))
        .collect();
    llenar(INICIO, &[("investigaciones", &filas), ("aviso", AVISO)])
}

#[cfg(test)]
mod pruebas {
    use super::*;
    use std::path::PathBuf;

    fn datos() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/datos/proyecto")
    }

    /// Un proyecto vacío, con el encabezado de participantes y la carpeta de
    /// entrevistas, como lo deja `osr nuevo`.
    fn vacio_en(tmp: &Path) -> PathBuf {
        let raiz = tmp.join("estudio");
        fs::create_dir_all(raiz.join("participantes")).unwrap();
        fs::create_dir_all(raiz.join("identificable/entrevistas")).unwrap();
        fs::write(
            raiz.join("participantes/participantes.csv"),
            "seudonimo,fecha_alta,estado\r\n",
        )
        .unwrap();
        raiz
    }

    #[test]
    fn escapa_los_cinco_caracteres_de_html() {
        assert_eq!(
            escapar(r#"<b>Estudio</b> & "otro" 'x'"#),
            "&lt;b&gt;Estudio&lt;/b&gt; &amp; &quot;otro&quot; &#39;x&#39;"
        );
        assert_eq!(escapar("año"), "año");
    }

    #[test]
    fn llenar_reemplaza_cada_marca_una_sola_vez() {
        assert_eq!(
            llenar("<{{a}}|{{b}}>", &[("a", "{{b}}"), ("b", "dos")]),
            "<{{b}}|dos>"
        );
        assert_eq!(llenar("sin marcas", &[]), "sin marcas");
    }

    #[test]
    #[should_panic(expected = "la plantilla pide c")]
    fn una_marca_sin_valor_es_un_error_de_la_plantilla() {
        llenar("{{c}}", &[]);
    }

    #[test]
    fn un_nombre_con_marcas_se_muestra_tal_cual() {
        let tmp = tempfile::tempdir().unwrap();
        let raiz = vacio_en(tmp.path());
        let pagina = pagina(&Abierto {
            carpeta: raiz,
            nombre: "{{carpeta}}".into(),
        });
        assert!(pagina.contains("<title>{{carpeta}} · Open Social Research</title>"));
    }

    #[test]
    fn la_lista_escapa_y_numera_cada_investigacion() {
        let lista = [
            Investigacion {
                carpeta: "uno".into(),
                nombre: "<b>Uno</b>".into(),
            },
            Investigacion {
                carpeta: "dos".into(),
                nombre: "Dos & más".into(),
            },
        ];
        let pagina = inicio(&lista);
        assert!(pagina.contains("<strong>&lt;b&gt;Uno&lt;/b&gt;</strong>"));
        assert!(pagina.contains("<strong>Dos &amp; más</strong>"));
        assert!(pagina.contains(r#"name="carpeta" value="uno""#));
        assert!(pagina.contains(r#"id="abrir-error-1""#));
        assert!(pagina.contains(r#"id="abrir-error-2""#));
        assert!(pagina.find("Uno").unwrap() < pagina.find("Dos").unwrap());
        assert!(!pagina.contains("{{"));
        assert!(pagina.contains("Crear investigación"));
    }

    #[test]
    fn un_proyecto_recien_creado_esta_vacio() {
        let tmp = tempfile::tempdir().unwrap();
        let raiz = vacio_en(tmp.path());
        assert!(vacio(&raiz));
        fs::write(
            raiz.join("participantes/participantes.csv"),
            "seudonimo,fecha_alta,estado\r\n  \r\n\r\n",
        )
        .unwrap();
        assert!(vacio(&raiz), "las líneas en blanco no cuentan");
    }

    #[test]
    fn una_fila_de_participante_o_una_entrevista_lo_llenan() {
        let tmp = tempfile::tempdir().unwrap();
        let raiz = vacio_en(tmp.path());
        fs::write(raiz.join("identificable/entrevistas/notas.txt"), "x").unwrap();
        assert!(vacio(&raiz), "un archivo suelto no es una entrevista");
        fs::create_dir(raiz.join("identificable/entrevistas/E01")).unwrap();
        assert!(!vacio(&raiz));
        let otro = tempfile::tempdir().unwrap();
        let raiz = vacio_en(otro.path());
        fs::write(
            raiz.join("participantes/participantes.csv"),
            "seudonimo,fecha_alta,estado\r\nE01,2026-10-10,activo\r\n",
        )
        .unwrap();
        assert!(!vacio(&raiz));
        assert!(!vacio(&datos()));
    }

    #[test]
    fn sin_archivos_cuenta_como_vacio() {
        let tmp = tempfile::tempdir().unwrap();
        assert!(vacio(tmp.path()));
    }

    #[test]
    fn el_resumen_vacio_empieza_por_el_diseno() {
        let tmp = tempfile::tempdir().unwrap();
        let raiz = vacio_en(tmp.path());
        let pagina = pagina(&Abierto {
            carpeta: raiz.clone(),
            nombre: "<b>Estudio</b> & otro".into(),
        });
        assert!(pagina.contains(
            "<title>&lt;b&gt;Estudio&lt;/b&gt; &amp; otro · Open Social Research</title>"
        ));
        assert!(!pagina.contains("<b>"));
        assert!(pagina.contains(&format!("{}", raiz.display())));
        assert!(pagina.contains("Proyecto nuevo"));
        assert!(pagina.contains("Diseño de la investigación"));
        assert!(!pagina.contains("{{"));
    }

    #[test]
    fn el_resumen_con_datos_avisa_que_no_los_muestra() {
        let pagina = pagina(&Abierto {
            carpeta: datos(),
            nombre: "Proyecto de prueba inventado".into(),
        });
        assert!(pagina.contains("Proyecto abierto"));
        assert!(pagina.contains("Este proyecto ya tiene participantes o entrevistas."));
        assert!(!pagina.contains("Diseño de la investigación"));
        assert!(!pagina.contains("{{"));
    }
}
