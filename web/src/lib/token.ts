// La clave de sesión llega en el fragmento (#t=…), que nunca se envía al servidor en la URL.
// Se guarda en sessionStorage (solo esta pestaña, mismo origen) y se borra de la barra de direcciones.
const KEY = "rastro.t";
const TOKEN_RE = /^[0-9a-f]{64}$/;

export function readToken(): string | null {
  const hash = new URLSearchParams(window.location.hash.slice(1)).get("t");
  if (hash && TOKEN_RE.test(hash)) {
    sessionStorage.setItem(KEY, hash);
    history.replaceState(null, "", window.location.pathname);
    return hash;
  }
  const saved = sessionStorage.getItem(KEY);
  return saved && TOKEN_RE.test(saved) ? saved : null;
}
