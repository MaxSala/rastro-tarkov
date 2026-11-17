import { ArrowDown, ArrowUp } from "lucide-react";
import { cn } from "../../lib/cn";

export type SortDir = "asc" | "desc";

/** Cabecera de columna ordenable (botón con aria-sort en la celda). */
export function SortHeader<K extends string>({
  label,
  k,
  sort,
  onSort,
  align = "right",
  className,
}: {
  label: string;
  k: K;
  sort: { key: K; dir: SortDir };
  onSort: (k: K) => void;
  align?: "left" | "right";
  className?: string;
}) {
  const active = sort.key === k;
  const Arrow = sort.dir === "asc" ? ArrowUp : ArrowDown;
  return (
    <th
      scope="col"
      aria-sort={active ? (sort.dir === "asc" ? "ascending" : "descending") : "none"}
      className={cn("sticky top-0 z-10 border-b border-line bg-surface p-0 font-normal", className)}
    >
      <button
        type="button"
        onClick={() => onSort(k)}
        className={cn(
          "flex h-9 w-full items-center gap-1 px-3 text-xs whitespace-nowrap transition-colors hover:text-fg",
          align === "right" ? "justify-end" : "justify-start",
          active ? "text-fg" : "text-fg-muted",
        )}
      >
        {label}
        <Arrow size={12} strokeWidth={2} className={cn("shrink-0", !active && "opacity-0")} />
      </button>
    </th>
  );
}
