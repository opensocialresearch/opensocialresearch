# Especificación del módulo diseño de la investigación (DIS)

Tramo 1 de la v0.1. El autor pidió el 2026-10-13 llevarlo hasta una versión
que pueda probar (D-54). Lo revisó un panel de tres perfiles, metodología
cualitativa y mixta, investigación participativa y feminista, y estándares
internacionales de publicación, que lo aprobó con cambios ya incorporados; la
revisión de los textos del autor llega al probarlo.

## 1. Propósito

Un proyecto empieza por el diseño de la investigación; un acercamiento breve
al campo puede acompañarlo (Villalobos Zamora, 2017, p. 261). Este módulo da
cinco pasos con orden sugerido y sin bloqueos, cada uno con preguntas guía
apoyadas en la bibliografía de los cursos del programa ISA de la UNED (D-47),
un espacio para escribir la respuesta propia y un panel con lo que dice la
bibliografía. El proceso es en espiral: se puede volver a cualquier paso, y
cuando un paso cambia, los demás que ya tenían texto quedan marcados para
revisar su coherencia, hacia adelante y hacia atrás (D-49). Las preguntas sirven
igual para una investigación que diseña una sola persona y para una que se
construye con las personas o comunidades involucradas.

### Requisitos de PRODUCTO.md que cubre

| Sección | Requisito (texto corto) | IDs |
|---|---|---|
| 6.1 | «Plantilla `.qmd` para redactar el proyecto», v1.0, adelantada al tramo 1 de la v0.1 como diseño guiado (D-54) | DIS-01 a DIS-06 |
| 6.7.7 | «Plantillas comentadas en español con su fuente citada», v1.0, adelantada (D-54) | DIS-01, DIS-02 |

## 2. Definiciones

| Término | Definición |
|---|---|
| Paso | Uno de los cinco pasos del diseño, numerado de `01` a `05`. |
| Pregunta guía | Una pregunta de un paso, con la obra del curso que la sostiene. |
| Respuesta | El texto que escribe la persona para una pregunta guía. |
| Versión de un paso | La huella SHA-256 de su archivo en disco, o la cadena vacía si todavía no existe. |

## 3. Requisitos

### DIS-01 Los cinco pasos y sus preguntas guía

**Comportamiento.** El diseño tiene estos cinco pasos, con estas preguntas
guía en este orden. Cada pregunta lleva su fuente, que la interfaz no muestra
junto a la pregunta sino en el panel de bibliografía (DIS-02). Ninguna
pregunta es obligatoria; la de hipótesis es para los diseños que las usan.

| Paso | Título | Preguntas guía | Fuente |
|---|---|---|---|
| 01 | `Tema e interés` | `¿Qué parte del mundo social interesa estudiar?` | Sautu et al. (2005, p. 30) |
| | | `¿De dónde surge el interés: experiencia propia, lecturas, un encargo, una comunidad?` | Flick (2015, p. 34); Abarca et al. (2012, p. 32) |
| | | `¿Para qué y para quién se investiga?` | Tutoría 1 de 05646, diap. 2; Fals Borda (2015, p. 263) |
| | | `¿Con quiénes se define el tema y el problema, y quién decide sobre ellos: quien investiga, un equipo, las personas o comunidades involucradas?` | Corona Berkin (2019, p. 58); Kaltmeier (2012, pp. 36 y 53); Fals Borda (2015, p. 285) |
| 02 | `Posicionamiento y supuestos` | `¿Desde dónde se investiga: trayectoria, experiencias, creencias, formación?` | Jiménez Cortés (2021, p. 183) |
| | | `¿Qué se supone que existe cuando se estudia este tema?` | Tutoría 1 de 05646, diap. 16; Sautu et al. (2005, p. 40) |
| | | `¿Cómo se puede conocer esa realidad?` | Tutoría 1 de 05646, diap. 16 |
| | | `¿Qué relación se establece entre quien investiga y las personas o los fenómenos que investiga?` | Sautu et al. (2005, p. 40); tutoría 1 de 05646, diap. 2; Pajares Sánchez (2020, p. 304) |
| | | `¿Qué papel tienen los valores en esta investigación?` | Villalobos Zamora (2017, pp. 7-8); Sautu et al. (2005, p. 40) |
| | | `¿Con quién se comparte este posicionamiento y cómo se va a revisar a lo largo de la investigación?` | Jiménez Cortés (2021, p. 183); Kaltmeier (2012, p. 52) |
| 03 | `Problema, estado de la cuestión y marco teórico` | `¿Cuál es el problema, todavía en forma preliminar?` | Villalobos Zamora (2017, p. 261); Pajares Sánchez (2020, p. 302) |
| | | `¿Por qué es importante estudiarlo y qué aporta, en la práctica y al conocimiento?` | Abarca et al. (2012, pp. 33 y 41); Villalobos Zamora (2017, p. 93) |
| | | `¿Qué se sabe ya, en la bibliografía y entre quienes viven el problema, y qué falta saber?` | Villalobos Zamora (2017, p. 266); Sautu et al. (2005, p. 30); Fals Borda (2015, p. 282); Kaltmeier (2012, p. 39) |
| | | `¿Con qué conceptos y teorías se va a mirar el problema, y cómo se relacionan entre sí?` | Sautu et al. (2005, pp. 161-162) |
| 04 | `Pregunta y objetivos` | `¿Qué pregunta concreta va a responder la investigación, con su lugar, su período y las personas o casos que abarca?` | Flick (2015, p. 39); Abarca et al. (2012, pp. 39-40) |
| | | `¿Qué objetivo general y qué objetivos específicos se desprenden de la pregunta?` | Sautu et al. (2005, pp. 36-37) |
| | | `Si la investigación pone a prueba hipótesis, ¿cuáles son y cómo se relacionan con el diseño?` | Abarca et al. (2012, p. 52); Villalobos Zamora (2017, pp. 106 y 273) |
| 05 | `Enfoque, metodología y métodos` | `¿Qué enfoque permite responder mejor la pregunta y por qué?` | Della Porta y Keating (2013, p. 339); tutoría 1 de 05646, diap. 12 |
| | | `¿Con quiénes, con qué casos o con qué fuentes se va a trabajar, en qué lugar, y con qué criterios se eligen?` | Sautu et al. (2005, p. 162); Flick (2015, p. 58); Villalobos Zamora (2017, p. 285) |
| | | `¿Con qué técnicas se producirán los datos, quiénes participarán en producirlos y analizarlos, y cómo se analizarán?` | Sautu et al. (2005, pp. 38 y 157); Corona Berkin (2019, p. 56) |
| | | `¿Qué acuerdos éticos pide este diseño: riesgos para las personas, consentimiento, anonimato o uso de sus nombres, devolución de resultados y autoría?` | Tutoría 1 de 05646, diap. 18-20; Kvale (2011, p. 77); Flick (2015, p. 91); Kaltmeier (2012, p. 54) |

Tres preguntas llevan debajo una ayuda en gris, porque la prueba con una
estudiante sin experiencia mostró que sin ejemplo quedan en blanco (D-55). Las
ayudas dan opciones de las distintas tradiciones, sin elegir por la persona.

| Pregunta | Ayuda | Fuente |
|---|---|---|
| `¿Qué se supone que existe cuando se estudia este tema?` | `Por ejemplo: si lo que se estudia es un hecho que se puede medir, algo que las personas viven y cuentan, algo que se construye entre personas, o varias de esas cosas.` | Tutoría 1 de 05646, diap. 16; Della Porta y Keating (2013, p. 36, tabla 2.1) |
| `¿Cómo se puede conocer esa realidad?` | `Por ejemplo: contando y midiendo, escuchando y observando, o construyendo el conocimiento con las personas.` | Tutoría 1 de 05646, diap. 16; Corona Berkin (2019, p. 58) |
| `¿Qué enfoque permite responder mejor la pregunta y por qué?` | `Cualitativo, cuantitativo, mixto o participativo, y por qué ese le sirve a la pregunta del paso 04.` | Della Porta y Keating (2013, p. 339) |

El paso 02 no es un menú para elegir un paradigma: los supuestos se escriben
y se revisan (D-49). La pregunta ética del paso 05 anticipa lo que el registro
de consentimiento formaliza después (PAR-06, PAR-07).

### DIS-02 Panel de bibliografía

**Comportamiento.** Cada paso muestra el panel `Qué dice la bibliografía` con
estas citas textuales, cada una con su referencia debajo.

| Paso | Cita | Referencia |
|---|---|---|
| 01 | `Toda investigación o proyecto se inicia con un conjunto de consideraciones acerca de aquella parte del mundo social que se desea estudiar.` | `Sautu et al. (2005, p. 30)` |
| 01 | `Se debe aclarar el por qué y el qué de la investigación antes de que se plantee la pregunta del cómo: el método.` | `Kvale (2011, p. 77)` |
| 01 | `Éste no surge de la inspiración introspectiva de un sujeto–investigador sino que es producto de interacciones sociales.` | `Kaltmeier (2012, p. 36), sobre el tema de investigación` |
| 01 | `resultó importante tener conciencia de “para quién” se trabajaba` | `Fals Borda (2015, p. 263)` |
| 02 | `la persona que investiga no lo hace al margen de sus vivencias, su formación, sus creencias y sus circunstancias vitales` | `Jiménez Cortés (2021, p. 183)` |
| 02 | `toda investigación participativa comienza con una reflexión personal orientada a que emerjan prejuicios, presunciones, dudas o posturas subconscientes o invisibles` | `Pajares Sánchez (2020, p. 304)` |
| 02 | `Coincidimos en que no todos los sociólogos tienen que ser filósofos y, ciertamente, gran parte de la investigación en ciencias sociales jamás se habría efectuado si tuviésemos que comenzar resolviendo las cuestiones fundamentales sobre el ser y el conocimiento. No obstante, es necesaria cierta reflexión sobre la esencia del conocimiento como premisa preliminar antes de abordar cualquier investigación.` | `Della Porta y Keating (2013, pp. 32-33)` |
| 03 | `no existe problema alguno si no hay una persona –o grupo de personas– que lo defina como tal y lo padezca. Un problema es siempre problema para alguien` | `Pajares Sánchez (2020, p. 302)` |
| 03 | `se plantea en forma preliminar un problema de investigación muy general` | `Villalobos Zamora (2017, p. 261), sobre el diseño cualitativo` |
| 03 | `el marco teórico no es un estado del arte` | `Sautu et al. (2005, p. 161)` |
| 04 | `Esta pregunta decide qué es importante [...] y, todavía en mayor medida, qué es menos importante y debe quedar fuera.` | `Flick (2015, p. 39)` |
| 04 | `Los objetivos constituyen el pilar de una investigación y sirven de nexo entre la teoría y la metodología.` | `Sautu et al. (2005, p. 37)` |
| 04 | `Las preguntas en la perspectiva horizontal no responden a las necesidades previamente construidas por un investigador.` | `Corona Berkin (2019, p. 58)` |
| 05 | `lo que deben hacer los especialistas es escoger un enfoque, una metodología y los métodos específicos que mejor se ajusten a las preguntas que plantean.` | `Della Porta y Keating (2013, p. 339)` |
| 05 | `La selección de la metodología se define de acuerdo con las características del objeto de estudio y con la información que se desea obtener.` | `Villalobos Zamora (2017, p. 57)` |

Debajo de las citas, el panel lleva `Fuentes de las preguntas`, con cada
pregunta guía del paso y su fuente de DIS-01. Las citas se copiaron del texto
de cada obra y se verificaron contra él (D-49, D-54). En la cita de Kvale las cursivas del original no se marcan. La
cita de Villalobos Zamora sobre el problema como «primer componente» del
diseño se retiró porque habla solo del diseño cuantitativo.

### DIS-03 Vista general del diseño

**Comportamiento.** Con un proyecto abierto, la página `/diseno` muestra el
rótulo `Diseño de la investigación`, el nombre del proyecto como título, el
texto `El diseño se escribe en espiral: los pasos tienen un orden sugerido,
pero se puede volver a cualquiera cuando haga falta.`, el texto `En un diseño
participativo, estos pasos se pueden escribir con las personas
participantes.` (Kaltmeier, 2012, p. 53; Corona Berkin, 2019, p. 58) y los cinco pasos en
orden, cada uno con su número, su título, la primera respuesta no vacía
recortada a 120 caracteres o `Todavía sin texto`, y su estado (DIS-05). Cada paso
es un enlace a su página. El resumen de un proyecto vacío (PRY-22) lleva a
esta vista con `Empezar el diseño`, que deja de estar inactivo, y el de un
proyecto con datos con el enlace `Ver el diseño de la investigación`. Sin
proyecto abierto, `/diseno` responde 404.

**Criterios de aceptación.**
- DIS-03.1 En un proyecto nuevo, `/diseno` muestra los cinco títulos en orden,
  cada uno con `Sin empezar`.
- DIS-03.2 Después de guardar el paso 01, su fila muestra el comienzo de la
  primera respuesta y `En borrador`.
- DIS-03.3 El resumen de un proyecto nuevo tiene `Empezar el diseño` como
  enlace a `/diseno`, sin `Todavía no se hace desde aquí.` en ese grupo.

### DIS-04 Página de un paso y guardado

**Comportamiento.** La página `/diseno/<paso>` muestra a la izquierda los
cinco pasos con su estado, con el actual marcado; en el centro el número y el
título del paso y, por cada pregunta guía, la pregunta, su ayuda si la tiene y
un campo de texto largo con la respuesta guardada; y a la derecha el panel de
DIS-02, también en una pantalla de 1366 px de ancho. Debajo de los botones va el
enlace `Siguiente paso: <título>`, salvo en el paso 05, y la barra de arriba
lleva el nombre del proyecto. Al pie, `Este diseño se guarda en la carpeta
diseno del proyecto, un archivo por paso, que se puede abrir con cualquier
editor de texto.` Debajo de
las respuestas van el botón `Guardar`, el botón `Marcar como revisado` (o
`Volver a borrador` si el paso está revisado) y el estado del guardado de
PRY-13 (`Guardado`, `Guardando…` o `No se pudo guardar: <motivo>.`). Un
`<paso>` que no es de `01` a `05` responde 404.

En cuanto se cambia una respuesta, el estado del guardado dice `Hay cambios
sin guardar`, y salir de la página con cambios sin guardar pide confirmación
del navegador. Después de guardar, el estado dice `Guardado a las <HH:MM>`
también junto a los botones, y las etiquetas del paso muestran su estado
nuevo sin recargar. `Marcar como revisado` con cambios sin guardar primero los
guarda y después marca el paso; si el guardado falla, no marca nada (D-55).

`Guardar` manda las respuestas a la API con la versión del paso que la página
recibió. La app normaliza cada respuesta a NFC, cambia los fines de línea a
LF, quita los espacios y saltos del principio y del final, y escribe el
archivo del paso de forma atómica (PRY-12). Si la versión en disco no es la
que recibió la página, no escribe y responde 409 con `{"error":
"cambio_externo", "mensaje": "El archivo <ruta> cambió fuera de la
herramienta mientras estaba abierto. No se guardó el cambio para no
pisarlo."}`, el mensaje de PRY-14; recargar la página muestra lo que hay en
el disco. Cada guardado anota en el registro de eventos `diseno_guardado` con
`detalle.paso`, sin el texto. Guardar respuestas idénticas a las del disco no
escribe nada ni anota nada.

**Entradas.** `POST` con `{"respuestas": [<texto>, …], "version": <texto>}`,
con exactamente una respuesta por pregunta guía del paso. La página y la API
de lectura (`GET`) entregan las respuestas, la versión y el estado del paso.
Revisar es otro `POST` con `{"revisado": <verdadero o falso>}`.

**Salidas.** Estado 200 con `{"version": <versión nueva>, "estado": <estado
del paso>}`.

**Errores.**

| Condición | Estado | `error` | Mensaje |
|---|---|---|---|
| Sin proyecto abierto | 409 | `sin_proyecto` | `No hay un proyecto abierto.` |
| El cuerpo no tiene la forma esperada o la cantidad de respuestas no es la del paso | 400 | `pedido_invalido` | `El pedido no tiene la forma esperada.` |
| Una respuesta pasa de 20 000 caracteres | 422 | `respuesta_larga` | `Cada respuesta puede tener hasta 20 000 caracteres.` |
| El archivo cambió en el disco | 409 | `cambio_externo` | el de arriba |
| Sin permiso de escritura o disco lleno | 500 | `escritura` | `No se pudo guardar el diseño (<motivo en español>).` |

**Criterios de aceptación.**
- DIS-04.1 Guardar el paso 01 con cuatro respuestas deja `diseno/01-tema-e-interes.md`
  con el formato de DIS-06, y la página del paso muestra las respuestas.
- DIS-04.2 Guardar con una versión que no es la del disco responde 409 y deja
  el archivo igual byte a byte.
- DIS-04.3 Guardar dos veces lo mismo deja una sola línea `diseno_guardado`.
- DIS-04.4 Una respuesta escrita con la ñ descompuesta se guarda en NFC.
- DIS-04.5 Cada fila de la tabla de errores, salvo la de escritura, responde
  su estado, su código y su mensaje sin escribir nada.

### DIS-05 Estado de cada paso y coherencia

**Comportamiento.** Cada paso tiene uno de cuatro estados, que se muestran
siempre con texto e ícono, nunca como porcentaje.

| Estado | Texto | Cuándo |
|---|---|---|
| Sin empezar | `○ Sin empezar` | Ninguna respuesta tiene texto |
| En borrador | `◐ En borrador` | Hay texto y el paso no está revisado |
| Revisado | `✓ Revisado` | La persona lo marcó como revisado después de su último guardado |
| Revisar coherencia | `△ Revisar coherencia` | Este paso estaba revisado y otro paso, anterior o posterior, que ya tenía texto, se guardó con otro texto después de esa revisión |

Solo un paso revisado pide revisar la coherencia: un paso en borrador todavía
se está escribiendo. Y el primer guardado con texto de un paso no la pide a los
demás; la piden los cambios siguientes. Así la espiral funciona en las dos
direcciones sin avisar en cada avance de la primera pasada (D-55).

Si el archivo de un paso se editó fuera de la app después de su último
guardado o revisión, el paso vuelve a `En borrador`, porque lo revisado ya no
es lo que está en el disco; marcarlo como revisado de nuevo revisa lo que hay.
Marcar como revisado un paso que ya está `Revisado`, o volver a borrador uno
que no lo está, no escribe ni anota nada.

`Marcar como revisado` anota la marca de tiempo de la revisión;
`Volver a borrador` la quita. Guardar el paso con otro texto lo vuelve a
`En borrador`. Marcar como revisado un paso con `△ Revisar coherencia` es la
manera de decir que sigue siendo coherente. En la página de un paso con ese
estado aparece arriba `El paso <n>, <título>, cambió el <fecha>, después de la
revisión de este paso. Hay que releerlo: si sigue de acuerdo con el cambio, se
marca como revisado; si no, se corrige y se guarda.`, con el paso que cambió
más recientemente como enlace a su página y la fecha escrita como `13 de
octubre`. Cada cambio de
revisión anota `diseno_revisado` con `detalle.paso` y `detalle.revisado`.

**Criterios de aceptación.**
- DIS-05.1 Un paso guardado queda `En borrador`, y marcado como revisado queda
  `Revisado`.
- DIS-05.2 Con 01 y 02 revisados, guardar otra vez el 01 deja el 02 en
  `Revisar coherencia`, y marcar el 02 como revisado lo deja `Revisado`.
- DIS-05.6 Con el 01 revisado, el primer guardado con texto del 02 deja el 01
  `Revisado`, y un paso en borrador nunca queda en `Revisar coherencia`.
- DIS-05.5 Con 02 y 04 revisados, guardar otra vez el 04 deja el 02 en
  `Revisar coherencia`: la espiral va también hacia atrás (Sautu et al., 2005,
  p. 34; Kvale, 2011, p. 87).
- DIS-05.3 Un paso sin texto nunca queda en `Revisar coherencia`.
- DIS-05.4 Marcar como revisado un paso sin texto responde 422 con
  `{"error": "paso_vacio", "mensaje": "Un paso sin texto no se puede marcar como revisado."}`.

### DIS-06 Formato en el disco

**Comportamiento.** El diseño vive en la carpeta `diseno/` del proyecto, en la
zona de trabajo, que la app crea con 0700 al guardar el primer paso. Cada paso
es un archivo Markdown, con permisos 0600, que se lee sin la herramienta.

| Paso | Archivo |
|---|---|
| 01 | `diseno/01-tema-e-interes.md` |
| 02 | `diseno/02-posicionamiento-y-supuestos.md` |
| 03 | `diseno/03-problema-estado-de-la-cuestion-y-marco-teorico.md` |
| 04 | `diseno/04-pregunta-y-objetivos.md` |
| 05 | `diseno/05-enfoque-metodologia-y-metodos.md` |

El archivo tiene el título del paso como encabezado de nivel 1 y, por cada
pregunta guía en orden, la pregunta como encabezado de nivel 2 seguida de la
respuesta, separados por una línea en blanco y con un salto de línea final.
Al leer, la respuesta de una pregunta es el texto entre su encabezado y el
siguiente encabezado de nivel 2. Una línea de una respuesta que empezaría un
encabezado de nivel 2 (`## `) se escribe con una barra invertida delante, que
Markdown muestra como texto, y la app la quita al leer, así esa línea sigue en
su respuesta. Una sección de nivel 2 cuyo título no es una
pregunta guía del paso, por ejemplo una pregunta de una versión anterior de
esta especificación o una sección agregada a mano, se conserva tal cual al
final del archivo cada vez que la app lo reescribe, así ninguna respuesta se
pierde si las preguntas cambian.

Las marcas de los guardados y de las revisiones viven en
`diseno/estado.toml`, una tabla por paso (`["01"]` a `["05"]`) con las claves
`guardado` y `orden_guardado` y, si está revisado, `revisado` y
`orden_revisado`, además de `orden_primero` (el orden de su primer guardado con
texto) y `huella` (la del archivo que escribió la app). `guardado` y
`revisado` son marcas de tiempo de PRY-01, que
dan la fecha del aviso de coherencia. `orden_guardado` y `orden_revisado` son
enteros de una secuencia que crece en uno con cada guardado o revisión del
diseño, y son los que ordenan los cambios, porque dos pueden caer en el mismo
segundo.

**Criterios de aceptación.**
- DIS-06.1 Un paso guardado y su `estado.toml` se leen con un editor de texto,
  en UTF-8 sin BOM, y `estado.toml` es TOML 1.0 válido.
- DIS-06.2 Después de borrar `.osr/` y volver a abrir, la vista general muestra
  los mismos estados (PRY-10).
- DIS-06.3 Una sección `## Una pregunta que ya no está` agregada a mano al
  archivo de un paso sigue en el archivo, con su texto, después de guardar el
  paso desde la app.
- DIS-06.4 Una respuesta con una línea `## Contexto` se guarda, se vuelve a
  leer completa y guardarla otra vez sin cambios no escribe nada.
- DIS-06.5 La huella del archivo que escribió la app queda en `estado.toml`
  (clave `huella`), y `README.md` tiene las filas de `diseno/` desde el primer
  guardado.

## 4. Fuera de alcance de este tramo

- La matriz de coherencia y el historial de versiones de cada paso, que ya
  están diseñados (D-49).
- El interruptor `Mostrar la guía` y los niveles de andamiaje de PRODUCTO.md
  6.7.7.
- La lista de referencias de Zotero del paso 03.
- El grupo `Para que sea publicable` del panel, con los estándares
  internacionales de calidad y de reporte.
- El guardado automático mientras se escribe; en este tramo se guarda con el
  botón.

## 5. Referencias

- Abarca Rodríguez, A., Alpízar Rodríguez, F., Rojas Benavides, C. y Sibaja
  Quesada, G. (2012). *Técnicas cualitativas de investigación*. Editorial UCR.
- Corona Berkin, S. (2019). *Producción horizontal del conocimiento*.
  Bielefeld University Press.
- Della Porta, D. y Keating, M. (Eds.). (2013). *Enfoques y metodologías de
  las ciencias sociales. Una perspectiva pluralista*. Akal.
- Fals Borda, O. (2015). *Una sociología sentipensante para América Latina*
  (V. M. Moncayo, Comp.). CLACSO; Siglo XXI Editores.
- Flick, U. (2015). *El diseño de investigación cualitativa*. Ediciones
  Morata.
- Kaltmeier, O. (2012). Hacia la descolonización de las metodologías:
  reciprocidad, horizontalidad y poder. En S. Corona Berkin y O. Kaltmeier
  (Eds.), *En diálogo. Metodologías horizontales en ciencias sociales y
  culturales* (pp. 25-54). Gedisa.
- Jiménez Cortés, R. (2021). Diseño y desafíos metodológicos de la
  investigación feminista. *Empiria*, (50), 177-200.
- Kvale, S. (2011). *Las entrevistas en investigación cualitativa*.
  Ediciones Morata.
- Pajares Sánchez, L. (2020). Fundamentación feminista de la investigación
  participativa. *Investigaciones Feministas, 11*(2), 297-306.
- Sautu, R., Boniolo, P., Dalle, P. y Elbert, R. (2005). *Manual de
  metodología. Construcción del marco teórico, formulación de los objetivos y
  elección de la metodología*. CLACSO.
- Villalobos Zamora, L. R. (2017). *Enfoques y diseños de investigación
  social: cuantitativos, cualitativos y mixtos*. EUNED.
- Tutoría 1 del curso 05646, Enfoques Emergentes en Investigación Social
  Aplicada, UNED, 2026.
