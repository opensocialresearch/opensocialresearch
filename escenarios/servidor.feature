# language: es
#
# Escenarios del módulo servidor (SRV), tramo 1 de la v0.1.
# Fuente especificaciones/servidor.md.
#
# Las convenciones, las etiquetas y los datos de prueba están al principio de proyecto.feature.
#
# Pedidos HTTP de estos escenarios
# - Un pedido nuevo lleva por defecto el Host propio, 127.0.0.1 con el puerto {P}, sin Origin,
#   sin cookie y sin token. Cada paso «el pedido lleva» agrega o reemplaza un encabezado.
# - {P+1} es el puerto siguiente al de la sesión y {token anterior} el de un arranque previo.
# - «el cuerpo de la respuesta es el JSON» compara el contenido JSON, no los espacios.

Característica: Servidor local
  El programa osr escucha solo en loopback, exige un token en cada pedido, rechaza a
  cualquier otra página o cuenta, no abre conexiones salientes y cuida sus permisos.

  @SRV-01
  Regla: Arranque y apertura del navegador

    @SRV-01.1 @interfaz
    Escenario: osr sin proyecto escribe una sola línea y muestra Abrir y Crear
      # Desde D-50, la primera vez se ve la bienvenida de PRY-23 y no esta pantalla.
      Dado una carpeta temporal vacía "{tmp}"
      Y la cuenta ya creó o abrió un proyecto con osr antes
      Cuando se ejecuta `osr --sin-navegador`
      Entonces la salida estándar tiene exactamente una línea con la forma "Open Social Research está abierto en http://127.0.0.1:{P}/?token={token}"
      Cuando el navegador abre la dirección de esa línea
      Entonces la interfaz muestra "Abrir proyecto"
      Y la interfaz muestra "Crear proyecto"

    @SRV-01.2 @PRY-18.1
    Escenario: Una opción desconocida termina con código 2
      Dado una carpeta temporal vacía "{tmp}"
      Cuando se ejecuta `osr --opcion-falsa`
      Entonces el código de salida es 2
      Y la salida de error es la línea "osr: no se reconoce --opcion-falsa. Las órdenes son osr, osr abrir <carpeta> y osr nuevo <carpeta>."

    @SRV-01.3 @PRY-18.1
    Escenario: Sin navegador disponible osr avisa y la dirección sigue funcionando
      Dado una carpeta temporal vacía "{tmp}"
      Y el sistema de prueba no tiene ningún navegador disponible
      Cuando se ejecuta `osr`
      Entonces la salida de error tiene la línea "osr: no se pudo abrir el navegador. Hay que abrir a mano la dirección de arriba."
      Y osr sigue corriendo
      Dado un pedido HTTP GET a la dirección de la línea de la salida estándar
      Cuando se manda el pedido
      Entonces la respuesta tiene estado 200

  @SRV-02
  Regla: Solo loopback y puerto aleatorio

    @SRV-02.1
    Escenario: osr escucha exactamente en 127.0.0.1 y ::1 con el mismo puerto
      Dado una carpeta temporal vacía "{tmp}"
      Cuando se ejecuta `osr --sin-navegador`
      Entonces los sockets en escucha del proceso osr son exactamente estos
        | dirección      |
        | 127.0.0.1:{P}  |
        | [::1]:{P}      |

    @SRV-02.2
    Escenario: Una conexión desde la dirección de red local es rechazada
      # SRV-14.1 pide correr la suite en un espacio de nombres con solo loopback, y este
      # escenario necesita una dirección que no lo sea. Se le da al espacio de nombres una
      # interfaz local sin salida ni rutas, que no permite ninguna conexión hacia fuera.
      Dado el sistema de prueba tiene una interfaz de red local con la dirección "10.200.0.1" y sin rutas de salida
      Y una carpeta temporal vacía "{tmp}"
      Cuando se ejecuta `osr --sin-navegador`
      Y la prueba intenta una conexión TCP a "10.200.0.1:{P}"
      Entonces la conexión es rechazada

    @SRV-02.3
    Escenario: El puerto cambia entre arranques
      Dado una carpeta temporal vacía "{tmp}"
      Cuando osr se arranca 5 veces seguidas con `osr --sin-navegador` y se cierra de forma ordenada después de cada arranque
      Entonces entre los 5 puertos anotados hay al menos 2 distintos

    @SRV-02.4
    Escenario: Sin IPv6 de loopback osr escucha solo en IPv4 y lo anota
      Dado el sistema de prueba no tiene IPv6 de loopback
      Y una carpeta temporal vacía "{tmp}"
      Cuando se ejecuta `osr --sin-navegador`
      Entonces los sockets en escucha del proceso osr son exactamente estos
        | dirección     |
        | 127.0.0.1:{P} |
      Y el registro técnico tiene una línea que contiene "sin IPv6 de loopback"
      Dado un pedido HTTP GET a "/"
      Y el pedido lleva el token válido en el encabezado Authorization
      Cuando se manda el pedido
      Entonces la respuesta tiene estado 200

    @PRY-18.1
    Escenario: Sin puerto libre en las dos direcciones osr termina con código 5
      Dado el rango de puertos efímeros del sistema de prueba es un solo puerto, ocupado en "[::1]"
      Y una carpeta temporal vacía "{tmp}"
      Cuando se ejecuta `osr --sin-navegador`
      Entonces el código de salida es 5
      Y la salida de error es la línea "osr: no se encontró un puerto libre en las direcciones locales."

  @SRV-03
  Regla: Token por arranque

    @SRV-03.1
    Escenario: Cada arranque tiene un token nuevo de 43 caracteres base64url
      Dado una carpeta temporal vacía "{tmp}"
      Cuando osr se arranca 2 veces seguidas con `osr --sin-navegador` y se cierra de forma ordenada después de cada arranque
      Entonces los 2 tokens anotados son distintos
      Y cada token anotado tiene 43 caracteres
      Y cada token anotado tiene solo caracteres de "A-Z a-z 0-9 - _"

    @SRV-03.2
    Escenario: El token no aparece en la línea de órdenes de ningún proceso
      Dado una copia del proyecto de prueba
      Y el navegador predeterminado del sistema de prueba es un programa que anota cómo lo llamaron
      Y la prueba lee la línea de órdenes de todos los procesos del sistema cada 10 ms
      Cuando se ejecuta `osr abrir {proyecto}`
      Y pasan 5 s
      Entonces ninguna línea de órdenes leída contiene el token de la sesión

    @SRV-03.3 @recorrido-completo
    Escenario: El token no queda escrito en ningún archivo
      Dado el proyecto de prueba después del recorrido completo de los escenarios del tramo 1
      Entonces ningún archivo del proyecto contiene ninguno de los tokens de las sesiones del recorrido
      Y ningún archivo de la carpeta de estado contiene ninguno de los tokens de las sesiones del recorrido
      Y ningún archivo de la carpeta temporal del usuario contiene ninguno de los tokens de las sesiones del recorrido

    @SRV-03.4
    Escenario: El token de un arranque anterior no sirve
      Dado una carpeta temporal vacía "{tmp}"
      Y osr se arrancó con `osr --sin-navegador` y se cerró de forma ordenada, anotando su token como {token anterior}
      Cuando se ejecuta `osr --sin-navegador`
      Dado un pedido HTTP GET a "/"
      Y el pedido lleva el encabezado "Authorization" con valor "Bearer {token anterior}"
      Cuando se manda el pedido
      Entonces la respuesta tiene estado 401

  @SRV-04
  Regla: Entrada con token y sesión del navegador

    @SRV-04.1
    Escenario: La entrada con token válido pone la cookie de sesión
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y un pedido HTTP GET a "/?token={token}"
      Cuando se manda el pedido
      Entonces la respuesta tiene estado 200
      Y la respuesta trae la interfaz
      Y la respuesta pone la cookie "osr_{P}" con valor "{token}" y los atributos "HttpOnly", "SameSite=Strict" y "Path=/"
      Y la cookie "osr_{P}" no tiene "Expires" ni "Max-Age"

    @SRV-04.1 @interfaz
    Escenario: Después de la entrada la dirección visible queda sin el token
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Cuando el navegador abre "http://127.0.0.1:{P}/?token={token}"
      Entonces la dirección visible de la pestaña es "http://127.0.0.1:{P}/"
      Y la pestaña no se recargó desde la entrada

    @SRV-04.2
    Escenario: Con la cookie de sesión la interfaz se sirve sin el token en el camino
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y un pedido HTTP GET a "/"
      Y el pedido lleva el token válido en la cookie "osr_{P}"
      Cuando se manda el pedido
      Entonces la respuesta tiene estado 200
      Y la respuesta trae la interfaz

    @SRV-04.5 @interfaz
    Escenario: Al abrir con navegador la primera página es el resumen, también al recargar
      # La especificación pide verificar el trato de la cookie Strict y de Sec-Fetch-Site al
      # recargar. La prueba fija como navegador predeterminado del sistema de prueba uno que
      # Playwright controla, para ver la pestaña que abre osr.
      Dado una copia del proyecto de prueba
      Y el navegador predeterminado del sistema de prueba es el navegador de la corrida, controlado por la prueba
      Cuando se ejecuta `osr abrir {proyecto}`
      Entonces la vista visible en la primera página que muestra el navegador es "Resumen del proyecto"
      Y ninguna respuesta de esa pestaña tuvo estado 401 ni 403
      Cuando la persona recarga la pestaña
      Entonces la vista visible es "Resumen del proyecto"

    @SRV-04.3
    Escenario: El encabezado Authorization sin cookie basta
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y un pedido HTTP GET a "/"
      Y el pedido lleva el token válido en el encabezado Authorization
      Cuando se manda el pedido
      Entonces la respuesta tiene estado 200

    @SRV-04.4
    Escenario: Una cookie con el nombre de otro puerto no autentica
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y un pedido HTTP GET a "/"
      Y el pedido lleva el encabezado "Cookie" con valor "osr_{P+1}={token}"
      Cuando se manda el pedido
      Entonces la respuesta tiene estado 401

    @PRY-18.1
    Escenario: La entrada con un token inválido responde 401 con la página de aviso
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y un pedido HTTP GET a "/?token=AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
      Cuando se manda el pedido
      Entonces la respuesta tiene estado 401
      Y el cuerpo de la respuesta es una página HTML que contiene "Esta dirección necesita el enlace que mostró osr al arrancar."

  @SRV-05
  Regla: Token obligatorio en todas las rutas

    @SRV-05.1 @PRY-18.1
    Escenario: Toda ruta sin token responde 401 con el cuerpo de su familia
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la lista completa de rutas que el servidor declara en su propia definición de rutas
      Cuando se manda a cada ruta de la lista, con cada método que acepta, un pedido con Host propio, con "Origin" con valor "http://127.0.0.1:{P}" si el método no es GET ni HEAD, y sin cookie ni token
      Entonces cada respuesta tiene estado 401
      Y cada respuesta tiene estos encabezados
        | encabezado       | valor              |
        | WWW-Authenticate | Bearer realm="osr" |
      Y el cuerpo de cada respuesta de la familia "/api/" es el JSON
        """
        {"error": "no_autorizado", "mensaje": "Falta el token de la sesión o no es válido."}
        """
      Y el cuerpo de cada respuesta de las demás familias es una página HTML que contiene "Esta dirección necesita el enlace que mostró osr al arrancar."

    @SRV-05.2
    Esquema del escenario: Una ruta inexistente responde 401 sin token y 404 con token
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y un pedido HTTP GET a "/no-existe"
      Y el pedido <token>
      Cuando se manda el pedido
      Entonces la respuesta tiene estado <estado>

      Ejemplos:
        | token                                                 | estado |
        | no lleva token                                        | 401    |
        | lleva el token válido en el encabezado Authorization | 404    |

    @SRV-05.3
    Escenario: Un POST sin token que crearía un participante no cambia nada
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y se tomó la huella de cada archivo del proyecto
      Y un pedido HTTP POST a la ruta de la API que crea un participante, con el cuerpo de un participante "E03"
      Y el pedido lleva el encabezado "Origin" con valor "http://127.0.0.1:{P}"
      Cuando se manda el pedido
      Entonces la respuesta tiene estado 401
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas

  @SRV-06
  Regla: Validación de Host

    @SRV-06.1 @PRY-18.1
    Escenario: Un Host ajeno recibe 403 aunque traiga el token
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y un pedido HTTP GET a "/"
      Y el pedido lleva el token válido en el encabezado Authorization
      Y el pedido lleva el encabezado "Host" con valor "ejemplo.invalid:{P}"
      Cuando se manda el pedido
      Entonces la respuesta tiene estado 403
      Y el cuerpo de la respuesta es el JSON
        """
        {"error": "host_no_permitido", "mensaje": "El pedido no viene de esta computadora."}
        """

    @SRV-06.2
    Escenario: Un Host con otro puerto recibe 403
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y un pedido HTTP GET a "/"
      Y el pedido lleva el token válido en el encabezado Authorization
      Y el pedido lleva el encabezado "Host" con valor "127.0.0.1:{P+1}"
      Cuando se manda el pedido
      Entonces la respuesta tiene estado 403

    @SRV-06.3
    Esquema del escenario: Los Host propios se aceptan sin distinguir mayúsculas
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y un pedido HTTP GET a "/"
      Y el pedido lleva el token válido en el encabezado Authorization
      Y el pedido lleva el encabezado "Host" con valor "<host>"
      Cuando se manda el pedido
      Entonces la respuesta tiene estado 200

      Ejemplos:
        | host            |
        | LOCALHOST:{P}   |
        | localhost:{P}   |
        | 127.0.0.1:{P}   |
        | [::1]:{P}       |

    @SRV-06.4
    Escenario: Un pedido sin Host recibe 403
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y un pedido HTTP GET a "/"
      Y el pedido lleva el token válido en el encabezado Authorization
      Y el pedido no lleva el encabezado "Host"
      Cuando se manda el pedido
      Entonces la respuesta tiene estado 403

  @SRV-07
  Regla: Validación de Origin y de Sec-Fetch-Site

    @SRV-07.1 @PRY-18.1
    Esquema del escenario: Un POST desde otro origen recibe 403 y no cambia nada
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y se tomó la huella de cada archivo del proyecto
      Y un pedido HTTP POST a la ruta de la API que crea un participante, con el cuerpo de un participante "E03"
      Y el pedido lleva el token válido en el encabezado Authorization
      Y el pedido lleva el encabezado "Origin" con valor "<origen>"
      Cuando se manda el pedido
      Entonces la respuesta tiene estado 403
      Y el cuerpo de la respuesta es el JSON
        """
        {"error": "origen_no_permitido", "mensaje": "El pedido viene de otra página."}
        """
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas

      Ejemplos:
        | origen                  |
        | http://ejemplo.invalid  |
        | null                    |
        | http://127.0.0.1:{P+1}  |

    @SRV-07.2
    Escenario: Un POST sin Origin recibe 403
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y se tomó la huella de cada archivo del proyecto
      Y un pedido HTTP POST a la ruta de la API que crea un participante, con el cuerpo de un participante "E03"
      Y el pedido lleva el token válido en el encabezado Authorization
      Cuando se manda el pedido
      Entonces la respuesta tiene estado 403
      Y ningún archivo del proyecto cambió respecto de las huellas tomadas

    @SRV-07.3
    Esquema del escenario: Un GET con Sec-Fetch-Site de otro sitio recibe 403
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y un pedido HTTP GET a "/"
      Y el pedido lleva el token válido en el encabezado Authorization
      Y el pedido lleva el encabezado "Sec-Fetch-Site" con valor "<valor>"
      Cuando se manda el pedido
      Entonces la respuesta tiene estado 403

      Ejemplos:
        | valor      |
        | cross-site |
        | same-site  |

    @SRV-07.4 @interfaz
    Escenario: Los pedidos legítimos de la interfaz pasan las tres reglas
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Cuando la persona pulsa "Agregar participante" y acepta la propuesta "E03"
      Entonces la lista de participantes de la interfaz muestra "E03"
      Y el archivo "participantes/participantes.csv" contiene una fila que empieza con "E03,"

    @SRV-07.5 @interfaz
    Escenario: Una página en otro puerto de loopback no obtiene datos
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y la prueba sirve en "http://127.0.0.1:{P+1}/" una página que hace fetch con credentials "include" a "http://127.0.0.1:{P}/api/" para cada ruta GET de la API
      Cuando el navegador abre esa página en otra pestaña de la misma sesión
      Entonces la página no obtiene ningún dato del proyecto
      Y ninguna respuesta del servidor a esos pedidos tuvo estado 200

  @SRV-08
  Regla: Sin CORS

    @SRV-08.1
    Escenario: Una verificación previa desde otro origen recibe 403 sin encabezados de CORS
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la lista completa de rutas que el servidor declara en su propia definición de rutas
      Cuando se manda a cada ruta de la lista un pedido OPTIONS con "Origin" con valor "http://ejemplo.invalid" y "Access-Control-Request-Method" con valor "POST"
      Entonces cada respuesta tiene estado 403
      Y ninguna respuesta tiene un encabezado que empiece con "Access-Control-"

    @SRV-08.2 @recorrido-completo
    Escenario: Ninguna respuesta del recorrido completo tiene encabezados de CORS
      Dado las respuestas HTTP registradas durante el recorrido completo de los escenarios del tramo 1
      Entonces ninguna respuesta tiene un encabezado que empiece con "Access-Control-"

  @SRV-09
  Regla: Encabezados de todas las respuestas

    @SRV-09.1
    Esquema del escenario: Cada estado de respuesta lleva los siete encabezados
      Dado una copia del proyecto de prueba
      Y el audio de E02 está eliminado según TRA-23
      Y osr está corriendo con el proyecto abierto
      Y un pedido HTTP <método> a "<camino>"
      Y el pedido <token>
      Y el pedido lleva el encabezado "<encabezado>" con valor "<valor>"
      Cuando se manda el pedido
      Entonces la respuesta tiene estado <estado>
      Y la respuesta tiene estos encabezados
        | encabezado                   | valor |
        | Content-Security-Policy      | default-src 'none'; script-src 'self'; style-src 'self'; img-src 'self' data: blob:; media-src 'self' blob:; font-src 'self'; connect-src 'self'; form-action 'self'; base-uri 'none'; frame-ancestors 'none' |
        | X-Content-Type-Options       | nosniff |
        | Referrer-Policy              | no-referrer |
        | Cache-Control                | no-store |
        | Cross-Origin-Opener-Policy   | same-origin |
        | Cross-Origin-Resource-Policy | same-origin |
        | X-Frame-Options              | DENY |

      Ejemplos:
        | método  | camino     | token                                                 | encabezado | valor                        | estado |
        | GET     | /          | lleva el token válido en el encabezado Authorization | Accept     | text/html                    | 200    |
        | GET     | /audio/E01 | lleva el token válido en el encabezado Authorization | Range      | bytes=0-99                   | 206    |
        | GET     | /          | no lleva token                                        | Accept     | text/html                    | 401    |
        | GET     | /          | lleva el token válido en el encabezado Authorization | Host       | ejemplo.invalid:{P}          | 403    |
        | GET     | /no-existe | lleva el token válido en el encabezado Authorization | Accept     | */*                          | 404    |
        | OPTIONS | /          | lleva el token válido en el encabezado Authorization | Origin     | http://127.0.0.1:{P}         | 405    |
        | GET     | /audio/E02 | lleva el token válido en el encabezado Authorization | Accept     | */*                          | 410    |
        | GET     | /audio/E01 | lleva el token válido en el encabezado Authorization | Range      | bytes={tamaño de E01.ogg}-   | 416    |

    @SRV-09.1
    Escenario: Un OPTIONS autenticado desde el origen propio responde 405 con Allow
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y un pedido HTTP OPTIONS a "/"
      Y el pedido lleva el token válido en el encabezado Authorization
      Y el pedido lleva el encabezado "Origin" con valor "http://127.0.0.1:{P}"
      Cuando se manda el pedido
      Entonces la respuesta tiene estado 405
      Y la respuesta tiene el encabezado "Allow"

    @SRV-09.2 @interfaz @recorrido-completo
    Escenario: La consola del navegador no registra violaciones de la política de contenido
      Dado los mensajes de consola de Firefox y de Chromium registrados durante los escenarios @interfaz del recorrido completo
      Entonces ningún mensaje es una violación de la política de contenido

  @SRV-10
  Regla: Familias de rutas y prueba de cobertura total

    @SRV-10.1
    Escenario: Cada ruta rechaza el pedido cuando falla un solo factor
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la lista completa de rutas que el servidor declara en su propia definición de rutas
      Y a la lista se suman los caminos inexistentes "/no-existe", "/api/no-existe" y "/recursos/no-existe.js"
      Cuando se manda la batería de pedidos con un solo factor inválido a cada ruta de la lista y con cada método que acepta
        | factor inválido | token  | Host                 | Origin                                                  |
        | token           | ninguno | 127.0.0.1:{P}       | http://127.0.0.1:{P} si el método no es GET ni HEAD     |
        | Host            | válido  | ejemplo.invalid:{P} | http://127.0.0.1:{P} si el método no es GET ni HEAD     |
        | Origin          | válido  | 127.0.0.1:{P}       | http://ejemplo.invalid                                  |
      Entonces las respuestas con factor inválido "token" tienen estado 401
      Y las respuestas con factor inválido "Host" tienen estado 403
      Y las respuestas con factor inválido "Origin" tienen estado 403

    @SRV-10.2
    Escenario: La batería de pedidos rechazados no cambia el proyecto ni el registro de eventos
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la lista completa de rutas que el servidor declara en su propia definición de rutas
      Y se tomó la huella de cada archivo del proyecto
      Y se guardó la cantidad de líneas del registro de eventos
      Cuando se manda la batería de pedidos con un solo factor inválido a cada ruta de la lista y con cada método que acepta
        | factor inválido | token   | Host                | Origin                                              |
        | token           | ninguno | 127.0.0.1:{P}       | http://127.0.0.1:{P} si el método no es GET ni HEAD |
        | Host            | válido  | ejemplo.invalid:{P} | http://127.0.0.1:{P} si el método no es GET ni HEAD |
        | Origin          | válido  | 127.0.0.1:{P}       | http://ejemplo.invalid                              |
      Entonces ningún archivo del proyecto cambió respecto de las huellas tomadas
      Y el registro de eventos no recibió líneas nuevas

    @SRV-10.3
    Escenario: Cada pedido rechazado deja una línea en el registro técnico
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la lista completa de rutas que el servidor declara en su propia definición de rutas
      Y se guardó la cantidad de líneas del registro técnico
      Cuando se manda la batería de pedidos con un solo factor inválido a cada ruta de la lista y con cada método que acepta
        | factor inválido | token   | Host                | Origin                                              |
        | token           | ninguno | 127.0.0.1:{P}       | http://127.0.0.1:{P} si el método no es GET ni HEAD |
        | Host            | válido  | ejemplo.invalid:{P} | http://127.0.0.1:{P} si el método no es GET ni HEAD |
        | Origin          | válido  | 127.0.0.1:{P}       | http://ejemplo.invalid                              |
      Entonces el registro técnico tiene exactamente una línea nueva por cada pedido rechazado
      Y cada una de esas líneas tiene el método, el camino sin parámetros, el código de estado y el motivo del rechazo

  @SRV-11
  Regla: Audio con pedidos por rangos

    @SRV-11.1
    Escenario: Un rango válido devuelve 206 con los bytes pedidos
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y un pedido HTTP GET a "/audio/E01"
      Y el pedido lleva el token válido en el encabezado Authorization
      Y el pedido lleva el encabezado "Range" con valor "bytes=0-99"
      Cuando se manda el pedido
      Entonces la respuesta tiene estado 206
      Y la respuesta tiene estos encabezados
        | encabezado          | valor                              |
        | Content-Range       | bytes 0-99/{tamaño de E01.ogg}     |
        | Accept-Ranges       | bytes                              |
        | Content-Disposition | inline                             |
        | Content-Type        | audio/ogg                          |
      Y el cuerpo de la respuesta tiene 100 bytes idénticos a los primeros 100 bytes de "identificable/entrevistas/E01/E01.ogg"

    @SRV-11.2
    Escenario: Un rango que empieza después del último byte devuelve 416
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y un pedido HTTP GET a "/audio/E01"
      Y el pedido lleva el token válido en el encabezado Authorization
      Y el pedido lleva el encabezado "Range" con valor "bytes={tamaño de E01.ogg}-"
      Cuando se manda el pedido
      Entonces la respuesta tiene estado 416

    @SRV-11.3
    Esquema del escenario: Sin Range la respuesta es el archivo entero con su tipo
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y un pedido HTTP GET a "/audio/<entrevista>"
      Y el pedido lleva el token válido en el encabezado Authorization
      Cuando se manda el pedido
      Entonces la respuesta tiene estado 200
      Y la respuesta tiene estos encabezados
        | encabezado   | valor  |
        | Content-Type | <tipo> |
      Y la huella del cuerpo de la respuesta es igual a la de "identificable/entrevistas/<entrevista>/<archivo>"

      Ejemplos:
        | entrevista | archivo | tipo      |
        | E01        | E01.ogg | audio/ogg |
        | E02        | E02.m4a | audio/mp4 |

    @PRY-18.1
    Esquema del escenario: Los errores del audio responden con su estado y su cuerpo
      Dado una copia del proyecto de prueba
      Y <preparación>
      Y osr está corriendo con el proyecto abierto
      Y un pedido HTTP GET a "<camino>"
      Y el pedido lleva el token válido en el encabezado Authorization
      Cuando se manda el pedido
      Entonces la respuesta tiene estado <estado>
      Y el cuerpo de la respuesta es el JSON
        """
        {"error": "<error>"}
        """

      Ejemplos:
        | preparación                                                        | camino     | estado | error           |
        | no hay ninguna preparación adicional                               | /audio/E09 | 404    | no_existe       |
        | el audio de E02 está eliminado según TRA-23                        | /audio/E02 | 410    | audio_eliminado |
        | con la app cerrada se borra el archivo "identificable/entrevistas/E01/E01.ogg" | /audio/E01 | 404 | audio_faltante |

  @SRV-12
  Regla: Registro técnico

    @SRV-12.1 @recorrido-completo
    Escenario: El registro técnico no tiene datos de las personas ni secretos
      Dado el proyecto de prueba después del recorrido completo de los escenarios del tramo 1
      Entonces el registro técnico no contiene ninguno de los nombres de la tabla de correspondencia de prueba
      Y el registro técnico no contiene ninguna frase de las transcripciones de prueba
      Y el registro técnico no contiene ninguno de los tokens de las sesiones del recorrido
      Y el registro técnico no contiene la ruta del proyecto
      Y el registro técnico no contiene el nombre original de ningún archivo importado

    @SRV-12.2
    Escenario: La rotación limita el registro técnico a 10.000.000 bytes
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Cuando se mandan 300.000 pedidos sin token a caminos distintos de 200 caracteres
      Entonces en la carpeta de estado solo existen "registro.log" y "registro.log.1" como archivos del registro técnico
      Y los tamaños de "registro.log" y "registro.log.1" suman 10.000.000 bytes o menos

    @SRV-12.3
    Esquema del escenario: La carpeta de estado y el registro tienen sus permisos con cualquier umask
      Dado una carpeta temporal vacía "{tmp}"
      Y la variable de entorno "XDG_STATE_HOME" vale "{tmp}/estado"
      Y la umask del proceso osr es <umask>
      Cuando se ejecuta `osr --sin-navegador`
      Entonces los permisos de "{tmp}/estado/osr" son 0700
      Y los permisos de "{tmp}/estado/osr/registro.log" son 0600

      Ejemplos:
        | umask |
        | 000   |
        | 022   |
        | 077   |

  @SRV-13
  Regla: Permisos de archivos

    @SRV-13.1
    Escenario: Con umask 000 cada cosa queda con sus permisos
      Dado la umask del proceso osr es 000
      Y un proyecto nuevo creado con `osr nuevo`
      Y el participante "E01" está registrado
      Y el participante "E01" tiene consentimiento registrado con grabar "otorgado"
      Y la entrevista de "E01" está creada
      Cuando se importa en la entrevista "E01" el audio de prueba "E01.ogg"
      Y se importan en la entrevista "E01" los archivos de la cadena de prueba de E01 con el modelo "large-v3-turbo"
      Entonces cada carpeta del proyecto tiene permisos 0700
      Y cada archivo gestionado modificable del proyecto tiene permisos 0600
      Y los permisos de "identificable/entrevistas/E01/E01.ogg" son 0400
      Y cada archivo bajo "identificable/entrevistas/E01/origen/" tiene permisos 0400

    @SRV-13.2 @interfaz
    Escenario: Una carpeta que otros pueden leer se avisa y se corrige a pedido
      # El mensaje se escribe tal como lo fija la especificación, también con n igual a 1.
      Dado una copia del proyecto de prueba
      Y con la app cerrada la carpeta "identificable/" queda con permisos 0755
      Y se guardó la cantidad de líneas del registro de eventos
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Entonces la interfaz muestra "1 archivos o carpetas del proyecto los pueden leer otras cuentas de esta computadora."
      Y los permisos de "identificable/" son 0755
      Cuando la persona pulsa "Corregir permisos"
      Entonces los permisos de "identificable/" son 0700
      Y el registro de eventos tiene exactamente una línea nueva de tipo "permisos_corregidos" cuyo detalle contiene "1"

    @SRV-13.3 @interfaz
    Escenario: Un archivo de la persona no cuenta para el aviso y conserva sus permisos
      Dado una copia del proyecto de prueba
      Y con la app cerrada la carpeta "identificable/" queda con permisos 0755
      Y con la app cerrada se crea en la raíz el archivo "notas-mias.txt" con permisos 0644
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Entonces la interfaz muestra "1 archivos o carpetas del proyecto los pueden leer otras cuentas de esta computadora."
      Cuando la persona pulsa "Corregir permisos"
      Entonces los permisos de "notas-mias.txt" son 0644

  @SRV-14
  Regla: Sin red saliente

    @SRV-14.1 @recorrido-completo
    Escenario: La suite completa pasa en un espacio de nombres de red con solo loopback
      # El escenario de SRV-02.2 agrega una interfaz local sin rutas de salida dentro del
      # mismo espacio de nombres, sin dar ninguna salida.
      Dado la suite completa de escenarios del tramo 1 corre con osr en un espacio de nombres de red que solo tiene loopback
      Entonces todos los escenarios pasan

    @SRV-14.2 @recorrido-completo
    Escenario: osr no intenta ninguna conexión saliente ni consulta de nombres
      Dado la suite completa de escenarios del tramo 1 corre con osr en un espacio de nombres de red que solo tiene loopback
      Y la prueba registra cada intento de osr y de sus procesos hijos de conectar o enviar datos a una dirección fuera de "127.0.0.0/8" y de "::1"
      Y la prueba registra cada consulta de nombres de osr y de sus procesos hijos
      Entonces la cantidad de intentos registrados es 0
      Y la cantidad de consultas de nombres registradas es 0

    @SRV-14.3 @interfaz @recorrido-completo
    Escenario: La página no pide nada fuera de su origen
      Dado los pedidos de la página registrados en Firefox y en Chromium durante los escenarios @interfaz del recorrido completo
      Entonces cada pedido va al origen "http://127.0.0.1:{P}" de su sesión

  @SRV-15
  Regla: Cierre ordenado

    @SRV-15.1 @interfaz
    Escenario: Una edición hecha medio segundo antes de SIGTERM queda en el disco
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y la transcripción de E01 está abierta en el editor
      Cuando la persona escribe "x" al final del texto del segmento "s0001"
      Y 0,5 s después se envía SIGTERM al proceso osr
      Entonces osr termina con código de salida 0
      Y el segmento "s0001" de "identificable/entrevistas/E01/transcripcion.json" tiene un texto que termina en "x"

    @SRV-15.2
    Escenario: osr termina en 3 s o menos y borra el bloqueo
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Cuando se envía SIGTERM al proceso osr
      Entonces osr termina con código de salida 0 a más tardar 3 s después
      Y no existe ".osr/bloqueo"
      Y el registro técnico tiene una línea nueva que anota el cierre

    @SRV-15.3 @interfaz @PRY-18.1
    Escenario: La pestaña avisa que osr se cerró y deshabilita la edición
      Dado una copia del proyecto de prueba
      Y osr está corriendo con el proyecto abierto
      Y la interfaz está abierta en el navegador
      Y la transcripción de E01 está abierta en el editor
      Cuando se envía SIGTERM al proceso osr
      Y osr termina con código de salida 0
      Entonces a más tardar 7 s después la interfaz muestra "Open Social Research se cerró. Lo guardado está en la carpeta del proyecto; los cambios de los últimos segundos pueden no haberse guardado."
      Cuando la persona escribe "x" al final del texto del segmento "s0001"
      Entonces el texto del segmento "s0001" en la interfaz no cambió

  @SRV-16
  Regla: Memoria y arranque

    @SRV-16.1 @equipo-autor
    Esquema del escenario: Cada medida cumple su meta en las 5 repeticiones
      # Las cifras valen solo en el equipo del autor (Linux x86-64, Arch, disco SSD) con
      # Firefox y Chromium estables, por eso se corre ahí y no en la integración continua.
      Dado el equipo del autor
      Cuando se mide 5 veces "<medida>" en la condición "<condición>"
      Entonces cada una de las 5 repeticiones da <meta>

      Ejemplos:
        | medida                     | condición                                                                                                                                                  | meta          |
        | memoria residente de osr   | sin proyecto, 10 s después de arrancar                                                                                                                     | 15 MB o menos |
        | memoria residente de osr   | con el proyecto de prueba abierto, en el resumen y después de abrir su transcripción más larga y reproducir 10 s                                           | 50 MB o menos |
        | memoria residente de osr   | con el proyecto real del autor abierto, en el resumen y después de abrir su transcripción más larga y reproducir 10 s                                      | 50 MB o menos |
        | arranque en frío           | desde lanzar `osr abrir {proyecto}` hasta que el resumen se ve y acepta el teclado, con la caché de archivos vaciada antes de cada repetición                    | 3 s o menos   |

    @SRV-16.2
    Escenario: Las mediciones quedan publicadas en docs/mediciones.md
      Dado el archivo "docs/mediciones.md" del repositorio
      Entonces tiene una fila por cada medida de SRV-16
      Y cada fila tiene 5 valores, la versión, la máquina y la fecha
