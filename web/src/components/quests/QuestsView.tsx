import { useEffect, useMemo, useRef, useState } from "react";
import { t } from "../../locales";
import { forFaction, mapsOf, statusOf, type QStatus } from "../../lib/questLogic";
import type { Progress } from "../../lib/questProtocol";
import { questIndex, type Quest, type QuestDb } from "../../lib/quests";
import { matches } from "../../lib/search";
import { useApp, type QuestUi } from "../../store/app";
import { QuestGate } from "./QuestGate";
import { QuestHeader } from "./QuestHeader";
import { QuestList, ResetProgress } from "./QuestList";
import { QuestDetail } from "./QuestDetail";
import { CatchUpCard, CatchUpDialog } from "./CatchUp";
import { ItemsView } from "./ItemsView";
import { TraderRail } from "./TraderRail";

const DEFAULT_UI: QuestUi = {
  status: "now",
  trader: "",
  map: "",
  kappa: false,
  query: "",
  selected: null,
};
const ORDER: Record<QStatus, number> = { active: 0, available: 1, locked: 2, failed: 3, completed: 4 };
const MAX_ROWS = 400;

export function QuestsView() {
  return <QuestGate>{(db, p) => <Quests db={db} p={p} />}</QuestGate>;
}

/** Text a search looks through: name, trader, objectives and the items they ask for. */
function haystack(q: Quest, trader: string): string[] {
  return [q.name, trader, ...q.objectives.flatMap((o) => [o.text, ...o.items.map((i) => i.name)])];
}

function Quests({ db, p }: { db: QuestDb; p: Progress }) {
  const ui = useApp((s) => s.questUi);
  const setUi = useApp((s) => s.setQuestUi);
  const u = ui ?? DEFAULT_UI;
  const idx = questIndex(db);
  const search = useRef<HTMLInputElement>(null);
  const list = useRef<HTMLUListElement>(null);

  const [catchUp, setCatchUp] = useState(false);

  const statuses = useMemo(() => new Map(db.quests.map((q) => [q.id, statusOf(q, p, idx)])), [db, p, idx]);
  const pool = useMemo(() => db.quests.filter((q) => forFaction(q, p)), [db, p]);
  const counts = useMemo(() => {
    const c: Record<QuestUi["status"], number> = { now: 0, locked: 0, completed: 0, all: 0, items: 0 };
    for (const q of pool) {
      if (u.trader && q.trader !== u.trader) continue;
      const s = statuses.get(q.id) ?? "locked";
      if (s === "active" || s === "available") c.now += 1;
      if (s === "locked" || s === "completed") c[s] += 1;
      c.all += 1;
    }
    return c;
  }, [pool, statuses, u.trader]);

  const shown = useMemo(() => {
    const status = (q: Quest) => statuses.get(q.id) ?? "locked";
    const trader = (q: Quest) => idx.traders.get(q.trader) ?? q.trader;
    return pool
      .filter((q) => {
        const st = status(q);
        if (u.status === "now" && st !== "active" && st !== "available") return false;
        if ((u.status === "locked" || u.status === "completed") && st !== u.status) return false;
        if (u.trader && q.trader !== u.trader) return false;
        const maps = mapsOf(q);
        if (u.map && !(u.map === "any" ? maps.length === 0 : maps.includes(u.map))) return false;
        if (u.kappa && !q.kappa) return false;
        return !u.query || matches(u.query, ...haystack(q, trader(q)));
      })
      .sort(
        (a, b) =>
          ORDER[status(a)] - ORDER[status(b)] ||
          a.level - b.level ||
          trader(a).localeCompare(trader(b)) ||
          a.name.localeCompare(b.name),
      );
  }, [pool, statuses, idx, u.status, u.trader, u.map, u.kappa, u.query]);

  const rows = shown.slice(0, MAX_ROWS);
  const selected = u.selected ? (idx.byId.get(u.selected) ?? null) : null;

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (
        e.key === "/" &&
        document.activeElement !== search.current &&
        !document.querySelector("dialog[open]")
      ) {
        e.preventDefault();
        search.current?.focus();
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, []);

  const move = (d: number) => {
    const i = rows.findIndex((q) => q.id === u.selected);
    const next = rows[Math.max(0, Math.min(rows.length - 1, i + d))];
    if (!next) return;
    setUi({ selected: next.id });
    list.current?.querySelector(`[data-id="${next.id}"]`)?.scrollIntoView({ block: "nearest" });
  };

  const fresh = Object.keys(p.quests).length === 0;
  return (
    <div className="grid h-full min-h-0 grid-cols-[64px_minmax(0,1fr)_400px]">
      <TraderRail db={db} idx={idx} statuses={statuses} trader={u.trader} />
      <section className="flex min-h-0 min-w-0 flex-col border-r border-line">
        <QuestHeader
          db={db}
          p={p}
          ui={u}
          counts={counts}
          searchRef={search}
          onMove={move}
          onCatchUp={() => setCatchUp(true)}
        />
        {fresh && <CatchUpCard onPick={() => setCatchUp(true)} />}
        <div className="min-h-0 flex-1 overflow-auto">
          {u.status === "items" ? (
            <ItemsView db={db} p={p} />
          ) : (
            <QuestList
              listRef={list}
              quests={rows}
              statuses={statuses}
              p={p}
              idx={idx}
              selected={u.selected}
              grouped={u.status === "now" && !u.query}
            />
          )}
        </div>
        <footer className="flex items-center justify-between gap-3 border-t border-line px-4 py-1.5 text-2xs text-fg-muted">
          <span className="num">{u.status === "items" ? "" : t.quests.shown(rows.length, shown.length)}</span>
          <ResetProgress />
        </footer>
      </section>
      <aside aria-live="polite" className="min-h-0 overflow-x-hidden overflow-y-auto bg-surface">
        <QuestDetail quest={selected} db={db} p={p} />
      </aside>
      <CatchUpDialog open={catchUp} onClose={() => setCatchUp(false)} db={db} p={p} />
    </div>
  );
}
