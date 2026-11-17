import { api } from "./api";
import { useApp } from "../store/app";

// Iconos de marcadores (PNG de tarkov.dev servidos por Rastro). Se piden con el token y se usan
// como URLs blob: (permitidas por la CSP). Se cargan una vez por sesión.
const urls = new Map<string, string>();
const pending = new Set<string>();

export function iconUrl(name: string | null | undefined): string | null {
  return (name && urls.get(name)) || null;
}

export async function loadIcons(names: string[]): Promise<void> {
  const todo = names.filter((n) => !urls.has(n) && !pending.has(n));
  if (todo.length === 0) return;
  todo.forEach((n) => pending.add(n));
  await Promise.all(
    todo.map(async (n) => {
      try {
        urls.set(n, await api.imageBlobUrl(`/api/icons/${encodeURIComponent(n)}`));
      } catch {
        /* sin icono: se usa la forma simple */
      } finally {
        pending.delete(n);
      }
    }),
  );
  useApp.getState().bumpIcons();
}
