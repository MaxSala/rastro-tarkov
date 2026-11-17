import { WifiOff } from "lucide-react";
import { t } from "../locales";
import { iconSm } from "../lib/icons";
import { reconnectNow } from "../lib/socket";
import { useApp } from "../store/app";
import { Button } from "../components/ui/button";

export function ConnectionBanner() {
  const connection = useApp((s) => s.connection);
  if (connection !== "offline") return null;
  return (
    <div
      role="alert"
      className="absolute top-3 left-1/2 z-[600] flex -translate-x-1/2 items-center gap-3 rounded-md border border-danger/60 bg-surface py-1.5 pr-1.5 pl-3 text-sm"
    >
      <WifiOff {...iconSm} className="text-danger" />
      {t.connection.offlineBody}
      <Button size="sm" onClick={reconnectNow}>
        {t.connection.retry}
      </Button>
    </div>
  );
}
