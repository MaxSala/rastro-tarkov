// Rastro's website: a landing page per language and the guide (docs/ in this repository),
// built as static files for GitHub Pages. No analytics, no cookies, no server.
import starlight from "@astrojs/starlight";
import { defineConfig, passthroughImageService } from "astro/config";

// Where the site lives. A custom domain later only changes these two (see site/README.md).
export const SITE = process.env.SITE_URL ?? "https://maxsala.github.io";
export const BASE = process.env.BASE_PATH ?? "/rastro-tarkov";

export default defineConfig({
  site: SITE,
  base: BASE,
  trailingSlash: "always",
  image: { service: passthroughImageService() },
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
        { slug: "docs/features" },
        { slug: "docs/guide" },
        { slug: "docs/faq" },
        {
          slug: "docs/changelog",
          label: "What's new",
          translations: {"es": "Novedades", "de": "Neuigkeiten", "fr": "Nouveautés", "pl": "Nowości", "pt-BR": "Novidades", "ru": "Что нового", "zh-CN": "更新内容"},
        },
      ],
      customCss: ["@fontsource-variable/geist", "@fontsource-variable/geist-mono", "./src/styles/starlight.css"],
      lastUpdated: false,
      pagination: false,
    }),
  ],
});
