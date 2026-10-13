# language: es
#
# Escenarios del módulo diseño de la investigación (DIS), tramo 1 de la v0.1.
# Fuente especificaciones/diseno.md. Valen las convenciones de proyecto.feature.
#
# «se guarda por la API el paso» manda las respuestas como la pestaña, con el token, el
# Origin propio y la versión del paso que devuelve la API en ese momento, salvo que el
# escenario diga otra versión.

Característica: Diseño de la investigación
  Un proyecto empieza por el diseño, en cinco pasos con preguntas guía apoyadas en la
  bibliografía del curso, que se escriben en espiral.

  @DIS-03
  Regla: Vista general del diseño

    @DIS-03.1
    Escenario: Un proyecto nuevo tiene los cinco pasos sin empezar
      Dado un proyecto nuevo creado con `osr nuevo`
      Entonces la página "/diseno" de la sesión contiene estos textos
        | texto                                                                                                        |
        | Diseño de la investigación                                                                                   |
        | El diseño se escribe en espiral: los pasos tienen un orden sugerido, pero se puede volver a cualquiera cuando haga falta. |
        | En un diseño participativo, estos pasos se pueden escribir con las personas participantes.                   |
        | Tema e interés                                                                                               |
        | Posicionamiento y supuestos                                                                                  |
        | Problema, estado de la cuestión y marco teórico                                                              |
        | Pregunta y objetivos                                                                                         |
        | Enfoque, metodología y métodos                                                                               |
      Y la página "/diseno" de la sesión contiene "Todavía sin texto"
      Y la página "/diseno" de la sesión muestra los pasos con estos estados
        | paso | estado      |
        | 01   | Sin empezar |
        | 02   | Sin empezar |
        | 03   | Sin empezar |
        | 04   | Sin empezar |
        | 05   | Sin empezar |

    @DIS-03.2
    Escenario: Un paso guardado muestra el comienzo de su texto y En borrador
      Dado un proyecto nuevo creado con `osr nuevo`
      Cuando se guarda por la API el paso "01" con estas respuestas
        | respuesta                                            |
        | Caminar de noche por el centro de San José.          |
        |                                                      |
        |                                                      |
        |                                                      |
      Entonces la respuesta tiene estado 200
      Y la página "/diseno" de la sesión contiene "Caminar de noche por el centro de San José."
      Y la página "/diseno" de la sesión muestra el paso "01" con el estado "En borrador"

    @DIS-03.3
    Escenario: El resumen lleva al diseño
      Dado un proyecto nuevo creado con `osr nuevo`
      Entonces la página "/" de la sesión tiene un enlace "Empezar el diseño" a "/diseno"

    @DIS-03.3
    Escenario: Sin proyecto abierto no hay diseño
      Dado la carpeta de la aplicación tiene una copia del proyecto de prueba en "previa"
      Y osr está corriendo sin proyecto
      Y un pedido HTTP GET a "/diseno"
      Y el pedido lleva el token válido en el encabezado Authorization
      Cuando se manda el pedido
      Entonces la respuesta tiene estado 404

  @DIS-04
  Regla: Página de un paso y guardado

    @DIS-04.1
    Escenario: Guardar un paso deja su archivo en Markdown
      Dado un proyecto nuevo creado con `osr nuevo`
      Cuando se guarda por la API el paso "01" con estas respuestas
        | respuesta                                       |
        | Caminar de noche por el centro de San José.     |
        | De la experiencia propia de caminar al trabajo. |
        | Para las personas que caminan esas calles.      |
        | Con vecinas del barrio Amón.                    |
      Entonces la respuesta tiene estado 200
      Y el archivo "diseno/01-tema-e-interes.md" es exactamente
        """
        # Tema e interés

        ## ¿Qué parte del mundo social interesa estudiar?

        Caminar de noche por el centro de San José.

        ## ¿De dónde surge el interés: experiencia propia, lecturas, un encargo, una comunidad?

        De la experiencia propia de caminar al trabajo.

        ## ¿Para qué y para quién se investiga?

        Para las personas que caminan esas calles.

        ## ¿Con quiénes se define el tema y el problema, y quién decide sobre ellos: quien investiga, un equipo, las personas o comunidades involucradas?

        Con vecinas del barrio Amón.
        """
      Y la página "/diseno/01" de la sesión contiene "De la experiencia propia de caminar al trabajo."
      Y la página "/diseno/01" de la sesión contiene "Qué dice la bibliografía"
      Y la página "/diseno/01" de la sesión contiene "Sautu et al. (2005, p. 30)"

    @DIS-04.2 @PRY-18.1
    Escenario: Un cambio hecho fuera de la app no se pisa
      Dado un proyecto nuevo creado con `osr nuevo`
      Y se guardó por la API el paso "01" con la primera respuesta "Primera versión."
      Y se anotó la versión del paso "01"
      Y con la app abierta se agrega al final de "diseno/01-tema-e-interes.md" la línea "Agregado a mano."
      Y se tomó la huella de cada archivo del proyecto
      Cuando se guarda por la API el paso "01" con la versión anotada y la primera respuesta "Segunda versión."
      Entonces la respuesta tiene estado 409
      Y el cuerpo de la respuesta es el JSON
        """
        {"error": "cambio_externo", "mensaje": "El archivo diseno/01-tema-e-interes.md cambió fuera de la herramienta mientras estaba abierto. No se guardó el cambio para no pisarlo."}
        """
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas

    @DIS-04.3
    Escenario: Guardar dos veces lo mismo no escribe ni anota de nuevo
      Dado un proyecto nuevo creado con `osr nuevo`
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando se guarda por la API el paso "02" con la primera respuesta "Desde la experiencia de caminar."
      Y se guarda por la API el paso "02" con la primera respuesta "Desde la experiencia de caminar."
      Entonces el registro de eventos tiene exactamente una línea nueva de tipo "diseno_guardado" cuyo detalle contiene "02"

    @DIS-04.4
    Escenario: Una respuesta con la ñ descompuesta se guarda en NFC
      Dado un proyecto nuevo creado con `osr nuevo`
      Cuando se guarda por la API el paso "01" con la primera respuesta "El {año con la ñ descompuesta} del estudio."
      Entonces el archivo "diseno/01-tema-e-interes.md" contiene "El año del estudio." en NFC y no en NFD

    @DIS-04.5 @PRY-18.1
    Esquema del escenario: Un pedido que no sirve no escribe nada
      Dado un proyecto nuevo creado con `osr nuevo`
      Y se tomó la huella de cada archivo del proyecto
      Cuando se manda a la API del paso "01" el cuerpo de <caso>
      Entonces la respuesta tiene estado <estado>
      Y el cuerpo de la respuesta es el JSON
        """
        {"error": "<error>", "mensaje": "<mensaje>"}
        """
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas

      Ejemplos:
        | caso                              | estado | error           | mensaje                                              |
        | tres respuestas                   | 400    | pedido_invalido | El pedido no tiene la forma esperada.                |
        | una respuesta de 20001 caracteres | 422    | respuesta_larga | Cada respuesta puede tener hasta 20 000 caracteres.  |
        | un texto que no es JSON           | 400    | pedido_invalido | El pedido no tiene la forma esperada.                |

    @DIS-04.5 @PRY-18.1
    Escenario: Sin proyecto abierto no se guarda
      Dado la carpeta de la aplicación tiene una copia del proyecto de prueba en "previa"
      Y osr está corriendo sin proyecto
      Cuando se manda a la API del paso "01" el cuerpo de cuatro respuestas
      Entonces la respuesta tiene estado 409
      Y el cuerpo de la respuesta es el JSON
        """
        {"error": "sin_proyecto", "mensaje": "No hay un proyecto abierto."}
        """

  @DIS-05
  Regla: Estado de cada paso y coherencia

    @DIS-05.1
    Escenario: Un paso guardado queda en borrador y revisado al marcarlo
      Dado un proyecto nuevo creado con `osr nuevo`
      Cuando se guarda por la API el paso "01" con la primera respuesta "Caminar de noche."
      Entonces la página "/diseno" de la sesión muestra el paso "01" con el estado "En borrador"
      Cuando se marca por la API el paso "01" como revisado
      Entonces la página "/diseno" de la sesión muestra el paso "01" con el estado "Revisado"
      Cuando se marca por la API el paso "01" como borrador
      Entonces la página "/diseno" de la sesión muestra el paso "01" con el estado "En borrador"

    @DIS-05.2
    Escenario: Cambiar un paso anterior pide revisar la coherencia de los siguientes
      Dado un proyecto nuevo creado con `osr nuevo`
      Y se guardó por la API el paso "01" con la primera respuesta "Caminar de noche."
      Y se guardó por la API el paso "02" con la primera respuesta "Desde la experiencia de caminar."
      Y se marcó por la API el paso "01" como revisado
      Y se marcó por la API el paso "02" como revisado
      Cuando se guarda por la API el paso "01" con la primera respuesta "Caminar de día y de noche."
      Entonces la página "/diseno" de la sesión muestra el paso "02" con el estado "Revisar coherencia"
      Y la página "/diseno/02" de la sesión contiene "El paso 01, Tema e interés, cambió el {hoy en palabras}, después de la revisión de este paso. Hay que releerlo: si sigue de acuerdo con el cambio, se marca como revisado; si no, se corrige y se guarda."
      Cuando se marca por la API el paso "02" como revisado
      Entonces la página "/diseno" de la sesión muestra el paso "02" con el estado "Revisado"

    @DIS-05.6
    Escenario: El primer texto de un paso no pide revisar a los demás
      Dado un proyecto nuevo creado con `osr nuevo`
      Y se guardó por la API el paso "01" con la primera respuesta "Caminar de noche."
      Y se marcó por la API el paso "01" como revisado
      Cuando se guarda por la API el paso "02" con la primera respuesta "Desde la experiencia de caminar."
      Entonces la página "/diseno" de la sesión muestra el paso "01" con el estado "Revisado"
      Cuando se guarda por la API el paso "02" con la primera respuesta "Desde la experiencia de caminar de noche."
      Entonces la página "/diseno" de la sesión muestra el paso "01" con el estado "Revisar coherencia"
      Y la página "/diseno" de la sesión muestra el paso "02" con el estado "En borrador"

    @DIS-05.3
    Escenario: Un paso sin texto nunca pide revisar la coherencia
      Dado un proyecto nuevo creado con `osr nuevo`
      Cuando se guarda por la API el paso "01" con la primera respuesta "Caminar de noche."
      Entonces la página "/diseno" de la sesión muestra el paso "03" con el estado "Sin empezar"

    @DIS-05.4 @PRY-18.1
    Escenario: Un paso sin texto no se marca como revisado
      Dado un proyecto nuevo creado con `osr nuevo`
      Cuando se marca por la API el paso "03" como revisado
      Entonces la respuesta tiene estado 422
      Y el cuerpo de la respuesta es el JSON
        """
        {"error": "paso_vacio", "mensaje": "Un paso sin texto no se puede marcar como revisado."}
        """

    @DIS-05.5
    Escenario: Cambiar un paso posterior también pide revisar los anteriores
      Dado un proyecto nuevo creado con `osr nuevo`
      Y se guardó por la API el paso "02" con la primera respuesta "Desde la experiencia de caminar."
      Y se guardó por la API el paso "04" con la primera respuesta "¿Cómo perciben la inseguridad quienes caminan de noche?"
      Y se marcó por la API el paso "02" como revisado
      Y se marcó por la API el paso "04" como revisado
      Cuando se guarda por la API el paso "04" con la primera respuesta "¿Qué hace la gente para no ser asaltada al caminar de noche?"
      Entonces la página "/diseno" de la sesión muestra el paso "02" con el estado "Revisar coherencia"

    @DIS-05
    Escenario: Un archivo editado fuera de la app vuelve a borrador
      Dado un proyecto nuevo creado con `osr nuevo`
      Y se guardó por la API el paso "01" con la primera respuesta "Caminar de noche."
      Y se marcó por la API el paso "01" como revisado
      Cuando con la app abierta se agrega al final de "diseno/01-tema-e-interes.md" la línea "Agregado a mano."
      Entonces la página "/diseno" de la sesión muestra el paso "01" con el estado "En borrador"

  @DIS-06
  Regla: Formato en el disco

    @DIS-06.1
    Escenario: El diseño se lee sin la herramienta
      Dado un proyecto nuevo creado con `osr nuevo`
      Cuando se guarda por la API el paso "01" con la primera respuesta "Caminar de noche."
      Y se marca por la API el paso "01" como revisado
      Entonces "diseno/estado.toml" es TOML 1.0 en UTF-8 sin BOM
      Y el archivo "diseno/01-tema-e-interes.md" no empieza con BOM
      Y los permisos de "diseno/01-tema-e-interes.md" son 0600
      Y los permisos de "diseno/" son 0700

    @DIS-06.2
    Escenario: Los estados sobreviven a borrar el índice
      Dado un proyecto nuevo creado con `osr nuevo`
      Y se guardó por la API el paso "01" con la primera respuesta "Caminar de noche."
      Y se marcó por la API el paso "01" como revisado
      Y osr se cierra de forma ordenada
      Y con la app cerrada se borra la carpeta ".osr/" con todo su contenido
      Cuando se abre el proyecto con `osr abrir {proyecto} --sin-navegador`
      Entonces la página "/diseno" de la sesión muestra el paso "01" con el estado "Revisado"

    @DIS-06.3
    Escenario: Una sección agregada a mano se conserva al guardar
      Dado un proyecto nuevo creado con `osr nuevo`
      Y se guardó por la API el paso "01" con la primera respuesta "Caminar de noche."
      Y con la app abierta se agrega al final de "diseno/01-tema-e-interes.md" la línea "## Una pregunta que ya no está"
      Y con la app abierta se agrega al final de "diseno/01-tema-e-interes.md" la línea "Una respuesta vieja."
      Cuando se guarda por la API el paso "01" con la primera respuesta "Caminar de día."
      Entonces la respuesta tiene estado 200
      Y el archivo "diseno/01-tema-e-interes.md" contiene "## Una pregunta que ya no está"
      Y el archivo "diseno/01-tema-e-interes.md" contiene "Una respuesta vieja."
      Y el archivo "diseno/01-tema-e-interes.md" contiene "Caminar de día."

    @DIS-06.4
    Escenario: Una línea que parece un encabezado queda en su respuesta
      Dado un proyecto nuevo creado con `osr nuevo`
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando se guarda por la API el paso "04" con la primera respuesta "Pregunta{salto de línea}## Contexto{salto de línea}Texto del contexto."
      Y se guarda por la API el paso "04" con la primera respuesta "Pregunta{salto de línea}## Contexto{salto de línea}Texto del contexto."
      Entonces la página "/diseno/04" de la sesión contiene "## Contexto Texto del contexto."
      Y el registro de eventos tiene exactamente una línea nueva de tipo "diseno_guardado" cuyo detalle contiene "04"

    @DIS-06.5
    Escenario: El README describe la carpeta del diseño
      Dado un proyecto nuevo creado con `osr nuevo`
      Cuando se guarda por la API el paso "01" con la primera respuesta "Caminar de noche."
      Entonces el archivo "README.md" contiene "| `diseno/` | El diseño de la investigación, un archivo por paso | de trabajo |"
      Y el archivo "diseno/estado.toml" contiene "huella = "

  @DIS-03
  Regla: El resumen muestra el diseño empezado

    @PRY-22.5
    Escenario: Con el diseño empezado, el resumen lo dice
      Dado un proyecto nuevo creado con `osr nuevo`
      Cuando se guarda por la API el paso "01" con la primera respuesta "Caminar de noche."
      Entonces la página "/" de la sesión contiene "Proyecto en diseño"
      Y la página "/" de la sesión tiene un enlace "Seguir el diseño" a "/diseno"
      Y la página "/" de la sesión contiene "Tema e interés ◐ En borrador"

  @DIS-04 @interfaz
  Regla: Escribir en la pantalla

    @DIS-04.1 @interfaz
    Escenario: Escribir una respuesta y guardarla
      Dado un proyecto nuevo creado con `osr nuevo`
      Y la interfaz está abierta en el navegador
      Cuando la persona pulsa "Empezar el diseño"
      Y la persona pulsa "Tema e interés"
      Y la persona escribe "Caminar de noche." en el campo "¿Qué parte del mundo social interesa estudiar?"
      Y la persona pulsa "Guardar"
      Entonces a más tardar 1 s después la interfaz muestra "Guardado"
