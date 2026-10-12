// Quita el token de la dirección visible sin recargar la página (SRV-04). La
// cookie de sesión ya autentica los pedidos siguientes de la pestaña.
const direccion = new URL(window.location.href);
if (direccion.searchParams.has("token")) {
  direccion.searchParams.delete("token");
  window.history.replaceState(window.history.state, "", direccion.pathname + direccion.search + direccion.hash);
}
