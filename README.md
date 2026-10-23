# Vapafane – Minecraft-Werkbank für Server & Client

Vapafane ist ein Programm für Windows und Linux, mit dem du ganz ohne Befehle einen
eigenen Minecraft-Server betreibst: anlegen, starten, Mods hinzufügen, sichern und
aktualisieren. Deine Freunde bekommen von dir einen Einladungs-Code – Vapafane richtet
damit ihr Minecraft passend zum Server ein.

> **Hinweis:** Vapafane ist kein offizielles Minecraft-Produkt. Es ist nicht von Mojang oder
> Microsoft genehmigt und steht in keiner Verbindung zu ihnen.
> *NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.*

## Installieren

Die Dateien gibt es unter **Releases** (rechts auf dieser Seite).

**Windows:** `Vapafane_…_x64-setup.exe` herunterladen und starten. Erscheint
„Der Computer wurde durch Windows geschützt“, auf **„Weitere Informationen“** →
**„Trotzdem ausführen“** klicken (das Programm ist nicht signiert).

**Linux:** die `.deb`-Datei herunterladen und im Dateimanager **doppelklicken** – die
Software-Verwaltung installiert sie dann. Oder im Terminal (Datei liegt im Download-Ordner):

```bash
sudo apt install ~/Downloads/Vapafane_0.8.1_amd64.deb
```

Alternativ das `.AppImage` ausführbar machen (Rechtsklick → Eigenschaften →
„Als Programm ausführen“) und starten.

**Für einen Server** brauchst du außerdem Java (Minecraft 26.x braucht Java 25):
Windows z. B. [Temurin von adoptium.net](https://adoptium.net), Linux
`sudo apt install openjdk-25-jre-headless`. Zum Mitspielen bei Freunden ist Java nicht nötig.

## Erste Schritte

1. **Server anlegen:** oben auf **„+“** klicken, Typ wählen (Fabric, Paper, NeoForge oder
   Vanilla) und einen Namen eingeben – oder einen vorhandenen Server-Ordner hinzufügen.
2. **Starten:** in der **Übersicht** auf **„Starten“**. Das Fenster darf danach zu – der
   Server läuft im Hintergrund weiter. Mods findest du unter **Mods/Plugins → Mods hinzufügen**.
3. **Freunde einladen:** in der Übersicht auf **„Freunde einladen“** → **„Code erzeugen“** und
   den Code per Messenger schicken. Für Freunde außerhalb deines Heimnetzes zuerst
   „Öffentliche Adresse ermitteln“ und im Router den Server-Port freigeben.
4. **Client einrichten (bei deinen Freunden):** Vapafane installieren, oben auf **„Client“**
   umschalten, Code einfügen, **„Einladung prüfen“** → **„Einrichten“**. Danach mit
   **„Spielen“** den Minecraft-Launcher (oder wahlweise die Modrinth-App) öffnen.

Backups, Updates und Einstellungen findest du in den Reitern oben. Unter **Einstellungen →
Aussehen** wählst du zwischen „Grasblock“ (hell/dunkel) und „Crying Obsidian“. Server-Autostart beim
PC-Start ist anfangs aus und lässt sich unter **Einstellungen** einschalten.

## Kontakt

Fragen und Fehler bitte als [GitHub-Issue](https://github.com/Debong93/Vapafane/issues) melden.

## Technisches (kurz)

- Rust + [Tauri 2](https://tauri.app), Oberfläche in HTML/CSS/JS. Die Pixel-Texturen sind
  eigene Pixel-Art (`assets/make_textures.py`), keine Dateien aus Minecraft.
- Ein Programm, drei Rollen: Fenster, Hintergrund-Dienst (`--service`, unter Linux als
  systemd-Benutzerdienst) und Befehlszeile (`--ctl help`).
- Mods kommen von Modrinth; CurseForge ist optional mit eigenem API-Schlüssel.
- Liest die Daten des alten Panels weiter (`~/.config/mc-server-panel/config.json`).
- Selbst bauen: `cargo install tauri-cli --locked`, dann `cd src-tauri && cargo tauri build`.
  Testmodus ohne echte Daten: `VAPAFANE_HOME=/tmp/test vapafane`.
- Ein Release entsteht beim Push eines Tags `v…` (GitHub Actions).
