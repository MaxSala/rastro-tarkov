import { Moon, Sun, TrainFront } from "lucide-react";
import { locale, t } from "../locales";
import { pct } from "../lib/bossProtocol";
import { cn } from "../lib/cn";
import { iconSm } from "../lib/icons";
import {
  clock,
  FIXED_TIME,
  hm,
  isDay,
  localMs,
  tarkovHours,
  train,
  TRAIN_MAPS,
  type Side,
} from "../lib/tarkovTime";
import { useApp } from "../store/app";

const title = "mb-1.5 text-2xs font-medium tracking-wider text-fg-muted uppercase";

/** Raid card: time left, Tarkov time on each side, the train and the bosses of the map. */
export function RaidCard({ now, side }: { now: number; side: Side | null }) {
  const game = useApp((s) => s.view?.game);
  const mapId = useApp((s) => s.view?.activeMap?.id);
  const mapName = useApp((s) => s.view?.activeMap?.name);
  const bosses = useApp((s) => s.view?.bosses.list);
  const hasData = useApp((s) => s.view?.quests.state === "ready");
  const inRaid = Boolean(game?.enabled && game.phase === "raid" && game.startedAt);
  const elapsed = inRaid && game?.startedAt ? (now - localMs(game.startedAt)) / 1000 : 0;
  const total = game?.raidMinutes ? game.raidMinutes * 60 : null;
  const remaining = inRaid && total ? total - elapsed : null;
  const fixed = Boolean(mapId && FIXED_TIME.has(mapId));
  const tr = remaining !== null && mapId && TRAIN_MAPS.has(mapId) ? train(remaining) : null;

  return (
    <div className="grid gap-4">
      <section>
        <h3 className={title}>
          {t.raid.title} · {inRaid ? (game?.mapName ?? mapName) : t.raid.menu}
        </h3>
        {remaining !== null && total ? (
          <div className="grid gap-1.5">
            <div className="flex items-baseline justify-between gap-2">
              <span className="text-fg-muted">{t.raid.left}</span>
              <span className={cn("num text-lg", remaining < 300 ? "text-accent" : "text-fg")}>
                {clock(remaining)}
              </span>
            </div>
            <div className="h-1 overflow-hidden rounded-full bg-active" aria-hidden>
              <div
                className="h-full bg-accent"
                style={{ width: `${Math.max(0, Math.min(100, (remaining / total) * 100))}%` }}
              />
            </div>
            <span className="text-fg-muted">
              {t.raid.ends(
                new Date(now + remaining * 1000).toLocaleTimeString(locale, {
                  hour: "2-digit",
                  minute: "2-digit",
                }),
              )}
            </span>
          </div>
        ) : (
          !game?.enabled && <p className="text-fg-muted">{t.raid.noLogs}</p>
        )}
      </section>

      <section>
        <h3 className={title}>{t.raid.tarkov}</h3>
        {inRaid && fixed ? (
          <p className="text-fg-muted">{t.raid.fixed}</p>
        ) : (
          <ul className="grid gap-1">
            {(["left", "right"] as const).map((s) => {
              const h = tarkovHours(now, s);
              const mine = inRaid && side === s;
              return (
                <li
                  key={s}
                  className={cn(
                    "flex items-center gap-2 rounded-sm px-2 py-1",
                    mine ? "bg-accent-soft" : "bg-app",
                  )}
                >
                  {isDay(h) ? <Sun {...iconSm} aria-hidden /> : <Moon {...iconSm} aria-hidden />}
                  <span className="flex-1">
                    {t.raid.sides[s]}
                    {mine && <span className="text-fg-muted"> · {t.raid.yours}</span>}
                  </span>
                  <span className="num text-sm text-fg">{hm(h)}</span>
                  <span className="w-10 text-right text-fg-muted">
                    {isDay(h) ? t.raid.day : t.raid.night}
                  </span>
                </li>
              );
            })}
            {inRaid && side && <li className="text-2xs text-fg-muted">{t.raid.detected}</li>}
          </ul>
        )}
      </section>

      {tr && (
        <section>
          <h3 className={cn(title, "flex items-center gap-1.5")}>
            <TrainFront {...iconSm} aria-hidden />
            {t.raid.train}
          </h3>
          <p className="text-sm text-fg">
            {tr.state === "soon"
              ? t.raid.trainSoon(clock(tr.from), clock(tr.to))
              : tr.state === "maybe"
                ? t.raid.trainMaybe(clock(tr.leaveBy))
                : t.raid.trainGone}
          </p>
          <p className="mt-1 text-2xs text-fg-muted">{t.raid.trainNote}</p>
        </section>
      )}

      <section>
        <h3 className={title}>{t.bosses.here}</h3>
        {!hasData ? (
          <p className="text-fg-muted">{t.bosses.noData}</p>
        ) : !bosses || bosses.length === 0 ? (
          <p className="text-fg-muted">{t.bosses.none}</p>
        ) : (
          <ul className="grid gap-1">
            {bosses.slice(0, 12).map((b) => (
              <li key={b.name} className="flex items-center gap-2">
                <span className="size-1.5 shrink-0 rounded-full bg-danger" />
                <span className="min-w-0 flex-1 truncate">{b.name}</span>
                {b.time && <span className="text-fg-muted">{t.bosses.arrives(Math.round(b.time / 60))}</span>}
                <span className="num w-12 text-right text-fg">{pct(b.chance)}</span>
              </li>
            ))}
          </ul>
        )}
      </section>
    </div>
  );
}
