import { Check, Info, Trash2, Upload } from "lucide-react";
import { useState } from "react";
import { t } from "../../locales";
import { cn } from "../../lib/cn";
import { icon, iconSm } from "../../lib/icons";
import { send } from "../../lib/socket";
import { useApp } from "../../store/app";
import { IconButton } from "../ui/button";
import { Tooltip } from "../ui/tooltip";
import { PanelFrame, PanelNote } from "./PanelFrame";
import { TarkovDevCard } from "./TarkovDevCard";

type Pack = NonNullable<ReturnType<typeof useApp.getState>["view"]>["packs"][number];

function PackRow({ p, active }: { p: Pack; active: boolean }) {
  const [confirm, setConfirm] = useState(false);
  return (
    <li
      className={cn(
        "group flex min-w-0 items-center gap-1 border-l-2 pr-1.5 pl-2.5",
        active ? "border-accent bg-accent-soft" : "border-transparent hover:bg-hover",
      )}
    >
      <button
        type="button"
        aria-pressed={active}
        onClick={() => !active && send({ type: "setActiveMap", mapId: p.id })}
        className="flex min-w-0 flex-1 items-center gap-2 py-2 text-left"
      >
        <span className="min-w-0 flex-1">
          <span className="block truncate text-sm">{p.name}</span>
          <span className="block truncate text-xs text-fg-muted">{t.maps.extractsCount(p.extracts)}</span>
        </span>
        {active && <Check {...iconSm} className="shrink-0 text-accent" aria-label={t.maps.active} />}
      </button>
      <Tooltip
        side="left"
        content={
          <span className="grid max-w-64 gap-0.5">
            <span className="num text-fg-muted">{p.id}</span>
            <span>{p.author}</span>
            <span className="text-fg-muted">{p.license}</span>
          </span>
        }
      >
        <button
          type="button"
          aria-label={t.a11y.credits(p.name)}
          className="inline-flex size-7 shrink-0 items-center justify-center rounded-sm text-fg-muted hover:bg-active hover:text-fg"
        >
          <Info {...iconSm} />
        </button>
      </Tooltip>
      <div
        className={cn(
          "shrink-0 opacity-0 transition-opacity group-hover:opacity-100 focus-within:opacity-100",
          confirm && "opacity-100",
        )}
      >
        <IconButton
          label={confirm ? t.maps.removeConfirm : `${t.maps.remove} ${p.name}`}
          className={confirm ? "text-danger" : ""}
          onBlur={() => setConfirm(false)}
          onClick={() => (confirm ? send({ type: "removePack", packId: p.id }) : setConfirm(true))}
        >
          <Trash2 {...iconSm} />
        </IconButton>
      </div>
    </li>
  );
}

export function MapsPanel() {
  const view = useApp((s) => s.view);
  const openDialog = useApp((s) => s.openDialog);
  const packs = view?.packs ?? [];
  return (
    <PanelFrame
      title={t.panels.maps}
      actions={
        <IconButton label={t.maps.import} shortcut="I" onClick={() => openDialog("import")}>
          <Upload {...icon} />
        </IconButton>
      }
    >
      <section aria-label={t.tarkovdev.title} className="grid gap-2 border-b border-line p-3">
        <h3 className="text-xs font-medium text-fg-muted">{t.tarkovdev.title}</h3>
        <TarkovDevCard compact />
      </section>
      <h3 className="px-3 pt-3 pb-1 text-xs font-medium text-fg-muted">{t.maps.installed}</h3>
      {packs.length === 0 ? (
        <PanelNote>{t.maps.none}</PanelNote>
      ) : (
        <ul className="pb-2">
          {packs.map((p) => (
            <PackRow key={p.id} p={p} active={p.id === view?.activeMap?.id} />
          ))}
        </ul>
      )}
    </PanelFrame>
  );
}
