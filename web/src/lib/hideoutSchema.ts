// Hideout stations as the backend sends them with the quest data (src/quests/hideout.rs).
// Only the schema lives here, so the quest data can use it without importing the logic.
import { z } from "zod";

const hexId = z.string().regex(/^[0-9a-f]{24}$/);
const level = z.number().int().min(1).max(10);
const short = z.string().max(40);

export const stationSchema = z.object({
  id: hexId,
  name: z.string().max(60),
  slug: z.string().regex(/^[a-z0-9-]{0,40}$/),
  levels: z
    .array(
      z.object({
        level,
        seconds: z.number().int().min(0),
        items: z
          .array(
            z.object({
              id: hexId,
              name: z.string().max(160),
              count: z.number().int().min(1),
              fir: z.boolean(),
            }),
          )
          .max(24),
        stations: z.array(z.tuple([hexId, level])).max(8),
        traders: z.array(z.tuple([short, level])).max(8),
        skills: z.array(z.tuple([short, z.number().int().min(1).max(51)])).max(8),
      }),
    )
    .max(10),
});

export type Station = z.infer<typeof stationSchema>;
export type StationLevel = Station["levels"][number];
