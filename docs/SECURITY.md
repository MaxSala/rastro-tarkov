# Security

Rastro runs on the player's own PC, opens a web server bound to `127.0.0.1`, and reads files the game writes to disk. When the user asks it to, it also downloads data from a short list of hosts and, for the party feature, accepts or opens direct TCP connections to other players. This document describes what it protects, how, and what it deliberately does not protect against.

## Reporting a vulnerability

Please **do not** report security issues in public GitHub issues or on Discord.

Use GitHub's private reporting instead: open the repository's **Security** tab and choose **Report a vulnerability**. Include the version, your operating system, the steps to reproduce and, if you have one, a proof of concept.

You'll get an acknowledgement as soon as possible. Fixes are released for the latest version only; there are no long-term support branches.

## What is protected

- **Control of the app.** Nothing other than Rastro's own interface should be able to drive it.
- **The user's files.** Rastro only reads what it needs and never writes outside its own data folder.
- **The player's position.** It should only reach the people the player chose to share it with.
- **The party secret.** The invite code is the only thing that lets someone join a room.

## Threat model

| Threat | Main defences |
|---|---|
| A malicious website open in the user's browser attacks the local server (CSRF, cross-origin WebSocket, DNS rebinding) | Bound to `127.0.0.1` only. Exact `Host` and `Origin` checks. A random 256-bit session key is required on every API and WebSocket request. JSON `Content-Type` enforced. No CORS. |
| A malicious map pack (zip slip, zip bomb, decompression bomb image, SVG with scripts, HTML injection through text fields) | Defensive extraction with real byte counters. Image dimensions read from the header before decoding. SVG filtered and only ever displayed as an `<img>` from a `blob:` URL. All text rendered as text. |
| A stranger, or a misbehaving teammate, connecting to the party port | Noise `NNpsk0` handshake keyed with a 256-bit secret. Frame size checked before allocating. At most 4 pending handshakes, 5 s to complete one, 8 peers per room, per-peer rate limits. Every field validated. A peer's identity is its connection. |
| Importing a map pack from a URL (SSRF, redirects to internal addresses, huge downloads) | HTTPS only, no credentials in the URL. Custom DNS resolution that rejects private, loopback and link-local addresses, with the connection pinned to the address that was checked. At most 3 redirects, each revalidated. Byte limits enforced while streaming. No proxy, no cookies. |
| Data from tarkov.dev being tampered with or compromised | Only fetched on request. Fixed host allowlist, also applied to redirects. Same SSRF defences and byte limits. SVGs filtered. Generated map packs go through the same validation as imported ones. Text sanitised. |
| Game logs containing personal data (profile and account IDs, server IPs) | Off by default and read-only. Only two kinds of log file are opened, and only the game mode, the internal map name (validated) and a timestamp are extracted. Nothing else is kept, logged or shared. |
| Supply chain | Exact dependency versions. `cargo deny` and `cargo audit`. npm runs with `ignore-scripts=true`. No CDNs or remote resources at runtime. |
| Leaks through logs or disk | The session key and party code are never logged. Party positions live in memory only. Configuration files are size-limited and validated when read. |

## Controls

### Local server

- `src/server/mod.rs` checks every request (`Host`, `Origin`, `Sec-Fetch-Site`, session key compared in constant time, `Content-Type`, method). Every response carries CSP, `nosniff`, `no-referrer`, CORP, COOP and `X-Frame-Options` headers. In-flight requests and body sizes are capped (64 KiB, or 50 MB for map uploads only).
- `src/server/ws.rs` requires authentication in the first message within 5 seconds, caps messages at 64 KiB and allows at most 4 sockets.
- `src/main.rs` listens on `127.0.0.1` only and fails with a clear message if the port is taken.

### Session key

Generated with `getrandom` in `src/main.rs`. It travels in the URL fragment (`#t=…`), never in the query string. The interface moves it to `sessionStorage` and removes it from the address bar on load (`web/src/lib/token.ts`).

### Desktop window

`src/desktop.rs`: the WebView may only navigate within `http://127.0.0.1:<port>`. Any other navigation is blocked; HTTPS links are handed to the system browser instead. New windows are denied, developer tools are disabled in release builds, and the error page escapes its text.

### Web interface

- All map markers and tooltips are built from DOM nodes with `textContent` (`web/src/components/map/markers.ts`, `locks.ts`).
- ESLint forbids `innerHTML`, `insertAdjacentHTML`, `dangerouslySetInnerHTML` and `eval` (`web/eslint.config.js`).
- Every message from the backend is validated with zod (`web/src/lib/protocol.ts`).
- Vite is configured so that no asset is ever inlined, which keeps the CSP strict (`web/vite.config.ts`).
- Interface languages are part of the bundle; nothing is downloaded or evaluated at runtime. The saved language is only accepted if it is on the fixed list, and the backend only accepts the 16 codes of its `Language` enum.

### Map packs

- `src/maps/zipx.rs`: rejects absolute paths, `..`, links, duplicates and control characters. At most 10 entries, 50 MB compressed and 100 MB uncompressed, and a 100:1 compression ratio measured while extracting.
- `src/maps/image.rs`: magic-byte checks, 8192 px maximum, SVG filter.
- `src/maps/manifest.rs`: `deny_unknown_fields`, ID patterns, length limits, finite numbers only, at most 500 extracts and 3000 markers, schema version check. Lock markers must reference a valid 24-hex item ID, including in imported packs.
- `src/maps/pack.rs`: installs by atomic rename and never overwrites without confirmation.
- `src/maps/import.rs`: optional SHA-256 verification. One import at a time.
- `src/server/api.rs`: images are served only by manifest ID and file name, with a `sandbox` CSP. Floor variants hide the other floors with a generated `<style>` whose IDs are validated (letters, digits, `_` and `-`), so no CSS or markup can be injected.

### Downloads

- `src/maps/download.rs` implements the hardened fetch (allowlists, DNS checks, redirect revalidation, streaming limits).
- **tarkov.dev maps** (`src/tarkovdev/sync.rs`): fixed hosts `raw.githubusercontent.com`, `json.tarkov.dev` and `api.tarkov.dev`, with 4/20/32 MB limits. One failing map doesn't stop the others. The user sees the licence and the hosts and must agree before anything is downloaded.
- **Map data** (`src/tarkovdev/calib.rs`, `api.rs`, `jsondoc.rs`, `locks.rs`): every field is optional, text is stripped of control and bidirectional characters and truncated, IDs must be unique and valid, coordinates must be finite and bounded. Lock types are normalised to a closed list.
- **Market** (`src/market/fetch.rs`, `build.rs`): fixed hosts `json.tarkov.dev` and `assets.tarkov.dev`, one download at a time, 160 MB limit for items and 64 MB per translation file, JSON parsed off the async runtime. Item IDs and slugs are validated, prices must be finite and bounded, offers from unknown traders are dropped, and the catalog is capped at 20,000 items. The cache is written atomically and read back with a 64 MB limit. The interface receives the processed catalog, never the raw download, and searches never leave the PC.
- **Update check** (`src/update.rs`): off until the user answers a one-time question; *Check now* is an explicit action. A single GET to `api.github.com` (fixed host, 512 KB limit) through the same hardened fetch. Nothing but the request itself is sent. The reply is only trusted for two fields: the tag must be a plain `vX.Y.Z` version, and the release URL must start with this repository's `/releases/` address, or it is discarded. The interface checks both again before showing a link, and nothing is downloaded or installed automatically.
- **Icons** (`src/tarkovdev/icons.rs`, `src/market/icons.rs`): marker icons come from a closed list of names; item icons are addressed by a 24-hex ID only. After a connection or DNS failure, item icons stop trying the network for 60 seconds. Both are checked by magic bytes, size (64 or 96 KB) and dimensions (256 px), cached on disk and used by the interface as `blob:` URLs.

### Game logs

`src/gamelog/`: `parse.rs` is a pure function built on regular expressions, and map names must match `[A-Za-z0-9_]{1,40}`. `watcher.rs` polls the most recent session once a second, reads at most the last 4 MB on start and then up to 1 MB per pass, limits an incomplete line to 64 KB and starts over if a file is truncated. `control.rs` can only switch the active map and start a new raid. Tests use anonymised lines from real logs.

### Party

- `src/party/secure.rs`: protocol version exchanged before the Noise handshake.
- `src/party/frame.rs`: 16 KiB frame limit.
- `src/party/session.rs`: 5 s heartbeat, connection dropped after 21 s of silence, 10 messages per second with bursts of 30.
- `src/party/host.rs`: pending-connection limit, 8-player rooms, kicking, peer IDs assigned by the host.
- `src/party/proto.rs`: field-by-field validation.
- `src/party/code.rs`: the invite code is wiped from memory with `zeroize` when no longer needed.

### Local data

- `src/points.rs`: the points file is limited to 4 MB, unknown fields are rejected, IDs are validated, and there are at most 500 points per map across 64 maps. Invalid entries are dropped on load.
- `src/logs.rs`: 2,000 lines in memory, plus `rastro.log`, rewritten on each start and capped at 5 MB. Neither ever contains the session key or the party code.
- `src/setup.rs`: checking a screenshots folder only lists file names (at most 20,000) and never opens a file.

### Code

`#![forbid(unsafe_code)]`. Clippy's `unwrap_used`, `expect_used`, `panic` and `indexing_slicing` lints are denied (`Cargo.toml`). An error in one connection ends that connection, never the server.

## Tests

Security-relevant tests live next to the code they cover:

| File | Covers |
|---|---|
| `src/maps/zipx_tests.rs` | Zip slip, zip bombs, duplicate and disallowed entries |
| `src/maps/image_tests.rs` | Oversized images, magic bytes, SVG with scripts or external references |
| `src/maps/import_tests.rs` | Manifests with unknown fields and oversized strings |
| `src/maps/download_tests.rs` | SSRF to loopback, `169.254.169.254` and `localhost`, directly and through redirects |
| `src/server/guard_tests.rs` | `Host`, `Origin`, session key, `Content-Type`, path traversal, cross-origin WebSocket |
| `src/party/frame_tests.rs` | Oversized frames |
| `src/party/secure_tests.rs` | Replayed and tampered frames, wrong key, version mismatch |
| `src/party/session_tests.rs` | Heartbeat timeout, flooding |
| `src/party/proto_tests.rs` | Control characters in names, unknown types and fields |
| `src/party/host_tests.rs` | Pending limit, full room, kicking, port released on close |
| `src/party/code_tests.rs` | Error messages never include the code or the secret |
| `src/tarkovdev/locks.rs`, `jsondoc_tests.rs` | Invalid key IDs, unknown lock types, unnamed locks |
| `src/market/build_tests.rs` | Invalid IDs, untranslated items, control and bidi characters in names |
| `src/update.rs` | Malformed versions, release URLs outside this repository |

The interface has also been checked in Chromium for CSP violations (none) and for the session key being removed from the URL after loading.

## Known risks

These are limits of the design, stated plainly.

- **A compromised PC.** Any process running as the same user can read the session key from memory and the party code from the app. Rastro does not try to defend against malware already on the machine.
- **The session key can be seen.** With `--no-open`, or if opening the browser fails, the URL with the key is printed to the console. In browser mode the fragment stays in the browser history until the page removes it. It stops working when Rastro closes.
- **Static files are served without the key.** The compiled interface contains no data, but it is served to any request with the correct `Host`.
- **The invite code is a bearer secret.** Whoever has it can join. A legitimate teammate can lie about their own position or ping anywhere, and can see yours.
- **An open room is visible on the network.** While a room is open, its port accepts connections. Without the secret they never get past the handshake, but a port scan will show something is listening. A mesh VPN avoids exposing it at all.
- **A forged invite code** can make your app connect to an arbitrary host and port. Only a 6-byte preamble is sent before the handshake fails.
- **The SVG filter is pattern-based,** not a full XML parser. The real protection is that SVGs are only displayed as images, where browsers don't run scripts, under a `sandbox` CSP.
- **DNS.** Internal addresses are rejected at resolution time, but if a legitimate domain later serves malicious content, only the optional SHA-256 check will catch it.
- **Dependencies.** Rastro trusts `snow`, `rustls`, `ring` and the rest of its dependencies. `cargo deny`, `cargo audit` and `npm audit` reduce that risk; they don't remove it.
- **Update check.** If GitHub or this repository were compromised, the check could point to a malicious release. Rastro only ever opens the release page in the browser; it never downloads or runs anything by itself. Release builds publish `SHA256SUMS` alongside the `.exe`.
- **tarkov.dev.** If its repository or API were compromised, it could serve misleading maps, names or prices. It could not serve code: SVGs are filtered and shown as images, and text is shown as text. Rastro relies on the TLS certificates of GitHub and tarkov.dev; the data itself is not signed.
- **The system WebView.** The window uses WebView2 on Windows and WebKitGTK on Linux. Their security depends on the system being up to date. The window only ever shows Rastro's own local interface.
- **The local log.** `rastro.log` can contain folder paths and map names.
- **The market cache** can be edited by any process running as the same user. At worst it shows wrong prices or names, always as plain text.
- **Game log format.** Battlestate can change it at any time. If it changes, detection stops working; nothing else breaks.
- **`--dev` mode** also accepts `localhost:5173` as an origin. It's for development only; don't use it to play.
- **Map licence.** The tarkov.dev maps forbid use in software that gives an unfair advantage. Whether the party feature fits that licence is ultimately for the map authors to decide.
