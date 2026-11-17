// Capa de cerraduras. Cada una lleva el id de su llave; el nombre y el precio salen del catálogo
// del Mercado si está descargado. Clic: tarjeta de la llave. Con una llave resaltada, el resto se atenúa.
import L from "leaflet";
import { itemById, type Catalog } from "../../lib/catalog";
import type { MarkerView } from "../../lib/protocol";
import { t } from "../../locales";
import { useApp } from "../../store/app";
import { toLatLng, tooltipContent } from "./markers";

const SVG_NS = "http://www.w3.org/2000/svg";
export type LockType = keyof typeof t.keys.types;

export const lockType = (group: string | null): LockType =>
  group === "door" || group === "container" || group === "trunk" ? group : "lock";

/** Nombre a mostrar: catálogo (idioma actual), texto de tarkov.dev o «sin identificar». */
export function keyName(key: string | undefined, fallback: string, catalog: Catalog | null): string {
  return itemById(catalog, key)?.name ?? (fallback || t.keys.unknown);
}

function glyph(): SVGSVGElement {
  const svg = document.createElementNS(SVG_NS, "svg");
  svg.setAttribute("viewBox", "0 0 24 24");
  svg.setAttribute("aria-hidden", "true");
  const body = document.createElementNS(SVG_NS, "rect");
  for (const [k, v] of Object.entries({ x: "5", y: "11", width: "14", height: "10", rx: "2" }))
    body.setAttribute(k, v);
  const shackle = document.createElementNS(SVG_NS, "path");
  shackle.setAttribute("d", "M8 11V8a4 4 0 0 1 8 0v3");
  svg.append(shackle, body);
  return svg;
}

export function drawLocks(
  group: L.LayerGroup,
  list: MarkerView[],
  focus: string | null,
  catalog: Catalog | null,
) {
  const tip = { className: "rt-tip", direction: "top" as const, offset: [0, -10] as [number, number] };
  for (const m of list) {
    if (m.kind !== "lock" || !m.key) continue;
    const key = m.key;
    const focused = focus === key;
    const root = document.createElement("div");
    root.className = "rt-lock";
    root.dataset.state = focus === null ? "idle" : focused ? "focus" : "dim";
    root.dataset.power = String(Boolean(m.power));
    root.append(glyph());
    const size = focused ? 26 : 20;
    const name = keyName(key, m.name, catalog);
    const detail = [t.keys.types[lockType(m.group)], m.power ? t.keys.needsPower : ""]
      .filter(Boolean)
      .join(" · ");
    const icon = L.divIcon({
      html: root,
      className: "",
      iconSize: [size, size],
      iconAnchor: [size / 2, size / 2],
    });
    const open = (e: L.LeafletMouseEvent) => {
      L.DomEvent.stop(e);
      (e.target as L.Marker).closeTooltip(); // la tarjeta ocupa su sitio
      const oe = e.originalEvent;
      useApp.getState().setKeyCard({
        x: oe.clientX,
        y: oe.clientY,
        key,
        group: m.group,
        power: Boolean(m.power),
        name: m.name,
      });
    };
    L.marker(toLatLng(m.px, m.py), { icon, title: name, keyboard: false, zIndexOffset: focused ? 1000 : 0 })
      .bindTooltip(tooltipContent(name, detail), tip)
      .on("click", open)
      .on("contextmenu", open)
      .addTo(group);
  }
}

/** Puntos de las cerraduras de una llave, para encuadrarlas al resaltarla. */
export function lockPoints(list: MarkerView[], key: string): L.LatLng[] {
  return list.filter((m) => m.kind === "lock" && m.key === key).map((m) => toLatLng(m.px, m.py));
}
