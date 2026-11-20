import { LayoutGrid } from "lucide-react";
import { t } from "../../locales";
import { cn } from "../../lib/cn";
import type { QStatus } from "../../lib/questLogic";
import type { QuestDb, QuestIndex } from "../../lib/quests";
import { useApp } from "../../store/app";
import { TraderAvatar } from "./media";

/** The traders, as the game shows them: pick one to see their quests. */
export function TraderRail({
  db,
  idx,
  statuses,
  trader,
}: {
  db: QuestDb;
  idx: QuestIndex;
  statuses: Map<string, QStatus>;
  trader: string;
}) {
  const setUi = useApp((s) => s.setQuestUi);
  const counts = new Map<string, { active: number; available: number }>();
  for (const q of db.quests) {
    const c = counts.get(q.trader) ?? { active: 0, available: 0 };
    const s = statuses.get(q.id);
    if (s === "active") c.active += 1;
    if (s === "available") c.available += 1;
    counts.set(q.trader, c);
  }
  const item = (on: boolean) =>
    cn(
      "relative grid size-11 place-items-center rounded-md border transition-colors",
      on ? "border-accent bg-accent-soft" : "border-transparent hover:bg-hover",
    );
  return (
    <nav
      aria-label={t.quests.filters.trader}
      className="flex flex-col items-center gap-1.5 overflow-y-auto border-r border-line py-3"
    >
      <button
        type="button"
        aria-pressed={trader === ""}
        title={t.quests.filters.allTraders}
        onClick={() => setUi({ trader: "" })}
        className={item(trader === "")}
      >
        <LayoutGrid size={16} className="text-fg-soft" aria-hidden />
      </button>
      {db.traders.map((tr) => {
        const c = counts.get(tr.id) ?? { active: 0, available: 0 };
        const on = trader === tr.id;
        return (
          <button
            key={tr.id}
            type="button"
            aria-pressed={on}
            title={t.quests.rail.hint(tr.name, c.active, c.available)}
            onClick={() => setUi({ trader: on ? "" : tr.id })}
            className={item(on)}
          >
            <TraderAvatar image={idx.portraits.get(tr.id) ?? null} name={tr.name} size={36} />
            {c.active + c.available > 0 && (
              <span
                className={cn(
                  "num absolute -top-1 -right-1 grid h-4 min-w-4 place-items-center rounded-full px-1 text-[10px] leading-none",
                  c.active > 0 ? "bg-accent text-accent-fg" : "bg-active text-fg-soft",
                )}
              >
                {c.active > 0 ? c.active : c.available}
              </span>
            )}
          </button>
        );
      })}
    </nav>
  );
}
