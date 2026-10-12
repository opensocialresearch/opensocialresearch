"""Genera los datos de prueba inventados de PRY-17, salvo el audio.

uso: generar.py
     generar.py --comprobar-base <carpeta de un proyecto recién creado con osr nuevo>

Lee ../sueltos/tiempos-E01.json y ../sueltos/tiempos-E02.json, que escribe
sintetizar.py junto con el audio, y escribe

- en ../sueltos/ los archivos que las pruebas importan, la cadena del autor de
  cada entrevista (whisper.json, hablantes.json, md, encabezado.md) con el
  formato exacto de transcribir.py, hablantes.py y markdown.py, las versiones
  WebVTT y JSON de Whisper, los archivos de sospechas y los inválidos de
  TRA-04 y TRA-05, el audio sin duración declarada y las imágenes,
- en ../proyecto/ el proyecto de prueba poblado, como lo deja la app después
  de registrar a E01 y E02 e importar su audio y su cadena,
- y comprueba al final lo que se puede comprobar sin la app.

Solo usa la biblioteca estándar y ffmpeg (para codificar los JPEG). Es
determinista: no usa el reloj ni el azar, todas las fechas son fijas e
inventadas, y correrlo dos veces deja los mismos bytes con la misma versión de
ffmpeg (los JPEG se codifican con -fflags +bitexact).

La base del proyecto (proyecto.toml, README.md, bitacora/bitacora.md, los
encabezados de los CSV y la línea proyecto_creado) es copia de lo que escribió
`osr nuevo` (commit fee0f7c) con la fecha de creación cambiada por una fija.
Si la app cambia esas plantillas, `--comprobar-base` lo detecta comparando con
un proyecto recién creado.
"""
import hashlib
import json
import re
import shutil
import struct
import subprocess
import sys
import tomllib
import unicodedata
import zlib
from pathlib import Path

AQUI = Path(__file__).resolve().parent
DATOS = AQUI.parent
SUELTOS = DATOS / "sueltos"
PROYECTO = DATOS / "proyecto"

ZONA = "-06:00"
NOMBRE_PROYECTO = "Proyecto de prueba inventado"
CREADO = "2026-10-05T09:00:00" + ZONA
FECHA_README = "2026-10-05"
MODELO = "large-v3-turbo"

# Mapa de markdown.py para cada entrevista, en el orden en que lo escribe el
# autor, y los roles que propone TRA-08 a partir de esas etiquetas.
MAPAS = {"E01": "0=Entrevistador,1=E01", "E02": "1=Entrevistador,0=E02"}
ROLES = {"Entrevistador": "entrevistadora", "E01": "entrevistada", "E02": "entrevistada"}

# Datos inventados de cada entrevista. El segundo conjunto de nombres de
# PRY-17 (Nombre Inventado, Carla Supuesta Vega) solo aparece en los nombres
# originales de los archivos importados.
ENTREVISTAS = {
    "E01": {
        "audio": "E01.ogg",
        "formato_audio": "ogg",
        "nombre": "Ana Inventada Muñoz",
        "contacto": "ana.munoz@ejemplo.invalid",
        "nota": "Prefiere conversar por la tarde",
        "fecha": "2026-10-03",
        "lugar": "Parque Inventado Las Gaviotas",
        "audio_original": "grabación Carla Supuesta Vega.ogg",
        "prefijo_original": "Carla Supuesta Vega",
        "consentimiento": ["v1, octubre", "2026-10-03", "escrito", "ninguno", "", "",
                           "otorgado", "otorgado", "no_consultado", "otorgado", "denegado"],
        "encabezado": (
            "# Entrevista E01\n"
            "\n"
            "Fecha: viernes 3 de octubre de 2026, a media tarde.\n"
            "Lugar: Parque Inventado Las Gaviotas, en una banca junto a los juegos.\n"
            "Persona entrevistada: Ana Inventada Muñoz, vecina desde hace doce años.\n"
            "Nota de campo: cielo nublado, poca gente en el parque; ficha inventada para pruebas.\n"
        ),
    },
    "E02": {
        "audio": "E02.m4a",
        "formato_audio": "m4a",
        "nombre": "Bruno Ficticio Ramírez",
        "contacto": "bruno.ramirez@ejemplo.invalid",
        "nota": "Avisar antes por mensaje, trabaja en turno de noche",
        "fecha": "2026-10-04",
        "lugar": "Parada Ficticia del Puente Viejo",
        "audio_original": "entrevista con Nombre Inventado.m4a",
        "prefijo_original": "Nombre Inventado",
        "consentimiento": ["v1, octubre", "2026-10-04", "oral_grabado", "ninguno", "", "",
                           "otorgado", "denegado", "no_consultado", "otorgado", "no_consultado"],
        "encabezado": (
            "# Entrevista E02\n"
            "\n"
            "Fecha: sábado 4 de octubre de 2026, de noche.\n"
            "Lugar: Parada Ficticia del Puente Viejo, mientras esperaba el bus.\n"
            "Persona entrevistada: Bruno Ficticio Ramírez, usuario diario de esa parada.\n"
            "Nota de campo: entrevista corta por la hora; ficha inventada para pruebas.\n"
        ),
    },
}

# Historia fija de las dos sesiones de trabajo, con la marca de tiempo de cada
# acción. Las marcas de las importaciones dan nombre a origen/<marca>/.
SESION = {
    "E01": {"participante": "2026-10-05T09:04:10", "correspondencia": "2026-10-05T09:05:02",
            "consentimiento": "2026-10-05T09:07:45", "entrevista": "2026-10-06T10:02:15",
            "audio": "2026-10-06T10:04:40", "transcripcion": "2026-10-06T10:15:00"},
    "E02": {"participante": "2026-10-05T09:10:20", "correspondencia": "2026-10-05T09:11:03",
            "consentimiento": "2026-10-05T09:13:30", "entrevista": "2026-10-06T10:20:05",
            "audio": "2026-10-06T10:22:30", "transcripcion": "2026-10-06T10:30:00"},
}
APERTURA = "2026-10-06T10:00:00"  # osr abrir del segundo día, que anota verificacion

ALCANCES = ["grabar", "citar_textualmente", "usar_puntos_mapa", "archivar", "reutilizar"]
UMBRALES = {"no_speech_prob": 0.6, "avg_logprob": -1.0, "compression_ratio": 2.4}

# Expresión de las líneas de intervención de TRA-04.
INTERVENCION = re.compile(r"^\*\*\[(\d{2,}):(\d{2})\] (.+?):\*\* (.*)$")

# ---------------------------------------------------------------------------
# Base copiada de `osr nuevo` (commit fee0f7c), con la fecha fija.

BASE_PROYECTO_TOML = """\
formato = "osr-proyecto"
version_formato = 1
nombre = "Proyecto de prueba inventado"
creado = 2026-10-05T09:00:00-06:00

[[consentimiento.alcances]]
id = "grabar"
etiqueta = "Grabar la entrevista"

[[consentimiento.alcances]]
id = "citar_textualmente"
etiqueta = "Citar textualmente sus palabras"

[[consentimiento.alcances]]
id = "usar_puntos_mapa"
etiqueta = "Usar sus puntos del mapa"

[[consentimiento.alcances]]
id = "archivar"
etiqueta = "Archivar los datos al terminar"

[[consentimiento.alcances]]
id = "reutilizar"
etiqueta = "Reutilizar los datos en otras investigaciones"
"""

BASE_BITACORA_MD = """\
# Bitácora

Esta bitácora es de quien investiga y Open Social Research no la vuelve a
escribir. Cada entrada empieza con un título de nivel dos con la fecha y el
tipo de entrada, que es uno de estos cuatro, decisión metodológica, reflexión,
analítica o de campo, por ejemplo `## 2026-10-11 · reflexión`. Lo que la
herramienta hace queda anotado aparte, en `registro.jsonl`.
"""

BASE_README_MD = """\
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
"""

BASE_SENSIBILIDAD_CSV = "ruta,nivel\r\n"
BASE_PARTICIPANTES_CSV = "seudonimo,fecha_alta,estado\r\n"
BASE_CORRESPONDENCIA_CSV = "seudonimo,nombre,contacto,nota\r\n"
BASE_CONSENTIMIENTOS_CSV = (
    "seudonimo,version_formulario,fecha,modalidad,respaldo,respaldo_inicio_s,respaldo_fin_s,"
    + ",".join(f"alcance_{a}" for a in ALCANCES) + "\r\n"
)


def base_registro():
    return evento(CREADO, "proyecto_creado", {})


# ---------------------------------------------------------------------------
# Utilidades


def escribir(ruta, contenido):
    """Escribe texto en UTF-8 sin traducir los fines de línea, o bytes."""
    ruta.parent.mkdir(parents=True, exist_ok=True)
    if isinstance(contenido, str):
        contenido = contenido.encode("utf-8")
    ruta.write_bytes(contenido)


def sha256(datos):
    return hashlib.sha256(datos).hexdigest()


def evento(fecha, tipo, detalle):
    """Una línea de bitacora/registro.jsonl como la escribe serde_json."""
    linea = json.dumps({"fecha": fecha, "tipo": tipo, "detalle": detalle},
                       ensure_ascii=False, separators=(",", ":"))
    return linea + "\n"


def marca_compacta(momento):
    """2026-10-06T10:15:00 → 20261006T101500 (PRY-02)."""
    return momento.replace("-", "").replace(":", "")


def mmss(t):
    """Igual que markdown.py."""
    return f"{int(t // 60):02d}:{int(t % 60):02d}"


def vtt_tiempo(t):
    """mm:ss.ttt como format_timestamp de Whisper sin horas."""
    ms = round(t * 1000)
    m, ms = divmod(ms, 60_000)
    s, ms = divmod(ms, 1000)
    return f"{m:02d}:{s:02d}.{ms:03d}"


def normalizar(texto):
    """Texto normalizado de la regla repeticion de TRA-14."""
    texto = unicodedata.normalize("NFC", texto).lower()
    texto = re.sub(r"[.,;:¡!¿?…\"«»'()\-]", "", texto)
    return re.sub(r"\s+", " ", texto).strip()


def repetidos(textos):
    """Índices de los segmentos que pertenecen a una serie repetida (TRA-14)."""
    marcados = set()
    norm = [normalizar(t) for t in textos]
    for i in range(1, len(norm)):
        if norm[i] and norm[i] == norm[i - 1]:
            marcados.update((i - 1, i))
    return marcados


def cadena_toml(texto):
    return '"' + texto.replace("\\", "\\\\").replace('"', '\\"') + '"'


def campo_csv(texto):
    """RFC 4180 con comillas solo si hacen falta (PRY-06)."""
    if any(c in texto for c in ',"\r\n'):
        return '"' + texto.replace('"', '""') + '"'
    return texto


def fila_csv(campos):
    return ",".join(campo_csv(c) for c in campos) + "\r\n"


# ---------------------------------------------------------------------------
# Cadena del autor


def leer_tiempos(codigo):
    return json.loads((SUELTOS / f"tiempos-{codigo}.json").read_text(encoding="utf-8"))


def whisper_cadena(tiempos):
    """Lo que escribe transcribir.py, un segmento por intervención.

    faster-whisper redondea los tiempos de palabra a 2 decimales y, con
    word_timestamps, el segmento empieza en su primera palabra y termina en la
    última. Aquí cada palabra ocupa una parte de la intervención proporcional a
    su largo, y `w` lleva el espacio inicial que le pone Whisper.
    """
    segmentos = []
    for it in tiempos["intervenciones"]:
        fichas = it["texto"].split(" ")
        total = sum(len(f) for f in fichas)
        dur = it["fin"] - it["inicio"]
        palabras, acumulado = [], 0
        for f in fichas:
            s = round(it["inicio"] + dur * acumulado / total, 2)
            acumulado += len(f)
            e = round(it["inicio"] + dur * acumulado / total, 2)
            assert e > s, (it, f)
            palabras.append({"s": s, "e": e, "w": " " + f})
        segmentos.append({"start": palabras[0]["s"], "end": palabras[-1]["e"],
                          "text": it["texto"].strip(), "words": palabras})
    return segmentos


def volcar_cadena(objeto):
    """json.dump(..., ensure_ascii=False, indent=1) como transcribir.py y hablantes.py."""
    return json.dumps(objeto, ensure_ascii=False, indent=1)


def hablantes_cadena(segmentos, turnos):
    """Port literal del pegado de palabras de hablantes.py."""

    def hablante(s, e):
        cub = {}
        for ts, te, sp in turnos:
            o = min(e, te) - max(s, ts)
            if o > 0:
                cub[sp] = cub.get(sp, 0) + o
        if cub:
            return max(cub, key=cub.get)
        m = (s + e) / 2
        return min(turnos, key=lambda t: min(abs(m - t[0]), abs(m - t[1])))[2]

    frases = []
    for seg in segmentos:
        for w in seg["words"] or [{"s": seg["start"], "e": seg["end"], "w": seg["text"]}]:
            sp = hablante(w["s"], w["e"])
            if frases and frases[-1]["sp"] == sp and w["s"] - frases[-1]["end"] < 1.5:
                frases[-1]["text"] += w["w"]
                frases[-1]["end"] = w["e"]
            else:
                frases.append({"sp": sp, "start": w["s"], "end": w["e"], "text": w["w"]})
    for f in frases:
        f["text"] = f["text"].strip()
    return {"turnos": [list(t) for t in turnos], "frases": frases}


def markdown_cadena(hablantes, codigo, mapa, encabezado):
    """Port literal de markdown.py."""
    enc = encabezado.rstrip() + "\n\n" if encabezado is not None else f"# Entrevista {codigo}\n\n"
    nombres = dict(p.split("=") for p in mapa.split(","))
    lineas = [f"**[{mmss(f['start'])}] {nombres.get(str(f['sp']), 'Hablante ' + str(f['sp']))}:** {f['text']}"
              for f in hablantes["frases"]]
    return enc + "\n\n".join(lineas) + "\n"


# ---------------------------------------------------------------------------
# Otras salidas de Whisper


def metricas(i):
    """Métricas plausibles y por debajo de los umbrales, fijas por posición."""
    return {"avg_logprob": round(-0.18 - 0.021 * (i % 7), 4),
            "compression_ratio": round(1.12 + 0.031 * (i % 9), 4),
            "no_speech_prob": round(0.012 + 0.0047 * (i % 5), 4)}


def whisper_openai(segmentos, quitar=()):
    """JSON de la CLI de Whisper de OpenAI: json.dump(result, f), con escapes
    \\u para lo no ASCII, `text` con el espacio inicial y las métricas. Se
    omiten `tokens` porque ninguna regla los usa."""
    salida = []
    for i, (inicio, fin, texto, m) in enumerate(segmentos):
        seg = {"id": i, "seek": int(inicio // 30) * 3000, "start": inicio, "end": fin,
               "text": " " + texto, "temperature": 0.0}
        for clave in ("avg_logprob", "compression_ratio", "no_speech_prob"):
            if clave not in quitar:
                seg[clave] = m[clave]
        salida.append(seg)
    resultado = {"text": "".join(s["text"] for s in salida), "segments": salida, "language": "es"}
    return json.dumps(resultado)


def vtt(segmentos):
    """WebVTT como lo escribe la CLI de Whisper, sin voces ni identificadores."""
    partes = ["WEBVTT\n\n"]
    for inicio, fin, texto in segmentos:
        partes.append(f"{vtt_tiempo(inicio)} --> {vtt_tiempo(fin)}\n{texto}\n\n")
    return "".join(partes)


# ---------------------------------------------------------------------------
# Audio sin duración declarada


def crc_ogg(datos, _tabla=[]):
    if not _tabla:
        for i in range(256):
            r = i << 24
            for _ in range(8):
                r = ((r << 1) ^ 0x04C11DB7) if r & 0x80000000 else (r << 1)
            _tabla.append(r & 0xFFFFFFFF)
    c = 0
    for b in datos:
        c = ((c << 8) ^ _tabla[((c >> 24) ^ b) & 0xFF]) & 0xFFFFFFFF
    return c


def ogg_sin_duracion(datos):
    """El audio de E01 con el contenedor alterado para que no declare duración.

    En Ogg Opus la duración no está escrita en ningún encabezado, se deduce de
    la posición de gránulo (muestras a 48 kHz) de la última página, menos el
    pre-skip. Aquí se pone en 0 el gránulo de todas las páginas de audio (de la
    tercera en adelante; las dos primeras son OpusHead y OpusTags y ya tienen
    0) y se recalcula el CRC de cada página, que cubre la página entera con el
    campo del CRC en cero. Poner en 0 solo la última no alcanza, porque ffprobe
    retrocede a la página anterior con gránulo válido, y -1 hace que ffmpeg no
    pueda decodificar. Con todo en 0 el audio se decodifica completo pero
    ffprobe da duration=N/A, y un lector que use el gránulo de la última página
    obtiene 0 - pre-skip < 0, o sea duración desconocida.
    """
    datos = bytearray(datos)
    paginas, i = [], 0
    while i < len(datos):
        assert datos[i:i + 4] == b"OggS"
        n = datos[i + 26]
        largo = 27 + n + sum(datos[i + 27:i + 27 + n])
        paginas.append((i, largo))
        i += largo
    for o, largo in paginas[2:]:
        struct.pack_into("<q", datos, o + 6, 0)
        struct.pack_into("<I", datos, o + 22, 0)
        struct.pack_into("<I", datos, o + 22, crc_ogg(datos[o:o + largo]))
    return bytes(datos)


# ---------------------------------------------------------------------------
# Imágenes


def ppm(ancho, alto, pixel):
    filas = bytearray()
    for y in range(alto):
        for x in range(ancho):
            filas += bytes(pixel(x, y))
    return b"P6 %d %d 255\n" % (ancho, alto) + bytes(filas)


def png(ancho, alto, pixel):
    """PNG RGB de 8 bits, sin dependencias."""
    crudo = bytearray()
    for y in range(alto):
        crudo.append(0)
        for x in range(ancho):
            crudo += bytes(pixel(x, y))

    def trozo(tipo, datos):
        return struct.pack(">I", len(datos)) + tipo + datos + struct.pack(">I", zlib.crc32(tipo + datos))

    return (b"\x89PNG\r\n\x1a\n"
            + trozo(b"IHDR", struct.pack(">IIBBBBB", ancho, alto, 8, 2, 0, 0, 0))
            + trozo(b"IDAT", zlib.compress(bytes(crudo), 9))
            + trozo(b"IEND", b""))


def jpeg(ppm_bytes):
    """Codifica un PPM a JPEG con ffmpeg en modo bitexact y sin metadatos."""
    r = subprocess.run(
        ["ffmpeg", "-v", "error", "-f", "image2pipe", "-c:v", "ppm", "-i", "-",
         "-fflags", "+bitexact", "-flags", "+bitexact", "-map_metadata", "-1",
         "-q:v", "5", "-f", "mjpeg", "-"],
        input=ppm_bytes, capture_output=True, check=True)
    assert r.stdout[:3] == b"\xff\xd8\xff"
    return r.stdout


def hoja_firmada(numero):
    """Hoja de formulario inventada: papel, renglones grises y una firma."""
    def pixel(x, y):
        if 20 <= x <= 180 and 30 + 16 * numero >= y >= 20 and (y - 20) % 8 < 3:
            return (40, 40, 40)  # título, más largo en la hoja 2
        if 20 <= x <= 180 and 60 <= y <= 190 and (y - 60) % 14 < 2:
            return (170, 170, 170)  # renglones
        firma = 225 + int(8 * ((x * (numero + 2)) % 23) / 23)
        if 40 <= x <= 160 and abs(y - firma) <= 1:
            return (20, 30, 110)  # firma en tinta azul
        return (247, 243, 230)
    return jpeg(ppm(200, 260, pixel))


def foto_campo():
    def pixel(x, y):
        if y < 50:
            return (150 + y, 180 + y // 2, 210)  # cielo
        return (60 + (x * 3) % 40, 110 + (y % 30), 60)  # pasto
    return jpeg(ppm(160, 120, pixel))


def formulario_png():
    return png(64, 48, lambda x, y: (250, 250, 250) if (y // 6) % 2 else (200, 210, 230))


# ---------------------------------------------------------------------------
# Archivos sueltos


def generar_sueltos():
    escritos = {}

    def guardar(nombre, contenido):
        escribir(SUELTOS / nombre, contenido)
        escritos[nombre] = contenido if isinstance(contenido, bytes) else contenido.encode("utf-8")

    cadenas = {}
    for codigo, datos in ENTREVISTAS.items():
        tiempos = leer_tiempos(codigo)
        segs = whisper_cadena(tiempos)
        turnos = [(it["inicio"], it["fin"], it["hablante"]) for it in tiempos["intervenciones"]]
        hab = hablantes_cadena(segs, turnos)
        md = markdown_cadena(hab, codigo, MAPAS[codigo], datos["encabezado"])
        guardar(f"{codigo}.whisper.json", volcar_cadena(segs))
        guardar(f"{codigo}.hablantes.json", volcar_cadena(hab))
        guardar(f"{codigo}.encabezado.md", datos["encabezado"])
        guardar(f"{codigo}.md", md)
        trozos = [(s["start"], s["end"], s["text"]) for s in segs]
        guardar(f"{codigo}.vtt", vtt(trozos))
        guardar(f"{codigo}.openai.json",
                whisper_openai([(a, b, t, metricas(i)) for i, (a, b, t) in enumerate(trozos)]))
        cadenas[codigo] = {"tiempos": tiempos, "whisper": segs, "hablantes": hab, "md": md}

    e01 = cadenas["E01"]
    duracion = e01["tiempos"]["duracion_s"]
    trozos = [(s["start"], s["end"], s["text"]) for s in e01["whisper"]]

    # TRA-03.2 y TRA-07.3: audio sin duración declarada.
    guardar("E01-sin-duracion.ogg", ogg_sin_duracion((SUELTOS / "E01.ogg").read_bytes()))

    # TRA-05.4: el último fin pasa la duración de E01 en 5 s.
    largo = trozos[:-1] + [(trozos[-1][0], round(duracion + 5, 3), trozos[-1][2])]
    guardar("E01-largo.vtt", vtt(largo))

    # TRA-14.1, 14.2, 14.3 y 14.6.
    seguras = {"avg_logprob": -0.3, "compression_ratio": 1.2, "no_speech_prob": 0.1}
    sospechas = [
        ("Se oye un ruido de fondo en la grabación.", {"no_speech_prob": 0.7, "avg_logprob": -1.2}),
        ("Creo que alguien llamó a la puerta de al lado.", {"no_speech_prob": 0.7, "avg_logprob": -0.5}),
        ("La calle tiene árboles en cada esquina del barrio.", {"compression_ratio": 2.4}),
        ("Muy oscuro todo el camino hasta la casa.", {"compression_ratio": 2.41}),
        ("Sí, claro.", {}),
        ("sí claro", {}),
        ("Sí, claro", {}),
        ("Bueno, eso sí, eso sí, eso sí lo vi.", {"compression_ratio": 2.5}),
        ("La parada queda a dos cuadras.", {}),
        ("Antes había un guarda en la esquina.", {}),
        ("Ahora casi nadie camina de noche.", {}),
        ("Gracias por la conversación.", {}),
    ]
    filas = []
    for i, (texto, cambios) in enumerate(sospechas):
        inicio = round(0.5 + 5 * i, 2)
        filas.append((inicio, round(inicio + 4, 2), texto, {**seguras, **cambios}))
    guardar("sospechas.openai.json", whisper_openai(filas))
    assert repetidos([f[2] for f in filas]) == {4, 5, 6}

    # TRA-06.3: sin compression_ratio.
    guardar("sin-compresion.openai.json",
            whisper_openai([(a, b, t, metricas(i)) for i, (a, b, t) in enumerate(trozos)],
                           quitar=("compression_ratio",)))

    # TRA-08.2: tres hablantes.
    tres = [
        (0, "Buenas tardes, ¿podemos conversar un momento?"),
        (1, "Sí, pase, estamos aquí afuera."),
        (2, "Yo también vivo aquí, soy la vecina de al lado."),
        (0, "¿Desde cuándo viven en el barrio?"),
        (1, "Desde hace doce años."),
        (2, "Yo llegué hace cinco, después de la tormenta."),
    ]
    frases, turnos_tres = [], []
    for i, (sp, texto) in enumerate(tres):
        inicio = round(0.5 + 6 * i, 2)
        frases.append({"sp": sp, "start": inicio, "end": round(inicio + 4.5, 2), "text": texto})
        turnos_tres.append([round(inicio - 0.03, 3), round(inicio + 4.6, 3), sp])
    guardar("tres-hablantes.hablantes.json", volcar_cadena({"turnos": turnos_tres, "frases": frases}))

    # Inválidos de TRA-05.1, en el orden de sus Ejemplos.
    guardar("desconocido.json", json.dumps(
        {"titulo": "Lista de compras inventada", "elementos": ["pan", "café", "arroz"]},
        ensure_ascii=False, indent=2) + "\n")
    guardar("latin1.vtt", ("WEBVTT\n\n00:00.500 --> 00:02.480\n"
                           "Canción inventada del año pasado en la esquina.\n\n").encode("latin-1"))
    # serde_json y Python coinciden: línea 3, columna 15 (1-based, en
    # caracteres; el archivo es ASCII, así que también en bytes). La columna 15
    # es la comilla de "text", donde faltaba una coma.
    guardar("roto.json", '[\n {"start": 0.5,\n  "end": 2.48 "text": "Buenas tardes."}\n]\n')
    guardar("copia.whisper.json", escritos["E01.whisper.json"])
    hab01 = e01["hablantes"]
    faltante = {"turnos": hab01["turnos"][:4], "frases": [dict(f) for f in hab01["frases"][:4]]}
    del faltante["frases"][1]["end"]
    guardar("campo-faltante.hablantes.json", volcar_cadena(faltante))
    ws = e01["whisper"]
    invalidos = [ws[0], ws[1], ws[2],
                 {"start": 12.5, "end": 11.25, "text": "Texto con los tiempos al revés.",
                  "words": [{"s": 12.5, "e": 11.25, "w": " Texto"}]},
                 ws[5]]
    guardar("tiempos-invalidos.whisper.json", volcar_cadena(invalidos))
    guardar("fuera-de-orden.whisper.json", volcar_cadena([ws[0], ws[2], ws[1], ws[3]]))
    guardar("vacio.whisper.json", volcar_cadena([]))
    guardar("marca-mala.vtt", (
        "WEBVTT\n"
        "\n"
        "NOTE Archivo inventado con una marca de tiempo mal escrita en la línea 5.\n"
        "\n"
        "00:00.5 --> 00:02.480\n"
        "Buenas tardes, gracias por recibirme.\n"
        "\n"
        "00:03.082 --> 00:04.364\n"
        "Con gusto, pase adelante.\n"
        "\n"))
    # 20.001 segmentos dentro de los primeros 60 s de E01, compactos para que
    # el archivo pese poco. `words` vacía es lo que da faster-whisper para un
    # segmento sin palabras, y hablantes.py lo contempla.
    muchos = [{"start": round(i * 0.003, 3), "end": round(i * 0.003 + 0.002, 3), "text": "a", "words": []}
              for i in range(20_001)]
    guardar("muchos.whisper.json", json.dumps(muchos, separators=(",", ":")) + "\n")

    # PAR-08.1, PAR-08.2 y PRY-11.5.
    guardar("firma hoja 1.jpg", hoja_firmada(1))
    guardar("firma hoja 2.jpg", hoja_firmada(2))
    guardar("formulario.pdf", formulario_png())
    guardar("foto de campo.jpg", foto_campo())
    guardar("apuntes sueltos.txt", (
        "Apuntes inventados de una salida de campo, para las pruebas.\n"
        "- Revisar la iluminación de la calle de atrás.\n"
        "- Volver a la parada un día de semana a las seis y media.\n"))
    return cadenas, escritos


# ---------------------------------------------------------------------------
# Proyecto de prueba


def toml_entrevista(codigo):
    d = ENTREVISTAS[codigo]
    return (
        'formato = "osr-entrevista"\n'
        "version_formato = 1\n"
        f'id = "{codigo}"\n'
        f'participante = "{codigo}"\n'
        f"creada = {SESION[codigo]['entrevista']}{ZONA}\n"
        f"fecha = {d['fecha']}\n"
        f"lugar = {cadena_toml(d['lugar'])}\n"
    )


def toml_ficha(codigo, audio, duracion):
    d = ENTREVISTAS[codigo]
    return (
        'formato = "osr-ficha"\n'
        "version_formato = 1\n"
        'tipo = "audio_entrevista"\n'
        f'archivo = "{d["audio"]}"\n'
        f'sha256 = "{sha256(audio)}"\n'
        f"bytes = {len(audio)}\n"
        f'formato_archivo = "{d["formato_audio"]}"\n'
        f"importado = {SESION[codigo]['audio']}{ZONA}\n"
        f"fecha_material = {d['fecha']}\n"
        f'seudonimo = "{codigo}"\n'
        'consentimiento = "participantes/consentimientos.csv"\n'
        f"nombre_original = {cadena_toml(d['audio_original'])}\n"
        f"duracion_s = {duracion}\n"
        'estado = "presente"\n'
    )


def lista_toml(valores):
    return "[" + ", ".join(cadena_toml(v) for v in valores) + "]"


def toml_importacion(codigo, marca, archivos, hablantes, aplicadas, omitidas):
    partes = [
        'formato = "osr-importacion"\n'
        "version_formato = 1\n"
        f'marca = "{marca}"\n'
        'fuente = "cadena_hablantes"\n'
        f'modelo = "{MODELO}"\n'
        'markdown_alineado = "si"\n'
        "excepciones = []\n"
    ]
    for tipo, archivo, original, datos in archivos:
        partes.append(
            "\n[[archivos]]\n"
            f'tipo = "{tipo}"\n'
            f'archivo = "{archivo}"\n'
            f"nombre_original = {cadena_toml(original)}\n"
            f'sha256 = "{sha256(datos)}"\n'
            f"bytes = {len(datos)}\n")
    for origen, rol in hablantes:
        partes.append(f'\n[[hablantes]]\norigen = "{origen}"\nrol = "{rol}"\n')
    partes.append(
        "\n[reglas_sospecha]\n"
        f"aplicadas = {lista_toml(aplicadas)}\n"
        f"omitidas = {lista_toml(omitidas)}\n"
        f"no_speech_prob = {UMBRALES['no_speech_prob']}\n"
        f"avg_logprob = {UMBRALES['avg_logprob']}\n"
        f"compression_ratio = {UMBRALES['compression_ratio']}\n")
    return "".join(partes)


def roles_propuestos(md, codigo):
    """Rol por hablante de origen según las etiquetas del Markdown (TRA-08)."""
    mapa = dict(p.split("=") for p in MAPAS[codigo].split(","))
    return {sp: ROLES[etiqueta] for sp, etiqueta in mapa.items()}


def transcripcion(codigo, cadena, audio, marca):
    """transcripcion.json canónico (TRA-11) desde hablantes.json y el Markdown
    alineado (TRA-06, TRA-07), con las reglas de TRA-14 que se pueden aplicar
    a la cadena del autor."""
    frases = cadena["hablantes"]["frases"]
    textos_md = [INTERVENCION.match(l).group(4).strip()
                 for l in cadena["md"].splitlines() if INTERVENCION.match(l)]
    assert len(textos_md) == len(frases)
    roles = roles_propuestos(cadena["md"], codigo)
    textos = [unicodedata.normalize("NFC", re.sub(r"\s*\n\s*", " ", t)).strip() for t in textos_md]
    marcados = repetidos(textos)
    segmentos = []
    for i, f in enumerate(frases):
        segmentos.append({
            "id": f"s{i + 1:04d}",
            "inicio": round(f["start"], 3),
            "fin": round(f["end"], 3),
            "hablante": roles[str(f["sp"])],
            "hablante_origen": str(f["sp"]),
            "texto": textos[i],
            "estado": "sin_revisar",
            "revisado_en": None,
            "sospecha": ["repeticion"] if i in marcados else [],
            "metricas": None,
        })
    doc = {
        "formato": "osr-transcripcion",
        "version_formato": 1,
        "entrevista": codigo,
        "audio": ENTREVISTAS[codigo]["audio"],
        "audio_sha256": sha256(audio),
        "importacion": f"origen/{marca}",
        "modelo": MODELO,
        "estado": "sin_revisar",
        "modificada": False,
        "revisada_en": None,
        "sin_revisar_al_cerrar": None,
        "tiempo_revision_s": 0,
        "reglas_sospecha": {"aplicadas": ["repeticion"], "omitidas": ["sin_voz", "compresion"]},
        "siguiente_id": len(segmentos) + 1,
        "segmentos": segmentos,
    }
    return json.dumps(doc, ensure_ascii=False, indent=2) + "\n", marcados


def generar_proyecto(cadenas, escritos):
    if PROYECTO.exists():
        shutil.rmtree(PROYECTO)
    escribir(PROYECTO / "proyecto.toml", BASE_PROYECTO_TOML)
    escribir(PROYECTO / "README.md", BASE_README_MD)
    escribir(PROYECTO / "sensibilidad.csv", BASE_SENSIBILIDAD_CSV)
    escribir(PROYECTO / "bitacora/bitacora.md", BASE_BITACORA_MD)

    participantes = BASE_PARTICIPANTES_CSV
    correspondencia = BASE_CORRESPONDENCIA_CSV
    consentimientos = BASE_CONSENTIMIENTOS_CSV
    registro = [base_registro()]
    importaciones = []
    for codigo, d in ENTREVISTAS.items():
        t = SESION[codigo]
        participantes += fila_csv([codigo, t["participante"][:10], "activo"])
        correspondencia += fila_csv([codigo, d["nombre"], d["contacto"], d["nota"]])
        consentimientos += fila_csv([codigo] + d["consentimiento"])
        version, fecha, modalidad = d["consentimiento"][:3]
        valores = d["consentimiento"][6:]
        registro += [
            (t["participante"], evento(t["participante"] + ZONA, "participante_creado",
                                       {"seudonimo": codigo})),
            (t["correspondencia"], evento(t["correspondencia"] + ZONA, "correspondencia_modificada",
                                          {"seudonimo": codigo, "campos": ["nombre", "contacto", "nota"]})),
            (t["consentimiento"], evento(t["consentimiento"] + ZONA, "consentimiento_registrado",
                                         {"seudonimo": codigo, "version_formulario": version,
                                          "fecha": fecha, "modalidad": modalidad,
                                          "alcances": dict(zip(ALCANCES, valores))})),
        ]

        carpeta = PROYECTO / "identificable/entrevistas" / codigo
        audio = (SUELTOS / d["audio"]).read_bytes()
        duracion = round(cadenas[codigo]["tiempos"]["duracion_s"], 1)
        marca = marca_compacta(t["transcripcion"])
        escribir(carpeta / "entrevista.toml", toml_entrevista(codigo))
        escribir(carpeta / d["audio"], audio)
        escribir(carpeta / f"{d['audio']}.ficha.toml", toml_ficha(codigo, audio, duracion))

        origen = carpeta / "origen" / marca
        archivos = []
        for tipo, sufijo in [("cadena_whisper", "whisper.json"), ("cadena_hablantes", "hablantes.json"),
                             ("cadena_markdown", "md"), ("encabezado", "encabezado.md")]:
            datos = escritos[f"{codigo}.{sufijo}"]
            escribir(origen / f"{codigo}.{sufijo}", datos)
            archivos.append((tipo, f"{codigo}.{sufijo}", f"{d['prefijo_original']}.{sufijo}", datos))
        texto, marcados = transcripcion(codigo, cadenas[codigo], audio, marca)
        escribir(carpeta / "transcripcion.json", texto)
        roles = roles_propuestos(cadenas[codigo]["md"], codigo)
        orden = []
        for f in cadenas[codigo]["hablantes"]["frases"]:
            if str(f["sp"]) not in orden:
                orden.append(str(f["sp"]))
        escribir(origen / "importacion.toml", toml_importacion(
            codigo, marca, archivos, [(o, roles[o]) for o in orden],
            ["repeticion"], ["sin_voz", "compresion"]))
        segmentos = len(cadenas[codigo]["hablantes"]["frases"])
        importaciones += [
            (t["entrevista"], evento(t["entrevista"] + ZONA, "entrevista_creada", {"entrevista": codigo})),
            (t["audio"], evento(t["audio"] + ZONA, "audio_importado",
                                {"entrevista": codigo, "formato": d["formato_audio"], "bytes": len(audio),
                                 "sha256": sha256(audio), "duracion_s": duracion})),
            (t["transcripcion"], evento(t["transcripcion"] + ZONA, "transcripcion_importada",
                                        {"entrevista": codigo, "fuente": "cadena_hablantes",
                                         "segmentos": segmentos, "marca": marca})),
        ]
    verificacion = {"modificados_fuera": [], "con_error": [], "faltantes": [],
                    "originales_alterados": [], "no_registrados": 0, "temporales_eliminados": 0}
    lineas = registro[1:] + importaciones + [(APERTURA, evento(APERTURA + ZONA, "verificacion", verificacion))]
    lineas.sort(key=lambda par: par[0])
    escribir(PROYECTO / "bitacora/registro.jsonl", registro[0] + "".join(l for _, l in lineas))
    escribir(PROYECTO / "participantes/participantes.csv", participantes)
    escribir(PROYECTO / "participantes/consentimientos.csv", consentimientos)
    escribir(PROYECTO / "identificable/correspondencia.csv", correspondencia)


# ---------------------------------------------------------------------------
# Comprobaciones


def tipo_reconocido(datos):
    """Reconocimiento por contenido de TRA-04, para los archivos de texto."""
    try:
        texto = datos.decode("utf-8")
    except UnicodeDecodeError:
        return "no_utf8"
    if texto.split("\n", 1)[0].rstrip("\r") == "WEBVTT" or re.match(r"WEBVTT[ \t]", texto):
        return "webvtt"
    try:
        raiz = json.loads(texto)
    except ValueError:
        if any(INTERVENCION.match(l) for l in texto.splitlines()):
            return "cadena_markdown"
        return "texto"
    num = lambda v: isinstance(v, (int, float)) and not isinstance(v, bool)
    if isinstance(raiz, list) and all(
            isinstance(s, dict) and num(s.get("start")) and num(s.get("end"))
            and isinstance(s.get("text"), str) and isinstance(s.get("words"), list)
            and all(num(w.get("s")) and num(w.get("e")) and isinstance(w.get("w"), str) for w in s["words"])
            for s in raiz):
        return "cadena_whisper"
    if isinstance(raiz, dict) and isinstance(raiz.get("turnos"), list) and isinstance(raiz.get("frases"), list):
        if all(isinstance(t, list) and len(t) == 3 and num(t[0]) and num(t[1]) and isinstance(t[2], int)
               for t in raiz["turnos"]) and all(
                isinstance(f.get("sp"), int) and num(f.get("start")) and num(f.get("end"))
                and isinstance(f.get("text"), str) for f in raiz["frases"]):
            return "cadena_hablantes"
    if isinstance(raiz, dict) and isinstance(raiz.get("segments"), list):
        return "whisper_json"
    return "desconocido"


def comprobar(cadenas):
    errores = []

    def exigir(condicion, mensaje):
        if not condicion:
            errores.append(mensaje)

    for codigo, c in cadenas.items():
        interv = c["tiempos"]["intervenciones"]
        frases = c["hablantes"]["frases"]
        exigir(len(frases) == len(interv), f"{codigo}: {len(frases)} frases y {len(interv)} intervenciones")
        for f, it in zip(frases, interv):
            exigir(f["sp"] == it["hablante"] and f["text"] == it["texto"], f"{codigo}: frase distinta {f}")
        lineas = [l for l in c["md"].splitlines() if INTERVENCION.match(l)]
        exigir(len(lineas) == len(frases), f"{codigo}.md no tiene una línea por frase")
        for i, (l, f) in enumerate(zip(lineas, frases), 1):
            m = INTERVENCION.match(l)
            exigir(f"{m.group(1)}:{m.group(2)}" == mmss(f["start"]), f"{codigo}.md intervención {i} desalineada")
        resto = c["md"].split(lineas[0], 1)[1].splitlines()
        exigir(all(not l.strip() or INTERVENCION.match(l) for l in resto), f"{codigo}.md con texto suelto")
        enc = ENTREVISTAS[codigo]["encabezado"]
        exigir(not any(INTERVENCION.match(l) for l in enc.splitlines()), f"{codigo}.encabezado.md con intervención")
        for sufijo, tipo in [("whisper.json", "cadena_whisper"), ("hablantes.json", "cadena_hablantes"),
                             ("md", "cadena_markdown"), ("vtt", "webvtt"), ("openai.json", "whisper_json")]:
            obtenido = tipo_reconocido((SUELTOS / f"{codigo}.{sufijo}").read_bytes())
            exigir(obtenido == tipo, f"{codigo}.{sufijo} se reconoce como {obtenido}")
        ultimo = max(s["end"] for s in c["whisper"])
        exigir(ultimo <= c["tiempos"]["duracion_s"] + 2.0, f"{codigo}: la transcripción pasa el audio")

    e01 = cadenas["E01"]["hablantes"]["frases"]
    serie = [i for i, f in enumerate(e01) if f["text"] == "Sí, claro."]
    exigir(len(serie) == 2 and serie[1] == serie[0] + 1, "E01 sin la serie repetida")
    exigir(len(e01) >= 20, "E01 con menos de 20 segmentos")
    exigir("Ana Inventada Muñoz" in e01[2]["text"], "la intervención 3 de E01 no dice el nombre")
    exigir(60 <= cadenas["E01"]["tiempos"]["duracion_s"] <= 120, "E01 fuera de 60 a 120 s")
    exigir(cadenas["E02"]["tiempos"]["duracion_s"] <= 120, "E02 pasa de 120 s")

    for nombre, tipo in [("desconocido.json", "desconocido"), ("latin1.vtt", "no_utf8"),
                         ("copia.whisper.json", "cadena_whisper"),
                         ("tiempos-invalidos.whisper.json", "cadena_whisper"),
                         ("fuera-de-orden.whisper.json", "cadena_whisper"), ("vacio.whisper.json", "cadena_whisper"),
                         ("marca-mala.vtt", "webvtt"), ("muchos.whisper.json", "cadena_whisper"),
                         ("tres-hablantes.hablantes.json", "cadena_hablantes"),
                         ("sospechas.openai.json", "whisper_json"), ("sin-compresion.openai.json", "whisper_json"),
                         ("E01-largo.vtt", "webvtt")]:
        obtenido = tipo_reconocido((SUELTOS / nombre).read_bytes())
        exigir(obtenido == tipo, f"{nombre} se reconoce como {obtenido}")
    try:
        json.loads((SUELTOS / "roto.json").read_text())
        exigir(False, "roto.json es JSON válido")
    except json.JSONDecodeError as e:
        exigir((e.lineno, e.colno) == (3, 15), f"roto.json falla en {e.lineno}:{e.colno}")
    exigir(len(json.loads((SUELTOS / "muchos.whisper.json").read_text())) == 20_001, "muchos sin 20.001")
    exigir(json.loads((SUELTOS / "muchos.whisper.json").read_text())[-1]["end"]
           <= cadenas["E01"]["tiempos"]["duracion_s"], "muchos pasa la duración de E01")
    vtt_lineas = (SUELTOS / "marca-mala.vtt").read_text().splitlines()
    exigir("-->" in vtt_lineas[4] and vtt_lineas[4].startswith("00:00.5 "), "marca-mala.vtt sin la marca en la línea 5")

    # Sospechas: cada segmento con las tres métricas, reglas como pide el Ejemplo.
    sosp = json.loads((SUELTOS / "sospechas.openai.json").read_text())["segments"]
    marcas = []
    for s in sosp:
        exigir(all(k in s for k in UMBRALES), f"sospechas: segmento {s['id']} sin las tres métricas")
        m = []
        if s["no_speech_prob"] > 0.6 and s["avg_logprob"] < -1.0:
            m.append("sin_voz")
        if s["compression_ratio"] > 2.4:
            m.append("compresion")
        marcas.append(m)
    for i in repetidos([s["text"].strip() for s in sosp]):
        marcas[i].append("repeticion")
    esperadas = [["sin_voz"], [], [], ["compresion"], ["repeticion"], ["repeticion"], ["repeticion"],
                 ["compresion"]] + [[]] * (len(sosp) - 8)
    exigir(marcas == esperadas, f"sospechas con marcas {marcas}")
    exigir(sosp[7]["compression_ratio"] <= 3.0, "segmento 8 marcaría con umbral 3.0")
    exigir(sosp[-1]["end"] <= cadenas["E01"]["tiempos"]["duracion_s"], "sospechas pasa la duración")
    for nombre in ("E01.openai.json", "E02.openai.json", "sin-compresion.openai.json"):
        for s in json.loads((SUELTOS / nombre).read_text())["segments"]:
            exigir(s["no_speech_prob"] <= 0.6 and s.get("compression_ratio", 0) <= 2.4,
                   f"{nombre}: métrica sobre el umbral")
    exigir(all("compression_ratio" not in s for s in
               json.loads((SUELTOS / "sin-compresion.openai.json").read_text())["segments"]),
           "sin-compresion con compression_ratio")

    # Proyecto: TOML, JSON y CSV legibles, sin .osr/ ni .gitkeep.
    for ruta in sorted(PROYECTO.rglob("*")):
        rel = ruta.relative_to(PROYECTO).as_posix()
        exigir(not rel.startswith(".osr") and ruta.name != ".gitkeep", f"sobra {rel}")
        if ruta.suffix == ".toml":
            tomllib.loads(ruta.read_text(encoding="utf-8"))
        if ruta.suffix in (".json", ".jsonl"):
            for linea in ([ruta.read_text()] if ruta.suffix == ".json" else ruta.read_text().splitlines()):
                json.loads(linea)
        # Lo de origen/ es copia byte a byte de la cadena del autor, que no
        # termina sus JSON con salto de línea; PRY-06 vale para lo que escribe la app.
        if ruta.is_file() and ruta.suffix not in (".ogg", ".m4a") and "/origen/" not in rel:
            datos = ruta.read_bytes()
            exigir(not datos.startswith(b"\xef\xbb\xbf"), f"{rel} con BOM")
            exigir(datos.decode("utf-8") == unicodedata.normalize("NFC", datos.decode("utf-8")), f"{rel} no NFC")
            exigir(datos.endswith(b"\n"), f"{rel} sin salto final")
    exigir(BASE_PROYECTO_TOML.splitlines()[2].startswith("nombre = "), "la línea 3 de proyecto.toml no es nombre")
    consent = (PROYECTO / "participantes/consentimientos.csv").read_bytes().split(b"\r\n")
    exigir(len(consent) == 4 and consent[2].startswith(b'E02,"v1, octubre",'), "consentimientos.csv mal armado")
    for codigo in ENTREVISTAS:
        doc = json.loads((PROYECTO / f"identificable/entrevistas/{codigo}/transcripcion.json").read_text())
        frases = cadenas[codigo]["hablantes"]["frases"]
        exigir([(s["inicio"], s["fin"]) for s in doc["segmentos"]] ==
               [(round(f["start"], 3), round(f["end"], 3)) for f in frases], f"{codigo}: tiempos")
    doc = json.loads((PROYECTO / "identificable/entrevistas/E01/transcripcion.json").read_text())
    exigir([s["id"] for s in doc["segmentos"] if s["sospecha"]] == ["s0012", "s0013"], "E01 sin la serie marcada")

    # Nada identificable fuera de identificable/ dentro del proyecto.
    buscadas = []
    for d in ENTREVISTAS.values():
        buscadas += [d["nombre"], d["contacto"], d["nota"], d["lugar"], d["audio_original"], d["prefijo_original"]]
        buscadas += [l for l in d["encabezado"].splitlines() if l.strip()]
    buscadas += ["Carmen Inventada Solís", "Nombre Inventado", "Carla Supuesta Vega"]
    buscadas += [f["text"] for c in cadenas.values() for f in c["hablantes"]["frases"]]
    agujas = {unicodedata.normalize(forma, b).lower() for b in buscadas for forma in ("NFC", "NFD")}
    for ruta in PROYECTO.rglob("*"):
        rel = ruta.relative_to(PROYECTO).as_posix()
        if ruta.is_file() and not rel.startswith("identificable/"):
            texto = ruta.read_text(encoding="utf-8").lower()
            for aguja in agujas:
                exigir(aguja not in texto, f"{rel} contiene {aguja!r}")
    # Los datos de correspondencia solo están en correspondencia.csv.
    for d in ENTREVISTAS.values():
        for valor in (d["contacto"], d["nota"]):
            for ruta in DATOS.rglob("*"):
                rel = ruta.relative_to(DATOS).as_posix()
                if (ruta.is_file() and rel != "proyecto/identificable/correspondencia.csv"
                        and rel != "fuentes/generar.py" and ruta.suffix not in (".ogg", ".m4a", ".jpg", ".pdf")):
                    exigir(valor not in ruta.read_text(encoding="utf-8", errors="replace"),
                           f"{rel} contiene {valor!r}")
    if errores:
        sys.exit("generar.py: fallaron las comprobaciones\n- " + "\n- ".join(errores))


def comprobar_base(carpeta):
    """Compara la base incrustada con un proyecto recién creado con osr nuevo
    con el nombre «Proyecto de prueba inventado», salvo la fecha."""
    carpeta = Path(carpeta)
    toml = (carpeta / "proyecto.toml").read_text(encoding="utf-8")
    creado = re.search(r"^creado = (\S+)$", toml, re.M).group(1)
    fecha = creado[:10]
    pares = [
        ("proyecto.toml", BASE_PROYECTO_TOML, toml.replace(creado, CREADO)),
        ("README.md", BASE_README_MD,
         (carpeta / "README.md").read_text(encoding="utf-8").replace(f"generó el {fecha}", f"generó el {FECHA_README}")),
        ("bitacora/bitacora.md", BASE_BITACORA_MD, (carpeta / "bitacora/bitacora.md").read_text(encoding="utf-8")),
        ("bitacora/registro.jsonl", base_registro(),
         (carpeta / "bitacora/registro.jsonl").read_text(encoding="utf-8").replace(creado, CREADO)),
        ("sensibilidad.csv", BASE_SENSIBILIDAD_CSV, (carpeta / "sensibilidad.csv").read_bytes().decode()),
        ("participantes/participantes.csv", BASE_PARTICIPANTES_CSV,
         (carpeta / "participantes/participantes.csv").read_bytes().decode()),
        ("participantes/consentimientos.csv", BASE_CONSENTIMIENTOS_CSV,
         (carpeta / "participantes/consentimientos.csv").read_bytes().decode()),
        ("identificable/correspondencia.csv", BASE_CORRESPONDENCIA_CSV,
         (carpeta / "identificable/correspondencia.csv").read_bytes().decode()),
    ]
    distintos = [nombre for nombre, esperado, real in pares if esperado != real]
    if distintos:
        sys.exit("generar.py: la base cambió en " + ", ".join(distintos))
    print("La base coincide con osr nuevo.")


def main():
    if len(sys.argv) == 3 and sys.argv[1] == "--comprobar-base":
        comprobar_base(sys.argv[2])
        return
    if len(sys.argv) != 1:
        sys.exit(__doc__)
    cadenas, escritos = generar_sueltos()
    generar_proyecto(cadenas, escritos)
    comprobar(cadenas)
    print("Datos de prueba generados en", SUELTOS, "y", PROYECTO)


if __name__ == "__main__":
    main()
