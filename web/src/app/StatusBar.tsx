import { CircleAlert, FolderOpen } from "lucide-react";
import { t } from "../locales";
import { coord, raidClock } from "../lib/format";
import { iconSm } from "../lib/icons";
import { useApp } from "../store/app";

export function StatusBar() {
  const view = useApp((s) => s.view);
  if (!view) return <footer className="h-7 border-t border-line" />;
  const w = view.watcher;
  const p = view.player;
  return (
    <footer className="flex h-7 items-center gap-4 border-t border-line bg-surface px-3 text-xs text-fg-muted">
      <span className="flex min-w-0 items-center gap-1.5" title={w.folder ?? ""}>
        {w.error ? (
          <CircleAlert {...iconSm} className="shrink-0 text-danger" />
        ) : (
          <FolderOpen {...iconSm} className="shrink-0" />
        )}
        <span className="shrink-0">{w.error ? t.status.notWatching : t.status.watching}</span>
        <span className="num truncate text-fg-soft">{w.error ?? w.folder}</span>
      </span>
      <div className="flex-1" />
      {w.lastTakenAt ? (
        <span className="shrink-0">
          {t.status.lastCapture} <span className="num text-fg-soft">{w.lastTakenAt}</span>
          {w.raidHours !== null && (
            <>
              {" "}
              <span className="num text-fg-soft">{raidClock(w.raidHours)}</span> {t.status.raidTime}
            </>
          )}
        </span>
      ) : (
        <span className="shrink-0">{t.status.noCapture}</span>
      )}
      {p && (
        <span className="num shrink-0 text-fg-soft" aria-label={t.a11y.coords}>
          X {coord(p.gameX)} Y {coord(p.gameY)} Z {coord(p.gameZ)}
        </span>
      )}
    </footer>
  );
}
