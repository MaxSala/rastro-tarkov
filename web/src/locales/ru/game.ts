// Русский.
import type { Game } from "../es/game";

const game: Game = {
  tab: "Игра",
  enable: "Определять карту и рейд автоматически",
  enableHint:
    "Читает логи, которые игра пишет на твоём ПК. Сам меняет карту, начинает новый рейд и показывает время рейда. Используются только название карты и режим; ничего не сохраняется и не передаётся.",
  folder: "Папка логов игры",
  folderHint: "Пусто = папка установки по умолчанию (Battlestate Games\\Escape from Tarkov\\Logs).",
  apply: "Сохранить папку",
  found: "Папка найдена",
  missing: "Папка логов не найдена",
  status: "Состояние",
  phase: {
    off: "Отключено",
    menu: "В меню",
    loading: (m: string) => `Загрузка: ${m}…`,
    raid: (m: string) => `В рейде: ${m}`,
  },
  mode: "Обнаруженный режим",
  modes: { regular: "PvP", pve: "PvE" },
  unknown: "Ещё не определён",
  dataMode: "Данные tarkov.dev",
  dataModeHint: "У PvP и PvE разные данные. Скачай карты заново, чтобы применить изменение.",
  dataModes: { auto: "Авто", regular: "PvP", pve: "PvE" },
  chip: {
    loading: (m: string) => `Загрузка: ${m}`,
    left: (t: string) => `осталось ${t}`,
    notInstalled: "Эта карта не установлена: скачай её на панели «Карты».",
  },
};

export default game;
