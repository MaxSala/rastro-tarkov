import { Minus, Plus, Search } from "lucide-react";
import { useMemo, useState, type RefObject } from "react";
import { t } from "../../locales";
import { cn } from "../../lib/cn";
import { iconSm } from "../../lib/icons";
import { byMap, mapsOf } from "../../lib/questLogic";
import type { Faction, Progress } from "../../lib/questProtocol";
import type { QuestDb } from "../../lib/quests";
import { send } from "../../lib/socket";
import { useApp, type QuestUi } from "../../store/app";
import { mapName } from "./bits";
import { QuestDataStatus } from "./QuestGate";
import { QuestMeta } from "./QuestMeta";

const TABS: QuestUi["status"][] = ["now", "locked", "completed", "all", "items"];
const select =
  "h-7 max-w-44 rounded-sm border border-line-strong bg-app px-2 text-xs text-fg focus-visible:border-accent focus-visible:outline-none";

/** Your level and faction: they decide what's available. Saved per game mode. */
function Player({ p }: { p: Progress }) {
  const [draft, setDraft] = useState<string | null>(null);
  const level = (n: number) => send({ type: "questsLevel", level: Math.max(1, Math.min(79, n)) });
  const step = "grid size-7 place-items-center text-fg-soft hover:bg-hover hover:text-fg";
  return (
    <div className="flex shrink-0 items-center gap-2">
      <div
        title={t.quests.player.levelHint}
        className="flex h-8 items-center overflow-hidden rounded-sm border border-line-strong bg-app"
      >
        <span className="pl-2.5 text-xs text-fg-muted">{t.quests.player.level}</span>
        <button
          type="button"
          aria-label={t.quests.player.less}
          className={step}
          onClick={() => level(p.level - 1)}
        >
          <Minus size={12} />
        </button>
        <input
          aria-label={t.quests.player.level}
          inputMode="numeric"
          value={draft ?? String(p.level)}
          onChange={(e) => setDraft(e.target.value.replace(/\D/g, "").slice(0, 2))}
          onBlur={() => {
            if (draft) level(Number(draft));
            setDraft(null);
          }}
          onKeyDown={(e) => e.key === "Enter" && e.currentTarget.blur()}
          className="num w-7 bg-transparent text-center text-sm focus-visible:outline-none"
        />
        <button
          type="button"
          aria-label={t.quests.player.more}
          className={step}
          onClick={() => level(p.level + 1)}
        >
          <Plus size={12} />
        </button>
      </div>
      <label className="sr-only" htmlFor="quest-faction">
        {t.quests.player.faction}
      </label>
      <select
        id="quest-faction"
        title={t.quests.player.faction}
        value={p.faction}
        onChange={(e) => send({ type: "questsFaction", faction: e.target.value as Faction })}
        className={cn(select, "h-8")}
      >
        {(["any", "usec", "bear"] as const).map((f) => (
          <option key={f} value={f}>
            {t.quests.player.factions[f]}
          </option>
        ))}
      </select>
    </div>
  );
}

export function QuestHeader({
  db,
  p,
  ui,
  counts,
  searchRef,
  onMove,
  onCatchUp,
}: {
  db: QuestDb;
  p: Progress;
  ui: QuestUi;
  counts: Record<QuestUi["status"], number>;
  searchRef: RefObject<HTMLInputElement | null>;
  onMove: (d: number) => void;
  onCatchUp: () => void;
}) {
  const setUi = useApp((s) => s.setQuestUi);
  const maps = useMemo(() => {
    const set = new Set(db.quests.flatMap(mapsOf));
    return [...set].map((id) => ({ id, name: mapName(id) })).sort((a, b) => a.name.localeCompare(b.name));
  }, [db]);
  const where = useMemo(() => [...byMap(db, p)].sort((a, b) => b[1] - a[1]), [db, p]);
  const best = where.find(([m]) => m)?.[0] ?? null;

  return (
    <div className="grid gap-3 border-b border-line px-4 pt-4 pb-3">
      <div className="flex items-center gap-3">
        <div className="relative min-w-0 flex-1">
          <Search
            {...iconSm}
            className="pointer-events-none absolute top-1/2 left-3 -translate-y-1/2 text-fg-muted"
          />
          <label htmlFor="quest-search" className="sr-only">
            {t.quests.search}
          </label>
          <input
            ref={searchRef}
            id="quest-search"
            type="search"
            autoComplete="off"
            spellCheck={false}
            value={ui.query}
            placeholder={t.quests.search}
            onChange={(e) => setUi({ query: e.target.value })}
            onKeyDown={(e) => {
              if (e.key === "/" && !e.currentTarget.value) e.preventDefault();
              if (e.key === "ArrowDown" || e.key === "ArrowUp") {
                e.preventDefault();
                onMove(e.key === "ArrowDown" ? 1 : -1);
              }
            }}
            className="h-10 w-full rounded-md border border-line-strong bg-app pr-12 pl-9 text-base placeholder:text-placeholder focus-visible:border-accent focus-visible:outline-none"
          />
          <kbd className="num pointer-events-none absolute top-1/2 right-2.5 -translate-y-1/2 rounded-sm border border-line-strong px-1.5 text-2xs text-fg-muted">
            /
          </kbd>
        </div>
        <Player p={p} />
        <QuestDataStatus />
      </div>
      <div className="flex flex-wrap items-center gap-x-3 gap-y-2">
        <div role="radiogroup" aria-label={t.quests.filters.label} className="flex flex-wrap gap-1">
          {TABS.map((s) => (
            <button
              key={s}
              type="button"
              role="radio"
              aria-checked={ui.status === s}
              onClick={() => setUi({ status: s })}
              className={cn(
                "flex h-7 items-center gap-1.5 rounded-sm border px-2.5 text-xs transition-colors",
                ui.status === s
                  ? "border-line-strong bg-active text-fg"
                  : "border-transparent text-fg-muted hover:text-fg",
              )}
            >
              {t.quests.filters[s]}
              {s !== "items" && <span className="num text-2xs text-fg-muted">{counts[s]}</span>}
            </button>
          ))}
        </div>
        {ui.status !== "items" && (
          <div className="ml-auto flex items-center gap-1.5">
            <select
              aria-label={t.quests.filters.map}
              value={ui.map}
              onChange={(e) => setUi({ map: e.target.value })}
              className={select}
            >
              <option value="">{t.quests.filters.allMaps}</option>
              <option value="any">{t.quests.filters.anyMap}</option>
              {maps.map((m) => (
                <option key={m.id} value={m.id}>
                  {m.name}
                </option>
              ))}
            </select>
            <button
              type="button"
              aria-pressed={ui.kappa}
              title={t.quests.filters.kappaHint}
              onClick={() => setUi({ kappa: !ui.kappa })}
              className={cn(
                "h-7 rounded-sm border px-2.5 text-xs transition-colors",
                ui.kappa
                  ? "border-line-strong bg-active text-fg"
                  : "border-line-strong text-fg-muted hover:text-fg",
              )}
            >
              {t.quests.filters.kappa}
            </button>
          </div>
        )}
      </div>
      <QuestMeta db={db} p={p} onCatchUp={onCatchUp} />
      {ui.status === "now" && where.length > 0 && (
        <div className="flex flex-wrap items-center gap-1.5">
          <span className="mr-1 text-xs text-fg-muted">{t.quests.whereTo}</span>
          {where.map(([m, n]) => {
            const value = m || "any";
            const on = ui.map === value;
            return (
              <button
                key={value}
                type="button"
                aria-pressed={on}
                title={m === best ? t.quests.bestMap : undefined}
                onClick={() => setUi({ map: on ? "" : value })}
                className={cn(
                  "flex h-6 items-center gap-1.5 rounded-full border px-2.5 text-xs transition-colors",
                  on
                    ? "border-accent bg-accent-soft text-fg"
                    : m === best
                      ? "border-accent/50 text-fg"
                      : "border-line-strong text-fg-soft hover:text-fg",
                )}
              >
                {m === best && <span aria-hidden className="size-1.5 rounded-full bg-accent" />}
                {m ? mapName(m) : t.quests.filters.anyMap}
                <span className="num text-2xs text-accent">{n}</span>
              </button>
            );
          })}
        </div>
      )}
    </div>
  );
}
