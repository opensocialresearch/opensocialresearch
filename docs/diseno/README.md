# Diseño de la interfaz

El diseño de la interfaz se hizo en Claude Design el 2026-10-12, a partir de
[prompt-diseno.md](../prompt-diseno.md). Vive en el proyecto
https://claude.ai/design/p/b1686b60-6c9c-4724-9961-8d79fb5e93f2 y no se copia
acá, porque cada archivo depende del entorno de Claude Design (`support.js` y
sus componentes `dc-import`).

Toda página que sirve el programa lo usa desde la primera versión (D-40). No
es contrato: si el diseño y un escenario aprobado no coinciden, manda el
escenario. La hoja de estilos con los tokens está en `recursos/estilos.css` y
la fuente Source Serif 4 en `recursos/fuentes/`.

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
| `Bienvenida.dc.html` | Bienvenida del primer uso (PRY-23) |
| `PrimerProyecto.dc.html` | Crear el primer proyecto o abrir uno existente (PRY-23) |
| `Inicio.dc.html` | Inicio con el resultado de la verificación al abrir, y el proyecto vacío que empieza por el diseño de la investigación (PRY-22) |
| `Diseno.dc.html`, `DisenoPaso.dc.html`, `DisenoMatriz.dc.html`, `DisenoHistorial.dc.html` | Diseño de la investigación: vista general, un paso, matriz de coherencia e historial (D-49) |
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

Rehecho en Claude Design el 2026-10-12 y adoptado en D-51: una herramienta
de trabajo con la claridad de una app de escritorio.

**Color.** Ventana gris claro con tarjetas blancas, un azul para actuar y
color semántico que siempre acompaña al texto y al ícono. Todo va como
variables CSS en `:root` y en `[data-tema="oscuro"]` (ver
`recursos/estilos.css`), y ningún color se escribe fuera de los tokens.

| Token | Claro | Uso |
|---|---|---|
| `--fondo` | `#F5F5F7` | ventana |
| `--superficie` | `#FFFFFF` | tarjetas, paneles |
| `--relleno` | `rgba(120,120,128,.12)` | botón secundario, avisos |
| `--tinta`, `--tinta-2`, `--tinta-3` | `#1D1D1F`, `#515154`, `#6E6E73` | texto, secundario, metadatos |
| `--separador` | `rgba(60,60,67,.14)` | líneas de filas |
| `--borde-control` | `#8E8E93` | campos |
| `--acento`, `--acento-relleno` | `#0066CC`, `#0071E3` | enlaces, botón principal |
| `--foco` | `#3A86D1` | anillo de foco |
| `--exito`, `--aviso`, `--error` | `#1E7B34`, `#9A4A00`, `#D70015` | hecho, revisar, error |
| `--ident` | `#5856D6` | zona identificable |

**Tipografía.** La interfaz usa la fuente del sistema (`--ui`). Source Serif 4
a 16,5 px con interlínea 1,55 y unos 72 caracteres de ancho para lo que se
escribe y se lee largo (transcripciones, diseño de la investigación), servida
local en WOFF2. Una mono del sistema (`--mono`) para tiempos, rutas y atajos.
Escala: marca 40/700 (solo la bienvenida), título 26/700 una vez por vista,
sección 15/600, cuerpo 13, ayuda 12.

**Espaciado y radios.** Retícula de 4 px. Barra de herramientas de 52 px,
controles de 30 a 32 px. Radios concéntricos: cápsula 11, control 7, fila 10,
tarjeta 12, diálogo 14. Solo lo flotante lleva sombra.

**Zona identificable.** Índigo con candado: cápsula sólida para la zona,
tinte para la sensibilidad. No usa rojo, porque es una zona y no un error.

**Etiquetas de estado.** Siempre texto e ícono, y la forma del ícono lleva el
significado (✓ hecho, ○ pendiente, ◐ en curso, △ revisar, candado
identificable). Componente `Etiqueta` con una clave por estado.

**Componentes.** Un botón principal azul por vista, al final a la derecha del
grupo. Secundario en relleno gris. El peligroso solo dentro del diálogo de
confirmación. Listas y tablas en tarjetas, sin cebra, con separadores con
sangría de 16 px.

**Tema.** Claro por defecto, sin seguir la preferencia del sistema (D-41,
que se mantiene aunque la nota del diseño proponga seguirla). El oscuro solo
con `data-tema="oscuro"`.

**Notas para implementar.** El estado de guardado va en la barra con
`role="status"`. Los 1000 segmentos se renderizan todos con
`content-visibility: auto`. Los atajos usan Ctrl+Alt y Esc reproduce o pausa.
Un bloqueo nunca ofrece «Aceptar», y la excepción pide un motivo de 20
caracteres o más. Sin animaciones, respetando `prefers-reduced-motion`. Cada
pantalla del diseño termina con un comentario «Clases para
recursos/estilos.css» con las clases que propone.
