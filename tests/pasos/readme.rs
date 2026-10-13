//! Pasos que leen las secciones del README generado (PRY-16).

use std::fs;

use cucumber::then;

use super::Mundo;

/// El texto de la sección `## <nombre>` del Markdown, hasta la siguiente
/// sección del mismo nivel o el final, sin el título.
fn seccion<'a>(markdown: &'a str, nombre: &str) -> &'a str {
    let titulo = format!("\n## {nombre}\n");
    let inicio = markdown
        .find(&titulo)
        .unwrap_or_else(|| panic!("falta la sección «{nombre}»"))
        + titulo.len();
    let resto = &markdown[inicio..];
    &resto[..resto.find("\n## ").unwrap_or(resto.len())]
}

/// Las órdenes de una sección, las líneas de sus bloques de código, sean
/// sangrados con cuatro espacios o entre cercas de tres acentos graves.
fn ordenes(seccion: &str) -> Vec<&str> {
    let mut ordenes = Vec::new();
    let mut en_cerca = false;
    for linea in seccion.lines() {
        if linea.trim_start().starts_with("```") {
            en_cerca = !en_cerca;
        } else if en_cerca {
            ordenes.push(linea.trim());
        } else if let Some(orden) = linea.strip_prefix("    ") {
            ordenes.push(orden.trim());
        }
    }
    ordenes.retain(|orden| !orden.is_empty());
    ordenes
}

fn ordenes_de(mundo: &Mundo, nombre: &str, archivo: &str) -> Vec<String> {
    let texto = fs::read_to_string(mundo.ruta(archivo)).expect("leer el README");
    ordenes(seccion(&texto, nombre))
        .into_iter()
        .map(String::from)
        .collect()
}

#[then(regex = r#"^la sección «([^»]+)» de "([^"]+)" tiene una orden que contiene "([^"]+)"$"#)]
fn orden_que_contiene(mundo: &mut Mundo, nombre: String, archivo: String, texto: String) {
    let ordenes = ordenes_de(mundo, &nombre, &archivo);
    assert!(
        ordenes.iter().any(|orden| orden.contains(&texto)),
        "ninguna orden contiene {texto:?}: {ordenes:?}"
    );
}

/// Otra orden, además de la que copia todo, que excluye la carpeta.
#[then(
    regex = r#"^la sección «([^»]+)» de "([^"]+)" tiene otra orden que copia todo salvo "([^"]+)"$"#
)]
fn orden_que_excluye(mundo: &mut Mundo, nombre: String, archivo: String, carpeta: String) {
    let ordenes = ordenes_de(mundo, &nombre, &archivo);
    let excluye = |orden: &&String| {
        [" ", "=", " '", "='"]
            .iter()
            .any(|union| orden.contains(&format!("--exclude{union}{carpeta}")))
    };
    let (excluyen, completas): (Vec<&String>, Vec<&String>) = ordenes.iter().partition(excluye);
    assert!(
        !excluyen.is_empty(),
        "ninguna orden excluye {carpeta:?}: {ordenes:?}"
    );
    assert!(
        !completas.is_empty(),
        "falta la orden que copia todo: {ordenes:?}"
    );
}
