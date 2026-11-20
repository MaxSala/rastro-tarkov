import { Check } from "lucide-react";
import { t } from "../../../locales";
import type { Quest, QuestIndex } from "../../../lib/quests";
import { send } from "../../../lib/socket";
import { useApp } from "../../../store/app";
import { TraderAvatar } from "../../quests/media";

/** After the raid: quests with every objective ticked, waiting to be handed in. */
export function HandIn({ quests, idx }: { quests: Quest[]; idx: QuestIndex }) {
  if (quests.length === 0) return null;
  return (
    <section className="border-t border-line">
      <h3 className="px-3 pt-3 pb-1 text-2xs font-medium tracking-wider text-fg-muted uppercase">
        {t.quests.handIn.title}
      </h3>
      <ul className="pb-2">
        {quests.map((q) => {
          const trader = idx.traders.get(q.trader) ?? q.trader;
          return (
            <li key={q.id} className="flex items-center gap-2.5 px-3 py-1.5">
              <TraderAvatar image={idx.portraits.get(q.trader) ?? null} name={trader} size={26} />
              <button
                type="button"
                onClick={() => useApp.getState().openQuest(q.id)}
                className="min-w-0 flex-1 text-left"
              >
                <span className="block truncate text-sm">{q.name}</span>
                <span className="block truncate text-xs text-fg-muted">{t.quests.handIn.to(trader)}</span>
              </button>
              <button
                type="button"
                title={t.quests.handIn.done}
                aria-label={t.quests.handIn.done}
                onClick={() => send({ type: "questSet", questId: q.id, status: "completed" })}
                className="grid size-7 shrink-0 place-items-center rounded-sm border border-line-strong text-fg-soft hover:bg-hover hover:text-fg"
              >
                <Check size={13} />
              </button>
            </li>
          );
        })}
      </ul>
    </section>
  );
}
