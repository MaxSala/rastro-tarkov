// Text that comes from map packs but is really interface vocabulary: floor names and the
// "requires switch" note on extracts. Packs store it in English (older ones in Spanish);
// it is shown in the interface language.
import { t } from "../locales";

type FloorKey = keyof Omit<typeof t.floors.names, "floor">;

const FLOORS: [RegExp, FloorKey][] = [
  [/^(ground floor|planta baja)$/i, "ground"],
  [/^(upper floor|planta alta)$/i, "upper"],
  [/^(basement|sótano)$/i, "basement"],
  [/^(underground|subterráneo)$/i, "underground"],
  [/^(tunnels|túneles)$/i, "tunnels"],
  [/^(bunkers|búnkeres)$/i, "bunkers"],
  [/^(roof|tejado)$/i, "roof"],
  [/^(garage|garaje)$/i, "garage"],
];

export function floorName(name: string): string {
  const trimmed = name.trim();
  for (const [re, key] of FLOORS) if (re.test(trimmed)) return t.floors.names[key];
  const n = /^(\d)(?:st|nd|rd|th) floor$|^(\d)\.ª planta$/i.exec(trimmed);
  const num = n?.[1] ?? n?.[2];
  return num ? t.floors.names.floor(Number(num)) : name;
}

const LEGACY_REQUIRES = "Requiere activar: ";

/** Extra line under an extract: switches to activate first, or the pack's own note. */
export function extractNote(e: { conditions: string; switches?: string[] | undefined }): string {
  if (e.switches && e.switches.length > 0) return t.extracts.requires(e.switches.join(", "));
  if (e.conditions.startsWith(LEGACY_REQUIRES))
    return t.extracts.requires(e.conditions.slice(LEGACY_REQUIRES.length));
  return e.conditions;
}
