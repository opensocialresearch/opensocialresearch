# Prompt para Claude Design

Diseñá la interfaz de **Open Social Research**, versión 0.1. Necesito un sistema de diseño y las pantallas de alta fidelidad descritas abajo, que van a servir de referencia para implementarla en Svelte.

## Qué es

Open Social Research es una herramienta libre y gratuita para hacer investigación social aplicada de punta a punta. La versión 0.1 cubre la parte más delicada del trabajo con entrevistas:
- registrar a las personas participantes con un seudónimo y su consentimiento;
- separar el material que identifica a alguien del material de trabajo;
- importar audios y transcripciones hechas con Whisper;
- revisar cada transcripción escuchando el audio.

Más adelante va a sumar codificación cualitativa, notebooks de R y Python, mapas y escritura en Quarto, pero eso no se diseña ahora.

**Cómo funciona:** es una aplicación web local. Un programa corre en la computadora de quien investiga y abre la herramienta en el navegador. No hay cuentas, ni inicio de sesión, ni nube, ni nada que se comparta. Abre siempre un solo proyecto, que es una carpeta en el disco.

## Quién la usa

- **Primero, su autor:** un estudiante de posgrado que investiga la percepción de inseguridad al caminar en el centro de San José, Costa Rica. Hace entrevistas grabadas y pasa horas revisando transcripciones.
- **Después, colegas que empiezan a investigar:** estudiantes con laptops modestas, en su mayoría con Windows, 8 GB de RAM y pantallas de 1366 × 768. Saben poco de programación, pero sí qué es una entrevista y un consentimiento informado.

## Principios de diseño

1. **Sobria, editorial y compacta.** Es una herramienta de trabajo largo, no una app de consumo. Una base en blanco y negro con un solo color de acento, mucha jerarquía tipográfica y poco adorno. La referencia de gusto son las hojas de mapa impresas del proyecto: tipografía limpia, líneas finas y negro sobre papel. Sin emojis, sin ilustraciones, sin gradientes, sin sombras grandes.
2. **El texto manda, el color acompaña.** Cada estado se escribe con palabras y puede llevar un ícono: guardado, zona, sensibilidad, revisado o sospechoso. El color puede reforzar, nunca reemplazar al texto. La interfaz debe cumplir WCAG 2.2 AA.
3. **Lo identificable se nota.** La herramienta distingue dos zonas, la **Zona identificable** (audios, nombres reales, consentimientos firmados) y la **Zona de trabajo** (material seudonimizado). Cualquier cosa de la zona identificable debe verse distinta de inmediato, sin parecer una alarma permanente.
4. **Teclado primero en el editor.** Quien revisa una transcripción no suelta el teclado. Los atajos tienen que verse y descubrirse.
5. **Liviana.** Debe correr fluida en máquinas modestas. Nada de animaciones pesadas, fondos de video o imágenes decorativas. Las listas largas, de hasta 1000 segmentos, se ven completas sin que la página se trabe.
6. **En español, con mensajes impersonales.** Botones en infinitivo («Importar audio», «Registrar consentimiento»). Mensajes que dicen qué pasó y qué hacer, sin tutear ni ustedear: «No se pudo guardar: el disco está lleno. Se reintenta en 5 s.». Los textos en español son más largos que en inglés, así que el diseño tiene que aguantarlos.
7. **Modo claro y modo oscuro**, los dos con el mismo cuidado.

## Sistema de diseño

Definí estos tokens y componentes:
- **Tokens:** colores, tipografía, espaciado, radios y bordes.
  - Proponé una tipografía para la interfaz, por ejemplo una sans semicondensada como Archivo, que ya usa el proyecto en sus mapas.
  - Proponé otra para leer transcripciones largas, que puede ser una serif de buena legibilidad.
- **Etiquetas de estado**, con texto e ícono cada una:
  - Zona: `Zona identificable` y `Zona de trabajo`.
  - Sensibilidad: `Identificable`, `Sensible`, `No sensible` y `Sin marca`.
  - Guardado: `Guardado`, `Guardando…` y `No se pudo guardar: <motivo>. Se reintenta en 5 s.`
  - Audio: `Sin audio`, `Audio 1:02:15`, `Audio eliminado el 12/10/2026`, `Falta el audio` y `El audio cambió desde su importación`.
  - Transcripción: `Sin transcripción`, `Transcripción sin revisar`, `Transcripción en revisión (412 de 980 segmentos revisados)` y `Transcripción revisada`.
  - Segmento: `Revisado`, `Sin revisar` y `Sospechoso: texto repetido`.
  - Consentimiento: `Consentimiento registrado` y `Consentimiento pendiente`.
- **Componentes:** botones (principal, secundario y peligroso), tablas densas, formularios con errores por campo, diálogos de confirmación, un panel lateral y la barra de reproducción de audio.

## Pantallas

Diseñá cada pantalla en 1366 × 768, que es la de referencia, y en 1920 × 1080. Usá datos de ejemplo **inventados**: participantes E01 «Ana Inventada Muñoz», E02 «Bruno Ficticio Ramírez» y E03 «Carmen Inventada Solís». Las transcripciones de ejemplo son conversaciones ficticias sobre caminar de noche por el centro de una ciudad.

### 1. Marco general

- **Encabezado:** el nombre del proyecto, el estado del guardado y un acceso a la lista de atajos (`Ctrl+K`).
- **Navegación:** Inicio, Participantes, Entrevistas, Archivos del proyecto y Registro. Dejá lugar para módulos futuros (Codificación, Análisis, Documento, Mapa), pero no los muestres ahora.
- **Aviso de verificación al abrir:** al abrir el proyecto, la herramienta revisa que los archivos estén completos. Mostrá el caso en que todo está bien y el caso con un aviso, por ejemplo `Falta el audio de la entrevista E02`.

### 2. Inicio del proyecto

Un resumen breve:
- cuántas personas participantes hay y cuántas tienen el consentimiento registrado;
- cuántas entrevistas hay, en qué estado está cada transcripción y cuántos segmentos quedan por revisar;
- un acceso directo a seguir revisando donde se quedó.

Incluí también el **estado vacío** de un proyecto recién creado, que invite a registrar a la primera persona participante.

### 3. Participantes

Una tabla con:
- el seudónimo (E01, E02…);
- el estado del consentimiento;
- las etiquetas del audio y de la transcripción de su entrevista.

La acción principal es «Agregar participante». El seudónimo lo genera la herramienta.

### 4. Ficha de participante y consentimiento

- La ficha muestra el seudónimo, el consentimiento y la entrevista.
- **Formulario «Registrar consentimiento»:** versión del formulario, fecha, modalidad (`Escrito` u `Oral grabado`) y respaldo (`Ninguno`, `Archivos` o `Audio de la entrevista`, con un tramo de inicio y fin).
- **Alcances del consentimiento,** cada uno con tres valores, `Otorgado`, `Denegado` o `No consultado`, que empieza en `No consultado`. Los alcances son grabar, citar textualmente, usar sus puntos del mapa, archivar y reutilizar.
- Mostrá un error de validación en un campo.

### 5. Correspondencia

Es la tabla que liga cada seudónimo con el nombre real y el contacto de la persona. Vive en la **Zona identificable** y es la vista más sensible de la herramienta. Debe verse claramente distinta, con su etiqueta de zona bien visible y una nota breve que recuerde que estos datos no salen de la computadora. Cada fila se edita y se guarda con «Guardar».

### 6. Archivos del proyecto

Lista todos los archivos de la carpeta agrupados por carpeta. Cada archivo lleva dos etiquetas, la zona y la sensibilidad, y su tamaño. Tiene que distinguirse de un vistazo qué es identificable.

### 7. Importar a una entrevista

Un flujo de dos o tres pasos dentro de la entrevista de E01:

1. **Elegir los archivos:** el audio (`.ogg`, `.opus`, `.mp3`, `.wav` o `.m4a`) y los archivos de la transcripción, que pueden ser un `.whisper.json`, un `.hablantes.json`, un `.md` o un `.vtt`. La herramienta dice qué reconoció en cada uno.
2. **Asignar hablantes:** cada hablante que detectó la transcripción (Hablante 1, Hablante 2) se asigna a `Entrevistadora`, `E01` o `Otra persona`, con una muestra de lo que dice.
3. **Declarar el modelo de Whisper,** que puede ser `desconocido`, y confirmar.

Estados a mostrar:
- un error de validación, por ejemplo `El archivo E01.md no coincide con E01.hablantes.json en la línea 3`;
- el **bloqueo por consentimiento**, `No hay consentimiento registrado para E01`, con la opción de seguir mediante una excepción que pide escribir el motivo y queda registrada.

### 8. Editor de revisión de transcripciones (la pantalla más importante)

Aquí se pasan horas, así que es la pantalla que más cuidado necesita.

**Encabezado:**
- el identificador E01;
- la etiqueta del audio y la de la transcripción;
- `412 de 980 segmentos revisados`;
- `Sospechosos sin revisar: 3`;
- `Modelo: large-v3-turbo`;
- el tiempo de revisión acumulado;
- el estado del guardado;
- la velocidad de reproducción (0,75×, 1×, 1,25×…);
- los controles de reproducción;
- el botón «Marcar transcripción como revisada».

**Lista de segmentos.** Cada segmento muestra:
- un botón con su tiempo de inicio en `m:ss` que reproduce desde ahí;
- la etiqueta del hablante (`Entrevistadora`, `E01`, `Otra persona` o `Sin asignar`);
- el texto editable;
- la etiqueta `Revisado` o `Sin revisar`;
- si corresponde, `Sospechoso: texto repetido`.

El segmento que suena se distingue por texto o ícono, no solo por color, y la lista lo sigue. El segmento con foco también se distingue.

**Barra de reproducción:** fija, con reproducir y pausar, retroceder unos segundos, la posición y la duración, y la velocidad.

**Panel de atajos (`Ctrl+K`):** una tabla que se cierra con `Esc`.

| Acción | Atajo |
|---|---|
| Reproducir o pausar | `Esc` |
| Retroceder unos segundos | `Ctrl+Alt+J` |
| Reproducir desde el inicio del segmento | `Ctrl+Alt+Espacio` |
| Bajar o subir la velocidad | `Ctrl+Alt+,` y `Ctrl+Alt+.` |
| Segmento anterior o siguiente | `Alt+↑` y `Alt+↓` |
| Marcar o desmarcar como revisado | `Ctrl+Enter` |
| Siguiente sospechoso sin revisar | `Ctrl+Alt+S` |
| Hablante entrevistadora, entrevistada, otra o sin asignar | `Ctrl+Alt+1`, `2`, `3` y `0` |
| Dividir en el cursor | `Ctrl+Alt+D` |
| Unir con el siguiente | `Ctrl+Alt+U` |
| Eliminar el segmento | `Ctrl+Alt+E` |
| Fijar el inicio o el fin en la posición del audio | `Ctrl+Alt+I` y `Ctrl+Alt+O` |
| Deshacer y rehacer | `Ctrl+Z` y `Ctrl+Shift+Z` |

Mostrá también estos estados del editor:
- **Revisada, en solo lectura:** el audio se puede reproducir pero el texto no se edita. Tiene el botón «Reabrir para editar», que pide confirmar con `La transcripción vuelve a estar en revisión.`
- **Bloqueo al cerrar:** `Quedan 12 segmentos sin revisar.` al intentar marcarla como revisada, con la opción de excepción.
- **Aviso de reglas omitidas**, arriba de la lista: `No se aplicaron las reglas de posible texto en un silencio y posible texto repetitivo porque el archivo no trae no_speech_prob, avg_logprob y compression_ratio.`
- **«Eliminar audio».** Se ofrece con la transcripción revisada, para cumplir lo prometido en el consentimiento. Es una acción peligrosa, con una confirmación clara y que no se puede deshacer.

### 9. Diálogos

- **Excepción confirmada:** cuando una regla bloquea algo, la herramienta explica por qué y deja seguir solo si se escribe un motivo, que queda registrado. Debe sentirse como una decisión deliberada, no como un «Aceptar» automático.
- **Archivo cambiado fuera de la herramienta:** `El archivo participantes/participantes.csv cambió fuera de la herramienta mientras estaba abierta. No se guardó el cambio para no pisarlo.` Ofrece dos opciones, «Recargar desde el disco» y «Conservar mi cambio».

### 10. Registro

Una lista cronológica y sobria de lo que pasó en el proyecto, por ejemplo consentimiento registrado, transcripción importada, excepción confirmada o conflicto resuelto. Cada evento muestra la fecha y la hora, el tipo y su detalle, y se puede filtrar por tipo.

## Lo que no hay que diseñar

- Inicio de sesión, cuentas, compartir, notificaciones o nube.
- Asistentes de IA.
- Codificación, notebooks, mapas o escritura, que llegan en versiones siguientes.
- Pantallas de marketing.

## Entregables

1. El sistema de diseño, con tokens, tipografía, etiquetas de estado y componentes, en modo claro y oscuro.
2. Las pantallas 1 a 10 en 1366 × 768, y las principales también en 1920 × 1080.
3. Los estados indicados en cada pantalla: vacío, error, bloqueo, solo lectura y aviso.
4. Una nota breve con las decisiones de diseño y cómo se usan los componentes, para quien implemente la interfaz.
