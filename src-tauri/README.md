# src-tauri – das Vapafane-Programm

Rust-Backend und Programmrahmen (Tauri 2). Eine Programmdatei mit drei Rollen:

- **Fenster** (ohne Argumente) – lädt die Oberfläche aus `../frontend/dist` und stellt ihr die Befehle bereit (`gui.rs`, `manage.rs`, `clients_gui.rs`, `updates_gui.rs`).
- **Hintergrund-Dienst** (`--service`) – startet und überwacht den Minecraft-Server, startet nach Absturz neu, macht das tägliche Backup (`service.rs`).
- **Befehlszeile** (`--ctl <befehl>`, Übersicht mit `--ctl help`) – für Skripte und Tests (`ctl.rs`).

Daneben: Server anlegen und aktualisieren (`flavor.rs`, `updater.rs`), Mods (`mods.rs`, `catalog.rs`, `curseforge.rs`), Backups (`archive.rs`), Einladungen und Client-Einrichtung (`invite.rs`, `client.rs`), Einstellungen aus einer anderen Minecraft-Installation übernehmen (`transfer.rs`, Server-Liste `nbt.rs`), Erreichbarkeit (`reach.rs`).

```bash
cargo test
VAPAFANE_HOME=/tmp/test cargo run      # Testmodus ohne echte Daten
```

Für Veröffentlichungen so bauen, dass keine Pfade des eigenen Rechners (Benutzername) im Programm landen:

```bash
RUSTFLAGS="--remap-path-prefix=$HOME=/build" cargo tauri build --bundles deb
RUSTFLAGS="--remap-path-prefix=$HOME=/build" cargo tauri build --target x86_64-pc-windows-gnu --bundles nsis
```
