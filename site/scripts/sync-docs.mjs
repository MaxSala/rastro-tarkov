// Turns the guide in ../docs (the same files GitHub shows) into the website's pages, so there
// is only one copy to keep up to date. Runs before every build; its output is not committed.
//
//   docs/<lang>/FEATURES.md, GUIDE.md, FAQ.md  →  src/content/docs/<locale>/docs/*.md
//   CHANGELOG.md                               →  src/content/docs/docs/changelog.md (English)
//   docs/images/*.png                          →  public/images/
//
// Links are rewritten for the site: sibling pages become site pages, images get the base path,
// and anything else in the repository points to GitHub.
import { copyFileSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const repo = join(here, "..", "..");
const site = join(here, "..");
const BASE = (process.env.BASE_PATH ?? "/rastro-tarkov").replace(/\/$/, "");
const GITHUB = "https://github.com/MaxSala/rastro-tarkov/blob/main/";
// Folder in docs/ → locale on the site ("" is English, at the root).
const LOCALES = { en: "", es: "es", de: "de", fr: "fr", pl: "pl", "pt-BR": "pt-br", ru: "ru", "zh-CN": "zh-cn" };
const PAGES = { "FEATURES.md": "features", "GUIDE.md": "guide", "FAQ.md": "faq" };

const out = join(site, "src", "content", "docs");
rmSync(out, { recursive: true, force: true });
mkdirSync(join(site, "public", "images"), { recursive: true });
for (const f of readdirSync(join(repo, "docs", "images")).filter((f) => f.endsWith(".png"))) {
  copyFileSync(join(repo, "docs", "images", f), join(site, "public", "images", f));
}

function link(target) {
  if (/^(https?:|mailto:|#)/.test(target)) return target;
  const [path, anchor] = target.split("#");
  const hash = anchor ? `#${anchor}` : "";
  if (PAGES[path]) return `../${PAGES[path]}/${hash}`;
  if (path.startsWith("../images/")) return `${BASE}/images/${path.slice("../images/".length)}`;
  if (path === "../../CHANGELOG.md") return `${BASE}/docs/changelog/`;
  if (path.startsWith("../../")) return GITHUB + path.slice(6) + hash;
  if (path.startsWith("../")) return `${GITHUB}docs/${path.slice(3)}${hash}`;
  return target;
}

function convert(md) {
  // Drop the "back to the README · Features · Guide · FAQ" bar: the site has its own menu.
  let body = md.replace(/^<p align="center">[\s\S]*?<\/p>\s*/, "");
  const title = (body.match(/^# (.+)$/m) ?? [, "Rastro"])[1].trim();
  body = body.replace(/^# .+\n+/m, "");
  body = body.replace(/\]\(([^)\s]+)\)/g, (_, t) => `](${link(t)})`);
  body = body.replace(/(href|src)="([^"]+)"/g, (_, a, t) => `${a}="${link(t)}"`);
  return `---\ntitle: ${JSON.stringify(title)}\n---\n\n${body.trim()}\n`;
}

for (const [folder, locale] of Object.entries(LOCALES)) {
  const dir = join(out, locale, "docs");
  mkdirSync(dir, { recursive: true });
  for (const [file, slug] of Object.entries(PAGES)) {
    writeFileSync(join(dir, `${slug}.md`), convert(readFileSync(join(repo, "docs", folder, file), "utf8")));
  }
}

// The changelog is written in English; the other languages show it with Starlight's notice.
const changelog = readFileSync(join(repo, "CHANGELOG.md"), "utf8").replace(/^# .+\n+/m, "");
writeFileSync(join(out, "docs", "changelog.md"), `---\ntitle: "What's new"\n---\n\n${changelog.trim()}\n`);
console.log("docs synced for", Object.keys(LOCALES).length, "languages");
