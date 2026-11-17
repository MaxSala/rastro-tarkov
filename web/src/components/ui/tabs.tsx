import { cn } from "../../lib/cn";

/** Pestañas accesibles (tablist) con flechas izquierda/derecha. */
export function Tabs<T extends string>({
  value,
  onChange,
  tabs,
  label,
}: {
  value: T;
  onChange: (v: T) => void;
  tabs: { value: T; label: string }[];
  label: string;
}) {
  const move = (d: number) => {
    const i = tabs.findIndex((tab) => tab.value === value);
    const next = tabs[(i + d + tabs.length) % tabs.length];
    if (next) {
      onChange(next.value);
      document.getElementById(`tab-${next.value}`)?.focus();
    }
  };
  return (
    <div
      role="tablist"
      aria-label={label}
      className="flex gap-1 border-b border-line px-4"
      onKeyDown={(e) => {
        if (e.key === "ArrowRight") move(1);
        if (e.key === "ArrowLeft") move(-1);
      }}
    >
      {tabs.map((tab) => (
        <button
          key={tab.value}
          id={`tab-${tab.value}`}
          type="button"
          role="tab"
          aria-selected={tab.value === value}
          aria-controls={`panel-${tab.value}`}
          tabIndex={tab.value === value ? 0 : -1}
          onClick={() => onChange(tab.value)}
          className={cn(
            "-mb-px h-9 border-b-2 px-2 text-sm transition-colors",
            tab.value === value ? "border-accent text-fg" : "border-transparent text-fg-muted hover:text-fg",
          )}
        >
          {tab.label}
        </button>
      ))}
    </div>
  );
}
