import { Check, Lock, Minus, Plus } from "lucide-react";
import { t } from "../../locales";
import { cn } from "../../lib/cn";
import { iconSm } from "../../lib/icons";
import type { QStatus } from "../../lib/questLogic";
import type { Objective } from "../../lib/quests";
import { send } from "../../lib/socket";
import { useApp } from "../../store/app";

/** Name of a map: the installed pack's, or the id made readable («streets-of-tarkov»). */
export function mapName(id: string): string {
  const pack = useApp.getState().view?.packs.find((p) => p.id === id);
  if (pack) return pack.name;
  return id
    .split("-")
    .map((w) => (w.length > 2 ? w.charAt(0).toUpperCase() + w.slice(1) : w))
    .join(" ");
}

export const isInstalled = (id: string) => Boolean(useApp.getState().view?.packs.some((p) => p.id === id));

/** State marker used in every quest list: ring (active), outline (available), lock, tick. */
export function StatusDot({ status, className }: { status: QStatus; className?: string }) {
  if (status === "completed")
    return (
      <span
        aria-hidden
        className={cn("grid size-4 shrink-0 place-items-center rounded-full bg-ok text-app", className)}
      >
        <Check size={11} strokeWidth={3} />
      </span>
    );
  if (status === "locked")
    return <Lock aria-hidden {...iconSm} className={cn("shrink-0 text-fg-muted", className)} />;
  return (
    <span
      aria-hidden
      className={cn(
        "size-3.5 shrink-0 rounded-full border-2",
        status === "active" && "border-accent bg-accent-soft",
        status === "available" && "border-fg-soft",
        status === "failed" && "border-danger",
        className,
      )}
    />
  );
}

export function Badges({ kappa, lightkeeper }: { kappa: boolean; lightkeeper: boolean }) {
  if (!kappa && !lightkeeper) return null;
  return (
    <span className="flex shrink-0 gap-1">
      {kappa && (
        <span title={t.quests.filters.kappaHint} className="rounded-sm bg-active px-1 text-2xs text-fg-soft">
          K
        </span>
      )}
      {lightkeeper && (
        <span title={t.quests.lightkeeper} className="rounded-sm bg-active px-1 text-2xs text-fg-soft">
          LK
        </span>
      )}
    </span>
  );
}

/** Tick box, or − n/total + for objectives done several times. Only for active quests. */
export function ObjectiveCheck({ o, done, label }: { o: Objective; done: number; label: string }) {
  const set = (count: number) => {
    send({ type: "questObjective", objectiveId: o.id, count: Math.max(0, Math.min(o.count, count)) });
    // Finishing the objective you're being guided to ends the guide.
    const sel = useApp.getState().view?.selectedExtract ?? "";
    if (count >= o.count && sel.startsWith(`qo-${o.id}-`)) send({ type: "selectExtract", extractId: null });
  };
  if (o.count === 1) {
    const checked = done >= 1;
    return (
      <button
        type="button"
        role="checkbox"
        aria-checked={checked}
        aria-label={label}
        onClick={() => set(checked ? 0 : 1)}
        className={cn(
          "grid size-4.5 shrink-0 place-items-center rounded-sm border transition-colors",
          checked
            ? "border-transparent bg-accent text-accent-fg"
            : "border-line-strong hover:border-fg-muted",
        )}
      >
        {checked && <Check size={12} strokeWidth={3} />}
      </button>
    );
  }
  const step = "grid size-6 place-items-center rounded-sm text-fg-soft hover:bg-hover hover:text-fg";
  return (
    <span role="group" aria-label={label} className="flex shrink-0 items-center">
      <button type="button" aria-label={t.quests.less} className={step} onClick={() => set(done - 1)}>
        <Minus size={12} />
      </button>
      <span className={cn("num min-w-12 text-center text-xs", done >= o.count ? "text-accent" : "text-fg")}>
        {t.quests.count(done, o.count)}
      </span>
      <button type="button" aria-label={t.quests.more} className={step} onClick={() => set(done + 1)}>
        <Plus size={12} />
      </button>
    </span>
  );
}
