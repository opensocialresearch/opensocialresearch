# Decisiones

Registro de las decisiones del proyecto, de la más reciente a la más antigua.
Cada cambio de alcance, de umbral, de herramienta o de especificación aprobada
se anota aquí con fecha y razón (ver [DESARROLLO.md](../DESARROLLO.md)).

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
