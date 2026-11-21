<p align="center">
  <a href="../../README.es.md">Volver al README</a> · <a href="FEATURES.md">Funciones</a> · <a href="GUIDE.md">Guía</a> · <b>Preguntas y seguridad</b>
</p>

# Preguntas y seguridad

## ¿Es seguro?

Rastro es un programa pequeño y portable. Esto es lo que significa y cómo puedes comprobarlo.

- **Sin instalador y sin permisos de administrador.** Es un único `.exe`. No instala servicios ni drivers, no arranca con Windows y no toca los archivos del juego. Sus ajustes y mapas descargados viven en una carpeta (mira [Dónde se guardan tus datos](GUIDE.md#dónde-se-guardan-tus-datos)), y borrando esa carpeta y el `.exe` desaparece por completo.
- **Código abierto.** Todo el código está en este repositorio. Cualquiera puede leerlo.
- **Compilado en público.** Cada versión la compilan los propios servidores de GitHub directamente desde el código, no el PC de nadie. Puedes ver cada compilación en la pestaña [Actions](https://github.com/MaxSala/rastro-tarkov/actions), y la página de cada versión muestra el SHA-256 del archivo para que compruebes que tienes exactamente ese:

  ```powershell
  Get-FileHash .\Rastro-windows-x64.exe
  ```

- **No envía nada a casa.** Sin telemetría, sin anuncios, sin cuenta. Solo se conecta a internet cuando tú se lo pides, y la lista completa de conexiones está en [Privacidad](#privacidad-y-uso-de-red).

**"Windows protegió su PC".** La primera vez que abras Rastro, lo más probable es que Windows SmartScreen muestre una ventana azul diciendo que ha detenido una aplicación no reconocida. Pasa con cualquier programa nuevo que no esté firmado con un certificado de pago, no porque el archivo tenga algo malo. Haz clic en **Más información** y luego en **Ejecutar de todas formas**. Windows recuerda tu elección. Algunos antivirus también desconfían de los programas nuevos sin firmar. Si el tuyo marca Rastro, puedes restaurarlo y añadir una excepción, subir el archivo a [VirusTotal](https://www.virustotal.com/) para tener una segunda opinión y reportarlo como falso positivo a tu antivirus.

> [!IMPORTANT]
> Rastro es un proyecto de fans no oficial, sin relación con Battlestate Games ni con tarkov.dev, y sin su respaldo. Está hecho para mantenerse fuera del juego: solo lee archivos del disco y nunca interactúa con el proceso del juego. Aun así, usar herramientas de terceros con Escape from Tarkov es responsabilidad tuya según los términos de servicio de Battlestate Games.

## Preguntas

**¿Me pueden banear por usarlo?**
Rastro nunca lee la memoria del juego, nunca toca el proceso del juego y nunca envía pulsaciones al juego. Lee nombres de capturas y logs después de que el juego los haya escrito, los mismos archivos que puedes abrir tú en el Explorador. Pero no podemos hablar por Battlestate Games, así que lee la nota de [¿Es seguro?](#es-seguro).

**¿Funciona en PvE?**
Sí. La detección de raids se ha probado con logs de PvE, y Rastro descarga los precios y datos de PvE cuando detecta PvE.

**¿Necesita internet?**
Solo para descargar mapas y precios, buscar actualizaciones (si lo permites) y jugar en squad. Todo lo demás funciona sin conexión.

**¿Por qué capturas?**
Son el único sitio donde el juego escribe tu posición exacta en el disco. Unos segundos de retraso a cambio de no tocar nunca el juego.

**¿Linux o macOS?**
Rastro compila y funciona en Linux; macOS no está probado. Tarkov solo funciona en Windows, así que la descarga es solo para Windows. Mira [Compilar desde el código](GUIDE.md#compilar-desde-el-código).

## Privacidad y uso de red

Rastro no tiene cuenta, ni telemetría, ni anuncios. La interfaz solo habla con la propia app en `127.0.0.1`, protegida por una clave aleatoria que cambia en cada arranque. Todo lo que lee de tu disco se queda en tu PC. Lo demás solo pasa cuando tú lo pides:

| Cuándo | Se conecta a |
|---|---|
| Siempre | Nada fuera de tu PC |
| Descargas mapas de tarkov.dev | `raw.githubusercontent.com`, `json.tarkov.dev` y, como alternativa, `api.tarkov.dev` |
| Descargas o actualizas precios | `json.tarkov.dev`, y `assets.tarkov.dev` para los iconos de objetos |
| Descargas o actualizas misiones | `json.tarkov.dev` y, como alternativa, `api.tarkov.dev`, y `assets.tarkov.dev` para las imágenes |
| Importas un pack de mapas desde una URL | Solo la dirección que escribas (solo HTTPS, nunca direcciones internas) |
| Abres una sala o te unes a una party | Directamente a los PC de tus compañeros |
| Una vez por arranque, si dijiste que sí a buscar actualizaciones (o al pulsar *Buscar ahora*) | `api.github.com`, para leer el número de la última versión. No se envía nada sobre ti. |

Cada servidor de descarga está en una lista fija de permitidos, que también se comprueba en las redirecciones, y todo lo descargado se valida antes de usarse. El modelo de amenazas completo está en [docs/SECURITY.md](../SECURITY.md) (en inglés).

## Limitaciones conocidas

- Tu posición solo se actualiza cuando haces una captura. Es así a propósito: es el único sitio donde el juego escribe tu posición.
- Labs no tiene mapa SVG en tarkov.dev. Rastro lo reconoce, pero no puede mostrarlo a menos que importes tu propio pack de mapas.
- La detección de raids se ha comprobado con logs de PvE. En PvP debería funcionar igual, pero no se ha confirmado con logs reales.
- Las traducciones al español y al inglés están revisadas. Las otras 14 todavía no las han comprobado hablantes nativos. Las correcciones son muy bienvenidas, aquí o en [Discord](https://discord.gg/H4pRxGBGDb).
- Para ver los nombres nuevos al cambiar de idioma o de modo de juego (PvP/PvE) hay que volver a descargar mapas y precios.
- Algunos detalles técnicos de los mensajes de error y del registro están en inglés sea cual sea tu idioma.

## Sin ánimo de lucro

Rastro es un proyecto hecho por afición para la comunidad. Es gratis, no tiene versión de pago, no muestra anuncios y no recoge datos. Seguirá así, por decisión propia y porque los mapas de tarkov.dev en los que se basa tienen una licencia solo para uso no comercial.
