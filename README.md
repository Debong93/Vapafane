# Vapafane – Minecraft-Werkbank für Server & Client

Vapafane ist ein Programm für Windows und Linux, mit dem du ganz ohne Befehle einen
eigenen Minecraft-Server betreibst: anlegen, starten, Mods hinzufügen, sichern und
aktualisieren. Deine Freunde bekommen von dir einen Einladungs-Code – Vapafane richtet
damit ihr Minecraft passend zum Server ein.

> **Hinweis:** Vapafane ist kein offizielles Minecraft-Produkt. Es ist nicht von Mojang oder
> Microsoft genehmigt und steht in keiner Verbindung zu ihnen.
> *NOT AN OFFICIAL MINECRAFT PRODUCT. NOT APPROVED BY OR ASSOCIATED WITH MOJANG OR MICROSOFT.*

## Installieren

Die Dateien gibt es unter **[neueste Version](../../releases/latest)**
(rechts auf dieser Seite unter „Releases“).

**Windows:** `Vapafane_…_x64-setup.exe` herunterladen und starten. Erscheint
„Der Computer wurde durch Windows geschützt“, auf **„Weitere Informationen“** →
**„Trotzdem ausführen“** klicken (das Programm ist nicht signiert).

**Linux (Ubuntu, Debian, Mint …):** direkt im Terminal – lädt immer die neueste Version
von GitHub und installiert sie (auch zum Aktualisieren):

```bash
wget -O /tmp/vapafane.deb https://github.com/Debong93/Vapafane/releases/latest/download/Vapafane_amd64.deb && sudo apt install /tmp/vapafane.deb
```

Oder die `.deb`-Datei unter Releases herunterladen und im Dateimanager **doppelklicken** –
die Software-Verwaltung installiert sie dann.

**Nur wer selbst einen Server betreibt,** braucht zusätzlich Java (Minecraft 26.x braucht
Java 25): Windows z. B. [Temurin von adoptium.net](https://adoptium.net), Linux
`sudo apt install openjdk-25-jre-headless`. Zum Mitspielen bei Freunden ist kein Java nötig.

## So benutzt du Vapafane

Oben im Fenster wählst du zwischen **Client** (mitspielen) und **Server** (selbst einen
Server betreiben). Vapafane startet immer im Client-Bereich. Das **Zahnrad** oben rechts
öffnet die Einstellungen des jeweiligen Bereichs, der Knopf daneben wechselt zwischen hell
und dunkel.

### Eigenen Server erstellen

1. Oben auf **„Server“** umschalten.
2. Auf **„+“** klicken (es leuchtet, solange es noch keinen Server gibt) und
   **„Neuen Server erstellen“** wählen. Typ aussuchen – *Vanilla* (ohne Mods),
   *Fabric* oder *NeoForge* (mit Mods), *Paper* (mit Plugins) –, Minecraft-Version und
   Namen angeben und die Minecraft-EULA bestätigen. Einen vorhandenen Server-Ordner oder
   ein Backup fügst du über dasselbe Menü hinzu.
3. In der **Übersicht** auf **„▶ Starten“** klicken. Das Fenster darf danach zu – der
   Server läuft im Hintergrund weiter.
4. Links findest du alles Weitere: **Konsole**, **Spieler** (Whitelist, OP),
   **Mods/Plugins** („Mods hinzufügen …“ sucht auf Modrinth), **Version & Updates**,
   **Backups** und **Erreichbarkeit**.

### Freunde einladen

1. In der Übersicht auf **„Freunde einladen“** klicken.
2. Für Freunde im selben WLAN passt die vorgeschlagene Adresse. Für Freunde von außen
   **„Öffentliche Adresse ermitteln“** klicken und im Router den Server-Port freigeben
   (Anleitung für alle Router: [portforward.com](https://portforward.com/minecraft-java-edition-server/)).
3. **„Code erzeugen“** und den Code z. B. per Messenger schicken.

Unter **Erreichbarkeit** zeigt eine Ampel, ob dein Server von außen erreichbar ist –
grün über IPv4 und IPv6, gelb nur über IPv6 (z. B. bei DS-Lite), rot gar nicht.

### Mit dem Client einem Server beitreten

1. Vapafane installieren und im Bereich **„Client“** bleiben.
2. Den Einladungs-Code einfügen und **„Einladung prüfen“** klicken (oder eine
   `.vapafane`-Datei öffnen). Vapafane zeigt Server, Version und Mods.
3. Wählen, wofür eingerichtet wird – **Minecraft-Launcher** oder **Modrinth-App** – und
   **„Einrichten“** klicken. Vapafane lädt die passenden Mods und legt ein Profil an.
4. Unter **„Meine Server“** mit **„Spielen“** den Launcher öffnen. Ob der Server gerade
   online ist, siehst du direkt daneben.

Vapafane startet Minecraft nicht selbst und fasst kein Microsoft-Konto an.

## Kontakt

Fragen und Fehler bitte als [GitHub-Issue](../../issues) melden.

## Aufbau (für Entwickler)

Komplett in Rust; jede Komponente hat eine eigene kurze README:

- [`src-tauri/`](src-tauri/README.md) – das Programm: Fenster, Hintergrund-Dienst, Befehlszeile
- [`frontend/`](frontend/README.md) – die Oberfläche (Leptos/WebAssembly)
- [`shared/`](shared/README.md) – gemeinsame Datentypen
- [`werkzeuge/`](werkzeuge/README.md) – erzeugt die Pixel-Texturen

Selbst bauen: `rustup target add wasm32-unknown-unknown`, `cargo install tauri-cli trunk --locked`,
dann `cd src-tauri && cargo tauri build`. Testmodus ohne echte Daten: `VAPAFANE_HOME=/tmp/test vapafane`.
