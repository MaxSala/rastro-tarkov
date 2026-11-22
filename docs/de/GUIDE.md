<p align="center">
  <a href="https://maxsala.github.io/rastro-tarkov/de/docs/guide/">Website</a> · <a href="../../README.de.md">Zurück zur README</a> · <a href="FEATURES.md">Funktionen</a> · <b>Anleitung</b> · <a href="FAQ.md">Fragen und Sicherheit</a>
</p>

# Anleitung

## Installation

1. **[Lade `Rastro-windows-x64.exe` herunter](https://github.com/MaxSala/rastro-tarkov/releases/latest/download/Rastro-windows-x64.exe)**. Dieser Link liefert immer die neueste Version.
2. Leg die Datei, wohin du willst: auf den Desktop, in einen Ordner in Dokumente, egal wohin. Sie braucht keine weiteren Dateien daneben.
3. Doppelklick. Wenn Windows eine blaue Warnung zeigt, siehe [Ist es sicher?](FAQ.md#ist-es-sicher)

Benötigt Windows 10 oder 11, 64 Bit.

### Erster Start

Beim ersten Mal öffnet sich eine kurze Anleitung:

1. **Screenshot-Ordner.** Rastro findet `Documents\Escape from Tarkov\Screenshots` von selbst, auch wenn Windows die Dokumente nach OneDrive verschoben hat. Falls nicht, füg den Pfad ein und es zeigt dir, wie viele Screenshots mit Koordinaten es sieht. Hier kannst du auch die Raid-Erkennung einschalten.
2. **Karten.** Lade die Community-Karten von tarkov.dev herunter (empfohlen), importiere ein Kartenpaket oder probier die Demokarte aus.
3. Fertig. Mach im Spiel einen Screenshot und dein Pfeil erscheint auf der Karte.

Für den Markt, die Munitionstabelle und Schlüsselnamen öffne einmal den Tab Markt und drück *Preise laden*.

### Aktualisieren

Wenn du Update-Prüfungen erlaubt hast, erscheint bei einer neuen Version oben ein Knopf, der dich zur Download-Seite bringt. Andernfalls lade die neue `.exe` über den [Link oben](#installation) herunter, schließ Rastro und nutze die neue Datei statt der alten. Einstellungen, Punkte, Karten und Preise liegen [separat](#wo-deine-daten-liegen), es geht also nichts verloren.

Nach einem Update zeigt Rastro, was sich geändert hat. Den vollständigen Verlauf findest du unter *Einstellungen → Über → Neuigkeiten* und in [CHANGELOG.md](../../CHANGELOG.md). Bringt ein Update neue Kartendaten, hol sie dir mit *Von tarkov.dev aktualisieren* im Karten-Panel.

### Tastenkürzel

| Aktion | Wie |
|---|---|
| Position aktualisieren | Im Spiel einen Screenshot machen |
| Bereich wechseln (Karte, Markt, Munition, Quests, Versteck) | `Ctrl+1`, `Ctrl+2`, `Ctrl+3`, `Ctrl+4`, `Ctrl+5` |
| Befehlspalette | `Ctrl+K` |
| Panels für Extraktionen, Punkte, Gruppe, Karten und Quests | `1`, `2`, `3`, `4`, `5` |
| Neuer Raid (Spur und Markierungen löschen) | `N` |
| Ganze Karte zeigen / deiner Position folgen | `F` / `G` |
| Zoom | `+` / `-` oder Mausrad |
| Punkt hinzufügen, zu einem Ort führen | Rechtsklick auf die Karte |
| Squad anpingen | `Shift` + Klick |
| Im Markt suchen | `/` |
| Einstellungen | `,` |

### Wo deine Daten liegen

Einstellungen, deine Punkte, dein Quest-Fortschritt, heruntergeladene Karten sowie der Preis- und Quest-Cache liegen in einem Ordner, getrennt von der `.exe`:

| System | Ordner |
|---|---|
| Windows | `%APPDATA%\Rastro\data\` |
| Linux | `~/.local/share/rastro/` |
| macOS | `~/Library/Application Support/Rastro/` |

Unter *Einstellungen → Protokoll* gibt es einen Knopf, um ihn zu öffnen.

### Wenn etwas nicht funktioniert

- **Es öffnet sich im Browser statt im eigenen Fenster.** Rastro zeichnet sein Fenster mit Microsoft WebView2. Das ist in Windows 10 und 11 enthalten; wurde es entfernt, weicht Rastro auf deinen Browser aus. Installier die [WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/), dann ist das Fenster zurück.
- **Eine Fehlerseite meldet, dass der Port belegt ist.** Wahrscheinlich läuft noch eine andere Rastro-Instanz. Beende sie über die Taskleiste oder den Task-Manager (`rastro.exe`) und starte neu.
- **Meine Position bewegt sich nie.** Prüf, ob der Screenshot-Ordner unter *Einstellungen → Allgemein* der ist, in den das Spiel wirklich speichert, und ob du die Screenshot-Taste im Spiel drückst.
- **Meine Mitspieler kommen nicht in meinen Raum.** Sie müssen deinen PC erreichen können. Nutz beim Öffnen des Raums die Adresse aus deinem Mesh-VPN (Tailscale, ZeroTier) oder öffne den Gruppen-Port an deinem Router.
- **Immer noch nicht?** Frag auf [Discord](https://discord.gg/H4pRxGBGDb). Unter *Einstellungen → Protokoll* gibt es einen *Kopieren*-Knopf, und dieses Protokoll hilft sehr.

## So funktioniert es

Rastro liest zwei Arten von Dateien, die Escape from Tarkov ohnehin auf deinem PC speichert. Mehr braucht es nicht.

| Was gelesen wird | Wo | Was Rastro daraus gewinnt |
|---|---|---|
| **Die Dateinamen deiner Screenshots** | `Documents\Escape from Tarkov\Screenshots` | Deine Position und Blickrichtung. Tarkov schreibt deine Koordinaten in den Namen jedes gespeicherten Screenshots. Rastro liest den Namen, nie das Bild, und verschiebt, bearbeitet oder löscht deine Screenshots nie. |
| **Die Logdateien des Spiels** (optional, aus, bis du es einschaltest) | `Battlestate Games\Escape from Tarkov\Logs` | Welche Karte lädt, wann ein Raid beginnt und ob du PvP oder PvE spielst. Damit wechselt Rastro selbst zur richtigen Karte, beginnt in jedem Raid eine neue Spur und zeigt die verbleibende Raidzeit. Gespeichert werden nur Kartenname und Spielmodus. Außerdem liest es die Händlernachrichten, um Quests abzuhaken, die du annimmst, abgibst oder verlierst. |

In der Praxis: Du drückst im Spiel die Screenshot-Taste, und eine Sekunde später bewegt sich dein Pfeil auf der Karte.

Was Rastro **nicht** tut: Es liest keinen Spielspeicher, injiziert nichts, hängt sich nicht in den Spielprozess, schneidet keinen Netzwerkverkehr mit, zeichnet nichts über das Spiel und drückt keine Tasten für dich. Es läuft als eigenes Programm und schaut nur auf Dateien auf der Festplatte.

## Aus dem Quellcode bauen

Das brauchst du nur, wenn du den Code ändern oder die `.exe` selbst bauen willst. Du brauchst [Rust](https://rustup.rs/) 1.85 oder neuer und [Node.js](https://nodejs.org/) LTS (unter Linux zusätzlich `libwebkit2gtk-4.1-dev` und `libgtk-3-dev`):

```sh
git clone https://github.com/MaxSala/rastro-tarkov.git
cd rastro-tarkov
./build.sh        # build.bat unter Windows
```

Das Ergebnis ist eine einzelne ausführbare Datei in `target/release/`. Entwicklungsumgebung, Projektaufbau, Code-Regeln und das Hinzufügen einer Übersetzung stehen in **[CONTRIBUTING.md](../../CONTRIBUTING.md)** (auf Englisch).
