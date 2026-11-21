import L from "leaflet";
import { useCallback, useEffect, useMemo, useRef, useState, useSyncExternalStore } from "react";
import { t } from "../../locales";
import type { View } from "../../lib/protocol";
import { send } from "../../lib/socket";
import { currentTheme, subscribeTheme } from "../../lib/theme";
import { useApp } from "../../store/app";
import { Button } from "../ui/button";
import { Skeleton } from "../ui/skeleton";
import { drawDynamic, drawExtracts, drawMarkers, drawPoints } from "./draw";
import { mapRef } from "./mapRef";
import { declutter, toLatLng } from "./markers";
import { drawLocks, useLockFocus } from "./locks";
import { drawBosses } from "./bosses";
import { useAutoCatalog } from "../market/DataGate";
import { useMapImage } from "./useMapImage";
import { NO_FLOORS, type FloorCtx } from "./hover";
import { drawQuests } from "./quests";
import { useQuestOptions } from "./useQuestOptions";
import { useAutoQuests, useQuests } from "../../lib/quests";

type ActiveMap = NonNullable<View["activeMap"]>;
const LAYERS = [
  "base",
  "trail",
  "markers",
  "bosses",
  "extracts",
  "points",
  "quests",
  "guide",
  "peers",
  "pings",
  "player",
] as const;

export function MapView({ active }: { active: ActiveMap }) {
  const host = useRef<HTMLDivElement>(null);
  const map = useRef<L.Map | null>(null);
  const groups = useRef<Record<string, L.LayerGroup>>({});
  const canvas = useRef<L.Canvas | null>(null);
  const fit = useRef({ userMoved: false });
  const userFlew = useCallback(() => {
    fit.current.userMoved = true;
  }, []);
  const [nonce, setNonce] = useState(0);
  const floorChoice = useApp((s) => s.floors[active.id] ?? "auto");
  const floor = floorChoice === "auto" ? active.autoFloor : floorChoice;
  const imagePath =
    active.floors.length > 0 && floor
      ? `${active.imageUrl}?floor=${encodeURIComponent(floor)}`
      : active.imageUrl;
  const image = useMapImage(imagePath, nonce);
  // The floor on screen: the one picked, the one you're on, or the base floor.
  const shownFloor = active.floors.length > 0 ? (floor ?? active.baseFloor) : null;
  const floorNames = JSON.stringify(active.floors);
  const floors = useMemo<FloorCtx>(
    () =>
      shownFloor
        ? {
            shown: shownFloor,
            names: Object.fromEntries(
              (JSON.parse(floorNames) as ActiveMap["floors"]).map((f) => [f.id, f.name]),
            ),
          }
        : NO_FLOORS,
    [shownFloor, floorNames],
  );
  const hasShown = useRef(false);
  const hiddenGroups = useApp((s) => s.hiddenGroups);
  const iconsVersion = useApp((s) => s.iconsVersion);
  const view = useApp((s) => s.view);
  const mapMarkers = useApp((s) => s.markers);
  const layerOn = useApp((s) => s.layers);
  const focusKey = useApp((s) => s.focusKey);
  const { catalog } = useAutoCatalog(layerOn.lock);
  const questMarkers = view?.questMarkers;
  const { db: questDb } = useAutoQuests((questMarkers?.length ?? 0) > 0);
  const questDone = useQuests((s) => s.progress?.objectives);
  const questFocus = useApp((s) => s.questFocus);
  const questOpts = useQuestOptions(active.id, questMarkers);
  const prefs = useApp((s) => s.prefs);
  const theme = useSyncExternalStore(subscribeTheme, currentTheme);

  // Mapa: se crea una vez. Sin controles por defecto; zoom con rueda, teclado y la barra propia.
  useEffect(() => {
    const el = host.current;
    if (!el || map.current) return;
    const m = L.map(el, {
      crs: L.CRS.Simple,
      zoomControl: false,
      attributionControl: false,
      minZoom: -4,
      maxZoom: 4,
      zoomSnap: 0.25,
      zoomDelta: 0.5,
      wheelPxPerZoomLevel: 120,
      keyboard: true,
    });
    m.setView([0, 0], 0); // Leaflet exige centro y zoom antes de proyectar nada.
    canvas.current = L.canvas({ padding: 0.5 });
    for (const name of LAYERS) groups.current[name] = L.layerGroup().addTo(m);
    m.on("click", (e: L.LeafletMouseEvent) => {
      useApp.getState().setMenu(null);
      if (e.originalEvent.shiftKey) send({ type: "mapPing", px: e.latlng.lng, py: -e.latlng.lat });
    });
    m.on("contextmenu", (e: L.LeafletMouseEvent) => {
      const oe = e.originalEvent;
      useApp.getState().setMenu({
        x: oe.clientX,
        y: oe.clientY,
        px: e.latlng.lng,
        py: -e.latlng.lat,
        target: { kind: "map" },
      });
    });
    m.on("movestart", () => useApp.getState().setMenu(null));
    const userMoved = () => {
      fit.current.userMoved = true;
    };
    m.on("dragstart", userMoved);
    el.addEventListener("wheel", userMoved, { passive: true });
    // Si el contenedor cambia de tamaño, Leaflet debe saberlo; sin interacción, se reencuadra.
    const ro = new ResizeObserver(() => {
      m.invalidateSize({ animate: false });
      if (mapRef.bounds && !fit.current.userMoved) m.fitBounds(mapRef.bounds, { animate: false });
    });
    ro.observe(el);
    map.current = m;
    mapRef.map = m;
    return () => {
      ro.disconnect();
      m.remove();
      map.current = null;
      mapRef.map = null;
    };
  }, []);

  // Imagen base. Encuadrar ANTES de fijar límites: al revés, Leaflet lanza un desplazamiento
  // animado que deja la imagen y el lienzo calculados con un origen antiguo.
  useEffect(() => {
    const m = map.current;
    const base = groups.current.base;
    if (!m || !base || image.status !== "ready") return;
    const bounds = L.latLngBounds(toLatLng(0, active.height), toLatLng(active.width, 0));
    base.clearLayers();
    L.imageOverlay(image.url, bounds, { interactive: false }).addTo(base);
    mapRef.bounds = bounds;
    // Solo se encuadra la primera vez: cambiar de piso no debe mover tu vista.
    if (!hasShown.current) {
      hasShown.current = true;
      fit.current.userMoved = false;
      m.invalidateSize({ animate: false });
      m.fitBounds(bounds, { animate: false });
      m.setMaxBounds(bounds.pad(0.5));
    }
  }, [image, active.width, active.height]);

  // Etiquetas y tamaño de marcadores: atributos del contenedor que lee el CSS.
  useEffect(() => {
    const el = host.current;
    if (!el) return;
    el.dataset.labels = prefs.labels ? "on" : "off";
    el.style.setProperty("--rt-scale", String(prefs.markerScale));
  }, [prefs.labels, prefs.markerScale]);

  const staticKey = JSON.stringify([
    view?.extracts,
    view?.points,
    view?.selectedExtract,
    layerOn.extracts,
    iconsVersion,
    floors,
  ]);
  useEffect(() => {
    const m = map.current;
    const { extracts, points } = groups.current;
    if (!m || !extracts || !points || !view) return;
    extracts.clearLayers();
    points.clearLayers();
    const items = [
      ...(layerOn.extracts ? drawExtracts(extracts, view.extracts, view.selectedExtract, floors) : []),
      ...drawPoints(points, view.points, view.selectedExtract, floors),
    ];
    const run = () => declutter(m, items);
    requestAnimationFrame(run);
    m.on("zoomend", run);
    return () => {
      m.off("zoomend", run);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps -- la clave serializada resume las dependencias
  }, [staticKey]);

  useEffect(() => {
    const g = groups.current.markers;
    if (!g) return;
    g.clearLayers();
    if (mapMarkers && mapMarkers.mapId === active.id) {
      drawMarkers(
        g,
        mapMarkers.list,
        (k) => k !== "lock" && layerOn[k],
        new Set(hiddenGroups),
        canvas.current,
        floors,
      );
      if (layerOn.lock) drawLocks(g, mapMarkers.list, focusKey, catalog, floors);
    }
  }, [mapMarkers, layerOn, active.id, theme, hiddenGroups, iconsVersion, focusKey, catalog, floors]);

  // Quest objectives; the first time a previewed quest shows up, frame its spots.
  const questState = [questMarkers, view?.selectedExtract, layerOn.quests, floors, questDone, questOpts.key];
  const questKey = JSON.stringify(questState);
  const framedQuest = useRef("");
  useEffect(() => {
    const m = map.current;
    const g = groups.current.quests;
    if (!m || !g) return;
    g.clearLayers();
    if (!layerOn.quests || !questMarkers || image.status !== "ready") return;
    const sel = useApp.getState().view?.selectedExtract ?? null;
    const focused = drawQuests(g, questMarkers, questDb, questDone ?? {}, sel, floors, questFocus, questOpts);
    const id = `${active.id}:${questFocus}`;
    if (questFocus && focused.length > 0 && framedQuest.current !== id) {
      framedQuest.current = id;
      fit.current.userMoved = true;
      const reduce = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
      m.flyToBounds(L.latLngBounds(focused), {
        padding: [140, 140],
        maxZoom: 1,
        animate: !reduce,
        duration: 0.6,
      });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps -- la clave serializada resume las dependencias
  }, [questKey, questDb, questFocus, active.id, image.status]);

  useLockFocus(map, active.id, focusKey, mapMarkers, userFlew);

  // Bosses: where each one can appear on this map, under the quests.
  const bossKey = JSON.stringify([view?.bosses.markers, layerOn.bosses, floors]);
  useEffect(() => {
    const g = groups.current.bosses;
    if (!g) return;
    g.clearLayers();
    const list = useApp.getState().view?.bosses.markers;
    if (layerOn.bosses && list && image.status === "ready") drawBosses(g, list, floors);
    // eslint-disable-next-line react-hooks/exhaustive-deps -- la clave serializada resume las dependencias
  }, [bossKey, image.status]);

  const dynamicKey = JSON.stringify([
    theme,
    prefs.trail,
    view?.trail,
    view?.player,
    view?.guide,
    view?.party.peers,
    view?.party.pings,
    floors,
  ]);
  useEffect(() => {
    if (view) drawDynamic(groups.current, view, prefs.trail, floors);
    // eslint-disable-next-line react-hooks/exhaustive-deps -- la clave serializada resume las dependencias
  }, [dynamicKey]);

  // Seguir al jugador: centra sin cambiar el zoom cada vez que llega una captura nueva.
  const playerKey = view?.player ? `${view.player.px},${view.player.py}` : "";
  useEffect(() => {
    const m = map.current;
    const p = useApp.getState().view?.player;
    if (!m || !p || !prefs.follow) return;
    const reduce = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
    m.panTo(toLatLng(p.px, p.py), { animate: !reduce });
  }, [playerKey, prefs.follow]);

  const overlay = useMemo(() => {
    if (image.status === "loading" || image.status === "idle")
      return <Skeleton className="absolute inset-4" />;
    if (image.status === "error")
      return (
        <div className="absolute inset-0 grid place-items-center">
          <div className="grid justify-items-center gap-3 text-center">
            <p className="text-sm text-fg-soft">{t.errors.image}</p>
            <p className="max-w-80 text-xs text-fg-muted">{image.message}</p>
            <Button size="sm" onClick={() => setNonce((n) => n + 1)}>
              {t.connection.retry}
            </Button>
          </div>
        </div>
      );
    return null;
  }, [image]);

  return (
    <div className="relative h-full w-full">
      <div ref={host} className="absolute inset-0" aria-label={active.name} />
      {overlay}
    </div>
  );
}
