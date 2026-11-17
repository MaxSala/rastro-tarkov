// Polski.
import type { Game } from "../es/game";

const game: Game = {
  tab: "Gra",
  enable: "Wykrywaj mapę i rajd automatycznie",
  enableHint:
    "Czyta logi, które gra zapisuje na twoim PC. Sam zmienia mapę, zaczyna nowy rajd i pokazuje czas rajdu. Używane są tylko nazwa mapy i tryb; nic nie jest zapisywane ani udostępniane.",
  folder: "Folder logów gry",
  folderHint: "Puste = folder domyślnej instalacji (Battlestate Games\\Escape from Tarkov\\Logs).",
  apply: "Zapisz folder",
  found: "Folder znaleziony",
  missing: "Nie znaleziono folderu logów",
  status: "Stan",
  phase: {
    off: "Wyłączone",
    menu: "W menu",
    loading: (m: string) => `Wczytywanie: ${m}…`,
    raid: (m: string) => `W rajdzie: ${m}`,
  },
  mode: "Wykryty tryb",
  modes: { regular: "PvP", pve: "PvE" },
  unknown: "Jeszcze nie wykryto",
  dataMode: "Dane z tarkov.dev",
  dataModeHint: "PvP i PvE mają różne dane. Pobierz mapy ponownie, aby zastosować zmianę.",
  dataModes: { auto: "Automatycznie", regular: "PvP", pve: "PvE" },
  chip: {
    loading: (m: string) => `Wczytywanie: ${m}`,
    left: (t: string) => `zostało ${t}`,
    notInstalled: "Nie masz zainstalowanej tej mapy: pobierz ją w panelu Mapy.",
  },
};

export default game;
