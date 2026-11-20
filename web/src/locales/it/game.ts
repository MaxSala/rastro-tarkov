// Italiano.
import type { Game } from "../es/game";

const game: Game = {
  tab: "Gioco",
  enable: "Rileva mappa, raid e missioni automaticamente",
  enableHint:
    "Legge i log che il gioco scrive sul tuo PC. Cambia mappa da solo, inizia un nuovo raid, mostra il tempo del raid e segna le missioni che accetti, consegni o fallisci. Si usano solo la mappa, la modalità e gli id delle missioni; nient’altro viene salvato o condiviso.",
  folder: "Cartella dei log del gioco",
  folderHint: "Vuoto = quella dell’installazione predefinita (Battlestate Games\\Escape from Tarkov\\Logs).",
  apply: "Salva cartella",
  found: "Cartella trovata",
  missing: "Cartella dei log non trovata",
  status: "Stato",
  phase: {
    off: "Disattivato",
    menu: "Nel menu",
    loading: (m: string) => `Caricamento di ${m}…`,
    raid: (m: string) => `In raid su ${m}`,
  },
  mode: "Modalità rilevata",
  modes: { regular: "PvP", pve: "PvE" },
  unknown: "Non ancora rilevata",
  dataMode: "Dati di tarkov.dev",
  dataModeHint: "PvP e PvE hanno dati diversi. Riscarica le mappe per applicare la modifica.",
  dataModes: { auto: "Automatico", regular: "PvP", pve: "PvE" },
  chip: {
    loading: (m: string) => `Caricamento di ${m}`,
    left: (t: string) => `mancano ${t}`,
    notInstalled: "Non hai questa mappa installata: scaricala nel pannello Mappe.",
  },
};

export default game;
