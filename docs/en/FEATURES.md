<p align="center">
  <a href="../../README.md">Back to the README</a> · <b>Features</b> · <a href="GUIDE.md">Guide</a> · <a href="FAQ.md">Questions and safety</a>
</p>

# Features

<table>
  <tr><td width="50%"><img src="../images/hero.png" alt="Customs in raid"><br><sub>Customs in raid</sub></td><td width="50%"><img src="../images/squad.png" alt="Your squad on one map"><br><sub>Your squad on one map</sub></td></tr>
  <tr><td width="50%"><img src="../images/quests.png" alt="Quests by trader"><br><sub>Quests by trader</sub></td><td width="50%"><img src="../images/hideout.png" alt="Hideout"><br><sub>Hideout</sub></td></tr>
  <tr><td width="50%"><img src="../images/bosses.png" alt="Bosses and their spawn points"><br><sub>Bosses and their spawn points</sub></td><td width="50%"><img src="../images/clock.png" alt="Raid clock and train"><br><sub>Raid clock and train</sub></td></tr>
  <tr><td width="50%"><img src="../images/keys.png" alt="Locks and key prices"><br><sub>Locks and key prices</sub></td><td width="50%"><img src="../images/floors.png" alt="Floors, automatically"><br><sub>Floors, automatically</sub></td></tr>
  <tr><td width="50%"><img src="../images/market.png" alt="Market"><br><sub>Market</sub></td><td width="50%"><img src="../images/ammo.png" alt="Ammo table"><br><sub>Ammo table</sub></td></tr>
</table>

<sub>Map imagery by the <a href="https://github.com/the-hideout/tarkov-dev-svg-maps">tarkov.dev contributors</a>, CC BY-NC-SA 4.0. Prices and item data from <a href="https://tarkov.dev">tarkov.dev</a>. Squad screenshots use example positions.</sub>

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

**Hideout**
- Set the level of each station and see what the next one asks for: items, other stations, trader loyalty, skills and build time. Requirements turn green as you meet them.
- *Items* adds up quests and hideout: what both still ask for, for the next level or for every level left. The Market's item card says which stations need it.

**Bosses and raid clock**
- The *Bosses* layer shows where each boss can appear and the chance of meeting it there. Hover for the zone, escorts and arrival time; click to see every spawn point. Chances follow your game mode.
- In the menu, the top bar shows the time in Tarkov on both sides, to pick day or night. In a raid: the time left and, from your last screenshot, the time in your raid.
- Click it for when the raid ends, the armored train window on Reserve and Lighthouse, and the bosses of the map.

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
- Switches to the right map when a raid loads, resets your trail and marks at the start of each raid, and shows the time left. Reads the game's log files, as described in [How it works](GUIDE.md#how-it-works).

**Everything else**
- 16 languages, the same ones tarkov.dev supports: Čeština, Deutsch, English, Español, Français, Magyar, Italiano, 日本語, 한국어, Polski, Português, Română, Русский, Slovenčina, Türkçe and 中文. Item, map and extract names follow your language too. Change it and Rastro offers to bring your maps, prices and quests over.
- PvP and PvE data, picked automatically from the game logs or by hand.
- Optional update notifications and a *What's new* page after every update.
- Keyboard shortcuts for everything, and a command palette (`Ctrl+K`).

## Play with your squad

Rastro's party mode puts your whole squad on the same map, each with their own name and colour.

- **See where everyone is.** Every teammate's arrow shows where they were and which way they were facing at their last screenshot. If someone is on a different floor, their arrow fades out and hovering it tells you which floor they're on.
- **Ping a spot.** `Shift` + click anywhere on the map and the spot flashes on everyone's screen.
- **Easy to set up.** One player opens a room and copies an invite code. The others paste it under *Join*. That's it. Up to 8 players.
- **No server, no account.** Positions go straight from PC to PC. Nobody else, including us, ever sees them.
- **Encrypted.** All traffic is end-to-end encrypted with the [Noise protocol](https://noiseprotocol.org/). The invite code is the key, so only share it with your squad.

The host needs to be reachable by the others. The easiest way is a free mesh VPN such as [Tailscale](https://tailscale.com/) or [ZeroTier](https://www.zerotier.com/): everyone installs it once and Rastro works as if you were on the same network, with no ports to open on your router. Opening a port on the host's router works too.

Each player still runs Rastro on their own PC and takes their own screenshots: that's what keeps everyone's position up to date.
