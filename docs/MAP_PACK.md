# Map pack format (schema version 2)

Most players never need this: the maps from tarkov.dev are built and installed by Rastro with one click. This document is for anyone who wants to make their own map pack, for example for a location tarkov.dev doesn't have an SVG for, such as Labs.

## The package

A map pack is a `.zip` with exactly two files at its root and no folders:

```
manifest.json
<image>.svg | .webp | .png | .jpg
```

Limits: 50 MB compressed, 100 MB uncompressed, a compression ratio of at most 100:1, and an image between 1 and 8192 pixels on each side.

## manifest.json

```json
{
  "schema_version": 2,
  "id": "my-map",
  "name": "My map",
  "image": { "file": "my-map.webp", "width": 4096, "height": 4096 },
  "calibration": {
    "points": [
      { "game": [-500, 500], "pixel": [0, 0] },
      { "game": [500, -500], "pixel": [4096, 4096] }
    ],
    "invert_x": false,
    "invert_z": false,
    "rotation": 0
  },
  "levels": [{ "id": "ground-floor", "name": "Ground floor", "y_min": -10, "y_max": 5 }],
  "extracts": [
    { "id": "north-gate", "name": "North gate", "faction": "pmc", "x": -80, "y": 0, "z": 470, "conditions": "Always open." }
  ],
  "credits": { "author": "Your name", "license": "CC-BY-4.0", "source": "https://…" }
}
```

### Fields

| Field | Rules |
|---|---|
| `schema_version` | `2`. Version `1` packs, which have no `markers`, are still accepted. |
| `id` | Matches `^[a-z0-9-]{1,40}$`. Also used as the name of the installed folder. |
| `name` | 1 to 60 characters, no control characters. |
| `image.file` | A plain lowercase file name: `[a-z0-9_-]{1,40}` followed by `.svg`, `.webp`, `.png`, `.jpg` or `.jpeg`. |
| `image.width`, `image.height` | 1 to 8192. For raster images they must match the image header. |
| `image.base_layer` | Optional. For layered SVGs, the id of the `<g>` group that is always visible (the base floor). |
| `calibration.points` | Two points, each mapping game coordinates `[x, z]` to a pixel `[x, y]`, with the origin at the top-left of the image. The two points must differ on both axes. |
| `calibration.invert_x`, `invert_z` | Optional. Flip an axis before rotating. |
| `calibration.rotation` | `0`, `90`, `180` or `270`, clockwise. |
| `levels` | Optional, up to 16. A level applies when `y_min ≤ height < y_max`. In layered SVGs, `svg_layer` names the `<g>` group for that floor, and `zones` (up to 64 boxes of the form `{ "x": [a, b], "z": [a, b], "y": [min, max] }`) drive automatic floor selection. |
| `extracts` | Up to 500. `faction` is `pmc`, `scav` or `shared`; `conditions` is up to 300 characters. `switches` is optional: up to 8 names (60 characters each) of switches to activate first, shown as a translated "Requires: …" note. |
| `markers` | Optional, version 2 only, up to 3000. See below. |
| `game_ids` | Optional, up to 8. The game's internal names for this map (`[A-Za-z0-9_]{1,40}`, for example `bigmap` for Customs), used to switch maps automatically from the game logs. |
| `raid_minutes` | Optional, 1 to 600. Raid length, used to show the time left. |
| `credits` | `author` and `license` are required (up to 100 characters each); `source` is optional (up to 300). |

Unknown fields are rejected. Every piece of text is displayed as plain text, never as HTML.

### Markers

```json
{ "kind": "loot", "group": "weapon-box", "name": "Weapon box", "x": 0, "y": 0, "z": 0 }
```

- `kind` is `loot`, `transit`, `switch` or `lock`.
- `group` is optional. For `loot` it's the container type, used for the icon and the container filter. For `lock` it's `door`, `container`, `trunk` or `lock`.
- `name` is up to 80 characters. It may only be empty for locks, whose name comes from the key.
- Locks also take `key`, the 24-character hexadecimal tarkov.dev ID of the key item, and `power`, set to `true` when the lock needs power (omit it otherwise).

## Calibrating

1. In game, take two screenshots at spots that are easy to find on your image, far apart and not lined up on either axis. Each screenshot's file name contains `x, y, z`; use `x` and `z`.
2. Note the pixel position of each spot on the image.
3. Put both pairs in `calibration.points`.
4. If the map comes out rotated or mirrored, adjust `rotation`, `invert_x` or `invert_z`.

## SVG rules

SVGs are rejected if they contain `<script>`, `<foreignObject>`, `<iframe>`, `<embed>` or `<object>`, a DOCTYPE or entities, `on*` event attributes, `javascript:` URLs, `@import`, or any `href`, `src` or `url()` that isn't a local `#id` or an embedded raster image (`data:`). On top of that, an SVG is only ever displayed as an image, never inserted into the page.

## Example

The demo pack in `assets/demo/` is a complete, working example. You can install it from the app with *Install demo*.
