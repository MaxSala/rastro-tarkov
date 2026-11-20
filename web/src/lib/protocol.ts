// Espejo de los mensajes del WebSocket. Fuente de verdad: src/protocol.rs (ClientMsg, ServerMsg)
// y src/view.rs (View) en el backend. Todo lo que llega se valida con zod antes de usarse.
import { z } from "zod";
import { LANGS, type Lang } from "../locales";
import { languageDataSchema } from "./languageData";
import { userMsgSchema } from "./msg";
import { progressSchema, questMarkerSchema, questsStatusSchema, type QuestClientMsg } from "./questProtocol";

const num = z.number().finite();
const point = z.tuple([num, num]);
const text = z.string().max(2000);

/** Floor id (maps with floors only): used to dim what isn't on the floor you're looking at. */
const floorId = z
  .string()
  .regex(/^[a-z0-9-]{1,40}$/)
  .optional();

const marker = z.object({
  px: num,
  py: num,
  angle: num,
  gameX: num,
  gameY: num,
  gameZ: num,
  floor: floorId,
});

export const viewSchema = z.object({
  version: text,
  settings: z.object({
    screenshotsDir: text,
    screenshotsDirIsDefault: z.boolean(),
    trailLength: z.number().int(),
    playerName: text,
    playerColor: z.number().int().min(0).max(7),
    partyPort: z.number().int(),
    shareWarningAck: z.boolean(),
    openInWindow: z.boolean(),
    language: z.enum(LANGS),
    checkUpdates: z.boolean().nullable(),
  }),
  watcher: z.object({
    folder: text.nullable(),
    error: userMsgSchema.nullable(),
    lastFile: text.nullable(),
    lastTakenAt: text.nullable(),
    raidHours: num.nullable(),
  }),
  packs: z
    .array(z.object({ id: text, name: text, author: text, license: text, extracts: z.number().int() }))
    .max(500),
  activeMap: z
    .object({
      id: text,
      name: text,
      width: z.number().int().positive(),
      height: z.number().int().positive(),
      imageUrl: z.string().regex(/^\/api\/packs\/[a-z0-9-]{1,40}\/image$/),
      level: text.nullable(),
      floors: z.array(z.object({ id: z.string().regex(/^[a-z0-9-]{1,40}$/), name: text })).max(16),
      autoFloor: z
        .string()
        .regex(/^[a-z0-9-]{1,40}$/)
        .nullable(),
      baseFloor: z
        .string()
        .regex(/^[a-z0-9-]{1,40}$/)
        .nullable(),
    })
    .nullable(),
  player: marker.nullable(),
  trail: z.array(point).max(2000),
  extracts: z
    .array(
      z.object({
        id: text,
        name: text,
        faction: z.enum(["pmc", "scav", "shared"]),
        conditions: text,
        switches: z.array(text).max(8).optional(),
        px: num,
        py: num,
        available: z.boolean(),
        dimmed: z.boolean(),
        distanceM: num.nullable(),
        floor: floorId,
      }),
    )
    .max(500),
  selectedExtract: text.nullable(),
  points: z
    .array(
      z.object({
        id: z.string().regex(/^pt-[0-9a-f]{1,20}$/),
        name: text,
        kind: z.enum(["marker", "loot", "danger", "key", "quest", "stash"]),
        note: text,
        px: num,
        py: num,
        distanceM: num.nullable(),
        floor: floorId,
      }),
    )
    .max(500),
  dataDir: text,
  guide: z.object({ from: point, to: point, distanceM: num, heightDiffM: num, relativeDeg: num }).nullable(),
  party: z.object({
    status: z.enum(["idle", "hosting", "connecting", "connected", "error"]),
    error: userMsgSchema.nullable(),
    isHost: z.boolean(),
    code: text.nullable(),
    peers: z
      .array(
        z.object({
          id: z.number().int(),
          name: text,
          color: z.number().int().min(0).max(7),
          onThisMap: z.boolean(),
          marker: marker.nullable(),
        }),
      )
      .max(16),
    pings: z
      .array(z.object({ color: z.number().int().min(0).max(7), px: num, py: num, remainingMs: num }))
      .max(64),
  }),
  onboarding: z.boolean(),
  game: z.object({
    enabled: z.boolean(),
    folder: text.nullable(),
    folderFound: z.boolean(),
    mode: z.enum(["regular", "pve"]).nullable(),
    modePref: z.enum(["auto", "regular", "pve"]),
    phase: z.enum(["menu", "loading", "raid"]).nullable(),
    mapName: text.nullable(),
    mapInstalled: z.boolean(),
    startedAt: z
      .string()
      .regex(/^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}$/)
      .nullable(),
    raidMinutes: z.number().int().min(1).max(600).nullable(),
  }),
  market: z.object({
    state: z.enum(["empty", "loading", "ready", "error"]),
    fetchedAt: z.number().int().nullable(),
    count: z.number().int(),
    mode: text.nullable(),
    lang: text.nullable(),
    error: userMsgSchema.nullable(),
  }),
  update: z.object({
    state: z.enum(["idle", "checking", "upToDate", "available", "error"]),
    latest: z
      .string()
      .regex(/^v\d{1,4}\.\d{1,4}\.\d{1,4}$/)
      .nullable(),
    url: z
      .string()
      .max(200)
      .regex(/^https:\/\/github\.com\/[\w.-]+\/[\w.-]+\/releases\//)
      .nullable(),
    error: userMsgSchema.nullable(),
  }),
  tarkovdev: z.object({
    state: z.enum(["idle", "running", "done", "error"]),
    step: z.enum(["maps", "data", "install", "icons"]).nullable(),
    current: text.nullable(),
    done: z.number().int(),
    total: z.number().int(),
    installed: z.number().int(),
    failed: z.array(text).max(64),
    iconsMissing: z.number().int(),
    noData: z.boolean(),
    error: userMsgSchema.nullable(),
  }),
  quests: questsStatusSchema,
  languageData: languageDataSchema,
  questMarkers: z.array(questMarkerSchema).max(400),
});

const iconName = z.string().regex(/^[a-z0-9_-]{1,48}$/);

/** Id de objeto de tarkov.dev: 24 caracteres hexadecimales. */
export const itemId = z.string().regex(/^[0-9a-f]{24}$/);

export const markerSchema = z.object({
  kind: z.enum(["loot", "transit", "switch", "lock"]),
  group: z
    .string()
    .regex(/^[a-z0-9-]{1,40}$/)
    .nullable(),
  icon: iconName.nullable(),
  name: text,
  px: num,
  py: num,
  // Cerraduras: id de la llave (objeto de tarkov.dev) y si necesita corriente.
  key: itemId.optional(),
  power: z.boolean().optional(),
  floor: floorId,
});

export const serverMsgSchema = z.discriminatedUnion("type", [
  z.object({ type: z.literal("snapshot"), view: viewSchema }),
  z.object({
    type: z.literal("interfaces"),
    list: z.array(z.object({ name: text, ip: text })).max(64),
  }),
  z.object({ type: z.literal("notice"), level: z.enum(["info", "error"]) }).extend(userMsgSchema.shape),
  z.object({
    type: z.literal("folderCheck"),
    result: z.object({ path: text, exists: z.boolean(), captures: z.number().int() }),
  }),
  z.object({
    type: z.literal("mapData"),
    mapId: text,
    markers: z.array(markerSchema).max(3000),
    icons: z.array(iconName).max(64),
  }),
  z.object({ type: z.literal("logs"), lines: z.array(text).max(5000) }),
  z.object({ type: z.literal("questProgress"), progress: progressSchema }),
]);

export type View = z.infer<typeof viewSchema>;
export type ExtractView = View["extracts"][number];
export type PeerView = View["party"]["peers"][number];
export type Faction = ExtractView["faction"];
export type ServerMsg = z.infer<typeof serverMsgSchema>;
export type Interface = Extract<ServerMsg, { type: "interfaces" }>["list"][number];
export type MarkerView = z.infer<typeof markerSchema>;
export type MarkerKind = MarkerView["kind"];
export type PointView = View["points"][number];
export type PointKind = PointView["kind"];
export type FolderCheck = Extract<ServerMsg, { type: "folderCheck" }>["result"];

export interface SettingsInput {
  screenshotsDir: string;
  trailLength: number;
  playerName: string;
  playerColor: number;
  partyPort: number;
}

export type ClientMsg =
  | { type: "auth"; token: string }
  | { type: "setActiveMap"; mapId: string }
  | { type: "newRaid" }
  | { type: "selectExtract"; extractId: string | null }
  | { type: "toggleExtract"; extractId: string }
  | { type: "updateSettings"; settings: SettingsInput }
  | { type: "ackShareWarning" }
  | { type: "partyHost"; bindIp: string; advertise: string }
  | { type: "partyJoin"; code: string }
  | { type: "partyLeave" }
  | { type: "partyKick"; peerId: number }
  | { type: "mapPing"; px: number; py: number }
  | { type: "listInterfaces" }
  | { type: "removePack"; packId: string }
  | { type: "tarkovDevSync" }
  | { type: "finishOnboarding" }
  | { type: "checkFolder"; path: string }
  | { type: "setScreenshotsDir"; path: string }
  | { type: "addPoint"; px: number; py: number; name: string; kind: PointKind; note: string }
  | { type: "updatePoint"; id: string; name: string; kind: PointKind; note: string }
  | { type: "deletePoint"; id: string }
  | { type: "getLogs" }
  | { type: "openDataDir" }
  | { type: "setOpenInWindow"; value: boolean }
  | { type: "quit" }
  | { type: "setGameLogs"; enabled: boolean; path?: string }
  | { type: "setGameMode"; mode: "auto" | "regular" | "pve" }
  | { type: "marketRefresh"; fleaName: string }
  | { type: "setLanguage"; lang: Lang }
  | { type: "updateLanguageData"; fleaName: string }
  | { type: "setAutoLanguageData"; enabled: boolean }
  | { type: "setCheckUpdates"; enabled: boolean }
  | { type: "checkUpdates" }
  | QuestClientMsg;

/** Respuesta de vista previa de importación (src/maps/import.rs, Preview). */
export const previewSchema = z.object({
  stagingId: z.string().regex(/^[0-9a-f]{32}$/),
  id: text,
  name: text,
  author: text,
  license: text,
  source: text,
  extracts: z.number().int(),
  width: z.number().int(),
  height: z.number().int(),
  sha256: z.string().regex(/^[0-9a-f]{64}$/),
  sha256Verified: z.boolean().nullable(),
  exists: z.boolean(),
});
export type Preview = z.infer<typeof previewSchema>;
