# Vapafane – das Minecraft-Servertool

> **Hinweis:** Vapafane ist kein offizielles Minecraft-Produkt. Es ist nicht von Mojang oder
> Microsoft genehmigt und steht in keiner Verbindung zu ihnen.
> *NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.*
>
> Kontakt, Fragen und Fehlermeldungen: [GitHub-Issues](https://github.com/Debong93/Vapafane/issues)

**Va**nilla, **Pa**per, **Fa**bric, **Ne**oForge: Minecraft-Serververwaltung für Linux und Windows – ein Fenster, mit dem man einen
Minecraft-Server startet, stoppt, überwacht, sichert und aktualisiert.
Nachfolger des Python/GTK-Panels aus `~/MC_Panel`.

## Was es kann

- **Übersicht:** Statusampel (läuft / startet / stoppt / abgestürzt / gestoppt), Laufzeit,
  Knöpfe Start, Stopp, Neustart und Backup. Sind Spieler online, gibt es beim Stoppen
  einen Countdown mit Warnung im Chat. Karten für Spieler, TPS, CPU und RAM (mit
  kleinen Live-Kurven), Ordnergröße, Adresse im Heimnetz, letztes Backup, Version und Mods.
- **Konsole:** das Server-Log live, farbig (Warnungen, Fehler, Beitritte, Chat),
  Befehle eingeben (mit Verlauf über ↑/↓) und Schnellknöpfe.
- **Spieler:** wer online ist (Spielmodus, OP, rauswerfen) und die Whitelist.
- **Mods/Plugins:** Liste aller Mods (bzw. Paper-Plugins) und Datapacks mit An/Aus-Schalter. Ausgeschaltet heißt
  umbenannt in `….jar.disabled` (wie bei der Modrinth-App) – die Datei bleibt da, wird
  aber nicht geladen. Ausgeschaltete stehen ausgegraut mit Grund in der Liste
  (z. B. „gibt es nicht für Minecraft 26.3“). Knopf **Aufräumen** zeigt Reste alter
  Versionen (z. B. `mods/.old`, alte `versions/`, ältere Bibliotheken, alte Fabric-
  Zwischenspeicher) mit Größe an und löscht sie nach Bestätigung (nur bei gestopptem Server).
- **Mods hinzufügen:** Suche auf Modrinth direkt im Fenster, passend zu Typ und Version
  des aktiven Servers (Mods bei Fabric/NeoForge, Plugins bei Paper, Datapacks bei allen).
  Reine Client-Mods werden ausgeblendet, schon installierte markiert. „Installieren“ lädt
  die neueste passende Version samt Pflicht-Abhängigkeiten (mit SHA512-Prüfung).
- **CurseForge (optional, experimentell):** Mit einem eigenen, persönlichen CurseForge-API-Schlüssel (Einstellungen,
  dort steht Schritt für Schritt, wie man ihn beantragt) sucht Vapafane zusätzlich auf
  CurseForge. Modrinth hat immer Vorrang; angezeigt werden nur Projekte, die es auf
  Modrinth nicht gibt. Erlaubt ein Autor keine Downloads über andere Programme, gibt es
  den Knopf „Im Browser herunterladen“. Der Update-Assistent erkennt CurseForge-Mods über
  deren Fingerprint. Der Schlüssel liegt nur lokal in einer Datei, die nur du lesen kannst.
  Er ist persönlich und darf nicht weitergegeben werden; es gelten die
  [CurseForge-Bedingungen für Drittprogramme](https://support.curseforge.com/en/support/solutions/articles/9000207405-curse-forge-3rd-party-api-terms-and-conditions).
- **Mehrere Server:** Liste mit beliebig vielen Servern, es läuft immer nur einer.
  Hinzufügen per Ordner, Import aus ZIP/tar oder neu erstellen als **Fabric**, **Paper**,
  **NeoForge** oder **Vanilla** – wahlweise in `MC_Servers/<Name>` oder einem anderen Ordner.
  Server-Dateien werden immer mit Prüfsumme geladen (Mojang, PaperMC, NeoForged, Fabric).
- **Backups:** immer der komplette Server-Ordner als ZIP mit Datum im Namen
  (`<Backups>/<Name>/<Name>_JJJJ-MM-TT_HH-MM.zip`), auch während der Server läuft.
  Wiederherstellen behält den alten Stand als `…_vor_Wiederherstellung_<Datum>`.
- **Updates (Fabric, Paper, NeoForge, Vanilla):** prüft über Modrinth, ob es alle Mods
  bzw. Paper-Plugins und Datapacks für die
  gewünschte Version gibt, lädt nötige Abhängigkeiten nach, erkennt doppelte Datapacks,
  prüft Java und Speicherplatz. Das Update macht vorher ein Backup und bietet bei
  Problemen den Rückweg an. Mods ohne passende Version werden ausgeschaltet (und beim
  nächsten Update automatisch wieder eingeschaltet, sobald es eine passende gibt). Erst
  wenn der Server mit der neuen Version fertig gestartet ist, werden die alten Dateien
  gelöscht – das ZIP-Backup davor bleibt die Sicherung. **Probelauf:** testet das Update vorher an einer Kopie auf
  einem anderen Port – der echte Server bleibt unberührt. Export der Mods als
  `.mrpack` für Freunde (Modrinth-App).
- **Automatik:** Server beim PC-Start starten (auch ohne Anmeldung), Fenster beim
  Anmelden öffnen, tägliches Backup um 5:00 Uhr, Benachrichtigungen.

## Installieren

Die Installer gibt es unter **Releases** auf GitHub.

**Windows:** `Vapafane_…_x64-setup.exe` herunterladen und starten. Weil das
Programm nicht signiert ist, zeigt Windows evtl. „Der Computer wurde durch Windows
geschützt“ → „Weitere Informationen“ → „Trotzdem ausführen“.
Java installieren, z. B. [Temurin von adoptium.net](https://adoptium.net)
(Minecraft 26.x braucht Java 25).

**Linux:** das `.AppImage` herunterladen, ausführbar machen (Rechtsklick →
Eigenschaften → „Als Programm ausführen“) und starten – oder das `.deb` installieren:

```bash
sudo apt install ./Vapafane_0.4.1_amd64.deb
```

Java: `sudo apt install openjdk-25-jre-headless`

## So funktioniert es

Eine Programmdatei mit drei Rollen:

| Aufruf | Rolle |
|---|---|
| `vapafane` | das Fenster |
| `vapafane --service` | Hintergrund-Dienst: startet den aktiven Server, merkt Abstürze und startet ihn neu, macht das tägliche Backup |
| `vapafane --ctl …` | Befehlszeile, z. B. `--ctl status`, `--ctl backup`, `--ctl stop 30` (Liste: `--ctl help`) |

Das Fenster zu schließen stoppt den Server **nicht** – er läuft im Hintergrund-Dienst weiter.
Gestoppt wird immer mit dem Befehl `stop` (die Welt wird gespeichert); erst wenn der
Server nach 3 Minuten nicht reagiert, wird er hart beendet.

- **Linux:** Der Dienst läuft als systemd-Benutzerdienst `vapafane.service`.
  „Beim PC-Start starten“ schaltet zusätzlich `vapafane-autostart.service` ein.
  Damit das auch ohne Anmeldung klappt, fragt das Fenster einmal nach
  `loginctl enable-linger`.
- **Windows:** Der Dienst läuft als Hintergrundprozess. „Beim PC-Start starten“ legt
  in der Aufgabenplanung die Aufgabe „Vapafane“ an (Windows fragt dabei nach
  Administratorrechten). Hinweis: Beim Herunterfahren von Windows kann der Server
  nicht immer sauber gespeichert werden – vorher im Fenster stoppen ist sicherer.

## Umstieg von Redcraftserver (bis 0.3.0)

Vapafane hieß bis Version 0.3.0 „Redcraftserver“. Beim ersten Start übernimmt Vapafane
alles automatisch: Ein noch laufender alter Hintergrund-Dienst stoppt den Server sauber
(Welt wird gespeichert) und beendet sich, danach startet Vapafane den Server wieder. Die
systemd-Units heißen jetzt `vapafane.service` und `vapafane-autostart.service` (der
An/Aus-Zustand wird übernommen), der Login-Autostart `vapafane.desktop`. Die
Konfiguration und alle Server bleiben, wie sie sind.

```bash
sudo apt install ./Vapafane_0.4.1_amd64.deb
sudo apt remove redcraftserver
```

## Umstieg vom alten Panel (`~/MC_Panel`)

Vapafane liest dieselben Daten weiter:
`~/.config/mc-server-panel/config.json` (Server-Liste, Backup-Ordner, „Aufheben: N“),
`~/MC_Active`, die `.mcpanel.env` im Server-Ordner (wird nur gelesen, nie ausgeführt)
und die Backups in `~/MC_Backups`.

Solange der alte Dienst `minecraft.service` läuft, zeigt Vapafane den Server als
„läuft außerhalb“ an (Konsole, Spieler und Stoppen gehen trotzdem). Unter
**Einstellungen → Altes Panel abschalten** wird der alte Dienst sauber gestoppt und
beim Start ausgeschaltet; danach übernimmt Vapafane. Die Dateien des alten
Panels bleiben liegen.

## Selbst bauen

Voraussetzungen: Rust, unter Linux `libwebkit2gtk-4.1-dev` und Co.
([Liste](https://tauri.app/start/prerequisites/)), dazu

```bash
cargo install tauri-cli --locked
```

```bash
cd src-tauri && cargo tauri build
```

Zum Ausprobieren ohne die echten Daten anzufassen gibt es einen Testmodus:
`VAPAFANE_HOME=/tmp/test vapafane` legt Konfiguration, Server und Backups
dort an und benutzt kein systemd.

Ein neuer Release entsteht, wenn ein Tag `v…` gepusht wird (GitHub Actions baut dann
die Installer für Windows und Linux).

Technik: Rust + [Tauri 2](https://tauri.app), Oberfläche in schlichtem HTML/CSS/JS.
Die Gras- und Erd-Texturen sind eigene Pixel-Grafiken (`assets/make_textures.py`), keine Originaldateien aus Minecraft.
