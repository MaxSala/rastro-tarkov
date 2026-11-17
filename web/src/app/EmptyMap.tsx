import { Upload } from "lucide-react";
import { t } from "../locales";
import { icon } from "../lib/icons";
import { useApp } from "../store/app";
import { Button } from "../components/ui/button";

export function EmptyMap() {
  const openDialog = useApp((s) => s.openDialog);
  return (
    <div className="grid h-full place-items-center bg-app p-6">
      <div className="grid max-w-sm gap-4">
        <div className="grid gap-1.5">
          <h1 className="text-lg font-medium">{t.empty.title}</h1>
          <p className="text-sm text-fg-soft">{t.empty.body}</p>
        </div>
        <div className="flex flex-wrap items-center gap-2">
          <Button variant="primary" onClick={() => openDialog("import")}>
            <Upload {...icon} />
            {t.maps.import}
          </Button>
          <Button onClick={() => openDialog("import", "demo")}>{t.empty.demo}</Button>
          <a
            href="/docs/MAP_PACK.md"
            target="_blank"
            rel="noreferrer"
            className="px-1 text-sm text-fg-soft underline underline-offset-4 hover:text-fg"
          >
            {t.empty.docs}
          </a>
        </div>
      </div>
    </div>
  );
}
