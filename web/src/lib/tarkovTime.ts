// Time in Tarkov and raid timings, worked out from the clock: nothing is read from the game.
//
// Tarkov time runs 7 times faster than real time, on Moscow time (UTC+3); the right side of
// the raid selection is 12 hours ahead of the left (formula as in adamburgess/tarkov-time, MIT).

const DAY = 24 * 3600 * 1000;
const RATIO = 7;
const MOSCOW = 3 * 3600 * 1000;

export type Side = "left" | "right";

/** Local time of a log or screenshot stamp (`AAAA-MM-DD HH:MM:SS`, no zone, like the game). */
export const localMs = (stamp: string): number => new Date(stamp.replace(" ", "T")).getTime();

/** Hours (0–24) on one side of the raid selection at a real instant (ms since 1970). */
export function tarkovHours(nowMs: number, side: Side): number {
  const offset = MOSCOW + (side === "right" ? 12 * 3600 * 1000 : 0);
  const ms = (((offset + nowMs * RATIO) % DAY) + DAY) % DAY;
  return ms / 3600_000;
}

export function hm(hours: number): string {
  const total = Math.floor((((hours % 24) + 24) % 24) * 60);
  return `${String(Math.floor(total / 60)).padStart(2, "0")}:${String(total % 60).padStart(2, "0")}`;
}

/** Roughly when it's light out in a raid. */
export const isDay = (hours: number): boolean => hours >= 5 && hours < 21.5;

const gap = (a: number, b: number) => Math.min(Math.abs(a - b), 24 - Math.abs(a - b));

/**
 * The side of your raid, from a screenshot: its in-game hour against what each side showed
 * when it was taken. `null` when neither is close (a different raid, or a map with fixed time).
 */
export function sideFrom(shotHours: number, takenAtMs: number): Side | null {
  const l = gap(shotHours, tarkovHours(takenAtMs, "left"));
  const r = gap(shotHours, tarkovHours(takenAtMs, "right"));
  const best = Math.min(l, r);
  if (best > 0.75) return null;
  return l <= r ? "left" : "right";
}

/** Maps whose time never changes: no clock to show. */
export const FIXED_TIME = new Set(["factory", "the-lab", "the-labyrinth"]);

/**
 * Maps with an armored train. It arrives between 20 and 15 minutes before the end of the raid
 * and stays 7 minutes (Escape from Tarkov wiki, Reserve and Lighthouse pages), so it follows
 * the length of the raid in PvP and PvE alike.
 */
export const TRAIN_MAPS = new Set(["reserve", "lighthouse"]);
const TRAIN = { earliest: 20 * 60, latest: 15 * 60, stays: 7 * 60 };

export type Train =
  { state: "soon"; from: number; to: number } | { state: "maybe"; leaveBy: number } | { state: "gone" };

/** The train from the seconds left in the raid; times are seconds from now. */
export function train(left: number): Train {
  if (left > TRAIN.earliest) return { state: "soon", from: left - TRAIN.earliest, to: left - TRAIN.latest };
  const leaveBy = left - TRAIN.latest + TRAIN.stays;
  if (leaveBy > 0 && left > 0) return { state: "maybe", leaveBy: Math.min(leaveBy, left) };
  return { state: "gone" };
}

/** "m:ss" or "h:mm:ss". */
export function clock(totalSeconds: number): string {
  const s = Math.max(0, Math.floor(totalSeconds));
  const h = Math.floor(s / 3600);
  const mm = String(Math.floor((s % 3600) / 60)).padStart(h > 0 ? 2 : 1, "0");
  const ss = String(s % 60).padStart(2, "0");
  return h > 0 ? `${h}:${mm}:${ss}` : `${mm}:${ss}`;
}
