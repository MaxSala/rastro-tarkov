# Roadmap

Ideas for future versions, roughly grouped. None of them has a date, and the order within each group isn't a priority list. Suggestions are welcome on [Discord](https://discord.gg/H4pRxGBGDb) or as GitHub issues.

Every idea here has to fit the project's ground rule: Rastro only reads files the game writes to disk. Nothing that reads game memory, injects input or draws over the game will be added.

## Released

- **0.4.0**: every app message in the user's language, update notifications (opt-in) and a *What's new* section.
- **0.3.0**: keys on the map (locks layer, key cards, key ↔ map links) and the interface in 16 languages.
- **0.2.0**: Market with offline price search, Ammo table, loading screen and the new logo.

## Map

- Quest objectives and loose loot as map layers. Containers, transits, switches and locks already come from tarkov.dev.
- Mark the keys you own, so the locks you can open stand out.
- A built-in editor to calibrate custom maps by clicking on them.

## Market and ammo

- Favourites and a shopping list, with a notice when their price changes after a refresh.
- A penetration chart against each armor class in the Ammo table.

## Game logs

- Raid history: map, duration and time spent in the queue.
- Track accepted and completed quests, which appear in the logs as notifications. The exact format still needs to be confirmed with real logs.

## App

- System tray icon and an option to start minimised.
- Export and import your own points, to move them between PCs or share them with your party.
- Optional cleanup of old screenshots. Today Rastro never touches them, and any cleanup would stay strictly opt-in.
- Native-speaker review of the 14 translations other than Spanish and English.
- A signed `.exe`, so Windows SmartScreen doesn't warn on first launch.

## Not planned

- **A LAN mode for the local server.** Rastro's interface is only reachable from the same PC, on purpose. Sharing positions is what the end-to-end encrypted party is for.
