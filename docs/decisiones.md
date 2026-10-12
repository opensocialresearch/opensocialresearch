# Decisiones

Registro de las decisiones del proyecto, de la más reciente a la más antigua.
Cada cambio de alcance, de umbral, de herramienta o de especificación aprobada
se anota aquí con fecha y razón (ver [DESARROLLO.md](../DESARROLLO.md)).

## 2026-10-12 · Primer requisito implementado (PRY-01)

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
