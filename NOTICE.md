# Third-party notices

Rastro's own source code is released under the MIT License (see [LICENSE](LICENSE)). It includes or links to the third-party components listed below, each under its own license.

This list covers the main components. The complete dependency trees, with every license, can be printed with `cargo deny list` (Rust) and `npm ls --all` (web).

## Bundled with the application

### Typeface

- **Geist Sans** and **Geist Mono** by Vercel, SIL Open Font License 1.1, via `@fontsource-variable/geist` and `@fontsource-variable/geist-mono`.

### Web interface

| Component | License |
|---|---|
| React, React DOM | MIT |
| Leaflet | BSD-2-Clause |
| zod | MIT |
| zustand | MIT |
| lucide-react | ISC |
| @radix-ui/react-tooltip | MIT |
| Tailwind CSS (build only) | MIT |
| Vite (build only) | MIT |

### Rust

| Component | License |
|---|---|
| tokio, axum, hyper, tower, tracing, bytes | MIT |
| serde, serde_json, regex, anyhow, thiserror, getrandom, sha2, data-encoding, zeroize, directories, rust-embed, reqwest, zip, opener | MIT or Apache-2.0 |
| snow | Apache-2.0 or MIT |
| rustls | Apache-2.0, ISC or MIT |
| ring | Apache-2.0 and ISC |
| webpki-roots | CDLA-Permissive-2.0 |
| notify | CC0-1.0 |
| subtle, curve25519-dalek | BSD-3-Clause |
| if-addrs | MIT or BSD-3-Clause |
| tao, wry (window and WebView) | MIT or Apache-2.0 |
| gtk-rs, webkit2gtk-rs (Linux only) | MIT |
| webview2-com (Windows only) | MIT |
| embed-resource (build only, embeds the `.exe` icon) | MIT |
| option-ext | MPL-2.0 |
| ICU data | Unicode-3.0 |

### Ideas and formats learned from other projects

- **Reading quest progress from the game logs** (`src/gamelog/questlog.rs`, `history.rs`): the format of the trader messages in `push-notifications` logs, telling PvP and PvE characters apart, and ordering launch folders by date were learned from [tarkov-map-companion](https://github.com/Battleroid/tarkov-map-companion) by Casey Weed, MIT License, Copyright (c) 2026 Casey Weed. Rastro's code is its own; no code was copied.
- **What to bring into a raid** (`web/src/lib/questPlanning.ts`): the rule that only marked, planted and used items go into your rig follows the same project's `TaskRequirements`.
- **Time in Tarkov** (`web/src/lib/tarkovTime.ts`): real time × 7 on Moscow time, the right side 12 hours ahead, as in [tarkov-time](https://github.com/adamburgess/tarkov-time) by Adam Burgess, MIT License. Rastro's code is its own.
- **Armored train timings** (same file): arriving between 20 and 15 minutes before the end of the raid and staying 7 minutes, as stated on the Reserve and Lighthouse pages of the [Escape from Tarkov wiki](https://escapefromtarkov.fandom.com/). Only the facts are used; no text was copied.
- **Hideout and boss data formats** (`src/quests/hideout.rs`, `bosses.rs`): the shape of `json.tarkov.dev`'s `hideout` and `maps` files was learned from the tarkov.dev client and [tarkov-data-manager](https://github.com/the-hideout/tarkov-data-manager) (both MIT). No code was copied.
- **Boss skull icon** (`web/src/components/map/bosses.ts`): the *Skull* icon from [Lucide](https://lucide.dev), ISC License, already used by the interface through `lucide-react`.

### Demo map

- `assets/demo/` was made for this project and is released under CC0-1.0. It is an invented map and does not represent any real Escape from Tarkov location.

## Downloaded by the user, not bundled

Rastro does not ship any of the following. They are downloaded only when the user asks for them, and each keeps its own license.

### tarkov.dev

[tarkov.dev](https://tarkov.dev) is maintained by [The Hideout](https://github.com/the-hideout) community. It started as [tarkov-tools](https://github.com/kokarn/tarkov-tools) by Oskar Risberg ([@kokarn](https://github.com/kokarn)).

- **SVG maps** from [the-hideout/tarkov-dev-svg-maps](https://github.com/the-hideout/tarkov-dev-svg-maps), licensed [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/), with an explicit ban on use in cheats or software that gives an unfair advantage. Rastro stores the author and license inside every installed map pack.
- **Map calibration and floors** from `src/data/maps.json` in [the-hideout/tarkov-dev](https://github.com/the-hideout/tarkov-dev).
- **Marker icons** from `public/maps/interactive` in [the-hideout/tarkov-dev](https://github.com/the-hideout/tarkov-dev), MIT License, Copyright (c) 2019 Oskar Risberg. Rastro's container-to-icon table (`src/tarkovdev/icons.rs`) follows the one in `src/pages/map/map-images.mjs` of the same project.
- **Extracts, transits, switches, containers and locks** from the public tarkov.dev API and data files.
- **Item catalog, quests, traders, hideout stations, boss spawns and translations** from `json.tarkov.dev`, the same source the tarkov.dev website uses (or its GraphQL API as a fallback). The data format was learned by reading the tarkov.dev client (MIT); no code was copied.

### Item icons, trader portraits and quest pictures

- Served by `assets.tarkov.dev`. The images belong to Battlestate Games; Rastro only caches them locally to display them.

### Other data

- **Caliber names** (`src/market/build.rs`) are Rastro's own table of commonly used names.

## Trademarks

*Escape from Tarkov* is a trademark of Battlestate Games. Game content and materials are trademarks and copyrights of Battlestate Games and its licensors. Rastro is not affiliated with or endorsed by Battlestate Games or by tarkov.dev.
