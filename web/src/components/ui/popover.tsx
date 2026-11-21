import { useEffect, useId, useRef, useState, type ReactNode } from "react";

/**
 * A button that opens a small card under it, aligned to its right edge (it lives on the right of
 * the top bar). Closes on Escape, on a click outside and on a
 * second click; the card is announced as a dialog tied to its button.
 */
export function Popover({
  label,
  button,
  buttonClass,
  children,
}: {
  label: string;
  button: ReactNode;
  buttonClass: string;
  children: ReactNode;
}) {
  const [open, setOpen] = useState(false);
  const root = useRef<HTMLDivElement>(null);
  const id = useId();
  useEffect(() => {
    if (!open) return;
    const down = (e: PointerEvent) => {
      if (!root.current?.contains(e.target as Node)) setOpen(false);
    };
    const key = (e: KeyboardEvent) => {
      if (e.key === "Escape") setOpen(false);
    };
    document.addEventListener("pointerdown", down);
    document.addEventListener("keydown", key);
    return () => {
      document.removeEventListener("pointerdown", down);
      document.removeEventListener("keydown", key);
    };
  }, [open]);
  return (
    <div ref={root} className="relative min-w-0">
      <button
        type="button"
        title={label}
        aria-expanded={open}
        aria-controls={open ? id : undefined}
        onClick={() => setOpen((o) => !o)}
        className={buttonClass}
      >
        {button}
      </button>
      {open && (
        <div
          id={id}
          role="dialog"
          aria-label={label}
          className="absolute top-full right-0 z-[1000] mt-1.5 w-80 max-w-[calc(100vw-2rem)] rounded-md border border-line-strong bg-surface p-3 text-xs shadow-lg"
        >
          {children}
        </div>
      )}
    </div>
  );
}
