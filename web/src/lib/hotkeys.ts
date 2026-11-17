import { useEffect } from "react";

export interface Hotkey {
  key: string;
  ctrl?: boolean;
  run: () => void;
}

function typing(target: EventTarget | null): boolean {
  if (!(target instanceof HTMLElement)) return false;
  return target.isContentEditable || ["INPUT", "TEXTAREA", "SELECT"].includes(target.tagName);
}

/** Atajos globales. Los que no usan Ctrl se ignoran mientras se escribe o hay un diálogo abierto. */
export function useHotkeys(keys: Hotkey[]): void {
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      for (const k of keys) {
        const ctrl = e.ctrlKey || e.metaKey;
        if (e.key.toLowerCase() !== k.key || Boolean(k.ctrl) !== ctrl || e.altKey) continue;
        if (!k.ctrl && (typing(e.target) || document.querySelector("dialog[open]"))) continue;
        e.preventDefault();
        k.run();
        return;
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [keys]);
}
