# language: es
#
# Escenarios del módulo participantes (PAR), tramo 1 de la v0.1.
# Fuente especificaciones/participantes.md.
#
# Las convenciones, las etiquetas y los datos de prueba están al principio de proyecto.feature.
#
# Pasos de preparación de este archivo
# - «el participante "E01" está registrado» lo agrega con la acción Agregar participante.
# - «tiene consentimiento registrado con grabar "<valor>"» registra versión v1, fecha 2026-10-01,
#   modalidad escrito, respaldo ninguno, grabar con ese valor y los demás alcances en no_consultado.
# - En el proyecto de prueba el contacto de E01 es "ana.munoz@ejemplo.invalid" y su nota
#   "Prefiere conversar por la tarde", los dos inventados.

Característica: Participantes y consentimiento
  Cada persona entra con un seudónimo, su nombre vive en un único archivo de la zona
  identificable, su consentimiento queda registrado por alcance y cada archivo muestra
  su zona y su sensibilidad, sin ninguna vía para sacarlo de la máquina.

  @PAR-01
  Regla: Seudónimos

    @PAR-01.1 @interfaz
    Escenario: La propuesta empieza en E01 y sigue en E02
      Dado un proyecto nuevo creado con `osr nuevo`
      Y la interfaz está abierta en el navegador
      Cuando la persona pulsa "Agregar participante"
      Entonces la propuesta de seudónimo es "E01"
      Cuando la persona pulsa "Guardar"
      Y la persona pulsa "Agregar participante"
      Entonces la propuesta de seudónimo es "E02"

    @PAR-01.2 @interfaz
    Escenario: La propuesta sigue al mayor número usado
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y el participante "E07" está registrado
      Y la interfaz está abierta en el navegador
      Cuando la persona pulsa "Agregar participante"
      Entonces la propuesta de seudónimo es "E08"

    @PAR-01.3 @interfaz
    Escenario: Después de E99 la propuesta es E100
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E99" está registrado
      Y la interfaz está abierta en el navegador
      Cuando la persona pulsa "Agregar participante"
      Entonces la propuesta de seudónimo es "E100"

    @PAR-01.4 @interfaz @PRY-18.1
    Escenario: Un seudónimo eliminado queda reservado
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y el participante "E02" está registrado
      Y la interfaz está abierta en el navegador
      Y se elimina el participante "E02" confirmando con "E02"
      Cuando la persona pulsa "Agregar participante"
      Entonces la propuesta de seudónimo es "E03"
      Cuando la persona cambia la propuesta por "E02"
      Y la persona pulsa "Guardar"
      Entonces la interfaz muestra "El seudónimo E02 ya se usó en este proyecto y no se puede repetir."
      Y el archivo "participantes/participantes.csv" no contiene una fila que empiece con "E03,"

    @PAR-01.5 @interfaz @PRY-18.1
    Esquema del escenario: Un seudónimo con formato inválido no registra nada
      Dado un proyecto nuevo creado con `osr nuevo`
      Y la interfaz está abierta en el navegador
      Y se tomó la huella de cada archivo del proyecto
      Cuando la persona pulsa "Agregar participante"
      Y la persona cambia la propuesta por "<seudónimo>"
      Y la persona pulsa "Guardar"
      Entonces la interfaz muestra "El seudónimo tiene que empezar con una letra mayúscula y seguir con 1 a 15 letras mayúsculas o números, sin tildes ni espacios."
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas

      Ejemplos:
        | seudónimo |
        | e01       |
        | E 01      |
        | É01       |
        | E         |

  @PAR-02
  Regla: Registro de participantes

    @PAR-02.1
    Escenario: Agregar E01 escribe su fila en los dos archivos
      Dado un proyecto nuevo creado con `osr nuevo`
      Cuando el participante "E01" está registrado
      Entonces el archivo "participantes/participantes.csv" tiene exactamente estas filas, con fin de línea CRLF
        """
        seudonimo,fecha_alta,estado
        E01,{hoy},activo
        """
      Y el archivo "identificable/correspondencia.csv" tiene exactamente estas filas, con fin de línea CRLF
        """
        seudonimo,nombre,contacto,nota
        E01,,,
        """

    @PAR-02.2 @interfaz
    Escenario: La lista del resumen muestra 500 participantes
      Dado un proyecto nuevo creado con `osr nuevo`
      Y 500 participantes están registrados con las propuestas automáticas, de "E01" a "E500"
      Y la interfaz está abierta en el navegador
      Cuando la interfaz muestra la vista "Resumen del proyecto"
      Entonces la lista de participantes de la interfaz tiene 500 elementos, de "E01" a "E500", en el orden del archivo

    @PAR-02.3 @interfaz @PRY-18.1
    Escenario: Un seudónimo repetido a mano marca el archivo con error en la segunda aparición
      # Se lee «el número de la segunda línea» como la línea donde el seudónimo aparece por
      # segunda vez.
      Dado una copia del proyecto de prueba
      Y con la app cerrada se agrega al final de "participantes/participantes.csv" la fila "E01,2026-10-10,activo"
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Entonces el resumen muestra un aviso con la ruta "participantes/participantes.csv" y la línea 4
      Y el evento "verificacion" más reciente tiene "participantes/participantes.csv" en la lista "con_error"
      Y el botón "Agregar participante" está deshabilitado con el texto "El archivo de participantes tiene un error."

  @PAR-03
  Regla: Tabla de correspondencia

    @PAR-03.1 @interfaz
    Escenario: Guardar un nombre lo deja solo en la tabla y anota el campo sin su valor
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y la interfaz está abierta en el navegador
      Y la interfaz muestra la vista "Correspondencia"
      Cuando la persona escribe "Ana Inventada Muñoz" en el campo "nombre" de "E01"
      Y la persona pulsa "Guardar"
      Entonces el archivo "identificable/correspondencia.csv" contiene la fila "E01,Ana Inventada Muñoz,,"
      Y la última línea del registro de eventos es de tipo "correspondencia_modificada" con "E01" en su detalle
      Y el detalle de la última línea del registro de eventos contiene el JSON
        """
        ["nombre"]
        """
      Y el archivo "bitacora/registro.jsonl" no contiene el texto "Ana Inventada Muñoz"

    @PAR-03.2 @interfaz @PRY-18.1
    Esquema del escenario: Un campo que pasa su límite no se guarda
      # La especificación escribe el límite de la nota como 2.000, y el mensaje se escribe
      # con el mismo separador de miles que usa TRA-19.
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y la interfaz está abierta en el navegador
      Y la interfaz muestra la vista "Correspondencia"
      Y se tomó la huella de cada archivo del proyecto
      Cuando la persona escribe "<valor>" en el campo "<campo>" de "E01"
      Y la persona pulsa "Guardar"
      Entonces la interfaz muestra "<mensaje>"
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas

      Ejemplos:
        | campo    | valor                               | mensaje                                                  |
        | nombre   | {201 veces la letra a}              | nombre admite hasta 200 caracteres, sin saltos de línea. |
        | nombre   | {Ana, salto de línea, Muñoz}        | nombre admite hasta 200 caracteres, sin saltos de línea. |
        | contacto | {201 veces la letra a}              | contacto admite hasta 200 caracteres, sin saltos de línea. |
        | nota     | {2001 veces la letra a}             | nota admite hasta 2.000 caracteres.                      |

    @PAR-03.3 @interfaz
    Escenario: Solo la vista Correspondencia muestra nombre, contacto y nota
      # Los nombres del primer conjunto también aparecen dichos en las transcripciones y
      # encabezados, así que en las vistas que muestran ese texto solo se buscan el
      # contacto y la nota, como hace PAR-04.4 con las respuestas.
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Cuando la persona recorre estas vistas
        | vista                                    | muestra texto de transcripciones |
        | Resumen del proyecto                     | no                               |
        | Archivos del proyecto                    | no                               |
        | vista del participante E01               | no                               |
        | formulario Modificar consentimiento de E01 | no                             |
        | vista de la entrevista E01               | sí                               |
        | editor de la transcripción de E01        | sí                               |
      Entonces ninguna vista muestra "ana.munoz@ejemplo.invalid"
      Y ninguna vista muestra "Prefiere conversar por la tarde"
      Y ninguna vista que no muestra texto de transcripciones muestra "Ana Inventada Muñoz"

    @PRY-18.1 @interfaz
    Escenario: Una fila de correspondencia sin participante registrado se avisa sin tocar el archivo
      Dado una copia del proyecto de prueba
      Y con la app cerrada se agrega al final de "identificable/correspondencia.csv" la fila "E09,,,"
      Y se tomó la huella de cada archivo del proyecto
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Entonces la interfaz muestra "La tabla de correspondencia tiene una fila para E09, que no está registrado."
      Y el archivo "identificable/correspondencia.csv" es idéntico byte a byte a su versión de las huellas tomadas

  @PAR-04
  Regla: Los nombres no salen de la tabla de correspondencia

    @PAR-04.1 @recorrido-completo
    Escenario: Fuera de la zona identificable ningún archivo tiene los nombres
      Dado el proyecto de prueba después del recorrido completo de los escenarios del tramo 1
      Entonces ningún archivo del proyecto fuera de "identificable/" contiene, en UTF-8 en NFC ni en NFD y sin distinguir mayúsculas, ninguno de los nombres de la tabla de correspondencia de prueba

    @PAR-04.2 @recorrido-completo
    Escenario: El índice y los dos registros no tienen nombres ni contactos
      Dado el proyecto de prueba después del recorrido completo de los escenarios del tramo 1
      Entonces el archivo ".osr/indice.sqlite" no contiene ninguno de los nombres ni de los contactos de la tabla de correspondencia de prueba
      Y el registro técnico no contiene ninguno de los nombres ni de los contactos de la tabla de correspondencia de prueba
      Y el archivo "bitacora/registro.jsonl" no contiene ninguno de los nombres ni de los contactos de la tabla de correspondencia de prueba

    @PAR-04.3 @interfaz
    Escenario: El almacenamiento del navegador no guarda los nombres
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Cuando la interfaz muestra la vista "Correspondencia"
      Y la persona vuelve a la vista "Resumen del proyecto"
      Entonces el almacenamiento local del origen de la app no contiene ninguno de los nombres de la tabla de correspondencia de prueba
      Y el almacenamiento de sesión del origen de la app no contiene ninguno de los nombres de la tabla de correspondencia de prueba
      Y la base IndexedDB del origen de la app no contiene ninguno de los nombres de la tabla de correspondencia de prueba

    @PAR-04.4 @recorrido-completo
    Escenario: Las respuestas de las demás rutas no tienen los nombres
      Dado las respuestas HTTP registradas durante el recorrido completo de los escenarios del tramo 1
      Entonces ninguna respuesta de una ruta que no es de la vista "Correspondencia" ni sirve transcripciones, encabezados, material de origen o respaldos contiene ninguno de los nombres de la tabla de correspondencia de prueba

  @PAR-05
  Regla: Eliminar un participante sin material

    @PAR-05.1 @interfaz
    Escenario: Eliminar a un participante sin material borra su nombre del proyecto
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y el participante "E03" está registrado
      Y el nombre de "E03" en la correspondencia es "Carmen Inventada Solís"
      Y el participante "E03" tiene consentimiento registrado con grabar "otorgado"
      Y la interfaz está abierta en el navegador
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando la interfaz muestra la vista del participante "E03"
      Y la persona pulsa "Eliminar participante"
      Y la persona escribe "E03" en el campo de confirmación
      Y la persona confirma la eliminación
      Entonces el archivo "participantes/participantes.csv" contiene una fila que empieza con "E03," y termina con ",eliminado"
      Y el archivo "identificable/correspondencia.csv" no contiene una fila que empiece con "E03,"
      Y el archivo "participantes/consentimientos.csv" no contiene una fila que empiece con "E03,"
      Y ningún archivo del proyecto contiene el texto "Carmen Inventada Solís"
      Y el registro de eventos tiene exactamente una línea nueva de tipo "participante_eliminado" cuyo detalle contiene "E03"

    @PAR-05.2 @interfaz @PRY-18.1
    Escenario: Con una entrevista creada no se puede eliminar
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y la entrevista de "E01" está creada
      Y la interfaz está abierta en el navegador
      Cuando la interfaz muestra la vista del participante "E01"
      Entonces el botón "Eliminar participante" está deshabilitado con el texto "E01 tiene una entrevista o un respaldo de consentimiento y no se puede eliminar en esta versión."

    @PAR-05.3 @interfaz
    Escenario: Una confirmación que no coincide no elimina nada
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y la interfaz está abierta en el navegador
      Y se tomó la huella de cada archivo del proyecto
      Cuando la interfaz muestra la vista del participante "E01"
      Y la persona pulsa "Eliminar participante"
      Y la persona escribe "E02" en el campo de confirmación
      Y la persona confirma la eliminación
      Entonces ningún archivo del proyecto cambió respecto de las huellas tomadas

  @PAR-06
  Regla: Alcances del consentimiento

    @PAR-06.1
    Escenario: Un proyecto nuevo tiene los cinco alcances por defecto en orden
      Dado un proyecto nuevo creado con `osr nuevo`
      Entonces la lista "consentimiento.alcances" de "proyecto.toml" es exactamente
        | id                 | etiqueta                                      |
        | grabar             | Grabar la entrevista                          |
        | citar_textualmente | Citar textualmente sus palabras               |
        | usar_puntos_mapa   | Usar sus puntos del mapa                      |
        | archivar           | Archivar los datos al terminar                |
        | reutilizar         | Reutilizar los datos en otras investigaciones |

    @PAR-06.2 @interfaz
    Escenario: Agregar un alcance suma su columna con no_consultado
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando la persona agrega desde la interfaz el alcance "transcribir_local" con la etiqueta "Transcribir con software local"
      Entonces la última columna de "participantes/consentimientos.csv" se llama "alcance_transcribir_local"
      Y la columna "alcance_transcribir_local" de "participantes/consentimientos.csv" vale "no_consultado" en las filas de "E01" y "E02"
      Y el registro de eventos tiene exactamente una línea nueva de tipo "alcance_agregado" cuyo detalle contiene "transcribir_local"

    @PAR-06.3 @interfaz
    Escenario: Si falta grabar en proyecto.toml se toma igual y se avisa
      # La especificación no fija el texto de este aviso. Se comprueba que haya un aviso
      # que nombra el alcance grabar.
      Dado una copia del proyecto de prueba
      Y con la app cerrada se quita de "proyecto.toml" el alcance con id "grabar"
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Cuando la interfaz muestra la vista del participante "E01"
      Y la persona pulsa "Modificar consentimiento"
      Entonces el formulario de consentimiento muestra el alcance "Grabar la entrevista"
      Y la interfaz muestra un aviso que nombra el alcance "grabar"

    @PRY-18.1 @interfaz
    Esquema del escenario: Errores al agregar un alcance
      Dado una copia del proyecto de prueba
      Y <preparación>
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y se tomó la huella de cada archivo del proyecto
      Cuando la persona agrega desde la interfaz el alcance "<id>" con la etiqueta "<etiqueta>"
      Entonces la interfaz muestra "<mensaje>"
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas

      Ejemplos:
        | preparación                                        | id                    | etiqueta                    | mensaje                                                                                                         |
        | no hay ninguna preparación adicional               | Transcribir           | Transcribir localmente      | El identificador del alcance va en minúsculas sin tildes, con letras, números o guion bajo, hasta 40 caracteres. |
        | no hay ninguna preparación adicional               | {41 veces la letra a} | Alcance largo               | El identificador del alcance va en minúsculas sin tildes, con letras, números o guion bajo, hasta 40 caracteres. |
        | no hay ninguna preparación adicional               | grabar                | Grabar otra vez             | Ya existe un alcance grabar.                                                                                    |
        | no hay ninguna preparación adicional               | evaluar_transcripcion |                             | La etiqueta tiene que tener entre 1 y 100 caracteres.                                                           |
        | no hay ninguna preparación adicional               | evaluar_transcripcion | {101 veces la letra a}      | La etiqueta tiene que tener entre 1 y 100 caracteres.                                                           |
        | el proyecto tiene 30 alcances                      | otro_alcance          | Otro alcance                | El proyecto ya tiene 30 alcances, que es el máximo.                                                             |

  @PAR-07
  Regla: Registro de consentimiento

    @PAR-07.1 @interfaz
    Escenario: Registrar un consentimiento deja los alcances no tocados en no_consultado
      # La especificación no dice qué respaldo trae el ejemplo. Se usa ninguno, y las
      # columnas del tramo de respaldo se escriben vacías.
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y la interfaz está abierta en el navegador
      Cuando la interfaz muestra la vista del participante "E01"
      Y la persona pulsa "Registrar consentimiento"
      Y la persona completa el formulario de consentimiento con estos valores
        | campo              | valor        |
        | version_formulario | v2           |
        | fecha              | 2026-10-09   |
        | modalidad          | oral_grabado |
        | respaldo           | ninguno      |
        | grabar             | otorgado     |
      Y la persona pulsa "Guardar"
      Entonces el archivo "participantes/consentimientos.csv" tiene exactamente estas filas, con fin de línea CRLF
        """
        seudonimo,version_formulario,fecha,modalidad,respaldo,respaldo_inicio_s,respaldo_fin_s,alcance_grabar,alcance_citar_textualmente,alcance_usar_puntos_mapa,alcance_archivar,alcance_reutilizar
        E01,v2,2026-10-09,oral_grabado,ninguno,,,otorgado,no_consultado,no_consultado,no_consultado,no_consultado
        """
      Y la última línea del registro de eventos es de tipo "consentimiento_registrado" con "E01" en su detalle

    @PAR-07.2 @interfaz @PRY-18.1
    Escenario: Una fecha de mañana no se guarda
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y la interfaz está abierta en el navegador
      Y se tomó la huella de cada archivo del proyecto
      Cuando la interfaz muestra la vista del participante "E01"
      Y la persona pulsa "Registrar consentimiento"
      Y la persona completa el formulario de consentimiento con estos valores
        | campo              | valor     |
        | version_formulario | v2        |
        | fecha              | {mañana}  |
        | modalidad          | escrito   |
        | respaldo           | ninguno   |
      Y la persona pulsa "Guardar"
      Entonces la interfaz muestra "La fecha del consentimiento no puede ser posterior a hoy."
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas

    @PAR-07.3
    Escenario: consentimientos.csv se lee en R con una columna por alcance
      Dado una copia del proyecto de prueba
      Cuando se lee "participantes/consentimientos.csv" en R con `read.csv(ruta, fileEncoding = "UTF-8")`
      Entonces R devuelve una columna por cada alcance de "proyecto.toml", con los nombres "alcance_grabar", "alcance_citar_textualmente", "alcance_usar_puntos_mapa", "alcance_archivar" y "alcance_reutilizar"
      Y R devuelve una fila para "E01" y otra para "E02"

    @PAR-07.4 @interfaz
    Escenario: Con consentimiento la acción es Modificar consentimiento con los valores guardados
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Cuando la interfaz muestra la vista del participante "E01"
      Entonces la interfaz muestra el botón "Modificar consentimiento"
      Y la interfaz no muestra el botón "Registrar consentimiento"
      Cuando la persona pulsa "Modificar consentimiento"
      Entonces el formulario de consentimiento muestra los valores de la fila de "E01" en "participantes/consentimientos.csv"

    @PRY-18.1 @interfaz
    Esquema del escenario: Errores del formulario de consentimiento
      # La especificación no fija cómo se nombra el campo en «Falta <campo>.». Se usa el
      # nombre de la columna.
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y la interfaz está abierta en el navegador
      Y se tomó la huella de cada archivo del proyecto
      Cuando la interfaz muestra la vista del participante "E01"
      Y la persona pulsa "Registrar consentimiento"
      Y la persona completa el formulario de consentimiento con estos valores
        | campo              | valor        |
        | version_formulario | <versión>    |
        | fecha              | <fecha>      |
        | modalidad          | <modalidad>  |
        | respaldo           | ninguno      |
      Y la persona pulsa "Guardar"
      Entonces la interfaz muestra "<mensaje>"
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas

      Ejemplos:
        | versión                | fecha      | modalidad | mensaje                                               |
        |                        | 2026-10-09 | escrito   | Falta version_formulario.                             |
        | v2                     |            | escrito   | Falta fecha.                                          |
        | v2                     | 2026-10-09 |           | Falta modalidad.                                      |
        | {51 veces la letra v}  | 2026-10-09 | escrito   | La versión del formulario admite hasta 50 caracteres. |

  @PAR-08
  Regla: Respaldo del consentimiento

    @PAR-08.1 @interfaz
    Escenario: Dos fotos JPEG se guardan como originales con su ficha
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y la interfaz está abierta en el navegador
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando la interfaz muestra la vista del participante "E01"
      Y la persona pulsa "Registrar consentimiento"
      Y la persona completa el formulario de consentimiento con estos valores
        | campo              | valor      |
        | version_formulario | v2         |
        | fecha              | 2026-10-09 |
        | modalidad          | escrito    |
        | respaldo           | archivos   |
      Y la persona agrega como respaldo los archivos de prueba "firma hoja 1.jpg" y "firma hoja 2.jpg"
      Y la persona pulsa "Guardar"
      Entonces los permisos de "identificable/consentimientos/E01-consentimiento-1.jpg" son 0400
      Y los permisos de "identificable/consentimientos/E01-consentimiento-2.jpg" son 0400
      Y existe el archivo "identificable/consentimientos/E01-consentimiento-1.jpg.ficha.toml"
      Y existe el archivo "identificable/consentimientos/E01-consentimiento-2.jpg.ficha.toml"
      Y la clave "tipo" de "identificable/consentimientos/E01-consentimiento-1.jpg.ficha.toml" vale "respaldo_consentimiento"
      Y la clave "seudonimo" de "identificable/consentimientos/E01-consentimiento-2.jpg.ficha.toml" vale "E01"
      Y la columna "respaldo" de "participantes/consentimientos.csv" vale "archivos" en la fila de "E01"
      Y el registro de eventos tiene exactamente una línea nueva de tipo "respaldo_consentimiento_agregado" cuyo detalle contiene "E01"
      Y el registro de eventos no contiene el texto "firma hoja"

    @PAR-08.2 @interfaz @PRY-18.1
    Escenario: Un PDF cuyo contenido es PNG no se importa
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y el participante "E01" tiene consentimiento registrado con grabar "otorgado"
      Y la interfaz está abierta en el navegador
      Y se tomó la huella de cada archivo del proyecto
      Cuando la interfaz muestra la vista del participante "E01"
      Y la persona pulsa "Modificar consentimiento"
      Y la persona elige el respaldo "archivos"
      Y la persona agrega como respaldo el archivo de prueba "formulario.pdf", cuyo contenido es una imagen PNG
      Entonces la interfaz muestra "formulario.pdf no es un PDF, una imagen o un audio admitido."
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas
      Y el registro técnico no contiene el texto "formulario.pdf"

    @PAR-08.3 @interfaz
    Escenario: El botón del tramo oral reproduce entre inicio y fin
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y el consentimiento de "E01" tiene respaldo "audio_entrevista" entre 2,0 y 9,5 s
      Y la interfaz está abierta en el navegador
      Cuando la interfaz muestra la vista del participante "E01"
      Y la persona pulsa el botón que reproduce el tramo del consentimiento
      Entonces la reproducción empieza en 2,0 s con 0,25 s de tolerancia
      Y la reproducción se detiene en 9,5 s con 0,25 s de tolerancia

    @PAR-08.4 @interfaz
    Escenario: Sin respaldo se muestra con texto
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Cuando la interfaz muestra la vista del participante "E01"
      Entonces junto al consentimiento la interfaz muestra el texto "Sin respaldo"

    @PRY-18.1 @interfaz
    Esquema del escenario: Errores del respaldo del consentimiento
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y el participante "E03" está registrado
      Y <preparación>
      Y la interfaz está abierta en el navegador
      Y se tomó la huella de cada archivo del proyecto
      Cuando la interfaz muestra la vista del participante "<seudónimo>"
      Y la persona pulsa "Modificar consentimiento"
      Y <acción>
      Entonces la interfaz muestra "<mensaje>"
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas

      Ejemplos:
        | seudónimo | preparación                                                                     | acción                                                                                       | mensaje                                                                                    |
        | E01       | no hay ninguna preparación adicional                                            | la persona elige el respaldo "archivos" y agrega el archivo generado "grande.pdf" de 501 MB   | grande.pdf pasa de 500 MB.                                                                 |
        | E01       | no hay ninguna preparación adicional                                            | la persona elige el respaldo "archivos" y agrega 11 archivos JPEG de prueba                   | Un consentimiento admite hasta 10 archivos de respaldo.                                    |
        | E03       | el participante "E03" tiene consentimiento registrado con grabar "otorgado"     | la persona elige el respaldo "audio_entrevista" con el tramo de 2,0 a 9,5 s y pulsa "Guardar" | La entrevista de E03 no tiene audio todavía.                                               |
        | E01       | no hay ninguna preparación adicional                                            | la persona elige el respaldo "audio_entrevista" con el tramo de 9,5 a 2,0 s y pulsa "Guardar" | El tramo del consentimiento tiene que estar dentro del audio y terminar después de empezar. |
        | E01       | no hay ninguna preparación adicional                                            | la persona elige el respaldo "audio_entrevista" con el tramo de 2,0 a 999,0 s y pulsa "Guardar" | El tramo del consentimiento tiene que estar dentro del audio y terminar después de empezar. |

  @PAR-09
  Regla: Cambios en un consentimiento

    @PAR-09.1 @interfaz
    Escenario: Modificar un alcance anota su valor anterior y el nuevo
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y el participante "E01" tiene consentimiento registrado con grabar "otorgado"
      Y la interfaz está abierta en el navegador
      Cuando la interfaz muestra la vista del participante "E01"
      Y la persona pulsa "Modificar consentimiento"
      Y la persona elige "denegado" para el alcance "Citar textualmente sus palabras"
      Y la persona pulsa "Guardar"
      Entonces la última línea del registro de eventos es de tipo "consentimiento_modificado" con "E01" en su detalle
      Y el detalle de la última línea del registro de eventos contiene el JSON
        """
        {"citar_textualmente": ["no_consultado", "denegado"]}
        """

    @PAR-09.2 @interfaz @PRY-18.1
    Escenario: Retirar el permiso de grabar con audio importado muestra un aviso y no toca el audio
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y se tomó la huella de "identificable/entrevistas/E01/E01.ogg"
      Cuando la interfaz muestra la vista del participante "E01"
      Y la persona pulsa "Modificar consentimiento"
      Y la persona elige "denegado" para el alcance "Grabar la entrevista"
      Y la persona pulsa "Guardar"
      Entonces la vista de la entrevista "E01" muestra "El consentimiento de E01 ya no autoriza grabar y la entrevista tiene audio. Hay que decidir qué hacer con ese audio."
      Y la vista "Resumen del proyecto" muestra "El consentimiento de E01 ya no autoriza grabar y la entrevista tiene audio. Hay que decidir qué hacer con ese audio."
      Y la huella de "identificable/entrevistas/E01/E01.ogg" es la misma que se tomó

    @PAR-09.3 @interfaz
    Escenario: Volver grabar a otorgado quita el aviso
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y el consentimiento de "E01" tiene grabar "denegado"
      Y la interfaz está abierta en el navegador
      Cuando la interfaz muestra la vista del participante "E01"
      Y la persona pulsa "Modificar consentimiento"
      Y la persona elige "otorgado" para el alcance "Grabar la entrevista"
      Y la persona pulsa "Guardar"
      Entonces la vista de la entrevista "E01" no muestra "El consentimiento de E01 ya no autoriza grabar y la entrevista tiene audio. Hay que decidir qué hacer con ese audio."
      Y la vista "Resumen del proyecto" no muestra "El consentimiento de E01 ya no autoriza grabar y la entrevista tiene audio. Hay que decidir qué hacer con ese audio."

  @PAR-10
  Regla: Cobertura del consentimiento

    @PAR-10.1 @interfaz
    Escenario: Con dos entrevistas y un consentimiento el resumen dice 1 de 2
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y el participante "E01" tiene consentimiento registrado con grabar "otorgado"
      Y el participante "E02" está registrado
      Y la entrevista de "E01" está creada
      Y la entrevista de "E02" está creada
      Y la interfaz está abierta en el navegador
      Cuando la interfaz muestra la vista "Resumen del proyecto"
      Entonces la interfaz muestra "Consentimiento registrado en 1 de 2 entrevistas."
      Y la entrevista "E01" muestra la etiqueta "Consentimiento registrado"
      Y la entrevista "E02" muestra la etiqueta "Consentimiento pendiente"
      Y el participante "E02" muestra la etiqueta "Sin consentimiento registrado"

    @PAR-10.2 @interfaz
    Escenario: Al registrar el segundo el resumen pasa a 2 de 2 sin recargar
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y el participante "E01" tiene consentimiento registrado con grabar "otorgado"
      Y el participante "E02" está registrado
      Y la entrevista de "E01" está creada
      Y la entrevista de "E02" está creada
      Y la interfaz está abierta en el navegador
      Y la interfaz muestra la vista "Resumen del proyecto"
      Cuando la persona registra desde la interfaz el consentimiento de "E02" con grabar "otorgado"
      Y la persona vuelve a la vista "Resumen del proyecto"
      Entonces la interfaz muestra "Consentimiento registrado en 2 de 2 entrevistas."
      Y la pestaña no se recargó desde la entrada

  @PAR-11
  Regla: Consentimiento al importar audio

    @PAR-11.1 @interfaz @PRY-18.1
    Escenario: Con grabar denegado el audio no se importa y no hay excepción
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y el participante "E01" tiene consentimiento registrado con grabar "denegado"
      Y la entrevista de "E01" está creada
      Y la interfaz está abierta en el navegador
      Y se tomó la huella de cada archivo del proyecto
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando la persona importa el archivo "E01.ogg" con "Importar audio" en la entrevista "E01"
      Entonces la interfaz muestra "El consentimiento de E01 no autoriza grabar. No se puede importar el audio."
      Y el diálogo muestra solo el botón "Entendido"
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas
      Y el registro de eventos no recibió líneas nuevas

    @PAR-11.2 @interfaz @PRY-18.1
    Escenario: Sin consentimiento, Cancelar no escribe nada
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y la entrevista de "E01" está creada
      Y la interfaz está abierta en el navegador
      Y se tomó la huella de cada archivo del proyecto
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando la persona importa el archivo "E01.ogg" con "Importar audio" en la entrevista "E01"
      Entonces la interfaz muestra "E01 no tiene consentimiento registrado."
      Cuando la persona pulsa "Cancelar"
      Entonces ningún archivo del proyecto cambió respecto de las huellas tomadas
      Y el registro de eventos no recibió líneas nuevas

    @PAR-11.2 @interfaz
    Escenario: Sin consentimiento, Hacerlo igual importa y anota la excepción
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y la entrevista de "E01" está creada
      Y la interfaz está abierta en el navegador
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando la persona importa el archivo "E01.ogg" con "Importar audio" en la entrevista "E01"
      Y la persona pulsa "Hacerlo igual"
      Entonces existe el archivo "identificable/entrevistas/E01/E01.ogg"
      Y el registro de eventos tiene exactamente una línea nueva de tipo "excepcion_confirmada" cuyo detalle contiene "audio_sin_consentimiento", "E01" y "sin_registro"
      Y la entrevista "E01" muestra la etiqueta "Consentimiento pendiente"

    @PAR-11.3 @interfaz
    Escenario: Con grabar otorgado el audio se importa sin diálogo
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y el participante "E01" tiene consentimiento registrado con grabar "otorgado"
      Y la entrevista de "E01" está creada
      Y la interfaz está abierta en el navegador
      Cuando la persona importa el archivo "E01.ogg" con "Importar audio" en la entrevista "E01"
      Entonces no apareció ningún diálogo
      Y existe el archivo "identificable/entrevistas/E01/E01.ogg"

    @PRY-18.1 @interfaz
    Escenario: Con grabar no consultado el bloqueo admite excepción con su motivo
      Dado un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y el participante "E01" tiene consentimiento registrado con grabar "no_consultado"
      Y la entrevista de "E01" está creada
      Y la interfaz está abierta en el navegador
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando la persona importa el archivo "E01.ogg" con "Importar audio" en la entrevista "E01"
      Entonces la interfaz muestra "El consentimiento de E01 no dice si autoriza grabar."
      Y el diálogo muestra los botones "Cancelar" y "Hacerlo igual"
      Cuando la persona pulsa "Hacerlo igual"
      Entonces el registro de eventos tiene exactamente una línea nueva de tipo "excepcion_confirmada" cuyo detalle contiene "audio_sin_consentimiento", "E01" y "grabar_no_consultado"

  @PAR-12
  Regla: Dos zonas

    @PAR-12.1 @recorrido-completo
    Escenario: Fuera de la zona identificable no hay material que identifique
      Dado el proyecto de prueba después del recorrido completo de los escenarios del tramo 1
      Entonces fuera de "identificable/" y de ".osr/" no hay ningún archivo de audio, imagen ni PDF
      Y ningún archivo fuera de "identificable/" y de ".osr/" contiene ninguna frase de las transcripciones de prueba
      Y ningún archivo fuera de "identificable/" y de ".osr/" contiene ninguna frase de los encabezados de prueba
      Y ningún archivo fuera de "identificable/" y de ".osr/" contiene ninguno de los lugares de los "entrevista.toml" de prueba
      Y ningún archivo fuera de "identificable/" y de ".osr/" contiene ninguno de los nombres originales de los archivos importados

    @PAR-12.2 @recorrido-completo
    Escenario: Todo lo identificable que crea la app queda bajo identificable/
      Dado el proyecto de prueba después del recorrido completo de los escenarios del tramo 1
      Entonces cada archivo que la app creó con contenido de la columna «Zona identificable» de PAR-12 está bajo "identificable/"

  @PAR-13
  Regla: Zona y sensibilidad visibles en la interfaz

    @PAR-13.1 @interfaz
    Escenario: El audio de una entrevista muestra su zona y su marca
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Cuando la interfaz muestra la vista de la entrevista "E01"
      Entonces junto al audio la interfaz muestra el texto "Zona identificable"
      Y junto al audio la interfaz muestra el texto "Identificable"

    @PAR-13.2 @interfaz
    Escenario: Sin colores las etiquetas siguen visibles por su texto
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y los estilos de color de la página están anulados y todo se ve en gris
      Cuando la interfaz muestra la vista "Archivos del proyecto"
      Entonces cada archivo de la lista muestra visible una etiqueta de zona con el texto "Zona identificable" o "Zona de trabajo"
      Y cada archivo de la lista muestra visible una etiqueta de sensibilidad con el texto "Identificable", "Sensible", "No sensible" o "Sin marca"

    @PAR-13.3 @interfaz
    Escenario: El nombre accesible de cada archivo incluye su zona y su marca
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Cuando la interfaz muestra la vista "Archivos del proyecto"
      Entonces el árbol de accesibilidad da para cada archivo de la lista un nombre accesible que incluye su zona y su marca de sensibilidad
      Y el nombre accesible de "README.md" incluye "Zona de trabajo" y "No sensible"
      Y el nombre accesible de "identificable/correspondencia.csv" incluye "Zona identificable" e "Identificable"

  @PAR-14
  Regla: Marcas de sensibilidad

    @PAR-14.1 @interfaz
    Escenario: Las marcas por defecto de un proyecto nuevo
      Dado un proyecto nuevo creado con `osr nuevo`
      Y la interfaz está abierta en el navegador
      Cuando la interfaz muestra la vista "Archivos del proyecto"
      Entonces el archivo "participantes/participantes.csv" muestra la marca "Sensible"
      Y el archivo "README.md" muestra la marca "No sensible"
      Y el archivo "sensibilidad.csv" tiene exactamente estas filas, con fin de línea CRLF
        """
        ruta,nivel
        """

    @PAR-14.2 @interfaz
    Escenario: Marcar un archivo propio lo guarda en sensibilidad.csv y en el README
      Dado un proyecto nuevo creado con `osr nuevo`
      Y la persona deja en la raíz del proyecto el archivo "notas.txt" con el texto "apuntes inventados"
      Y la interfaz está abierta en el navegador
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando la interfaz muestra la vista "Archivos del proyecto"
      Entonces el archivo "notas.txt" muestra la marca "Sin marca"
      Cuando la persona cambia la marca de "notas.txt" a "Sensible"
      Entonces el archivo "sensibilidad.csv" contiene la fila "notas.txt,sensible"
      Y la tabla de sensibilidad de "README.md" tiene la fila de "notas.txt" con nivel "sensible"
      Y el registro de eventos tiene exactamente una línea nueva de tipo "sensibilidad_cambiada" cuyo detalle contiene "notas.txt"

    @PAR-14.3 @interfaz @PRY-18.1
    Escenario: Ningún control cambia la marca de un archivo identificable
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Cuando la interfaz muestra la vista "Archivos del proyecto"
      Entonces ningún archivo bajo "identificable/" tiene un control para cambiar su marca
      Y el archivo "identificable/correspondencia.csv" muestra la marca "Identificable (por estar en la zona identificable)"
      Y el control de marca de "README.md" no ofrece la opción "Identificable"
      Y la ayuda del control de marca de "README.md" dice "Lo identificable va en la carpeta identificable/. Hay que mover ahí el archivo."

  @PAR-15
  Regla: Sin vía de salida

    @PAR-15.1 @recorrido-completo
    Escenario: Ninguna respuesta se sirve como descarga
      Dado las respuestas HTTP registradas durante el recorrido completo de los escenarios del tramo 1
      Entonces ninguna respuesta tiene el encabezado "Content-Disposition" con un valor que empiece con "attachment"

    @PAR-15.2 @recorrido-completo
    Escenario: Fuera del proyecto solo cambia el registro técnico
      Dado el proyecto de prueba después del recorrido completo de los escenarios del tramo 1
      Entonces fuera de la carpeta del proyecto el único archivo que cambió durante el recorrido es el registro técnico
      Y el archivo de un solo uso de SRV-03, si existió, ya no existe

    @PAR-15.3 @interfaz
    Escenario: Ninguna vista ofrece exportar, compartir, enviar ni descargar
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Cuando la persona recorre todas las vistas de la interfaz del tramo 1
      Entonces ninguna vista tiene un botón o enlace con alguno de estos textos
        | texto     |
        | Exportar  |
        | Compartir |
        | Enviar    |
        | Descargar |
