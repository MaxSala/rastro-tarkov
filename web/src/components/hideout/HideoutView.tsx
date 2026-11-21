import { CloudDownload } from "lucide-react";
import { useMemo, useState } from "react";
import { t } from "../../locales";
import { cn } from "../../lib/cn";
import { built, maxLevel, nextLevel } from "../../lib/hideout";
import { icon } from "../../lib/icons";
import type { Progress } from "../../lib/questProtocol";
import type { QuestDb } from "../../lib/quests";
import { Button } from "../ui/button";
import { ItemsView } from "../quests/ItemsView";
import { QuestDataStatus, QuestGate, refreshQuests } from "../quests/QuestGate";
import { StationCard } from "./StationCard";

type Tab = "todo" | "all" | "items";
const TABS: Tab[] = ["todo", "all", "items"];

/** The hideout: the level of each station and what the next one asks for. */
export function HideoutView() {
  return <QuestGate>{(db, p) => <Hideout db={db} p={p} />}</QuestGate>;
}

function Hideout({ db, p }: { db: QuestDb; p: Progress }) {
  const [tab, setTab] = useState<Tab>("todo");
  const names = useMemo(() => new Map(db.hideout.map((s) => [s.id, s.name])), [db]);
  const maxed = db.hideout.filter((s) => built(p, s) >= maxLevel(s)).length;
  const list = tab === "todo" ? db.hideout.filter((s) => nextLevel(p, s)) : db.hideout;

  if (db.hideout.length === 0)
    return (
      <div className="grid h-full place-items-center p-6">
        <div className="grid max-w-sm justify-items-center gap-3 text-center">
          <p className="text-sm text-fg-soft">{t.hideout.noData}</p>
          <Button variant="primary" onClick={refreshQuests}>
            <CloudDownload {...icon} />
            {t.quests.data.refresh}
          </Button>
        </div>
      </div>
    );

  return (
    <div className="grid h-full grid-rows-[auto_1fr]">
      <header className="flex flex-wrap items-center gap-3 border-b border-line px-4 py-3">
        <h1 className="text-sm font-medium">{t.hideout.title}</h1>
        <span className="num text-xs text-fg-muted">{t.hideout.summary(maxed, db.hideout.length)}</span>
        <div role="radiogroup" aria-label={t.hideout.title} className="flex gap-1">
          {TABS.map((s) => (
            <button
              key={s}
              type="button"
              role="radio"
              aria-checked={tab === s}
              onClick={() => setTab(s)}
              className={cn(
                "h-7 rounded-sm border px-2.5 text-xs transition-colors",
                tab === s
                  ? "border-line-strong bg-active text-fg"
                  : "border-transparent text-fg-muted hover:text-fg",
              )}
            >
              {t.hideout.tabs[s]}
            </button>
          ))}
        </div>
        <div className="flex-1" />
        <QuestDataStatus />
      </header>
      <div className="min-h-0 overflow-y-auto">
        {tab === "items" ? (
          <ItemsView db={db} p={p} initialSource="hideout" />
        ) : list.length === 0 ? (
          <p className="py-16 text-center text-sm text-fg-muted">{t.hideout.allDone}</p>
        ) : (
          <>
            <p className="px-4 pt-3 text-xs text-fg-muted">{t.hideout.hint}</p>
            <div className="grid grid-cols-[repeat(auto-fill,minmax(300px,1fr))] items-start gap-3 p-4">
              {list.map((s) => (
                <StationCard key={s.id} s={s} p={p} names={names} />
              ))}
            </div>
          </>
        )}
      </div>
    </div>
  );
}
