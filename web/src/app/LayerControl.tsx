import { t } from "../locales";
import { cn } from "../lib/cn";
import { useApp, type LayerId } from "../store/app";
import { ContainerFilter } from "./ContainerFilter";
import { X } from "lucide-react";
import { iconSm } from "../lib/icons";
import { useCatalog } from "../lib/catalog";
import { keyName } from "../components/map/locks";
import { stopQuestPreview } from "../lib/questMap";
import { useQuests } from "../lib/quests";

const order: LayerId[] = ["extracts", "quests", "bosses", "transit", "switch", "lock", "loot"];

/** Interruptores de capas del mapa; solo aparecen las que tienen datos. */
export function LayerControl() {
  const layers = useApp((s) => s.layers);
  const toggle = useApp((s) => s.toggleLayer);
  const markers = useApp((s) => s.markers);
  const focusKey = useApp((s) => s.focusKey);
  const setFocusKey = useApp((s) => s.setFocusKey);
  const catalog = useCatalog((s) => s.catalog);
  const activeId = useApp((s) => s.view?.activeMap?.id);
  const hasExtracts = useApp((s) => (s.view?.extracts.length ?? 0) > 0);
  const hasQuests = useApp((s) => (s.view?.questMarkers.length ?? 0) > 0);
  const hasBosses = useApp((s) => (s.view?.bosses.markers.length ?? 0) > 0);
  const questFocus = useApp((s) => s.questFocus);
  const focusName = useQuests((s) => s.db?.quests.find((q) => q.id === questFocus)?.name);
  const kinds = new Set<string>(markers?.mapId === activeId ? markers?.list.map((m) => m.kind) : []);
  const available = order.filter((l) =>
    l === "extracts" ? hasExtracts : l === "quests" ? hasQuests : l === "bosses" ? hasBosses : kinds.has(l),
  );
  if (available.length === 0) return null;
  return (
    <div
      role="group"
      aria-label={t.layers.title}
      className="absolute top-3 right-3 z-[500] flex gap-0.5 rounded-md border border-line-strong bg-surface p-0.5"
    >
      {available.map((l) => (
        <button
          key={l}
          type="button"
          aria-pressed={layers[l]}
          onClick={() => toggle(l)}
          className={cn(
            "h-7 rounded-sm px-2.5 text-xs transition-colors",
            layers[l] ? "bg-active text-fg" : "text-fg-muted hover:text-fg",
          )}
        >
          {t.layers[l]}
        </button>
      ))}
      {questFocus && focusName && (
        <>
          <div className="mx-0.5 w-px self-stretch bg-line" />
          <button
            type="button"
            onClick={stopQuestPreview}
            title={t.quests.clearFocus}
            className="flex h-7 max-w-56 items-center gap-1.5 rounded-sm bg-accent-soft px-2 text-xs text-fg hover:bg-hover"
          >
            <span className="truncate">{t.quests.focus(focusName)}</span>
            <X {...iconSm} aria-hidden />
          </button>
        </>
      )}
      {layers.lock && focusKey && (
        <>
          <div className="mx-0.5 w-px self-stretch bg-line" />
          <button
            type="button"
            onClick={() => setFocusKey(null)}
            title={t.keys.clearFocus}
            className="flex h-7 max-w-56 items-center gap-1.5 rounded-sm bg-accent-soft px-2 text-xs text-fg hover:bg-hover"
          >
            <span className="truncate">{t.keys.focused(keyName(focusKey, "", catalog))}</span>
            <X {...iconSm} aria-hidden />
          </button>
        </>
      )}
      {layers.loot && kinds.has("loot") && (
        <>
          <div className="mx-0.5 w-px self-stretch bg-line" />
          <ContainerFilter />
        </>
      )}
    </div>
  );
}
