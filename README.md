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

<p align="center">Free and open source · No installer · No account · Nothing to configure</p>

---

**Rastro is a live map of your raid for your second monitor.** Take a screenshot in game and your position appears, with the way to every extract. Your quests, your hideout, the bosses and your squad, on the same screen, without alt-tabbing.

It never touches the game: it only reads files Tarkov already writes to your disk. Free, open source, made by players for players.

<p align="center">
  <img src="docs/images/hero.png" alt="Customs in raid: your position, a raid plan with its route, the bosses and the time left." width="100%">
  <br><sub>Customs in raid: your position, a raid plan with its route, the bosses and the time left.</sub>
</p>

<table>
  <tr><td width="50%" valign="top"><img src="docs/images/squad.png" alt="Play as a squad"><br><b>Play as a squad</b><br>Everyone on the same map, encrypted PC to PC. No server, no account.</td><td width="50%" valign="top"><img src="docs/images/quests.png" alt="Quests that follow the game"><br><b>Quests that follow the game</b><br>Ticked off from the game logs, with every open objective on the map.</td></tr>
  <tr><td width="50%" valign="top"><img src="docs/images/hideout.png" alt="Hideout"><br><b>Hideout</b><br>What each next level needs, and which items not to sell.</td><td width="50%" valign="top"><img src="docs/images/bosses.png" alt="Bosses on the map"><br><b>Bosses on the map</b><br>Where each boss can spawn and the chance it's there, PvP and PvE.</td></tr>
  <tr><td width="50%" valign="top"><img src="docs/images/clock.png" alt="Raid clock"><br><b>Raid clock</b><br>Time left, Tarkov time on both sides and the armored train window.</td><td width="50%" valign="top"><img src="docs/images/market.png" alt="Prices and ammo"><br><b>Prices and ammo</b><br>Flea and trader prices, keys and every round, searched offline.</td></tr>
</table>

## Is it safe?

- Reads only files the game writes to disk: the **names** of your screenshots and, if you turn it on, the log files.
- Never reads game memory, injects input or draws over the game.
- Listens only on your own PC (`127.0.0.1`). No telemetry, no account.
- Open source (MIT). Every release is built by GitHub Actions from this code.

The full answer, the Windows SmartScreen warning and how to check the file: [Questions and safety](docs/en/FAQ.md).

## Get started

1. **Download** [`Rastro-windows-x64.exe`](https://github.com/MaxSala/rastro-tarkov/releases/latest/download/Rastro-windows-x64.exe) and run it. There's nothing to install.
2. **Follow the short setup.** It finds your screenshots folder and downloads the maps from tarkov.dev.
3. **Take a screenshot in game.** Your arrow appears on the map.

## Learn more

**[Features](docs/en/FEATURES.md)** · **[Guide: install, shortcuts and data](docs/en/GUIDE.md)** · **[Questions and safety](docs/en/FAQ.md)** · **[Roadmap](docs/ROADMAP.md)** · **[What's new](CHANGELOG.md)** · **[Contributing](CONTRIBUTING.md)**

> [!TIP]
> Rastro is in public beta and the community decides what comes next. **[⭐ Star the repo](https://github.com/MaxSala/rastro-tarkov/stargazers)** and **[join the Discord](https://discord.gg/H4pRxGBGDb)**: it takes five seconds and helps a lot.

---

<sub>Map imagery by the <a href="https://github.com/the-hideout/tarkov-dev-svg-maps">tarkov.dev contributors</a>, CC BY-NC-SA 4.0. Prices and item data from <a href="https://tarkov.dev">tarkov.dev</a>. Squad screenshots use example positions.</sub>
<br><sub>*Escape from Tarkov* is a trademark of Battlestate Games. Rastro is not affiliated with or endorsed by Battlestate Games or tarkov.dev.</sub>
<br><sub>MIT License · [Credits and third-party licenses](NOTICE.md)</sub>
