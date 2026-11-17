// Română.
import type { Game } from "../es/game";

const game: Game = {
  tab: "Joc",
  enable: "Detectează automat harta și raidul",
  enableHint:
    "Citește jurnalele pe care jocul le scrie pe PC-ul tău. Schimbă singur harta, pornește un raid nou și arată timpul de raid. Se folosesc doar numele hărții și modul; nimic nu se salvează și nu se partajează.",
  folder: "Dosarul jurnalelor jocului",
  folderHint: "Gol = cel al instalării implicite (Battlestate Games\\Escape from Tarkov\\Logs).",
  apply: "Salvează dosarul",
  found: "Dosar găsit",
  missing: "Dosarul jurnalelor nu a fost găsit",
  status: "Stare",
  phase: {
    off: "Dezactivat",
    menu: "În meniu",
    loading: (m: string) => `Se încarcă ${m}…`,
    raid: (m: string) => `În raid pe ${m}`,
  },
  mode: "Mod detectat",
  modes: { regular: "PvP", pve: "PvE" },
  unknown: "Încă nedetectat",
  dataMode: "Date tarkov.dev",
  dataModeHint: "PvP și PvE au date diferite. Descarcă din nou hărțile ca să aplici schimbarea.",
  dataModes: { auto: "Automat", regular: "PvP", pve: "PvE" },
  chip: {
    loading: (m: string) => `Se încarcă ${m}`,
    left: (t: string) => `mai sunt ${t}`,
    notInstalled: "Nu ai această hartă instalată: descarc-o din panoul Hărți.",
  },
};

export default game;
