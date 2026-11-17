import { cn } from "../../lib/cn";

/**
 * Logotipo: la flecha del jugador (la misma del mapa) girada 45° y el rastro que deja.
 * Fuente única de la geometría: assets/brand/rastro.svg (icono del .exe y favicon).
 */
export function LogoMark({
  size = 20,
  animated = false,
  className,
}: {
  size?: number;
  animated?: boolean;
  className?: string;
}) {
  return (
    <svg
      viewBox="0 0 64 64"
      width={size}
      height={size}
      aria-hidden="true"
      className={cn("shrink-0", animated && "rt-logo-anim", className)}
    >
      <rect
        className="rt-logo-tile"
        x="0.5"
        y="0.5"
        width="63"
        height="63"
        rx="14"
        fill="#141414"
        stroke="#2a2a2a"
      />
      <circle className="rt-logo-dot2" cx="17.26" cy="46.74" r="2.6" fill="#f0a43a" opacity="0.32" />
      <circle className="rt-logo-dot1" cx="23.27" cy="40.73" r="3.3" fill="#f0a43a" opacity="0.62" />
      <path
        className="rt-logo-arrow"
        d="M50.25 13.75 38.19 43.34 34.36 29.64 20.66 25.81Z"
        fill="#f0a43a"
        stroke="#f0a43a"
        strokeWidth="1.4"
        strokeLinejoin="round"
      />
    </svg>
  );
}

export function Brand() {
  return (
    <span className="flex items-center gap-2 pr-1">
      <LogoMark size={22} />
      <span className="text-[15px] font-semibold tracking-[-0.01em]">Rastro</span>
    </span>
  );
}
