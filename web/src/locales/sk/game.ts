// Slovenčina.
import type { Game } from "../es/game";

const game: Game = {
  tab: "Hra",
  enable: "Automaticky rozpoznať mapu a raid",
  enableHint:
    "Číta logy, ktoré hra zapisuje na tvojom PC. Sám prepne mapu, začne nový raid a ukáže čas raidu. Používa sa len názov mapy a režim; nič sa neukladá ani nezdieľa.",
  folder: "Priečinok logov hry",
  folderHint: "Prázdne = priečinok predvolenej inštalácie (Battlestate Games\\Escape from Tarkov\\Logs).",
  apply: "Uložiť priečinok",
  found: "Priečinok nájdený",
  missing: "Priečinok logov sa nenašiel",
  status: "Stav",
  phase: {
    off: "Vypnuté",
    menu: "V menu",
    loading: (m: string) => `Načítava sa: ${m}…`,
    raid: (m: string) => `V raide: ${m}`,
  },
  mode: "Zistený režim",
  modes: { regular: "PvP", pve: "PvE" },
  unknown: "Zatiaľ nezistené",
  dataMode: "Dáta z tarkov.dev",
  dataModeHint: "PvP a PvE majú rôzne dáta. Na použitie zmeny stiahni mapy znova.",
  dataModes: { auto: "Automaticky", regular: "PvP", pve: "PvE" },
  chip: {
    loading: (m: string) => `Načítava sa: ${m}`,
    left: (t: string) => `zostáva ${t}`,
    notInstalled: "Túto mapu nemáš nainštalovanú: stiahni ju v paneli Mapy.",
  },
};

export default game;
