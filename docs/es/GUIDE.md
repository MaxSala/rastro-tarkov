<p align="center">
  <a href="../../README.es.md">Volver al README</a> · <a href="FEATURES.md">Funciones</a> · <b>Guía</b> · <a href="FAQ.md">Preguntas y seguridad</a>
</p>

# Guía

## Instalación

1. **[Descarga `Rastro-windows-x64.exe`](https://github.com/MaxSala/rastro-tarkov/releases/latest/download/Rastro-windows-x64.exe)**. Este enlace siempre descarga la última versión.
2. Ponlo donde quieras: el escritorio, una carpeta en Documentos, donde sea. No necesita ningún otro archivo al lado.
3. Haz doble clic. Si Windows muestra un aviso azul, mira [¿Es seguro?](FAQ.md#es-seguro).

Necesita Windows 10 u 11 de 64 bits.

### Primer arranque

La primera vez se abre una guía corta:

1. **Carpeta de capturas.** Rastro encuentra `Documents\Escape from Tarkov\Screenshots` por sí solo, también cuando Windows ha movido Documentos a OneDrive. Si no la encuentra, pega la ruta y te dice cuántas capturas con coordenadas ve. Aquí también puedes activar la detección de raids.
2. **Mapas.** Descarga los mapas de la comunidad desde tarkov.dev (recomendado), importa un pack de mapas o prueba el mapa de demostración.
3. Listo. Haz una captura en el juego y tu flecha aparece en el mapa.

Para usar el Mercado, la tabla de Munición y los nombres de llaves, abre una vez la pestaña Mercado y pulsa *Descargar precios*.

### Actualizar

Si permitiste buscar actualizaciones, cuando sale una versión nueva aparece un botón en la barra superior que te lleva a su página de descarga. Si no, descarga el nuevo `.exe` desde el [enlace de arriba](#instalación), cierra Rastro y usa el archivo nuevo en lugar del viejo. Tus ajustes, puntos, mapas y precios se guardan [aparte](#dónde-se-guardan-tus-datos), así que no se pierde nada.

Después de actualizar, Rastro te enseña qué ha cambiado. El historial completo está en *Ajustes → Acerca de → Novedades* y en [CHANGELOG.md](../../CHANGELOG.md). Si una actualización trae datos de mapas nuevos, usa *Actualizar desde tarkov.dev* en el panel de Mapas para conseguirlos.

### Atajos

| Acción | Cómo |
|---|---|
| Actualizar tu posición | Haz una captura en el juego |
| Cambiar de sección (Mapa, Mercado, Munición, Misiones, Escondite) | `Ctrl+1`, `Ctrl+2`, `Ctrl+3`, `Ctrl+4`, `Ctrl+5` |
| Paleta de comandos | `Ctrl+K` |
| Paneles de extracciones, puntos, party, mapas y misiones | `1`, `2`, `3`, `4`, `5` |
| Nueva raid (borrar rastro y marcas) | `N` |
| Ver el mapa entero / seguir tu posición | `F` / `G` |
| Zoom | `+` / `-` o la rueda del ratón |
| Añadir un punto, guiar hacia un sitio | Clic derecho en el mapa |
| Hacer ping a tu squad | `Shift` + clic |
| Buscar en el Mercado | `/` |
| Ajustes | `,` |

### Dónde se guardan tus datos

Los ajustes, tus puntos, tu progreso de misiones, los mapas descargados y las cachés de precios y misiones se guardan en una carpeta, separada del `.exe`:

| Sistema | Carpeta |
|---|---|
| Windows | `%APPDATA%\Rastro\data\` |
| Linux | `~/.local/share/rastro/` |
| macOS | `~/Library/Application Support/Rastro/` |

En *Ajustes → Registro* hay un botón para abrirla.

### Si algo no funciona

- **Se abre en el navegador en vez de en su propia ventana.** Rastro usa Microsoft WebView2 para dibujar su ventana. Viene con Windows 10 y 11, pero si se ha desinstalado, Rastro se abre en tu navegador. Instalando el [WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/) vuelve la ventana.
- **Una página de error dice que el puerto ya está en uso.** Seguramente sigue abierta otra copia de Rastro. Ciérrala desde la barra de tareas o desde el Administrador de tareas (busca `rastro.exe`) y vuelve a abrirlo.
- **Mi posición nunca se mueve.** Comprueba que la carpeta de capturas de *Ajustes → General* es la que el juego usa de verdad y que estás pulsando la tecla de captura del juego.
- **Mis compañeros no pueden entrar en mi sala.** Tienen que poder llegar a tu PC. Usa la dirección de tu VPN mesh (Tailscale, ZeroTier) al abrir la sala, o abre el puerto de party en tu router.
- **¿Sigues atascado?** Pregunta en [Discord](https://discord.gg/H4pRxGBGDb). *Ajustes → Registro* tiene un botón *Copiar*, y pegar ese registro ayuda mucho.

## Cómo funciona

Rastro lee dos tipos de archivos que Escape from Tarkov ya guarda en tu PC. No necesita nada más.

| Qué lee | Dónde | Qué saca de ahí |
|---|---|---|
| **Los nombres de tus capturas** | `Documents\Escape from Tarkov\Screenshots` | Tu posición y hacia dónde miras. Tarkov escribe tus coordenadas en el nombre de cada captura que guarda. Rastro lee el nombre, nunca la imagen, y nunca mueve, edita ni borra tus capturas. |
| **Los logs del juego** (opcional, desactivado hasta que lo actives) | `Battlestate Games\Escape from Tarkov\Logs` | Qué mapa se está cargando, cuándo empieza una raid y si juegas PvP o PvE. Con eso Rastro cambia solo al mapa correcto, empieza un rastro nuevo en cada raid y te enseña cuánto tiempo de raid queda. Solo se guardan el nombre del mapa y el modo de juego. También lee los mensajes de los comerciantes, para marcar las misiones que aceptas, entregas o fallas. |

En la práctica: pulsas la tecla de captura en el juego y un segundo después tu flecha se mueve en el mapa.

Lo que Rastro **no** hace: no lee la memoria del juego, no inyecta nada, no se engancha al proceso del juego, no captura tráfico de red, no dibuja encima del juego ni pulsa teclas por ti. Es un programa aparte que solo mira archivos en el disco.

## Compilar desde el código

Solo lo necesitas si quieres cambiar el código o compilar el `.exe` tú mismo. Te hacen falta [Rust](https://rustup.rs/) 1.85 o superior y [Node.js](https://nodejs.org/) LTS (en Linux, también `libwebkit2gtk-4.1-dev` y `libgtk-3-dev`):

```sh
git clone https://github.com/MaxSala/rastro-tarkov.git
cd rastro-tarkov
./build.sh        # build.bat en Windows
```

El resultado es un único ejecutable en `target/release/`. La configuración para desarrollar, la estructura del proyecto, las normas de código y cómo añadir una traducción están en **[CONTRIBUTING.md](../../CONTRIBUTING.md)** (en inglés).
