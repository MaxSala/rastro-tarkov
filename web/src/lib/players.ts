/** Paleta fija del party (índices 0-7). Los valores viven en index.css (--player-N). */
export const PLAYER_COLORS = [0, 1, 2, 3, 4, 5, 6, 7] as const;
export const playerVar = (i: number) => `var(--player-${Math.max(0, Math.min(7, i))})`;

/** Valor real de una variable CSS (para atributos SVG y Leaflet, que no resuelven var()). */
export function cssVar(name: string, fallback = "#888"): string {
  return getComputedStyle(document.documentElement).getPropertyValue(name).trim() || fallback;
}
export const playerColor = (i: number) => cssVar(`--player-${Math.max(0, Math.min(7, i))}`);
