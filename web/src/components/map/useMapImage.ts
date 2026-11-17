import { useEffect, useState } from "react";
import { api } from "../../lib/api";

export type ImageState =
  | { status: "idle" }
  | { status: "loading" }
  | { status: "ready"; url: string }
  | { status: "error"; message: string };

/** Descarga la imagen del mapa con el token y la expone como blob: (se libera al cambiar). */
export function useMapImage(path: string | null, nonce: number): ImageState {
  const key = `${path}#${nonce}`;
  const [result, setResult] = useState<{ key: string; state: ImageState } | null>(null);
  useEffect(() => {
    if (!path) return;
    let url: string | null = null;
    let cancelled = false;
    api
      .imageBlobUrl(path)
      .then((u) => {
        url = u;
        if (cancelled) URL.revokeObjectURL(u);
        else setResult({ key, state: { status: "ready", url: u } });
      })
      .catch((e: unknown) => {
        const message = e instanceof Error ? e.message : String(e);
        if (!cancelled) setResult({ key, state: { status: "error", message } });
      });
    return () => {
      cancelled = true;
      if (url) URL.revokeObjectURL(url);
    };
  }, [path, key]);
  if (!path) return { status: "idle" };
  return result?.key === key ? result.state : { status: "loading" };
}
