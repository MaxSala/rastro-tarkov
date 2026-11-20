import { Navigation } from "lucide-react";
import { t } from "../../../locales";
import { meters } from "../../../lib/format";
import { iconSm } from "../../../lib/icons";
import { send } from "../../../lib/socket";
import { useApp } from "../../../store/app";
import { Button } from "../../ui/button";
import { QuestGlyph } from "../../quests/QuestGlyph";
import type { Entry } from "./ObjectiveLine";

/** The one thing to do next, big enough to read at a glance from the other monitor. */
export function NextObjective({ e, number }: { e: Entry; number: number | null }) {
  const selected = useApp((s) => s.view?.selectedExtract);
  if (!e.spot) return null;
  const guiding = selected === e.spot.id;
  return (
    <section
      aria-label={t.quests.panel.next}
      className="m-3 grid gap-2 rounded-md border border-line-strong bg-app p-3"
    >
      <p className="flex items-center gap-2 text-2xs font-medium tracking-wider text-fg-muted uppercase">
        {t.quests.panel.next}
        {number !== null && <span className="num text-accent">#{number}</span>}
      </p>
      <div className="flex items-start gap-2.5">
        <QuestGlyph kind={e.o.kind} className="mt-0.5 size-6" />
        <div className="min-w-0 flex-1">
          <p className="text-sm leading-snug font-medium">{e.o.text}</p>
          <p className="truncate text-xs text-fg-muted">{e.quest.name}</p>
        </div>
        {e.spot.distanceM != null && (
          <span className="num shrink-0 text-lg leading-none font-medium">{meters(e.spot.distanceM)}</span>
        )}
      </div>
      <Button
        size="sm"
        variant={guiding ? "secondary" : "primary"}
        onClick={() => send({ type: "selectExtract", extractId: guiding ? null : (e.spot?.id ?? null) })}
      >
        <Navigation {...iconSm} />
        {guiding ? t.quests.panel.guiding : t.quests.panel.guide}
      </Button>
    </section>
  );
}
