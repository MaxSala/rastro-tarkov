// Türkçe.
import type { Game } from "../es/game";

const game: Game = {
  tab: "Oyun",
  enable: "Haritayı ve baskını otomatik algıla",
  enableHint:
    "Oyunun bilgisayarına yazdığı günlükleri okur. Haritayı kendisi değiştirir, yeni baskın başlatır ve baskın süresini gösterir. Yalnızca harita adı ve mod kullanılır; hiçbir şey kaydedilmez veya paylaşılmaz.",
  folder: "Oyun günlükleri klasörü",
  folderHint: "Boş = varsayılan kurulum klasörü (Battlestate Games\\Escape from Tarkov\\Logs).",
  apply: "Klasörü kaydet",
  found: "Klasör bulundu",
  missing: "Günlük klasörü bulunamadı",
  status: "Durum",
  phase: {
    off: "Kapalı",
    menu: "Menüde",
    loading: (m: string) => `${m} yükleniyor…`,
    raid: (m: string) => `${m} baskınında`,
  },
  mode: "Algılanan mod",
  modes: { regular: "PvP", pve: "PvE" },
  unknown: "Henüz algılanmadı",
  dataMode: "tarkov.dev verileri",
  dataModeHint: "PvP ve PvE farklı verilere sahip. Değişikliği uygulamak için haritaları yeniden indir.",
  dataModes: { auto: "Otomatik", regular: "PvP", pve: "PvE" },
  chip: {
    loading: (m: string) => `${m} yükleniyor`,
    left: (t: string) => `${t} kaldı`,
    notInstalled: "Bu harita yüklü değil: Haritalar panelinden indir.",
  },
};

export default game;
