// Rastro's website: a landing page per language and the guide (docs/ in this repository),
// built as static files for GitHub Pages. No analytics, no cookies, no server.
import { readFileSync } from "node:fs";
import starlight from "@astrojs/starlight";
import { defineConfig, passthroughImageService } from "astro/config";

// Sidebar labels come from the same translations as the landing page.
const texts = JSON.parse(readFileSync(new URL("./src/i18n/landing.json", import.meta.url), "utf8"));
const label = (key) => ({
  label: texts[0][key],
  translations: Object.fromEntries(texts.slice(1).map((l) => [l.lang, l[key]])),
});
const GITHUB = "https://github.com/MaxSala/rastro-tarkov";

// Where the site lives. A custom domain later only changes these two (see site/README.md).
export const SITE = process.env.SITE_URL ?? "https://maxsala.github.io";
export const BASE = process.env.BASE_PATH ?? "/rastro-tarkov";

export default defineConfig({
  site: SITE,
  base: BASE,
  trailingSlash: "always",
  image: { service: passthroughImageService() },
  // Never inline scripts: the landing page's CSP only allows scripts from the site itself.
  vite: { build: { assetsInlineLimit: 0 } },
  integrations: [
    starlight({
      title: "Rastro",
      favicon: "/favicon.svg",
      logo: { src: "./public/favicon.svg", alt: "Rastro" },
      social: [
        { icon: "github", label: "GitHub", href: "https://github.com/MaxSala/rastro-tarkov" },
        { icon: "discord", label: "Discord", href: "https://discord.gg/H4pRxGBGDb" },
      ],
      defaultLocale: "root",
      locales: {
        root: { label: "English", lang: "en" },
        es: { label: "Español", lang: "es" },
        de: { label: "Deutsch", lang: "de" },
        fr: { label: "Français", lang: "fr" },
        pl: { label: "Polski", lang: "pl" },
        "pt-br": { label: "Português", lang: "pt-BR" },
        ru: { label: "Русский", lang: "ru" },
        "zh-cn": { label: "中文", lang: "zh-CN" },
      },
      sidebar: [
        { ...label("startHere"), items: [{ slug: "docs/guide" }] },
        { ...label("reference"), items: [{ slug: "docs/features" }, { slug: "docs/faq" }] },
        {
          ...label("project"),
          items: [
            {
              slug: "docs/changelog",
              label: "What's new",
              translations: Object.fromEntries(texts.slice(1).map((l) => [l.lang, l.nav[2]])),
            },
            { ...label("downloadShort"), link: `${GITHUB}/releases/latest` },
            { label: "GitHub", link: GITHUB },
            { label: "Discord", link: "https://discord.gg/H4pRxGBGDb" },
          ],
        },
      ],
      components: { SocialIcons: "./src/components/docs/SocialIcons.astro" },
      customCss: [
        "@fontsource-variable/ibm-plex-sans/wdth.css",
        "@fontsource/ibm-plex-mono/400.css",
        "@fontsource/ibm-plex-mono/500.css",
        "./src/styles/starlight.css",
      ],
      lastUpdated: false,
    }),
  ],
});
