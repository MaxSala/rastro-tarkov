import { CircleAlert, Moon, Sun } from "lucide-react";
import { useEffect, useState } from "react";
import { t } from "../locales";
import { cn } from "../lib/cn";
import { iconSm } from "../lib/icons";
import { clock, FIXED_TIME, hm, isDay, localMs, sideFrom, tarkovHours, type Side } from "../lib/tarkovTime";
import { useApp } from "../store/app";
import { Popover } from "../components/ui/popover";
import { RaidCard } from "./RaidCard";

/** Every second while the chip is shown: the raid clock and Tarkov time both move. */
function useNow(): number {
  const [now, setNow] = useState(() => Date.now());
  useEffect(() => {
    const timer = window.setInterval(() => setNow(Date.now()), 1000);
    return () => window.clearInterval(timer);
  }, []);
  return now;
}

/** The side of the current raid, when the last screenshot belongs to it and tells. */
export function useRaidSide(): Side | null {
  const startedAt = useApp((s) => s.view?.game.startedAt);
  const taken = useApp((s) => s.view?.watcher.lastTakenAt);
  const hours = useApp((s) => s.view?.watcher.raidHours);
  if (!startedAt || !taken || hours === null || hours === undefined) return null;
  const at = localMs(taken);
  return at >= localMs(startedAt) ? sideFrom(hours, at) : null;
}

const DayIcon = ({ h }: { h: number }) =>
  isDay(h) ? (
    <Sun {...iconSm} className="shrink-0" aria-hidden />
  ) : (
    <Moon {...iconSm} className="shrink-0" aria-hidden />
  );

/**
 * Top bar chip on the map: Tarkov time on both sides while in the menu; map, time left and the
 * time in your raid while playing. Opens the raid card.
 */
export function RaidChip() {
  const game = useApp((s) => s.view?.game);
  const mapId = useApp((s) => s.view?.activeMap?.id);
  const side = useRaidSide();
  const now = useNow();
  const inRaid = Boolean(game?.enabled && game.phase === "raid" && game.startedAt);
  const loading = Boolean(game?.enabled && game.phase === "loading" && game.mapName);
  const left = tarkovHours(now, "left");
  const right = tarkovHours(now, "right");

  let content;
  if (inRaid && game?.startedAt) {
    const elapsed = (now - localMs(game.startedAt)) / 1000;
    const remaining = game.raidMinutes ? game.raidMinutes * 60 - elapsed : null;
    const yours = side && mapId && !FIXED_TIME.has(mapId) ? tarkovHours(now, side) : null;
    content = (
      <>
        <span
          className={cn("size-1.5 shrink-0 rounded-full", game.mapInstalled ? "bg-accent" : "bg-fg-muted")}
        />
        <span className="truncate">{game.mapName}</span>
        {game.mode === "pve" && (
          <span className="shrink-0 rounded-sm bg-active px-1 text-2xs text-fg-muted">
            {t.game.modes.pve}
          </span>
        )}
        <span
          className={cn("num shrink-0", remaining !== null && remaining < 300 ? "text-accent" : "text-fg")}
        >
          {remaining !== null ? t.game.chip.left(clock(remaining)) : clock(elapsed)}
        </span>
        {yours !== null && (
          <span className="num flex shrink-0 items-center gap-1 text-fg-muted">
            <DayIcon h={yours} />
            {hm(yours)}
          </span>
        )}
      </>
    );
  } else if (loading && game?.mapName) {
    content = (
      <>
        {game.mapInstalled ? (
          <span className="size-1.5 shrink-0 animate-pulse rounded-full bg-fg-muted" />
        ) : (
          <CircleAlert {...iconSm} className="shrink-0 text-fg-muted" />
        )}
        <span className="truncate">{t.game.chip.loading(game.mapName)}</span>
      </>
    );
  } else {
    content = (
      <span className="num flex items-center gap-2">
        <span className="flex items-center gap-1">
          <DayIcon h={left} />
          {hm(left)}
        </span>
        <span className="text-fg-muted">·</span>
        <span className="flex items-center gap-1">
          <DayIcon h={right} />
          {hm(right)}
        </span>
      </span>
    );
  }
  return (
    <Popover
      label={t.raid.open}
      button={content}
      buttonClass="flex h-7 min-w-0 max-w-[26rem] items-center gap-2 rounded-sm border border-line-strong px-2 text-xs text-fg-soft hover:bg-hover hover:text-fg aria-expanded:bg-active"
    >
      <RaidCard now={now} side={side} />
    </Popover>
  );
}
