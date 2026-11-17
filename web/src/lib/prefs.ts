// Preferencias de visualización (solo de esta interfaz, no afectan a los datos): localStorage.
export interface Prefs {
  follow: boolean;
  labels: boolean;
  trail: boolean;
  markerScale: number;
  units: "m";
}

const KEY = "rastro.prefs";
export const DEFAULT_PREFS: Prefs = { follow: false, labels: true, trail: true, markerScale: 1, units: "m" };

export function loadPrefs(): Prefs {
  try {
    const raw: unknown = JSON.parse(localStorage.getItem(KEY) ?? "{}");
    if (!raw || typeof raw !== "object") return DEFAULT_PREFS;
    const r = raw as Record<string, unknown>;
    const bool = (k: keyof Prefs) =>
      typeof r[k] === "boolean" ? (r[k] as boolean) : (DEFAULT_PREFS[k] as boolean);
    const scale =
      typeof r.markerScale === "number" && r.markerScale >= 0.75 && r.markerScale <= 1.5 ? r.markerScale : 1;
    return {
      follow: bool("follow"),
      labels: bool("labels"),
      trail: bool("trail"),
      markerScale: scale,
      units: "m",
    };
  } catch {
    return DEFAULT_PREFS;
  }
}

export function savePrefs(p: Prefs): void {
  localStorage.setItem(KEY, JSON.stringify(p));
}
