<p align="center">
  <img src="docs/images/banner.png" alt="Rastro: eine Live-Raidkarte für Escape from Tarkov auf deinem zweiten Monitor" width="100%">
</p>

<p align="center">
  🌐 <a href="README.md">English</a> · <a href="README.es.md">Español</a> · <a href="README.ru.md">Русский</a> · <b>Deutsch</b> · <a href="README.pt-BR.md">Português</a> · <a href="README.fr.md">Français</a> · <a href="README.pl.md">Polski</a> · <a href="README.zh-CN.md">中文</a>
  <br>
  <sub>Übersetzung der Community. Wenn etwas vom <a href="README.md">englischen README</a> abweicht, ist die englische Version aktuell. Fehler gefunden? <a href="https://discord.gg/H4pRxGBGDb">Sag uns auf Discord Bescheid</a>.</sub>
</p>

<p align="center">
  <a href="https://github.com/MaxSala/rastro-tarkov/releases/latest"><img src="https://img.shields.io/github/v/release/MaxSala/rastro-tarkov?label=Neueste%20Version&color=f0a43a" alt="Neueste Version"></a>
  <img src="https://img.shields.io/badge/Status-%C3%96ffentliche%20Beta-f0a43a" alt="Öffentliche Beta">
  <a href="https://github.com/MaxSala/rastro-tarkov/stargazers"><img src="https://img.shields.io/github/stars/MaxSala/rastro-tarkov?label=Sterne&color=f0a43a&logo=github" alt="GitHub-Sterne"></a>
  <a href="https://discord.gg/H4pRxGBGDb"><img src="https://img.shields.io/badge/Discord-Tritt%20der%20Community%20bei-5865F2?logo=discord&logoColor=white" alt="Zum Discord"></a>
  <img src="https://img.shields.io/badge/Windows-10%20%7C%2011-0078D4?logo=windows&logoColor=white" alt="Windows 10 und 11">
  <a href="LICENSE"><img src="https://img.shields.io/github/license/MaxSala/rastro-tarkov?color=4c4c4c" alt="MIT-Lizenz"></a>
</p>

<h3 align="center">
  <a href="https://github.com/MaxSala/rastro-tarkov/releases/latest/download/Rastro-windows-x64.exe">⬇ Rastro für Windows herunterladen</a>
</h3>

<p align="center">
  Kostenlos und Open Source · Kein Installer · Kein Konto · Nichts einzurichten
  <br>
  <a href="#so-funktioniert-es">So funktioniert es</a> ·
  <a href="#spiel-mit-deinem-squad">Squad-Modus</a> ·
  <a href="#ist-es-sicher">Ist es sicher?</a> ·
  <a href="#installation">Installation</a> ·
  <a href="#wohin-rastro-geht">Roadmap</a> ·
  <a href="#mitmachen">Mitmachen</a> ·
  <a href="https://discord.gg/H4pRxGBGDb">Discord</a>
</p>

> [!TIP]
> **Rastro ist in der öffentlichen Beta, und das ist erst der Anfang.** Quests, das Versteck und vieles mehr sind unterwegs, und die Community entscheidet, was zuerst kommt.
> Wenn dir gefällt, wohin es geht, **[⭐ gib dem Repo einen Stern](https://github.com/MaxSala/rastro-tarkov/stargazers)** und **[komm auf den Discord](https://discord.gg/H4pRxGBGDb)**. Das dauert fünf Sekunden und hilft mehr, als du denkst.

---

**Rastro ist eine Live-Karte deines Raids auf deinem zweiten Monitor.** Mach im Spiel einen Screenshot und deine Position erscheint auf der Karte, mit Entfernung und Richtung zu jeder Extraktion. Öffne einen Raum und dein Squad sieht sich gegenseitig auf derselben Karte, jeder an seinem eigenen PC. Fahr mit der Maus über eine verschlossene Tür und du siehst, welcher Schlüssel sie öffnet und was er kostet. Prüf Preise und Munition, ohne per Alt-Tab in den Browser zu wechseln.

Das Spiel wird nie angefasst. Rastro liest nur Dateien, die Tarkov ohnehin auf deine Festplatte schreibt. Es ist kostenlos, Open Source und von Spielern für Spieler gemacht.

## Screenshots

<table>
  <tr>
    <td width="50%"><img src="docs/images/party.png" alt="Drei Squad-Mitglieder auf Customs, mit der Karte eines Mitspielers"></td>
    <td width="50%"><img src="docs/images/map.png" alt="Customs mit deiner Position, deiner Spur und der Führung zu einer Extraktion"></td>
  </tr>
  <tr>
    <td align="center"><b>Squad.</b> Position und Blickrichtung aller auf einer Karte, mit Pings. Kein Server dazwischen.</td>
    <td align="center"><b>Karte.</b> Deine Position und Spur, alle Extraktionen und live Entfernung und Richtung zur gewählten.</td>
  </tr>
  <tr>
    <td width="50%"><img src="docs/images/floors.png" alt="Die Wohnheime im zweiten Stock, der Rest der Karte und die Mitspieler unten abgedunkelt"></td>
    <td width="50%"><img src="docs/images/keys.png" alt="Die Schlösser-Ebene mit geöffneter Schlüsselkarte"></td>
  </tr>
  <tr>
    <td align="center"><b>Stockwerke.</b> Folgt dir von Stockwerk zu Stockwerk und blendet alles aus, was nicht auf deinem liegt.</td>
    <td align="center"><b>Schlüssel.</b> Jede verschlossene Tür auf der Karte, welcher Schlüssel sie öffnet und was er kostet.</td>
  </tr>
  <tr>
    <td width="50%"><img src="docs/images/market.png" alt="Marktsuche mit den Angeboten eines Gegenstands"></td>
    <td width="50%"><img src="docs/images/ammo.png" alt="Munitionstabelle für ein Kaliber"></td>
  </tr>
  <tr>
    <td align="center"><b>Markt.</b> Sofortige Offline-Suche mit Flohmarkt- und Händlerpreisen, sortiert nach Wert pro Feld.</td>
    <td align="center"><b>Munition.</b> Schaden, Durchschlag und Panzerungsschaden jeder Patrone, mit dem günstigsten Ort zum Kaufen.</td>
  </tr>
</table>

<sub>Kartenmaterial von den <a href="https://github.com/the-hideout/tarkov-dev-svg-maps">tarkov.dev-Mitwirkenden</a>, CC BY-NC-SA 4.0. Preise und Gegenstandsdaten von <a href="https://tarkov.dev">tarkov.dev</a>. Die Squad-Screenshots verwenden Beispielpositionen. Die Screenshots zeigen die App auf Englisch.</sub>

## Wohin Rastro geht

Rastro begann als Karte auf dem zweiten Monitor, damit du dich nie wieder verläufst. Der Plan ist viel größer: **eine Begleit-App für alles, was du zwischen und während der Raids nachschaust**, ohne das Spiel jemals anzufassen. Das liegt auf dem Tisch:

| | Als Nächstes | Was das für dich bedeutet |
|---|---|---|
| 📜 | **Quest-Tracker** | Deine aktiven Quests auf der Karte, jedes Ziel markiert. Schritt-für-Schritt-Hilfe für Neue, eine schnelle Checkliste für Veteranen. |
| 👥 | **Squad-Quests** | Die offenen Quests deiner Mitspieler auf derselben Karte, damit das Squad eine gemeinsame Route plant. |
| 🏠 | **Versteck-Tracker** | Jedes Upgrade, was es braucht und welche Gegenstände du behalten solltest. Nie wieder heute verkaufen, was du morgen brauchst. |
| 🔑 | **Dein Schlüsselbund** | Markiere die Schlüssel, die du hast, und die Türen, die du öffnen kannst, stechen auf jeder Karte hervor. |
| 🛒 | **Einkaufsliste** | Favorisiere Gegenstände und werde benachrichtigt, wenn sich ihr Preis ändert. |
| 📊 | **Raid-Verlauf** | Karte, Dauer, Wartezeit und Spielmodus jedes Raids. |
| 🎯 | **Munition gegen Panzerung** | Ein Durchschlagsdiagramm jeder Patrone gegen jede Panzerungsklasse. |
| 📍 | **Punkte teilen** | Schick deine Loot-Spots und Verstecke an dein Squad oder nimm sie auf einen anderen PC mit. |

Noch hat nichts davon ein Datum, und die Reihenfolge steht nicht fest: **sie richtet sich danach, was Spieler sich wünschen.** Die vollständige Liste mit kleineren Punkten und bereits Veröffentlichtem steht in [docs/ROADMAP.md](docs/ROADMAP.md) (auf Englisch).

**Eine Idee?** Poste sie im Feature-Requests-Forum auf [Discord](https://discord.gg/H4pRxGBGDb) oder [eröffne ein Issue](https://github.com/MaxSala/rastro-tarkov/issues). Jeder Vorschlag wird gelesen, und die meistgewünschten rücken nach oben.

## So funktioniert es

Rastro liest zwei Arten von Dateien, die Escape from Tarkov ohnehin auf deinem PC speichert. Mehr braucht es nicht.

| Was gelesen wird | Wo | Was Rastro daraus gewinnt |
|---|---|---|
| **Die Dateinamen deiner Screenshots** | `Documents\Escape from Tarkov\Screenshots` | Deine Position und Blickrichtung. Tarkov schreibt deine Koordinaten in den Namen jedes gespeicherten Screenshots. Rastro liest den Namen, nie das Bild, und verschiebt, bearbeitet oder löscht deine Screenshots nie. |
| **Die Logdateien des Spiels** (optional, aus, bis du es einschaltest) | `Battlestate Games\Escape from Tarkov\Logs` | Welche Karte lädt, wann ein Raid beginnt und ob du PvP oder PvE spielst. Damit wechselt Rastro selbst zur richtigen Karte, beginnt in jedem Raid eine neue Spur und zeigt die verbleibende Raidzeit. Gespeichert werden nur Kartenname und Spielmodus. |

In der Praxis: Du drückst im Spiel die Screenshot-Taste, und eine Sekunde später bewegt sich dein Pfeil auf der Karte.

Was Rastro **nicht** tut: Es liest keinen Spielspeicher, injiziert nichts, hängt sich nicht in den Spielprozess, schneidet keinen Netzwerkverkehr mit, zeichnet nichts über das Spiel und drückt keine Tasten für dich. Es läuft als eigenes Programm und schaut nur auf Dateien auf der Festplatte.

## Spiel mit deinem Squad

Der Gruppenmodus von Rastro bringt dein ganzes Squad auf dieselbe Karte, jeden mit eigenem Namen und eigener Farbe.

- **Sieh, wo alle sind.** Der Pfeil jedes Mitspielers zeigt, wo er beim letzten Screenshot war und wohin er geschaut hat. Ist jemand auf einem anderen Stockwerk, wird sein Pfeil abgedunkelt, und beim Drüberfahren siehst du, auf welchem.
- **Ping einen Ort.** `Shift` + Klick irgendwo auf die Karte, und der Ort blinkt bei allen auf.
- **Schnell eingerichtet.** Einer öffnet einen Raum und kopiert einen Einladungscode. Die anderen fügen ihn unter *Beitreten* ein. Fertig. Bis zu 8 Spieler.
- **Kein Server, kein Konto.** Positionen gehen direkt von PC zu PC. Niemand sonst sieht sie, auch wir nicht.
- **Verschlüsselt.** Der gesamte Verkehr ist Ende-zu-Ende-verschlüsselt mit dem [Noise-Protokoll](https://noiseprotocol.org/). Der Einladungscode ist der Schlüssel, also teil ihn nur mit deinem Squad.

Die anderen müssen den PC des Hosts erreichen können. Am einfachsten geht das mit einem kostenlosen Mesh-VPN wie [Tailscale](https://tailscale.com/) oder [ZeroTier](https://www.zerotier.com/): Jeder installiert es einmal, und Rastro funktioniert, als wärt ihr im selben Netzwerk, ohne Ports am Router zu öffnen. Ein geöffneter Port am Router des Hosts funktioniert auch.

Jeder Spieler nutzt Rastro trotzdem auf seinem eigenen PC und macht eigene Screenshots: Nur so bleiben alle Positionen aktuell.

## Funktionen

**Karte**
- Deine Position und Blickrichtung, aktualisiert bei jedem Screenshot, mit einer verblassenden Spur deines Weges.
- Klick auf eine Extraktion oder einen beliebigen Punkt für Entfernung, Abbiegerichtung und Höhenunterschied.
- Extraktionen gefiltert nach PMC, Scav oder gemeinsam. Markier die, die das Spiel in diesem Raid als offen anzeigt, und der Rest wird abgedunkelt.
- Ebenen von tarkov.dev: Übergänge, Schalter, Loot-Behälter (nach Typ filterbar) und Schlösser.
- Karten mit mehreren Stockwerken folgen dir automatisch, oder du wählst eines im Stockwerk-Panel. Der Rest der Karte und alles auf anderen Stockwerken wird abgedunkelt.
- Fahr mit der Maus über alles für eine Karte mit Name, Entfernung, Stockwerk und dem, was ein Klick bewirkt.
- Eigene Punkte: Rechtsklick auf die Karte, um Loot-Spots, Gefahrenzonen, Verstecke oder Quest-Orte zu speichern. Sie werden in Spielkoordinaten gespeichert und überstehen eine Neuinstallation der Karte.

**Schlüssel**
- Die Ebene *Schlösser* zeigt jede Tür, jeden Behälter und jeden Kofferraum, der einen Schlüssel braucht. Schlösser, die Strom brauchen, haben einen gelben Rand.
- Klick auf ein Schloss, um zu sehen, welcher Schlüssel es öffnet, wo er am günstigsten ist und wie viel er beim Verkauf bringt.
- *Diesen Schlüssel hervorheben* dunkelt alle anderen Schlösser ab, damit du alles siehst, was ein einzelner Schlüssel öffnet.
- Jeder Schlüssel im Markt listet die installierten Karten, auf denen er gebraucht wird, mit einem Link direkt dorthin.

**Markt**
- Durchsuche den kompletten Gegenstandskatalog nach Name oder Kurzname. Akzente und Wortreihenfolge spielen keine Rolle. Drück `/` zum Suchen und die Pfeiltasten, um durch die Ergebnisse zu gehen.
- Sortiere nach bestem Verkaufspreis, Flohmarktpreis, 24-Stunden-Durchschnitt, 48-Stunden-Änderung oder Preis pro Inventarfeld.
- Jeder Gegenstand listet alle Händler- und Flohmarktangebote, mit benötigtem Händlerlevel und Originalpreis in Dollar oder Euro.
- Der Katalog wird einmal heruntergeladen und auf deinem PC durchsucht, daher sind Suchen sofort da und funktionieren offline.

**Munition**
- Alle Patronen nach Kaliber gruppiert und nach Durchschlag sortiert.
- Schaden (pro Schrotkugel × Anzahl bei Schrot), Durchschlag, Panzerungsschaden, Fragmentierungschance, Mündungsgeschwindigkeit und der günstigste Kaufort. Leuchtspur ist markiert.

**Raid-Erkennung** (optional)
- Wechselt beim Laden eines Raids zur richtigen Karte, setzt Spur und Markierungen zu Beginn jedes Raids zurück und zeigt die verbleibende Zeit. Liest die Logdateien des Spiels, wie unter [So funktioniert es](#so-funktioniert-es) beschrieben.

**Alles andere**
- 16 Sprachen, dieselben wie bei tarkov.dev: Čeština, Deutsch, English, Español, Français, Magyar, Italiano, 日本語, 한국어, Polski, Português, Română, Русский, Slovenčina, Türkçe und 中文. Auch Gegenstands-, Karten- und Extraktionsnamen folgen deiner Sprache.
- PvP- und PvE-Daten, automatisch anhand der Spiel-Logs oder von Hand gewählt.
- Optionale Update-Hinweise und eine *Neuigkeiten*-Seite nach jedem Update.
- Tastenkürzel für alles und eine Befehlspalette (`Ctrl+K`).

## Ist es sicher?

Rastro ist ein kleines, portables Programm. Das bedeutet es, und so kannst du es prüfen.

- **Kein Installer, keine Adminrechte.** Es ist eine einzelne `.exe`. Es installiert keine Dienste oder Treiber, startet nicht mit Windows und rührt die Spieldateien nicht an. Einstellungen und heruntergeladene Karten liegen in einem Ordner (siehe [Wo deine Daten liegen](#wo-deine-daten-liegen)); diesen Ordner und die `.exe` löschen, und es ist komplett weg.
- **Open Source.** Der gesamte Code liegt in diesem Repository. Jeder kann ihn lesen.
- **Öffentlich gebaut.** Jede Version wird von GitHubs eigenen Servern direkt aus dem Quellcode gebaut, nicht auf irgendeinem PC. Jeden Build siehst du im Tab [Actions](https://github.com/MaxSala/rastro-tarkov/actions), und die Release-Seite nennt den SHA-256 der Datei, damit du prüfen kannst, dass du genau diese Datei hast:

  ```powershell
  Get-FileHash .\Rastro-windows-x64.exe
  ```

- **Telefoniert nicht nach Hause.** Keine Telemetrie, keine Werbung, kein Konto. Es verbindet sich nur mit dem Internet, wenn du es darum bittest, und die vollständige Liste der Verbindungen steht unter [Datenschutz](#datenschutz-und-netzwerk).

**„Der Computer wurde durch Windows geschützt".** Beim ersten Start zeigt Windows SmartScreen sehr wahrscheinlich ein blaues Fenster, dass eine unbekannte App gestoppt wurde. Das passiert bei jedem neuen Programm ohne kostenpflichtiges Zertifikat, nicht weil mit der Datei etwas nicht stimmt. Klick auf **Weitere Informationen** und dann auf **Trotzdem ausführen**. Windows merkt sich deine Wahl. Manche Virenscanner sind bei neuen, unsignierten Programmen ebenfalls misstrauisch. Wenn deiner Rastro meldet, kannst du die Datei wiederherstellen und eine Ausnahme hinzufügen, sie für eine zweite Meinung bei [VirusTotal](https://www.virustotal.com/) hochladen und sie deinem Antivirus-Hersteller als Fehlalarm melden.

> [!IMPORTANT]
> Rastro ist ein inoffizielles Fanprojekt, nicht mit Battlestate Games oder tarkov.dev verbunden und nicht von ihnen unterstützt. Es ist so gebaut, dass es sich aus dem Spiel heraushält: Es liest nur Dateien auf der Festplatte und interagiert nie mit dem Spielprozess. Trotzdem liegt die Nutzung von Drittanbieter-Tools mit Escape from Tarkov gemäß den Nutzungsbedingungen von Battlestate Games in deiner eigenen Verantwortung.

## Installation

1. **[Lade `Rastro-windows-x64.exe` herunter](https://github.com/MaxSala/rastro-tarkov/releases/latest/download/Rastro-windows-x64.exe)**. Dieser Link liefert immer die neueste Version.
2. Leg die Datei, wohin du willst: auf den Desktop, in einen Ordner in Dokumente, egal wohin. Sie braucht keine weiteren Dateien daneben.
3. Doppelklick. Wenn Windows eine blaue Warnung zeigt, siehe [Ist es sicher?](#ist-es-sicher)

Benötigt Windows 10 oder 11, 64 Bit.

### Erster Start

Beim ersten Mal öffnet sich eine kurze Anleitung:

1. **Screenshot-Ordner.** Rastro findet `Documents\Escape from Tarkov\Screenshots` von selbst, auch wenn Windows die Dokumente nach OneDrive verschoben hat. Falls nicht, füg den Pfad ein und es zeigt dir, wie viele Screenshots mit Koordinaten es sieht. Hier kannst du auch die Raid-Erkennung einschalten.
2. **Karten.** Lade die Community-Karten von tarkov.dev herunter (empfohlen), importiere ein Kartenpaket oder probier die Demokarte aus.
3. Fertig. Mach im Spiel einen Screenshot und dein Pfeil erscheint auf der Karte.

Für den Markt, die Munitionstabelle und Schlüsselnamen öffne einmal den Tab Markt und drück *Preise laden*.

### Aktualisieren

Wenn du Update-Prüfungen erlaubt hast, erscheint bei einer neuen Version oben ein Knopf, der dich zur Download-Seite bringt. Andernfalls lade die neue `.exe` über den [Link oben](#installation) herunter, schließ Rastro und nutze die neue Datei statt der alten. Einstellungen, Punkte, Karten und Preise liegen [separat](#wo-deine-daten-liegen), es geht also nichts verloren.

Nach einem Update zeigt Rastro, was sich geändert hat. Den vollständigen Verlauf findest du unter *Einstellungen → Über → Neuigkeiten* und in [CHANGELOG.md](CHANGELOG.md). Bringt ein Update neue Kartendaten, hol sie dir mit *Von tarkov.dev aktualisieren* im Karten-Panel.

### Tastenkürzel

| Aktion | Wie |
|---|---|
| Position aktualisieren | Im Spiel einen Screenshot machen |
| Bereich wechseln (Karte, Markt, Munition) | `Ctrl+1`, `Ctrl+2`, `Ctrl+3` |
| Befehlspalette | `Ctrl+K` |
| Panels für Extraktionen, Punkte, Gruppe und Karten | `1`, `2`, `3`, `4` |
| Neuer Raid (Spur und Markierungen löschen) | `N` |
| Ganze Karte zeigen / deiner Position folgen | `F` / `G` |
| Zoom | `+` / `-` oder Mausrad |
| Punkt hinzufügen, zu einem Ort führen | Rechtsklick auf die Karte |
| Squad anpingen | `Shift` + Klick |
| Im Markt suchen | `/` |
| Einstellungen | `,` |

### Wo deine Daten liegen

Einstellungen, deine Punkte, heruntergeladene Karten und der Preis-Cache liegen in einem Ordner, getrennt von der `.exe`:

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
Rastro lässt sich unter Linux bauen und läuft dort; macOS ist ungetestet. Tarkov selbst läuft nur unter Windows, daher gibt es den Download nur für Windows. Siehe [Aus dem Quellcode bauen](#aus-dem-quellcode-bauen).

## Datenschutz und Netzwerk

Rastro hat kein Konto, keine Telemetrie und keine Werbung. Die Oberfläche spricht nur mit der App selbst auf `127.0.0.1`, geschützt durch einen zufälligen Schlüssel, der sich bei jedem Start ändert. Alles, was von deiner Festplatte gelesen wird, bleibt auf deinem PC. Alles andere passiert nur, wenn du es willst:

| Wann | Verbindet sich mit |
|---|---|
| Immer | Nichts außerhalb deines PCs |
| Du lädst Karten von tarkov.dev | `raw.githubusercontent.com`, `json.tarkov.dev` und als Ausweichlösung `api.tarkov.dev` |
| Du lädst oder aktualisierst Preise | `json.tarkov.dev`, dazu `assets.tarkov.dev` für Gegenstandssymbole |
| Du importierst ein Kartenpaket per URL | Nur die Adresse, die du eingibst (nur HTTPS, nie interne Adressen) |
| Du hostest eine Gruppe oder trittst einer bei | Direkt zu den PCs deiner Mitspieler |
| Einmal pro Start, wenn du Update-Prüfungen erlaubt hast (oder bei *Jetzt prüfen*) | `api.github.com`, um die neueste Versionsnummer zu lesen. Über dich wird nichts gesendet. |

Jeder Download-Host steht auf einer festen Erlaubnisliste, die auch bei Weiterleitungen geprüft wird, und alles Heruntergeladene wird vor der Nutzung validiert. Das vollständige Bedrohungsmodell steht in [docs/SECURITY.md](docs/SECURITY.md) (auf Englisch).

## Mitmachen

Rastro wird offen von einem kleinen Team entwickelt, und wir wollen es zusammen mit den Leuten wachsen lassen, die es nutzen. Ob du fünf Sekunden oder fünf Stunden pro Woche hast: Es gibt einen Weg zu helfen.

### ⭐ Unterstütze das Projekt

- **Gib dem Repository einen Stern.** Kostet nichts, ist ein Klick und der wichtigste Weg, wie neue Spieler Rastro auf GitHub finden.
- **Watch → Custom → Releases**, um bei neuen Versionen benachrichtigt zu werden.
- **Erzähl es deinem Squad.** Der Gruppenmodus macht mit Freunden mehr Spaß, und über Mundpropaganda finden uns die meisten.
- **Teil einen Clip oder Screenshot** von Rastro in Aktion und markier uns auf Discord.

### 💬 Werde Teil der Community

Auf dem **[Rastro-Discord](https://discord.gg/H4pRxGBGDb)** passiert alles: Release-Ankündigungen, die Live-Roadmap, Hilfe von anderen Spielern, Foren für Bugs und Ideen und ein Ort, um ein Squad zum Testen des Gruppenmodus zu finden. Die Entwicklung läuft dort jeden Tag, also siehst du dort als Erster, was kommt.

### 🙋 Wir suchen Leute

Wir haben viel mehr Ideen als Hände. Wenn du dich in einer dieser Rollen wiedererkennst, melde dich:

| Rolle | Was du tun würdest |
|---|---|
| 🛡️ **Moderation** | Den Discord freundlich und hilfreich halten, während die Community wächst. |
| 🌍 **Übersetzung** | Eine der 14 Sprachen prüfen, die noch kein Muttersprachler gesehen hat, oder deine aktuell halten. |
| 🦀 **Entwicklung** | Rust im Backend, React und TypeScript in der Oberfläche. Von kleinen Fixes bis zu ganzen Features. |
| 🧪 **Testen** | Neue Versionen als Erster ausprobieren, vor allem in PvP, auf weniger gespielten Karten und auf verschiedenen Systemen. |
| 🗺️ **Karten und Daten** | Positionen, Stockwerke und Schlösser mit dem Spiel abgleichen und bei Karten helfen, die tarkov.dev noch nicht abdeckt. |
| 🎥 **Content Creator** | Streamer und Videomacher, die Rastro zeigen oder Feedback aus ihrer Community geben wollen. |

**So bewirbst du dich:** Öffne ein *Join the team*-Ticket im Support-Kanal auf [Discord](https://discord.gg/H4pRxGBGDb). Entwickler und Übersetzer können auch direkt mit einem Pull Request loslegen: [CONTRIBUTING.md](CONTRIBUTING.md) (auf Englisch) erklärt, wie.

### 💡 Ideen, Feedback und Bugs

Ehrliches Feedback ist das Wertvollste, was du einer Beta geben kannst. Sag uns, was verwirrt, was fehlt und was du in jedem Raid nutzen würdest:

- **Ideen:** das Feature-Requests-Forum auf [Discord](https://discord.gg/H4pRxGBGDb) oder ein [GitHub-Issue](https://github.com/MaxSala/rastro-tarkov/issues).
- **Bugs:** das Bug-Reports-Forum auf Discord oder ein GitHub-Issue. Gib deine Rastro-Version an (*Einstellungen → Über*) und was du gemacht hast. Bitte veröffentliche nie komplette Spiel-Logs, denn sie enthalten deine Konto-ID.
- **Sicherheitsprobleme:** nie öffentlich. Nutz die private Meldung auf GitHub (Tab *Security* → *Report a vulnerability*) oder ein privates *Security & privacy*-Ticket auf Discord. Details in [docs/SECURITY.md](docs/SECURITY.md).

## Danksagung

Rastro gäbe es nicht ohne **[tarkov.dev](https://tarkov.dev)**, die Open-Source-Datenbank zu Escape from Tarkov, gepflegt von der Community **[The Hideout](https://github.com/the-hideout)**. tarkov.dev begann als [tarkov-tools](https://github.com/kokarn/tarkov-tools) von **Oskar Risberg ([@kokarn](https://github.com/kokarn))** und wurde von den Mitwirkenden von The Hideout wiederbelebt und geöffnet. Rastro nutzt:

- **Karten:** die SVG-Karten aus [the-hideout/tarkov-dev-svg-maps](https://github.com/the-hideout/tarkov-dev-svg-maps) der tarkov.dev-Mitwirkenden, lizenziert unter [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/). Ihre Urheber erlauben keine Nutzung in Cheats oder Software, die einen unfairen Vorteil verschafft, etwa Radare. Die Karten lädst du selbst herunter, sie werden nie mit Rastro verteilt, und jede installierte Karte behält ihre Credits und Lizenz.
- **Daten:** Extraktionen, Übergänge, Schalter, Behälter, Schlösser, Gegenstände, Preise und Übersetzungen aus der tarkov.dev-API und ihren Datendateien.
- **Markierungssymbole:** aus [tarkov-dev](https://github.com/the-hideout/tarkov-dev) (`public/maps/interactive`), MIT-Lizenz, © 2019 Oskar Risberg.
- **Gegenstandssymbole** gehören Battlestate Games und werden nur lokal zur Anzeige zwischengespeichert.

Wenn dir Rastro nützt, unterstütze tarkov.dev, trag dazu bei oder tritt ihrer Community bei.

Rastro baut außerdem auf vielen anderen Open-Source-Projekten auf, darunter Rust, Tokio, Axum, Snow, Rustls, Wry, React, Leaflet, Tailwind CSS und die Schrift Geist. Die vollständige Liste mit Lizenzen steht in [NOTICE.md](NOTICE.md).

*Escape from Tarkov* ist eine Marke von Battlestate Games. Spielinhalte und -materialien sind Marken und urheberrechtlich geschütztes Eigentum von Battlestate Games und seinen Lizenzgebern.

## Nicht kommerziell

Rastro ist ein Hobbyprojekt für die Community. Es ist kostenlos, es gibt keine Bezahlversion, es zeigt keine Werbung und sammelt keine Daten. Das bleibt so, aus Überzeugung und weil die tarkov.dev-Karten, auf denen es aufbaut, nur für nicht-kommerzielle Nutzung lizenziert sind.

## Bekannte Einschränkungen

- Deine Position wird nur aktualisiert, wenn du einen Screenshot machst. Das ist Absicht: Nur dort schreibt das Spiel deine Position.
- Labs hat auf tarkov.dev keine SVG-Karte. Rastro erkennt Labs, kann es aber nur anzeigen, wenn du ein eigenes Kartenpaket importierst.
- Die Raid-Erkennung wurde mit PvE-Logs geprüft. PvP sollte genauso funktionieren, ist aber mit echten Logs noch nicht bestätigt.
- Die spanische und englische Übersetzung sind geprüft. Die anderen 14 hat noch kein Muttersprachler kontrolliert. Korrekturen sind sehr willkommen, hier oder auf [Discord](https://discord.gg/H4pRxGBGDb).
- Nach einem Wechsel der Sprache oder des Spielmodus (PvP/PvE) müssen Karten und Preise neu geladen werden, um die neuen Namen zu sehen.
- Einige technische Details in Fehlermeldungen und im Protokoll sind unabhängig von der Sprache auf Englisch.

## Aus dem Quellcode bauen

Das brauchst du nur, wenn du den Code ändern oder die `.exe` selbst bauen willst. Du brauchst [Rust](https://rustup.rs/) 1.85 oder neuer und [Node.js](https://nodejs.org/) LTS (unter Linux zusätzlich `libwebkit2gtk-4.1-dev` und `libgtk-3-dev`):

```sh
git clone https://github.com/MaxSala/rastro-tarkov.git
cd rastro-tarkov
./build.sh        # build.bat unter Windows
```

Das Ergebnis ist eine einzelne ausführbare Datei in `target/release/`. Entwicklungsumgebung, Projektaufbau, Code-Regeln und das Hinzufügen einer Übersetzung stehen in **[CONTRIBUTING.md](CONTRIBUTING.md)** (auf Englisch).

## Lizenz

Der Quellcode von Rastro steht unter der [MIT-Lizenz](LICENSE). Karten und Daten von tarkov.dev behalten ihre eigenen Lizenzen, aufgeführt unter [Danksagung](#danksagung) und in [NOTICE.md](NOTICE.md).

---

<p align="center">
  <b>Wenn dir Rastro geholfen hat, den Ausgang zu finden, <a href="https://github.com/MaxSala/rastro-tarkov/stargazers">gib ihm einen ⭐</a> und <a href="https://discord.gg/H4pRxGBGDb">sag Hallo auf Discord</a>.</b>
  <br>
  <sub>Von Spielern für Spieler. Wir sehen uns an der Extraktion.</sub>
</p>
