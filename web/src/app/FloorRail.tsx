import { Layers } from "lucide-react";
import { t } from "../locales";
import { cn } from "../lib/cn";
import { iconSm } from "../lib/icons";
import { floorName } from "../lib/mapText";
import { useApp } from "../store/app";

/**
 * Floor switcher on the map for layered maps, stacked like the building (highest at the top).
 * "Automatic" follows your height and position; the floor on screen is always highlighted, and
 * the map dims every other floor and whatever is on it.
 */
export function FloorRail() {
  const active = useApp((s) => s.view?.activeMap);
  const choice = useApp((s) => (active ? (s.floors[active.id] ?? "auto") : "auto"));
  const setFloor = useApp((s) => s.setFloor);
  if (!active || active.floors.length < 2) return null;
  const shown = choice === "auto" ? (active.autoFloor ?? active.baseFloor) : choice;
  const autoName = active.floors.find((f) => f.id === (active.autoFloor ?? active.baseFloor))?.name;
  const item = (selected: boolean) =>
    cn(
      "flex h-7 w-full items-center gap-2 rounded-sm px-2 text-left text-xs transition-colors",
      selected ? "bg-active text-fg" : "text-fg-muted hover:bg-hover hover:text-fg",
    );
  return (
    <div
      role="radiogroup"
      aria-label={t.floors.label}
      className="absolute top-3 left-3 z-[500] grid w-44 gap-0.5 rounded-md border border-line-strong bg-surface p-1 shadow-lg"
    >
      <div className="flex items-center gap-1.5 px-2 pt-0.5 pb-1 text-[10.5px] tracking-wide text-fg-muted uppercase">
        <Layers {...iconSm} aria-hidden />
        {t.floors.label}
      </div>
      {[...active.floors].reverse().map((f) => {
        const on = f.id === shown;
        return (
          <button
            key={f.id}
            type="button"
            role="radio"
            aria-checked={choice === f.id}
            onClick={() => setFloor(active.id, f.id)}
            className={item(choice === f.id)}
          >
            <span
              aria-hidden
              className={cn("h-3.5 w-1 shrink-0 rounded-full", on ? "bg-accent" : "bg-line-strong")}
            />
            <span className={cn("truncate", on && "text-fg")}>{floorName(f.name)}</span>
          </button>
        );
      })}
      <div className="mx-1 my-0.5 h-px bg-line" />
      <button
        type="button"
        role="radio"
        aria-checked={choice === "auto"}
        onClick={() => setFloor(active.id, "auto")}
        className={item(choice === "auto")}
        title={t.floors.auto(autoName ? floorName(autoName) : null)}
      >
        <span className="truncate">{t.floors.auto(null)}</span>
        {choice === "auto" && autoName && (
          <span className="ml-auto truncate text-[11px] text-fg-muted">{floorName(autoName)}</span>
        )}
      </button>
    </div>
  );
}
