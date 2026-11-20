// Quest data (src/quests/mod.rs: QuestDb, Quest, Objective) and the player's progress.
// The data arrives over HTTP (/api/quests), validated with zod; the progress comes over the
// WebSocket whenever it changes.
import { useEffect } from "react";
import { z } from "zod";
import { create } from "zustand";
import { api } from "./api";
import { spotKindSchema, type Progress } from "./questProtocol";
import { useApp } from "../store/app";

const num = z.number().finite();
const hexId = z.string().regex(/^[0-9a-f]{24}$/);
const slug = z.string().regex(/^[a-z0-9-]{1,40}$/);
const name = z.string().max(160);
const itemRef = z.object({ id: hexId, name });

const objectiveSchema = z.object({
  id: hexId,
  kind: z.string().regex(/^[a-zA-Z]{1,24}$/),
  text: z.string().max(400),
  optional: z.boolean(),
  count: z.number().int().min(1).max(100_000),
  fir: z.boolean(),
  maps: z.array(slug).max(16),
  items: z.array(itemRef).max(16),
  keys: z.array(z.array(itemRef).max(8)).max(8),
  positions: z
    .array(
      z.object({
        map: slug,
        x: num,
        y: num,
        z: num,
        outline: z
          .array(z.tuple([num, num]))
          .max(48)
          .optional(),
        kind: spotKindSchema.default("exact"),
      }),
    )
    .max(64),
});

const short = z.string().max(40);
const rewardsSchema = z.object({
  experience: z.number().int().min(0),
  standing: z.array(z.tuple([short, num])).max(16),
  items: z.array(z.object({ id: hexId, name, count: z.number().int().min(1) })).max(16),
  offers: z
    .array(z.object({ id: hexId, name, trader: short, level: z.number().int().min(1).max(10) }))
    .max(16),
  skills: z.array(z.tuple([short, num])).max(16),
  traders: z.array(short).max(16),
  crafts: z.array(itemRef).max(16),
});

const questSchema = z.object({
  id: hexId,
  name,
  slug: z.string().regex(/^[a-z0-9-]{0,80}$/),
  trader: slug,
  map: slug.nullable(),
  level: z.number().int().min(0).max(100),
  kappa: z.boolean(),
  lightkeeper: z.boolean(),
  faction: z.enum(["any", "usec", "bear"]),
  wiki: z
    .string()
    .max(300)
    .regex(/^https:\/\/escapefromtarkov\.fandom\.com\//)
    .nullable(),
  requires: z.array(z.object({ quest: hexId, status: z.array(z.string().max(20)).max(4) })).max(16),
  loyalty: z.array(z.tuple([z.string().max(40), z.number().int().min(1).max(10)])).max(4),
  objectives: z.array(objectiveSchema).max(40),
  rewards: rewardsSchema,
  image: z.boolean(),
});

const dbSchema = z.object({
  mode: z.string().max(10),
  lang: z.string().max(5),
  fetchedAt: z.number().int(),
  traders: z.array(z.object({ id: slug, name, image: hexId.nullable().optional() })).max(64),
  quests: z.array(questSchema).max(3000),
});

export type QuestDb = z.infer<typeof dbSchema>;
export type Quest = z.infer<typeof questSchema>;
export type Objective = Quest["objectives"][number];
export type ItemRef = z.infer<typeof itemRef>;
export type Rewards = Quest["rewards"];

interface QuestState {
  db: QuestDb | null;
  loadedAt: number | null;
  loading: boolean;
  error: string | null;
  progress: Progress | null;
  load: (fetchedAt: number) => Promise<void>;
  setProgress: (p: Progress) => void;
}

export const useQuests = create<QuestState>((set, get) => ({
  db: null,
  loadedAt: null,
  loading: false,
  error: null,
  progress: null,
  load: async (fetchedAt) => {
    if (get().loading || get().loadedAt === fetchedAt) return;
    set({ loading: true, error: null });
    try {
      const raw: unknown = await api.json("/api/quests");
      const parsed = dbSchema.safeParse(raw);
      if (!parsed.success) throw new Error(parsed.error.issues[0]?.message ?? "invalid quests");
      set({ db: parsed.data, loadedAt: parsed.data.fetchedAt, loading: false });
    } catch (e) {
      set({ loading: false, error: e instanceof Error ? e.message : String(e) });
    }
  },
  setProgress: (progress) => set({ progress }),
}));

/** Loads the quest data when the backend has it (or after a download). */
export function useAutoQuests(enabled = true) {
  const status = useApp((s) => s.view?.quests);
  const db = useQuests((s) => s.db);
  const loading = useQuests((s) => s.loading);
  const error = useQuests((s) => s.error);
  const load = useQuests((s) => s.load);
  useEffect(() => {
    if (enabled && status?.fetchedAt && (status.state === "ready" || status.state === "loading"))
      void load(status.fetchedAt);
  }, [enabled, status?.fetchedAt, status?.state, load]);
  // A change of mode or language makes the loaded data stale until the right one is loaded.
  const fresh = db && status?.fetchedAt === db.fetchedAt ? db : null;
  return { db: fresh, loading, error, status };
}

export interface QuestIndex {
  byId: Map<string, Quest>;
  /** Quest each objective belongs to. */
  owner: Map<string, Quest>;
  /** Quests that need this one first. */
  unlocks: Map<string, Quest[]>;
  traders: Map<string, string>;
  /** Trader slug → the id of its portrait. */
  portraits: Map<string, string>;
}

const indexes = new WeakMap<QuestDb, QuestIndex>();
/** Lookups built once per data set. */
export function questIndex(db: QuestDb): QuestIndex {
  const cached = indexes.get(db);
  if (cached) return cached;
  const byId = new Map(db.quests.map((q) => [q.id, q]));
  const owner = new Map<string, Quest>();
  const unlocks = new Map<string, Quest[]>();
  for (const q of db.quests) {
    for (const o of q.objectives) owner.set(o.id, q);
    for (const r of q.requires) unlocks.set(r.quest, [...(unlocks.get(r.quest) ?? []), q]);
  }
  const portraits = new Map<string, string>();
  for (const t of db.traders) if (t.image) portraits.set(t.id, t.image);
  const idx = { byId, owner, unlocks, traders: new Map(db.traders.map((t) => [t.id, t.name])), portraits };
  indexes.set(db, idx);
  return idx;
}
