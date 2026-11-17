# Protocolo web ↔ backend

Un único WebSocket en `/ws` con mensajes JSON etiquetados por `type`. Fuente de verdad: `src/protocol.rs` y `src/view.rs`; espejo validado con zod en `web/src/lib/protocol.ts`.

## Conexión

1. La web abre `ws://127.0.0.1:<puerto>/ws` (el servidor exige `Host` y `Origin` exactos).
2. Primer mensaje, en menos de 5 s: `{"type":"auth","token":"<64 hex>"}`. Si falla, el servidor cierra.
3. El servidor responde con un `snapshot` y envía otro cada vez que algo cambia (agrupa ráfagas).

Límites: 64 KiB por mensaje entrante, 4 sockets a la vez.

## Web → backend (`ClientMsg`)

| type | campos | efecto |
|------|--------|--------|
| `auth` | `token` | solo el primero |
| `setActiveMap` | `mapId` | cambia el mapa activo |
| `newRaid` | — | borra rastro, última posición, selección y disponibilidad |
| `selectExtract` | `extractId \| null` | activa o quita la guía |
| `toggleExtract` | `extractId` | marca/desmarca «disponible esta raid» en el mapa activo |
| `updateSettings` | `settings {screenshotsDir, trailLength, playerName, playerColor, partyPort}` | valida y guarda |
| `ackShareWarning` | — | no volver a mostrar el aviso del código |
| `partyHost` | `bindIp`, `advertise` | abre la sala |
| `partyJoin` | `code` | se une a una sala |
| `partyLeave` | — | sale o cierra la sala |
| `partyKick` | `peerId` | expulsa (solo anfitrión) |
| `mapPing` | `px`, `py` | ping en píxeles; el backend lo pasa a coordenadas del juego |
| `listInterfaces` | — | responde `interfaces` |
| `removePack` | `packId` | borra un paquete |
| `tarkovDevSync` | — | descarga e instala los mapas de tarkov.dev (progreso en `snapshot.view.tarkovdev`) |
| `finishOnboarding` | — | marca el asistente como hecho |
| `checkFolder` | `path` | responde `folderCheck` (existe y cuántas capturas con coordenadas) |
| `setScreenshotsDir` | `path` | fija la carpeta de capturas y reinicia el vigilante |
| `addPoint` | `px`, `py`, `name`, `kind`, `note` | crea un punto propio en el mapa activo (el backend lo pasa a coordenadas del juego) |
| `updatePoint` | `id`, `name`, `kind`, `note` | edita un punto |
| `deletePoint` | `id` | borra un punto |
| `getLogs` | — | responde `logs` |
| `openDataDir` | — | abre la carpeta de datos en el explorador |
| `setOpenInWindow` | `value` | ventana propia o navegador en el próximo arranque |
| `quit` | — | cierra Rastro |
| `setGameLogs` | `enabled`, `path` | activa o desactiva la detección por logs y fija su carpeta (vacía = la por defecto) |
| `setGameMode` | `mode`: `auto` \| `regular` \| `pve` | modo de los datos de tarkov.dev |
| `marketRefresh` | `fleaName` | descarga el catálogo de precios del modo e idioma actuales; `fleaName` es el nombre del mercadillo en el idioma de la interfaz (sin control, ≤ 40 caracteres) |
| `setLanguage` | `lang`: `es` \| `en` | idioma de los datos de tarkov.dev; la web lo envía al conectar y al cambiarlo |

`selectExtract` acepta también ids de puntos propios (`pt-…`) para guiarte hasta ellos.

## Backend → web (`ServerMsg`)

- `snapshot { view }` — todo lo que la web dibuja, ya proyectado a píxeles del mapa activo: ajustes, estado del vigilante, paquetes, mapa activo (con `imageUrl` y nivel por altura), jugador (`px`, `py`, `angle` en grados de pantalla, coordenadas del juego), rastro, extracciones (con `distanceM`, `available`, `dimmed`), guía (`distanceM`, `heightDiffM`, `relativeDeg`: negativo = izquierda) y party (estado, código si eres anfitrión, compañeros con marcador solo si están en tu mapa, pings con `remainingMs`).
- `snapshot` incluye además `onboarding` (mostrar el asistente) y `tarkovdev {state: idle|running|done|error, message, done, total}`.
- `mapData { mapId, markers: [{kind: loot|transit|switch, group, icon, name, px, py}], icons }` — capas del mapa activo e iconos descargados; se envía al conectar, al cambiar de mapa o cuando cambian los iconos disponibles.
- `snapshot.view.activeMap` incluye `floors: [{id, name}]` y `autoFloor` (piso según tu última posición).
- `folderCheck { result: {path, exists, captures} }`
- `logs { lines }`
- `snapshot.view.game`: `{enabled, folder, folderFound, mode, modePref, phase: menu|loading|raid|null, mapName, mapInstalled, startedAt, raidMinutes}`. `startedAt` es la hora local del log (`AAAA-MM-DDTHH:MM:SS.mmm`, sin zona); la web calcula el reloj a partir de ella.
- `snapshot.view` incluye además `points` (puntos propios del mapa activo, con `distanceM`), `dataDir` y `settings.openInWindow`.
- `snapshot.view.market`: `{state: empty|loading|ready|error, fetchedAt, count, mode, lang, error}`. La web vuelve a pedir `/api/market` cuando cambia `fetchedAt`. `snapshot.view.settings.language` es el idioma guardado.
- `interfaces { list: [{name, ip}] }`
- `notice { level: "info" | "error", text }` — texto en español para mostrar tal cual.

## HTTP

Todas exigen la cabecera `x-rastro-token`; los POST, además, `Origin` y `Content-Type: application/json` (salvo la subida).

| Método y ruta | Cuerpo | Respuesta |
|---------------|--------|-----------|
| `GET /api/packs` | — | `{packs}` |
| `GET /api/packs/{id}/image` | `?floor=<id>` opcional | la imagen del manifest (lista blanca); en SVG por pisos, con ese piso visible |
| `GET /api/icons/{nombre}` | — | icono PNG de la lista cerrada, si está descargado |
| `GET /api/market` | — | catálogo procesado `{mode, lang, fetchedAt, items}` del modo e idioma actuales; 404 si no se ha descargado |
| `GET /api/item-icon/{id}` | — | icono del objeto (id de 24 hex), de la caché o de `assets.tarkov.dev` |
| `POST /api/import/upload` | zip crudo, `Content-Type: application/zip`, `x-expected-sha256` opcional | `Preview` |
| `POST /api/import/demo` | `{}` | `Preview` |
| `POST /api/import/inspect-url` | `{url}` | `{host}` (sin red) |
| `POST /api/import/url` | `{url, sha256?}` | `Preview` |
| `POST /api/import/commit` | `{stagingId, overwrite, acceptLicense: true}` | `{id}` |
| `POST /api/import/cancel` | `{stagingId}` | `{}` |

Los errores devuelven `{"error": "<mensaje en español>"}` con 400, 403, 404, 409 (ya existe) o 500.

## Protocolo del party (TCP)

1. Cliente → anfitrión: `RSTP` + versión `u16` big-endian. El anfitrión responde `[0, ver]` o `[1, su_ver]` y cierra: así un desajuste de versión se explica antes de cualquier dato.
2. Handshake `Noise_NNpsk0_25519_ChaChaPoly_BLAKE2s` con el secreto del código como PSK y `RSTP+versión` como prólogo.
3. Tramas: longitud `u16` + mensaje Noise, máximo 16 KiB. Dentro, JSON con etiqueta `t`: `hello`, `pos`, `ping`, `beat`, `bye` (cliente) y `welcome`, `peer`, `peerPos`, `peerPing`, `peerLeft`, `closed` (anfitrión). El anfitrión asigna los ids según la conexión.
