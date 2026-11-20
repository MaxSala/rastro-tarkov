// English.
import type { Game } from "../es/game";

const game: Game = {
  tab: "Game",
  enable: "Detect map, raid and quests automatically",
  enableHint:
    "Reads the logs the game writes on your PC. Switches map on its own, starts a fresh raid, shows the raid time and ticks off the quests you accept, hand in or fail. Only the map, the mode and quest ids are used; nothing else is stored or shared.",
  folder: "Game logs folder",
  folderHint: "Empty = the default install folder (Battlestate Games\\Escape from Tarkov\\Logs).",
  apply: "Save folder",
  found: "Folder found",
  missing: "Logs folder not found",
  status: "Status",
  phase: {
    off: "Off",
    menu: "In the menu",
    loading: (m: string) => `Loading ${m}…`,
    raid: (m: string) => `In raid on ${m}`,
  },
  mode: "Detected mode",
  modes: { regular: "PvP", pve: "PvE" },
  unknown: "Not detected yet",
  dataMode: "tarkov.dev data",
  dataModeHint: "PvP and PvE have different data. Download the maps again to apply the change.",
  dataModes: { auto: "Automatic", regular: "PvP", pve: "PvE" },
  chip: {
    loading: (m: string) => `Loading ${m}`,
    left: (time: string) => `${time} left`,
    notInstalled: "You do not have this map installed: download it from the Maps panel.",
  },
};
export default game;
