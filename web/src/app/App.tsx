import { t } from "../locales";
import { useApp } from "../store/app";
import { TooltipProvider } from "../components/ui/tooltip";
import { FullMessage } from "./Screens";
import { Shell } from "./Shell";
import { Splash } from "./Splash";

export function App() {
  const connection = useApp((s) => s.connection);
  const hasView = useApp((s) => s.view !== null);
  if (connection === "no-token")
    return <FullMessage title={t.connection.noToken} body={t.connection.noTokenBody} />;
  if (connection === "expired")
    return <FullMessage title={t.connection.expired} body={t.connection.expiredBody} />;
  return (
    <TooltipProvider>
      {hasView && <Shell />}
      <Splash />
    </TooltipProvider>
  );
}
