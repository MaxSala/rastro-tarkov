import { CloudDownload, Coins, Crosshair, X, Zap } from "lucide-react";
import { useEffect, useLayoutEffect, useRef, useState } from "react";
import { t } from "../locales";
import { itemById, type Offer } from "../lib/catalog";
import { rub } from "../lib/format";
import { iconSm } from "../lib/icons";
import { useApp } from "../store/app";
import { Button } from "../components/ui/button";
import { ItemIcon } from "../components/market/ItemIcon";
import { refreshMarket, useAutoCatalog } from "../components/market/DataGate";
import { keyName, lockType } from "../components/map/locks";

function Price({ label, offer }: { label: string; offer: Offer | null | undefined }) {
  const vendor = offer ? (offer.vendor === "flea" ? t.market.flea : offer.vendorName) : null;
  return (
    <div className="grid gap-0.5 bg-surface px-3 py-2">
      <dt className="text-2xs text-fg-muted">{label}</dt>
      <dd className="num text-sm">{offer ? rub(offer.priceRub) : t.keys.noPrice}</dd>
      {vendor && (
        <dd className="truncate text-2xs text-fg-muted">
          {vendor}
          {offer?.level ? ` · ${t.market.level(offer.level)}` : ""}
        </dd>
      )}
    </div>
  );
}

/** Tarjeta de una cerradura: qué llave la abre, cuánto cuesta y atajos al Mercado o a resaltarla. */
export function KeyCard() {
  const card = useApp((s) => s.keyCard);
  const setCard = useApp((s) => s.setKeyCard);
  const focusKey = useApp((s) => s.focusKey);
  const setFocusKey = useApp((s) => s.setFocusKey);
  const showInMarket = useApp((s) => s.showInMarket);
  const { catalog, market } = useAutoCatalog(card !== null);
  const ref = useRef<HTMLDivElement>(null);
  const [pos, setPos] = useState({ left: 0, top: 0 });

  useEffect(() => {
    if (!card) return;
    const close = (e: Event) => {
      if (e instanceof PointerEvent && ref.current?.contains(e.target as Node)) return;
      if (e instanceof KeyboardEvent && e.key !== "Escape") return;
      setCard(null);
    };
    window.addEventListener("pointerdown", close, true);
    window.addEventListener("keydown", close);
    window.addEventListener("resize", close);
    return () => {
      window.removeEventListener("pointerdown", close, true);
      window.removeEventListener("keydown", close);
      window.removeEventListener("resize", close);
    };
  }, [card, setCard]);

  // Dentro de la ventana, junto al candado (a la derecha y centrada; si no cabe, a la izquierda).
  useLayoutEffect(() => {
    const el = ref.current;
    if (!card || !el) return;
    const r = el.getBoundingClientRect();
    const right = card.x + 16;
    setPos({
      left: right + r.width > window.innerWidth - 8 ? Math.max(8, card.x - r.width - 16) : right,
      top: Math.min(Math.max(8, card.y - r.height / 2), window.innerHeight - r.height - 8),
    });
  }, [card, catalog]);

  if (!card) return null;
  const item = itemById(catalog, card.key);
  const name = keyName(card.key, card.name, catalog);
  const focused = focusKey === card.key;
  return (
    <div
      ref={ref}
      role="dialog"
      aria-label={name}
      style={{ left: pos.left, top: pos.top }}
      className="fixed z-[1000] grid w-80 gap-3 rounded-md border border-line-strong bg-surface p-3 shadow-[0_8px_32px_#0008]"
    >
      <header className="flex items-start gap-3">
        <ItemIcon id={card.key} size={48} />
        <div className="min-w-0 flex-1">
          <h2 className="text-sm leading-snug font-medium break-words">{name}</h2>
          <p className="mt-1 flex flex-wrap items-center gap-1.5 text-2xs text-fg-muted">
            <span className="rounded-sm border border-line-strong px-1.5 py-px">
              {t.keys.types[lockType(card.group)]}
            </span>
            {card.power && (
              <span className="inline-flex items-center gap-1 rounded-sm border border-[#e5c84a55] px-1.5 py-px text-[#e5c84a]">
                <Zap size={11} strokeWidth={2} aria-hidden />
                {t.keys.needsPower}
              </span>
            )}
          </p>
        </div>
        <button
          type="button"
          aria-label={t.a11y.close}
          onClick={() => setCard(null)}
          className="-mt-1 -mr-1 grid size-7 place-items-center rounded-sm text-fg-muted hover:bg-hover hover:text-fg"
        >
          <X {...iconSm} />
        </button>
      </header>
      {item ? (
        <dl className="grid grid-cols-2 gap-px overflow-hidden rounded-sm border border-line bg-line">
          <Price label={t.keys.buy} offer={item.cheapestBuy} />
          <Price label={t.keys.sell} offer={item.bestSell} />
        </dl>
      ) : !catalog && market?.state !== "loading" ? (
        <div className="grid justify-items-start gap-2 rounded-sm border border-line bg-app p-3">
          <p className="text-xs text-fg-soft">{t.keys.noCatalog}</p>
          <Button size="sm" onClick={refreshMarket}>
            <CloudDownload {...iconSm} />
            {t.keys.download}
          </Button>
        </div>
      ) : null}
      <div className="flex flex-wrap gap-2">
        <Button
          size="sm"
          variant={focused ? "primary" : undefined}
          onClick={() => setFocusKey(focused ? null : card.key)}
        >
          <Crosshair {...iconSm} />
          {focused ? t.keys.clearFocus : t.keys.highlight}
        </Button>
        {item && (
          <Button size="sm" onClick={() => showInMarket(card.key)}>
            <Coins {...iconSm} />
            {t.keys.inMarket}
          </Button>
        )}
      </div>
    </div>
  );
}
