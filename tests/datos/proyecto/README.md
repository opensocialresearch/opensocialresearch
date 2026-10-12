<!-- Generado por Open Social Research. Los cambios a mano se guardan aparte y este archivo se reemplaza. -->

# Proyecto de prueba inventado

## Qué es esta carpeta

Es el proyecto «Proyecto de prueba inventado» de Open Social Research, en la versión 1 del
formato de carpeta. Este README se generó el 2026-10-05. Todo lo que hay aquí se
puede leer sin la herramienta, con un editor de texto, una hoja de cálculo o R.

## Carpetas

| Carpeta | Qué contiene | Zona |
|---|---|---|
| `./` | La identidad del proyecto, este README y las marcas de sensibilidad | de trabajo |
| `bitacora/` | La bitácora de quien investiga y el registro de eventos de la herramienta | de trabajo |
| `participantes/` | El registro de participantes por seudónimo y sus consentimientos | de trabajo |
| `identificable/` | Todo lo que identifica a una persona | identificable |
| `identificable/consentimientos/` | Respaldos de los consentimientos con sus fichas | identificable |
| `identificable/entrevistas/` | Una carpeta por entrevista, con su audio y su transcripción | identificable |
| `.osr/` | El índice interno, que se puede borrar porque se reconstruye | interna |

## Tipos de archivo

| Archivo | Formato | Qué contiene | Cómo se abre sin la herramienta | Especificación |
|---|---|---|---|---|
| `proyecto.toml` | TOML 1.0 | Nombre, versión del formato y configuración | Editor de texto | PRY-03 |
| `README.md` | Markdown | Esta explicación | Editor de texto | PRY-16 |
| `sensibilidad.csv` | CSV | Marcas de sensibilidad asignadas a mano | Hoja de cálculo o R | PAR-14 |
| `bitacora/bitacora.md` | Markdown | Entradas fechadas de quien investiga | Editor de texto | PRY-08 |
| `bitacora/registro.jsonl` | JSON Lines | Un evento de la herramienta por línea | Editor de texto | PRY-08 |
| `participantes/participantes.csv` | CSV | Participantes por seudónimo | Hoja de cálculo o R | PAR-02 |
| `participantes/consentimientos.csv` | CSV | Consentimiento de cada participante por alcance | Hoja de cálculo o R | PAR-07 |
| `identificable/correspondencia.csv` | CSV | La tabla que liga cada seudónimo con la persona | Hoja de cálculo o R | PAR-03 |
| `.osr/indice.sqlite` | SQLite | Copia derivada de los archivos de texto | SQLite | PRY-10 |

## Zonas y sensibilidad

La zona identificable es todo lo que está bajo `identificable/` y la zona de
trabajo es todo lo demás salvo `.osr/`. La zona identificable solo se copia a
un disco cifrado.

Cada archivo tiene uno de cuatro niveles de sensibilidad. `identificable` es
todo archivo bajo `identificable/`, `sensible` son los archivos de la zona de
trabajo con datos de participantes, `no_sensible` son `proyecto.toml` y
`README.md`, y `sin_marca` son los demás archivos de la zona de trabajo hasta
que se marquen.

| Archivo | Nivel |
|---|---|
| `proyecto.toml` | no_sensible |
| `README.md` | no_sensible |
| `sensibilidad.csv` | sensible |
| `bitacora/bitacora.md` | sensible |
| `bitacora/registro.jsonl` | sensible |
| `participantes/participantes.csv` | sensible |
| `participantes/consentimientos.csv` | sensible |
| `identificable/correspondencia.csv` | identificable |

## Cómo respaldar

Para copiar la carpeta entera a un disco cifrado, conservando los permisos:

    rsync -a ./ /ruta/al/disco/cifrado/proyecto/

Para copiar todo salvo la zona identificable a un destino sin cifrar:

    rsync -a --exclude identificable/ ./ /ruta/al/respaldo/proyecto/

La carpeta `.osr/` se puede omitir en cualquier respaldo, porque la
herramienta la reconstruye.

## Qué no hacer

- No guardar esta carpeta en servicios de sincronización en la nube.
- No editar los archivos de `origen/` ni los originales.
- Borrar un archivo en un disco SSD no garantiza que sea irrecuperable, por lo
  que el disco tiene que estar cifrado.
