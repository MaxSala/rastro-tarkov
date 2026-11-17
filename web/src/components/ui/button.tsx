import type { ButtonHTMLAttributes, ReactNode } from "react";
import { cn } from "../../lib/cn";
import { Kbd } from "./kbd";
import { Tooltip } from "./tooltip";

type Variant = "primary" | "secondary" | "ghost" | "danger";
type Size = "sm" | "md";

const base =
  "inline-flex items-center justify-center gap-1.5 rounded-sm border font-medium whitespace-nowrap select-none " +
  "transition-colors disabled:pointer-events-none disabled:opacity-50";
const variants: Record<Variant, string> = {
  primary: "bg-accent text-accent-fg border-transparent hover:brightness-110",
  secondary: "bg-raised text-fg border-line-strong hover:bg-hover",
  ghost: "bg-transparent text-fg-soft border-transparent hover:bg-hover hover:text-fg",
  danger: "bg-transparent text-danger border-line-strong hover:bg-hover",
};
const sizes: Record<Size, string> = {
  sm: "h-7 px-2 text-xs",
  md: "h-8 px-3 text-sm",
};

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: Variant;
  size?: Size;
}

export function Button({
  variant = "secondary",
  size = "md",
  className,
  type = "button",
  ...rest
}: ButtonProps) {
  return <button type={type} className={cn(base, variants[variant], sizes[size], className)} {...rest} />;
}

interface IconButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  label: string;
  shortcut?: string;
  active?: boolean;
  children: ReactNode;
}

/** Botón de icono con nombre accesible y tooltip que muestra el atajo. */
export function IconButton({ label, shortcut, active, className, children, ...rest }: IconButtonProps) {
  return (
    <Tooltip
      content={
        <span className="flex items-center gap-2">
          {label}
          {shortcut && <Kbd>{shortcut}</Kbd>}
        </span>
      }
    >
      <button
        type="button"
        aria-label={label}
        aria-pressed={active}
        className={cn(
          "inline-flex size-8 items-center justify-center rounded-sm text-fg-soft transition-colors",
          "hover:bg-hover hover:text-fg",
          active && "bg-active text-fg",
          className,
        )}
        {...rest}
      >
        {children}
      </button>
    </Tooltip>
  );
}
