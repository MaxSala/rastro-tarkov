import { History, Loader2, RefreshCw, ScrollText } from "lucide-react";
import { t } from "../../locales";
import { cn } from "../../lib/cn";
import { collectorProgress } from "../../lib/questPlanning";
import type { Progress } from "../../lib/questProtocol";
import type { QuestDb } from "../../lib/quests";
import { send } from "../../lib/socket";
import { useApp } from "../../store/app";

const chip = "inline-flex h-6 items-center gap-1.5 rounded-sm px-1.5 text-xs";
const link = cn(chip, "text-fg-muted hover:bg-hover hover:text-fg");

/** Whether quests follow the game, how far along Kappa and Lightkeeper are, and catching up. */
export function QuestMeta({ db, p, onCatchUp }: { db: QuestDb; p: Progress; onCatchUp: () => void }) {
  const logs = useApp((s) => s.view?.quests.logs);
  const c = collectorProgress(db, p);
  const pct = (d: number, total: number) => (total > 0 ? Math.round((d / total) * 100) : 0);
  return (
    <div className="flex flex-wrap items-center gap-x-1 gap-y-1">
      {logs?.scanning ? (
        <span className={cn(chip, "text-fg-soft")}>
          <Loader2 size={12} className="animate-spin" aria-hidden />
          {t.quests.sync.scanning}
        </span>
      ) : logs?.enabled ? (
        <span className={cn(chip, "text-fg-soft")} title={t.quests.sync.known(logs.known)}>
          <span aria-hidden className="size-1.5 rounded-full bg-ok" />
          {t.quests.sync.on}
          <button
            type="button"
            title={t.quests.sync.reread}
            aria-label={t.quests.sync.reread}
            onClick={() => send({ type: "questsReadLogs" })}
            className="grid size-5 place-items-center rounded-sm text-fg-muted hover:bg-hover hover:text-fg"
          >
            <RefreshCw size={11} />
          </button>
        </span>
      ) : (
        <button
          type="button"
          title={t.quests.sync.offHint}
          onClick={() => send({ type: "setGameLogs", enabled: true })}
          className={link}
        >
          <ScrollText size={12} aria-hidden />
          {t.quests.sync.off}
        </button>
      )}
      <span aria-hidden className="mx-1 h-3 w-px bg-line-strong" />
      <span
        className={cn(chip, "text-fg-soft")}
        title={t.quests.collector.kappaHint(c.kappa.done, c.kappa.total)}
      >
        {t.quests.filters.kappa}
        <span className="num text-accent">{pct(c.kappa.done, c.kappa.total)} %</span>
      </span>
      <span
        className={cn(chip, "text-fg-soft")}
        title={t.quests.collector.lightkeeperHint(c.lightkeeper.done, c.lightkeeper.total)}
      >
        Lightkeeper
        <span className="num text-accent">{pct(c.lightkeeper.done, c.lightkeeper.total)} %</span>
      </span>
      <button type="button" onClick={onCatchUp} className={cn(link, "ml-auto")}>
        <History size={12} aria-hidden />
        {t.quests.catchUp.button}
      </button>
    </div>
  );
}
