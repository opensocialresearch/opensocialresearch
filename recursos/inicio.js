// Crear y abrir una investigación desde la pantalla (PRY-19, PRY-20). Cada
// formulario con data-accion manda su pedido a la API con la cookie de la
// sesión; si sale bien, la página pasa al resumen (PRY-22), y si no, el
// mensaje queda debajo del botón, asociado al campo si lo hay, sin perder lo
// escrito.

const SIN_RESPUESTA =
  "No hubo respuesta de Open Social Research. Hay que comprobar que siga abierto en la terminal.";

// El campo visible del formulario, si tiene uno: el nombre al crear. Los
// botones «Abrir» de la lista no tienen campo visible.
function campoVisible(formulario) {
  return formulario.querySelector("input[data-ayuda]");
}

function mostrarError(formulario, mensaje) {
  const aviso = formulario.querySelector(".aviso-error");
  aviso.querySelector("span").textContent = mensaje;
  aviso.hidden = false;
  const campo = campoVisible(formulario);
  if (campo) {
    campo.setAttribute("aria-invalid", "true");
    campo.setAttribute("aria-describedby", `${campo.dataset.ayuda} ${aviso.id}`);
    campo.focus();
  } else {
    const boton = formulario.querySelector("button[type=submit]");
    boton.setAttribute("aria-describedby", aviso.id);
    boton.focus();
  }
}

function limpiarError(formulario) {
  formulario.querySelector(".aviso-error").hidden = true;
  const campo = campoVisible(formulario);
  if (campo) {
    campo.removeAttribute("aria-invalid");
    campo.setAttribute("aria-describedby", campo.dataset.ayuda);
  }
}

function esperar(formulario, esperando) {
  const boton = formulario.querySelector("button[type=submit]");
  boton.disabled = esperando;
  boton.textContent = esperando ? boton.dataset.esperando : boton.dataset.texto;
  for (const campo of formulario.querySelectorAll("input[data-ayuda]")) campo.readOnly = esperando;
}

async function enviar(formulario) {
  limpiarError(formulario);
  esperar(formulario, true);
  const datos = {};
  for (const campo of formulario.querySelectorAll("input")) datos[campo.name] = campo.value;
  let mensaje = SIN_RESPUESTA;
  try {
    const respuesta = await fetch(formulario.dataset.accion, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(datos),
      credentials: "same-origin",
    });
    if (respuesta.ok) {
      window.location.assign("/");
      return;
    }
    const cuerpo = await respuesta.json();
    mensaje = cuerpo.mensaje || mensaje;
  } catch (_) {
    // Sin respuesta: queda el mensaje de SIN_RESPUESTA.
  }
  esperar(formulario, false);
  mostrarError(formulario, mensaje);
}

for (const formulario of document.querySelectorAll("form[data-accion]")) {
  const boton = formulario.querySelector("button[type=submit]");
  boton.dataset.texto = boton.textContent;
  formulario.addEventListener("submit", (evento) => {
    evento.preventDefault();
    enviar(formulario);
  });
}
