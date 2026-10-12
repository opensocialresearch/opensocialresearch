# Decisiones

Registro de las decisiones del proyecto, de la más reciente a la más antigua.
Cada cambio de alcance, de umbral, de herramienta o de especificación aprobada
se anota aquí con fecha y razón (ver [DESARROLLO.md](../DESARROLLO.md)).

## 2026-10-12 · Primer requisito implementado (PRY-01)

- **D-33.** Encabezados de todas las respuestas (SRV-09). Los siete
  encabezados se ponen en una sola capa que envuelve a todas las demás,
  también a la que valida el pedido y a la ruta de reserva, así llegan a los
  200, 401, 403, 404 y 405, y llegarán a los 206, 410 y 416 de SRV-11 y
  TRA-23 sin tocar nada. La capa reemplaza un encabezado con el mismo nombre
  que ya traiga la respuesta, para que ninguna ruta pueda dejar otro valor.
  El esquema de SRV-09.1 tiene como segundo paso «el audio de E02 está
  eliminado según TRA-23», que no existe hasta TRA-23 y PRY-17, así que sus
  ocho ejemplos quedan omitidos, y como comparten la etiqueta `@SRV-09.1` con
  el escenario del `OPTIONS`, la corrida estricta de `@SRV-09` no puede
  pasar. Siguiendo D-24, las barandas de SRV-09 se corrieron con `@SRV-09 and
  not @SRV-09.1`, y el escenario del `OPTIONS` se corrió aparte en modo
  estricto por su nombre, porque cucumber-rs no deja combinar `--tags` con
  `--name`. Las pruebas unitarias comprueban los siete valores exactos en el
  200, el 401, los dos 403, el 404 y el 405, en lugar de los ejemplos
  omitidos. En los pasos, «la respuesta tiene estado» captura el 405 y se
  definen «la respuesta tiene estos encabezados» y «la respuesta tiene el
  encabezado». La marca [verificar] sobre los estilos en línea de la
  interfaz compilada y SRV-09.2 quedan para la interfaz en Svelte.

- **D-32.** Sin CORS (SRV-08). No hace falta código, porque el servidor no
  tiene ninguna capa de CORS, una verificación previa desde otro origen ya
  recibe el 403 de SRV-07 y un `OPTIONS` autenticado desde el origen propio
  recibe de axum el 405 con `Allow`. El commit suma solo los pasos de
  SRV-08.1. La verificación previa se manda una vez a cada camino de la
  lista, aunque el camino acepte varios métodos, porque `OPTIONS` no depende
  del método de la ruta y repetirla daría la misma respuesta. «Ninguna
  respuesta tiene un encabezado que empiece con» compara el nombre sin
  distinguir mayúsculas y falla si no hay respuestas que revisar. SRV-08.2 es
  del recorrido completo (D-21).

- **D-31.** Validación de Origin y de Sec-Fetch-Site (SRV-07). Las tres
  reglas corren en la capa de SRV-06, después del Host y antes del token, y
  el 403 lleva el cuerpo JSON en todas las familias. El `Origin` se compara
  exacto con los tres orígenes propios, sin ignorar mayúsculas, porque la
  especificación solo dice eso del Host y el navegador siempre serializa el
  origen en minúsculas, así que `null`, otro puerto, `https` o un origen sin
  puerto fallan. Un `OPTIONS` también tiene que traer `Origin`, porque no es
  GET ni HEAD. Un valor que no es ASCII visible en `Origin` o en
  `Sec-Fetch-Site` cuenta como ajeno. La excepción de la entrada vale para
  `GET /` con el parámetro `token` en la consulta y `Sec-Fetch-Mode:
  navigate`, sin mirar si el token es válido, porque si no lo es el pedido
  recibe igual el 401 de SRV-05 un paso después, y no exceptúa la regla del
  `Origin`, solo la de `Sec-Fetch-Site`. En los pasos, «el pedido lleva el
  encabezado» captura ahora también `Origin` con cualquier valor y
  `Sec-Fetch-Site`, en lugar del paso aparte del Origin propio, y con eso
  SRV-07.1 corre y la corrida sin etiquetas no tiene fallidos. SRV-07.4 y
  SRV-07.5 son `@interfaz` y quedan omitidos, y las dos marcas [verificar] de
  la especificación sobre lo que mandan Firefox y Chromium siguen abiertas
  hasta las pruebas de Playwright.

- **D-30.** Validación de Host (SRV-06). El servidor lee el encabezado
  `Host` tal cual, sin mirar `X-Forwarded-Host` ni la autoridad del camino,
  porque esos son justo los caminos del reenlace de DNS. Es propio si es
  `127.0.0.1:P`, `[::1]:P` o `localhost:P` sin distinguir mayúsculas, y un
  Host sin puerto, como `localhost`, es ajeno, porque los navegadores siempre
  mandan el puerto cuando no es el 80. El 403 lleva el cuerpo JSON en todas
  las familias, sin `WWW-Authenticate`, porque la especificación no da otro
  cuerpo para las páginas. La validación corre en la misma capa que el token,
  antes que él, y por eso el servidor recibe el puerto además del token. En
  los pasos, «el pedido lleva el encabezado» captura también `Host`, «la
  respuesta tiene estado» captura el 403 y se definen «el cuerpo de la
  respuesta es el JSON» y «el pedido no lleva el encabezado "Host"», que
  escribe el pedido a mano por TCP porque reqwest y curl siempre agregan
  `Host`. Al capturar el 403 empezó a correr SRV-07.2, que tenía definidos
  todos sus demás pasos, compartidos con SRV-05.3 y SRV-06, así que según
  D-20 no había ninguna captura que acotar sin dejar fuera a SRV-06. El
  commit de SRV-06 queda con ese único escenario en rojo en la corrida sin
  etiquetas, la cobertura y la mutación no pueden correr hasta que pase, y
  el commit de SRV-07, que lo pone en verde, es donde se corrieron las
  barandas completas de los dos. Se descartó juntar los dos requisitos en un
  commit y se descartó saltar el paso en ejecución, que sería debilitar el
  escenario.

- **D-29.** Archivo `proyecto.toml` (PRY-03). Al abrir, `osr` valida
  `proyecto.toml` antes de tomar el bloqueo, así un error no escribe nada en
  la carpeta. Las claves obligatorias se revisan en el orden de la tabla,
  `formato`, `version_formato`, `nombre` y `creado`, y la versión se mira
  antes que `nombre` y `creado`, porque otra versión del formato puede tener
  otras claves. Una clave obligatoria de otro tipo cuenta como faltante, y
  `creado` tiene que ser una marca de tiempo con fecha, hora y
  desplazamiento. La línea del error de TOML es la del primer byte del
  tramo que señala el analizador, y el motivo es en español y fijo, `no es
  TOML válido`, o `no está escrito en UTF-8`, porque el analizador da su
  motivo en inglés y PRY-18 pide mensajes en español, así que el detalle no
  llega a la persona. Si `proyecto.toml` existe pero no se puede leer, por
  permisos o porque es una carpeta, `osr` termina con código 6 y `osr: no se
  pudo leer proyecto.toml (<motivo>).`, un caso que la especificación no
  fija y que queda como pregunta para el autor. Quedan fuera de este commit,
  con sus requisitos, las claves opcionales con sus valores por defecto y el
  aviso de valor no válido, que solo se ven en la interfaz y las usan PAR-01,
  TRA-14, TRA-15 y TRA-17 (PRY-03.3 es `@interfaz`), la reescritura que
  conserva comentarios y claves desconocidas, que es de PAR-06 (PRY-03.4 es
  `@interfaz`), y el límite de 64 KiB, cuyo error la especificación no
  define, pregunta para el autor. En los pasos, las alteraciones «con la app
  cerrada» capturan solo `proyecto.toml` y sus claves de primer nivel, y se
  define «un proyecto nuevo creado con `osr nuevo`», que deja el proyecto
  abierto, con lo que PRY-02.1 corre y pasa (D-20).

- **D-28.** Un solo proceso por proyecto (PRY-05). El bloqueo se escribe
  completo en un temporal de `.osr/` con el nombre de PRY-12 y se pone en su
  lugar con un enlace duro, que falla si ya existe, así dos procesos no lo
  toman a la vez y nadie lee un bloqueo a medio escribir. Si ya existe, `osr`
  lee su PID y mira `/proc/<pid>`, y lo da por vivo y de un `osr` si el
  proceso existe, no es un zombi y su nombre de programa en
  `/proc/<pid>/comm` es el mismo que el propio, sin fijar el texto `osr`. Con
  un `osr` vivo termina con código 4 y el mensaje de la tabla, sin tocar el
  bloqueo. Si no, lo reemplaza con un `rename` y anota `bloqueo_abandonado`
  antes de `verificacion`, con el detalle `{"pid": <n>, "inicio": <marca>}`
  del bloqueo reemplazado, o `null` en lo que no se pudo leer, porque la
  especificación no fija el detalle. Un bloqueo con el PID de este mismo
  proceso, o que no se puede leer, cuenta como abandonado. Si después de tomar
  el bloqueo falla la escritura del registro de eventos, `osr` lo suelta antes
  de terminar con código 6. Dos procesos que encuentran a la vez el mismo
  bloqueo abandonado pueden reemplazarlo los dos, una carrera que el tramo 1
  no cubre y que la instancia única de la v1.0 resuelve. En los pasos,
  «el registro de eventos tiene exactamente una línea nueva de tipo» cuenta
  solo las líneas nuevas de ese tipo, porque cada apertura anota además su
  `verificacion` (D-17) y PRY-05.2 tendría dos líneas nuevas. Un `osr` que
  queda corriendo cuando el escenario lanza otro se guarda aparte, para
  resolver `{PID del primer proceso}`, preguntarle si sigue atendiendo y
  cerrarlo al final, y el paso de SIGKILL espera al proceso para que no quede
  como zombi.

- **D-27.** Abrir un proyecto (PRY-04). `osr abrir <ruta>` y `osr <ruta>` son
  la misma acción, así que una palabra suelta después de `osr` es la ruta de
  un proyecto, `osr borrar` intenta abrir la carpeta `borrar` y el mensaje de
  orden desconocida de SRV-01 queda para las opciones, como `--opcion-falsa`.
  La prueba unitaria que pedía ese mensaje para `osr borrar` pasa a pedirlo
  para una opción. Abrir comprueba que la ruta sea una carpeta, toma el
  bloqueo, anota `verificacion` y levanta el servidor, y la validación de
  `proyecto.toml` llega con PRY-03 y la revisión del bloqueo existente con
  PRY-05. La verificación al abrir es PRY-11, así que hasta entonces el evento
  `verificacion` sale con el detalle de PRY-11 en su orden, con las listas
  vacías y los contadores en cero, sin haber revisado nada, y PRY-11 lo llena.
  El README no se regenera al abrir todavía, porque regenerarlo sin la regla
  de edición a mano de PRY-16 pisaría los cambios de la persona sin guardarlos
  aparte, y ningún escenario de PRY-04 lo verifica, llega con PRY-16. Si al
  abrir no se puede escribir el bloqueo o el registro de eventos, `osr`
  termina con código 6 y `osr: no se pudo abrir el proyecto en <ruta>
  (<motivo>).`, por analogía con la tabla de PRY-01, porque la especificación
  no fija ese caso, y queda como pregunta para el autor. Las líneas del
  registro de eventos se agregan al final y se llevan a disco una por una,
  como pide PRY-12. En PRY-04.1 el estado que muestra la app es, hasta la
  vista «Resumen del proyecto», el cuerpo de `GET /` con el token de la
  sesión. En los pasos, «una copia del proyecto de prueba» ahora crea un
  proyecto vacío con `osr nuevo` y lo cierra en orden, y «osr está corriendo
  con el proyecto abierto» lo abre con `osr abrir`, en lugar de lo que fijó
  D-25, hasta que PRY-17 traiga los datos de prueba. «Se ejecuta» captura
  también `osr abrir` y `osr <ruta>`, con lo que SRV-03.2 corre y pasa, y «el
  código de salida es» captura solo los códigos que `osr` ya produce, para que
  el 4 de PRY-05.1 quede omitido hasta PRY-05 (D-20). PRY-04.2 es `@interfaz`
  y necesita las entrevistas de PRY-17, así que queda omitido.

- **D-26.** Cierre ordenado (SRV-15). Con la primera SIGINT o SIGTERM los
  servidores dejan de aceptar conexiones y terminan los pedidos en curso, con
  una espera de 2 s como máximo para que todo el cierre quepa en los 3 s. Después
  `osr` borra el bloqueo, anota `cierre ordenado` en el registro técnico y
  termina con 0. Si llega una segunda señal durante esa espera, la corta en el
  acto, borra igual el bloqueo, anota `cierre forzado por una segunda señal` y
  termina con 1, porque la especificación no fija el código y un 0 diría que
  el cierre fue completo. Todavía no hay escrituras pendientes que terminar,
  llegan con PRY-12 y PRY-13. El aviso de la pestaña es de la interfaz
  (SRV-15.3) y llega con ella.

- **D-25.** Token en todas las rutas (SRV-05). El servidor arma sus rutas solo
  desde la lista de `src/rutas.rs`, que no depende de nada del programa, y la
  prueba incluye ese mismo archivo con `#[path]`, así la lista sale de la
  propia definición del servidor sin ser una biblioteca aparte y sin cambiar
  las barandas. Hoy la lista tiene solo `GET /`. La capa del token envuelve
  también la ruta de reserva, así un camino inexistente da 401 sin token y 404
  con token. El token vale por `Authorization: Bearer` y por la entrada
  `GET /?token=`, y la cookie de sesión llega con SRV-04. La familia de un
  camino se decide por el prefijo `/api/`, con barra final, y `/api` sin barra
  cae en las demás. Como no existen los datos de PRY-17 ni `osr abrir`, «una
  copia del proyecto de prueba» y «osr está corriendo con el proyecto abierto»
  crean por ahora un proyecto vacío con `osr nuevo`, que queda abierto igual.
  PRY-04 y PRY-17 los reemplazan por la copia de los datos de prueba y su
  apertura. La ruta que crea un participante no existe hasta PAR, el paso usa
  el camino `/api/participantes`, y sin token da 401 exista o no. Al definir
  esos dos pasos compartidos, muchos escenarios de requisitos pendientes
  empezaron a correr y a fallar, así que, siguiendo D-20, los pasos de pedidos
  capturan solo lo que ya existe, los estados 200, 401 y 404, los caminos `/`,
  `/?token=` y `/no-existe`, los encabezados Authorization y Cookie y el Origin
  propio, y las huellas de carpetas bajo `{tmp}`. Quien implemente SRV-06,
  SRV-07, SRV-11 o PRY-17 amplía esas capturas.

- **D-24.** Token por arranque (SRV-03). El token llegó con SRV-01, porque la
  línea de arranque ya lo lleva, y se genera con 256 bits de `/dev/urandom` en
  base64url hecho a mano y probado con los vectores del RFC 4648, sin crates
  nuevos. Este requisito suma los pasos de sus escenarios. SRV-03.2 abre el
  proyecto con `osr abrir`, que es PRY-04, así que queda omitido hasta que
  PRY-04 amplíe la captura de «se ejecuta», y por eso las barandas de SRV-03 se
  corrieron sin ese escenario. Sus demás pasos ya existen. Como el `xdg-open`
  de prueba dura muy poco para verlo leyendo los procesos cada 10 ms, el paso
  final suma su propia anotación de cómo lo llamaron, que además prueba que el
  navegador sí se llamó. SRV-03.3 es del recorrido completo (D-21).

- **D-23.** Escucha en loopback (SRV-02). `osr` pide al sistema un puerto
  efímero en `127.0.0.1` y toma el mismo en `[::1]`, y si ese está ocupado en
  IPv6 suelta los dos y prueba otro, hasta 10 veces. Se toma como falta de IPv6
  de loopback que `[::1]` no exista o que el sistema no tenga IPv6. El puerto
  se toma antes de crear el proyecto, así un arranque sin puerto libre no deja
  nada creado ni un bloqueo huérfano. Los escenarios de red corren `osr` en un
  espacio de nombres de usuario y de red propio, sin privilegios, con
  `unshare` y `nsenter`, y desde ahí también la conexión de la prueba (con
  python3) y los pedidos HTTP (con curl), porque el loopback de ese espacio no
  se ve desde fuera y la dirección local de SRV-02.2 iría a la red real. La
  interfaz local es una `dummy` con una dirección /32, sin ninguna ruta de
  salida. El rango de un solo puerto es el 40000, que en un espacio propio no
  choca con nada, y lo ocupa en `[::1]` el mismo proceso que sostiene la red.
  El registro técnico existe desde aquí con lo mínimo que pide SRV-02.4, en
  `<carpeta de estado>/registro.log` con permisos 0600 y líneas con marca,
  nivel y mensaje, y anota el arranque, el puerto y si hay IPv6 de loopback. La
  rotación, los pedidos rechazados y lo que el registro nunca contiene quedan
  para SRV-12.

- **D-22.** Arranque del servidor (SRV-01). El navegador se abre con
  `xdg-open`, sin crates nuevos, porque la v0.1 es solo para Linux. Para que
  el token no quede en los argumentos de ningún proceso (SRV-03), `xdg-open`
  recibe la ruta de una página de un solo uso que redirige a la dirección con
  una etiqueta `meta refresh`, en la carpeta de estado (0700), con permisos
  0600, y que se borra a los 30 s o al cerrar `osr`, lo que pase antes. Si
  `xdg-open` no existe o termina con error, `osr` avisa con el mensaje de la
  tabla y sigue corriendo. Las pruebas simulan el sistema sin navegador
  (SRV-01.3) corriendo `osr` con un `PATH` que solo tiene una carpeta vacía, y
  en los demás escenarios el `PATH` tiene un `xdg-open` de prueba que anota cómo
  lo llamaron, así ninguna prueba abre el navegador del equipo. La carpeta de
  estado de las pruebas también va aparte, fuera de `{tmp}` y del equipo.
  `--sin-navegador` es una opción global, así vale antes o después de la orden.
  Hasta que existan PRY-04 y la interfaz en Svelte, la entrada con token
  sirve la pantalla sin proyecto, con o sin proyecto abierto, y sus
  formularios todavía no hacen nada. `osr nuevo` ahora crea, levanta el
  servidor, escribe la línea de SRV-01 y espera la señal, y los pasos esperan
  esa línea en lugar del bloqueo para saber que `osr` quedó listo. Se suma
  tower como dependencia de desarrollo, ya presente por axum, para probar las
  rutas sin abrir un puerto.

- **D-19, agregado.** Se suma tower 0.5 como dependencia de desarrollo, con
  su función `util`, para mandar pedidos a las rutas del servidor en las
  pruebas unitarias sin abrir un puerto. Ya venía en el árbol por axum, así que
  no agrega código nuevo al programa (D-22).
- **D-21.** Los escenarios `@recorrido-completo` no entran en la corrida
  estricta de cada requisito, porque se verifican sobre el proyecto de prueba
  después de ejecutar todas las acciones del tramo, y ese recorrido no existe
  hasta que el tramo esté implementado. Corren todos juntos al cerrar el tramo,
  con su propio paso de recorrido. Hasta entonces cuentan como omitidos en el
  avance. No se debilita ningún escenario, solo se fija cuándo se ejecuta.
- **Diseño de la interfaz.** El diseño hecho en Claude Design queda como
  referencia en [diseno/README.md](diseno/README.md), con los tokens y las
  notas para implementar. Si difiere de un escenario aprobado, manda el
  escenario.
- **D-20.** Las definiciones de pasos se acotan a lo que ya está implementado.
  Un paso compartido entre requisitos, como «se ejecuta `...`», captura solo
  las órdenes que existen (hoy `osr nuevo`), para que los escenarios de
  requisitos pendientes queden omitidos y no fallidos. Quien implemente una
  orden nueva amplía la captura. Se descartó que el modo no estricto tolere
  fallas fuera de la etiqueta en curso, porque escondería regresiones.
- **D-17, precisión.** `osr nuevo` no anota `verificacion`, porque un proyecto
  recién creado no tiene nada que verificar. El evento lo anota cada apertura
  de un proyecto existente con `osr abrir`, y así PRY-01.6 sigue pidiendo una
  sola línea.
- **D-19, agregado.** Se suma jiff para la fecha y hora con desplazamiento
  local, que la biblioteca estándar no da y que piden `creado` en PRY-03, la
  fecha del registro en PRY-08 y las marcas de PRY-02.
- **Barandas, arreglo.** El paso de mutación no probaba nada, porque la
  configuración de git del equipo usa prefijos mnemónicos en los diffs y
  cargo-mutants no reconocía los archivos, y en la integración continua
  fallaba por pasar `-q` junto con el `--verbose` de cargo-mutants. Se fijaron
  los prefijos y se quitó `-q`. Los umbrales no cambiaron.
- La integración continua habilita los espacios de nombres sin privilegios que
  usa el escenario de disco lleno (PRY-01.5).

## 2026-10-11 · Arranque del código

- **D-18.** Los escenarios con archivos de varios gigabytes o audios largos se
  etiquetan `@lento` y corren en la integración continua. Los archivos grandes
  se crean dispersos, con su tamaño declarado sin ocupar el disco.
- **D-19.** Dependencias base del orquestador, fijadas para que cada tramo no
  las vuelva a elegir. clap para la línea de órdenes, tokio y axum para el
  servidor, rusqlite con SQLite incluido para el índice, serde con toml y
  serde_json para los archivos, y para las pruebas cucumber, reqwest y
  tempfile. Cualquier otra se agrega cuando un requisito la necesite, con su
  razón en este registro.
- **Prueba del primer día.** El R del autor (4.6.1) carga tidyverse 2.0.0 y
  sf 1.1.2 sin compilar nada. El plan B del tramo 3 no se activa por esto.

## 2026-10-11 · Lecturas adoptadas al escribir los escenarios

Al pasar las especificaciones a escenarios aparecieron ambigüedades menores.
En todas se adoptó la lectura más conservadora, que queda comentada en el
escenario correspondiente de `escenarios/`. La única que afecta a varios
requisitos se fija aquí.

- **D-17.** Cada apertura de un proyecto anota un evento `verificacion` en el
  registro, después de reparar una línea incompleta si la hubiera. PRY-01.6
  cuenta solo lo que escribe `osr nuevo`, sin abrir el proyecto, y en PRY-08.3
  el evento `linea_incompleta_descartada` es la primera línea nueva, no la
  última.

Las demás lecturas, por ejemplo el plural literal de SRV-13.2 o cómo se
escriben los números dentro de los mensajes, se corrigen en la especificación
cuando la implementación las toque.

## 2026-10-11 · Decisiones del tramo 1 de la v0.1

El autor delegó las dieciséis decisiones abiertas de la especificación del
tramo 1 y pidió corregirlas cuando el software esté en uso real. Se adoptó en
todas la propuesta razonada que trae cada especificación, donde están las
opciones descartadas y la razón.

| ID | Decisión | Detalle |
|---|---|---|
| D-01 | Formato canónico de transcripción en JSON documentado, WebVTT solo para importar | [transcripciones.md](../especificaciones/transcripciones.md) |
| D-02 | El consentimiento registrado bloquea la importación de un audio, con excepción registrada, salvo si la persona negó la grabación | [participantes.md](../especificaciones/participantes.md) |
| D-03 | La app registra en un archivo de solo agregar y la bitácora del autor es un Markdown sin interfaz en el tramo 1 | [proyecto.md](../especificaciones/proyecto.md) |
| D-04 | Codificación por selección libre, anclada al segmento | [transcripciones.md](../especificaciones/transcripciones.md) |
| D-05 | Una transcripción revisada se puede reabrir para editar y la app no impone convención de transcripción | [transcripciones.md](../especificaciones/transcripciones.md) |
| D-06 | Escala de diseño de 50 entrevistas de 90 minutos, recorridos con ubicación para después | [transcripciones.md](../especificaciones/transcripciones.md) |
| D-07 | Alcances del consentimiento por defecto, los cinco de PRODUCTO.md, ampliables por proyecto | [participantes.md](../especificaciones/participantes.md) |
| D-08 | Acción «Eliminar audio» para cumplir la promesa de borrar el audio tras la revisión | [transcripciones.md](../especificaciones/transcripciones.md) |
| D-09 | Una persona, un audio y una entrevista por participante | [transcripciones.md](../especificaciones/transcripciones.md) |
| D-10 | El Markdown de la cadena actual se usa solo si se alinea exactamente con la transcripción | [transcripciones.md](../especificaciones/transcripciones.md) |
| D-11 | Atajos de teclado de la tabla de TRA-18, a validar en uso | [transcripciones.md](../especificaciones/transcripciones.md) |
| D-12 | El token viaja en una cookie HttpOnly y Strict más el encabezado Authorization | [servidor.md](../especificaciones/servidor.md) |
| D-13 | Sin exportar transcripciones en el tramo 1 | [transcripciones.md](../especificaciones/transcripciones.md) |
| D-14 | Sin fotos ni notas en el tramo 1 | [transcripciones.md](../especificaciones/transcripciones.md) |
| D-15 | Cuatro niveles de sensibilidad, identificable, sensible, no sensible y sin marca | [participantes.md](../especificaciones/participantes.md) |
| D-16 | La fecha y el lugar de la entrevista viven en la zona identificable, fuera de la tabla de correspondencia | [participantes.md](../especificaciones/participantes.md) |

## 2026-10-11 · Bases de datos públicas en un proyecto aparte

Reunir y curar bases de datos públicas de Costa Rica será un repositorio
aparte, donde la gente puede pedir bases y colaborar. La herramienta se
conectará solo con ese repositorio, en lugar de mantener conectores con cada
fuente. Se descartó un conector por protocolo (NADA, CKAN, Dataverse) dentro de
la herramienta.

## 2026-10-11 · Método de desarrollo

Especificación, escenarios en Gherkin, código guiado por pruebas y barandas
automáticas, según [DESARROLLO.md](../DESARROLLO.md).

## 2026-10-11 · Documentos con Quarto y colaboración con Automerge

Quarto para PDF, sitio web, citas y resultados incrustados, en lugar de
construir un motor de documentos propio. Automerge e iroh para colaborar sin
nube.

## 2026-10-11 · R y Python como motores de análisis

El análisis lo corren R y Python, y la herramienta los orquesta. Se descartó
implementar estadística propia en Rust.

## 2026-10-11 · Aplicación web local en Rust

Un solo programa que levanta un servidor local y abre el navegador, con el
orquestador en Rust y la interfaz en Svelte, para que corra en computadoras
modestas y los datos no salgan de la máquina.

## 2026-10-11 · Licencia MIT

Se eligió MIT frente a AGPL-3.0, para que cualquiera pueda usar el software
como quiera.

## 2026-10-11 · Nombre

Open Social Research. Se descartó OpenASR porque ASR es la sigla habitual del
reconocimiento automático de voz.
