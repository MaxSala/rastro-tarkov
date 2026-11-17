import { t } from "../locales";
import { userMsgSchema, say } from "./msg";
import { previewSchema, type Preview } from "./protocol";
import { readToken } from "./token";

const TOKEN_HEADER = "x-rastro-token";

async function errorText(res: Response): Promise<string> {
  try {
    const body: unknown = await res.json();
    const parsed = userMsgSchema.safeParse(
      body && typeof body === "object" && "error" in body ? { ...body, text: body.error } : null,
    );
    if (parsed.success) return say(parsed.data).slice(0, 300);
  } catch {
    /* respuesta sin JSON */
  }
  return t.errors.generic;
}

async function request(path: string, init: RequestInit = {}): Promise<Response> {
  const token = readToken();
  if (!token) throw new Error(t.connection.noTokenBody);
  const headers = new Headers(init.headers);
  headers.set(TOKEN_HEADER, token);
  let res: Response;
  try {
    res = await fetch(path, { ...init, headers, credentials: "omit", cache: "no-store" });
  } catch {
    throw new Error(t.errors.network);
  }
  if (!res.ok) throw new Error(await errorText(res));
  return res;
}

function postJson(path: string, body: unknown): Promise<Response> {
  return request(path, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(body),
  });
}

async function preview(res: Response): Promise<Preview> {
  const parsed = previewSchema.safeParse(await res.json());
  if (!parsed.success) throw new Error(t.errors.badMessage);
  return parsed.data;
}

export const api = {
  async uploadZip(file: File, sha: string): Promise<Preview> {
    const headers: Record<string, string> = { "content-type": "application/zip" };
    if (sha.trim()) headers["x-expected-sha256"] = sha.trim();
    return preview(await request("/api/import/upload", { method: "POST", headers, body: file }));
  },
  async demo(): Promise<Preview> {
    return preview(await postJson("/api/import/demo", {}));
  },
  async inspectUrl(url: string): Promise<string> {
    const body: unknown = await (await postJson("/api/import/inspect-url", { url })).json();
    if (body && typeof body === "object" && "host" in body && typeof body.host === "string") return body.host;
    throw new Error(t.errors.badMessage);
  },
  async fromUrl(url: string, sha: string): Promise<Preview> {
    return preview(await postJson("/api/import/url", { url, sha256: sha.trim() || null }));
  },
  async commit(stagingId: string, overwrite: boolean): Promise<void> {
    await postJson("/api/import/commit", { stagingId, overwrite, acceptLicense: true });
  },
  async cancel(stagingId: string): Promise<void> {
    await postJson("/api/import/cancel", { stagingId });
  },
  async json(path: string): Promise<unknown> {
    return (await request(path)).json();
  },
  /** La imagen se pide con la cabecera del token y se muestra como blob: nunca como SVG inline. */
  async text(path: string): Promise<string> {
    return (await request(path)).text();
  },
  /** Like imageBlobUrl, but 204 (nothing to show) gives null instead of an empty image. */
  async optionalBlobUrl(path: string): Promise<string | null> {
    const res = await request(path);
    if (res.status === 204) return null;
    return URL.createObjectURL(await res.blob());
  },
  async imageBlobUrl(path: string): Promise<string> {
    const blob = await (await request(path)).blob();
    return URL.createObjectURL(blob);
  },
};
