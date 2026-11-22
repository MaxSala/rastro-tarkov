<p align="center">
  <a href="https://maxsala.github.io/rastro-tarkov/docs/guide/">Website</a> · <a href="../../README.md">Back to the README</a> · <a href="FEATURES.md">Features</a> · <b>Guide</b> · <a href="FAQ.md">Questions and safety</a>
</p>

# Guide

## Install

1. **[Download `Rastro-windows-x64.exe`](https://github.com/MaxSala/rastro-tarkov/releases/latest/download/Rastro-windows-x64.exe)**. This link always gets the latest version.
2. Put it wherever you like: your desktop, a folder in Documents, anywhere. It doesn't need any other file next to it.
3. Double-click it. If Windows shows a blue warning, see [Is it safe?](FAQ.md#is-it-safe).

Requires Windows 10 or 11, 64-bit.

### First launch

A short guide opens the first time:

1. **Screenshots folder.** Rastro finds `Documents\Escape from Tarkov\Screenshots` on its own, including when Windows has moved Documents into OneDrive. If it can't, paste the path and it tells you how many screenshots with coordinates it sees. Here you can also turn on raid detection.
2. **Maps.** Download the community maps from tarkov.dev (recommended), import a map pack, or try the demo map.
3. Done. Take a screenshot in game and your arrow appears on the map.

To use the Market, the Ammo table and key names, open the Market tab once and press *Download prices*.

### Updating

If you allowed update checks, a button appears in the top bar when a new version is out and takes you to its download page. Otherwise, download the new `.exe` from the [link above](#install), close Rastro and use the new file instead of the old one. Your settings, points, maps and prices are kept [separately](#where-your-data-lives), so nothing is lost.

After updating, Rastro shows what changed. The full history is in *Settings → About → What's new* and in [CHANGELOG.md](../../CHANGELOG.md). If an update adds new map data, use *Update from tarkov.dev* in the Maps panel to get it.

### Shortcuts

| Action | How |
|---|---|
| Update your position | Take a screenshot in game |
| Switch section (Map, Market, Ammo, Quests, Hideout) | `Ctrl+1`, `Ctrl+2`, `Ctrl+3`, `Ctrl+4`, `Ctrl+5` |
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

## How it works

Rastro reads two kinds of files that Escape from Tarkov already saves on your PC. It doesn't need anything else.

| What it reads | Where | What Rastro gets from it |
|---|---|---|
| **The file names of your screenshots** | `Documents\Escape from Tarkov\Screenshots` | Your position and the direction you're facing. Tarkov writes your coordinates into the name of every screenshot it saves. Rastro reads the name, never the picture, and never moves, edits or deletes your screenshots. |
| **The game's log files** (optional, off until you turn it on) | `Battlestate Games\Escape from Tarkov\Logs` | Which map is loading, when a raid starts and whether you're playing PvP or PvE. Rastro uses that to switch to the right map by itself, start a fresh trail every raid and show how much raid time is left. Only the map name and game mode are kept. It also reads the trader messages, to tick off the quests you accept, hand in or fail. |

So in practice: you press your screenshot key in game, and a second later your arrow moves on the map.

What Rastro **doesn't** do: it doesn't read game memory, inject anything, hook into the game process, capture network traffic, draw over the game or press keys for you. It runs as a separate program and only looks at files on disk.

## Building from source

You only need this if you want to change the code or build the `.exe` yourself. You'll need [Rust](https://rustup.rs/) 1.85 or newer and [Node.js](https://nodejs.org/) LTS (on Linux, also `libwebkit2gtk-4.1-dev` and `libgtk-3-dev`):

```sh
git clone https://github.com/MaxSala/rastro-tarkov.git
cd rastro-tarkov
./build.sh        # build.bat on Windows
```

The result is a single executable in `target/release/`. Development setup, project layout, code rules and how to add a translation are in **[CONTRIBUTING.md](../../CONTRIBUTING.md)**.
