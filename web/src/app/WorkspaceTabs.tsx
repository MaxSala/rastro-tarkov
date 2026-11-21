import { Coins, ListChecks, Map as MapIcon, Target, Warehouse } from "lucide-react";
import { t } from "../locales";
import { cn } from "../lib/cn";
import { iconSm } from "../lib/icons";
import { useApp, type Workspace } from "../store/app";
import { Kbd } from "../components/ui/kbd";
import { Tooltip } from "../components/ui/tooltip";

const items: { id: Workspace; Icon: typeof MapIcon; key: string }[] = [
  { id: "map", Icon: MapIcon, key: "1" },
  { id: "market", Icon: Coins, key: "2" },
  { id: "ammo", Icon: Target, key: "3" },
  { id: "quests", Icon: ListChecks, key: "4" },
  { id: "hideout", Icon: Warehouse, key: "5" },
];

/** Secciones de la app. Ctrl+1…5 desde cualquier sitio. */
export function WorkspaceTabs() {
  const current = useApp((s) => s.workspace);
  const setWorkspace = useApp((s) => s.setWorkspace);
  return (
    <nav
      aria-label={t.workspace.label}
      className="flex h-8 items-center gap-0.5 rounded-md border border-line bg-app p-0.5"
    >
      {items.map(({ id, Icon, key }) => (
        <Tooltip
          key={id}
          content={
            <span className="flex items-center gap-2">
              {t.workspace[id]}
              <Kbd>Ctrl {key}</Kbd>
            </span>
          }
        >
          <button
            type="button"
            aria-current={current === id ? "page" : undefined}
            onClick={() => setWorkspace(id)}
            className={cn(
              "flex h-full items-center gap-1.5 rounded-[5px] px-2.5 text-xs font-medium transition-colors",
              current === id ? "bg-active text-fg" : "text-fg-muted hover:text-fg",
            )}
          >
            <Icon {...iconSm} className={current === id ? "text-accent" : undefined} />
            {t.workspace[id]}
          </button>
        </Tooltip>
      ))}
    </nav>
  );
}
