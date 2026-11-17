import { t } from "../locales";
import { CircleAlert } from "lucide-react";
import { useEffect, useState } from "react";
import { cn } from "../lib/cn";
import { iconSm } from "../lib/icons";
import { useApp } from "../store/app";

function clock(totalSeconds: number): string {
  const s = Math.max(0, Math.floor(totalSeconds));
  const h = Math.floor(s / 3600);
  const mm = String(Math.floor((s % 3600) / 60)).padStart(h > 0 ? 2 : 1, "0");
  const ss = String(s % 60).padStart(2, "0");
  return h > 0 ? `${h}:${mm}:${ss}` : `${mm}:${ss}`;
}

/** Mapa y tiempo de la raid actual (detectados en los logs). Solo aparece si hay algo que mostrar. */
export function RaidChip() {
  const game = useApp((s) => s.view?.game);
  const [now, setNow] = useState(() => Date.now());
  const inRaid = game?.phase === "raid" && Boolean(game.startedAt);
  useEffect(() => {
    if (!inRaid) return;
    const timer = window.setInterval(() => setNow(Date.now()), 1000);
    return () => window.clearInterval(timer);
  }, [inRaid]);
  if (!game?.enabled || !game.mapName || (game.phase !== "raid" && game.phase !== "loading")) return null;

  let time: string | null = null;
  let warn = false;
  if (inRaid && game.startedAt) {
    // Hora local del log sin zona: el navegador la interpreta como hora local, igual que el juego.
    const elapsed = (now - new Date(game.startedAt).getTime()) / 1000;
    if (game.raidMinutes) {
      const left = game.raidMinutes * 60 - elapsed;
      warn = left < 300;
      time = t.game.chip.left(clock(left));
    } else {
      time = clock(elapsed);
    }
  }
  return (
    <span
      role="status"
      title={game.mapInstalled ? undefined : t.game.chip.notInstalled}
      className="flex h-7 min-w-0 items-center gap-2 rounded-sm border border-line-strong px-2 text-xs text-fg-soft"
    >
      {game.mapInstalled ? (
        <span
          className={cn(
            "size-1.5 shrink-0 rounded-full",
            game.phase === "loading" ? "animate-pulse bg-fg-muted" : "bg-accent",
          )}
        />
      ) : (
        <CircleAlert {...iconSm} className="shrink-0 text-fg-muted" />
      )}
      <span className="truncate">
        {game.phase === "loading" ? t.game.chip.loading(game.mapName) : game.mapName}
      </span>
      {game.mode === "pve" && (
        <span className="shrink-0 rounded-sm bg-active px-1 text-2xs text-fg-muted">{t.game.modes.pve}</span>
      )}
      {time && <span className={cn("num shrink-0", warn ? "text-accent" : "text-fg")}>{time}</span>}
    </span>
  );
}
