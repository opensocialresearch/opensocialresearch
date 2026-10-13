// Las dos vistas de la bienvenida del primer uso (PRY-23). «Iniciar» pasa a
// «Primera investigación» sin pedir otra página y deja el foco en el nombre.

document.getElementById("iniciar").addEventListener("click", () => {
  document.getElementById("bienvenida").hidden = true;
  document.getElementById("primer-proyecto").hidden = false;
  document.getElementById("crear-nombre").focus();
});
