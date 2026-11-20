// Pictures from tarkov.dev (item icons, trader portraits, quest pictures): asked of the backend,
// which caches them on disk, at most 6 at a time, and used as blob: URLs (allowed by the CSP).
import { useEffect, useSyncExternalStore } from "react";
import { api } from "./api";

const MAX_CONCURRENT = 6;
const urls = new Map<string, string | null>();
const queue: string[] = [];
const listeners = new Set<() => void>();
let active = 0;
let version = 0;

function emit() {
  version += 1;
  listeners.forEach((l) => l());
}

function pump() {
  while (active < MAX_CONCURRENT && queue.length > 0) {
    const path = queue.shift();
    if (!path) break;
    active += 1;
    api
      .optionalBlobUrl(path)
      .then(
        (u) => urls.set(path, u),
        () => urls.set(path, null),
      )
      .finally(() => {
        active -= 1;
        emit();
        pump();
      });
  }
}

function request(path: string) {
  if (urls.has(path) || queue.includes(path)) return;
  queue.push(path);
  pump();
}

const iconPath = (id: string) => `/api/item-icon/${id}`;
const picturePath = (id: string) => `/api/tarkov-image/${id}`;

function useRemote(path: string | null, visible: boolean): string | null {
  useSyncExternalStore(
    (cb) => {
      listeners.add(cb);
      return () => listeners.delete(cb);
    },
    () => version,
  );
  useEffect(() => {
    if (path && visible) request(path);
  }, [path, visible]);
  return path ? (urls.get(path) ?? null) : null;
}

/** There's no picture (offline, or tarkov.dev has none): a generic one is shown. */
export const iconFailed = (id: string) => urls.get(iconPath(id)) === null && urls.has(iconPath(id));
export const pictureFailed = (id: string) => urls.get(picturePath(id)) === null && urls.has(picturePath(id));

/** Item icon URL (null while loading or if there's none). Rows off screen don't ask for it. */
export const useItemIcon = (id: string, visible = true) => useRemote(iconPath(id), visible);

/** Trader portrait or quest picture by its 24-hex id; `null` id asks for nothing. */
export const useTarkovPicture = (id: string | null, visible = true) =>
  useRemote(id ? picturePath(id) : null, visible);
