import { Crosshair, Maximize, Minus, Plus } from "lucide-react";
import { t } from "../locales";
import { icon } from "../lib/icons";
import { useApp } from "../store/app";
import { IconButton } from "../components/ui/button";
import { fitWhole, zoomBy } from "../components/map/mapRef";

/** Controles del mapa propios, en lugar de los de Leaflet. */
export function MapToolbar() {
  const follow = useApp((s) => s.prefs.follow);
  const setPrefs = useApp((s) => s.setPrefs);
  return (
    <div
      role="toolbar"
      aria-label={t.a11y.mapToolbar}
      className="absolute right-3 bottom-3 z-[500] grid gap-px overflow-hidden rounded-md border border-line-strong bg-surface p-0.5"
    >
      <IconButton label={t.toolbar.zoomIn} shortcut="+" onClick={() => zoomBy(0.5)}>
        <Plus {...icon} />
      </IconButton>
      <IconButton label={t.toolbar.zoomOut} shortcut="−" onClick={() => zoomBy(-0.5)}>
        <Minus {...icon} />
      </IconButton>
      <div className="mx-1.5 h-px bg-line" />
      <IconButton label={t.toolbar.fit} shortcut="F" onClick={fitWhole}>
        <Maximize {...icon} />
      </IconButton>
      <IconButton
        label={t.toolbar.follow}
        shortcut="G"
        active={follow}
        onClick={() => setPrefs({ follow: !follow })}
      >
        <Crosshair {...icon} />
      </IconButton>
    </div>
  );
}
