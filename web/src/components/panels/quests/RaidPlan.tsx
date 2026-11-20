import { Backpack, ChevronDown, KeyRound, X } from "lucide-react";
import { useState } from "react";
import { t } from "../../../locales";
import { cn } from "../../../lib/cn";
import { iconSm } from "../../../lib/icons";
import { useQuestPlan } from "../../../lib/questPlan";
import { kitFor } from "../../../lib/questPlanning";
import type { Progress } from "../../../lib/questProtocol";
import type { Quest } from "../../../lib/quests";
import { useApp } from "../../../store/app";
import { ItemIcon } from "../../market/ItemIcon";

/**
 * Before the raid: pick which quests you're doing on this map. The map then shows only those,
 * numbered in route order, and this says what to bring.
 */
export function RaidPlan({
  map,
  quests,
  plan,
  p,
  colors,
}: {
  map: string;
  quests: Quest[];
  plan: string[];
  p: Progress;
  colors: Map<string, string>;
}) {
  const [open, setOpen] = useState(plan.length > 0);
  const toggle = useQuestPlan((s) => s.toggle);
  const clear = useQuestPlan((s) => s.clear);
  if (quests.length === 0) return null;
  const planned = quests.filter((q) => plan.includes(q.id));
  const kit = kitFor(planned.length > 0 ? planned : [], p, map);
  return (
    <section className="border-b border-line">
      <div className="flex items-center gap-1 pr-2">
        <button
          type="button"
          aria-expanded={open}
          onClick={() => setOpen(!open)}
          className="flex min-w-0 flex-1 items-center gap-2 px-3 py-2.5 text-left text-xs font-medium"
        >
          <ChevronDown {...iconSm} className={cn("shrink-0 transition-transform", !open && "-rotate-90")} />
          <span className="truncate">{t.quests.plan.title}</span>
          {plan.length > 0 && (
            <span className="num rounded-full bg-accent-soft px-1.5 text-2xs text-accent">{plan.length}</span>
          )}
        </button>
        {plan.length > 0 && (
          <button
            type="button"
            onClick={() => clear(map)}
            className="flex h-6 items-center gap-1 rounded-sm px-1.5 text-2xs text-fg-muted hover:bg-hover hover:text-fg"
          >
            <X size={11} aria-hidden />
            {t.quests.plan.clear}
          </button>
        )}
      </div>
      {open && (
        <div className="grid gap-2 px-3 pb-3">
          <p className="text-2xs text-fg-muted">{t.quests.plan.hint}</p>
          <ul className="grid">
            {quests.map((q) => (
              <li key={q.id}>
                <label className="flex cursor-pointer items-center gap-2 rounded-sm px-1 py-1 text-sm hover:bg-hover">
                  <input
                    type="checkbox"
                    checked={plan.includes(q.id)}
                    onChange={() => toggle(map, q.id)}
                    className="size-3.5 accent-[var(--accent)]"
                  />
                  {colors.size > 1 && (
                    <span
                      aria-hidden
                      className="size-2 shrink-0 rounded-full"
                      style={{ background: colors.get(q.id) }}
                    />
                  )}
                  <span className="min-w-0 flex-1 truncate">{q.name}</span>
                </label>
              </li>
            ))}
          </ul>
          {kit.length > 0 && (
            <div className="grid gap-1 rounded-md border border-line bg-app p-2">
              <p className="flex items-center gap-1.5 text-2xs font-medium tracking-wider text-fg-muted uppercase">
                <Backpack size={11} aria-hidden />
                {t.quests.plan.kit}
              </p>
              {kit.map((k) => (
                <button
                  key={k.item.id}
                  type="button"
                  onClick={() =>
                    k.key
                      ? useApp.getState().showKeyOnMap(k.item.id)
                      : useApp.getState().showInMarket(k.item.id)
                  }
                  className="flex items-center gap-2 rounded-sm px-1 py-0.5 text-left text-xs hover:bg-hover"
                >
                  <ItemIcon id={k.item.id} size={22} />
                  <span className="min-w-0 flex-1 truncate">{k.item.name}</span>
                  {k.key ? (
                    <KeyRound size={11} className="shrink-0 text-fg-muted" aria-label={t.quests.plan.key} />
                  ) : (
                    <span className="num shrink-0 text-fg-muted">×{k.count}</span>
                  )}
                </button>
              ))}
            </div>
          )}
        </div>
      )}
    </section>
  );
}
