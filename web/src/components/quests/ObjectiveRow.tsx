import { Check, Flag, KeyRound, MapPinned, Package } from "lucide-react";
import { t } from "../../locales";
import { cn } from "../../lib/cn";
import { iconSm } from "../../lib/icons";
import { doneCount, objectiveDone, type QStatus } from "../../lib/questLogic";
import { showQuestOnMap } from "../../lib/questMap";
import type { Progress } from "../../lib/questProtocol";
import type { Objective, Quest } from "../../lib/quests";
import { useApp } from "../../store/app";
import { isInstalled, mapName, ObjectiveCheck } from "./bits";

const chip = "inline-flex h-6 max-w-full items-center gap-1 rounded-sm px-1.5 text-2xs";
const link = `${chip} border border-line-strong text-fg-soft hover:bg-hover hover:text-fg`;

/** Items to hand over or find: they open in the Market. */
const ITEM_KINDS = new Set(["giveItem", "findItem"]);

/** A prefilled GitHub issue about a misplaced spot: public quest data only, nothing about you. */
function reportUrl(q: Quest, o: Objective, map: string): string {
  const v = useApp.getState().view?.version ?? "";
  const title = `Quest spot: ${q.name} on ${map}`;
  const body = [
    `**Quest:** ${q.name} (\`${q.id}\`)`,
    `**Objective:** ${o.text} (\`${o.id}\`)`,
    `**Map:** ${map}`,
    "",
    "**What's wrong:** (where it is in the game, a screenshot helps)",
    "",
    `Rastro ${v}, quest data from tarkov.dev.`,
  ].join("\n");
  return `https://github.com/MaxSala/rastro-tarkov/issues/new?title=${encodeURIComponent(title)}&body=${encodeURIComponent(body)}`;
}

export function ObjectiveRow({ q, o, status, p }: { q: Quest; o: Objective; status: QStatus; p: Progress }) {
  const done = objectiveDone(o, p, status);
  const spawns = o.positions.filter((s) => s.kind === "candidate").length;
  const precision =
    spawns > 1
      ? t.quests.spot.oneOf(spawns)
      : o.positions.some((s) => s.kind === "area")
        ? t.quests.spot.area
        : null;
  const spotMaps = [...new Set(o.positions.map((s) => s.map))].filter(isInstalled);
  const otherMaps = o.maps.filter((m) => m !== q.map);
  return (
    <li className="flex gap-3 border-b border-line py-3 last:border-b-0">
      <div className="pt-0.5">
        {status === "active" ? (
          <ObjectiveCheck o={o} done={doneCount(o, p)} label={o.text} />
        ) : status === "completed" ? (
          <span className="grid size-4.5 place-items-center rounded-sm bg-active text-fg-muted">
            <Check size={12} strokeWidth={3} />
          </span>
        ) : (
          <span className="mx-1.5 mt-1.5 block size-1.5 rounded-full bg-fg-muted" />
        )}
      </div>
      <div className="grid min-w-0 flex-1 gap-1.5">
        <p
          className={cn("text-sm leading-snug", done && status === "active" && "text-fg-muted line-through")}
        >
          {o.text}
        </p>
        {(o.optional || o.fir || otherMaps.length > 0 || o.keys.length > 0 || ITEM_KINDS.has(o.kind)) && (
          <div className="flex flex-wrap gap-1.5">
            {o.optional && <span className={cn(chip, "bg-active text-fg-soft")}>{t.quests.optional}</span>}
            {o.fir && <span className={cn(chip, "bg-accent-soft text-accent")}>{t.quests.fir}</span>}
            {otherMaps.length > 0 && (
              <span className={cn(chip, "bg-active text-fg-soft")}>{otherMaps.map(mapName).join(", ")}</span>
            )}
            {ITEM_KINDS.has(o.kind) &&
              o.items
                .filter((i) => i.name)
                .slice(0, 4)
                .map((i) => (
                  <button
                    key={i.id}
                    type="button"
                    title={t.keys.inMarket}
                    className={link}
                    onClick={() => useApp.getState().showInMarket(i.id)}
                  >
                    <Package size={12} className="shrink-0" />
                    <span className="truncate">{i.name}</span>
                  </button>
                ))}
            {o.keys.map((group) => {
              const first = group[0];
              if (!first) return null;
              return (
                <button
                  key={first.id}
                  type="button"
                  title={t.keys.highlight}
                  className={link}
                  onClick={() => useApp.getState().showKeyOnMap(first.id)}
                >
                  <KeyRound size={12} className="shrink-0" />
                  <span className="truncate">
                    {group
                      .map((k) => k.name)
                      .filter(Boolean)
                      .join(t.quests.orKey) || t.quests.keyNeeded}
                  </span>
                </button>
              );
            })}
          </div>
        )}
        {spotMaps.length > 0 && (
          <div className="flex flex-wrap items-center gap-1.5">
            {spotMaps.map((m) => {
              const n = o.positions.findIndex((s) => s.map === m);
              return (
                <button
                  key={m}
                  type="button"
                  className={cn(link, "text-accent hover:text-accent")}
                  onClick={() => showQuestOnMap(q.id, m, `qo-${o.id}-${n}`)}
                >
                  <MapPinned {...iconSm} className="shrink-0" />
                  {spotMaps.length === 1 && m === q.map ? t.quests.showOnMap : t.quests.showOn(mapName(m))}
                </button>
              );
            })}
            {precision && <span className={cn(chip, "bg-active text-fg-soft")}>{precision}</span>}
            <a
              href={reportUrl(q, o, spotMaps[0] ?? "")}
              target="_blank"
              rel="noreferrer"
              title={t.quests.spot.reportHint}
              className={cn(chip, "text-fg-muted hover:text-fg")}
            >
              <Flag size={11} className="shrink-0" aria-hidden />
              {t.quests.spot.report}
            </a>
          </div>
        )}
      </div>
    </li>
  );
}
