# Änderungsprotokoll

Alle nennenswerten Änderungen an Vapafane. Das Format folgt
[Keep a Changelog](https://keepachangelog.com/de/1.1.0/), die Versionsnummern folgen
[Semantic Versioning](https://semver.org/lang/de/).

## [0.9.4] – 2026-10-24

Fasst die nicht veröffentlichten Testversionen 0.9.1 bis 0.9.3 zusammen.

### Neu

- **Einstellungen übernehmen:** Beim Einrichten eines Clients lassen sich Optionen,
  Tastenbelegung, Mod-Einstellungen, Shader, Ressourcenpakete, Server-Liste und auf Wunsch
  Welten aus einer bisherigen Installation übernehmen – aus einer Instanz der Modrinth-App,
  dem Minecraft-Launcher, einem anderen Vapafane-Client oder einem beliebigen Ordner. Die
  Quelle wird nur gelesen.
- **Modrinth-Instanz aktualisieren:** Nach einem Server-Update baut Vapafane das Paket für
  „Modpack neu installieren“ in der Modrinth-App und zeigt Schritt für Schritt, wie es geht.
  Welten, Einstellungen, Shader und selbst hinzugefügte Mods bleiben erhalten.
- **Anleitung „Shader & Extras“** mit Skizzen der Modrinth-App: So kommen Shader,
  Ressourcenpakete und eigene Mods dazu.
- **„Aktualisieren“ unter „Meine Server“** lädt die gespeicherte Einladung zum erneuten
  Einrichten – oder erklärt, dass nach einem Server-Update ein neuer Einladungs-Code nötig ist.
- **Whitelist-Hinweise** für Einsteiger in Übersicht, Erreichbarkeit, Spieler und
  Einstellungen, mit Knopf „Whitelist einschalten“.
- Befehlszeile: `einloesen … --einstellungen-von ORDNER` und `--neue-instanz`.

### Geändert

- Die Server-Liste im Spiel wird beim Einrichten zusammengeführt statt überschrieben: Eigene
  Einträge bleiben, Namen mit Emojis werden richtig gespeichert.
- Neue Server bekommen freie Ports (Spiel und RCON), auch wenn schon ein anderer Server läuft.
- Einstellungen: Vor dem ersten Start zeigen die Felder die Minecraft-Standardwerte.
  „Über Vapafane“ sieht im Client- und Server-Bereich gleich aus. Wer in den Einstellungen
  den Bereich wechselt, bleibt in den Einstellungen.
- Client-Einstellungen: Die Zeile heißt jetzt „Spielordner aller Clients“.
- Häkchen und Auswahlknöpfe behalten nach einem Mausklick keinen gelben Rahmen mehr.

### Behoben

- Ein Start bei belegtem Port bricht jetzt mit einer klaren Meldung ab, statt dass
  Minecraft mit einem Fehler aussteigt.
- Die Erreichbarkeit meldete „erreichbar“, obwohl der Server gestoppt war, wenn ein anderes
  Programm denselben Port benutzte. Jetzt erscheint „Port belegt“.
- Ein Start während einer laufenden Erreichbarkeits-Prüfung konnte am belegten Port scheitern.
- Ein abgebrochenes Einrichten (z. B. ohne Internet) blockierte einen zweiten Versuch mit
  „Ordner nicht leer“.
- Einzelne gesperrte Dateien (z. B. eine gerade offene Welt) brechen das Übernehmen nicht ab.
- Der Hinweis „Minecraft-Launcher nicht gefunden“ erscheint nur noch, wenn der Launcher auch
  das Ziel ist.

## [0.9.0] – 2026-10-23

### Geändert

- Die Oberfläche ist komplett in Rust geschrieben (Leptos/WebAssembly), ebenso das
  Werkzeug für die Pixel-Texturen.
- Fehler werden durchgehend als verständliche Meldung behandelt statt das Programm zu beenden.

## [0.8.2] – 2026-10-23

### Neu

- Seitenleiste mit Gruppen, Zahnrad für die Einstellungen beider Bereiche,
  Client-Einstellungen, stiller Client-Wächter (alle 20 Sekunden).
- Creeper bei Tag, Enderman, Sonne und Mond im Himmel.

### Behoben

- Das „+“-Menü lag hinter den Knöpfen.

## [0.8.1] – 2026-10-23

### Neu

- Neues Programm-Symbol, Himmel mit Enderman, Thema „Crying Obsidian“ mit Portal,
  Animationen beim Starten und Stoppen.

## [0.8.0] – 2026-10-21

### Neu

- Erreichbarkeits-Ampel: prüft, ob der Server von außen über IPv4 und IPv6 erreichbar ist
  (erkennt DS-Lite).
- Vapafane startet im Client-Bereich; Einladungen enthalten auch die IPv6-Adresse.
- Uhrzeit für das tägliche Backup einstellbar.

Ältere Versionen stehen unter [Releases](../../releases).

[0.9.4]: ../../releases/tag/v0.9.4
[0.9.0]: ../../releases/tag/v0.9.0
[0.8.2]: ../../releases/tag/v0.8.2
[0.8.1]: ../../releases/tag/v0.8.1
[0.8.0]: ../../releases/tag/v0.8.0
