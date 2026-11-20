// The raid plan: which of your active quests you're doing in the next raid on each map. While a
// plan is set, the map and the Quests panel show only those quests, numbered in route order.
// Kept in this browser only (a convenience, like a filter): it's easy to make again.
import { create } from "zustand";

const KEY = "rastro.questPlan";

type Plans = Record<string, string[]>;

function load(): Plans {
  try {
    const raw: unknown = JSON.parse(localStorage.getItem(KEY) ?? "{}");
    if (!raw || typeof raw !== "object") return {};
    const out: Plans = {};
    for (const [map, ids] of Object.entries(raw as Record<string, unknown>)) {
      if (!/^[a-z0-9-]{1,40}$/.test(map) || !Array.isArray(ids)) continue;
      out[map] = ids
        .filter((i): i is string => typeof i === "string" && /^[0-9a-f]{24}$/.test(i))
        .slice(0, 50);
    }
    return out;
  } catch {
    return {};
  }
}

function save(plans: Plans) {
  try {
    localStorage.setItem(KEY, JSON.stringify(plans));
  } catch {
    // Without storage the plan lasts until Rastro is closed.
  }
}

interface PlanState {
  plans: Plans;
  /** Adds or removes a quest from the plan of `map`. */
  toggle: (map: string, quest: string) => void;
  clear: (map: string) => void;
}

export const useQuestPlan = create<PlanState>((set, get) => ({
  plans: load(),
  toggle: (map, quest) => {
    const cur = get().plans[map] ?? [];
    const next = cur.includes(quest) ? cur.filter((q) => q !== quest) : [...cur, quest];
    const plans = { ...get().plans, [map]: next };
    save(plans);
    set({ plans });
  },
  clear: (map) => {
    const plans = { ...get().plans, [map]: [] };
    save(plans);
    set({ plans });
  },
}));

const EMPTY: string[] = [];

/** Quests planned for `map` (only those still active count). */
export function usePlan(map: string | undefined, active: (id: string) => boolean): string[] {
  const ids = useQuestPlan((s) => (map ? s.plans[map] : undefined) ?? EMPTY);
  return ids.filter(active);
}
