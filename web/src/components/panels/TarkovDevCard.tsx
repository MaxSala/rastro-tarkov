import { CircleAlert, CircleCheck, Download } from "lucide-react";
import { useState } from "react";
import { t } from "../../locales";
import { icon, iconSm } from "../../lib/icons";
import { send } from "../../lib/socket";
import { useApp } from "../../store/app";
import { Button } from "../ui/button";
import { syncText } from "../../lib/syncText";

/** Descarga de mapas desde tarkov.dev: licencia, consentimiento y progreso. */
export function TarkovDevCard({ compact = false }: { compact?: boolean }) {
  const status = useApp((s) => s.view?.tarkovdev);
  const hasPacks = useApp((s) => (s.view?.packs.length ?? 0) > 0);
  const [accepted, setAccepted] = useState(false);
  if (!status) return null;
  const running = status.state === "running";
  const pct = status.total > 0 ? Math.round((status.done / status.total) * 100) : 0;
  return (
    <div className="grid gap-3">
      {!compact && <p className="text-sm text-fg-soft">{t.tarkovdev.body}</p>}
      {compact && hasPacks ? (
        <details className="group text-xs text-fg-muted">
          <summary className="cursor-pointer select-none hover:text-fg">{t.tarkovdev.details}</summary>
          <p className="mt-1.5">{t.tarkovdev.license}</p>
          <p className="mt-1.5">{t.tarkovdev.hosts}</p>
        </details>
      ) : (
        <>
          <p className="text-xs text-fg-muted">{t.tarkovdev.license}</p>
          <p className="text-xs text-fg-muted">{t.tarkovdev.hosts}</p>
        </>
      )}
      {!running && (
        <label className="flex items-start gap-2 text-sm">
          <input
            type="checkbox"
            checked={accepted}
            onChange={(e) => setAccepted(e.target.checked)}
            className="mt-0.5 size-4 accent-[var(--accent)]"
          />
          {t.tarkovdev.accept}
        </label>
      )}
      <Button
        variant="primary"
        disabled={!accepted || running}
        onClick={() => send({ type: "tarkovDevSync" })}
      >
        <Download {...icon} />
        {running ? t.tarkovdev.running : hasPacks ? t.tarkovdev.update : t.tarkovdev.action}
      </Button>
      {status.state !== "idle" && (
        <div className="grid gap-1.5" role="status" aria-live="polite">
          {running && (
            <div
              className="h-1 overflow-hidden rounded-full bg-active"
              role="progressbar"
              aria-valuenow={pct}
              aria-valuemin={0}
              aria-valuemax={100}
            >
              <div className="h-full bg-accent transition-[width]" style={{ width: `${pct}%` }} />
            </div>
          )}
          <p className="flex items-start gap-1.5 text-xs text-fg-soft">
            {status.state === "done" && <CircleCheck {...iconSm} className="mt-0.5 shrink-0 text-ok" />}
            {status.state === "error" && <CircleAlert {...iconSm} className="mt-0.5 shrink-0 text-danger" />}
            <span>
              {syncText(status)}
              {running && status.total > 0 && (
                <span className="num text-fg-muted">
                  {" "}
                  {status.done}/{status.total}
                </span>
              )}
            </span>
          </p>
        </div>
      )}
    </div>
  );
}
