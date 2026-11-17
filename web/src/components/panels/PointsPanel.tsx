import { fold } from "../../lib/search";
import { MapPin, Pencil, Search, Trash2 } from "lucide-react";
import { useMemo, useState } from "react";
import { t } from "../../locales";
import { cn } from "../../lib/cn";
import { meters } from "../../lib/format";
import { iconSm } from "../../lib/icons";
import { POINT_KINDS, kindColor, kindLabel } from "../../lib/pointKinds";
import type { PointKind, PointView } from "../../lib/protocol";
import { send } from "../../lib/socket";
import { useApp } from "../../store/app";
import { IconButton } from "../ui/button";
import { PanelFrame, PanelNote } from "./PanelFrame";

function Row({ p, selected }: { p: PointView; selected: boolean }) {
  const editPoint = useApp((s) => s.editPoint);
  const [confirm, setConfirm] = useState(false);
  return (
    <li
      className={cn(
        "group flex min-w-0 items-center gap-1 border-l-2 pr-1.5 pl-2.5",
        selected ? "border-accent bg-accent-soft" : "border-transparent hover:bg-hover",
      )}
    >
      <button
        type="button"
        aria-pressed={selected}
        onClick={() => send({ type: "selectExtract", extractId: selected ? null : p.id })}
        className="flex min-w-0 flex-1 items-center gap-2.5 py-2 text-left"
        title={p.note || p.name}
      >
        <span
          aria-hidden
          className="size-2.5 shrink-0 rounded-full"
          style={{ background: kindColor(p.kind) }}
        />
        <span className="min-w-0 flex-1">
          <span className="block truncate text-sm">{p.name}</span>
          <span className="block truncate text-xs text-fg-muted">{p.note || kindLabel(p.kind)}</span>
        </span>
        {p.distanceM !== null && (
          <span className="num shrink-0 text-xs text-fg-soft">{meters(p.distanceM)}</span>
        )}
      </button>
      <div
        className={cn(
          "flex shrink-0 opacity-0 transition-opacity group-hover:opacity-100 focus-within:opacity-100",
          confirm && "opacity-100",
        )}
      >
        <IconButton
          label={`${t.points.edit}: ${p.name}`}
          onClick={() => editPoint({ mode: "edit", id: p.id })}
        >
          <Pencil {...iconSm} />
        </IconButton>
        <IconButton
          label={confirm ? t.points.deleteConfirm(p.name) : `${t.points.delete}: ${p.name}`}
          className={confirm ? "text-danger" : ""}
          onBlur={() => setConfirm(false)}
          onClick={() => (confirm ? send({ type: "deletePoint", id: p.id }) : setConfirm(true))}
        >
          <Trash2 {...iconSm} />
        </IconButton>
      </div>
    </li>
  );
}

export function PointsPanel() {
  const view = useApp((s) => s.view);
  const [query, setQuery] = useState("");
  const [kind, setKind] = useState<PointKind | "all">("all");
  const points = useMemo(() => view?.points ?? [], [view?.points]);
  const shown = useMemo(() => {
    const q = fold(query.trim());
    return points.filter(
      (p) => (kind === "all" || p.kind === kind) && (!q || fold(`${p.name} ${p.note}`).includes(q)),
    );
  }, [points, query, kind]);
  const kinds = POINT_KINDS.filter((k) => points.some((p) => p.kind === k.kind));

  return (
    <PanelFrame
      title={t.panels.points}
      actions={
        <span className="num pr-1 text-xs text-fg-muted">
          {points.length > 0 && t.points.count(points.length)}
        </span>
      }
    >
      {!view?.activeMap ? (
        <PanelNote>{t.points.noMap}</PanelNote>
      ) : points.length === 0 ? (
        <div className="grid justify-items-center gap-2 px-6 py-10 text-center">
          <MapPin size={20} strokeWidth={1.75} className="text-fg-muted" />
          <p className="text-sm text-fg-soft">{t.points.empty}</p>
          <p className="text-xs text-fg-muted">{t.points.emptyHint}</p>
        </div>
      ) : (
        <>
          <div className="grid gap-2 border-b border-line p-3">
            <div className="relative">
              <Search {...iconSm} className="pointer-events-none absolute top-2 left-2.5 text-fg-muted" />
              <label htmlFor="pt-search" className="sr-only">
                {t.points.search}
              </label>
              <input
                id="pt-search"
                type="search"
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                placeholder={t.points.search}
                className="h-8 w-full rounded-sm border border-line-strong bg-app pr-2 pl-8 text-sm placeholder:text-placeholder focus-visible:border-accent focus-visible:outline-none"
              />
            </div>
            {kinds.length > 1 && (
              <div className="flex flex-wrap gap-1" role="radiogroup" aria-label={t.points.kind}>
                {[{ kind: "all" as const, color: "" }, ...kinds].map((k) => (
                  <button
                    key={k.kind}
                    type="button"
                    role="radio"
                    aria-checked={kind === k.kind}
                    onClick={() => setKind(k.kind)}
                    className={cn(
                      "flex h-6 items-center gap-1.5 rounded-sm border px-2 text-xs",
                      kind === k.kind
                        ? "border-line-strong bg-active text-fg"
                        : "border-transparent text-fg-muted hover:text-fg",
                    )}
                  >
                    {k.color && (
                      <span aria-hidden className="size-2 rounded-full" style={{ background: k.color }} />
                    )}
                    {k.kind === "all" ? t.points.all : kindLabel(k.kind)}
                  </button>
                ))}
              </div>
            )}
          </div>
          <ul className="py-1">
            {shown.map((p) => (
              <Row key={p.id} p={p} selected={p.id === view.selectedExtract} />
            ))}
          </ul>
        </>
      )}
    </PanelFrame>
  );
}
