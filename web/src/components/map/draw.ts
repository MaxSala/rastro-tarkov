// Dibujo de capas sobre Leaflet. Todo llega ya proyectado desde el backend: aquí solo se pinta.
import L from "leaflet";
import type { ExtractView, MarkerView, PointView, View } from "../../lib/protocol";
import { cssVar, playerColor } from "../../lib/players";
import { send } from "../../lib/socket";
import { useApp } from "../../store/app";
import {
  arrowIcon,
  extractIcon,
  imageIcon,
  markerIcon,
  pingIcon,
  pinIcon,
  toLatLng,
  tooltipContent,
} from "./markers";
import { iconUrl } from "../../lib/iconCache";
import { extractNote } from "../../lib/mapText";

const TRAIL_CHUNKS = 8;
const tip = { className: "rt-tip", direction: "top" as const, offset: [0, -8] as [number, number] };

export type Labelled = { latlng: L.LatLng; label: HTMLElement; priority: number };

function openMenu(
  e: L.LeafletMouseEvent,
  target: NonNullable<ReturnType<typeof useApp.getState>["menu"]>["target"],
) {
  L.DomEvent.stop(e);
  const oe = e.originalEvent;
  useApp.getState().setMenu({ x: oe.clientX, y: oe.clientY, px: e.latlng.lng, py: -e.latlng.lat, target });
}

export function drawExtracts(group: L.LayerGroup, list: ExtractView[], selected: string | null): Labelled[] {
  return list.map((e) => {
    const isSel = e.id === selected;
    const { icon, label } = extractIcon(e, isSel);
    const latlng = toLatLng(e.px, e.py);
    L.marker(latlng, { icon, title: e.name, keyboard: true, riseOnHover: true })
      .bindTooltip(tooltipContent(e.name, extractNote(e)), tip)
      .on("click", () => send({ type: "selectExtract", extractId: isSel ? null : e.id }))
      .on("contextmenu", (ev: L.LeafletMouseEvent) => openMenu(ev, { kind: "extract", id: e.id }))
      .addTo(group);
    return { latlng, label, priority: (isSel ? 3 : 0) + (e.available ? 1 : 0) };
  });
}

export function drawPoints(group: L.LayerGroup, list: PointView[], selected: string | null): Labelled[] {
  return list.map((p) => {
    const isSel = p.id === selected;
    const { icon, label } = pinIcon(p.kind, p.name, isSel);
    const latlng = toLatLng(p.px, p.py);
    L.marker(latlng, { icon, title: p.name, keyboard: true, riseOnHover: true, zIndexOffset: 500 })
      .bindTooltip(tooltipContent(p.name, p.note), { ...tip, offset: [0, -20] })
      .on("click", () => send({ type: "selectExtract", extractId: isSel ? null : p.id }))
      .on("contextmenu", (ev: L.LeafletMouseEvent) => openMenu(ev, { kind: "point", id: p.id }))
      .addTo(group);
    return { latlng, label, priority: isSel ? 4 : 2 };
  });
}

export function drawMarkers(
  group: L.LayerGroup,
  list: MarkerView[],
  on: (k: MarkerView["kind"]) => boolean,
  hidden: Set<string>,
  renderer: L.Canvas | null,
) {
  const fg = cssVar("--gray-12", "#ededed");
  for (const mk of list) {
    if (mk.kind === "lock" || !on(mk.kind) || (mk.group && hidden.has(mk.group))) continue;
    const latlng = toLatLng(mk.px, mk.py);
    const smallTip = { ...tip, offset: [0, -6] as [number, number] };
    const url = iconUrl(mk.icon);
    if (url) {
      const size = mk.kind === "loot" ? 20 : 22;
      L.marker(latlng, { icon: imageIcon(url, size), title: mk.name, keyboard: false })
        .bindTooltip(tooltipContent(mk.name, ""), smallTip)
        .addTo(group);
    } else if (mk.kind === "loot") {
      L.circleMarker(latlng, {
        renderer: renderer ?? undefined,
        radius: 3,
        color: "#111",
        weight: 1,
        fillColor: fg,
        fillOpacity: 0.85,
      })
        .bindTooltip(tooltipContent(mk.name, ""), smallTip)
        .addTo(group);
    } else {
      L.marker(latlng, { icon: markerIcon(mk.kind), title: mk.name, keyboard: false })
        .bindTooltip(tooltipContent(mk.name, ""), smallTip)
        .addTo(group);
    }
  }
}

/** Rastro que se desvanece, guía, jugador, compañeros y pings. */
export function drawDynamic(g: Record<string, L.LayerGroup | undefined>, view: View, showTrail: boolean) {
  const accent = cssVar("--accent", "#f0a43a");
  g.trail?.clearLayers();
  if (showTrail && g.trail) {
    const pts = view.trail.map(([x, y]) => toLatLng(x, y));
    const size = Math.max(2, Math.ceil(pts.length / TRAIL_CHUNKS));
    const chunks = Math.max(1, Math.ceil(pts.length / (size - 1)));
    for (let i = 0, c = 0; i < pts.length - 1; i += size - 1, c++) {
      const opacity = 0.15 + 0.65 * ((c + 1) / chunks);
      L.polyline(pts.slice(i, i + size), { color: accent, weight: 2, opacity, interactive: false }).addTo(
        g.trail,
      );
    }
  }
  g.guide?.clearLayers();
  if (view.guide && g.guide) {
    const line = [toLatLng(...view.guide.from), toLatLng(...view.guide.to)];
    L.polyline(line, { color: accent, weight: 2, dashArray: "6 6", interactive: false }).addTo(g.guide);
  }
  g.player?.clearLayers();
  if (view.player && g.player) {
    const icon = arrowIcon(view.player.angle, accent);
    L.marker(toLatLng(view.player.px, view.player.py), {
      icon,
      interactive: false,
      zIndexOffset: 1000,
    }).addTo(g.player);
  }
  g.peers?.clearLayers();
  for (const p of view.party.peers) {
    if (!p.marker || !g.peers) continue;
    L.marker(toLatLng(p.marker.px, p.marker.py), {
      icon: arrowIcon(p.marker.angle, playerColor(p.color)),
      title: p.name,
      zIndexOffset: 900,
    })
      .bindTooltip(tooltipContent(p.name, ""), { ...tip, offset: [0, -10] })
      .addTo(g.peers);
  }
  g.pings?.clearLayers();
  for (const p of view.party.pings) {
    if (g.pings)
      L.marker(toLatLng(p.px, p.py), { icon: pingIcon(p.color), interactive: false }).addTo(g.pings);
  }
}
