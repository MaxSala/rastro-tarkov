import { CircleAlert, CloudDownload, RefreshCw } from "lucide-react";
import { useEffect, useState, type ReactNode } from "react";
import { t } from "../../locales";
import { useCatalog, type Catalog } from "../../lib/catalog";
import { cn } from "../../lib/cn";
import { ago } from "../../lib/format";
import { icon, iconSm } from "../../lib/icons";
import { send } from "../../lib/socket";
import { useApp } from "../../store/app";
import { Button, IconButton } from "../ui/button";
import { Skeleton } from "../ui/skeleton";
import { say } from "../../lib/msg";

export const refreshMarket = () => send({ type: "marketRefresh", fleaName: t.market.flea });

/** Estado de los datos: modo, antigüedad y botón de actualizar. Va en la cabecera de cada vista. */
export function DataStatus() {
  const m = useApp((s) => s.view?.market);
  const [, tick] = useState(0);
  useEffect(() => {
    const id = window.setInterval(() => tick((n) => n + 1), 30_000);
    return () => window.clearInterval(id);
  }, []);
  if (!m || m.state === "empty") return null;
  const loading = m.state === "loading";
  return (
    <div className="flex shrink-0 items-center gap-2 text-xs text-fg-muted">
      {m.mode && (m.mode === "pve" || m.mode === "regular") && (
        <span className="rounded-sm bg-active px-1.5 py-0.5 text-2xs text-fg-soft">
          {t.data.mode[m.mode]}
        </span>
      )}
      {m.fetchedAt && (
        <span title={new Date(m.fetchedAt * 1000).toLocaleString()}>{t.data.updated(ago(m.fetchedAt))}</span>
      )}
      <IconButton label={t.data.refresh} onClick={refreshMarket} disabled={loading}>
        <RefreshCw {...iconSm} className={cn(loading && "animate-spin motion-reduce:animate-none")} />
      </IconButton>
    </div>
  );
}

function Centered({ children }: { children: ReactNode }) {
  return (
    <div className="grid h-full place-items-center p-6">
      <div className="grid w-full max-w-md justify-items-start gap-4">{children}</div>
    </div>
  );
}

/** Carga el catálogo de disco cuando el backend lo tiene (o tras una descarga). `enabled` evita
 *  pedirlo en el mapa si no hace falta (capa de cerraduras apagada). */
export function useAutoCatalog(enabled = true) {
  const m = useApp((s) => s.view?.market);
  const state = useCatalog();
  const { load } = state;
  useEffect(() => {
    if (enabled && m?.fetchedAt && (m.state === "ready" || m.state === "loading")) void load(m.fetchedAt);
  }, [enabled, m?.fetchedAt, m?.state, load]);
  return { ...state, market: m };
}

/** Muestra el contenido solo cuando hay catálogo; si no, invita a descargarlo o explica el error. */
export function DataGate({ children }: { children: (c: Catalog) => ReactNode }) {
  const { catalog, loading, error, market: m } = useAutoCatalog();

  if (catalog) return <>{children(catalog)}</>;
  if (m?.state === "loading" || loading)
    return (
      <div className="grid h-full grid-rows-[auto_1fr]" aria-busy="true">
        <div className="flex items-center gap-3 border-b border-line px-4 py-3">
          <RefreshCw {...iconSm} className="animate-spin text-accent motion-reduce:animate-none" />
          <span className="text-sm">{t.data.loading}</span>
          <span className="text-xs text-fg-muted">{t.data.loadingHint}</span>
        </div>
        <div className="grid content-start gap-px p-4">
          {Array.from({ length: 12 }, (_, i) => (
            <Skeleton key={i} className="h-10" />
          ))}
        </div>
      </div>
    );
  const failed = m?.state === "error" || error;
  return (
    <Centered>
      {failed ? (
        <CircleAlert {...icon} className="text-danger" />
      ) : (
        <CloudDownload size={22} strokeWidth={1.5} className="text-fg-muted" />
      )}
      <div className="grid gap-1.5">
        <h2 className="text-lg font-medium">{failed ? t.data.failed : t.data.title}</h2>
        <p className="text-sm text-fg-soft">{failed ? (m?.error ? say(m.error) : error) : t.data.body}</p>
        <p className="text-xs text-fg-muted">{t.data.hosts}</p>
      </div>
      <Button variant="primary" onClick={refreshMarket}>
        {failed ? <RefreshCw {...icon} /> : <CloudDownload {...icon} />}
        {failed ? t.connection.retry : t.data.action}
      </Button>
    </Centered>
  );
}
