// Layers drawn on top of Leaflet. Everything arrives already projected from the backend: this only paints.
import L from "leaflet";
import type { ExtractView, MarkerView, PointView, View } from "../../lib/protocol";
import { cssVar, playerColor } from "../../lib/players";
import { send } from "../../lib/socket";
import { meters } from "../../lib/format";
import { useApp } from "../../store/app";
import { t } from "../../locales";
import { arrowIcon, extractIcon, imageIcon, markerIcon, pingIcon, pinIcon, toLatLng } from "./markers";
import { hoverCard, offFloor, otherFloor, tipOptions, type FloorCtx } from "./hover";
import { iconUrl } from "../../lib/iconCache";
import { extractNote } from "../../lib/mapText";

const TRAIL_CHUNKS = 8;
/** Things on another floor go behind the ones on the floor being shown. */
const BEHIND = -200;

export type Labelled = { latlng: L.LatLng; label: HTMLElement; priority: number };

function openMenu(
  e: L.LeafletMouseEvent,
  target: NonNullable<ReturnType<typeof useApp.getState>["menu"]>["target"],
) {
  L.DomEvent.stop(e);
  const oe = e.originalEvent;
  useApp.getState().setMenu({ x: oe.clientX, y: oe.clientY, px: e.latlng.lng, py: -e.latlng.lat, target });
}

const distance = (m: number | null) => (m === null ? [] : [t.hover.away(meters(m))]);

export function drawExtracts(
  group: L.LayerGroup,
  list: ExtractView[],
  selected: string | null,
  floors: FloorCtx,
): Labelled[] {
  return list.map((e) => {
    const isSel = e.id === selected;
    const off = offFloor(floors, e.floor);
    const { icon, label } = extractIcon(e, isSel, off);
    const latlng = toLatLng(e.px, e.py);
    const card = hoverCard({
      tone: e.faction,
      kind: t.hover.extract[e.faction],
      title: e.name,
      body: extractNote(e),
      chips: [...distance(e.distanceM), ...(e.available ? [t.extracts.available] : [])],
      floor: otherFloor(floors, e.floor),
      hint: isSel ? t.hover.stopGuide : t.hover.guide,
    });
    L.marker(latlng, {
      icon,
      title: e.name,
      keyboard: true,
      riseOnHover: true,
      zIndexOffset: off ? BEHIND : 0,
    })
      .bindTooltip(card, tipOptions(14))
      .on("click", () => send({ type: "selectExtract", extractId: isSel ? null : e.id }))
      .on("contextmenu", (ev: L.LeafletMouseEvent) => openMenu(ev, { kind: "extract", id: e.id }))
      .addTo(group);
    return { latlng, label, priority: (isSel ? 3 : 0) + (e.available ? 1 : 0) - (off ? 5 : 0) };
  });
}

export function drawPoints(
  group: L.LayerGroup,
  list: PointView[],
  selected: string | null,
  floors: FloorCtx,
): Labelled[] {
  return list.map((p) => {
    const isSel = p.id === selected;
    const off = offFloor(floors, p.floor);
    const { icon, label } = pinIcon(p.kind, p.name, isSel, off);
    const latlng = toLatLng(p.px, p.py);
    const card = hoverCard({
      tone: "point",
      kind: `${t.hover.point} · ${t.points.kinds[p.kind]}`,
      title: p.name,
      body: p.note,
      chips: distance(p.distanceM),
      floor: otherFloor(floors, p.floor),
      hint: isSel ? t.hover.stopGuide : t.hover.guide,
    });
    L.marker(latlng, {
      icon,
      title: p.name,
      keyboard: true,
      riseOnHover: true,
      zIndexOffset: off ? BEHIND : 500,
    })
      .bindTooltip(card, tipOptions(24))
      .on("click", () => send({ type: "selectExtract", extractId: isSel ? null : p.id }))
      .on("contextmenu", (ev: L.LeafletMouseEvent) => openMenu(ev, { kind: "point", id: p.id }))
      .addTo(group);
    return { latlng, label, priority: (isSel ? 4 : 2) - (off ? 5 : 0) };
  });
}

function markerCard(mk: MarkerView, floors: FloorCtx): HTMLElement {
  const tone = mk.kind === "loot" ? "loot" : mk.kind === "transit" ? "transit" : "switch";
  const kind =
    mk.kind === "loot" ? t.hover.container : mk.kind === "transit" ? t.hover.transit : t.hover.switch;
  return hoverCard({ tone, kind, title: mk.name, floor: otherFloor(floors, mk.floor) });
}

export function drawMarkers(
  group: L.LayerGroup,
  list: MarkerView[],
  on: (k: MarkerView["kind"]) => boolean,
  hidden: Set<string>,
  renderer: L.Canvas | null,
  floors: FloorCtx,
) {
  const fg = cssVar("--gray-12", "#ededed");
  const accent = cssVar("--accent", "#f0a43a");
  for (const mk of list) {
    if (mk.kind === "lock" || !on(mk.kind) || (mk.group && hidden.has(mk.group))) continue;
    const latlng = toLatLng(mk.px, mk.py);
    const off = offFloor(floors, mk.floor);
    const card = markerCard(mk, floors);
    const url = iconUrl(mk.icon);
    const z = off ? BEHIND : 0;
    if (url) {
      const size = mk.kind === "loot" ? 20 : 22;
      L.marker(latlng, { icon: imageIcon(url, size, off), title: mk.name, keyboard: false, zIndexOffset: z })
        .bindTooltip(card, tipOptions(size / 2 + 2))
        .addTo(group);
    } else if (mk.kind === "loot") {
      const base = { radius: 3, color: "#111", weight: 1, fillColor: fg, fillOpacity: off ? 0.25 : 0.85 };
      const dot = L.circleMarker(latlng, {
        ...base,
        renderer: renderer ?? undefined,
        opacity: off ? 0.3 : 1,
      });
      dot
        .bindTooltip(card, tipOptions(6))
        .on("mouseover", () => dot.setStyle({ radius: 5, fillColor: accent, fillOpacity: 1, opacity: 1 }))
        .on("mouseout", () => dot.setStyle({ ...base, opacity: off ? 0.3 : 1 }))
        .addTo(group);
    } else {
      L.marker(latlng, { icon: markerIcon(mk.kind, off), title: mk.name, keyboard: false, zIndexOffset: z })
        .bindTooltip(card, tipOptions(8))
        .addTo(group);
    }
  }
}

/** Fading trail, guide line, player, teammates and pings. */
export function drawDynamic(
  g: Record<string, L.LayerGroup | undefined>,
  view: View,
  showTrail: boolean,
  floors: FloorCtx,
) {
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
    const color = playerColor(p.color);
    const off = offFloor(floors, p.marker.floor);
    const card = hoverCard({
      tone: "peer",
      kind: t.hover.teammate,
      title: p.name,
      floor: otherFloor(floors, p.marker.floor),
      color,
    });
    L.marker(toLatLng(p.marker.px, p.marker.py), {
      icon: arrowIcon(p.marker.angle, color, off),
      title: p.name,
      zIndexOffset: 900,
    })
      .bindTooltip(card, tipOptions(14))
      .addTo(g.peers);
  }
  g.pings?.clearLayers();
  for (const p of view.party.pings) {
    if (g.pings)
      L.marker(toLatLng(p.px, p.py), { icon: pingIcon(p.color), interactive: false }).addTo(g.pings);
  }
}
