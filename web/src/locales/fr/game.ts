// Français.
import type { Game } from "../es/game";

const game: Game = {
  tab: "Jeu",
  enable: "Détecter la carte, le raid et les quêtes automatiquement",
  enableHint:
    "Lit les journaux que le jeu écrit sur votre PC. Change de carte tout seul, démarre un nouveau raid, affiche le temps de raid et coche les quêtes que vous acceptez, rendez ou ratez. Seuls la carte, le mode et les identifiants de quête sont utilisés ; rien d’autre n’est enregistré ni partagé.",
  folder: "Dossier des journaux du jeu",
  folderHint: "Vide = celui de l’installation par défaut (Battlestate Games\\Escape from Tarkov\\Logs).",
  apply: "Enregistrer le dossier",
  found: "Dossier trouvé",
  missing: "Dossier des journaux introuvable",
  status: "État",
  phase: {
    off: "Désactivé",
    menu: "Dans le menu",
    loading: (m: string) => `Chargement de ${m}…`,
    raid: (m: string) => `En raid sur ${m}`,
  },
  mode: "Mode détecté",
  modes: { regular: "PvP", pve: "PvE" },
  unknown: "Pas encore détecté",
  dataMode: "Données de tarkov.dev",
  dataModeHint:
    "PvP et PvE ont des données différentes. Retélécharge les cartes pour appliquer le changement.",
  dataModes: { auto: "Automatique", regular: "PvP", pve: "PvE" },
  chip: {
    loading: (m: string) => `Chargement de ${m}`,
    left: (t: string) => `reste ${t}`,
    notInstalled: "Cette carte n’est pas installée : télécharge-la dans le panneau Cartes.",
  },
};

export default game;
