// Português.
import type { Game } from "../es/game";

const game: Game = {
  tab: "Jogo",
  enable: "Detectar mapa e raid automaticamente",
  enableHint:
    "Lê os logs que o jogo grava no seu PC. Troca de mapa sozinho, começa uma nova raid e mostra o tempo de raid. Só o nome do mapa e o modo são usados; nada é salvo nem compartilhado.",
  folder: "Pasta de logs do jogo",
  folderHint: "Vazio = a da instalação padrão (Battlestate Games\\Escape from Tarkov\\Logs).",
  apply: "Salvar pasta",
  found: "Pasta encontrada",
  missing: "Pasta de logs não encontrada",
  status: "Estado",
  phase: {
    off: "Desativado",
    menu: "No menu",
    loading: (m: string) => `Carregando ${m}…`,
    raid: (m: string) => `Em raid em ${m}`,
  },
  mode: "Modo detectado",
  modes: { regular: "PvP", pve: "PvE" },
  unknown: "Ainda não detectado",
  dataMode: "Dados do tarkov.dev",
  dataModeHint: "PvP e PvE têm dados diferentes. Baixe os mapas de novo para aplicar a mudança.",
  dataModes: { auto: "Automático", regular: "PvP", pve: "PvE" },
  chip: {
    loading: (m: string) => `Carregando ${m}`,
    left: (t: string) => `faltam ${t}`,
    notInstalled: "Você não tem este mapa instalado: baixe-o no painel Mapas.",
  },
};

export default game;
