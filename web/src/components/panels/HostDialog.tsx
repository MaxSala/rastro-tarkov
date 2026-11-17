import { ShieldAlert } from "lucide-react";
import { useEffect, useState } from "react";
import { t } from "../../locales";
import { icon } from "../../lib/icons";
import { send } from "../../lib/socket";
import { useApp } from "../../store/app";
import { Button } from "../ui/button";
import { Dialog } from "../ui/dialog";
import { Field } from "../ui/field";
import { Input } from "../ui/input";

const ALL = "0.0.0.0";

export function HostDialog() {
  const open = useApp((s) => s.dialog === "host");
  return open ? <HostFlow /> : null;
}

function HostFlow() {
  const open = true;
  const openDialog = useApp((s) => s.openDialog);
  const interfaces = useApp((s) => s.interfaces);
  const settings = useApp((s) => s.view?.settings);
  const [bind, setBind] = useState(ALL);
  const [advertise, setAdvertise] = useState("");
  const [warned, setWarned] = useState(() => Boolean(useApp.getState().view?.settings.shareWarningAck));

  useEffect(() => {
    send({ type: "listInterfaces" });
  }, []);

  if (!settings) return null;
  const close = () => openDialog(null);

  if (!warned) {
    return (
      <Dialog
        open={open}
        onClose={close}
        title={t.party.warnTitle}
        footer={
          <Button
            variant="primary"
            onClick={() => {
              send({ type: "ackShareWarning" });
              setWarned(true);
            }}
          >
            {t.party.warnAccept}
          </Button>
        }
      >
        <p className="flex gap-3 text-sm text-fg-soft">
          <ShieldAlert {...icon} className="mt-0.5 shrink-0 text-accent" />
          {t.party.warnBody}
        </p>
      </Dialog>
    );
  }

  return (
    <Dialog
      open={open}
      onClose={close}
      title={t.party.hostTitle}
      description={t.party.hostBody}
      footer={
        <>
          <Button onClick={close}>{t.settings.cancel}</Button>
          <Button
            variant="primary"
            disabled={!advertise.trim()}
            onClick={() => {
              send({ type: "partyHost", bindIp: bind, advertise: advertise.trim() });
              if (useApp.getState().panel !== "party") useApp.getState().togglePanel("party");
              close();
            }}
          >
            {t.party.open}
          </Button>
        </>
      }
    >
      <Field id="host-iface" label={t.party.iface}>
        <select
          id="host-iface"
          value={bind}
          onChange={(e) => {
            setBind(e.target.value);
            if (e.target.value !== ALL) setAdvertise(e.target.value);
          }}
          className="h-8 rounded-sm border border-line-strong bg-app px-2 text-sm"
        >
          <option value={ALL}>{t.party.allIfaces}</option>
          {interfaces.map((i) => (
            <option key={`${i.name}-${i.ip}`} value={i.ip}>
              {i.name} ({i.ip})
            </option>
          ))}
        </select>
      </Field>
      <Field id="host-adv" label={t.party.advertise} hint={t.party.advertiseHint}>
        <Input
          id="host-adv"
          className="num"
          value={advertise}
          onChange={(e) => setAdvertise(e.target.value)}
          placeholder="100.64.0.12"
          spellCheck={false}
          aria-describedby="host-adv-hint"
        />
      </Field>
      <Field id="host-port" label={t.party.port} hint={t.party.portHint}>
        <Input
          id="host-port"
          className="num"
          value={settings.partyPort}
          readOnly
          aria-describedby="host-port-hint"
        />
      </Field>
    </Dialog>
  );
}
