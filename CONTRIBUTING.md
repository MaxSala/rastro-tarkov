# Contributing to Rastro

Thanks for wanting to help. Rastro is a young project with a long [roadmap](docs/ROADMAP.md), and every fix, translation, test or idea moves it forward.

Not a developer? You can still help a lot: report bugs, suggest features, review a translation or help other players. Everything starts on the **[Rastro Discord](https://discord.gg/H4pRxGBGDb)**. To join the team as a moderator, translator, tester or developer, open a *Join the team* ticket there.

## Before you start

- **Small fixes** (typos, translations, obvious bugs): open a pull request directly.
- **New features or bigger changes:** open an issue or post in the feature requests forum on Discord first, so we can agree on the approach before you spend time on it.
- **Security issues:** never in public. See [docs/SECURITY.md](docs/SECURITY.md).

## The ground rule

Rastro only reads files the game writes to disk: screenshot file names and, if the user turns it on, the game's log files. Any contribution has to respect that.

- Never read game memory, inject input, hook the game process, capture network traffic or draw over the game.
- Never move, edit or delete the user's screenshots.
- No telemetry. The app only connects to the network when the user asks for it, and only to hosts on the allowlist.
- Never store, log or share private data from the game logs, such as account or profile IDs and IP addresses.

## Development setup

**Requirements:** [Rust](https://rustup.rs/) 1.85 or newer and [Node.js](https://nodejs.org/) LTS. On Linux, also `libwebkit2gtk-4.1-dev` and `libgtk-3-dev`.

```sh
git clone https://github.com/MaxSala/rastro-tarkov.git
cd rastro-tarkov
```

To build the release executable, run `build.bat` on Windows or `./build.sh` on Linux and macOS. Both build the web interface first and then the app, which embeds it. The result is `target\release\rastro.exe` on Windows and `target/release/rastro` elsewhere.

For day-to-day work, run the interface and the backend separately:

```sh
cd web && npm ci && npm run dev    # terminal 1: Vite dev server on 127.0.0.1:5173
cargo run -- --dev                 # terminal 2: backend; opens the browser with the session key
```

`--dev` only widens the accepted origins to `localhost:5173`. Don't use it to play.

## Project layout

```
src/          Rust backend: screenshot watcher, local server, map packs, party, market, game logs
web/          Interface: React, TypeScript, Leaflet and Tailwind
  src/locales/  One folder per language
assets/       Logo, app icon and the demo map
docs/         Security model, protocol, map pack format, roadmap and release process
```

Technical documentation lives in `docs/`: [security model](docs/SECURITY.md), [interface protocol](docs/PROTOCOL.md), [map pack format](docs/MAP_PACK.md), [roadmap](docs/ROADMAP.md) and [release process](docs/RELEASING.md).

## Code rules

- **Rust:** `unsafe` is forbidden, and Clippy denies `unwrap`, `expect`, `panic!` and unchecked slice indexing. Handle errors explicitly.
- **Interface:** every piece of text the user sees lives in the locale files, never hard-coded in a component. Build DOM content with nodes, never with `innerHTML`.
- **Dependencies** are pinned to exact versions. `cargo deny check` and `cargo audit` cover the Rust side, and npm runs with `ignore-scripts=true`. Adding a dependency needs a good reason.
- **Small files.** Keep each source file under 300 lines. Split it before it grows past that.
- **Tests** for new logic, especially anything that parses files or network messages.

## Before sending a pull request

```sh
cargo fmt --check
cargo clippy --all-targets
cargo test
cd web && npm run typecheck && npm run lint && npm run format:check
```

In the pull request, explain what changes for the player and how you tested it. Screenshots help for anything visual. If the change is user-facing, add a line under an *Unreleased* heading at the top of [CHANGELOG.md](CHANGELOG.md).

## Translations

Each language lives in `web/src/locales/<code>/` (`main.ts`, `game.ts` and `tools.ts`) and is type-checked against the Spanish files, so a missing key won't compile.

To fix a translation, edit the file and open a pull request, or point it out on [Discord](https://discord.gg/H4pRxGBGDb). Spanish and English are reviewed; the other 14 languages still need a native speaker to check them, so any help there is very welcome.

## Releases

How to publish a new version is described in [docs/RELEASING.md](docs/RELEASING.md). Only maintainers publish releases.

## License

By contributing, you agree that your contribution is released under the project's [MIT License](LICENSE).
