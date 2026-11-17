import type { InputHTMLAttributes, TextareaHTMLAttributes } from "react";
import { cn } from "../../lib/cn";

const field =
  "w-full rounded-sm border border-line-strong bg-app px-2.5 text-sm text-fg placeholder:text-placeholder " +
  "focus-visible:border-accent focus-visible:outline-none disabled:opacity-50";

export function Input({ className, ...rest }: InputHTMLAttributes<HTMLInputElement>) {
  return <input className={cn(field, "h-8", className)} {...rest} />;
}

export function Textarea({ className, ...rest }: TextareaHTMLAttributes<HTMLTextAreaElement>) {
  return <textarea className={cn(field, "min-h-20 resize-none py-2", className)} {...rest} />;
}
