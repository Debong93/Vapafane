# e2e – Oberflächen-Tests

Ein Rundgang durch alle Funktionen von Vapafane: Der Test startet das echte Programm in einem
unsichtbaren Bildschirm (Xvfb) und bedient es über [tauri-driver](https://v2.tauri.app/develop/tests/webdriver/)
wie ein Mensch – Server anlegen, starten, Backups, Einladung, Client einrichten, Einstellungen
übernehmen, Sprachwechsel, Themen. Alle Daten landen in einem eigenen, leeren Ordner; echte
Server und Minecraft-Installationen bleiben unberührt. Der Rundgang läuft fest auf Deutsch
(`VAPAFANE_LANG=de`), der Sprachwechsel-Schritt prüft Englisch und zurück.

Nur unter Linux. Braucht `xvfb`, `webkit2gtk-driver`, `tauri-driver` (`cargo install tauri-driver --locked`),
Java und Internet.

```bash
cd ../src-tauri && cargo build          # Programm bauen (Oberfläche vorher mit trunk)
cd ../e2e && cargo test --test rundgang -- --ignored --nocapture
```

Bildschirmfotos und Protokolle liegen danach im Testordner (`VAPAFANE_E2E_HOME`, sonst im Temp-Ordner).
