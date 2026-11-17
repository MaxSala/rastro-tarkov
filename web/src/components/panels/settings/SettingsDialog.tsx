import { X } from "lucide-react";
import { useEffect, useRef, useState } from "react";
import { t } from "../../../locales";
import { icon } from "../../../lib/icons";
import type { SettingsInput } from "../../../lib/protocol";
import { send } from "../../../lib/socket";
import { useApp } from "../../../store/app";
import { Button } from "../../ui/button";
import { Tabs } from "../../ui/tabs";
import { AboutTab } from "./AboutTab";
import { GeneralTab, LogsTab, MapTab, PartyTab } from "./tabs";
import { GameTab } from "./GameTab";

type Tab = "general" | "map" | "game" | "party" | "logs" | "about";
type Current = NonNullable<ReturnType<typeof useApp.getState>["view"]>["settings"];

export function SettingsDialog() {
  const open = useApp((s) => s.dialog === "settings");
  const current = useApp((s) => s.view?.settings);
  return open && current ? <SettingsShell current={current} /> : null;
}

/** Ajustes en pestañas. Los campos del servidor se guardan con «Guardar»; las preferencias de vista al momento. */
function SettingsShell({ current }: { current: Current }) {
  const ref = useRef<HTMLDialogElement>(null);
  const openDialog = useApp((s) => s.openDialog);
  const [tab, setTab] = useState<Tab>("general");
  const [form, setForm] = useState<SettingsInput>(() => ({
    screenshotsDir: current.screenshotsDirIsDefault ? "" : current.screenshotsDir,
    trailLength: current.trailLength,
    playerName: current.playerName,
    playerColor: current.playerColor,
    partyPort: current.partyPort,
  }));
  useEffect(() => {
    ref.current?.showModal();
  }, []);
  const close = () => openDialog(null);
  const set = <K extends keyof SettingsInput>(k: K, v: SettingsInput[K]) =>
    setForm((f) => ({ ...f, [k]: v }));
  const tabs: { value: Tab; label: string }[] = (
    ["general", "map", "game", "party", "logs", "about"] as const
  ).map((v) => ({ value: v, label: v === "game" ? t.game.tab : t.settings.tabs[v] }));
  const editable = tab === "general" || tab === "map" || tab === "party";

  return (
    <dialog
      ref={ref}
      aria-labelledby="settings-title"
      onClose={close}
      onCancel={(e) => {
        e.preventDefault();
        close();
      }}
      onMouseDown={(e) => e.target === ref.current && close()}
      className="m-auto flex h-[min(620px,calc(100vh-48px))] w-[min(680px,calc(100vw-32px))] flex-col rounded-md border border-line-strong bg-surface p-0 text-fg"
    >
      <header className="flex items-center justify-between px-4 pt-3 pb-2">
        <h2 id="settings-title" className="text-base font-medium">
          {t.settings.title}
        </h2>
        <button
          type="button"
          onClick={close}
          aria-label={t.a11y.close}
          className="inline-flex size-7 items-center justify-center rounded-sm text-fg-muted hover:bg-hover hover:text-fg"
        >
          <X {...icon} />
        </button>
      </header>
      <Tabs label={t.settings.title} value={tab} onChange={setTab} tabs={tabs} />
      <form
        id="settings-form"
        role="tabpanel"
        aria-labelledby={`tab-${tab}`}
        className="min-h-0 flex-1 overflow-x-hidden overflow-y-auto px-4 py-4"
        onSubmit={(e) => {
          e.preventDefault();
          send({ type: "updateSettings", settings: form });
        }}
      >
        {tab === "general" && <GeneralTab form={form} set={set} current={current} />}
        {tab === "map" && <MapTab form={form} set={set} />}
        {tab === "game" && <GameTab />}
        {tab === "party" && <PartyTab form={form} set={set} />}
        {tab === "logs" && <LogsTab />}
        {tab === "about" && <AboutTab />}
      </form>
      <footer className="flex justify-end gap-2 border-t border-line px-4 py-3">
        <Button onClick={close}>{t.settings.cancel}</Button>
        {editable && (
          <Button variant="primary" type="submit" form="settings-form">
            {t.settings.save}
          </Button>
        )}
      </footer>
    </dialog>
  );
}
