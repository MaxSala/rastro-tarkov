// Magyar.
import type { Game } from "../es/game";

const game: Game = {
  tab: "Játék",
  enable: "Térkép és raid automatikus felismerése",
  enableHint:
    "Olvassa a naplókat, amelyeket a játék a gépedre ír. Magától térképet vált, új raidet kezd, és mutatja a raid idejét. Csak a térkép nevét és a módot használja; semmit nem ment és nem oszt meg.",
  folder: "A játék naplómappája",
  folderHint: "Üres = az alapértelmezett telepítés mappája (Battlestate Games\\Escape from Tarkov\\Logs).",
  apply: "Mappa mentése",
  found: "Mappa megtalálva",
  missing: "A naplómappa nem található",
  status: "Állapot",
  phase: {
    off: "Kikapcsolva",
    menu: "A menüben",
    loading: (m: string) => `Betöltés: ${m}…`,
    raid: (m: string) => `Raidben: ${m}`,
  },
  mode: "Felismert mód",
  modes: { regular: "PvP", pve: "PvE" },
  unknown: "Még nincs felismerve",
  dataMode: "tarkov.dev adatok",
  dataModeHint: "A PvP és a PvE adatai eltérnek. A változás alkalmazásához töltsd le újra a térképeket.",
  dataModes: { auto: "Automatikus", regular: "PvP", pve: "PvE" },
  chip: {
    loading: (m: string) => `Betöltés: ${m}`,
    left: (t: string) => `még ${t}`,
    notInstalled: "Ez a térkép nincs telepítve: töltsd le a Térképek panelen.",
  },
};

export default game;
