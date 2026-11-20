// Čeština.
import type { Game } from "../es/game";

const game: Game = {
  tab: "Hra",
  enable: "Automaticky rozpoznat mapu, raid a úkoly",
  enableHint:
    "Čte logy, které hra zapisuje do tvého PC. Sám přepne mapu, začne nový raid, ukáže čas raidu a odškrtne úkoly, které přijmeš, odevzdáš nebo nesplníš. Používá se jen mapa, režim a ID úkolů; nic dalšího se neukládá ani nesdílí.",
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
