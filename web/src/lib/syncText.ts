// Progress and summary of the tarkov.dev map download, written in the interface language.
import { t } from "../locales";
import { say } from "./msg";
import type { View } from "./protocol";

export function syncText(s: View["tarkovdev"]): string {
  if (s.state === "error") return say(s.error);
  if (s.state === "running") {
    if (s.step === "install") return t.sync.install(s.current ?? "");
    return s.step ? t.sync[s.step] : t.sync.maps;
  }
  if (s.state !== "done") return "";
  const parts = [t.sync.done(s.installed)];
  if (s.failed.length > 0) parts.push(t.sync.failed(s.failed.join(", ")));
  if (s.iconsMissing > 0) parts.push(t.sync.iconsMissing(s.iconsMissing));
  if (s.noData) parts.push(t.sync.noData);
  return parts.join(" ");
}
