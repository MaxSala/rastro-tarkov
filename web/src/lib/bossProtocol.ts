// Bosses of the map shown, in every snapshot. Source of truth: src/quests/bossview.rs.
import { z } from "zod";

const num = z.number().finite();
const chance = z.number().min(0).max(1);
const name = z.string().max(60);

export const bossMarkerSchema = z.object({
  id: z.string().regex(/^bz-\d{1,3}-\d{1,2}$/),
  boss: name,
  zone: name,
  chance,
  zoneChance: chance,
  px: num,
  py: num,
  floor: z
    .string()
    .regex(/^[a-z0-9-]{1,40}$/)
    .optional(),
  points: z.array(z.tuple([num, num])).max(64),
  escorts: z.array(name).max(6),
  time: z.number().int().min(1).max(7200).optional(),
  triggered: z.boolean(),
});

export const bossesViewSchema = z.object({
  markers: z.array(bossMarkerSchema).max(200),
  list: z
    .array(
      z.object({
        name,
        chance,
        time: z.number().int().min(1).max(7200).optional(),
        triggered: z.boolean(),
      }),
    )
    .max(400),
});

export type BossMarker = z.infer<typeof bossMarkerSchema>;
export type BossesView = z.infer<typeof bossesViewSchema>;
export type BossLine = BossesView["list"][number];

/** "45 %" style, rounded; "<1 %" for tiny chances. */
export function pct(c: number): string {
  const v = Math.round(c * 100);
  return v < 1 ? "<1 %" : `${v} %`;
}
