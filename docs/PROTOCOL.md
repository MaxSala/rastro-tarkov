# Interface ↔ backend protocol

Rastro's interface talks to its backend over a single WebSocket at `/ws`, using JSON messages tagged by `type`, plus a handful of HTTP endpoints for larger or binary data.

The source of truth is `src/protocol.rs` and `src/view.rs`. The interface mirrors both in `web/src/lib/protocol.ts` and validates every incoming message with zod.

## Connecting

1. The interface opens `ws://127.0.0.1:<port>/ws`. The server requires exact `Host` and `Origin` headers.
2. The first message, sent within 5 seconds, must be `{"type":"auth","token":"<64 hex characters>"}`. Otherwise the server closes the socket.
3. The server answers with a `snapshot`, and sends a new one every time something changes (bursts are coalesced).

Limits: 64 KiB per incoming message and 4 sockets at a time.

## Interface → backend (`ClientMsg`)

| `type` | Fields | Effect |
|---|---|---|
| `auth` | `token` | Authenticates the socket. Only valid as the first message. |
| `setActiveMap` | `mapId` | Switches the active map. |
| `newRaid` | | Clears the trail, last position, selection and extract availability. |
| `selectExtract` | `extractId` or `null` | Starts or stops guidance. Also accepts point IDs (`pt-…`) and quest objective spots (`qo-…`). |
| `toggleExtract` | `extractId` | Marks or unmarks an extract as available this raid, on the active map. |
| `updateSettings` | `settings { screenshotsDir, trailLength, playerName, playerColor, partyPort }` | Validates and saves the settings. |
| `ackShareWarning` | | Stops showing the warning about sharing the invite code. |
| `partyHost` | `bindIp`, `advertise` | Opens a party room. |
| `partyJoin` | `code` | Joins a room. |
| `partyLeave` | | Leaves the room, or closes it when hosting. |
| `partyKick` | `peerId` | Removes a peer. Host only. |
| `mapPing` | `px`, `py` | Pings a spot, given in map pixels; the backend converts it to game coordinates. |
| `listInterfaces` | | Replies with `interfaces`. |
| `removePack` | `packId` | Deletes an installed map pack. |
| `tarkovDevSync` | | Downloads and installs the tarkov.dev maps. Progress is reported in `snapshot.view.tarkovdev`. |
| `finishOnboarding` | | Marks the first-run guide as done. |
| `checkFolder` | `path` | Replies with `folderCheck`: whether the folder exists and how many screenshots with coordinates it holds. |
| `setScreenshotsDir` | `path` | Sets the screenshots folder and restarts the watcher. |
| `addPoint` | `px`, `py`, `name`, `kind`, `note` | Creates a point on the active map; the backend converts pixels to game coordinates. |
| `updatePoint` | `id`, `name`, `kind`, `note` | Edits a point. |
| `deletePoint` | `id` | Deletes a point. |
| `getLogs` | | Replies with `logs`. |
| `openDataDir` | | Opens the data folder in the system file manager. |
| `setOpenInWindow` | `value` | Chooses the app window or the browser for the next launch. |
| `quit` | | Closes Rastro. |
| `setGameLogs` | `enabled`, `path?` | Turns log-based detection on or off. `path` sets the logs folder (empty means the default one); without it, the current folder is kept. |
| `setGameMode` | `mode`: `auto`, `regular` or `pve` | Which game mode's tarkov.dev data to use. |
| `marketRefresh` | `fleaName` | Downloads the price catalog for the current mode and language. `fleaName` is the flea market's name in the interface language (no control characters, at most 40). |
| `setCheckUpdates` | `enabled` | Answers the one-time question, or changes the setting in *Settings → About*. Turning it on checks right away. |
| `checkUpdates` | | Checks for a new release now. The result arrives in `snapshot.view.update`. |
| `setLanguage` | `lang`: `cs`, `de`, `en`, `es`, `fr`, `hu`, `it`, `ja`, `ko`, `pl`, `pt`, `ro`, `ru`, `sk`, `tr` or `zh` | Language for tarkov.dev data. The interface sends it on connect and whenever the user changes it. |
| `updateLanguageData` | `fleaName` | Brings the maps, prices and quests listed in `snapshot.view.languageData.outdated` over to the current language. Maps are rebuilt from their SVG on disk with freshly downloaded data; prices and quests are downloaded again. `fleaName` as in `marketRefresh`. |
| `setAutoLanguageData` | `enabled` | Whether the interface may do that on its own after a language change (*Settings → General*). |
| `questsRefresh` | | Downloads the quests for the current mode and language from `json.tarkov.dev`, or from `api.tarkov.dev` if that fails. Progress is reported in `snapshot.view.quests`. |
| `questSet` | `questId`, `status`: `active`, `completed`, `failed` or `null` | Sets a quest's status in the current mode; `null` clears it. |
| `questObjective` | `objectiveId`, `count` | How much of an objective is done (`0` clears it). For simple objectives the count is `0` or `1`. |
| `questsLevel` | `level` (1–79) | Your PMC level in the current mode. |
| `questsFaction` | `faction`: `any`, `usec` or `bear` | Your faction, which hides the other faction's quests. |
| `questsCompleteBefore` | `questId` | Marks every quest that has to be completed before this one as completed. |
| `questsReset` | | Clears the quest progress of the current mode. |
| `questsReadLogs` | | Reads the quest history in the game logs again (raid detection must be on, or the reply is the user message `logsOff`). |
| `questsCompleteThrough` | `questIds` (max 32) | Catching up: each quest, and everything that has to be completed before it, is marked as completed. |
| `questFocus` | `questId` or `null` | Previews a quest on the map (its objectives are sent with `focus: true`), or stops the preview. |

## Backend → interface (`ServerMsg`)

### `snapshot { view }`

Everything the interface draws, already projected to pixels on the active map:

- **Settings** (`settings`, including `openInWindow`, `language` and `checkUpdates`: `true`, `false` or `null` when not asked yet), watcher status, installed packs and the data folder (`dataDir`).
- **Active map** (`activeMap`): `imageUrl`, the level for the current height, `floors: [{ id, name }]` (lowest first), `autoFloor`, the floor picked from your last position, and `baseFloor`, the one shown when nothing else applies.
- **Floors.** On maps with floors, the player, extracts, points, peers' markers and `mapData` markers carry `floor`, the id of the floor they are on. The interface dims whatever isn't on the floor being shown.
- **Player**: `px`, `py`, `angle` (screen degrees) and game coordinates.
- **Trail**, **extracts** (with `distanceM`, `available` and `dimmed`) and **points** on the active map (with `distanceM`).
- **Guidance**: `distanceM`, `heightDiffM` and `relativeDeg` (negative means turn left).
- **Party**: status, the invite code when hosting, peers (with a marker only when they are on your map) and pings (with `remainingMs`).
- `onboarding`: whether to show the first-run guide.
- **Extracts** may carry `switches`: the names of switches to activate first. The interface shows them as a translated note.
- `tarkovdev`: `{ state: idle | running | done | error, step: maps | data | install | icons | null, current, done, total, installed, failed, iconsMissing, noData, error }`. The interface composes the progress and summary text from these fields.
- `update`: `{ state: idle | checking | upToDate | available | error, latest, url, error }`. `url` is always a release page of this repository.
- `game`: `{ enabled, folder, folderFound, mode, modePref, phase: menu | loading | raid | null, mapName, mapInstalled, startedAt, raidMinutes }`. `startedAt` is the local time from the log (`YYYY-MM-DDTHH:MM:SS.mmm`, no time zone); the interface computes the raid clock from it.
- `market`: `{ state: empty | loading | ready | error, fetchedAt, count, mode, lang, error }`. Every `error` field in the view (watcher, party, market, quests, tarkovdev) is a user message, described below. The interface fetches `/api/market` again whenever `fetchedAt` changes.
- `quests`: `{ state: empty | loading | ready | error, fetchedAt, count, mode, lang, error, logs }`, like `market`. `logs` is `{ enabled, scanning, known, sessions }`: whether quests follow the game logs, whether the history is being read, how many quests the logs know about in this mode and how many launches were read. The interface fetches `/api/quests` again whenever `fetchedAt` changes.
- `questMarkers`: open objectives of your active quests on the active map, plus the previewed quest's: `[{ id, quest, objective, px, py, distanceM, floor?, focus?, outline?, kind }]`. `outline` is the zone's area in map pixels, when the objective has one. `kind` says how precise the spot is: `exact` (a place), `area` (anywhere inside the outline) or `candidate` (one of the places an item can spawn). Spots that project outside the map image are left out. `id` (`qo-<objective>-<n>`) is also a guide target for `selectExtract`, and guidance falls back to it like an extract.
- `languageData`: `{ outdated: [{ kind: maps | prices | quests, lang }], auto }`. What was downloaded in another language than the current one (`lang` is `null` for maps installed by an earlier build), and whether to update it on its own (`auto`: `true`, `false` or `null` when not asked yet).

### Other messages

| Message | Content |
|---|---|
| `mapData { mapId, markers, icons }` | Layers for the active map and the list of downloaded icons. Each marker is `{ kind: loot \| transit \| switch \| lock, group, icon, name, px, py }`, and locks add `key` and `power`. On maps with floors, each marker also has `floor`. Sent on connect, when the map changes and when the available icons change. |
| `folderCheck { result: { path, exists, captures } }` | Reply to `checkFolder`. |
| `logs { lines }` | Reply to `getLogs`. |
| `interfaces { list: [{ name, ip }] }` | Reply to `listInterfaces`. |
| `notice { level: "info" \| "error", code, arg?, text }` | A user message to show as a toast. |
| `questProgress { progress }` | Your quest progress in the current mode: `{ mode, level, faction, quests: { id: status }, objectives: { id: count }, revision }`. Sent on connect and whenever it or the game mode changes. |

## User messages

Anything the backend wants the user to read is a *user message*: `{ code, arg?, text }`.

- `code` identifies the message (`[a-zA-Z]{1,40}`, for example `portInUse` or `wrongCode`). The interface translates it with `t.msg[code]` in the user's language.
- `arg` is an optional detail inserted into the translation: a port, a path, a version or a technical error in English.
- `text` is the same message in English. The interface shows it when it doesn't know the code, and it's what goes to the log.

The list of codes lives in `web/src/locales/es/tools.ts` (`msg`); the backend side is spread across the modules that raise them.

## HTTP endpoints

Every request needs the `x-rastro-token` header. `POST` requests also need a matching `Origin` and `Content-Type: application/json`, except the upload, which is `application/zip`.

| Method and path | Body | Response |
|---|---|---|
| `GET /api/packs` | | `{ packs }` |
| `GET /api/packs/{id}/image` | Optional `?floor=<id>` | The pack's image (allowlisted by manifest). For layered SVGs, that floor at full strength, the base floor dimmed and the other floors faint. |
| `GET /api/icons/{name}` | | A marker icon (PNG) from the fixed list, if downloaded. |
| `GET /api/market` | | The processed catalog `{ mode, lang, fetchedAt, items }` for the current mode and language, or 404 if it hasn't been downloaded. |
| `GET /api/quests` | | The processed quests `{ mode, lang, fetchedAt, traders, quests }` for the current mode and language, or 404 if they haven't been downloaded. Traders carry `image` (the id of their portrait); quests carry `rewards` and `image` (whether tarkov.dev has its picture), and each objective spot its `kind` (`exact`, `area`, `candidate`). |
| `GET /api/tarkov-image/{id}` | | A trader's portrait or a quest's picture (24-hex id), from the cache or from `assets.tarkov.dev`. `204 No Content` when there isn't one. |
| `GET /api/key-locations/{id}` | | `[{ mapId, mapName, count }]`: installed maps with locks for that key (24-hex item ID). |
| `GET /api/item-icon/{id}` | | An item icon (24-hex item ID), from the cache or from `assets.tarkov.dev`. `204 No Content` when there isn't one (offline or unknown item). |
| `GET /api/changelog` | | `CHANGELOG.md` as embedded in this build, as `text/plain`. |
| `POST /api/import/upload` | Raw zip, optional `x-expected-sha256` header | `Preview` |
| `POST /api/import/demo` | `{}` | `Preview` |
| `POST /api/import/inspect-url` | `{ url }` | `{ host }`, without touching the network |
| `POST /api/import/url` | `{ url, sha256? }` | `Preview` |
| `POST /api/import/commit` | `{ stagingId, overwrite, acceptLicense: true }` | `{ id }` |
| `POST /api/import/cancel` | `{ stagingId }` | `{}` |

Errors come back as `{ "error": "<English text>", "code": "<code>", "arg": <detail or null> }` with status 400, 403, 404, 409 (already exists) or 500. `code` and `arg` work like a user message.

## Party protocol (TCP)

1. **Version check.** The client sends `RSTP` followed by its protocol version as a big-endian `u16`. The host answers `[0, version]` to continue, or `[1, its_version]` and closes, so a version mismatch can be explained to the user before any data is exchanged.
2. **Handshake.** `Noise_NNpsk0_25519_ChaChaPoly_BLAKE2s`, with the secret from the invite code as the pre-shared key and `RSTP` plus the version as the prologue.
3. **Frames.** Each frame is a `u16` length followed by a Noise message, at most 16 KiB. Inside is JSON tagged by `t`:
   - From client to host: `hello`, `pos`, `ping`, `beat`, `bye`.
   - From host to clients: `welcome`, `peer`, `peerPos`, `peerPing`, `peerLeft`, `closed`.

Peer IDs are assigned by the host and tied to each connection, so a client can't claim to be someone else.
