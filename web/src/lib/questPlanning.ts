// Planning helpers built on the quest rules: what to bring into a raid, the order to do things
// in, what can be handed in, which items to keep, how far along Kappa and Lightkeeper are, and
// the colour each quest wears on the map.
import { doneCount, forFaction, inRaid, objectiveDone, progressOf, statusOf } from "./questLogic";
import type { Progress, QuestMarker } from "./questProtocol";
import { questIndex, type ItemRef, type Objective, type Quest, type QuestDb } from "./quests";

/**
 * Objectives whose items go into your rig rather than onto a trader's counter: a marker to
 * place, something to plant or to use. (Idea from tarkov-map-companion's TaskRequirements, MIT.)
 */
const CARRIED = new Set(["mark", "plantItem", "plantQuestItem", "useItem"]);

const onMap = (o: Objective, map: string) => o.maps.includes(map) || o.positions.some((s) => s.map === map);

export interface KitLine {
  item: ItemRef;
  count: number;
  key: boolean;
}

/** What the given quests want you to bring to `map`: items to use there, and the keys. */
export function kitFor(quests: Quest[], p: Progress, map: string): KitLine[] {
  const out = new Map<string, KitLine>();
  for (const q of quests) {
    for (const o of q.objectives) {
      if (!onMap(o, map) || objectiveDone(o, p, "active")) continue;
      if (CARRIED.has(o.kind)) {
        // Several items in one objective are alternatives: the first one is what to bring.
        const item = o.items[0];
        if (item?.name) {
          const prev = out.get(item.id);
          out.set(item.id, { item, count: (prev?.count ?? 0) + (o.count - doneCount(o, p)), key: false });
        }
      }
      // Keys don't add up: three objectives behind one door still want one key.
      for (const group of o.keys) {
        const k = group[0];
        if (k?.name && !out.has(k.id)) out.set(k.id, { item: k, count: 1, key: true });
      }
    }
  }
  return [...out.values()].sort(
    (a, b) => Number(a.key) - Number(b.key) || a.item.name.localeCompare(b.item.name),
  );
}

/**
 * Route order through the objectives: the shortest path from where you are through every stop
 * (all orders are tried for up to 7 stops; beyond that, nearest first). Possible spawns count
 * as one stop: the nearest of them.
 */
export function routeOrder(markers: QuestMarker[], start: [number, number] | null): Map<string, number> {
  const stops = new Map<string, QuestMarker>();
  for (const m of markers) {
    const cur = stops.get(m.objective);
    if (!cur || (m.distanceM ?? Infinity) < (cur.distanceM ?? Infinity)) stops.set(m.objective, m);
  }
  const list = [...stops.values()];
  const from = start ?? (list[0] ? [list[0].px, list[0].py] : [0, 0]);
  const dist = (a: [number, number], b: QuestMarker) => Math.hypot(b.px - a[0], b.py - a[1]);
  const length = (path: QuestMarker[]) =>
    path.reduce((sum, m, i) => {
      const prev = path[i - 1];
      return sum + dist(prev ? [prev.px, prev.py] : (from as [number, number]), m);
    }, 0);
  let best: QuestMarker[] = [];
  if (list.length <= 7) {
    let bestLen = Infinity;
    const permute = (done: QuestMarker[], left: QuestMarker[]) => {
      if (left.length === 0) {
        const len = length(done);
        if (len < bestLen) [best, bestLen] = [done, len];
        return;
      }
      left.forEach((m, i) => permute([...done, m], [...left.slice(0, i), ...left.slice(i + 1)]));
    };
    permute([], list);
  } else {
    const left = [...list];
    let at = from as [number, number];
    while (left.length > 0) {
      let i = 0;
      left.forEach((m, j) => {
        const cur = left[i];
        if (cur && dist(at, m) < dist(at, cur)) i = j;
      });
      const [m] = left.splice(i, 1);
      if (!m) break;
      best.push(m);
      at = [m.px, m.py];
    }
  }
  return new Map(best.map((m, i) => [m.objective, i + 1]));
}

/** Active quests whose required objectives are all ticked: time to see the trader. */
export function readyToHandIn(db: QuestDb, p: Progress): Quest[] {
  return db.quests.filter((q) => {
    if (p.quests[q.id] !== "active") return false;
    const [done, total] = progressOf(q, p, "active");
    return total > 0 && done === total;
  });
}

export interface ItemToKeep {
  item: ItemRef;
  count: number;
  fir: boolean;
  quests: Quest[];
}

/** Items your quests still ask to hand over: active ones, plus those you can start if asked. */
export function itemsToKeep(db: QuestDb, p: Progress, withAvailable: boolean): ItemToKeep[] {
  const idx = questIndex(db);
  const out = new Map<string, ItemToKeep>();
  for (const q of db.quests) {
    if (!forFaction(q, p)) continue;
    const s = statusOf(q, p, idx);
    if (s !== "active" && !(withAvailable && s === "available")) continue;
    for (const o of q.objectives) {
      // Only handing over: the matching "find" objectives ask for the same items again.
      if (o.kind !== "giveItem" || !o.items[0]?.name) continue;
      const left = o.count - doneCount(o, p);
      if (left <= 0) continue;
      const item = o.items[0];
      const key = `${item.id}:${o.fir}`;
      const cur = out.get(key) ?? { item, count: 0, fir: o.fir, quests: [] };
      cur.count += left;
      if (!cur.quests.includes(q)) cur.quests.push(q);
      out.set(key, cur);
    }
  }
  return [...out.values()].sort((a, b) => Number(b.fir) - Number(a.fir) || b.count - a.count);
}

/** Completed / total quests needed for the Kappa container and for Lightkeeper. */
export function collectorProgress(db: QuestDb, p: Progress) {
  const pool = db.quests.filter((q) => forFaction(q, p));
  const count = (pick: (q: Quest) => boolean) => {
    const all = pool.filter(pick);
    return { done: all.filter((q) => p.quests[q.id] === "completed").length, total: all.length };
  };
  return { kappa: count((q) => q.kappa), lightkeeper: count((q) => q.lightkeeper) };
}

/** Objectives you can do in a raid on `map` for one quest. */
export const raidObjectives = (q: Quest, map: string) =>
  q.objectives.filter((o) => inRaid(o) && onMap(o, map));

// Distinct, readable on dark and light maps; amber is left for "quest" in general.
const COLORS = ["#4fa3f7", "#3dd68c", "#f76b6b", "#c084fc", "#2dd4bf", "#f472b6", "#a3e635", "#fb923c"];

/** Colour of each active quest on a map, stable while the set doesn't change. */
export function questColors(questIds: string[]): Map<string, string> {
  const sorted = [...new Set(questIds)].sort();
  return new Map(sorted.map((id, i) => [id, COLORS[i % COLORS.length] ?? "#4fa3f7"]));
}
