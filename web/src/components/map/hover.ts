// Hover cards for everything on the map, and the floor context used to dim what isn't on the
// floor being shown. Built from DOM nodes with textContent only, never from HTML strings.
import type L from "leaflet";
import { t } from "../../locales";
import { floorName } from "../../lib/mapText";

export type Tone =
  "pmc" | "scav" | "shared" | "transit" | "switch" | "loot" | "lock" | "point" | "peer" | "quest";

export interface Hover {
  tone: Tone;
  /** Small line above the title: what kind of thing this is. */
  kind: string;
  title: string;
  body?: string;
  /** Short facts shown as chips: distance, "available this raid"… */
  chips?: string[];
  /** What clicking does. */
  hint?: string;
  /** Name of the floor it's on, only when that isn't the floor being shown. */
  floor?: string | null;
  /** Colour of the dot, for teammates. */
  color?: string;
}

function el<K extends keyof HTMLElementTagNameMap>(tag: K, className: string, text?: string) {
  const node = document.createElement(tag);
  node.className = className;
  if (text !== undefined) node.textContent = text;
  return node;
}

export function hoverCard(h: Hover): HTMLElement {
  const root = el("div", "rt-card");
  root.dataset.tone = h.tone;
  const head = el("div", "rt-card-kind");
  const dot = el("span", "rt-card-dot");
  if (h.color) dot.style.background = h.color;
  head.append(dot, el("span", "", h.kind));
  root.append(head, el("div", "rt-card-title", h.title || h.kind));
  if (h.body) root.append(el("div", "rt-card-body", h.body));
  const chips = [...(h.chips ?? [])];
  if (h.floor || chips.length > 0) {
    const row = el("div", "rt-card-chips");
    for (const c of chips) row.append(el("span", "rt-chip", c));
    if (h.floor) {
      const f = el("span", "rt-chip", t.hover.onFloor(h.floor));
      f.dataset.kind = "floor";
      row.append(f);
    }
    root.append(row);
  }
  if (h.hint) root.append(el("div", "rt-card-hint", h.hint));
  return root;
}

/** Shared tooltip options: above the marker, fully opaque, no arrow. */
export const tipOptions = (dy: number): L.TooltipOptions => ({
  className: "rt-tip",
  direction: "top",
  offset: [0, -dy],
  opacity: 1,
});

/** Which floor is on screen, and the names of every floor of the active map. */
export interface FloorCtx {
  shown: string | null;
  names: Record<string, string>;
}

export const NO_FLOORS: FloorCtx = { shown: null, names: {} };

export const offFloor = (ctx: FloorCtx, floor: string | undefined): boolean =>
  Boolean(ctx.shown && floor && floor !== ctx.shown);

/** Translated name of the floor something is on, when it isn't the one being shown. */
export function otherFloor(ctx: FloorCtx, floor: string | undefined): string | null {
  if (!floor || !offFloor(ctx, floor)) return null;
  return floorName(ctx.names[floor] ?? floor);
}
