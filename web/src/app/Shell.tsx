import { useMemo } from "react";
import { useHotkeys, type Hotkey } from "../lib/hotkeys";
import { send } from "../lib/socket";
import { useApp } from "../store/app";
import { MapView } from "../components/map/MapView";
import { ExtractsPanel } from "../components/panels/ExtractsPanel";
import { HostDialog } from "../components/panels/HostDialog";
import { ImportDialog } from "../components/panels/ImportDialog";
import { MapsPanel } from "../components/panels/MapsPanel";
import { PointDialog } from "../components/panels/PointDialog";
import { PointsPanel } from "../components/panels/PointsPanel";
import { PartyPanel } from "../components/panels/PartyPanel";
import { SettingsDialog } from "../components/panels/settings/SettingsDialog";
import { Toaster } from "../components/ui/toaster";
import { buildCommands } from "./commands";
import { zoomBy } from "../components/map/mapRef";
import { CommandPalette } from "./CommandPalette";
import { ConnectionBanner } from "./ConnectionBanner";
import { ContextMenu } from "./ContextMenu";
import { KeyCard } from "./KeyCard";
import { MapToolbar } from "./MapToolbar";
import { EmptyMap } from "./EmptyMap";
import { GuideReadout } from "./GuideReadout";
import { LayerControl } from "./LayerControl";
import { Onboarding } from "./Onboarding";
import { Rail } from "./Rail";
import { StatusBar } from "./StatusBar";
import { TopBar } from "./TopBar";
import { MarketView } from "../components/market/MarketView";
import { AmmoView } from "../components/ammo/AmmoView";

export function Shell() {
  const view = useApp((s) => s.view);
  const panel = useApp((s) => s.panel);
  const openDialog = useApp((s) => s.openDialog);
  const workspace = useApp((s) => s.workspace);

  const hotkeys = useMemo<Hotkey[]>(() => {
    const fromCommands = buildCommands(view)
      .filter((c) => c.shortcut?.length === 1)
      .map((c) => ({ key: (c.shortcut ?? "").toLowerCase(), run: c.run }));
    return [
      {
        key: "k",
        ctrl: true,
        run: () => openDialog(useApp.getState().dialog === "palette" ? null : "palette"),
      },
      ...(["map", "market", "ammo"] as const).map((w, i) => ({
        key: String(i + 1),
        ctrl: true,
        run: () => useApp.getState().setWorkspace(w),
      })),
      { key: "escape", run: () => send({ type: "selectExtract", extractId: null }) },
      { key: "+", run: () => zoomBy(0.5) },
      { key: "-", run: () => zoomBy(-0.5) },
      ...fromCommands,
    ];
  }, [view, openDialog]);
  useHotkeys(hotkeys);

  return (
    <div className="grid h-full grid-rows-[auto_1fr_auto]">
      <TopBar />
      {workspace === "map" ? (
        <div className="flex min-h-0">
          <Rail />
          {panel === "extracts" && <ExtractsPanel />}
          {panel === "points" && <PointsPanel />}
          {panel === "party" && <PartyPanel />}
          {panel === "maps" && <MapsPanel />}
          <main className="relative min-w-0 flex-1" onContextMenu={(e) => e.preventDefault()}>
            {view?.activeMap ? <MapView key={view.activeMap.id} active={view.activeMap} /> : <EmptyMap />}
            {view?.activeMap && <LayerControl />}
            {view?.activeMap && <MapToolbar />}
            <GuideReadout />
            <ConnectionBanner />
          </main>
        </div>
      ) : (
        <main className="relative min-h-0" key={workspace}>
          {workspace === "market" ? <MarketView /> : <AmmoView />}
          <ConnectionBanner />
        </main>
      )}
      <StatusBar />
      <ImportDialog />
      <SettingsDialog />
      <HostDialog />
      <CommandPalette />
      <Onboarding />
      <PointDialog />
      <ContextMenu />
      <KeyCard />
      <Toaster />
    </div>
  );
}
