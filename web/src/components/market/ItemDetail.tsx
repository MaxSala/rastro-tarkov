import { Copy, ExternalLink, PackageSearch } from "lucide-react";
import { t } from "../../locales";
import type { Item, Offer } from "../../lib/catalog";
import { money, rub } from "../../lib/format";
import { iconSm } from "../../lib/icons";
import { useApp } from "../../store/app";
import { Button } from "../ui/button";
import { ItemIcon } from "./ItemIcon";
import { KeyLocations } from "./KeyLocations";
import { QuestNeeds } from "./QuestNeeds";

function Stat({ label, value, sub }: { label: string; value: string; sub?: string }) {
  return (
    <div className="grid gap-0.5 bg-surface px-3 py-2.5">
      <dt className="text-2xs text-fg-muted">{label}</dt>
      <dd className="num text-sm">{value}</dd>
      {sub && <dd className="truncate text-2xs text-fg-muted">{sub}</dd>}
    </div>
  );
}

function OfferList({ title, offers, best }: { title: string; offers: Offer[]; best?: Offer | null }) {
  return (
    <section className="grid gap-1">
      <h3 className="text-xs font-medium text-fg-muted">{title}</h3>
      {offers.length === 0 ? (
        <p className="text-xs text-fg-muted">{t.market.noOffers}</p>
      ) : (
        <ul className="grid gap-px overflow-hidden rounded-sm border border-line bg-line">
          {offers.map((o, i) => (
            <li
              key={`${o.vendor}-${i}`}
              className="flex items-center justify-between gap-3 bg-app px-2.5 py-1.5 text-sm"
            >
              <span className="flex min-w-0 items-center gap-2">
                <span
                  className={o === best ? "size-1.5 shrink-0 rounded-full bg-accent" : "size-1.5 shrink-0"}
                />
                <span className="truncate">{o.vendor === "flea" ? t.market.flea : o.vendorName}</span>
                {o.level && (
                  <span className="shrink-0 text-2xs text-fg-muted">{t.market.level(o.level)}</span>
                )}
              </span>
              <span className="num shrink-0 text-right">
                {rub(o.priceRub)}
                {o.original && (
                  <span className="block text-2xs text-fg-muted">{money(o.original[0], o.original[1])}</span>
                )}
              </span>
            </li>
          ))}
        </ul>
      )}
    </section>
  );
}

export function ItemDetail({ item }: { item: Item | null }) {
  const notify = useApp((s) => s.notify);
  if (!item)
    return (
      <div className="grid h-full place-items-center p-6 text-center">
        <div className="grid justify-items-center gap-2">
          <PackageSearch size={20} strokeWidth={1.5} className="text-fg-muted" />
          <p className="max-w-56 text-sm text-fg-muted">{t.market.pick}</p>
        </div>
      </div>
    );
  const slots = item.width * item.height;
  const noFlea = item.types.includes("noFlea");
  return (
    <div className="grid content-start gap-4 p-4">
      <header className="flex items-start gap-3">
        <ItemIcon id={item.id} size={64} />
        <div className="min-w-0">
          <h2 className="text-base leading-snug font-medium break-words">{item.name}</h2>
          <p className="mt-0.5 flex flex-wrap items-center gap-x-2 text-xs text-fg-muted">
            <span className="num">{item.short}</span>
            <span>
              {item.width}×{item.height} · {t.market.slots(slots)}
            </span>
          </p>
        </div>
      </header>
      <dl className="grid grid-cols-2 gap-px overflow-hidden rounded-sm border border-line bg-line">
        <Stat
          label={t.market.cols.bestSell}
          value={rub(item.bestSell?.priceRub)}
          sub={
            item.bestSell
              ? item.bestSell.vendor === "flea"
                ? t.market.flea
                : item.bestSell.vendorName
              : undefined
          }
        />
        <Stat label={t.market.cols.perSlot} value={rub(item.perSlot)} />
        <Stat
          label={t.market.cols.flea}
          value={noFlea ? "—" : rub(item.fleaLow)}
          sub={noFlea ? t.market.noFlea : undefined}
        />
        <Stat label={t.market.cols.avg} value={noFlea ? "—" : rub(item.fleaAvg)} />
      </dl>
      <OfferList title={t.market.sellTo} offers={item.sell} best={item.bestSell} />
      <OfferList title={t.market.buyFrom} offers={item.buy} />
      <QuestNeeds id={item.id} />
      {item.types.includes("keys") && <KeyLocations id={item.id} />}
      <p className="text-2xs text-fg-muted">{t.market.feeNote}</p>
      <div className="flex flex-wrap gap-2">
        {item.slug && (
          <a
            href={`https://tarkov.dev/item/${item.slug}`}
            target="_blank"
            rel="noreferrer"
            className="inline-flex h-7 items-center gap-1.5 rounded-sm border border-line-strong bg-raised px-2 text-xs font-medium hover:bg-hover"
          >
            <ExternalLink {...iconSm} />
            {t.market.openWeb}
          </a>
        )}
        <Button
          size="sm"
          onClick={() =>
            navigator.clipboard.writeText(item.name).then(
              () => notify("info", t.market.copied),
              () => notify("error", t.errors.generic),
            )
          }
        >
          <Copy {...iconSm} />
          {t.market.copyName}
        </Button>
      </div>
    </div>
  );
}
