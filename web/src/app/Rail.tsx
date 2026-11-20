import { Layers, ListChecks, LogOut, MapPin, Users } from "lucide-react";
import { t } from "../locales";
import { icon } from "../lib/icons";
import { useApp, type PanelId } from "../store/app";
import { IconButton } from "../components/ui/button";

const items: { id: PanelId; label: string; key: string; Icon: typeof Layers }[] = [
  { id: "extracts", label: t.panels.extracts, key: "1", Icon: LogOut },
  { id: "points", label: t.panels.points, key: "2", Icon: MapPin },
  { id: "party", label: t.panels.party, key: "3", Icon: Users },
  { id: "maps", label: t.panels.maps, key: "4", Icon: Layers },
  { id: "quests", label: t.panels.quests, key: "5", Icon: ListChecks },
];

export function Rail() {
  const panel = useApp((s) => s.panel);
  const toggle = useApp((s) => s.togglePanel);
  const peers = useApp((s) => s.view?.party.peers.length ?? 0);
  // Open objectives of your active quests on this map (each counted once).
  const quests = useApp(
    (s) => new Set(s.view?.questMarkers.filter((m) => !m.focus).map((m) => m.objective)).size,
  );
  return (
    <nav
      aria-label={t.a11y.panelsNav}
      className="flex w-11 flex-col items-center gap-1 border-r border-line bg-surface py-2"
    >
      {items.map(({ id, label, key, Icon }) => (
        <div key={id} className="relative">
          <IconButton label={label} shortcut={key} active={panel === id} onClick={() => toggle(id)}>
            <Icon {...icon} />
          </IconButton>
          {((id === "party" && peers > 0) || (id === "quests" && quests > 0)) && (
            <span className="num pointer-events-none absolute -top-0.5 -right-0.5 rounded-sm bg-accent px-1 text-2xs text-accent-fg">
              {id === "party" ? peers : quests}
            </span>
          )}
        </div>
      ))}
    </nav>
  );
}
