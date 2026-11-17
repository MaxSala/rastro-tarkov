import { ExternalLink, RefreshCw, Sparkles } from "lucide-react";
import { t } from "../../../locales";
import { iconSm } from "../../../lib/icons";
import { say } from "../../../lib/msg";
import { send } from "../../../lib/socket";
import { useApp } from "../../../store/app";
import { LogoMark } from "../../brand/Logo";
import { Button } from "../../ui/button";
import { Switch } from "../../ui/switch";

function UpdateStatusLine() {
  const u = useApp((s) => s.view?.update);
  if (!u || u.state === "idle") return null;
  if (u.state === "available" && u.latest && u.url)
    return (
      <p className="flex flex-wrap items-center gap-2 text-xs text-fg-soft">
        {t.update.available(u.latest)}
        <a
          href={u.url}
          target="_blank"
          rel="noreferrer"
          className="inline-flex items-center gap-1 text-accent hover:underline"
        >
          <ExternalLink {...iconSm} />
          {t.update.download}
        </a>
      </p>
    );
  const text =
    u.state === "checking" ? t.update.checking : u.state === "upToDate" ? t.update.upToDate : say(u.error);
  return <p className={u.state === "error" ? "text-xs text-danger" : "text-xs text-fg-muted"}>{text}</p>;
}

export function AboutTab() {
  const version = useApp((s) => s.view?.version ?? "");
  const checkUpdates = useApp((s) => s.view?.settings.checkUpdates === true);
  const checking = useApp((s) => s.view?.update.state === "checking");
  const openDialog = useApp((s) => s.openDialog);
  return (
    <div className="grid max-w-prose gap-4">
      <div className="flex items-center gap-3">
        <LogoMark size={36} />
        <div>
          <p className="text-sm font-medium">{t.app.name}</p>
          <p className="num text-xs text-fg-muted">
            {t.settings.version} {version}
          </p>
        </div>
      </div>
      <p className="text-sm text-fg-soft">{t.settings.aboutBody}</p>
      <div className="grid gap-2 rounded-md border border-line p-3">
        <div className="flex items-center justify-between gap-4">
          <div className="min-w-0">
            <p className="text-sm">{t.update.setting}</p>
            <p className="text-xs text-fg-muted">{t.update.settingHint}</p>
          </div>
          <Switch
            checked={checkUpdates}
            label={t.update.setting}
            onChange={(v) => send({ type: "setCheckUpdates", enabled: v })}
          />
        </div>
        <UpdateStatusLine />
        <div className="flex flex-wrap gap-2">
          <Button size="sm" disabled={checking} onClick={() => send({ type: "checkUpdates" })}>
            <RefreshCw
              {...iconSm}
              className={checking ? "animate-spin motion-reduce:animate-none" : undefined}
            />
            {t.update.checkNow}
          </Button>
          <Button size="sm" onClick={() => openDialog("changelog")}>
            <Sparkles {...iconSm} />
            {t.changelog.open}
          </Button>
        </div>
      </div>
      <p className="text-xs text-fg-muted">{t.settings.licenses}</p>
    </div>
  );
}
