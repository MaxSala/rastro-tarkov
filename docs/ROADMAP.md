# Roadmap

Rastro is in public beta, and this is where it's heading. Nothing here has a date, and the order isn't fixed: it's shaped by what players ask for. Suggestions are welcome in the feature requests forum on [Discord](https://discord.gg/H4pRxGBGDb) or as [GitHub issues](https://github.com/MaxSala/rastro-tarkov/issues), and the most requested ideas move up.

Every idea here has to fit the project's ground rule: Rastro only reads files the game writes to disk. Nothing that reads game memory, injects input or draws over the game will be added.

## Next up

These are the big features we want to build next.

- **Squad quests.** In a party, see your teammates' pending quests on the same map, so the squad can plan one route that helps everyone.
- **Hideout tracker.** Every hideout upgrade, what it needs and which items you still have to keep, linked to their prices in the Market.

## Map

- Loose loot as a map layer. Containers, transits, switches and locks already come from tarkov.dev.
- Your key ring: mark the keys you own, so the locks you can open stand out on every map.
- A built-in editor to calibrate custom maps by clicking on them.

## Market and ammo

- Favourites and a shopping list, with a notice when their price changes after a refresh.
- A penetration chart against each armor class in the Ammo table.

## Game logs

- Raid history: map, duration, game mode and time spent in the queue.

## App

- Export and import your own points, to move them between PCs or share them with your party.
- System tray icon and an option to start minimised.
- Optional cleanup of old screenshots. Today Rastro never touches them, and any cleanup would stay strictly opt-in.
- Native-speaker review of the 14 translations other than Spanish and English.
- A signed `.exe`, so Windows SmartScreen doesn't warn on first launch.

## Released

- **0.6.2**: quests follow the game logs (with the history), catch-up per trader, raid plan with route and kit, next objective, hand-in list, items to keep, honest quest spots (exact, area, possible spawns), quests by trader with Kappa and Lightkeeper progress.
- **0.6.0**: quest tracker: every quest in your language with its trader, picture and rewards, what's next and why the rest is locked, your active objectives on the map with guidance and their real areas, the items your quests need in the Market, and maps, prices and quests that follow your language.
- **0.5.0**: floor switcher with dimming of other floors, hover cards on the map, clearer explanation of what Rastro reads.
- **0.4.0**: every app message in the user's language, update notifications (opt-in) and a *What's new* section.
- **0.3.0**: keys on the map (locks layer, key cards, key ↔ map links) and the interface in 16 languages.
- **0.2.0**: Market with offline price search, Ammo table, loading screen and the new logo.

## Not planned

- **A LAN mode for the local server.** Rastro's interface is only reachable from the same PC, on purpose. Sharing positions is what the end-to-end encrypted party is for.

## Help build it

We have more ideas than hands. If you'd like to help as a developer, translator, tester or moderator, see [CONTRIBUTING.md](../CONTRIBUTING.md) or open a *Join the team* ticket on [Discord](https://discord.gg/H4pRxGBGDb).
