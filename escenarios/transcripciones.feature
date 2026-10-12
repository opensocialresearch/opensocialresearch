# language: es
#
# Escenarios del módulo transcripciones (TRA), tramo 1 de la v0.1.
# Fuente especificaciones/transcripciones.md.
#
# Las convenciones, las etiquetas y los datos de prueba están al principio de proyecto.feature.
#
# Pasos compuestos de este archivo
# - «un proyecto nuevo con la entrevista "E01" sin audio» crea el proyecto con `osr nuevo`,
#   registra a E01, registra su consentimiento con grabar = otorgado y crea su entrevista.
# - «un proyecto nuevo con la entrevista "E01" y su audio de prueba importado» hace lo mismo
#   e importa además E01.ogg.
# - «se importa en la entrevista "E01" el archivo generado "<nombre>"» lo importa con el modelo
#   "prueba" y, si trae voces, asigna "A" a entrevistadora y "B" a entrevistada.
#
# Archivos de prueba que estos escenarios suponen, todos inventados
# - E01.ogg dura entre 60 y 120 s, E02.m4a 120 s o menos.
# - E01-sin-duracion.ogg, el audio de E01 con el contenedor alterado para que no declare su duración.
# - Cadena del autor por entrevista, E01.whisper.json, E01.hablantes.json, E01.md y E01.encabezado.md,
#   con el hablante 0 etiquetado "Entrevistador" y el 1 etiquetado "E01" en el Markdown.
# - E01.vtt (WebVTT) y E01.openai.json (JSON de Whisper con las tres métricas).
# - sospechas.openai.json, con estos segmentos
#   1 no_speech_prob 0.7 y avg_logprob -1.2, 2 no_speech_prob 0.7 y avg_logprob -0.5,
#   3 compression_ratio 2.4, 4 compression_ratio 2.41, 5 a 7 los textos "Sí, claro.", "sí claro" y "Sí, claro",
#   8 compression_ratio 2.5, y el resto sin marcas.
# - sin-compresion.openai.json, con avg_logprob y no_speech_prob y sin compression_ratio.
# - tres-hablantes.hablantes.json, con frases de los hablantes 0, 1 y 2.
# - editor.vtt, generado, con diez bloques y estas voces, tiempos y textos
#   s0001 A 0 a 2 "Buenas tardes, gracias por venir", s0002 B 2 a 5 "Con mucho gusto",
#   s0003 B 5 a 8 "Vivo en el barrio desde hace años", s0004 A 8 a 11 "¿Qué cambió en ese tiempo?",
#   s0005 B 11 a 14 "Muchas cosas", s0006 A 14 a 20 "Cuénteme más", s0007 B 20 a 30 "Hola mundo",
#   s0008 A 30 a 40 "Siga, por favor", s0009 A 40 a 50 "siga por favor", s0010 A 50 a 55 "Muchas gracias".
#   s0008 y s0009 forman la única serie repetida, así que son los únicos sospechosos.
# - Un archivo inválido por cada error de TRA-04 y TRA-05, nombrado en los Ejemplos de TRA-05.1.

Característica: Audios y transcripciones
  La app trae los audios y las transcripciones de la cadena del autor sin rehacer nada,
  los guarda en un formato canónico y ofrece un editor de revisión sincronizado con el
  audio y operable solo con teclado.

  @TRA-01
  Regla: Entrevistas

    @TRA-01.1 @TRA-01.3 @interfaz
    Escenario: Crear la entrevista guarda fecha y lugar solo en la zona identificable
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y la interfaz está abierta en el navegador
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando la interfaz muestra la vista del participante "E01"
      Y la persona pulsa "Nueva entrevista"
      Y la persona escribe "2026-10-09" en el campo "fecha"
      Y la persona escribe "Barrio Inventado" en el campo "lugar"
      Y la persona pulsa "Guardar"
      Entonces "identificable/entrevistas/E01/entrevista.toml" tiene estas claves con estos valores
        | clave           | valor              |
        | formato         | "osr-entrevista"   |
        | version_formato | 1                  |
        | id              | "E01"              |
        | participante    | "E01"              |
        | fecha           | 2026-10-09         |
        | lugar           | "Barrio Inventado" |
      Y la clave "creada" de "identificable/entrevistas/E01/entrevista.toml" es una marca de tiempo ISO 8601 con desplazamiento y precisión de segundos
      Y el registro de eventos tiene exactamente una línea nueva de tipo "entrevista_creada" cuyo detalle contiene "E01"
      Y el archivo "bitacora/registro.jsonl" no contiene el texto "Barrio Inventado"
      Y ningún archivo del proyecto fuera de "identificable/" contiene el texto "Barrio Inventado"

    @TRA-01.2 @interfaz @PRY-18.1
    Escenario: Un participante con entrevista no puede crear otra
      Dado un proyecto nuevo con la entrevista "E01" sin audio
      Y la interfaz está abierta en el navegador
      Cuando la interfaz muestra la vista del participante "E01"
      Entonces el botón "Nueva entrevista" está deshabilitado con el texto "E01 ya tiene una entrevista. En esta versión cada participante tiene una sola."

    @PRY-18.1 @interfaz
    Esquema del escenario: Errores del formulario de entrevista
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y la interfaz está abierta en el navegador
      Y se tomó la huella de cada archivo del proyecto
      Cuando la interfaz muestra la vista del participante "E01"
      Y la persona pulsa "Nueva entrevista"
      Y la persona escribe "<fecha>" en el campo "fecha"
      Y la persona escribe "<lugar>" en el campo "lugar"
      Y la persona pulsa "Guardar"
      Entonces la interfaz muestra "<mensaje>"
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas

      Ejemplos:
        | fecha      | lugar                            | mensaje                                                    |
        | {mañana}   | Barrio Inventado                 | La fecha de la entrevista no puede ser posterior a hoy.    |
        | 2026-10-09 | {201 veces la letra a}           | El lugar admite hasta 200 caracteres, sin saltos de línea. |
        | 2026-10-09 | {Barrio, salto de línea, Norte}  | El lugar admite hasta 200 caracteres, sin saltos de línea. |

  @TRA-02
  Regla: Importar audio

    @TRA-02.1
    Escenario: Importar el audio de E01 crea el original con su ficha
      Dado un proyecto nuevo con la entrevista "E01" sin audio
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando se importa en la entrevista "E01" una copia del audio de prueba "E01.ogg" llamada "grabación Carla Supuesta Vega.ogg"
      Entonces el archivo "identificable/entrevistas/E01/E01.ogg" es idéntico byte a byte a "E01.ogg" de los datos de prueba
      Y los permisos de "identificable/entrevistas/E01/E01.ogg" son 0400
      Y existe el archivo "identificable/entrevistas/E01/E01.ogg.ficha.toml"
      Y el registro de eventos tiene exactamente una línea nueva de tipo "audio_importado" cuyo detalle contiene "E01", "ogg" y la huella de "E01.ogg"
      Y el archivo "bitacora/registro.jsonl" no contiene el texto "Carla Supuesta Vega"

    @TRA-02.2 @interfaz @PRY-18.1
    Esquema del escenario: Un archivo cuyo contenido no corresponde a su extensión no se importa
      Dado un proyecto nuevo con la entrevista "E01" sin audio
      Y la interfaz está abierta en el navegador
      Y se tomó la huella de cada archivo del proyecto
      Cuando la persona importa el archivo generado "<archivo>", cuyo contenido empieza con "<comienzo>", con "Importar audio" en la entrevista "E01"
      Entonces la interfaz muestra "<archivo> dice ser <formato> pero su contenido no lo es."
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas

      Ejemplos:
        | archivo   | comienzo | formato |
        | falso.mp3 | OggS     | mp3     |
        | falso.wav | OggS     | wav     |
        | falso.m4a | RIFF     | m4a     |
        | falso.ogg | ID3      | ogg     |

    @TRA-02.3 @interfaz @PRY-18.1
    Escenario: El mismo audio en otra entrevista se rechaza como repetido
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Y el participante "E02" está registrado
      Y el participante "E02" tiene consentimiento registrado con grabar "otorgado"
      Y la entrevista de "E02" está creada
      Y la interfaz está abierta en el navegador
      Cuando la persona importa el archivo "E01.ogg" con "Importar audio" en la entrevista "E02"
      Entonces la interfaz muestra "Este audio ya está importado en la entrevista E01."
      Y no existe "identificable/entrevistas/E02/E02.ogg"

    @TRA-02.4 @interfaz
    Escenario: Cancelar a mitad de la transferencia no deja ningún archivo
      Dado un proyecto nuevo con la entrevista "E01" sin audio
      Y la interfaz está abierta en el navegador
      Y se tomó la huella de cada archivo del proyecto
      Cuando la persona empieza a importar el archivo generado "largo.wav" de 500 MB con "Importar audio" en la entrevista "E01"
      Y la persona cancela cuando se transfirió la mitad del archivo
      Entonces la carpeta "identificable/entrevistas/E01/" tiene solo el archivo "entrevista.toml"
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas

    @TRA-02.5 @interfaz @equipo-autor
    Esquema del escenario: Los audios de prueba se reproducen en los navegadores del autor
      # Depende de los códecs del sistema del autor, por eso se corre en su equipo.
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Cuando la persona reproduce el audio de la entrevista "<entrevista>" durante 3 s
      Entonces la posición del audio avanzó al menos 2,5 s
      Y el navegador no informó ningún error de reproducción

      Ejemplos:
        | entrevista |
        | E01        |
        | E02        |

    @PRY-18.1 @interfaz
    Esquema del escenario: Otros errores al importar audio
      Dado <preparación>
      Y la interfaz está abierta en el navegador
      Y se tomó la huella de cada archivo del proyecto
      Cuando la persona importa el archivo <archivo> con "Importar audio" en la entrevista "E01"
      Entonces la interfaz muestra "<mensaje>"
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas

      Ejemplos:
        | preparación                                                        | archivo                              | mensaje                                                                  |
        | un proyecto nuevo con la entrevista "E01" y su audio de prueba importado | "E02.m4a"                       | La entrevista E01 ya tiene audio.                                        |
        | un proyecto nuevo con la entrevista "E01" sin audio                | generado "notas.flac"                | notas.flac no es un audio admitido. Se aceptan m4a, mp3, wav, ogg y opus. |
        | un proyecto nuevo con la entrevista "E01" sin audio                | generado "enorme.wav" de 4 GiB más 1 byte | enorme.wav pasa de 4 GiB.                                           |

  @TRA-03
  Regla: Duración y estado visibles

    @TRA-03.1 @interfaz
    Esquema del escenario: La duración mostrada difiere 1 s o menos de la real
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Cuando la interfaz muestra la vista de la entrevista "<entrevista>"
      Entonces la duración que muestra la etiqueta del audio difiere en 1 s o menos de la duración real de "<archivo>"

      Ejemplos:
        | entrevista | archivo |
        | E01        | E01.ogg |
        | E02        | E02.m4a |

    @TRA-03.2 @interfaz
    Escenario: Un audio sin duración declarada se importa y lo dice
      Dado un proyecto nuevo con la entrevista "E01" sin audio
      Y la interfaz está abierta en el navegador
      Cuando la persona importa el archivo "E01-sin-duracion.ogg" con "Importar audio" en la entrevista "E01"
      Entonces existe el archivo "identificable/entrevistas/E01/E01.ogg"
      Y "identificable/entrevistas/E01/E01.ogg.ficha.toml" no tiene la clave "duracion_s"
      Y la entrevista "E01" muestra la etiqueta "Duración desconocida"

    @TRA-03.3 @interfaz
    Escenario: Las dos etiquetas muestran el audio y el avance de la revisión
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Y se importa en la entrevista "E01" el archivo generado "cuarenta.vtt" con 40 bloques de 1 s
      Y se marcan como revisados los segmentos "s0001" a "s0012" de "E01"
      Y la interfaz está abierta en el navegador
      Cuando la interfaz muestra la vista "Resumen del proyecto"
      Entonces la entrevista "E01" muestra las etiquetas "Audio {duración E01}" y "Transcripción en revisión (12 de 40 segmentos revisados)"

  @TRA-04
  Regla: Archivos que se reconocen

    @TRA-04.1
    Esquema del escenario: Los cuatro archivos de la cadena se reconocen por su contenido
      # La especificación no dice dónde se ve el tipo reconocido. Se observa en la lista
      # archivos de importacion.toml.
      Dado un proyecto nuevo con la entrevista "<entrevista>" y su audio de prueba importado
      Cuando se importan en la entrevista "<entrevista>" estos archivos de prueba con el modelo "large-v3-turbo"
        | archivo de prueba              | nombre al importar |
        | <entrevista>.whisper.json      | uno.json           |
        | <entrevista>.hablantes.json    | dos.json           |
        | <entrevista>.md                | tres.md            |
        | <entrevista>.encabezado.md     | cuatro.md          |
      Entonces la lista "archivos" de "identificable/entrevistas/<entrevista>/origen/{marca}/importacion.toml" tiene estos tipos por nombre original
        | nombre_original | tipo             |
        | uno.json        | cadena_whisper   |
        | dos.json        | cadena_hablantes |
        | tres.md         | cadena_markdown  |
        | cuatro.md       | encabezado       |

      Ejemplos:
        | entrevista |
        | E01        |
        | E02        |

    @TRA-04.2
    Escenario: Un whisper.json renombrado datos.txt se reconoce igual
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Cuando se importan en la entrevista "E01" estos archivos de prueba con el modelo "large-v3-turbo"
        | archivo de prueba | nombre al importar |
        | E01.whisper.json  | datos.txt          |
      Entonces la lista "archivos" de "identificable/entrevistas/E01/origen/{marca}/importacion.toml" tiene estos tipos por nombre original
        | nombre_original | tipo           |
        | datos.txt       | cadena_whisper |
      Y existe el archivo "identificable/entrevistas/E01/origen/{marca}/E01.whisper.json"

    @TRA-04.3 @interfaz @PRY-18.1
    Escenario: Una lista JSON sin words no se reconoce
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Y la interfaz está abierta en el navegador
      Y se tomó la huella de cada archivo del proyecto
      Cuando la persona importa con "Importar transcripción" en la entrevista "E01" el archivo generado "sin-words.json" con este contenido
        """
        [{"start": 0.0, "end": 2.0, "text": "Hola"}]
        """
      Entonces la interfaz muestra "No se reconoce el formato de sin-words.json."
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas

  @TRA-05
  Regla: Combinaciones y validación

    @TRA-05.1 @interfaz @PRY-18.1
    Esquema del escenario: Cada archivo inválido produce exactamente su mensaje y no escribe nada
      # La especificación no fija cómo se escriben el tipo esperado ni los tiempos dentro de
      # los mensajes. Se usan el nombre del tipo en español y los decimales con coma.
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Y la interfaz está abierta en el navegador
      Y se tomó la huella de cada archivo del proyecto
      Cuando la persona importa con "Importar transcripción" en la entrevista "E01" los archivos de prueba "<archivos>" con el modelo "large-v3-turbo"
      Entonces la interfaz muestra "<mensaje>"
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas

      Ejemplos:
        | archivos                                   | mensaje                                                                                                     |
        | desconocido.json                           | No se reconoce el formato de desconocido.json.                                                              |
        | latin1.vtt                                 | latin1.vtt no está en UTF-8.                                                                                |
        | roto.json                                  | roto.json no es JSON válido (línea 3, columna 15).                                                          |
        | E01.whisper.json, copia.whisper.json       | Hay dos archivos de tipo cadena_whisper.                                                                    |
        | E01.encabezado.md                          | Falta la transcripción. Se necesita un whisper.json, un hablantes.json, un JSON de Whisper o un WebVTT.     |
        | E01.whisper.json, E01.vtt                  | Hay más de una transcripción. Se usa una sola por importación.                                              |
        | E01.whisper.json, E01.md                   | El Markdown de la cadena solo se importa junto con su hablantes.json.                                       |
        | campo-faltante.hablantes.json              | campo-faltante.hablantes.json, segmento 2: falta end o no es número.                                        |
        | tiempos-invalidos.whisper.json             | tiempos-invalidos.whisper.json, segmento 4: los tiempos no son válidos (12,5 a 11,25).                      |
        | fuera-de-orden.whisper.json                | fuera-de-orden.whisper.json, segmento 3: empieza antes que el anterior.                                     |
        | vacio.whisper.json                         | vacio.whisper.json no tiene segmentos.                                                                      |
        | marca-mala.vtt                             | marca-mala.vtt, línea 5: marca de tiempo no válida.                                                         |
        | grande.whisper.json                        | grande.whisper.json pasa el límite de 50 MB.                                                                |
        | muchos.whisper.json                        | muchos.whisper.json pasa el límite de 20.000 segmentos.                                                     |

    @TRA-05.1 @interfaz @PRY-18.1
    Escenario: Sin audio presente la transcripción no se importa
      Dado un proyecto nuevo con la entrevista "E01" sin audio
      Y la interfaz está abierta en el navegador
      Y se tomó la huella de cada archivo del proyecto
      Cuando la persona importa con "Importar transcripción" en la entrevista "E01" los archivos de prueba "E01.hablantes.json" con el modelo "large-v3-turbo"
      Entonces la interfaz muestra "Primero hay que importar el audio de la entrevista."
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas

    @TRA-05.2 @interfaz
    Escenario: Un whisper.json y un WebVTT juntos son más de una transcripción
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Y la interfaz está abierta en el navegador
      Cuando la persona importa con "Importar transcripción" en la entrevista "E01" los archivos de prueba "E01.whisper.json, E01.vtt" con el modelo "large-v3-turbo"
      Entonces la interfaz muestra "Hay más de una transcripción. Se usa una sola por importación."

    @TRA-05.3 @interfaz
    Escenario: Si el audio de la misma importación se bloquea no se importa nada
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y el participante "E01" tiene consentimiento registrado con grabar "denegado"
      Y la entrevista de "E01" está creada
      Y la interfaz está abierta en el navegador
      Y se tomó la huella de cada archivo del proyecto
      Cuando la persona importa con "Importar transcripción" en la entrevista "E01" los archivos de prueba "E01.ogg, E01.hablantes.json" con el modelo "large-v3-turbo"
      Entonces la interfaz muestra "El consentimiento de E01 no autoriza grabar. No se puede importar el audio."
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas

    @TRA-05.4 @interfaz @PRY-18.1
    Escenario: Una transcripción 5 s más larga que el audio se bloquea con excepción
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Y la interfaz está abierta en el navegador
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando la persona importa con "Importar transcripción" en la entrevista "E01" los archivos de prueba "E01-largo.vtt" con el modelo "large-v3-turbo"
      Entonces la interfaz muestra "La transcripción termina en {fin de E01-largo.vtt en m:ss} y el audio dura {duración E01}. Puede que no sea el audio correcto."
      Y el diálogo muestra los botones "Cancelar" y "Hacerlo igual"
      Cuando la persona pulsa "Hacerlo igual"
      Entonces existe el archivo "identificable/entrevistas/E01/transcripcion.json"
      Y el registro de eventos tiene exactamente una línea nueva de tipo "excepcion_confirmada" cuyo detalle contiene "transcripcion_mas_larga_que_audio"
      Y la lista "excepciones" de "identificable/entrevistas/E01/origen/{marca}/importacion.toml" es exactamente ["transcripcion_mas_larga_que_audio"]

  @TRA-06
  Regla: Construcción de los segmentos

    @TRA-06.1
    Escenario: Un hablantes.json sin Markdown da un segmento por frase
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Cuando se importan en la entrevista "E01" los archivos de prueba "E01.hablantes.json" con el modelo "large-v3-turbo"
      Entonces "identificable/entrevistas/E01/transcripcion.json" tiene tantos segmentos como frases tiene "E01.hablantes.json"
      Y cada segmento i tiene inicio y fin iguales a start y end de la frase i redondeados al milisegundo
      Y cada segmento i tiene como texto el text de la frase i sin espacios en los extremos
      Y los segmentos tienen los id "s0001", "s0002" y así en orden
      Y cada segmento tiene estado "sin_revisar" y revisado_en null

    @TRA-06.2
    Escenario: Un bloque WebVTT con voz y entidades da su texto y su hablante de origen
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Y el archivo generado "bloque.vtt" con este contenido
        """
        WEBVTT

        00:00:01.000 --> 00:00:03.000
        <v Persona A>Hola &amp;
        chau</v>
        """
      Cuando se importan en la entrevista "E01" los archivos generados "bloque.vtt" con el modelo "prueba"
      Entonces el segmento "s0001" de "identificable/entrevistas/E01/transcripcion.json" tiene el texto "Hola & chau"
      Y el segmento "s0001" de "identificable/entrevistas/E01/transcripcion.json" tiene hablante_origen "Persona A"

    @TRA-06.3
    Escenario: Una métrica que falta queda en null
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Cuando se importan en la entrevista "E01" los archivos de prueba "sin-compresion.openai.json" con el modelo "large-v3-turbo"
      Entonces cada segmento de "identificable/entrevistas/E01/transcripcion.json" tiene metricas con las claves "avg_logprob", "no_speech_prob" y "compression_ratio"
      Y cada segmento de "identificable/entrevistas/E01/transcripcion.json" tiene metricas.compression_ratio null

    @TRA-06.4
    Escenario: Un segmento con solo espacios queda con texto vacío
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Y el archivo generado "espacios.whisper.json" con este contenido
        """
        [{"start": 0.0, "end": 1.0, "text": "   ", "words": [{"s": 0.0, "e": 1.0, "w": "   "}]}]
        """
      Cuando se importan en la entrevista "E01" los archivos generados "espacios.whisper.json" con el modelo "prueba"
      Entonces el segmento "s0001" de "identificable/entrevistas/E01/transcripcion.json" tiene el texto ""

  @TRA-07
  Regla: Markdown de la cadena

    @TRA-07.1
    Escenario: El Markdown de la cadena queda alineado
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Cuando se importan en la entrevista "E01" los archivos de la cadena de prueba de E01 con el modelo "large-v3-turbo"
      Entonces la clave "markdown_alineado" de "identificable/entrevistas/E01/origen/{marca}/importacion.toml" vale "si"
      Y cada segmento i de "identificable/entrevistas/E01/transcripcion.json" tiene el texto de la intervención i de "E01.md" sin espacios en los extremos

    @TRA-07.1
    Escenario: Una corrección a mano en el Markdown llega al segmento
      # Se lee «la línea 3» como la tercera intervención del Markdown, que es la que
      # corresponde al segmento 3.
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Y una copia de "E01.md" en la que el nombre inventado de la intervención 3 se cambió por "[E01]"
      Cuando se importan en la entrevista "E01" los archivos "E01.hablantes.json", "E01.whisper.json", "E01.encabezado.md" y esa copia de "E01.md" con el modelo "large-v3-turbo"
      Entonces la clave "markdown_alineado" de "identificable/entrevistas/E01/origen/{marca}/importacion.toml" vale "si"
      Y el segmento "s0003" de "identificable/entrevistas/E01/transcripcion.json" tiene un texto que contiene "[E01]"

    @TRA-07.2 @interfaz @PRY-18.1
    Escenario: Un Markdown con una intervención borrada se puede importar sin él
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Y una copia de "E01.md" sin su última intervención
      Y la interfaz está abierta en el navegador
      Cuando la persona importa con "Importar transcripción" en la entrevista "E01" los archivos "E01.hablantes.json" y esa copia de "E01.md" con el modelo "large-v3-turbo"
      Entonces la interfaz muestra "El Markdown tiene {frases de E01.hablantes.json menos 1} intervenciones y hablantes.json tiene {frases de E01.hablantes.json} frases."
      Y el diálogo muestra los botones "Cancelar" y "Importar sin el Markdown"
      Cuando la persona pulsa "Importar sin el Markdown"
      Y la persona confirma la asignación de hablantes propuesta
      Entonces cada segmento i de "identificable/entrevistas/E01/transcripcion.json" tiene como texto el text de la frase i de "E01.hablantes.json" sin espacios en los extremos
      Y la clave "markdown_alineado" de "identificable/entrevistas/E01/origen/{marca}/importacion.toml" vale "no"
      Y existe el archivo "identificable/entrevistas/E01/origen/{marca}/E01.md"

    @TRA-07.3
    Escenario: Los minutos del Markdown tienen al menos dos cifras
      Dado un proyecto nuevo con la entrevista "E01" sin audio
      Y se importa en la entrevista "E01" el audio de prueba "E01-sin-duracion.ogg"
      Y el archivo generado "largo.hablantes.json" con este contenido
        """
        {"turnos": [[600.4, 601.0, 0], [6000.0, 6001.0, 1]],
         "frases": [{"sp": 0, "start": 600.4, "end": 601.0, "text": "Primera"},
                    {"sp": 1, "start": 6000.0, "end": 6001.0, "text": "Segunda"}]}
        """
      Y el archivo generado "largo.md" con este contenido
        """
        # Entrevista E01

        **[10:00] Entrevistador:** Primera

        **[100:00] E01:** Segunda
        """
      Cuando se importan en la entrevista "E01" los archivos generados "largo.hablantes.json" y "largo.md" con el modelo "prueba"
      Entonces la clave "markdown_alineado" de "identificable/entrevistas/E01/origen/{marca}/importacion.toml" vale "si"

    @PRY-18.1 @interfaz
    Esquema del escenario: Otros desajustes del Markdown detienen la importación
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Y <copia>
      Y la interfaz está abierta en el navegador
      Cuando la persona importa con "Importar transcripción" en la entrevista "E01" los archivos "E01.hablantes.json" y esa copia de "E01.md" con el modelo "large-v3-turbo"
      Entonces la interfaz muestra "<mensaje>"

      Ejemplos:
        | copia                                                                         | mensaje                                                                                                           |
        | una copia de "E01.md" con el tiempo de la intervención 2 cambiado a "[59:59]" | El Markdown no coincide con hablantes.json en la intervención 2 (Markdown [59:59], hablantes.json [{mm:ss de la frase 2 de E01}]). |
        | una copia de "E01.md" con la línea "texto suelto" agregada después de la intervención 1, en la línea {línea agregada} | El Markdown tiene texto fuera de las intervenciones en la línea {línea agregada}.                  |

  @TRA-08
  Regla: Asignación de hablantes al importar

    @TRA-08.1 @interfaz
    Escenario: Con el Markdown alineado se proponen los roles por su etiqueta
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Y la interfaz está abierta en el navegador
      Cuando la persona importa con "Importar transcripción" en la entrevista "E01" los archivos de prueba "E01.whisper.json, E01.hablantes.json, E01.md, E01.encabezado.md" con el modelo "large-v3-turbo"
      Entonces el paso de asignación muestra una fila para el hablante "0" con el rol propuesto "Persona entrevistadora"
      Y el paso de asignación muestra una fila para el hablante "1" con el rol propuesto "Persona entrevistada"

    @TRA-08.2 @interfaz
    Escenario: Dos hablantes de origen pueden ir al mismo rol
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Y la interfaz está abierta en el navegador
      Cuando la persona importa con "Importar transcripción" en la entrevista "E01" los archivos de prueba "tres-hablantes.hablantes.json" con el modelo "large-v3-turbo"
      Y la persona elige en el paso de asignación "Persona entrevistadora" para "0" y para "2" y "Persona entrevistada" para "1"
      Y la persona confirma la importación
      Entonces cada segmento de "identificable/entrevistas/E01/transcripcion.json" con hablante_origen "0" o "2" tiene hablante "entrevistadora"

    @TRA-08.3 @interfaz
    Escenario: Un WebVTT sin voces no tiene paso de asignación
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Y el archivo generado "sin-voces.vtt" con este contenido
        """
        WEBVTT

        00:00:01.000 --> 00:00:03.000
        Primera línea

        00:00:03.000 --> 00:00:05.000
        Segunda línea
        """
      Y la interfaz está abierta en el navegador
      Cuando la persona importa con "Importar transcripción" en la entrevista "E01" los archivos generados "sin-voces.vtt" con el modelo "prueba"
      Entonces no apareció el paso de asignación de hablantes
      Y cada segmento de "identificable/entrevistas/E01/transcripcion.json" tiene hablante null

  @TRA-09
  Regla: Modelo declarado

    @TRA-09.1 @interfaz @PRY-18.1
    Escenario: Sin modelo la importación no avanza
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Y la interfaz está abierta en el navegador
      Y se tomó la huella de cada archivo del proyecto
      Cuando la persona importa con "Importar transcripción" en la entrevista "E01" los archivos de prueba "E01.vtt" sin escribir el modelo ni marcar "No se sabe"
      Entonces la interfaz muestra "Falta el modelo con que se generó la transcripción."
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas

    @TRA-09.2 @interfaz
    Esquema del escenario: El editor muestra el modelo declarado
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Y la interfaz está abierta en el navegador
      Cuando la persona importa con "Importar transcripción" en la entrevista "E01" los archivos de prueba "E01.vtt" con <modelo>
      Y la transcripción de E01 está abierta en el editor
      Entonces la interfaz muestra "<texto>"
      Y la clave "modelo" de "identificable/entrevistas/E01/transcripcion.json" vale "<valor>"

      Ejemplos:
        | modelo                     | texto                   | valor          |
        | el modelo "large-v3-turbo" | Modelo: large-v3-turbo  | large-v3-turbo |
        | la casilla "No se sabe"    | Modelo: desconocido     | desconocido    |

  @TRA-10
  Regla: Conservación del material de origen

    @TRA-10.1
    Escenario: La importación de la cadena deja cinco archivos de solo lectura en origen
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Cuando se importan en la entrevista "E01" los archivos de la cadena de prueba de E01 con el modelo "large-v3-turbo"
      Entonces la carpeta "identificable/entrevistas/E01/origen/{marca}/" tiene exactamente estas rutas
        | ruta               |
        | E01.whisper.json   |
        | E01.hablantes.json |
        | E01.md             |
        | E01.encabezado.md  |
        | importacion.toml   |
      Y cada uno de los cuatro primeros es idéntico byte a byte a su archivo de la cadena de prueba de E01
      Y cada archivo bajo "identificable/entrevistas/E01/origen/" tiene permisos 0400

    @TRA-10.2 @interfaz
    Escenario: Editar segmentos no cambia el material de origen
      Dado una copia del proyecto de prueba
      Y se tomó la huella de cada archivo de "identificable/entrevistas/E01/origen/"
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y la transcripción de E01 está abierta en el editor
      Cuando la persona escribe "x" al final del texto de cada uno de los primeros 20 segmentos
      Y la interfaz muestra "Guardado"
      Entonces ningún archivo de "identificable/entrevistas/E01/origen/" cambió respecto de las huellas tomadas

    @TRA-10.3
    Escenario: El nombre original de un archivo de origen queda solo en importacion.toml
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Cuando se importan en la entrevista "E01" estos archivos de prueba con el modelo "large-v3-turbo"
        | archivo de prueba  | nombre al importar                  |
        | E01.hablantes.json | Carla Supuesta Vega.hablantes.json  |
      Entonces el archivo "identificable/entrevistas/E01/origen/{marca}/importacion.toml" contiene el texto "Carla Supuesta Vega"
      Y ningún archivo del proyecto fuera de "identificable/" contiene el texto "Carla Supuesta Vega"
      Y ningún archivo de ".osr/" contiene el texto "Carla Supuesta Vega"
      Y el archivo "bitacora/registro.jsonl" no contiene el texto "Carla Supuesta Vega"
      Y el registro técnico no contiene el texto "Carla Supuesta Vega"

  @TRA-11
  Regla: Formato canónico de la transcripción

    @TRA-11.1
    Esquema del escenario: La transcripción importada cumple el esquema de TRA-11
      Dado una copia del proyecto de prueba
      Entonces "identificable/entrevistas/<entrevista>/transcripcion.json" es válido según el esquema JSON derivado de las tablas de TRA-11
      Y las claves de "identificable/entrevistas/<entrevista>/transcripcion.json" y de cada segmento están en el orden de las tablas de TRA-11
      Y los permisos de "identificable/entrevistas/<entrevista>/transcripcion.json" son 0600

      Ejemplos:
        | entrevista |
        | E01        |
        | E02        |

    @TRA-11.2
    Escenario: Los mismos archivos con la misma hora dan el mismo transcripcion.json
      Dado el reloj de la app está fijo en "2026-10-11T21:30:05-06:00"
      Y dos proyectos nuevos, cada uno con la entrevista "E01" y su audio de prueba importado
      Cuando se importan en la entrevista "E01" de cada proyecto los archivos de la cadena de prueba de E01 con el modelo "large-v3-turbo"
      Entonces los dos "identificable/entrevistas/E01/transcripcion.json" son idénticos byte a byte

    @TRA-11.3 @interfaz
    Escenario: Un inicio editado a mano fuera de orden marca el archivo con error
      Dado una copia del proyecto de prueba
      Y con la app cerrada se cambia en "identificable/entrevistas/E01/transcripcion.json" el inicio del segmento "s0003" a un valor menor que el inicio de "s0002"
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Entonces el evento "verificacion" más reciente tiene "identificable/entrevistas/E01/transcripcion.json" en la lista "con_error"
      Cuando la persona pide abrir la transcripción de E01 en el editor
      Entonces el editor no se abre
      Y el resumen muestra un aviso con la ruta "identificable/entrevistas/E01/transcripcion.json"

  @TRA-12
  Regla: Reimportar

    @TRA-12.1 @interfaz @PRY-18.1
    Escenario: Reimportar sobre una transcripción sin cambios la reemplaza
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y el reloj de la app avanza 2 s
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando la persona importa con "Importar transcripción" en la entrevista "E01" los archivos de prueba "E01.vtt" con el modelo "large-v3-turbo"
      Entonces la interfaz muestra "La transcripción actual de E01 no tiene cambios y se va a reemplazar."
      Y el diálogo muestra los botones "Cancelar" y "Reemplazar"
      Cuando la persona pulsa "Reemplazar"
      Entonces la carpeta "identificable/entrevistas/E01/origen/" tiene 2 carpetas
      Y la clave "importacion" de "identificable/entrevistas/E01/transcripcion.json" vale "origen/{marca}" de la importación nueva
      Y el registro de eventos tiene exactamente una línea nueva de tipo "transcripcion_reemplazada" cuyo detalle contiene "E01"

    @TRA-12.2 @interfaz @PRY-18.1
    Escenario: Con cambios de revisión, Cancelar no cambia nada
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y la transcripción de E01 está abierta en el editor
      Y la persona escribe "x" al final del texto del segmento "s0001"
      Y la interfaz muestra "Guardado"
      Y se tomó la huella de cada archivo del proyecto
      Cuando la persona importa con "Importar transcripción" en la entrevista "E01" los archivos de prueba "E01.vtt" con el modelo "large-v3-turbo"
      Entonces la interfaz muestra "La transcripción de E01 tiene cambios de revisión."
      Cuando la persona pulsa "Cancelar"
      Entonces ningún archivo del proyecto cambió respecto de las huellas tomadas

    @TRA-12.3 @interfaz
    Escenario: Con la excepción confirmada la transcripción anterior queda guardada aparte
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y la transcripción de E01 está abierta en el editor
      Y la persona escribe "x" al final del texto del segmento "s0001"
      Y la interfaz muestra "Guardado"
      Y se guardó una copia de "identificable/entrevistas/E01/transcripcion.json" como versión anterior
      Y el reloj de la app avanza 2 s
      Cuando la persona importa con "Importar transcripción" en la entrevista "E01" los archivos de prueba "E01.vtt" con el modelo "large-v3-turbo"
      Y la persona pulsa "Hacerlo igual"
      Entonces el archivo "identificable/entrevistas/E01/transcripcion.anterior-{marca}.json" es idéntico byte a byte a la versión anterior
      Y los permisos de "identificable/entrevistas/E01/transcripcion.anterior-{marca}.json" son 0400
      Y la carpeta "identificable/entrevistas/E01/origen/" tiene 2 carpetas

  @TRA-13
  Regla: Estados de revisión

    @TRA-13.1 @interfaz
    Escenario: Una transcripción recién importada está sin revisar
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Cuando la transcripción de E01 está abierta en el editor
      Entonces la interfaz muestra "Transcripción sin revisar"
      Y cada segmento del editor muestra la etiqueta "Sin revisar"

    @TRA-13.2 @interfaz
    Escenario: Editar una letra pasa la transcripción a en revisión
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y la transcripción de E01 está abierta en el editor
      Cuando la persona escribe "x" al final del texto del segmento "s0001"
      Y la interfaz muestra "Guardado"
      Entonces la clave "estado" de "identificable/entrevistas/E01/transcripcion.json" vale "en_revision"
      Y la clave "modificada" de "identificable/entrevistas/E01/transcripcion.json" vale true

    @TRA-13.3 @interfaz @PRY-18.1
    Escenario: Marcar revisada con segmentos sin revisar pide excepción y la guarda
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y la transcripción de E01 está abierta en el editor
      Y todos los segmentos de E01 salvo los 3 últimos están marcados como revisados
      Cuando la persona pulsa "Marcar transcripción como revisada"
      Entonces la interfaz muestra "Quedan 3 segmentos sin revisar."
      Cuando la persona pulsa "Hacerlo igual"
      Entonces la clave "estado" de "identificable/entrevistas/E01/transcripcion.json" vale "revisada"
      Y la clave "sin_revisar_al_cerrar" de "identificable/entrevistas/E01/transcripcion.json" vale 3

    @TRA-13.4 @interfaz
    Escenario: Una transcripción revisada no se edita
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la transcripción de E01 está revisada
      Y la interfaz está abierta en el navegador
      Y la transcripción de E01 está abierta en el editor
      Y se tomó la huella de "identificable/entrevistas/E01/transcripcion.json"
      Cuando la persona escribe "x" al final del texto del segmento "s0001"
      Y el foco está en el texto del segmento "s0002"
      Y la persona pulsa la tecla "Ctrl+Enter"
      Entonces el texto del segmento "s0001" en la interfaz no cambió
      Y la etiqueta de estado del segmento "s0002" no cambió
      Y la huella de "identificable/entrevistas/E01/transcripcion.json" es la misma que se tomó

    @TRA-13.5 @interfaz @PRY-18.1
    Escenario: Reabrir para editar vuelve a permitir la edición
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la transcripción de E01 está revisada
      Y la interfaz está abierta en el navegador
      Y la transcripción de E01 está abierta en el editor
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando la persona pulsa "Reabrir para editar"
      Entonces la interfaz muestra "La transcripción vuelve a estar en revisión."
      Cuando la persona confirma
      Y la persona escribe "x" al final del texto del segmento "s0001"
      Y la interfaz muestra "Guardado"
      Entonces el segmento "s0001" de "identificable/entrevistas/E01/transcripcion.json" tiene un texto que termina en "x"
      Y la clave "estado" de "identificable/entrevistas/E01/transcripcion.json" vale "en_revision"
      Y la clave "revisada_en" de "identificable/entrevistas/E01/transcripcion.json" vale null
      Y el registro de eventos tiene exactamente una línea nueva de tipo "transcripcion_reabierta" cuyo detalle contiene "E01"

  @TRA-14
  Regla: Segmentos sospechosos

    @TRA-14.1
    Escenario: La regla sin_voz necesita las dos condiciones
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Cuando se importan en la entrevista "E01" los archivos de prueba "sospechas.openai.json" con el modelo "large-v3-turbo"
      Entonces el segmento "s0001" de "identificable/entrevistas/E01/transcripcion.json" tiene en sospecha "sin_voz"
      Y el segmento "s0002" de "identificable/entrevistas/E01/transcripcion.json" no tiene en sospecha "sin_voz"

    @TRA-14.2
    Escenario: La regla de compresión exige pasar estrictamente el umbral
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Cuando se importan en la entrevista "E01" los archivos de prueba "sospechas.openai.json" con el modelo "large-v3-turbo"
      Entonces el segmento "s0003" de "identificable/entrevistas/E01/transcripcion.json" no tiene en sospecha "compresion"
      Y el segmento "s0004" de "identificable/entrevistas/E01/transcripcion.json" tiene en sospecha "compresion"

    @TRA-14.3
    Escenario: Tres textos iguales tras normalizar se marcan los tres
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Cuando se importan en la entrevista "E01" los archivos de prueba "sospechas.openai.json" con el modelo "large-v3-turbo"
      Entonces los segmentos "s0005", "s0006" y "s0007" de "identificable/entrevistas/E01/transcripcion.json" tienen en sospecha "repeticion"

    @TRA-14.4 @interfaz
    Escenario: Con la cadena del autor se omiten sin_voz y compresion con un aviso
      # La especificación no fija cómo se enumeran los textos y los campos dentro del aviso,
      # así que se comprueba el comienzo y que estén todos.
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Cuando la transcripción de E01 está abierta en el editor
      Entonces la lista "reglas_sospecha.omitidas" de "identificable/entrevistas/E01/transcripcion.json" es exactamente ["sin_voz", "compresion"]
      Y la interfaz muestra un texto que empieza con "No se aplicaron las reglas de " y contiene "posible texto en un silencio", "posible texto repetitivo", "no_speech_prob", "avg_logprob" y "compression_ratio"

    @TRA-14.5 @interfaz @PRY-18.1
    Escenario: El atajo recorre los sospechosos sin revisar y avisa cuando no quedan
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Y se importa en la entrevista "E01" el archivo generado "sospechosos-5-y-20.openai.json" con 25 segmentos y compression_ratio 2.5 solo en los segmentos 5 y 20
      Y la interfaz está abierta en el navegador
      Y la transcripción de E01 está abierta en el editor
      Y el foco está en el texto del segmento "s0010"
      Cuando la persona pulsa la tecla "Ctrl+Alt+S"
      Entonces el foco está en el texto del segmento "s0020"
      Cuando la persona pulsa la tecla "Ctrl+Alt+S"
      Entonces el foco está en el texto del segmento "s0005"
      Cuando se marcan como revisados los segmentos "s0005" y "s0020" de "E01"
      Y la persona pulsa la tecla "Ctrl+Alt+S"
      Entonces la interfaz muestra "No quedan segmentos sospechosos sin revisar."

    @TRA-14.6
    Escenario: Un umbral nuevo en proyecto.toml vale para la importación siguiente
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Y osr se cierra de forma ordenada
      Y con la app cerrada se cambia en "proyecto.toml" la clave "transcripcion.sospecha.compression_ratio" a 3.0
      Y osr está corriendo con el proyecto abierto
      Cuando se importan en la entrevista "E01" los archivos de prueba "sospechas.openai.json" con el modelo "large-v3-turbo"
      Entonces el segmento "s0008" de "identificable/entrevistas/E01/transcripcion.json" no tiene en sospecha "compresion"

  @TRA-15
  Regla: Abrir y mostrar una transcripción

    @TRA-15.1 @interfaz @equipo-autor
    Escenario: Una transcripción de 60 minutos abre en 1 s o menos
      # La meta vale en el equipo del autor, por eso se corre ahí.
      Dado un proyecto nuevo con la entrevista "E01" sin audio
      Y se importa en la entrevista "E01" el audio generado de 60 minutos para la medición
      Y se importa en la entrevista "E01" la transcripción generada de 60 minutos para la medición, con unas 10.000 palabras y 1.000 segmentos
      Y la interfaz está abierta en el navegador
      Cuando se mide 5 veces el tiempo desde pedir abrir la transcripción de E01 hasta que su primer segmento es editable
      Entonces cada una de las 5 repeticiones da 1 s o menos

    @TRA-15.2 @interfaz
    Esquema del escenario: La etiqueta del hablante depende del rol
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y el segmento "s0001" de E01 tiene hablante <rol>
      Y la interfaz está abierta en el navegador
      Cuando la transcripción de E01 está abierta en el editor
      Entonces el segmento "s0001" muestra la etiqueta de hablante "<etiqueta>"

      Ejemplos:
        | rol              | etiqueta               |
        | "entrevistada"   | E01                    |
        | null             | Sin asignar            |
        | "entrevistadora" | Persona entrevistadora |
        | "otra"           | Otra persona           |

    @TRA-15.3 @interfaz
    Escenario: Un inicio después de una hora se muestra como h:mm:ss
      Dado un proyecto nuevo con la entrevista "E01" sin audio
      Y se importa en la entrevista "E01" el audio de prueba "E01-sin-duracion.ogg"
      Y el archivo generado "hora.vtt" con este contenido
        """
        WEBVTT

        01:02:05.800 --> 01:02:08.000
        Texto después de una hora
        """
      Y se importan en la entrevista "E01" los archivos generados "hora.vtt" con el modelo "prueba"
      Y la interfaz está abierta en el navegador
      Cuando la transcripción de E01 está abierta en el editor
      Entonces el botón de tiempo del segmento "s0001" muestra "1:02:05"

  @TRA-16
  Regla: Reproducción sincronizada

    @TRA-16.1 @interfaz
    Escenario: Veinte saltos al azar llegan rápido y cerca del inicio
      Dado un proyecto nuevo con la entrevista "E01" sin audio
      Y se importa en la entrevista "E01" el audio generado de 60 minutos para la medición
      Y se importa en la entrevista "E01" la transcripción generada de 60 minutos para la medición, con unas 10.000 palabras y 1.000 segmentos
      Y la interfaz está abierta en el navegador
      Y la transcripción de E01 está abierta en el editor
      Cuando la persona pulsa el botón de tiempo de 20 segmentos elegidos al azar, uno por vez
      Entonces cada salto tarda 200 ms o menos desde la pulsación hasta la reproducción en la nueva posición
      Y en cada salto la posición del audio queda a 0,1 s o menos del inicio del segmento

    @TRA-16.2 @interfaz
    Escenario: Un clic simple en el texto no mueve el audio
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y la transcripción de E01 está abierta en el editor
      Y la posición del audio es 10,0 s en pausa
      Cuando la persona hace un clic simple en el texto del segmento "s0001"
      Entonces la posición del audio es 10,0 s con 0,1 s de tolerancia
      Y el cursor de edición está en el texto del segmento "s0001"

    @TRA-16.3 @interfaz
    Escenario: El segmento en reproducción es el de mayor inicio que contiene la posición
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Y el archivo generado "solapados.vtt" con este contenido
        """
        WEBVTT

        00:00:10.000 --> 00:00:14.000
        Primero

        00:00:11.500 --> 00:00:13.000
        Segundo
        """
      Y se importan en la entrevista "E01" los archivos generados "solapados.vtt" con el modelo "prueba"
      Y la interfaz está abierta en el navegador
      Y la transcripción de E01 está abierta en el editor
      Cuando la posición del audio es 12,0 s
      Entonces a más tardar 250 ms después el segmento "s0002" muestra el ícono "▶" y el texto accesible "En reproducción"
      Y el segmento "s0001" no muestra el texto accesible "En reproducción"

    @TRA-16.4 @interfaz
    Escenario: Con el audio eliminado se edita igual y no se reproduce
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y el audio de E01 está eliminado con la excepción "eliminar_audio_sin_revisar"
      Y la interfaz está abierta en el navegador
      Y la transcripción de E01 está abierta en el editor
      Entonces los controles de reproducción están deshabilitados con la etiqueta "Audio eliminado el {hoy}"
      Cuando la persona escribe "x" al final del texto del segmento "s0001"
      Y la interfaz muestra "Guardado"
      Entonces el segmento "s0001" de "identificable/entrevistas/E01/transcripcion.json" tiene un texto que termina en "x"

  @TRA-17
  Regla: Velocidad, pausa y retroceso

    @TRA-17.1 @interfaz
    Esquema del escenario: Retroceder 3 s sin bajar de cero
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y la transcripción de E01 está abierta en el editor
      Y la posición del audio es <antes> s
      Cuando la persona pulsa la tecla "Ctrl+Alt+J"
      Entonces la posición del audio es <después> s con 0,1 s de tolerancia

      Ejemplos:
        | antes | después |
        | 10,0  | 7,0     |
        | 2,0   | 0       |

    @TRA-17.2 @interfaz
    Esquema del escenario: La velocidad sube y baja por pasos sin pasar de los extremos
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y la transcripción de E01 está abierta en el editor
      Y la velocidad es <inicial>
      Cuando la persona pulsa <veces> veces la tecla "<atajo>"
      Entonces la velocidad es <final>

      Ejemplos:
        | inicial | veces | atajo      | final |
        | 1       | 3     | Ctrl+Alt+. | 1,75  |
        | 2       | 1     | Ctrl+Alt+. | 2     |
        | 1       | 1     | Ctrl+Alt+, | 0,75  |
        | 0,5     | 1     | Ctrl+Alt+, | 0,5   |

    @TRA-17.3 @interfaz
    Escenario: Esc pausa y reanuda desde la misma posición
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y la transcripción de E01 está abierta en el editor
      Y el audio se está reproduciendo
      Cuando la persona pulsa la tecla "Esc"
      Entonces el audio está en pausa en una posición que se anota
      Cuando la persona pulsa la tecla "Esc"
      Entonces el audio se reproduce desde la posición anotada con 0,1 s de tolerancia

  @TRA-18
  Regla: Atajos de teclado

    Antecedentes:
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Y se importa en la entrevista "E01" el archivo generado "editor.vtt"
      Y la interfaz está abierta en el navegador
      Y la transcripción de E01 está abierta en el editor

    @TRA-18.1 @interfaz
    Esquema del escenario: Cada atajo funciona con el foco en el texto y no escribe nada
      Dado <preparación>
      Y el foco está en el texto del segmento "s0002"
      Cuando la persona pulsa la tecla "<atajo>"
      Entonces <efecto>
      Y la tecla no escribió ningún carácter en el texto de ningún segmento

      Ejemplos:
        | atajo            | preparación                                             | efecto                                                                    |
        | Esc              | la posición del audio es 3,0 s en pausa                 | el audio se está reproduciendo                                            |
        | Ctrl+Alt+J       | la posición del audio es 10,0 s en pausa                | la posición del audio es 7,0 s con 0,1 s de tolerancia                    |
        | Ctrl+Alt+Espacio | la posición del audio es 40,0 s en pausa                | el audio se reproduce desde 2,0 s con 0,1 s de tolerancia                 |
        | Ctrl+Alt+,       | la velocidad es 1                                       | la velocidad es 0,75                                                      |
        | Ctrl+Alt+.       | la velocidad es 1                                       | la velocidad es 1,25                                                      |
        | Alt+↑            | no hay ninguna preparación adicional                    | el foco está en el texto del segmento "s0001"                             |
        | Alt+↓            | no hay ninguna preparación adicional                    | el foco está en el texto del segmento "s0003"                             |
        | Ctrl+Enter       | no hay ninguna preparación adicional                    | el segmento "s0002" muestra la etiqueta "Revisado" y el foco está en el texto del segmento "s0003" |
        | Ctrl+Alt+S       | no hay ninguna preparación adicional                    | el foco está en el texto del segmento "s0008"                             |
        | Ctrl+Alt+1       | no hay ninguna preparación adicional                    | el segmento "s0002" muestra la etiqueta de hablante "Persona entrevistadora" |
        | Ctrl+Alt+2       | el segmento "s0002" de E01 tiene hablante null          | el segmento "s0002" muestra la etiqueta de hablante "E01"                 |
        | Ctrl+Alt+3       | no hay ninguna preparación adicional                    | el segmento "s0002" muestra la etiqueta de hablante "Otra persona"        |
        | Ctrl+Alt+0       | no hay ninguna preparación adicional                    | el segmento "s0002" muestra la etiqueta de hablante "Sin asignar"         |
        | Ctrl+Alt+D       | el cursor está en la posición 3 del texto de "s0002"    | el editor tiene 11 segmentos y "s0002" tiene el texto "Con"               |
        | Ctrl+Alt+U       | no hay ninguna preparación adicional                    | el editor tiene 9 segmentos y "s0002" tiene el texto "Con mucho gusto Vivo en el barrio desde hace años" |
        | Ctrl+Alt+E       | no hay ninguna preparación adicional                    | el editor tiene 9 segmentos y no muestra "s0002"                          |
        | Ctrl+Alt+I       | la posición del audio es 3,0 s en pausa                 | el segmento "s0002" tiene inicio 3,000                                    |
        | Ctrl+Alt+O       | la posición del audio es 4,0 s en pausa                 | el segmento "s0002" tiene fin 4,000                                       |
        | Ctrl+Z           | la persona escribió "abc" al final del texto del segmento "s0002" | el segmento "s0002" tiene el texto "Con mucho gusto"            |
        | Ctrl+Shift+Z     | la persona escribió "abc" al final del texto del segmento "s0002" y pulsó "Ctrl+Z" | el segmento "s0002" tiene el texto "Con mucho gustoabc" |
        | Ctrl+Y           | la persona escribió "abc" al final del texto del segmento "s0002" y pulsó "Ctrl+Z" | el segmento "s0002" tiene el texto "Con mucho gustoabc" |
        | Ctrl+K           | no hay ninguna preparación adicional                    | la interfaz muestra el panel de atajos                                    |

    @TRA-18.2 @interfaz
    Escenario: Ningún atajo dispara una acción del navegador
      Dado el foco está en el texto del segmento "s0002"
      Cuando la persona pulsa, uno por vez, cada atajo de la tabla de TRA-18
      Entonces la cantidad de pestañas abiertas no cambió
      Y la pestaña de la app siguió activa después de cada atajo
      Y el foco nunca salió del documento de la app hacia la barra de direcciones ni hacia un menú del navegador

    @TRA-18.3 @interfaz
    Escenario: Ctrl+K muestra el panel con toda la tabla y Esc lo cierra
      Dado la posición del audio es 3,0 s en pausa
      Cuando la persona pulsa la tecla "Ctrl+K"
      Entonces el panel de atajos muestra estas filas
        | acción                                                    | atajo                                             |
        | Reproducir o pausar                                       | Esc                                               |
        | Retroceder retroceso_s segundos                           | Ctrl+Alt+J                                        |
        | Reproducir desde el inicio del segmento con foco          | Ctrl+Alt+Espacio                                  |
        | Bajar o subir la velocidad                                | Ctrl+Alt+, y Ctrl+Alt+.                           |
        | Segmento anterior o siguiente                             | Alt+↑ y Alt+↓                                     |
        | Marcar o desmarcar como revisado                          | Ctrl+Enter                                        |
        | Siguiente sospechoso sin revisar                          | Ctrl+Alt+S                                        |
        | Hablante entrevistadora, entrevistada, otra o sin asignar | Ctrl+Alt+1, Ctrl+Alt+2, Ctrl+Alt+3 y Ctrl+Alt+0   |
        | Dividir en el cursor                                      | Ctrl+Alt+D                                        |
        | Unir con el siguiente                                     | Ctrl+Alt+U                                        |
        | Eliminar el segmento                                      | Ctrl+Alt+E                                        |
        | Fijar el inicio o el fin en la posición del audio         | Ctrl+Alt+I y Ctrl+Alt+O                           |
        | Deshacer                                                  | Ctrl+Z                                            |
        | Rehacer                                                   | Ctrl+Shift+Z o Ctrl+Y                             |
        | Ver la lista de atajos                                    | Ctrl+K                                            |
      Cuando la persona pulsa la tecla "Esc"
      Entonces la interfaz no muestra el panel de atajos
      Y la posición del audio es 3,0 s con 0,1 s de tolerancia
      Y el audio está en pausa

  @TRA-19
  Regla: Edición de segmentos

    Antecedentes:
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Y se importa en la entrevista "E01" el archivo generado "editor.vtt"
      Y la interfaz está abierta en el navegador
      Y la transcripción de E01 está abierta en el editor

    @TRA-19.1 @interfaz
    Escenario: Dividir con el audio fuera del segmento corta en proporción al cursor
      Dado se anotó el siguiente_id de "identificable/entrevistas/E01/transcripcion.json"
      Y la posición del audio es 50,0 s en pausa
      Y el cursor está en la posición 4 del texto de "s0007"
      Cuando la persona pulsa la tecla "Ctrl+Alt+D"
      Y la interfaz muestra "Guardado"
      Entonces el segmento "s0007" de "identificable/entrevistas/E01/transcripcion.json" va de 20,000 a 24,000 con el texto "Hola"
      Y el segmento siguiente a "s0007" en "identificable/entrevistas/E01/transcripcion.json" va de 24,000 a 30,000 con el texto "mundo"
      Y el id de ese segmento nuevo es "s" seguido del siguiente_id anotado con al menos 4 cifras

    @TRA-19.2 @interfaz
    Escenario: Dividir con el audio dentro del segmento corta en la posición del audio
      Dado la posición del audio es 22,5 s en pausa
      Y el cursor está en la posición 4 del texto de "s0007"
      Cuando la persona pulsa la tecla "Ctrl+Alt+D"
      Y la interfaz muestra "Guardado"
      Entonces el segmento "s0007" de "identificable/entrevistas/E01/transcripcion.json" va de 20,000 a 22,500 con el texto "Hola"
      Y el segmento siguiente a "s0007" en "identificable/entrevistas/E01/transcripcion.json" va de 22,500 a 30,000 con el texto "mundo"

    @TRA-19.3 @interfaz @PRY-18.1
    Escenario: No se unen segmentos de hablantes distintos
      Dado se tomó la huella de "identificable/entrevistas/E01/transcripcion.json"
      Y el foco está en el texto del segmento "s0003"
      Cuando la persona pulsa la tecla "Ctrl+Alt+U"
      Entonces la interfaz muestra "No se pueden unir segmentos de hablantes distintos."
      Y la huella de "identificable/entrevistas/E01/transcripcion.json" es la misma que se tomó
      Y el editor tiene 10 segmentos

    @TRA-19.4 @interfaz
    Escenario: Un salto de línea pegado se vuelve un espacio
      Dado el texto del segmento "s0005" está vacío y tiene el foco
      Cuando la persona pega en el texto del segmento "s0005" el texto "uno" seguido de un salto de línea y "dos"
      Y la interfaz muestra "Guardado"
      Entonces el segmento "s0005" de "identificable/entrevistas/E01/transcripcion.json" tiene el texto "uno dos"

    @TRA-19.5 @interfaz
    Escenario: Eliminar y deshacer devuelve el segmento igual
      Dado se guardó una copia del segmento "s0004" de "identificable/entrevistas/E01/transcripcion.json"
      Y el foco está en el texto del segmento "s0004"
      Cuando la persona pulsa la tecla "Ctrl+Alt+E"
      Y la persona pulsa la tecla "Ctrl+Z"
      Y la interfaz muestra "Guardado"
      Entonces el cuarto segmento de "identificable/entrevistas/E01/transcripcion.json" es idéntico a la copia guardada de "s0004"

    @PRY-18.1 @interfaz
    Esquema del escenario: Errores de edición de segmentos
      Dado se tomó la huella de "identificable/entrevistas/E01/transcripcion.json"
      Y <preparación>
      Cuando la persona pulsa la tecla "<atajo>"
      Entonces la interfaz muestra "<mensaje>"
      Y la huella de "identificable/entrevistas/E01/transcripcion.json" es la misma que se tomó

      Ejemplos:
        | preparación                                                                          | atajo      | mensaje                                                |
        | el cursor está en la posición 0 del texto de "s0007"                                 | Ctrl+Alt+D | No hay nada que dividir en esa posición.               |
        | el cursor está en la posición 10 del texto de "s0007"                                | Ctrl+Alt+D | No hay nada que dividir en esa posición.               |
        | el foco está en el texto del segmento "s0010"                                        | Ctrl+Alt+U | No hay un segmento siguiente.                          |
        | el foco está en el texto del segmento "s0005" y la posición del audio es 1,0 s en pausa  | Ctrl+Alt+I | Esa posición rompe el orden de los segmentos.      |
        | el foco está en el texto del segmento "s0005" y la posición del audio es 15,0 s en pausa | Ctrl+Alt+I | Esa posición rompe el orden de los segmentos.      |
        | el foco está en el texto del segmento "s0005" y la posición del audio es 11,0 s en pausa | Ctrl+Alt+O | El fin tiene que ser posterior al inicio.          |

    @PRY-18.1 @interfaz
    Escenario: No se puede eliminar el único segmento
      Dado la transcripción de E01 queda con un solo segmento después de eliminar los otros nueve
      Y el foco está en el texto del segmento "s0001"
      Cuando la persona pulsa la tecla "Ctrl+Alt+E"
      Entonces la interfaz muestra "La transcripción tiene que tener al menos un segmento."
      Y el editor tiene 1 segmento

    @PRY-18.1 @interfaz
    Escenario: Un segmento no acepta más de 10.000 caracteres
      Dado el texto del segmento "s0005" tiene 9.999 caracteres y tiene el foco
      Cuando la persona escribe "xy" al final del texto del segmento "s0005"
      Entonces el texto del segmento "s0005" en la interfaz tiene 10.000 caracteres
      Y la interfaz muestra "El segmento admite hasta 10.000 caracteres."

    @PRY-18.1 @interfaz
    Escenario: Sin audio presente fijar inicio o fin está deshabilitado
      Dado el audio de E01 está eliminado con la excepción "eliminar_audio_sin_revisar"
      Y el foco está en el texto del segmento "s0005"
      Entonces la acción de fijar el inicio en la posición del audio está deshabilitada
      Y la acción de fijar el fin en la posición del audio está deshabilitada

  @TRA-20
  Regla: Identificadores estables

    @TRA-20.1 @TRA-20.2
    Escenario: Mil operaciones al azar conservan los identificadores
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Y se importa en la entrevista "E01" el archivo generado "quinientos.vtt" con 500 bloques
      Y la transcripción de E01 está abierta en el editor
      Cuando se hacen 1.000 operaciones elegidas al azar entre editar, dividir, unir, eliminar, deshacer y rehacer, anotando cada una
      Entonces todos los id de "identificable/entrevistas/E01/transcripcion.json" son únicos
      Y ningún id retirado reaparece salvo al deshacer o rehacer la operación que lo retiró
      Y el siguiente_id es mayor que el número de todo id usado durante la prueba
      Y el siguiente_id nunca bajó durante la prueba
      Y cada segmento que no fue dividido, unido ni eliminado conserva su id y su texto, salvo las ediciones que recibió

  @TRA-21
  Regla: Deshacer y rehacer

    Antecedentes:
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Y se importa en la entrevista "E01" el archivo generado "editor.vtt"
      Y la interfaz está abierta en el navegador
      Y la transcripción de E01 está abierta en el editor

    @TRA-21.1 @interfaz
    Escenario: Una palabra escrita sin pausa se deshace de una vez
      Dado el foco está en el texto del segmento "s0005"
      Cuando la persona escribe "hola" al final del texto del segmento "s0005" sin pausas mayores de 1 s
      Y la persona pulsa la tecla "Ctrl+Z"
      Entonces el texto del segmento "s0005" en la interfaz es "Muchas cosas"

    @TRA-21.2 @interfaz
    Escenario: Después de 250 acciones se deshacen al menos 200
      Dado el foco está en el texto del segmento "s0005"
      Cuando la persona hace 250 acciones de edición separadas por pausas de más de 1 s
      Y la persona pulsa 200 veces la tecla "Ctrl+Z"
      Entonces cada una de las 200 pulsaciones deshizo un paso
      Y la transcripción en la interfaz es igual a la que había después de la acción 50

    @TRA-21.3 @interfaz
    Escenario: Deshacer una división devuelve el segmento y llega al disco
      Dado se guardó una copia del segmento "s0007" de "identificable/entrevistas/E01/transcripcion.json"
      Y la posición del audio es 50,0 s en pausa
      Y el cursor está en la posición 4 del texto de "s0007"
      Y la persona pulsa la tecla "Ctrl+Alt+D"
      Cuando la persona pulsa la tecla "Ctrl+Z"
      Entonces a más tardar 1 s después el segmento "s0007" de "identificable/entrevistas/E01/transcripcion.json" es idéntico a la copia guardada
      Y el editor tiene 10 segmentos

    @TRA-21.4 @interfaz
    Escenario: Después de marcar la transcripción revisada no hay nada que deshacer
      Dado la persona escribió "abc" al final del texto del segmento "s0002"
      Y todos los segmentos de E01 están marcados como revisados
      Y la persona pulsa "Marcar transcripción como revisada"
      Y se tomó la huella de "identificable/entrevistas/E01/transcripcion.json"
      Cuando la persona pulsa la tecla "Ctrl+Z"
      Entonces la huella de "identificable/entrevistas/E01/transcripcion.json" es la misma que se tomó
      Y el segmento "s0002" tiene el texto "Con mucho gustoabc"

  @TRA-22
  Regla: Tiempo de revisión

    # Abrir el editor ya es actividad, así que cada escenario mide cuánto suma el tiempo
    # de revisión desde un momento sin actividad en los 60 s anteriores.

    @TRA-22.1 @interfaz
    Escenario: Diez minutos de reproducción continua suman 600 s
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador con un reloj simulado
      Y la transcripción de E01 está abierta en el editor
      Y pasan 2 minutos del reloj simulado sin actividad
      Y se anota el tiempo de revisión que muestra el encabezado
      Cuando el audio se reproduce sin parar durante 10 minutos del reloj simulado
      Entonces el tiempo de revisión que muestra el encabezado sumó 600 s al anotado

    @TRA-22.2 @interfaz
    Escenario: Una tecla y cinco minutos sin actividad suman 60 s
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador con un reloj simulado
      Y la transcripción de E01 está abierta en el editor
      Y el audio está en pausa
      Y pasan 2 minutos del reloj simulado sin actividad
      Y se anota el tiempo de revisión que muestra el encabezado
      Cuando la persona escribe "x" al final del texto del segmento "s0001"
      Y pasan 5 minutos del reloj simulado sin actividad
      Entonces el tiempo de revisión que muestra el encabezado sumó 60 s al anotado
      Y la clave "tiempo_revision_s" de "identificable/entrevistas/E01/transcripcion.json" vale lo que muestra el encabezado

    @TRA-22.3 @interfaz
    Escenario: Con la pestaña oculta no se suma nada
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador con un reloj simulado
      Y la transcripción de E01 está abierta en el editor
      Y pasan 2 minutos del reloj simulado sin actividad
      Y se anota el tiempo de revisión que muestra el encabezado
      Y la pestaña de la app queda oculta
      Cuando el audio se reproduce sin parar durante 10 minutos del reloj simulado
      Y la pestaña de la app vuelve a ser visible
      Entonces el tiempo de revisión que muestra el encabezado sumó 0 s al anotado

    @TRA-22.4 @interfaz
    Escenario: El encabezado muestra la proporción con la duración del audio
      Dado un proyecto nuevo con la entrevista "E01" sin audio
      Y se importa en la entrevista "E01" el audio generado de 20 minutos
      Y se importa en la entrevista "E01" el archivo generado "veinte-minutos.vtt" con 20 bloques de 60 s
      Y osr se cierra de forma ordenada
      Y con la app cerrada se cambia en "identificable/entrevistas/E01/transcripcion.json" la clave "tiempo_revision_s" a 1800
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Cuando la transcripción de E01 está abierta en el editor
      Entonces la interfaz muestra un texto que empieza con "Tiempo de revisión: 0:30:00 (1,5 min por minuto de audio)"

  @TRA-23
  Regla: Eliminar el audio tras la revisión

    @TRA-23.1 @interfaz
    Escenario: Con la transcripción revisada eliminar el audio deja el rastro
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la transcripción de E01 está revisada
      Y la interfaz está abierta en el navegador
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando la interfaz muestra la vista de la entrevista "E01"
      Y la persona pulsa "Eliminar audio"
      Y la persona escribe "E01" en el campo de confirmación
      Y la persona confirma la eliminación
      Entonces no existe "identificable/entrevistas/E01/E01.ogg"
      Y "identificable/entrevistas/E01/E01.ogg.ficha.toml" tiene estas claves con estos valores
        | clave              | valor                                      |
        | estado             | "eliminado"                                |
        | motivo_eliminacion | "retencion"                                |
        | sha256             | {huella de E01.ogg de los datos de prueba} |
      Y la clave "eliminado" de "identificable/entrevistas/E01/E01.ogg.ficha.toml" es una marca de tiempo ISO 8601 con desplazamiento y precisión de segundos
      Y la clave "audio_sha256" de "identificable/entrevistas/E01/transcripcion.json" vale {huella de E01.ogg de los datos de prueba}
      Y el registro de eventos tiene exactamente una línea nueva de tipo "audio_eliminado" cuyo detalle contiene "E01" y la huella de "E01.ogg"
      Y la entrevista "E01" muestra la etiqueta "Audio eliminado el {hoy}"

    @TRA-23.2 @interfaz
    Escenario: Una confirmación que no coincide no borra nada
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la transcripción de E01 está revisada
      Y la interfaz está abierta en el navegador
      Y se tomó la huella de cada archivo del proyecto
      Cuando la interfaz muestra la vista de la entrevista "E01"
      Y la persona pulsa "Eliminar audio"
      Y la persona escribe "E02" en el campo de confirmación
      Y la persona confirma la eliminación
      Entonces existe el archivo "identificable/entrevistas/E01/E01.ogg"
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas

    @TRA-23.3
    Escenario: Un audio eliminado no figura como faltante al abrir
      Dado una copia del proyecto de prueba
      Y el audio de E01 está eliminado según TRA-23
      Y osr se cierra de forma ordenada
      Cuando se abre el proyecto con `osr abrir {proyecto} --sin-navegador`
      Entonces el evento "verificacion" más reciente no tiene "identificable/entrevistas/E01/E01.ogg" en la lista "faltantes"

    @TRA-23.4
    Escenario: El audio eliminado responde 410
      Dado una copia del proyecto de prueba
      Y el audio de E01 está eliminado según TRA-23
      Y osr está corriendo con el proyecto abierto
      Y un pedido HTTP GET a "/audio/E01"
      Y el pedido lleva el token válido en el encabezado Authorization
      Cuando se manda el pedido
      Entonces la respuesta tiene estado 410

    @PRY-18.1 @interfaz
    Escenario: Sin la transcripción revisada eliminar el audio pide excepción
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Cuando la interfaz muestra la vista de la entrevista "E01"
      Y la persona pulsa "Eliminar audio"
      Entonces la interfaz muestra "La transcripción de E01 no está revisada."
      Y el diálogo muestra los botones "Cancelar" y "Hacerlo igual"

    @PRY-18.1 @interfaz
    Escenario: Si el consentimiento oral está en el audio el diálogo lo advierte
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y el consentimiento de "E01" tiene respaldo "audio_entrevista" entre 2,0 y 9,5 s
      Y la transcripción de E01 está revisada
      Y la interfaz está abierta en el navegador
      Cuando la interfaz muestra la vista de la entrevista "E01"
      Y la persona pulsa "Eliminar audio"
      Entonces la interfaz muestra "El consentimiento oral de E01 está grabado en este audio y se pierde con él."

  @TRA-24
  Regla: Accesibilidad del editor

    @TRA-24.1 @interfaz
    Escenario: Toda la revisión se hace solo con el teclado
      Dado un proyecto nuevo con la entrevista "E01" y su audio de prueba importado
      Y la interfaz está abierta en el navegador
      Y la prueba usa solo el teclado desde este paso
      Cuando la persona importa con el teclado los archivos de la cadena de prueba de E01 con el modelo "large-v3-turbo"
      Y abre la transcripción de E01 con el teclado
      Y reproduce, retrocede y cambia la velocidad con sus atajos
      Y asigna un hablante, edita un texto, divide un segmento y une dos segmentos con sus atajos
      Y salta al siguiente sospechoso con "Ctrl+Alt+S"
      Y marca todos los segmentos como revisados con "Ctrl+Enter"
      Y marca la transcripción como revisada con el teclado
      Entonces la clave "estado" de "identificable/entrevistas/E01/transcripcion.json" vale "revisada"
      Y la prueba no usó el ratón en ningún paso

    @TRA-24.2 @interfaz
    Escenario: Sin colores cada estado y cada sospecha se distinguen por su texto
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y el segmento "s0001" de E01 está marcado como revisado
      Y la interfaz está abierta en el navegador
      Y los estilos de color de la página están anulados y todo se ve en gris
      Cuando la transcripción de E01 está abierta en el editor
      Entonces el segmento "s0001" muestra visible la etiqueta "Revisado"
      Y el segmento "s0002" muestra visible la etiqueta "Sin revisar"
      Y cada segmento con sospecha muestra visible el texto "Sospechoso: " seguido de los textos de sus marcas
