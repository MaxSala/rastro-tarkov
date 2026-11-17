// Iconos de objetos: se piden al backend (que los cachea en disco) con un máximo de 6 a la vez,
// y se usan como URLs blob: (permitidas por la CSP).
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
    const id = queue.shift();
    if (!id) break;
    active += 1;
    api
      .imageBlobUrl(`/api/item-icon/${id}`)
      .then(
        (u) => urls.set(id, u),
        () => urls.set(id, null),
      )
      .finally(() => {
        active -= 1;
        emit();
        pump();
      });
  }
}

function request(id: string) {
  if (urls.has(id) || queue.includes(id)) return;
  queue.push(id);
  pump();
}

/** El icono no se pudo obtener (sin conexión o sin icono): se muestra uno genérico. */
export const iconFailed = (id: string) => urls.has(id) && urls.get(id) === null;

/** URL del icono (o null mientras carga o si no hay). Las filas fuera de pantalla no lo piden. */
export function useItemIcon(id: string, visible = true): string | null {
  useSyncExternalStore(
    (cb) => {
      listeners.add(cb);
      return () => listeners.delete(cb);
    },
    () => version,
  );
  useEffect(() => {
    if (visible) request(id);
  }, [id, visible]);
  return urls.get(id) ?? null;
}
