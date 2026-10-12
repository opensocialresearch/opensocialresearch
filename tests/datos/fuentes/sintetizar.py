"""Sintetiza los audios inventados de las entrevistas de prueba (PRY-17).

uso: sintetizar.py

Lee guion-E01.tsv y guion-E02.tsv, sintetiza cada intervención con Supertonic 3
(voz de quien entrevista M1, participantes F2 y M3), quita el silencio de los
extremos de cada una, las une con las pausas del guion y deja en ../sueltos/
E01.ogg (Ogg Opus) y E02.m4a (AAC), más tiempos-E01.json y tiempos-E02.json con
el inicio y el fin exactos de cada intervención en el audio final.

Necesita el paquete supertonic con el modelo en ~/.cache/supertonic3 y ffmpeg.
Solo se corre para regenerar el audio; las pruebas usan los archivos ya
generados que están en el repositorio.
"""
import json
import os
import subprocess
import sys
import tempfile
import wave
from pathlib import Path

import numpy as np
import supertonic

AQUI = Path(__file__).resolve().parent
SUELTOS = AQUI.parent / "sueltos"
VOCES = {"E01": {"0": "M1", "1": "F2"}, "E02": {"1": "M1", "0": "M3"}}
SALIDA = {
    "E01": ["-c:a", "libopus", "-b:a", "32k", "E01.ogg"],
    "E02": ["-c:a", "aac", "-b:a", "64k", "E02.m4a"],
}
FINAL_S = 0.6
UMBRAL = 0.01


def leer_guion(ruta):
    lineas = []
    for linea in ruta.read_text(encoding="utf-8").splitlines():
        if linea.strip() and not linea.startswith("#"):
            hablante, pausa, texto = linea.split("\t")
            lineas.append((hablante, float(pausa), texto))
    return lineas


def recortar(onda, tasa):
    """Quita el silencio de los extremos, con 20 ms de margen."""
    voz = np.flatnonzero(np.abs(onda) > UMBRAL)
    margen = round(0.02 * tasa)
    if voz.size == 0:
        return onda
    return onda[max(voz[0] - margen, 0) : voz[-1] + 1 + margen]


def sintetizar(tts, codigo):
    tasa = tts.sample_rate
    partes, tiempos, t = [], [], 0.0
    estilos = {}
    for hablante, pausa, texto in leer_guion(AQUI / f"guion-{codigo}.tsv"):
        voz = VOCES[codigo][hablante]
        if voz not in estilos:
            estilos[voz] = tts.get_voice_style(voz)
        onda, _ = tts.synthesize(texto, voice_style=estilos[voz], lang="es")
        onda = recortar(np.asarray(onda, dtype=np.float32).reshape(-1), tasa)
        silencio = np.zeros(round(pausa * tasa), dtype=np.float32)
        partes += [silencio, onda]
        inicio = t + len(silencio) / tasa
        fin = inicio + len(onda) / tasa
        tiempos.append({"hablante": int(hablante), "inicio": round(inicio, 3),
                        "fin": round(fin, 3), "texto": texto})
        t = fin
    partes.append(np.zeros(round(FINAL_S * tasa), dtype=np.float32))
    return np.concatenate(partes), tasa, tiempos


def guardar(codigo, onda, tasa, tiempos):
    with tempfile.TemporaryDirectory() as tmp:
        wav = Path(tmp) / "audio.wav"
        with wave.open(str(wav), "wb") as w:
            w.setnchannels(1)
            w.setsampwidth(2)
            w.setframerate(tasa)
            w.writeframes((np.clip(onda, -1, 1) * 32767).astype("<i2").tobytes())
        *opciones, nombre = SALIDA[codigo]
        subprocess.run(["ffmpeg", "-loglevel", "error", "-y", "-i", str(wav), "-ac", "1",
                        "-map_metadata", "-1", "-fflags", "+bitexact", *opciones,
                        str(SUELTOS / nombre)], check=True)
    duracion = round(len(onda) / tasa, 3)
    (SUELTOS / f"tiempos-{codigo}.json").write_text(
        json.dumps({"duracion_s": duracion, "intervenciones": tiempos},
                   ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    print(codigo, duracion, "s,", len(tiempos), "intervenciones", file=sys.stderr)


def main():
    SUELTOS.mkdir(exist_ok=True)
    tts = supertonic.TTS(model_dir=os.path.expanduser("~/.cache/supertonic3"),
                         auto_download=False)
    for codigo in ("E01", "E02"):
        guardar(codigo, *sintetizar(tts, codigo))


if __name__ == "__main__":
    main()
