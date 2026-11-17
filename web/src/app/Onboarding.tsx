import { CircleAlert, CircleCheck } from "lucide-react";
import { useEffect, useState } from "react";
import { t } from "../locales";
import { cn } from "../lib/cn";
import { iconSm } from "../lib/icons";
import { send } from "../lib/socket";
import { useApp } from "../store/app";
import { TarkovDevCard } from "../components/panels/TarkovDevCard";
import { Button } from "../components/ui/button";
import { Dialog } from "../components/ui/dialog";
import { Input } from "../components/ui/input";

type Choice = "tarkov" | "manual" | "demo";

function FolderStep() {
  const settings = useApp((s) => s.view?.settings);
  const watcher = useApp((s) => s.view?.watcher);
  const check = useApp((s) => s.folderCheck);
  const [path, setPath] = useState(settings?.screenshotsDir ?? "");
  useEffect(() => {
    if (settings?.screenshotsDir) send({ type: "checkFolder", path: settings.screenshotsDir });
    // Solo al entrar en el paso.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);
  const ok = watcher && !watcher.error;
  return (
    <div className="grid gap-3">
      <p className="text-sm text-fg-soft">{t.onboarding.folderBody}</p>
      <div className="grid gap-1 rounded-sm border border-line p-3">
        <p className="flex items-center gap-1.5 text-sm">
          {ok ? (
            <CircleCheck {...iconSm} className="text-ok" />
          ) : (
            <CircleAlert {...iconSm} className="text-danger" />
          )}
          {ok ? t.onboarding.folderFound : t.onboarding.folderMissing}
        </p>
        <p className="num text-xs break-all text-fg-muted">{watcher?.folder ?? settings?.screenshotsDir}</p>
        {ok && check?.exists && (
          <p className="text-xs text-fg-soft">{t.onboarding.captures(check.captures)}</p>
        )}
      </div>
      {!ok && (
        <div className="grid gap-2">
          <p className="text-xs text-fg-muted">{t.onboarding.folderHint}</p>
          <div className="flex gap-2">
            <label htmlFor="ob-path" className="sr-only">
              {t.onboarding.folderStep}
            </label>
            <Input
              id="ob-path"
              className="num text-xs"
              value={path}
              onChange={(e) => setPath(e.target.value)}
              spellCheck={false}
            />
            <Button onClick={() => send({ type: "checkFolder", path })} disabled={!path.trim()}>
              {t.onboarding.check}
            </Button>
          </div>
          {check && check.path && (
            <p className={cn("text-xs", check.exists ? "text-fg-soft" : "text-danger")}>
              {check.exists ? t.onboarding.captures(check.captures) : t.onboarding.folderMissing}
            </p>
          )}
          {check?.exists && (
            <Button variant="primary" onClick={() => send({ type: "setScreenshotsDir", path })}>
              {t.onboarding.use}
            </Button>
          )}
        </div>
      )}
    </div>
  );
}

function Option({
  value,
  choice,
  setChoice,
  title,
  hint,
}: {
  value: Choice;
  choice: Choice;
  setChoice: (c: Choice) => void;
  title: string;
  hint: string;
}) {
  return (
    <label
      className={cn(
        "flex cursor-pointer items-start gap-3 rounded-sm border px-3 py-2.5",
        choice === value ? "border-accent bg-accent-soft" : "border-line hover:bg-hover",
      )}
    >
      <input
        type="radio"
        name="ob-maps"
        checked={choice === value}
        onChange={() => setChoice(value)}
        className="mt-1 accent-[var(--accent)]"
      />
      <span className="grid gap-0.5">
        <span className="text-sm">{title}</span>
        <span className="text-xs text-fg-muted">{hint}</span>
      </span>
    </label>
  );
}

export function Onboarding() {
  const show = useApp((s) => s.view?.onboarding ?? false);
  const openDialog = useApp((s) => s.openDialog);
  const tarkov = useApp((s) => s.view?.tarkovdev.state);
  const [step, setStep] = useState<0 | 1 | 2>(0);
  const [choice, setChoice] = useState<Choice>("tarkov");
  if (!show) return null;
  const finish = () => send({ type: "finishOnboarding" });

  const next = () => {
    if (step === 0) return setStep(1);
    if (choice === "tarkov") return setStep(2);
    finish();
    openDialog("import", choice === "demo" ? "demo" : undefined);
  };

  const titles = [t.onboarding.folderStep, t.onboarding.mapsStep, t.tarkovdev.title];
  const footer = (
    <>
      <Button variant="ghost" onClick={finish} className="mr-auto">
        {t.onboarding.skip}
      </Button>
      {step > 0 && <Button onClick={() => setStep(step === 2 ? 1 : 0)}>{t.onboarding.back}</Button>}
      {step < 2 ? (
        <Button variant="primary" onClick={next}>
          {t.onboarding.next}
        </Button>
      ) : (
        <Button variant="primary" onClick={finish} disabled={tarkov === "running"}>
          {t.onboarding.finish}
        </Button>
      )}
    </>
  );

  return (
    <Dialog
      open
      onClose={finish}
      title={`${t.onboarding.title}: ${titles[step]}`}
      description={`${step + 1} / 3`}
      footer={footer}
    >
      {step === 0 && <FolderStep />}
      {step === 1 && (
        <div className="grid gap-2">
          <p className="text-sm text-fg-soft">{t.onboarding.mapsBody}</p>
          <Option
            value="tarkov"
            choice={choice}
            setChoice={setChoice}
            title={t.onboarding.optTarkov}
            hint={t.onboarding.optTarkovHint}
          />
          <Option
            value="manual"
            choice={choice}
            setChoice={setChoice}
            title={t.onboarding.optManual}
            hint={t.onboarding.optManualHint}
          />
          <Option
            value="demo"
            choice={choice}
            setChoice={setChoice}
            title={t.onboarding.optDemo}
            hint={t.onboarding.optDemoHint}
          />
        </div>
      )}
      {step === 2 && (
        <div className="grid gap-3">
          <TarkovDevCard />
          {tarkov === "done" && <p className="text-sm">{t.onboarding.done}</p>}
        </div>
      )}
    </Dialog>
  );
}
