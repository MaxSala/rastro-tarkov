import { cn } from "../../lib/cn";

export function Switch({
  checked,
  onChange,
  label,
}: {
  checked: boolean;
  onChange: (v: boolean) => void;
  label: string;
}) {
  return (
    <button
      type="button"
      role="switch"
      aria-checked={checked}
      aria-label={label}
      onClick={() => onChange(!checked)}
      className={cn(
        "relative inline-flex h-4 w-7 shrink-0 items-center rounded-full border transition-colors",
        checked ? "border-transparent bg-accent" : "border-line-strong bg-active",
      )}
    >
      <span
        className={cn(
          "absolute size-3 rounded-full transition-transform",
          checked ? "translate-x-3.5 bg-accent-fg" : "translate-x-0.5 bg-fg-muted",
        )}
      />
    </button>
  );
}
