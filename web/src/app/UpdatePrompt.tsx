import { BellRing } from "lucide-react";
import { t } from "../locales";
import { iconSm } from "../lib/icons";
import { send } from "../lib/socket";
import { useApp } from "../store/app";
import { Button } from "../components/ui/button";

/** Asked once, after the first-run guide: may Rastro check GitHub for new versions on startup? */
export function UpdatePrompt() {
  const ask = useApp((s) => s.view !== null && s.view.settings.checkUpdates === null && !s.view.onboarding);
  if (!ask) return null;
  const answer = (enabled: boolean) => send({ type: "setCheckUpdates", enabled });
  return (
    <section
      aria-label={t.update.askTitle}
      className="fixed right-4 bottom-10 z-[900] grid w-80 gap-3 rounded-md border border-line-strong bg-surface p-4 shadow-[0_8px_32px_#0008]"
    >
      <div className="flex items-start gap-2.5">
        <BellRing {...iconSm} className="mt-0.5 shrink-0 text-accent" />
        <div className="grid gap-1">
          <h2 className="text-sm font-medium">{t.update.askTitle}</h2>
          <p className="text-xs text-fg-soft">{t.update.askBody}</p>
        </div>
      </div>
      <div className="flex justify-end gap-2">
        <Button size="sm" onClick={() => answer(false)}>
          {t.update.no}
        </Button>
        <Button size="sm" variant="primary" onClick={() => answer(true)}>
          {t.update.yes}
        </Button>
      </div>
    </section>
  );
}
