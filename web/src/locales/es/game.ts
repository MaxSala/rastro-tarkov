// Español (idioma de referencia). Detección por logs del juego.
// El resto de idiomas se tipan contra estos archivos: una clave que falte o sobre no compila.
const game = {
  tab: "Juego",
  enable: "Detectar mapa, raid y misiones automáticamente",
  enableHint:
    "Lee los logs que el juego escribe en tu PC. Cambia de mapa solo, empieza una raid nueva, muestra el tiempo de raid y marca las misiones que aceptas, entregas o fallas. Solo se usan el mapa, el modo y los identificadores de misión; no se guarda ni se comparte nada más.",
  folder: "Carpeta de logs del juego",
  folderHint: "Vacío = la de la instalación por defecto (Battlestate Games\\Escape from Tarkov\\Logs).",
  apply: "Guardar carpeta",
  found: "Carpeta encontrada",
  missing: "No se encuentra la carpeta de logs",
  status: "Estado",
  phase: {
    off: "Desactivado",
    menu: "En el menú",
    loading: (m: string) => `Cargando ${m}…`,
    raid: (m: string) => `En raid en ${m}`,
  },
  mode: "Modo detectado",
  modes: { regular: "PvP", pve: "PvE" },
  unknown: "Aún no detectado",
  dataMode: "Datos de tarkov.dev",
  dataModeHint: "PvP y PvE tienen datos distintos. Vuelve a descargar los mapas para aplicar el cambio.",
  dataModes: { auto: "Automático", regular: "PvP", pve: "PvE" },
  chip: {
    loading: (m: string) => `Cargando ${m}`,
    left: (t: string) => `quedan ${t}`,
    notInstalled: "No tienes este mapa instalado: descárgalo en el panel Mapas.",
  },
};

export type Game = typeof game;
export default game;
