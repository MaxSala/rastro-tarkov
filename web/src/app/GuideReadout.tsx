import { X } from "lucide-react";
import { t } from "../locales";
import { meters, relativeBearing, signedMeters } from "../lib/format";
import { iconSm } from "../lib/icons";
import { send } from "../lib/socket";
import { useApp } from "../store/app";
import { useQuests } from "../lib/quests";

/** Lectura de la guía hacia la extracción seleccionada; los números los calcula el backend. */
export function GuideReadout() {
  const view = useApp((s) => s.view);
  const questDb = useQuests((s) => s.db);
  const quest = view?.questMarkers.find((m) => m.id === view.selectedExtract);
  const objective = quest
    ? questDb?.quests.find((q) => q.id === quest.quest)?.objectives.find((o) => o.id === quest.objective)
    : undefined;
  const target =
    view?.extracts.find((e) => e.id === view.selectedExtract) ??
    view?.points.find((p) => p.id === view.selectedExtract) ??
    (quest ? { name: objective?.text ?? t.quests.hover.kind } : undefined);
  if (!view || !target) return null;
  const g = view.guide;
  return (
    <section
      aria-label={`${t.extracts.guideTo} ${target.name}`}
      className="absolute bottom-3 left-3 z-[500] w-72 rounded-md border border-line-strong bg-surface"
    >
      <header className="flex items-center justify-between gap-2 border-b border-line px-3 py-2">
        <p className="truncate text-sm">
          <span className="text-fg-muted">{t.extracts.guideTo} </span>
          {target.name}
        </p>
        <button
          type="button"
          aria-label={t.extracts.clearGuide}
          onClick={() => send({ type: "selectExtract", extractId: null })}
          className="text-fg-muted hover:text-fg"
        >
          <X {...iconSm} />
        </button>
      </header>
      {g ? (
        <dl className="grid grid-cols-3 divide-x divide-line">
          {[
            [t.extracts.distance, meters(g.distanceM)],
            [t.extracts.bearing, relativeBearing(g.relativeDeg)],
            [t.extracts.height, signedMeters(g.heightDiffM)],
          ].map(([k, v]) => (
            <div key={k} className="grid gap-0.5 px-3 py-2">
              <dt className="text-2xs text-fg-muted">{k}</dt>
              <dd className="num text-sm">{v}</dd>
            </div>
          ))}
        </dl>
      ) : (
        <p className="px-3 py-2 text-xs text-fg-muted">{t.extracts.noPosition}</p>
      )}
    </section>
  );
}
