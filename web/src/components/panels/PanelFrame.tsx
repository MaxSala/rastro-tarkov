import { PanelLeftClose } from "lucide-react";
import type { ReactNode } from "react";
import { t } from "../../locales";
import { icon } from "../../lib/icons";
import { useApp } from "../../store/app";
import { IconButton } from "../ui/button";

export function PanelFrame({
  title,
  actions,
  children,
}: {
  title: string;
  actions?: ReactNode;
  children: ReactNode;
}) {
  const panel = useApp((s) => s.panel);
  const toggle = useApp((s) => s.togglePanel);
  return (
    <aside aria-label={title} className="flex w-[300px] min-w-0 flex-col border-r border-line bg-surface">
      <header className="flex h-10 shrink-0 items-center gap-1 border-b border-line pr-1 pl-3">
        <h2 className="min-w-0 flex-1 truncate text-sm font-medium">{title}</h2>
        {actions}
        <IconButton label={t.panels.collapse} onClick={() => panel && toggle(panel)}>
          <PanelLeftClose {...icon} />
        </IconButton>
      </header>
      <div className="min-h-0 flex-1 overflow-x-hidden overflow-y-auto">{children}</div>
    </aside>
  );
}

export function PanelNote({ children }: { children: ReactNode }) {
  return <p className="px-3 py-6 text-center text-sm text-fg-muted">{children}</p>;
}
