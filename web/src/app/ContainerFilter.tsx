import { SlidersHorizontal } from "lucide-react";
import { useEffect, useMemo, useRef, useState } from "react";
import { t } from "../locales";
import { cn } from "../lib/cn";
import { iconUrl } from "../lib/iconCache";
import { iconSm } from "../lib/icons";
import { useApp } from "../store/app";

/** Filtro por tipo de contenedor (caja de armas, chaqueta, caja fuerte…), con su icono y recuento. */
export function ContainerFilter() {
  const markers = useApp((s) => s.markers);
  const activeId = useApp((s) => s.view?.activeMap?.id);
  const hidden = useApp((s) => s.hiddenGroups);
  const setHidden = useApp((s) => s.setHiddenGroups);
  useApp((s) => s.iconsVersion);
  const [open, setOpen] = useState(false);
  const ref = useRef<HTMLDivElement>(null);

  const groups = useMemo(() => {
    const map = new Map<string, { name: string; icon: string | null; count: number }>();
    if (markers?.mapId !== activeId) return [];
    for (const m of markers?.list ?? []) {
      if (m.kind !== "loot") continue;
      const key = m.group ?? "otros";
      const g = map.get(key) ?? { name: m.group ? m.name : t.filter.other, icon: m.icon, count: 0 };
      g.count += 1;
      map.set(key, g);
    }
    return [...map.entries()].sort((a, b) => b[1].count - a[1].count);
  }, [markers, activeId]);

  useEffect(() => {
    if (!open) return;
    const close = (e: PointerEvent) => !ref.current?.contains(e.target as Node) && setOpen(false);
    const esc = (e: KeyboardEvent) => e.key === "Escape" && setOpen(false);
    window.addEventListener("pointerdown", close);
    window.addEventListener("keydown", esc);
    return () => {
      window.removeEventListener("pointerdown", close);
      window.removeEventListener("keydown", esc);
    };
  }, [open]);

  if (groups.length === 0) return null;
  const hiddenSet = new Set(hidden);
  const toggle = (k: string) => setHidden(hiddenSet.has(k) ? hidden.filter((h) => h !== k) : [...hidden, k]);
  const activeFilter = groups.some(([k]) => hiddenSet.has(k));

  return (
    <div ref={ref} className="relative">
      <button
        type="button"
        aria-expanded={open}
        aria-label={t.filter.button}
        title={t.filter.button}
        onClick={() => setOpen(!open)}
        className={cn(
          "flex h-7 items-center rounded-sm px-2 text-fg-muted hover:text-fg",
          (open || activeFilter) && "bg-active text-fg",
        )}
      >
        <SlidersHorizontal {...iconSm} />
      </button>
      {open && (
        <div
          role="dialog"
          aria-label={t.filter.title}
          className="absolute top-9 right-0 z-[600] w-64 rounded-md border border-line-strong bg-surface p-1"
        >
          <div className="flex items-center justify-between px-2 py-1.5">
            <span className="text-xs font-medium text-fg-muted">{t.filter.title}</span>
            <span className="flex gap-2 text-xs">
              <button
                type="button"
                className="text-fg-soft hover:text-fg"
                onClick={() => setHidden(hidden.filter((h) => !groups.some(([k]) => k === h)))}
              >
                {t.filter.all}
              </button>
              <button
                type="button"
                className="text-fg-soft hover:text-fg"
                onClick={() => setHidden([...new Set([...hidden, ...groups.map(([k]) => k)])])}
              >
                {t.filter.none}
              </button>
            </span>
          </div>
          <ul className="max-h-80 overflow-x-hidden overflow-y-auto">
            {groups.map(([key, g]) => {
              const url = iconUrl(g.icon);
              return (
                <li key={key}>
                  <label className="flex h-8 cursor-pointer items-center gap-2 rounded-sm px-2 text-sm hover:bg-hover">
                    <input
                      type="checkbox"
                      checked={!hiddenSet.has(key)}
                      onChange={() => toggle(key)}
                      className="size-3.5 accent-[var(--accent)]"
                    />
                    {url ? (
                      <img src={url} alt="" className="size-5 shrink-0" />
                    ) : (
                      <span aria-hidden className="size-5 shrink-0" />
                    )}
                    <span className="min-w-0 flex-1 truncate">{g.name}</span>
                    <span className="num text-xs text-fg-muted">{g.count}</span>
                  </label>
                </li>
              );
            })}
          </ul>
        </div>
      )}
    </div>
  );
}
