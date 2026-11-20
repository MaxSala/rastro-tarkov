// Downloaded data in another language than the interface. Source of truth: src/langdata.rs.
import { z } from "zod";
import { LANGS } from "../locales";

export const languageDataSchema = z.object({
  /** What is in another language, and in which (`null`: maps installed by an earlier build). */
  outdated: z
    .array(z.object({ kind: z.enum(["maps", "prices", "quests"]), lang: z.enum(LANGS).nullable() }))
    .max(3),
  /** Update it on its own after a language change; `null` until the player chooses. */
  auto: z.boolean().nullable(),
});
