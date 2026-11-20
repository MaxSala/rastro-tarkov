// Trader portraits and quest pictures from tarkov.dev. They load only once on screen, and
// whatever can't be shown (offline, or no picture) falls back to something quiet.
import { useEffect, useRef, useState } from "react";
import { cn } from "../../lib/cn";
import { pictureFailed, useTarkovPicture } from "../../lib/itemIcons";
import type { QStatus } from "../../lib/questLogic";
import { StatusDot } from "./bits";

function useOnScreen<T extends Element>() {
  const ref = useRef<T>(null);
  const [visible, setVisible] = useState(false);
  useEffect(() => {
    const el = ref.current;
    if (!el || visible) return;
    const io = new IntersectionObserver(([e]) => e?.isIntersecting && setVisible(true), {
      rootMargin: "200px",
    });
    io.observe(el);
    return () => io.disconnect();
  }, [visible]);
  return [ref, visible] as const;
}

/** A trader's portrait, or their initial while it loads or if there is none. */
export function TraderAvatar({
  image,
  name,
  size = 32,
  status,
  className,
}: {
  image: string | null;
  name: string;
  size?: number;
  /** Quest state shown as a small badge on the corner. */
  status?: QStatus;
  className?: string;
}) {
  const [ref, visible] = useOnScreen<HTMLSpanElement>();
  const url = useTarkovPicture(image, visible);
  return (
    <span ref={ref} style={{ width: size, height: size }} className={cn("relative shrink-0", className)}>
      <span
        className={cn(
          "grid size-full place-items-center overflow-hidden rounded-md border border-line bg-app text-fg-muted",
          status === "completed" && "opacity-60",
        )}
      >
        {url ? (
          <img src={url} alt="" draggable={false} className="size-full object-cover" />
        ) : (
          <span aria-hidden className="text-xs font-medium uppercase">
            {name.charAt(0)}
          </span>
        )}
      </span>
      {status && (
        <span className="absolute -right-1 -bottom-1 grid place-items-center rounded-full bg-surface p-px">
          <StatusDot status={status} className="size-3.5" />
        </span>
      )}
    </span>
  );
}

/** The quest's picture as tarkov.dev shows it; nothing at all when it has none. */
export function QuestPicture({ id, has }: { id: string; has: boolean }) {
  const [ref, visible] = useOnScreen<HTMLDivElement>();
  const url = useTarkovPicture(has ? id : null, visible);
  if (!has || pictureFailed(id)) return null;
  return (
    <div
      ref={ref}
      className="relative aspect-[2/1] overflow-hidden rounded-md border border-line bg-app"
      aria-hidden
    >
      {url && <img src={url} alt="" draggable={false} className="size-full object-cover" />}
      <div className="pointer-events-none absolute inset-0 rounded-md ring-1 ring-black/20 ring-inset" />
    </div>
  );
}
