import { FileArchive, TriangleAlert } from "lucide-react";
import { useCallback, useEffect, useRef, useState, type ReactNode } from "react";
import { t } from "../../locales";
import { api } from "../../lib/api";
import { cn } from "../../lib/cn";
import { icon, iconSm } from "../../lib/icons";
import type { Preview } from "../../lib/protocol";
import { useApp } from "../../store/app";
import { Button } from "../ui/button";
import { Dialog } from "../ui/dialog";
import { Field } from "../ui/field";
import { Input } from "../ui/input";
import { Segmented } from "../ui/segmented";

type Step =
  | { kind: "source" }
  | { kind: "confirmUrl"; host: string }
  | { kind: "working" }
  | { kind: "review"; preview: Preview };

const MAX_BYTES = 50 * 1024 * 1024;

function Review({
  p,
  accepted,
  setAccepted,
}: {
  p: Preview;
  accepted: boolean;
  setAccepted: (v: boolean) => void;
}) {
  const rows: [string, string, boolean?][] = [
    [t.importer.id, p.id, true],
    [t.importer.size, `${p.width} × ${p.height} px`, true],
    [t.importer.author, p.author],
    [t.importer.license, p.license],
    [t.importer.source, p.source || "—"],
    [t.importer.hash, p.sha256, true],
  ];
  return (
    <div className="grid gap-3">
      <p className="text-sm">
        <span className="font-medium">{p.name}</span>{" "}
        <span className="text-fg-muted">{t.maps.extractsCount(p.extracts)}</span>
      </p>
      <dl className="grid grid-cols-[88px_1fr] gap-x-3 gap-y-1.5 rounded-sm border border-line p-3 text-xs">
        {rows.map(([k, v, mono]) => (
          <div key={k} className="contents">
            <dt className="text-fg-muted">{k}</dt>
            <dd className={cn("break-all", mono && "num")}>{v}</dd>
          </div>
        ))}
      </dl>
      {p.sha256Verified && <p className="text-xs text-ok">{t.importer.verified}</p>}
      {p.exists && (
        <p className="flex gap-2 text-xs text-fg-soft">
          <TriangleAlert {...iconSm} className="mt-0.5 shrink-0 text-accent" />
          {t.importer.exists}
        </p>
      )}
      <label className="flex items-start gap-2 text-sm">
        <input
          type="checkbox"
          checked={accepted}
          onChange={(e) => setAccepted(e.target.checked)}
          className="mt-0.5 size-4 accent-[var(--accent)]"
        />
        {t.importer.accept}
      </label>
    </div>
  );
}

export function ImportDialog() {
  const open = useApp((s) => s.dialog === "import");
  return open ? <ImportFlow /> : null;
}

function ImportFlow() {
  const open = true;
  const openDialog = useApp((s) => s.openDialog);
  const notify = useApp((s) => s.notify);
  // La demo arranca directamente en «validando»; el resto, en la elección de origen.
  const [step, setStep] = useState<Step>(() =>
    useApp.getState().dialogArg === "demo" ? { kind: "working" } : { kind: "source" },
  );
  const [mode, setMode] = useState<"file" | "url">("file");
  const [url, setUrl] = useState("");
  const [sha, setSha] = useState("");
  const [accepted, setAccepted] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [dragging, setDragging] = useState(false);
  const fileInput = useRef<HTMLInputElement>(null);
  const pending = useRef<string | null>(null);

  const run = useCallback(async (task: () => Promise<Preview>) => {
    setError(null);
    setStep({ kind: "working" });
    try {
      const preview = await task();
      pending.current = preview.stagingId;
      setAccepted(false);
      setStep({ kind: "review", preview });
    } catch (e) {
      setError(e instanceof Error ? e.message : t.errors.generic);
      setStep({ kind: "source" });
    }
  }, []);

  useEffect(() => {
    if (useApp.getState().dialogArg !== "demo") return;
    let alive = true;
    api
      .demo()
      .then((preview) => {
        if (!alive) return void api.cancel(preview.stagingId).catch(() => undefined);
        pending.current = preview.stagingId;
        setStep({ kind: "review", preview });
      })
      .catch((e: unknown) => {
        if (!alive) return;
        setError(e instanceof Error ? e.message : t.errors.generic);
        setStep({ kind: "source" });
      });
    return () => {
      alive = false;
    };
  }, []);

  const close = () => {
    if (pending.current) void api.cancel(pending.current).catch(() => undefined);
    pending.current = null;
    openDialog(null);
  };

  const onFile = (file: File | undefined) => {
    if (!file) return;
    if (!file.name.toLowerCase().endsWith(".zip")) return setError(t.a11y.zipOnly);
    if (file.size > MAX_BYTES) return setError(t.a11y.zipTooBig);
    void run(() => api.uploadZip(file, sha));
  };

  const install = async (p: Preview) => {
    try {
      await api.commit(p.stagingId, p.exists);
      pending.current = null;
      notify("info", t.importer.done(p.name));
      openDialog(null);
    } catch (e) {
      setError(e instanceof Error ? e.message : t.errors.generic);
    }
  };

  let body: ReactNode;
  let footer: ReactNode = null;
  if (step.kind === "working") {
    body = (
      <p className="py-6 text-center text-sm text-fg-muted" role="status">
        {t.importer.working}
      </p>
    );
  } else if (step.kind === "review") {
    body = <Review p={step.preview} accepted={accepted} setAccepted={setAccepted} />;
    footer = (
      <>
        <Button onClick={close}>{t.importer.cancel}</Button>
        <Button variant="primary" disabled={!accepted} onClick={() => void install(step.preview)}>
          {step.preview.exists ? t.importer.replace : t.importer.install}
        </Button>
      </>
    );
  } else if (step.kind === "confirmUrl") {
    body = (
      <div className="grid gap-2 text-sm">
        <p>{t.importer.confirmHost(step.host)}</p>
        {!sha.trim() && (
          <p className="flex gap-2 text-xs text-fg-soft">
            <TriangleAlert {...iconSm} className="mt-0.5 shrink-0 text-accent" />
            {t.importer.noShaWarning}
          </p>
        )}
      </div>
    );
    footer = (
      <>
        <Button onClick={() => setStep({ kind: "source" })}>{t.importer.back}</Button>
        <Button variant="primary" onClick={() => void run(() => api.fromUrl(url, sha))}>
          {t.importer.download}
        </Button>
      </>
    );
  } else {
    body = (
      <>
        <Segmented
          label={t.importer.title}
          value={mode}
          onChange={setMode}
          options={[
            { value: "file", label: t.importer.fromFile },
            { value: "url", label: t.importer.fromUrl },
          ]}
        />
        {mode === "file" ? (
          <div
            onDragOver={(e) => {
              e.preventDefault();
              setDragging(true);
            }}
            onDragLeave={() => setDragging(false)}
            onDrop={(e) => {
              e.preventDefault();
              setDragging(false);
              onFile(e.dataTransfer.files[0]);
            }}
            className={cn(
              "grid justify-items-center gap-2 rounded-sm border border-dashed px-4 py-6 text-center",
              dragging ? "border-accent bg-accent-soft" : "border-line-strong",
            )}
          >
            <FileArchive {...icon} className="text-fg-muted" />
            <p className="text-sm text-fg-soft">{t.importer.drop}</p>
            <input
              ref={fileInput}
              type="file"
              accept=".zip,application/zip"
              className="sr-only"
              tabIndex={-1}
              onChange={(e) => onFile(e.target.files?.[0])}
            />
            <Button size="sm" onClick={() => fileInput.current?.click()}>
              {t.importer.choose}
            </Button>
          </div>
        ) : (
          <Field id="import-url" label={t.importer.url}>
            <Input
              id="import-url"
              type="url"
              inputMode="url"
              placeholder="https://"
              value={url}
              onChange={(e) => setUrl(e.target.value)}
              spellCheck={false}
            />
          </Field>
        )}
        <Field id="import-sha" label={t.importer.sha} hint={t.importer.shaHint}>
          <Input
            id="import-sha"
            className="num text-xs"
            value={sha}
            onChange={(e) => setSha(e.target.value)}
            spellCheck={false}
            aria-describedby="import-sha-hint"
          />
        </Field>
      </>
    );
    if (mode === "url") {
      footer = (
        <Button
          variant="primary"
          disabled={!url.trim()}
          onClick={async () => {
            setError(null);
            try {
              setStep({ kind: "confirmUrl", host: await api.inspectUrl(url) });
            } catch (e) {
              setError(e instanceof Error ? e.message : t.errors.generic);
            }
          }}
        >
          {t.importer.check}
        </Button>
      );
    }
  }

  return (
    <Dialog
      open={open}
      onClose={close}
      title={step.kind === "review" ? t.importer.reviewTitle : t.importer.title}
      footer={footer}
    >
      {error && (
        <p role="alert" className="rounded-sm border border-danger/60 px-3 py-2 text-sm text-danger">
          {error}
        </p>
      )}
      {body}
    </Dialog>
  );
}
