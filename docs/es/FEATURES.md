<p align="center">
  <a href="../../README.es.md">Volver al README</a> · <b>Funciones</b> · <a href="GUIDE.md">Guía</a> · <a href="FAQ.md">Preguntas y seguridad</a>
</p>

# Funciones

<table>
  <tr><td width="50%"><img src="../images/hero.png" alt="Customs en raid"><br><sub>Customs en raid</sub></td><td width="50%"><img src="../images/squad.png" alt="Tu squad en un mapa"><br><sub>Tu squad en un mapa</sub></td></tr>
  <tr><td width="50%"><img src="../images/quests.png" alt="Misiones por comerciante"><br><sub>Misiones por comerciante</sub></td><td width="50%"><img src="../images/hideout.png" alt="Escondite"><br><sub>Escondite</sub></td></tr>
  <tr><td width="50%"><img src="../images/bosses.png" alt="Jefes y sus puntos de aparición"><br><sub>Jefes y sus puntos de aparición</sub></td><td width="50%"><img src="../images/clock.png" alt="Reloj de raid y tren"><br><sub>Reloj de raid y tren</sub></td></tr>
  <tr><td width="50%"><img src="../images/keys.png" alt="Cerraduras y precio de llaves"><br><sub>Cerraduras y precio de llaves</sub></td><td width="50%"><img src="../images/floors.png" alt="Pisos, automáticamente"><br><sub>Pisos, automáticamente</sub></td></tr>
  <tr><td width="50%"><img src="../images/market.png" alt="Mercado"><br><sub>Mercado</sub></td><td width="50%"><img src="../images/ammo.png" alt="Tabla de munición"><br><sub>Tabla de munición</sub></td></tr>
</table>

<sub>Mapas de los <a href="https://github.com/the-hideout/tarkov-dev-svg-maps">colaboradores de tarkov.dev</a>, CC BY-NC-SA 4.0. Precios y datos de objetos de <a href="https://tarkov.dev">tarkov.dev</a>. Las capturas de la squad usan posiciones de ejemplo. Las capturas muestran la app en inglés.</sub>

## Funciones

**Mapa**
- Tu posición y orientación, actualizadas cada vez que haces una captura, con un rastro que se va desvaneciendo de por dónde has pasado.
- Haz clic en una extracción o en cualquier punto para ver la distancia, hacia dónde girar y la diferencia de altura.
- Extracciones filtradas por PMC, Scav o compartidas. Marca las que el juego te dice que están abiertas en esta raid y el resto se atenúa.
- Capas de tarkov.dev: tránsitos, interruptores, contenedores de loot (filtrables por tipo) y cerraduras.
- Los mapas con varias plantas te siguen automáticamente, o eliges una en el panel de plantas. El resto del mapa y todo lo que está en otras plantas se atenúa.
- Pasa el ratón por cualquier cosa para ver una tarjeta con qué es, a qué distancia está, en qué planta y qué pasa si haces clic.
- Tus propios puntos: clic derecho en el mapa para guardar sitios de loot, zonas peligrosas, escondites o lugares de misiones. Se guardan en coordenadas del juego, así que sobreviven a reinstalar un mapa.

**Misiones**
- Todas las misiones en tu idioma: las que llevas, las que puedes empezar ya y, para el resto, exactamente qué te falta (tu nivel o las misiones anteriores). Filtra por comerciante, mapa o Kappa, o busca misiones, objetivos y objetos.
- **Las misiones siguen al juego.** Con la detección de raid activada, las misiones que aceptas, entregas o fallas se marcan solas desde los logs del juego, PvP y PvE por separado. La primera vez, Rastro lee todas las partidas que el juego ha guardado, así que a mitad de wipe te pones al día al instante. ¿Sin logs? *Ponte al día* eligiendo la última misión que hiciste con cada comerciante.
- Cada misión muestra su comerciante, su imagen y sus recompensas: experiencia, reputación, objetos, y las ofertas y fabricaciones que desbloquea.
- Tus objetivos pendientes en el mapa, honestos con los datos: un distintivo para un sitio exacto, el contorno real para una zona y puntos pequeños para los sitios donde un objeto *puede* aparecer. Haz clic en uno y Rastro te guía, como a una extracción. ¿Ves uno mal? *¿Mal situado?* lo reporta en un clic.
- **Plan de raid** en el panel *Misiones*: marca lo que harás en esta raid y el mapa enseña solo eso, numerado por la ruta más corta, con lo que tienes que llevarte. *Siguiente* muestra en grande el próximo objetivo; *Para entregar*, lo que ya está listo.
- Misiones por comerciante, como en el juego, con tu progreso de Kappa y Lightkeeper, el mapa donde más avanzas y *Objetos*: lo que tus misiones aún piden, para que no lo vendas.

**Escondite**
- Marca el nivel de cada módulo y mira qué pide el siguiente: objetos, otros módulos, confianza con comerciantes, habilidades y tiempo de obra. Los requisitos se ponen en verde cuando los cumples.
- *Objetos* suma misiones y escondite: lo que ambos aún piden, para el siguiente nivel o para todos los que faltan. La ficha del objeto en el Mercado dice qué módulos lo necesitan.

**Jefes y reloj de raid**
- La capa *Jefes* muestra dónde puede aparecer cada jefe y la probabilidad de encontrarlo ahí. Pasa el ratón para ver la zona, sus escoltas y cuándo llega; haz clic para ver todos sus puntos de aparición. Las probabilidades siguen tu modo de juego.
- En el menú, la barra superior muestra la hora en Tarkov a cada lado, para elegir día o noche. En raid: el tiempo restante y, según tu última captura, la hora en tu raid.
- Haz clic para ver cuándo acaba la raid, la ventana del tren blindado en Reserve y Lighthouse, y los jefes del mapa.

**Llaves**
- La capa *Cerraduras* muestra cada puerta, contenedor y maletero que necesita llave. Las que necesitan electricidad llevan un borde amarillo.
- Haz clic en una cerradura para ver qué llave la abre, dónde comprarla más barata y su mejor precio de venta.
- *Resaltar esta llave* atenúa el resto de cerraduras, para que veas todo lo que abre una sola llave.
- Cada llave del Mercado indica en qué mapas instalados se usa, con un enlace que te lleva directamente allí.

**Mercado**
- Busca en el catálogo completo por nombre o nombre corto. Los acentos y el orden de las palabras no importan. Pulsa `/` para buscar y las flechas para moverte por los resultados.
- Ordena por mejor venta, precio en el flea, media de 24 horas, cambio en 48 horas o precio por casilla del inventario.
- Cada objeto muestra todas las ofertas de comerciantes y del flea, con el nivel de comerciante necesario y el precio original en dólares o euros.
- El catálogo se descarga una vez y se busca en tu PC, así que las búsquedas son instantáneas y funcionan sin conexión.

**Munición**
- Todas las balas agrupadas por calibre y ordenadas por penetración.
- Daño (por perdigón × perdigones en postas), penetración, daño a la armadura, probabilidad de fragmentación, velocidad en boca y el sitio más barato para comprarla. Las trazadoras van marcadas.

**Detección de raids** (opcional)
- Cambia al mapa correcto cuando carga una raid, borra tu rastro y tus marcas al empezar cada raid y muestra el tiempo que queda. Lee los logs del juego, como se explica en [Cómo funciona](GUIDE.md#cómo-funciona).

**Todo lo demás**
- 16 idiomas, los mismos que tarkov.dev: Čeština, Deutsch, English, Español, Français, Magyar, Italiano, 日本語, 한국어, Polski, Português, Română, Русский, Slovenčina, Türkçe y 中文. Los nombres de objetos, mapas y extracciones también siguen tu idioma. Si lo cambias, Rastro te ofrece pasar tus mapas, precios y misiones al nuevo idioma.
- Datos de PvP y PvE, elegidos automáticamente a partir de los logs del juego o a mano.
- Avisos de actualización opcionales y una página de *Novedades* después de cada actualización.
- Atajos de teclado para todo y una paleta de comandos (`Ctrl+K`).

## Juega con tu squad

El modo party de Rastro pone a toda tu squad en el mismo mapa, cada uno con su nombre y su color.

- **Ve dónde está cada uno.** La flecha de cada compañero muestra dónde estaba y hacia dónde miraba en su última captura. Si alguien está en otra planta, su flecha se atenúa y al pasar el ratón te dice en qué planta está.
- **Marca un sitio.** `Shift` + clic en cualquier punto del mapa y ese sitio parpadea en la pantalla de todos.
- **Fácil de montar.** Uno abre una sala y copia un código de invitación. Los demás lo pegan en *Unirse*. Y ya está. Hasta 8 jugadores.
- **Sin servidor y sin cuenta.** Las posiciones van directamente de PC a PC. Nadie más las ve, ni siquiera nosotros.
- **Cifrado.** Todo el tráfico va cifrado de extremo a extremo con el [protocolo Noise](https://noiseprotocol.org/). El código de invitación es la clave, así que compártelo solo con tu squad.

Los demás tienen que poder llegar al PC del anfitrión. Lo más fácil es una VPN mesh gratuita como [Tailscale](https://tailscale.com/) o [ZeroTier](https://www.zerotier.com/): cada uno la instala una vez y Rastro funciona como si estuvierais en la misma red, sin abrir puertos en el router. Abrir un puerto en el router del anfitrión también vale.

Cada jugador sigue usando Rastro en su propio PC y hace sus propias capturas: eso es lo que mantiene al día la posición de todos.
