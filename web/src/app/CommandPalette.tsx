import { fold } from "../lib/search";
import { Search } from "lucide-react";
import { useEffect, useMemo, useRef, useState } from "react";
import { t } from "../locales";
import { cn } from "../lib/cn";
import { iconSm } from "../lib/icons";
import { useApp } from "../store/app";
import { Kbd } from "../components/ui/kbd";
import { buildCommands } from "./commands";

export function CommandPalette() {
  const open = useApp((s) => s.dialog === "palette");
  const openDialog = useApp((s) => s.openDialog);
  const view = useApp((s) => s.view);
  const workspace = useApp((s) => s.workspace);
  const ref = useRef<HTMLDialogElement>(null);
  const [query, setQuery] = useState("");
  const [index, setIndex] = useState(0);
  // `workspace` en las dependencias: los comandos «Ir a…» cambian según la sección actual.
  // eslint-disable-next-line react-hooks/exhaustive-deps
  const commands = useMemo(() => buildCommands(view), [view, workspace]);
  const shown = useMemo(() => {
    const q = fold(query.trim());
    return q ? commands.filter((c) => fold(c.label).includes(q)) : commands;
  }, [commands, query]);

  useEffect(() => {
    const d = ref.current;
    if (!d) return;
    if (open && !d.open) {
      setQuery("");
      setIndex(0);
      d.showModal();
    }
    if (!open && d.open) d.close();
  }, [open]);

  const run = (i: number) => {
    const cmd = shown[i];
    if (!cmd) return;
    openDialog(null);
    cmd.run();
  };

  return (
    <dialog
      ref={ref}
      aria-label={t.top.commands}
      onClose={() => openDialog(null)}
      onCancel={(e) => {
        e.preventDefault();
        openDialog(null);
      }}
      onMouseDown={(e) => e.target === ref.current && openDialog(null)}
      className="mx-auto mt-[15vh] w-[min(520px,calc(100vw-32px))] rounded-md border border-line-strong bg-surface p-0 text-fg"
    >
      <div className="flex items-center gap-2 border-b border-line px-3">
        <Search {...iconSm} className="text-fg-muted" />
        <input
          autoFocus
          role="combobox"
          aria-expanded="true"
          aria-controls="palette-list"
          aria-activedescendant={shown[index] ? `cmd-${shown[index].id}` : undefined}
          value={query}
          placeholder={t.palette.placeholder}
          onChange={(e) => {
            setQuery(e.target.value);
            setIndex(0);
          }}
          onKeyDown={(e) => {
            if (e.key === "ArrowDown") setIndex((i) => Math.min(i + 1, shown.length - 1));
            else if (e.key === "ArrowUp") setIndex((i) => Math.max(i - 1, 0));
            else if (e.key === "Enter") run(index);
            else return;
            e.preventDefault();
          }}
          className="h-11 flex-1 bg-transparent text-sm outline-none placeholder:text-placeholder"
        />
      </div>
      <ul id="palette-list" role="listbox" className="max-h-80 overflow-y-auto p-1">
        {shown.length === 0 && (
          <li className="px-3 py-6 text-center text-sm text-fg-muted">{t.palette.empty}</li>
        )}
        {shown.map((c, i) => (
          <li
            key={c.id}
            id={`cmd-${c.id}`}
            role="option"
            aria-selected={i === index}
            onMouseMove={() => setIndex(i)}
            onClick={() => run(i)}
            className={cn(
              "flex h-8 cursor-default items-center justify-between rounded-sm px-2 text-sm",
              i === index && "bg-active",
            )}
          >
            {c.label}
            {c.shortcut && <Kbd>{c.shortcut}</Kbd>}
          </li>
        ))}
      </ul>
    </dialog>
  );
}
