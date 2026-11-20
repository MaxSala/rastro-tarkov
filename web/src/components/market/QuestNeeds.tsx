import { useMemo } from "react";
import { t } from "../../locales";
import { itemNeeds } from "../../lib/questLogic";
import { useAutoQuests, useQuests } from "../../lib/quests";
import { useApp } from "../../store/app";
import { StatusDot } from "../quests/bits";

const MAX = 5;

/** Quests that still ask for this item: a reason not to sell it. */
export function QuestNeeds({ id }: { id: string }) {
  const { db } = useAutoQuests();
  const progress = useQuests((s) => s.progress);
  const needs = useMemo(() => (db && progress ? itemNeeds(db, progress, id) : []), [db, progress, id]);
  if (needs.length === 0) return null;
  const active = needs.filter((n) => n.status === "active").reduce((sum, n) => sum + n.count, 0);
  return (
    <section className="grid gap-1">
      <h3 className="text-xs font-medium text-fg-muted">{t.quests.needs.title}</h3>
      {active > 0 && <p className="text-xs text-accent">{t.quests.needs.active(active)}</p>}
      <ul className="grid gap-px overflow-hidden rounded-sm border border-line bg-line">
        {needs.slice(0, MAX).map((n) => (
          <li key={n.objective.id} className="bg-app">
            <button
              type="button"
              onClick={() => useApp.getState().openQuest(n.quest.id)}
              className="flex w-full items-center gap-2.5 px-2.5 py-1.5 text-left text-sm hover:bg-hover"
            >
              <StatusDot status={n.status} />
              <span className="min-w-0 flex-1 truncate">{n.quest.name}</span>
              {n.objective.fir && <span className="shrink-0 text-2xs text-accent">{t.quests.firShort}</span>}
              <span className="num shrink-0 text-xs text-fg-soft">×{n.count}</span>
            </button>
          </li>
        ))}
      </ul>
      {needs.length > MAX && (
        <p className="text-2xs text-fg-muted">{t.quests.needs.more(needs.length - MAX)}</p>
      )}
    </section>
  );
}
