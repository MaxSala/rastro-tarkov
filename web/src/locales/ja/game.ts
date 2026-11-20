// 日本語。
import type { Game } from "../es/game";

const game: Game = {
  tab: "ゲーム",
  enable: "マップ・レイド・タスクを自動で検出",
  enableHint:
    "ゲームが PC に書き出すログを読み取ります。マップを自動で切り替え、新しいレイドを開始し、レイド時間を表示し、受注・完了・失敗したタスクを記録します。使うのはマップ・モード・タスク ID だけで、それ以外は保存も共有もしません。",
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
