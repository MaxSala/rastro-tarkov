import { History, ListChecks, ScrollText } from "lucide-react";
import { useMemo, useState } from "react";
import { t } from "../../locales";
import { iconSm } from "../../lib/icons";
import { chainBefore } from "../../lib/questLogic";
import type { Progress } from "../../lib/questProtocol";
import { questIndex, type QuestDb } from "../../lib/quests";
import { send } from "../../lib/socket";
import { useApp } from "../../store/app";
import { Button } from "../ui/button";
import { Dialog } from "../ui/dialog";
import { TraderAvatar } from "./media";

const select =
  "h-8 w-full min-w-0 rounded-sm border border-line-strong bg-app px-2 text-xs text-fg focus-visible:border-accent focus-visible:outline-none";

/** For each trader, the last quest you completed: everything before it is done too. */
export function CatchUpDialog({
  open,
  onClose,
  db,
  p,
}: {
  open: boolean;
  onClose: () => void;
  db: QuestDb;
  p: Progress;
}) {
  const idx = questIndex(db);
  const [picks, setPicks] = useState<Record<string, string>>({});
  const byTrader = useMemo(
    () =>
      db.traders.map((tr) => ({
        tr,
        quests: db.quests
          .filter((q) => q.trader === tr.id)
          .sort((a, b) => a.level - b.level || a.name.localeCompare(b.name)),
      })),
    [db],
  );
  const chosen = Object.values(picks).filter(Boolean);
  const total = new Set(
    chosen.flatMap((id) => {
      const q = idx.byId.get(id);
      return q ? [id, ...chainBefore(q, p, idx).map((x) => x.id)] : [];
    }),
  ).size;
  const apply = () => {
    if (chosen.length > 0) send({ type: "questsCompleteThrough", questIds: chosen });
    setPicks({});
    onClose();
  };
  return (
    <Dialog
      open={open}
      onClose={onClose}
      title={t.quests.catchUp.dialogTitle}
      description={t.quests.catchUp.dialogHint}
      className="w-[min(560px,calc(100vw-32px))]"
      footer={
        <>
          <Button onClick={onClose}>{t.quests.catchUp.cancel}</Button>
          <Button variant="primary" disabled={total === 0} onClick={apply}>
            {t.quests.catchUp.apply(total)}
          </Button>
        </>
      }
    >
      <ul className="grid max-h-[60vh] gap-2 overflow-y-auto p-4">
        {byTrader.map(({ tr, quests }) => (
          <li key={tr.id} className="flex items-center gap-3">
            <TraderAvatar image={idx.portraits.get(tr.id) ?? null} name={tr.name} size={32} />
            <label className="w-28 shrink-0 truncate text-sm" htmlFor={`catch-${tr.id}`}>
              {tr.name}
            </label>
            <select
              id={`catch-${tr.id}`}
              value={picks[tr.id] ?? ""}
              onChange={(e) => setPicks({ ...picks, [tr.id]: e.target.value })}
              className={select}
            >
              <option value="">{t.quests.catchUp.none}</option>
              {quests.map((q) => (
                <option key={q.id} value={q.id}>
                  {t.quests.levelShort(q.level)} · {q.name}
                </option>
              ))}
            </select>
          </li>
        ))}
      </ul>
    </Dialog>
  );
}

/** First visit: how to get your progress in without ticking a hundred quests by hand. */
export function CatchUpCard({ onPick }: { onPick: () => void }) {
  const logs = useApp((s) => s.view?.quests.logs);
  const scanning = logs?.scanning ?? false;
  return (
    <div className="grid gap-2 border-b border-line bg-accent-soft px-4 py-3">
      <p className="flex items-center gap-2 text-sm font-medium">
        <History {...iconSm} className="text-accent" />
        {t.quests.catchUp.title}
      </p>
      <p className="text-xs text-fg-soft">{scanning ? t.quests.sync.scanning : t.quests.catchUp.body}</p>
      <div className="flex flex-wrap gap-2">
        {!logs?.enabled && (
          <Button size="sm" variant="primary" onClick={() => send({ type: "setGameLogs", enabled: true })}>
            <ScrollText {...iconSm} />
            {t.quests.catchUp.fromLogs}
          </Button>
        )}
        <Button size="sm" onClick={onPick}>
          <ListChecks {...iconSm} />
          {t.quests.catchUp.pick}
        </Button>
      </div>
    </div>
  );
}
