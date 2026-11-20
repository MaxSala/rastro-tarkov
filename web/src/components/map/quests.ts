// Quest objectives on the map, drawn for what the data really knows:
//   exact      a round amber badge whose glyph says what to do there (go, mark, plant, find…);
//   area       the zone's real outline on the ground, with a smaller badge in the middle;
//   candidate  a small dot for each place a quest item can spawn: it's in one of them.
// With a raid plan, only the planned quests are drawn and badges carry their route number.
// Each quest wears its own colour as a ring, so several quests on one map stay apart.
// Texts come from the quest data; positions arrive already projected by the backend.
import L from "leaflet";
import { t } from "../../locales";
import { meters } from "../../lib/format";
import { glyphOf, SHAPES, type Glyph } from "../../lib/questGlyphs";
import type { QuestMarker } from "../../lib/questProtocol";
import type { QuestDb } from "../../lib/quests";
import { questIndex } from "../../lib/quests";
import { send } from "../../lib/socket";
import { hoverCard, offFloor, otherFloor, tipOptions, type FloorCtx } from "./hover";
import { toLatLng } from "./markers";
import "./quests.css";

const SVG = "http://www.w3.org/2000/svg";

function glyphSvg(g: Glyph): SVGSVGElement {
  const svg = document.createElementNS(SVG, "svg");
  svg.setAttribute("viewBox", "0 0 24 24");
  svg.setAttribute("aria-hidden", "true");
  for (const s of SHAPES[g]) {
    const el = document.createElementNS(SVG, s[0]);
    if (s[0] === "path") el.setAttribute("d", s[1]);
    else {
      el.setAttribute("cx", String(s[1]));
      el.setAttribute("cy", String(s[2]));
      el.setAttribute("r", String(s[3]));
    }
    svg.append(el);
  }
  return svg;
}

interface Look {
  glyph: Glyph;
  kind: QuestMarker["kind"];
  selected: boolean;
  focus: boolean;
  off: boolean;
  color: string | null;
  number: number | null;
}

function questIcon(l: Look): L.DivIcon {
  const root = document.createElement("div");
  root.className = "rt-qm";
  root.dataset.kind = l.kind;
  root.dataset.selected = String(l.selected);
  root.dataset.focus = String(l.focus);
  root.dataset.off = String(l.off);
  if (l.color) root.style.setProperty("--rt-quest-color", l.color);
  if (l.kind !== "candidate") root.append(glyphSvg(l.glyph));
  if (l.number !== null) {
    const n = document.createElement("span");
    n.className = "rt-qm-n";
    n.textContent = String(l.number);
    root.append(n);
  }
  const size = l.kind === "candidate" ? 14 : l.kind === "area" ? 20 : 24;
  return L.divIcon({ html: root, className: "", iconSize: [size, size], iconAnchor: [size / 2, size / 2] });
}

function zone(m: QuestMarker, strong: boolean, color: string | null): L.Polygon | null {
  if (!m.outline || m.outline.length < 3) return null;
  const poly = L.polygon(
    m.outline.map(([x, y]) => toLatLng(x, y)),
    { className: strong ? "rt-qzone rt-qzone-strong" : "rt-qzone", interactive: false },
  );
  // The CSS draws it; the quest's colour reaches it as a variable.
  if (color)
    poly.on("add", () =>
      (poly.getElement() as SVGElement | undefined)?.style.setProperty("--rt-quest-color", color),
    );
  return poly;
}

export interface QuestDrawOptions {
  /** Planned quests on this map: when set, only these are drawn. */
  plan: Set<string> | null;
  /** Route number of each objective in the plan. */
  order: Map<string, number>;
  colors: Map<string, string>;
}

export function drawQuests(
  group: L.LayerGroup,
  list: QuestMarker[],
  db: QuestDb | null,
  objectivesDone: Record<string, number>,
  selected: string | null,
  floors: FloorCtx,
  focusQuest: string | null,
  opts: QuestDrawOptions,
): L.LatLng[] {
  const idx = db ? questIndex(db) : null;
  const focused: L.LatLng[] = [];
  const shown = list.filter((m) => !opts.plan || opts.plan.has(m.quest) || m.focus);
  const candidates = new Map<string, number>();
  for (const m of shown)
    if (m.kind === "candidate") candidates.set(m.objective, (candidates.get(m.objective) ?? 0) + 1);
  for (const m of shown) {
    const quest = idx?.byId.get(m.quest);
    const o = quest?.objectives.find((x) => x.id === m.objective);
    const isSel = m.id === selected;
    const off = offFloor(floors, m.floor);
    const latlng = toLatLng(m.px, m.py);
    const color = opts.colors.size > 1 ? (opts.colors.get(m.quest) ?? null) : null;
    if (m.quest === focusQuest) focused.push(latlng);
    // Areas always show where they are; other zones only while guided to or hovered.
    const pinned = m.kind === "area" || isSel || Boolean(m.focus) ? zone(m, isSel, color) : null;
    pinned?.addTo(group);
    const spawns = candidates.get(m.objective) ?? 0;
    const chips = [
      ...(m.distanceM === null ? [] : [t.hover.away(meters(m.distanceM))]),
      ...(m.kind === "candidate" ? [t.quests.spot.oneOf(spawns)] : []),
      ...(m.kind === "area" ? [t.quests.spot.area] : []),
      ...(o && o.count > 1 ? [t.quests.count(Math.min(objectivesDone[o.id] ?? 0, o.count), o.count)] : []),
      ...(o && o.keys.length > 0 ? [t.quests.keyNeeded] : []),
    ];
    const card = hoverCard({
      tone: "quest",
      kind: `${m.focus ? t.quests.hover.preview : t.quests.hover.kind} · ${quest?.name ?? ""}`,
      title: o?.text ?? quest?.name ?? t.quests.hover.kind,
      chips,
      floor: otherFloor(floors, m.floor),
      hint: isSel ? t.hover.stopGuide : t.hover.guide,
    });
    let hovered: L.Polygon | null = null;
    const look: Look = {
      glyph: glyphOf(o?.kind),
      kind: m.kind,
      selected: isSel,
      focus: Boolean(m.focus),
      off,
      color,
      number: opts.plan ? (opts.order.get(m.objective) ?? null) : null,
    };
    L.marker(latlng, {
      icon: questIcon(look),
      title: o?.text ?? "",
      keyboard: true,
      riseOnHover: true,
      zIndexOffset: off ? -200 : isSel ? 700 : m.kind === "candidate" ? 500 : 600,
    })
      .bindTooltip(card, tipOptions(12))
      .on("mouseover", () => {
        if (pinned || hovered) return;
        hovered = zone(m, false, color);
        hovered?.addTo(group);
      })
      .on("mouseout", () => {
        hovered?.remove();
        hovered = null;
      })
      .on("click", () => send({ type: "selectExtract", extractId: isSel ? null : m.id }))
      .addTo(group);
  }
  return focused;
}
