# Desarrollo

Open Social Research se construye con agentes de IA que escriben el código
bajo supervisión humana. Para que eso produzca software confiable, la revisión
humana no se hace leyendo cada línea de código, sino sobre lo que define qué es
correcto, la especificación, los escenarios de comportamiento y los umbrales
que las pruebas tienen que cumplir. El método se adapta del flujo de trabajo
con agentes que propone Robert C. Martin, y su idea central es subir la capa de
revisión, en lugar de preguntar si el código está bien, preguntar si la
especificación y las restricciones definen bien qué significa que esté bien.

## Los niveles

Cada función recorre los mismos niveles, en orden. Ningún nivel empieza sin que
el anterior esté aprobado.

| Nivel | Qué es | Dónde vive | Quién lo escribe | Quién lo aprueba |
|---|---|---|---|---|
| 0. Visión y producto | Por qué existe la herramienta y qué hace cada versión | `VISION.md`, `PRODUCTO.md` | El autor, con ayuda de IA | El autor |
| 1. Especificación | Comportamiento exacto de un módulo, con entradas, salidas, errores, límites y cifras | `especificaciones/<módulo>.md` | Un agente, a partir de `PRODUCTO.md` | El autor |
| 2. Escenarios | La especificación en Gherkin, en español, como ejemplos verificables de Dado, Cuando y Entonces | `escenarios/<módulo>.feature` | Un agente, a partir de la especificación | El autor |
| 3. Código y pruebas | Implementación en ciclos cortos de desarrollo guiado por pruebas | El código y sus pruebas | Un agente | Las barandas automáticas |
| 4. Barandas | Pruebas, métrica CRAP y pruebas de mutación con umbrales fijos | Configuración del repositorio | Se fijan una vez y cambian solo por decisión registrada | Las herramientas, sin excepciones silenciosas |

### 1. Especificación

Cada requisito de `PRODUCTO.md` que entra a una versión se baja a una
especificación dura antes de escribir código. La especificación dice qué hace
el módulo con precisión suficiente para que dos personas distintas escriban las
mismas pruebas, incluye los casos de error y los límites, usa cifras en lugar
de adjetivos y no dice cómo implementarlo. Cada requisito lleva un
identificador estable, por ejemplo `SEG-03`, que lo sigue en los escenarios,
las pruebas y los commits.

### 2. Escenarios en Gherkin

La especificación se traduce a escenarios en Gherkin escritos en español, con
la línea `# language: es` y las palabras clave Característica, Escenario, Dado,
Cuando y Entonces. Cada escenario lleva la etiqueta del requisito que
verifica.

```gherkin
# language: es
@SEG-03
Característica: El servidor local solo atiende a quien tiene el token

  Escenario: Un pedido sin token es rechazado
    Dado que la herramienta está abierta con un token de sesión
    Cuando llega un pedido a cualquier ruta sin el token
    Entonces la respuesta es 401
    Y el pedido queda registrado como rechazado
```

Este es el punto principal de revisión humana. Un escenario aprobado es un
contrato, y ningún agente lo cambia para que una prueba pase.

### 3. Código guiado por pruebas

Un agente implementa en ciclos cortos, primero una prueba que falla, después el
mínimo código que la hace pasar, después la limpieza con todas las pruebas en
verde. Los escenarios se ejecutan como pruebas de aceptación, y las pruebas
unitarias cubren las piezas internas. Cada commit deja todas las pruebas en
verde.

### 4. Barandas automáticas

En vez de leer miles de líneas, se audita el comportamiento con herramientas y
umbrales fijos.

- **Pruebas.** Todos los escenarios y todas las pruebas unitarias pasan, sin
  pruebas marcadas para saltarse.
- **Métrica CRAP.** Combina la complejidad ciclomática de cada función con su
  cobertura de pruebas, y crece cuando una función es a la vez compleja y poco
  probada (Savoia y Evans, 2007). Ninguna función supera 30, el umbral original
  de la métrica. Con complejidad mayor que 30 ninguna cobertura alcanza, así
  que esas funciones se dividen.
- **Pruebas de mutación.** La herramienta introduce cambios pequeños a
  propósito en el código, como invertir una condición o devolver un valor por
  defecto, y comprueba que alguna prueba falle. Un mutante que sobrevive señala
  lógica que ninguna prueba vigila. En el código nuevo no puede sobrevivir
  ningún mutante sin una justificación escrita en la configuración.
- **Estilo y advertencias.** El formateador y el analizador estático del
  lenguaje pasan sin advertencias.

### Cómo se corren

`bin/barandas @PRY-01` corre todo para el requisito en curso, el formateador,
el analizador, las pruebas unitarias, los escenarios de esa etiqueta en modo
estricto (un paso sin definir es una falla), la cobertura con la métrica CRAP,
la mutación sobre lo que cambió respecto de `origin/main` y, al final, el
avance de todos los escenarios. Se corre después de commitear y antes del push,
y la integración continua corre lo mismo en cada push y en cada pull request.

- Los escenarios de requisitos todavía no implementados no son una falla, sus
  pasos sin definir cuentan como omitidos. El avance se lee en la última línea.
- Un mutante que no se puede matar sin debilitar un escenario se justifica con
  `#[mutants::skip]` y un comentario `// porque:` al lado, con la razón.
- Los escenarios etiquetados `@lento`, como los de archivos de varios
  gigabytes, corren en la integración continua y no en cada ejecución local.
  Los archivos grandes de prueba son archivos dispersos, creados con su tamaño
  sin ocupar el disco.

## Herramientas

| Parte | Escenarios | Pruebas | Cobertura y CRAP | Mutación |
|---|---|---|---|---|
| Orquestador en Rust | `cucumber` (cucumber-rs), que lee Gherkin en español | `cargo test` | `cargo llvm-cov` y `cargo-crap` | `cargo-mutants` |
| Interfaz en Svelte | `playwright-bdd` sobre Playwright | Vitest | Cobertura de Vitest | StrykerJS con su ejecutor de Vitest |
| Paquete de R | Por definir cuando exista | testthat | covr | Por definir |

Las versiones exactas se fijan al introducir cada herramienta, en el archivo de
bloqueo correspondiente. Cuando el repositorio tenga código, las barandas
corren también en la integración continua de GitHub, de modo que nada entra a
`main` sin pasarlas.

## Reglas para los agentes

- No se escribe código sin escenarios aprobados para lo que se implementa.
- Nunca se debilita una prueba, se borra un escenario, se baja un umbral o se
  agrega una excepción para que algo pase. Si un umbral parece mal puesto, se
  propone el cambio y lo decide el autor.
- Los cambios de umbral, de herramienta o de especificación aprobada se anotan
  en `docs/decisiones.md` con fecha, razón y alternativas.
- Cada commit lleva en el asunto el identificador del requisito que avanza,
  sigue Conventional Commits en español y no menciona a la IA.
- Lo que no está especificado no se implementa por iniciativa propia. Se anota
  como pregunta para el autor.

## Qué revisa el autor

El autor aprueba especificaciones y escenarios, fija y cambia umbrales, y mira
los informes de las barandas. Puede leer el código cuando quiera, pero el
método no depende de eso. Una función está terminada cuando sus escenarios
aprobados pasan, las barandas están en verde y el autor la usó en trabajo
real.

## Referencias

- Martin, R. C. (2026). *swarm-forge* [Software]. GitHub.
  https://github.com/unclebob/swarm-forge
- Savoia, A. y Evans, B. (2007). *Crap4j* [Software y métrica]. La métrica CRAP
  se define como comp(m)² × (1 − cov(m))³ + comp(m), con umbral 30.
