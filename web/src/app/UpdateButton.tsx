import { ArrowUpCircle } from "lucide-react";
import { t } from "../locales";
import { iconSm } from "../lib/icons";
import { useApp } from "../store/app";
import { Tooltip } from "../components/ui/tooltip";

/** Shown in the top bar only when a newer release exists. Opens its page in the browser. */
export function UpdateButton() {
  const u = useApp((s) => s.view?.update);
  if (u?.state !== "available" || !u.latest || !u.url) return null;
  return (
    <Tooltip content={t.update.tooltip}>
      <a
        href={u.url}
        target="_blank"
        rel="noreferrer"
        className="flex h-7 items-center gap-1.5 rounded-sm border border-accent/50 bg-accent-soft px-2 text-xs font-medium text-fg hover:bg-hover"
      >
        <ArrowUpCircle {...iconSm} className="text-accent" />
        {t.update.available(u.latest)}
      </a>
    </Tooltip>
  );
}
