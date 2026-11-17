import { useEffect, useState } from "react";
import { t } from "../locales";
import { useApp } from "../store/app";
import { LogoMark } from "../components/brand/Logo";

const MIN_MS = 1100;
const LEAVE_MS = 240;

/**
 * Pantalla de carga al abrir: se queda al menos lo que dura la animación del logotipo y hasta que
 * llega el primer estado de Rastro; luego se desvanece sobre la app ya montada (sin parpadeo).
 */
export function Splash() {
  const ready = useApp((s) => s.view !== null);
  const connection = useApp((s) => s.connection);
  const [minDone, setMinDone] = useState(() => window.matchMedia("(prefers-reduced-motion: reduce)").matches);
  const [gone, setGone] = useState(false);
  const leaving = ready && minDone;

  useEffect(() => {
    const id = window.setTimeout(() => setMinDone(true), MIN_MS);
    return () => window.clearTimeout(id);
  }, []);
  useEffect(() => {
    if (!leaving) return;
    const id = window.setTimeout(() => setGone(true), LEAVE_MS);
    return () => window.clearTimeout(id);
  }, [leaving]);

  if (gone) return null;
  const status = ready
    ? t.splash.ready
    : connection === "connecting"
      ? t.splash.connecting
      : t.splash.starting;
  return (
    <div
      role="status"
      aria-live="polite"
      aria-label={status}
      data-leaving={leaving}
      className="rt-splash fixed inset-0 z-[2000] grid place-items-center bg-app"
    >
      <div className="grid justify-items-center gap-5">
        <LogoMark size={72} animated />
        <div className="grid justify-items-center gap-3">
          <span className="rt-splash-word text-2xl font-semibold tracking-[-0.02em]">Rastro</span>
          <span className="rt-splash-bar relative block h-0.5 w-24 overflow-hidden rounded-full bg-active" />
          <span className="rt-splash-status num text-2xs text-fg-muted">{status}</span>
        </div>
      </div>
      <span className="num absolute bottom-6 text-2xs text-fg-muted">v{__APP_VERSION__}</span>
    </div>
  );
}
