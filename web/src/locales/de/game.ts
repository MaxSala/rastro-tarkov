// Deutsch.
import type { Game } from "../es/game";

const game: Game = {
  tab: "Spiel",
  enable: "Karte und Raid automatisch erkennen",
  enableHint:
    "Liest die Logs, die das Spiel auf deinem PC schreibt. Wechselt selbst die Karte, startet einen neuen Raid und zeigt die Raidzeit. Es werden nur Kartenname und Modus genutzt; nichts wird gespeichert oder geteilt.",
  folder: "Ordner der Spiel-Logs",
  folderHint: "Leer = der Standard-Installationsordner (Battlestate Games\\Escape from Tarkov\\Logs).",
  apply: "Ordner speichern",
  found: "Ordner gefunden",
  missing: "Log-Ordner nicht gefunden",
  status: "Status",
  phase: {
    off: "Deaktiviert",
    menu: "Im Menü",
    loading: (m: string) => `Lade ${m}…`,
    raid: (m: string) => `Im Raid auf ${m}`,
  },
  mode: "Erkannter Modus",
  modes: { regular: "PvP", pve: "PvE" },
  unknown: "Noch nicht erkannt",
  dataMode: "Daten von tarkov.dev",
  dataModeHint: "PvP und PvE haben unterschiedliche Daten. Lade die Karten neu, um die Änderung anzuwenden.",
  dataModes: { auto: "Automatisch", regular: "PvP", pve: "PvE" },
  chip: {
    loading: (m: string) => `Lade ${m}`,
    left: (t: string) => `noch ${t}`,
    notInstalled: "Diese Karte ist nicht installiert: Lade sie in der Leiste Karten herunter.",
  },
};

export default game;
