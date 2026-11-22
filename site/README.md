# Rastro's website

The public site: a landing page in eight languages and the guide. It is published on GitHub
Pages by `.github/workflows/site.yml` every time `site/`, `docs/` or `CHANGELOG.md` change on
`main`.

- **Landing page:** `src/components/Landing.astro` and one component per section in
  `src/components/landing/`. Text in `src/i18n/landing.json`, styles in `src/styles/landing*.css`,
  and the few behaviours (feature tabs, reveal on scroll) in `src/scripts/landing.ts`. The page is
  complete without scripts, and every animation respects *reduce motion*.
- **Guide theme:** `src/styles/starlight.css`, plus the header's download button in
  `src/components/docs/`. Sidebar labels come from `landing.json` too.
- **Guide:** written once, in `docs/<language>/` at the root of the repository (the same files
  GitHub shows). `scripts/sync-docs.mjs` turns them into the site's pages before every build, and
  copies `docs/images/` and `CHANGELOG.md`. Don't edit `src/content/docs/`: it's regenerated.
- **Built with** [Astro](https://astro.build) and [Starlight](https://starlight.astro.build):
  static files only, no server, no analytics and no cookies. The download button points to the
  latest GitHub release, so the `.exe` always comes from the public build.

## Work on it

Requires Node.js 22.23 or later.

```sh
cd site
npm ci
npm run dev       # http://localhost:4321/rastro-tarkov/
npm run build     # static files in site/dist
```

## A custom domain later

1. Point the domain to GitHub Pages and set it in *Settings → Pages → Custom domain*.
2. In `.github/workflows/site.yml`, set `SITE_URL` to `https://your-domain` and `BASE_PATH` to `/`.

Nothing else changes: every link on the site is built from those two values.
