import { t } from "../locales";
import type { PointKind } from "./protocol";

/** Tipos de punto: etiqueta y color (fijos, legibles sobre cualquier mapa). */
export const POINT_KINDS: { kind: PointKind; color: string }[] = [
  { kind: "marker", color: "#4fa3f7" },
  { kind: "loot", color: "#f2c94c" },
  { kind: "danger", color: "#f76b6b" },
  { kind: "key", color: "#2dd4bf" },
  { kind: "quest", color: "#c084fc" },
  { kind: "stash", color: "#3dd68c" },
];

export const kindColor = (k: PointKind) => POINT_KINDS.find((p) => p.kind === k)?.color ?? "#4fa3f7";
export const kindLabel = (k: PointKind) => t.points.kinds[k];
