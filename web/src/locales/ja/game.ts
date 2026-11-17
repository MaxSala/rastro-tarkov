// 日本語。
import type { Game } from "../es/game";

const game: Game = {
  tab: "ゲーム",
  enable: "マップとレイドを自動検出",
  enableHint:
    "ゲームが PC に書き出すログを読み取ります。マップの自動切り替え、新しいレイドの開始、レイド時間の表示を行います。使うのはマップ名とモードだけで、保存や共有は一切しません。",
  folder: "ゲームのログフォルダー",
  folderHint: "空欄 = 既定のインストール先 (Battlestate Games\\Escape from Tarkov\\Logs)。",
  apply: "フォルダーを保存",
  found: "フォルダーが見つかりました",
  missing: "ログフォルダーが見つかりません",
  status: "状態",
  phase: {
    off: "オフ",
    menu: "メニュー",
    loading: (m: string) => `${m} を読み込み中…`,
    raid: (m: string) => `${m} でレイド中`,
  },
  mode: "検出されたモード",
  modes: { regular: "PvP", pve: "PvE" },
  unknown: "未検出",
  dataMode: "tarkov.dev のデータ",
  dataModeHint: "PvP と PvE ではデータが異なります。変更を反映するにはマップを再ダウンロードしてください。",
  dataModes: { auto: "自動", regular: "PvP", pve: "PvE" },
  chip: {
    loading: (m: string) => `${m} を読み込み中`,
    left: (t: string) => `残り ${t}`,
    notInstalled: "このマップはインストールされていません。マップパネルからダウンロードしてください。",
  },
};

export default game;
