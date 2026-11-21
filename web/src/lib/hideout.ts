// What the hideout still asks for, from the level built of each station (kept per game mode
// with the quest progress). Rastro never sees your stash: it lists needs, not what you have.
import type { Station, StationLevel } from "./hideoutSchema";
import type { Progress } from "./questProtocol";
import { itemsToKeep } from "./questPlanning";
import type { ItemRef, Quest, QuestDb } from "./quests";

export const built = (p: Progress, s: Station): number => p.hideout[s.id] ?? 0;
export const maxLevel = (s: Station): number => s.levels.at(-1)?.level ?? 0;
export const nextLevel = (p: Progress, s: Station): StationLevel | null =>
  s.levels.find((l) => l.level > built(p, s)) ?? null;

/** Station requirements of a level that your hideout doesn't meet yet. */
export function missingStations(p: Progress, l: StationLevel): [string, number][] {
  return l.stations.filter(([id, lv]) => (p.hideout[id] ?? 0) < lv);
}

export interface StationUse {
  station: Station;
  level: number;
}

export interface KeepLine {
  item: ItemRef;
  count: number;
  fir: boolean;
  quests: Quest[];
  stations: StationUse[];
}

export type Scope = "next" | "all";
export type Source = "all" | "quests" | "hideout";

/** Items for the stations' next level, or for every level still to build. */
export function hideoutNeeds(stations: Station[], p: Progress, scope: Scope): KeepLine[] {
  const out = new Map<string, KeepLine>();
  for (const s of stations) {
    const todo = s.levels.filter((l) => l.level > built(p, s));
    for (const l of scope === "next" ? todo.slice(0, 1) : todo) {
      for (const n of l.items) {
        const key = `${n.id}:${n.fir}`;
        const cur = out.get(key) ?? {
          item: { id: n.id, name: n.name },
          count: 0,
          fir: n.fir,
          quests: [],
          stations: [],
        };
        cur.count += n.count;
        cur.stations.push({ station: s, level: l.level });
        out.set(key, cur);
      }
    }
  }
  return [...out.values()];
}

/** Quest and hideout needs together, one line per item (found in raid apart). */
export function keepList(
  db: QuestDb,
  p: Progress,
  opts: { source: Source; scope: Scope; withAvailable: boolean },
): KeepLine[] {
  const out = new Map<string, KeepLine>();
  const add = (l: KeepLine) => {
    const key = `${l.item.id}:${l.fir}`;
    const cur = out.get(key);
    if (!cur) out.set(key, { ...l, quests: [...l.quests], stations: [...l.stations] });
    else {
      cur.count += l.count;
      cur.quests.push(...l.quests);
      cur.stations.push(...l.stations);
    }
  };
  if (opts.source !== "hideout")
    for (const n of itemsToKeep(db, p, opts.withAvailable)) add({ ...n, stations: [] });
  if (opts.source !== "quests") for (const n of hideoutNeeds(db.hideout, p, opts.scope)) add(n);
  return [...out.values()].sort((a, b) => Number(b.fir) - Number(a.fir) || b.count - a.count);
}

/** Stations whose next levels still ask for this item, for the Market's item card. */
export function stationsNeeding(
  db: QuestDb,
  p: Progress,
  itemId: string,
): { use: StationUse; count: number }[] {
  return db.hideout.flatMap((s) =>
    s.levels
      .filter((l) => l.level > built(p, s))
      .flatMap((l) => {
        const count = l.items.filter((n) => n.id === itemId).reduce((sum, n) => sum + n.count, 0);
        return count > 0 ? [{ use: { station: s, level: l.level }, count }] : [];
      }),
  );
}

/** Construction time as "2 h 30 min", "45 min" or null when instant. */
export function buildTime(seconds: number): string | null {
  if (seconds <= 0) return null;
  const h = Math.floor(seconds / 3600);
  const m = Math.round((seconds % 3600) / 60);
  return h > 0 ? (m > 0 ? `${h} h ${m} min` : `${h} h`) : `${Math.max(1, m)} min`;
}
