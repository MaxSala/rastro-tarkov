// What handing a quest in gives, as tarkov.dev lists it: experience, reputation and skills as
// chips; items, trader offers and crafts as rows that open in the Market.
import { Hammer, ShoppingCart, Sparkles, UserPlus } from "lucide-react";
import type { ReactNode } from "react";
import { t } from "../../locales";
import { cn } from "../../lib/cn";
import { int } from "../../lib/format";
import type { QuestIndex, Rewards } from "../../lib/quests";
import { useApp } from "../../store/app";
import { ItemIcon } from "../market/ItemIcon";
import { TraderAvatar } from "./media";

const signed = (v: number, digits: number) => `${v < 0 ? "−" : "+"}${Math.abs(v).toFixed(digits)}`;
const chip = "inline-flex h-7 items-center gap-1.5 rounded-sm border border-line bg-app px-2 text-xs";

function ItemLine({ id, name, side, icon }: { id: string; name: string; side: ReactNode; icon?: ReactNode }) {
  return (
    <button
      type="button"
      title={t.keys.inMarket}
      onClick={() => useApp.getState().showInMarket(id)}
      className="flex w-full items-center gap-2.5 rounded-sm px-2 py-1.5 text-left hover:bg-hover"
    >
      <ItemIcon id={id} size={28} />
      <span className="min-w-0 flex-1 truncate text-sm">{name || t.quests.rewards.unnamed}</span>
      <span className="flex shrink-0 items-center gap-1 text-2xs text-fg-muted">
        {icon}
        {side}
      </span>
    </button>
  );
}

export function QuestRewards({ r, idx }: { r: Rewards; idx: QuestIndex }) {
  const portrait = (name: string) => {
    for (const [slug, n] of idx.traders) if (n === name) return idx.portraits.get(slug) ?? null;
    return null;
  };
  const chips = r.experience > 0 || r.standing.length > 0 || r.skills.length > 0;
  const rows = r.items.length + r.offers.length + r.crafts.length + r.traders.length;
  if (!chips && rows === 0) return <p className="text-xs text-fg-muted">{t.quests.rewards.none}</p>;
  return (
    <div className="grid gap-2">
      {chips && (
        <div className="flex flex-wrap gap-1.5">
          {r.experience > 0 && (
            <span className={cn(chip, "border-accent/30 bg-accent-soft font-medium text-accent")}>
              <Sparkles size={12} aria-hidden />
              <span className="num">{t.quests.rewards.xp(int(r.experience))}</span>
            </span>
          )}
          {r.standing.map(([name, v]) => (
            <span key={name} className={chip} title={t.quests.rewards.standing}>
              <TraderAvatar image={portrait(name)} name={name} size={18} className="-ml-1" />
              {name}
              <span className={cn("num", v < 0 ? "text-danger" : "text-ok")}>{signed(v, 2)}</span>
            </span>
          ))}
          {r.skills.map(([name, lv]) => (
            <span key={name} className={chip} title={t.quests.rewards.skill}>
              {name}
              <span className="num text-ok">{signed(lv, lv % 1 === 0 ? 0 : 1)}</span>
            </span>
          ))}
        </div>
      )}
      {rows > 0 && (
        <div className="-mx-2 grid">
          {r.items.map((i) => (
            <ItemLine
              key={i.id}
              id={i.id}
              name={i.name}
              side={<span className="num">×{int(i.count)}</span>}
            />
          ))}
          {r.offers.length + r.crafts.length + r.traders.length > 0 && (
            <p className="px-2 pt-2 pb-0.5 text-2xs text-fg-muted">{t.quests.rewards.unlocks}</p>
          )}
          {r.offers.map((o) => (
            <ItemLine
              key={`o-${o.id}`}
              id={o.id}
              name={o.name}
              icon={<ShoppingCart size={11} aria-hidden />}
              side={t.quests.rewards.offer(o.trader, o.level)}
            />
          ))}
          {r.crafts.map((c) => (
            <ItemLine
              key={`c-${c.id}`}
              id={c.id}
              name={c.name}
              icon={<Hammer size={11} aria-hidden />}
              side={t.quests.rewards.craft}
            />
          ))}
          {r.traders.map((name) => (
            <span key={name} className="flex items-center gap-2.5 px-2 py-1.5 text-sm">
              <TraderAvatar image={portrait(name)} name={name} size={28} />
              <span className="min-w-0 flex-1 truncate">{name}</span>
              <span className="flex items-center gap-1 text-2xs text-fg-muted">
                <UserPlus size={11} aria-hidden />
                {t.quests.rewards.trader}
              </span>
            </span>
          ))}
        </div>
      )}
    </div>
  );
}
