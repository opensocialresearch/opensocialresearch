# Especificación del módulo proyecto (PRY)

Tramo 1 de la v0.1, borrador para aprobación del autor.

## 1. Propósito

Este módulo define qué es un proyecto de Open Social Research en el disco, una
carpeta con archivos de texto abierto que son la única fuente de verdad, un
índice interno que se puede borrar y reconstruir, y un README que explica todo
sin la app. También fija cómo la app crea, abre, verifica y escribe esa carpeta
para que ningún cierre forzado pierda o corrompa datos.

### Requisitos de PRODUCTO.md que cubre

| Sección | Requisito (texto corto) | IDs |
|---|---|---|
| 6.0 | «Un proyecto es una carpeta con un archivo `proyecto.toml` que declara la versión del formato, y el binario `osr` la abre» | PRY-01, PRY-02, PRY-03, PRY-04 (el servidor y el navegador en SRV-01) |
| 6.0 | «Todo lo que no es audio o imagen queda en texto abierto […] con un README generado» | PRY-06, PRY-16 |
| 6.0 | «Los archivos de texto son la fuente de verdad, ningún dato ni metadato vive solo en SQLite, y su formato está documentado» | PRY-06, PRY-10 |
| 6.0 | «El índice SQLite vive en `.osr/` y es reconstruible, con una prueba automática» | PRY-10 |
| 6.0 | «Guardado automático con escrituras atómicas […] recuperación tras cierre forzado y verificación al abrir» | PRY-11, PRY-12, PRY-13, PRY-14 (deshacer y rehacer en TRA-21) |
| 6.0 | «Datos de prueba inventados y mínimos en el repositorio […] nunca datos reales» | PRY-17 |
| 6.1 | «Bitácora en Markdown con entradas fechadas de tipo explícito» (v0.1 sin tramo asignado) | PRY-08 |
| 6.5 | «guardar el original inmutable, sin recomprimir y con su hash» y «Ficha corta para el material propio de campo» | PRY-07, PRY-15 |
| 6.5 | «Datos crudos inmutables, los archivos de `datos/crudos/` quedan de solo lectura» (solo la parte de originales propios, el resto es tramo 3) | PRY-15 |
| 7, Durabilidad | «Persistencia de cada acción […] 1 s o menos, con pérdida máxima de 2 s» y «Escrituras atómicas […] verificadas con pruebas que matan el proceso» | PRY-12, PRY-13 |
| 7, Idioma y formatos | «Interfaz y mensajes 100 % en español» y «UTF-8 sin BOM, CSV según RFC 4180, fechas ISO 8601» | PRY-06, PRY-18 |
| 5, terminación | «Cero pérdidas de datos […] incluidos cierres forzados», «El README generado describe cada carpeta y cada tipo de archivo», «la de reconstrucción del índice» | PRY-10, PRY-11, PRY-12, PRY-16 |
| 9, amenazas | «Laptop robada o perdida» y «Disco de respaldo perdido», cubiertas en la v0.1 por el README | PRY-16 |
| 6.0 | regla general de bloqueo, «bloquea por defecto y permite una excepción que la persona confirma y que queda registrada» | PRY-09 |
| 6.0 | «el binario `osr` la abre» y la interfaz sin proyecto que ofrece crear y abrir (SRV-01) | PRY-19, PRY-20, PRY-21, PRY-22, PRY-23 |

## 2. Definiciones

| Término | Definición |
|---|---|
| Proyecto | Una carpeta que contiene un `proyecto.toml` válido según PRY-03. |
| Raíz | La carpeta del proyecto. Todas las rutas de esta especificación son relativas a ella y usan `/`. |
| Archivo gestionado | Archivo que la app crea o mantiene y cuyo formato define esta especificación o las de SRV, PAR y TRA. La lista completa está en PRY-02. Los demás archivos de la carpeta son de la persona y la app no los modifica. |
| Archivo de texto gestionado | Archivo gestionado que no es audio, imagen ni PDF. |
| Original | Copia byte a byte de un archivo traído de fuera (audio, escaneo, salida de la cadena de transcripción). Nunca cambia después de importarse. |
| Ficha | Archivo TOML que acompaña a un original con su huella y sus datos de procedencia (PRY-07). |
| Índice | La base SQLite `.osr/indice.sqlite`. Es una copia derivada de los archivos de texto y se puede borrar sin perder nada. |
| Escritura atómica | Escritura en la que, ante cualquier interrupción, el archivo en disco queda con su contenido anterior completo o con el nuevo completo, nunca con una mezcla ni truncado. |
| Cierre forzado | Fin del proceso `osr` sin aviso, por SIGKILL, corte de luz o caída del sistema. |
| Marca de tiempo | Fecha y hora en ISO 8601 con desplazamiento local y precisión de segundos, por ejemplo `2026-10-11T21:30:05-06:00`. |
| Fecha | Fecha en ISO 8601 sin hora, `AAAA-MM-DD`. |
| Registro de eventos | El archivo `bitacora/registro.jsonl`, donde la app anota lo que hace y las excepciones confirmadas (PRY-08). |
| Excepción confirmada | Acción que la app bloquea por defecto y que la persona autoriza de forma explícita, según PRY-09. |
| Huella | SHA-256 del contenido de un archivo, escrita en 64 caracteres hexadecimales en minúscula. |

## 3. Requisitos

### PRY-01 Crear un proyecto

**Comportamiento.** `osr nuevo <ruta>` crea un proyecto vacío en `<ruta>` con la
estructura de PRY-02, escribe `proyecto.toml`, el README de PRY-16, la
bitácora de PRY-08 y los archivos CSV vacíos de PAR, anota el evento
`proyecto_creado` y abre el proyecto como en PRY-04. La interfaz sin proyecto
abierto (SRV-01) ofrece la misma acción con un campo para la ruta y otro para
el nombre.

**Entradas.**
- `<ruta>`, absoluta o relativa al directorio actual. Puede contener espacios,
  tildes, ñ y cualquier carácter que el sistema de archivos admita.
- `--nombre <texto>`, opcional. Si falta, el nombre es el último componente de
  la ruta.

**Salidas.** La carpeta creada, con permisos según SRV-13. En la salida
estándar, la línea de SRV-01. Código de salida 0 al cerrar normalmente.

**Errores.** Nada se crea si hay un error. Los mensajes van a la salida de
error con el prefijo `osr: `.

| Condición | Código de salida | Mensaje |
|---|---|---|
| `<ruta>` existe y no es una carpeta vacía | 3 | `osr: la carpeta <ruta> ya existe y no está vacía. Hay que elegir una carpeta nueva o vacía.` |
| La carpeta que contendría a `<ruta>` no existe | 3 | `osr: no existe la carpeta <padre>.` |
| Sin permiso de escritura o disco lleno | 6 | `osr: no se pudo crear el proyecto en <ruta> (<motivo en español>).` |
| `--nombre` vacío, con más de 200 caracteres o con caracteres de control | 2 | `osr: el nombre del proyecto tiene que tener entre 1 y 200 caracteres, sin saltos de línea.` |

**Límites.** Nombre de 1 a 200 caracteres Unicode, sin caracteres de control.

**Criterios de aceptación.**
- PRY-01.1 Dado una ruta inexistente cuyo padre existe, `osr nuevo` crea la
  carpeta con exactamente los archivos y carpetas de la columna «Al crear» de
  PRY-02.
- PRY-01.2 Dado una carpeta existente y vacía, `osr nuevo` la usa sin error.
- PRY-01.3 Dado una carpeta con al menos un archivo, `osr nuevo` termina con
  código 3, el mensaje de la tabla, y la carpeta queda igual byte a byte.
- PRY-01.4 Con la ruta `/tmp/prueba/Investigación año uno` el proyecto se crea
  y se abre sin error, y `proyecto.toml` guarda el nombre `Investigación año
  uno`.
- PRY-01.5 Si la escritura falla a mitad de la creación, no queda ninguna
  carpeta ni archivo nuevo en `<ruta>`.
- PRY-01.6 El registro de eventos del proyecto nuevo tiene exactamente una
  línea, de tipo `proyecto_creado`.

### PRY-02 Estructura de la carpeta

**Comportamiento.** La carpeta tiene la estructura de la tabla. La zona
identificable es todo lo que está bajo `identificable/`, la zona de trabajo es
todo lo demás salvo `.osr/` (PAR-12). Las carpetas marcadas «tramo 2» o
«tramo 3» no se crean en el tramo 1, pero sus nombres quedan reservados y la
app no los usa para otra cosa.

| Ruta | Qué es | Al crear | Especificación |
|---|---|---|---|
| `proyecto.toml` | Identidad y configuración del proyecto | sí | PRY-03 |
| `README.md` | Explicación generada de la carpeta | sí | PRY-16 |
| `sensibilidad.csv` | Marcas de sensibilidad asignadas a mano | sí, solo encabezado | PAR-14 |
| `bitacora/bitacora.md` | Bitácora que escribe la persona | sí | PRY-08 |
| `bitacora/registro.jsonl` | Registro de eventos que escribe la app | sí | PRY-08 |
| `participantes/participantes.csv` | Registro de participantes por seudónimo | sí, solo encabezado | PAR-02 |
| `participantes/consentimientos.csv` | Registro de consentimientos | sí, solo encabezado | PAR-07 |
| `identificable/correspondencia.csv` | Tabla seudónimo y persona | sí, solo encabezado | PAR-03 |
| `identificable/consentimientos/` | Respaldos de consentimientos con sus fichas | sí, vacía | PAR-08 |
| `identificable/entrevistas/<id>/` | Una carpeta por entrevista | sí, `entrevistas/` vacía | TRA-01 |
| `identificable/entrevistas/<id>/entrevista.toml` | Datos de la entrevista | no | TRA-01 |
| `identificable/entrevistas/<id>/<id>.<ext>` y `<id>.<ext>.ficha.toml` | Audio original y su ficha | no | TRA-02 |
| `identificable/entrevistas/<id>/transcripcion.json` | Transcripción canónica en revisión | no | TRA-11 |
| `identificable/entrevistas/<id>/origen/<marca>/` | Material de origen de cada importación, de solo lectura | no | TRA-10 |
| `entrevistas/` | Versiones de trabajo seudonimizadas (tramo 2) | no | tramo 2 |
| `datos/crudos/`, `datos/derivados/` | Datos externos y derivados (tramo 3) | no | tramo 3 |
| `.osr/indice.sqlite` | Índice reconstruible | sí | PRY-10 |
| `.osr/bloqueo` | Bloqueo de proceso único | mientras esté abierto | PRY-05 |

Además pueden aparecer archivos de resguardo con estos patrones, que también
son gestionados.

| Patrón | Qué es | Especificación |
|---|---|---|
| `<nombre>.conflicto-<marca>.<ext>` | Versión en disco guardada aparte ante un conflicto | PRY-14 |
| `transcripcion.anterior-<marca>.json` | Transcripción reemplazada por una reimportación | TRA-12 |
| `README.modificado-<marca>.md` | README editado a mano y reemplazado | PRY-16 |

`<marca>` es la marca de tiempo compacta `AAAAMMDDTHHMMSS` en hora local.

**Entradas.** No aplica.

**Salidas.** No aplica.

**Errores.** Si al abrir falta una carpeta de la columna «Al crear», la app la
vuelve a crear vacía y anota el evento `carpeta_recreada` con su ruta.

**Límites.** Ninguna ruta gestionada contiene nombres de personas, porque todos
los nombres de archivo gestionados se derivan del seudónimo o de la marca de
tiempo (PAR-01).

**Criterios de aceptación.**
- PRY-02.1 Un proyecto recién creado tiene exactamente las rutas marcadas «sí»
  en la columna «Al crear», salvo `.osr/bloqueo`, que existe solo mientras el
  proyecto está abierto.
- PRY-02.2 Si se borra `identificable/consentimientos/` con la app cerrada, al
  abrir la carpeta existe de nuevo vacía y el registro de eventos tiene una
  línea `carpeta_recreada` con esa ruta.
- PRY-02.3 Después de recorrer todos los escenarios del tramo 1 con los datos
  de prueba, ninguna ruta gestionada contiene ninguno de los nombres de la
  tabla de correspondencia de prueba.

### PRY-03 Archivo `proyecto.toml`

**Comportamiento.** `proyecto.toml` identifica el proyecto y guarda su
configuración. La app lo lee al abrir y en el tramo 1 lo reescribe solo al agregar un
alcance o cambiar su etiqueta (PAR-06), conservando las claves y los
comentarios que no conoce.

**Formato.** TOML 1.0, UTF-8 sin BOM. Claves del tramo 1.

| Clave | Tipo | Obligatoria | Valor por defecto al crear | Uso |
|---|---|---|---|---|
| `formato` | texto | sí | `"osr-proyecto"` | Identifica el archivo |
| `version_formato` | entero | sí | `1` | Versión del formato de la carpeta |
| `nombre` | texto | sí | según PRY-01 | Nombre visible |
| `creado` | marca de tiempo | sí | momento de creación | Procedencia |
| `participantes.prefijo` | texto | no | `"E"` | PAR-01 |
| `participantes.digitos` | entero de 2 a 4 | no | `2` | PAR-01 |
| `consentimiento.alcances` | lista de tablas `{id, etiqueta}` | no | los de PAR-06 | PAR-06 |
| `transcripcion.etiqueta_entrevistadora` | texto | no | `"Persona entrevistadora"` | TRA-15 |
| `transcripcion.retroceso_s` | entero de 3 a 5 | no | `3` | TRA-17 |
| `transcripcion.sospecha.no_speech_prob` | decimal | no | `0.6` | TRA-14 |
| `transcripcion.sospecha.avg_logprob` | decimal | no | `-1.0` | TRA-14 |
| `transcripcion.sospecha.compression_ratio` | decimal | no | `2.4` | TRA-14 |

Una clave opcional ausente toma su valor por defecto. Una clave opcional con un
valor fuera de rango o de otro tipo toma su valor por defecto, y la interfaz
muestra el aviso `El valor de <clave> en proyecto.toml no es válido; se usa
<valor por defecto>.`

**Errores al abrir.** El proyecto no se abre.

| Condición | Código de salida | Mensaje |
|---|---|---|
| No existe `proyecto.toml` | 3 | `osr: <ruta> no es un proyecto de Open Social Research (falta proyecto.toml).` |
| No es TOML válido | 3 | `osr: proyecto.toml tiene un error en la línea <n>: <motivo>.` |
| `formato` distinto de `"osr-proyecto"` o falta una clave obligatoria | 3 | `osr: proyecto.toml no es de Open Social Research o le falta <clave>.` |
| `version_formato` distinto de 1 | 3 | `osr: este proyecto usa la versión <n> del formato y esta versión de osr solo abre la versión 1.` |

**Límites.** Hasta 64 KiB.

**Criterios de aceptación.**
- PRY-03.1 Un proyecto recién creado tiene un `proyecto.toml` con las cuatro
  claves obligatorias y los valores por defecto de la tabla.
- PRY-03.2 Con `version_formato = 2`, `osr abrir` termina con código 3 y el
  mensaje de la tabla, y no escribe nada en la carpeta.
- PRY-03.3 Con `transcripcion.retroceso_s = 9`, el proyecto abre, el retroceso
  es de 3 s y la interfaz muestra el aviso de valor no válido.
- PRY-03.4 Si `proyecto.toml` tiene un comentario y una clave `extra.color =
  "azul"`, después de agregar un alcance desde la interfaz el comentario y la
  clave siguen en el archivo sin cambios.

### PRY-04 Abrir un proyecto

**Comportamiento.** `osr abrir <ruta>`, o `osr <ruta>`, abre el proyecto en
este orden. Valida `proyecto.toml` (PRY-03), toma el bloqueo (PRY-05), corre la
verificación al abrir (PRY-11), regenera el README (PRY-16), levanta el
servidor y abre el navegador (SRV-01). La interfaz sin proyecto (SRV-01) ofrece
la misma acción con un campo para la ruta. Un proceso abre un solo proyecto y
lo mantiene abierto hasta que termina.

**Entradas.** `<ruta>` del proyecto, con las mismas reglas de caracteres que
PRY-01.

**Salidas.** La vista «Resumen del proyecto» en el navegador, que muestra el
nombre, la lista de participantes con el estado de su consentimiento (PAR-10),
la lista de entrevistas con el estado de su audio y de su transcripción
(TRA-03, TRA-13) y el resultado de la verificación al abrir.

**Errores.** Los de PRY-03 y PRY-05. Si `<ruta>` no existe, código 3 y
`osr: no existe la carpeta <ruta>.`

**Límites.** El tiempo hasta la interfaz usable se rige por SRV-16.

**Criterios de aceptación.**
- PRY-04.1 `osr abrir <ruta>` y `osr <ruta>` producen el mismo resultado.
- PRY-04.2 Abrir el proyecto de prueba muestra en el resumen las dos
  entrevistas de prueba con sus estados.
- PRY-04.3 Abrir una ruta que no existe termina con código 3 y el mensaje
  indicado, sin crear nada.

### PRY-05 Un solo proceso por proyecto

**Comportamiento.** Mientras un proceso `osr` tiene abierto un proyecto, existe
`.osr/bloqueo` con su número de proceso y su marca de tiempo de inicio. Otro
proceso que intenta abrir el mismo proyecto no lo abre. Si el bloqueo es de un
proceso que ya no existe, el nuevo proceso lo reemplaza y anota el evento
`bloqueo_abandonado`. Al cerrar de forma ordenada (SRV-15) el bloqueo se borra.

**Errores.**

| Condición | Código de salida | Mensaje |
|---|---|---|
| Bloqueo de un proceso `osr` vivo | 4 | `osr: el proyecto ya está abierto en otro proceso (PID <n>). Hay que usar esa pestaña o cerrar ese proceso.` |

**Límites.** Solo proyectos en discos locales. Las carpetas en red quedan fuera
del tramo 1.

**Criterios de aceptación.**
- PRY-05.1 Con un proyecto abierto en un proceso, un segundo `osr abrir` sobre
  la misma ruta termina con código 4 y el mensaje con el PID del primero, y el
  primero sigue funcionando.
- PRY-05.2 Después de matar con SIGKILL el proceso que tenía el proyecto
  abierto, un nuevo `osr abrir` lo abre y el registro de eventos tiene una
  línea `bloqueo_abandonado`.
- PRY-05.3 Después de un cierre ordenado no existe `.osr/bloqueo`.

### PRY-06 Formatos de texto y fuente de verdad

**Comportamiento.** Todo dato y todo metadato del proyecto vive en un archivo
de texto gestionado, en un original o en una ficha. El índice solo copia lo que
ya está en esos archivos. Cada archivo de texto gestionado cumple estas reglas.

- Codificación UTF-8 sin BOM, con el texto en forma de normalización NFC.
- Fin de línea LF y salto de línea al final del archivo, salvo los CSV.
- CSV según RFC 4180, con fila de encabezado, separador coma, comillas dobles
  solo cuando el campo contiene coma, comillas o salto de línea, y fin de línea
  CRLF como pide la norma. Al leer, la app acepta CRLF y LF.
- JSON con sangría de dos espacios, claves en el orden que documenta cada
  especificación, caracteres no ASCII escritos tal cual sin secuencias de
  escape, y números decimales con punto.
- JSON Lines con un objeto JSON completo por línea, sin sangría.
- TOML 1.0.
- Fechas y marcas de tiempo según las definiciones de la sección 2.

Cada formato gestionado está documentado en estas especificaciones, con sus
campos, tipos y valores permitidos, para que se pueda leer sin la app y
migrar más adelante a Automerge.

**Salidas.** Para el mismo contenido lógico, la app escribe siempre los mismos
bytes.

**Errores.** Un archivo de texto gestionado que no cumple su formato se trata
según PRY-11.

**Criterios de aceptación.**
- PRY-06.1 Ningún archivo de texto gestionado de un proyecto de prueba empieza
  con los bytes EF BB BF.
- PRY-06.2 Si se escribe «año» en la interfaz con la ñ descompuesta (n más
  tilde combinante), el archivo guarda la forma NFC.
- PRY-06.3 Guardar dos veces el mismo participante sin cambios deja
  `participantes/participantes.csv` igual byte a byte.
- PRY-06.4 Cada CSV gestionado del proyecto de prueba se lee en R con
  `read.csv(ruta, fileEncoding = "UTF-8")` y devuelve las mismas filas,
  columnas y valores que muestra la interfaz, con tildes y ñ intactas.
- PRY-06.5 Después de borrar `.osr/`, todo lo que muestra la interfaz se puede
  reconstruir (PRY-10.1), lo que prueba que ningún dato vivía solo en el
  índice.

### PRY-07 Fichas de material propio

**Comportamiento.** Cada original propio que entra al proyecto, sea audio de
entrevista (TRA-02) o respaldo de consentimiento (PAR-08), lleva al lado una
ficha `<archivo>.ficha.toml` en la misma carpeta. La ficha se escribe al
importar y solo cambia si el original se elimina (TRA-23).

**Formato.**

| Clave | Tipo | Obligatoria | Contenido |
|---|---|---|---|
| `formato` | texto | sí | `"osr-ficha"` |
| `version_formato` | entero | sí | `1` |
| `tipo` | texto | sí | `"audio_entrevista"` o `"respaldo_consentimiento"` |
| `archivo` | texto | sí | Nombre del original en la carpeta |
| `sha256` | texto | sí | Huella del original |
| `bytes` | entero | sí | Tamaño del original |
| `formato_archivo` | texto | sí | `m4a`, `mp3`, `wav`, `ogg`, `opus`, `pdf`, `jpeg`, `png` o `heic` |
| `importado` | marca de tiempo | sí | Momento de la importación |
| `fecha_material` | fecha | no | Fecha de la grabación o de la firma, si la persona la da |
| `seudonimo` | texto | sí | Seudónimo de la persona (PAR-01) |
| `consentimiento` | texto | sí | `"participantes/consentimientos.csv"`, el registro donde está la fila de ese seudónimo |
| `nombre_original` | texto | sí | Nombre que tenía el archivo antes de importarse |
| `duracion_s` | decimal | no | Duración del audio en segundos con un decimal, si se pudo leer (TRA-03) |
| `estado` | texto | sí | `"presente"` o `"eliminado"` |
| `eliminado` | marca de tiempo | solo si `estado = "eliminado"` | Momento de la eliminación |
| `motivo_eliminacion` | texto | solo si `estado = "eliminado"` | `"retencion"` |

Como la ficha vive en la zona identificable, puede guardar el nombre original
del archivo, que a veces contiene un nombre de persona. Ese nombre no se copia
a ningún archivo fuera de `identificable/`, ni al índice ni al registro técnico.

**Criterios de aceptación.**
- PRY-07.1 Después de importar un audio, su ficha existe, tiene todas las
  claves obligatorias y su `sha256` coincide con la huella del original.
- PRY-07.2 Si el archivo importado se llamaba `entrevista con Nombre Inventado.ogg`,
  con un nombre del segundo conjunto de PRY-17, la cadena `Nombre Inventado`
  aparece en la ficha y en ningún archivo fuera
  de `identificable/`, ni en `.osr/`, ni en el registro técnico.

### PRY-08 Bitácora y registro de eventos [D-03]

**Comportamiento.** La carpeta `bitacora/` tiene dos archivos con dueños
distintos.

- `bitacora/bitacora.md` es de la persona. La app lo crea al crear el proyecto
  con un título y un párrafo que explica la convención de entradas fechadas de
  tipo explícito (decisión metodológica, reflexión, analítica, de campo), y
  después no lo vuelve a escribir. La persona lo edita con su propio editor.
- `bitacora/registro.jsonl` es de la app. Solo se agregan líneas al final,
  nunca se cambia ni se borra una línea escrita.

**Formato de cada línea del registro.**

| Clave | Tipo | Contenido |
|---|---|---|
| `fecha` | marca de tiempo | Momento del evento |
| `tipo` | texto | Uno de los tipos de la tabla siguiente |
| `detalle` | objeto | Campos propios de cada tipo |

Tipos del tramo 1, con el módulo que los produce.

| Tipo | Módulo |
|---|---|
| `proyecto_creado`, `carpeta_recreada`, `bloqueo_abandonado`, `temporales_eliminados`, `linea_incompleta_descartada`, `verificacion`, `indice_reconstruido`, `conflicto_resuelto`, `readme_modificado_guardado`, `configuracion_cambiada` | PRY |
| `permisos_corregidos` | SRV |
| `participante_creado`, `participante_eliminado`, `correspondencia_modificada`, `consentimiento_registrado`, `consentimiento_modificado`, `alcance_agregado`, `respaldo_consentimiento_agregado`, `sensibilidad_cambiada` | PAR |
| `entrevista_creada`, `audio_importado`, `audio_eliminado`, `transcripcion_importada`, `transcripcion_reemplazada`, `transcripcion_revisada`, `transcripcion_reabierta` | TRA |
| `excepcion_confirmada` | PRY-09 |

El registro nunca contiene nombres de la tabla de correspondencia, texto de
transcripciones o de encabezados, nombres originales de archivos, ni nombres
de archivos de `identificable/` que la app no haya generado. Para nombrar
entrevistas y personas usa el seudónimo, y para los archivos no registrados
solo da su cantidad.

**Escritura interrumpida.** Si al abrir la última línea no termina en salto de
línea, se descarta esa línea incompleta, que es el único caso en que la app
recorta el archivo, y se anota `linea_incompleta_descartada`.

**Criterios de aceptación.**
- PRY-08.1 Después de crear un participante, la última línea del registro es
  JSON válido con `tipo = "participante_creado"` y el seudónimo en `detalle`.
- PRY-08.2 Después de cualquier secuencia de acciones, las primeras N líneas
  del registro son idénticas byte a byte a las N líneas que tenía antes.
- PRY-08.3 Si se agrega a mano al final del registro el texto `{"fecha":` sin
  salto de línea, al abrir esa línea ya no está y la última línea es de tipo
  `linea_incompleta_descartada`.
- PRY-08.4 La app no modifica `bitacora/bitacora.md` después de crearlo, lo
  que se comprueba con su huella antes y después de recorrer todos los
  escenarios del tramo 1.
- PRY-08.5 Después de recorrer todos los escenarios con los datos de prueba,
  el registro no contiene ninguno de los nombres de la tabla de correspondencia
  ni ninguna frase de las transcripciones de prueba.

### PRY-09 Excepciones confirmadas

**Comportamiento.** Cuando una acción está bloqueada por defecto y su
especificación permite una excepción, la interfaz muestra el motivo del
bloqueo en una o dos oraciones y dos botones, `Cancelar`, que tiene el foco
inicial, y `Hacerlo igual`. Elegir `Hacerlo igual` ejecuta la acción y anota
`excepcion_confirmada` con el código de la excepción, el objeto afectado
(seudónimo, entrevista o ruta generada por la app) y los datos que pida cada
especificación. Una acción sin excepción permitida muestra solo el motivo y
`Entendido`.

**Códigos de excepción del tramo 1.**

| Código | Especificación |
|---|---|
| `audio_sin_consentimiento` | PAR-11 |
| `transcripcion_mas_larga_que_audio` | TRA-05 |
| `reemplazar_transcripcion_con_cambios` | TRA-12 |
| `revisada_con_segmentos_sin_revisar` | TRA-13 |
| `eliminar_audio_sin_revisar` | TRA-23 |

**Criterios de aceptación.**
- PRY-09.1 Ante cualquier bloqueo con excepción, pulsar Enter sin mover el
  foco cancela la acción y no escribe nada.
- PRY-09.2 Elegir `Hacerlo igual` agrega exactamente una línea
  `excepcion_confirmada` con el código correcto, además del evento propio de
  la acción.

### PRY-10 Índice reconstruible

**Comportamiento.** El índice `.osr/indice.sqlite` guarda, para acelerar la
interfaz, la lista de archivos gestionados con su tamaño, fecha de
modificación y huella, y copias de los datos de participantes, consentimientos,
entrevistas, fichas y estados de transcripción. No guarda nombres de la tabla
de correspondencia, nombres originales de archivos, texto de transcripciones,
de encabezados ni de notas, ni ningún otro contenido de `identificable/` salvo
rutas generadas, huellas, tamaños, duraciones, estados y conteos.

La app reconstruye el índice desde los archivos cuando falta, cuando no se
puede abrir como base SQLite válida o cuando su versión de esquema no es la que
espera, y anota `indice_reconstruido` con el motivo (`ausente`, `danado` o
`version`).

**Definición de mismo estado.** Dos aperturas muestran el mismo estado cuando
la vista «Resumen del proyecto» (PRY-04) y las vistas de cada participante y
de cada entrevista muestran los mismos datos, y las rutas de la interfaz que
las alimentan (SRV-10) devuelven el mismo contenido salvo las marcas de tiempo
de la propia apertura.

**Límites.** El tiempo de reconstrucción no tiene meta en la v0.1, su meta de
30 s para 50 entrevistas es de la v1.0.

**Criterios de aceptación.**
- PRY-10.1 Con el proyecto de prueba después de recorrer todos los escenarios,
  borrar `.osr/` con la app cerrada y volver a abrir produce el mismo estado,
  y el registro tiene una línea `indice_reconstruido` con motivo `ausente`.
- PRY-10.2 Reemplazar `.osr/indice.sqlite` por 1 KiB de bytes aleatorios
  produce el mismo estado al abrir y la línea con motivo `danado`.
- PRY-10.3 Después de recorrer todos los escenarios, ningún byte de
  `.osr/indice.sqlite` contiene, en UTF-8 y en NFC o NFD, ninguno de los nombres
  de la tabla de correspondencia de prueba ni ninguna frase de las
  transcripciones de prueba.

### PRY-11 Verificación al abrir

**Comportamiento.** Cada vez que abre un proyecto, la app hace estos pasos y
anota un único evento `verificacion` con el resultado.

1. Borra los archivos temporales que dejó una escritura interrumpida
   (PRY-12) y cuenta cuántos borró. Si borró alguno, anota
   `temporales_eliminados` con la cantidad.
2. Repara una última línea incompleta del registro (PRY-08).
3. Para cada archivo de texto gestionado compara su huella con la del índice.
   Si cambió y es válido según su formato, el archivo manda, el índice se
   actualiza y la ruta queda en la lista `modificados_fuera`. Si no es válido,
   la ruta queda en la lista `con_error` con la línea y el motivo, y la app no
   escribe ese archivo ni ofrece las acciones que dependen de él hasta que una
   nueva verificación lo encuentre válido.
4. Para cada archivo que el índice conoce y que ya no está en disco, la ruta
   queda en la lista `faltantes`.
5. Para cada original con ficha en estado `presente`, compara tamaño y fecha
   de modificación con el índice, y si alguno cambió calcula la huella. Si la
   huella no coincide con la de la ficha, la ruta queda en `originales_alterados`.
6. Cuenta los archivos bajo `identificable/` que no son gestionados y guarda
   la cantidad en `no_registrados`.
7. Revisa los permisos según SRV-13.

La interfaz muestra el resultado en el resumen. Sin diferencias muestra
`Verificación al abrir: sin diferencias.` Con diferencias muestra un aviso por
lista con sus rutas, salvo los no registrados, de los que muestra la cantidad
y, a pedido, las rutas. El botón `Volver a verificar` repite los pasos 3 a 7
sin cerrar el proyecto.

**Salidas.** El evento `verificacion` con `detalle` igual a `{"modificados_fuera":
[...], "con_error": [...], "faltantes": [...], "originales_alterados": [...],
"no_registrados": <n>, "temporales_eliminados": <n>}`. Las rutas son solo de
archivos gestionados.

**Criterios de aceptación.**
- PRY-11.1 Si con la app cerrada se agrega una fila válida a
  `identificable/correspondencia.csv` con un editor de texto, al abrir la
  fila aparece en la interfaz y la ruta figura en `modificados_fuera`.
- PRY-11.2 Si con la app cerrada se borra una comilla de
  `participantes/consentimientos.csv` de modo que deja de ser CSV válido, al
  abrir la interfaz muestra la ruta con su número de línea, el archivo queda
  igual byte a byte y la acción `Registrar consentimiento` aparece
  deshabilitada con el texto `El archivo de consentimientos tiene un error.`
- PRY-11.3 Si se borra el audio de una entrevista con la app cerrada, al abrir
  la ruta figura en `faltantes` y la entrevista muestra `Falta el audio`.
- PRY-11.4 Si se cambia un byte de un audio importado (después de darle
  permiso de escritura a mano), al abrir la ruta figura en
  `originales_alterados` y la entrevista muestra `El audio cambió desde su
  importación`.
- PRY-11.5 Si se copian dos archivos cualesquiera a `identificable/`, al abrir
  el aviso dice `Hay 2 archivos en la zona identificable que la herramienta no
  registró.` y el evento tiene `no_registrados = 2` sin sus nombres.
- PRY-11.6 Un proyecto sin cambios desde el último cierre produce el texto
  `Verificación al abrir: sin diferencias.` y un evento con listas vacías y
  contadores en cero.

### PRY-12 Escrituras atómicas y cierres forzados

**Comportamiento.** Toda escritura de un archivo gestionado es atómica. La app
escribe primero un archivo temporal en la misma carpeta con el nombre
`.<nombre>.osr-tmp-<8 caracteres aleatorios>`, lo lleva a disco, lo renombra
sobre el destino y lleva a disco la carpeta. Las líneas del registro de
eventos se agregan al final y se llevan a disco una por una. Un cambio se
considera guardado recién cuando terminó ese ciclo.

**Criterios de aceptación.**
- PRY-12.1 Prueba de cierre forzado. Durante una secuencia automática de
  ediciones de transcripción, altas de participantes, cambios de consentimiento
  e importaciones, se envía SIGKILL al proceso en 200 instantes elegidos al
  azar con distribución uniforme. Después de cada uno se reabre el proyecto y
  se comprueba que
  - el 100 % de los archivos de texto gestionados es válido según su formato,
  - cada archivo gestionado es idéntico a alguna versión completa que la app
    escribió, nunca una mezcla,
  - no queda ningún archivo `.osr-tmp-` después de la apertura, y
  - toda edición del editor hecha 2 s o más antes del SIGKILL, y todo
    formulario que mostró `Guardado` antes del SIGKILL, están presentes.
- PRY-12.2 La prueba anterior termina con cero fallos en las 200
  repeticiones.

### PRY-13 Guardado automático y estado del guardado

**Comportamiento.** Los cambios en el editor de transcripción se guardan solos
1 s o menos después de hechos. Los formularios de participantes,
correspondencia y consentimientos se guardan al pulsar `Guardar`, y la
confirmación aparece solo después de que el cambio quedó guardado según
PRY-12. Cada vista con cambios muestra uno de estos textos, nunca solo un
color o un ícono.

| Estado | Texto |
|---|---|
| Sin cambios pendientes | `Guardado` |
| Escribiendo | `Guardando…` |
| Falló la escritura | `No se pudo guardar: <motivo en español>. Se reintenta en 5 s.` |

Si una escritura falla por disco lleno, falta de permisos u otro error del
sistema, la app conserva los cambios en memoria, reintenta cada 5 s y no
descarta nada mientras el proceso siga vivo.

**Criterios de aceptación.**
- PRY-13.1 Después de escribir una letra en un segmento, el archivo en disco
  la contiene a más tardar 1 s después.
- PRY-13.2 Con el disco de prueba lleno, una edición muestra el texto de falla,
  y al liberar espacio el texto pasa a `Guardado` en 10 s o menos y el archivo
  contiene la edición.
- PRY-13.3 El mensaje `Guardado` no aparece antes de que el archivo en disco
  tenga el cambio.

### PRY-14 Cambios hechos fuera de la app

**Comportamiento.** Antes de reescribir un archivo de texto gestionado, la app
compara su huella en disco con la última que ella leyó o escribió. Si son
distintas no escribe, y muestra `El archivo <ruta> cambió fuera de la
herramienta mientras estaba abierta. No se guardó el cambio para no pisarlo.`
con dos opciones.

- `Recargar desde el disco` descarta el cambio pendiente de la app y muestra el
  contenido del disco.
- `Conservar mi cambio` guarda la versión del disco como
  `<nombre>.conflicto-<marca>.<ext>` en la misma carpeta y escribe el cambio
  de la app.

Las dos opciones anotan `conflicto_resuelto` con la ruta y la opción elegida.
Esta regla no se aplica a `README.md`, que sigue PRY-16, ni al registro de
eventos, al que solo se agregan líneas.

**Criterios de aceptación.**
- PRY-14.1 Con el proyecto abierto, si se modifica `participantes/participantes.csv`
  con otro programa y luego se guarda un participante en la interfaz, el
  archivo en disco conserva la modificación externa y aparece el mensaje.
- PRY-14.2 Elegir `Conservar mi cambio` deja dos archivos, el de conflicto
  idéntico a la versión externa y el original con el cambio de la app.
- PRY-14.3 Elegir `Recargar desde el disco` deja el archivo idéntico a la
  versión externa y la interfaz la muestra.

### PRY-15 Originales inmutables

**Comportamiento.** Un original se copia byte a byte, sin recomprimir ni
convertir, se verifica que la huella de la copia sea igual a la de los bytes
recibidos, y queda con permisos 0400. La app nunca lo modifica. La única
operación permitida sobre un original es eliminarlo según TRA-23.

**Errores.** Si la huella de la copia no coincide, la app borra la copia, no
registra nada y muestra `La copia de <nombre original> no coincide con el
archivo recibido. No se importó; hay que intentarlo de nuevo.`

**Criterios de aceptación.**
- PRY-15.1 Después de importar, la huella del original en el proyecto es igual
  a la del archivo de origen.
- PRY-15.2 Los permisos del original son exactamente 0400 sin importar la
  umask del proceso.
- PRY-15.3 Una falla simulada que altera un byte de la copia deja el proyecto
  sin el archivo, sin ficha y sin eventos nuevos, y muestra el mensaje.

### PRY-16 README generado

**Comportamiento.** La app escribe `README.md` al crear el proyecto, cada vez
que lo abre y después de cada operación que agrega o quita archivos
gestionados. El README empieza con la línea `<!-- Generado por Open Social
Research. Los cambios a mano se guardan aparte y este archivo se reemplaza. -->`
y contiene estas secciones, en este orden.

1. **Qué es esta carpeta.** Nombre del proyecto, versión del formato y fecha
   de generación, y la frase de que todo se puede leer sin la app.
2. **Carpetas.** Una fila por cada carpeta presente, con qué contiene y a qué
   zona pertenece.
3. **Tipos de archivo.** Una fila por cada patrón de archivo gestionado
   presente (por ejemplo `*.ficha.toml` o `transcripcion.json`), con su
   formato, qué contiene, con qué se abre sin la app y en qué especificación
   está documentado.
4. **Zonas y sensibilidad.** Qué es cada zona, la regla de que la zona
   identificable solo se copia a un disco cifrado, los niveles de sensibilidad
   de PAR-14 y una tabla con la ruta y el nivel de cada archivo gestionado y de
   cada archivo con marca en `sensibilidad.csv`.
5. **Cómo respaldar.** Al menos una orden completa de una herramienta del
   sistema que copia la carpeta entera conservando los permisos, por ejemplo
   `rsync -a`, otra que copia todo salvo `identificable/` para destinos sin
   cifrar, y la aclaración de que `.osr/` se puede omitir porque se
   reconstruye.
6. **Qué no hacer.** No guardar la carpeta en servicios de sincronización en
   la nube, no editar los archivos de `origen/` ni los originales, y saber que
   borrar un archivo en un disco SSD no garantiza que sea irrecuperable, por
   lo que el disco tiene que estar cifrado.

El README no contiene nombres de la tabla de correspondencia, texto de
transcripciones ni de encabezados, ni nombres originales de archivos.

**Edición a mano.** Si la huella de `README.md` en disco no es la que la app
escribió por última vez, antes de reemplazarlo la app lo guarda como
`README.modificado-<marca>.md` y anota `readme_modificado_guardado`.

**Criterios de aceptación.**
- PRY-16.1 En el proyecto de prueba después de recorrer todos los escenarios,
  cada carpeta presente y cada patrón de archivo gestionado presente tiene su
  fila en el README.
- PRY-16.2 Después de importar el primer audio, el README tiene la fila del
  patrón de audio y la del patrón de ficha, y la tabla de sensibilidad incluye
  la ruta del audio con nivel `identificable`.
- PRY-16.3 El README contiene una orden con `rsync -a` y otra que excluye
  `identificable/`.
- PRY-16.4 El README no contiene ninguno de los nombres de la tabla de
  correspondencia de prueba ni frases de las transcripciones de prueba.
- PRY-16.5 Si se edita el README a mano y se abre el proyecto, existe un
  `README.modificado-<marca>.md` idéntico a la versión editada y el README
  vuelve a ser el generado.

### PRY-17 Datos de prueba inventados

**Comportamiento.** El repositorio incluye un proyecto de prueba y archivos
sueltos que usan las pruebas automáticas. Todo es inventado y ningún archivo
viene de una persona participante real.

- Dos entrevistas cortas, `E01` y `E02`, de 120 s o menos cada una, con audio
  de voz sintética o leída por el autor desde un guion inventado, con licencia
  que permita incluirlo en un repositorio público. `E01` en Ogg Opus y `E02` en
  m4a.
- Para cada entrevista, los archivos de la cadena del autor con exactamente su
  formato (TRA-04), `*.whisper.json`, `*.hablantes.json`, `*.md` y
  `*.encabezado.md`, y además una versión WebVTT y una en el JSON de Whisper
  con `avg_logprob`, `no_speech_prob` y `compression_ratio`.
- Dos conjuntos disjuntos de nombres inventados. Los del primero están en la
  tabla de correspondencia y también aparecen dichos dentro del texto de las
  transcripciones y del encabezado, para que las pruebas de PAR-04 tengan qué
  buscar. Los del segundo se usan solo en los nombres originales de los
  archivos que se importan, para las pruebas de PRY-07 y TRA-10.
- Registros de consentimiento que cubren los tres valores de alcance.
- Al menos una serie de segmentos repetidos y un segmento que supere cada
  umbral de TRA-14.
- Un archivo inválido por cada error de importación de TRA-04 y TRA-05.

La hoja de mapa, la capa y el script de R que pide la sección 6.0 llegan con
los tramos 3 y 2, y se agregan a los mismos datos de prueba.

**Criterios de aceptación.**
- PRY-17.1 Cada escenario del tramo 1 usa solo archivos de los datos de prueba
  o generados por la prueba.
- PRY-17.2 Los datos de prueba incluyen un archivo `LEEME` que declara que
  todo es inventado y la licencia del audio.

### PRY-18 Idioma de la interfaz y de los mensajes

**Comportamiento.** Todo texto visible de la interfaz y todo mensaje de `osr`
en la terminal está en español. Los mensajes se escriben en forma impersonal
hasta que se decida el registro de la interfaz (PRODUCTO.md, sección 13, antes
de la v1.0). Cuando un error viene de una biblioteca o del sistema, la interfaz
muestra un mensaje en español que dice qué pasó y qué hacer, y el detalle
técnico va solo al registro técnico (SRV-12).

**Criterios de aceptación.**
- PRY-18.1 Cada mensaje que estas especificaciones citan entre comillas
  invertidas aparece exactamente con ese texto en su situación.
- PRY-18.2 Al forzar un error de permisos del sistema, el texto visible no
  contiene la palabra `Permission` ni ninguna traza técnica.

### PRY-19 Crear un proyecto desde la pantalla de inicio [D-48]

**Comportamiento.** En la pantalla sin proyecto (SRV-01), el formulario
«Crear proyecto» tiene los campos `Carpeta nueva o vacía` y `Nombre` y el
botón `Crear proyecto`. Pulsar el botón, o Enter en cualquiera de los dos
campos, manda a la API la acción de crear con lo escrito. La app interpreta la
carpeta según las reglas de la ruta escrita, crea el proyecto como `osr nuevo`
(PRY-01), con los mismos archivos, los mismos permisos y el mismo único evento
`proyecto_creado`, y lo deja abierto en el mismo proceso, con su bloqueo
(PRY-05). La pestaña pasa al resumen del proyecto (PRY-22) sin recargar a mano.
Mientras espera la respuesta, el botón queda deshabilitado con el texto
`Creando…`.

**Ruta escrita.** Vale igual para PRY-20.

1. Se quitan los espacios y las tabulaciones del principio y del final.
2. Un `~` solo, o un `~` al principio seguido de `/`, se reemplaza por la
   carpeta personal de la cuenta, el `HOME` del proceso `osr`. Un `~` en otra
   posición queda como está.
3. El resultado tiene que ser una ruta absoluta. Una ruta relativa se rechaza,
   porque la persona no ve desde qué carpeta corre `osr`.
4. La ruta se usa tal cual, sin resolver enlaces simbólicos. Una `/` final no
   cambia la carpeta.

La ruta que aparece en los mensajes y en el resumen es la del paso 2.

**Entradas.** Un cuerpo JSON `{"ruta": <texto>, "nombre": <texto>}`. Si
`nombre` falta o es la cadena vacía, el nombre es el último componente de la
ruta, como `osr nuevo` sin `--nombre`. Los demás límites son los de PRY-01.

**Salidas.** Estado 200 con `{"nombre": <nombre>, "carpeta": <ruta>}`. Desde
ese momento el proceso tiene el proyecto abierto (PRY-21).

**Errores.** Nada se crea, el proceso sigue sin proyecto y la pantalla de
inicio permite corregir y volver a intentar. El cuerpo de la respuesta es
`{"error": <código>, "mensaje": <mensaje>}` y la interfaz muestra el mensaje
debajo del botón, conservando lo escrito en los campos.

| Condición | Estado | `error` | Mensaje |
|---|---|---|---|
| El cuerpo no es JSON, no es un objeto o `ruta` falta o no es texto | 400 | `pedido_invalido` | `El pedido no tiene la forma esperada.` |
| La ruta queda vacía después del paso 1 | 422 | `ruta_vacia` | `Hay que escribir la carpeta del proyecto.` |
| La ruta no es absoluta después del paso 2 | 422 | `ruta_relativa` | `La carpeta tiene que ser una ruta completa, que empiece con / o con ~/.` |
| Nombre con más de 200 caracteres o con caracteres de control | 422 | `nombre_invalido` | `El nombre del proyecto tiene que tener entre 1 y 200 caracteres, sin saltos de línea.` |
| La carpeta existe y no está vacía | 409 | `carpeta_ocupada` | `La carpeta <ruta> ya existe y no está vacía. Hay que elegir una carpeta nueva o vacía.` |
| La carpeta que contendría a la ruta no existe | 422 | `sin_carpeta` | `No existe la carpeta <padre>.` |
| Sin permiso de escritura o disco lleno | 500 | `escritura` | `No se pudo crear el proyecto en <ruta> (<motivo en español>).` |
| El proceso ya tiene un proyecto abierto | 409 | `proyecto_ya_abierto` | el de PRY-21 |

**Criterios de aceptación.**
- PRY-19.1 Con `osr` arrancado sin proyecto, crear en `{tmp}/estudio` con el
  nombre `Estudio de prueba` responde 200 con ese nombre y esa carpeta, la
  carpeta tiene exactamente las rutas de PRY-01.1, `.osr/bloqueo` tiene el PID
  de ese `osr`, el registro de eventos tiene una sola línea, de tipo
  `proyecto_creado`, y la página `/` muestra `Estudio de prueba`.
- PRY-19.2 Con `HOME` en `{tmp}/casa`, crear en `~/estudio` crea
  `{tmp}/casa/estudio`.
- PRY-19.3 Con el nombre vacío, `proyecto.toml` guarda como nombre el último
  componente de la ruta.
- PRY-19.4 Cada fila de la tabla de errores, salvo la de escritura y la de
  PRY-21, responde su estado, su código y su mensaje, no crea nada y la página
  `/` sigue mostrando `Sin proyecto abierto`.
- PRY-19.5 Con la carpeta del paso 2 escrita con espacios alrededor, el
  proyecto se crea en la carpeta sin esos espacios.
- PRY-19.6 En Firefox y en Chromium, escribir la carpeta y el nombre y pulsar
  `Crear proyecto` muestra el resumen con el nombre a más tardar 3 s después.
- PRY-19.7 En Firefox y en Chromium, una carpeta ocupada muestra su mensaje
  debajo del botón y los dos campos conservan lo escrito.

### PRY-20 Abrir un proyecto desde la pantalla de inicio [D-48]

**Comportamiento.** En la pantalla sin proyecto, el formulario «Abrir
proyecto» tiene el campo `Carpeta del proyecto` y el botón `Abrir proyecto`.
Pulsar el botón, o Enter en el campo, manda a la API la acción de abrir. La
app interpreta la carpeta con las reglas de la ruta escrita de PRY-19 y abre
el proyecto como `osr abrir` (PRY-04), en el mismo orden, con la validación de
`proyecto.toml`, el bloqueo, las reparaciones, la verificación y el README. La
pestaña pasa al resumen del proyecto (PRY-22). Mientras espera, el botón queda
deshabilitado con el texto `Abriendo…`.

**Entradas.** Un cuerpo JSON `{"ruta": <texto>}`.

**Salidas.** Estado 200 con `{"nombre": <nombre>, "carpeta": <ruta>}`.

**Errores.** Nada se escribe en la carpeta, el proceso sigue sin proyecto, y
la respuesta y la interfaz siguen las mismas reglas que en PRY-19.

| Condición | Estado | `error` | Mensaje |
|---|---|---|---|
| El cuerpo no tiene la forma esperada | 400 | `pedido_invalido` | `El pedido no tiene la forma esperada.` |
| La ruta queda vacía | 422 | `ruta_vacia` | `Hay que escribir la carpeta del proyecto.` |
| La ruta no es absoluta | 422 | `ruta_relativa` | `La carpeta tiene que ser una ruta completa, que empiece con / o con ~/.` |
| La carpeta no existe | 422 | `sin_carpeta` | `No existe la carpeta <ruta>.` |
| Alguno de los errores de PRY-03 | 422 | `proyecto_invalido` | El mensaje de PRY-03 sin el prefijo `osr: ` y con mayúscula inicial, por ejemplo `<ruta> no es un proyecto de Open Social Research (falta proyecto.toml).` |
| Otro proceso tiene el proyecto abierto | 409 | `abierto_en_otro_proceso` | `El proyecto ya está abierto en otro proceso (PID <n>). Hay que usar esa pestaña o cerrar ese proceso.` |
| No se pudo escribir al abrir | 500 | `escritura` | `No se pudo abrir el proyecto en <ruta> (<motivo en español>).` |
| El proceso ya tiene un proyecto abierto | 409 | `proyecto_ya_abierto` | el de PRY-21 |

**Criterios de aceptación.**
- PRY-20.1 Con `osr` arrancado sin proyecto, abrir una copia del proyecto de
  prueba responde 200 con `Proyecto de prueba inventado`, `.osr/bloqueo` tiene
  el PID de ese `osr`, el registro de eventos tiene una línea nueva
  `verificacion` y la página `/` muestra `Proyecto de prueba inventado`.
- PRY-20.2 Abrir una carpeta que no existe, una carpeta sin `proyecto.toml` y
  un proyecto con `version_formato = 2` responden 422 con su mensaje, no
  escriben nada y la página `/` sigue mostrando `Sin proyecto abierto`.
- PRY-20.3 Con el proyecto abierto en otro `osr`, abrirlo responde 409 con el
  PID de ese proceso, que sigue atendiendo pedidos.
- PRY-20.4 En Firefox y en Chromium, escribir la carpeta de la copia del
  proyecto de prueba y pulsar `Abrir proyecto` muestra el resumen a más tardar
  3 s después.

### PRY-21 Un proyecto por proceso [D-48]

**Comportamiento.** Un proceso `osr` abre a lo sumo un proyecto y lo mantiene
hasta terminar, lo haya abierto con `osr abrir`, con `osr nuevo` o desde la
pantalla de inicio. Con un proyecto abierto, la página `/` muestra su resumen
y no la pantalla de inicio, y toda acción de crear o de abrir responde 409 con
`{"error": "proyecto_ya_abierto", "mensaje": "Ya hay un proyecto abierto en
esta sesión. Para abrir otro hay que cerrar Open Social Research y volver a
abrirlo."}` sin escribir nada. Si llegan a la vez varias acciones de crear o
de abrir a un proceso sin proyecto, se atiende una sola y las demás reciben
ese 409. Cerrar el proceso es SRV-15.

**Criterios de aceptación.**
- PRY-21.1 Después de crear `{tmp}/uno` desde la pantalla de inicio, crear
  `{tmp}/dos` responde 409 con el mensaje, `{tmp}/dos` no existe y `{tmp}/uno`
  sigue abierto con su bloqueo.
- PRY-21.2 Con un proyecto abierto con `osr abrir`, abrir otro proyecto
  responde 409 y no escribe nada en ninguno de los dos.
- PRY-21.3 Diez acciones de crear simultáneas, cada una con una carpeta
  distinta, dan exactamente una respuesta 200 y nueve 409, y existe
  exactamente una de las diez carpetas.
- PRY-21.4 Después de abrir un proyecto desde la pantalla de inicio, SIGTERM
  termina `osr` con código 0 y no queda `.osr/bloqueo` (SRV-15).

### PRY-22 Resumen del proyecto en el tramo 1 [D-48, D-49]

**Comportamiento.** Con un proyecto abierto, la página `/` es el resumen del
proyecto (PRY-04). En el tramo 1, hasta que lleguen las listas de PAR-10,
TRA-03 y TRA-13 y el resultado de PRY-11, el resumen muestra el nombre del
proyecto como título, la carpeta (la ruta absoluta con la que se abrió) y uno
de dos bloques. El título de la página es `<nombre> · Open Social Research`.

Un proyecto está vacío cuando `participantes/participantes.csv` no tiene
ninguna línea con texto después del encabezado y `identificable/entrevistas/`
no tiene ninguna carpeta. La app solo cuenta esas líneas y esas carpetas, no
lee su contenido.

**Proyecto vacío.** El rótulo es `Proyecto nuevo`, seguido de `La carpeta del
proyecto está lista. Lo primero es el diseño de la investigación.` y de `Antes
del trabajo de campo se escriben el tema, el posicionamiento, el problema, la
pregunta y el enfoque. Van en espiral: se puede volver a cualquier paso cuando
haga falta.` Debajo van dos grupos numerados.

1. `Diseño de la investigación`, con sus cinco pasos, `Tema e interés`,
   `Posicionamiento y supuestos`, `Problema y estado de la cuestión`,
   `Pregunta y objetivos` y `Enfoque, metodología y métodos`, y el botón
   `Empezar el diseño`.
2. `Trabajo de campo`, con sus tres pasos y el botón `Agregar participante`.

   | Paso | Texto |
   |---|---|
   | `Registrar participantes` | `La herramienta genera el seudónimo; el nombre real queda en la Zona identificable.` |
   | `Registrar su consentimiento` | `Versión del formulario, modalidad, respaldo y alcances.` |
   | `Importar las entrevistas` | `Audio y archivos de Whisper; después, revisar la transcripción.` |

   Nota del grupo: `Se puede empezar en cualquier momento; el diseño se puede
   revisar después.`

Mientras el diseño de la investigación y el registro de participantes no
estén implementados, los dos botones quedan inactivos y cada uno muestra al
lado `Todavía no se hace desde aquí.`

El orden y la espiral siguen la revisión de la bibliografía del curso de
D-49. La investigación «se inicia con un conjunto de consideraciones acerca
de aquella parte del mundo social que se desea estudiar» (Sautu et al., 2005,
p. 30), y teoría, objetivos y metodología «se piensan en conjunto» (Sautu et
al., 2005, p. 34). El diseño cualitativo no es «una secuencia lineal y
ordenada» (Villalobos, 2017, p. 274), y se elige «un enfoque, una metodología
y los métodos específicos que mejor se ajusten a las preguntas» (Della Porta
y Keating, 2013, p. 339).

**Proyecto con datos.** El rótulo es `Proyecto abierto`, seguido de `Este
proyecto ya tiene participantes o entrevistas. Esta versión todavía no los
muestra aquí; están en la carpeta del proyecto.`

El nombre y la carpeta se muestran como texto, nunca como código de la
página. El resumen no muestra nada de `identificable/` ni nada que cambie
entre dos aperturas sin cambios en los archivos, como la hora o el puerto.

**Criterios de aceptación.**
- PRY-22.1 Después de `osr nuevo`, la página `/` muestra el nombre, la
  carpeta, `Proyecto nuevo`, los dos grupos con sus pasos, sus textos y sus
  botones, y no muestra `Sin proyecto abierto`.
- PRY-22.2 Con una copia del proyecto de prueba abierta con `osr abrir`, la
  página `/` muestra `Proyecto abierto` y el texto del proyecto con datos, y no
  muestra `Diseño de la investigación`.
- PRY-22.3 Con la copia del proyecto de prueba abierta, la página `/` no
  contiene ninguno de los nombres de la tabla de correspondencia de prueba.
- PRY-22.4 Un proyecto creado con el nombre `<b>Estudio</b> & otro` muestra
  ese texto tal cual en el título, y la página no tiene ningún elemento `b`.

### PRY-23 Bienvenida del primer uso [D-50]

**Comportamiento.** Mientras la cuenta no haya creado ni abierto ningún
proyecto con `osr`, la página `/` sin proyecto abierto no es la pantalla de
inicio de SRV-01 sino la bienvenida, en dos vistas de la misma página.

1. **Bienvenida.** El título `Open Social Research`, el lema `Investigación
   social aplicada de punta a punta, desde la primera pregunta hasta la
   publicación.`, tres rasgos, el botón `Iniciar`, que tiene el foco inicial,
   y el pie `Versión 0.1 · Nace en el programa de Investigación Social Aplicada
   de la UNED.` Los rasgos son `Libre` (`Código abierto con licencia MIT, sin
   licencias que pagar.`), `Local` (`Todo queda en esta computadora, sin
   cuentas ni conexión a internet.`) y `Pluralista` (`Sirve igual para
   investigación cualitativa, cuantitativa o mixta.`).
2. **Primer proyecto.** Pulsar `Iniciar` muestra, sin pedir otra página, el
   rótulo `Primer proyecto`, el título `Para empezar, hay que crear el primer
   proyecto.`, el texto `Un proyecto es una carpeta de esta computadora. Todo
   lo que se registra queda en ella, en archivos de texto abierto que se
   pueden leer sin la herramienta.`, el formulario `Crear proyecto` de PRY-19
   con el foco en `Carpeta nueva o vacía`, y debajo `¿Ya existe un proyecto?`
   con el botón `Abrir un proyecto existente`, que despliega en el mismo lugar
   el campo `Carpeta del proyecto` y el botón `Abrir proyecto` de PRY-20.

Cada campo de ruta lleva la ayuda `Una ruta completa, que empiece con / o con
~/.`, la de la carpeta nueva agrega `Si la carpeta no existe, se crea.`, y el
nombre lleva `Si queda vacío, se usa el nombre de la carpeta.`

**Primer uso.** Después del primer proyecto creado o abierto con éxito, por
la pantalla o con `osr nuevo` u `osr abrir`, `osr` escribe en su carpeta de
estado (SRV-12) el archivo `primer-proyecto`, con permisos 0600, que contiene
solo la marca de tiempo de ese momento y nunca la ruta ni el nombre del
proyecto. Si el archivo existe, la página `/` sin proyecto es la pantalla de
inicio de SRV-01. Si no se puede escribir, `osr` sigue y la bienvenida vuelve
a salir en el próximo arranque. Pulsar `Iniciar` y cerrar sin crear ni abrir
un proyecto no escribe nada.

**Criterios de aceptación.**
- PRY-23.1 Con una carpeta de estado nueva, `osr` sin proyecto sirve en `/` la
  bienvenida con su título, su lema, sus rasgos y `Iniciar`, y no muestra `Sin
  proyecto abierto`.
- PRY-23.2 Después de crear un proyecto desde la pantalla y cerrar `osr`, un
  `osr` nuevo sin proyecto muestra la pantalla de inicio y no la bienvenida.
- PRY-23.3 Después de `osr nuevo` en la terminal también.
- PRY-23.4 Un intento de crear que termina en error no escribe
  `primer-proyecto`.
- PRY-23.5 `primer-proyecto` tiene permisos 0600 y su contenido es una sola
  línea con una marca de tiempo, sin la ruta del proyecto.
- PRY-23.6 En Firefox y en Chromium, `Iniciar` tiene el foco al cargar, y
  pulsarlo muestra `Para empezar, hay que crear el primer proyecto.` con el
  foco en `Carpeta nueva o vacía`.
- PRY-23.7 En Firefox y en Chromium, crear el proyecto desde la vista
  `Primer proyecto` muestra el resumen a más tardar 3 s después.
- PRY-23.8 En Firefox y en Chromium, `Abrir un proyecto existente` despliega
  `Carpeta del proyecto` y `Abrir proyecto` sin cambiar de página.

## 4. Fuera de alcance de este tramo

- Migraciones de formato con respaldo previo, `FORMATO.md` y negarse a abrir
  formatos más nuevos en modo escritura (v1.0). En el tramo 1 solo existe la
  versión 1 y cualquier otra se rechaza (PRY-03).
- Instancia única que enfoca la pestaña existente, botón de cierre y cierre
  automático (v1.0). En el tramo 1 el bloqueo de PRY-05 evita dos procesos.
- Cambiar de proyecto sin cerrar `osr`, una lista de proyectos recientes y un
  selector de carpetas. El navegador no puede entregar la ruta de una carpeta
  elegida, así que en el tramo 1 la carpeta se escribe (PRY-19, PRY-21).
- Respaldo gestionado a disco externo (v1.0). En el tramo 1 solo la guía del
  README.
- Interfaz para la bitácora y para la hoja de resumen de contacto. La
  plantilla de hoja de resumen (6.1, v0.1 importante) no está en ningún tramo y
  queda para cuando el autor la pida.
- Búsqueda insensible a tildes y mayúsculas (sección 7). El tramo 1 no tiene
  ninguna función de búsqueda propia.
- La prueba del primer día del tramo 1, que el R del autor carga tidyverse y
  sf sin compilar nada nuevo. Es trabajo del tramo 1, pero no pertenece a
  ninguno de estos cuatro módulos.
- Entradas automáticas de la bitácora al fijar, recalcular o cambiar códigos
  (tramos 2 y 3), que usarán el mismo registro de eventos.

## 5. Decisiones

### D-03 Bitácora en el tramo 1 y lugar de los registros automáticos

- **Pregunta.** La bitácora (6.1) es v0.1 imprescindible pero no figura en
  ningún tramo, y el tramo 1 necesita un lugar donde quedan registradas las
  excepciones y los eventos. ¿Dónde escribe la app sus registros?
- **Opciones.** (a) Un archivo aparte de solo agregar, `bitacora/registro.jsonl`,
  y la bitácora de la persona en `bitacora/bitacora.md` sin interfaz. (b)
  Entradas automáticas dentro de `bitacora/bitacora.md`. (c) Postergar la
  bitácora y registrar solo en el índice, lo que viola la regla de fuente de
  verdad.
- **Decisión (2026-10-11).** (a). El autor edita la bitácora con su propio
  editor, y si la app agregara entradas a ese mismo archivo, cualquier editor
  con el archivo abierto podría pisarlas o ser pisado. Un archivo de solo
  agregar en JSON Lines es legible, se audita fácil y no compite con la
  persona. Las entradas automáticas de 6.1 de los tramos 2 y 3 irían al mismo
  registro.
- **Dependen.** PRY-08, PRY-09, PRY-11 y todos los eventos de PAR y TRA.
