import { locale, t } from "../locales";

const nf0 = new Intl.NumberFormat(locale, { maximumFractionDigits: 0 });
const nf1 = new Intl.NumberFormat(locale, { minimumFractionDigits: 1, maximumFractionDigits: 1 });
const pct = new Intl.NumberFormat(locale, { maximumFractionDigits: 1, signDisplay: "exceptZero" });

export const meters = (m: number) => `${nf0.format(m)} m`;
export const signedMeters = (m: number) => `${m >= 0 ? "+" : "−"}${nf0.format(Math.abs(m))} m`;
export const coord = (v: number) => (v < 0 ? "−" : "") + nf1.format(Math.abs(v));
export const int = (v: number) => nf0.format(v);
export const decimal = (v: number) => nf1.format(v);
export const percent = (v: number) => `${pct.format(v)} %`;

/** Precio en rublos con separador de miles del idioma: «45.300 ₽» / «45,300 ₽». */
export const rub = (v: number | null | undefined) => (v == null ? "—" : `${nf0.format(v)} ₽`);

/** Moneda original de una oferta (USD/EUR) con su símbolo. */
export function money(value: number, currency: string): string {
  const symbol = currency === "USD" ? "$" : currency === "EUR" ? "€" : currency;
  return `${nf0.format(value)} ${symbol}`;
}

export function relativeBearing(deg: number): string {
  const a = Math.round(Math.abs(deg));
  if (a <= 5) return t.extracts.ahead;
  return `${a}° ${deg < 0 ? t.extracts.left : t.extracts.right}`;
}

/** Horas de raid (11.67) → "11:40". */
export function raidClock(hours: number): string {
  const total = Math.round(hours * 60);
  return `${Math.floor(total / 60)}:${String(total % 60).padStart(2, "0")}`;
}

/** Hace cuánto (segundos Unix) en palabras: «hace 12 min». */
export function ago(unixSeconds: number, now = Date.now()): string {
  const s = Math.max(0, Math.floor(now / 1000 - unixSeconds));
  if (s < 60) return t.ago.now;
  if (s < 3600) return t.ago.min(Math.floor(s / 60));
  if (s < 86400) return t.ago.h(Math.floor(s / 3600));
  return t.ago.d(Math.floor(s / 86400));
}
