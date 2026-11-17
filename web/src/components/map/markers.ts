// Constructores de marcadores con nodos DOM. Nunca se usa HTML en texto: todo va por textContent.
import L from "leaflet";
import type { ExtractView, PointKind } from "../../lib/protocol";
import { kindColor } from "../../lib/pointKinds";
import { iconUrl } from "../../lib/iconCache";
import { playerVar } from "../../lib/players";

const SVG_NS = "http://www.w3.org/2000/svg";

/** Píxel del mapa → LatLng en CRS.Simple (y crece hacia abajo en la imagen). */
export const toLatLng = (px: number, py: number) => L.latLng(-py, px);

function el<K extends keyof HTMLElementTagNameMap>(tag: K, className: string, text?: string) {
  const node = document.createElement(tag);
  node.className = className;
  if (text !== undefined) node.textContent = text;
  return node;
}

export function tooltipContent(title: string, body: string): HTMLElement {
  const root = el("div", "");
  root.append(el("div", "rt-tip-title", title));
  if (body) root.append(el("div", "rt-tip-body", body));
  return root;
}

function img(url: string, size: number): HTMLImageElement {
  const i = document.createElement("img");
  i.src = url;
  i.alt = "";
  i.width = size;
  i.height = size;
  i.draggable = false;
  return i;
}

/** Icono de tarkov.dev como marcador (sin etiqueta). */
export function imageIcon(url: string, size: number): L.DivIcon {
  const root = el("div", "rt-ico");
  root.style.width = `${size}px`;
  root.style.height = `${size}px`;
  root.append(img(url, size));
  return L.divIcon({ html: root, className: "", iconSize: [size, size], iconAnchor: [size / 2, size / 2] });
}

export function extractIcon(e: ExtractView, selected: boolean): { icon: L.DivIcon; label: HTMLElement } {
  const url = iconUrl(`extract_${e.faction}`);
  const root = el("div", "rt-marker");
  if (url) {
    const wrap = el("div", "rt-ico rt-ico-exfil");
    wrap.dataset.dimmed = String(e.dimmed);
    wrap.dataset.selected = String(selected);
    wrap.append(img(url, 24));
    const label = el("span", "rt-label", e.name);
    root.append(wrap, label);
    return {
      icon: L.divIcon({ html: root, className: "", iconSize: [24, 24], iconAnchor: [12, 12] }),
      label,
    };
  }
  const shape = el("div", "rt-exfil");
  shape.dataset.faction = e.faction;
  shape.dataset.dimmed = String(e.dimmed);
  shape.dataset.selected = String(selected);
  shape.dataset.available = String(e.available);
  const label = el("span", "rt-label", e.name);
  root.append(shape, label);
  return { icon: L.divIcon({ html: root, className: "", iconSize: [16, 16], iconAnchor: [8, 8] }), label };
}

/** Transición (triángulo) o interruptor (cuadrado hueco). */
export function markerIcon(kind: "transit" | "switch"): L.DivIcon {
  const root = el("div", "rt-mk");
  root.dataset.kind = kind;
  return L.divIcon({ html: root, className: "", iconSize: [12, 12], iconAnchor: [6, 6] });
}

function arrowSvg(fill: string): SVGSVGElement {
  const svg = document.createElementNS(SVG_NS, "svg");
  svg.setAttribute("viewBox", "0 0 24 24");
  svg.setAttribute("width", "24");
  svg.setAttribute("height", "24");
  svg.setAttribute("aria-hidden", "true");
  const path = document.createElementNS(SVG_NS, "path");
  path.setAttribute("d", "M12 2 20 21 12 16.5 4 21Z");
  path.setAttribute("fill", fill);
  path.setAttribute("stroke", "#111");
  path.setAttribute("stroke-width", "1.5");
  path.setAttribute("stroke-linejoin", "round");
  svg.append(path);
  return svg;
}

/** Flecha con rumbo. `angle` ya viene calculado por el backend en grados de pantalla. */
export function arrowIcon(angle: number, color: string): L.DivIcon {
  const root = el("div", "rt-arrow");
  root.style.transform = `rotate(${angle}deg)`;
  root.append(arrowSvg(color));
  return L.divIcon({ html: root, className: "", iconSize: [24, 24], iconAnchor: [12, 12] });
}

export function pingIcon(color: number): L.DivIcon {
  const root = el("div", "rt-ping");
  root.style.color = playerVar(color);
  return L.divIcon({ html: root, className: "", iconSize: [28, 28], iconAnchor: [14, 14] });
}

/** Oculta etiquetas que se solapan; prioridad: seleccionada, disponibles y luego el resto. */
export function declutter(map: L.Map, items: { latlng: L.LatLng; label: HTMLElement; priority: number }[]) {
  const placed: DOMRect[] = [];
  const sorted = [...items].sort((a, b) => b.priority - a.priority);
  for (const it of sorted) {
    it.label.dataset.hidden = "false";
    const p = map.latLngToContainerPoint(it.latlng);
    const w = it.label.offsetWidth || (it.label.textContent ?? "").length * 6.5;
    const box = new DOMRect(p.x - w / 2 - 2, p.y + 12, w + 4, 16);
    const hit = placed.some(
      (r) => box.left < r.right && box.right > r.left && box.top < r.bottom && box.bottom > r.top,
    );
    if (hit) it.label.dataset.hidden = "true";
    else placed.push(box);
  }
}

/** Símbolo blanco de cada tipo dentro de la chincheta (coordenadas de un lienzo de 24×24). */
const GLYPHS: Record<PointKind, string> = {
  marker: "M12 6.5a3 3 0 1 1 0 6 3 3 0 0 1 0-6Z",
  loot: "M8 7h8v6H8z",
  danger: "M12 5.5 16.5 13h-9z",
  key: "M10 6.5a2.6 2.6 0 1 1 0 5.2 2.6 2.6 0 0 1 0-5.2Zm2.2 2h4v1.6h-1v1.2h-1.4v-1.2h-1.6z",
  quest: "m12 5.5 1.6 3.3 3.6.5-2.6 2.5.6 3.6-3.2-1.7-3.2 1.7.6-3.6-2.6-2.5 3.6-.5z",
  stash: "M12 5.5 16 9.5 12 13.5 8 9.5z",
};

export function pinIcon(
  kind: PointKind,
  name: string,
  selected: boolean,
): { icon: L.DivIcon; label: HTMLElement } {
  const root = el("div", "rt-pin");
  root.dataset.selected = String(selected);
  const svg = document.createElementNS(SVG_NS, "svg");
  svg.setAttribute("viewBox", "0 0 24 24");
  svg.setAttribute("width", "22");
  svg.setAttribute("height", "22");
  svg.setAttribute("aria-hidden", "true");
  const body = document.createElementNS(SVG_NS, "path");
  body.setAttribute(
    "d",
    "M12 1.5c-4.4 0-8 3.4-8 7.9 0 5.3 6.2 11.6 7.3 12.7.4.4 1 .4 1.4 0C13.8 21 20 14.7 20 9.4c0-4.5-3.6-7.9-8-7.9Z",
  );
  body.setAttribute("fill", kindColor(kind));
  body.setAttribute("stroke", "#111");
  body.setAttribute("stroke-width", "1.2");
  const glyph = document.createElementNS(SVG_NS, "path");
  glyph.setAttribute("d", GLYPHS[kind]);
  glyph.setAttribute("fill", "#fff");
  svg.append(body, glyph);
  const label = el("span", "rt-pin-label", name);
  root.append(svg, label);
  return { icon: L.divIcon({ html: root, className: "", iconSize: [22, 22], iconAnchor: [11, 21] }), label };
}
