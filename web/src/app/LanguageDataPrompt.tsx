// After a language change, the maps, prices and quests downloaded before are still in the old
// language. This card offers to bring them over (maps only rebuild their texts), or does it on
// its own when the player chose "always". It never connects anywhere by itself otherwise.
import { Languages } from "lucide-react";
import { useEffect, useState } from "react";
import { NATIVE_NAMES, lang, t } from "../locales";
import { iconSm } from "../lib/icons";
import { send } from "../lib/socket";
import { useApp } from "../store/app";
import { Button } from "../components/ui/button";

const DISMISSED = "rastro.languageDataDismissed";
/** Languages already brought over automatically in this session (once each). */
const autoDone = new Set<string>();

function readDismissed(): string | null {
  try {
    return localStorage.getItem(DISMISSED);
  } catch {
    return null;
  }
}

function writeDismissed(value: string) {
  try {
    localStorage.setItem(DISMISSED, value);
  } catch {
    // Without storage the card simply comes back next time.
  }
}

const update = () => send({ type: "updateLanguageData", fleaName: t.market.flea });

export function LanguageDataPrompt() {
  const data = useApp((s) => s.view?.languageData ?? null);
  const busy = useApp(
    (s) =>
      s.view?.tarkovdev.state === "running" ||
      s.view?.market.state === "loading" ||
      s.view?.quests.state === "loading",
  );
  const onboarding = useApp((s) => s.view?.onboarding ?? true);
  const [dismissed, setDismissed] = useState(readDismissed);
  const [always, setAlways] = useState(false);
  const outdated = data?.outdated ?? [];
  const pending = outdated.length > 0 && !busy && !onboarding;

  useEffect(() => {
    if (!pending || data?.auto !== true || autoDone.has(lang)) return;
    autoDone.add(lang);
    update();
    useApp.getState().notify("info", t.language.prompt.started);
  }, [pending, data?.auto]);

  if (!pending || data?.auto === true || dismissed === lang) return null;
  const close = () => {
    writeDismissed(lang);
    setDismissed(lang);
  };
  const p = t.language.prompt;
  return (
    <section
      aria-label={p.title(NATIVE_NAMES[lang])}
      className="fixed right-4 bottom-10 z-[900] grid w-84 gap-3 rounded-md border border-line-strong bg-surface p-4 shadow-[0_8px_32px_#0008]"
    >
      <div className="flex items-start gap-2.5">
        <Languages {...iconSm} className="mt-0.5 shrink-0 text-accent" />
        <div className="grid gap-1.5">
          <h2 className="text-sm font-medium">{p.title(NATIVE_NAMES[lang])}</h2>
          <p className="text-xs text-fg-soft">{p.body}</p>
          <ul className="grid gap-0.5 text-xs">
            {outdated.map((o) => (
              <li key={o.kind} className="flex items-baseline gap-1.5">
                <span className="size-1 shrink-0 translate-y-[-2px] rounded-full bg-fg-muted" aria-hidden />
                <span>{p.kinds[o.kind]}</span>
                <span className="text-fg-muted">{o.lang ? p.from(NATIVE_NAMES[o.lang]) : p.unknown}</span>
              </li>
            ))}
          </ul>
          {outdated.some((o) => o.kind === "maps") && <p className="text-2xs text-fg-muted">{p.mapsHint}</p>}
        </div>
      </div>
      <label className="flex items-center gap-2 text-xs text-fg-soft">
        <input
          type="checkbox"
          checked={always}
          onChange={(e) => setAlways(e.target.checked)}
          className="size-3.5 accent-[var(--accent)]"
        />
        {p.always}
      </label>
      <div className="flex justify-end gap-2">
        <Button size="sm" onClick={close}>
          {p.later}
        </Button>
        <Button
          size="sm"
          variant="primary"
          onClick={() => {
            if (always) send({ type: "setAutoLanguageData", enabled: true });
            autoDone.add(lang);
            update();
            useApp.getState().notify("info", p.started);
            close();
          }}
        >
          {p.update(NATIVE_NAMES[lang])}
        </Button>
      </div>
    </section>
  );
}
