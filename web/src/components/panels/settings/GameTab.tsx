import { t } from "../../../locales";
import { CircleAlert, CircleCheck } from "lucide-react";
import { useState, type ReactNode } from "react";
import { iconSm } from "../../../lib/icons";
import { send } from "../../../lib/socket";
import { useApp } from "../../../store/app";
import { Button } from "../../ui/button";
import { Field } from "../../ui/field";
import { Input } from "../../ui/input";
import { Segmented } from "../../ui/segmented";
import { Switch } from "../../ui/switch";

function Row({ label, hint, children }: { label: string; hint?: string; children: ReactNode }) {
  return (
    <div className="flex items-center justify-between gap-6 border-b border-line py-3 last:border-0">
      <div className="min-w-0">
        <p className="text-sm">{label}</p>
        {hint && <p className="text-xs text-fg-muted">{hint}</p>}
      </div>
      <div className="shrink-0">{children}</div>
    </div>
  );
}

/** Ajustes de la detección por logs: activar, carpeta, estado actual y modo de datos. */
export function GameTab() {
  const game = useApp((s) => s.view?.game);
  const [path, setPath] = useState("");
  if (!game) return null;
  const phase = !game.enabled
    ? t.game.phase.off
    : game.phase === "raid"
      ? t.game.phase.raid(game.mapName ?? "?")
      : game.phase === "loading"
        ? t.game.phase.loading(game.mapName ?? "?")
        : t.game.phase.menu;
  return (
    <div className="grid gap-4">
      <Row label={t.game.enable} hint={t.game.enableHint}>
        <Switch
          label={t.game.enable}
          checked={game.enabled}
          onChange={(v) => send({ type: "setGameLogs", enabled: v, path })}
        />
      </Row>
      <Field id="game-logs" label={t.game.folder} hint={t.game.folderHint}>
        <div className="flex gap-2">
          <Input
            id="game-logs"
            className="num text-xs"
            placeholder={game.folder ?? ""}
            value={path}
            onChange={(e) => setPath(e.target.value)}
            spellCheck={false}
            aria-describedby="game-logs-hint"
          />
          <Button onClick={() => send({ type: "setGameLogs", enabled: game.enabled, path })}>
            {t.game.apply}
          </Button>
        </div>
      </Field>
      <div
        className="grid gap-1.5 rounded-sm border border-line p-3 text-sm"
        role="status"
        aria-live="polite"
      >
        <p className="flex items-center gap-1.5">
          {game.folderFound ? (
            <CircleCheck {...iconSm} className="text-ok" />
          ) : (
            <CircleAlert {...iconSm} className="text-danger" />
          )}
          {game.folderFound ? t.game.found : t.game.missing}
        </p>
        {game.folder && (
          <p className="num truncate text-xs text-fg-muted" title={game.folder}>
            {game.folder}
          </p>
        )}
        <p className="flex justify-between gap-4 text-xs">
          <span className="text-fg-muted">{t.game.status}</span>
          <span>{phase}</span>
        </p>
        <p className="flex justify-between gap-4 text-xs">
          <span className="text-fg-muted">{t.game.mode}</span>
          <span>{game.mode ? t.game.modes[game.mode] : t.game.unknown}</span>
        </p>
      </div>
      <Row label={t.game.dataMode} hint={t.game.dataModeHint}>
        <Segmented
          label={t.game.dataMode}
          value={game.modePref}
          onChange={(mode) => send({ type: "setGameMode", mode })}
          options={(["auto", "regular", "pve"] as const).map((v) => ({
            value: v,
            label: t.game.dataModes[v],
          }))}
        />
      </Row>
    </div>
  );
}
