import { t } from "../locales";
import { send } from "../lib/socket";
import type { View } from "../lib/protocol";
import { useApp, type Workspace } from "../store/app";
import { copyPartyCode } from "../components/panels/PartyPanel";
import { fitWhole } from "../components/map/mapRef";
import { refreshMarket } from "../components/market/DataGate";
import { refreshQuests } from "../components/quests/QuestGate";

export interface Command {
  id: string;
  label: string;
  shortcut?: string;
  run: () => void;
}

/** Fuente única de acciones: la usan la paleta (Ctrl+K) y los atajos de teclado. */
export function buildCommands(view: View | null): Command[] {
  const s = useApp.getState();
  // Los paneles viven en la sección Mapa: abrir uno desde otra sección te lleva allí.
  const showPanel = (p: "extracts" | "points" | "party" | "maps" | "quests") => {
    if (useApp.getState().workspace !== "map") {
      s.setWorkspace("map");
      if (useApp.getState().panel === p) return;
    }
    s.togglePanel(p);
  };
  const go = (w: Workspace) => ({
    id: `go-${w}`,
    label: t.workspace.goTo(t.workspace[w]),
    run: () => s.setWorkspace(w),
  });
  const cmds: Command[] = [
    ...(["map", "market", "ammo", "quests"] as const).filter((w) => w !== s.workspace).map(go),
    { id: "refresh-prices", label: t.workspace.refresh, run: refreshMarket },
    { id: "refresh-quests", label: t.quests.data.refresh, run: refreshQuests },
    { id: "new-raid", label: t.palette.newRaid, shortcut: "N", run: () => send({ type: "newRaid" }) },
    { id: "import", label: t.palette.import, shortcut: "I", run: () => s.openDialog("import") },
    { id: "settings", label: t.palette.settings, shortcut: ",", run: () => s.openDialog("settings") },
    {
      id: "panel-extracts",
      label: t.palette.showExtracts,
      shortcut: "1",
      run: () => showPanel("extracts"),
    },
    { id: "panel-points", label: t.palette.showPoints, shortcut: "2", run: () => showPanel("points") },
    { id: "panel-party", label: t.palette.showParty, shortcut: "3", run: () => showPanel("party") },
    { id: "panel-maps", label: t.palette.showMaps, shortcut: "4", run: () => showPanel("maps") },
    { id: "panel-quests", label: t.quests.palette, shortcut: "5", run: () => showPanel("quests") },
    { id: "fit", label: t.palette.fit, shortcut: "F", run: fitWhole },
    {
      id: "follow",
      label: t.palette.follow,
      shortcut: "G",
      run: () => s.setPrefs({ follow: !useApp.getState().prefs.follow }),
    },
    { id: "changelog", label: t.changelog.open, run: () => s.openDialog("changelog") },
    { id: "check-updates", label: t.update.checkNow, run: () => send({ type: "checkUpdates" }) },
    { id: "quit", label: t.palette.quit, run: () => send({ type: "quit" }) },
  ];
  if (view?.party.code) {
    const code = view.party.code;
    cmds.splice(2, 0, {
      id: "copy-code",
      label: t.palette.copyCode,
      shortcut: "C",
      run: () => void copyPartyCode(code),
    });
  }
  for (const p of view?.packs ?? []) {
    if (p.id !== view?.activeMap?.id) {
      cmds.push({
        id: `map-${p.id}`,
        label: t.palette.switchMap(p.name),
        run: () => send({ type: "setActiveMap", mapId: p.id }),
      });
    }
  }
  return cmds;
}
