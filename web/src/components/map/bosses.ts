// Bosses on the map: one red badge per zone where a boss can appear, at the zone's most central
// spawn point, with the chance of finding it there. Hovering (or clicking, to keep them) shows
// every spawn point of the zone. Texts come from tarkov.dev; positions arrive projected.
import L from "leaflet";
import { t } from "../../locales";
import { pct, type BossMarker } from "../../lib/bossProtocol";
import { hoverCard, offFloor, otherFloor, tipOptions, type FloorCtx } from "./hover";
import { toLatLng } from "./markers";
import "./bosses.css";

const SVG = "http://www.w3.org/2000/svg";
// Skull, from Lucide (ISC), already a dependency of the interface.
const SKULL: [string, Record<string, string>][] = [
  ["circle", { cx: "9", cy: "12", r: "1" }],
  ["circle", { cx: "15", cy: "12", r: "1" }],
  ["path", { d: "M8 20v2h8v-2" }],
  ["path", { d: "m12.5 17-.5-1-.5 1h1z" }],
  ["path", { d: "M16 20a2 2 0 0 0 1.56-3.25 8 8 0 1 0-11.12 0A2 2 0 0 0 8 20" }],
];

function badge(m: BossMarker, off: boolean): L.DivIcon {
  const root = document.createElement("div");
  root.className = "rt-boss";
  root.dataset.off = String(off);
  const svg = document.createElementNS(SVG, "svg");
  svg.setAttribute("viewBox", "0 0 24 24");
  svg.setAttribute("aria-hidden", "true");
  for (const [tag, attrs] of SKULL) {
    const node = document.createElementNS(SVG, tag);
    for (const [k, v] of Object.entries(attrs)) node.setAttribute(k, v);
    svg.append(node);
  }
  const label = document.createElement("span");
  label.className = "rt-boss-c";
  label.textContent = pct(m.chance * m.zoneChance);
  root.append(svg, label);
  return L.divIcon({ html: root, className: "", iconSize: [24, 24], iconAnchor: [12, 12] });
}

function spawnPoints(m: BossMarker): L.LayerGroup {
  const g = L.layerGroup();
  for (const [x, y] of m.points)
    L.circleMarker(toLatLng(x, y), { radius: 4, className: "rt-boss-pt", interactive: false }).addTo(g);
  return g;
}

export function drawBosses(group: L.LayerGroup, markers: BossMarker[], floors: FloorCtx): void {
  for (const m of markers) {
    const off = offFloor(floors, m.floor);
    const chips = [
      ...(m.zone ? [m.zone] : []),
      ...(m.zoneChance < 1 ? [t.bosses.zone(pct(m.zoneChance))] : []),
      ...(m.escorts.length > 0 ? [t.bosses.with(m.escorts.join(", "))] : []),
      ...(m.time ? [t.bosses.arrives(Math.round(m.time / 60))] : []),
      ...(m.triggered ? [t.bosses.triggered] : []),
    ];
    const card = hoverCard({
      tone: "boss",
      kind: `${t.bosses.kind} · ${t.bosses.onMap(pct(m.chance))}`,
      title: m.boss,
      chips,
      floor: otherFloor(floors, m.floor),
      hint: m.points.length > 1 ? t.bosses.points : undefined,
    });
    let shown: L.LayerGroup | null = null;
    let pinned = false;
    const show = () => {
      if (!shown) shown = spawnPoints(m).addTo(group);
    };
    const hide = () => {
      shown?.remove();
      shown = null;
    };
    L.marker(toLatLng(m.px, m.py), {
      icon: badge(m, off),
      title: m.boss,
      keyboard: true,
      riseOnHover: true,
      zIndexOffset: off ? -250 : 450,
    })
      .bindTooltip(card, tipOptions(12))
      .on("mouseover", show)
      .on("mouseout", () => {
        if (!pinned) hide();
      })
      .on("click", () => {
        pinned = !pinned;
        if (pinned) show();
        else hide();
      })
      .addTo(group);
  }
}
