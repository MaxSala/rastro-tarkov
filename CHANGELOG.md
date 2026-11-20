# Changelog

All notable changes to Rastro. Each version lists what was added, changed and fixed; the newest is at the top.

## [0.6.2] - 2026-11-20

### Added
- Quests follow the game. With raid detection on, the quests you accept, hand in or fail are ticked off by themselves, straight from the game logs, separately for PvP and PvE. The first time, Rastro reads every launch the game has kept, so your progress catches up on its own. Accepting a quest also marks everything it requires as done and raises your level to fit.
- *Catch up*: for each trader, pick the last quest you completed and everything before it is marked as done.
- Raid plan in the Quests panel (`5`): tick the quests you're doing this raid and the map shows only those, numbered along the shortest route, with what to bring (markers, items to plant, keys).
- *Next*: the panel shows the next objective in big, with its distance and a button to be guided there.
- *To hand in*: quests with every objective ticked, and which trader to see.
- *Items*: what your quests still ask you to hand over, found in raid first, so you don't sell it.
- The Quests section is organised by trader, like the game, with *Now* (what you're on and what you can start), your Kappa and Lightkeeper progress, and the map where you can get the most done.
- *Wrong spot?* next to each objective opens a GitHub issue with that objective's data, so bad positions get fixed.

### Changed
- The map says how sure each quest spot is: a badge for a precise place, the real outline for an area, and small dots for the places an item *can* spawn (it's in one of them). Before, a list of possible spawns looked like a list of places to go.
- Each quest has its own colour on the map when several share it.
- A zone's floor is now taken from its bottom rather than its middle, so objectives in tall rooms or stairwells show on the right floor.

### Fixed
- The newest game log session is now found by date, not by name: the game writes the hour without a leading zero, so a morning session could be read before an evening one.

## [0.6.0] - 2026-11-19

### Added
- Quests. A new Quests section (`Ctrl+4`) with every quest from tarkov.dev, in your language: what you're on, what you can start next and, for the rest, exactly what's missing (your level or the quests that come first). Set your level and faction, filter by trader, map or Kappa, and search quests, objectives and items.
- Each quest shows its trader's portrait, the quest's picture and its rewards: experience, reputation, items, the trader offers and crafts it unlocks.
- Your active quests on the map. Open objectives appear as amber markers whose symbol says what to do there (go, mark, plant, find, shoot, extract). Areas show their real outline on the ground. Hover one for a card; click it and Rastro guides you there with distance, direction and height, like an extract. The new Quests panel (`5`) lists what you can do on the map you're on, nearest first, so you can tick objectives off or count kills as you go.
- See any quest on its map before taking it, with *Show on map*.
- *Mark the previous quests as done*: if you start using Rastro halfway through a wipe, one click brings your progress up to date.
- The Market tells you which of your quests still ask for an item and how many you need, so you don't sell it by mistake.
- Quest progress is saved on your PC, separately for PvP and PvE. Rastro only connects to tarkov.dev when you download or refresh the quests, and to fetch the pictures you open.
- Changing the language now offers to bring your maps, prices and quests over to it, or does it on its own if you choose so (*Settings → General*). Maps aren't downloaded again: only their names and texts are rebuilt.

### Fixed
- Running a command from the command palette that opens a dialog, such as *What's new*, no longer closes that dialog straight away.
- Quest spots that fall outside the map image (bad data) are no longer drawn.

## [0.5.0] - 2026-11-17

### Added
- Floor switcher on the map. Maps with several floors show a panel at the top left, stacked like the building. The floor you're looking at is highlighted, the rest of the map is dimmed, and extracts, containers, locks, points and teammates on other floors fade out so you can tell at a glance what's on your level.
- Hover cards. Hovering anything on the map shows what it is, its name, the distance from you, the floor it's on when that isn't the one shown, and what clicking does. Markers grow slightly on hover and hidden labels appear.
- The setup guide now explains raid detection, which reads the game's log files, and lets you turn it on right there.

### Changed
- *Settings → About* and the README now say plainly that Rastro reads two things: screenshot file names and, if you turn on raid detection, the game's log files.
- Every release now has the same file name, `Rastro-windows-x64.exe`, and is published as the latest release as soon as it's built. The download link in the README always gets the newest version.
- English floor names read naturally ("2nd floor" instead of "Floor 2").

### Fixed
- Turning raid detection on or off no longer resets a custom logs folder.
- A few internal error messages were still in Spanish.

## [0.4.0] - 2026-11-17

### Added
- Update notifications. Rastro asks once whether it may check GitHub for new versions when it starts. If you agree and a new version is out, a button in the top bar takes you to its download page. You can change your mind, or check by hand, in *Settings → About*.
- What's new. After updating, Rastro shows what changed in the new version. The full history is in *Settings → About*.

### Changed
- Everything Rastro tells you now follows the language you chose: error messages, party connection problems, map download progress, floor names and the switches an extract needs. Before, many of these stayed in Spanish.
- Technical messages that aren't translated (the log and rare errors) are now in English instead of Spanish.
- The demo map and the default player name are now in English.

### Fixed
- Floor names such as "Ground floor" stayed in Spanish whatever language you picked. Re-download the maps from the Maps panel to refresh them.
- The "requires switch" note on extracts stayed in Spanish.
- Missing item icons no longer fill the developer console with errors when you're offline.

## [0.3.0] - 2026-11-16

### Added
- Keys on the map. The new *Locks* layer shows every door, container and car trunk that needs a key. Click one to see which key opens it, where to buy it and what it sells for, or highlight every lock that key opens.
- Every key in the Market lists the installed maps it's used on, with a link straight to it.
- The interface in the 16 languages tarkov.dev supports.

### Fixed
- A map with an unnamed lock in tarkov.dev's data no longer gets rejected when installing.
- The Market no longer sticks to the previous catalog after changing language or game mode.

## [0.2.0] - 2026-11-16

### Added
- Market: search every item offline, with flea and trader prices, sorted by value per slot.
- Ammo table: damage, penetration, armor damage, fragmentation and velocity for every round, with the cheapest place to buy it.
- Map, Market and Ammo sections (`Ctrl+1`, `Ctrl+2`, `Ctrl+3`).
- English interface, loading screen and a new logo, also on the `.exe`.

## [0.1.0] - 2026-11-15

### Added
- Your position on the map from the screenshots you take in game, with a fading trail.
- Extracts with distance and direction, your own points, and an end-to-end encrypted party to share positions with your squad.
- One-click install of the tarkov.dev community maps, with transits, switches, containers and multi-floor maps.
- Automatic map and raid detection from the game's log files (optional).
- Desktop window, no console.
