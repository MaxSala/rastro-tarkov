// Shared by the landing components: the texts for one language, the links, and the version, read
// from Cargo.toml at build time so the page never shows a stale number.
import { readFileSync } from "node:fs";
import { join } from "node:path";
import all from "../../i18n/landing.json";

export type Texts = (typeof all)[number];

export const GITHUB = "https://github.com/MaxSala/rastro-tarkov";
export const DISCORD = "https://discord.gg/H4pRxGBGDb";
export const DOWNLOAD = `${GITHUB}/releases/latest/download/Rastro-windows-x64.exe`;

export function pick(slug: string): Texts {
  return all.find((l) => l.slug === slug) ?? all[0]!;
}

export function links(slug: string) {
  const base = import.meta.env.BASE_URL.replace(/\/?$/, "/");
  const home = (s: string) => `${base}${s ? `${s}/` : ""}`;
  return {
    base,
    home,
    docs: (page: string) => `${home(slug)}docs/${page}/`,
    img: (name: string) => `${base}images/${name}.png`,
    download: DOWNLOAD,
  };
}

function readVersion(): string {
  try {
    // Builds run from site/ (npm run build), so the app's manifest is one folder up.
    const toml = readFileSync(join(process.cwd(), "..", "Cargo.toml"), "utf8");
    return toml.match(/^version\s*=\s*"([^"]+)"/m)?.[1] ?? "";
  } catch {
    return "";
  }
}
export const VERSION = readVersion();

// "**bold** and `code`" from the shared texts, as plain segments: nothing is injected as HTML.
export type Seg = { text: string; kind: "plain" | "bold" | "code" };
export function rich(s: string): Seg[] {
  return s
    .split(/(\*\*[^*]+\*\*|`[^`]+`)/)
    .filter(Boolean)
    .map((p) =>
      p.startsWith("**")
        ? { text: p.slice(2, -2), kind: "bold" }
        : p.startsWith("`")
          ? { text: p.slice(1, -1), kind: "code" }
          : { text: p, kind: "plain" },
    );
}

// A real screenshot name, split into what each part means (the same one the hero shows).
export const FILE: { text: string; part?: number }[] = [
  { text: "2026-11-21[19-38]", part: 0 },
  { text: "_" },
  { text: "218.0, 1.0, 136.0", part: 1 },
  { text: "_" },
  { text: "0.0, 0.81915, 0.0, -0.57358", part: 2 },
  { text: "_" },
  { text: "13.27", part: 3 },
  { text: " (0).png" },
];
