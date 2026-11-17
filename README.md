<p align="center">
  <img src="assets/brand/rastro.svg" width="72" height="72" alt="Rastro logo">
</p>

<h1 align="center">Rastro</h1>

<p align="center">
  A second-monitor map companion for <i>Escape from Tarkov</i>.<br>
  Free, open source and non-commercial. It never touches the game.
</p>

<p align="center">
  <a href="https://discord.gg/H4pRxGBGDb"><img src="https://img.shields.io/badge/Discord-Join%20the%20community-5865F2?logo=discord&logoColor=white" alt="Join the Discord"></a>
</p>

---

Rastro shows where you are on the map, how far each extract is and in which direction, which key opens which door, and what an item sells for. It runs on your PC as a small desktop app, meant to sit on a second monitor while you play.

It works from the screenshots you take in game. Every screenshot Tarkov saves has your coordinates in its file name. Rastro reads that name and nothing else: it doesn't read memory, inject anything, send keystrokes or interact with the game process in any way, and it never edits or deletes your screenshots.

> [!IMPORTANT]
> Rastro is an unofficial fan project. It is not affiliated with or endorsed by Battlestate Games, nor by the tarkov.dev team. Using third-party tools with Escape from Tarkov is your own responsibility under BSG's terms of service.

## Features

**Map**
- Your position and heading, updated every time you take a screenshot, with a fading trail of where you've been.
- Extracts filtered by PMC, Scav or shared. Mark the ones the game says are open this raid and the rest fade out.
- Click an extract (or any point) to get the distance, the turn to make and the height difference from where you last were.
- Your own points: right-click the map to save loot spots, danger areas, stashes or quest locations. They are stored in game coordinates, so they survive reinstalling a map.
- Map layers from tarkov.dev: transits, switches, loot containers (filterable by type) and locks.
- Multi-level maps switch floors automatically based on your height, or you can pick one by hand.

**Keys**
- The *Locks* layer shows every door, container and car trunk that needs a key. Locks that need power have a yellow outline.
- Click a lock to see which key opens it, the cheapest place to buy it and its best sell price.
- *Highlight this key* dims every other lock so you can see everything a single key opens.
- The other way round, every key in the Market tells you which of your installed maps it is used on, with a link that jumps straight to it.

**Market**
- Search the full item catalog by name or short name. Accents and word order don't matter. Press `/` to focus the search box and the arrow keys to move through results.
- Sort by best sell, flea price, 24-hour average, 48-hour change or price per inventory slot.
- Each item lists every trader and flea offer, including the trader level required and the original price in dollars or euros.
- The catalog is downloaded once and searched locally, so lookups are instant, work offline and never leave your PC.

**Ammo**
- Every round grouped by caliber, sorted by penetration by default.
- Damage (per pellet × pellet count for buckshot), penetration, armor damage, fragmentation chance, muzzle velocity and the cheapest purchase option. Tracers are marked.

**Raid detection** (optional, off by default)
- Reads the log files the game writes to disk to switch to the right map when a raid loads, reset your trail and marks at the start of each raid, and show the raid timer.
- Only the map name and game mode are used. Nothing is stored or shared.

**Party**
- Share positions with your squad without any server in between: one player hosts a room and the others join it directly with an invite code.
- Traffic is end-to-end encrypted using the Noise protocol. The invite code works like a password.
- A mesh VPN such as Tailscale or ZeroTier is recommended over opening ports on your router.

**Languages**
- The interface is available in the 16 languages tarkov.dev supports: Čeština, Deutsch, English, Español, Français, Magyar, Italiano, 日本語, 한국어, Polski, Português, Română, Русский, Slovenčina, Türkçe and 中文.
- Item, map and extract names come from tarkov.dev in the same language.

## Getting started

### Requirements

- Windows 10 or 11 (WebView2 ships with both). Linux works too; macOS is untested.
- [Rust](https://rustup.rs/) 1.85 or newer.
- [Node.js](https://nodejs.org/) LTS.
- On Linux only: `libwebkit2gtk-4.1-dev` and `libgtk-3-dev`.

### Build

```sh
git clone https://github.com/MaxSala/rastro-tarkov.git
cd rastro-tarkov
```

On Windows, run `build.bat`. On Linux or macOS, run `./build.sh`. Both build the web interface first and then the app, which embeds it. The result is a single executable:

- `target\release\rastro.exe` on Windows
- `target/release/rastro` elsewhere

Copy it wherever you like; it doesn't need anything else next to it.

### First run

A short setup guide opens the first time:

1. **Screenshots folder.** Rastro looks for `Documents\Escape from Tarkov\Screenshots`, including when Windows has moved Documents into OneDrive. If it can't find it, paste the path and it will tell you how many screenshots with coordinates it sees.
2. **Maps.** Download the community maps from tarkov.dev (recommended), import a map pack, or try the demo map.
3. Done. Take a screenshot in game and your arrow appears on the map.

To use the Market, Ammo table and key names, open the Market tab once and press *Download prices*.

## Everyday use

| Action | How |
|---|---|
| Update your position | Take a screenshot in game |
| Switch section (Map, Market, Ammo) | `Ctrl+1`, `Ctrl+2`, `Ctrl+3` |
| Command palette | `Ctrl+K` |
| Extracts, points, party and maps panels | `1`, `2`, `3`, `4` |
| New raid (clear trail and marks) | `N` |
| Fit the whole map / follow your position | `F` / `G` |
| Zoom | `+` / `-` or the mouse wheel |
| Add a point, guide to a place | Right-click on the map |
| Ping your party | `Shift` + click |
| Search the Market | `/` |
| Settings | `,` |

Settings let you choose the language, the theme, whether Rastro opens in its own window or in your browser, marker size, trail length and the folders it reads.

### Where your data lives

| System | Folder |
|---|---|
| Windows | `%APPDATA%\Rastro\data\` |
| Linux | `~/.local/share/rastro/` |
| macOS | `~/Library/Application Support/Rastro/` |

That folder holds your settings, your points, installed maps, the market catalog and a log file (`rastro.log`), which is rewritten on every start and never contains your session key or party code. Party positions are never written to disk. *Settings → Log* has a button that opens the folder.

## Privacy and network use

Rastro has no account, no telemetry, no update checks and no ads. The interface talks only to the app itself on `127.0.0.1`, protected by a random key that changes on every launch. Anything beyond that happens only when you ask for it:

| When | Connects to |
|---|---|
| Always | Nothing outside your PC |
| You download maps from tarkov.dev | `raw.githubusercontent.com`, `json.tarkov.dev` and, as a fallback, `api.tarkov.dev` |
| You download or refresh prices | `json.tarkov.dev`, plus `assets.tarkov.dev` for item icons |
| You import a map pack from a URL | Only the address you enter (HTTPS, never internal IPs) |
| You host or join a party | Directly to your teammates' PCs |

Every download host is on a fixed allowlist, also checked on redirects. Everything downloaded is treated as untrusted and validated before use. The full threat model is in [docs/SECURITY.md](docs/SECURITY.md).

## Community

Questions, bug reports, feature ideas or just looking for a squad to try the party mode with: join the **[Rastro Discord](https://discord.gg/H4pRxGBGDb)**. It's also where development happens day to day, so it's the best place to follow what's coming next or help with translations.

Bugs and pull requests are also welcome here on GitHub.

## Credits

Rastro would not exist without **[tarkov.dev](https://tarkov.dev)**, the open-source Escape from Tarkov database maintained by **[The Hideout](https://github.com/the-hideout)** community. tarkov.dev began as [tarkov-tools](https://github.com/kokarn/tarkov-tools) by **Oskar Risberg ([@kokarn](https://github.com/kokarn))** and was revived and opened up by The Hideout's contributors. Rastro uses:

- **Maps:** the SVG maps from [the-hideout/tarkov-dev-svg-maps](https://github.com/the-hideout/tarkov-dev-svg-maps) by tarkov.dev's contributors, licensed [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/). Their authors don't allow the maps to be used in cheats or software that gives an unfair advantage, such as radars. Maps are downloaded by you and never redistributed with Rastro, and each installed map keeps its credits and license.
- **Data:** extracts, transits, switches, containers, locks, items, prices and translations from the tarkov.dev API and its data files.
- **Marker icons:** from [tarkov-dev](https://github.com/the-hideout/tarkov-dev) (`public/maps/interactive`), MIT License, © 2019 Oskar Risberg.
- **Item icons** belong to Battlestate Games and are only cached locally for display.

If you find Rastro useful, consider supporting tarkov.dev, contributing to it or joining their community.

Rastro is also built on many other open-source projects, including Rust, Tokio, Axum, Snow, Rustls, Wry, React, Leaflet, Tailwind CSS and the Geist typeface. The full list with licenses is in [NOTICE.md](NOTICE.md).

*Escape from Tarkov* is a trademark of Battlestate Games. Game content and materials are trademarks and copyrights of Battlestate Games and its licensors.

## Non-commercial

Rastro is a hobby project made for the community. It is free, there is no paid version, it shows no ads and collects no data. It will stay that way, both by choice and because the tarkov.dev maps it relies on are licensed for non-commercial use only.

## Known limitations

- Your position only updates when you take a screenshot. That's by design: it's the only information Rastro reads.
- Labs has no SVG map on tarkov.dev. Rastro recognises it, but it can't be shown unless you import your own map pack for it.
- Raid detection has been verified with PvE logs. PvP should behave the same, but hasn't been confirmed with real logs.
- Messages generated by the app itself (import and download errors) are still in Spanish whatever language you pick.
- Spanish and English translations are reviewed. The other 14 haven't been checked by native speakers yet. Corrections are very welcome, here or on [Discord](https://discord.gg/H4pRxGBGDb).
- Changing the language or game mode (PvP/PvE) needs a fresh download of maps and prices to see the new names.

## Development

```sh
cd web && npm ci && npm run dev    # terminal 1: Vite dev server on 127.0.0.1:5173
cargo run -- --dev                 # terminal 2: backend; opens the browser with the session key
```

`--dev` only widens the accepted origins to `localhost:5173`. Don't use it to play.

Before sending changes:

```sh
cargo fmt --check
cargo clippy --all-targets
cargo test
cd web && npm run typecheck && npm run lint && npm run format:check
```

Dependencies are pinned to exact versions, `cargo deny check` and `cargo audit` cover the Rust side, and npm runs with `ignore-scripts=true`.

### Project layout

```
src/          Rust backend: screenshot watcher, local server, map packs, party, market, game logs
web/          Interface: React, TypeScript, Leaflet and Tailwind
  src/locales/  One folder per language
assets/       Logo, app icon and the demo map
docs/         Security model, protocol, map pack format and roadmap
```

[docs/SECURITY.md](docs/SECURITY.md) is in English. The protocol, map pack format and roadmap documents in `docs/` are still in Spanish.

### Translations

Each language lives in `web/src/locales/<code>/` (`main.ts`, `game.ts` and `tools.ts`) and is type-checked against the Spanish files, so a missing key won't compile. To fix a translation, edit the file and open a pull request, or point it out in the `#translations` channel on [Discord](https://discord.gg/H4pRxGBGDb).

## License

Rastro's source code is released under the [MIT License](LICENSE). Maps and data downloaded from tarkov.dev keep their own licenses, listed under [Credits](#credits) and in [NOTICE.md](NOTICE.md).
