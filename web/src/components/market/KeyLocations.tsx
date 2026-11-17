import { MapPin } from "lucide-react";
import { useEffect, useState } from "react";
import { t } from "../../locales";
import { api } from "../../lib/api";
import { iconSm } from "../../lib/icons";
import { send } from "../../lib/socket";
import { useApp } from "../../store/app";

interface Loc {
  mapId: string;
  mapName: string;
  count: number;
}

function parse(raw: unknown): Loc[] {
  if (!Array.isArray(raw)) return [];
  return raw
    .filter(
      (x): x is Loc =>
        typeof x === "object" &&
        x !== null &&
        typeof (x as Loc).mapId === "string" &&
        typeof (x as Loc).mapName === "string" &&
        typeof (x as Loc).count === "number",
    )
    .slice(0, 32);
}

/** Para llaves: mapas instalados con cerraduras que abre. Clic → mapa con la llave resaltada. */
export function KeyLocations({ id }: { id: string }) {
  const [state, setState] = useState<{ id: string; list: Loc[] } | null>(null);
  const activeId = useApp((s) => s.view?.activeMap?.id);
  const showKeyOnMap = useApp((s) => s.showKeyOnMap);
  useEffect(() => {
    let live = true;
    api.json(`/api/key-locations/${id}`).then(
      (raw) => live && setState({ id, list: parse(raw) }),
      () => live && setState({ id, list: [] }),
    );
    return () => {
      live = false;
    };
  }, [id]);
  if (!state || state.id !== id) return null;
  return (
    <section className="grid gap-1">
      <h3 className="text-xs font-medium text-fg-muted">{t.keys.onMap}</h3>
      {state.list.length === 0 ? (
        <p className="text-xs text-fg-muted">{t.keys.onMapNone}</p>
      ) : (
        <ul className="grid gap-px overflow-hidden rounded-sm border border-line bg-line">
          {state.list.map((l) => (
            <li key={l.mapId}>
              <button
                type="button"
                onClick={() => {
                  if (l.mapId !== activeId) send({ type: "setActiveMap", mapId: l.mapId });
                  showKeyOnMap(id);
                }}
                className="flex h-9 w-full items-center gap-2 bg-surface px-3 text-left text-sm hover:bg-hover"
              >
                <MapPin {...iconSm} className="text-accent" />
                <span className="min-w-0 flex-1 truncate">{l.mapName}</span>
                <span className="text-2xs text-fg-muted">{t.keys.locks(l.count)}</span>
              </button>
            </li>
          ))}
        </ul>
      )}
    </section>
  );
}
