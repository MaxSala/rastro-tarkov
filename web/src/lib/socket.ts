import { lang, t } from "../locales";
import { useApp } from "../store/app";
import { serverMsgSchema, type ClientMsg } from "./protocol";
import { readToken } from "./token";
import { loadIcons } from "./iconCache";

let ws: WebSocket | null = null;
let retry = 0;
let timer: number | undefined;
let quickFailures = 0;

export function send(msg: ClientMsg): void {
  if (ws?.readyState === WebSocket.OPEN) ws.send(JSON.stringify(msg));
}

function schedule() {
  window.clearTimeout(timer);
  const delay = Math.min(5000, 500 * 2 ** retry);
  retry += 1;
  timer = window.setTimeout(connect, delay);
}

export function connect(): void {
  const token = readToken();
  const store = useApp.getState();
  if (!token) return store.setConnection("no-token");
  window.clearTimeout(timer);
  ws?.close();
  store.setConnection("connecting");
  const proto = window.location.protocol === "https:" ? "wss" : "ws";
  const socket = new WebSocket(`${proto}://${window.location.host}/ws`);
  ws = socket;
  let gotSnapshot = false;

  socket.onopen = () => {
    socket.send(JSON.stringify({ type: "auth", token } satisfies ClientMsg));
    // El idioma de la interfaz manda: el backend descarga los datos de tarkov.dev en ese idioma.
    socket.send(JSON.stringify({ type: "setLanguage", lang } satisfies ClientMsg));
  };
  socket.onmessage = (ev) => {
    if (typeof ev.data !== "string" || ev.data.length > 4_000_000) return;
    let raw: unknown;
    try {
      raw = JSON.parse(ev.data);
    } catch {
      return;
    }
    const parsed = serverMsgSchema.safeParse(raw);
    if (!parsed.success) return useApp.getState().notify("error", t.errors.badMessage);
    const msg = parsed.data;
    const s = useApp.getState();
    if (msg.type === "snapshot") {
      gotSnapshot = true;
      retry = 0;
      quickFailures = 0;
      s.setView(msg.view);
      s.setConnection("open");
    } else if (msg.type === "interfaces") {
      s.setInterfaces(msg.list);
    } else if (msg.type === "mapData") {
      s.setMarkers(msg.mapId, msg.markers);
      void loadIcons(msg.icons);
    } else if (msg.type === "logs") {
      s.setLogs(msg.lines);
    } else if (msg.type === "folderCheck") {
      s.setFolderCheck(msg.result);
    } else {
      s.notify(msg.level, msg.text);
    }
  };
  socket.onclose = () => {
    if (ws !== socket) return;
    // Si el servidor cierra antes de la primera instantánea varias veces, el token ya no vale.
    if (!gotSnapshot) quickFailures += 1;
    if (quickFailures >= 3) return useApp.getState().setConnection("expired");
    useApp.getState().setConnection("offline");
    schedule();
  };
}

export function reconnectNow(): void {
  retry = 0;
  connect();
}
