import { cn } from "../../lib/cn";
import { glyphOf, SHAPES } from "../../lib/questGlyphs";

/** The same badge the map uses for an objective, for lists next to it. */
export function QuestGlyph({ kind, className }: { kind: string; className?: string }) {
  return (
    <span
      aria-hidden
      className={cn(
        "grid size-5 shrink-0 place-items-center rounded-full bg-accent text-accent-fg",
        className,
      )}
    >
      <svg
        viewBox="0 0 24 24"
        className="size-3"
        fill="none"
        stroke="currentColor"
        strokeWidth={2.5}
        strokeLinecap="round"
        strokeLinejoin="round"
      >
        {SHAPES[glyphOf(kind)].map((s, i) =>
          s[0] === "path" ? <path key={i} d={s[1]} /> : <circle key={i} cx={s[1]} cy={s[2]} r={s[3]} />,
        )}
      </svg>
    </span>
  );
}
