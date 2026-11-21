<p align="center">
  <a href="../../README.md">Back to the README</a> · <a href="FEATURES.md">Features</a> · <a href="GUIDE.md">Guide</a> · <b>Questions and safety</b>
</p>

# Questions and safety

## Is it safe?

Rastro is a small, portable program. Here's what that means and how you can check it.

- **No installer, no admin rights.** It's a single `.exe`. It doesn't install services or drivers, doesn't start with Windows and doesn't touch the game's files. Its settings and downloaded maps live in one folder (see [Where your data lives](GUIDE.md#where-your-data-lives)), and deleting that folder and the `.exe` removes it completely.
- **Open source.** All the code is in this repository. Anyone can read it.
- **Built in public.** Every release is built by GitHub's own servers straight from the source code, not on someone's PC. You can see each build in the [Actions](https://github.com/MaxSala/rastro-tarkov/actions) tab, and the release page lists the file's SHA-256 so you can check you got exactly that file:

  ```powershell
  Get-FileHash .\Rastro-windows-x64.exe
  ```

- **Doesn't phone home.** No telemetry, no ads, no account. It only connects to the internet when you ask it to, and the full list of connections is in [Privacy](#privacy-and-network-use).

**"Windows protected your PC."** The first time you open Rastro, Windows SmartScreen will most likely show a blue window saying it stopped an unrecognised app. This happens with any new program that isn't signed with a paid certificate, not because something is wrong with the file. Click **More info**, then **Run anyway**. Windows remembers your choice. Some antivirus programs are wary of new unsigned programs too. If yours flags Rastro, you can restore it and add an exception, upload the file to [VirusTotal](https://www.virustotal.com/) to get a second opinion, and report it as a false positive to your antivirus vendor.

> [!IMPORTANT]
> Rastro is an unofficial fan project, not affiliated with or endorsed by Battlestate Games or tarkov.dev. It's built to stay out of the game: it only reads files on disk and never interacts with the game process. Even so, using third-party tools with Escape from Tarkov is your own responsibility under Battlestate Games' terms of service.

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
Rastro builds and runs on Linux; macOS is untested. Tarkov itself runs on Windows, so the download is Windows only. See [Building from source](GUIDE.md#building-from-source).

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

Every download host is on a fixed allowlist, also checked on redirects, and everything downloaded is validated before use. The full threat model is in [docs/SECURITY.md](../SECURITY.md).

## Known limitations

- Your position only updates when you take a screenshot. That's by design: it's the only place the game writes your position.
- Labs has no SVG map on tarkov.dev. Rastro recognises it, but can't show it unless you import your own map pack.
- Raid detection has been verified with PvE logs. PvP should behave the same, but hasn't been confirmed with real logs.
- Spanish and English translations are reviewed. The other 14 haven't been checked by native speakers yet. Corrections are very welcome, here or on [Discord](https://discord.gg/H4pRxGBGDb).
- Changing the language or game mode (PvP/PvE) needs a fresh download of maps and prices to see the new names.
- Some technical details inside error messages and the log are in English whatever language you pick.

## Non-commercial

Rastro is a hobby project made for the community. It's free, there's no paid version, it shows no ads and collects no data. It will stay that way, both by choice and because the tarkov.dev maps it relies on are licensed for non-commercial use only.
