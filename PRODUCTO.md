# Producto

## 1. Propósito de este documento

[VISION.md](VISION.md) explica por qué existe Open Social Research, qué
principios la guían y cómo se arma por dentro. Este documento baja esa visión
a un producto concreto, dice para quién es, qué hace y qué no hace cada
versión, con qué cifras se mide que funciona bien y qué queda deliberadamente
afuera. Si algo de aquí choca con la visión, manda la visión en los principios
y este documento en el alcance y el orden.

Se escribió a partir de diez revisiones hechas con IA, cada una desde la
perspectiva de un área. Las áreas fueron el análisis cualitativo y CAQDAS, la
metodología cuantitativa y las encuestas, el SIG aplicado a la investigación
social, la docencia de metodología a distancia, la ética de la investigación y
la protección de datos en Costa Rica, la ciencia abierta y la gestión de datos,
la IA responsable, la experiencia de usuario y la accesibilidad, la
arquitectura local-first y el producto en software libre sin ingresos. Esas
revisiones no son personas ni reemplazan la revisión de especialistas reales,
que queda pendiente para las plantillas metodológicas, la ética y la
geoprivacidad antes de la v1.0 (sección 5).

Donde las revisiones discreparon sobre la versión de un requisito, se eligió
como propuesta provisional el alcance más chico para la v0.1, salvo en el
resguardo de los datos de participantes, que tiene que existir desde la primera
entrevista. Cada discrepancia queda en la sección 13 para que el autor la
confirme o la cambie.

Es un documento vivo. Cada cambio de alcance se anota con fecha y razón en
`docs/decisiones.md`.

## 2. El problema que resuelve

El costo de las licencias deja afuera a buena parte de quienes investigan en
América Latina, y por eso la herramienta es gratuita. Pero ya existen opciones
libres como QualCoder y Taguette, y QualCoder ya habla REFI-QDA, así que lo que
falta es el rastro entre los tramos de una investigación. Una marca de lapicero
en una hoja de mapa, la entrevista de la que salió, el minuto del audio donde la
persona la explicó, el código que se le puso a ese fragmento, la zona donde se
cruza con los datos policiales, la cifra que resume ese cruce y la oración del
informe que la cita viven hoy en cinco programas distintos, unidos a mano. En
cada salto se pierde algo, y lo que se pierde es justamente lo que permite
defender un resultado.

Ese rastro roto produce errores concretos y conocidos. Whisper inventa frases
sobre todo en silencios y pausas largas (Koenecke et al., 2024), y es razonable
esperar lo mismo con el ruido de la calle, algo que se va a medir. Esas frases
terminan codificadas y citadas como si alguien las hubiera dicho. Un nombre real
se cuela en el nombre de un archivo o en una cita. Una cifra vieja llega a una
entrega porque los datos cambiaron y nadie recalculó. Un notebook ejecutado
fuera de orden da un resultado que nadie puede reproducir (Pimentel et al.,
2019). Un punto del mapa queda sin saber de qué entrevista salió. Un mapa de
calor con un ancho de banda arbitrario se publica como verdad sobre un barrio.

Para quien empieza a investigar el problema es doble, porque además de los
saltos entre herramientas tiene que aprender el método mientras aprende los
programas, y la literatura advierte que en ese proceso se confunde la función
del programa con la estrategia de análisis (Woolf y Silver, 2018). Esa persona
suele trabajar en Windows, con 8 GB de RAM, sin GPU, con una conexión irregular
y en español, y ninguna herramienta libre le ofrece hoy transcribir, revisar,
codificar y escribir con buena experiencia, sin internet y con los datos de sus
participantes en su propia máquina.

## 3. Para quién

**El autor, primer usuario.** Estudiante del programa de Investigación Social
Aplicada de la UNED, en Costa Rica. Su investigación estudia la percepción de
inseguridad al caminar en el centro de San José con un diseño mixto triangulado
que integra por zona lo cualitativo y lo cuantitativo. Hace entrevistas
semiestructuradas con guía, usa una hoja de mapa en papel donde las personas
entrevistadas marcan los lugares que consideran peligrosos, trabaja con datos
del OIJ georreferenciados, analiza el contenido de noticias por zona, hace
mapas de densidad en QGIS con sus propios scripts de PyQGIS, transcribe
localmente con Whisper, cita con Zotero y escribe en Quarto y Typst, todo en
Arch Linux. Programa en R. Las primeras entrevistas son en octubre de 2026 y
las entregas de cursos son continuas. Ya tiene una cadena que funciona, por eso
la v0.1 cubre solo lo que esa cadena no resuelve y nunca le impide seguir
trabajando fuera de ella.

**La colega novata.** Estudiante de posgrado o de grado, o investigadora de una
organización pequeña, en su primera investigación con trabajo de campo. Usa una
laptop con Windows 10 u 11, 8 GB de RAM o menos, sin GPU, a veces con la
carpeta Documentos sincronizada con OneDrive sin saberlo. Programa poco o nada,
quizá no conoce QGIS y su tutora usa Atlas.ti o MAXQDA. Necesita un camino
propuesto, saber que puede irse con su trabajo y no trabar su máquina. Es el
público de la v1.0.

**La tutora o docente.** Revisa entregas, libros de códigos y transcripciones.
No va a instalar nada. Necesita salidas legibles, PDF, documentos de texto y un
informe del libro de códigos con ejemplos.

**El equipo pequeño.** Dos a cinco personas que codifican y escriben juntas,
con una persona responsable que custodia los datos identificables. Es el
público de lo que viene después de la v1.0.

### Supuestos

Varias metas de este documento descansan en supuestos que todavía no tienen
dato. Se listan aparte para saber qué medir primero.

| Supuesto | Cómo se valida | Qué cambia si es falso |
|---|---|---|
| Las colegas novatas usan Windows con 8 GB de RAM o menos | Entrevistas de descubrimiento (sección 5) | El hardware de referencia y el orden de las plataformas |
| Valoran más la trazabilidad y la integración con R, mapas y Quarto que la paridad con Atlas.ti | Entrevistas de descubrimiento y rondas de pensar en voz alta | El orden de la v1.0, con más peso en codificación y consultas |
| Las tutoras aceptan entregas en PDF y en documentos de texto | Preguntar a dos o tres tutoras del ISA | Las salidas legibles de la v1.0 |
| El R del autor carga sus paquetes sin compilar nada nuevo | Prueba del primer día del tramo 1 | El tramo 3 y la fecha del plan B |
| Codificar solo transcripciones ya revisadas no estorba el trabajo del autor | Uso real en el tramo 2 | Se adelanta un sistema de anclas más robusto o Automerge |
| El autor dispone de un número estable de horas semanales para construir | Cifra anotada en `docs/decisiones.md` (sección 13) | El tamaño de cada tramo y la fecha del plan B |

## 4. Qué es y qué no es

Open Social Research es un banco de trabajo local para investigación social
que une campo, transcripción, codificación, análisis en R y Python, mapas y
escritura en Quarto alrededor de una carpeta, con un rastro auditable de punta
a punta y con los datos en la máquina de quien investiga (Kleppmann et al.,
2019). Su valor está en la integración y la trazabilidad, no en igualar a cada
herramienta especializada. En la v0.1 y la v1.0 convive con QGIS, Zotero,
Quarto y el editor de cada persona, los orquesta y los complementa. Que más
tramos pasen adentro, como producir los mapas finales, lo decide el uso real,
en línea con la promesa de la visión de trabajar en un solo lugar.

Estos son sus no-objetivos explícitos. Una propuesta que caiga en esta lista no
entra en ninguna versión sin cambiar antes este documento.

| No-objetivo | Razón |
|---|---|
| Paridad de funciones con Atlas.ti, MAXQDA o NVivo, lista por lista | Son treinta años de desarrollo y el valor está en la integración y la trazabilidad |
| Motor estadístico, espacial o de minería de texto propio en Rust o JavaScript | Sería menos probado y menos auditable que survey, sf, spatstat o statsmodels, y contradice el principio de orquestar |
| Interfaz de menús al estilo SPSS o un modo que esconda por completo el código | Duplica R y Python y rompe el principio de código visible y editable, el andamiaje simplifica pero no oculta |
| Un QGIS pequeño, con edición topológica, editor de simbología o compositor de láminas de impresión | QGIS es libre y maduro y competir con él en edición agota el proyecto, la app muestra los mapas y ofrece abrir la capa en QGIS |
| Temas, hallazgos, conclusiones o párrafos de resultados generados por IA y presentados como análisis | La IA no firma, vaciaría la interpretación, que es el trabajo de quien investiga |
| Cifras en la prosa del documento que no provengan de un resultado fijado | Rompe la cadena de trazabilidad |
| IA que elige la prueba estadística y redacta la conclusión sustantiva | Convierte el umbral de significancia en interpretación automática |
| Autocodificación sin revisión humana segmento por segmento, o un botón de aceptar todo | Rompe la responsabilidad interpretativa y alimenta el sesgo de automatización (Parasuraman y Manzey, 2010) |
| Indicador automático de saturación | La saturación no es una cifra calculable y su uso como criterio está discutido (Braun y Clarke, 2021) |
| Ruta de análisis obligatoria o cálculo obligatorio de acuerdo entre codificadores | El análisis temático reflexivo rechaza el acuerdo como criterio (Braun y Clarke, 2019), otras posturas lo defienden (O'Connor y Joffe, 2020), y el pluralismo exige que sea opcional |
| Participantes sintéticos, entrevistas simuladas o datos inventados por IA | Sustituye la voz de personas reales y borra la diferencia entre dato y suposición |
| Entrevistador automático o IA que conduce entrevistas | La entrevista es una relación entre personas |
| Detección de emociones, sentimiento, veracidad o rasgos de participantes por su voz o su texto | Es inferencia de validez dudosa sobre personas vulnerables, sin pregunta metodológica que la justifique |
| Consultas por cédula, cruces con padrones, reconocimiento facial, identificación biométrica de hablantes entre entrevistas o seguimiento de perfiles individuales en redes | Son capacidades de vigilancia, no de investigación, separar quién habla dentro de una misma entrevista sí es válido |
| Transcripción en servicios de nube de terceros, o entrenar y ajustar modelos con datos de participantes | La voz es un identificador directo y el consentimiento típico no cubre esa transferencia, transcribir en infraestructura que controla quien investiga o su universidad, como un clúster por SSH, se permite solo si el consentimiento lo cubre y con la confirmación explícita y registrada de la sección 9 |
| Telemetría, analítica de uso o reporte automático de fallos, aunque sean anónimos u opcionales | Contradice local-first y un volcado de fallo puede llevar fragmentos de entrevistas |
| Cuentas de usuario, servidor central, servicio de sincronización propio o alojamiento de datos de terceros | Tiene costos fijos y obligaciones de seguridad que un proyecto sin ingresos no puede sostener |
| Servidor propio de teselas o de geocodificación | Rompe local-first, tiene costo y concentra datos sensibles |
| Ser un repositorio de datos o emitir identificadores persistentes propios (DOI, ARK) | La preservación exige una institución, para eso existen Zenodo y los repositorios universitarios |
| Un esquema de metadatos propio o DDI-Lifecycle completo | Ya existen DDI-Codebook, DataCite, RO-Crate y CSL, uno propio rompe la interoperabilidad |
| Conectores por raspado de sitios, como el del OIJ, dentro del núcleo | Se rompen con cada cambio del sitio, esos datos entran como archivos con su ficha |
| Certificar que algo está anonimizado, que cumple la Ley 8968 o que tiene aval ético | La anonimización es un juicio contextual y la aprobación es de un comité, la herramienta asiste y registra |
| Publicar audios o transcripciones sin anonimizar | La zona identificable no tiene camino de salida pública, por diseño |
| Un servicio de puntajes de zonas peligrosas, rutas seguras o alertas de delito | Es otro producto, con riesgo de estigma territorial sobre barrios ya marginados (Wacquant, 2007), y no impide publicar resultados de investigación con mapas agregados, como el sitio de devolución del autor |
| Alojar encuestas en línea | KoboToolbox y LimeSurvey ya lo hacen gratis y exigiría un servidor público |
| Calificar, rúbricas o gestión de cursos | Es una herramienta para investigar y las universidades ya tienen Moodle, usarla para enseñar metodología es otra cuestión (sección 13) |
| Ejecutar código en la computadora de otra persona por la red P2P | Sería ejecución remota de código entre pares, cada quien ejecuta en su máquina |
| Compilar paquetes de R o Python desde el código fuente en la máquina de la persona usuaria | Es la principal fuente de fallos para novatas, si no hay binario la app avisa o propone cómputo externo, y en la v0.1 el autor usa la biblioteca de R que ya tiene |
| Recorridos guiados largos que tapan la interfaz al primer arranque | La gente se los salta y quiere empezar a trabajar (Carroll y Rosson, 1987), la ayuda va en contexto |
| Plan de pago, versión premium, núcleo abierto con extras cerrados o venta de soporte | El proyecto no se monetiza y ese modelo empuja a dejar funciones fuera de la versión libre |
| Garantías de soporte o tiempos de respuesta comprometidos | Una sola persona voluntaria no puede cumplirlos sin agotarse |
| Publicar la aplicación de escritorio en tiendas de aplicaciones, y un consejo formal de gobernanza antes de que haya contribuciones regulares | Las tiendas cobran o imponen revisiones y gobernanza sin comunidad es burocracia, la distribución de la aplicación móvil queda abierta junto con su tecnología |
| Windows de 32 bits, Windows 7 u 8.1 y navegadores sin soporte vigente | R, Python, Quarto y whisper.cpp ya no los soportan bien, Windows 10 es una excepción declarada en la sección 7 |

## 5. Versiones

Hay tres horizontes, v0.1, v1.0 y después. Ninguna fecha se promete, porque la
investigación del autor siempre tiene prioridad sobre la herramienta.

### v0.1, el núcleo que usa el autor este semestre

**Objetivo.** Cubrir solo los tramos que la cadena actual del autor no
resuelve, encajados en la misma carpeta y sin desplazar nada. Son seis piezas.
La primera es el resguardo de los datos de participantes desde el primer día,
con las dos zonas, el consentimiento y la seudonimización. La segunda es la
importación de las transcripciones que ya produce su Whisper y su revisión
sincronizada con el audio. La tercera es la codificación con un libro de
códigos y su recuperación. La cuarta es el paso de esa codificación a R como
tablas. La quinta son los resultados fijados desde sus scripts de R, con
manifiesto, aviso de desactualización y citas desde Quarto. La sexta es el
enlace entre las marcas de la hoja de mapa, la entrevista y el fragmento.

**Quién la usa.** Solo el autor, en su equipo con Arch Linux x86-64.

**Qué no trae.** Transcripción dentro de la app, notebooks y kernels de
Jupyter, entornos gestionados (rig, rv, uv), Python, Automerge y colaboración,
editor de documentos, integración directa con Zotero, IA de ningún tipo, visor
de mapas, conectores a repositorios, respaldo gestionado, Windows y macOS. El
autor sigue usando su Whisper, su editor, Quarto, QGIS y su R para todo eso.

**Cómo se construye.** En tres tramos, cada uno usado en trabajo real antes de
empezar el siguiente.

1. Carpeta y formato, seguridad del servidor local, participantes,
   consentimiento y las dos zonas, importación de audios y de transcripciones,
   y editor de revisión. El primer día se prueba que el R del autor carga sus
   paquetes, como tidyverse y sf, sin compilar nada nuevo, porque Arch no está
   entre las distribuciones con binarios de Posit Package Manager [verificar].
2. Seudonimización con `osr seudonimizar`, libro de códigos, codificación,
   memos de código y de segmento, recuperación, tabla de casos, tablas CSV para
   R y citas de fragmentos en Quarto.
3. Paquete de R con `fijar()` y `leer_insumo()`, subcomandos `osr recalcular`
   y `osr estado`, manifiesto, aviso de desactualización, extensión de Quarto
   para resultados fijados, importación de las marcas de la hoja de mapa y
   fichas de procedencia de los datos externos.

**Plan B.** La v0.1 no va a existir para las primeras entrevistas de octubre.
Esas entrevistas se graban, se transcriben con la cadena actual y se resguardan
fuera de carpetas sincronizadas y en un disco cifrado, y la v0.1 las importa
después sin rehacer nada. Antes de empezar el tramo 2 el autor anota en
`docs/decisiones.md` una fecha concreta. Si en esa fecha el núcleo de
codificación no está usable, la codificación sigue en QualCoder, la v0.1 trae
las transcripciones y lee en R las tablas de segmentos que QualCoder exporta en
CSV [verificar formato de exportación], y la codificación de ese período se
queda en QualCoder hasta que exista la importación de `.qdpx` (sección 8). Si
el tramo 3 no está listo para la entrega, la v0.1 se cierra con los tramos 1 y
2, y los resultados fijados pasan a una v0.2 junto con su criterio de
terminación. Activar el plan B no da por terminada la v0.1, solo evita que la
investigación espere.

**Qué la da por terminada.**

- El autor entrega al menos un trabajo de curso antes de que termine 2026 con
  entrevistas revisadas, seudonimizadas y codificadas en la herramienta,
  fragmentos citados desde el documento Quarto por referencia estable y al
  menos un resultado fijado citado en el texto.
- Cero pérdidas de datos o de codificación en el uso real, incluidos cierres
  forzados, comprobado con las pruebas que matan el proceso mientras escribe y
  con la verificación al abrir el proyecto, que compara el índice con los
  archivos de texto y registra cualquier diferencia.
- El README generado describe cada carpeta y cada tipo de archivo, y las tablas
  CSV abren en R sin la app.
- El 100 % de las entrevistas tiene registro de consentimiento con alcances.
- Las pruebas automáticas pasan sin un solo fallo. Son las de seguridad del
  servidor local, donde los intentos sin token, con un Host ajeno y desde otro
  origen fallan en el 100 % de las rutas, la de ejecución sin red, la de
  desactualización sembrada, donde se marcan el 100 % de los resultados
  dependientes de un insumo alterado y ninguno de los que no dependen, la de
  reconstrucción del índice, la de anclas al reabrir una transcripción
  codificada y la de recalcular sin cambios con la misma huella.

### v1.0, abierta a colegas novatas

**Objetivo.** Que una colega novata con una laptop Windows modesta instale la
herramienta sola, transcriba, revise, seudonimice, codifique, analice con un
camino propuesto y escriba, y que pueda irse con su trabajo cuando quiera.

**Quién la usa.** El autor y un primer grupo de colegas, sobre todo
estudiantes de posgrado, en Windows 10 y 11 y en Linux.

**Qué agrega.** Windows como plataforma principal, instalación gestionada de
R, Python y Quarto con un primer uso que explica qué se descarga, transcripción
integrada con whisper.cpp y recomendación de modelo según la máquina,
notebooks de R y Python dentro de la app que fijan resultados con un clic,
editor de documentos con citas de Zotero, asistente de IA en notebooks con
compuerta de salida de datos si alguien lo usa en una investigación real,
exportación REFI-QDA, consultas y matrices cualitativas en la interfaz,
andamiaje según la tradición metodológica declarada, proyecto de ejemplo
completo, visor de mapas, proyectos recibidos que abren como no confiables,
chequeos de privacidad antes de publicar y accesibilidad WCAG 2.2 AA en los
flujos centrales.

**Qué la da por terminada.**

- Cada función nueva se usó en al menos una investigación real, del autor o de
  una colega piloto, antes de publicarse. La que no tuvo ese uso pasa a
  después con una entrada en `docs/decisiones.md`.
- Al menos una persona real con formación en el área revisa las plantillas
  metodológicas, la sección 9 y el chequeo de geoprivacidad, y sus
  observaciones quedan resueltas o registradas.
- Cinco colegas novatas, en computadoras como la de referencia, completan sin
  ayuda la tarea guiada (instalar, importar una entrevista, transcribirla,
  revisarla, crear tres códigos definidos, codificar y citar un fragmento) y
  al menos cuatro lo logran en 30 minutos o menos sin contar el tiempo de
  transcripción.
- Puntaje SUS promedio de 68 o más, que es el promedio de referencia (Sauro y
  Lewis, 2016), con meta de 75, medido con una versión en español validada del
  cuestionario (Brooke, 1996; Sevilla-Gonzalez et al., 2020) y reportado con su
  intervalo, porque con cinco personas el intervalo es muy ancho.
- Un proyecto exportado a `.qdpx` abre en QualCoder con el 100 % de códigos,
  segmentos y memos.
- La instalación desde cero pasa en una máquina virtual Windows limpia de
  8 GB, con un usuario cuyo nombre lleva tilde y ñ.
- Un proyecto creado en v0.1 abre en v1.0 sin pérdida y sin pasos manuales.
- Una prueba de salida demuestra que una persona que no conoce la app entiende
  la carpeta y recupera todo su trabajo solo con el explorador de archivos y
  otra herramienta.
- Borrar el entorno y restaurarlo desde el archivo de bloqueo regenera todos
  los resultados fijados en 30 minutos o menos, idénticos en la misma
  plataforma y dentro de la tolerancia anotada entre plataformas.
- Los presupuestos de memoria y arranque de la sección 7 se cumplen en
  mediciones repetidas en el hardware de referencia.

### Después

Aquí va lo que depende de tener la v1.0 estable o de decisiones todavía
abiertas, que es la colaboración P2P con Automerge e iroh, los permisos por
proyecto, la codificación en equipo y el acuerdo entre codificadores, la
aplicación móvil de campo, los conectores a repositorios de datos, el cómputo
externo, macOS, la bóveda con contraseña para los datos sin anonimizar junto
con la preparación de la versión censurada antes del análisis y la guía de
los lineamientos de la PRODHAB, la IA para codificación y clasificación, el análisis de corpus
grandes, la digitalización de la hoja de mapa dentro de la app, la importación
de `.qdpx` y el sitio web de divulgación generado desde la app.

**Quién lo usa.** Equipos pequeños y colegas que ya trabajan con la v1.0.

**Qué lo da por terminado.** No hay un corte único, cada pieza entra con su
propio criterio de terminación cuando una investigación real la pide, y la
decisión queda registrada en `docs/decisiones.md`.

### Validación con usuarias antes de cerrar el alcance de la v1.0

Los criterios de la v1.0 miden con usuarias al final, y hace falta también
validar antes de construir.

- De 5 a 8 entrevistas de descubrimiento con estudiantes del ISA y de otros
  posgrados sobre su flujo actual, su sistema operativo y su hardware, para
  responder los supuestos de la sección 3.
- Rondas de pensar en voz alta con cinco novatas en sus propias computadoras
  (Nielsen y Landauer, 1993), con la tarea guiada de los criterios de la v1.0,
  un guion escrito y una persona que observa y anota.
- Esas sesiones usan solo el proyecto de ejemplo o los datos de prueba, nunca
  datos reales de participantes, y llevan su propio consentimiento informado,
  con la grabación como opción.
- Como el autor construye la herramienta y es compañero de quienes la prueban,
  otra persona facilita las sesiones cuando se pueda, el cuestionario SUS se
  responde de forma anónima y los hallazgos se anotan antes de discutirlos con
  él.

## 6. Requisitos funcionales por módulo

Los módulos siguen el ciclo de la visión, con un núcleo transversal al
principio. Los requisitos de consentimiento, seudonimización y resguardo están
en la sección 9, y los de IA y conectores en la sección 8.

Las prioridades significan lo siguiente. «Imprescindible» bloquea la versión.
«Importante» puede pasar a la versión siguiente con una entrada en
`docs/decisiones.md`, y si un tramo de la v0.1 se atrasa, lo importante de ese
tramo es lo primero que se recorta. «Deseable» no tiene compromiso, y la v0.1
no tiene filas deseables. En las filas de después la prioridad solo indica el
orden probable.

Cuando un requisito habla de bloquear, la regla general es que la herramienta
bloquea por defecto y permite una excepción que la persona confirma y que queda
registrada. Solo lo que exige la ética no tiene excepción, como sacar material
identificable a la nube o citar textualmente a alguien cuyo consentimiento no
lo autoriza.

### 6.0 Núcleo transversal

| Requisito | Versión | Prioridad |
|---|---|---|
| Un proyecto es una carpeta con un archivo `proyecto.toml` que declara la versión del formato, y el binario `osr` la abre, levanta el servidor y abre el navegador | v0.1 | imprescindible |
| Todo lo que no es audio o imagen queda en texto abierto (UTF-8, CSV, JSON, JSON Lines o YAML, `.qmd`, WebVTT, GeoPackage), con un README generado que explica cada carpeta y cada tipo de archivo, cómo respaldar la carpeta con una herramienta del sistema y que la zona identificable solo va a un disco cifrado | v0.1 | imprescindible |
| Los archivos de texto son la fuente de verdad, ningún dato ni metadato vive solo en SQLite, y su formato está documentado para migrarlo a Automerge cuando llegue la colaboración | v0.1 | imprescindible |
| El índice SQLite vive en `.osr/` y es reconstruible, con una prueba automática que lo borra, reabre y verifica el mismo estado | v0.1 | imprescindible |
| Guardado automático con escrituras atómicas, deshacer y rehacer en codificación y edición, recuperación tras cierre forzado y verificación al abrir que compara el índice con los archivos | v0.1 | imprescindible |
| Servidor solo en 127.0.0.1 y ::1, puerto aleatorio, token aleatorio de 128 bits o más por arranque en el 100 % de las rutas, validación de Host y Origin, sin CORS | v0.1 | imprescindible |
| La v0.1 no contiene código de red saliente, y una prueba automática corre la app en un espacio de nombres de red sin salida y falla si intenta conectarse | v0.1 | imprescindible |
| Datos de prueba inventados y mínimos en el repositorio, con dos entrevistas cortas, una hoja, una capa y un script de R, usados por las pruebas automáticas, y nunca datos reales | v0.1 | imprescindible |
| La app se niega a abrir en modo escritura un proyecto de un formato más nuevo que el suyo | v1.0 | imprescindible |
| `FORMATO.md` que documenta la carpeta, migraciones versionadas con respaldo previo y registro en la bitácora | v1.0 | imprescindible |
| Ninguna conexión saliente sin acción explícita, con un indicador visible cuando hay una en curso | v1.0 | imprescindible |
| Archivos de conexión de los motores de análisis con permisos 0600 y firma HMAC | v1.0 | imprescindible |
| Instancia única (abrir de nuevo enfoca la pestaña existente), indicador de que la app está corriendo, botón explícito para cerrarla, cierre automático tras un tiempo sin pestañas y manejo del puerto ocupado sin mensajes técnicos | v1.0 | imprescindible |
| Textos de la interfaz externalizados en catálogos de mensajes | v1.0 | importante |
| Respaldo del proyecto a un disco externo con verificación de integridad y aviso cuando pasan más de 7 días sin respaldo. Si el destino no está cifrado, la app lo advierte y por defecto excluye la zona identificable o la copia en un archivo cifrado con contraseña, por ejemplo con age, y registra en la bitácora qué se respaldó y dónde | v1.0 | importante |
| Actualizar la app con aviso de versión nueva, migración de proyectos con respaldo previo y posibilidad de volver a la versión anterior | v1.0 | importante |
| Ver cuánto ocupa cada componente descargado (R, Python, Quarto, modelos de Whisper) y desinstalarlo, o desinstalar todo lo descargado, sin tocar los proyectos | v1.0 | importante |
| Botón para copiar un diagnóstico (versiones, sistema, últimas líneas del registro) sin rutas ni contenido del proyecto, que la persona revisa antes de pegar | v1.0 | importante |

### 6.1 Proyecto y bitácora

| Requisito | Versión | Prioridad |
|---|---|---|
| Bitácora en Markdown con entradas fechadas de tipo explícito (decisión metodológica, reflexión, analítica, de campo) | v0.1 | imprescindible |
| Plantilla Markdown de hoja de resumen de contacto en la carpeta de cada entrevista (temas que aparecieron, qué sorprendió, qué preguntar en la próxima), al estilo de Miles et al. (2014), sin interfaz propia | v0.1 | importante |
| Entradas automáticas en la bitácora al fijar o recalcular un resultado y al renombrar, fusionar o mover códigos | v0.1 | importante |
| Interfaz para la hoja de resumen de contacto, ligada a su entrevista | v1.0 | importante |
| Plantilla `.qmd` para redactar el proyecto (problema, objetivos, diseño, instrumentos, cronograma) | v1.0 | importante |
| Declaración de tradición metodológica por proyecto que define el camino recomendado, las vistas por defecto y las advertencias, y que se puede cambiar dejando registro | v1.0 | imprescindible |
| Matriz de coherencia que enlaza pregunta, objetivos, categorías o variables, ítems de cada instrumento y técnica de análisis, y señala objetivos sin instrumento o preguntas de la guía sin objetivo | v1.0 | importante |

### 6.2 Lecturas y referencias

| Requisito | Versión | Prioridad |
|---|---|---|
| Los documentos del proyecto usan el `.bib` o CSL-JSON que la persona exporta de Zotero, y la cita sugerida de cada ficha de procedencia se carga a Zotero como cualquier otra referencia | v0.1 | imprescindible |
| Bibliografía de datos en CSL-JSON generada desde las fichas de procedencia, con tipo dataset, versión y fecha de acceso, que Quarto usa junto con la de Zotero | v1.0 | importante |
| Buscar e insertar citas desde la API local de Zotero, que en Zotero 7 es de solo lectura y hay que activar en las preferencias [verificar], detectando si Zotero está abierto y cayendo al `.bib` exportado si no lo está | v1.0 | importante |
| Agregar ítems a Zotero desde la app, por el conector o la API web con llave propia | después | deseable |

### 6.3 Instrumentos

| Requisito | Versión | Prioridad |
|---|---|---|
| Registro de cada hoja de mapa con su `hoja_id`, su entrevista, su fecha y su archivo escaneado | v0.1 | importante |
| Extensión, escala y sistema de referencia de cada hoja, registrados junto con la plantilla de hojas | v1.0 | importante |
| Plantilla con código visible para generar hojas de mapa A4 y carta desde la capa de zonas, con `hoja_id`, campo para el código de entrevista, cuatro marcas de esquina, escala, norte y fuente | v1.0 | importante |
| Guías de entrevista con versiones fechadas y la razón de cada cambio en la bitácora, cada pregunta ligada a un objetivo de la matriz de coherencia | v1.0 | importante |
| Crear el conjunto inicial de códigos a partir de las preguntas o temas de la guía de entrevista | v1.0 | deseable |
| Cuestionarios en un formato de intercambio abierto, como XLSForm, con exportación a libro de códigos y a esquema de la base | después | deseable |
| Diseñador visual de guías e instrumentos imprimibles | después | deseable |

### 6.4 Campo

En la v0.1 los audios, las fotos y las notas que vienen del teléfono entran por
la importación de la sección 6.5, sin aplicación propia.

| Requisito | Versión | Prioridad |
|---|---|---|
| Importar GPX, GeoJSON, KML y CSV con coordenadas de aplicaciones existentes (GPS del teléfono, QField, OsmAnd), convertidos a GeoPackage con su procedencia | v1.0 | importante |
| Aplicación móvil de campo sin conexión que graba, toma notas, fotos y ubicaciones, y se sincroniza con el proyecto | después | importante |
| Registro de consentimiento en la aplicación móvil, oral grabado o firmado en pantalla | después | importante |
| Recorridos acompañados que sincronizan la traza GPS con el tiempo del audio para ubicar cada fragmento donde se dijo (Kusenbach, 2003) | después | deseable |
| Mapa participativo digital en tableta como alternativa a la hoja de papel, sin reemplazarla | después | deseable |

### 6.5 Datos

| Requisito | Versión | Prioridad |
|---|---|---|
| Importar audio (m4a, mp3, wav, ogg, opus), fotos (JPEG, HEIC) y notas (TXT, Markdown) arrastrando o eligiendo el archivo, a la zona que corresponda, con duración y estado visibles, y guardar el original inmutable, sin recomprimir y con su hash | v0.1 | imprescindible |
| Ficha de procedencia completa para cada dato externo, como los del OIJ, las capas oficiales o las bases públicas (título, productor, vía de obtención, fecha de acceso, licencia o condiciones de uso, cita sugerida, SHA-256 y nota de precisión), guardada como archivo de texto junto al dato | v0.1 | imprescindible |
| Ficha corta para el material propio de campo (audios, fotos, escaneos), con hash, fecha, seudónimo y vínculo al consentimiento | v0.1 | imprescindible |
| Datos crudos inmutables, los archivos de `datos/crudos/` quedan de solo lectura en el sistema de archivos al importarse y `fijar()` se niega a escribir en esa ruta, y toda limpieza produce un archivo derivado cuyo script queda registrado como insumo o resultado fijado (Wilson et al., 2017) | v0.1 | imprescindible |
| Importar documentos para codificar (DOCX, PDF con texto, TXT, Markdown), incluidas notas de prensa | v1.0 | imprescindible |
| Detección de codificación (UTF-8, Windows-1252, Latin-1) y de separador (coma, punto y coma, tabulador) al importar CSV, avisando antes de convertir | v1.0 | imprescindible |
| Diccionario de datos automático al importar `.sav`, `.dta`, `.csv` y `.xlsx` sin perder etiquetas de variable y de valor, con porcentaje de faltantes | v1.0 | importante |
| Diccionario de datos editable para las tablas propias, exportable a CSV | v1.0 | importante |
| Importar respuestas abiertas de encuestas desde CSV, cada fila como documento con sus variables | después | deseable |

### 6.6 Transcripción

| Requisito | Versión | Prioridad |
|---|---|---|
| Importar transcripciones en el JSON de Whisper o faster-whisper que produce la cadena del autor y en WebVTT, junto con su audio, conservando las marcas de tiempo por segmento y las probabilidades cuando el archivo las trae | v0.1 | imprescindible |
| Editor de revisión sincronizado con el audio, clic en el texto reproduce desde esa marca, atajos para pausar y retroceder de 3 a 5 s, velocidad de 0,5x a 2x, todo operable con teclado | v0.1 | imprescindible |
| Toda transcripción nace como automática sin revisar, con estado por segmento y por transcripción, registro de cuándo se revisó y con qué modelo se generó | v0.1 | imprescindible |
| La salida cruda de la máquina se conserva junto a la versión revisada, y nunca se re-transcribe automáticamente sobre una transcripción revisada | v0.1 | imprescindible |
| Turnos de habla asignados a mano (persona entrevistadora o entrevistada) con atajo, y filtro para codificar o recuperar solo los turnos de la persona entrevistada | v0.1 | importante |
| Marcar como sospechosos los segmentos con `no_speech_prob` mayor que 0,6 y `avg_logprob` menor que -1,0, con `compression_ratio` mayor que 2,4, o con el mismo texto repetido en segmentos seguidos, que son los umbrales por defecto de Whisper (Radford et al., 2023) [verificar en el código de la versión usada], configurables y anotados en el proyecto, omitiendo con un aviso la regla cuyo campo no traiga el JSON, y saltar al siguiente sospechoso con un atajo (Koenecke et al., 2024) | v0.1 | importante |
| Registrar el tiempo de revisión de cada transcripción, como dato y sin meta | v0.1 | importante |
| Exportar la transcripción revisada a Markdown o texto plano con marcas de tiempo, hablantes y seudónimos aplicados | v0.1 | importante |
| Importar transcripciones en SRT, TXT y DOCX | v1.0 | importante |
| Transcribir dentro de la app con whisper.cpp, idioma fijado en español sin autodetección ni traducción, detección de voz previa, en segundo plano con prioridad baja, progreso, estimación, pausa, cancelación y reanudación, sin bloquear la codificación de otras entrevistas | v1.0 | imprescindible |
| Recomendación del modelo según la RAM disponible, con una prueba de velocidad de un minuto de audio real, tamaño y memoria anunciados antes de descargar y hash verificado antes de usarlo | v1.0 | imprescindible |
| Glosario por proyecto (barrios, calles, siglas, instituciones) usado como contexto inicial de whisper.cpp y como lista de verificación en la revisión, midiendo su efecto porque también puede inducir errores | v1.0 | importante |
| Reporte por transcripción y por modelo de la proporción de palabras corregidas, y de cuántas correcciones cayeron fuera de los segmentos marcados como sospechosos | v1.0 | importante |
| Muestra de evaluación de la transcripción hecha con audio cuyo consentimiento lo permite, o grabado por el autor y voluntarias en calles del centro, con tamaño fijado en minutos y número de hablantes, transcripción de referencia hecha por una persona y una métrica de alucinaciones como frases insertadas por hora de audio | v1.0 | importante |
| Separación automática de hablantes | después | deseable |

### 6.7 Análisis

#### 6.7.1 Cualitativo

El modelo de codificar y recuperar tiende a favorecer el análisis temático y el
de contenido, y sirve peor al análisis narrativo, al de discurso o al
conversacional. Para cumplir el principio de pluralismo, la herramienta ofrece
una unidad de codificación libre, memos sin ruta impuesta y rutas guiadas
presentadas como una opción entre varias.

| Requisito | Versión | Prioridad |
|---|---|---|
| Libro de códigos jerárquico en archivo abierto, cada código con nombre, definición, criterios de inclusión y exclusión, ejemplo y color, en el estilo de MacQueen et al. (1998), y la app señala los códigos sin definición | v0.1 | imprescindible |
| Codificar una selección libre de texto con uno o varios códigos superpuestos, con teclado y con ratón, descodificar y comentar cada segmento, sobre la versión de trabajo seudonimizada de una transcripción marcada como revisada | v0.1 | imprescindible |
| La codificación y los memos se guardan en archivos de texto canónicos, en JSON Lines por transcripción, con anclas por identificador de segmento y desplazamiento, y reabrir una transcripción codificada para editarla avisa qué segmentos codificados toca, sin correr ninguno en silencio | v0.1 | imprescindible |
| Memos de código y de segmento, con fecha, buscables y enlazados al objeto que comentan | v0.1 | imprescindible |
| Recuperar los segmentos de un código y sus subcódigos con contexto, salto al original y al audio, y filtro por atributos | v0.1 | imprescindible |
| Tabla de casos con seudónimo por participante y atributos definidos por quien investiga, cada transcripción ligada a un caso | v0.1 | imprescindible |
| Renombrar, fusionar y mover códigos con entrada automática en la bitácora, el rastro de auditoría que piden Lincoln y Guba (1985) | v0.1 | importante |
| Memos libres y de documento como tipos propios, que en la v0.1 se cubren con la bitácora y la hoja de resumen de contacto | v1.0 | importante |
| Búsqueda léxica insensible a tildes y mayúsculas con palabra en contexto, y autocodificación de resultados revisada uno por uno | v1.0 | importante |
| Consultas por códigos con Y, O, NO, superposición y cercanía, combinadas con atributos, guardables y repetibles | v1.0 | importante |
| Tabla de co-ocurrencia y matriz código por documento o por atributo, con cada celda enlazada a sus segmentos | v1.0 | importante |
| Matriz caso por tema con resúmenes escritos por quien investiga y enlazados a los segmentos que los sostienen (Kuckartz y Rädiker, 2023) | v1.0 | importante |
| Relaciones tipadas entre códigos (causa, parte de, contradice y las que se definan) con una vista de red de solo lectura | v1.0 | importante |
| En tradiciones interpretativas las frecuencias de códigos no se muestran por defecto, y cuando se piden llevan una nota sobre su alcance (Braun y Clarke, 2006) | v1.0 | importante |
| Seguimiento de códigos nuevos por entrevista como apoyo para discutir saturación o poder de información (Malterud et al., 2016), nunca como veredicto | v1.0 | deseable |
| Codificar sobre la línea de tiempo del audio o del video sin transcripción | después | deseable |
| Codificar regiones de imágenes, como la foto de una hoja de mapa | después | deseable |
| Codificar áreas de PDF con su maquetación original | después | deseable |

#### 6.7.2 Resultados fijados y notebooks

Esta es la promesa central del producto y su mayor riesgo técnico. En la v0.1
se cumple sin interfaz de notebook, con un paquete pequeño de R que acompaña a
la app y que el autor llama desde sus propios scripts o `.qmd`, y los notebooks
dentro de la app llegan en la v1.0. En la v0.1 se usa el R del sistema con la
biblioteca de paquetes que el autor ya tiene, lo que no es una excepción a la
regla de no compilar, que vale desde la v1.0 en las plataformas con binarios.
Que la cifra salga de código ejecutado certifica el cómputo, no la validez, por
eso las reglas de ejecución limpia y, desde la v1.0, las advertencias
metodológicas son parte del requisito.

| Requisito | Versión | Prioridad |
|---|---|---|
| Paquete de R del proyecto con `fijar(objeto, id)`, que fija una figura (PNG y SVG), una tabla (CSV) o una cifra (JSON), y `leer_insumo(ruta)`, que lee un dato y lo registra como insumo | v0.1 | imprescindible |
| Solo se puede fijar un resultado producido por una ejecución completa del script en un proceso de R nuevo lanzado por `osr recalcular`, que garantiza por construcción la ejecución lineal (Pimentel et al., 2019) | v0.1 | imprescindible |
| Manifiesto en texto dentro de la carpeta por cada resultado fijado, con identificador estable, SHA-256 del código y de cada insumo, `R.version.string`, `sessionInfo()`, versión de cada paquete cargado, hash de un `rv.lock` o `renv.lock` si ya existe, semilla si la hubo y fecha (Sandve et al., 2013) | v0.1 | imprescindible |
| Un resultado se marca desactualizado cuando cambia su código, un insumo o la versión de un paquete registrado, y se puede recalcular | v0.1 | imprescindible |
| Funciones auxiliares de R para leer las tablas del proyecto (casos, códigos, segmentos, memos, capas) con rutas relativas | v0.1 | imprescindible |
| Notebooks con kernel de R (IRkernel) que se inicia solo al abrir un notebook, con interrumpir, reiniciar, detección de caída y apagado por inactividad, y salidas guardadas como archivos aparte | v1.0 | imprescindible |
| Fijar desde un notebook es una sola acción, el botón reinicia el motor, ejecuta el notebook completo de forma lineal y fija el resultado en un clic, avisando del tiempo si pasa de 10 s | v1.0 | imprescindible |
| Entorno de R por proyecto con rv y archivo de bloqueo versionado en la carpeta | v1.0 | imprescindible |
| Instalación gestionada de R con rig, paquetes con rv, Python con uv y Quarto, solo binarios, con descargas verificadas y reanudables y aviso del espacio antes de descargar | v1.0 | imprescindible |
| Restaurar el entorno desde el archivo de bloqueo en una máquina limpia con una acción, y verificar que los resultados fijados se regeneran con las mismas cifras | v1.0 | imprescindible |
| Kernel de Python con entorno uv y bloqueo, con las mismas reglas de procedencia | v1.0 | importante |
| Registro de solo agregar de todas las ejecuciones, incluidas las especificaciones descartadas, con el número de variantes probadas visible junto a cada resultado fijado (Simmons et al., 2011; Gelman y Loken, 2014) | v1.0 | importante |
| Estado del motor de análisis en lenguaje claro, sin la palabra kernel en la interfaz | v1.0 | importante |
| Explicación en español de los errores más comunes de R y Python (paquete no instalado, objeto no encontrado, archivo inexistente, codificación) | v1.0 | importante |

#### 6.7.3 Cuantitativo

El autor hoy no trabaja con microdatos de encuesta, así que este módulo entra a
la v1.0 solo si una colega piloto los usa en su investigación. Si no, pasa a
después con una entrada en `docs/decisiones.md`.

| Requisito | Versión | Prioridad |
|---|---|---|
| Declaración del diseño muestral al importar microdatos (ponderador, estratos, unidades primarias, réplicas, o no probabilístico), aplicada por las plantillas con survey o srvyr (Lumley, 2010) | v1.0 | importante |
| Advertencia cuando una estimación no usa el diseño declarado, o cuando se presentan valores p o intervalos sobre una muestra declarada no probabilística (Baker et al., 2013) | v1.0 | imprescindible |
| Las estimaciones de encuesta muestran por defecto el n sin ponderar, el error estándar, el intervalo y el coeficiente de variación (Heeringa et al., 2017), con umbrales configurables y citados, como los de Parker et al. (2017) o los que publique el INEC [verificar] | v1.0 | importante |
| Formato numérico es-CR por defecto (coma decimal, fecha día/mes/año), configurable por proyecto, sin alterar los valores guardados | v1.0 | importante |
| Calculadora de tamaño de muestra con cada fórmula citada, que no se ofrece para diseños cualitativos | después | deseable |
| Construcción de ponderadores propios (no respuesta, posestratificación, calibración) | después | deseable |
| Control de divulgación estadística de microdatos propios con sdcMicro | después | importante |

#### 6.7.4 Geográfico

GeoPackage es el formato nativo, el análisis espacial ocurre en R o Python, y
la app se ocupa de que nada pierda su rastro. En la v0.1 y la v1.0 QGIS sigue
siendo la herramienta para editar capas y componer láminas, y desde la v1.0 la
app muestra los mapas del proyecto.

| Requisito | Versión | Prioridad |
|---|---|---|
| Las capas vectoriales viven como GeoPackage en la carpeta, con ficha de procedencia que incluye el sistema de referencia de origen y una nota de precisión (por ejemplo, georreferencia por dirección aproximada) | v0.1 | imprescindible |
| Esquema de marcas de hoja de mapa con `entrevista_id`, `hoja_id`, número de marca, geometría de cualquier tipo (punto, línea o polígono), categoría libre, certeza (exacta, aproximada, zona) y nota, importable desde un GeoPackage o CSV hecho en QGIS. La app valida que los identificadores existan y que el sistema de referencia esté definido, sin reproyectar ni convertir a centroides en silencio, y la validez de las geometrías la informa la función de R con `sf::st_is_valid` (Boschmann y Cubbon, 2014) | v0.1 | imprescindible |
| El escaneo o la foto de cada hoja marcada se guarda como fuente original sensible, ligada a su entrevista y a su `hoja_id` | v0.1 | imprescindible |
| Lectura de todas las capas desde R con sf, con rutas relativas y los identificadores de entrevista, hoja, marca y zona como columnas | v0.1 | imprescindible |
| La capa de zonas de integración es un insumo más con su ficha, y la elección de la zonificación la anota el autor en la bitácora porque los resultados dependen de ella (Openshaw, 1984) | v0.1 | importante |
| Registrar una figura hecha fuera de la app, como una lámina de QGIS o la salida de un script de PyQGIS, con su script, sus capas de insumo y su huella, para que se rastree y se marque desactualizada si cambia una capa | v1.0 | importante |
| Ninguna petición de red geográfica por defecto, sin teselas remotas ni geocodificación en línea | v1.0 | imprescindible |
| Visor MapLibre de las capas del proyecto, cargado solo al abrirse, con tabla de atributos, filtros por entrevista, código o categoría, salto del punto al fragmento, una vista que pone los lugares mencionados en las entrevistas junto a los datos policiales de la zona, y respaldo a imagen estática y tabla si no hay WebGL2 | v1.0 | importante |
| Plantillas espaciales en R y Python con criterio citado, densidad por núcleo comparando al menos tres anchos de banda, densidad sobre la red de calles (Okabe et al., 2009), conteos por zona normalizados por exposición y no por residentes, puntos calientes con Gi* (Getis y Ord, 1992; Ord y Getis, 1995), y advertencia sobre la cifra negra y la población flotante (Chainey y Ratcliffe, 2005; Andresen, 2006) | v1.0 | importante |
| Comparación de un resultado por zona con una zonificación alternativa | v1.0 | importante |
| Nomenclátor del proyecto con lugares, su geometría y sus alias populares, construido a mano desde capas oficiales y OSM | v1.0 | importante |
| Mapa base sin conexión desde un recorte de OSM en un solo archivo, descargado una vez con consentimiento | v1.0 | deseable |
| Rasters GeoTIFF o COG con procedencia, accesibles desde terra y rasterio | v1.0 | deseable |
| Digitalizar marcas dentro de la app sobre la hoja escaneada, georreferenciada con su extensión registrada y las cuatro marcas de esquina | después | importante |

#### 6.7.5 Integración mixta

| Requisito | Versión | Prioridad |
|---|---|---|
| Tablas ordenadas en CSV UTF-8 dentro de la carpeta, actualizadas al codificar (documentos, casos y atributos, códigos, segmentos con texto y tiempos, memos), con nombres de columnas estables y documentados | v0.1 | imprescindible |
| Un segmento puede llevar un vínculo por identificador a una marca de hoja de mapa o a una zona, sin coordenadas sueltas, y eso sale en las tablas (Cope y Elwood, 2009) | v0.1 | importante |
| Vínculo de un segmento a una entrada del nomenclátor | v1.0 | importante |
| Matriz de casos por códigos (presencia y frecuencia) como data frame en R y Python, con los atributos de cada caso | v1.0 | importante |
| Vista cruzada por zona con las marcas, los fragmentos vinculados y los conteos de fuentes externas, con una columna para anotar si convergen, divergen o no hay datos | v1.0 | importante |
| Plantilla de presentación conjunta en Quarto que pone lado a lado resultados fijados y fragmentos por tema o por lugar (Fetters et al., 2013) | v1.0 | deseable |

#### 6.7.6 Análisis de textos y clasificación

| Requisito | Versión | Prioridad |
|---|---|---|
| Análisis de corpus grandes de prensa o comentarios | después | deseable |
| Clasificación asistida con muestra de validación humana obligatoria y métricas de acuerdo visibles (Krippendorff, 2018) | después | importante |

#### 6.7.7 Andamiaje

El andamiaje tiene que retirarse a medida que se gana dominio (Wood et al.,
1976), y el mismo apoyo que ayuda a quien empieza estorba a quien ya sabe
(Kalyuga et al., 2003). Por eso se organiza por tradición declarada y por
niveles que la persona baja cuando quiere.

| Requisito | Versión | Prioridad |
|---|---|---|
| Proyecto de ejemplo completo en español con entrevistas ficticias, hoja de mapa, tabla, libro de códigos, análisis y documento, que recorre el ciclo de punta a punta (Atkinson et al., 2000) | v1.0 | imprescindible |
| Rutas guiadas opcionales, por ejemplo análisis temático reflexivo y análisis de contenido cualitativo, cada paso con su referencia y ninguna marcada como la correcta | v1.0 | importante |
| Niveles de apoyo (guiado, estándar, experto) que se bajan por módulo, con la explicación y la referencia de cada paso recomendado | v1.0 | importante |
| Navegación por tarea analítica antes que por función, por ejemplo «comparar lo que dicen dos grupos» antes que «consulta de co-ocurrencia» (Woolf y Silver, 2018) | v1.0 | importante |
| Plantillas comentadas en español con su fuente citada (libro de códigos, guía de entrevista, análisis descriptivo), editables o descartables | v1.0 | importante |
| Glosario metodológico en español como ayuda contextual, con cada definición citada | v1.0 | importante |

### 6.8 Escritura y divulgación

En la v0.1 los `.qmd` los edita el autor en su propio editor y los renderiza su
Quarto. La app aporta lo que Quarto solo no ofrece.

| Requisito | Versión | Prioridad |
|---|---|---|
| Extensión de Quarto incluida en la app que inserta un resultado fijado por su identificador en cualquier `.qmd`, escrito dentro o fuera de la app, y muestra un aviso visible en el render si está desactualizado | v0.1 | imprescindible |
| Citar un fragmento en el `.qmd` con una referencia estable (seudónimo y minuto o párrafo) que sigue apuntando al segmento original y avisa si el segmento cambió. Si el segmento no está revisado, el render muestra un aviso en lugar del texto salvo una excepción confirmada y registrada, y si el consentimiento no autoriza cita textual, muestra el aviso sin excepción | v0.1 | imprescindible |
| Subcomando `osr estado` que lista los resultados desactualizados y las citas con avisos | v0.1 | importante |
| Editor de documentos Quarto dentro de la app con vista previa y render a PDF por Typst con una versión de Quarto fijada por la app | v1.0 | importante |
| Salidas legibles para tutoras sin instalar nada (PDF, documento con las transcripciones, informe del libro de códigos con ejemplos) | v1.0 | imprescindible |
| Las tablas fijadas se generan como tablas reales con encabezados, y toda figura exige texto alternativo antes de publicarse | v1.0 | importante |
| Declaración de uso de IA para la sección de métodos, generada desde el registro | v1.0 | deseable |
| Sitio web de divulgación generado desde la app, solo con capas que pasaron el chequeo de privacidad. El autor ya decidió un sitio web público como devolución de su investigación, así que entra cuando la investigación llegue a ese punto, según la regla de alcance | después | importante |
| Mapas interactivos en el sitio, solo con capas agregadas | después | deseable |

### 6.9 Trabajo en equipo

La colaboración llega después de la v1.0. Desde la v0.1 el formato de texto de
la codificación y los memos está documentado para migrarlo a Automerge cuando
llegue.

Abrir el proyecto de otra persona es ejecutar código ajeno, porque R ejecuta el
`.Rprofile` del directorio de trabajo al arrancar y Quarto ejecuta los filtros
Lua del proyecto. Por eso un proyecto recibido se trata como no confiable, como
hace Jupyter con los notebooks.

| Requisito | Versión | Prioridad |
|---|---|---|
| Empaquetar un proyecto en un solo archivo para entregarlo a una tutora o colega, eligiendo si incluye o no lo marcado como sensible | v1.0 | importante |
| Un proyecto empaquetado o recibido abre como no confiable, sin ejecutar ningún notebook ni script, sin arrancar R con el `.Rprofile`, el `.Renviron` o el `renv/activate.R` del proyecto y sin renderizar con filtros Lua o extensiones de Quarto del proyecto, hasta que la persona lo marque como confiable | v1.0 | imprescindible |
| Sincronización con Automerge sobre iroh, edición simultánea de documentos, bitácora, notebooks y codificación, y copia directa de audios y bases solo por acción explícita | después | importante |
| Codificación por persona, vista lado a lado de dos codificaciones del mismo fragmento y cálculo opcional de acuerdo (porcentaje, kappa, alfa de Krippendorff) por documento o por unidad predefinida, con una plantilla de R editable (Krippendorff, 2018) | después | importante |
| Permisos por proyecto sin servidor central, por ejemplo quién ve audios con nombres y quién solo transcripciones seudonimizadas, con revocación y sus límites declarados | después | importante |
| Una base con restricciones de redistribución no se sincroniza sin confirmación explícita, y en su lugar se comparten la cita y el script de descarga | después | importante |

## 7. Requisitos no funcionales

Las cifras son metas que se miden antes de cada versión, las de la v0.1 en el
equipo Linux del autor y las de la v1.0 en el hardware de referencia. La v0.1
solo tiene metas que se notan al usar la app, las demás empiezan en la v1.0.
Donde dicen «a medir», el número es una hipótesis inicial que se corrige con
mediciones reales y se publica.

Cada meta se mide con un script sobre los datos de prueba del repositorio, o
con un subcomando `osr medir` cuando exista, con al menos cinco repeticiones,
la memoria residente tomada del sistema operativo y el resultado publicado en
`docs/mediciones.md` con la versión, la máquina y la fecha. En Windows se mide
con Microsoft Defender activo, porque encarece las escrituras atómicas y el
cálculo de huellas.

### Hardware y plataformas

| Aspecto | Meta | Desde |
|---|---|---|
| Plataforma de v0.1 | Linux x86-64, el equipo del autor con Arch Linux, sin compromiso de soporte en otras | v0.1 |
| Navegadores de v0.1 | Firefox y Chromium estables en el equipo del autor | v0.1 |
| Hardware de referencia | Laptop x86-64 de alrededor de 2018, 4 núcleos y 8 hilos (clase Core i5-8250U o Ryzen 5 3500U), 8 GB de RAM, SSD de 256 GB con al menos 20 GB libres, gráficos integrados, sin GPU dedicada, pantalla de 1366 × 768 | v1.0 |
| Piso de RAM | 8 GB | v1.0 |
| Plataformas de v1.0 | Windows 11 de 64 bits como plataforma principal, Windows 10 22H2 como excepción declarada mientras R, Quarto y los navegadores lo soporten, porque buena parte de las novatas lo usa aunque su soporte general terminó el 14 de octubre de 2025, revisada en cada versión 1.x, y Linux x86-64 con glibc 2.35 o superior (Ubuntu 22.04 y 24.04 de referencia) | v1.0 |
| Navegadores de v1.0 | Dos últimas versiones estables de Edge, Chrome y Firefox, más Firefox ESR | v1.0 |
| Instalación en Windows | Sin permisos de administrador, probada con usuario con tildes, ñ y espacios en la ruta y con Documentos redirigido a OneDrive | v1.0 |
| Modo de 4 GB | Degradación anunciada, un solo motor de análisis a la vez, modelo de Whisper base y sin transcribir mientras corre un análisis, a medir | después |
| macOS | Apple Silicon, cuando se resuelva el costo de la notarización | después |

### Memoria y arranque

La visión estima del orden de 10 MB en reposo para el servidor solo, y la meta
sin proyecto abierto la sigue. El consumo real lo ponen el proyecto abierto, el
navegador, los motores de análisis y el modelo de Whisper, y por eso hay
presupuestos para el conjunto.

| Aspecto | Meta | Desde |
|---|---|---|
| Orquestador en reposo sin proyecto abierto | 15 MB de memoria residente o menos | v0.1 |
| Orquestador con proyecto abierto y sin motores | 50 MB de memoria residente o menos | v0.1 |
| Arranque en frío hasta interfaz usable, sin motores, con SSD | 3 s o menos | v0.1 |
| Arranque en frío con disco mecánico | 8 s o menos | v1.0 |
| Pestaña con una transcripción de 1 h abierta (unas 10.000 palabras) | 250 MB o menos | v1.0 |
| App completa sin motores ni transcripción, con el proyecto de referencia abierto | 500 MB o menos, incluida la pestaña | v1.0 |
| Abrir un proyecto con 30 entrevistas transcritas | 2 s o menos | v1.0 |
| Orquestador más un kernel de R con tidyverse y sf cargados | 800 MB o menos sin contar el navegador | v1.0 |
| Uso típico (orquestador, pestaña y un kernel de R) | 1,5 GB o menos, a medir | v1.0 |
| Con transcripción activa con modelo small | 2,5 GB o menos, a medir | v1.0 |
| Transcripción en equipos de 8 GB | No más del 50 % de la RAM total y nunca a la vez que un motor de análisis | v1.0 |
| Memoria de whisper.cpp de referencia | Del orden de 0,4 GB con base, 0,9 GB con small, 2,1 GB con medium y 3,9 GB con large (Gerganov, s. f.), a medir | v1.0 |
| Modelo por defecto según RAM | small con 8 GB, medium o turbo con 16 GB o más, y base cuando exista el modo de 4 GB | v1.0 |
| Kernel de R listo para ejecutar | 5 s o menos en caliente y 10 s o menos en frío | v1.0 |
| Kernel de Python listo para ejecutar | 3 s o menos | v1.0 |
| Primer gráfico en un notebook nuevo, paquetes ya instalados | 10 s o menos | v1.0 |
| Motores inactivos | Se apagan tras 30 minutos por defecto, configurable, y una cola serializa las tareas pesadas | v1.0 |

### Respuesta y escala

El proyecto de referencia tiene 40 entrevistas de 60 minutos (unas 40 h de
audio, del orden de 2 a 3 GB en m4a), unas 10.000 palabras por transcripción,
5.000 segmentos codificados y 200 códigos. La escala máxima que se prueba es
la de un posgrado grande, 50 entrevistas de 90 minutos y 300 códigos.

| Aspecto | Meta | Desde |
|---|---|---|
| Aplicar o quitar un código | 100 ms o menos (Nielsen, 1993) | v0.1 |
| Saltar a una marca de tiempo del audio | 200 ms o menos | v0.1 |
| Abrir una transcripción de 60 minutos | 1 s o menos | v0.1 |
| Recuperar los segmentos de un código | 500 ms o menos en el proyecto del autor | v0.1 |
| Desplazamiento en transcripciones largas | 60 cuadros por segundo dibujando solo lo visible, también en transcripciones de 2 h | v1.0 |
| Recuperar los segmentos de un código a escala | 500 ms o menos en el proyecto de referencia, 1 s o menos en la escala máxima | v1.0 |
| Cualquier consulta cualitativa | 1 s o menos en el proyecto de referencia | v1.0 |
| Operaciones largas | Progreso para todo lo que pase de 1 s, y estimación y cancelación para lo que pase de 10 s | v1.0 |
| Interfaz mientras transcribe | Responde en 200 ms o menos | v1.0 |
| Aplicar una tabla de 200 reemplazos a una transcripción de 1 h | 1 s o menos | v1.0 |
| Listar todos los derivados de un participante | 5 s o menos en un proyecto de 50 entrevistas | v1.0 |
| Visor de mapas | GeoPackage de 50 MB con 50.000 entidades en 3 s o menos, 30 cuadros por segundo o más, no más de 300 MB extra en la pestaña | v1.0 |

### Durabilidad y reproducibilidad

| Aspecto | Meta | Desde |
|---|---|---|
| Persistencia de cada acción de codificación o memo | 1 s o menos, con pérdida máxima de 2 s ante un corte de luz | v0.1 |
| Escrituras | Atómicas (archivo temporal, fsync y renombrado), verificadas con pruebas que matan el proceso durante escrituras | v0.1 |
| Anclas al reabrir una transcripción codificada | Cero segmentos corridos en silencio, cada segmento afectado por una edición se lista para revisar | v0.1 |
| Recalcular sin cambios | Las cifras y tablas tienen la misma huella SHA-256 en la máquina del autor, y las semillas se anotan siempre | v0.1 |
| Reconstrucción del índice SQLite | 30 s o menos para 50 entrevistas y 5 GB de datos | v1.0 |
| Comprobación de desactualización | 1 s o menos para 200 resultados sobre 2 GB de datos, filtrando por tamaño y fecha y calculando la huella solo de lo que cambió | v1.0 |
| Cálculo de SHA-256 | Incremental, a 300 MB/s o más, sin bloquear la interfaz | v1.0 |
| Determinismo entre plataformas | Igualdad exacta en la misma plataforma, y entre plataformas una tolerancia por tipo de resultado anotada en el manifiesto, por ejemplo 1e-8 relativa para estimaciones lineales y más laxa para modelos iterativos, por diferencias de BLAS, LAPACK y punto flotante | v1.0 |
| Reconstruir el entorno y regenerar todos los resultados | 30 minutos o menos desde la carpeta y su archivo de bloqueo | v1.0 |
| Exportaciones | El mismo proyecto sin cambios produce el mismo paquete byte a byte, salvo la fecha de generación | v1.0 |
| Anclas en Automerge | Cero segmentos corridos o perdidos tras 1.000 ediciones aleatorias sobre un documento con 500 segmentos, antes de migrar la codificación | después |

### Transcripción

| Aspecto | Meta | Desde |
|---|---|---|
| Velocidad | 1 h de audio en 1,5 h o menos con el modelo small en el hardware de referencia, a medir con audios reales | v1.0 |
| Estimación de tiempo | Error inicial menor al 25 % | v1.0 |
| Recursos | Prioridad baja, núcleos menos uno por defecto, pausa y reanudación desde el último segmento terminado | v1.0 |
| Calidad | Tasa de error por palabra medida en la muestra de evaluación de la sección 6.6 antes de recomendar un modelo por defecto | v1.0 |

### Descarga e instalación

| Aspecto | Meta | Desde |
|---|---|---|
| Binario descargable | 50 MB o menos comprimido, con meta de 40 MB | v1.0 |
| Entorno base sin modelo de Whisper (R con paquetes base, Python y Quarto) | 1,5 GB o menos, medido y publicado en cada versión | v1.0 |
| Modelos de Whisper en disco | Del orden de 140 MB base, 470 MB small, 1,5 GB medium y 2,9 GB large, anunciados antes de descargar [verificar] | v1.0 |
| Instalación completa típica | 3 GB o menos | v1.0 |
| Entorno base de R desde binarios | 10 minutos o menos con 10 Mbps, sin compilar nada en Windows | v1.0 |
| Descargas | Reanudables, verificadas con SHA-256, usables con conexiones de 2 Mbps | v1.0 |

### Red y seguridad

| Aspecto | Meta | Desde |
|---|---|---|
| Conexiones salientes en v0.1 | Ninguna, porque no hay código de red saliente, verificado con la prueba en un espacio de nombres de red sin salida | v0.1 |
| Sin conexión | El 100 % de las funciones de v0.1 y v1.0 trabaja sin internet después de la instalación, salvo las integraciones que la persona active | v0.1 |
| Registros técnicos | Sin texto de transcripciones, sin nombres de archivos de la zona identificable y sin contenido enviado a IA, rotación con máximo de 10 MB | v0.1 |
| Conexiones salientes en v1.0 | Cero desde el arranque hasta la primera acción explícita, verificado con una prueba automática que corre la app sin red y falla si intenta conectarse, y con una captura de paquetes de 10 minutos | v1.0 |
| Modelos descargados | Hash verificado contra una lista publicada antes de usarlos | v1.0 |

### Idioma y formatos

| Aspecto | Meta | Desde |
|---|---|---|
| Idioma | Interfaz y mensajes 100 % en español | v0.1 |
| Formatos internos | UTF-8 sin BOM, CSV según RFC 4180, fechas ISO 8601 | v0.1 |
| Búsqueda | Insensible a tildes, mayúsculas y diéresis, con la ñ tratada como letra propia | v0.1 |
| Geodatos en v0.1 | Cada capa en su sistema de referencia de origen, sin reproyectar en silencio | v0.1 |
| Lenguaje claro | Mensajes, ayuda y notas de versión en lenguaje claro, errores que dicen qué hacer y nunca solo una traza técnica | v1.0 |
| Advertencias | Las de privacidad tienen como máximo dos oraciones antes de su acción, las metodológicas llevan su referencia | v1.0 |
| CSV para Excel | Exportación opcional en UTF-8 con BOM | v1.0 |
| Geodatos en v1.0 | Distancias y áreas en CRTM05 (EPSG:5367), visualización en EPSG:3857, exportación en EPSG:4326, coordenadas guardadas con al menos 7 decimales y redondeo solo al publicar | v1.0 |

### Accesibilidad

| Aspecto | Meta | Desde |
|---|---|---|
| Teclado y semántica | HTML semántico, y revisión y codificación completas solo con teclado | v0.1 |
| Color | Los códigos y las marcas de IA o de sospecha nunca se distinguen solo por el color, siempre llevan nombre, ícono o etiqueta | v0.1 |
| Conformidad | WCAG 2.2 nivel AA en los flujos centrales (importar, transcribir, revisar, codificar, exportar) (W3C, 2023) | v1.0 |
| Detalle | Contraste de 4,5 a 1 en texto y 3 a 1 en componentes, foco siempre visible, objetivos de 24 × 24 px o más, texto ampliable al 200 %, respeto al movimiento reducido, uso sin desplazamiento horizontal en 1280 × 720 | v1.0 |
| Lectores de pantalla | Probado con NVDA en Windows y Orca en Linux | v1.0 |
| Paletas | Aptas para daltonismo por defecto en las plantillas de gráficos y mapas | v1.0 |

## 8. Integraciones

| Integración | v0.1 | v1.0 | Después |
|---|---|---|---|
| Zotero | `.bib` o CSL-JSON exportado por la persona, y la cita sugerida de cada ficha se carga a Zotero | API local para buscar e insertar citas, con respaldo al `.bib`, y bibliografía de datos generada desde las fichas | Agregar ítems desde la app |
| Quarto y Typst | Extensión para resultados fijados y citas de fragmentos, render con el Quarto de la persona | Quarto instalado y fijado por la app, editor y render dentro de la app | Sitio de divulgación desde la app, cuando la investigación del autor llegue a su devolución |
| R | R del sistema con la biblioteca que la persona ya tiene, paquete de R del proyecto y scripts lanzados con `osr recalcular` | Notebooks con IRkernel, instalación gestionada con rig y rv | Cómputo externo |
| Python | Ninguna | uv y ipykernel | Cómputo externo |
| whisper.cpp | Importación de transcripciones hechas fuera de la app | Transcripción integrada, recomendación de modelo, glosario, métricas de corrección | Separación de hablantes |
| QGIS | Importar capas y marcas digitalizadas | Abrir una capa del proyecto en QGIS y registrar láminas hechas en QGIS o PyQGIS | Lo que pida la investigación |
| QualCoder | Lectura en R de las tablas de segmentos exportadas en CSV, solo si se activa el plan B | Validación de la exportación `.qdpx` | Importación de su proyecto por `.qdpx` |
| REFI-QDA | Ninguna | Exportar e importar libro de códigos `.qdc` y exportar proyecto `.qdpx` (REFI-QDA, 2019), validado contra QualCoder y al menos una herramienta comercial | Importar proyectos `.qdpx` completos |
| Archivo y depósito | Ninguna | Paquete RO-Crate de la zona compartible (Soiland-Reyes et al., 2022), con verificación previa de identificadores | Depósito directo en Zenodo o Dataverse con llave propia, metadatos DDI-Codebook 2.5 (DDI Alliance, 2014) |
| Repositorios de datos | Descarga manual con ficha de procedencia | Ninguna | Conector NADA de solo lectura con DDI-Codebook para el INEC, luego CKAN y Dataverse |
| IA | Ninguna | Asistente de código en notebooks con compuerta de salida, si el autor o una colega piloto lo usan en una investigación real | Codificación asistida, contraste crítico, resúmenes, clasificación |
| Cómputo externo | Ninguna | Ninguna | Clúster por SSH o nube con llave propia, con resultados que traen su manifiesto |

Los conectores entran de a uno por versión y solo por protocolo, empezando por
NADA, que es el de más valor para la investigación social costarricense. El de
NADA guarda junto a cada base sus términos de uso, en especial la prohibición
de reidentificar que traen los microdatos del INEC, y deja la aceptación de
términos y la descarga restringida en manos de la persona.

### Dependencias externas

Cada dependencia queda detrás de un módulo propio y con su versión exacta
fijada. Las licencias de todo lo que la app incluye o descarga van al archivo
de avisos de terceros (sección 11).

| Dependencia | Desde | Licencia | Si se abandona o se rompe |
|---|---|---|---|
| Axum | v0.1 | MIT | El servidor es pequeño y se puede portar a otro marco de Rust |
| Svelte | v0.1 | MIT | Compila a JavaScript que sigue funcionando sin el proyecto |
| Quarto, con Typst | v0.1 el de la persona, v1.0 fijado por la app | MIT [verificar] y Apache-2.0 | Versión fijada y render de un proyecto de ejemplo en integración continua |
| Decodificador de audio | v0.1 para duración y metadatos, v1.0 para decodificar a PCM de 16 kHz para whisper.cpp | Symphonia (MPL-2.0) o ffmpeg (LGPL o GPL según la compilación) | Symphonia cubre AAC y MP3, su soporte de Opus es inmaduro [verificar]. Antes de la v1.0 se decide cuál, su licencia y si va dentro del binario. En la v0.1 el audio lo reproduce el navegador |
| Protocolo Jupyter con IRkernel e ipykernel | v1.0 | MIT y BSD-3 | Protocolo estable y documentado, con otras implementaciones |
| rig | v1.0 | MIT [verificar] | Instalación de R desde los instaladores oficiales |
| rv | v1.0 | [verificar] | Proyecto joven, la salida es renv con un archivo de bloqueo equivalente |
| uv | v1.0 | MIT o Apache-2.0 | La salida es pip con un archivo de requisitos con hashes |
| whisper.cpp y sus modelos | v1.0 | MIT | Modelos verificados por hash y conservación del binario de la versión probada |
| API local de Zotero | v1.0 | Zotero es AGPL y se usa por HTTP sin incluir su código | Respaldo al `.bib` exportado |
| MapLibre GL JS | v1.0 | BSD-3 | Respaldo a imagen estática y tabla |
| GDAL, PROJ y GEOS | v1.0, por R y Python | MIT, MIT y LGPL | Vienen dentro de los binarios de sf y de las ruedas de Python, la app no las enlaza |
| Automerge | después | MIT | El formato de texto canónico sigue siendo legible sin Automerge |
| iroh | después | MIT o Apache-2.0 | Proyecto joven, la colaboración queda detrás de un módulo que admite otro transporte |

### Reglas para la IA

El producto tiene que estar completo y ser bueno sin IA. En una máquina de
8 GB sin GPU los modelos locales útiles para programar son lentos o flojos, y
la nube cuesta, así que la IA es un acelerador opcional y el camino manual se
diseña primero. Este bloque entra a la v1.0 solo si el autor o una colega
piloto usan el asistente en una investigación real.

| Requisito | Versión | Prioridad |
|---|---|---|
| El asistente escribe celdas de código marcadas como propuesta de IA, con el modelo exacto y la instrucción que las originó, hasta que la persona las ejecuta y las acepta, edita o rechaza, y cada estado queda registrado con quién y cuándo (Amershi et al., 2019) | v1.0 | imprescindible |
| Solo la salida de celdas ejecutadas se puede fijar, y el asistente nunca escribe cifras en la prosa, todo número que proponga en texto es una referencia a un resultado fijado (Ji et al., 2023) | v1.0 | imprescindible |
| El asistente explica en lenguaje llano qué hace cada análisis y sus supuestos, y advierte cuando se aplica una prueba inferencial a una muestra no probabilística | v1.0 | imprescindible |
| Política de salida por proyecto con tres niveles (nada sale, solo código y esquema de variables, texto seudonimizado), con «nada sale» por defecto y vista previa exacta de cada envío y su destino | v1.0 | imprescindible |
| Ficha de cada modelo conectado, local o en la nube, con nombre y versión exacta, dónde corre, política de retención del proveedor y limitaciones conocidas en español, y el identificador exacto guardado en cada resultado (Mitchell et al., 2019; Chen et al., 2024) | v1.0 | importante |
| Las llaves de acceso se guardan en el llavero del sistema operativo, nunca en la carpeta del proyecto | v1.0 | imprescindible |
| En codificación la IA llega solo después del juicio humano, como contraste que señala fragmentos sin codificar o que contradicen una categoría, para no anclar la interpretación de quien aprende | después | importante |
| Sugerencias de códigos usando el libro de códigos como criterio, tratadas como un codificador más, con medición de acuerdo humano e IA sobre una muestra antes de usarlas a escala (Ziems et al., 2024; Gilardi et al., 2023) | después | importante |
| Resúmenes de entrevistas por IA, marcados y separados del informe | después | deseable |

## 9. Ética y datos de las personas participantes

Esta investigación mezcla voces grabadas, relatos de victimización, menciones
de terceros señalados como responsables de delitos y puntos en un mapa que
pueden revelar la vivienda, el trabajo o la rutina de quien habla, en una zona
pequeña donde barrio, oficio y horario bastan para reidentificar a alguien.
La Ley 8968 trata algunos de esos datos como sensibles y otros como de acceso
restringido, según el caso, y exige consentimiento informado para
recolectarlos y transferirlos (Asamblea Legislativa de la República de Costa
Rica, 2011, arts. 3, 5, 9 y 14 [verificar]; Poder Ejecutivo de la República de
Costa Rica, 2013). Como las primeras entrevistas se graban antes de que exista
el software, el registro de consentimiento y la separación del material
identificable nacen en la v0.1 y no se agregan después.

El modelo es de dos zonas dentro de la carpeta. La zona identificable guarda
audios, escaneos de consentimientos y de hojas de mapa, la tabla que liga cada
seudónimo con su persona y las transcripciones crudas. La zona de trabajo
guarda el material seudonimizado, que es lo único que se codifica, se comparte
o se publica. Separar la llave de enlace de los datos es la práctica básica de
confidencialidad (Wiles et al., 2008; Corti et al., 2014).

Tres hechos técnicos obligan a requisitos concretos. Un servidor local sin
token ni control de origen es alcanzable desde cualquier página que la persona
visite, por reenlace de DNS o CSRF (Jackson et al., 2007), y además ejecuta
código. Automerge, cuando llegue la colaboración, guarda el historial completo
de ediciones, así que un nombre que entra a un documento colaborativo no se
suprime de verdad aunque se reemplace después. Una copia enviada por iroh a
otra computadora no se puede recuperar. Por eso la supresión y el retiro del
consentimiento solo se pueden cumplir si lo identificable nunca entra a
Automerge y no viaja por defecto, solo por una acción explícita por archivo que
queda registrada.

La anonimización es un juicio contextual y la reidentificación con cuasi
identificadores es frecuente (Sweeney, 2002), así que la herramienta asiste,
registra y nunca rotula nada como anonimizado. Reemplazos coherentes preservan
el sentido del relato mejor que tachar (Saunders et al., 2015), y los puntos
precisos reidentifican aunque el texto esté limpio (de Montjoye et al., 2013;
Kounadi y Leitner, 2014).

### Amenazas que se consideran

| Amenaza | Qué la cubre | Desde |
|---|---|---|
| Laptop robada o perdida | Proyecto en un disco cifrado según el README, y comprobación del cifrado de disco | v0.1 en el README, v1.0 en la app |
| Computadora compartida con otra cuenta del sistema | Token por arranque, servidor solo en loopback, archivos de conexión con permisos 0600 | v0.1 y v1.0 |
| Página web maliciosa que alcanza el servidor local | Token, validación de Host y Origin, sin CORS, con pruebas automáticas | v0.1 |
| Proyecto recibido de otra persona que ejecuta código al abrirse | Proyecto no confiable hasta que la persona lo marque (sección 6.9) | v1.0 |
| Disco de respaldo perdido con audios y la tabla de correspondencia | Zona identificable solo a disco cifrado, y respaldo que la excluye o la cifra si el destino no está cifrado | v0.1 en el README, v1.0 en la app |
| Paquete comprometido de CRAN o PyPI | Versiones fijadas en el archivo de bloqueo, solo binarios de repositorios conocidos y descargas verificadas por hash | v1.0 |

### Requisitos

| Requisito | Versión | Prioridad |
|---|---|---|
| Registro de participantes con un código seudónimo por persona (por ejemplo E01), usado en nombres de archivo, transcripciones, códigos y puntos del mapa | v0.1 | imprescindible |
| Registro de consentimiento por persona con versión del formulario, fecha, modalidad (escrito u oral grabado) y alcances otorgados por separado (grabar, citar textualmente, usar sus puntos del mapa, archivar, reutilizar y los que el proyecto defina), en archivo plano | v0.1 | imprescindible |
| Zona identificable y zona de trabajo separadas en la carpeta, con la zona de cada archivo visible en la interfaz sin depender solo del color | v0.1 | imprescindible |
| La tabla que liga seudónimos con personas vive en un único archivo de la zona identificable y en ningún otro lugar, ni en el índice SQLite ni en ningún otro documento | v0.1 | imprescindible |
| Marca de sensibilidad por archivo, por capa y por fuente, visible en la interfaz y en el README, y la v0.1 no ofrece ninguna vía de salida | v0.1 | imprescindible |
| Seudonimización con una tabla de reemplazos en CSV por proyecto (nombres, lugares, oficios, instituciones, terceros y cuasi identificadores) y el subcomando `osr seudonimizar`, que genera la versión de trabajo conservando los identificadores de segmento y sus marcas de tiempo, deja la versión cruda en la zona identificable y produce un diff para revisar a mano | v0.1 | imprescindible |
| Confirmación de cada reemplazo uno por uno en la interfaz y estado de anonimización visible por documento | v1.0 | importante |
| Lo sensible no sale de la máquina sin una confirmación explícita que queda registrada | v1.0 | imprescindible |
| La transcripción corre solo en la máquina, sin ningún respaldo automático a un servicio en la nube aunque el equipo sea lento | v1.0 | imprescindible |
| Compuerta de IA en la nube que impide enviar contenido de la zona identificable, muestra antes de cada envío qué se manda y a qué proveedor, y permite fijar por proyecto solo modelos locales, entregada en la misma versión que cualquier IA en la nube | v1.0 | imprescindible |
| Compartir o empaquetar algo identificable exige una acción explícita por archivo con advertencia, que queda registrada | v1.0 | imprescindible |
| Revisión previa a publicar el PDF o el sitio, que verifica que todo salga de la zona de trabajo, que no se incrusten audios, que cada cita tenga consentimiento de cita, que no aparezcan los nombres de la tabla de correspondencia, y lista las citas para revisión humana | v1.0 | imprescindible |
| Chequeo de geoprivacidad que bloquea capas sensibles publicadas sin agregar a zonas con al menos 5 marcas, umbral provisional tomado de las reglas de frecuencia mínima del control de divulgación estadística (Hundepool et al., 2012), o sin enmascaramiento documentado (Armstrong et al., 1999), y advierte celdas con conteos bajos de datos policiales o de participantes | v1.0 | imprescindible |
| Bitácora de salidas que registra cada exportación, envío a IA, envío a cómputo externo y copia a una colega, con fecha, destino y archivos | v1.0 | importante |
| Retiro del consentimiento, que dado un seudónimo lista todo lo derivado (audio, transcripciones, segmentos, resultados, puntos, citas), permite retirarlo y avisa qué copias externas y qué respaldos conocidos hay que limpiar | v1.0 | importante |
| Quitar metadatos EXIF y coordenadas GPS de fotos y escaneos al pasarlos a la zona de trabajo y siempre al publicar | v1.0 | importante |
| Aviso cuando la carpeta está dentro de OneDrive, Google Drive, Dropbox o iCloud, con la recomendación de moverla | v1.0 | importante |
| Comprobación del cifrado de disco (BitLocker o cifrado de dispositivo, FileVault, LUKS) con aviso y guía corta si falta | v1.0 | importante |
| Sugerencias de identificadores por reglas (seudónimos ya usados, teléfonos, cédulas, correos, direcciones) que la persona confirma | v1.0 | importante |
| Plantillas de consentimiento y hoja informativa en español claro, con criterios citados (CIOMS, 2016), que mencionan la transcripción local y, si se usa, el envío de texto seudonimizado a la nube, con la advertencia de que deben revisarlas la tutora o el comité | v1.0 | importante |
| Licencia para datos y resultados compartibles separada de la MIT del código, con CC BY 4.0 como sugerencia y sin licencia abierta para la zona identificable | v1.0 | importante |
| Plantilla de plan de gestión de datos y resumen generado desde el proyecto, útil como insumo para la tutora o el comité, sin sustituir su juicio | v1.0 | deseable |
| Lo identificable nunca entra a Automerge ni se replica por sincronización | después | imprescindible |
| Reconocimiento local de nombres y lugares como sugerencia adicional | después | deseable |
| Bóveda protegida con contraseña para la versión sin anonimizar, tanto de las entrevistas como de las bases de datos, que se abre solo para preparar la versión de trabajo y se vuelve a cerrar | después | importante |
| Flujo de preparación antes del análisis, en el que la persona anonimiza sus bases de datos (quita identificadores directos y generaliza cuasi identificadores) y seudonimiza o censura partes de sus entrevistas, y desde entonces todo el análisis trabaja solo sobre esa versión censurada, con registro de qué se cambió | después | importante |
| Lista de verificación guiada con los lineamientos de la Agencia de Protección de Datos de los Habitantes (PRODHAB) para las bases de datos del proyecto, que ayuda a cumplirlos y registra las respuestas, sin certificar el cumplimiento (sección 4) | después | importante |
| Plan de retención por tipo de archivo con fecha de destrucción y recordatorio, que en la v0.1 se anota en la bitácora | después | deseable |

## 10. Criterios de éxito sin dinero

El éxito no se mide en descargas ni en usuarios activos, sino en investigación
hecha mejor. Todo se levanta sin telemetría, con pruebas automáticas, la
bitácora del propio autor, pruebas con personas y lo que las colegas cuenten.

**Con la v0.1**

Los criterios de la sección 5 son los que dan por terminada la v0.1. Además,
sin bloquear la versión, el autor anota en la bitácora estas observaciones,
que alimentan la v1.0.

- Cada vez que tuvo que salir a otra herramienta para revisar, seudonimizar o
  codificar, anotada como issue.
- El tiempo de revisión por minuto de audio, que la app registra por
  transcripción, como dato y no como meta.
- Si las cifras y tablas cuantitativas de las entregas que usan la herramienta
  salen de resultados fijados, sin transcribir ninguna a mano. Las láminas de
  QGIS quedan fuera de esta cuenta en la v0.1, pero sus capas de insumo entran
  al proyecto con su ficha.
- Si el autor, frente a la docente y en la app, rastrea una cifra o una cita
  hasta su código, sus datos y sus versiones, o hasta su segmento, su seudónimo
  y su minuto del audio, en dos clics o menos y en menos de 2 minutos.
- Si las hojas de mapa del período quedan en el proyecto, cada marca ligada a
  su entrevista, a su hoja escaneada y, cuando se mencionó, a su fragmento,
  sin enlaces rotos.
- Si el 100 % de los datos externos usados tiene ficha de procedencia y
  aparece citado en el documento.
- Si solo con la bitácora el autor reconstruye cómo y por qué cambió el libro
  de códigos entre la primera y la última entrevista.
- Si las entregas contienen cero identificadores directos, verificado con una
  búsqueda automática, en la máquina del autor, de cada nombre de la tabla de
  correspondencia en el PDF final, que debe dar cero coincidencias, y con la
  lectura de una compañera o compañero del ISA que busca identificadores
  indirectos sin ver la tabla.

**Con la v1.0**

- Los criterios de terminación de la sección 5.
- Una persona usuaria de NVDA completa importar, revisar y codificar solo con
  teclado, sin fallas de nivel A ni AA.
- Ninguna colega piloto informa que su máquina se trabó al transcribir o
  analizar.
- En los proyectos de colegas novatas, al menos el 80 % de los códigos tiene
  definición y criterios.
- Las tutoras leen las entregas producidas con la herramienta sin instalar
  nada.
- Si el módulo cuantitativo entró a la v1.0, las colegas que analizan
  microdatos lo hacen con diseño declarado y una revisión externa no encuentra
  estimaciones sin ponderar donde correspondía ponderar.
- Ninguna propuesta de IA aparece en un resultado sin que conste quién la
  aceptó.

**Un año después de la v1.0**

- Al menos cinco personas además del autor usan la herramienta en una
  investigación real, y al menos tres siguen usándola tres meses después por
  decisión propia.
- Al menos la mitad de las primeras colegas la sigue usando al cerrar un
  semestre completo.
- Al menos una colega deposita en un repositorio un paquete generado por la
  herramienta y recibe un identificador persistente.
- Al menos una contribución externa aceptada, de código, documentación o
  traducción.
- Una segunda persona compila y publica una versión siguiendo solo la
  documentación.
- El autor sigue manteniendo el proyecto con un ritmo sostenible, al menos una
  versión cada tres meses y sin semanas de soporte que desplacen su
  investigación.
- Cero incidentes de privacidad reportados por participantes o colegas.

## 11. Sostenibilidad del proyecto libre

Los proyectos libres mueren sobre todo por falta de tiempo de quien los
mantiene, por quedar obsoletos o desplazados por otro proyecto y por alcance
desmedido (Coelho y Valente, 2017; Eghbal, 2020). La mejor defensa que tiene
este proyecto ya está en la visión, que una función entra solo cuando la
investigación real la pide, y aquí se convierte en reglas verificables.

**Desde la v0.1**

- Regla de alcance escrita en el repositorio. Una función entra a una versión
  solo si la investigación del autor o de una colega piloto la necesita en las
  próximas semanas, y toda propuesta nueva pasa por esa regla y por la lista de
  no-objetivos. Las funciones de la v1.0 de este documento son candidatas, no
  compromisos.
- Cuando la herramienta y la investigación chocan, gana la investigación, y el
  camino alternativo con la cadena actual se mantiene vivo.
- Modelo de mantenedor único declarado. El registro de decisiones en
  `docs/decisiones.md`, con fecha, contexto, alternativas y razón, es la
  gobernanza mientras no haya comunidad (Fogel, 2005).
- El README dice que el proyecto está en desarrollo temprano, que lo usa una
  persona y que todavía no acepta contribuciones de código, aunque sí reportes.

**Para la v1.0**

- Versionado semántico y un CHANGELOG en español en cada versión.
- Una página que explica cómo usa el autor la herramienta en su investigación,
  semilla de la guía para colegas.
- Versiones etiquetadas en GitHub Releases con sumas SHA-256 y notas en
  español, compilación reproducible desde un clon limpio con un comando, en
  30 minutos o menos en la integración continua gratuita.
- Compilación de Windows en la integración continua desde antes de la v1.0, y
  prueba de instalación limpia en una máquina virtual Windows antes de cada
  versión 1.x, porque el autor no usa Windows a diario.
- Firma de código para Windows por un programa gratuito para software libre,
  como el de SignPath [verificar condiciones], o en su defecto una guía con
  capturas para pasar la advertencia de SmartScreen.
- `GOVERNANCE.md` breve que dice quién decide, cómo se proponen cambios y qué
  pasa si el autor deja de mantener.
- `CONTRIBUTING.md` en español con cómo compilar y probar, aceptación por DCO y
  no por CLA, plantillas de issues y el Contributor Covenant en español, porque
  no saber por dónde empezar y no poder levantar el entorno son las barreras
  más citadas para quien llega (Steinmacher et al., 2015).
- Un solo canal público de ayuda en GitHub Discussions e issues para errores,
  sin soporte por correo ni por mensajes privados, con una meta, no una
  garantía, de primera respuesta en 7 días.
- Archivo de avisos de terceros con las licencias de todo lo que la app
  descarga o incluye (sección 8). R es GPL y se descarga aparte, lo que no
  choca con la MIT pero hay que dejarlo claro.
- Comprobación de versiones nuevas que solo consulta la página de versiones y
  nunca envía datos, apagada o preguntada en el primer uso.
- `CITATION.cff` y DOI por versión mediante la integración de Zenodo con
  GitHub, para que la herramienta se pueda citar.

**Después**

- Una segunda persona con permisos para publicar versiones, que sube el factor
  camión de uno a dos (Avelino et al., 2016).
- Paquetes en winget, AUR y Homebrew, y un paquete sin conexión con R, Python
  y Quarto para talleres.
- Etiquetas para primeras contribuciones y guía para traducir la interfaz.

El proyecto evita todo servicio con costo fijo. Los costos que un producto
gratuito igual tiene, como la firma de código o la notarización de macOS, se
deciden de forma explícita (sección 13). El dominio queda para más adelante,
como dice la visión.

## 12. Riesgos y mitigaciones

| Riesgo | Mitigación |
|---|---|
| Construir la herramienta compite con la investigación, el alcance de la visión equivale a varios productos y la v0.1 no llega a usarse | v0.1 de seis piezas en tres tramos, plan B con la cadena actual y QualCoder, fecha anotada antes del tramo 2, cierre con los tramos 1 y 2 si el 3 se atrasa, recorte de lo importante antes que mover fechas, y la regla de que la investigación gana cuando chocan |
| Si se activa el plan B, la codificación queda en QualCoder hasta que exista la importación de `.qdpx` | Las tablas de segmentos que QualCoder exporta en CSV se leen en R mientras tanto, y la importación de `.qdpx` se prioriza si el plan B se activa |
| Agotamiento o abandono de la única persona que mantiene | Regla de alcance escrita, sin contribuciones ni soporte hasta la v1.0, un solo canal público, y sesiones grupales en lugar de ayuda individual |
| Las colegas quedan con datos atrapados si el proyecto se detiene | Carpeta legible sin la app desde la v0.1, `FORMATO.md`, tablas en CSV, REFI-QDA en la v1.0 y la prueba de salida como criterio de publicación |
| Pérdida o corrupción de datos de una tesis por el formato o una migración | Texto plano como única fuente de verdad, versión de formato en el manifiesto, escrituras atómicas, verificación al abrir, respaldo antes de migrar y pruebas con proyectos de muestra |
| Las anclas de los segmentos se corren o se pierden al corregir una transcripción ya codificada | Codificar solo transcripciones marcadas como revisadas, anclas por identificador de segmento, aviso al reabrir que lista los segmentos afectados, y la prueba de 1.000 ediciones aleatorias antes de pasar la codificación a Automerge |
| Whisper inventa o repite frases en silencios y ruido de calle y se citan como dichas por una persona | Segmentos sospechosos con umbrales conocidos, estado sin revisar por defecto, salida cruda conservada, revisión enlazada al audio y, en la cita textual de un segmento sin revisar, un aviso en lugar del texto salvo excepción confirmada y registrada |
| Errores con voseo, jerga y topónimos de San José rompen las citas y el cruce con el mapa | Glosario del proyecto, medición de corrección por modelo con la muestra de evaluación y revisión prioritaria de nombres de lugares |
| Una página web maliciosa lee entrevistas o ejecuta código a través del servidor local | Solo loopback, token por arranque, validación de Host y Origin, sin CORS desde el primer commit, con pruebas automáticas que intentan entrar sin token y desde otro origen como criterio de la v0.1 |
| Abrir el proyecto de otra persona ejecuta su código por el `.Rprofile` o los filtros de Quarto | Proyectos recibidos abren como no confiables hasta que la persona los marque (sección 6.9) |
| Se pierde un disco de respaldo con audios y la tabla de correspondencia | El README de la v0.1 manda la zona identificable solo a un disco cifrado, y desde la v1.0 el respaldo la excluye o la cifra si el destino no está cifrado |
| Fragmentos de entrevistas terminan en un proveedor de IA sin que la persona lo note | Nada de IA ni de red en la v0.1, y desde la v1.0 la compuerta con «nada sale» por defecto, vista previa de cada envío y bitácora de salidas |
| Un nombre queda para siempre en el historial de Automerge o en la copia de una colega | Lo crudo nunca entra a Automerge ni se replica cuando llegue la colaboración, se seudonimiza antes de compartir y el retiro avisa qué copias externas hay que borrar |
| Reidentificación por cuasi identificadores, por puntos del mapa o por metadatos de fotos | Tabla de reemplazos con cuasi identificadores, nunca rotular como anonimizado, sensibilidad por capa, chequeo de publicación con agregación o enmascaramiento y limpieza de EXIF |
| Estigma territorial por mapas de calor de asaltos en un sitio público | Plantillas que comparan anchos de banda, normalizan por exposición, muestran incertidumbre y advierten la cifra negra en el pie de cada mapa |
| Falsa precisión de marcas a mano alzada y resultados que dependen de cómo se dibujaron las zonas | Esquema con punto, línea y polígono y campo de certeza, capa de zonas explícita y registrada, y en la v1.0 la comparación con una zonificación alternativa |
| La IA genera código que corre pero es metodológicamente incorrecto y el resultado fijado le da apariencia de rigor | Declaración de diseño, advertencias citadas, y plantillas revisadas por una persona real con formación metodológica como criterio de la v1.0 |
| Jardín de senderos que se bifurcan, se prueban muchas especificaciones y solo se fija la que conviene | Registro de todas las ejecuciones con el número de variantes visible junto al resultado |
| Las novatas delegan la interpretación en la IA o aceptan sin leer | IA de codificación fuera de la v0.1 y la v1.0, sin botón de aceptar todo, propuestas marcadas y registradas, andamiaje que insiste en memos y definiciones |
| El andamiaje opinionado se vuelve un positivismo implícito que favorece contar sobre interpretar | Tradición declarada por proyecto, caminos distintos por tradición y frecuencias ocultas por defecto en tradiciones interpretativas |
| La persona novata confunde las funciones del programa con el método | Navegación por tarea analítica y explicación con referencia antes de cada camino recomendado |
| Diseñar desde la experiencia del autor, que ya domina R, Quarto y QGIS | Validación con usuarias antes y durante la v1.0 (sección 5) y vocabulario sin kernel, servidor, puerto ni commit |
| Navegador, motor de análisis y transcripción juntos agotan 8 GB y la máquina se traba | Presupuestos medidos, motores bajo demanda, una tarea pesada a la vez, aviso antes de pasarse y modelo de Whisper según la RAM |
| El modelo de app web local confunde, se cierra la pestaña y se cree cerrado el programa | Instancia única, indicador visible, botón de cierre, cierre automático y puerto estable |
| Los binarios de paquetes de R no cubren la distribución del autor y los paquetes espaciales en Linux dependen de GDAL, GEOS y PROJ del sistema | El equipo del autor usa Arch Linux, que no está entre las distribuciones con binarios de Posit Package Manager [verificar], así que la v0.1 usa la biblioteca de R que el autor ya tiene y la prueba del primer día se hace en ese equipo. Desde la v1.0 los binarios se garantizan solo en Windows, macOS y las distribuciones con repositorios de binarios, y las ruedas de Python traen GDAL |
| La orquestación de kernels varía entre sistemas (interrupciones en Windows, procesos huérfanos, caídas silenciosas) | Kernels fuera de la v0.1, canal de control de Jupyter para interrumpir, cierre por grupo de procesos y pruebas de integración en los tres sistemas antes de la v1.0 |
| Doble fuente de verdad entre un `.qmd` en disco y su documento Automerge | En la v0.1 los `.qmd` quedan fuera de la app y el texto es la única fuente de verdad, y la regla de cuál manda se decide antes de meter Automerge (sección 13) |
| Windows se prueba tarde y la primera colega choca con rutas con tildes, OneDrive, antivirus o SmartScreen | Compilación de Windows en integración continua, máquina virtual limpia antes de cada 1.x, instalación por usuario, pruebas con usuario con tilde y ñ, firma gratuita o guía con capturas |
| Windows 10 deja de recibir actualizaciones y R, Quarto o los navegadores lo abandonan | Excepción declarada y revisada en cada versión 1.x, con aviso a las colegas que lo usan |
| La primera ejecución pesa demasiado para conexiones lentas o discos pequeños | Tamaños medidos y publicados, descargas reanudables, componentes solo cuando el proyecto los pide y desinstalación por componente |
| Dependencias jóvenes como iroh, Automerge o rv, o cambios en rig y Quarto, rompen la instalación o el render | Versiones exactas fijadas, cada dependencia detrás de un módulo propio con su plan de salida (sección 8), colaboración P2P después de la v1.0 y render de un proyecto de ejemplo en integración continua |
| La API local de Zotero 7 es de solo lectura, requiere Zotero abierto y hay que activarla en las preferencias [verificar] | Detectar si está disponible, avisar con claridad y trabajar con el `.bib` exportado como respaldo |
| Licencias desconocidas en datos externos, como los policiales, impiden publicar o empaquetar | Campo de licencia obligatorio en la ficha, con «desconocida» que bloquea su inclusión en paquetes públicos hasta aclararla |
| Datos reales terminan en el repositorio público, en un issue o en un diagnóstico pegado | Proyecto real fuera del repositorio, datos de prueba inventados, `.gitignore` defensivo y diagnóstico sin rutas ni contenido |
| Las colegas esperan funciones de Atlas.ti que no están y abandonan | Decir con claridad qué hace y qué no cada versión, y mostrar el valor diferencial con el proyecto de ejemplo integrado con R, mapas y Quarto |
| La experiencia queda partida entre quien paga IA en la nube y quien no | Todo flujo completo y bueno sin IA |
| Descubrir tarde que la investigación necesitaba revisión de un comité ético o un consentimiento con otros alcances | Resolver esas preguntas antes de la primera entrevista y registrar la respuesta en la bitácora (sección 13) |

## 13. Decisiones abiertas para el autor

Las preguntas se ordenan por el momento en que hacen falta, porque varias
cambian el formato de la carpeta y conviene cerrarlas antes del tramo que las
usa.

### Antes de la primera entrevista de octubre

Estas no son de software, pero cuestan nada ahora y mucho después.

- **Hoja de mapa.** ¿La hoja impresa llevará identificador, campo para el
  código de entrevista y cuatro marcas de esquina, y el protocolo pedirá
  numerar cada marca y decir el número en voz alta durante la grabación? Eso
  enlaza la marca con el minuto del audio. Las hojas ya impresas se registran
  con un identificador asignado a posteriori.
- **Comité ético.** ¿La investigación necesita revisión de un comité ético
  científico de la UNED o basta con el aval de la tutora? ¿Cubre a una
  investigación que se va a publicar la excepción de la Ley 8968 para bases de
  uso interno o personal? Conviene consultarlo con la asesoría legal de la UNED
  o con la PRODHAB.
- **Consentimiento.** ¿Es escrito u oral grabado, dónde se guarda, y pide por
  separado grabar, transcribir con software local, citar textualmente, usar los
  puntos del mapa, archivar y reutilizar? Lo que no se pida ahora no se podrá
  hacer después.
- **Evaluación de la transcripción.** ¿El consentimiento pide también usar
  fragmentos para evaluar la transcripción automática sin que salgan de la
  máquina? Sin ese alcance, la muestra de calidad de la v1.0 se arma con
  grabaciones del autor y de voluntarias.
- **Terceros y menores.** ¿La guía advierte que no se pidan nombres de
  terceros? ¿Se excluye a menores de edad de las entrevistas en la calle?
- **Resguardo.** ¿El proyecto vive fuera de carpetas sincronizadas y en un
  disco cifrado?
- **Retención.** ¿Los audios se destruyen al terminar la revisión o se
  conservan, por cuánto tiempo y quién decide?

### Antes del tramo 1

- ¿Cuántas horas semanales dedica el autor a construir la herramienta? Sin esa
  cifra el tamaño de los tramos no se puede planificar.
- ¿El formato canónico de las transcripciones es WebVTT o un JSON documentado
  con hablantes y estado de revisión?
- ¿La unidad de codificación es la selección libre, el turno de habla o el
  párrafo? Afecta la operación por teclado y la accesibilidad.
- ¿Se corrigen transcripciones después de empezar a codificar o se cierran
  antes? ¿Qué convención se adopta, literal con pausas y muletillas o limpia?
- ¿Registrar el consentimiento es requisito para importar un audio o solo un
  aviso?
- ¿Cuántas entrevistas se esperan y de qué duración? ¿Alguna será un recorrido
  acompañado con registro de ubicación?

### Antes del tramo 2

- ¿Cuál es la fecha concreta del plan B? Se propone el cierre de la primera
  tanda de entrevistas convertido en una fecha, anotada en
  `docs/decisiones.md`.
- ¿Qué entregas concretas del ISA, con sus fechas, tiene que servir primero la
  v0.1, y cuáles piden transcripciones, libro de códigos, citas o cifras?
- ¿Qué enfoque de análisis usará el autor (análisis temático reflexivo,
  análisis de contenido cualitativo, teoría fundamentada u otro)? Define si la
  v0.1 necesita también resúmenes de caso.
- ¿La tutora exige una segunda codificadora o un índice de acuerdo?
- El análisis de contenido de las noticias por zona es parte de la capa
  cuantitativa de la investigación. ¿Necesita importar notas de prensa este
  semestre? Si la respuesta es sí, la importación de TXT y Markdown para
  codificar sube a la v0.1 como importante.

### Antes del tramo 3

- ¿Cuál es la unidad de integración por zona, los barrios del municipio, una
  cuadrícula hexagonal, tramos de calle o zonas que definan las propias
  personas?
- ¿Qué tipos de marca se piden (punto, recorrido, área), con qué categorías
  (peligro, evitación, refugio) y se distingue el momento del día?
- ¿Qué se entiende por islas de calor, puntos calientes de delito o la capa de
  temperatura superficial? Cambia si los rasters son v1.0 o después.
- ¿Se publicarán mapas con marcas individuales y con qué agregación (celda de
  100 o 250 m, cuadra, distrito)?
- ¿El registro de procedencia es estrictamente de solo agregar, con las
  correcciones como entradas nuevas?
- En la v0.1 los insumos se declaran con `leer_insumo()` y la unidad de
  procedencia es el script completo. ¿Se intenta después detectar insumos de
  forma automática, con sus falsos negativos?

### Antes de la v1.0

- ¿Quiénes serán las primeras colegas de la v1.0, sin nombres en este
  documento, y con qué sistema operativo y hardware trabajan en realidad?
- ¿Los notebooks se guardan como `.ipynb`, por interoperabilidad con Jupyter y
  con Quarto, o como `.qmd`, por legibilidad y diferencias limpias?
- ¿Quién manda entre un `.qmd` en disco y su documento Automerge cuando los
  documentos pasen a Automerge, y desde qué versión?
- ¿QGIS sigue haciendo las láminas finales en la v1.0, o el visor de la app
  pasa a producir mapas para el documento?
- Al renderizar con un resultado desactualizado, ¿se marca de forma visible,
  como está decidido para la v0.1, o se bloquea el render en la v1.0?
- Ante inferencia sobre una muestra no probabilística, ¿la herramienta solo
  advierte o impide fijar el resultado? ¿Qué umbrales de coeficiente de
  variación y de n mínimo se adoptan, y publica el INEC criterios propios?
- ¿Para quién es el andamiaje de la v1.0, estudiantes de posgrado, de grado,
  personal de organizaciones o docentes? ¿Qué tradiciones tienen camino
  recomendado completo en la v1.0, si no caben todas?
- ¿Hace falta un modo de revisión con comentarios para la tutora, y la
  herramienta se usará en cursos de metodología sin funciones de aula?
- ¿Qué registro usa la interfaz, tú, usted o vos? La UNED escribe de usted, el
  voseo es lo natural en Costa Rica y para colegas de otros países conviene uno
  neutral.
- ¿Qué IA se ofrece por defecto a una novata sin dinero ni GPU, ninguna, un
  modelo local pequeño o la nube con su llave? ¿Se permite que la nube vea
  transcripciones seudonimizadas?
- ¿Qué modelo de Whisper será el recomendado después de medir el error en
  español costarricense, y qué umbral de alucinaciones no detectadas haría que
  no se recomiende la transcripción automática?
- ¿Cómo se declara el uso de IA externa a la herramienta, como texto copiado
  desde un chat, que la app no puede registrar sola?
- ¿Las codificaciones pueden anclarse también a tramos de audio sin
  transcribir?
- ¿La app abre en una pestaña normal o en una ventana propia en modo
  aplicación de Edge o Chrome, que se parece más a un programa de escritorio?
- ¿Se publican versiones con aceleración por GPU (Vulkan, CUDA, Metal), que
  agrandan el binario y la matriz de pruebas?
- ¿Qué distribuciones Linux reciben soporte completo en la v1.0?
- ¿El formato de la carpeta se publica como especificación abierta antes de la
  v1.0?

### Después de la v1.0

- Cuando haya colaboración, ¿quién ve el registro de procedencia, la salida
  cruda y la tabla de seudónimos, y quién custodia esa tabla?

### Proyecto libre, sin urgencia

- ¿Se aceptan donaciones o un anfitrión fiscal como Open Collective solo para
  costos como la firma de código, o el proyecto se compromete a no tener ningún
  costo? ¿Se paga de bolsillo la notarización de macOS o se publica sin ella?
- ¿En qué idioma van el código, los identificadores, los commits y los issues,
  español como la interfaz o inglés para atraer contribuciones?
- ¿Cadencia de versiones en v0.x, cuando haga falta o fija, por ejemplo
  mensual?
- ¿Qué pasa si el autor deja de mantener, quién puede publicar y cómo se avisa
  a las usuarias?
- ¿Qué condiciones de uso tienen los datos policiales descargados, y dónde se
  archivará el paquete final de la investigación, en Zenodo, en el repositorio
  de la UNED o en ambos? ¿Qué licencia por defecto se ofrece para datos, CC BY
  4.0 o CC0?

### Propuestas provisionales ante discrepancias entre revisiones

| Tema | Propuesta provisional | La discrepancia |
|---|---|---|
| Transcripción integrada | v1.0, en la v0.1 solo se importan transcripciones hechas fuera de la app | Cuatro revisiones la querían imprescindible en v0.1 pensando en la colega novata, la de arquitectura la dejaba para v1.0 |
| Notebooks | v1.0, en la v0.1 los resultados fijados salen de scripts de R lanzados por `osr recalcular` | Las revisiones de experiencia de usuario y de docencia proponían una v0.1 sin notebooks, otras los querían en la v0.1 por ser la promesa central |
| Automerge para codificación y memos | Llega con la colaboración, en la v0.1 texto canónico documentado para migrarlo | Guardar en Automerge desde la v0.1 evitaba una migración posterior |
| Entorno de R con rv | v1.0, en la v0.1 el R del sistema con `sessionInfo()` en el manifiesto | Tenerlo desde la v0.1 daba procedencia completa desde el primer resultado |
| Consentimiento y seudonimización | v0.1, la seudonimización por subcomando y su interfaz en v1.0 | La revisión de arquitectura los dejaba para v1.0 con el autor seudonimizando a mano |
| Respaldo a disco externo | v1.0 con manejo del destino sin cifrar, en la v0.1 una guía en el README | La revisión de docencia lo pedía imprescindible en la v0.1 |
| Editor de documentos en la app | v1.0 importante | La revisión de experiencia de usuario lo dejaba para después |
| Asistente de IA en notebooks | v1.0, solo si alguien lo usa en una investigación real | La revisión de experiencia de usuario lo dejaba para después |
| Importar `.qdpx` completo | Después, con las tablas CSV de QualCoder como puente del plan B | La revisión cualitativa lo pedía en v1.0 para quien migra con una tesis empezada, y otra salida era un importador mínimo de QualCoder en la v1.0 |
| Codificación en equipo y acuerdo | Después | La revisión cualitativa la pedía en v1.0 |
| Conector NADA del INEC | Después | Las revisiones de metodología cuantitativa y de ciencia abierta lo pedían en v1.0 |
| Visor de mapas | v1.0 importante | La revisión de SIG lo pedía importante, la de arquitectura lo dejaba para después |
| Digitalizar la hoja de mapa en la app | Después | La revisión de SIG la pedía en v1.0, la de arquitectura la ponía fuera de alcance, y depende de si las colegas saben QGIS |
| Presentación conjunta | v1.0 deseable, como plantilla de Quarto | La revisión de docencia la pedía importante, la de metodología cuantitativa la dejaba para después |
| Matriz de coherencia | v1.0 importante | La revisión de docencia la pedía imprescindible |
| Cifrado de audios y tabla de claves en la app | Después, con comprobación del cifrado de disco en v1.0 | La revisión de docencia lo pedía en v1.0 |
| macOS | Después | Tres revisiones lo querían en v1.0 |
| Piso de RAM | 8 GB en la v1.0 y el modo de 4 GB después | Garantizar 4 GB amplía el público pero sube el costo de pruebas sin que ninguna colega lo haya pedido |
| Velocidad de transcripción | 1,5 veces la duración del audio con small, a medir | Las propuestas iban de 1 a 2 veces |

## 14. Referencias

- Amershi, S., Weld, D., Vorvoreanu, M., Fourney, A., Nushi, B., Collisson,
  P., Suh, J., Iqbal, S., Bennett, P. N., Inkpen, K., Teevan, J., Kikin-Gil, R.
  y Horvitz, E. (2019). Guidelines for human-AI interaction. En *Proceedings of
  the 2019 CHI Conference on Human Factors in Computing Systems* (pp. 1-13).
  ACM. https://doi.org/10.1145/3290605.3300233
- Andresen, M. A. (2006). Crime measures and the spatial analysis of criminal
  activity. *British Journal of Criminology*, *46*(2), 258-285.
  https://doi.org/10.1093/bjc/azi054
- Armstrong, M. P., Rushton, G. y Zimmerman, D. L. (1999). Geographically
  masking health data to preserve confidentiality. *Statistics in Medicine*,
  *18*(5), 497-525.
- Asamblea Legislativa de la República de Costa Rica. (2011). *Ley N.° 8968,
  Ley de Protección de la Persona frente al Tratamiento de sus Datos
  Personales*. La Gaceta N.° 170, 5 de setiembre de 2011. [verificar número y
  fecha de La Gaceta en el SCIJ]
- Atkinson, R. K., Derry, S. J., Renkl, A. y Wortham, D. (2000). Learning from
  examples. Instructional principles from the worked examples research.
  *Review of Educational Research*, *70*(2), 181-214.
  https://doi.org/10.3102/00346543070002181
- Avelino, G., Passos, L., Hora, A. y Valente, M. T. (2016). A novel approach
  for estimating truck factors. En *Proceedings of the 24th IEEE International
  Conference on Program Comprehension (ICPC 2016)*. IEEE.
- Baker, R., Brick, J. M., Bates, N. A., Battaglia, M., Couper, M. P., Dever,
  J. A., Gile, K. J. y Tourangeau, R. (2013). Summary report of the AAPOR task
  force on non-probability sampling. *Journal of Survey Statistics and
  Methodology*, *1*(2), 90-143. https://doi.org/10.1093/jssam/smt008
- Boschmann, E. E. y Cubbon, E. (2014). Sketch maps and qualitative GIS. Using
  cartographies of individual spatial narratives in geographic research. *The
  Professional Geographer*, *66*(2), 236-248. [verificar páginas]
- Braun, V. y Clarke, V. (2006). Using thematic analysis in psychology.
  *Qualitative Research in Psychology*, *3*(2), 77-101.
  https://doi.org/10.1191/1478088706qp063oa
- Braun, V. y Clarke, V. (2019). Reflecting on reflexive thematic analysis.
  *Qualitative Research in Sport, Exercise and Health*, *11*(4), 589-597.
  https://doi.org/10.1080/2159676X.2019.1628806
- Braun, V. y Clarke, V. (2021). To saturate or not to saturate? Questioning
  data saturation as a useful concept for thematic analysis and sample-size
  rationales. *Qualitative Research in Sport, Exercise and Health*, *13*(2),
  201-216.
- Brooke, J. (1996). SUS. A quick and dirty usability scale. En P. W. Jordan,
  B. Thomas, B. A. Weerdmeester e I. L. McClelland (eds.), *Usability
  evaluation in industry* (pp. 189-194). Taylor & Francis.
- Carroll, J. M. y Rosson, M. B. (1987). Paradox of the active user. En J. M.
  Carroll (ed.), *Interfacing thought. Cognitive aspects of human-computer
  interaction* (pp. 80-111). MIT Press.
- Chainey, S. y Ratcliffe, J. (2005). *GIS and crime mapping*. Wiley.
- Chen, L., Zaharia, M. y Zou, J. (2024). How is ChatGPT's behavior changing
  over time? *Harvard Data Science Review*, *6*(2). [verificar DOI]
- Coelho, J. y Valente, M. T. (2017). Why modern open source projects fail. En
  *Proceedings of the 2017 11th Joint Meeting on Foundations of Software
  Engineering (ESEC/FSE 2017)* (pp. 186-196). ACM. [verificar páginas]
- Cope, M. y Elwood, S. (eds.) (2009). *Qualitative GIS. A mixed methods
  approach*. SAGE.
- Corti, L., Van den Eynden, V., Bishop, L. y Woollard, M. (2014). *Managing
  and sharing research data. A guide to good practice*. SAGE.
- Council for International Organizations of Medical Sciences [CIOMS]. (2016).
  *International ethical guidelines for health-related research involving
  humans*. CIOMS.
- DDI Alliance. (2014). *DDI-Codebook 2.5* [Especificación técnica]. DDI
  Alliance. [verificar]
- de Montjoye, Y.-A., Hidalgo, C. A., Verleysen, M. y Blondel, V. D. (2013).
  Unique in the crowd. The privacy bounds of human mobility. *Scientific
  Reports*, *3*, 1376. https://doi.org/10.1038/srep01376
- Eghbal, N. (2020). *Working in public. The making and maintenance of open
  source software*. Stripe Press.
- Fetters, M. D., Curry, L. A. y Creswell, J. W. (2013). Achieving integration
  in mixed methods designs. Principles and practices. *Health Services
  Research*, *48*(6, pt. 2), 2134-2156.
  https://doi.org/10.1111/1475-6773.12117
- Fogel, K. (2005). *Producing open source software. How to run a successful
  free software project*. O'Reilly Media.
- Gelman, A. y Loken, E. (2014). The statistical crisis in science. *American
  Scientist*, *102*(6), 460-465.
- Gerganov, G. (s. f.). *whisper.cpp* [Software]. GitHub.
  https://github.com/ggml-org/whisper.cpp [citar la versión o el commit usado
  y verificar las cifras de memoria en su README]
- Getis, A. y Ord, J. K. (1992). The analysis of spatial association by use of
  distance statistics. *Geographical Analysis*, *24*(3), 189-206.
- Gilardi, F., Alizadeh, M. y Kubli, M. (2023). ChatGPT outperforms crowd
  workers for text-annotation tasks. *Proceedings of the National Academy of
  Sciences*, *120*(30), e2305016120. https://doi.org/10.1073/pnas.2305016120
- Heeringa, S. G., West, B. T. y Berglund, P. A. (2017). *Applied survey data
  analysis* (2.ª ed.). CRC Press.
- Hundepool, A., Domingo-Ferrer, J., Franconi, L., Giessing, S., Schulte
  Nordholt, E., Spicer, K. y de Wolf, P.-P. (2012). *Statistical disclosure
  control*. Wiley.
- Jackson, C., Barth, A., Bortz, A., Shao, W. y Boneh, D. (2007). Protecting
  browsers from DNS rebinding attacks. En *Proceedings of the 14th ACM
  Conference on Computer and Communications Security (CCS '07)* (pp. 421-431).
  ACM. [verificar páginas]
- Ji, Z., Lee, N., Frieske, R., Yu, T., Su, D., Xu, Y., Ishii, E., Bang, Y. J.,
  Madotto, A. y Fung, P. (2023). Survey of hallucination in natural language
  generation. *ACM Computing Surveys*, *55*(12), 1-38.
  https://doi.org/10.1145/3571730
- Kalyuga, S., Ayres, P., Chandler, P. y Sweller, J. (2003). The expertise
  reversal effect. *Educational Psychologist*, *38*(1), 23-31.
  https://doi.org/10.1207/S15326985EP3801_4
- Kleppmann, M., Wiggins, A., van Hardenberg, P. y McGranaghan, M. (2019).
  Local-first software. You own your data, in spite of the cloud. En
  *Proceedings of the 2019 ACM SIGPLAN International Symposium on New Ideas,
  New Paradigms, and Reflections on Programming and Software (Onward! 2019)*
  (pp. 154-178). ACM. https://doi.org/10.1145/3359591.3359737
- Koenecke, A., Choi, A. S. G., Mei, K. X., Schellmann, H. y Sloane, M.
  (2024). Careless Whisper. Speech-to-text hallucination harms. En
  *Proceedings of the 2024 ACM Conference on Fairness, Accountability, and
  Transparency (FAccT '24)* (pp. 1672-1681). ACM.
  https://doi.org/10.1145/3630106.3658996 [verificar páginas y DOI]
- Kounadi, O. y Leitner, M. (2014). Why does geoprivacy matter? The scientific
  publication of confidential data presented on maps. *Journal of Empirical
  Research on Human Research Ethics*, *9*(4), 34-45. [verificar páginas]
- Krippendorff, K. (2018). *Content analysis. An introduction to its
  methodology* (4.ª ed.). SAGE.
- Kuckartz, U. y Rädiker, S. (2023). *Qualitative content analysis. Methods,
  practice and software* (2.ª ed.). SAGE.
- Kusenbach, M. (2003). Street phenomenology. The go-along as ethnographic
  research tool. *Ethnography*, *4*(3), 455-485.
- Lincoln, Y. S. y Guba, E. G. (1985). *Naturalistic inquiry*. SAGE.
- Lumley, T. (2010). *Complex surveys. A guide to analysis using R*. Wiley.
- MacQueen, K. M., McLellan, E., Kay, K. y Milstein, B. (1998). Codebook
  development for team-based qualitative analysis. *Cultural Anthropology
  Methods*, *10*(2), 31-36. https://doi.org/10.1177/1525822X980100020301
- Malterud, K., Siersma, V. D. y Guassora, A. D. (2016). Sample size in
  qualitative interview studies. Guided by information power. *Qualitative
  Health Research*, *26*(13), 1753-1760.
  https://doi.org/10.1177/1049732315617444
- Miles, M. B., Huberman, A. M. y Saldaña, J. (2014). *Qualitative data
  analysis. A methods sourcebook* (3.ª ed.). SAGE.
- Mitchell, M., Wu, S., Zaldivar, A., Barnes, P., Vasserman, L., Hutchinson,
  B., Spitzer, E., Raji, I. D. y Gebru, T. (2019). Model cards for model
  reporting. En *Proceedings of the Conference on Fairness, Accountability,
  and Transparency* (pp. 220-229). ACM. https://doi.org/10.1145/3287560.3287596
- Nielsen, J. (1993). *Usability engineering*. Academic Press.
- Nielsen, J. y Landauer, T. K. (1993). A mathematical model of the finding of
  usability problems. En *Proceedings of the INTERACT '93 and CHI '93
  Conference on Human Factors in Computing Systems* (pp. 206-213). ACM.
  https://doi.org/10.1145/169059.169166
- O'Connor, C. y Joffe, H. (2020). Intercoder reliability in qualitative
  research. Debates and practical guidelines. *International Journal of
  Qualitative Methods*, *19*. https://doi.org/10.1177/1609406919899220
- Okabe, A., Satoh, T. y Sugihara, K. (2009). A kernel density estimation
  method for networks, its computational method and a GIS-based tool.
  *International Journal of Geographical Information Science*, *23*(1), 7-32.
- Openshaw, S. (1984). *The modifiable areal unit problem* (Concepts and
  Techniques in Modern Geography, 38). Geo Books.
- Ord, J. K. y Getis, A. (1995). Local spatial autocorrelation statistics.
  Distributional issues and an application. *Geographical Analysis*, *27*(4),
  286-306.
- Parasuraman, R. y Manzey, D. H. (2010). Complacency and bias in human use of
  automation. An attentional integration. *Human Factors*, *52*(3), 381-410.
  https://doi.org/10.1177/0018720810376055
- Parker, J. D., Talih, M., Malec, D. J. et al. (2017). *National Center for
  Health Statistics data presentation standards for proportions* (Vital and
  Health Statistics, serie 2, n.° 175). National Center for Health Statistics.
  [verificar autores]
- Pimentel, J. F., Murta, L., Braganholo, V. y Freire, J. (2019). A
  large-scale study about quality and reproducibility of Jupyter notebooks. En
  *2019 IEEE/ACM 16th International Conference on Mining Software Repositories
  (MSR)* (pp. 507-517). IEEE. https://doi.org/10.1109/MSR.2019.00077
  [verificar páginas]
- Poder Ejecutivo de la República de Costa Rica. (2013). *Reglamento a la Ley
  de Protección de la Persona frente al Tratamiento de sus Datos Personales*
  (Decreto Ejecutivo N.° 37554-JP). La Gaceta. [verificar fecha y número de La
  Gaceta]
- Radford, A., Kim, J. W., Xu, T., Brockman, G., McLeavey, C. y Sutskever, I.
  (2023). Robust speech recognition via large-scale weak supervision. En
  *Proceedings of the 40th International Conference on Machine Learning*
  (PMLR 202, pp. 28492-28518). [verificar páginas]
- REFI-QDA. (2019). *REFI-QDA Project Exchange Standard* [Especificación
  técnica]. https://www.qdasoftware.org [verificar versión]
- Sandve, G. K., Nekrutenko, A., Taylor, J. y Hovig, E. (2013). Ten simple
  rules for reproducible computational research. *PLOS Computational
  Biology*, *9*(10), e1003285. https://doi.org/10.1371/journal.pcbi.1003285
- Sauro, J. y Lewis, J. R. (2016). *Quantifying the user experience. Practical
  statistics for user research* (2.ª ed.). Morgan Kaufmann.
- Saunders, B., Kitzinger, J. y Kitzinger, C. (2015). Anonymising interview
  data. Challenges and compromise in practice. *Qualitative Research*,
  *15*(5), 616-632.
- Sevilla-Gonzalez, M. del R. et al. (2020). Spanish version of the System
  Usability Scale for the assessment of electronic tools. Development and
  validation. *JMIR Human Factors*, *7*(4), e21161. [verificar]
- Simmons, J. P., Nelson, L. D. y Simonsohn, U. (2011). False-positive
  psychology. Undisclosed flexibility in data collection and analysis allows
  presenting anything as significant. *Psychological Science*, *22*(11),
  1359-1366. https://doi.org/10.1177/0956797611417632
- Soiland-Reyes, S., Sefton, P., Crosas, M., Castro, L. J., Coppens, F.,
  Fernández, J. M. et al. (2022). Packaging research artefacts with RO-Crate.
  *Data Science*, *5*(2), 97-138. https://doi.org/10.3233/DS-210053
- Steinmacher, I., Silva, M. A. G., Gerosa, M. A. y Redmiles, D. F. (2015). A
  systematic literature review on the barriers faced by newcomers to open
  source software projects. *Information and Software Technology*, *59*,
  67-85.
- Sweeney, L. (2002). k-anonymity. A model for protecting privacy.
  *International Journal of Uncertainty, Fuzziness and Knowledge-Based
  Systems*, *10*(5), 557-570. https://doi.org/10.1142/S0218488502001648
- W3C. (2023). *Web Content Accessibility Guidelines (WCAG) 2.2*. W3C
  Recommendation. https://www.w3.org/TR/WCAG22/
- Wacquant, L. (2007). Territorial stigmatization in the age of advanced
  marginality. *Thesis Eleven*, *91*(1), 66-77.
- Wiles, R., Crow, G., Heath, S. y Charles, V. (2008). The management of
  confidentiality and anonymity in social research. *International Journal of
  Social Research Methodology*, *11*(5), 417-428.
- Wilson, G., Bryan, J., Cranston, K., Kitzes, J., Nederbragt, L. y Teal, T. K.
  (2017). Good enough practices in scientific computing. *PLOS Computational
  Biology*, *13*(6), e1005510. https://doi.org/10.1371/journal.pcbi.1005510
- Wood, D., Bruner, J. S. y Ross, G. (1976). The role of tutoring in problem
  solving. *Journal of Child Psychology and Psychiatry*, *17*(2), 89-100.
  https://doi.org/10.1111/j.1469-7610.1976.tb00381.x
- Woolf, N. H. y Silver, C. (2018). *Qualitative analysis using ATLAS.ti. The
  five-level QDA method*. Routledge.
- Ziems, C., Held, W., Shaikh, O., Chen, J., Zhang, Z. y Yang, D. (2024). Can
  large language models transform computational social science?
  *Computational Linguistics*, *50*(1), 237-291.
  https://doi.org/10.1162/coli_a_00502
