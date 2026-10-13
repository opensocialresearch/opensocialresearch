# Especificación del módulo servidor (SRV)

Tramo 1 de la v0.1, borrador para aprobación del autor.

## 1. Propósito

Este módulo define el programa `osr` como servidor local, cómo arranca, en qué
direcciones escucha, cómo exige un token en cada pedido y cómo rechaza a
cualquier página web u otra cuenta de la computadora que intente usarlo.
También fija los permisos de los archivos, el registro técnico, la ausencia
total de conexiones salientes y las cifras de memoria y arranque.

### Requisitos de PRODUCTO.md que cubre

| Sección | Requisito (texto corto) | IDs |
|---|---|---|
| 6.0 | «el binario `osr` la abre, levanta el servidor y abre el navegador» | SRV-01 |
| 6.0 | «Servidor solo en 127.0.0.1 y ::1, puerto aleatorio, token aleatorio de 128 bits o más por arranque en el 100 % de las rutas, validación de Host y Origin, sin CORS» | SRV-02 a SRV-10 |
| 6.0 | «La v0.1 no contiene código de red saliente, y una prueba automática corre la app en un espacio de nombres de red sin salida» | SRV-14 |
| 7, Red y seguridad | «Conexiones salientes en v0.1. Ninguna», «Sin conexión. El 100 % de las funciones […]», «Registros técnicos. Sin texto de transcripciones, sin nombres de archivos de la zona identificable […] rotación con máximo de 10 MB» | SRV-12, SRV-14 |
| 7, Memoria y arranque | «Orquestador en reposo sin proyecto abierto. 15 MB», «con proyecto abierto y sin motores. 50 MB», «Arranque en frío hasta interfaz usable […] 3 s o menos» | SRV-16 |
| 7, Hardware y plataformas | «Linux x86-64, el equipo del autor» y «Firefox y Chromium estables» | SRV-16 y todas las pruebas de navegador |
| 9, amenazas | «Computadora compartida con otra cuenta del sistema» y «Página web maliciosa que alcanza el servidor local» | SRV-02, SRV-03, SRV-05 a SRV-09, SRV-13 |
| 5, terminación | «los intentos sin token, con un Host ajeno y desde otro origen fallan en el 100 % de las rutas, la de ejecución sin red» | SRV-10, SRV-14 |
| 6.5 | archivos de solo lectura y permisos de lo identificable | SRV-13 |

## 2. Definiciones

| Término | Definición |
|---|---|
| `P` | Puerto en el que escucha el servidor en un arranque dado. |
| Origen propio | Cualquiera de `http://127.0.0.1:P`, `http://[::1]:P` y `http://localhost:P`. |
| Host propio | Cualquiera de `127.0.0.1:P`, `[::1]:P` y `localhost:P`, comparado sin distinguir mayúsculas. |
| Token | Secreto de la sesión, generado en cada arranque. |
| Pedido autenticado | Pedido que lleva el token válido por uno de los canales de SRV-04. |
| Ruta | Combinación de método HTTP y patrón de camino que el servidor atiende. |
| Familia de rutas | Grupo de rutas con el mismo prefijo de camino (SRV-10). |
| Registro técnico | Archivo de diagnóstico de `osr`, fuera del proyecto (SRV-12). |
| Carpeta de estado | `$XDG_STATE_HOME/osr/`, o `~/.local/state/osr/` si la variable no está definida. |

## 3. Requisitos

### SRV-01 Arranque y apertura del navegador

**Comportamiento.** `osr` tiene tres formas de arranque.

| Orden | Efecto |
|---|---|
| `osr` | Levanta el servidor sin proyecto. La interfaz muestra la lista `Investigaciones` de la carpeta de la aplicación, con `Abrir` en cada una (PRY-20), y `Crear investigación`, que pide solo el nombre (PRY-19). Mientras la carpeta de la aplicación no tenga ninguna investigación, muestra la bienvenida de PRY-23 [D-50, D-53]. |
| `osr abrir <ruta>` o `osr <ruta>` | Abre el proyecto según PRY-04. |
| `osr nuevo <ruta> [--nombre <texto>]` | Crea y abre el proyecto según PRY-01. |

Las tres aceptan `--sin-navegador`. Al quedar listo, `osr` escribe en la salida
estándar una sola línea, `Open Social Research está abierto en
http://127.0.0.1:<P>/?token=<token>`, y salvo `--sin-navegador` abre el
navegador predeterminado del sistema en esa sesión. Después sigue corriendo
hasta recibir SIGINT o SIGTERM (SRV-15).

**Errores.**

| Condición | Código de salida | Mensaje |
|---|---|---|
| Orden u opción desconocida | 2 | `osr: no se reconoce <texto>. Las órdenes son osr, osr abrir <carpeta> y osr nuevo <carpeta>.` |
| Errores del proyecto | 3, 4 o 6 | Los de PRY-01, PRY-03, PRY-04 y PRY-05 |
| Sin puerto libre | 5 | El de SRV-02 |
| No se pudo abrir el navegador | sigue corriendo | `osr: no se pudo abrir el navegador. Hay que abrir a mano la dirección de arriba.` en la salida de error |

**Criterios de aceptación.**
- SRV-01.1 `osr --sin-navegador` escribe exactamente una línea en la salida
  estándar con el formato indicado, y un pedido a esa dirección muestra la
  pantalla con `Abrir proyecto` y `Crear proyecto`.
- SRV-01.2 `osr --opcion-falsa` termina con código 2 y el mensaje de la tabla.
- SRV-01.3 Sin navegador disponible en el sistema de prueba, `osr` muestra el
  mensaje de la tabla y la dirección impresa sigue funcionando.

### SRV-02 Solo loopback y puerto aleatorio

**Comportamiento.** El servidor escucha en `127.0.0.1:P` y en `[::1]:P`, con el
mismo `P` en las dos. `P` lo asigna el sistema entre sus puertos efímeros y no
se lee de ninguna configuración. Si `P` está libre en IPv4 pero ocupado en
IPv6, la app suelta los dos y prueba otro, hasta 10 intentos. Si el sistema no
tiene IPv6 de loopback, escucha solo en `127.0.0.1:P` y lo anota en el registro
técnico. Nunca escucha en `0.0.0.0`, `::` ni en ninguna dirección que no sea de
loopback.

**Errores.** Tras 10 intentos fallidos, código 5 y `osr: no se encontró un
puerto libre en las direcciones locales.`

**Criterios de aceptación.**
- SRV-02.1 Con `osr` corriendo, la lista de sockets en escucha del proceso
  tiene exactamente `127.0.0.1:P` y `[::1]:P`.
- SRV-02.2 Una conexión al puerto `P` desde la dirección de red local de la
  máquina, que no es de loopback, es rechazada.
- SRV-02.3 En 5 arranques seguidos no se usa 5 veces el mismo `P`.
- SRV-02.4 En un sistema de prueba sin IPv6, `osr` escucha en `127.0.0.1:P`,
  funciona, y el registro técnico tiene la línea `sin IPv6 de loopback`.

### SRV-03 Token por arranque

**Comportamiento.** En cada arranque `osr` genera un token nuevo de 256 bits
con el generador criptográfico del sistema, codificado en base64url sin relleno,
lo que da 43 caracteres del alfabeto `A-Z a-z 0-9 - _`. PRODUCTO.md pide 128
bits o más, y 256 cuesta lo mismo. El token vive solo en la memoria del
proceso. No aparece en los argumentos de ningún proceso que `osr` lance, porque
en Linux esos argumentos los puede leer cualquier cuenta, ni en el registro
técnico, ni en el registro de eventos, ni en ningún archivo del proyecto. Si
para abrir el navegador sin poner el token en los argumentos hace falta un
archivo, es de un solo uso, con permisos 0600, en una carpeta 0700 del usuario,
y se borra 30 s o menos después de creado.

**Criterios de aceptación.**
- SRV-03.1 Dos arranques seguidos producen tokens distintos, de 43 caracteres
  y solo del alfabeto indicado.
- SRV-03.2 Durante el arranque con navegador, ninguna línea de órdenes de
  ningún proceso del sistema contiene el token.
- SRV-03.3 Después de recorrer todos los escenarios del tramo 1, el token de
  cada sesión no aparece en ningún archivo del proyecto, de la carpeta de
  estado ni de la carpeta temporal del usuario.
- SRV-03.4 Un token de un arranque anterior recibe 401 en el arranque
  siguiente.

### SRV-04 Entrada con token y sesión del navegador [D-12]

**Comportamiento.** Hay dos canales para el token.

- **Cookie de sesión.** `GET /?token=<token>` con el token válido responde 200
  con la interfaz y con `Set-Cookie: osr_<P>=<token>; HttpOnly;
  SameSite=Strict; Path=/`, sin fecha de vencimiento. La interfaz quita el
  parámetro de la dirección visible sin recargar la página. Desde entonces los
  pedidos de la pestaña son del mismo origen y van autenticados por esa
  cookie. No se usa una redirección, porque cuando la entrada viene de un
  archivo local o de otra aplicación el navegador trata toda la cadena como
  pedida desde otro sitio, y la página siguiente llegaría sin la cookie
  `Strict` y con `Sec-Fetch-Site: cross-site`. El nombre lleva el puerto para
  que dos sesiones en puertos distintos no se pisen. Las cookies con otro
  nombre se ignoran.
- **Encabezado.** `Authorization: Bearer <token>`, para las pruebas y para los
  pedidos que la interfaz haga por programa.

**Errores.** `GET /?token=<x>` con `<x>` inválido responde 401 con la página de
SRV-05.

**Riesgo residual.** Los navegadores mandan las cookies de `127.0.0.1` a
cualquier puerto de esa dirección. Un proceso de otra cuenta que escuche en
otro puerto de loopback podría recibir la cookie si el navegador lo visita
desde una página del mismo sitio. Lo acotan `SameSite=Strict`, que no se manda
desde otros sitios, y el token nuevo en cada arranque. Se registra
en D-12.

**Criterios de aceptación.**
- SRV-04.1 `GET /?token=<válido>` responde 200 con la interfaz y la cookie
  `osr_<P>` con los atributos `HttpOnly`, `SameSite=Strict` y `Path=/`, y la
  dirección visible de la pestaña queda sin `?token=`.
- SRV-04.2 Después de esa respuesta, `GET /` con la cookie responde 200 con la
  interfaz.
- SRV-04.5 Al arrancar `osr abrir <ruta>` sin `--sin-navegador`, en Firefox y
  en Chromium, la primera página que se ve es el resumen del proyecto, no un
  401 ni un 403, y recargar la pestaña vuelve a mostrar el resumen
  [verificar el trato de la cookie `Strict` y de `Sec-Fetch-Site` al recargar
  en los dos navegadores].
- SRV-04.3 Un pedido con `Authorization: Bearer <válido>` y sin cookie es
  atendido.
- SRV-04.4 Una cookie `osr_<otro puerto>=<válido>` sin la de `P` recibe 401.

### SRV-05 Token obligatorio en todas las rutas

**Comportamiento.** Todo pedido a cualquier ruta, incluidos los recursos
estáticos, el audio y las rutas inexistentes, tiene que estar autenticado. La
única excepción es `GET /?token=<token>`, que trae el token en el camino. Un
pedido sin token válido recibe 401 con `WWW-Authenticate: Bearer realm="osr"`,
sin ejecutar nada ni leer ningún archivo del proyecto. El cuerpo depende de la
familia.

| Familia | Cuerpo del 401 |
|---|---|
| `/api/` | `{"error": "no_autorizado", "mensaje": "Falta el token de la sesión o no es válido."}` |
| Las demás | Página HTML con `Esta dirección necesita el enlace que mostró osr al arrancar.` |

Un pedido con Host propio, y con `Origin` propio cuando el método no es GET
ni HEAD, recibe 401 sin token aunque la ruta no exista, y 404 con token, para
que no se pueda averiguar qué rutas existen sin el token. Un pedido que falla
antes la validación de SRV-06 o de SRV-07 recibe el 403 de esas reglas.

**Criterios de aceptación.**
- SRV-05.1 Para cada ruta de la lista de SRV-10, un pedido con Host propio,
  con `Origin` propio si el método no es GET ni HEAD, y sin cookie ni
  encabezado de token recibe 401 con el cuerpo de su familia.
- SRV-05.2 `GET /no-existe` sin token recibe 401, y con token recibe 404.
- SRV-05.3 Un `POST` con Host y `Origin` propios y sin token a una ruta que
  crea un participante recibe 401 y no cambia
  ningún archivo del proyecto, verificado por huella de cada archivo antes y
  después.

### SRV-06 Validación de Host

**Comportamiento.** El encabezado `Host` tiene que ser un Host propio. Si falta
o es otro, la respuesta es 403 con `{"error": "host_no_permitido", "mensaje":
"El pedido no viene de esta computadora."}`, antes de mirar el token. Esto
bloquea el reenlace de DNS, en el que una página hace que su propio nombre
apunte a 127.0.0.1.

**Criterios de aceptación.**
- SRV-06.1 Un pedido con token válido y `Host: ejemplo.invalid:P` recibe 403
  con `host_no_permitido`.
- SRV-06.2 Un pedido con token válido y `Host: 127.0.0.1:<P+1>` recibe 403.
- SRV-06.3 `Host: LOCALHOST:P` con token válido es atendido.
- SRV-06.4 Un pedido sin `Host` recibe 403.

### SRV-07 Validación de Origin y de Sec-Fetch-Site

**Comportamiento.** Después de validar el Host y antes del token, el servidor
aplica tres reglas. Si alguna falla, la respuesta es 403 con `{"error":
"origen_no_permitido", "mensaje": "El pedido viene de otra página."}`.

1. Si el pedido trae `Origin`, tiene que ser un origen propio. `Origin: null`
   falla.
2. Si el método no es GET ni HEAD, el pedido tiene que traer `Origin`
   [verificar que Firefox y Chromium estables lo mandan en todos los pedidos
   del mismo origen con esos métodos, con una prueba de Playwright en los dos].
3. Si el pedido trae `Sec-Fetch-Site`, su valor tiene que ser `same-origin` o
   `none`. Se exceptúa la entrada `GET /?token=<token>` con `Sec-Fetch-Mode:
   navigate`, para que el navegador pueda llegar desde un archivo local o
   desde la terminal [verificar los valores que mandan Firefox y Chromium
   estables en la entrada, en los pedidos de la pestaña y al recargar].

**Criterios de aceptación.**
- SRV-07.1 Un `POST` con token válido y `Origin: http://ejemplo.invalid`
  recibe 403 con `origen_no_permitido` y no cambia ningún archivo.
- SRV-07.2 Un `POST` con token válido y sin `Origin` recibe 403.
- SRV-07.3 Un `GET` con token válido y `Sec-Fetch-Site: cross-site` recibe
  403, y con `same-site` también.
- SRV-07.4 Desde la interfaz abierta en Firefox y en Chromium, crear un
  participante funciona, lo que prueba que los pedidos legítimos pasan las tres
  reglas.
- SRV-07.5 Una página servida en otro puerto de 127.0.0.1 que hace `fetch` con
  credenciales a `http://127.0.0.1:P/api/...` no obtiene datos, en Firefox y en
  Chromium.

### SRV-08 Sin CORS

**Comportamiento.** Ninguna respuesta del servidor incluye un encabezado que
empiece con `Access-Control-`. Los pedidos `OPTIONS` siguen las mismas reglas
que los demás, así que una verificación previa desde otro origen recibe 403
por SRV-07, y un `OPTIONS` autenticado desde el origen propio recibe 405 con el
encabezado `Allow`.

**Criterios de aceptación.**
- SRV-08.1 Para cada ruta de SRV-10, un `OPTIONS` con `Origin:
  http://ejemplo.invalid` y `Access-Control-Request-Method: POST` recibe 403 y
  ningún encabezado `Access-Control-`.
- SRV-08.2 En todas las respuestas de todos los escenarios del tramo 1 no
  aparece ningún encabezado `Access-Control-`.

### SRV-09 Encabezados de todas las respuestas

**Comportamiento.** Toda respuesta, también las de error, lleva estos
encabezados.

| Encabezado | Valor |
|---|---|
| `Content-Security-Policy` | `default-src 'none'; script-src 'self'; style-src 'self'; img-src 'self' data: blob:; media-src 'self' blob:; font-src 'self'; connect-src 'self'; form-action 'self'; base-uri 'none'; frame-ancestors 'none'` [verificar que la interfaz compilada no necesita estilos en línea] |
| `X-Content-Type-Options` | `nosniff` |
| `Referrer-Policy` | `no-referrer` |
| `Cache-Control` | `no-store` |
| `Cross-Origin-Opener-Policy` | `same-origin` |
| `Cross-Origin-Resource-Policy` | `same-origin` |
| `X-Frame-Options` | `DENY` |

`Cache-Control: no-store` evita que el navegador guarde en su caché respuestas
con nombres o texto de entrevistas, y la política de contenido impide que la
interfaz cargue nada de fuera de su origen.

**Criterios de aceptación.**
- SRV-09.1 Las respuestas 200, 206, 401, 403, 404, 405, 410 y 416 de las
  pruebas llevan los siete encabezados con exactamente esos valores.
- SRV-09.2 Con la interfaz abierta, la consola del navegador no registra
  ninguna violación de la política de contenido en los escenarios del tramo 1.

### SRV-10 Familias de rutas y prueba de cobertura total

**Comportamiento.** El servidor atiende solo estas familias.

| Familia | Métodos | Contenido |
|---|---|---|
| `/` | GET | La interfaz y la entrada con `?token=` |
| `/recursos/…` | GET | Archivos de la interfaz (código, estilos, íconos) |
| `/api/…` | GET, POST, PUT, DELETE | Datos y acciones en JSON |
| `/audio/<entrevista>` | GET, HEAD | Audio de una entrevista (SRV-11) |
| `/respaldo/<seudonimo>/<n>` | GET | Archivo `<n>` del respaldo del consentimiento, para verlo en el navegador (PAR-08) |

Las rutas concretas de `/api/` las fija la implementación, pero la prueba de
seguridad obtiene la lista completa de la propia definición de rutas del
servidor, sin una lista escrita a mano, de modo que una ruta nueva queda
cubierta sin tocar la prueba. Los caminos de las rutas no llevan nombres de
personas ni nombres originales de archivos, solo seudónimos, identificadores
de entrevista y de segmento.

Cada pedido rechazado por SRV-05, SRV-06 o SRV-07 se anota en el registro
técnico (SRV-12).

**Criterios de aceptación.**
- SRV-10.1 Para el 100 % de las rutas de la lista obtenida del servidor, más
  tres caminos inexistentes, y para cada método que la ruta acepta, la prueba
  hace tres pedidos y en cada uno falla un solo factor mientras los otros dos
  son válidos.
  - Sin token, con Host propio y, si el método no es GET ni HEAD, con
    `Origin` propio. Responde 401.
  - Con token y `Host: ejemplo.invalid:P`. Responde 403.
  - Con token, Host propio y `Origin: http://ejemplo.invalid`. Responde 403.
- SRV-10.2 Durante la prueba anterior ningún archivo del proyecto cambia,
  verificado por huella, y el registro de eventos no recibe líneas nuevas.
- SRV-10.3 Cada pedido rechazado de la prueba deja exactamente una línea en el
  registro técnico con el método, el camino sin parámetros, el código de
  estado y el motivo.

### SRV-11 Audio con pedidos por rangos

**Comportamiento.** `GET /audio/<entrevista>` devuelve los bytes del original
sin transformarlos, con `Accept-Ranges: bytes`, `Content-Disposition: inline` y
el tipo de contenido del formato.

| Formato | `Content-Type` |
|---|---|
| ogg, opus | `audio/ogg` |
| m4a | `audio/mp4` |
| mp3 | `audio/mpeg` |
| wav | `audio/wav` |

Con `Range: bytes=a-b` válido responde 206 con `Content-Range`. Un rango fuera
del archivo responde 416. Sin `Range` responde 200 con el archivo entero. La familia `/respaldo/` sirve
los archivos de respaldo de PAR-08 con las mismas reglas, con
`application/pdf`, `image/jpeg`, `image/png` o `image/heic` para los
documentos e imágenes.

**Errores.**

| Condición | Estado | Cuerpo |
|---|---|---|
| La entrevista no existe | 404 | `{"error": "no_existe"}` |
| El audio se eliminó (TRA-23) | 410 | `{"error": "audio_eliminado"}` |
| El audio falta en disco | 404 | `{"error": "audio_faltante"}` |

**Criterios de aceptación.**
- SRV-11.1 `Range: bytes=0-99` devuelve 206 y 100 bytes idénticos a los
  primeros 100 del original.
- SRV-11.2 Un rango que empieza después del último byte devuelve 416.
- SRV-11.3 La respuesta completa tiene la misma huella que el original.

### SRV-12 Registro técnico

**Comportamiento.** `osr` escribe su registro técnico en
`<carpeta de estado>/registro.log`, con la carpeta en 0700 y el archivo en
0600. Cada línea tiene una marca de tiempo, un nivel (`info`, `aviso` o
`error`) y un mensaje en español. Anota el arranque y el cierre, el puerto, si
hay IPv6 de loopback, la apertura de un proyecto sin su ruta, cada pedido
rechazado y cada error con su detalle técnico.

El registro nunca contiene

- el token ni el valor de ninguna cookie,
- los parámetros de los caminos pedidos,
- rutas absolutas del sistema de archivos ni la ruta del proyecto,
- nombres de archivos de `identificable/` ni nombres originales de archivos,
- nombres de la tabla de correspondencia, ni
- texto de transcripciones, encabezados, consentimientos o notas.

Los caminos de los pedidos rechazados se recortan a 200 caracteres.

**Rotación.** Si escribir una línea haría pasar `registro.log` de 5.000.000
bytes, el archivo se renombra a `registro.log.1`, que reemplaza al anterior, y
se empieza uno nuevo. La suma de los dos nunca pasa de 10.000.000 bytes.

**Criterios de aceptación.**
- SRV-12.1 Después de recorrer todos los escenarios del tramo 1 con los datos
  de prueba, el registro no contiene ningún nombre de la tabla de
  correspondencia de prueba, ninguna frase de las transcripciones de prueba,
  ningún token, la ruta del proyecto ni el nombre original de ningún archivo
  importado.
- SRV-12.2 Después de 300.000 pedidos rechazados con caminos de 200 caracteres,
  existen como máximo `registro.log` y `registro.log.1` y suman 10.000.000
  bytes o menos.
- SRV-12.3 Los permisos de la carpeta de estado son 0700 y los del registro
  0600, con cualquier umask.

### SRV-13 Permisos de archivos

**Comportamiento.** Todo lo que la app crea dentro del proyecto y en la carpeta
de estado lleva estos permisos, sin importar la umask del proceso.

| Qué | Permisos |
|---|---|
| Carpetas | 0700 |
| Archivos gestionados modificables | 0600 |
| Originales y material de `origen/` (PRY-15, TRA-10) | 0400 |

Al abrir un proyecto (PRY-11, paso 7), si la raíz, la carpeta `identificable/`
o algún archivo gestionado tiene permisos para el grupo o para otros, la
interfaz muestra `<n> archivos o carpetas del proyecto los pueden leer otras
cuentas de esta computadora.` con el botón `Corregir permisos`. El botón deja
cada elemento con los permisos de la tabla y anota `permisos_corregidos` con
la cantidad. La app no corrige sin que la persona lo pida y no toca los
permisos de archivos que no son gestionados.

**Criterios de aceptación.**
- SRV-13.1 Con umask 000, un proyecto recién creado y con un audio importado
  tiene sus carpetas en 0700, sus archivos modificables en 0600 y el audio y
  el material de origen en 0400.
- SRV-13.2 Con `identificable/` en 0755, al abrir aparece el aviso con la
  cantidad correcta, y después de `Corregir permisos` la carpeta queda en 0700
  y el registro tiene `permisos_corregidos`.
- SRV-13.3 Un archivo de la persona en la raíz con permisos 0644 no cuenta para
  el aviso y conserva sus permisos después de corregir.

### SRV-14 Sin red saliente

**Comportamiento.** En la v0.1 `osr` no abre ninguna conexión saliente ni hace
ninguna consulta de nombres, nunca. Solo escucha en loopback y responde a los
pedidos que le llegan. La interfaz no pide ningún recurso fuera de su origen.
Abrir el navegador es lo único que `osr` le pide al sistema, y la red que use
el navegador por su cuenta no es parte de la app.

**Criterios de aceptación.**
- SRV-14.1 La suite completa de escenarios del tramo 1 pasa con `osr` corriendo
  en un espacio de nombres de red que solo tiene loopback.
- SRV-14.2 Durante esa corrida se registra todo intento de `osr` y de sus
  procesos hijos de conectar o enviar datos a una dirección fuera de
  127.0.0.0/8 y de ::1, y toda consulta de nombres, y la prueba falla si hay al
  menos uno.
- SRV-14.3 Durante los escenarios de interfaz en Firefox y en Chromium, se
  registran todos los pedidos de la página, y la prueba falla si alguno va a
  un origen que no es el propio.

### SRV-15 Cierre ordenado

**Comportamiento.** Al recibir SIGINT o SIGTERM, `osr` deja de aceptar pedidos
nuevos, termina las escrituras pendientes, borra `.osr/bloqueo`, anota el
cierre en el registro técnico y termina con código 0, todo en 3 s o menos. Una
segunda señal durante el cierre lo termina en el acto, sin riesgo para los
archivos gracias a PRY-12. Cuando la pestaña abierta deja de recibir
respuestas durante 5 s, muestra `Open Social Research se cerró. Lo guardado
está en la carpeta del proyecto; los cambios de los últimos segundos pueden no
haberse guardado.` y deshabilita la edición.

**Criterios de aceptación.**
- SRV-15.1 Una edición hecha 0,5 s antes de SIGTERM está en disco después de
  que `osr` termina con código 0.
- SRV-15.2 `osr` termina en 3 s o menos después de SIGTERM y no queda
  `.osr/bloqueo`.
- SRV-15.3 Después de terminar `osr`, la pestaña muestra el mensaje en 7 s o
  menos y no permite escribir en los segmentos.

### SRV-16 Memoria y arranque

**Comportamiento.** Metas de la v0.1, medidas en el equipo del autor (Linux
x86-64, Arch, disco SSD) con Firefox y Chromium estables.

| Medida | Condición | Meta |
|---|---|---|
| Memoria residente de `osr` | Sin proyecto, 10 s después de arrancar | 15 MB o menos |
| Memoria residente de `osr` | Con el proyecto de prueba abierto y con el proyecto real del autor, en el resumen y después de abrir su transcripción más larga y reproducir 10 s | 50 MB o menos |
| Arranque en frío | Desde lanzar `osr abrir <ruta>` hasta que el resumen se ve y acepta el teclado, con la caché de archivos del sistema vaciada antes de cada repetición | 3 s o menos |

La memoria es la memoria residente de `osr` y de sus hijos según el sistema
operativo, sin el navegador. Cada medida se repite 5 veces y se publica en
`docs/mediciones.md` con la versión, la máquina y la fecha.

**Criterios de aceptación.**
- SRV-16.1 Las 5 repeticiones de cada medida cumplen su meta.
- SRV-16.2 `docs/mediciones.md` tiene una fila por medida con los 5 valores, la
  versión, la máquina y la fecha.

## 4. Fuera de alcance de este tramo

- Instancia única, indicador de que la app está corriendo, botón para cerrarla,
  cierre automático sin pestañas y manejo del puerto ocupado sin mensajes
  técnicos (v1.0).
- Archivos de conexión de los motores de análisis con firma HMAC (v1.0).
- Indicador visible de conexión saliente en curso, porque en la v0.1 no existe
  ninguna (v1.0).
- Botón para copiar un diagnóstico sin rutas ni contenido (v1.0).
- Abrir la app en una ventana propia en modo aplicación (decisión de la v1.0).
- Windows y macOS.

## 5. Decisiones

### D-12 Canal del token en el navegador

- **Pregunta.** ¿Cómo se lleva el token en los pedidos del navegador después
  de la entrada?
- **Opciones.** (a) Cookie `HttpOnly` y `SameSite=Strict` con el puerto en el
  nombre, puesta por una entrada que responde 200 sin redirigir, más el
  encabezado `Authorization` para pedidos por programa. (a') Lo mismo con
  `SameSite=Lax` y una redirección 303 a `/`, en cuyo caso la protección de los
  métodos que modifican descansa solo en las reglas 1 y 2 de SRV-07 y ninguna
  ruta GET puede tener efectos. (b)
  Solo encabezado `Authorization`, con el token guardado en la memoria de la
  pestaña, lo que obliga a pedir el audio por programa en lugar de darle una
  dirección al reproductor y complica el avance por rangos. (c) Token en cada
  dirección como parámetro, que queda en el historial del navegador.
- **Decisión (2026-10-11).** (a). Es lo que hace Jupyter, el reproductor de
  audio funciona con rangos sin trabajo extra, y el riesgo residual, que la
  cookie se mande a otro puerto de loopback, solo se da si otra cuenta escucha
  en loopback y la persona navega hasta ahí desde la propia app. Se revisa si
  la app pasa a usarse en computadoras compartidas.
- **Dependen.** SRV-04, SRV-05, SRV-07 y SRV-11.
