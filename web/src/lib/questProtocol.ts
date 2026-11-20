// Quest parts of the WebSocket protocol. Source of truth: src/quests/view.rs (QuestMarker,
// ProgressView) and src/quests/mod.rs (QuestsStatus) in the backend.
import { z } from "zod";
import { userMsgSchema } from "./msg";

const num = z.number().finite();
/** tarkov.dev id: 24 hex digits. */
const hexId = z.string().regex(/^[0-9a-f]{24}$/);

export const questsStatusSchema = z.object({
  state: z.enum(["empty", "loading", "ready", "error"]),
  fetchedAt: z.number().int().nullable(),
  count: z.number().int(),
  mode: z.string().max(10).nullable(),
  lang: z.string().max(5).nullable(),
  error: userMsgSchema.nullable(),
  /** Following the game logs (src/quests/logsync.rs). */
  logs: z.object({
    enabled: z.boolean(),
    scanning: z.boolean(),
    known: z.number().int().min(0),
    sessions: z.number().int().min(0),
  }),
});

/** How precise a spot is: a place, an area to be in, or one of several possible spawns. */
export const spotKindSchema = z.enum(["exact", "area", "candidate"]);
export type SpotKind = z.infer<typeof spotKindSchema>;

export const questMarkerSchema = z.object({
  id: z.string().regex(/^qo-[0-9a-f]{24}-\d{1,2}$/),
  quest: hexId,
  objective: hexId,
  px: num,
  py: num,
  distanceM: num.nullable(),
  floor: z
    .string()
    .regex(/^[a-z0-9-]{1,40}$/)
    .optional(),
  focus: z.boolean().optional(),
  /** Area of the objective in map pixels, when it's a zone rather than a point. */
  outline: z
    .array(z.tuple([num, num]))
    .max(48)
    .optional(),
  kind: spotKindSchema,
});

export const progressSchema = z.object({
  mode: z.enum(["regular", "pve"]),
  level: z.number().int().min(1).max(79),
  faction: z.enum(["any", "usec", "bear"]),
  quests: z.record(hexId, z.enum(["active", "completed", "failed"])),
  objectives: z.record(hexId, z.number().int().min(0).max(100_000)),
  revision: z.number().int(),
});

export type QuestMarker = z.infer<typeof questMarkerSchema>;
export type Progress = z.infer<typeof progressSchema>;
export type Faction = Progress["faction"];
export type RecordStatus = Progress["quests"][string];

export type QuestClientMsg =
  | { type: "questsRefresh" }
  | { type: "questSet"; questId: string; status: RecordStatus | null }
  | { type: "questObjective"; objectiveId: string; count: number }
  | { type: "questsLevel"; level: number }
  | { type: "questsFaction"; faction: Faction }
  | { type: "questsCompleteBefore"; questId: string }
  | { type: "questsReset" }
  | { type: "questsReadLogs" }
  | { type: "questsCompleteThrough"; questIds: string[] }
  | { type: "questFocus"; questId: string | null };
