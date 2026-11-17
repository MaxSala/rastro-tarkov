# Changelog

All notable changes to Rastro. Each version lists what was added, changed and fixed; the newest is at the top.

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
