import { Check, Flag, MapPin, Navigation, Pencil, Trash2, X } from "lucide-react";
import { useEffect, useLayoutEffect, useRef, useState, type ReactNode } from "react";
import { t } from "../locales";
import { cn } from "../lib/cn";
import { iconSm } from "../lib/icons";
import { send } from "../lib/socket";
import { useApp } from "../store/app";

interface Item {
  id: string;
  label: string;
  icon: ReactNode;
  danger?: boolean;
  keepOpen?: boolean;
  run: () => void;
}

/** Menú contextual del mapa (clic derecho). Navegable con flechas, Intro y Esc. */
export function ContextMenu() {
  const menu = useApp((s) => s.menu);
  const view = useApp((s) => s.view);
  const setMenu = useApp((s) => s.setMenu);
  const editPoint = useApp((s) => s.editPoint);
  const ref = useRef<HTMLDivElement>(null);
  const [confirmDelete, setConfirmDelete] = useState(false);
  const [pos, setPos] = useState({ left: 0, top: 0 });

  useEffect(() => {
    if (!menu) return;
    const close = (e: Event) => {
      if (e instanceof PointerEvent && ref.current?.contains(e.target as Node)) return;
      setMenu(null);
    };
    window.addEventListener("pointerdown", close, true);
    window.addEventListener("resize", close);
    window.addEventListener("blur", close);
    return () => {
      window.removeEventListener("pointerdown", close, true);
      window.removeEventListener("resize", close);
      window.removeEventListener("blur", close);
    };
  }, [menu, setMenu]);

  // Mantener el menú dentro de la ventana y enfocar la primera opción.
  useLayoutEffect(() => {
    const el = ref.current;
    if (!menu || !el) return;
    const r = el.getBoundingClientRect();
    setPos({
      left: Math.min(menu.x, window.innerWidth - r.width - 8),
      top: Math.min(menu.y, window.innerHeight - r.height - 8),
    });
    setConfirmDelete(false);
    el.querySelector<HTMLButtonElement>("[role=menuitem]")?.focus();
  }, [menu]);

  if (!menu || !view) return null;
  const target = menu.target;
  const selected = view.selectedExtract;
  const guide = (id: string): Item =>
    selected === id
      ? {
          id: "unguide",
          label: t.points.stopGuide,
          icon: <X {...iconSm} />,
          run: () => send({ type: "selectExtract", extractId: null }),
        }
      : {
          id: "guide",
          label: t.points.guide,
          icon: <Navigation {...iconSm} />,
          run: () => send({ type: "selectExtract", extractId: id }),
        };
  const items: Item[] = [];
  if (target.kind === "map") {
    items.push({
      id: "add",
      label: t.points.add,
      icon: <MapPin {...iconSm} />,
      run: () => editPoint({ mode: "new", px: menu.px, py: menu.py }),
    });
    if (view.party.status === "hosting" || view.party.status === "connected") {
      items.push({
        id: "ping",
        label: t.points.ping,
        icon: <Flag {...iconSm} />,
        run: () => send({ type: "mapPing", px: menu.px, py: menu.py }),
      });
    }
  } else if (target.kind === "point") {
    const p = view.points.find((x) => x.id === target.id);
    if (p) {
      items.push(guide(p.id), {
        id: "edit",
        label: t.points.edit,
        icon: <Pencil {...iconSm} />,
        run: () => editPoint({ mode: "edit", id: p.id }),
      });
      items.push({
        id: "delete",
        label: confirmDelete ? t.points.deleteConfirm(p.name) : t.points.delete,
        icon: <Trash2 {...iconSm} />,
        danger: true,
        keepOpen: !confirmDelete,
        run: () => (confirmDelete ? send({ type: "deletePoint", id: p.id }) : setConfirmDelete(true)),
      });
    }
  } else {
    const e = view.extracts.find((x) => x.id === target.id);
    if (e) {
      items.push(guide(e.id), {
        id: "avail",
        label: e.available ? t.points.unavailable : t.points.available,
        icon: <Check {...iconSm} />,
        run: () => send({ type: "toggleExtract", extractId: e.id }),
      });
    }
  }

  const onKey = (e: React.KeyboardEvent) => {
    const buttons = [...(ref.current?.querySelectorAll<HTMLButtonElement>("[role=menuitem]") ?? [])];
    const i = buttons.indexOf(document.activeElement as HTMLButtonElement);
    if (e.key === "ArrowDown") buttons[(i + 1) % buttons.length]?.focus();
    else if (e.key === "ArrowUp") buttons[(i - 1 + buttons.length) % buttons.length]?.focus();
    else if (e.key === "Escape") setMenu(null);
    else return;
    e.preventDefault();
  };

  return (
    <div
      ref={ref}
      role="menu"
      aria-label={t.a11y.actions}
      onKeyDown={onKey}
      onContextMenu={(e) => e.preventDefault()}
      style={{ left: pos.left, top: pos.top }}
      className="fixed z-[1000] grid min-w-52 max-w-72 gap-px rounded-md border border-line-strong bg-surface p-1"
    >
      {items.map((it) => (
        <button
          key={it.id}
          type="button"
          role="menuitem"
          onClick={() => {
            it.run();
            if (!it.keepOpen) setMenu(null);
          }}
          className={cn(
            "flex h-8 items-center gap-2 rounded-sm px-2 text-left text-sm outline-none",
            "hover:bg-hover focus-visible:bg-active",
            it.danger ? "text-danger" : "text-fg",
          )}
        >
          <span className={it.danger ? "" : "text-fg-muted"}>{it.icon}</span>
          <span className="truncate">{it.label}</span>
        </button>
      ))}
    </div>
  );
}
