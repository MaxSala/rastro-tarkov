import { Search } from "lucide-react";
import { useEffect, useMemo, useRef, useState } from "react";
import { t } from "../../locales";
import { CATEGORIES, inCategory, itemById, type Category, type Catalog, type Item } from "../../lib/catalog";
import { useApp } from "../../store/app";
import { cn } from "../../lib/cn";
import { percent, rub } from "../../lib/format";
import { iconSm } from "../../lib/icons";
import { fold, matches } from "../../lib/search";
import { SortHeader, type SortDir } from "../ui/sortHeader";
import { DataGate, DataStatus } from "./DataGate";
import { ItemDetail } from "./ItemDetail";
import { ItemIcon } from "./ItemIcon";

type Key = "name" | "bestSell" | "flea" | "avg" | "change" | "perSlot";
const MAX_ROWS = 300;
const value: Record<Key, (i: Item) => number | string> = {
  name: (i) => i.name,
  bestSell: (i) => i.bestSell?.priceRub ?? -1,
  flea: (i) => i.fleaLow ?? -1,
  avg: (i) => i.fleaAvg ?? -1,
  change: (i) => i.change48h ?? -Infinity,
  perSlot: (i) => i.perSlot ?? -1,
};

/** Relevancia: abreviatura exacta, luego nombre que empieza igual, luego el resto. */
function rank(i: Item, q: string): number {
  if (fold(i.short) === q) return 0;
  if (fold(i.name).startsWith(q) || fold(i.short).startsWith(q)) return 1;
  return 2;
}

export function MarketView() {
  return <DataGate>{(c) => <Market catalog={c} />}</DataGate>;
}

function Market({ catalog }: { catalog: Catalog }) {
  // Si llegamos desde la tarjeta de una cerradura, abrir ya ese objeto (buscado por su nombre).
  const [incoming] = useState(() => itemById(catalog, useApp.getState().marketItem ?? undefined));
  useEffect(() => {
    if (useApp.getState().marketItem) useApp.getState().showInMarket(null);
  }, []);
  const [query, setQuery] = useState(incoming?.name ?? "");
  const [category, setCategory] = useState<Category>("all");
  const [sort, setSort] = useState<{ key: Key; dir: SortDir }>({ key: "perSlot", dir: "desc" });
  const [sortTouched, setSortTouched] = useState(false);
  const [selected, setSelected] = useState<string | null>(incoming?.id ?? null);
  const input = useRef<HTMLInputElement>(null);
  const body = useRef<HTMLTableSectionElement>(null);

  const results = useMemo(() => {
    const q = fold(query.trim());
    const list = catalog.items.filter((i) => inCategory(i, category) && matches(query, i.name, i.short));
    const dir = sort.dir === "asc" ? 1 : -1;
    const get = value[sort.key];
    list.sort((a, b) => {
      if (q && !sortTouched) {
        const r = rank(a, q) - rank(b, q);
        if (r !== 0) return r;
      }
      const va = get(a);
      const vb = get(b);
      return (typeof va === "string" ? va.localeCompare(vb as string) : va - (vb as number)) * dir;
    });
    return list;
  }, [catalog, query, category, sort, sortTouched]);

  const shown = results.slice(0, MAX_ROWS);
  const current = catalog.items.find((i) => i.id === selected) ?? null;

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (
        e.key === "/" &&
        document.activeElement !== input.current &&
        !document.querySelector("dialog[open]")
      ) {
        e.preventDefault();
        input.current?.focus();
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, []);

  const move = (d: number) => {
    const idx = shown.findIndex((i) => i.id === selected);
    const next = shown[Math.max(0, Math.min(shown.length - 1, idx + d))];
    if (!next) return;
    setSelected(next.id);
    body.current?.querySelector(`[data-id="${next.id}"]`)?.scrollIntoView({ block: "nearest" });
  };
  const onSort = (k: Key) => {
    setSortTouched(true);
    setSort((s) =>
      s.key === k
        ? { key: k, dir: s.dir === "asc" ? "desc" : "asc" }
        : { key: k, dir: k === "name" ? "asc" : "desc" },
    );
  };
  const cols: [Key, string][] = [
    ["bestSell", t.market.cols.bestSell],
    ["flea", t.market.cols.flea],
    ["avg", t.market.cols.avg],
    ["change", t.market.cols.change],
    ["perSlot", t.market.cols.perSlot],
  ];

  return (
    <div className="grid h-full min-h-0 grid-cols-[minmax(0,1fr)_360px]">
      <section className="flex min-h-0 min-w-0 flex-col border-r border-line">
        <div className="grid gap-3 border-b border-line px-4 pt-4 pb-3">
          <div className="flex items-center gap-3">
            <div className="relative min-w-0 flex-1">
              <Search
                {...iconSm}
                className="pointer-events-none absolute top-1/2 left-3 -translate-y-1/2 text-fg-muted"
              />
              <label htmlFor="market-search" className="sr-only">
                {t.market.search}
              </label>
              <input
                ref={input}
                id="market-search"
                type="search"
                autoFocus
                autoComplete="off"
                spellCheck={false}
                value={query}
                placeholder={`${t.market.search} — ${t.market.searchHint}`}
                onChange={(e) => {
                  setQuery(e.target.value);
                  setSortTouched(false);
                }}
                onKeyDown={(e) => {
                  // «/» con la caja ya enfocada y vacía es el atajo, no texto.
                  if (e.key === "/" && !e.currentTarget.value) e.preventDefault();
                  if (e.key === "ArrowDown" || e.key === "ArrowUp") {
                    e.preventDefault();
                    move(e.key === "ArrowDown" ? 1 : -1);
                  }
                }}
                className="h-10 w-full rounded-md border border-line-strong bg-app pr-12 pl-9 text-base placeholder:text-placeholder focus-visible:border-accent focus-visible:outline-none"
              />
              <kbd className="num pointer-events-none absolute top-1/2 right-2.5 -translate-y-1/2 rounded-sm border border-line-strong px-1.5 text-2xs text-fg-muted">
                /
              </kbd>
            </div>
            <DataStatus />
          </div>
          <div role="radiogroup" aria-label={t.workspace.market} className="flex flex-wrap gap-1">
            {(Object.keys(CATEGORIES) as Category[]).map((c) => (
              <button
                key={c}
                type="button"
                role="radio"
                aria-checked={category === c}
                onClick={() => setCategory(c)}
                className={cn(
                  "h-7 rounded-sm border px-2.5 text-xs transition-colors",
                  category === c
                    ? "border-line-strong bg-active text-fg"
                    : "border-transparent text-fg-muted hover:text-fg",
                )}
              >
                {t.market.categories[c]}
              </button>
            ))}
          </div>
        </div>
        <div className="min-h-0 flex-1 overflow-auto">
          {shown.length === 0 ? (
            <p className="px-4 py-10 text-center text-sm text-fg-muted">{t.market.noResults}</p>
          ) : (
            <table className="w-full table-fixed border-separate border-spacing-0 text-sm">
              <colgroup>
                <col />
                <col className="w-32" />
                <col className="w-28" />
                <col className="w-28" />
                <col className="w-20" />
                <col className="w-28" />
              </colgroup>
              <thead>
                <tr>
                  <SortHeader
                    label={t.market.cols.item}
                    k="name"
                    sort={sort}
                    onSort={onSort}
                    align="left"
                    className="pl-1"
                  />
                  {cols.map(([k, label]) => (
                    <SortHeader key={k} label={label} k={k} sort={sort} onSort={onSort} />
                  ))}
                </tr>
              </thead>
              <tbody ref={body}>
                {shown.map((i) => (
                  <Row key={i.id} item={i} selected={i.id === selected} onSelect={() => setSelected(i.id)} />
                ))}
              </tbody>
            </table>
          )}
        </div>
        <footer className="num border-t border-line px-4 py-1.5 text-2xs text-fg-muted">
          {t.market.shown(shown.length, results.length)}
        </footer>
      </section>
      <aside aria-live="polite" className="min-h-0 overflow-x-hidden overflow-y-auto bg-surface">
        <ItemDetail item={current} />
      </aside>
    </div>
  );
}

function Row({ item, selected, onSelect }: { item: Item; selected: boolean; onSelect: () => void }) {
  const change = item.change48h;
  const cell = "border-b border-line px-3 text-right num whitespace-nowrap";
  return (
    <tr
      data-id={item.id}
      aria-selected={selected}
      onClick={onSelect}
      className={cn("h-11 cursor-default", selected ? "bg-accent-soft" : "hover:bg-hover")}
    >
      <td
        className={cn(
          "border-b border-line py-1 pr-3 pl-3",
          selected && "shadow-[inset_2px_0_0_var(--accent)]",
        )}
      >
        <span className="flex min-w-0 items-center gap-2.5">
          <ItemIcon id={item.id} size={32} />
          <span className="min-w-0">
            <span className="block truncate">{item.name}</span>
            <span className="num block truncate text-2xs text-fg-muted">{item.short}</span>
          </span>
        </span>
      </td>
      <td className={cell}>
        {rub(item.bestSell?.priceRub)}
        {item.bestSell && (
          <span className="block truncate font-sans text-2xs text-fg-muted">
            {item.bestSell.vendor === "flea" ? t.market.flea : item.bestSell.vendorName}
          </span>
        )}
      </td>
      <td className={cell}>{rub(item.fleaLow)}</td>
      <td className={cn(cell, "text-fg-soft")}>{rub(item.fleaAvg)}</td>
      <td
        className={cn(
          cell,
          change == null
            ? "text-fg-muted"
            : change > 0
              ? "text-ok"
              : change < 0
                ? "text-danger"
                : "text-fg-muted",
        )}
      >
        {change == null ? "—" : percent(change)}
      </td>
      <td className={cn(cell, "text-fg")}>{rub(item.perSlot)}</td>
    </tr>
  );
}
