// Messages from the backend arrive with a stable code and an English fallback text.
// Here they are translated into the interface language; unknown codes fall back to the text.
import { z } from "zod";
import { t } from "../locales";

export const userMsgSchema = z.object({
  code: z.string().regex(/^[a-zA-Z]{1,40}$/),
  arg: z.string().max(500).nullable().optional(),
  text: z.string().max(2000),
});
export type UserMsg = z.infer<typeof userMsgSchema>;

type Entry = string | ((arg: string) => string);

export function say(m: UserMsg | null | undefined): string {
  if (!m) return "";
  const entry = (t.msg as Record<string, Entry | undefined>)[m.code];
  if (typeof entry === "string") return entry;
  if (typeof entry === "function") return entry(m.arg ?? "");
  return m.text;
}
