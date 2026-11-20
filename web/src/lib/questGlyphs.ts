// What each quest objective looks like on the map and in lists: one glyph per kind of task (go
// somewhere, mark, plant, find, shoot, extract). Shapes from Lucide (ISC licence), kept as data
// so the map can build DOM nodes and React can render them, never markup strings.

export type Glyph = "visit" | "mark" | "plant" | "find" | "shoot" | "extract" | "other";

/** Objective type (tarkov.dev) → glyph. */
export function glyphOf(kind: string | undefined): Glyph {
  switch (kind) {
    case "visit":
      return "visit";
    case "mark":
      return "mark";
    case "plantItem":
    case "plantQuestItem":
      return "plant";
    case "findItem":
    case "findQuestItem":
      return "find";
    case "shoot":
      return "shoot";
    case "extract":
      return "extract";
    default:
      return "other";
  }
}

export type Shape = ["path", string] | ["circle", number, number, number];
export const SHAPES: Record<Glyph, Shape[]> = {
  visit: [
    ["path", "M4 15s1-1 4-1 5 2 8 2 4-1 4-1V3s-1 1-4 1-5-2-8-2-4 1-4 1z"],
    ["path", "M4 22v-7"],
  ],
  mark: [
    [
      "path",
      "M20 10c0 4.993-5.539 10.193-7.399 11.799a1 1 0 0 1-1.202 0C9.539 20.193 4 14.993 4 10a8 8 0 0 1 16 0",
    ],
    ["circle", 12, 10, 3],
  ],
  plant: [
    ["path", "M12 17V3"],
    ["path", "m6 11 6 6 6-6"],
    ["path", "M19 21H5"],
  ],
  find: [
    ["circle", 11, 11, 8],
    ["path", "m21 21-4.3-4.3"],
  ],
  shoot: [
    ["circle", 12, 12, 10],
    ["path", "M22 12h-4"],
    ["path", "M6 12H2"],
    ["path", "M12 6V2"],
    ["path", "M12 22v-4"],
  ],
  extract: [
    ["path", "M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"],
    ["path", "m16 17 5-5-5-5"],
    ["path", "M21 12H9"],
  ],
  other: [
    ["path", "M12 5v9"],
    ["path", "M12 19h.01"],
  ],
};
