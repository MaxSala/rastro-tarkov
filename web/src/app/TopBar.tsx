import { Command, RotateCcw, Settings } from "lucide-react";
import { t } from "../locales";
import { icon } from "../lib/icons";
import { send } from "../lib/socket";
import { cn } from "../lib/cn";
import { useApp } from "../store/app";
import { IconButton } from "../components/ui/button";
import { FloorSelect } from "./FloorSelect";
import { RaidChip } from "./RaidChip";
import { WorkspaceTabs } from "./WorkspaceTabs";
import { Brand } from "../components/brand/Logo";

/** Solo aparece si estás en un party: Rastro no se conecta a ningún servidor por sí mismo. */
function PartyPill() {
  const party = useApp((s) => s.view?.party);
  if (!party || party.status === "idle") return null;
  const n = party.peers.length;
  const text =
    party.status === "hosting"
      ? t.partyPill.hosting(n)
      : party.status === "connected"
        ? t.partyPill.connected(n)
        : party.status === "connecting"
          ? t.partyPill.connecting
          : t.partyPill.error;
  const tone =
    party.status === "error"
      ? "bg-danger"
      : party.status === "connecting"
        ? "bg-fg-muted animate-pulse"
        : "bg-ok";
  return (
    <button
      type="button"
      onClick={() => useApp.getState().panel !== "party" && useApp.getState().togglePanel("party")}
      className="flex h-7 items-center gap-2 rounded-sm border border-line-strong px-2 text-xs text-fg-soft hover:bg-hover"
    >
      <span className={cn("size-1.5 rounded-full", tone)} />
      {text}
    </button>
  );
}

/** Mapa activo, piso y nivel: solo en la sección Mapa. */
function MapPicker() {
  const view = useApp((s) => s.view);
  const packs = view?.packs ?? [];
  const active = view?.activeMap;
  return (
    <>
      <div className="h-4 w-px bg-line" />
      <label className="sr-only" htmlFor="map-select">
        {t.top.chooseMap}
      </label>
      <select
        id="map-select"
        value={active?.id ?? ""}
        disabled={packs.length === 0}
        onChange={(e) => send({ type: "setActiveMap", mapId: e.target.value })}
        className="h-7 max-w-56 rounded-sm border border-line-strong bg-app px-2 text-sm text-fg disabled:text-fg-muted"
      >
        {packs.length === 0 && <option value="">{t.top.noMap}</option>}
        {packs.map((p) => (
          <option key={p.id} value={p.id}>
            {p.name}
          </option>
        ))}
      </select>
      <FloorSelect />
      {active?.level && (
        <span className="text-xs text-fg-muted">
          {t.top.level} <span className="text-fg-soft">{active.level}</span>
        </span>
      )}
    </>
  );
}

export function TopBar() {
  const openDialog = useApp((s) => s.openDialog);
  const workspace = useApp((s) => s.workspace);
  return (
    <header className="flex h-11 items-center gap-3 border-b border-line bg-surface px-3">
      <Brand />
      <WorkspaceTabs />
      {workspace === "map" && <MapPicker />}
      <div className="flex-1" />
      {workspace === "map" && <RaidChip />}
      <PartyPill />
      <div className="flex items-center gap-0.5">
        {workspace === "map" && (
          <IconButton label={t.top.newRaid} shortcut="N" onClick={() => send({ type: "newRaid" })}>
            <RotateCcw {...icon} />
          </IconButton>
        )}
        <IconButton label={t.top.commands} shortcut="Ctrl K" onClick={() => openDialog("palette")}>
          <Command {...icon} />
        </IconButton>
        <IconButton label={t.top.settings} shortcut="," onClick={() => openDialog("settings")}>
          <Settings {...icon} />
        </IconButton>
      </div>
    </header>
  );
}
