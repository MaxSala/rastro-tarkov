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
| `selectExtract` | `extractId` or `null` | Starts or stops guidance. Also accepts point IDs (`pt-…`). |
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
| `setGameLogs` | `enabled`, `path` | Turns log-based detection on or off and sets the logs folder (empty means the default one). |
| `setGameMode` | `mode`: `auto`, `regular` or `pve` | Which game mode's tarkov.dev data to use. |
| `marketRefresh` | `fleaName` | Downloads the price catalog for the current mode and language. `fleaName` is the flea market's name in the interface language (no control characters, at most 40). |
| `setLanguage` | `lang`: `cs`, `de`, `en`, `es`, `fr`, `hu`, `it`, `ja`, `ko`, `pl`, `pt`, `ro`, `ru`, `sk`, `tr` or `zh` | Language for tarkov.dev data. The interface sends it on connect and whenever the user changes it. |

## Backend → interface (`ServerMsg`)

### `snapshot { view }`

Everything the interface draws, already projected to pixels on the active map:

- **Settings** (`settings`, including `openInWindow` and `language`), watcher status, installed packs and the data folder (`dataDir`).
- **Active map** (`activeMap`): `imageUrl`, the level for the current height, `floors: [{ id, name }]` and `autoFloor`, the floor picked from your last position.
- **Player**: `px`, `py`, `angle` (screen degrees) and game coordinates.
- **Trail**, **extracts** (with `distanceM`, `available` and `dimmed`) and **points** on the active map (with `distanceM`).
- **Guidance**: `distanceM`, `heightDiffM` and `relativeDeg` (negative means turn left).
- **Party**: status, the invite code when hosting, peers (with a marker only when they are on your map) and pings (with `remainingMs`).
- `onboarding`: whether to show the first-run guide.
- `tarkovdev`: `{ state: idle | running | done | error, message, done, total }`.
- `game`: `{ enabled, folder, folderFound, mode, modePref, phase: menu | loading | raid | null, mapName, mapInstalled, startedAt, raidMinutes }`. `startedAt` is the local time from the log (`YYYY-MM-DDTHH:MM:SS.mmm`, no time zone); the interface computes the raid clock from it.
- `market`: `{ state: empty | loading | ready | error, fetchedAt, count, mode, lang, error }`. The interface fetches `/api/market` again whenever `fetchedAt` changes.

### Other messages

| Message | Content |
|---|---|
| `mapData { mapId, markers, icons }` | Layers for the active map and the list of downloaded icons. Each marker is `{ kind: loot \| transit \| switch \| lock, group, icon, name, px, py }`, and locks add `key` and `power`. Sent on connect, when the map changes and when the available icons change. |
| `folderCheck { result: { path, exists, captures } }` | Reply to `checkFolder`. |
| `logs { lines }` | Reply to `getLogs`. |
| `interfaces { list: [{ name, ip }] }` | Reply to `listInterfaces`. |
| `notice { level: "info" \| "error", text }` | A message to show as is. The text is currently always in Spanish. |

## HTTP endpoints

Every request needs the `x-rastro-token` header. `POST` requests also need a matching `Origin` and `Content-Type: application/json`, except the upload, which is `application/zip`.

| Method and path | Body | Response |
|---|---|---|
| `GET /api/packs` | | `{ packs }` |
| `GET /api/packs/{id}/image` | Optional `?floor=<id>` | The pack's image (allowlisted by manifest). For layered SVGs, with only that floor visible. |
| `GET /api/icons/{name}` | | A marker icon (PNG) from the fixed list, if downloaded. |
| `GET /api/market` | | The processed catalog `{ mode, lang, fetchedAt, items }` for the current mode and language, or 404 if it hasn't been downloaded. |
| `GET /api/key-locations/{id}` | | `[{ mapId, mapName, count }]`: installed maps with locks for that key (24-hex item ID). |
| `GET /api/item-icon/{id}` | | An item icon (24-hex item ID), from the cache or from `assets.tarkov.dev`. |
| `POST /api/import/upload` | Raw zip, optional `x-expected-sha256` header | `Preview` |
| `POST /api/import/demo` | `{}` | `Preview` |
| `POST /api/import/inspect-url` | `{ url }` | `{ host }`, without touching the network |
| `POST /api/import/url` | `{ url, sha256? }` | `Preview` |
| `POST /api/import/commit` | `{ stagingId, overwrite, acceptLicense: true }` | `{ id }` |
| `POST /api/import/cancel` | `{ stagingId }` | `{}` |

Errors come back as `{ "error": "<message>" }` with status 400, 403, 404, 409 (already exists) or 500. Error messages are currently in Spanish.

## Party protocol (TCP)

1. **Version check.** The client sends `RSTP` followed by its protocol version as a big-endian `u16`. The host answers `[0, version]` to continue, or `[1, its_version]` and closes, so a version mismatch can be explained to the user before any data is exchanged.
2. **Handshake.** `Noise_NNpsk0_25519_ChaChaPoly_BLAKE2s`, with the secret from the invite code as the pre-shared key and `RSTP` plus the version as the prologue.
3. **Frames.** Each frame is a `u16` length followed by a Noise message, at most 16 KiB. Inside is JSON tagged by `t`:
   - From client to host: `hello`, `pos`, `ping`, `beat`, `bye`.
   - From host to clients: `welcome`, `peer`, `peerPos`, `peerPing`, `peerLeft`, `closed`.

Peer IDs are assigned by the host and tied to each connection, so a client can't claim to be someone else.
