import { useMemo, useState } from "react";
import { t } from "../../locales";
import { int } from "../../lib/format";
import { itemsToKeep } from "../../lib/questPlanning";
import type { Progress } from "../../lib/questProtocol";
import type { QuestDb } from "../../lib/quests";
import { useApp } from "../../store/app";
import { ItemIcon } from "../market/ItemIcon";

/** What your quests still ask you to hand over: the list to keep, found in raid first. */
export function ItemsView({ db, p }: { db: QuestDb; p: Progress }) {
  const [withAvailable, setWithAvailable] = useState(false);
  const list = useMemo(() => itemsToKeep(db, p, withAvailable), [db, p, withAvailable]);
  return (
    <div className="grid content-start gap-3 p-4">
      <div className="flex flex-wrap items-center justify-between gap-2">
        <p className="text-xs text-fg-muted">{t.quests.items.hint}</p>
        <label className="flex items-center gap-2 text-xs text-fg-soft">
          <input
            type="checkbox"
            checked={withAvailable}
            onChange={(e) => setWithAvailable(e.target.checked)}
            className="size-3.5 accent-[var(--accent)]"
          />
          {t.quests.items.withAvailable}
        </label>
      </div>
      {list.length === 0 ? (
        <p className="py-10 text-center text-sm text-fg-muted">{t.quests.items.empty}</p>
      ) : (
        <ul className="grid gap-px overflow-hidden rounded-md border border-line bg-line">
          {list.map((n) => (
            <li key={`${n.item.id}:${n.fir}`} className="flex items-center gap-3 bg-surface px-3 py-2">
              <ItemIcon id={n.item.id} size={36} />
              <button
                type="button"
                title={t.keys.inMarket}
                onClick={() => useApp.getState().showInMarket(n.item.id)}
                className="min-w-0 flex-1 text-left"
              >
                <span className="block truncate text-sm">{n.item.name}</span>
                <span className="block truncate text-xs text-fg-muted">
                  {n.quests.map((q) => q.name).join(", ")}
                </span>
              </button>
              {n.fir && (
                <span
                  title={t.quests.fir}
                  className="shrink-0 rounded-sm bg-accent-soft px-1.5 py-0.5 text-2xs text-accent"
                >
                  {t.quests.firShort}
                </span>
              )}
              <span className="num w-14 shrink-0 text-right text-sm">×{int(n.count)}</span>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
