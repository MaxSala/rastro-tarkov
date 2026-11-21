import { Check, Clock, Minus, Plus } from "lucide-react";
import { t } from "../../locales";
import { cn } from "../../lib/cn";
import { int } from "../../lib/format";
import { buildTime, built, maxLevel, nextLevel } from "../../lib/hideout";
import type { Station } from "../../lib/hideoutSchema";
import { iconSm } from "../../lib/icons";
import type { Progress } from "../../lib/questProtocol";
import { send } from "../../lib/socket";
import { useApp } from "../../store/app";
import { ItemIcon } from "../market/ItemIcon";

const step =
  "grid size-7 place-items-center rounded-sm text-fg-soft hover:bg-hover hover:text-fg disabled:opacity-30 disabled:hover:bg-transparent";
const chip = "flex items-center gap-1 rounded-sm px-1.5 py-0.5 text-2xs";

/** One station: its level (yours to set) and what the next level asks for. */
export function StationCard({ s, p, names }: { s: Station; p: Progress; names: Map<string, string> }) {
  const level = built(p, s);
  const max = maxLevel(s);
  const next = nextLevel(p, s);
  const set = (n: number) =>
    send({ type: "hideoutLevel", stationId: s.id, level: Math.max(0, Math.min(max, n)) });
  const time = next ? buildTime(next.seconds) : null;
  return (
    <article className="grid content-start gap-3 rounded-md border border-line bg-surface p-3">
      <header className="flex items-center gap-2">
        <div className="min-w-0 flex-1">
          <h2 className="truncate text-sm font-medium">{s.name}</h2>
          <p className="text-xs text-fg-muted">
            {level === 0 ? t.hideout.notBuilt : t.hideout.level(level, max)}
          </p>
        </div>
        {level >= max && max > 0 && (
          <span className="shrink-0 rounded-sm bg-accent-soft px-1.5 py-0.5 text-2xs text-accent">
            {t.hideout.maxed}
          </span>
        )}
        <div className="flex shrink-0 items-center rounded-sm border border-line-strong">
          <button
            type="button"
            className={step}
            onClick={() => set(level - 1)}
            disabled={level === 0}
            aria-label={t.hideout.lower}
          >
            <Minus {...iconSm} />
          </button>
          <span className="num w-6 text-center text-xs">{level}</span>
          <button
            type="button"
            className={step}
            onClick={() => set(level + 1)}
            disabled={level >= max}
            aria-label={t.hideout.raise}
          >
            <Plus {...iconSm} />
          </button>
        </div>
      </header>
      {next && (
        <section className="grid gap-2" aria-label={t.hideout.next(next.level)}>
          <div className="flex items-center justify-between gap-2">
            <h3 className="text-2xs font-medium tracking-wider text-fg-muted uppercase">
              {t.hideout.next(next.level)}
            </h3>
            <span className="flex items-center gap-1 text-2xs text-fg-muted">
              <Clock {...iconSm} aria-hidden />
              {time ? t.hideout.time(time) : t.hideout.instant}
            </span>
          </div>
          {(next.stations.length > 0 || next.traders.length > 0 || next.skills.length > 0) && (
            <ul className="flex flex-wrap gap-1">
              {next.stations.map(([id, lv]) => {
                const ok = (p.hideout[id] ?? 0) >= lv;
                return (
                  <li
                    key={id}
                    title={ok ? t.hideout.met : t.hideout.missing}
                    className={cn(chip, ok ? "bg-active text-fg-soft" : "bg-danger/15 text-danger")}
                  >
                    {ok && <Check {...iconSm} aria-hidden />}
                    {t.hideout.station(names.get(id) ?? "?", lv)}
                  </li>
                );
              })}
              {next.traders.map(([name, lv]) => (
                <li key={name} className={cn(chip, "bg-active text-fg-soft")}>
                  {t.hideout.loyalty(name, lv)}
                </li>
              ))}
              {next.skills.map(([name, lv]) => (
                <li key={name} className={cn(chip, "bg-active text-fg-soft")}>
                  {t.hideout.skill(name, lv)}
                </li>
              ))}
            </ul>
          )}
          {next.items.length > 0 && (
            <ul className="grid gap-px overflow-hidden rounded-sm border border-line bg-line">
              {next.items.map((n) => (
                <li key={`${n.id}:${n.fir}`} className="bg-app">
                  <button
                    type="button"
                    title={t.keys.inMarket}
                    onClick={() => useApp.getState().showInMarket(n.id)}
                    className="flex w-full items-center gap-2 px-2 py-1 text-left hover:bg-hover"
                  >
                    <ItemIcon id={n.id} size={28} />
                    <span className="min-w-0 flex-1 truncate text-xs">{n.name}</span>
                    {n.fir && (
                      <span title={t.quests.fir} className="shrink-0 text-2xs text-accent">
                        {t.quests.firShort}
                      </span>
                    )}
                    <span className="num shrink-0 text-xs text-fg-soft">×{int(n.count)}</span>
                  </button>
                </li>
              ))}
            </ul>
          )}
        </section>
      )}
    </article>
  );
}
