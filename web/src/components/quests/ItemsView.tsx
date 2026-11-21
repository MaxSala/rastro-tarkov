import { useMemo, useState } from "react";
import { t } from "../../locales";
import { int } from "../../lib/format";
import { keepList, type Scope, type Source } from "../../lib/hideout";
import type { Progress } from "../../lib/questProtocol";
import type { QuestDb } from "../../lib/quests";
import { useApp } from "../../store/app";
import { ItemIcon } from "../market/ItemIcon";
import { Segmented } from "../ui/segmented";

/**
 * What your quests and your hideout still ask you to hand over: the list of things not to sell,
 * found in raid first. Shared by the Quests and Hideout sections.
 */
export function ItemsView({
  db,
  p,
  initialSource = "all",
}: {
  db: QuestDb;
  p: Progress;
  initialSource?: Source;
}) {
  const [source, setSource] = useState<Source>(initialSource);
  const [scope, setScope] = useState<Scope>("next");
  const [withAvailable, setWithAvailable] = useState(false);
  const hasHideout = db.hideout.length > 0;
  const shown: Source = hasHideout ? source : "quests";
  const list = useMemo(
    () => keepList(db, p, { source: shown, scope, withAvailable }),
    [db, p, shown, scope, withAvailable],
  );
  return (
    <div className="grid content-start gap-3 p-4">
      <p className="text-xs text-fg-muted">{hasHideout ? t.hideout.itemsHint : t.quests.items.hint}</p>
      <div className="flex flex-wrap items-center gap-3">
        {hasHideout && (
          <Segmented
            label={t.hideout.needs}
            value={source}
            onChange={setSource}
            options={(["all", "quests", "hideout"] as const).map((v) => ({
              value: v,
              label: t.hideout.source[v],
            }))}
          />
        )}
        {shown !== "quests" && (
          <Segmented
            label={t.hideout.title}
            value={scope}
            onChange={setScope}
            options={(["next", "all"] as const).map((v) => ({ value: v, label: t.hideout.scope[v] }))}
          />
        )}
        {shown !== "hideout" && (
          <label className="flex items-center gap-2 text-xs text-fg-soft">
            <input
              type="checkbox"
              checked={withAvailable}
              onChange={(e) => setWithAvailable(e.target.checked)}
              className="size-3.5 accent-[var(--accent)]"
            />
            {t.quests.items.withAvailable}
          </label>
        )}
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
                  {[
                    ...n.quests.map((q) => q.name),
                    ...n.stations.map((u) => t.hideout.station(u.station.name, u.level)),
                  ].join(", ")}
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
