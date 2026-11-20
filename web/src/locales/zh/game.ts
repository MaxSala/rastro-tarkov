// 中文（简体）。
import type { Game } from "../es/game";

const game: Game = {
  tab: "游戏",
  enable: "自动检测地图、战局和任务",
  enableHint:
    "读取游戏写在你电脑上的日志。自动切换地图、开始新战局、显示战局时间，并标记你接受、提交或失败的任务。只使用地图、模式和任务 ID；不保存也不分享其他任何内容。",
  folder: "游戏日志文件夹",
  folderHint: "留空 = 默认安装位置 (Battlestate Games\\Escape from Tarkov\\Logs)。",
  apply: "保存文件夹",
  found: "已找到文件夹",
  missing: "找不到日志文件夹",
  status: "状态",
  phase: {
    off: "已关闭",
    menu: "在菜单中",
    loading: (m: string) => `正在加载 ${m}…`,
    raid: (m: string) => `正在 ${m} 战局中`,
  },
  mode: "识别到的模式",
  modes: { regular: "PvP", pve: "PvE" },
  unknown: "尚未识别",
  dataMode: "tarkov.dev 数据",
  dataModeHint: "PvP 和 PvE 的数据不同。重新下载地图以应用更改。",
  dataModes: { auto: "自动", regular: "PvP", pve: "PvE" },
  chip: {
    loading: (m: string) => `正在加载 ${m}`,
    left: (t: string) => `剩余 ${t}`,
    notInstalled: "你没有安装此地图：请在地图面板中下载。",
  },
};

export default game;
