# Changelog

All notable changes to Vapafane. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), version numbers follow
[Semantic Versioning](https://semver.org/).

## [0.11.1] – 2026-10-25

### New

- **Recommended performance mods.** When you create a Fabric or NeoForge server, Vapafane
  offers a small set of well-established performance mods, for example Lithium, FerriteCore
  and Krypton. They are pre-selected, run on the server only and don't change gameplay –
  friends don't need to install anything. Only mods with a finished release (no alpha or
  beta) for the chosen Minecraft version are suggested.
- In the mod search, these mods have their own **"Recommended"** section with a short
  explanation in your language and an "Install all recommended" button; in search results
  they carry a "Recommended" badge.

### Changed

- Release files are now built on GitHub from the tagged source code; `SHA256SUMS.txt` and
  the short link `Vapafane_amd64.deb` are created from exactly these files. (In 0.9.4 to
  0.11.0 the checksum file did not match the versioned `.deb` and `.exe`; it has been
  corrected.)

### Fixed

- Texts that still appeared in German when another language was set are now translated:
  the backup date, progress messages during backups, updates and client setup, the names of
  the sources when taking over settings, the cleanup list, the theme names, the description
  of exported modpacks, the "Save modpack" dialog, the title of a running mod installation,
  the reason for browser-only downloads, the "something is already running" note, some error
  messages and the command line output. The language test now catches such texts more
  reliably.
- A wrong RCON password is now recognized in every language – before, only in German, which
  led to a pointless second connection attempt.

## [0.11.0] – 2026-10-25

### New

- **Remove mods, plugins and datapacks for good** with the trash button – also switched-off
  ones and datapack folders. Vapafane warns if other mods still need the file, and offers to
  remove helper mods that nothing needs anymore afterwards.
- **A more informative mod list:** every entry shows its icon, a short description and a
  "Page ↗" button that opens the mod's page on Modrinth or CurseForge. Helper mods are marked
  as "Library" and show which mods need them. The information is looked up once (also for
  mods copied in by hand, via the file's fingerprint) and remembered in the server folder.
- Search field above long mod lists (e.g. modpacks).
- In the mod search, the title opens the mod's page – so you can read what it does first.
- Command line: `mod-entfernen PATH`.

### Changed

- Switching off a mod that others need asks first.
- Datapacks show their real name instead of the file name.
- Vanilla servers only show datapacks – no empty mod list anymore.
- Every remaining program message (errors, progress, command line, reasons for switched-off
  mods) is now translated into all 9 languages; reasons follow a language switch. A new test
  makes sure no fixed German text slips in again.

### Fixed

- The buttons at the bottom of large dialogs (e.g. "Close" in the mod search) no longer get
  cut off in small windows.
- The "(off)" label next to switched-off mods was always German.

## [0.10.0] – 2026-10-24

### New

- **9 languages:** Vapafane now speaks English and German (maintained by hand) plus
  French, Spanish, Italian, Dutch, Portuguese, Polish and Turkish (machine-translated,
  corrections welcome). More languages will follow in 0.10.1.
- On first start Vapafane follows your **system language** (English otherwise); the
  language can be changed in the settings of both areas.
- The README is now in English, with a German version ([README.de.md](README.de.md)).

### Changed

- Every message follows the chosen language: the whole interface, program and error
  messages, reachability reports, in-game countdown announcements and the command-line
  help – including number formats and units (decimal separator, thousands separator,
  s/min/h).
- The background service's log file is always English (it is diagnostic output).
- Issue templates are in English now; German reports stay welcome.

## [0.9.5] – 2026-10-24

Final polish for the release.

### New

- If Java is missing, starting a server now explains that in plain words and shows how to
  install it – instead of a technical error message.
- The start-menu entry lists keywords and the category "Games" (Linux).

### Changed

- **"Create a new server"** now comes first in the "+" menu.
- The suggested name for new servers is "My server" (previously "Neuer_Server").
- README: system requirements added.

## [0.9.4] – 2026-10-24

Combines the unpublished test versions 0.9.1 to 0.9.3.

### New

- **Take over settings:** When setting up a client, options, key bindings, mod settings,
  shaders, resource packs, the server list and, if wanted, worlds can be taken over from an
  existing installation – from a Modrinth App instance, the Minecraft Launcher, another
  Vapafane client or any folder. The source is only read.
- **Update a Modrinth instance:** After a server update, Vapafane builds the pack for
  "Reinstall modpack" in the Modrinth App and walks you through it step by step. Worlds,
  settings, shaders and mods you added yourself are kept.
- **"Shaders & extras" guide** with sketches of the Modrinth App: how to add shaders,
  resource packs and your own mods.
- **"Refresh" under "My servers"** loads the saved invite for setting up again – or
  explains that a new invite code is needed after a server update.
- **Whitelist hints** for beginners in overview, reachability, players and settings, with
  a "Turn on whitelist" button.
- Command line: `einloesen … --einstellungen-von FOLDER` and `--neue-instanz`.

### Changed

- The in-game server list is merged instead of overwritten during setup: your own entries
  stay, names with emojis are stored correctly.
- New servers get free ports (game and RCON) even while another server is running.
- Settings: before the first start the fields show Minecraft's default values. "About
  Vapafane" looks the same in the client and server area. Switching areas inside the
  settings keeps you in the settings.
- Client settings: the row is now called "Game folders of all clients".
- Checkboxes and radio buttons no longer keep a yellow outline after a mouse click.

### Fixed

- Starting with an occupied port now stops with a clear message instead of Minecraft
  bailing out with an error.
- Reachability reported "reachable" although the server was stopped while another program
  used the same port. It now says "port in use".
- Starting during a running reachability check could fail on the occupied port.
- A cancelled setup (e.g. without internet) blocked a second attempt with "folder not
  empty".
- Individual locked files (e.g. a currently open world) no longer abort the take-over.
- The "Minecraft Launcher not found" hint only appears when the launcher is the target.

## [0.9.0] – 2026-10-23

### Changed

- The interface is written entirely in Rust (Leptos/WebAssembly), as is the tool that
  generates the pixel textures.
- Errors are consistently handled as readable messages instead of terminating the program.

## [0.8.2] – 2026-10-23

### New

- Sidebar with groups, gear for the settings of both areas, client settings, quiet client
  watcher (every 20 seconds).
- Creeper at daytime, Enderman, sun and moon in the sky.

### Fixed

- The "+" menu was hidden behind the buttons.

## [0.8.1] – 2026-10-23

### New

- New program icon, sky with Enderman, "Crying Obsidian" theme with portal, animations on
  start and stop.

## [0.8.0] – 2026-10-21

### New

- Reachability traffic light: checks whether the server can be reached from outside via
  IPv4 and IPv6 (detects DS-Lite).
- Vapafane starts in the client area; invites also carry the IPv6 address.
- The time of the daily backup is configurable.

Downloads are available from version 0.9.0 on; older downloads have been removed.

[0.11.1]: ../../releases/tag/v0.11.1
[0.11.0]: ../../releases/tag/v0.11.0
[0.10.0]: ../../releases/tag/v0.10.0
[0.9.5]: ../../releases/tag/v0.9.5
[0.9.4]: ../../releases/tag/v0.9.4
[0.9.0]: ../../releases/tag/v0.9.0
