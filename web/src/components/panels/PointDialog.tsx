import { useState } from "react";
import { t } from "../../locales";
import { cn } from "../../lib/cn";
import { POINT_KINDS, kindLabel } from "../../lib/pointKinds";
import type { PointKind } from "../../lib/protocol";
import { send } from "../../lib/socket";
import { useApp, type PointDraft } from "../../store/app";
import { Button } from "../ui/button";
import { Dialog } from "../ui/dialog";
import { Field } from "../ui/field";
import { Input, Textarea } from "../ui/input";

export function PointDialog() {
  const open = useApp((s) => s.dialog === "point");
  const draft = useApp((s) => s.pointDraft);
  return open && draft ? (
    <PointForm key={draft.mode === "edit" ? draft.id : `${draft.px},${draft.py}`} draft={draft} />
  ) : null;
}

function PointForm({ draft }: { draft: PointDraft }) {
  const openDialog = useApp((s) => s.openDialog);
  const points = useApp((s) => s.view?.points ?? []);
  const existing = draft.mode === "edit" ? points.find((p) => p.id === draft.id) : undefined;
  const [name, setName] = useState(existing?.name ?? t.points.defaultName(points.length + 1));
  const [kind, setKind] = useState<PointKind>(existing?.kind ?? "marker");
  const [note, setNote] = useState(existing?.note ?? "");
  const close = () => openDialog(null);
  const valid = name.trim().length > 0 && name.trim().length <= 40;

  const submit = () => {
    if (!valid) return;
    if (draft.mode === "new")
      send({ type: "addPoint", px: draft.px, py: draft.py, name: name.trim(), kind, note: note.trim() });
    else send({ type: "updatePoint", id: draft.id, name: name.trim(), kind, note: note.trim() });
    close();
  };

  return (
    <Dialog
      open
      onClose={close}
      title={draft.mode === "new" ? t.points.newTitle : t.points.editTitle}
      footer={
        <>
          <Button onClick={close}>{t.settings.cancel}</Button>
          <Button variant="primary" type="submit" form="point-form" disabled={!valid}>
            {draft.mode === "new" ? t.points.create : t.points.save}
          </Button>
        </>
      }
    >
      <form
        id="point-form"
        className="grid gap-4"
        onSubmit={(e) => {
          e.preventDefault();
          submit();
        }}
      >
        <Field id="pt-name" label={t.points.name}>
          <Input
            id="pt-name"
            autoFocus
            maxLength={40}
            value={name}
            onChange={(e) => setName(e.target.value)}
            onFocus={(e) => e.target.select()}
          />
        </Field>
        <fieldset className="grid gap-1.5">
          <legend className="mb-1.5 text-xs font-medium text-fg-soft">{t.points.kind}</legend>
          <div role="radiogroup" aria-label={t.points.kind} className="grid grid-cols-3 gap-1.5">
            {POINT_KINDS.map((k) => (
              <button
                key={k.kind}
                type="button"
                role="radio"
                aria-checked={kind === k.kind}
                onClick={() => setKind(k.kind)}
                className={cn(
                  "flex h-8 items-center gap-2 rounded-sm border px-2 text-sm",
                  kind === k.kind
                    ? "border-accent bg-accent-soft text-fg"
                    : "border-line-strong text-fg-soft hover:bg-hover",
                )}
              >
                <span
                  aria-hidden
                  className="size-2.5 shrink-0 rounded-full"
                  style={{ background: k.color }}
                />
                <span className="truncate">{kindLabel(k.kind)}</span>
              </button>
            ))}
          </div>
        </fieldset>
        <Field id="pt-note" label={t.points.note}>
          <Textarea
            id="pt-note"
            maxLength={200}
            value={note}
            onChange={(e) => setNote(e.target.value)}
            placeholder={t.points.notePlaceholder}
          />
        </Field>
      </form>
    </Dialog>
  );
}
