<p align="center">
  <a href="../../README.de.md">Zurück zur README</a> · <a href="FEATURES.md">Funktionen</a> · <a href="GUIDE.md">Anleitung</a> · <b>Fragen und Sicherheit</b>
</p>

# Fragen und Sicherheit

## Ist es sicher?

Rastro ist ein kleines, portables Programm. Das bedeutet es, und so kannst du es prüfen.

- **Kein Installer, keine Adminrechte.** Es ist eine einzelne `.exe`. Es installiert keine Dienste oder Treiber, startet nicht mit Windows und rührt die Spieldateien nicht an. Einstellungen und heruntergeladene Karten liegen in einem Ordner (siehe [Wo deine Daten liegen](GUIDE.md#wo-deine-daten-liegen)); diesen Ordner und die `.exe` löschen, und es ist komplett weg.
- **Open Source.** Der gesamte Code liegt in diesem Repository. Jeder kann ihn lesen.
- **Öffentlich gebaut.** Jede Version wird von GitHubs eigenen Servern direkt aus dem Quellcode gebaut, nicht auf irgendeinem PC. Jeden Build siehst du im Tab [Actions](https://github.com/MaxSala/rastro-tarkov/actions), und die Release-Seite nennt den SHA-256 der Datei, damit du prüfen kannst, dass du genau diese Datei hast:

  ```powershell
  Get-FileHash .\Rastro-windows-x64.exe
  ```

- **Telefoniert nicht nach Hause.** Keine Telemetrie, keine Werbung, kein Konto. Es verbindet sich nur mit dem Internet, wenn du es darum bittest, und die vollständige Liste der Verbindungen steht unter [Datenschutz](#datenschutz-und-netzwerk).

**„Der Computer wurde durch Windows geschützt".** Beim ersten Start zeigt Windows SmartScreen sehr wahrscheinlich ein blaues Fenster, dass eine unbekannte App gestoppt wurde. Das passiert bei jedem neuen Programm ohne kostenpflichtiges Zertifikat, nicht weil mit der Datei etwas nicht stimmt. Klick auf **Weitere Informationen** und dann auf **Trotzdem ausführen**. Windows merkt sich deine Wahl. Manche Virenscanner sind bei neuen, unsignierten Programmen ebenfalls misstrauisch. Wenn deiner Rastro meldet, kannst du die Datei wiederherstellen und eine Ausnahme hinzufügen, sie für eine zweite Meinung bei [VirusTotal](https://www.virustotal.com/) hochladen und sie deinem Antivirus-Hersteller als Fehlalarm melden.

> [!IMPORTANT]
> Rastro ist ein inoffizielles Fanprojekt, nicht mit Battlestate Games oder tarkov.dev verbunden und nicht von ihnen unterstützt. Es ist so gebaut, dass es sich aus dem Spiel heraushält: Es liest nur Dateien auf der Festplatte und interagiert nie mit dem Spielprozess. Trotzdem liegt die Nutzung von Drittanbieter-Tools mit Escape from Tarkov gemäß den Nutzungsbedingungen von Battlestate Games in deiner eigenen Verantwortung.

## Fragen

**Kann ich dafür gebannt werden?**
Rastro liest nie den Spielspeicher, rührt nie den Spielprozess an und sendet nie Eingaben an das Spiel. Es liest Screenshot-Dateinamen und Logdateien, nachdem das Spiel sie geschrieben hat, dieselben Dateien, die du selbst im Explorer öffnen kannst. Für Battlestate Games können wir aber nicht sprechen, siehe den Hinweis unter [Ist es sicher?](#ist-es-sicher)

**Funktioniert es in PvE?**
Ja. Die Raid-Erkennung wurde mit PvE-Logs getestet, und Rastro lädt PvE-Preise und -Daten, wenn es PvE erkennt.

**Braucht es Internet?**
Nur zum Herunterladen von Karten und Preisen, für Update-Prüfungen (wenn du sie erlaubst) und zum Spielen im Squad. Alles andere funktioniert offline.

**Warum Screenshots?**
Sie sind der einzige Ort, an dem das Spiel deine genaue Position auf die Festplatte schreibt. Ein paar Sekunden Verzögerung dafür, dass das Spiel nie angefasst wird.

**Linux oder macOS?**
Rastro lässt sich unter Linux bauen und läuft dort; macOS ist ungetestet. Tarkov selbst läuft nur unter Windows, daher gibt es den Download nur für Windows. Siehe [Aus dem Quellcode bauen](GUIDE.md#aus-dem-quellcode-bauen).

## Datenschutz und Netzwerk

Rastro hat kein Konto, keine Telemetrie und keine Werbung. Die Oberfläche spricht nur mit der App selbst auf `127.0.0.1`, geschützt durch einen zufälligen Schlüssel, der sich bei jedem Start ändert. Alles, was von deiner Festplatte gelesen wird, bleibt auf deinem PC. Alles andere passiert nur, wenn du es willst:

| Wann | Verbindet sich mit |
|---|---|
| Immer | Nichts außerhalb deines PCs |
| Du lädst Karten von tarkov.dev | `raw.githubusercontent.com`, `json.tarkov.dev` und als Ausweichlösung `api.tarkov.dev` |
| Du lädst oder aktualisierst Preise | `json.tarkov.dev`, dazu `assets.tarkov.dev` für Gegenstandssymbole |
| Du lädst oder aktualisierst Quests | `json.tarkov.dev` und als Ausweichlösung `api.tarkov.dev`, dazu `assets.tarkov.dev` für Bilder |
| Du importierst ein Kartenpaket per URL | Nur die Adresse, die du eingibst (nur HTTPS, nie interne Adressen) |
| Du hostest eine Gruppe oder trittst einer bei | Direkt zu den PCs deiner Mitspieler |
| Einmal pro Start, wenn du Update-Prüfungen erlaubt hast (oder bei *Jetzt prüfen*) | `api.github.com`, um die neueste Versionsnummer zu lesen. Über dich wird nichts gesendet. |

Jeder Download-Host steht auf einer festen Erlaubnisliste, die auch bei Weiterleitungen geprüft wird, und alles Heruntergeladene wird vor der Nutzung validiert. Das vollständige Bedrohungsmodell steht in [docs/SECURITY.md](../SECURITY.md) (auf Englisch).

## Bekannte Einschränkungen

- Deine Position wird nur aktualisiert, wenn du einen Screenshot machst. Das ist Absicht: Nur dort schreibt das Spiel deine Position.
- Labs hat auf tarkov.dev keine SVG-Karte. Rastro erkennt Labs, kann es aber nur anzeigen, wenn du ein eigenes Kartenpaket importierst.
- Die Raid-Erkennung wurde mit PvE-Logs geprüft. PvP sollte genauso funktionieren, ist aber mit echten Logs noch nicht bestätigt.
- Die spanische und englische Übersetzung sind geprüft. Die anderen 14 hat noch kein Muttersprachler kontrolliert. Korrekturen sind sehr willkommen, hier oder auf [Discord](https://discord.gg/H4pRxGBGDb).
- Nach einem Wechsel der Sprache oder des Spielmodus (PvP/PvE) müssen Karten und Preise neu geladen werden, um die neuen Namen zu sehen.
- Einige technische Details in Fehlermeldungen und im Protokoll sind unabhängig von der Sprache auf Englisch.

## Nicht kommerziell

Rastro ist ein Hobbyprojekt für die Community. Es ist kostenlos, es gibt keine Bezahlversion, es zeigt keine Werbung und sammelt keine Daten. Das bleibt so, aus Überzeugung und weil die tarkov.dev-Karten, auf denen es aufbaut, nur für nicht-kommerzielle Nutzung lizenziert sind.
