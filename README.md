<p align="center">
  <img src="docs/images/banner.png" alt="Rastro: a live raid map for Escape from Tarkov, for your second monitor" width="100%">
</p>

<p align="center">
  🌐 <b>English</b> · <a href="README.es.md">Español</a> · <a href="README.ru.md">Русский</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a> · <a href="README.fr.md">Français</a> · <a href="README.pl.md">Polski</a> · <a href="README.zh-CN.md">中文</a>
</p>

<p align="center">
  <a href="https://github.com/MaxSala/rastro-tarkov/releases/latest"><img src="https://img.shields.io/github/v/release/MaxSala/rastro-tarkov?label=Latest%20version&color=f0a43a" alt="Latest version"></a>
  <img src="https://img.shields.io/badge/Status-Public%20beta-f0a43a" alt="Public beta">
  <a href="https://github.com/MaxSala/rastro-tarkov/stargazers"><img src="https://img.shields.io/github/stars/MaxSala/rastro-tarkov?label=Stars&color=f0a43a&logo=github" alt="GitHub stars"></a>
  <a href="https://discord.gg/H4pRxGBGDb"><img src="https://img.shields.io/badge/Discord-Join%20the%20community-5865F2?logo=discord&logoColor=white" alt="Join the Discord"></a>
  <img src="https://img.shields.io/badge/Windows-10%20%7C%2011-0078D4?logo=windows&logoColor=white" alt="Windows 10 and 11">
  <a href="LICENSE"><img src="https://img.shields.io/github/license/MaxSala/rastro-tarkov?color=4c4c4c" alt="MIT License"></a>
</p>

<h3 align="center">
  <a href="https://github.com/MaxSala/rastro-tarkov/releases/latest/download/Rastro-windows-x64.exe">⬇ Download Rastro for Windows</a>
</h3>

<p align="center">
  Free and open source · No installer · No account · Nothing to configure
  <br>
  <a href="#how-it-works">How it works</a> ·
  <a href="#play-with-your-squad">Squad mode</a> ·
  <a href="#is-it-safe">Is it safe?</a> ·
  <a href="#install">Install</a> ·
  <a href="#where-rastro-is-going">Roadmap</a> ·
  <a href="#get-involved">Get involved</a> ·
  <a href="https://discord.gg/H4pRxGBGDb">Discord</a>
</p>

> [!TIP]
> **Rastro is in public beta, and this is only the beginning.** Quests just landed; the hideout, squad quests and much more are on the way, and the community decides what comes first.
> If you like where it's going, **[⭐ star the repo](https://github.com/MaxSala/rastro-tarkov/stargazers)** and **[join the Discord](https://discord.gg/H4pRxGBGDb)**. It takes five seconds and helps more than you think.

---

**Rastro is a live map of your raid that runs on your second monitor.** Take a screenshot in game and your position appears on the map, with the distance and direction to every extract. Open a room and your squad sees each other on the same map, on every PC. Hover a locked door to see which key opens it and what it costs. Track your quests with every open objective on the map. Check prices and ammo without alt-tabbing to a browser.

It never touches the game. Rastro only reads files Tarkov already writes to your disk, and it's free, open source and made by players for players.

## Screenshots

<table>
  <tr>
    <td width="50%"><img src="docs/images/party.png" alt="Three squad members on Customs, with one teammate's hover card"></td>
    <td width="50%"><img src="docs/images/map.png" alt="Customs with your position, trail and guidance to an extract"></td>
  </tr>
  <tr>
    <td align="center"><b>Squad.</b> Everyone's position and heading on one map, with pings. No server in between.</td>
    <td align="center"><b>Map.</b> Your position and trail, every extract, and live distance and direction to the one you pick.</td>
  </tr>
  <tr>
    <td width="50%"><img src="docs/images/floors.png" alt="The dorms on the second floor, with the rest of the map and teammates downstairs dimmed"></td>
    <td width="50%"><img src="docs/images/keys.png" alt="The locks layer with a key card open"></td>
  </tr>
  <tr>
    <td align="center"><b>Floors.</b> Follows you between floors and fades out everything that isn't on yours.</td>
    <td align="center"><b>Keys.</b> Every locked door on the map, which key opens it and what it costs.</td>
  </tr>
  <tr>
    <td width="50%"><img src="docs/images/market.png" alt="Market search with an item's offers"></td>
    <td width="50%"><img src="docs/images/ammo.png" alt="Ammo table for one caliber"></td>
  </tr>
  <tr>
    <td align="center"><b>Market.</b> Instant offline search with flea and trader prices, sorted by value per slot.</td>
    <td align="center"><b>Ammo.</b> Damage, penetration and armor damage for every round, with the cheapest place to buy it.</td>
  </tr>
</table>

<sub>Map imagery by the <a href="https://github.com/the-hideout/tarkov-dev-svg-maps">tarkov.dev contributors</a>, CC BY-NC-SA 4.0. Prices and item data from <a href="https://tarkov.dev">tarkov.dev</a>. Squad screenshots use example positions.</sub>

## Where Rastro is going

Rastro started as a way to never get lost on a second monitor. The plan is much bigger: **one companion app for everything you check between and during raids**, still without ever touching the game. Here's what's on the table:

| | Coming next | What it means for you |
|---|---|---|
| 👥 | **Squad quests** | See your teammates' pending quests on the same map, so the whole squad can plan one route. |
| 🏠 | **Hideout tracker** | Every upgrade, what it needs and which items to keep. No more selling what you'll need tomorrow. |
| 🔑 | **Your key ring** | Mark the keys you own and the doors you can open stand out on every map. |
| 🛒 | **Shopping list** | Favourite items and get notified when their price moves. |
| 📊 | **Raid history** | Map, duration, queue time and game mode of every raid you play. |
| 🎯 | **Ammo vs armor** | A penetration chart for every round against each armor class. |
| 📍 | **Share your points** | Send your loot spots and stashes to your squad, or move them to another PC. |

Nothing here has a date yet, and the order isn't fixed: **it's shaped by what players ask for.** The full list, including smaller items and what's already shipped, is in [docs/ROADMAP.md](docs/ROADMAP.md).

**Got an idea?** Post it in the feature requests forum on [Discord](https://discord.gg/H4pRxGBGDb) or [open an issue](https://github.com/MaxSala/rastro-tarkov/issues). Every suggestion is read, and the most requested ones move up.

## How it works

Rastro reads two kinds of files that Escape from Tarkov already saves on your PC. It doesn't need anything else.

| What it reads | Where | What Rastro gets from it |
|---|---|---|
| **The file names of your screenshots** | `Documents\Escape from Tarkov\Screenshots` | Your position and the direction you're facing. Tarkov writes your coordinates into the name of every screenshot it saves. Rastro reads the name, never the picture, and never moves, edits or deletes your screenshots. |
| **The game's log files** (optional, off until you turn it on) | `Battlestate Games\Escape from Tarkov\Logs` | Which map is loading, when a raid starts and whether you're playing PvP or PvE. Rastro uses that to switch to the right map by itself, start a fresh trail every raid and show how much raid time is left. Only the map name and game mode are kept. It also reads the trader messages, to tick off the quests you accept, hand in or fail. |

So in practice: you press your screenshot key in game, and a second later your arrow moves on the map.

What Rastro **doesn't** do: it doesn't read game memory, inject anything, hook into the game process, capture network traffic, draw over the game or press keys for you. It runs as a separate program and only looks at files on disk.

## Play with your squad

Rastro's party mode puts your whole squad on the same map, each with their own name and colour.

- **See where everyone is.** Every teammate's arrow shows where they were and which way they were facing at their last screenshot. If someone is on a different floor, their arrow fades out and hovering it tells you which floor they're on.
- **Ping a spot.** `Shift` + click anywhere on the map and the spot flashes on everyone's screen.
- **Easy to set up.** One player opens a room and copies an invite code. The others paste it under *Join*. That's it. Up to 8 players.
- **No server, no account.** Positions go straight from PC to PC. Nobody else, including us, ever sees them.
- **Encrypted.** All traffic is end-to-end encrypted with the [Noise protocol](https://noiseprotocol.org/). The invite code is the key, so only share it with your squad.

The host needs to be reachable by the others. The easiest way is a free mesh VPN such as [Tailscale](https://tailscale.com/) or [ZeroTier](https://www.zerotier.com/): everyone installs it once and Rastro works as if you were on the same network, with no ports to open on your router. Opening a port on the host's router works too.

Each player still runs Rastro on their own PC and takes their own screenshots: that's what keeps everyone's position up to date.

## Features

**Map**
- Your position and heading, updated every time you take a screenshot, with a fading trail of where you've been.
- Click an extract or any point to get the distance, which way to turn and the height difference.
- Extracts filtered by PMC, Scav or shared. Mark the ones the game says are open this raid and the rest fade out.
- Layers from tarkov.dev: transits, switches, loot containers (filterable by type) and locks.
- Multi-level maps follow you between floors automatically, or you pick one in the floor panel. The rest of the map and everything on other floors is dimmed.
- Hover anything for a card with what it is, how far it is, which floor it's on and what clicking does.
- Your own points: right-click the map to save loot spots, danger areas, stashes or quest locations. They're stored in game coordinates, so they survive reinstalling a map.

**Quests**
- Every quest in your language, with what you're on, what you can start next and, for the rest, exactly what's missing: your level or the quests that come first. Filter by trader, map or Kappa, or search quests, objectives and items.
- **Quests follow the game.** With raid detection on, the quests you accept, hand in or fail are ticked off by themselves from the game logs, PvP and PvE apart. The first time, Rastro reads every launch the game has kept, so mid-wipe you're caught up at once. No logs? *Catch up* by picking the last quest you did for each trader.
- Every quest shows its trader, its picture and its rewards: experience, reputation, items, and the trader offers and crafts it unlocks.
- Your open objectives on the map, honest about the data: a badge for a precise place, the real outline for an area, and small dots for the places an item *can* spawn. Click one and Rastro guides you there, like an extract. Spot a wrong one? *Wrong spot?* reports it in one click.
- **Raid plan** in the *Quests* panel: tick what you're doing this raid and the map shows only that, numbered along the shortest route, with what to bring. *Next* shows the next objective in big; *To hand in* lists what's ready.
- Quests by trader, like in the game, with your Kappa and Lightkeeper progress, the map where you get the most done, and *Items*: what your quests still ask for, so you don't sell it.

**Keys**
- The *Locks* layer shows every door, container and car trunk that needs a key. Locks that need power have a yellow outline.
- Click a lock to see which key opens it, the cheapest place to buy it and its best sell price.
- *Highlight this key* dims every other lock, so you can see everything a single key opens.
- Every key in the Market lists the installed maps it's used on, with a link that jumps straight there.

**Market**
- Search the full item catalog by name or short name. Accents and word order don't matter. Press `/` to search and the arrow keys to move through results.
- Sort by best sell, flea price, 24-hour average, 48-hour change or price per inventory slot.
- Each item lists every trader and flea offer, with the trader level required and the original price in dollars or euros.
- The catalog is downloaded once and searched on your PC, so lookups are instant and work offline.

**Ammo**
- Every round grouped by caliber and sorted by penetration.
- Damage (per pellet × pellets for buckshot), penetration, armor damage, fragmentation chance, muzzle velocity and the cheapest place to buy it. Tracers are marked.

**Raid detection** (optional)
- Switches to the right map when a raid loads, resets your trail and marks at the start of each raid, and shows the time left. Reads the game's log files, as described in [How it works](#how-it-works).

**Everything else**
- 16 languages, the same ones tarkov.dev supports: Čeština, Deutsch, English, Español, Français, Magyar, Italiano, 日本語, 한국어, Polski, Português, Română, Русский, Slovenčina, Türkçe and 中文. Item, map and extract names follow your language too. Change it and Rastro offers to bring your maps, prices and quests over.
- PvP and PvE data, picked automatically from the game logs or by hand.
- Optional update notifications and a *What's new* page after every update.
- Keyboard shortcuts for everything, and a command palette (`Ctrl+K`).

## Is it safe?

Rastro is a small, portable program. Here's what that means and how you can check it.

- **No installer, no admin rights.** It's a single `.exe`. It doesn't install services or drivers, doesn't start with Windows and doesn't touch the game's files. Its settings and downloaded maps live in one folder (see [Where your data lives](#where-your-data-lives)), and deleting that folder and the `.exe` removes it completely.
- **Open source.** All the code is in this repository. Anyone can read it.
- **Built in public.** Every release is built by GitHub's own servers straight from the source code, not on someone's PC. You can see each build in the [Actions](https://github.com/MaxSala/rastro-tarkov/actions) tab, and the release page lists the file's SHA-256 so you can check you got exactly that file:

  ```powershell
  Get-FileHash .\Rastro-windows-x64.exe
  ```

- **Doesn't phone home.** No telemetry, no ads, no account. It only connects to the internet when you ask it to, and the full list of connections is in [Privacy](#privacy-and-network-use).

**"Windows protected your PC."** The first time you open Rastro, Windows SmartScreen will most likely show a blue window saying it stopped an unrecognised app. This happens with any new program that isn't signed with a paid certificate, not because something is wrong with the file. Click **More info**, then **Run anyway**. Windows remembers your choice. Some antivirus programs are wary of new unsigned programs too. If yours flags Rastro, you can restore it and add an exception, upload the file to [VirusTotal](https://www.virustotal.com/) to get a second opinion, and report it as a false positive to your antivirus vendor.

> [!IMPORTANT]
> Rastro is an unofficial fan project, not affiliated with or endorsed by Battlestate Games or tarkov.dev. It's built to stay out of the game: it only reads files on disk and never interacts with the game process. Even so, using third-party tools with Escape from Tarkov is your own responsibility under Battlestate Games' terms of service.

## Install

1. **[Download `Rastro-windows-x64.exe`](https://github.com/MaxSala/rastro-tarkov/releases/latest/download/Rastro-windows-x64.exe)**. This link always gets the latest version.
2. Put it wherever you like: your desktop, a folder in Documents, anywhere. It doesn't need any other file next to it.
3. Double-click it. If Windows shows a blue warning, see [Is it safe?](#is-it-safe).

Requires Windows 10 or 11, 64-bit.

### First launch

A short guide opens the first time:

1. **Screenshots folder.** Rastro finds `Documents\Escape from Tarkov\Screenshots` on its own, including when Windows has moved Documents into OneDrive. If it can't, paste the path and it tells you how many screenshots with coordinates it sees. Here you can also turn on raid detection.
2. **Maps.** Download the community maps from tarkov.dev (recommended), import a map pack, or try the demo map.
3. Done. Take a screenshot in game and your arrow appears on the map.

To use the Market, the Ammo table and key names, open the Market tab once and press *Download prices*.

### Updating

If you allowed update checks, a button appears in the top bar when a new version is out and takes you to its download page. Otherwise, download the new `.exe` from the [link above](#install), close Rastro and use the new file instead of the old one. Your settings, points, maps and prices are kept [separately](#where-your-data-lives), so nothing is lost.

After updating, Rastro shows what changed. The full history is in *Settings → About → What's new* and in [CHANGELOG.md](CHANGELOG.md). If an update adds new map data, use *Update from tarkov.dev* in the Maps panel to get it.

### Shortcuts

| Action | How |
|---|---|
| Update your position | Take a screenshot in game |
| Switch section (Map, Market, Ammo, Quests) | `Ctrl+1`, `Ctrl+2`, `Ctrl+3`, `Ctrl+4` |
| Command palette | `Ctrl+K` |
| Extracts, points, party, maps and quests panels | `1`, `2`, `3`, `4`, `5` |
| New raid (clear trail and marks) | `N` |
| Fit the whole map / follow your position | `F` / `G` |
| Zoom | `+` / `-` or the mouse wheel |
| Add a point, guide to a place | Right-click on the map |
| Ping your squad | `Shift` + click |
| Search the Market | `/` |
| Settings | `,` |

### Where your data lives

Settings, your points, your quest progress, downloaded maps and the price and quest caches are kept in one folder, separate from the `.exe`:

| System | Folder |
|---|---|
| Windows | `%APPDATA%\Rastro\data\` |
| Linux | `~/.local/share/rastro/` |
| macOS | `~/Library/Application Support/Rastro/` |

*Settings → Log* has a button to open it.

### If something doesn't work

- **It opens in my browser instead of its own window.** Rastro uses Microsoft WebView2 to draw its window. It ships with Windows 10 and 11, but if it was removed, Rastro falls back to your browser. Installing the [WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/) brings the window back.
- **An error page says the port is already in use.** Another copy of Rastro is probably still running. Close it from the taskbar or from Task Manager (look for `rastro.exe`) and open it again.
- **My position never moves.** Check that the screenshots folder in *Settings → General* is the one the game actually saves to, and that you're pressing the in-game screenshot key.
- **My teammates can't join my room.** They need to reach your PC. Use the address from your mesh VPN (Tailscale, ZeroTier) when you open the room, or open the party port on your router.
- **Still stuck?** Ask on [Discord](https://discord.gg/H4pRxGBGDb). *Settings → Log* has a *Copy* button, and pasting that log helps a lot.

## Questions

**Can I get banned for using it?**
Rastro never reads game memory, never touches the game process and never sends input to the game. It reads screenshot file names and log files after the game has written them, the same files you can open yourself in Explorer. We can't speak for Battlestate Games, though, so see the note in [Is it safe?](#is-it-safe).

**Does it work in PvE?**
Yes. Raid detection has been tested with PvE logs, and Rastro downloads PvE prices and data when it detects PvE.

**Does it need the internet?**
Only to download maps and prices, check for updates (if you allow it) and play in a squad. Everything else works offline.

**Why screenshots?**
They're the only place the game writes your exact position to disk. It's a few seconds of delay in exchange for never touching the game.

**Linux or macOS?**
Rastro builds and runs on Linux; macOS is untested. Tarkov itself runs on Windows, so the download is Windows only. See [Building from source](#building-from-source).

## Privacy and network use

Rastro has no account, no telemetry and no ads. The interface talks only to the app itself on `127.0.0.1`, protected by a random key that changes on every launch. Everything it reads from your disk stays on your PC. Anything else happens only when you ask for it:

| When | Connects to |
|---|---|
| Always | Nothing outside your PC |
| You download maps from tarkov.dev | `raw.githubusercontent.com`, `json.tarkov.dev` and, as a fallback, `api.tarkov.dev` |
| You download or refresh prices | `json.tarkov.dev`, plus `assets.tarkov.dev` for item icons |
| You download or refresh quests | `json.tarkov.dev` and, as a fallback, `api.tarkov.dev`, plus `assets.tarkov.dev` for pictures |
| You import a map pack from a URL | Only the address you enter (HTTPS only, never internal addresses) |
| You host or join a party | Directly to your teammates' PCs |
| Once per launch, if you said yes to update checks (or when you press *Check now*) | `api.github.com`, to read the latest version number. Nothing about you is sent. |

Every download host is on a fixed allowlist, also checked on redirects, and everything downloaded is validated before use. The full threat model is in [docs/SECURITY.md](docs/SECURITY.md).

## Get involved

Rastro is built in the open by a small team, and we want to grow it together with the people who use it. Whether you have five seconds or five hours a week, there's a way to help.

### ⭐ Show your support

- **Star the repository.** It's free, takes a click and is the main way new players find Rastro on GitHub.
- **Watch → Custom → Releases** to get notified when a new version ships.
- **Tell your squad.** Party mode is better with friends, and word of mouth is how most people find us.
- **Share a clip or a screenshot** of Rastro in action and tag us on Discord.

### 💬 Join the community

The **[Rastro Discord](https://discord.gg/H4pRxGBGDb)** is where everything happens: release announcements, the live roadmap, help from other players, forums for bug reports and feature requests, and a place to find a squad to try party mode with. Development happens there day to day, so it's the best place to see what's coming before anyone else.

### 🙋 We're looking for people

We have far more ideas than hands. If any of these sounds like you, we'd love to hear from you:

| Role | What you'd do |
|---|---|
| 🛡️ **Moderators** | Keep the Discord friendly and helpful as the community grows. |
| 🌍 **Translators** | Review one of the 14 languages that native speakers haven't checked yet, or keep yours up to date. |
| 🦀 **Developers** | Rust on the backend, React and TypeScript on the interface. From small fixes to whole features. |
| 🧪 **Testers** | Try new versions early, especially in PvP, on less-played maps and on different setups. |
| 🗺️ **Map and data helpers** | Check positions, floors and locks against the game, and help with maps tarkov.dev doesn't cover yet. |
| 🎥 **Content creators** | Streamers and video makers who want to show Rastro or give feedback from their community. |

**How to apply:** open a *Join the team* ticket in the support channel on [Discord](https://discord.gg/H4pRxGBGDb). Developers and translators can also jump straight in with a pull request: [CONTRIBUTING.md](CONTRIBUTING.md) explains how.

### 💡 Ideas, feedback and bugs

Honest feedback is the most valuable thing you can give a beta. Tell us what's confusing, what's missing and what you'd use every raid:

- **Ideas:** the feature requests forum on [Discord](https://discord.gg/H4pRxGBGDb), or a [GitHub issue](https://github.com/MaxSala/rastro-tarkov/issues).
- **Bugs:** the bug reports forum on Discord, or a GitHub issue. Include your Rastro version (*Settings → About*) and what you did. Please never post full game logs publicly, since they contain your account ID.
- **Security issues:** never in public. Use GitHub's private reporting (*Security* tab → *Report a vulnerability*) or a private *Security & privacy* ticket on Discord. Details in [docs/SECURITY.md](docs/SECURITY.md).

## Credits

Rastro would not exist without **[tarkov.dev](https://tarkov.dev)**, the open-source Escape from Tarkov database maintained by the **[The Hideout](https://github.com/the-hideout)** community. tarkov.dev began as [tarkov-tools](https://github.com/kokarn/tarkov-tools) by **Oskar Risberg ([@kokarn](https://github.com/kokarn))** and was revived and opened up by The Hideout's contributors. Rastro uses:

- **Maps:** the SVG maps from [the-hideout/tarkov-dev-svg-maps](https://github.com/the-hideout/tarkov-dev-svg-maps) by tarkov.dev's contributors, licensed [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/). Their authors don't allow the maps to be used in cheats or software that gives an unfair advantage, such as radars. Maps are downloaded by you and never redistributed with Rastro, and each installed map keeps its credits and license.
- **Data:** extracts, transits, switches, containers, locks, items, prices, quests and translations from the tarkov.dev API and its data files.
- **Marker icons:** from [tarkov-dev](https://github.com/the-hideout/tarkov-dev) (`public/maps/interactive`), MIT License, © 2019 Oskar Risberg.
- **Item icons** belong to Battlestate Games and are only cached locally for display.

If you find Rastro useful, consider supporting tarkov.dev, contributing to it or joining their community.

Rastro is also built on many other open-source projects, including Rust, Tokio, Axum, Snow, Rustls, Wry, React, Leaflet, Tailwind CSS and the Geist typeface. The full list with licenses is in [NOTICE.md](NOTICE.md).

*Escape from Tarkov* is a trademark of Battlestate Games. Game content and materials are trademarks and copyrights of Battlestate Games and its licensors.

## Non-commercial

Rastro is a hobby project made for the community. It's free, there's no paid version, it shows no ads and collects no data. It will stay that way, both by choice and because the tarkov.dev maps it relies on are licensed for non-commercial use only.

## Known limitations

- Your position only updates when you take a screenshot. That's by design: it's the only place the game writes your position.
- Labs has no SVG map on tarkov.dev. Rastro recognises it, but can't show it unless you import your own map pack.
- Raid detection has been verified with PvE logs. PvP should behave the same, but hasn't been confirmed with real logs.
- Spanish and English translations are reviewed. The other 14 haven't been checked by native speakers yet. Corrections are very welcome, here or on [Discord](https://discord.gg/H4pRxGBGDb).
- Changing the language or game mode (PvP/PvE) needs a fresh download of maps and prices to see the new names.
- Some technical details inside error messages and the log are in English whatever language you pick.

## Building from source

You only need this if you want to change the code or build the `.exe` yourself. You'll need [Rust](https://rustup.rs/) 1.85 or newer and [Node.js](https://nodejs.org/) LTS (on Linux, also `libwebkit2gtk-4.1-dev` and `libgtk-3-dev`):

```sh
git clone https://github.com/MaxSala/rastro-tarkov.git
cd rastro-tarkov
./build.sh        # build.bat on Windows
```

The result is a single executable in `target/release/`. Development setup, project layout, code rules and how to add a translation are in **[CONTRIBUTING.md](CONTRIBUTING.md)**.

## License

Rastro's source code is released under the [MIT License](LICENSE). Maps and data downloaded from tarkov.dev keep their own licenses, listed under [Credits](#credits) and in [NOTICE.md](NOTICE.md).

---

<p align="center">
  <b>If Rastro helped you find your way out, <a href="https://github.com/MaxSala/rastro-tarkov/stargazers">give it a ⭐</a> and <a href="https://discord.gg/H4pRxGBGDb">come say hi on Discord</a>.</b>
  <br>
  <sub>Made by players, for players. See you at the extract.</sub>
</p>
