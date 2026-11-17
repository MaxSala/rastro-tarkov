import { t } from "../locales";
import { useApp } from "../store/app";

/** Selector de piso para mapas SVG por capas; «Automático» sigue tu altura y posición. */
export function FloorSelect() {
  const active = useApp((s) => s.view?.activeMap);
  const choice = useApp((s) => (active ? (s.floors[active.id] ?? "auto") : "auto"));
  const setFloor = useApp((s) => s.setFloor);
  if (!active || active.floors.length < 2) return null;
  const autoName = active.floors.find((f) => f.id === active.autoFloor)?.name ?? null;
  return (
    <>
      <label htmlFor="floor-select" className="sr-only">
        {t.floors.label}
      </label>
      <select
        id="floor-select"
        value={choice}
        onChange={(e) => setFloor(active.id, e.target.value)}
        className="h-7 max-w-52 truncate rounded-sm border border-line-strong bg-app px-2 text-sm text-fg"
      >
        <option value="auto">{t.floors.auto(autoName)}</option>
        {active.floors.map((f) => (
          <option key={f.id} value={f.id}>
            {f.name}
          </option>
        ))}
      </select>
    </>
  );
}
