import { X } from "lucide-react";
import { t } from "../../locales";
import { useEffect, useId, useRef, type ReactNode } from "react";
import { cn } from "../../lib/cn";

interface DialogProps {
  open: boolean;
  onClose: () => void;
  title: string;
  description?: string;
  children: ReactNode;
  footer?: ReactNode;
  className?: string;
}

/**
 * Diálogo modal sobre <dialog> nativo: trampa de foco, Esc y capa inerte del navegador,
 * sin inyectar <style> en tiempo de ejecución (compatible con la CSP estricta).
 */
export function Dialog({ open, onClose, title, description, children, footer, className }: DialogProps) {
  const ref = useRef<HTMLDialogElement>(null);
  const titleId = useId();
  const descId = useId();
  useEffect(() => {
    const d = ref.current;
    if (!d) return;
    if (open && !d.open) d.showModal();
    if (!open && d.open) d.close();
  }, [open]);
  return (
    <dialog
      ref={ref}
      aria-labelledby={titleId}
      aria-describedby={description ? descId : undefined}
      onClose={onClose}
      onCancel={(e) => {
        e.preventDefault();
        onClose();
      }}
      onMouseDown={(e) => {
        if (e.target === ref.current) onClose();
      }}
      className={cn(
        "m-auto w-[min(480px,calc(100vw-32px))] rounded-md border border-line-strong bg-surface p-0 text-fg",
        className,
      )}
    >
      {open && (
        <div className="grid">
          <header className="flex items-start justify-between gap-4 border-b border-line px-4 py-3">
            <div className="grid gap-0.5">
              <h2 id={titleId} className="text-base font-medium">
                {title}
              </h2>
              {description && (
                <p id={descId} className="text-xs text-fg-muted">
                  {description}
                </p>
              )}
            </div>
            <button
              type="button"
              onClick={onClose}
              aria-label={t.a11y.close}
              className="-mr-1 inline-flex size-7 items-center justify-center rounded-sm text-fg-muted hover:bg-hover hover:text-fg"
            >
              <X size={16} strokeWidth={1.75} />
            </button>
          </header>
          <div className="grid gap-4 px-4 py-4">{children}</div>
          {footer && (
            <footer className="flex justify-end gap-2 border-t border-line px-4 py-3">{footer}</footer>
          )}
        </div>
      )}
    </dialog>
  );
}
