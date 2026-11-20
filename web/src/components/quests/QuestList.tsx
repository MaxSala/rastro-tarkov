import { useEffect, useState, type RefObject } from "react";
import { t } from "../../locales";
import { cn } from "../../lib/cn";
import { mapsOf, progressOf, type QStatus } from "../../lib/questLogic";
import type { Progress } from "../../lib/questProtocol";
import type { Quest, QuestIndex } from "../../lib/quests";
import { send } from "../../lib/socket";
import { useApp } from "../../store/app";
import { Badges, mapName } from "./bits";
import { TraderAvatar } from "./media";

function Row({
  q,
  status,
  p,
  trader,
  portrait,
  selected,
}: {
  q: Quest;
  status: QStatus;
  p: Progress;
  trader: string;
  portrait: string | null;
  selected: boolean;
}) {
  const setUi = useApp((s) => s.setQuestUi);
  const maps = mapsOf(q);
  const [done, total] = progressOf(q, p, status);
  const meta = [trader, maps.length > 0 ? maps.map(mapName).join(", ") : t.quests.filters.anyMap];
  return (
    <li
      data-id={q.id}
      role="option"
      aria-selected={selected}
      onClick={() => setUi({ selected: q.id })}
      className={cn(
        "flex h-15 cursor-default items-center gap-3 border-b border-l-2 border-b-line pr-4 pl-3.5",
        selected ? "border-l-accent bg-accent-soft" : "border-l-transparent hover:bg-hover",
      )}
    >
      <TraderAvatar image={portrait} name={trader} status={status} />
      <span className="min-w-0 flex-1">
        <span className={cn("block truncate text-sm", status === "completed" && "text-fg-muted")}>
          {q.name}
        </span>
        <span className="block truncate text-xs text-fg-muted">{meta.join(" · ")}</span>
      </span>
      <Badges kappa={q.kappa} lightkeeper={q.lightkeeper} />
      {status === "active" && total > 0 && done === total ? (
        <span className="shrink-0 rounded-sm bg-accent-soft px-1.5 py-0.5 text-2xs text-accent">
          {t.quests.list.ready}
        </span>
      ) : status === "active" && total > 0 ? (
        <span className="num w-10 shrink-0 text-right text-xs text-fg-soft">
          {t.quests.count(done, total)}
        </span>
      ) : (
        <span className="num w-10 shrink-0 text-right text-xs text-fg-muted">
          {q.level > 0 ? t.quests.levelShort(q.level) : ""}
        </span>
      )}
    </li>
  );
}

export function QuestList({
  listRef,
  quests,
  statuses,
  p,
  idx,
  selected,
  grouped,
}: {
  listRef: RefObject<HTMLUListElement | null>;
  quests: Quest[];
  statuses: Map<string, QStatus>;
  p: Progress;
  idx: QuestIndex;
  selected: string | null;
  /** "Now": what you're on, then what you can start. */
  grouped: boolean;
}) {
  if (quests.length === 0)
    return (
      <p className="px-4 py-10 text-center text-sm text-fg-muted">
        {grouped ? t.quests.noActive : t.quests.noResults}
      </p>
    );
  const row = (q: Quest) => (
    <Row
      key={q.id}
      q={q}
      status={statuses.get(q.id) ?? "locked"}
      p={p}
      trader={idx.traders.get(q.trader) ?? q.trader}
      portrait={idx.portraits.get(q.trader) ?? null}
      selected={q.id === selected}
    />
  );
  if (!grouped)
    return (
      <ul ref={listRef} role="listbox" aria-label={t.quests.title}>
        {quests.map(row)}
      </ul>
    );
  const active = quests.filter((q) => statuses.get(q.id) === "active");
  const available = quests.filter((q) => statuses.get(q.id) !== "active");
  const heading =
    "border-b border-line bg-surface px-4 pt-3 pb-1.5 text-2xs font-medium tracking-wider text-fg-muted uppercase";
  return (
    <ul ref={listRef} role="listbox" aria-label={t.quests.title}>
      {active.length > 0 && (
        <li role="presentation" className={heading}>
          {t.quests.list.active} <span className="num">{active.length}</span>
        </li>
      )}
      {active.map(row)}
      {available.length > 0 && (
        <li role="presentation" className={heading}>
          {t.quests.list.available} <span className="num">{available.length}</span>
        </li>
      )}
      {available.map(row)}
    </ul>
  );
}

/** Two clicks: the first one asks, the second one clears this mode's progress. */
export function ResetProgress() {
  const [armed, setArmed] = useState(false);
  useEffect(() => {
    if (!armed) return;
    const id = window.setTimeout(() => setArmed(false), 4000);
    return () => window.clearTimeout(id);
  }, [armed]);
  return (
    <button
      type="button"
      onClick={() => {
        if (!armed) return setArmed(true);
        setArmed(false);
        send({ type: "questsReset" });
        useApp.getState().notify("info", t.quests.resetDone);
      }}
      className={cn("text-2xs transition-colors", armed ? "text-danger" : "text-fg-muted hover:text-fg")}
    >
      {armed ? t.quests.resetConfirm : t.quests.reset}
    </button>
  );
}
