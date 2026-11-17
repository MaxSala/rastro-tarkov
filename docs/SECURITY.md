# Seguridad

## Modelo de amenazas

Rastro corre en el PC del jugador, abre un servidor HTTP en 127.0.0.1 y, solo si se crea o se une a una sala, una conexión TCP del party. Activos que proteger: control de la app, archivos del usuario, la posición en partida y el secreto de la sala.

| # | Amenaza | Defensa principal |
|---|---------|-------------------|
| 1 | Una web cualquiera abierta en el navegador ataca el servidor local (CSRF, WebSocket cross-origin, DNS rebinding) | Solo 127.0.0.1; `Host` y `Origin` exactos; token de 256 bits obligatorio en /api y /ws; `Content-Type` exigido; sin CORS |
| 2 | Paquetes de mapa maliciosos (zip-slip, bomba zip, imagen bomba, SVG con scripts, XSS por textos) | Extracción defensiva con contadores reales; dimensiones leídas de la cabecera sin decodificar; SVG filtrado y mostrado solo como `<img>` desde blob; textos solo como texto |
| 3 | Compañeros maliciosos o desconocidos que conectan al puerto del party | Noise NNpsk0 con secreto de 256 bits; tope de trama antes de reservar; 4 pendientes, 5 s de handshake, 8 compañeros, límite de tasa; validación campo a campo; identidad = conexión |
| 4 | Importación por URL (SSRF, redirecciones internas, descargas enormes) | Solo https sin credenciales; DNS propio que rechaza IPs internas; conexión fijada a la IP validada; 3 redirecciones revalidadas; tope de bytes en streaming; sin proxy ni cookies |
| 5 | Cadena de suministro | Versiones exactas; `cargo deny`; `ignore-scripts=true`; sin CDN ni recursos remotos; dependencias con al menos dos semanas de antigüedad al fijarlas |
| 7 | Datos externos de tarkov.dev (SVG, maps.json, API) manipulados o comprometidos | Solo bajo petición; lista blanca fija de dominios también en redirecciones; mismas defensas SSRF; topes de bytes; SVG filtrado; manifest generado y validado como uno importado; textos saneados |
| 8 | Logs del juego con datos personales (id de perfil y de cuenta, IPs de servidores) | Desactivado por defecto; solo lectura; se leen dos tipos de archivo y se extraen con regex el modo, el nombre interno del mapa (validado) y la hora; nada más se conserva, registra ni comparte |
| 6 | Fugas en logs y disco | El token y el código no se registran; posiciones del party solo en memoria; config acotada y validada al leer |

## Controles implementados

- **Servidor local** — `src/server/mod.rs`: `check_request` (Host, Origin, `Sec-Fetch-Site`, token en tiempo constante con `subtle`, Content-Type, métodos), cabeceras CSP/nosniff/no-referrer/CORP/COOP/X-Frame-Options en todas las respuestas, límite de peticiones en vuelo, de cuerpo (64 KiB; 50 MB solo en la subida). `src/server/ws.rs`: autenticación con el primer mensaje en 5 s, 64 KiB por mensaje, 4 sockets. `src/main.rs`: escucha solo en `127.0.0.1`; error claro si el puerto está ocupado.
- **Token** — `src/main.rs` lo genera con `getrandom`; viaja en el fragmento `#t=` y la web lo pasa a `sessionStorage` y lo borra de la barra (`web/src/lib/token.ts`). Nunca va en la query.
- **Paquetes** — `src/maps/zipx.rs` (rutas, enlaces, duplicados, control, 10 entradas, 50/100 MB, ratio 100:1 contado al descomprimir); `src/maps/image.rs` (bytes mágicos, 8192 px, filtro de SVG); `src/maps/manifest.rs` (`deny_unknown_fields`, regex de id, longitudes, números finitos, 500 extracciones, versión de esquema); `src/maps/pack.rs` (instalación por renombrado, sin sobrescribir sin confirmación); `src/maps/import.rs` (SHA-256 opcional); `src/server/api.rs` (`image`: lista blanca id + archivo del manifest, CSP `sandbox`; una importación a la vez).
- **Web** — `web/src/components/map/markers.ts` construye todo con nodos DOM y `textContent`; `web/eslint.config.js` prohíbe `innerHTML`, `insertAdjacentHTML`, `dangerouslySetInnerHTML` y `eval`; `web/src/lib/protocol.ts` valida con zod todo lo que llega; `web/vite.config.ts` impide assets inline.
- **URL** — `src/maps/download.rs` (`fetch_with` con `allowed_hosts` para tarkov.dev).
- **tarkov.dev** — `src/tarkovdev/sync.rs` (dominios fijos `raw.githubusercontent.com`, `json.tarkov.dev` y `api.tarkov.dev`, topes de 4/20/32 MB, un mapa que falla no para al resto), `src/tarkovdev/calib.rs` (maps.json: solo ids y nombres de archivo válidos, rotaciones 0/90/180/270, números finitos), `src/tarkovdev/api.rs` (todo opcional, textos sin control/bidi y recortados, ids únicos y válidos, coordenadas finitas, límites de 500 extracciones y 3000 marcadores). El usuario ve la licencia y los dominios y debe aceptar antes de descargar.
- **Ventana de escritorio** — `src/desktop.rs`: la WebView solo navega dentro de `http://127.0.0.1:<puerto>`; cualquier otra navegación se bloquea y, si es https, se abre en el navegador del sistema; las ventanas nuevas se deniegan; herramientas de desarrollo desactivadas en release; la página de error escapa el texto.
- **Mercado** — `src/market/fetch.rs`: dominios fijos `json.tarkov.dev` y `assets.tarkov.dev` (también en redirecciones), solo al pulsar el botón, una descarga a la vez; topes de 160 MB para objetos y 64 MB por archivo de textos; el JSON se procesa en un hilo aparte; caché en `<datos>/market/{modo}-{idioma}.json` escrita de forma atómica y leída con tope de 64 MB. `src/market/build.rs`: ids de 24 hex, slugs validados, textos sin control/bidi y recortados, precios finitos y acotados, ofertas de comerciantes desconocidos descartadas, como mucho 20 000 objetos. `/api/market` sirve el catálogo procesado, no lo descargado. Las búsquedas se hacen en la web y nunca salen del PC.
- **Iconos de objetos** — `src/market/icons.rs`: id de 24 hex (sin rutas posibles), solo `assets.tarkov.dev`, bytes mágicos PNG/WebP/JPEG, ≤ 96 KB y ≤ 256 px, caché en disco, 4 descargas a la vez. `/api/item-icon/{id}` exige token; la web los usa como `blob:`.
- **Registro** — `src/logs.rs`: memoria (2000 líneas) y `rastro.log` (reescrito al arrancar, máximo 5 MB). Nunca se registran el token ni el código del party.
- **Puntos propios** — `src/points.rs`: archivo acotado (4 MB), `deny_unknown_fields`, ids `pt-` validados, textos sin control/bidi, 500 por mapa y 64 mapas; lo inválido se descarta al cargar.
- **Iconos** — `src/tarkovdev/icons.rs`: lista cerrada de nombres, PNG validado (bytes mágicos, ≤ 64 KB, ≤ 256 px), guardado atómico; `/api/icons/{nombre}` solo sirve nombres de la lista. La web los usa como `blob:`.
- **Pisos** — `src/server/api.rs` (`floor_variant`): el SVG se sirve con un `<style>` que oculta los demás pisos; los ids de grupo se validan en el manifest (`is_valid_layer_id`: letras, dígitos, `_` y `-`), así no se puede inyectar CSS ni marcado.
- **Logs del juego** — `src/gamelog/`: `parse.rs` (función pura con regex; el nombre de mapa se valida `[A-Za-z0-9_]{1,40}`), `watcher.rs` (sondeo cada 1 s de la sesión más reciente; al arrancar se leen como mucho los últimos 4 MB, luego hasta 1 MB por vuelta; línea incompleta limitada a 64 KB; archivo truncado → se reinicia), `control.rs` (solo cambia el mapa activo y reinicia la raid). Tests con líneas reales anonimizadas.
- **Primer arranque** — `src/setup.rs`: comprobar una carpeta solo lista nombres (máximo 20 000) y nunca abre archivos.
- **Party** — `src/party/secure.rs` (versión antes del handshake, Noise), `src/party/frame.rs` (16 KiB), `src/party/session.rs` (latido 5 s, corte a 21 s, 10 msg/s con ráfaga de 30), `src/party/host.rs` (pendientes, sala de 8, expulsión, ids asignados por el anfitrión), `src/party/proto.rs` (validación), `src/party/code.rs` + `zeroize`.
- **Código** — `#![forbid(unsafe_code)]`; clippy `unwrap_used`, `expect_used`, `panic`, `indexing_slicing` en `deny` (Cargo.toml); los errores de una conexión terminan esa conexión, no el servidor.

## Checklist de autorrevisión (hecha antes de entregar)

- [x] Ninguna ruta sirve archivos arbitrarios; el id se valida antes de tocar disco (test de traversal en `guard_tests.rs`).
- [x] Todo lo que cambia estado exige token + Origin; un formulario de otra web no puede fijar `application/json` ni la cabecera del token.
- [x] Ningún texto de manifest, nombre de compañero o error llega al DOM como HTML (revisado `markers.ts`, tooltips de Leaflet con `HTMLElement`, componentes React).
- [x] Corregido: las importaciones no tenían límite de concurrencia (32 × 50 MB en memoria); ahora una a la vez.
- [x] Corregido: carrera al cerrar y reabrir una sala (la tarea antigua reseteaba la nueva); número de generación y test `reopening_quickly_keeps_the_new_room`.
- [x] Corregido: nombres con caracteres bidi/invisibles (U+202E…) permitían suplantar visualmente; ahora se rechazan.
- [x] Orden de bloqueos sin anidamiento entre `inner` y `party` (sin interbloqueos).
- [x] Los mensajes de error del party no incluyen el código ni partes del secreto (test en `code_tests.rs`).
- [x] Corregido: los primeros manifests de Customs dados a mano tenían la calibración invertida en vertical; la regla automática se verificó con dos pruebas independientes en los 10 mapas.
- [x] Corregido: `setMaxBounds` antes de `fitBounds` desplazaba la imagen medio mapa y dejaba el lienzo del loot fuera de vista.
- [x] Verificado en Chromium real: ninguna violación de CSP, token eliminado de la URL tras cargar.

## Tests de seguridad

zip-slip, bomba zip, entradas duplicadas/no permitidas (`zipx_tests.rs`); imagen sobredimensionada, bytes mágicos, SVG con script/referencias externas (`image_tests.rs`); manifest con campos desconocidos y strings enormes (`import_tests.rs`); SSRF a loopback, 169.254.169.254, `localhost` y por redirección (`download_tests.rs`); Host, Origin, token, Content-Type, traversal, WebSocket cross-origin (`guard_tests.rs`); trama sobredimensionada (`frame_tests.rs`); repetición y manipulación de tramas, clave errónea, versión distinta (`secure_tests.rs`); timeout de latido y flood (`session_tests.rs`); nombres con caracteres de control, tipos y campos desconocidos (`proto_tests.rs`); límite de pendientes, sala llena, expulsión, puerto liberado (`host_tests.rs`).

## Debilidades de la referencia evitadas

La referencia (TarkovTracker, sin archivo de licencia: no se ha copiado nada) se leyó sin ejecutarla.

- **Actualizaciones ejecutables sin verificación criptográfica**: `Services/AppUpdateService.cs` descarga (l. 195) y lanza (l. 228, `Process.Start`) un ejecutable; solo comprueba que la URL es https de GitHub, sin hash ni firma (no aparece ninguna verificación en el archivo). Rastro no se actualiza solo.
- **Descarga de datos en ejecución desde un servicio externo**: `Services/TarkovDevJsonClient.cs` (l. 60, `json.tarkov.dev`; l. 175 vuelca la respuesta a disco). Rastro no contacta con nadie salvo la URL que escribe el usuario.
- **Descargas sin tope de bytes visible**: `AppUpdateService.cs` l. 308 y `TarkovDevJsonClient.cs` l. 175 copian el flujo completo. Rastro corta al superar el tope.
- **Lectura de JSON sin límite de tamaño**: `Services/MapDataService.cs` l. 157, 202, 249 (`File.ReadAllText`). Rastro usa `read_limited`.
- `Assets/map-view.js` usa `innerHTML` (l. 143, 145, 364) solo para vaciar capas; no es explotable, pero Rastro lo prohíbe por lint.
- No se encontraron en la referencia UPnP, consultas de IP pública ni un party en red, así que no se citan.

## Riesgos residuales (sin maquillar)

- **Malware local**: cualquier proceso con tu usuario puede leer el token de la memoria o de la consola y el código de la sala; Rastro no protege frente a un PC comprometido.
- **Token visible**: con `--no-open` o si falla abrir el navegador, la URL con el token se imprime en la consola. El fragmento queda en el historial del navegador hasta que la web lo borra; caduca al cerrar Rastro.
- **Assets estáticos sin token**: la web compilada se sirve sin token (no contiene datos); sí exige Host correcto.
- **El código del party es una contraseña portadora**: quien lo tenga entra. Un compañero legítimo puede mentir sobre su propia posición o hacer ping donde quiera; puede ver la tuya.
- **Puerto expuesto**: mientras la sala está abierta, el puerto acepta conexiones. Sin el secreto no pasan del handshake, pero un escaneo sabe que hay algo escuchando. Mejor una VPN de malla.
- **Código manipulado**: un código falso puede hacer que tu app conecte a un host:puerto arbitrario (solo envía 6 bytes de preámbulo).
- **SVG**: el filtro es por patrones, no un parser XML; la defensa real es mostrarlo como `<img>` (los navegadores no ejecutan scripts ahí) y la CSP `sandbox`.
- **DNS**: se rechazan IPs internas al resolver, pero si el dominio es legítimo y luego sirve contenido malicioso, solo el SHA-256 lo detecta.
- **Dependencias**: se confía en `snow`, `rustls`, `ring` y el resto; `cargo deny`/`cargo audit`/`npm audit` reducen, no eliminan, el riesgo.
- **tarkov.dev**: si su repositorio o su API se vieran comprometidos, podrían servir mapas o nombres engañosos (no código: el SVG se filtra y se muestra como imagen, los textos como texto). Rastro confía en el certificado TLS de GitHub y de api.tarkov.dev; no hay firma de los datos.
- **Licencia de los mapas**: su cláusula prohíbe el uso en software que dé ventaja injusta; la interpretación para el party corresponde a sus autores.
- **WebView del sistema**: la ventana usa WebView2 (Windows) o WebKitGTK (Linux); sus fallos de seguridad dependen de que el sistema esté actualizado. El contenido mostrado es siempre la web local de Rastro.
- **Registro en disco**: `rastro.log` puede contener rutas de carpetas y nombres de mapas.
- **Datos de mercado**: un catálogo manipulado en origen podría mostrar precios o nombres falsos (se muestran como texto, nunca como marcado). La caché local la puede editar cualquier proceso con tu usuario.
- **Formato de los logs**: Battlestate puede cambiarlo sin aviso; si cambia, la detección deja de funcionar (no falla nada más).
- **Modo `--dev`**: amplía los orígenes aceptados a `localhost:5173`; no usarlo para jugar.
