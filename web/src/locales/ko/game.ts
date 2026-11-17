// 한국어.
import type { Game } from "../es/game";

const game: Game = {
  tab: "게임",
  enable: "맵과 레이드 자동 감지",
  enableHint:
    "게임이 PC에 기록하는 로그를 읽습니다. 맵을 자동으로 바꾸고, 새 레이드를 시작하며, 레이드 시간을 표시합니다. 맵 이름과 모드만 사용하며, 아무것도 저장하거나 공유하지 않습니다.",
  folder: "게임 로그 폴더",
  folderHint: "비워 두면 기본 설치 폴더 (Battlestate Games\\Escape from Tarkov\\Logs).",
  apply: "폴더 저장",
  found: "폴더를 찾았습니다",
  missing: "로그 폴더를 찾을 수 없습니다",
  status: "상태",
  phase: {
    off: "꺼짐",
    menu: "메뉴",
    loading: (m: string) => `${m} 불러오는 중…`,
    raid: (m: string) => `${m}에서 레이드 중`,
  },
  mode: "감지된 모드",
  modes: { regular: "PvP", pve: "PvE" },
  unknown: "아직 감지되지 않음",
  dataMode: "tarkov.dev 데이터",
  dataModeHint: "PvP와 PvE는 데이터가 다릅니다. 변경을 적용하려면 맵을 다시 받으세요.",
  dataModes: { auto: "자동", regular: "PvP", pve: "PvE" },
  chip: {
    loading: (m: string) => `${m} 불러오는 중`,
    left: (t: string) => `${t} 남음`,
    notInstalled: "이 맵이 설치되어 있지 않습니다. 맵 패널에서 내려받으세요.",
  },
};

export default game;
