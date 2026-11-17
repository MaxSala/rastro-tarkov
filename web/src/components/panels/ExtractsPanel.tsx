import { fold } from "../../lib/search";
import { Search } from "lucide-react";
import { useMemo, useState } from "react";
import { t } from "../../locales";
import { cn } from "../../lib/cn";
import { meters } from "../../lib/format";
import { iconUrl } from "../../lib/iconCache";
import { iconSm } from "../../lib/icons";
import type { ExtractView, Faction } from "../../lib/protocol";
import { send } from "../../lib/socket";
import { useApp } from "../../store/app";
import { Segmented } from "../ui/segmented";
import { Switch } from "../ui/switch";
import { PanelFrame, PanelNote } from "./PanelFrame";
import { extractNote } from "../../lib/mapText";

type Filter = "all" | Faction;
const filters: { value: Filter; label: string }[] = [
  { value: "all", label: t.extracts.all },
  { value: "pmc", label: t.extracts.pmc },
  { value: "scav", label: t.extracts.scav },
  { value: "shared", label: t.extracts.shared },
];

/** Icono de tarkov.dev si está descargado; si no, la forma: círculo PMC, rombo scav, cuadrado compartida. */
function Shape({ faction }: { faction: Faction }) {
  useApp((s) => s.iconsVersion);
  const url = iconUrl(`extract_${faction}`);
  if (url) return <img src={url} alt="" className="size-4 shrink-0" />;
  return (
    <span
      aria-hidden
      className={cn(
        "size-2.5 shrink-0 border-[1.5px] border-fg-soft",
        faction === "pmc" && "rounded-full",
        faction === "scav" && "rotate-45 scale-90",
      )}
    />
  );
}

function Row({ e, selected }: { e: ExtractView; selected: boolean }) {
  return (
    <li
      className={cn(
        "group flex items-center gap-2 border-l-2 pr-3 pl-2.5",
        selected ? "border-accent bg-accent-soft" : "border-transparent hover:bg-hover",
        e.dimmed && "opacity-50",
      )}
    >
      <button
        type="button"
        aria-pressed={selected}
        onClick={() => send({ type: "selectExtract", extractId: selected ? null : e.id })}
        className="flex min-w-0 flex-1 items-center gap-2.5 py-2 text-left"
        title={extractNote(e)}
      >
        <Shape faction={e.faction} />
        <span className="min-w-0 flex-1">
          <span className="block truncate text-sm">{e.name}</span>
          {extractNote(e) && <span className="block truncate text-xs text-fg-muted">{extractNote(e)}</span>}
        </span>
        {e.distanceM !== null && (
          <span className="num shrink-0 text-xs text-fg-soft">{meters(e.distanceM)}</span>
        )}
      </button>
      <Switch
        checked={e.available}
        label={`${t.extracts.available}: ${e.name}`}
        onChange={() => send({ type: "toggleExtract", extractId: e.id })}
      />
    </li>
  );
}

export function ExtractsPanel() {
  const view = useApp((s) => s.view);
  const [query, setQuery] = useState("");
  const [filter, setFilter] = useState<Filter>("all");
  const extracts = useMemo(() => view?.extracts ?? [], [view?.extracts]);
  const shown = useMemo(() => {
    const q = fold(query.trim());
    return extracts.filter(
      (e) => (filter === "all" || e.faction === filter) && (!q || fold(e.name).includes(q)),
    );
  }, [extracts, query, filter]);

  return (
    <PanelFrame title={t.panels.extracts}>
      <div className="grid gap-2 border-b border-line p-3">
        <div className="relative">
          <Search {...iconSm} className="pointer-events-none absolute top-2 left-2.5 text-fg-muted" />
          <label htmlFor="extract-search" className="sr-only">
            {t.extracts.search}
          </label>
          <input
            id="extract-search"
            type="search"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder={t.extracts.search}
            className="h-8 w-full rounded-sm border border-line-strong bg-app pr-2 pl-8 text-sm placeholder:text-placeholder focus-visible:border-accent focus-visible:outline-none"
          />
        </div>
        <Segmented label={t.extracts.all} value={filter} options={filters} onChange={setFilter} />
        <p className="text-xs text-fg-muted">{t.extracts.availableHint}</p>
      </div>
      {extracts.length === 0 ? (
        <PanelNote>{t.extracts.none}</PanelNote>
      ) : shown.length === 0 ? (
        <PanelNote>{t.extracts.noMatch}</PanelNote>
      ) : (
        <ul className="py-1">
          {shown.map((e) => (
            <Row key={e.id} e={e} selected={e.id === view?.selectedExtract} />
          ))}
        </ul>
      )}
    </PanelFrame>
  );
}
