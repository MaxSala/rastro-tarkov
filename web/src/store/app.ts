import { create } from "zustand";
import type { FolderCheck, Interface, MarkerKind, MarkerView, View } from "../lib/protocol";
import { loadPrefs, savePrefs, type Prefs } from "../lib/prefs";

export type Connection = "connecting" | "open" | "offline" | "no-token" | "expired";
export type Workspace = "map" | "market" | "ammo";
const WS_KEY = "rastro.workspace";
function loadWorkspace(): Workspace {
  try {
    const v = localStorage.getItem(WS_KEY);
    if (v === "market" || v === "ammo") return v;
  } catch {
    /* sin almacenamiento */
  }
  return "map";
}

export type PanelId = "extracts" | "points" | "party" | "maps";
export type DialogId = "import" | "settings" | "host" | "palette" | "point" | "changelog" | null;

/** Menú contextual del mapa: posición en pantalla y en píxeles del mapa, y el objeto bajo el cursor. */
export interface ContextMenu {
  x: number;
  y: number;
  px: number;
  py: number;
  target: { kind: "map" } | { kind: "point"; id: string } | { kind: "extract"; id: string };
}

/** Punto en edición: nuevo (con px/py) o existente (con id). */
export type PointDraft = { mode: "new"; px: number; py: number } | { mode: "edit"; id: string };
export type LayerId = "extracts" | MarkerKind;

const LAYERS_KEY = "rastro.layers";
const HIDDEN_KEY = "rastro.hiddenGroups";
function loadHidden(): string[] {
  try {
    const v: unknown = JSON.parse(localStorage.getItem(HIDDEN_KEY) ?? "[]");
    return Array.isArray(v) ? v.filter((x): x is string => typeof x === "string").slice(0, 100) : [];
  } catch {
    return [];
  }
}
function loadLayers(): Record<LayerId, boolean> {
  const base = { extracts: true, transit: true, switch: false, loot: false, lock: false };
  try {
    const saved: unknown = JSON.parse(localStorage.getItem(LAYERS_KEY) ?? "{}");
    if (saved && typeof saved === "object") {
      for (const k of Object.keys(base) as LayerId[]) {
        const v = (saved as Record<string, unknown>)[k];
        if (typeof v === "boolean") base[k] = v;
      }
    }
  } catch {
    /* preferencias corruptas: valores por defecto */
  }
  return base;
}

/** Tarjeta de una cerradura del mapa (posición en pantalla y datos del marcador). */
export interface KeyCardState {
  x: number;
  y: number;
  key: string;
  group: string | null;
  power: boolean;
  name: string;
}

export interface Toast {
  id: number;
  level: "info" | "error";
  text: string;
}

interface AppState {
  view: View | null;
  connection: Connection;
  interfaces: Interface[];
  panel: PanelId | null;
  dialog: DialogId;
  dialogArg: string | null;
  toasts: Toast[];
  markers: { mapId: string; list: MarkerView[] } | null;
  folderCheck: FolderCheck | null;
  layers: Record<LayerId, boolean>;
  prefs: Prefs;
  workspace: Workspace;
  setWorkspace: (w: Workspace) => void;
  /** Tipos de contenedor ocultos por el usuario. */
  hiddenGroups: string[];
  /** Piso elegido por mapa: "auto" o el id del piso. */
  floors: Record<string, string>;
  iconsVersion: number;
  setHiddenGroups: (g: string[]) => void;
  setFloor: (mapId: string, floor: string) => void;
  bumpIcons: () => void;
  logs: string[];
  /** Llave resaltada en la capa de cerraduras (las demás se atenúan). */
  focusKey: string | null;
  setFocusKey: (k: string | null) => void;
  keyCard: KeyCardState | null;
  setKeyCard: (c: KeyCardState | null) => void;
  /** Objeto que el Mercado debe abrir al montarse (desde la tarjeta de una cerradura). */
  marketItem: string | null;
  showInMarket: (id: string | null) => void;
  /** Desde el Mercado: abre el mapa, enciende la capa de cerraduras y resalta la llave. */
  showKeyOnMap: (key: string) => void;
  menu: ContextMenu | null;
  pointDraft: PointDraft | null;
  setPrefs: (p: Partial<Prefs>) => void;
  setLogs: (l: string[]) => void;
  setMenu: (m: ContextMenu | null) => void;
  editPoint: (d: PointDraft) => void;
  setMarkers: (mapId: string, list: MarkerView[]) => void;
  setFolderCheck: (r: FolderCheck | null) => void;
  toggleLayer: (l: LayerId) => void;
  setView: (v: View) => void;
  setConnection: (c: Connection) => void;
  setInterfaces: (l: Interface[]) => void;
  togglePanel: (p: PanelId) => void;
  openDialog: (d: DialogId, arg?: string) => void;
  notify: (level: Toast["level"], text: string) => void;
  dismiss: (id: number) => void;
}

let nextToast = 1;

export const useApp = create<AppState>((set, get) => ({
  view: null,
  connection: "connecting",
  interfaces: [],
  panel: "extracts",
  dialog: null,
  dialogArg: null,
  toasts: [],
  markers: null,
  folderCheck: null,
  layers: loadLayers(),
  prefs: loadPrefs(),
  workspace: loadWorkspace(),
  setWorkspace: (workspace) => {
    try {
      localStorage.setItem(WS_KEY, workspace);
    } catch {
      /* sin almacenamiento */
    }
    set({ workspace });
  },
  hiddenGroups: loadHidden(),
  floors: {},
  iconsVersion: 0,
  setHiddenGroups: (hiddenGroups) => {
    localStorage.setItem(HIDDEN_KEY, JSON.stringify(hiddenGroups));
    set({ hiddenGroups });
  },
  setFloor: (mapId, floor) => set({ floors: { ...get().floors, [mapId]: floor } }),
  bumpIcons: () => set({ iconsVersion: get().iconsVersion + 1 }),
  logs: [],
  focusKey: null,
  setFocusKey: (focusKey) => set({ focusKey }),
  keyCard: null,
  setKeyCard: (keyCard) => set({ keyCard, menu: null }),
  marketItem: null,
  showInMarket: (marketItem) => {
    set({ marketItem, keyCard: null });
    if (marketItem) get().setWorkspace("market");
  },
  showKeyOnMap: (key) => {
    const layers = { ...get().layers, lock: true };
    try {
      localStorage.setItem(LAYERS_KEY, JSON.stringify(layers));
    } catch {
      /* sin almacenamiento */
    }
    set({ layers, focusKey: key });
    get().setWorkspace("map");
  },
  menu: null,
  pointDraft: null,
  setPrefs: (p) => {
    const prefs = { ...get().prefs, ...p };
    savePrefs(prefs);
    set({ prefs });
  },
  setLogs: (logs) => set({ logs }),
  setMenu: (menu) => set({ menu, keyCard: null }),
  editPoint: (pointDraft) => set({ pointDraft, dialog: "point", menu: null }),
  setMarkers: (mapId, list) => set({ markers: { mapId, list } }),
  setFolderCheck: (folderCheck) => set({ folderCheck }),
  toggleLayer: (l) => {
    const layers = { ...get().layers, [l]: !get().layers[l] };
    localStorage.setItem(LAYERS_KEY, JSON.stringify(layers));
    set({ layers });
  },
  setView: (view) => set({ view }),
  setConnection: (connection) => set({ connection }),
  setInterfaces: (interfaces) => set({ interfaces }),
  togglePanel: (p) => set({ panel: get().panel === p ? null : p }),
  openDialog: (dialog, arg) => set({ dialog, dialogArg: arg ?? null }),
  notify: (level, text) => {
    const id = nextToast++;
    set({ toasts: [...get().toasts.slice(-3), { id, level, text }] });
    window.setTimeout(() => get().dismiss(id), level === "error" ? 7000 : 3500);
  },
  dismiss: (id) => set({ toasts: get().toasts.filter((t) => t.id !== id) }),
}));
