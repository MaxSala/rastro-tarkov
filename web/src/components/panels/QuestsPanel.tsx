import { ArrowUpRight, CloudDownload } from "lucide-react";
import { useMemo } from "react";
import { t } from "../../locales";
import { icon } from "../../lib/icons";
import { doneCount, openHere } from "../../lib/questLogic";
import { usePlan } from "../../lib/questPlan";
import { questColors, raidObjectives, readyToHandIn, routeOrder } from "../../lib/questPlanning";
import type { QuestMarker } from "../../lib/questProtocol";
import { questIndex, useAutoQuests, useQuests, type Objective } from "../../lib/quests";
import { useApp } from "../../store/app";
import { Button, IconButton } from "../ui/button";
import { refreshQuests } from "../quests/QuestGate";
import { PanelFrame, PanelNote } from "./PanelFrame";
import { HandIn } from "./quests/HandIn";
import { NextObjective } from "./quests/NextObjective";
import { ObjectiveLine, type Entry } from "./quests/ObjectiveLine";
import { RaidPlan } from "./quests/RaidPlan";

const sectionTitle = "px-3 pt-3 pb-1 text-2xs font-medium tracking-wider text-fg-muted uppercase";

/**
 * Quests on the map you're on, in the order a raid goes: plan it, see what's next, tick things
 * off, then hand them in.
 */
export function QuestsPanel() {
  const { db, status } = useAutoQuests();
  const progress = useQuests((s) => s.progress);
  const map = useApp((s) => s.view?.activeMap?.id);
  const markers = useApp((s) => s.view?.questMarkers);
  const player = useApp((s) => s.view?.player);
  const selected = useApp((s) => s.view?.selectedExtract ?? null);
  const setWorkspace = useApp((s) => s.setWorkspace);
  const plan = usePlan(map, (id) => progress?.quests[id] === "active");
  const planKey = plan.join(",");

  const data = useMemo(() => {
    if (!db || !progress || !map) return null;
    const idx = questIndex(db);
    const { here, anywhere, elsewhere } = openHere(db, progress, map);
    const live = (markers ?? []).filter((m) => !m.focus);
    const nearest = (o: Objective): QuestMarker | null =>
      live
        .filter((m) => m.objective === o.id)
        .sort((a, b) => (a.distanceM ?? Infinity) - (b.distanceM ?? Infinity))[0] ?? null;
    const planSet = planKey ? new Set(planKey.split(",")) : null;
    const mine = here.filter((e) => !planSet || planSet.has(e.quest.id));
    const order = routeOrder(
      live.filter((m) => !planSet || planSet.has(m.quest)),
      player ? [player.px, player.py] : null,
    );
    const rank = (e: Entry) => order.get(e.o.id) ?? (e.spot?.distanceM ?? Infinity) + 1e6;
    const withSpot: Entry[] = mine.map(({ quest, o }) => ({ quest, o, spot: nearest(o) }));
    withSpot.sort((a, b) => Number(!a.spot) - Number(!b.spot) || rank(a) - rank(b));
    const onThisMap = db.quests.filter(
      (q) => progress.quests[q.id] === "active" && raidObjectives(q, map).length > 0,
    );
    return {
      idx,
      here: withSpot,
      anywhere: anywhere.map(({ quest, o }) => ({ quest, o, spot: null })),
      elsewhere,
      order: planSet ? order : new Map<string, number>(),
      onThisMap,
      colors: questColors(live.map((m) => m.quest)),
      ready: readyToHandIn(db, progress),
    };
  }, [db, progress, map, markers, planKey, player]);

  const actions = (
    <IconButton label={t.quests.panel.open} onClick={() => setWorkspace("quests")}>
      <ArrowUpRight {...icon} />
    </IconButton>
  );
  const active = progress ? Object.values(progress.quests).filter((s) => s === "active").length : 0;
  const isSel = (e: Entry) => Boolean(e.spot && e.spot.id === selected);
  const next = data?.here.find((e) => e.spot);
  const color = (e: Entry) => (data && data.colors.size > 1 ? (data.colors.get(e.quest.id) ?? null) : null);

  return (
    <PanelFrame title={t.panels.quests} actions={actions}>
      {!db || !progress ? (
        <div className="grid justify-items-center gap-3 px-4 py-8 text-center">
          <p className="text-sm text-fg-muted">{t.quests.panel.noData}</p>
          <Button size="sm" onClick={refreshQuests} disabled={status?.state === "loading"}>
            <CloudDownload {...icon} />
            {t.quests.data.action}
          </Button>
        </div>
      ) : active === 0 ? (
        <div className="grid justify-items-center gap-3 px-4 py-8 text-center">
          <p className="text-sm text-fg-muted">{t.quests.noActive}</p>
          <Button size="sm" onClick={() => setWorkspace("quests")}>
            {t.quests.panel.choose}
          </Button>
        </div>
      ) : (
        data &&
        map && (
          <div className="min-h-0 flex-1 overflow-y-auto">
            {next && <NextObjective e={next} number={data.order.get(next.o.id) ?? null} />}
            <RaidPlan map={map} quests={data.onThisMap} plan={plan} p={progress} colors={data.colors} />
            {data.here.length === 0 && data.anywhere.length === 0 && (
              <PanelNote>{t.quests.panel.noneHere}</PanelNote>
            )}
            {data.here.length > 0 && (
              <section>
                <h3 className={sectionTitle}>
                  {plan.length > 0 ? t.quests.plan.inPlan : t.quests.panel.here}
                </h3>
                <ul>
                  {data.here.map((e) => (
                    <ObjectiveLine
                      key={e.o.id}
                      e={e}
                      selected={isSel(e)}
                      done={doneCount(e.o, progress)}
                      color={color(e)}
                      number={data.order.get(e.o.id) ?? null}
                    />
                  ))}
                </ul>
              </section>
            )}
            {data.anywhere.length > 0 && (
              <section>
                <h3 className={sectionTitle}>{t.quests.panel.anywhere}</h3>
                <ul>
                  {data.anywhere.map((e) => (
                    <ObjectiveLine
                      key={e.o.id}
                      e={e}
                      selected={false}
                      done={doneCount(e.o, progress)}
                      color={null}
                      number={null}
                    />
                  ))}
                </ul>
              </section>
            )}
            <HandIn quests={data.ready} idx={data.idx} />
            {data.elsewhere > 0 && (
              <button
                type="button"
                onClick={() => setWorkspace("quests")}
                className="mx-3 my-3 block text-left text-xs text-fg-muted hover:text-fg"
              >
                {t.quests.panel.elsewhere(data.elsewhere)}
              </button>
            )}
          </div>
        )
      )}
    </PanelFrame>
  );
}
