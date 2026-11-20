// What the quest layer needs besides the markers: the raid plan for this map, the route order
// through it (from your position) and the colour of each quest.
import { useMemo } from "react";
import { usePlan } from "../../lib/questPlan";
import { questColors, routeOrder } from "../../lib/questPlanning";
import type { QuestMarker } from "../../lib/questProtocol";
import { useQuests } from "../../lib/quests";
import { useApp } from "../../store/app";
import type { QuestDrawOptions } from "./quests";

export function useQuestOptions(
  map: string,
  markers: QuestMarker[] | undefined,
): QuestDrawOptions & { key: string } {
  const progress = useQuests((s) => s.progress);
  const player = useApp((s) => s.view?.player);
  const plan = usePlan(map, (id) => progress?.quests[id] === "active");
  const planKey = plan.join(",");
  const start: [number, number] | null = player ? [player.px, player.py] : null;
  const startKey = start ? `${Math.round(start[0])},${Math.round(start[1])}` : "";
  return useMemo(() => {
    const list = markers ?? [];
    const set = planKey ? new Set(planKey.split(",")) : null;
    const planned = set ? list.filter((m) => set.has(m.quest) && !m.focus) : [];
    const colors = questColors(list.filter((m) => !m.focus).map((m) => m.quest));
    return { plan: set, order: routeOrder(planned, start), colors, key: `${planKey}|${startKey}` };
    // eslint-disable-next-line react-hooks/exhaustive-deps -- start is summed up by startKey
  }, [markers, planKey, startKey]);
}
