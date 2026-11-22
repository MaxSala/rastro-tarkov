import { defineCollection } from "astro:content";
import { docsLoader } from "@astrojs/starlight/loaders";
import { docsSchema } from "@astrojs/starlight/schema";

// The pages themselves are generated from ../docs by scripts/sync-docs.mjs.
export const collections = { docs: defineCollection({ loader: docsLoader(), schema: docsSchema() }) };
