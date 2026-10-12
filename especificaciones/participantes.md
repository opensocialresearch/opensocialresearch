# Especificación del módulo participantes (PAR)

Tramo 1 de la v0.1, borrador para aprobación del autor.

## 1. Propósito

Este módulo resguarda a las personas participantes desde la primera
entrevista. Cada persona entra con un seudónimo, su nombre vive en un único
archivo de la zona identificable, su consentimiento queda registrado con cada
alcance por separado, y cada archivo del proyecto muestra en qué zona está y
qué tan sensible es, sin ninguna vía para sacarlo de la máquina.

### Requisitos de PRODUCTO.md que cubre

| Sección | Requisito (texto corto) | IDs |
|---|---|---|
| 9 | «Registro de participantes con un código seudónimo por persona (por ejemplo E01), usado en nombres de archivo, transcripciones, códigos y puntos del mapa» | PAR-01, PAR-02 |
| 9 | «Registro de consentimiento por persona con versión del formulario, fecha, modalidad (escrito u oral grabado) y alcances otorgados por separado […] en archivo plano» | PAR-06, PAR-07, PAR-08, PAR-09 |
| 9 | «Zona identificable y zona de trabajo separadas en la carpeta, con la zona de cada archivo visible en la interfaz sin depender solo del color» | PAR-12, PAR-13 |
| 9 | «La tabla que liga seudónimos con personas vive en un único archivo de la zona identificable y en ningún otro lugar, ni en el índice SQLite ni en ningún otro documento» | PAR-03, PAR-04 |
| 9 | «Marca de sensibilidad por archivo, por capa y por fuente, visible en la interfaz y en el README, y la v0.1 no ofrece ninguna vía de salida» | PAR-14, PAR-15 |
| 9, introducción | «el registro de consentimiento y la separación del material identificable nacen en la v0.1» | PAR-07, PAR-12 |
| 6.5 | «Ficha corta para el material propio de campo […] con hash, fecha, seudónimo y vínculo al consentimiento» (respaldos de consentimiento) | PAR-08 con PRY-07 |
| 5, terminación | «El 100 % de las entrevistas tiene registro de consentimiento con alcances» | PAR-10 |
| 13, antes del tramo 1 | «¿Registrar el consentimiento es requisito para importar un audio o solo un aviso?» | PAR-11 |
| 7, Accesibilidad | «Los códigos y las marcas […] nunca se distinguen solo por el color» | PAR-13 |

## 2. Definiciones

| Término | Definición |
|---|---|
| Participante | Persona entrevistada o que aporta datos, registrada con un seudónimo. |
| Seudónimo | Código que reemplaza a la persona en todo el proyecto, por ejemplo `E01`. |
| Tabla de correspondencia | El archivo `identificable/correspondencia.csv`, único lugar donde un seudónimo se liga con un nombre. |
| Consentimiento | Registro de lo que la persona autorizó, una fila por persona en `participantes/consentimientos.csv`. |
| Alcance | Cada cosa que la persona autoriza o no por separado, como grabar o citar textualmente. |
| Valor de alcance | `otorgado`, `denegado` o `no_consultado`. `no_consultado` significa que el formulario no lo preguntó y se trata como no autorizado. |
| Respaldo | Archivo o tramo de audio que prueba el consentimiento, como el escaneo del formulario firmado. |
| Zona identificable | Todo lo que está bajo `identificable/`. |
| Zona de trabajo | Todo lo que está en la carpeta del proyecto fuera de `identificable/` y de `.osr/`. |
| Marca de sensibilidad | Nivel de cuidado de un archivo según PAR-14. |
| Vía de salida | Cualquier función de la app que copia o envía contenido del proyecto fuera de la carpeta del proyecto. |

## 3. Requisitos

### PAR-01 Seudónimos

**Comportamiento.** Un seudónimo cumple la expresión `^[A-Z][A-Z0-9]{1,15}$`.
Al agregar un participante, la app propone el siguiente seudónimo automático,
formado por `participantes.prefijo` de `proyecto.toml` y el número `n + 1` con
ceros a la izquierda hasta `participantes.digitos` cifras, donde `n` es el mayor
número entre los seudónimos ya usados con la forma `<prefijo><número>`, o 0 si
no hay. Con prefijo `E` y 2 dígitos, la serie es `E01`, `E02` hasta `E99`, y
sigue `E100`. La persona puede cambiar la propuesta por otro seudónimo válido,
por ejemplo para registrar un código que ya usó fuera de la app. Un seudónimo
usado una vez no se vuelve a usar, aunque el participante se haya eliminado
(PAR-05), y no se puede cambiar después de creado porque forma parte de nombres
de archivo.

**Errores.**

| Condición | Mensaje |
|---|---|
| No cumple la expresión | `El seudónimo tiene que empezar con una letra mayúscula y seguir con 1 a 15 letras mayúsculas o números, sin tildes ni espacios.` |
| Ya usado | `El seudónimo <S> ya se usó en este proyecto y no se puede repetir.` |

**Criterios de aceptación.**
- PAR-01.1 En un proyecto nuevo con la configuración por defecto, la propuesta
  del primer participante es `E01` y la del segundo `E02`.
- PAR-01.2 Con participantes `E01` y `E07`, la propuesta es `E08`.
- PAR-01.3 Con `E99` registrado y 2 dígitos, la propuesta es `E100`.
- PAR-01.4 Después de eliminar `E02`, que era el último, la propuesta es `E03`,
  e intentar registrar `E02` a mano muestra el mensaje de seudónimo ya usado.
- PAR-01.5 `e01`, `E 01`, `É01` y `E` muestran el mensaje de formato y no
  registran nada.

### PAR-02 Registro de participantes

**Comportamiento.** `Agregar participante` registra a una persona con el
seudónimo de PAR-01. La app agrega una fila a `participantes/participantes.csv`
y una fila con los campos vacíos a `identificable/correspondencia.csv`, en ese
orden, y anota `participante_creado` con el seudónimo. Las listas de la
interfaz muestran los participantes por seudónimo, en el orden del archivo.

**Formato de `participantes/participantes.csv`.**

| Columna | Tipo | Valores |
|---|---|---|
| `seudonimo` | texto | Según PAR-01, único en el archivo |
| `fecha_alta` | fecha | Día del registro |
| `estado` | texto | `activo` o `eliminado` |

El archivo está en la zona de trabajo y no contiene nombres ni datos de
contacto.

**Errores.** Si `participantes.csv` tiene un error de formato (PRY-11), la
acción está deshabilitada con el texto `El archivo de participantes tiene un
error.`

**Límites.** Se prueba con 500 participantes.

**Criterios de aceptación.**
- PAR-02.1 Después de agregar `E01`, `participantes.csv` tiene la fila
  `E01,<fecha de hoy>,activo` y `correspondencia.csv` la fila `E01,,,`.
- PAR-02.2 Con 500 participantes, la lista del resumen los muestra todos.
- PAR-02.3 Si un seudónimo aparece dos veces en `participantes.csv` por una
  edición a mano, el archivo queda marcado con error según PRY-11 con el
  número de la segunda línea.

### PAR-03 Tabla de correspondencia

**Comportamiento.** `identificable/correspondencia.csv` es el único archivo que
liga un seudónimo con la persona. La vista `Correspondencia` es la única de la
interfaz que muestra su contenido, y permite editar los campos de cada fila
con `Guardar`. Al guardar, la app anota `correspondencia_modificada` con el
seudónimo y los nombres de los campos que cambiaron, sin sus valores. La fecha
y el lugar de la entrevista no van en este archivo sino en `entrevista.toml`
(TRA-01) [D-16].

**Formato.**

| Columna | Tipo | Obligatoria | Límite |
|---|---|---|---|
| `seudonimo` | texto | sí | Según PAR-01, único |
| `nombre` | texto | no | 200 caracteres, sin saltos de línea |
| `contacto` | texto | no | 200 caracteres, sin saltos de línea |
| `nota` | texto | no | 2.000 caracteres, admite saltos de línea |

Cada participante activo tiene exactamente una fila.

**Errores.**

| Condición | Efecto |
|---|---|
| Campo más largo que su límite o con salto de línea donde no se admite | No se guarda, mensaje `<campo> admite hasta <n> caracteres<, sin saltos de línea>.` |
| Al abrir, fila con un seudónimo que no está en `participantes.csv` | Aviso `La tabla de correspondencia tiene una fila para <S>, que no está registrado.` sin cambiar el archivo |
| Al abrir, seudónimo repetido | El archivo queda marcado con error según PRY-11 |

**Criterios de aceptación.**
- PAR-03.1 Escribir un nombre en la vista `Correspondencia` y guardar deja el
  nombre en `correspondencia.csv` y una línea `correspondencia_modificada` con
  `["nombre"]` en el registro, sin el nombre.
- PAR-03.2 Un nombre de 201 caracteres no se guarda y muestra el mensaje.
- PAR-03.3 Ninguna vista salvo `Correspondencia` muestra el contenido de las
  columnas `nombre`, `contacto` o `nota`.

### PAR-04 Los nombres no salen de la tabla de correspondencia

**Comportamiento.** Los valores de `nombre`, `contacto` y `nota` de la tabla de
correspondencia no se copian a ningún otro archivo, al índice, al registro de
eventos, al registro técnico, al README, al almacenamiento del navegador ni a
la respuesta de ninguna ruta salvo las de la vista `Correspondencia`. La app
los lee del archivo cada vez que esa vista los pide.

**Criterios de aceptación.** Se verifican con los datos de prueba de PRY-17,
cuyos nombres inventados también aparecen dichos dentro de las transcripciones
y de los encabezados, después de recorrer todos los escenarios del tramo 1.
- PAR-04.1 Fuera de `identificable/`, ningún archivo del proyecto contiene
  ninguno de los nombres, buscados en UTF-8 en NFC y en NFD, sin distinguir
  mayúsculas.
- PAR-04.2 `.osr/indice.sqlite`, el registro técnico y el registro de eventos
  no contienen ninguno de los nombres ni de los contactos.
- PAR-04.3 Después de visitar la vista `Correspondencia`, el almacenamiento
  local, el de sesión y la base IndexedDB del origen de la app, en Firefox y en
  Chromium, no contienen ninguno de los nombres.
- PAR-04.4 Las respuestas de las rutas que no son de la vista `Correspondencia`
  ni sirven contenido de transcripciones, encabezados, material de origen o
  respaldos no contienen ninguno de los nombres.

### PAR-05 Eliminar un participante sin material

**Comportamiento.** `Eliminar participante` está disponible solo si la persona
no tiene ninguna entrevista ni ningún respaldo de consentimiento. Después de
una confirmación que pide escribir el seudónimo, la app marca su fila de
`participantes.csv` con `estado = eliminado`, borra su fila de
`correspondencia.csv` y su fila de `consentimientos.csv` si la hay, y anota
`participante_eliminado` con el seudónimo. El seudónimo queda reservado
(PAR-01).

**Errores.** Con material asociado, el botón está deshabilitado con el texto
`<S> tiene una entrevista o un respaldo de consentimiento y no se puede
eliminar en esta versión.`

**Criterios de aceptación.**
- PAR-05.1 Eliminar a un participante sin material deja su fila con
  `eliminado`, sin fila en correspondencia ni en consentimientos, y el nombre
  que tenía ya no está en ningún archivo del proyecto.
- PAR-05.2 Con una entrevista creada, el botón está deshabilitado con el texto
  indicado.
- PAR-05.3 Si el seudónimo escrito en la confirmación no coincide, no se
  elimina nada.

### PAR-06 Alcances del consentimiento [D-07]

**Comportamiento.** Los alcances del proyecto se definen en
`consentimiento.alcances` de `proyecto.toml`, en orden, cada uno con `id` y
`etiqueta`. Al crear un proyecto se escriben estos.

| `id` | `etiqueta` |
|---|---|
| `grabar` | Grabar la entrevista |
| `citar_textualmente` | Citar textualmente sus palabras |
| `usar_puntos_mapa` | Usar sus puntos del mapa |
| `archivar` | Archivar los datos al terminar |
| `reutilizar` | Reutilizar los datos en otras investigaciones |

`grabar` y `citar_textualmente` existen siempre, porque los usan PAR-11 y el
tramo 2. Si faltan en `proyecto.toml`, la app los toma como presentes con su
etiqueta por defecto y muestra un aviso. `Agregar alcance` pide un `id` y una
etiqueta, lo agrega al final de la lista, agrega su columna a
`consentimientos.csv` con `no_consultado` en todas las filas existentes y anota
`alcance_agregado`. La etiqueta de un alcance se puede cambiar, lo que anota
`configuracion_cambiada` con el `id` y las etiquetas anterior y nueva, y el
`id` no se puede cambiar.

**Errores.**

| Condición | Mensaje |
|---|---|
| `id` que no cumple `^[a-z][a-z0-9_]{0,39}$` | `El identificador del alcance va en minúsculas sin tildes, con letras, números o guion bajo, hasta 40 caracteres.` |
| `id` repetido | `Ya existe un alcance <id>.` |
| Etiqueta vacía o de más de 100 caracteres | `La etiqueta tiene que tener entre 1 y 100 caracteres.` |
| Ya hay 30 alcances | `El proyecto ya tiene 30 alcances, que es el máximo.` |

**Criterios de aceptación.**
- PAR-06.1 Un proyecto nuevo tiene los cinco alcances de la tabla en ese orden.
- PAR-06.2 Agregar `transcribir_local` con dos consentimientos registrados
  agrega la columna `alcance_transcribir_local` al final de
  `consentimientos.csv` con `no_consultado` en las dos filas.
- PAR-06.3 Si se borra `grabar` de `proyecto.toml` a mano, al abrir el
  formulario de consentimiento sigue mostrando `Grabar la entrevista` y la
  interfaz muestra el aviso.

### PAR-07 Registro de consentimiento

**Comportamiento.** Desde la vista de un participante, `Registrar
consentimiento` abre un formulario con todos los campos de la tabla. Cada
alcance empieza en `no_consultado` y la persona elige su valor uno por uno. Al
guardar, la app escribe la fila en `participantes/consentimientos.csv` y anota
`consentimiento_registrado` con el seudónimo, la versión, la fecha, la
modalidad y el valor de cada alcance. Hay como máximo una fila por persona.

El registro vive en la zona de trabajo porque no tiene datos que identifiquen
a la persona, y así el tramo 2 puede comprobar el alcance de cita sin leer la
zona identificable. Los respaldos, que sí la identifican, van a la zona
identificable (PAR-08).

**Formato de `participantes/consentimientos.csv`.**

| Columna | Tipo | Obligatoria | Valores |
|---|---|---|---|
| `seudonimo` | texto | sí | Un participante activo, único en el archivo |
| `version_formulario` | texto | sí | De 1 a 50 caracteres, sin saltos de línea |
| `fecha` | fecha | sí | No posterior al día de hoy |
| `modalidad` | texto | sí | `escrito` u `oral_grabado` |
| `respaldo` | texto | sí | `ninguno`, `archivos` o `audio_entrevista` (PAR-08) |
| `respaldo_inicio_s` | decimal | solo con `audio_entrevista` | Segundos desde el inicio del audio, con hasta 3 decimales |
| `respaldo_fin_s` | decimal | solo con `audio_entrevista` | Mayor que `respaldo_inicio_s` |
| `alcance_<id>` | texto | sí, una por alcance en el orden de `proyecto.toml` | `otorgado`, `denegado` o `no_consultado` |

**Errores.** El formulario no se guarda y marca el campo.

| Condición | Mensaje |
|---|---|
| Falta la versión, la fecha o la modalidad | `Falta <campo>.` |
| Fecha posterior a hoy | `La fecha del consentimiento no puede ser posterior a hoy.` |
| Versión de más de 50 caracteres | `La versión del formulario admite hasta 50 caracteres.` |

**Criterios de aceptación.**
- PAR-07.1 Registrar el consentimiento de `E01` con versión `v2`, fecha
  `2026-10-09`, modalidad `oral_grabado`, `grabar = otorgado` y los demás sin
  tocar escribe la fila con `no_consultado` en los otros cuatro alcances.
- PAR-07.2 Una fecha de mañana no se guarda y muestra el mensaje.
- PAR-07.3 `consentimientos.csv` se lee en R con `read.csv(ruta,
  fileEncoding = "UTF-8")` con una columna por alcance.
- PAR-07.4 Para un participante con consentimiento, la acción se llama
  `Modificar consentimiento` y abre el formulario con los valores guardados.

### PAR-08 Respaldo del consentimiento

**Comportamiento.** El campo `respaldo` admite tres casos.

- `ninguno`. La interfaz muestra `Sin respaldo` junto al consentimiento.
- `archivos`. La persona agrega de 1 a 10 archivos PDF, JPEG, PNG, HEIC, m4a,
  mp3, wav, ogg u opus, reconocidos por su contenido y no solo por la
  extensión. Cada uno se importa como original (PRY-15) en
  `identificable/consentimientos/<S>-consentimiento-<n>.<ext>`, con `<n>` el
  siguiente número libre desde 1, con su ficha de tipo
  `respaldo_consentimiento` (PRY-07). Los archivos del respaldo de un
  seudónimo son los de esa carpeta cuya ficha lleva ese seudónimo. La app anota
  `respaldo_consentimiento_agregado` con el seudónimo y la cantidad.
- `audio_entrevista`. El consentimiento oral está grabado en el audio de la
  entrevista de esa persona, entre `respaldo_inicio_s` y `respaldo_fin_s`. La
  vista del consentimiento tiene un botón que reproduce ese tramo.

Los respaldos se ven en el navegador por la familia `/respaldo/` (SRV-10), sin
descarga.

**Errores.**

| Condición | Mensaje |
|---|---|
| Formato no admitido o contenido que no coincide con la extensión | `<nombre del archivo> no es un PDF, una imagen o un audio admitido.` |
| Archivo de más de 500 MB | `<nombre del archivo> pasa de 500 MB.` |
| Más de 10 archivos | `Un consentimiento admite hasta 10 archivos de respaldo.` |
| `audio_entrevista` sin audio importado | `La entrevista de <S> no tiene audio todavía.` |
| Tramo con fin no mayor que inicio, o fin mayor que la duración conocida del audio | `El tramo del consentimiento tiene que estar dentro del audio y terminar después de empezar.` |

El mensaje muestra el nombre del archivo en la interfaz, pero ese nombre no se
anota en ningún registro.

**Criterios de aceptación.**
- PAR-08.1 Agregar dos fotos JPEG al consentimiento de `E01` crea
  `E01-consentimiento-1.jpg` y `E01-consentimiento-2.jpg` en 0400, cada una
  con su ficha, y `respaldo = archivos`.
- PAR-08.2 Un archivo `.pdf` cuyo contenido es una imagen PNG no se importa y
  muestra el mensaje.
- PAR-08.3 Con `respaldo = audio_entrevista` entre 2,0 y 9,5 s, el botón
  reproduce desde 2,0 s y se detiene en 9,5 s con una tolerancia de 0,25 s.
- PAR-08.4 Un consentimiento con `respaldo = ninguno` muestra `Sin respaldo`
  con texto, no solo con un ícono o un color.

### PAR-09 Cambios en un consentimiento

**Comportamiento.** `Modificar consentimiento` reescribe la fila y anota
`consentimiento_modificado` con el seudónimo y, por cada campo que cambió, su
valor anterior y el nuevo. Si `grabar` deja de ser `otorgado` y la entrevista
de esa persona ya tiene audio importado, la entrevista y el resumen muestran el
aviso permanente `El consentimiento de <S> ya no autoriza grabar y la
entrevista tiene audio. Hay que decidir qué hacer con ese audio.` La app no
borra ni oculta nada por su cuenta, porque el retiro del consentimiento es de
la v1.0.

**Criterios de aceptación.**
- PAR-09.1 Cambiar `citar_textualmente` de `no_consultado` a `denegado` deja
  en el registro `{"citar_textualmente": ["no_consultado", "denegado"]}` dentro
  del detalle.
- PAR-09.2 Con audio importado, cambiar `grabar` a `denegado` muestra el aviso
  en la entrevista y en el resumen, y el audio sigue en su lugar.
- PAR-09.3 Volver `grabar` a `otorgado` quita el aviso.

### PAR-10 Cobertura del consentimiento

**Comportamiento.** El resumen del proyecto muestra `Consentimiento registrado
en <n> de <m> entrevistas.`, donde `m` es la cantidad de entrevistas y `n` la
de entrevistas cuyo participante tiene fila en `consentimientos.csv`. Cada
entrevista muestra `Consentimiento registrado` o `Consentimiento pendiente`, y
cada participante sin fila muestra `Sin consentimiento registrado`, siempre con
texto e ícono.

**Criterios de aceptación.**
- PAR-10.1 Con dos entrevistas y un consentimiento, el resumen dice
  `Consentimiento registrado en 1 de 2 entrevistas.` y la entrevista sin
  consentimiento dice `Consentimiento pendiente`.
- PAR-10.2 Al registrar el segundo, el resumen dice `en 2 de 2` sin recargar la
  página.

### PAR-11 Consentimiento al importar audio [D-02]

**Comportamiento.** Antes de importar el audio de una entrevista (TRA-02), la
app aplica estas reglas al consentimiento del participante `S`, en orden.

| Situación | Resultado | Excepción |
|---|---|---|
| Hay fila y `grabar = otorgado` | Se importa | No hace falta |
| Hay fila y `grabar = denegado` | Bloqueado, `El consentimiento de <S> no autoriza grabar. No se puede importar el audio.` | No permitida |
| Hay fila y `grabar = no_consultado` | Bloqueado, `El consentimiento de <S> no dice si autoriza grabar.` | `audio_sin_consentimiento` con motivo `grabar_no_consultado` |
| No hay fila | Bloqueado, `<S> no tiene consentimiento registrado.` | `audio_sin_consentimiento` con motivo `sin_registro` |

La excepción sigue PRY-09. Después de una excepción por `sin_registro`, la
entrevista muestra `Consentimiento pendiente` (PAR-10) hasta que se registre.
La regla no se aplica a los audios de respaldo de PAR-08. Importar una
transcripción para una entrevista que ya tiene audio no vuelve a aplicar la
regla.

**Criterios de aceptación.**
- PAR-11.1 Con `grabar = denegado`, importar el audio muestra el mensaje con
  solo el botón `Entendido` y no escribe nada.
- PAR-11.2 Sin consentimiento, elegir `Cancelar` no escribe nada, y elegir
  `Hacerlo igual` importa el audio y anota `excepcion_confirmada` con
  `audio_sin_consentimiento`, el seudónimo y `sin_registro`.
- PAR-11.3 Con `grabar = otorgado`, el audio se importa sin ningún diálogo.

### PAR-12 Dos zonas

**Comportamiento.** La zona de un archivo la define su ruta. En el tramo 1, la
app escribe solo en la zona identificable todo lo que identifica o puede
identificar a una persona, y en la zona de trabajo solo seudónimos y datos que
no la identifican.

| Zona identificable | Zona de trabajo |
|---|---|
| Tabla de correspondencia | `participantes.csv` y `consentimientos.csv` |
| Respaldos de consentimiento y sus fichas | `bitacora/` |
| Audios, sus fichas y `entrevista.toml` con fecha y lugar | `sensibilidad.csv`, `proyecto.toml`, `README.md` |
| Transcripciones sin seudonimizar, encabezados y material de `origen/` | Las versiones seudonimizadas del tramo 2 en `entrevistas/` |

`.osr/` no pertenece a ninguna zona y cumple PRY-10.

**Criterios de aceptación.**
- PAR-12.1 Después de recorrer todos los escenarios del tramo 1, fuera de
  `identificable/` y de `.osr/` no hay ningún audio, imagen ni PDF, ni ningún
  archivo con texto de las transcripciones de prueba, de los encabezados de
  prueba, de los lugares de `entrevista.toml` ni con nombres originales de
  archivos.
- PAR-12.2 Todo archivo que la app crea con contenido de la columna
  identificable queda bajo `identificable/`.

### PAR-13 Zona y sensibilidad visibles en la interfaz

**Comportamiento.** Cada lugar de la interfaz que muestra un archivo, y la
vista `Archivos del proyecto`, que lista todos los archivos de la carpeta salvo
los de `.osr/` agrupados por carpeta, muestra junto a cada archivo dos
etiquetas de texto, la zona (`Zona identificable` o `Zona de trabajo`) y la
marca de sensibilidad (PAR-14), cada una con un ícono distinto y con su texto
en el nombre accesible del elemento. El color puede reforzar, nunca reemplazar
al texto.

**Criterios de aceptación.**
- PAR-13.1 En la vista de una entrevista con audio, el audio muestra el texto
  `Zona identificable` y el texto `Identificable`.
- PAR-13.2 Con los estilos de color anulados (todo en gris), las etiquetas de
  zona y sensibilidad siguen visibles y se distinguen por su texto.
- PAR-13.3 Un lector de pantalla obtiene, para cada archivo de `Archivos del
  proyecto`, un nombre accesible que incluye su zona y su marca.

### PAR-14 Marcas de sensibilidad [D-15]

**Comportamiento.** Cada archivo tiene una de cuatro marcas.

| Marca | Texto visible | A qué se aplica |
|---|---|---|
| `identificable` | `Identificable` | Todo archivo bajo `identificable/`, siempre, sin excepción y sin cambio posible |
| `sensible` | `Sensible` | Por defecto, los archivos que la app crea en la zona de trabajo con datos de participantes (`participantes.csv`, `consentimientos.csv`, `bitacora/bitacora.md`, `bitacora/registro.jsonl`, `sensibilidad.csv`) |
| `no_sensible` | `No sensible` | Por defecto, `proyecto.toml` y `README.md` |
| `sin_marca` | `Sin marca` | Los demás archivos de la zona de trabajo hasta que la persona los marque |

En la vista `Archivos del proyecto`, la persona puede cambiar la marca de un
archivo de la zona de trabajo a `sensible` o `no_sensible`. La app guarda solo
esas elecciones en `sensibilidad.csv`, con las columnas `ruta`, relativa a la
raíz, y `nivel`, y anota `sensibilidad_cambiada` con la ruta y los niveles
anterior y nuevo. Una fila de `sensibilidad.csv` cuya ruta no existe se ignora.
Las capas y las fuentes del tramo 3 usarán estas mismas cuatro marcas.

**Errores.**

| Condición | Mensaje |
|---|---|
| Cambiar la marca de un archivo bajo `identificable/` | La opción no existe, y la etiqueta dice `Identificable (por estar en la zona identificable)` |
| Pedir `identificable` para un archivo de la zona de trabajo | La opción no existe, y la ayuda dice `Lo identificable va en la carpeta identificable/. Hay que mover ahí el archivo.` |

**Criterios de aceptación.**
- PAR-14.1 En un proyecto nuevo, `Archivos del proyecto` muestra `Sensible`
  para `participantes.csv` y `No sensible` para `README.md`, y
  `sensibilidad.csv` solo tiene el encabezado.
- PAR-14.2 Un archivo `notas.txt` que la persona deja en la raíz aparece con
  `Sin marca`, y al marcarlo `Sensible`, `sensibilidad.csv` tiene la fila
  `notas.txt,sensible` y el README lo lista con esa marca.
- PAR-14.3 Ningún control permite cambiar la marca de un archivo bajo
  `identificable/`.

### PAR-15 Sin vía de salida

**Comportamiento.** La v0.1 no tiene ninguna función que exporte, comparta,
envíe, descargue o copie contenido del proyecto fuera de su carpeta. Ver un
audio o un respaldo en el navegador no es una salida, porque se sirve para
verse en la página y no como descarga. La exportación de transcripciones de
6.6 llega en el tramo 2 [D-13].

**Criterios de aceptación.**
- PAR-15.1 Ninguna respuesta de ninguna ruta de SRV-10 lleva
  `Content-Disposition: attachment`.
- PAR-15.2 Después de recorrer todos los escenarios del tramo 1, fuera de la
  carpeta del proyecto solo cambiaron el registro técnico y, si existió, el
  archivo de un solo uso de SRV-03, que ya no está.
- PAR-15.3 Ninguna vista del tramo 1 tiene un botón o enlace con los textos
  `Exportar`, `Compartir`, `Enviar` o `Descargar`.

## 4. Fuera de alcance de este tramo

- Seudonimización con la tabla de reemplazos y `osr seudonimizar` (tramo 2).
  El tramo 1 deja preparados los seudónimos, el alcance `citar_textualmente` y
  el registro de consentimientos en la zona de trabajo.
- Tabla de casos con atributos por participante (tramo 2), que extenderá
  `participantes.csv`.
- Uso del alcance `usar_puntos_mapa` en la importación de marcas (tramo 3).
- Retiro del consentimiento con lista de todo lo derivado (v1.0).
- Quitar un respaldo agregado por error (los originales son inmutables).
- Confirmación de reemplazos en la interfaz, compuerta de IA, revisión previa a
  publicar, chequeo de geoprivacidad, bitácora de salidas, limpieza de EXIF,
  aviso de carpetas sincronizadas y comprobación del cifrado de disco (v1.0).
- Bóveda con contraseña y plan de retención (después). La eliminación del audio
  tras la revisión está en TRA-23.
- Plantillas de consentimiento y de hoja informativa (v1.0).
- Más de una persona entrevistada por entrevista (ver D-09 en
  transcripciones.md).

## 5. Decisiones

### D-02 Consentimiento como requisito para importar audio

- **Pregunta.** ¿Registrar el consentimiento es requisito para importar un
  audio o solo un aviso? (PRODUCTO.md, sección 13, antes del tramo 1).
- **Opciones.** (a) Requisito sin excepción. (b) Solo un aviso. (c) Bloqueo por
  defecto con excepción confirmada y registrada, salvo cuando el consentimiento
  deniega grabar, que no tiene excepción.
- **Decisión (2026-10-11).** (c). Sigue la regla general de PRODUCTO.md 6.0,
  que bloquea por defecto y deja excepciones registradas salvo en lo que la
  ética no permite. Las entrevistas de octubre se graban antes de que exista la
  herramienta, con el consentimiento en papel o en el audio, y el plan B pide
  importarlas sin rehacer nada, así que exigirlo sin excepción frenaría esa
  importación. La entrevista queda marcada `Consentimiento pendiente` hasta
  registrarlo, y el criterio de terminación del 100 % lo sigue midiendo.
- **Dependen.** PAR-11, PAR-10, TRA-02.

### D-07 Alcances por defecto del consentimiento

- **Pregunta.** ¿Qué alcances trae un proyecto nuevo? La sección 9 nombra
  cinco, y la sección 13 pregunta además por transcribir con software local y
  por usar fragmentos para evaluar la transcripción automática.
- **Opciones.** (a) Los cinco de la sección 9. (b) Los cinco más
  `transcribir_local`. (c) Los cinco más `transcribir_local` y
  `evaluar_transcripcion`.
- **Decisión (2026-10-11).** (a), con la posibilidad de agregar los demás por
  proyecto (PAR-06). Un alcance por defecto que el formulario real no preguntó
  queda en `no_consultado` en todas las filas y solo agrega ruido. El autor
  agrega `transcribir_local` y `evaluar_transcripcion` si su formulario los
  pide.
- **Dependen.** PAR-06, PAR-07.

### D-15 Niveles de sensibilidad

- **Pregunta.** PRODUCTO.md pide una marca de sensibilidad por archivo, capa y
  fuente pero no define sus niveles. ¿Cuáles son?
- **Opciones.** (a) Cuatro marcas, `identificable` fija por zona, `sensible`,
  `no_sensible` y `sin_marca`. (b) Una escala numérica de 1 a 3. (c) Solo
  `identificable` y `no identificable`, igual que las zonas.
- **Decisión (2026-10-11).** (a). Separa lo que identifica, que manda la
  carpeta, de lo seudonimizado pero delicado, como la bitácora, y deja visible
  lo que nadie revisó todavía en lugar de suponerlo inocuo. La opción (c)
  repetiría la zona sin agregar información.
- **Dependen.** PAR-13, PAR-14, PRY-16.

Las decisiones D-09 (una persona y un audio por entrevista), D-13
(exportación en el tramo 2) y D-16 (dónde viven la fecha y el lugar) también
tocan este módulo y están desarrolladas en transcripciones.md.
