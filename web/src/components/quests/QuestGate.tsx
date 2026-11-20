import { CircleAlert, CloudDownload, RefreshCw } from "lucide-react";
import { useEffect, useState, type ReactNode } from "react";
import { t } from "../../locales";
import { cn } from "../../lib/cn";
import { ago } from "../../lib/format";
import { icon, iconSm } from "../../lib/icons";
import { say } from "../../lib/msg";
import type { Progress } from "../../lib/questProtocol";
import { useAutoQuests, useQuests, type QuestDb } from "../../lib/quests";
import { send } from "../../lib/socket";
import { useApp } from "../../store/app";
import { Button, IconButton } from "../ui/button";
import { Skeleton } from "../ui/skeleton";

export const refreshQuests = () => send({ type: "questsRefresh" });

/** Mode, age of the data and refresh button, for the header of the Quests section. */
export function QuestDataStatus() {
  const s = useApp((st) => st.view?.quests);
  const [, tick] = useState(0);
  useEffect(() => {
    const id = window.setInterval(() => tick((n) => n + 1), 30_000);
    return () => window.clearInterval(id);
  }, []);
  if (!s || s.state === "empty") return null;
  const loading = s.state === "loading";
  return (
    <div className="flex shrink-0 items-center gap-2 text-xs text-fg-muted">
      {(s.mode === "pve" || s.mode === "regular") && (
        <span className="rounded-sm bg-active px-1.5 py-0.5 text-2xs text-fg-soft">
          {t.data.mode[s.mode]}
        </span>
      )}
      {s.fetchedAt && (
        <span title={new Date(s.fetchedAt * 1000).toLocaleString()}>{t.data.updated(ago(s.fetchedAt))}</span>
      )}
      <IconButton label={t.quests.data.refresh} onClick={refreshQuests} disabled={loading}>
        <RefreshCw {...iconSm} className={cn(loading && "animate-spin motion-reduce:animate-none")} />
      </IconButton>
    </div>
  );
}

/** Shows its content only when the quest data and the progress are there; otherwise offers the
 *  download or explains what went wrong. */
export function QuestGate({ children }: { children: (db: QuestDb, p: Progress) => ReactNode }) {
  const { db, loading, error, status } = useAutoQuests();
  const progress = useQuests((s) => s.progress);
  if (db && progress) return <>{children(db, progress)}</>;
  if (status?.state === "loading" || loading || (db && !progress))
    return (
      <div className="grid h-full grid-rows-[auto_1fr]" aria-busy="true">
        <div className="flex items-center gap-3 border-b border-line px-4 py-3">
          <RefreshCw {...iconSm} className="animate-spin text-accent motion-reduce:animate-none" />
          <span className="text-sm">{t.quests.data.loading}</span>
          <span className="text-xs text-fg-muted">{t.quests.data.loadingHint}</span>
        </div>
        <div className="grid content-start gap-px p-4">
          {Array.from({ length: 12 }, (_, i) => (
            <Skeleton key={i} className="h-12" />
          ))}
        </div>
      </div>
    );
  // A load error only counts while the backend says there is data to load (a change of language
  // or mode in between makes the old error meaningless).
  const failed = status?.state === "error" || (Boolean(error) && status?.state === "ready");
  return (
    <div className="grid h-full place-items-center p-6">
      <div className="grid w-full max-w-md justify-items-start gap-4">
        {failed ? (
          <CircleAlert {...icon} className="text-danger" />
        ) : (
          <CloudDownload size={22} strokeWidth={1.5} className="text-fg-muted" />
        )}
        <div className="grid gap-1.5">
          <h2 className="text-lg font-medium">{failed ? t.quests.data.failed : t.quests.data.title}</h2>
          <p className="text-sm text-fg-soft">
            {failed ? (status?.error ? say(status.error) : error) : t.quests.data.body}
          </p>
          <p className="text-xs text-fg-muted">{t.quests.data.hosts}</p>
        </div>
        <Button variant="primary" onClick={refreshQuests}>
          {failed ? <RefreshCw {...icon} /> : <CloudDownload {...icon} />}
          {failed ? t.connection.retry : t.quests.data.action}
        </Button>
      </div>
    </div>
  );
}
