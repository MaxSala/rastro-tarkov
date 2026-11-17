# Avisos de terceros

Rastro (MIT, ver LICENSE) incluye o enlaza estos componentes principales. La lista completa de crates sale de `cargo deny list` y la de paquetes npm de `npm ls`.

## Tipografía (incluida en la web compilada)
- Geist Sans y Geist Mono — Vercel, SIL Open Font License 1.1, vía `@fontsource-variable/geist` y `@fontsource-variable/geist-mono`.

## Web
- React y React DOM — MIT · Leaflet — BSD-2-Clause · zod — MIT · zustand — MIT · lucide-react — ISC · @radix-ui/react-tooltip — MIT · Tailwind CSS (build) — MIT · Vite (build) — MIT.

## Rust
- tokio, axum, hyper, tower, tracing, bytes — MIT · serde, serde_json, regex, anyhow, thiserror, getrandom, sha2, data-encoding, zeroize, directories, rust-embed, reqwest, zip, opener — MIT o Apache-2.0 · snow — Apache-2.0 o MIT · rustls — Apache-2.0, ISC o MIT · ring — Apache-2.0 e ISC · webpki-roots — CDLA-Permissive-2.0 · notify — CC0-1.0 · subtle, curve25519-dalek — BSD-3-Clause · if-addrs — MIT o BSD-3-Clause · embed-resource (solo al compilar: icono del .exe) — MIT · tao, wry — MIT o Apache-2.0 (ventana y WebView; en Linux enlazan gtk-rs y webkit2gtk-rs, MIT; en Windows webview2-com, MIT) · option-ext — MPL-2.0 · datos ICU — Unicode-3.0.

## Mapas y datos de tarkov.dev (descargados por el usuario, no incluidos)
- Mapas SVG — the-hideout/tarkov-dev-svg-maps, CC BY-NC-SA 4.0, con prohibición expresa de uso en software de trampas. Rastro guarda la autoría y la licencia en cada paquete.
- Calibración y pisos — `src/data/maps.json` de the-hideout/tarkov-dev.
- Iconos de marcadores — `public/maps/interactive` de the-hideout/tarkov-dev, licencia MIT, Copyright (c) 2019 Oskar Risberg. La tabla de equivalencias de contenedores (`src/tarkovdev/icons.rs`) sigue la de `src/pages/map/map-images.mjs` del mismo proyecto.
- Extracciones, transiciones, interruptores y contenedores — API pública de tarkov.dev.

## Precios y munición de tarkov.dev (descargados por el usuario, no incluidos)
- Catálogo de objetos, comerciantes y textos — `json.tarkov.dev`, el mismo origen que usa la web de tarkov.dev. El formato se tomó leyendo su cliente (the-hideout/tarkov-dev, MIT); no se ha copiado código.
- Iconos de objetos — `assets.tarkov.dev`. Pertenecen a Battlestate Games; Rastro solo los guarda en caché para mostrarlos.
- Nombres de calibre (`src/market/build.rs`) — tabla propia con los nombres de uso común.

## Paquete demo
- `assets/demo/` — creado para este proyecto, CC0-1.0. No representa ningún mapa real de Escape from Tarkov.

Escape from Tarkov es una marca de Battlestate Games. Rastro no está afiliado ni respaldado por Battlestate Games.
