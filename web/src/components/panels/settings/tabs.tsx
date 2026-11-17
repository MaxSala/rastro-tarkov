import { Copy, FolderOpen, LogOut, RefreshCw } from "lucide-react";
import { useEffect, useRef, useState, type ReactNode } from "react";
import { LANGS, NATIVE_NAMES, lang, setLang, t, type Lang } from "../../../locales";
import { cn } from "../../../lib/cn";
import { iconSm } from "../../../lib/icons";
import { PLAYER_COLORS, playerVar } from "../../../lib/players";
import type { SettingsInput } from "../../../lib/protocol";
import { send } from "../../../lib/socket";
import { getThemePref, setThemePref, type ThemePref } from "../../../lib/theme";
import { useApp } from "../../../store/app";
import { Button } from "../../ui/button";
import { Field } from "../../ui/field";
import { Input } from "../../ui/input";
import { Segmented } from "../../ui/segmented";
import { Switch } from "../../ui/switch";

type Setter = <K extends keyof SettingsInput>(k: K, v: SettingsInput[K]) => void;
type Current = NonNullable<ReturnType<typeof useApp.getState>["view"]>["settings"];

/** Fila de ajuste: etiqueta y ayuda a la izquierda, control a la derecha. */
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

export function GeneralTab({ form, set, current }: { form: SettingsInput; set: Setter; current: Current }) {
  const [theme, setTheme] = useState<ThemePref>(getThemePref);
  return (
    <div className="grid gap-4">
      <Field id="set-folder" label={t.settings.folder} hint={t.settings.folderHint}>
        <Input
          id="set-folder"
          className="num text-xs"
          placeholder={current.screenshotsDirIsDefault ? current.screenshotsDir : ""}
          value={form.screenshotsDir}
          onChange={(e) => set("screenshotsDir", e.target.value)}
          spellCheck={false}
        />
      </Field>
      <div>
        <Row label={t.settings.window} hint={t.settings.windowHint}>
          <Segmented
            label={t.settings.window}
            value={current.openInWindow ? "app" : "browser"}
            onChange={(v) => send({ type: "setOpenInWindow", value: v === "app" })}
            options={[
              { value: "app", label: t.settings.windowApp },
              { value: "browser", label: t.settings.windowBrowser },
            ]}
          />
        </Row>
        <Row label={t.language.label} hint={t.language.hint}>
          <select
            aria-label={t.language.label}
            value={lang}
            onChange={(e) => {
              const v = e.target.value as Lang;
              send({ type: "setLanguage", lang: v });
              setLang(v);
            }}
            className="h-8 w-40 rounded-sm border border-line-strong bg-app px-2 text-sm text-fg"
          >
            {LANGS.map((v) => (
              <option key={v} value={v} lang={v}>
                {NATIVE_NAMES[v]}
              </option>
            ))}
          </select>
        </Row>
        <Row label={t.settings.theme}>
          <Segmented
            label={t.settings.theme}
            value={theme}
            onChange={(v) => {
              setTheme(v);
              setThemePref(v);
            }}
            options={[
              { value: "system", label: t.settings.themeSystem },
              { value: "dark", label: t.settings.themeDark },
              { value: "light", label: t.settings.themeLight },
            ]}
          />
        </Row>
        <Row label={t.settings.quit} hint={t.settings.quitHint}>
          <Button variant="danger" size="sm" onClick={() => send({ type: "quit" })}>
            <LogOut {...iconSm} />
            {t.settings.quit}
          </Button>
        </Row>
      </div>
    </div>
  );
}

export function MapTab({ form, set }: { form: SettingsInput; set: Setter }) {
  const prefs = useApp((s) => s.prefs);
  const setPrefs = useApp((s) => s.setPrefs);
  const size = prefs.markerScale < 0.95 ? "s" : prefs.markerScale > 1.1 ? "l" : "m";
  return (
    <div>
      <Row label={t.settings.follow}>
        <Switch label={t.settings.follow} checked={prefs.follow} onChange={(v) => setPrefs({ follow: v })} />
      </Row>
      <Row label={t.settings.labels}>
        <Switch label={t.settings.labels} checked={prefs.labels} onChange={(v) => setPrefs({ labels: v })} />
      </Row>
      <Row label={t.settings.showTrail}>
        <Switch label={t.settings.showTrail} checked={prefs.trail} onChange={(v) => setPrefs({ trail: v })} />
      </Row>
      <Row label={t.settings.trail}>
        <Input
          className="num w-24"
          type="number"
          min={1}
          max={2000}
          aria-label={t.settings.trail}
          value={form.trailLength}
          onChange={(e) => set("trailLength", Number(e.target.value))}
        />
      </Row>
      <Row label={t.settings.markerSize}>
        <Segmented
          label={t.settings.markerSize}
          value={size}
          onChange={(v) => setPrefs({ markerScale: v === "s" ? 0.85 : v === "l" ? 1.25 : 1 })}
          options={(["s", "m", "l"] as const).map((v) => ({ value: v, label: t.settings.sizes[v] }))}
        />
      </Row>
    </div>
  );
}

export function PartyTab({ form, set }: { form: SettingsInput; set: Setter }) {
  return (
    <div className="grid gap-4">
      <Field id="set-name" label={t.settings.name}>
        <Input
          id="set-name"
          maxLength={24}
          value={form.playerName}
          onChange={(e) => set("playerName", e.target.value)}
        />
      </Field>
      <fieldset className="grid gap-1.5">
        <legend className="mb-1.5 text-xs font-medium text-fg-soft">{t.settings.color}</legend>
        <div className="flex gap-2" role="radiogroup" aria-label={t.settings.color}>
          {PLAYER_COLORS.map((c) => (
            <button
              key={c}
              type="button"
              role="radio"
              aria-checked={form.playerColor === c}
              aria-label={`${t.settings.color} ${c + 1}`}
              onClick={() => set("playerColor", c)}
              className={cn(
                "size-6 rounded-sm border-2",
                form.playerColor === c ? "border-fg" : "border-transparent",
              )}
              style={{ background: playerVar(c) }}
            />
          ))}
        </div>
      </fieldset>
      <Field id="set-port" label={t.settings.partyPort}>
        <Input
          id="set-port"
          className="num w-32"
          type="number"
          min={1024}
          max={65535}
          value={form.partyPort}
          onChange={(e) => set("partyPort", Number(e.target.value))}
        />
      </Field>
    </div>
  );
}

export function LogsTab() {
  const logs = useApp((s) => s.logs);
  const dataDir = useApp((s) => s.view?.dataDir ?? "");
  const notify = useApp((s) => s.notify);
  const box = useRef<HTMLDivElement>(null);
  useEffect(() => {
    send({ type: "getLogs" });
  }, []);
  useEffect(() => {
    box.current?.scrollTo({ top: box.current.scrollHeight });
  }, [logs]);
  const copy = () =>
    navigator.clipboard.writeText(logs.join("\n")).then(
      () => notify("info", t.settings.copied),
      () => notify("error", t.errors.generic),
    );
  return (
    <div className="grid h-full min-h-0 grid-rows-[auto_1fr_auto] gap-3">
      <div className="flex items-center justify-between gap-3">
        <p className="min-w-0 text-xs text-fg-muted">{t.settings.logsHint}</p>
        <div className="flex shrink-0 gap-1.5">
          <Button size="sm" onClick={() => send({ type: "getLogs" })}>
            <RefreshCw {...iconSm} />
            {t.settings.refresh}
          </Button>
          <Button size="sm" onClick={copy}>
            <Copy {...iconSm} />
            {t.settings.copy}
          </Button>
        </div>
      </div>
      <div
        ref={box}
        className="num min-h-48 overflow-auto rounded-sm border border-line bg-app p-2 text-2xs leading-4 text-fg-soft"
      >
        {logs.length === 0 ? (
          <p className="text-fg-muted">{t.settings.noLogs}</p>
        ) : (
          logs.map((l, i) => (
            <div key={i} className="break-all whitespace-pre-wrap">
              {l}
            </div>
          ))
        )}
      </div>
      <div className="flex items-center justify-between gap-3 border-t border-line pt-3">
        <div className="min-w-0">
          <p className="text-xs text-fg-muted">{t.settings.dataDir}</p>
          <p className="num truncate text-xs" title={dataDir}>
            {dataDir}
          </p>
        </div>
        <Button size="sm" onClick={() => send({ type: "openDataDir" })}>
          <FolderOpen {...iconSm} />
          {t.settings.openDataDir}
        </Button>
      </div>
    </div>
  );
}

export function AboutTab() {
  const version = useApp((s) => s.view?.version ?? "");
  return (
    <div className="grid max-w-prose gap-3">
      <p className="flex items-center gap-2 text-sm">
        <svg viewBox="0 0 24 24" width="18" height="18" aria-hidden>
          <path d="M12 2 20 21 12 16.5 4 21Z" className="fill-accent" />
        </svg>
        <span className="font-medium">{t.app.name}</span>
        <span className="num text-xs text-fg-muted">
          {t.settings.version} {version}
        </span>
      </p>
      <p className="text-sm text-fg-soft">{t.settings.aboutBody}</p>
      <p className="text-xs text-fg-muted">{t.settings.licenses}</p>
    </div>
  );
}
