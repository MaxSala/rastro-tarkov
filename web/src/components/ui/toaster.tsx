import { CircleAlert, Info, X } from "lucide-react";
import { t } from "../../locales";
import { useApp } from "../../store/app";
import { cn } from "../../lib/cn";

export function Toaster() {
  const toasts = useApp((s) => s.toasts);
  const dismiss = useApp((s) => s.dismiss);
  return (
    <div
      role="status"
      aria-live="polite"
      className="pointer-events-none fixed right-4 bottom-10 z-50 grid w-80 gap-2"
    >
      {toasts.map((toast) => (
        <div
          key={toast.id}
          className={cn(
            "pointer-events-auto flex items-start gap-2 rounded-sm border bg-surface px-3 py-2 text-sm",
            toast.level === "error" ? "border-danger/60" : "border-line-strong",
          )}
        >
          {toast.level === "error" ? (
            <CircleAlert size={16} strokeWidth={1.75} className="mt-0.5 shrink-0 text-danger" />
          ) : (
            <Info size={16} strokeWidth={1.75} className="mt-0.5 shrink-0 text-fg-muted" />
          )}
          <p className="flex-1">{toast.text}</p>
          <button
            type="button"
            aria-label={t.a11y.closeToast}
            onClick={() => dismiss(toast.id)}
            className="text-fg-muted hover:text-fg"
          >
            <X size={14} strokeWidth={1.75} />
          </button>
        </div>
      ))}
    </div>
  );
}
