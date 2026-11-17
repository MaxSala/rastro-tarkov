import { useEffect, useState } from "react";
import { lang, t } from "../locales";
import { api } from "../lib/api";
import { inline, parseChangelog, type Release } from "../lib/changelog";
import { useApp } from "../store/app";
import { Dialog } from "../components/ui/dialog";
import { Skeleton } from "../components/ui/skeleton";

const SEEN_KEY = "rastro.seenVersion";

/** After an update (not on the first install), open "What's new" once for the new version. */
export function useWhatsNewOnUpdate() {
  const openDialog = useApp((s) => s.openDialog);
  useEffect(() => {
    try {
      const seen = localStorage.getItem(SEEN_KEY);
      localStorage.setItem(SEEN_KEY, __APP_VERSION__);
      if (seen && seen !== __APP_VERSION__) openDialog("changelog");
    } catch {
      /* no storage: simply don't show it */
    }
  }, [openDialog]);
}

function Text({ value }: { value: string }) {
  return (
    <>
      {inline(value).map((tok, i) =>
        tok.kind === "code" ? (
          <code key={i} className="num rounded-[3px] bg-active px-1 text-[0.92em]">
            {tok.value}
          </code>
        ) : tok.kind === "em" ? (
          <em key={i} className="text-fg not-italic">
            {tok.value}
          </em>
        ) : (
          <span key={i}>{tok.value}</span>
        ),
      )}
    </>
  );
}

function ReleaseBlock({ rel, current }: { rel: Release; current: boolean }) {
  return (
    <section className="grid gap-3 border-b border-line pb-5 last:border-0">
      <header className="flex items-baseline gap-2">
        <h3 className="num text-base font-semibold">v{rel.version}</h3>
        {current && (
          <span className="rounded-sm bg-accent-soft px-1.5 py-px text-2xs font-medium text-accent">
            {t.changelog.current}
          </span>
        )}
        {rel.date && <span className="num text-xs text-fg-muted">{rel.date}</span>}
      </header>
      {rel.groups.map((g, i) => (
        <div key={i} className="grid gap-1.5">
          {g.title && (
            <h4 className="text-xs font-medium tracking-wide text-fg-muted uppercase">{g.title}</h4>
          )}
          <ul className="grid gap-1.5 pl-4">
            {g.items.map((item, j) => (
              <li key={j} className="list-disc text-sm leading-relaxed text-fg-soft marker:text-fg-muted">
                <Text value={item} />
              </li>
            ))}
          </ul>
        </div>
      ))}
    </section>
  );
}

export function WhatsNew() {
  const open = useApp((s) => s.dialog === "changelog");
  const close = useApp((s) => s.openDialog);
  const [releases, setReleases] = useState<Release[] | null>(null);
  const [failed, setFailed] = useState(false);
  useEffect(() => {
    if (!open || releases) return;
    api.text("/api/changelog").then(
      (md) => setReleases(parseChangelog(md)),
      () => setFailed(true),
    );
  }, [open, releases]);
  return (
    <Dialog
      open={open}
      onClose={() => close(null)}
      title={t.changelog.title}
      description={lang === "en" ? undefined : t.changelog.englishNote}
      className="w-[min(640px,calc(100vw-32px))]"
    >
      <div className="grid max-h-[60vh] gap-5 overflow-y-auto pr-1">
        {failed && <p className="text-sm text-danger">{t.errors.generic}</p>}
        {!releases && !failed && (
          <div className="grid gap-2">
            <Skeleton className="h-5 w-24" />
            <Skeleton className="h-4" />
            <Skeleton className="h-4 w-4/5" />
          </div>
        )}
        {releases?.map((rel) => (
          <ReleaseBlock key={rel.version} rel={rel} current={rel.version === __APP_VERSION__} />
        ))}
      </div>
    </Dialog>
  );
}
