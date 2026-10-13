# language: es
#
# Escenarios del módulo proyecto (PRY), tramo 1 de la v0.1.
# Fuente especificaciones/proyecto.md.
#
# Convenciones comunes a los cuatro archivos de escenarios
# - {tmp} es una carpeta temporal nueva para cada escenario.
# - {proyecto} es la carpeta del proyecto de la prueba, {tmp}/estudio salvo que el escenario diga otra.
# - {P} es el puerto del arranque en curso y {token} el token de esa sesión.
# - {marca} es la marca compacta AAAAMMDDTHHMMSS que generó la app.
# - {hoy} y {mañana} son fechas ISO 8601 del reloj del sistema de prueba.
# - Los textos entre <> solo aparecen en los esquemas y se reemplazan con sus Ejemplos.
# - Los demás valores entre llaves, como {duración E01}, {huella de E01.ogg de los datos de prueba},
#   {PID del primer proceso} o {201 veces la letra a}, se describen por su propio texto y los
#   resuelve la prueba.
# - Las rutas de archivos del proyecto son relativas a {proyecto}.
# - «ningún archivo del proyecto cambió» compara las mismas rutas con las mismas huellas.
#
# Etiquetas comunes
# - @interfaz corre en el navegador con Playwright, una vez en Firefox y otra en Chromium.
# - @recorrido-completo se verifica sobre el proyecto de prueba después del recorrido
#   completo, la ejecución en orden de todas las acciones de los escenarios del tramo 1.
# - @equipo-autor es una medición automática que solo vale en el equipo del autor.
# - @manual no se puede verificar de forma automática y lo revisa una persona.
# - @PRY-18.1 marca los escenarios que comprueban un mensaje textual de la especificación.
#
# Datos de prueba (PRY-17), todos inventados
# - Proyecto de prueba con los participantes E01 y E02, cada uno con consentimiento
#   registrado con grabar = otorgado, su entrevista, su audio y su transcripción importada
#   desde la cadena del autor, en estado sin revisar.
#   El consentimiento de E02 tiene la versión de formulario "v1, octubre", escrita entre comillas en el CSV.
# - Audios E01.ogg (Ogg Opus) y E02.m4a, de 120 s o menos cada uno.
# - Primer conjunto de nombres, en la tabla de correspondencia y dichos dentro de las
#   transcripciones y encabezados, Ana Inventada Muñoz (E01), Bruno Ficticio Ramírez (E02)
#   y Carmen Inventada Solís (solo para PRY-11.1 y PAR-05.1).
# - Segundo conjunto de nombres, solo en nombres originales de archivos, Nombre Inventado
#   y Carla Supuesta Vega.

Característica: Proyecto en el disco
  Un proyecto es una carpeta de archivos de texto abierto que son la única fuente de
  verdad, con un índice que se puede borrar y un README que explica todo sin la app.

  @PRY-01
  Regla: Crear un proyecto

    @PRY-01.1
    Escenario: osr nuevo crea exactamente las rutas de la columna «Al crear»
      Dado una carpeta temporal vacía "{tmp}"
      Cuando se ejecuta `osr nuevo {tmp}/estudio --sin-navegador`
      Y osr se cierra de forma ordenada
      Entonces la carpeta "{tmp}/estudio" tiene exactamente estas rutas
        | ruta                              |
        | proyecto.toml                     |
        | README.md                         |
        | sensibilidad.csv                  |
        | bitacora/                         |
        | bitacora/bitacora.md              |
        | bitacora/registro.jsonl           |
        | participantes/                    |
        | participantes/participantes.csv   |
        | participantes/consentimientos.csv |
        | identificable/                    |
        | identificable/correspondencia.csv |
        | identificable/consentimientos/    |
        | identificable/entrevistas/        |
        | .osr/                             |
        | .osr/indice.sqlite                |

    @PRY-01.2
    Escenario: osr nuevo usa una carpeta existente y vacía
      Dado una carpeta temporal vacía "{tmp}"
      Y existe la carpeta vacía "{tmp}/vacia"
      Cuando se ejecuta `osr nuevo {tmp}/vacia --sin-navegador`
      Entonces la salida de error está vacía
      Y existe el archivo "{tmp}/vacia/proyecto.toml"
      Y osr está corriendo con el proyecto abierto

    @PRY-01.3 @PRY-18.1
    Escenario: osr nuevo rechaza una carpeta que no está vacía y no la toca
      Dado una carpeta temporal vacía "{tmp}"
      Y existe el archivo "{tmp}/ocupada/notas.txt" con el texto "hola"
      Y se tomó la huella de cada archivo de "{tmp}/ocupada"
      Cuando se ejecuta `osr nuevo {tmp}/ocupada --sin-navegador`
      Entonces el código de salida es 3
      Y la salida de error es la línea "osr: la carpeta {tmp}/ocupada ya existe y no está vacía. Hay que elegir una carpeta nueva o vacía."
      Y ningún archivo de "{tmp}/ocupada" cambió respecto de las huellas tomadas

    @PRY-01.4
    Escenario: La ruta puede tener espacios, tildes y ñ
      Dado existe la carpeta vacía "/tmp/prueba"
      Y no existe "/tmp/prueba/Investigación año uno"
      Cuando se ejecuta `osr nuevo "/tmp/prueba/Investigación año uno" --sin-navegador`
      Entonces la salida de error está vacía
      Y osr está corriendo con el proyecto abierto
      Y la clave "nombre" de "/tmp/prueba/Investigación año uno/proyecto.toml" vale "Investigación año uno"

    @PRY-01.5 @PRY-18.1
    Escenario: Si la escritura falla a mitad de la creación no queda nada
      Dado un disco de prueba montado en "{tmp}/disco" con solo 16 KiB libres
      Y la carpeta "{tmp}/disco" está vacía
      Cuando se ejecuta `osr nuevo {tmp}/disco/estudio --sin-navegador`
      Entonces el código de salida es 6
      Y la salida de error empieza con "osr: no se pudo crear el proyecto en {tmp}/disco/estudio ("
      Y no existe "{tmp}/disco/estudio"
      Y la carpeta "{tmp}/disco" sigue vacía

    @PRY-01.6
    Escenario: El registro de eventos de un proyecto nuevo tiene solo la creación
      # La especificación pide exactamente una línea, aunque PRY-04 corre la verificación
      # al abrir, que anota otro evento. Se escribe la lectura literal del criterio y la
      # tensión queda anotada para el autor.
      Dado una carpeta temporal vacía "{tmp}"
      Cuando se ejecuta `osr nuevo {tmp}/estudio --sin-navegador`
      Entonces el archivo "{tmp}/estudio/bitacora/registro.jsonl" tiene exactamente 1 línea
      Y la última línea del registro de eventos es de tipo "proyecto_creado"

    @PRY-18.1
    Esquema del escenario: Errores de osr nuevo con su código y su mensaje
      Dado una carpeta temporal vacía "{tmp}"
      Cuando se ejecuta `<orden>`
      Entonces el código de salida es <código>
      Y la salida de error es la línea "<mensaje>"
      Y la carpeta "{tmp}" sigue vacía

      Ejemplos:
        | orden                                                                          | código | mensaje                                                                                     |
        | osr nuevo {tmp}/falta/estudio --sin-navegador                                  | 3      | osr: no existe la carpeta {tmp}/falta.                                                      |
        | osr nuevo {tmp}/estudio --nombre "" --sin-navegador                            | 2      | osr: el nombre del proyecto tiene que tener entre 1 y 200 caracteres, sin saltos de línea. |
        | osr nuevo {tmp}/estudio --nombre "{201 veces la letra a}" --sin-navegador      | 2      | osr: el nombre del proyecto tiene que tener entre 1 y 200 caracteres, sin saltos de línea. |
        | osr nuevo {tmp}/estudio --nombre "{Uno, salto de línea, Dos}" --sin-navegador  | 2      | osr: el nombre del proyecto tiene que tener entre 1 y 200 caracteres, sin saltos de línea. |

    @PRY-18.1
    Escenario: Sin permiso de escritura osr nuevo termina con código 6
      # El motivo en español no está fijado por la especificación, por eso se compara solo
      # el comienzo del mensaje.
      Dado una carpeta temporal vacía "{tmp}"
      Y la carpeta "{tmp}" queda con permisos 0500
      Cuando se ejecuta `osr nuevo {tmp}/estudio --sin-navegador`
      Entonces el código de salida es 6
      Y la salida de error empieza con "osr: no se pudo crear el proyecto en {tmp}/estudio ("
      Y la salida de error termina con ")."
      Y la carpeta "{tmp}" sigue vacía

  @PRY-02
  Regla: Estructura de la carpeta

    @PRY-02.1
    Escenario: Un proyecto recién creado tiene las rutas «Al crear» y el bloqueo solo mientras está abierto
      Dado un proyecto nuevo creado con `osr nuevo`
      Entonces la carpeta "{proyecto}" tiene exactamente estas rutas
        | ruta                              |
        | proyecto.toml                     |
        | README.md                         |
        | sensibilidad.csv                  |
        | bitacora/                         |
        | bitacora/bitacora.md              |
        | bitacora/registro.jsonl           |
        | participantes/                    |
        | participantes/participantes.csv   |
        | participantes/consentimientos.csv |
        | identificable/                    |
        | identificable/correspondencia.csv |
        | identificable/consentimientos/    |
        | identificable/entrevistas/        |
        | .osr/                             |
        | .osr/indice.sqlite                |
        | .osr/bloqueo                      |
      Cuando osr se cierra de forma ordenada
      Entonces no existe ".osr/bloqueo"
      Y no existe "entrevistas/"
      Y no existe "datos/"

    @PRY-02.2
    Escenario: Una carpeta de «Al crear» que falta se vuelve a crear al abrir
      Dado una copia del proyecto de prueba
      Y con la app cerrada se borra la carpeta "identificable/consentimientos/" con todo su contenido
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando se abre el proyecto con `osr abrir {proyecto} --sin-navegador`
      Entonces existe la carpeta vacía "identificable/consentimientos/"
      Y el registro de eventos tiene exactamente una línea nueva de tipo "carpeta_recreada" cuyo detalle contiene "identificable/consentimientos"

    @PRY-02.3 @recorrido-completo
    Escenario: Ninguna ruta gestionada contiene nombres de personas
      Dado el proyecto de prueba después del recorrido completo de los escenarios del tramo 1
      Entonces ninguna ruta gestionada del proyecto contiene, sin distinguir mayúsculas, ninguno de los nombres de la tabla de correspondencia de prueba

  @PRY-03
  Regla: Archivo proyecto.toml

    @PRY-03.1
    Escenario: proyecto.toml nuevo tiene las claves obligatorias y los valores por defecto
      # La especificación no dice si las claves opcionales se escriben al crear. Se exige que
      # las obligatorias estén y que cada opcional, si está, valga su valor por defecto.
      Dado un proyecto nuevo creado con `osr nuevo`
      Entonces "proyecto.toml" es TOML 1.0 en UTF-8 sin BOM
      Y "proyecto.toml" tiene estas claves con estos valores
        | clave           | valor          |
        | formato         | "osr-proyecto" |
        | version_formato | 1              |
        | nombre          | "estudio"      |
      Y la clave "creado" de "proyecto.toml" es una marca de tiempo ISO 8601 con desplazamiento y precisión de segundos
      Y cada una de estas claves de "proyecto.toml", si está en el archivo, tiene este valor
        | clave                                    | valor                    |
        | participantes.prefijo                    | "E"                      |
        | participantes.digitos                    | 2                        |
        | transcripcion.etiqueta_entrevistadora    | "Persona entrevistadora" |
        | transcripcion.retroceso_s                | 3                        |
        | transcripcion.sospecha.no_speech_prob    | 0.6                      |
        | transcripcion.sospecha.avg_logprob       | -1.0                     |
        | transcripcion.sospecha.compression_ratio | 2.4                      |

    @PRY-03.2 @PRY-18.1
    Escenario: Una versión de formato distinta de 1 no se abre y no escribe nada
      Dado una copia del proyecto de prueba
      Y con la app cerrada se cambia en "proyecto.toml" la clave "version_formato" a 2
      Y se tomó la huella de cada archivo del proyecto
      Cuando se ejecuta `osr abrir {proyecto} --sin-navegador`
      Entonces el código de salida es 3
      Y la salida de error es la línea "osr: este proyecto usa la versión 2 del formato y esta versión de osr solo abre la versión 1."
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas

    @PRY-03.3 @interfaz
    Escenario: Una clave opcional fuera de rango toma su valor por defecto con un aviso
      Dado una copia del proyecto de prueba
      Y con la app cerrada se cambia en "proyecto.toml" la clave "transcripcion.retroceso_s" a 9
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Entonces la interfaz muestra "El valor de transcripcion.retroceso_s en proyecto.toml no es válido; se usa 3."
      Cuando la transcripción de E01 está abierta en el editor
      Y la posición del audio es 10,0 s
      Y la persona pulsa la tecla "Ctrl+Alt+J"
      Entonces la posición del audio es 7,0 s con 0,1 s de tolerancia

    @PRY-03.4 @interfaz
    Escenario: Agregar un alcance conserva comentarios y claves desconocidas
      Dado una copia del proyecto de prueba
      Y con la app cerrada se agregan al final de "proyecto.toml" estas líneas
        """
        # Comentario de prueba que la app no conoce
        [extra]
        color = "azul"
        """
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Cuando la persona agrega desde la interfaz el alcance "transcribir_local" con la etiqueta "Transcribir con software local"
      Entonces el archivo "proyecto.toml" contiene estas líneas sin cambios
        """
        # Comentario de prueba que la app no conoce
        [extra]
        color = "azul"
        """

    @PRY-18.1
    Esquema del escenario: Errores de proyecto.toml al abrir
      Dado una copia del proyecto de prueba
      Y con la app cerrada <alteración>
      Y se tomó la huella de cada archivo del proyecto
      Cuando se ejecuta `osr abrir {proyecto} --sin-navegador`
      Entonces el código de salida es 3
      Y la salida de error empieza con "<comienzo del mensaje>"
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas

      Ejemplos:
        | alteración                                                        | comienzo del mensaje                                                               |
        | se borra el archivo "proyecto.toml"                               | osr: {proyecto} no es un proyecto de Open Social Research (falta proyecto.toml).   |
        | se reemplaza la línea 3 de "proyecto.toml" por "nombre = "        | osr: proyecto.toml tiene un error en la línea 3:                                   |
        | se cambia en "proyecto.toml" la clave "formato" a "otro-formato"  | osr: proyecto.toml no es de Open Social Research o le falta formato.               |
        | se quita de "proyecto.toml" la clave "nombre"                     | osr: proyecto.toml no es de Open Social Research o le falta nombre.                |

  @PRY-04
  Regla: Abrir un proyecto

    @PRY-04.1
    Escenario: osr abrir y osr con la ruta producen el mismo resultado
      Dado una copia del proyecto de prueba
      Cuando se ejecuta `osr abrir {proyecto} --sin-navegador`
      Y se guarda el estado que muestra la app
      Y osr se cierra de forma ordenada
      Y se ejecuta `osr {proyecto} --sin-navegador`
      Entonces la salida estándar tiene exactamente una línea con la forma "Open Social Research está abierto en http://127.0.0.1:{P}/?token={token}"
      Y la app muestra el mismo estado que el guardado

    @PRY-04.2 @interfaz
    Escenario: El resumen del proyecto de prueba muestra sus dos entrevistas con sus estados
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Entonces la vista visible es "Resumen del proyecto"
      Y la entrevista "E01" muestra las etiquetas "Audio {duración E01}" y "Transcripción sin revisar"
      Y la entrevista "E02" muestra las etiquetas "Audio {duración E02}" y "Transcripción sin revisar"

    @PRY-04.3 @PRY-18.1
    Escenario: Abrir una ruta que no existe no crea nada
      Dado una carpeta temporal vacía "{tmp}"
      Cuando se ejecuta `osr abrir {tmp}/no-existe --sin-navegador`
      Entonces el código de salida es 3
      Y la salida de error es la línea "osr: no existe la carpeta {tmp}/no-existe."
      Y no existe "{tmp}/no-existe"

  @PRY-05
  Regla: Un solo proceso por proyecto

    @PRY-05.1 @PRY-18.1
    Escenario: Un segundo proceso no abre el mismo proyecto
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Cuando se ejecuta `osr abrir {proyecto} --sin-navegador`
      Entonces el código de salida es 4
      Y la salida de error es la línea "osr: el proyecto ya está abierto en otro proceso (PID {PID del primer proceso}). Hay que usar esa pestaña o cerrar ese proceso."
      Y el primer proceso osr sigue atendiendo pedidos autenticados

    @PRY-05.2
    Escenario: Un bloqueo de un proceso muerto se reemplaza
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y se envía SIGKILL al proceso osr
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando se abre el proyecto con `osr abrir {proyecto} --sin-navegador`
      Entonces osr está corriendo con el proyecto abierto
      Y el registro de eventos tiene exactamente una línea nueva de tipo "bloqueo_abandonado"

    @PRY-05.3
    Escenario: Después de un cierre ordenado no queda el bloqueo
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Cuando osr se cierra de forma ordenada
      Entonces no existe ".osr/bloqueo"

  @PRY-06
  Regla: Formatos de texto y fuente de verdad

    @PRY-06.1 @recorrido-completo
    Escenario: Ningún archivo de texto gestionado tiene BOM
      Dado el proyecto de prueba después del recorrido completo de los escenarios del tramo 1
      Entonces ningún archivo de texto gestionado del proyecto empieza con los bytes "EF BB BF"

    @PRY-06.2 @interfaz
    Escenario: El texto escrito con la ñ descompuesta se guarda en NFC
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y la interfaz muestra la vista "Correspondencia"
      Cuando la persona escribe en el campo "nota" de "E01" el texto "año" con la ñ descompuesta en n y tilde combinante U+0303
      Y la persona pulsa "Guardar"
      Y la interfaz muestra "Guardado"
      Entonces el archivo "identificable/correspondencia.csv" contiene los bytes "61 C3 B1 6F"
      Y el archivo "identificable/correspondencia.csv" no contiene los bytes "6E CC 83"

    @PRY-06.3 @interfaz
    Escenario: Guardar dos veces sin cambios deja los archivos iguales byte a byte
      # El tramo 1 no tiene un formulario propio para editar un participante. Se usa el
      # guardado de su fila en la vista Correspondencia y se vigilan los dos archivos.
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y la interfaz muestra la vista "Correspondencia"
      Y se tomó la huella de cada archivo del proyecto
      Cuando la persona pulsa "Guardar" en la fila de "E01" sin cambiar nada
      Y la persona pulsa "Guardar" en la fila de "E01" sin cambiar nada
      Entonces el archivo "participantes/participantes.csv" es idéntico byte a byte a su versión de las huellas tomadas
      Y el archivo "identificable/correspondencia.csv" es idéntico byte a byte a su versión de las huellas tomadas

    @PRY-06.4
    Esquema del escenario: Cada CSV gestionado se lee en R con las tildes intactas
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Cuando se lee "<archivo>" en R con `read.csv(ruta, fileEncoding = "UTF-8")`
      Entonces R devuelve las mismas filas, columnas y valores que la app muestra para "<archivo>"
      Y los valores con tildes y ñ de "<archivo>" quedan iguales en R y en la app

      Ejemplos:
        | archivo                           |
        | participantes/participantes.csv   |
        | participantes/consentimientos.csv |
        | identificable/correspondencia.csv |
        | sensibilidad.csv                  |

  @PRY-07
  Regla: Fichas de material propio

    @PRY-07.1
    Escenario: La ficha de un audio importado tiene todas sus claves obligatorias
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y el participante "E01" tiene consentimiento registrado con grabar "otorgado"
      Y la entrevista de "E01" está creada
      Cuando se importa en la entrevista "E01" el audio de prueba "E01.ogg"
      Entonces existe el archivo "identificable/entrevistas/E01/E01.ogg.ficha.toml"
      Y "identificable/entrevistas/E01/E01.ogg.ficha.toml" tiene estas claves con estos valores
        | clave           | valor                                  |
        | formato         | "osr-ficha"                            |
        | version_formato | 1                                      |
        | tipo            | "audio_entrevista"                     |
        | archivo         | "E01.ogg"                              |
        | sha256          | {huella de E01.ogg de los datos de prueba} |
        | bytes           | {tamaño de E01.ogg de los datos de prueba} |
        | formato_archivo | "ogg"                                  |
        | seudonimo       | "E01"                                  |
        | consentimiento  | "participantes/consentimientos.csv"    |
        | nombre_original | "E01.ogg"                              |
        | estado          | "presente"                             |
      Y la clave "importado" de "identificable/entrevistas/E01/E01.ogg.ficha.toml" es una marca de tiempo ISO 8601 con desplazamiento y precisión de segundos

    @PRY-07.2
    Escenario: El nombre original del archivo queda solo en la ficha
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y el participante "E01" tiene consentimiento registrado con grabar "otorgado"
      Y la entrevista de "E01" está creada
      Cuando se importa en la entrevista "E01" una copia del audio de prueba "E01.ogg" llamada "entrevista con Nombre Inventado.ogg"
      Entonces el archivo "identificable/entrevistas/E01/E01.ogg.ficha.toml" contiene el texto "Nombre Inventado"
      Y ningún archivo del proyecto fuera de "identificable/" contiene el texto "Nombre Inventado"
      Y ningún archivo de ".osr/" contiene el texto "Nombre Inventado"
      Y el registro técnico no contiene el texto "Nombre Inventado"

  @PRY-08
  Regla: Bitácora y registro de eventos

    @PRY-08.1
    Escenario: Crear un participante agrega una línea JSON con su seudónimo
      Dado un proyecto nuevo creado con `osr nuevo`
      Cuando el participante "E01" está registrado
      Entonces la última línea del registro de eventos es JSON válido
      Y la última línea del registro de eventos es de tipo "participante_creado" con "E01" en su detalle

    @PRY-08.2
    Escenario: El registro de eventos solo crece al final
      Dado un proyecto nuevo creado con `osr nuevo`
      Y se guardó una copia del registro de eventos
      Cuando el participante "E01" está registrado
      Y el participante "E01" tiene consentimiento registrado con grabar "otorgado"
      Y la entrevista de "E01" está creada
      Y se importa en la entrevista "E01" el audio de prueba "E01.ogg"
      Y se importan en la entrevista "E01" los archivos de la cadena de prueba de E01 con el modelo "large-v3-turbo"
      Y se edita el texto del segmento "s0001" de "E01"
      Entonces las primeras líneas del registro de eventos son idénticas byte a byte a todas las líneas de la copia guardada

    @PRY-08.3
    Escenario: Una última línea incompleta se descarta al abrir
      # La verificación al abrir anota su propio evento después de reparar el registro, así
      # que la línea de descarte se busca como primera línea nueva y no como última.
      Dado una copia del proyecto de prueba
      Y se guardó la cantidad de líneas del registro de eventos
      Y con la app cerrada se agrega al final de "bitacora/registro.jsonl" el texto "{"fecha":" sin salto de línea
      Cuando se abre el proyecto con `osr abrir {proyecto} --sin-navegador`
      Entonces el archivo "bitacora/registro.jsonl" no contiene una línea que sea exactamente "{"fecha":"
      Y la primera línea nueva del registro de eventos es de tipo "linea_incompleta_descartada"
      Y cada línea del registro de eventos es JSON válido

    @PRY-08.4 @recorrido-completo
    Escenario: La app no modifica la bitácora de la persona
      Dado el proyecto de prueba después del recorrido completo de los escenarios del tramo 1
      Entonces la huella de "bitacora/bitacora.md" es la misma que tenía antes del recorrido completo

    @PRY-08.5 @recorrido-completo
    Escenario: El registro de eventos no tiene nombres ni frases de transcripciones
      Dado el proyecto de prueba después del recorrido completo de los escenarios del tramo 1
      Entonces el archivo "bitacora/registro.jsonl" no contiene ninguno de los nombres de la tabla de correspondencia de prueba
      Y el archivo "bitacora/registro.jsonl" no contiene ninguna frase de las transcripciones de prueba

  @PRY-09
  Regla: Excepciones confirmadas

    @PRY-09.1 @interfaz
    Esquema del escenario: Enter sin mover el foco cancela la acción bloqueada
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y la interfaz muestra el bloqueo con excepción "<código>"
      Y se tomó la huella de cada archivo del proyecto
      Y se guardó la cantidad de líneas del registro de eventos
      Entonces el diálogo muestra los botones "Cancelar" y "Hacerlo igual"
      Y el botón con el foco es "Cancelar"
      Cuando la persona pulsa la tecla "Enter"
      Entonces ningún archivo del proyecto cambió respecto de las huellas tomadas
      Y el registro de eventos no recibió líneas nuevas

      Ejemplos:
        | código                               |
        | audio_sin_consentimiento             |
        | transcripcion_mas_larga_que_audio    |
        | reemplazar_transcripcion_con_cambios |
        | revisada_con_segmentos_sin_revisar   |
        | eliminar_audio_sin_revisar           |

    @PRY-09.2 @interfaz
    Esquema del escenario: Hacerlo igual anota una sola excepción y el evento de la acción
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y la interfaz muestra el bloqueo con excepción "<código>"
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando la persona pulsa "Hacerlo igual"
      Entonces el registro de eventos tiene exactamente una línea nueva de tipo "excepcion_confirmada" cuyo detalle contiene "<código>"
      Y el registro de eventos tiene exactamente una línea nueva de tipo "<evento de la acción>"

      Ejemplos:
        | código                               | evento de la acción       |
        | audio_sin_consentimiento             | audio_importado           |
        | transcripcion_mas_larga_que_audio    | transcripcion_importada   |
        | reemplazar_transcripcion_con_cambios | transcripcion_reemplazada |
        | revisada_con_segmentos_sin_revisar   | transcripcion_revisada    |
        | eliminar_audio_sin_revisar           | audio_eliminado           |

  @PRY-10
  Regla: Índice reconstruible

    @PRY-10.1 @PRY-06.5 @recorrido-completo
    Escenario: Borrar .osr/ y volver a abrir muestra el mismo estado
      Dado el proyecto de prueba después del recorrido completo de los escenarios del tramo 1
      Y osr está corriendo con el proyecto abierto
      Y se guarda el estado que muestra la app
      Y osr se cierra de forma ordenada
      Y se borra la carpeta ".osr/" con todo su contenido
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando se abre el proyecto con `osr abrir {proyecto} --sin-navegador`
      Entonces la app muestra el mismo estado que el guardado
      Y el registro de eventos tiene exactamente una línea nueva de tipo "indice_reconstruido" cuyo detalle contiene "ausente"

    @PRY-10.2
    Escenario: Un índice dañado se reconstruye con el mismo estado
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y se guarda el estado que muestra la app
      Y osr se cierra de forma ordenada
      Y se reemplaza ".osr/indice.sqlite" por 1024 bytes aleatorios
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando se abre el proyecto con `osr abrir {proyecto} --sin-navegador`
      Entonces la app muestra el mismo estado que el guardado
      Y el registro de eventos tiene exactamente una línea nueva de tipo "indice_reconstruido" cuyo detalle contiene "danado"

    @PRY-10.3 @recorrido-completo
    Escenario: El índice no contiene nombres ni frases de transcripciones
      Dado el proyecto de prueba después del recorrido completo de los escenarios del tramo 1
      Entonces el archivo ".osr/indice.sqlite" no contiene, en UTF-8 en NFC ni en NFD, ninguno de los nombres de la tabla de correspondencia de prueba
      Y el archivo ".osr/indice.sqlite" no contiene, en UTF-8 en NFC ni en NFD, ninguna frase de las transcripciones de prueba

  @PRY-11
  Regla: Verificación al abrir

    @PRY-11.1 @interfaz
    Escenario: Una fila agregada a mano en la correspondencia aparece y se reporta
      # Una fila nueva solo es válida si su seudónimo está registrado (PAR-03), así que la
      # prueba agrega a mano el participante E03 y su fila de correspondencia.
      Dado una copia del proyecto de prueba
      Y con la app cerrada se agrega al final de "participantes/participantes.csv" la fila "E03,2026-10-10,activo"
      Y con la app cerrada se agrega al final de "identificable/correspondencia.csv" la fila "E03,Carmen Inventada Solís,,"
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Cuando la interfaz muestra la vista "Correspondencia"
      Entonces la fila de "E03" muestra "Carmen Inventada Solís" en el campo "nombre"
      Y el evento "verificacion" más reciente tiene "identificable/correspondencia.csv" en la lista "modificados_fuera"

    @PRY-11.2 @interfaz
    Escenario: Un CSV inválido se reporta con su línea y bloquea sus acciones
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y el participante "E03" está registrado
      Y osr se cierra de forma ordenada
      Y con la app cerrada se borra la comilla de cierre de "v1, octubre" en la línea 3 de "participantes/consentimientos.csv"
      Y se tomó la huella de cada archivo del proyecto
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Entonces el resumen muestra un aviso con la ruta "participantes/consentimientos.csv" y la línea 3
      Y el archivo "participantes/consentimientos.csv" es idéntico byte a byte a su versión de las huellas tomadas
      Cuando la interfaz muestra la vista del participante "E03"
      Entonces el botón "Registrar consentimiento" está deshabilitado con el texto "El archivo de consentimientos tiene un error."

    @PRY-11.3 @interfaz
    Escenario: Un audio borrado con la app cerrada figura como faltante
      Dado una copia del proyecto de prueba
      Y con la app cerrada se borra el archivo "identificable/entrevistas/E01/E01.ogg"
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Entonces el evento "verificacion" más reciente tiene "identificable/entrevistas/E01/E01.ogg" en la lista "faltantes"
      Y la entrevista "E01" muestra la etiqueta "Falta el audio"

    @PRY-11.4 @interfaz
    Escenario: Un audio alterado figura como original alterado
      Dado una copia del proyecto de prueba
      Y con la app cerrada se da permiso de escritura a "identificable/entrevistas/E01/E01.ogg" y se cambia su byte 1000 sin cambiar su tamaño
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Entonces el evento "verificacion" más reciente tiene "identificable/entrevistas/E01/E01.ogg" en la lista "originales_alterados"
      Y la entrevista "E01" muestra la etiqueta "El audio cambió desde su importación"

    @PRY-11.5 @interfaz
    Escenario: Los archivos no registrados en la zona identificable se cuentan sin nombrarlos
      Dado una copia del proyecto de prueba
      Y con la app cerrada se copian a "identificable/" los archivos "apuntes sueltos.txt" y "foto de campo.jpg"
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Entonces la interfaz muestra "Hay 2 archivos en la zona identificable que la herramienta no registró."
      Y el evento "verificacion" más reciente tiene "no_registrados" igual a 2
      Y el archivo "bitacora/registro.jsonl" no contiene el texto "apuntes sueltos"
      Y el archivo "bitacora/registro.jsonl" no contiene el texto "foto de campo"

    @PRY-11.6 @interfaz
    Escenario: Un proyecto sin cambios desde el último cierre no tiene diferencias
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y osr se cierra de forma ordenada
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Entonces la interfaz muestra "Verificación al abrir: sin diferencias."
      Y el detalle del evento "verificacion" más reciente es el JSON
        """
        {"modificados_fuera": [], "con_error": [], "faltantes": [], "originales_alterados": [], "no_registrados": 0, "temporales_eliminados": 0}
        """

  @PRY-12
  Regla: Escrituras atómicas y cierres forzados

    @PRY-12.1 @PRY-12.2
    Escenario: 200 cierres forzados al azar no pierden ni corrompen nada
      Dado una copia del proyecto de prueba
      Y una secuencia automática de ediciones de transcripción, altas de participantes, cambios de consentimiento e importaciones
      Cuando se envía SIGKILL al proceso osr en 200 instantes de esa secuencia elegidos al azar con distribución uniforme, reabriendo el proyecto después de cada uno
      Entonces después de cada reapertura el 100 % de los archivos de texto gestionados es válido según su formato
      Y después de cada reapertura cada archivo gestionado es idéntico a alguna versión completa que la app escribió
      Y después de cada reapertura no queda ningún archivo cuyo nombre contenga ".osr-tmp-"
      Y después de cada reapertura está toda edición del editor hecha 2 s o más antes del SIGKILL
      Y después de cada reapertura está todo formulario que mostró "Guardado" antes del SIGKILL
      Y la prueba termina con cero fallos en las 200 repeticiones

  @PRY-13
  Regla: Guardado automático y estado del guardado

    @PRY-13.1 @interfaz
    Escenario: Una letra escrita llega al disco en 1 s o menos
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y la transcripción de E01 está abierta en el editor
      Cuando la persona escribe "x" al final del texto del segmento "s0001"
      Entonces a más tardar 1 s después el segmento "s0001" de "identificable/entrevistas/E01/transcripcion.json" tiene un texto que termina en "x"

    @PRY-13.2 @interfaz @PRY-18.1
    Escenario: Con el disco lleno se muestra la falla y se reintenta hasta guardar
      Dado una copia del proyecto de prueba en un disco de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y la transcripción de E01 está abierta en el editor
      Y el disco de prueba está lleno
      Cuando la persona escribe "x" al final del texto del segmento "s0001"
      Entonces la interfaz muestra un texto que empieza con "No se pudo guardar: " y termina con ". Se reintenta en 5 s."
      Cuando se libera espacio en el disco de prueba
      Entonces a más tardar 10 s después la interfaz muestra "Guardado"
      Y el segmento "s0001" de "identificable/entrevistas/E01/transcripcion.json" tiene un texto que termina en "x"

    @PRY-13.3 @interfaz
    Escenario: En el editor Guardado aparece solo cuando el cambio ya está en el disco
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y cada escritura del proyecto en el disco de prueba tarda 500 ms
      Y la transcripción de E01 está abierta en el editor
      Cuando la persona escribe "x" al final del texto del segmento "s0001"
      Entonces en el instante en que la interfaz muestra "Guardado" el segmento "s0001" de "identificable/entrevistas/E01/transcripcion.json" ya tiene un texto que termina en "x"

    @PRY-13.3 @interfaz
    Escenario: En un formulario Guardado aparece solo cuando el cambio ya está en el disco
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y cada escritura del proyecto en el disco de prueba tarda 500 ms
      Y la interfaz muestra la vista "Correspondencia"
      Cuando la persona escribe "Contacto inventado 0001" en el campo "contacto" de "E01"
      Y la persona pulsa "Guardar"
      Entonces en el instante en que la interfaz muestra "Guardado" el archivo "identificable/correspondencia.csv" ya contiene el texto "Contacto inventado 0001"

  @PRY-14
  Regla: Cambios hechos fuera de la app

    @PRY-14.1 @interfaz @PRY-18.1
    Escenario: La app no pisa un archivo que cambió fuera de ella
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Cuando otro programa agrega al final de "participantes/participantes.csv" la fila "E09,2026-10-10,activo"
      Y la persona pulsa "Agregar participante" y acepta la propuesta "E03"
      Entonces el archivo "participantes/participantes.csv" contiene la fila "E09,2026-10-10,activo"
      Y el archivo "participantes/participantes.csv" no contiene el texto "E03"
      Y la interfaz muestra "El archivo participantes/participantes.csv cambió fuera de la herramienta mientras estaba abierta. No se guardó el cambio para no pisarlo."
      Y la interfaz muestra los botones "Recargar desde el disco" y "Conservar mi cambio"

    @PRY-14.2 @interfaz
    Escenario: Conservar mi cambio guarda aparte la versión externa
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y otro programa agrega al final de "participantes/participantes.csv" la fila "E09,2026-10-10,activo"
      Y se guardó una copia de "participantes/participantes.csv" como versión externa
      Y la persona pulsa "Agregar participante" y acepta la propuesta "E03"
      Cuando la persona pulsa "Conservar mi cambio"
      Entonces existe el archivo "participantes/participantes.conflicto-{marca}.csv"
      Y el archivo "participantes/participantes.conflicto-{marca}.csv" es idéntico byte a byte a la versión externa
      Y el archivo "participantes/participantes.csv" contiene una fila que empieza con "E03,"
      Y el registro de eventos tiene exactamente una línea nueva de tipo "conflicto_resuelto" cuyo detalle contiene "participantes/participantes.csv"

    @PRY-14.3 @interfaz
    Escenario: Recargar desde el disco deja la versión externa
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y otro programa agrega al final de "participantes/participantes.csv" la fila "E09,2026-10-10,activo"
      Y se guardó una copia de "participantes/participantes.csv" como versión externa
      Y la persona pulsa "Agregar participante" y acepta la propuesta "E03"
      Cuando la persona pulsa "Recargar desde el disco"
      Entonces el archivo "participantes/participantes.csv" es idéntico byte a byte a la versión externa
      Y la lista de participantes de la interfaz muestra "E09"
      Y la lista de participantes de la interfaz no muestra "E03"
      Y el registro de eventos tiene exactamente una línea nueva de tipo "conflicto_resuelto" cuyo detalle contiene "participantes/participantes.csv"

  @PRY-15
  Regla: Originales inmutables

    @PRY-15.1
    Escenario: La huella del original es igual a la del archivo de origen
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y el participante "E01" tiene consentimiento registrado con grabar "otorgado"
      Y la entrevista de "E01" está creada
      Cuando se importa en la entrevista "E01" el audio de prueba "E01.ogg"
      Entonces la huella de "identificable/entrevistas/E01/E01.ogg" es igual a la de "E01.ogg" de los datos de prueba

    @PRY-15.2
    Esquema del escenario: El original queda en 0400 con cualquier umask
      Dado la umask del proceso osr es <umask>
      Y un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y el participante "E01" tiene consentimiento registrado con grabar "otorgado"
      Y la entrevista de "E01" está creada
      Cuando se importa en la entrevista "E01" el audio de prueba "E01.ogg"
      Entonces los permisos de "identificable/entrevistas/E01/E01.ogg" son 0400

      Ejemplos:
        | umask |
        | 000   |
        | 022   |
        | 077   |

    @PRY-15.3 @interfaz @PRY-18.1
    Escenario: Una copia que no coincide no deja rastro
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y el participante "E01" tiene consentimiento registrado con grabar "otorgado"
      Y la entrevista de "E01" está creada
      Y la interfaz está abierta en el navegador
      Y la prueba altera un byte de la copia del audio en el disco durante la importación
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando la persona importa el archivo "E01.ogg" con "Importar audio" en la entrevista "E01"
      Entonces no existe "identificable/entrevistas/E01/E01.ogg"
      Y no existe "identificable/entrevistas/E01/E01.ogg.ficha.toml"
      Y el registro de eventos no recibió líneas nuevas
      Y la interfaz muestra "La copia de E01.ogg no coincide con el archivo recibido. No se importó; hay que intentarlo de nuevo."

  @PRY-16
  Regla: README generado

    @PRY-16.1 @recorrido-completo
    Escenario: El README describe cada carpeta y cada patrón de archivo presente
      Dado el proyecto de prueba después del recorrido completo de los escenarios del tramo 1
      Entonces la primera línea de "README.md" es "<!-- Generado por Open Social Research. Los cambios a mano se guardan aparte y este archivo se reemplaza. -->"
      Y la sección «Carpetas» de "README.md" tiene una fila por cada carpeta presente en el proyecto
      Y la sección «Tipos de archivo» de "README.md" tiene una fila por cada patrón de archivo gestionado presente en el proyecto

    @PRY-16.2
    Escenario: Al importar el primer audio el README suma sus patrones y su sensibilidad
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y el participante "E01" tiene consentimiento registrado con grabar "otorgado"
      Y la entrevista de "E01" está creada
      Cuando se importa en la entrevista "E01" el audio de prueba "E01.ogg"
      Entonces la sección «Tipos de archivo» de "README.md" tiene una fila para el patrón del audio de entrevista
      Y la sección «Tipos de archivo» de "README.md" tiene una fila para el patrón "*.ficha.toml"
      Y la tabla de sensibilidad de "README.md" tiene la fila de "identificable/entrevistas/E01/E01.ogg" con nivel "identificable"

    @PRY-16.3
    Escenario: El README trae las órdenes de respaldo
      Dado un proyecto nuevo creado con `osr nuevo`
      Entonces la sección «Cómo respaldar» de "README.md" tiene una orden que contiene "rsync -a"
      Y la sección «Cómo respaldar» de "README.md" tiene otra orden que copia todo salvo "identificable/"

    @PRY-16.4 @recorrido-completo
    Escenario: El README no tiene nombres ni frases de transcripciones
      Dado el proyecto de prueba después del recorrido completo de los escenarios del tramo 1
      Entonces el archivo "README.md" no contiene ninguno de los nombres de la tabla de correspondencia de prueba
      Y el archivo "README.md" no contiene ninguna frase de las transcripciones de prueba

    @PRY-16.5
    Escenario: Un README editado a mano se guarda aparte y se regenera
      Dado una copia del proyecto de prueba
      Y con la app cerrada se agrega al final de "README.md" la línea "Nota escrita a mano"
      Y se guardó una copia de "README.md" como versión editada
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando se abre el proyecto con `osr abrir {proyecto} --sin-navegador`
      Entonces existe el archivo "README.modificado-{marca}.md"
      Y el archivo "README.modificado-{marca}.md" es idéntico byte a byte a la versión editada
      Y el archivo "README.md" no contiene el texto "Nota escrita a mano"
      Y la primera línea de "README.md" es "<!-- Generado por Open Social Research. Los cambios a mano se guardan aparte y este archivo se reemplaza. -->"
      Y el registro de eventos tiene exactamente una línea nueva de tipo "readme_modificado_guardado"

  @PRY-17
  Regla: Datos de prueba inventados

    @PRY-17.1 @manual
    Escenario: Cada escenario usa solo datos de prueba o archivos generados por la prueba
      # Es una propiedad de la suite misma. Una persona revisa los escenarios y las
      # definiciones de pasos y comprueba que ningún archivo viene de fuera de los datos
      # de prueba ni de lo que la prueba genera.
      Dado los escenarios del tramo 1 y sus definiciones de pasos
      Entonces cada archivo que leen está en los datos de prueba o lo generó la propia prueba

    @PRY-17.2
    Escenario: Los datos de prueba incluyen el archivo LEEME
      Dado los datos de prueba del repositorio
      Entonces existe el archivo "LEEME" en la raíz de los datos de prueba

    @PRY-17.2 @manual
    Escenario: El LEEME declara que todo es inventado y la licencia del audio
      # La especificación no fija el texto del LEEME, así que una persona lo lee y
      # confirma las dos declaraciones.
      Dado los datos de prueba del repositorio
      Entonces el archivo "LEEME" declara que todo su contenido es inventado
      Y el archivo "LEEME" declara la licencia del audio

  @PRY-18
  Regla: Idioma de la interfaz y de los mensajes

    @PRY-18.1 @manual
    Escenario: Cada mensaje citado en las especificaciones tiene un escenario que lo comprueba
      # Los escenarios etiquetados @PRY-18.1 y los criterios con mensaje comprueban cada
      # texto en su situación. Separar los mensajes de los demás textos entre comillas
      # invertidas, como claves o nombres de archivo, pide criterio humano, por eso esta
      # revisión de la lista es manual.
      Dado la lista de textos entre comillas invertidas de proyecto.md, servidor.md, participantes.md y transcripciones.md
      Entonces cada texto que es un mensaje visible aparece textual en al menos un escenario de su situación

    @PRY-18.2 @interfaz
    Escenario: Un error de permisos del sistema se muestra en español sin detalle técnico
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y la carpeta "participantes/" queda con permisos 0500 sin que la app lo sepa
      Cuando la persona pulsa "Agregar participante" y acepta la propuesta "E03"
      Entonces el texto visible de la interfaz no contiene ninguno de estos textos
        | texto       |
        | Permission  |
        | os error    |
        | errno       |
        | panicked    |
        | .rs:        |
        | backtrace   |
      Y la interfaz muestra un texto que empieza con "No se pudo guardar: "

  @PRY-19
  Regla: Crear una investigación desde la pantalla
    # «se pide a la API» manda la acción con el token válido en el encabezado
    # Authorization y "Origin" con el origen propio, como la pestaña.
    # {aplicación} es la carpeta de la aplicación de PRY-24 en la carpeta personal
    # que usa osr en el escenario, nunca la del equipo.

    @PRY-19.1
    Escenario: Crear con solo el nombre deja la investigación creada y abierta
      Dado osr está corriendo sin proyecto
      Cuando se pide a la API crear la investigación "Estudio de prueba"
      Entonces la respuesta tiene estado 200
      Y el cuerpo de la respuesta es el JSON
        """
        {"nombre": "Estudio de prueba", "carpeta": "{aplicación}/estudio-de-prueba"}
        """
      Y la carpeta "{aplicación}/estudio-de-prueba" tiene exactamente estas rutas
        | ruta                              |
        | proyecto.toml                     |
        | README.md                         |
        | sensibilidad.csv                  |
        | bitacora/                         |
        | bitacora/bitacora.md              |
        | bitacora/registro.jsonl           |
        | participantes/                    |
        | participantes/participantes.csv   |
        | participantes/consentimientos.csv |
        | identificable/                    |
        | identificable/correspondencia.csv |
        | identificable/consentimientos/    |
        | identificable/entrevistas/        |
        | .osr/                             |
        | .osr/indice.sqlite                |
        | .osr/bloqueo                      |
      Y osr está corriendo con el proyecto abierto
      Y el archivo "bitacora/registro.jsonl" tiene exactamente 1 línea
      Y la última línea del registro de eventos es de tipo "proyecto_creado"
      Y la página "/" de la sesión contiene "Estudio de prueba"

    @PRY-19.2
    Escenario: Los espacios alrededor del nombre no cuentan
      Dado osr está corriendo sin proyecto
      Cuando se pide a la API crear la investigación "  Estudio  "
      Entonces la respuesta tiene estado 200
      Y la clave "nombre" de "{aplicación}/estudio/proyecto.toml" vale "Estudio"

    @PRY-19.3 @PRY-18.1
    Esquema del escenario: Un nombre que no sirve no crea nada
      Dado osr está corriendo sin proyecto
      Y se tomó la huella de cada archivo de "{aplicación}"
      Cuando se pide a la API crear la investigación "<nombre>"
      Entonces la respuesta tiene estado 422
      Y el cuerpo de la respuesta es el JSON
        """
        {"error": "<error>", "mensaje": "<mensaje>"}
        """
      Y ningún archivo de "{aplicación}" cambió respecto de las huellas tomadas
      Y la página "/" de la sesión no contiene "Proyecto nuevo"

      Ejemplos:
        | nombre                       | error           | mensaje                                                                               |
        |                              | nombre_vacio    | Hay que escribir el nombre de la investigación.                                       |
        | {201 veces la letra a}       | nombre_invalido | El nombre del proyecto tiene que tener entre 1 y 200 caracteres, sin saltos de línea. |
        | {Uno, salto de línea, Dos}   | nombre_invalido | El nombre del proyecto tiene que tener entre 1 y 200 caracteres, sin saltos de línea. |

    @PRY-19.3 @PRY-18.1
    Escenario: Un cuerpo que no es JSON no crea nada
      Dado osr está corriendo sin proyecto
      Y se tomó la huella de cada archivo de "{aplicación}"
      Cuando se manda a la API de crear un proyecto el cuerpo "no es json"
      Entonces la respuesta tiene estado 400
      Y el cuerpo de la respuesta es el JSON
        """
        {"error": "pedido_invalido", "mensaje": "El pedido no tiene la forma esperada."}
        """
      Y ningún archivo de "{aplicación}" cambió respecto de las huellas tomadas

    @PRY-19.4 @interfaz
    Escenario: Crear desde la pantalla muestra el resumen
      Dado la carpeta de la aplicación tiene una copia del proyecto de prueba en "previa"
      Y osr está corriendo sin proyecto
      Y la interfaz está abierta en el navegador
      Cuando la persona escribe "Estudio de prueba" en el campo "Nombre de la investigación"
      Y la persona pulsa "Crear investigación"
      Entonces a más tardar 3 s después la interfaz muestra "Estudio de prueba"
      Y la interfaz muestra "Proyecto nuevo"

    @PRY-19.5 @interfaz @PRY-18.1
    Escenario: Un nombre vacío se avisa debajo del botón
      Dado la carpeta de la aplicación tiene una copia del proyecto de prueba en "previa"
      Y osr está corriendo sin proyecto
      Y la interfaz está abierta en el navegador
      Cuando la persona pulsa "Crear investigación"
      Entonces la interfaz muestra "Hay que escribir el nombre de la investigación."
      Y el campo "Nombre de la investigación" está marcado como inválido

  @PRY-20
  Regla: Abrir una investigación de la lista

    @PRY-20.1
    Escenario: La pantalla lista solo las investigaciones, en orden alfabético
      Dado la carpeta de la aplicación tiene una copia del proyecto de prueba en "proyecto-de-prueba"
      Y la carpeta de la aplicación tiene un proyecto nuevo "Caminar de noche" en "caminar-de-noche"
      Y existe la carpeta vacía "{aplicación}/sin-proyecto"
      Y existe el archivo "{aplicación}/notas.txt" con el texto "notas"
      Y osr está corriendo sin proyecto
      Entonces la página "/" de la sesión contiene "Investigaciones"
      Y la página "/" de la sesión lista estas investigaciones en este orden
        | nombre                       | carpeta            |
        | Caminar de noche             | caminar-de-noche   |
        | Proyecto de prueba inventado | proyecto-de-prueba |
      Y la página "/" de la sesión no contiene "sin-proyecto"
      Y la página "/" de la sesión no contiene "notas.txt"

    @PRY-20.2
    Escenario: Abrir una investigación de la lista la deja abierta
      Dado la carpeta de la aplicación tiene una copia del proyecto de prueba en "proyecto-de-prueba"
      Y osr está corriendo sin proyecto
      Y {proyecto} es "{aplicación}/proyecto-de-prueba"
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando se pide a la API abrir la investigación "proyecto-de-prueba"
      Entonces la respuesta tiene estado 200
      Y el cuerpo de la respuesta es el JSON
        """
        {"nombre": "Proyecto de prueba inventado", "carpeta": "{aplicación}/proyecto-de-prueba"}
        """
      Y osr está corriendo con el proyecto abierto
      Y el registro de eventos tiene exactamente una línea nueva de tipo "verificacion"
      Y la página "/" de la sesión contiene "Proyecto de prueba inventado"
      Y la página "/" de la sesión contiene "Proyecto abierto"

    @PRY-20.3 @PRY-18.1
    Esquema del escenario: Una carpeta que no se puede abrir no se toca
      Dado la carpeta de la aplicación tiene una copia del proyecto de prueba con version_formato 2 en "version-2"
      Y osr está corriendo sin proyecto
      Y se tomó la huella de cada archivo de "{aplicación}"
      Cuando se pide a la API abrir la investigación "<carpeta>"
      Entonces la respuesta tiene estado <estado>
      Y el cuerpo de la respuesta es el JSON
        """
        {"error": "<error>", "mensaje": "<mensaje>"}
        """
      Y ningún archivo de "{aplicación}" cambió respecto de las huellas tomadas
      Y la página "/" de la sesión contiene "Investigaciones"

      Ejemplos:
        | carpeta   | estado | error             | mensaje                                                                                  |
        | ../otra   | 400    | pedido_invalido   | El pedido no tiene la forma esperada.                                                    |
        | a/b       | 400    | pedido_invalido   | El pedido no tiene la forma esperada.                                                    |
        |           | 400    | pedido_invalido   | El pedido no tiene la forma esperada.                                                    |
        | no-existe | 422    | sin_carpeta       | No existe la carpeta {aplicación}/no-existe.                                             |
        | version-2 | 422    | proyecto_invalido | Este proyecto usa la versión 2 del formato y esta versión de osr solo abre la versión 1. |

    @PRY-20.4 @PRY-18.1
    Escenario: Una investigación abierta en otro proceso no se abre
      Dado la carpeta de la aplicación tiene una copia del proyecto de prueba en "proyecto-de-prueba"
      Y {proyecto} es "{aplicación}/proyecto-de-prueba"
      Y osr está corriendo con el proyecto abierto
      Y osr está corriendo sin proyecto
      Cuando se pide a la API abrir la investigación "proyecto-de-prueba"
      Entonces la respuesta tiene estado 409
      Y el cuerpo de la respuesta es el JSON
        """
        {"error": "abierto_en_otro_proceso", "mensaje": "El proyecto ya está abierto en otro proceso (PID {PID del primer proceso}). Hay que usar esa pestaña o cerrar ese proceso."}
        """
      Y el primer proceso osr sigue atendiendo pedidos autenticados

    @PRY-20.5 @interfaz
    Escenario: Abrir desde la lista muestra el resumen
      Dado la carpeta de la aplicación tiene una copia del proyecto de prueba en "proyecto-de-prueba"
      Y osr está corriendo sin proyecto
      Y la interfaz está abierta en el navegador
      Cuando la persona pulsa "Abrir" en la fila de "Proyecto de prueba inventado"
      Entonces a más tardar 3 s después la interfaz muestra "Proyecto abierto"

  @PRY-21
  Regla: Un proyecto por proceso

    @PRY-21.1 @PRY-18.1
    Escenario: Con una investigación creada, crear otra no hace nada
      Dado osr está corriendo sin proyecto
      Y se pidió a la API crear la investigación "Uno"
      Cuando se pide a la API crear la investigación "Dos"
      Entonces la respuesta tiene estado 409
      Y el cuerpo de la respuesta es el JSON
        """
        {"error": "proyecto_ya_abierto", "mensaje": "Ya hay un proyecto abierto en esta sesión. Para abrir otro hay que cerrar Open Social Research y volver a abrirlo."}
        """
      Y no existe "{aplicación}/dos"
      Y osr está corriendo con el proyecto abierto
      Y la página "/" de la sesión contiene "Uno"

    @PRY-21.2
    Escenario: Con un proyecto abierto con osr abrir, abrir otro no escribe nada
      Dado una copia del proyecto de prueba
      Y la carpeta de la aplicación tiene un proyecto nuevo "Otro" en "otro"
      Y osr está corriendo con el proyecto abierto
      Y se tomó la huella de cada archivo de "{tmp}"
      Y se tomó la huella de cada archivo de "{aplicación}" como segunda huella
      Cuando se pide a la API abrir la investigación "otro"
      Entonces la respuesta tiene estado 409
      Y ningún archivo de "{tmp}" cambió respecto de las huellas tomadas
      Y ningún archivo de "{aplicación}" cambió respecto de la segunda huella

    @PRY-21.3
    Escenario: De diez pedidos de crear a la vez se atiende uno solo
      Dado osr está corriendo sin proyecto
      Cuando se piden a la API a la vez 10 investigaciones con los nombres "Estudio 1" a "Estudio 10"
      Entonces exactamente 1 respuesta tiene estado 200 y 9 tienen estado 409
      Y la carpeta de la aplicación tiene exactamente 1 investigación

    @PRY-21.4
    Escenario: Una investigación abierta desde la pantalla se cierra en orden
      Dado la carpeta de la aplicación tiene una copia del proyecto de prueba en "proyecto-de-prueba"
      Y osr está corriendo sin proyecto
      Y se pidió a la API abrir la investigación "proyecto-de-prueba"
      Cuando se envía SIGTERM al proceso osr
      Entonces osr termina con código de salida 0 a más tardar 3 s después
      Y no existe ".osr/bloqueo"

  @PRY-22
  Regla: Resumen del proyecto en el tramo 1

    @PRY-22.1
    Escenario: Un proyecto vacío empieza por el diseño de la investigación
      Dado un proyecto nuevo creado con `osr nuevo`
      Entonces la página "/" de la sesión contiene estos textos
        | texto                                                                                                                                             |
        | Proyecto nuevo                                                                                                                                    |
        | {proyecto}                                                                                                                                        |
        | La carpeta del proyecto está lista. Lo primero es el diseño de la investigación.                                                                  |
        | Antes del trabajo de campo se escriben el tema, el posicionamiento, el problema, la pregunta y el enfoque. Van en espiral: se puede volver a cualquier paso cuando haga falta. |
        | Diseño de la investigación                                                                                                                        |
        | Tema e interés                                                                                                                                    |
        | Posicionamiento y supuestos                                                                                                                       |
        | Problema y estado de la cuestión                                                                                                                  |
        | Pregunta y objetivos                                                                                                                              |
        | Enfoque, metodología y métodos                                                                                                                    |
        | Empezar el diseño                                                                                                                                 |
        | Trabajo de campo                                                                                                                                  |
        | Registrar participantes                                                                                                                           |
        | La herramienta genera el seudónimo; el nombre real queda en la Zona identificable.                                                                |
        | Registrar su consentimiento                                                                                                                       |
        | Versión del formulario, modalidad, respaldo y alcances.                                                                                           |
        | Importar las entrevistas                                                                                                                          |
        | Audio y archivos de Whisper; después, revisar la transcripción.                                                                                   |
        | Agregar participante                                                                                                                              |
        | Se puede empezar en cualquier momento; el diseño se puede revisar después.                                                                        |
        | Todavía no se hace desde aquí.                                                                                                                    |
      Y la página "/" de la sesión tiene el título "estudio · Open Social Research"
      Y la página "/" de la sesión no contiene "Sin proyecto abierto"

    @PRY-22.2
    Escenario: Un proyecto con datos avisa que todavía no los muestra
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Entonces la página "/" de la sesión contiene "Proyecto abierto"
      Y la página "/" de la sesión contiene "Este proyecto ya tiene participantes o entrevistas. Esta versión todavía no los muestra aquí; están en la carpeta del proyecto."
      Y la página "/" de la sesión no contiene "Diseño de la investigación"

    @PRY-22.3
    Escenario: El resumen no muestra nombres de la correspondencia
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Entonces la página "/" de la sesión no contiene ninguno de los nombres de la tabla de correspondencia de prueba

    @PRY-22.4
    Escenario: El nombre del proyecto se muestra como texto
      Dado osr está corriendo sin proyecto
      Y se pidió a la API crear la investigación "<b>Estudio</b> & otro"
      Entonces la página "/" de la sesión tiene el título "<b>Estudio</b> & otro · Open Social Research"
      Y la página "/" de la sesión no tiene ningún elemento "b"

  @PRY-23
  Regla: Bienvenida del primer uso

    @PRY-23.1
    Escenario: Sin investigaciones se ve la bienvenida
      Dado osr está corriendo sin proyecto
      Entonces la página "/" de la sesión contiene estos textos
        | texto                                                                                         |
        | Open Social Research                                                                          |
        | Investigación social aplicada de punta a punta, desde la primera pregunta hasta la publicación. |
        | Libre                                                                                         |
        | Código abierto con licencia MIT, sin licencias que pagar.                                     |
        | Local                                                                                         |
        | Todo queda en esta computadora, sin cuentas ni conexión a internet.                           |
        | Pluralista                                                                                    |
        | Sirve igual para investigación cualitativa, cuantitativa o mixta.                             |
        | Iniciar                                                                                       |
        | Nace en el programa de Investigación Social Aplicada de la UNED.                              |
        | Primera investigación                                                                         |
        | Para empezar, hay que ponerle nombre a la investigación.                                      |
        | Todo lo que se registre queda en la carpeta Open Social Research de esta computadora, en archivos de texto abierto que se pueden leer sin la herramienta. |
        | La carpeta de la investigación lleva este nombre, sin tildes ni espacios.                     |
      Y la página "/" de la sesión no contiene "Investigaciones"

    @PRY-23.2
    Escenario: Después de crear la primera investigación ya no sale la bienvenida
      Dado osr está corriendo sin proyecto
      Y se pidió a la API crear la investigación "Estudio"
      Y osr se cierra de forma ordenada
      Cuando osr está corriendo sin proyecto
      Entonces la página "/" de la sesión contiene "Investigaciones"
      Y la página "/" de la sesión no contiene "Investigación social aplicada de punta a punta"

    @PRY-23.3
    Escenario: Un error al crear no deja nada y sigue la bienvenida
      Dado osr está corriendo sin proyecto
      Cuando se pide a la API crear la investigación ""
      Entonces la respuesta tiene estado 422
      Y la carpeta "{aplicación}" está vacía
      Y la página "/" de la sesión contiene "Iniciar"

    @PRY-23.4 @interfaz
    Escenario: Iniciar lleva a la primera investigación con el foco en el nombre
      Dado osr está corriendo sin proyecto
      Y la interfaz está abierta en el navegador
      Entonces el botón con el foco es "Iniciar"
      Cuando la persona pulsa "Iniciar"
      Entonces la interfaz muestra "Para empezar, hay que ponerle nombre a la investigación."
      Y el campo con el foco es "Nombre de la investigación"

    @PRY-23.5 @interfaz
    Escenario: Crear desde la primera investigación muestra el resumen
      Dado osr está corriendo sin proyecto
      Y la interfaz está abierta en el navegador
      Cuando la persona pulsa "Iniciar"
      Y la persona escribe "Estudio" en el campo "Nombre de la investigación"
      Y la persona pulsa "Crear investigación"
      Entonces a más tardar 3 s después la interfaz muestra "Proyecto nuevo"

  @PRY-24
  Regla: Carpeta de la aplicación

    @PRY-24.1
    Escenario: osr crea la carpeta de la aplicación al arrancar
      Dado osr está corriendo sin proyecto
      Entonces los permisos de "{aplicación}" son 0700

    @PRY-24.2
    Esquema del escenario: La subcarpeta es el nombre simplificado
      Dado osr está corriendo sin proyecto
      Cuando se pide a la API crear la investigación "<nombre>"
      Entonces la respuesta tiene estado 200
      Y existe el archivo "{aplicación}/<subcarpeta>/proyecto.toml"

      Ejemplos:
        | nombre                            | subcarpeta                        |
        | Caminar en el centro de San José  | caminar-en-el-centro-de-san-jose  |
        | Año 2026: ¿por qué?               | ano-2026-por-que                  |
        | !!!                               | investigacion                     |
        | {120 veces la letra a}            | {60 veces la letra a}             |

    @PRY-24.3
    Escenario: Un nombre repetido lleva -2
      Dado osr está corriendo sin proyecto
      Y se pidió a la API crear la investigación "Estudio"
      Y osr se cierra de forma ordenada
      Y osr está corriendo sin proyecto
      Cuando se pide a la API crear la investigación "Estudio"
      Entonces la respuesta tiene estado 200
      Y existe el archivo "{aplicación}/estudio/proyecto.toml"
      Y existe el archivo "{aplicación}/estudio-2/proyecto.toml"
