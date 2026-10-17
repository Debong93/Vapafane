# Redcraftserver

Minecraft-Serververwaltung für Linux und Windows – ein Bedienfenster, mit dem man
einen Minecraft-Server starten, stoppen und überwachen kann.

Geplant (Nachfolger des Python/GTK-Panels in `~/MC_Panel`):

- Übersicht mit Live-Status (Spieler, TPS, CPU, RAM)
- Konsole mit Live-Log und eigenen Befehlen (RCON)
- Spieler und Whitelist verwalten
- Server-Auswahl (immer nur einer aktiv), Import aus ZIP, neuer Fabric-Server
- Backups: ganzer Server-Ordner als ZIP mit Datum, Wiederherstellen
- Update-Assistent über Modrinth (Mods und Datapacks prüfen, Backup, Update, Rückweg)
- Autostart beim PC-Start (Linux: systemd, Windows: Aufgabenplanung)

Technik: Rust + [Tauri 2](https://tauri.app). Installer für Windows (`.exe`/`.msi`)
und Linux (AppImage/`.deb`) werden über GitHub Actions gebaut.
