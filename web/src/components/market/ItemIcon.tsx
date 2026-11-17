import { useEffect, useRef, useState } from "react";
import { cn } from "../../lib/cn";
import { Package } from "lucide-react";
import { iconFailed, useItemIcon } from "../../lib/itemIcons";

/** Icono de objeto con carga perezosa: solo se pide cuando la fila entra en pantalla. */
export function ItemIcon({ id, size = 32, className }: { id: string; size?: number; className?: string }) {
  const ref = useRef<HTMLSpanElement>(null);
  const [visible, setVisible] = useState(false);
  useEffect(() => {
    const el = ref.current;
    if (!el) return;
    const io = new IntersectionObserver(([e]) => e?.isIntersecting && setVisible(true), {
      rootMargin: "200px",
    });
    io.observe(el);
    return () => io.disconnect();
  }, []);
  const url = useItemIcon(id, visible);
  return (
    <span
      ref={ref}
      style={{ width: size, height: size }}
      className={cn(
        "grid shrink-0 place-items-center overflow-hidden rounded-sm border border-line bg-app",
        className,
      )}
    >
      {url ? (
        <img src={url} alt="" draggable={false} className="size-full object-contain" />
      ) : iconFailed(id) ? (
        <Package size={Math.round(size * 0.45)} strokeWidth={1.5} className="text-fg-muted/60" aria-hidden />
      ) : null}
    </span>
  );
}
