// From the Quests section to the map: preview a quest there (its objectives are drawn even if
// it isn't active) and, optionally, guide to one of its spots.
import { send } from "./socket";
import { useApp } from "../store/app";

export function showQuestOnMap(questId: string, map: string, target?: string): void {
  const s = useApp.getState();
  send({ type: "questFocus", questId });
  s.setQuestFocus(questId);
  if (s.view?.activeMap?.id !== map) send({ type: "setActiveMap", mapId: map });
  if (target) send({ type: "selectExtract", extractId: target });
  s.setWorkspace("map");
}

export function stopQuestPreview(): void {
  send({ type: "questFocus", questId: null });
  useApp.getState().setQuestFocus(null);
}
