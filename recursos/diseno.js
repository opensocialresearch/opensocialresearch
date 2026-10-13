// Guardar un paso del diseño y marcarlo como revisado (DIS-04, DIS-05). El
// estado del guardado va con texto, en la barra y junto a los botones, nunca
// solo con un color (PRY-13). Nada de lo escrito se pierde sin aviso: con
// cambios sin guardar, salir pide confirmación y «Marcar como revisado»
// guarda primero (D-55).

const formulario = document.getElementById("paso");
const guardado = document.getElementById("guardado");
const guardadoLocal = document.getElementById("guardado-local");
const aviso = document.getElementById("paso-error");
const SIN_RESPUESTA =
  "No hubo respuesta de Open Social Research. Hay que comprobar que siga abierto en la terminal.";
const ETIQUETAS = {
  "Sin empezar": ["sin", "○"],
  "En borrador": ["borrador", "◐"],
  Revisado: ["revisado", "✓"],
  "Revisar coherencia": ["coherencia", "△"],
};

let sinGuardar = false;

function mostrarEstadoDelGuardado(texto) {
  guardado.textContent = texto;
  guardadoLocal.textContent = texto;
}

function mostrarError(mensaje) {
  aviso.querySelector("span").textContent = mensaje;
  aviso.hidden = false;
}

// Las etiquetas del paso actual, en el rótulo y en la navegación.
function mostrarEstadoDelPaso(estado) {
  const [clave, icono] = ETIQUETAS[estado] || ETIQUETAS["En borrador"];
  const actual = document.querySelector(".pasos-nav a[aria-current=page] .etiqueta");
  for (const etiqueta of [document.querySelector("#estado-paso .etiqueta"), actual]) {
    if (!etiqueta) continue;
    etiqueta.className = `etiqueta etiqueta--${clave}`;
    etiqueta.innerHTML = "";
    const simbolo = document.createElement("span");
    simbolo.setAttribute("aria-hidden", "true");
    simbolo.textContent = icono;
    etiqueta.append(simbolo, ` ${estado}`);
  }
}

async function pedir(camino, datos) {
  try {
    const respuesta = await fetch(camino, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(datos),
      credentials: "same-origin",
    });
    const cuerpo = await respuesta.json();
    return { ok: respuesta.ok, cuerpo };
  } catch (_) {
    return { ok: false, cuerpo: { mensaje: SIN_RESPUESTA } };
  }
}

function hora() {
  return new Date().toLocaleTimeString("es-CR", { hour: "2-digit", minute: "2-digit", hour12: false });
}

// Guarda las respuestas; devuelve si se guardaron.
async function guardar() {
  aviso.hidden = true;
  mostrarEstadoDelGuardado("Guardando…");
  const respuestas = [...formulario.querySelectorAll("textarea")].map((campo) => campo.value);
  const { ok, cuerpo } = await pedir(`/api/diseno/${formulario.dataset.paso}`, {
    respuestas,
    version: formulario.dataset.version,
  });
  if (!ok) {
    mostrarEstadoDelGuardado(`No se pudo guardar: ${cuerpo.mensaje}`);
    mostrarError(cuerpo.mensaje);
    return false;
  }
  formulario.dataset.version = cuerpo.version;
  sinGuardar = false;
  mostrarEstadoDelGuardado(`Guardado a las ${hora()}`);
  mostrarEstadoDelPaso(cuerpo.estado);
  return true;
}

formulario.addEventListener("input", () => {
  sinGuardar = true;
  mostrarEstadoDelGuardado("Hay cambios sin guardar");
});

window.addEventListener("beforeunload", (evento) => {
  if (sinGuardar) {
    evento.preventDefault();
    evento.returnValue = "";
  }
});

formulario.addEventListener("submit", (evento) => {
  evento.preventDefault();
  guardar();
});

document.getElementById("revision").addEventListener("click", async (evento) => {
  const revisado = evento.currentTarget.dataset.revisado === "true";
  if (sinGuardar && !(await guardar())) return;
  aviso.hidden = true;
  const { ok, cuerpo } = await pedir(`/api/diseno/${formulario.dataset.paso}/revision`, { revisado });
  if (ok) {
    window.location.reload();
  } else {
    mostrarError(cuerpo.mensaje);
  }
});
