# Especificación del módulo transcripciones (TRA)

Tramo 1 de la v0.1, borrador para aprobación del autor.

## 1. Propósito

Este módulo trae al proyecto los audios de las entrevistas y las
transcripciones que ya produce la cadena del autor, sin rehacer nada, las
convierte a un formato canónico documentado y ofrece un editor de revisión
sincronizado con el audio y operable solo con teclado. Toda transcripción nace
sin revisar, la salida de la máquina se conserva intacta, y cada segmento
lleva un identificador estable que el tramo 2 va a usar para codificar.

### Requisitos de PRODUCTO.md que cubre

| Sección | Requisito (texto corto) | IDs |
|---|---|---|
| 6.5 | «Importar audio (m4a, mp3, wav, ogg, opus) […] a la zona que corresponda, con duración y estado visibles, y guardar el original inmutable, sin recomprimir y con su hash» (solo audio, ver D-14) | TRA-02, TRA-03 |
| 6.5 | «Ficha corta para el material propio de campo» (audios) | TRA-02 con PRY-07 |
| 6.6 | «Importar transcripciones en el JSON de Whisper o faster-whisper que produce la cadena del autor y en WebVTT, junto con su audio, conservando las marcas de tiempo por segmento y las probabilidades cuando el archivo las trae» | TRA-04 a TRA-08, TRA-11 |
| 6.6 | «Editor de revisión sincronizado con el audio, clic en el texto reproduce desde esa marca, atajos para pausar y retroceder de 3 a 5 s, velocidad de 0,5x a 2x, todo operable con teclado» | TRA-15 a TRA-19, TRA-24 |
| 6.6 | «Toda transcripción nace como automática sin revisar, con estado por segmento y por transcripción, registro de cuándo se revisó y con qué modelo se generó» | TRA-09, TRA-13 |
| 6.6 | «La salida cruda de la máquina se conserva junto a la versión revisada, y nunca se re-transcribe automáticamente sobre una transcripción revisada» | TRA-10, TRA-12 |
| 6.6 | «Turnos de habla asignados a mano (persona entrevistadora o entrevistada) con atajo» (el filtro para codificar es del tramo 2) | TRA-08, TRA-18, TRA-19 |
| 6.6 | «Marcar como sospechosos los segmentos con `no_speech_prob` mayor que 0,6 y `avg_logprob` menor que -1,0, con `compression_ratio` mayor que 2,4, o con el mismo texto repetido […] omitiendo con un aviso la regla cuyo campo no traiga el JSON, y saltar al siguiente sospechoso con un atajo» | TRA-14 |
| 6.6 | «Registrar el tiempo de revisión de cada transcripción, como dato y sin meta» | TRA-22 |
| 6.0 | «deshacer y rehacer en codificación y edición» | TRA-21 |
| 6.7.1 | anclas «por identificador de segmento y desplazamiento» (preparación para el tramo 2) | TRA-11, TRA-20 |
| 7, Respuesta y escala | «Saltar a una marca de tiempo del audio. 200 ms o menos» y «Abrir una transcripción de 60 minutos. 1 s o menos» | TRA-15, TRA-16 |
| 7, Accesibilidad | «revisión y codificación completas solo con teclado» y «las marcas de IA o de sospecha nunca se distinguen solo por el color» | TRA-18, TRA-24 |
| 5, Plan B | Las entrevistas de octubre se transcriben con la cadena actual «y la v0.1 las importa después sin rehacer nada» | TRA-04 a TRA-08 |
| 13, antes del tramo 1 | formato canónico, unidad de codificación, corregir después de codificar, convención, cantidad y duración de entrevistas | D-01, D-04, D-05, D-06 |

## 2. Definiciones

| Término | Definición |
|---|---|
| Entrevista | Unidad que reúne el audio y la transcripción de una conversación con un participante. Su identificador es el seudónimo del participante. |
| Cadena del autor | Los scripts `transcribir.py`, `hablantes.py` y `markdown.py` con los que el autor transcribe hoy fuera de la app. |
| Archivo de origen | Archivo que entra en una importación de transcripción, cualquiera de los tipos de TRA-04 salvo el audio. |
| Fuente de segmentos | El archivo de origen del que salen los segmentos, `cadena_hablantes`, `cadena_whisper`, `whisper_json` o `webvtt`. |
| Segmento | Tramo de la transcripción con inicio, fin, hablante, texto y estado. |
| Transcripción canónica | El archivo `transcripcion.json` de una entrevista (TRA-11). |
| Rol de hablante | `entrevistadora`, `entrevistada`, `otra` o sin asignar (`null`). |
| Hablante de origen | Etiqueta que trae el archivo de origen para quien habla, como el número de la diarización o la voz de WebVTT. |
| Segmento con foco | El segmento cuyo campo de texto tiene el foco del teclado. |
| Posición del audio | Segundo del audio en el que está la reproducción, en marcha o en pausa. |
| `m:ss` | Formato de tiempo de la interfaz, minutos y segundos, y `h:mm:ss` desde una hora. Los segundos se truncan. |

## 3. Requisitos

### TRA-01 Entrevistas [D-09, D-16]

**Comportamiento.** En la vista de un participante activo, `Nueva entrevista`
crea la carpeta `identificable/entrevistas/<id>/` con `entrevista.toml`, donde
`<id>` es el seudónimo, y anota `entrevista_creada`. En el tramo 1 cada
participante tiene como máximo una entrevista. El formulario pide la fecha y el
lugar, los dos opcionales, porque son datos que pueden identificar a la persona
y por eso viven en la zona identificable.

**Formato de `entrevista.toml`.**

| Clave | Tipo | Obligatoria | Contenido |
|---|---|---|---|
| `formato` | texto | sí | `"osr-entrevista"` |
| `version_formato` | entero | sí | `1` |
| `id` | texto | sí | Seudónimo del participante |
| `participante` | texto | sí | El mismo seudónimo |
| `creada` | marca de tiempo | sí | Momento de creación |
| `fecha` | fecha | no | Día de la entrevista, no posterior a hoy |
| `lugar` | texto | no | Hasta 200 caracteres, sin saltos de línea |

**Errores.**

| Condición | Mensaje |
|---|---|
| El participante ya tiene entrevista | `<S> ya tiene una entrevista. En esta versión cada participante tiene una sola.` |
| Fecha posterior a hoy | `La fecha de la entrevista no puede ser posterior a hoy.` |
| Lugar de más de 200 caracteres o con saltos de línea | `El lugar admite hasta 200 caracteres, sin saltos de línea.` |

**Criterios de aceptación.**
- TRA-01.1 Crear la entrevista de `E01` con fecha `2026-10-09` y lugar `Barrio
  Inventado` crea `identificable/entrevistas/E01/entrevista.toml` con esas
  claves y anota `entrevista_creada` sin el lugar.
- TRA-01.2 Para un participante con entrevista, `Nueva entrevista` está
  deshabilitado con el mensaje de la tabla.
- TRA-01.3 El lugar no aparece en ningún archivo fuera de `identificable/`.

### TRA-02 Importar audio [D-02, D-14]

**Comportamiento.** En la vista de una entrevista sin audio, `Importar audio`
acepta un archivo elegido o arrastrado. La app aplica la regla de
consentimiento de PAR-11, reconoce el formato por el contenido, recibe el
archivo, calcula su huella, comprueba que no esté repetido, lo guarda como
original en `identificable/entrevistas/<id>/<id>.<ext>` según PRY-15, escribe
su ficha según PRY-07 y anota `audio_importado` con el identificador, el
formato, los bytes, la huella y la duración. El formulario permite dar la fecha
de grabación, que va a `fecha_material` de la ficha.

**Formatos.**

| Formato | Extensiones aceptadas | Contenido que se exige | `<ext>` guardada |
|---|---|---|---|
| m4a | `.m4a` | Bytes 4 a 7 iguales a `ftyp` | `m4a` |
| mp3 | `.mp3` | Empieza con `ID3`, o con un byte `FF` seguido de un byte cuyos 3 bits altos valen 1 | `mp3` |
| wav | `.wav` | `RIFF` en el byte 0 y `WAVE` en el byte 8 | `wav` |
| ogg | `.ogg` | `OggS` en el byte 0 | `ogg` |
| opus | `.opus` | `OggS` en el byte 0 | `opus` |

La extensión se compara sin distinguir mayúsculas.

**Errores.** Nada queda escrito si hay un error o si la persona cancela
durante la transferencia.

| Condición | Mensaje |
|---|---|
| La entrevista ya tiene audio, presente o eliminado | `La entrevista <id> ya tiene audio.` |
| Extensión no admitida | `<archivo> no es un audio admitido. Se aceptan m4a, mp3, wav, ogg y opus.` |
| El contenido no corresponde a la extensión | `<archivo> dice ser <formato> pero su contenido no lo es.` |
| Más de 4 GiB | `<archivo> pasa de 4 GiB.` |
| La misma huella que otro audio del proyecto, presente o eliminado | `Este audio ya está importado en la entrevista <id>.` |
| Bloqueos de consentimiento | Los de PAR-11 |
| Copia que no coincide | El de PRY-15 |

`<archivo>` es el nombre original, que se muestra solo en la interfaz.

**Límites.** Un audio por entrevista. Hasta 4 GiB por archivo [D-06].

**Criterios de aceptación.**
- TRA-02.1 Importar `E01` de los datos de prueba crea `E01.ogg` idéntico al
  original, en 0400, con su ficha, y el registro tiene `audio_importado` sin el
  nombre original.
- TRA-02.2 Un archivo `.mp3` cuyo contenido empieza con `OggS` no se importa y
  muestra el mensaje de contenido.
- TRA-02.3 Importar dos veces el mismo audio en entrevistas distintas muestra
  el mensaje de audio repetido con el identificador de la primera.
- TRA-02.4 Cancelar a mitad de la transferencia de un archivo de 500 MB no deja
  ningún archivo nuevo, ni temporal, en la carpeta de la entrevista.
- TRA-02.5 Los audios de prueba en Ogg Opus y en m4a se reproducen en Firefox y
  en Chromium estables en el equipo del autor.

### TRA-03 Duración y estado visibles

**Comportamiento.** La app lee la duración del audio de los datos del
contenedor, sin depender de que el navegador lo reproduzca. Si no la puede
leer, la importación sigue, la ficha queda sin `duracion_s` y la interfaz
muestra `Duración desconocida`. La duración se muestra en `m:ss` o `h:mm:ss`.
Cada entrevista muestra dos etiquetas de texto con ícono, una del audio y otra
de la transcripción.

| Etiqueta del audio | Cuándo |
|---|---|
| `Sin audio` | No hay audio |
| `Audio <duración>` | Audio presente |
| `Audio eliminado el <fecha>` | TRA-23 |
| `Falta el audio` | PRY-11, paso 4 |
| `El audio cambió desde su importación` | PRY-11, paso 5 |

| Etiqueta de la transcripción | Cuándo |
|---|---|
| `Sin transcripción` | No hay `transcripcion.json` |
| `Transcripción sin revisar` | `estado = sin_revisar` |
| `Transcripción en revisión (<r> de <t> segmentos revisados)` | `estado = en_revision` |
| `Transcripción revisada` | `estado = revisada` |

**Criterios de aceptación.**
- TRA-03.1 La duración que muestra la app para cada audio de prueba difiere en
  1 s o menos de la duración real del archivo.
- TRA-03.2 Un audio de prueba con el contenedor alterado para que no declare su
  duración se importa y muestra `Duración desconocida`.
- TRA-03.3 Una entrevista con audio y transcripción en revisión con 12 de 40
  segmentos revisados muestra las dos etiquetas con esos números.

### TRA-04 Archivos que se reconocen [D-01]

**Comportamiento.** En la vista de una entrevista, `Importar transcripción`
acepta de 1 a 6 archivos elegidos o arrastrados juntos. La app reconoce el tipo
de cada uno por su contenido.

| Tipo | Qué es | Cómo se reconoce |
|---|---|---|
| `audio` | El audio de la entrevista | Según TRA-02 |
| `cadena_whisper` | `*.whisper.json` de `transcribir.py` | JSON cuya raíz es una lista, y cada elemento es un objeto con `start` y `end` numéricos, `text` de texto y `words` lista de objetos con `s` y `e` numéricos y `w` de texto |
| `cadena_hablantes` | `*.hablantes.json` de `hablantes.py` | JSON cuya raíz es un objeto con `turnos`, lista de listas de tres elementos `[número, número, entero]`, y `frases`, lista de objetos con `sp` entero, `start` y `end` numéricos y `text` de texto |
| `whisper_json` | Salida JSON de Whisper de OpenAI [verificar nombres de campos con la versión usada] | JSON cuya raíz es un objeto con `segments`, lista de objetos con `start` y `end` numéricos y `text` de texto |
| `webvtt` | Subtítulos WebVTT | Texto UTF-8 cuya primera línea es `WEBVTT`, sola o seguida de espacio o tabulador y más texto |
| `cadena_markdown` | `*.md` de `markdown.py` | Texto UTF-8 con al menos una línea que cumple `^\*\*\[(\d{2,}):(\d{2})\] (.+?):\*\* (.*)$` |
| `encabezado` | `*.encabezado.md` escrito por el autor | Texto UTF-8 con extensión `.md` y sin ninguna línea que cumpla la expresión anterior |

**Formato de la cadena del autor.** Así escriben hoy los scripts, y la app lo
acepta tal cual.

- `transcribir.py` usa faster-whisper con `language="es"`, filtro de voz y
  tiempos por palabra, y escribe una lista de segmentos sin sangría fija. Cada
  segmento tiene `start` y `end` en segundos, `text` sin espacios en los
  extremos y `words`, la lista de palabras con `s`, `e` y `w`, donde `w` trae
  el espacio inicial que le pone Whisper. No guarda el modelo, el idioma ni
  `avg_logprob`, `no_speech_prob` o `compression_ratio`.

  ```json
  [{"start": 0.0, "end": 3.52, "text": "Buenas noches, ¿tiene un minuto?",
    "words": [{"s": 0.0, "e": 0.41, "w": " Buenas"}, {"s": 0.41, "e": 0.9, "w": " noches,"}]}]
  ```

- `hablantes.py` separa hablantes y escribe un objeto con `turnos`, la lista de
  turnos de la diarización como `[inicio, fin, número de hablante]`, y
  `frases`. Cada frase une las palabras seguidas de un mismo hablante mientras
  la pausa entre ellas sea menor de 1,5 s, y tiene `sp`, el número de hablante,
  `start`, `end` y `text`. Los números de hablante salen en un orden
  arbitrario.

  ```json
  {"turnos": [[0.03, 3.6, 1], [3.9, 9.2, 0]],
   "frases": [{"sp": 1, "start": 0.0, "end": 3.52, "text": "Buenas noches, ¿tiene un minuto?"}]}
  ```

- `markdown.py` escribe primero el encabezado, que es el contenido de
  `*.encabezado.md` sin espacios finales seguido de una línea en blanco, o
  `# Entrevista <código>` si no hay encabezado. Después escribe una línea por
  frase, separadas por líneas en blanco, con la forma `**[mm:ss] Etiqueta:**
  texto`, donde `mm` son los minutos enteros con al menos dos cifras, `ss` los
  segundos enteros con dos cifras, y `Etiqueta` el nombre que el autor dio a
  ese número de hablante o `Hablante <n>` si no le dio ninguno.

- `*.encabezado.md` es texto libre del autor con la ficha de la entrevista.

**Errores.**

| Condición | Mensaje |
|---|---|
| Un archivo no corresponde a ningún tipo | `No se reconoce el formato de <archivo>.` |
| Texto que no es UTF-8 válido | `<archivo> no está en UTF-8.` |
| JSON inválido | `<archivo> no es JSON válido (línea <l>, columna <c>).` |

**Criterios de aceptación.**
- TRA-04.1 Los cuatro archivos de la cadena de cada entrevista de prueba se
  reconocen como `cadena_whisper`, `cadena_hablantes`, `cadena_markdown` y
  `encabezado`, sin importar sus nombres.
- TRA-04.2 Un `whisper.json` de prueba renombrado `datos.txt` se reconoce como
  `cadena_whisper`.
- TRA-04.3 Un JSON con raíz lista cuyos elementos no tienen `words` no se
  reconoce y muestra el mensaje de formato.

### TRA-05 Combinaciones y validación

**Comportamiento.** Una importación es todo o nada. La app comprueba las
combinaciones, después valida cada archivo, y solo si todo pasa escribe algo.
Cada importación tiene como máximo un archivo de cada tipo y exactamente una
fuente de segmentos.

| Fuente de segmentos | Archivos que la pueden acompañar |
|---|---|
| `cadena_hablantes` | `cadena_whisper`, `cadena_markdown`, `encabezado`, `audio` |
| `cadena_whisper` | `encabezado`, `audio` |
| `whisper_json` | `encabezado`, `audio` |
| `webvtt` | `encabezado`, `audio` |

Cuando `cadena_hablantes` y `cadena_whisper` vienen juntos, los segmentos salen
de `cadena_hablantes` y `cadena_whisper` se guarda solo como material de
origen. Si viene un `audio`, se importa primero según TRA-02 y PAR-11, y si
falla no se importa nada. La entrevista tiene que tener su audio presente, ya
importado o en la misma operación.

**Validación de la fuente.**
- Cada `start`, `end` o marca de WebVTT es un número finito con `0 ≤ inicio ≤
  fin`.
- Cada segmento empieza en el mismo instante o después que el anterior. Los
  solapamientos entre un segmento y el siguiente se aceptan.
- Hay al menos un segmento.
- En WebVTT, cada marca de tiempo tiene la forma `hh:mm:ss.ttt` o `mm:ss.ttt`.

**Errores.** `<n>` es la posición del segmento contando desde 1.

| Condición | Mensaje | Excepción |
|---|---|---|
| Dos archivos del mismo tipo | `Hay dos archivos de tipo <tipo>.` | No |
| Ninguna fuente de segmentos | `Falta la transcripción. Se necesita un whisper.json, un hablantes.json, un JSON de Whisper o un WebVTT.` | No |
| Una combinación fuera de la tabla | `Hay más de una transcripción. Se usa una sola por importación.` | No |
| `cadena_markdown` sin `cadena_hablantes` | `El Markdown de la cadena solo se importa junto con su hablantes.json.` | No |
| La entrevista no tiene audio presente | `Primero hay que importar el audio de la entrevista.` | No |
| Campo que falta o de otro tipo | `<archivo>, segmento <n>: falta <campo> o no es <tipo>.` | No |
| Tiempos inválidos | `<archivo>, segmento <n>: los tiempos no son válidos (<inicio> a <fin>).` | No |
| Fuera de orden | `<archivo>, segmento <n>: empieza antes que el anterior.` | No |
| Sin segmentos | `<archivo> no tiene segmentos.` | No |
| Marca de WebVTT mal escrita | `<archivo>, línea <l>: marca de tiempo no válida.` | No |
| Archivo de más de 50 MB o más de 20.000 segmentos | `<archivo> pasa el límite de <límite>.` | No |
| Con la duración del audio conocida, el último fin pasa la duración en más de 2,0 s | `La transcripción termina en <m:ss> y el audio dura <m:ss>. Puede que no sea el audio correcto.` | `transcripcion_mas_larga_que_audio` |

**Límites.** 50 MB por archivo y 20.000 segmentos por transcripción [D-06].

**Criterios de aceptación.**
- TRA-05.1 Cada archivo inválido de los datos de prueba produce exactamente su
  mensaje, con el número de segmento o de línea correcto, y no escribe nada.
- TRA-05.2 Importar juntos `whisper.json` y un WebVTT muestra `Hay más de una
  transcripción.`
- TRA-05.3 Importar juntos el audio con un consentimiento que deniega grabar y
  un `hablantes.json` válido no escribe ni el audio ni la transcripción.
- TRA-05.4 Un WebVTT cuyo último segmento termina 5 s después del fin del
  audio muestra el bloqueo, y con `Hacerlo igual` se importa y se anota la
  excepción.

### TRA-06 Construcción de los segmentos

**Comportamiento.** La app crea un segmento canónico por cada segmento de la
fuente, en el mismo orden, con estas reglas comunes.

- `id` según TRA-11, `s0001` para el primero y así en orden.
- `inicio` y `fin` redondeados al milisegundo más cercano.
- `texto` sin espacios en blanco en los extremos y en NFC, con cada salto de
  línea, junto con los espacios que lo rodean, reemplazado por un solo
  espacio. Los segmentos con texto vacío se conservan.
- `estado = sin_revisar`, `revisado_en = null`.
- `hablante` según TRA-08 y `sospecha` según TRA-14.

| Fuente | Un segmento por | `texto` | `hablante_origen` | `metricas` |
|---|---|---|---|---|
| `cadena_hablantes` | Cada elemento de `frases` | `text`, o la línea alineada del Markdown (TRA-07) | `sp` como texto | `null` |
| `cadena_whisper` | Cada elemento de la lista | `text` | `null` | Según la regla siguiente |
| `whisper_json` | Cada elemento de `segments` | `text` | `null` | Según la regla siguiente |
| `webvtt` | Cada bloque de subtítulo | Las líneas del bloque unidas con un espacio, sin etiquetas `<…>` y con `&amp;`, `&lt;`, `&gt;`, `&nbsp;`, `&lrm;` y `&rlm;` decodificadas | El texto de la primera etiqueta `<v …>`, o `null` | `null` |

`metricas` es `null` si el segmento de origen no trae ninguno de los campos
`avg_logprob`, `no_speech_prob` y `compression_ratio`, y si trae alguno es un
objeto con esas tres claves, con `null` en las que falten. En WebVTT se ignoran
los identificadores de bloque y los bloques `NOTE`, `STYLE` y `REGION`.

**Criterios de aceptación.**
- TRA-06.1 Importar el `hablantes.json` de prueba de `E01` sin Markdown produce
  tantos segmentos como frases, con los mismos tiempos redondeados al
  milisegundo y los mismos textos sin espacios en los extremos.
- TRA-06.2 Un bloque WebVTT con `<v Persona A>Hola &amp; chau</v>` en dos
  líneas produce `texto = "Hola & chau"` unido con un espacio y
  `hablante_origen = "Persona A"`.
- TRA-06.3 Un `whisper_json` con `avg_logprob` y `no_speech_prob` pero sin
  `compression_ratio` produce `metricas` con `compression_ratio = null`.
- TRA-06.4 Un segmento de la fuente con `text = "   "` produce un segmento con
  `texto = ""`.

### TRA-07 Markdown de la cadena [D-10]

**Comportamiento.** El Markdown de la cadena puede traer correcciones a mano
que no están en los JSON, como un nombre reemplazado. Por eso, cuando viene con
su `hablantes.json`, la app intenta alinearlo.

- Las líneas de intervención son las que cumplen la expresión de TRA-04, en
  orden. Las líneas antes de la primera son el encabezado y se ignoran, porque
  el encabezado entra por su propio archivo.
- El Markdown está alineado si tiene tantas líneas de intervención como frases
  tiene `hablantes.json`, y si la línea `i` lleva exactamente el `mm:ss` que
  `markdown.py` escribiría para el `start` de la frase `i`, es decir los
  minutos enteros con al menos dos cifras y los segundos enteros con dos.
- Después de la primera intervención, toda línea que no esté en blanco tiene
  que ser de intervención.

Si está alineado, el texto de cada segmento es el último grupo de la línea `i`
sin espacios en los extremos, y la etiqueta de la línea sirve para proponer
los hablantes (TRA-08). Si no está alineado, la importación se detiene con uno
de estos mensajes y dos botones, `Cancelar` e `Importar sin el Markdown`. La
segunda opción toma los textos de `hablantes.json`, guarda igual el Markdown
como material de origen y anota `markdown_alineado = false` en la importación.

| Condición | Mensaje |
|---|---|
| Cantidad distinta | `El Markdown tiene <k> intervenciones y hablantes.json tiene <m> frases.` |
| Tiempo distinto | `El Markdown no coincide con hablantes.json en la intervención <i> (Markdown [<mm:ss>], hablantes.json [<mm:ss>]).` |
| Texto fuera de una intervención | `El Markdown tiene texto fuera de las intervenciones en la línea <l>.` |

**Criterios de aceptación.**
- TRA-07.1 El Markdown de prueba de `E01`, generado por `markdown.py`, queda
  alineado, y si en la línea 3 se cambió un nombre inventado por `[E01]`, el
  segmento 3 tiene el texto con `[E01]`.
- TRA-07.2 Un Markdown con una intervención borrada muestra el mensaje de
  cantidad, y `Importar sin el Markdown` produce los textos de
  `hablantes.json`.
- TRA-07.3 Una frase con `start = 600.4` se alinea con `**[10:00] …`, y una con
  `start = 6000.0` con `**[100:00] …`.

### TRA-08 Asignación de hablantes al importar

**Comportamiento.** Si la fuente trae hablantes de origen, antes de escribir
la app muestra una fila por cada valor distinto, en el orden en que aparece por
primera vez, con el valor, la cantidad de segmentos, la suma de sus duraciones
en `m:ss`, los primeros 120 caracteres de su primer segmento, un botón que
reproduce ese segmento, y un selector con `Persona entrevistadora`, `Persona
entrevistada`, `Otra persona` y `Sin asignar`. Varios valores pueden ir al
mismo rol, por ejemplo cuando la diarización partió a una persona en dos.

El valor inicial de cada selector es `Sin asignar`, salvo que haya un Markdown
alineado, en cuyo caso se propone según la etiqueta de las líneas de ese
hablante.

| Etiqueta del Markdown | Rol propuesto |
|---|---|
| Igual al seudónimo de la entrevista | `entrevistada` |
| `Entrevistador`, `Entrevistadora` o `Persona entrevistadora` sin distinguir mayúsculas, o igual a `transcripcion.etiqueta_entrevistadora` | `entrevistadora` |
| `Hablante <n>` | sin asignar |
| Cualquier otra | `otra` |

Si un mismo valor tiene líneas con etiquetas distintas, se propone `Sin
asignar`. Si la fuente no trae hablantes de origen, no hay paso de asignación
y todos los segmentos quedan sin asignar. La asignación elegida queda en
`importacion.toml` (TRA-10).

**Criterios de aceptación.**
- TRA-08.1 Con los archivos de prueba de `E01`, cuyo Markdown usa
  `Entrevistador` para el hablante 0 y `E01` para el 1, el paso propone
  `Persona entrevistadora` para 0 y `Persona entrevistada` para 1.
- TRA-08.2 Asignar 0 y 2 a `entrevistadora` deja `hablante = "entrevistadora"`
  en todos los segmentos con `hablante_origen` `"0"` o `"2"`.
- TRA-08.3 Un WebVTT sin etiquetas `<v>` se importa sin paso de asignación y
  con `hablante = null` en todos los segmentos.

### TRA-09 Modelo declarado

**Comportamiento.** Los archivos de la cadena no guardan con qué modelo se
generaron, así que el formulario de importación pide `Modelo con que se
generó`, de 1 a 100 caracteres, o la casilla `No se sabe`, que guarda
`desconocido`. El valor va a `modelo` en `transcripcion.json` y en
`importacion.toml`, y el editor lo muestra en su encabezado.

**Criterios de aceptación.**
- TRA-09.1 Sin escribir el modelo ni marcar la casilla, la importación no
  avanza y muestra `Falta el modelo con que se generó la transcripción.`
- TRA-09.2 Con `large-v3-turbo`, el editor muestra `Modelo: large-v3-turbo`.

### TRA-10 Conservación del material de origen

**Comportamiento.** Cada importación crea
`identificable/entrevistas/<id>/origen/<marca>/` y guarda ahí una copia byte a
byte de cada archivo de origen con un nombre derivado del identificador, más
`importacion.toml`. Todo queda en 0400 y la app nunca lo modifica ni lo borra. Al terminar la
importación, la app escribe `transcripcion.json` (TRA-11) y anota
`transcripcion_importada` con el identificador, la fuente, la cantidad de
segmentos y la marca.
La vista de la entrevista muestra el encabezado de la importación en uso, si lo
hay, como texto de solo lectura.

| Tipo | Nombre guardado |
|---|---|
| `cadena_whisper` | `<id>.whisper.json` |
| `cadena_hablantes` | `<id>.hablantes.json` |
| `whisper_json` | `<id>.whisper-json.json` |
| `webvtt` | `<id>.vtt` |
| `cadena_markdown` | `<id>.md` |
| `encabezado` | `<id>.encabezado.md` |

**Formato de `importacion.toml`.**

| Clave | Tipo | Contenido |
|---|---|---|
| `formato` | texto | `"osr-importacion"` |
| `version_formato` | entero | `1` |
| `marca` | texto | La `<marca>` de la carpeta |
| `fuente` | texto | Tipo de la fuente de segmentos |
| `modelo` | texto | TRA-09 |
| `markdown_alineado` | texto | `"si"`, `"no"` o `"no_aplica"` |
| `archivos` | lista de tablas | Por archivo, `tipo`, `archivo`, `nombre_original`, `sha256` y `bytes` |
| `hablantes` | lista de tablas | Por hablante de origen, `origen` y `rol` |
| `reglas_sospecha` | tabla | `aplicadas`, `omitidas` y los umbrales usados (TRA-14) |
| `excepciones` | lista de textos | Códigos de PRY-09 confirmados en esta importación |

**Criterios de aceptación.**
- TRA-10.1 Después de importar los cuatro archivos de la cadena de `E01`, la
  carpeta `origen/<marca>/` tiene cinco archivos, cuatro idénticos a los de
  origen y `importacion.toml`, todos en 0400.
- TRA-10.2 Después de editar 20 segmentos, las huellas de los archivos de
  `origen/` no cambiaron.
- TRA-10.3 Si un archivo de origen se llamaba con un nombre del segundo
  conjunto de PRY-17, ese nombre aparece en `importacion.toml` y en ningún
  archivo fuera de `identificable/`, ni en `.osr/`, ni en el registro de
  eventos, ni en el registro técnico.

### TRA-11 Formato canónico de la transcripción [D-01, D-04]

**Comportamiento.** Cada entrevista con transcripción tiene
`identificable/entrevistas/<id>/transcripcion.json`, en 0600, en JSON según
PRY-06, con las claves en este orden. Es la única transcripción que la app
edita, y su formato es contrato público para la lectura sin la app y para la
migración futura a Automerge.

**Nivel superior.**

| Clave | Tipo | Contenido |
|---|---|---|
| `formato` | texto | `"osr-transcripcion"` |
| `version_formato` | entero | `1` |
| `entrevista` | texto | Identificador de la entrevista |
| `audio` | texto o `null` | Nombre del audio en la carpeta de la entrevista |
| `audio_sha256` | texto o `null` | Huella del audio |
| `importacion` | texto | `"origen/<marca>"` de la importación en uso |
| `modelo` | texto | TRA-09 |
| `estado` | texto | `sin_revisar`, `en_revision` o `revisada` |
| `modificada` | booleano | `true` desde el primer cambio de una persona |
| `revisada_en` | marca de tiempo o `null` | TRA-13 |
| `sin_revisar_al_cerrar` | entero o `null` | Segmentos sin revisar al marcarla revisada con excepción |
| `tiempo_revision_s` | entero | TRA-22 |
| `reglas_sospecha` | objeto | `{"aplicadas": [...], "omitidas": [...]}` con códigos de TRA-14 |
| `siguiente_id` | entero | Número que tomará el próximo segmento nuevo |
| `segmentos` | lista | Segmentos ordenados por `inicio` |

**Cada segmento.**

| Clave | Tipo | Contenido |
|---|---|---|
| `id` | texto | `s` seguido del número con al menos 4 cifras, por ejemplo `s0001` o `s12345` |
| `inicio` | número | Segundos, hasta 3 decimales, mayor o igual que 0 |
| `fin` | número | Segundos, hasta 3 decimales, mayor o igual que `inicio` |
| `hablante` | texto o `null` | Rol de hablante |
| `hablante_origen` | texto o `null` | TRA-06 |
| `texto` | texto | NFC, sin saltos de línea |
| `estado` | texto | `sin_revisar` o `revisado` |
| `revisado_en` | marca de tiempo o `null` | Último paso a `revisado` |
| `sospecha` | lista de textos | Códigos de TRA-14 en el orden `sin_voz`, `compresion`, `repeticion` |
| `metricas` | objeto o `null` | TRA-06 |

**Validez.** Un archivo es válido si cumple los tipos de las tablas, si los
`id` son únicos y su número es menor que `siguiente_id`, y si cada `inicio` es
mayor o igual que el del segmento anterior. Un archivo inválido se trata según
PRY-11. Los desplazamientos dentro de `texto`, que el tramo 2 usará para anclar
la codificación, se cuentan en caracteres Unicode (valores escalares) sobre el
texto en NFC.

**Criterios de aceptación.**
- TRA-11.1 La transcripción importada de cada entrevista de prueba es válida
  según estas tablas, verificado con un esquema JSON derivado de ellas.
- TRA-11.2 Importar dos veces los mismos archivos en proyectos distintos con la
  misma marca de tiempo produce dos `transcripcion.json` idénticos byte a byte.
- TRA-11.3 Si se cambia a mano un `inicio` para que quede antes que el del
  segmento anterior, al abrir la transcripción queda marcada con error según
  PRY-11 y el editor no la abre.

### TRA-12 Reimportar

**Comportamiento.** Importar una transcripción en una entrevista que ya tiene
una nunca ocurre sin una acción de la persona.

- Si `modificada = false`, la app pide confirmar con `La transcripción actual
  de <id> no tiene cambios y se va a reemplazar.` y los botones `Cancelar` y
  `Reemplazar`. Al confirmar, escribe la nueva y anota
  `transcripcion_reemplazada`.
- Si `modificada = true`, la acción está bloqueada con `La transcripción de
  <id> tiene cambios de revisión.` y la excepción
  `reemplazar_transcripcion_con_cambios`. Al confirmarla, la actual se guarda
  como `transcripcion.anterior-<marca>.json` en 0400, se escribe la nueva y se
  anota el evento.

En los dos casos, las importaciones anteriores siguen en `origen/`.

**Criterios de aceptación.**
- TRA-12.1 Reimportar sobre una transcripción sin cambios la reemplaza y deja
  dos carpetas en `origen/`.
- TRA-12.2 Reimportar sobre una transcripción con un segmento editado y elegir
  `Cancelar` no cambia ningún archivo.
- TRA-12.3 Con la excepción confirmada, `transcripcion.anterior-<marca>.json`
  es idéntico a la transcripción que había antes.

### TRA-13 Estados de revisión [D-05]

**Comportamiento.**

- Toda transcripción importada empieza con `estado = sin_revisar` y todos sus
  segmentos `sin_revisar`.
- El primer cambio de una persona, sea editar, marcar, asignar hablante,
  dividir, unir, eliminar o ajustar tiempos, pone `modificada = true` y, si el
  estado era `sin_revisar`, lo pasa a `en_revision`.
- `Ctrl+Enter` sobre un segmento `sin_revisar` lo pasa a `revisado`, pone
  `revisado_en` y mueve el foco al segmento siguiente. Sobre uno `revisado` lo
  vuelve a `sin_revisar`, pone `revisado_en = null` y deja el foco donde está.
- Editar el texto o el hablante de un segmento `revisado` no cambia su estado.
  Dividir o unir deja los segmentos que resultan en `sin_revisar`.
- `Marcar transcripción como revisada` con todos los segmentos `revisado` pone
  `estado = revisada`, `revisada_en` y anota `transcripcion_revisada` con la
  cantidad de segmentos. Con `n > 0` segmentos sin revisar, la acción está
  bloqueada con `Quedan <n> segmentos sin revisar.` y la excepción
  `revisada_con_segmentos_sin_revisar`, que al confirmarse la marca igual y
  guarda `sin_revisar_al_cerrar = n`.
- Una transcripción `revisada` se abre en solo lectura. La reproducción
  funciona, la edición no. `Reabrir para editar` pide confirmar con `La
  transcripción vuelve a estar en revisión.`, la pasa a `en_revision`, pone
  `revisada_en = null` y `sin_revisar_al_cerrar = null` y anota
  `transcripcion_reabierta`. En el tramo 2 esta acción también va a listar los
  segmentos codificados.

**Criterios de aceptación.**
- TRA-13.1 Una transcripción recién importada muestra `Transcripción sin
  revisar` y todos sus segmentos `Sin revisar`.
- TRA-13.2 Editar una letra la pasa a `en_revision` con `modificada = true` en
  disco.
- TRA-13.3 Marcarla revisada con 3 segmentos sin revisar muestra `Quedan 3
  segmentos sin revisar.`, y con la excepción queda `revisada` con
  `sin_revisar_al_cerrar = 3`.
- TRA-13.4 En una transcripción revisada, escribir en un segmento no cambia su
  texto y `Ctrl+Enter` no cambia su estado.
- TRA-13.5 `Reabrir para editar` vuelve a permitir la edición y anota el
  evento.

### TRA-14 Segmentos sospechosos

**Comportamiento.** Al importar, la app aplica tres reglas con los umbrales de
`transcripcion.sospecha` de `proyecto.toml`, que por defecto son los de
PRODUCTO.md, tomados de los valores por defecto de Whisper [verificar en el
código de la versión usada].

| Código | Texto en la interfaz | Regla | Campos que necesita |
|---|---|---|---|
| `sin_voz` | `posible texto en un silencio` | `no_speech_prob > 0,6` y `avg_logprob < -1,0` | `no_speech_prob` y `avg_logprob` |
| `compresion` | `posible texto repetitivo` | `compression_ratio > 2,4` | `compression_ratio` |
| `repeticion` | `texto repetido` | El segmento pertenece a una serie de 2 o más segmentos seguidos con el mismo texto normalizado no vacío | Ninguno |

El texto normalizado se obtiene pasando a NFC y a minúsculas, quitando los
caracteres `. , ; : ¡ ! ¿ ? … " « » ' ( ) -`, reemplazando cada serie de
espacios por uno solo y quitando los espacios de los extremos. Se marcan todos
los segmentos de la serie, también el primero.

Si al menos un segmento de la fuente no trae alguno de los campos que una
regla necesita, esa regla se omite para toda la transcripción, queda en
`reglas_sospecha.omitidas` y el editor muestra `No se aplicaron las reglas de
<textos de las reglas> porque el archivo no trae <campos>.` Con los archivos
actuales de la cadena del autor solo se aplica `repeticion`. Las marcas se
calculan al importar y no se recalculan al editar ni al cambiar los umbrales,
que valen para las importaciones siguientes. Una marca no impide revisar el
segmento, y un segmento marcado y revisado conserva la marca.

Cada segmento marcado muestra `Sospechoso: <textos>` con ícono. El encabezado
del editor muestra `Sospechosos sin revisar: <n>`. `Ctrl+Alt+S` lleva el foco
al siguiente segmento con marca y en `sin_revisar` después del segmento con
foco, vuelve al principio al llegar al final, y si no queda ninguno muestra `No
quedan segmentos sospechosos sin revisar.`

**Criterios de aceptación.**
- TRA-14.1 Con el `whisper_json` de prueba, un segmento con `no_speech_prob =
  0,7` y `avg_logprob = -1,2` queda marcado `sin_voz`, y uno con `0,7` y
  `-0,5` no.
- TRA-14.2 Un segmento con `compression_ratio = 2,4` no queda marcado, y uno
  con `2,41` sí.
- TRA-14.3 Tres segmentos seguidos con textos `Sí, claro.`, `sí claro` y `Sí,
  claro` quedan los tres marcados `repeticion`.
- TRA-14.4 Con los archivos de la cadena de prueba, `reglas_sospecha.omitidas`
  es `["sin_voz", "compresion"]` y el editor muestra el aviso.
- TRA-14.5 Con dos segmentos sospechosos sin revisar en las posiciones 5 y 20
  y el foco en el 10, `Ctrl+Alt+S` lleva al 20, otra vez al 5, y después de
  marcar los dos como revisados muestra el mensaje.
- TRA-14.6 Con `transcripcion.sospecha.compression_ratio = 3.0` en
  `proyecto.toml`, una importación nueva no marca un segmento con `2,5`.

### TRA-15 Abrir y mostrar una transcripción

**Comportamiento.** El editor de una entrevista muestra un encabezado y la
lista de segmentos.

- **Encabezado.** Identificador, etiqueta del audio y de la transcripción
  (TRA-03), `<r> de <t> segmentos revisados`, `Sospechosos sin revisar: <n>`,
  `Modelo: <modelo>`, tiempo de revisión (TRA-22), estado del guardado
  (PRY-13), velocidad, controles de reproducción y el aviso de reglas omitidas
  si lo hay.
- **Cada segmento.** Un botón con su tiempo de inicio en `m:ss`, la etiqueta
  del hablante, el texto editable, la etiqueta de estado (`Revisado` o `Sin
  revisar`) y la de sospecha si la tiene, todas con texto.

| Rol | Etiqueta |
|---|---|
| `entrevistada` | El seudónimo de la entrevista |
| `entrevistadora` | `transcripcion.etiqueta_entrevistadora` |
| `otra` | `Otra persona` |
| `null` | `Sin asignar` |

**Límites.** Una transcripción de 60 minutos, de unas 10.000 palabras y 1.000
segmentos, muestra su primer segmento editable 1 s o menos después de pedir
abrirla. Se aceptan hasta 20.000 segmentos, sin meta de tiempo por encima de
los 60 minutos en la v0.1 [D-06].

**Criterios de aceptación.**
- TRA-15.1 En el equipo del autor, con Firefox y con Chromium, abrir una
  transcripción de prueba de 60 minutos generada para la medición tarda 1 s o
  menos en 5 de 5 repeticiones.
- TRA-15.2 Un segmento con `hablante = "entrevistada"` en la entrevista `E01`
  muestra `E01`, y uno con `null` muestra `Sin asignar`.
- TRA-15.3 Un segmento que empieza en 3725,8 s muestra `1:02:05`.

### TRA-16 Reproducción sincronizada [D-11]

**Comportamiento.**

- Clic en el botón de tiempo de un segmento, `Ctrl+clic` en su texto o
  `Ctrl+Alt+Espacio` con el foco en él reproducen desde su `inicio`. Un clic
  simple en el texto solo pone el cursor para editar y no mueve el audio.
- El segmento en reproducción es el de mayor `inicio` menor o igual que la
  posición del audio y con `fin` mayor que ella. Muestra el ícono `▶` y el
  texto accesible `En reproducción`, que se actualizan 250 ms o menos después
  de que la posición entra en él.
- `Seguir el audio`, activado por defecto, desplaza la vista para mostrar el
  segmento en reproducción cuando sale de la pantalla, salvo durante los 3 s
  siguientes a la última tecla escrita en un segmento o a un desplazamiento
  manual. Reproducir nunca mueve el cursor de edición.
- Sin audio presente, los controles de reproducción están deshabilitados con
  la etiqueta del audio (TRA-03), y la edición sigue funcionando.

**Límites.** De la acción a la reproducción en la nueva posición pasan 200 ms
o menos, y la posición queda a 0,1 s o menos del `inicio` pedido.

**Criterios de aceptación.**
- TRA-16.1 En 20 saltos a segmentos al azar de un audio de prueba de 60
  minutos, en Firefox y en Chromium, cada salto tarda 200 ms o menos y queda a
  0,1 s o menos del `inicio`.
- TRA-16.2 Un clic simple en el texto de un segmento no cambia la posición del
  audio.
- TRA-16.3 Con el audio en 12,0 s y segmentos `[10,0; 14,0)` y `[11,5;
  13,0)`, el marcado es el segundo.
- TRA-16.4 Con el audio eliminado, los controles están deshabilitados y la
  edición de un segmento se guarda.

### TRA-17 Velocidad, pausa y retroceso

**Comportamiento.**

- `Esc` reproduce o pausa, y al reanudar sigue desde donde pausó.
- `Ctrl+Alt+J` retrocede `transcripcion.retroceso_s` segundos, sin bajar de 0,
  y conserva el estado de reproducción o de pausa.
- La velocidad toma uno de los valores 0,5, 0,75, 1, 1,25, 1,5, 1,75 y 2, con
  un selector y con `Ctrl+Alt+,` y `Ctrl+Alt+.`, que bajan o suben un valor sin
  pasar de los extremos. Al abrir el editor la velocidad es 1. El tono de la
  voz no cambia con la velocidad.

**Criterios de aceptación.**
- TRA-17.1 Con `retroceso_s = 3` y el audio en 10,0 s, `Ctrl+Alt+J` lo deja en
  7,0 s con 0,1 s de tolerancia, y en 2,0 s lo deja en 0.
- TRA-17.2 Desde velocidad 1, tres `Ctrl+Alt+.` dan 1,75, y desde 2 otro
  `Ctrl+Alt+.` la deja en 2.
- TRA-17.3 `Esc` en reproducción pausa, y otro `Esc` reanuda desde la misma
  posición con 0,1 s de tolerancia.

### TRA-18 Atajos de teclado [D-11]

**Comportamiento.** Los atajos funcionan con el foco en cualquier parte del
editor, también dentro del texto de un segmento, y la tecla no llega al campo
ni dispara la acción del navegador. Con un diálogo abierto, `Esc` lo cierra y no
toca el audio.

| Acción | Atajo |
|---|---|
| Reproducir o pausar | `Esc` |
| Retroceder `retroceso_s` segundos | `Ctrl+Alt+J` |
| Reproducir desde el inicio del segmento con foco | `Ctrl+Alt+Espacio` |
| Bajar o subir la velocidad | `Ctrl+Alt+,` y `Ctrl+Alt+.` |
| Segmento anterior o siguiente | `Alt+↑` y `Alt+↓` |
| Marcar o desmarcar como revisado | `Ctrl+Enter` |
| Siguiente sospechoso sin revisar | `Ctrl+Alt+S` |
| Hablante entrevistadora, entrevistada, otra o sin asignar | `Ctrl+Alt+1`, `Ctrl+Alt+2`, `Ctrl+Alt+3` y `Ctrl+Alt+0` |
| Dividir en el cursor | `Ctrl+Alt+D` |
| Unir con el siguiente | `Ctrl+Alt+U` |
| Eliminar el segmento | `Ctrl+Alt+E` |
| Fijar el inicio o el fin en la posición del audio | `Ctrl+Alt+I` y `Ctrl+Alt+O` |
| Deshacer | `Ctrl+Z` |
| Rehacer | `Ctrl+Shift+Z` o `Ctrl+Y` |
| Ver la lista de atajos | `Ctrl+K` |

La tabla evita las teclas que los navegadores o el equipo del autor ya usan,
como `F1`, `F3`, `F5`, `F6`, `F7`, `F11`, `F12`, `Alt+←`, `Alt+→`, `Alt+1` a
`Alt+9`, `Ctrl+1` a `Ctrl+9`, `Ctrl+T`, `Ctrl+W`, `Ctrl+N` y `F9`. `Ctrl+K`
abre un panel con esta tabla que se cierra con `Esc`.

**Criterios de aceptación.**
- TRA-18.1 En Firefox y en Chromium, cada atajo produce su efecto con el foco
  dentro del texto de un segmento y no escribe ningún carácter en él.
- TRA-18.2 Ningún atajo cambia de pestaña, abre un menú del navegador ni mueve
  el foco a la barra de direcciones.
- TRA-18.3 `Ctrl+K` muestra el panel con todas las filas de la tabla.

### TRA-19 Edición de segmentos [D-05]

**Comportamiento.** Mientras la transcripción no está `revisada`, se puede
hacer lo siguiente. Cada cambio se guarda según PRY-13 y aplica TRA-13.

- **Texto.** Edición libre de hasta 10.000 caracteres por segmento. El texto
  no admite saltos de línea, `Enter` solo no hace nada y un salto de línea
  pegado se reemplaza por un espacio.
- **Hablante.** Con los atajos de TRA-18 o con un selector.
- **Dividir.** Con el cursor en la posición `c`, contada en caracteres Unicode
  con `0 < c < largo del texto`. El tiempo de corte `t` es la posición del
  audio si está estrictamente entre `inicio` y `fin`, y si no `inicio + (fin −
  inicio) × c / largo`, redondeado al milisegundo. El segmento conserva su `id`
  con `[inicio, t)` y el texto anterior al cursor, y se crea uno nuevo con `id`
  tomado de `siguiente_id`, con `[t, fin)`, el texto desde el cursor, el mismo
  hablante y hablante de origen, la misma `sospecha` y las mismas `metricas`.
  Los textos quedan sin espacios en los extremos.
- **Unir.** Con el segmento siguiente, si tienen el mismo `hablante`. El
  resultado conserva el `id`, el `inicio` y el `hablante_origen` del primero,
  toma como `fin` el mayor de los dos, une los textos con un espacio, o sin él
  si alguno está vacío, junta las marcas de sospecha, pone `metricas = null` y
  queda `sin_revisar`. El `id` del segundo se retira.
- **Eliminar.** Quita el segmento y retira su `id`. El foco pasa al siguiente,
  o al anterior si era el último. No pide confirmación porque se puede
  deshacer.
- **Fijar inicio o fin.** Toma la posición del audio redondeada al
  milisegundo.

**Errores.**

| Condición | Mensaje |
|---|---|
| Dividir con el cursor al principio o al final | `No hay nada que dividir en esa posición.` |
| Unir hablantes distintos | `No se pueden unir segmentos de hablantes distintos.` |
| Unir el último segmento | `No hay un segmento siguiente.` |
| Eliminar el único segmento | `La transcripción tiene que tener al menos un segmento.` |
| Fijar un inicio menor que el inicio del segmento anterior o mayor que el del siguiente, o no menor que el propio fin | `Esa posición rompe el orden de los segmentos.` |
| Fijar un fin no mayor que el propio inicio | `El fin tiene que ser posterior al inicio.` |
| Fijar sin audio presente | La acción está deshabilitada |
| Texto de más de 10.000 caracteres | No se aceptan más caracteres y se muestra `El segmento admite hasta 10.000 caracteres.` |

**Criterios de aceptación.**
- TRA-19.1 Dividir `s0007` `[20,000; 30,000)` con texto de 10 caracteres en
  `c = 4` y el audio en pausa en 50,0 s da `s0007 [20,000; 24,000)` y un
  segmento nuevo `[24,000; 30,000)` con el `id` igual al `siguiente_id` previo.
- TRA-19.2 Con el audio en 22,5 s, la misma división corta en 22,500.
- TRA-19.3 Unir `s0003` (`entrevistada`) con `s0004` (`entrevistadora`) muestra
  el mensaje y no cambia nada.
- TRA-19.4 Pegar `uno\ndos` deja `uno dos` en el texto.
- TRA-19.5 Eliminar un segmento y deshacer lo devuelve con el mismo `id`, en la
  misma posición y con los mismos datos.

### TRA-20 Identificadores estables

**Comportamiento.** Esta es la preparación del tramo 1 para codificar en el
tramo 2. El `id` de un segmento no cambia mientras el segmento existe, un `id`
retirado no vuelve a usarse en esa transcripción, y `siguiente_id` nunca baja,
ni siquiera al deshacer. Deshacer la creación de un segmento lo retira, y
rehacerla lo devuelve con el mismo `id`.

**Criterios de aceptación.**
- TRA-20.1 Sobre una transcripción de prueba de 500 segmentos, 1.000
  operaciones al azar entre editar, dividir, unir, eliminar, deshacer y rehacer
  dejan todos los `id` únicos, ningún `id` retirado reaparece salvo por
  deshacer o rehacer la operación que lo retiró, y `siguiente_id` es mayor que
  todo número de `id` usado.
- TRA-20.2 En la misma prueba, cada segmento que no fue dividido, unido ni
  eliminado conserva su `id` y su texto, salvo las ediciones que recibió.

### TRA-21 Deshacer y rehacer

**Comportamiento.** El editor guarda la historia de cambios de la
transcripción abierta desde que se abrió en esa pestaña. Una serie de
caracteres escritos o borrados en un mismo segmento, sin pausa mayor de 1 s y
sin salir del segmento, es un paso. Cada otra acción de TRA-13 y TRA-19 es un
paso. Se conservan al menos los últimos 200 pasos. Una acción nueva borra lo
que se podía rehacer. Deshacer y rehacer se guardan en disco como cualquier
cambio. Marcar la transcripción revisada o reabrirla no se puede deshacer y
vacía la historia, y la historia no se conserva al cerrar la pestaña.

**Criterios de aceptación.**
- TRA-21.1 Escribir `hola` sin pausa y deshacer borra las cuatro letras de una
  vez.
- TRA-21.2 Después de 250 acciones, se pueden deshacer al menos 200.
- TRA-21.3 Deshacer una división vuelve el segmento a sus tiempos, su texto y
  su estado anteriores, y el archivo en disco lo refleja en 1 s o menos.
- TRA-21.4 Después de marcar la transcripción revisada, `Ctrl+Z` no cambia
  nada.

### TRA-22 Tiempo de revisión

**Comportamiento.** El editor suma los segundos en que se cumplen tres
condiciones a la vez, que el editor de esa transcripción es la pestaña visible,
que la transcripción no está `revisada`, y que el audio se está reproduciendo o
hubo una tecla, un clic o un desplazamiento en los últimos 60 s. El total va a
`tiempo_revision_s` con cada guardado y al menos cada 60 s. El encabezado
muestra `Tiempo de revisión: <h:mm:ss> (<x,x> min por minuto de audio)`, con la
proporción solo si se conoce la duración. No hay meta ni comparación.

**Criterios de aceptación.**
- TRA-22.1 Con un reloj simulado, 10 minutos de reproducción continua suman
  600 s.
- TRA-22.2 Sin reproducción, una tecla y después 5 minutos sin actividad suman
  60 s.
- TRA-22.3 Con la pestaña oculta no se suma nada.
- TRA-22.4 Con 30 minutos de revisión y un audio de 20 minutos, el encabezado
  dice `1,5 min por minuto de audio`.

### TRA-23 Eliminar el audio tras la revisión [D-08]

**Comportamiento.** El consentimiento del autor promete borrar el audio apenas
se revisa la transcripción. `Eliminar audio` está en la vista de la entrevista
y pide escribir el identificador para confirmar. Con la transcripción
`revisada`, la app borra el original, pone en su ficha `estado = "eliminado"`,
`eliminado` y `motivo_eliminacion = "retencion"`, y anota `audio_eliminado` con
el identificador y la huella. La huella sigue en la ficha y en
`transcripcion.json`. El material de `origen/` y la transcripción no se tocan.
Si el consentimiento de esa persona tiene `respaldo = audio_entrevista`, el
diálogo agrega `El consentimiento oral de <S> está grabado en este audio y se
pierde con él.`

**Errores.** Sin la transcripción `revisada`, la acción está bloqueada con `La
transcripción de <id> no está revisada.` y la excepción
`eliminar_audio_sin_revisar`.

**Criterios de aceptación.**
- TRA-23.1 Con la transcripción revisada, eliminar el audio de `E01` deja la
  carpeta sin `E01.ogg`, la ficha con `estado = "eliminado"`, el evento en el
  registro y la etiqueta `Audio eliminado el <fecha>`.
- TRA-23.2 Si el identificador escrito no coincide, no se borra nada.
- TRA-23.3 Después de eliminar, la verificación al abrir no reporta el audio
  como faltante.
- TRA-23.4 `GET /audio/E01` responde 410 (SRV-11).

### TRA-24 Accesibilidad del editor

**Comportamiento.** El editor usa HTML semántico. La lista de segmentos es una
lista, cada texto tiene el nombre accesible `Segmento <id>, <m:ss>, <hablante>`,
cada botón tiene nombre, el foco es siempre visible, y los estados, las
sospechas, el segmento en reproducción, la zona y la sensibilidad se
distinguen por texto o ícono con nombre y nunca solo por el color.

**Criterios de aceptación.**
- TRA-24.1 Una prueba en Firefox y en Chromium que usa solo el teclado importa
  la entrevista de prueba, la abre, reproduce, retrocede, cambia la velocidad,
  asigna un hablante, edita, divide, une, salta al siguiente sospechoso, marca
  todos los segmentos y marca la transcripción revisada.
- TRA-24.2 Con los estilos de color anulados, cada estado y cada sospecha se
  distinguen por su texto.

## 4. Fuera de alcance de este tramo

- Transcribir dentro de la app, recomendación de modelo y glosario (v1.0).
- Importar SRT, TXT y DOCX (v1.0).
- Separación automática de hablantes (después). La app solo usa la que ya hizo
  la cadena del autor.
- Filtro para codificar o recuperar solo los turnos de la persona entrevistada
  (tramo 2, junto con la codificación).
- Exportar la transcripción revisada a Markdown o texto plano con seudónimos
  aplicados (tramo 2, ver D-13).
- Fotos y notas (ver D-14).
- Reporte de palabras corregidas por modelo y muestra de evaluación (v1.0).
- Más de un audio o más de una persona entrevistada por entrevista (ver D-09).
- Recorridos acompañados con registro de ubicación (después, ver D-06).
- Recalcular las marcas de sospecha después de importar.

## 5. Decisiones

### D-01 Formato canónico de las transcripciones

- **Pregunta.** ¿El formato canónico es WebVTT o un JSON documentado con
  hablantes y estado de revisión? (PRODUCTO.md, sección 13, antes del tramo 1).
- **Opciones.** (a) JSON documentado (`transcripcion.json`, TRA-11). (b)
  WebVTT con los estados y los hablantes en etiquetas y bloques `NOTE`. (c)
  JSON Lines, un segmento por línea.
- **Decisión (2026-10-11).** (a). WebVTT no tiene lugar estándar para el
  estado por segmento, el identificador estable, las métricas ni las marcas de
  sospecha, y meterlos en comentarios lo vuelve un formato propio disfrazado.
  JSON Lines facilita agregar al final, pero la revisión cambia segmentos en
  cualquier lugar y una escritura atómica del archivo entero de 1 a 2 MB por
  segundo es barata. WebVTT queda como formato de importación, y como
  exportación posible más adelante.
- **Dependen.** TRA-04, TRA-06, TRA-11, TRA-13, TRA-20.

### D-04 Unidad de codificación y unidad de los desplazamientos

- **Pregunta.** ¿La unidad de codificación es la selección libre, el turno de
  habla o el párrafo? (sección 13). En el tramo 1 solo afecta lo que hay que
  dejar preparado.
- **Opciones.** (a) Selección libre anclada por `id` de segmento y
  desplazamiento en caracteres Unicode sobre texto NFC. (b) Turno de habla
  completo, sin desplazamientos. (c) Párrafo.
- **Decisión (2026-10-11).** (a), que es lo que dice 6.7.1. Contar en
  caracteres Unicode sobre NFC da el mismo número en Rust, en R y en cualquier
  editor, a diferencia de bytes o de unidades UTF-16. Si se elige (b), los
  segmentos ya son la unidad y nada del tramo 1 cambia.
- **Dependen.** TRA-11, TRA-19, TRA-20.

### D-05 Corregir después de codificar y convención de transcripción

- **Pregunta.** ¿Se corrigen transcripciones después de empezar a codificar o
  se cierran antes? ¿La convención es literal, con pausas y muletillas, o
  limpia? (sección 13).
- **Opciones.** (a) Se pueden reabrir con `Reabrir para editar`, y el tramo 2
  avisa qué segmentos codificados toca. (b) Una transcripción revisada no se
  reabre nunca. Para la convención, (c) la app no la impone y el autor la anota
  en su bitácora, o (d) la app ofrece marcas fijas como `[inaudible]` o
  `[risas]` con atajos.
- **Decisión (2026-10-11).** (a) y (c). Reabrir con aviso es lo que pide
  6.7.1, y cerrar para siempre obligaría a rehacer la codificación ante un
  error de transcripción. Imponer una convención no le toca a la herramienta
  según el principio de pluralismo, y las marcas con atajo se pueden agregar
  cuando el autor las pida.
- **Dependen.** TRA-13, TRA-19.

### D-06 Cantidad y duración de las entrevistas y recorridos

- **Pregunta.** ¿Cuántas entrevistas se esperan y de qué duración? ¿Alguna será
  un recorrido acompañado con registro de ubicación? (sección 13).
- **Opciones.** (a) Diseñar para la escala máxima de la sección 7, 50
  entrevistas de 90 minutos. (b) Diseñar solo para las entrevistas cortas en la
  calle, de unos 20 a 30 minutos. (c) Incluir recorridos con ubicación.
- **Decisión (2026-10-11).** (a), sin recorridos. Los límites de 4 GiB por
  audio, 50 MB por archivo de transcripción y 20.000 segmentos dejan margen
  sobre 90 minutos, y la meta de 1 s se mide con 60 minutos como pide la
  sección 7. Los recorridos con traza GPS son de después.
- **Dependen.** TRA-02, TRA-05, TRA-15.

### D-08 Eliminar el audio tras la revisión

- **Pregunta.** El consentimiento del autor promete borrar el audio apenas se
  revisa la transcripción, y la app guarda los originales como inmutables.
  ¿Cómo se cumple esa promesa? (sección 13, retención).
- **Opciones.** (a) Acción `Eliminar audio` en el tramo 1, habilitada con la
  transcripción revisada y con excepción registrada si no lo está. (b) Borrar a
  mano fuera de la app, que la verificación al abrir vería como audio faltante
  sin poder distinguirlo de un accidente. (c) Conservar los audios y cambiar la
  promesa del consentimiento.
- **Decisión (2026-10-11).** (a). Cumple lo prometido a las personas, deja
  rastro con la huella y la fecha, y la herramienta distingue un borrado
  decidido de una pérdida. En discos SSD borrar no garantiza que el contenido
  sea irrecuperable, por eso el README insiste en el disco cifrado.
- **Dependen.** TRA-23, PRY-07, PRY-11, SRV-11.

### D-09 Una persona y un audio por entrevista

- **Pregunta.** ¿Una entrevista puede tener más de una persona entrevistada o
  más de un audio, o una persona más de una entrevista?
- **Opciones.** (a) En el tramo 1, una persona, un audio y una entrevista por
  participante, con el seudónimo como identificador. (b) Varias personas por
  entrevista, cada una con su consentimiento. (c) Varias entrevistas por
  persona, con identificadores `E01-2`.
- **Decisión (2026-10-11).** (a). Es como trabaja hoy el autor, con el mismo
  código para la persona y la entrevista, y mantiene simples los nombres de
  archivo y la regla de consentimiento. Una voz de un tercero en la grabación
  queda como `Otra persona`. Si una entrevista real lo pide, se amplía antes del
  tramo 2.
- **Dependen.** TRA-01, TRA-02, PAR-05, PAR-11.

### D-10 Uso del Markdown de la cadena al importar

- **Pregunta.** El `*.md` de la cadena puede tener correcciones a mano que los
  JSON no tienen. ¿Qué hace la importación con él?
- **Opciones.** (a) Usarlo como texto de los segmentos cuando se alinea
  exactamente con `hablantes.json`, y si no, detenerse y ofrecer importar sin
  él. (b) Guardarlo solo como material de origen. (c) Importarlo como fuente
  propia de segmentos, con la precisión de segundos enteros.
- **Decisión (2026-10-11).** (a). Conserva cualquier corrección ya hecha, como
  el nombre reemplazado por el seudónimo, sin perder la precisión de los
  tiempos del JSON, y la alineación exacta no admite errores silenciosos.
- **Dependen.** TRA-07, TRA-08.

### D-11 Atajos de teclado y clic para reproducir

- **Pregunta.** ¿Qué teclas usa el editor, y cómo se concilia «clic en el
  texto reproduce» con hacer clic para editar?
- **Opciones.** Para el clic, (a) clic en el tiempo o `Ctrl+clic` en el texto
  reproducen y el clic simple edita, o (b) el clic simple en el texto también
  reproduce. Para las teclas, (c) la tabla de TRA-18, o (d) teclas de función
  como en otros programas de transcripción.
- **Decisión (2026-10-11).** (a) y (c). Si cada clic para corregir una palabra
  moviera el audio, revisar sería incómodo. Las teclas de función chocan con el
  navegador o con el dictado del autor en `F9`. La tabla se valida en el equipo
  del autor con Firefox y Chromium antes de escribir los escenarios. En Windows,
  `Ctrl+Alt` equivale a `AltGr` y choca con caracteres del teclado español, así
  que la tabla se revisa para la v1.0.
- **Dependen.** TRA-16, TRA-17, TRA-18, TRA-24.

### D-13 Exportación de la transcripción en el tramo 2

- **Pregunta.** La exportación a Markdown o texto plano de 6.6 es v0.1, pero
  pide «seudónimos aplicados», que dependen de `osr seudonimizar`, y la sección
  9 dice que la v0.1 no tiene vías de salida. ¿Entra en el tramo 1?
- **Opciones.** (a) Pasa al tramo 2 y exporta solo la versión seudonimizada a
  la zona de trabajo. (b) Entra al tramo 1 exportando a la zona identificable
  con los hablantes como seudónimos pero el texto sin seudonimizar.
- **Decisión (2026-10-11).** (a). Una exportación de texto sin seudonimizar
  sería otra copia identificable, y la cadena actual del autor ya produce su
  Markdown mientras tanto.
- **Dependen.** PAR-15 y la sección 4 de este archivo.

### D-14 Fotos y notas fuera del tramo 1

- **Pregunta.** La fila de 6.5 junta audio, fotos y notas, pero el tramo 1 solo
  nombra la importación de audios. ¿Entran las fotos y las notas?
- **Opciones.** (a) Fuera del tramo 1, con la misma mecánica de originales y
  fichas cuando se pidan. (b) Dentro del tramo 1.
- **Decisión (2026-10-11).** (a). El tramo 1 ya es grande, las fotos piden
  decidir qué hacer con sus metadatos de ubicación, y las hojas de mapa
  escaneadas llegan en el tramo 3.
- **Dependen.** TRA-02.

### D-16 Dónde viven la fecha y el lugar de la entrevista

- **Pregunta.** Hoy el autor guarda en un mismo archivo el código, el nombre,
  la fecha y el lugar. ¿Dónde van la fecha y el lugar en la herramienta?
- **Opciones.** (a) En `entrevista.toml`, dentro de la zona identificable. (b)
  En la tabla de correspondencia, junto al nombre. (c) En la zona de trabajo,
  como atributos de caso.
- **Decisión (2026-10-11).** (a). Fecha y lugar son datos que pueden
  identificar a alguien en un barrio pequeño, así que no van a la zona de
  trabajo tal cual, y separarlos del nombre mantiene la tabla de
  correspondencia como la única llave entre seudónimo y persona. En el tramo 2,
  la tabla de casos puede llevar versiones generalizadas, como el barrio o la
  franja horaria.
- **Dependen.** TRA-01, PAR-03, PAR-12.
