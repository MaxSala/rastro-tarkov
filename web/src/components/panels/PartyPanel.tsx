import { Copy, UserX } from "lucide-react";
import { useState } from "react";
import { t } from "../../locales";
import { iconSm } from "../../lib/icons";
import { playerVar } from "../../lib/players";
import type { PeerView } from "../../lib/protocol";
import { send } from "../../lib/socket";
import { useApp } from "../../store/app";
import { Button, IconButton } from "../ui/button";
import { Textarea } from "../ui/input";
import { PanelFrame } from "./PanelFrame";

export async function copyPartyCode(code: string | null | undefined) {
  if (!code) return;
  try {
    await navigator.clipboard.writeText(code);
    useApp.getState().notify("info", t.party.copied);
  } catch {
    useApp.getState().notify("error", t.errors.generic);
  }
}

function Dot({ color }: { color: number }) {
  return (
    <span aria-hidden className="size-2 shrink-0 rounded-full" style={{ background: playerVar(color) }} />
  );
}

function PeerRow({ p, canKick }: { p: PeerView; canKick: boolean }) {
  return (
    <li className="flex items-center gap-2.5 px-3 py-1.5">
      <Dot color={p.color} />
      <span className="min-w-0 flex-1">
        <span className="block truncate text-sm">{p.name}</span>
        <span className="block text-xs text-fg-muted">{p.onThisMap ? t.party.onMap : t.party.otherMap}</span>
      </span>
      {canKick && (
        <IconButton
          label={`${t.party.kick} ${p.name}`}
          onClick={() => send({ type: "partyKick", peerId: p.id })}
        >
          <UserX {...iconSm} />
        </IconButton>
      )}
    </li>
  );
}

function Idle({ error }: { error: string | null }) {
  const openDialog = useApp((s) => s.openDialog);
  const [code, setCode] = useState("");
  return (
    <div className="grid gap-4 p-3">
      {error && (
        <p role="alert" className="rounded-sm border border-danger/60 px-3 py-2 text-sm text-danger">
          {error}
        </p>
      )}
      <div className="grid gap-1">
        <h3 className="text-sm font-medium">{t.party.idleTitle}</h3>
        <p className="text-xs text-fg-muted">{t.party.idleBody}</p>
      </div>
      <Button variant="primary" onClick={() => openDialog("host")}>
        {t.party.host}
      </Button>
      <form
        className="grid gap-2"
        onSubmit={(e) => {
          e.preventDefault();
          if (code.trim()) send({ type: "partyJoin", code });
        }}
      >
        <label htmlFor="join-code" className="sr-only">
          {t.party.join}
        </label>
        <Textarea
          id="join-code"
          value={code}
          onChange={(e) => setCode(e.target.value)}
          placeholder={t.party.joinPlaceholder}
          className="num text-xs"
          spellCheck={false}
          autoComplete="off"
        />
        <Button type="submit" disabled={!code.trim()}>
          {t.party.join}
        </Button>
      </form>
    </div>
  );
}

export function PartyPanel() {
  const party = useApp((s) => s.view?.party);
  const me = useApp((s) => s.view?.settings);
  if (!party || !me) return null;
  if (party.status === "idle" || party.status === "error") {
    return (
      <PanelFrame title={t.panels.party}>
        <Idle error={party.error} />
      </PanelFrame>
    );
  }
  const statusText = party.isHost
    ? t.party.hosting
    : party.status === "connecting"
      ? t.party.connecting
      : t.party.connected;
  return (
    <PanelFrame title={t.panels.party}>
      <div className="grid gap-3 border-b border-line p-3">
        <p className="flex items-center gap-2 text-sm">
          <span
            className={
              party.status === "connecting"
                ? "size-1.5 animate-pulse rounded-full bg-fg-muted"
                : "size-1.5 rounded-full bg-ok"
            }
          />
          {statusText}
        </p>
        {party.isHost && party.code && (
          <div className="grid gap-1.5">
            <span className="text-xs text-fg-muted">{t.party.code}</span>
            <code className="num block rounded-sm border border-line-strong bg-app px-2 py-1.5 text-xs break-all select-all">
              {party.code}
            </code>
            <Button size="sm" onClick={() => copyPartyCode(party.code)}>
              <Copy {...iconSm} />
              {t.party.copy}
            </Button>
          </div>
        )}
        <p className="text-xs text-fg-muted">{t.party.pingHint}</p>
      </div>
      <h3 className="px-3 pt-3 pb-1 text-xs font-medium text-fg-muted">{t.party.peers}</h3>
      <ul>
        <li className="flex items-center gap-2.5 px-3 py-1.5">
          <Dot color={me.playerColor} />
          <span className="text-sm">
            {me.playerName} <span className="text-fg-muted">({t.party.you})</span>
          </span>
        </li>
        {party.peers.map((p) => (
          <PeerRow key={p.id} p={p} canKick={party.isHost} />
        ))}
      </ul>
      {party.peers.length === 0 && <p className="px-3 py-2 text-xs text-fg-muted">{t.party.noPeers}</p>}
      <div className="p-3">
        <Button variant="danger" size="sm" className="w-full" onClick={() => send({ type: "partyLeave" })}>
          {party.isHost ? t.party.close : t.party.leave}
        </Button>
      </div>
    </PanelFrame>
  );
}
