import { t } from "../../../locales";
import { cn } from "../../../lib/cn";
import { meters } from "../../../lib/format";
import { floorName } from "../../../lib/mapText";
import type { QuestMarker } from "../../../lib/questProtocol";
import type { Objective, Quest } from "../../../lib/quests";
import { send } from "../../../lib/socket";
import { useApp } from "../../../store/app";
import { ObjectiveCheck } from "../../quests/bits";
import { QuestGlyph } from "../../quests/QuestGlyph";

export interface Entry {
  quest: Quest;
  o: Objective;
  /** Nearest spot of this objective on the map, when it has one. */
  spot: QuestMarker | null;
}

/** One open objective in the map panel: tick it off, or click it to be guided there. */
export function ObjectiveLine({
  e,
  selected,
  done,
  color,
  number,
}: {
  e: Entry;
  selected: boolean;
  done: number;
  color: string | null;
  number: number | null;
}) {
  const floors = useApp((s) => s.view?.activeMap?.floors);
  const base = useApp((s) => s.view?.activeMap?.baseFloor);
  // The floor is worth saying when it isn't the ground floor.
  const floor = e.spot?.floor && e.spot.floor !== base ? floors?.find((f) => f.id === e.spot?.floor) : null;
  const meta = [
    e.quest.name,
    e.spot?.distanceM != null ? meters(e.spot.distanceM) : null,
    floor ? floorName(floor.name) : null,
    e.spot?.kind === "area" ? t.quests.spot.area : null,
  ];
  return (
    <li
      className={cn(
        "flex items-start gap-2.5 border-l-2 py-2 pr-3 pl-2.5",
        selected ? "border-accent bg-accent-soft" : "border-transparent hover:bg-hover",
      )}
    >
      <div className="pt-0.5">
        <ObjectiveCheck o={e.o} done={done} label={e.o.text} />
      </div>
      <button
        type="button"
        aria-pressed={e.spot ? selected : undefined}
        onClick={() =>
          e.spot
            ? send({ type: "selectExtract", extractId: selected ? null : e.spot.id })
            : useApp.getState().openQuest(e.quest.id)
        }
        className="min-w-0 flex-1 text-left"
        title={e.spot ? t.quests.panel.guide : t.quests.panel.openQuest}
      >
        <span className="line-clamp-2 text-sm leading-snug">{e.o.text}</span>
        <span className="mt-0.5 flex min-w-0 items-center gap-1.5 text-xs text-fg-muted">
          {color && (
            <span aria-hidden className="size-2 shrink-0 rounded-full" style={{ background: color }} />
          )}
          <span className="truncate">{meta.filter(Boolean).join(" · ")}</span>
        </span>
      </button>
      {e.spot && (
        <span className="relative mt-0.5 shrink-0">
          <QuestGlyph kind={e.o.kind} />
          {number !== null && (
            <span className="num absolute -top-1.5 -right-1.5 grid h-3.5 min-w-3.5 place-items-center rounded-full bg-fg px-0.5 text-[9px] leading-none text-app">
              {number}
            </span>
          )}
        </span>
      )}
    </li>
  );
}
