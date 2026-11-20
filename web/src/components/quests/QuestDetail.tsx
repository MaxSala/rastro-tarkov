import { ExternalLink, ListChecks, MapPinned } from "lucide-react";
import type { ReactNode } from "react";
import { t } from "../../locales";
import { cn } from "../../lib/cn";
import { iconSm } from "../../lib/icons";
import { blockers, chainBefore, mapsOf, progressOf, statusOf, type QStatus } from "../../lib/questLogic";
import { showQuestOnMap } from "../../lib/questMap";
import type { Progress } from "../../lib/questProtocol";
import { questIndex, type Quest, type QuestDb, type QuestIndex } from "../../lib/quests";
import { send } from "../../lib/socket";
import { useApp } from "../../store/app";
import { Button } from "../ui/button";
import { Badges, isInstalled, mapName, StatusDot } from "./bits";
import { QuestPicture, TraderAvatar } from "./media";
import { ObjectiveRow } from "./ObjectiveRow";
import { QuestRewards } from "./QuestRewards";

const set = (q: Quest, status: "active" | "completed" | null) =>
  send({ type: "questSet", questId: q.id, status });

function QuestLink({ q, p, idx }: { q: Quest; p: Progress; idx: QuestIndex }) {
  return (
    <button
      type="button"
      onClick={() => useApp.getState().setQuestUi({ selected: q.id })}
      className="flex w-full items-center gap-2.5 rounded-sm px-2 py-1.5 text-left text-sm hover:bg-hover"
    >
      <StatusDot status={statusOf(q, p, idx)} />
      <span className="min-w-0 flex-1 truncate">{q.name}</span>
      <span className="num shrink-0 text-2xs text-fg-muted">{t.quests.levelShort(q.level)}</span>
    </button>
  );
}

function Section({ title, children }: { title: string; children: ReactNode }) {
  return (
    <section className="grid gap-1.5">
      <h3 className="text-xs font-medium text-fg-muted">{title}</h3>
      {children}
    </section>
  );
}

/** State, what to do next and why a quest is locked. */
function StatusCard({ q, status, p, idx }: { q: Quest; status: QStatus; p: Progress; idx: QuestIndex }) {
  const [done, total] = progressOf(q, p, status);
  const b = blockers(q, p, idx);
  const before = status === "completed" ? [] : chainBefore(q, p, idx);
  return (
    <div className="grid gap-3 rounded-md border border-line-strong bg-app p-3">
      <div className="flex items-center gap-2.5">
        <StatusDot status={status} />
        <span className="flex-1 text-sm font-medium">{t.quests.state[status]}</span>
        {status === "active" && total > 0 && (
          <span className="num text-xs text-fg-soft">{t.quests.count(done, total)}</span>
        )}
      </div>
      {status === "locked" && (
        <ul className="grid gap-1 text-xs text-fg-soft">
          {b.level !== null && <li>{t.quests.needsLevel(b.level)}</li>}
          {b.quests.length > 0 && (
            <li className="grid gap-1">
              <span>{t.quests.requires}</span>
              <div className="grid">
                {b.quests.map((r) => (
                  <QuestLink key={r.id} q={r} p={p} idx={idx} />
                ))}
              </div>
            </li>
          )}
          {q.loyalty.map(([tr, lv]) => (
            <li key={tr} className="text-fg-muted">
              {t.quests.loyalty(tr, lv)}
            </li>
          ))}
        </ul>
      )}
      {status === "active" && total > 0 && done === total && (
        <p className="rounded-sm bg-accent-soft px-2.5 py-2 text-xs text-fg">{t.quests.ready}</p>
      )}
      <div className="flex flex-wrap gap-2">
        {status === "available" && (
          <Button variant="primary" onClick={() => set(q, "active")}>
            {t.quests.start}
          </Button>
        )}
        {status === "locked" && <Button onClick={() => set(q, "active")}>{t.quests.startAnyway}</Button>}
        {status === "active" && (
          <Button variant="primary" onClick={() => set(q, "completed")}>
            {t.quests.complete}
          </Button>
        )}
        {(status === "available" || status === "locked") && (
          <Button variant="ghost" onClick={() => set(q, "completed")}>
            {t.quests.alreadyDone}
          </Button>
        )}
        {status === "active" && (
          <Button variant="ghost" onClick={() => set(q, null)}>
            {t.quests.stop}
          </Button>
        )}
        {(status === "completed" || status === "failed") && (
          <Button onClick={() => set(q, null)}>{t.quests.undo}</Button>
        )}
      </div>
      {before.length > 0 && (
        <div className="grid gap-1 border-t border-line pt-2.5">
          <button
            type="button"
            onClick={() => send({ type: "questsCompleteBefore", questId: q.id })}
            className="flex items-center gap-1.5 justify-self-start text-xs text-accent hover:underline"
          >
            <ListChecks {...iconSm} />
            {t.quests.completeBefore(before.length)}
          </button>
          <p className="text-2xs text-fg-muted">{t.quests.completeBeforeHint}</p>
        </div>
      )}
    </div>
  );
}

export function QuestDetail({ quest, db, p }: { quest: Quest | null; db: QuestDb; p: Progress }) {
  if (!quest)
    return (
      <div className="grid h-full place-items-center p-6 text-center">
        <div className="grid justify-items-center gap-2">
          <ListChecks size={20} strokeWidth={1.5} className="text-fg-muted" />
          <p className="max-w-56 text-sm text-fg-muted">{t.quests.pick}</p>
        </div>
      </div>
    );
  const idx = questIndex(db);
  const status = statusOf(quest, p, idx);
  const maps = mapsOf(quest);
  const unlocks = idx.unlocks.get(quest.id) ?? [];
  const requires = quest.requires.map((r) => idx.byId.get(r.quest)).filter((x): x is Quest => Boolean(x));
  const firstMap = maps.find(isInstalled);
  const linkCls =
    "inline-flex h-7 items-center gap-1.5 rounded-sm border border-line-strong bg-raised px-2 text-xs font-medium hover:bg-hover";
  const trader = idx.traders.get(quest.trader) ?? quest.trader;
  return (
    <div className="grid content-start gap-5 p-4">
      <header className="grid gap-3">
        <QuestPicture key={quest.id} id={quest.id} has={quest.image} />
        <div className="flex items-start gap-3">
          <TraderAvatar image={idx.portraits.get(quest.trader) ?? null} name={trader} size={44} />
          <div className="grid min-w-0 gap-0.5">
            <p className="text-2xs tracking-wider text-fg-muted uppercase">{trader}</p>
            <h2 className="text-lg leading-snug font-medium break-words">{quest.name}</h2>
            <p className="flex flex-wrap items-center gap-x-2 gap-y-1 text-xs text-fg-muted">
              <span>{maps.length > 0 ? maps.map(mapName).join(", ") : t.quests.filters.anyMap}</span>
              <span aria-hidden>·</span>
              <span className="num">{t.quests.levelShort(quest.level)}</span>
              <Badges kappa={quest.kappa} lightkeeper={quest.lightkeeper} />
            </p>
          </div>
        </div>
      </header>
      <StatusCard q={quest} status={status} p={p} idx={idx} />
      <Section title={t.quests.objectives}>
        {status !== "active" && status !== "completed" && (
          <p className="text-2xs text-fg-muted">{t.quests.startToTrack}</p>
        )}
        <ul className={cn("grid")}>
          {quest.objectives.map((o) => (
            <ObjectiveRow key={o.id} q={quest} o={o} status={status} p={p} />
          ))}
        </ul>
      </Section>
      <Section title={t.quests.rewards.title}>
        <QuestRewards r={quest.rewards} idx={idx} />
      </Section>
      {requires.length > 0 && status !== "locked" && (
        <Section title={t.quests.requires}>
          <div className="-mx-2 grid">
            {requires.map((r) => (
              <QuestLink key={r.id} q={r} p={p} idx={idx} />
            ))}
          </div>
        </Section>
      )}
      {unlocks.length > 0 && (
        <Section title={t.quests.unlocks}>
          <div className="-mx-2 grid">
            {unlocks.map((r) => (
              <QuestLink key={r.id} q={r} p={p} idx={idx} />
            ))}
          </div>
        </Section>
      )}
      <div className="flex flex-wrap gap-2">
        {firstMap && quest.objectives.filter((o) => o.positions.length > 0).length > 1 && (
          <Button size="sm" onClick={() => showQuestOnMap(quest.id, firstMap)}>
            <MapPinned {...iconSm} />
            {t.quests.showOnMap}
          </Button>
        )}
        {quest.slug && (
          <a
            href={`https://tarkov.dev/task/${quest.slug}`}
            target="_blank"
            rel="noreferrer"
            className={linkCls}
          >
            <ExternalLink {...iconSm} />
            {t.quests.openTarkovDev}
          </a>
        )}
        {quest.wiki && (
          <a href={quest.wiki} target="_blank" rel="noreferrer" className={linkCls}>
            <ExternalLink {...iconSm} />
            {t.quests.openWiki}
          </a>
        )}
      </div>
    </div>
  );
}
