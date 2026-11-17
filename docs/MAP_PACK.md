# Formato del paquete de mapa (schema_version 2)

Un paquete es un `.zip` con exactamente dos archivos en la raíz, sin carpetas:

```
manifest.json
<imagen>.svg | .webp | .png | .jpg
```

Límites: 50 MB comprimido, 100 MB descomprimido, ratio 100:1, imagen de 1 a 8192 px por lado.

## manifest.json

```json
{
  "schema_version": 2,
  "id": "mi-mapa",
  "name": "Mi mapa",
  "image": { "file": "mi-mapa.webp", "width": 4096, "height": 4096 },
  "calibration": {
    "points": [
      { "game": [-500, 500], "pixel": [0, 0] },
      { "game": [500, -500], "pixel": [4096, 4096] }
    ],
    "invert_x": false,
    "invert_z": false,
    "rotation": 0
  },
  "levels": [{ "id": "planta-baja", "name": "Planta baja", "y_min": -10, "y_max": 5 }],
  "extracts": [
    { "id": "puerta-norte", "name": "Puerta norte", "faction": "pmc", "x": -80, "y": 0, "z": 470, "conditions": "Siempre abierta." }
  ],
  "credits": { "author": "Tu nombre", "license": "CC-BY-4.0", "source": "https://…" }
}
```

| Campo | Regla |
|-------|-------|
| `schema_version` | `2` (se aceptan paquetes `1`, sin `markers`) |
| `id` | `^[a-z0-9-]{1,40}$`; también es el nombre de la carpeta instalada |
| `name` | 1-60 caracteres, sin caracteres de control |
| `image.file` | nombre plano en minúsculas: `[a-z0-9_-]{1,40}` + `.svg/.webp/.png/.jpg/.jpeg` |
| `image.width/height` | 1-8192; en raster deben coincidir con la cabecera de la imagen |
| `calibration.points` | dos puntos: coordenadas del juego `[x, z]` → píxel `[x, y]` (origen arriba a la izquierda). Deben diferir en ambos ejes |
| `invert_x`, `invert_z` | opcionales; invierten el eje antes de rotar |
| `rotation` | 0, 90, 180 o 270 (sentido horario) |
| `levels` | opcional, máximo 16; `y_min ≤ altura < y_max`. En SVG por capas: `svg_layer` (id del grupo `<g>`) y `zones` (hasta 64 `{ "x": [a, b], "z": [a, b], "y": [min, max] }` para la selección automática) |
| `image.base_layer` | opcional: grupo `<g>` del SVG que siempre se ve (planta base) |
| `extracts` | máximo 500; `faction`: `pmc`, `scav` o `shared`; `conditions` hasta 300 caracteres |
| `markers` | opcional (v2), máximo 3000: `{ "kind": "loot" \| "transit" \| "switch", "group": "weapon-box", "name": "…", "x": 0, "y": 0, "z": 0 }`; `group` opcional (tipo de contenedor, para icono y filtro); `name` hasta 80 caracteres |
| `game_ids` | opcional, máximo 8: nombres internos del mapa en el juego (`[A-Za-z0-9_]{1,40}`, p. ej. `bigmap`) para la detección automática por logs |
| `raid_minutes` | opcional, 1-600: duración de la raid, para el tiempo restante |
| `credits` | `author` y `license` obligatorios (≤100); `source` opcional (≤300) |

No se admiten campos desconocidos. Todo texto se muestra como texto plano.

### Calibrar

1. Haz dos capturas en el juego en puntos fáciles de reconocer en la imagen, lejos entre sí y no alineados en ningún eje. El nombre del archivo trae `x, y, z`: usa `x` y `z`.
2. Anota el píxel de cada punto en la imagen.
3. Si el mapa sale girado o en espejo, ajusta `rotation` o `invert_x`/`invert_z`.

### SVG

Se rechazan SVG con `<script>`, `<foreignObject>`, `<iframe>`, `<embed>`, `<object>`, DOCTYPE o entidades, atributos `on*`, `javascript:`, `@import` y cualquier `href`/`src`/`url()` que no sea `#id` o una imagen raster en `data:`. Además, el SVG solo se muestra como imagen, nunca dentro de la página.

El paquete de ejemplo está en `assets/demo/` y se instala desde la app con «Instalar demo».
