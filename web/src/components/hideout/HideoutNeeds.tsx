import { Warehouse } from "lucide-react";
import { useMemo } from "react";
import { t } from "../../locales";
import { stationsNeeding } from "../../lib/hideout";
import { iconSm } from "../../lib/icons";
import { useAutoQuests, useQuests } from "../../lib/quests";
import { useApp } from "../../store/app";

/** Hideout levels still to build that ask for this item: another reason not to sell it. */
export function HideoutNeeds({ id }: { id: string }) {
  const { db } = useAutoQuests();
  const progress = useQuests((s) => s.progress);
  const needs = useMemo(() => (db && progress ? stationsNeeding(db, progress, id) : []), [db, progress, id]);
  if (needs.length === 0) return null;
  return (
    <section className="grid gap-1">
      <h3 className="text-xs font-medium text-fg-muted">{t.hideout.needs}</h3>
      <ul className="grid gap-px overflow-hidden rounded-sm border border-line bg-line">
        {needs.slice(0, 6).map(({ use, count }) => (
          <li key={`${use.station.id}:${use.level}`} className="bg-app">
            <button
              type="button"
              onClick={() => useApp.getState().setWorkspace("hideout")}
              className="flex w-full items-center gap-2.5 px-2.5 py-1.5 text-left text-sm hover:bg-hover"
            >
              <Warehouse {...iconSm} className="shrink-0 text-fg-muted" aria-hidden />
              <span className="min-w-0 flex-1 truncate">
                {t.hideout.station(use.station.name, use.level)}
              </span>
              <span className="num shrink-0 text-xs text-fg-soft">×{count}</span>
            </button>
          </li>
        ))}
      </ul>
    </section>
  );
}
