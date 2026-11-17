import { Search } from "lucide-react";
import { useMemo, useState } from "react";
import { t } from "../../locales";
import type { Catalog, Item } from "../../lib/catalog";
import { cn } from "../../lib/cn";
import { decimal, int, rub } from "../../lib/format";
import { iconSm } from "../../lib/icons";
import { matches } from "../../lib/search";
import { DataGate, DataStatus } from "../market/DataGate";
import { ItemIcon } from "../market/ItemIcon";
import { SortHeader, type SortDir } from "../ui/sortHeader";

type Round = Item & { ammo: NonNullable<Item["ammo"]> };
type Key = "name" | "damage" | "pen" | "armor" | "frag" | "speed" | "price";
const ALL = "*";

const value: Record<Key, (r: Round) => number | string> = {
  name: (r) => r.name,
  damage: (r) => r.ammo.damage * r.ammo.projectiles,
  pen: (r) => r.ammo.penetration,
  armor: (r) => r.ammo.armorDamage,
  frag: (r) => r.ammo.fragmentation,
  speed: (r) => r.ammo.speed,
  price: (r) => r.cheapestBuy?.priceRub ?? Number.MAX_SAFE_INTEGER,
};

export function AmmoView() {
  return <DataGate>{(c) => <Ammo catalog={c} />}</DataGate>;
}

/** Barra horizontal fina: valor relativo al máximo de la tabla. */
function Bar({ v, max, strong }: { v: number; max: number; strong?: boolean }) {
  const w = max > 0 ? Math.max(2, Math.min(100, (v / max) * 100)) : 0;
  return (
    <span aria-hidden className="block h-1 w-full overflow-hidden rounded-full bg-active">
      <span
        className={cn("block h-full rounded-full", strong ? "bg-accent" : "bg-fg-muted")}
        style={{ width: `${w}%` }}
      />
    </span>
  );
}

function Ammo({ catalog }: { catalog: Catalog }) {
  const rounds = useMemo(() => catalog.items.filter((i): i is Round => Boolean(i.ammo)), [catalog]);
  const calibers = useMemo(() => {
    const m = new Map<string, number>();
    for (const r of rounds) m.set(r.ammo.caliber, (m.get(r.ammo.caliber) ?? 0) + 1);
    return [...m.entries()].sort((a, b) => a[0].localeCompare(b[0], undefined, { numeric: true }));
  }, [rounds]);
  const [caliber, setCaliber] = useState<string>(ALL);
  const [query, setQuery] = useState("");
  const [sort, setSort] = useState<{ key: Key; dir: SortDir }>({ key: "pen", dir: "desc" });

  const list = useMemo(() => {
    const out = rounds.filter(
      (r) =>
        (caliber === ALL || r.ammo.caliber === caliber) && matches(query, r.name, r.short, r.ammo.caliber),
    );
    const get = value[sort.key];
    const dir = sort.dir === "asc" ? 1 : -1;
    return out.sort((a, b) => {
      const va = get(a);
      const vb = get(b);
      return (typeof va === "string" ? va.localeCompare(vb as string) : va - (vb as number)) * dir;
    });
  }, [rounds, caliber, query, sort]);

  const maxPen = Math.max(1, ...list.map((r) => r.ammo.penetration));
  const maxDmg = Math.max(1, ...list.map((r) => r.ammo.damage * r.ammo.projectiles));
  const onSort = (k: Key) =>
    setSort((s) =>
      s.key === k
        ? { key: k, dir: s.dir === "asc" ? "desc" : "asc" }
        : { key: k, dir: k === "name" || k === "price" ? "asc" : "desc" },
    );
  const num = "border-b border-line px-3 text-right num whitespace-nowrap";

  return (
    <div className="grid h-full min-h-0 grid-cols-[220px_minmax(0,1fr)]">
      <nav aria-label={t.ammo.calibers} className="flex min-h-0 flex-col border-r border-line bg-surface">
        <h2 className="px-3 pt-3 pb-1.5 text-xs font-medium text-fg-muted">{t.ammo.calibers}</h2>
        <ul className="min-h-0 flex-1 overflow-x-hidden overflow-y-auto pb-2">
          {[[ALL, rounds.length] as [string, number], ...calibers].map(([c, n]) => (
            <li key={c}>
              <button
                type="button"
                aria-pressed={caliber === c}
                onClick={() => setCaliber(c)}
                className={cn(
                  "flex h-8 w-full items-center justify-between gap-2 border-l-2 px-3 text-left text-sm",
                  caliber === c
                    ? "border-accent bg-accent-soft text-fg"
                    : "border-transparent text-fg-soft hover:bg-hover hover:text-fg",
                )}
              >
                <span className="truncate">{c === ALL ? t.ammo.all : c}</span>
                <span className="num shrink-0 text-2xs text-fg-muted">{n}</span>
              </button>
            </li>
          ))}
        </ul>
      </nav>
      <section className="flex min-h-0 min-w-0 flex-col">
        <div className="flex items-center gap-3 border-b border-line px-4 py-3">
          <div className="relative w-72 max-w-full">
            <Search
              {...iconSm}
              className="pointer-events-none absolute top-1/2 left-2.5 -translate-y-1/2 text-fg-muted"
            />
            <label htmlFor="ammo-search" className="sr-only">
              {t.ammo.search}
            </label>
            <input
              id="ammo-search"
              type="search"
              value={query}
              autoComplete="off"
              spellCheck={false}
              placeholder={t.ammo.search}
              onChange={(e) => setQuery(e.target.value)}
              className="h-8 w-full rounded-sm border border-line-strong bg-app pr-2 pl-8 text-sm placeholder:text-placeholder focus-visible:border-accent focus-visible:outline-none"
            />
          </div>
          <h2 className="min-w-0 flex-1 truncate text-sm">
            <span className="font-medium">{caliber === ALL ? t.ammo.all : caliber}</span>{" "}
            <span className="text-fg-muted">{t.ammo.count(list.length)}</span>
          </h2>
          <DataStatus />
        </div>
        <div className="min-h-0 flex-1 overflow-auto">
          {list.length === 0 ? (
            <p className="px-4 py-10 text-center text-sm text-fg-muted">{t.ammo.noResults}</p>
          ) : (
            <table className="w-full min-w-[820px] table-fixed border-separate border-spacing-0 text-sm">
              <colgroup>
                <col />
                <col className="w-32" />
                <col className="w-36" />
                <col className="w-28" />
                <col className="w-28" />
                <col className="w-24" />
                <col className="w-28" />
              </colgroup>
              <thead>
                <tr>
                  <SortHeader label={t.ammo.cols.name} k="name" sort={sort} onSort={onSort} align="left" />
                  <SortHeader label={t.ammo.cols.damage} k="damage" sort={sort} onSort={onSort} />
                  <SortHeader label={t.ammo.cols.pen} k="pen" sort={sort} onSort={onSort} />
                  <SortHeader label={t.ammo.cols.armor} k="armor" sort={sort} onSort={onSort} />
                  <SortHeader label={t.ammo.cols.frag} k="frag" sort={sort} onSort={onSort} />
                  <SortHeader label={t.ammo.cols.speed} k="speed" sort={sort} onSort={onSort} />
                  <SortHeader label={t.ammo.cols.price} k="price" sort={sort} onSort={onSort} />
                </tr>
              </thead>
              <tbody>
                {list.map((r) => {
                  const dmg = r.ammo.damage * r.ammo.projectiles;
                  return (
                    <tr key={r.id} className="h-11 hover:bg-hover">
                      <td className="border-b border-line px-3 py-1">
                        <span className="flex min-w-0 items-center gap-2.5">
                          <ItemIcon id={r.id} size={28} />
                          <span className="min-w-0">
                            <span className="block truncate">{r.name}</span>
                            <span className="flex gap-2 text-2xs text-fg-muted">
                              {caliber === ALL && <span className="truncate">{r.ammo.caliber}</span>}
                              {r.ammo.tracer && <span className="shrink-0 text-accent">{t.ammo.tracer}</span>}
                            </span>
                          </span>
                        </span>
                      </td>
                      <td
                        className={num}
                        title={r.ammo.projectiles > 1 ? t.ammo.projectiles(r.ammo.projectiles) : undefined}
                      >
                        <span className="grid gap-1">
                          <span>
                            {r.ammo.projectiles > 1
                              ? `${int(r.ammo.damage)}×${r.ammo.projectiles}`
                              : int(dmg)}
                          </span>
                          <Bar v={dmg} max={maxDmg} />
                        </span>
                      </td>
                      <td className={num}>
                        <span className="grid gap-1">
                          <span className="text-fg">{int(r.ammo.penetration)}</span>
                          <Bar v={r.ammo.penetration} max={maxPen} strong />
                        </span>
                      </td>
                      <td className={num}>{int(r.ammo.armorDamage)} %</td>
                      <td className={num}>{decimal(r.ammo.fragmentation)} %</td>
                      <td className={cn(num, "text-fg-soft")}>{int(r.ammo.speed)} m/s</td>
                      <td className={num}>
                        {r.cheapestBuy ? (
                          rub(r.cheapestBuy.priceRub)
                        ) : (
                          <span className="text-fg-muted">{t.ammo.noPrice}</span>
                        )}
                        {r.cheapestBuy && (
                          <span className="block truncate font-sans text-2xs text-fg-muted">
                            {r.cheapestBuy.vendor === "flea" ? t.market.flea : r.cheapestBuy.vendorName}
                            {r.cheapestBuy.level ? ` · ${r.cheapestBuy.level}` : ""}
                          </span>
                        )}
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          )}
        </div>
        <footer className="border-t border-line px-4 py-1.5 text-2xs text-fg-muted">{t.ammo.sortHint}</footer>
      </section>
    </div>
  );
}
