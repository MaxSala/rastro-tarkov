// Quest rules, kept apart from the interface so every view agrees: what state a quest is in,
// why it's locked, how far along it is and which items it still needs.
import type { Progress } from "./questProtocol";
import { questIndex, type Objective, type Quest, type QuestDb, type QuestIndex } from "./quests";

export type QStatus = "active" | "completed" | "failed" | "available" | "locked";

/** Objective types that happen during a raid (the rest are done at a trader or in the menu). */
const IN_RAID = new Set([
  "visit",
  "mark",
  "plantItem",
  "plantQuestItem",
  "findItem",
  "findQuestItem",
  "shoot",
  "extract",
  "useItem",
  "experience",
]);
export const inRaid = (o: Objective) => IN_RAID.has(o.kind);

/** Items handed over or found for a quest (not quest items, which can't be bought or sold). */
const ITEM_KINDS = new Set(["giveItem", "findItem"]);

function satisfied(status: string[], rec: Progress["quests"][string] | undefined): boolean {
  return status.some(
    (s) =>
      (s === "complete" && rec === "completed") ||
      (s === "active" && (rec === "active" || rec === "completed")) ||
      (s === "failed" && rec === "failed"),
  );
}

/** Why a quest isn't available yet. Unknown prerequisites (not in the data) don't block. */
export function blockers(q: Quest, p: Progress, idx: QuestIndex): { level: number | null; quests: Quest[] } {
  const quests = q.requires
    .filter((r) => idx.byId.has(r.quest) && !satisfied(r.status, p.quests[r.quest]))
    .map((r) => idx.byId.get(r.quest))
    .filter((x): x is Quest => Boolean(x));
  return { level: q.level > p.level ? q.level : null, quests };
}

export function statusOf(q: Quest, p: Progress, idx: QuestIndex): QStatus {
  const rec = p.quests[q.id];
  if (rec) return rec;
  const b = blockers(q, p, idx);
  return b.level === null && b.quests.length === 0 ? "available" : "locked";
}

/** Quests for the other faction are hidden once the player has chosen one. */
export const forFaction = (q: Quest, p: Progress) =>
  q.faction === "any" || p.faction === "any" || q.faction === p.faction || Boolean(p.quests[q.id]);

export const doneCount = (o: Objective, p: Progress) => Math.min(p.objectives[o.id] ?? 0, o.count);

export function objectiveDone(o: Objective, p: Progress, status: QStatus): boolean {
  return status === "completed" || doneCount(o, p) >= o.count;
}

/** Required objectives done / total. */
export function progressOf(q: Quest, p: Progress, status: QStatus): [number, number] {
  const req = q.objectives.filter((o) => !o.optional);
  return [req.filter((o) => objectiveDone(o, p, status)).length, req.length];
}

/** Every quest before this one that isn't completed yet (mirrors src/quests/actions.rs). */
export function chainBefore(q: Quest, p: Progress, idx: QuestIndex): Quest[] {
  const seen = new Set<string>();
  const out: Quest[] = [];
  const stack = [q];
  while (stack.length > 0 && out.length < 3000) {
    const cur = stack.pop();
    for (const r of cur?.requires ?? []) {
      if (!r.status.includes("complete") || r.quest === q.id || seen.has(r.quest)) continue;
      seen.add(r.quest);
      const prev = idx.byId.get(r.quest);
      if (!prev) continue;
      stack.push(prev);
      if (p.quests[prev.id] !== "completed") out.push(prev);
    }
  }
  return out;
}

/** Maps a quest takes place on: its own and those of its objectives. */
export function mapsOf(q: Quest): string[] {
  const set = new Set<string>(q.map ? [q.map] : []);
  for (const o of q.objectives) for (const m of o.maps) set.add(m);
  return [...set];
}

/** Open in-raid objectives of active quests that can be done on `map` (or on any map). */
export function openHere(db: QuestDb, p: Progress, map: string) {
  const here: { quest: Quest; o: Objective }[] = [];
  const anywhere: { quest: Quest; o: Objective }[] = [];
  let elsewhere = 0;
  for (const q of db.quests) {
    if (p.quests[q.id] !== "active") continue;
    let counted = false;
    for (const o of q.objectives) {
      if (!inRaid(o) || objectiveDone(o, p, "active")) continue;
      if (o.maps.includes(map)) here.push({ quest: q, o });
      else if (o.maps.length === 0) anywhere.push({ quest: q, o });
      else if (!counted) {
        counted = true;
        elsewhere += 1;
      }
    }
  }
  return { here, anywhere, elsewhere };
}

/** Open objectives of active quests per map, for "where to go next". */
export function byMap(db: QuestDb, p: Progress): Map<string, number> {
  const out = new Map<string, number>();
  for (const q of db.quests) {
    if (p.quests[q.id] !== "active") continue;
    for (const o of q.objectives) {
      if (!inRaid(o) || objectiveDone(o, p, "active")) continue;
      for (const m of o.maps.length > 0 ? o.maps : [""]) out.set(m, (out.get(m) ?? 0) + 1);
    }
  }
  return out;
}

export interface ItemNeed {
  quest: Quest;
  objective: Objective;
  count: number;
  status: QStatus;
}

/** Quests not completed yet that ask for this item, active ones first. */
export function itemNeeds(db: QuestDb, p: Progress, itemId: string): ItemNeed[] {
  const idx = questIndex(db);
  const out: ItemNeed[] = [];
  for (const q of db.quests) {
    if (!forFaction(q, p)) continue;
    const status = statusOf(q, p, idx);
    if (status === "completed" || status === "failed") continue;
    for (const o of q.objectives) {
      if (!ITEM_KINDS.has(o.kind) || !o.items.some((i) => i.id === itemId)) continue;
      const left = o.count - doneCount(o, p);
      if (left > 0) out.push({ quest: q, objective: o, count: left, status });
    }
  }
  const rank = { active: 0, available: 1, locked: 2, completed: 3, failed: 3 } as const;
  return out.sort((a, b) => rank[a.status] - rank[b.status] || a.quest.level - b.quest.level);
}
