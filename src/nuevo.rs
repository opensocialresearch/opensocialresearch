//! `osr nuevo`: crea un proyecto vacío (PRY-01).
//!
//! El proyecto se arma en una carpeta de preparación al lado de la ruta pedida
//! y se mueve a su lugar con un solo `rename`, así que ante un error no queda
//! nada a medias en la ruta.

use std::fmt;
use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use crate::plantillas;

/// Momento de la creación, ya escrito en los dos formatos que se usan.
pub struct Momento {
    /// Marca de tiempo ISO 8601 con desplazamiento local y segundos.
    pub marca: String,
    /// Fecha ISO 8601 sin hora.
    pub fecha: String,
}

impl Momento {
    pub fn ahora() -> Momento {
        let ahora = jiff::Zoned::now();
        Momento {
            marca: ahora.strftime("%Y-%m-%dT%H:%M:%S%:z").to_string(),
            fecha: ahora.strftime("%Y-%m-%d").to_string(),
        }
    }

    /// La marca compacta `AAAAMMDDTHHMMSS` en hora local de PRY-02.
    pub fn compacta(&self) -> String {
        self.marca[..19].replace(['-', ':'], "")
    }
}

/// Errores de `osr nuevo`, con el código de salida de la tabla de PRY-01.
#[derive(Debug)]
pub enum Error {
    NombreInvalido,
    Ocupada(PathBuf),
    SinPadre(PathBuf),
    Escritura(PathBuf, io::Error),
}

impl Error {
    pub fn codigo(&self) -> i32 {
        match self {
            Error::NombreInvalido => 2,
            Error::Ocupada(_) | Error::SinPadre(_) => 3,
            Error::Escritura(..) => 6,
        }
    }
}

/// El motivo de un error de escritura en español, sin el texto del sistema.
pub fn motivo(error: &io::Error) -> &'static str {
    match error.kind() {
        io::ErrorKind::PermissionDenied => "sin permiso de escritura",
        io::ErrorKind::StorageFull => "el disco está lleno",
        io::ErrorKind::ReadOnlyFilesystem => "el disco es de solo lectura",
        _ => "error de escritura del sistema",
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NombreInvalido => write!(
                f,
                "el nombre del proyecto tiene que tener entre 1 y 200 caracteres, sin saltos de línea."
            ),
            Error::Ocupada(ruta) => write!(
                f,
                "la carpeta {} ya existe y no está vacía. Hay que elegir una carpeta nueva o vacía.",
                ruta.display()
            ),
            Error::SinPadre(padre) => write!(f, "no existe la carpeta {}.", padre.display()),
            Error::Escritura(ruta, error) => write!(
                f,
                "no se pudo crear el proyecto en {} ({}).",
                ruta.display(),
                motivo(error)
            ),
        }
    }
}

/// Un nombre válido tiene de 1 a 200 caracteres y ninguno de control.
fn nombre_valido(nombre: &str) -> bool {
    let largo = nombre.chars().count();
    (1..=200).contains(&largo) && !nombre.chars().any(char::is_control)
}

/// El nombre pedido o, si falta, el último componente de la ruta.
fn nombre_del_proyecto(ruta: &Path, nombre: Option<&str>) -> Result<String, Error> {
    let nombre = match nombre {
        Some(nombre) => nombre.to_string(),
        None => std::path::absolute(ruta)
            .ok()
            .and_then(|absoluta| {
                absoluta
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
            })
            .unwrap_or_default(),
    };
    if nombre_valido(&nombre) {
        Ok(nombre)
    } else {
        Err(Error::NombreInvalido)
    }
}

/// Comprueba que la ruta sea nueva con un padre existente, o una carpeta vacía.
fn comprobar_destino(ruta: &Path) -> Result<(), Error> {
    if ruta.exists() {
        let vacia = fs::read_dir(ruta).is_ok_and(|mut entradas| entradas.next().is_none());
        return if vacia {
            Ok(())
        } else {
            Err(Error::Ocupada(ruta.to_path_buf()))
        };
    }
    let padre = carpeta_padre(ruta);
    if padre.is_dir() {
        Ok(())
    } else {
        Err(Error::SinPadre(padre.to_path_buf()))
    }
}

/// La carpeta que contiene a `ruta`; la actual si la ruta es un solo nombre.
fn carpeta_padre(ruta: &Path) -> &Path {
    match ruta.parent() {
        Some(padre) if !padre.as_os_str().is_empty() => padre,
        _ => Path::new("."),
    }
}

/// Crea una carpeta con permisos 0700 sin importar la umask (SRV-13).
pub fn carpeta(ruta: &Path) -> io::Result<()> {
    fs::create_dir(ruta)?;
    fs::set_permissions(ruta, fs::Permissions::from_mode(0o700))
}

/// Escribe un archivo gestionado con permisos 0600 sin importar la umask.
pub fn archivo(ruta: &Path, contenido: &str) -> io::Result<()> {
    fs::write(ruta, contenido)?;
    fs::set_permissions(ruta, fs::Permissions::from_mode(0o600))
}

/// Escribe en `raiz` la estructura de PRY-02 y el bloqueo del proceso.
fn armar(raiz: &Path, nombre: &str, momento: &Momento) -> io::Result<()> {
    carpeta(raiz)?;
    archivo(
        &raiz.join("proyecto.toml"),
        &plantillas::proyecto_toml(nombre, &momento.marca),
    )?;
    archivo(
        &raiz.join("README.md"),
        &plantillas::readme(nombre, &momento.fecha),
    )?;
    archivo(&raiz.join("sensibilidad.csv"), plantillas::SENSIBILIDAD_CSV)?;
    carpeta(&raiz.join("bitacora"))?;
    archivo(&raiz.join("bitacora/bitacora.md"), plantillas::BITACORA_MD)?;
    archivo(
        &raiz.join("bitacora/registro.jsonl"),
        &plantillas::evento_creado(&momento.marca),
    )?;
    carpeta(&raiz.join("participantes"))?;
    archivo(
        &raiz.join("participantes/participantes.csv"),
        plantillas::PARTICIPANTES_CSV,
    )?;
    archivo(
        &raiz.join("participantes/consentimientos.csv"),
        &plantillas::consentimientos_csv(),
    )?;
    carpeta(&raiz.join("identificable"))?;
    archivo(
        &raiz.join("identificable/correspondencia.csv"),
        plantillas::CORRESPONDENCIA_CSV,
    )?;
    carpeta(&raiz.join("identificable/consentimientos"))?;
    carpeta(&raiz.join("identificable/entrevistas"))?;
    carpeta(&raiz.join(".osr"))?;
    crate::indice::construir(raiz)?;
    archivo(
        &raiz.join(".osr/bloqueo"),
        &plantillas::bloqueo(std::process::id(), &momento.marca),
    )
}

/// Crea el proyecto en `ruta`, lo deja abierto con su bloqueo y devuelve su
/// nombre.
///
/// Si algo falla no queda nada nuevo en la ruta ni a su lado.
pub fn crear(ruta: &Path, nombre: Option<&str>, momento: &Momento) -> Result<String, Error> {
    let nombre = nombre_del_proyecto(ruta, nombre)?;
    comprobar_destino(ruta)?;
    let preparacion = carpeta_padre(ruta).join(format!(".osr-nuevo-{}", std::process::id()));
    let resultado =
        armar(&preparacion, &nombre, momento).and_then(|()| fs::rename(&preparacion, ruta));
    if let Err(error) = resultado {
        let _ = fs::remove_dir_all(&preparacion);
        return Err(Error::Escritura(ruta.to_path_buf(), error));
    }
    Ok(nombre)
}

#[cfg(test)]
mod pruebas {
    use super::*;

    fn momento() -> Momento {
        Momento {
            marca: "2026-10-11T21:30:05-06:00".into(),
            fecha: "2026-10-11".into(),
        }
    }

    fn modo(ruta: &Path) -> u32 {
        fs::metadata(ruta).unwrap().permissions().mode() & 0o777
    }

    #[test]
    fn ahora_escribe_marca_y_fecha_iso() {
        let momento = Momento::ahora();
        let marca = momento.marca.as_bytes();
        assert_eq!(marca.len(), 25, "{}", momento.marca);
        assert_eq!(&momento.marca[..10], momento.fecha);
        assert_eq!(marca[10], b'T');
        assert!(matches!(marca[19], b'+' | b'-'));
        assert_eq!(marca[22], b':');
        jiff::Timestamp::strptime("%Y-%m-%dT%H:%M:%S%:z", &momento.marca).unwrap();
    }

    #[test]
    fn la_marca_compacta_no_tiene_separadores_ni_desplazamiento() {
        assert_eq!(momento().compacta(), "20261011T213005");
    }

    #[test]
    fn nombre_entre_1_y_200_caracteres_sin_control() {
        assert!(nombre_valido("a"));
        assert!(nombre_valido(&"ñ".repeat(200)));
        assert!(!nombre_valido(""));
        assert!(!nombre_valido(&"a".repeat(201)));
        assert!(!nombre_valido("Uno\nDos"));
        assert!(!nombre_valido("tab\t"));
    }

    #[test]
    fn el_nombre_por_defecto_es_el_ultimo_componente() {
        let nombre = nombre_del_proyecto(Path::new("/tmp/Investigación año uno"), None);
        assert_eq!(nombre.unwrap(), "Investigación año uno");
        assert_eq!(
            nombre_del_proyecto(Path::new("x"), Some("Otro")).unwrap(),
            "Otro"
        );
        assert!(matches!(
            nombre_del_proyecto(Path::new("/"), None),
            Err(Error::NombreInvalido)
        ));
    }

    #[test]
    fn la_carpeta_padre_de_un_nombre_suelto_es_la_actual() {
        assert_eq!(carpeta_padre(Path::new("estudio")), Path::new("."));
        assert_eq!(carpeta_padre(Path::new("/a/b")), Path::new("/a"));
    }

    #[test]
    fn codigos_y_mensajes_de_la_tabla() {
        let ruta = PathBuf::from("/x/estudio");
        let casos = [
            (Error::NombreInvalido, 2),
            (Error::Ocupada(ruta.clone()), 3),
            (Error::SinPadre(PathBuf::from("/x")), 3),
            (Error::Escritura(ruta, io::ErrorKind::StorageFull.into()), 6),
        ];
        for (error, codigo) in casos {
            assert_eq!(error.codigo(), codigo, "{error}");
        }
    }

    #[test]
    fn el_motivo_esta_en_espanol() {
        let motivo_de = |tipo: io::ErrorKind| motivo(&tipo.into());
        assert_eq!(
            motivo_de(io::ErrorKind::PermissionDenied),
            "sin permiso de escritura"
        );
        assert_eq!(motivo_de(io::ErrorKind::StorageFull), "el disco está lleno");
        assert_eq!(
            motivo_de(io::ErrorKind::ReadOnlyFilesystem),
            "el disco es de solo lectura"
        );
        assert_eq!(
            motivo_de(io::ErrorKind::Other),
            "error de escritura del sistema"
        );
    }

    #[test]
    fn crea_con_permisos_0700_y_0600() {
        let tmp = tempfile::tempdir().unwrap();
        let raiz = tmp.path().join("estudio");
        assert_eq!(crear(&raiz, None, &momento()).unwrap(), "estudio");
        for carpeta in [
            "",
            "bitacora",
            "participantes",
            "identificable/entrevistas",
            ".osr",
        ] {
            assert_eq!(modo(&raiz.join(carpeta)), 0o700, "{carpeta}");
        }
        for archivo in [
            "proyecto.toml",
            "README.md",
            "bitacora/registro.jsonl",
            ".osr/indice.sqlite",
        ] {
            assert_eq!(modo(&raiz.join(archivo)), 0o600, "{archivo}");
        }
    }

    #[test]
    fn el_indice_nuevo_se_puede_usar_sin_reconstruir() {
        let tmp = tempfile::tempdir().unwrap();
        let raiz = tmp.path().join("estudio");
        crear(&raiz, None, &momento()).unwrap();
        assert_eq!(crate::indice::revisar(&raiz), None);
    }

    #[test]
    fn el_bloqueo_tiene_el_pid_y_el_inicio() {
        let tmp = tempfile::tempdir().unwrap();
        let raiz = tmp.path().join("estudio");
        crear(&raiz, None, &momento()).unwrap();
        let bloqueo = fs::read_to_string(raiz.join(".osr/bloqueo")).unwrap();
        assert_eq!(
            bloqueo,
            format!(
                "pid = {}\ninicio = 2026-10-11T21:30:05-06:00\n",
                std::process::id()
            )
        );
    }

    #[test]
    fn una_ruta_que_es_archivo_esta_ocupada() {
        let tmp = tempfile::tempdir().unwrap();
        let ruta = tmp.path().join("archivo");
        fs::write(&ruta, "x").unwrap();
        assert!(matches!(
            crear(&ruta, None, &momento()),
            Err(Error::Ocupada(_))
        ));
    }
}
