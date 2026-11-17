// Čeština.
import type { Game } from "../es/game";

const game: Game = {
  tab: "Hra",
  enable: "Automaticky rozpoznat mapu a raid",
  enableHint:
    "Čte logy, které hra zapisuje na tvém PC. Sám přepne mapu, začne nový raid a ukáže čas raidu. Používá se jen název mapy a režim; nic se neukládá ani nesdílí.",
  folder: "Složka logů hry",
  folderHint: "Prázdné = složka výchozí instalace (Battlestate Games\\Escape from Tarkov\\Logs).",
  apply: "Uložit složku",
  found: "Složka nalezena",
  missing: "Složka logů nenalezena",
  status: "Stav",
  phase: {
    off: "Vypnuto",
    menu: "V menu",
    loading: (m: string) => `Načítání: ${m}…`,
    raid: (m: string) => `V raidu: ${m}`,
  },
  mode: "Zjištěný režim",
  modes: { regular: "PvP", pve: "PvE" },
  unknown: "Zatím nezjištěno",
  dataMode: "Data z tarkov.dev",
  dataModeHint: "PvP a PvE mají různá data. Pro použití změny stáhni mapy znovu.",
  dataModes: { auto: "Automaticky", regular: "PvP", pve: "PvE" },
  chip: {
    loading: (m: string) => `Načítání: ${m}`,
    left: (t: string) => `zbývá ${t}`,
    notInstalled: "Tuto mapu nemáš nainstalovanou: stáhni ji v panelu Mapy.",
  },
};

export default game;
