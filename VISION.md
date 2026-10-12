# Visión

Open Social Research es una herramienta libre para hacer investigación social
aplicada de punta a punta, desde la primera pregunta hasta la publicación. La
idea es que una persona que investiga pueda diseñar su proyecto, salir a campo,
transcribir, analizar con el enfoque que su pregunta pida, escribir y divulgar,
todo en un solo lugar y sin pagar licencias.

Nace dentro del programa de Investigación Social Aplicada (ISA) de la UNED, en
Costa Rica, donde la usa primero su autor con una investigación real sobre la
percepción de inseguridad al caminar en el centro de San José. Cada parte de la
herramienta se va a probar con trabajo de verdad antes de ofrecerse a colegas.

## Por qué

Las herramientas de análisis cualitativo más usadas, como Atlas.ti, MAXQDA o
NVivo, son caras, cerradas y resuelven solo un tramo del trabajo. Quien
investiga termina saltando entre una aplicación para codificar, otra para la
estadística, otra para los mapas, un gestor de referencias, un procesador de
texto y algo más para publicar, y en cada salto se pierde el rastro de cómo se
llegó a cada resultado. En América Latina, además, el costo de las licencias
deja afuera a buena parte de estudiantes, organizaciones y equipos pequeños.

## Principios

**Pluralismo metodológico.** No existe una única «vía óptima» para investigar,
y hay formas de combinar enfoques y métodos (Della Porta y Keating, 2013). La
herramienta no toma partido, sirve igual para la investigación cuantitativa,
la cualitativa y la mixta, y para distintas epistemologías. Ningún módulo
supone que el análisis temático o la regresión son la forma correcta de
trabajar.

**Campo y análisis entrelazados.** Kvale (2011) contrapone al entrevistador
como minero, que extrae datos para analizarlos después, y como viajero, para
quien la entrevista y el análisis son fases entrelazadas. La herramienta
permite las dos maneras, pero su diseño parte de la segunda, lo que se anota en
campo llega al análisis sin exportar ni copiar, y lo que surge del análisis
vuelve a la guía de la próxima salida.

**Combinar datos y métodos.** La triangulación y los métodos mixtos buscan
juntar las fortalezas de distintos diseños (Flick, 2015). Por eso una
entrevista, una tabla, un mapa y un texto de prensa viven en el mismo proyecto
y se pueden cruzar, por ejemplo, ver en el mapa los lugares que mencionan las
personas entrevistadas junto a los datos policiales de esa zona.

**La IA ayuda, quien investiga decide.** La inteligencia artificial transcribe,
sugiere, resume y clasifica, pero no firma. Toda propuesta de la IA queda
marcada como tal hasta que una persona la acepta, y queda registrada. Siguiendo
la idea de vigilancia epistemológica de Bourdieu et al. (2008), la IA también
puede hacer lo contrario de lo cómodo, cuestionar un código, señalar un
fragmento que contradice una categoría o preguntar qué supuesto hay detrás de
una pregunta de la guía.

**Trazabilidad.** Cada resultado debe poder rastrearse hasta sus datos y sus
decisiones. La bitácora del proyecto no es un anexo, es parte del método, y
anota qué se decidió, cuándo y por qué.

**Los datos son de quien investiga y de quienes participan.** Las entrevistas
tienen nombres, voces y lugares de personas reales. El consentimiento, la
anonimización y el resguardo de los datos son funciones de primera clase, y lo
sensible se queda en manos de quien investiga, no en servidores de terceros.

**Formatos abiertos.** Lo que se hace en la herramienta tiene que poder salir
de ella. Se propone soportar el estándar REFI-QDA (archivos `.qdpx`), que ya
usan Atlas.ti, MAXQDA y NVivo, para que un proyecto pueda llegar desde esas
herramientas o irse a ellas.

**Español primero.** Pensada desde y para la investigación en español, con la
puerta abierta a otros idiomas.

## El ciclo de una investigación

La herramienta se organiza según el ciclo de trabajo, no según una lista de
funciones.

### 1. Proyecto y bitácora

Redactar el proyecto (problema, objetivos, diseño, instrumentos, cronograma) y
llevar la bitácora con memos, decisiones y reflexiones a lo largo de toda la
investigación.

### 2. Lecturas y referencias

Integración con Zotero para buscar, citar y armar la bibliografía mientras se
escribe, sin copiar referencias a mano.

### 3. Instrumentos

Guías de entrevista, cuestionarios, hojas de mapa para que las personas marquen
y otros instrumentos, que se diseñan en la herramienta y se llevan a campo o se
imprimen.

### 4. Campo

Aplicación móvil para recolectar en terreno, que funcione sin conexión. Graba
entrevistas, toma fotos y notas, registra ubicaciones y recorridos, aplica
cuestionarios y documenta el consentimiento.

### 5. Datos

Todo llega al mismo proyecto, audios, transcripciones, documentos, tablas,
capas geográficas, imágenes y textos de prensa o redes, con su origen y su
estado de anonimización.

### 6. Transcripción

Transcripción automática de entrevistas en español, que corre en la computadora
de quien investiga, con revisión humana y marcas de tiempo que enlazan el texto
con el audio.

### 7. Análisis

- **Cualitativo**, con codificación, memos, categorías y relaciones entre
  códigos, que es lo que se espera de una alternativa a Atlas.ti.
- **Análisis de textos**, para corpus grandes como noticias o comentarios.
- **Clasificación**, a mano o asistida por IA, con criterios explícitos y
  revisables.
- **Cuantitativo**, con estadística descriptiva e inferencial y visualización
  de datos.
- **Geográfico**, con mapas, densidades y cruces entre lo que dicen las
  personas y dónde ocurre.
- **Integración mixta**, para poner lado a lado resultados de distintos
  métodos.

### 8. Escritura y divulgación

Redactar el informe con citas, figuras y tablas que salen del análisis, y
generar desde el mismo documento el PDF final y un sitio web para divulgar el
proyecto.

## Cómo se construye

Al revés de lo recomendado, se empieza por el techo, primero el nombre y esta
visión, después cada parte, a medida que la investigación real la necesite. Una
función entra cuando resuelve algo que hacía falta en el trabajo de campo o de
análisis, y se pule con ese uso antes de pasar a la siguiente.

Por ahora la usa una sola persona. La meta es que después sea útil para otras
investigadoras e investigadores, en universidades, organizaciones y equipos
pequeños.

## Licencia

El código se publica con licencia MIT. Cualquiera puede usarlo, modificarlo,
redistribuirlo o incorporarlo a otro software, también cerrado, con la única
condición de conservar el aviso de copyright y la licencia. Se eligió así para
que la herramienta llegue lo más lejos posible.

## Decisiones abiertas

- **Tecnología** de la web, de la aplicación móvil y del motor de análisis.
- **Dónde viven los datos**, en la computadora de quien investiga, en un
  servidor propio o en ambos con sincronización.
- **Motor de documentos** para el PDF y el sitio web.
- **Dominio** del proyecto.

## Referencias

- Bourdieu, P., Chamboredon, J.-C. y Passeron, J.-C. (2008). *El oficio de
  sociólogo. Presupuestos epistemológicos*. Siglo XXI.
- Della Porta, D. y Keating, M. (eds.) (2013). *Enfoques y metodologías de las
  ciencias sociales. Una perspectiva pluralista*. Akal.
- Flick, U. (2015). *El diseño de investigación cualitativa*. Morata.
- Kvale, S. (2011). *Las entrevistas en investigación cualitativa*. Morata.
