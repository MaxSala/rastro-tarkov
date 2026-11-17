import type { ReactNode } from "react";

export function Kbd({ children }: { children: ReactNode }) {
  return (
    <kbd className="num inline-flex h-4.5 min-w-4.5 items-center justify-center rounded-sm border border-line-strong bg-raised px-1 text-2xs text-fg-muted">
      {children}
    </kbd>
  );
}
