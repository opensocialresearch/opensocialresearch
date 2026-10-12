# Diseño de la interfaz

El diseño de la interfaz se hizo en Claude Design el 2026-10-12, a partir de
[prompt-diseno.md](../prompt-diseno.md). Vive en el proyecto
https://claude.ai/design/p/b1686b60-6c9c-4724-9961-8d79fb5e93f2 y no se copia
acá, porque cada archivo depende del entorno de Claude Design (`support.js` y
sus componentes `dc-import`).

Toda página que sirve el programa lo usa desde la primera versión (D-40). No
es contrato: si el diseño y un escenario aprobado no coinciden, manda el
escenario. La hoja de estilos con los tokens está en `recursos/estilos.css` y
las fuentes en `recursos/fuentes/`.

## Cómo leerlo

Desde una sesión de Claude Code con acceso de diseño (`/design-login`), la
herramienta DesignSync lee cada archivo con `get_file` y el id del proyecto.
Los subagentes no tienen esa herramienta, así que la sesión principal les pasa
el contenido que necesiten.

| Archivo | Qué tiene |
|---|---|
| `Pantallas.dc.html` | Entrada principal, todas las pantallas en 1366 × 768 y 1920 × 1080, en claro y oscuro |
| `Sistema de diseño.dc.html` | Tokens, tipografía, espaciado, etiquetas de estado, componentes y notas para implementar |
| `Marco.dc.html` | Marco general que monta cada pantalla según `pantalla`, `tema`, `guardado`, `dialogo` y `variante` |
| `Inicio.dc.html` | Inicio con el resultado de la verificación al abrir, y el proyecto vacío |
| `Participantes.dc.html` | Lista de participantes |
| `Ficha.dc.html` | Ficha de una persona con el panel «Registrar consentimiento» |
| `Correspondencia.dc.html` | Tabla de correspondencia, en la zona identificable |
| `Archivos.dc.html` | Archivos del proyecto con su zona |
| `Importar.dc.html` | Importar a una entrevista en tres pasos |
| `Editor.dc.html` | Editor de revisión de transcripciones |
| `BarraAudio.dc.html` | Barra de reproducción fija del editor |
| `Atajos.dc.html` | Panel de atajos (Ctrl+K) |
| `Dialogo.dc.html` | Diálogos de confirmación y de excepción |
| `Etiqueta.dc.html` | Componente de etiqueta de estado |
| `Registro.dc.html` | Registro de eventos filtrable |
| `support.js` | Entorno de ejecución de Claude Design |

## Resumen del sistema de diseño

**Color.** Negro sobre papel, líneas finas y un solo acento. Todo va como
variables CSS en `:root` y en `[data-tema="oscuro"]`, y ningún color se escribe
fuera de los tokens.

| Token | Claro | Oscuro | Uso |
|---|---|---|---|
| `--papel` | `#FBFAF7` | `#161513` | fondo |
| `--superficie` | `#FFFFFF` | `#1E1D1A` | paneles, diálogos |
| `--hundido` | `#F1EFEA` | `#27251F` | avisos, fila activa |
| `--tinta` | `#1A1916` | `#ECE9E2` | texto, botón principal |
| `--tinta-2` | `#4A4741` | `#BEB9AF` | texto secundario |
| `--tinta-3` | `#67635B` | `#9C978D` | metadatos |
| `--linea` | `#DDD9D1` | `#36332D` | separadores |
| `--linea-fuerte` | `#8A857B` | `#7A766C` | bordes de controles |
| `--acento` | `#2350A8` | `#8FB2F2` | foco, segmento que suena, progreso |
| `--acento-suave` | `#E5ECF8` | `#1E2A42` | fondo del segmento que suena |
| `--sobre-acento` | `#FFFFFF` | `#0F1420` | texto sobre acento |
| `--error` | `#B3261E` | `#F09A90` | error, peligro |
| `--error-suave` | `#FBEAE7` | `#3A1E1B` | fondo de error |
| `--sobre-error` | `#FFFFFF` | `#1A0D0B` | texto sobre error |
| `--trama` | `rgba(26,25,22,.22)` | `rgba(236,233,226,.24)` | trama de la zona identificable |
| `--velo` | `rgba(26,25,22,.42)` | `rgba(0,0,0,.62)` | fondo de diálogos |

**Tipografía.** Archivo semicondensada (`font-stretch: 87.5%`) para la
interfaz, Source Serif 4 a 16,5 px con interlínea 1,55 y 72 caracteres de ancho
para las transcripciones, IBM Plex Mono para tiempos, rutas y atajos. Las tres
se sirven locales en WOFF2, sin CDN, para que la app funcione sin internet.
Escala, 28 display, 22 título, 18 título de diálogo, 13 cuerpo, 12,5 pequeño,
12 etiqueta y 11,5 rótulo en versalitas.

**Espaciado y bordes.** Base de 4 px (4, 8, 12, 16, 20, 24, 32, 48). Filas de
tabla de 36 a 44 px, controles de 28 o 32 px. Radios de 2, 3 y 4 px. El foco es
un contorno de 2 px de acento con 2 px de separación.

**Zona identificable.** No usa color. Usa la etiqueta invertida con rombo (◆)
y, si es una vista entera, un marco con trama a −45° cada 6 px.

**Etiquetas de estado.** Siempre texto e ícono, y la forma del ícono lleva el
significado (✓ hecho, ○ pendiente, ◐ en curso, △ revisar, ◆ identificable,
▢ trabajo, – ausente). Un solo componente `Etiqueta` con una clave por estado
(`cons-ok`, `tr-enrev`, `audio-falta`…), con el ícono oculto a los lectores de
pantalla.

**Componentes.** Un botón principal por vista, en tinta llena y con verbo en
infinitivo. Secundario con contorno. Peligroso en rojo, siempre con diálogo que
repite la consecuencia. Inactivo con borde punteado y el motivo al lado. Tablas
densas sin cebra, con números y rutas en mono a la derecha. Panel lateral de
440 a 480 px sin velo. Barra de reproducción fija abajo del editor.

**Notas para implementar.** El estado de guardado va en el encabezado con
`role="status"`. Los 1000 segmentos se renderizan todos, sin virtualizar, con
`content-visibility: auto`. Los atajos usan Ctrl+Alt y Esc reproduce o pausa.
Un bloqueo nunca ofrece «Aceptar», y la excepción pide un motivo de 20
caracteres o más. Sin animaciones, respetando `prefers-reduced-motion`.
