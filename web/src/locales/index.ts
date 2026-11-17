// Idioma de la interfaz: los 16 de tarkov.dev. Se elige al cargar (preferencia guardada o idioma
// del sistema) y solo se descarga ese idioma: Vite separa cada carpeta en su propio archivo.
// Al cambiarlo se recarga la interfaz, así cada componente lee los textos una sola vez, sin estado.
import type { Main } from "./es/main";
import type { Game } from "./es/game";
import type { Tools } from "./es/tools";

export const LANGS = [
  "cs",
  "de",
  "en",
  "es",
  "fr",
  "hu",
  "it",
  "ja",
  "ko",
  "pl",
  "pt",
  "ro",
  "ru",
  "sk",
  "tr",
  "zh",
] as const;
export type Lang = (typeof LANGS)[number];

/** Nombre de cada idioma en su propia lengua: se muestra igual sea cual sea el idioma activo. */
export const NATIVE_NAMES: Record<Lang, string> = {
  cs: "Čeština",
  de: "Deutsch",
  en: "English",
  es: "Español",
  fr: "Français",
  hu: "Magyar",
  it: "Italiano",
  ja: "日本語",
  ko: "한국어",
  pl: "Polski",
  pt: "Português",
  ro: "Română",
  ru: "Русский",
  sk: "Slovenčina",
  tr: "Türkçe",
  zh: "中文",
};

const LOCALES: Record<Lang, string> = {
  cs: "cs-CZ",
  de: "de-DE",
  en: "en-GB",
  es: "es-ES",
  fr: "fr-FR",
  hu: "hu-HU",
  it: "it-IT",
  ja: "ja-JP",
  ko: "ko-KR",
  pl: "pl-PL",
  pt: "pt-BR",
  ro: "ro-RO",
  ru: "ru-RU",
  sk: "sk-SK",
  tr: "tr-TR",
  zh: "zh-CN",
};

export type Strings = Main & { game: Game } & Tools;
const KEY = "rastro.lang";
const isLang = (v: unknown): v is Lang => typeof v === "string" && (LANGS as readonly string[]).includes(v);

function detect(): Lang {
  try {
    const saved = localStorage.getItem(KEY);
    if (isLang(saved)) return saved;
  } catch {
    /* almacenamiento no disponible */
  }
  for (const pref of navigator.languages) {
    const base = pref.toLowerCase().split("-")[0];
    if (isLang(base)) return base;
  }
  return "en";
}

type Part = { default: unknown };
const files = import.meta.glob<Part>("./*/{main,game,tools}.ts");

async function load(l: Lang): Promise<Strings> {
  const part = (f: string) => files[`./${l}/${f}.ts`]?.() ?? Promise.reject(new Error(f));
  const [main, game, tools] = await Promise.all([part("main"), part("game"), part("tools")]);
  // Cada archivo ya está tipado contra el español: aquí solo se juntan.
  return { ...(main.default as Main), game: game.default as Game, ...(tools.default as Tools) };
}

const detected = detect();
const loaded = await load(detected).then(
  (s) => [detected, s] as const,
  async () => ["en", await load("en")] as const,
);

export const lang: Lang = loaded[0];
export const locale = LOCALES[lang];
export const t: Strings = loaded[1];

export function setLang(next: Lang): void {
  try {
    localStorage.setItem(KEY, next);
  } catch {
    /* sin almacenamiento: se aplica solo en esta sesión */
  }
  window.location.reload();
}
