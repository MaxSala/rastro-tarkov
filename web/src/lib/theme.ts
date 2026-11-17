export type ThemePref = "system" | "dark" | "light";
const KEY = "rastro.theme";
const media = window.matchMedia("(prefers-color-scheme: light)");

export function getThemePref(): ThemePref {
  const v = localStorage.getItem(KEY);
  return v === "dark" || v === "light" ? v : "system";
}

export function applyTheme(pref: ThemePref = getThemePref()): void {
  const theme = pref === "system" ? (media.matches ? "light" : "dark") : pref;
  document.documentElement.dataset.theme = theme;
}

export function setThemePref(pref: ThemePref): void {
  if (pref === "system") localStorage.removeItem(KEY);
  else localStorage.setItem(KEY, pref);
  applyTheme(pref);
}

media.addEventListener("change", () => applyTheme());

/** Tema efectivo actual; se actualiza al cambiar data-theme (para redibujar capas de Leaflet). */
export function subscribeTheme(cb: () => void): () => void {
  const obs = new MutationObserver(cb);
  obs.observe(document.documentElement, { attributes: true, attributeFilter: ["data-theme"] });
  return () => obs.disconnect();
}
export const currentTheme = () => document.documentElement.dataset.theme ?? "dark";
