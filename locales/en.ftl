# Vapafane – English (maintained by hand)
# One line per text: key = text. Placeholders: { $name }

## Language
sprache-titel = Language
sprache-system = System language (recommended)
sprache-ki-hinweis = This translation was machine-generated (by AI) – please report mistakes via "About Vapafane".
sprache-neustart = The interface briefly reloads when switching.

## Java
java-hinweis-windows = Install it e.g. from https://adoptium.net (Temurin { $version }).
java-hinweis-linux = Install it with: sudo apt install openjdk-{ $version }-jre-headless
err-java-fehlt = Java is not installed on this PC – no Minecraft server can start without it. This server needs Java { $version }. { $hinweis }
err-java-noetig = Minecraft { $mc } needs Java { $version }, which is not installed. { $hinweis }

## Common buttons
gem-ok = OK
gem-schliessen = Close
gem-abbrechen = Cancel
gem-weiter = Continue
gem-entfernen = Remove
gem-speichern = Save
task-abgebrochen = Cancelled

## Header
hdr-untertitel = Minecraft workbench for server & client
hdr-bereich = Area
hdr-client = Client
hdr-server = Server
hdr-aktiver-server = Active server (only one runs at a time)
hdr-kein-server = (no server)
hdr-plus-erster = Create your first server here
hdr-plus = Add a server
hdr-menu-neu = Create a new server (Fabric, Paper, NeoForge, Vanilla) …
hdr-menu-ordner = Add an existing folder …
hdr-menu-import = Import a server from ZIP/backup …
hdr-menu-entfernen = Remove the selected server from the list
hdr-einstellungen = Settings
hdr-hell-dunkel = Toggle light/dark
hdr-ev-gewechselt = Switched server: { $name }
hdr-err-umschalten = Could not switch

## Sidebar
side-bereiche = Sections
side-betrieb = Operation
side-uebersicht = Overview
side-konsole = Console
side-spieler = Players
side-inhalte = Content
side-mods = Mods/Plugins
side-updates = Version & updates
side-backups = Backups
side-netzwerk = Network
side-erreichbarkeit = Reachability

## Welcome
willkommen-titel = Welcome to Vapafane
willkommen-text = Vapafane is not an official Minecraft product. It is not approved by or associated with Mojang or Microsoft.
    
    Questions, ideas or bugs: via the project's GitHub page (Settings → About Vapafane).

## Common
gem-ordner = Folder
gem-ordner-oeffnen = Open folder
gem-launcher = Official Minecraft Launcher
gem-modrinth-app = Modrinth App

## About Vapafane
about-titel = About Vapafane
about-version = Version { $version }
about-testmodus = TEST MODE
about-nicht-offiziell = Vapafane is not an official Minecraft product. It is not approved by or associated with Mojang or Microsoft.
about-marken = Minecraft is a trademark of Mojang/Microsoft. Mods and plugins come from their respective authors (via Modrinth or CurseForge).
about-kontakt = Contact & report bugs (GitHub) ↗
about-projektseite = Project page ↗

## Appearance
skin-titel = Appearance
skin-grass-sub = Light or dark (button in the top right)
skin-obsidian-sub = Always dark

## Client settings
cset-einstellung = Setting
cset-einrichten = Setup
cset-ziel = Default target
cset-ziel-sub = Where new servers get set up
cset-spielordner = Game folders of all clients
cset-spielordner-sub = Mods, options and worlds per server live here

## Backups
bak-restore-frage = Reset “{ $name }” to { $datei }?
bak-restore-text = The server will be stopped for this. The current state is not deleted – it stays next to it as a folder named “…_vor_Wiederherstellung_<date>”.
bak-wiederherstellen = Restore
bak-ev-wiederhergestellt = Backup restored
bak-restore-fertig-titel = Restored
bak-restore-fertig = The server is back at the state of { $label }. You can start it now.
bak-restore-daneben = The previous state is in:
    { $ordner }
    (can be deleted once everything is fine)
bak-err-restore = Restore failed
bak-waehlen = Choose a backup
bak-erklaerung = Each backup is the complete server folder as a ZIP, with the date in its name. Stored in { $ordner }. The newest { $anzahl } are kept, older ones are deleted.
bak-jetzt = Back up now
bak-ordner = Open backup folder
bak-andere = Restore another ZIP …
bak-aufheben = Keep:
bak-leer = No backups for this server yet.

## Reachability
rch-fehlgeschlagen = Check failed
rch-noch-nicht = Not checked yet
rch-kein-server = No server selected
rch-kein-server-text = Create a server with “+” at the top or select one, then it gets checked.
rch-prueft = Checking …
rch-geprueft-um = Checked at { $zeit }
rch-ip-erklaerung = IPv4 is the classic internet address, IPv6 the new one with its own address per device. With DS-Lite your provider shares the IPv4 address – then only IPv6 works.
rch-erneut = Check again
rch-portfreigabe = Port forwarding guide ↗
rch-dienst-hinweis = To check, Vapafane asks a service on the internet (ifconfig.co) whether the server port is open from outside. Turn off: Settings → Automation.

## Invite friends
inv-titel = Invite friends
inv-erklaerung = Send your friends a code – Vapafane uses it to set up their Minecraft with the matching mods.
inv-adresse-label = Address your friends use to reach the server
inv-oeffentlich = Find public address
inv-adresse-hinweis = Prefilled is the home network address (same Wi-Fi only). For friends outside: find the public address and forward the server port in your router.
inv-code-erzeugen = Create code
inv-als-datei = Save as file …
inv-kein-passwort = The code contains no passwords – only address, version and mods.
inv-datei-gespeichert = Saved:
inv-datei-schicken = send this file to your friends.
inv-code-label = Invite code ({ $zeichen } characters) – copy and send it e.g. via messenger:
inv-kopieren = Copy
inv-kopiert = Copied
inv-mods = { $anzahl } mods for the players
inv-mods-optional = ({ $anzahl } of them optional)
inv-keine-mods = No mods needed – just version { $version }.
inv-ipv6 = Also contains your IPv6 address { $adresse }.
inv-nur-ipv6 = – according to the last check your server is only reachable via IPv6.
inv-nur-server = Server-only (players don't need them): { $mods }
inv-datei-noetig = These mods are neither on Modrinth nor on CurseForge: { $mods }. They don't fit into the code – please use “Save as file” instead, then they are packed in.

## Players & whitelist
spi-online-titel = Online right now
spi-wl-titel = Whitelist (who may join)
spi-wl-erklaerung = With the whitelist on (Settings → “Whitelist only”), only the players listed here can join the server. That keeps strangers out – add yourself too.
spi-name-platzhalter = Minecraft name
spi-hinzufuegen = Add
spi-name-ungueltig = Please enter a valid Minecraft name (3–16 characters).
spi-nur-laufend = Adding only works while the server is running (it verifies the name with Mojang).
spi-wl-leer = The whitelist is empty.
spi-niemand = Nobody online
spi-kreativ = Creative
spi-ueberleben = Survival
spi-op = Make OP
spi-op-weg = Revoke OP
spi-kick = Kick
spi-kick-frage = Kick { $name }?
spi-kick-text = They can rejoin immediately.
spi-err-befehl = Command failed
wl-titel = Whitelist
wl-einschalten = Turn on whitelist
wl-zu-spielern = Go to Players
wl-warnung-offen = Warning: your server is reachable from the internet and the whitelist is off – anyone who knows or stumbles upon the address can join. With the whitelist, only players you add can get in.
wl-warnung-leer = The whitelist is on but still empty – right now nobody can join the server, not even you. Add your Minecraft name and your friends' names under “Players”.
wl-tipp = Tip: the whitelist is off – anyone with the address can join. It's recommended as soon as friends play from outside: then only players listed here can get in.

## Console
kon-neues-log = new log file (server was restarted)
kon-beigetreten = { $name } joined
kon-verlassen = { $name } left the game
kon-bereit = Server ready (started in { $sekunden } s)
kon-fehler = Error: { $fehler }
kon-tag = Day
kon-nacht = Night
kon-wetter = Clear weather
kon-wer-online = Who is online?
kon-platzhalter = Enter a command, e.g. say Hello  ·  time set day  ·  weather clear
kon-senden = Send
kon-log-ordner = Open log folder

## Overview
ueb-laeuft = Running
ueb-startet = Starting …
ueb-stoppt = Stopping …
ueb-neustart-absturz = Restarting after crash …
ueb-gestoppt = Stopped
ueb-abgestuerzt = Crashed
ueb-extern = Running outside Vapafane
ueb-anderer = Another server is running
ueb-starten = Start
ueb-stoppen = Stop
ueb-neustart-knopf = Restart
ueb-seit = for { $dauer }
ueb-neustarts = restarted automatically { $anzahl }×
ueb-hinweis-extern = The server was not started via Vapafane (e.g. old panel or terminal). Console and players still work; “Stop” sends it the stop command.
ueb-hinweis-anderer = The background service is currently running another server. Please stop that one first.
ueb-hinweis-absturz = The server crashed and was not restarted. Check the console for the last error message.
ueb-hinweis-kein-server = Add a server with “＋” at the top.
ueb-ev-bereit = Server is ready
ueb-ev-gestoppt = Server stopped
ueb-ev-startet = Server is starting
ueb-ev-neustart = Server is restarting
ueb-ev-stoppt = Server is stopping
ueb-ev-in-sek = in { $sekunden } s
ueb-ev-absturz = Server crashed
ueb-ev-absturz-neustart = Server crashed – restarting automatically
ueb-notify-absturz = Minecraft server crashed
ueb-notify-absturz-neustart = It is being restarted automatically.
ueb-ev-backup-start = Backup started
ueb-ev-backup-fertig = Backup done: { $datei } ({ $groesse })
ueb-err-start = Start failed
ueb-err-stop = Stopping failed
ueb-err-backup = Backup failed
ueb-eula-titel = Confirm EULA
ueb-eula-text = No server starts without agreeing to the Minecraft EULA.
ueb-eula-akzeptiere = I accept the
ueb-eula-fuer = for “{ $name }”.
ueb-verb-stoppen = stop
ueb-verb-neustart = restart
ueb-stop-frage = { $verb } the server?
ueb-stop-extern-text = The server runs outside Vapafane. I'll send it the “stop” command – it saves the world and shuts down.
ueb-stop-niemand = Nobody is online right now.
ueb-stop-spieler = { $anzahl } players online: { $namen }
ueb-sofort = Now
ueb-mit-warnung = In { $sekunden } s with warning
ueb-karte-spieler = Players online
ueb-karte-tps = TPS (ticks per second)
ueb-karte-ram = Memory
ueb-karte-ordner = Server folder
ueb-karte-aussen = From outside
ueb-karte-adresse = Address (home network)
ueb-karte-backup = Last backup
ueb-karte-version = Version
ueb-server-aus = server off
ueb-keine-verbindung = no connection
ueb-niemand = nobody
ueb-ms-pro-tick = { $ms } ms per tick
ueb-ruckelt = stuttering
ueb-fluessig = smooth
ueb-von-kernen = of { $anzahl } cores
ueb-ram-belegt-limit = used (limit { $limit } GB)
ueb-ram-belegt = used
ueb-ram-limit = limit { $limit } GB
ueb-frei = { $groesse } free
ueb-vor = { $dauer } ago
ueb-noch-keins = none yet
ueb-backup-knopf = press the “Backup” button
ueb-backup-sub = { $groesse } · { $anzahl } stored
ueb-erreichbar = Reachable
ueb-nur-ipv6 = IPv6 only
ueb-nicht-erreichbar = Not reachable
ueb-ipv4-und-6 = IPv4 and IPv6
ueb-ueber-ipv4 = via IPv4
ueb-portfreigabe = check port forwarding
ueb-ereignisse = Recent events
ueb-nichts-passiert = Nothing happened yet.
ueb-fenster-zu = Closing the window does not stop the server – it keeps running in the background.
ueb-kontakt = Contact & report bugs (GitHub)

## Settings (server)
set-spiel = Game (this server)
set-motd = Server name (MOTD)
set-max-spieler = Max players
set-schwierigkeit = Difficulty
set-wert-peaceful = Peaceful
set-wert-easy = Easy
set-wert-normal = Normal
set-wert-hard = Hard
set-spielmodus = Game mode for new players
set-wert-survival = Survival
set-wert-creative = Creative
set-wert-adventure = Adventure
set-wert-spectator = Spectator
set-sichtweite = View distance (chunks)
set-simulationsweite = Simulation distance (chunks)
set-spawn-schutz = Spawn protection (blocks)
set-port = Server port
set-whitelist = Whitelist only
set-whitelist-hilfe = Recommended for safety: only players on the whitelist (Players page) can join. Without it, anyone who knows your address can join.
set-leistung = Performance (this server)
set-ram = Max memory for the server (GB)
set-ram-hinweis = This PC has { $gb } GB. Leave about 3–4 GB for the system.
set-speichern = Save settings
set-gespeichert = Saved.
set-gespeichert-neustart = Saved. Takes effect on the next restart.
set-err-speichern = Saving failed
set-automatik = Automation
set-autostart = Start the active server automatically at boot
set-auto-testmodus = Disabled in test mode.
set-auto-kein-systemd = Not available on this system (no systemd).
set-auto-windows = Starts at boot, even without signing in (Windows asks for administrator rights when enabling).
set-auto-linger-fehlt = Note: it only starts once you sign in. Click here to allow starting without signing in.
set-auto-aktiv = Starts even without signing in and restarts itself after a crash.
set-linger-frage = Allow starting without signing in?
set-linger-text = For this, systemd must be allowed to start your user services at boot (“loginctl enable-linger”). After that the server also runs when nobody is signed in.
set-linger-erlauben = Allow
set-linger-abgelehnt = Not allowed
set-fenster-autostart = Open this window at sign-in
set-backup-um = Daily backup at 
set-backup-uhr =  (whole server as ZIP)
set-backup-sub = Done by the background service. Catches up if the PC was off at that time.
set-uhrzeit = Time for the daily backup
set-reach = Check reachability from outside
set-reach-sub = When switching to the server area. Asks a checking service on the internet.
set-notify = Notifications
set-notify-sub = When someone joins or leaves and when the server crashes (while the window is open). Silent.
set-ordner = Folders
set-ordner-server = Server folder
set-an = on
set-aus = off
set-panel-titel = Old control panel still set up.
set-panel-status = Service “minecraft.service”: { $dienst }, at boot { $start }; daily backup timer { $backup }. While it runs, Vapafane cannot start the server itself (same port), and backups would be made twice.
set-panel-knopf = Disable old panel …
set-panel-frage = Disable the old panel?
set-panel-text = I'll stop the “minecraft.service” service (the server saves cleanly and shuts down) and disable it and its daily backup at boot.
    
    The old panel itself and its files remain – it can be re-enabled at any time. Afterwards you start the server here with “Start”.
set-panel-abschalten = Disable
set-panel-aus = Old panel disabled
set-panel-err = Disabling failed

## CurseForge
cf-titel = CurseForge (optional)
cf-experimentell = Experimental
cf-brauche = Do I need this?
cf-erklaerung = No – most mods are on Modrinth, which works without setup. CurseForge additionally finds mods that only exist there and needs a personal API key for that (a kind of access code).
cf-anleitung-titel = How to get a key (free)
cf-schritt-1 = Click “Request a key” and fill in the form (e.g. purpose: “Mods for my own server”).
cf-schritt-2 = Wait for the reply by email – CurseForge reviews by hand, which can take a few days.
cf-schritt-3 = Paste the key below, “Test key” and “Save”.
cf-persoenlich = The key is personal
cf-persoenlich-text = – don’t share or publish it. It stays on this PC only. Subject to the
cf-bedingungen = CurseForge third-party terms
cf-experimentell-text = Experimental: whether CurseForge grants keys for private tools is up to CurseForge.
cf-beantragen = Request a key ↗
cf-konsole = CurseForge console ↗
cf-mehr = More about the CurseForge API ↗
cf-platzhalter = Paste API key here
cf-testen = Test key
cf-teste = Testing …
cf-funktioniert = The key works.
cf-schluessel-da = Stored key: { $maskiert } – CurseForge is included in searches.
cf-kein-schluessel = No key stored – searching Modrinth only.
cf-gespeichert = CurseForge key saved
cf-entfernen-frage = Remove the CurseForge key?
cf-entfernen-text = Afterwards Vapafane searches Modrinth only again.

## Add server
gem-anderer-ordner = Other folder …
srv-typ-fabric = Fabric (mods)
srv-typ-paper = Paper (plugins)
srv-typ-neoforge = NeoForge (mods)
srv-typ-vanilla = Vanilla (no mods)
srv-name-label = Name in the server list
srv-typ-label = Server type
srv-version-label = Minecraft version
srv-laedt = loading …
srv-keine-verbindung = No connection: { $fehler }
srv-ordner-waehlen = Choose a folder for the server
srv-server-ordner-waehlen = Choose the server folder
srv-backup-waehlen = Choose a server backup (ZIP or tar)
srv-ordner-nicht-leer = The chosen folder is not empty – the server goes into a new subfolder.
srv-ordner-ungueltig = This folder cannot be used.
srv-eula-zusatz = (no server starts without it)
srv-hinweis = Note
srv-ev-hinzu = Server added: { $name }
srv-hinzu-titel = Add server
srv-hinzu-text = “{ $name }” is in the list. Stop the running server to switch.
srv-err-hinzu = Adding failed
srv-kein-server = No server found
srv-import-titel = Import server
srv-err-import = Import failed
srv-neu-titel = Create a new server
srv-neu-name = My Server
srv-err-erstellen = Creating failed
srv-entfernen-frage = Remove “{ $name }” from the list?
srv-entfernen-text = The files in { $ordner } are kept – only the entry disappears.
srv-err-entfernen = Removing failed

## Mods/Plugins
mods-erklaerung = Disabled mods stay in the folder but are not loaded. Changes take effect on the next server start.
mods-ev-an = { $name } enabled
mods-ev-aus = { $name } disabled
mods-neustart-noetig = The server is running – the change takes effect after the next restart.
mods-nur-client = client only
mods-aus-grund = Off: { $grund }
mods-an-tipp = Enabled – click to disable
mods-aus-tipp = Disabled – click to enable
mods-anzahl = { $an } of { $gesamt } { $wort } enabled
mods-datapacks-anzahl = { $anzahl } datapacks
mods-leer = No { $wort } in the “{ $ordner }” folder.
mods-datapacks-leer = No datapacks in the world.
mods-datapacks-titel = Datapacks in the world
mods-hinzufuegen = Add { $wort } …
mods-hinzufuegen-titel = Add { $wort }
mods-ordner-oeffnen = Open mods folder
mods-aufraeumen = Clean up …
mods-aufraeumen-titel = Clean up
mods-aufraeumen-tipp = Delete leftovers of old versions (server must be stopped)
mods-alles-sauber = All clean – there are no leftovers of old versions.
mods-aufraeumen-text-1 = These leftovers of old versions would be deleted (together
mods-aufraeumen-text-2 = World, settings and disabled mods stay. Anything the server still needs it re-downloads on the next start.
mods-erst-stoppen = Please stop the server first to delete.
mods-loeschen-knopf = Delete { $groesse }
mods-ev-aufgeraeumt = Cleaned up: { $groesse } freed
mods-aufgeraeumt-titel = Cleaned up
mods-aufgeraeumt = { $anzahl } entries deleted, { $groesse } freed.
mods-nicht-loeschbar = Could not delete:
mods-err-aufraeumen = Clean-up failed

## Catalog (add mods)
kat-suche = Search on Modrinth … (empty = most popular)
kat-dp-option = Datapacks (for the world)
kat-treffer = { $anzahl } matching { $wort } for Minecraft { $mc }
kat-beliebtheit = sorted by popularity
kat-sucht = Searching …
kat-nichts = Nothing found.
kat-mehr = Load more
kat-cf-tipp = Tip: with a CurseForge key (Settings) you additionally find mods that only exist there.
kat-nur-cf = Only on CurseForge
kat-cf-erklaerung = These are not on Modrinth. Some authors don't allow downloads through other programs – then you download the file in the browser and put it into the folder yourself.
kat-beides = Server + client
kat-beides-tipp = Players need this themselves too – e.g. via “Export mods for friends (.mrpack)”
kat-nur-server = Server only
kat-meta = { $downloads } downloads · by { $autor }
kat-laedt = Loading …
kat-installiert = Installed
kat-installieren = Install
kat-ev-installiert = Installed: { $name }
kat-err-install = Installing did not work: { $fehler }
kat-ist-installiert = is installed (in “{ $ordner }”)
kat-abhaengigkeiten = Added as dependency: { $namen }.
kat-schon-da = Already present: { $namen }.
kat-fehlende = Warning: this dependency does not exist for this version: { $namen } – the server may not start like this.
kat-client-noetig = Players need this mod too – easiest via “Export mods for friends (.mrpack)” under “Version & updates”.
kat-manuell-1 = The author does not allow downloads through other programs. Download the file
kat-manuell-2 = in the browser and put it into the “{ $ordner }” folder.
kat-browser = Download in browser ↗
kat-dp-ordner = Open datapack folder
kat-neustart = The server is running – this takes effect after a restart.
kat-jetzt-neustarten = Restart now

## Version & updates
upd-st-aktuell = up to date
upd-st-fehlt = missing
upd-st-unbekannt = unknown
upd-st-neu = new (needed)
upd-st-manuell = update only in browser (CurseForge)
upd-st-reenable = fits again – will be enabled
upd-st-doppelt = duplicate (already there as a mod)
upd-alles-aktuell = Everything is up to date.
upd-zusammenfassung = { $updates } updates, { $aktuell } already current, { $fehlend } without a matching version, { $unbekannt } unknown
upd-neu-gebraucht = { $anzahl } newly needed
upd-passen-wieder = { $anzahl } fit again
upd-doppelt = { $anzahl } duplicate (will be set aside)
upd-server-stand = New server build for Minecraft { $mc }: { $label }.
upd-beta-hinweis = Note: so far there is only a test/beta build for this.
upd-wechsel = Server: Minecraft { $von } → { $nach } ({ $label }).
upd-welt-warnung = Warning: a world that ran on the new version once cannot be downgraded. The only way back is the backup that is made automatically.
upd-java-da = Java { $version } needed – available ({ $pfad }).
upd-java-fehlt = Java { $version } needed but not installed. { $hinweis }
upd-manuell-hinweis = For mods with “update only in browser” the author does not allow automatic downloads – download the new file on CurseForge and swap it yourself.
upd-platz = Not enough disk space for the backup ({ $frei } free, server { $server }).
upd-nur-mods = { $version } (update mods/plugins only)
upd-keine-neuere = There is no newer stable Minecraft version.
upd-keine-verbindung = No connection
upd-err-pruefen = Check failed
upd-rollback-frage = Go back to the state before the last update?
upd-rollback-text = The server is stopped and restored from the backup made during the update. The current state stays next to it as a folder named “…_vor_Wiederherstellung_<date>”.
upd-zuruecksetzen = Reset
upd-ev-rollback = State before the update restored
upd-rollback-fertig-titel = Reset done
upd-rollback-fertig = The server is back at the state of { $label }. You can start it now.
upd-rollback-daneben = The state after the update is in:
upd-err-rollback = Reset failed
upd-rollback-knopf = Back to the state before the last update
upd-was-mods = Update mods
upd-ablauf = Steps: stop the server (with a warning if someone is online) → full backup → download and verify { $anzahl } files → swap → start the server and check.
upd-werden-deaktiviert = { $anzahl } mod(s)/datapack(s) without a matching version will be disabled.
upd-probelauf-tipp = Tip: with “Trial run” you can safely test this on a copy first.
upd-start-frage = Start the update? ({ $was })
upd-starten = Start update
upd-ev-gestartet = Update started ({ $was })
upd-err-update = Update failed
upd-ev-fertig = Update done ({ $was })
upd-fertig-titel = Update done
upd-fertig-1 = The server is running again ({ $was }).
upd-fertig-backup = Backup of the old state:
upd-fertig-aufgeraeumt = Cleaned up: { $anzahl } old files/folders deleted ({ $groesse }).
upd-fertig-2 = Mods without a matching version are disabled and visible under “Mods/Plugins”.
upd-fehlgeschlagen-titel = The update did not work
upd-fehlgeschlagen-frage = Check the console for the error message. Should I restore the state before the update from the backup?
upd-trial = Trial run
upd-trial-starten = Start trial run
upd-trial-frage = Start a trial run?
upd-trial-text = I copy the server to the temp folder, update the copy, start it on a different port and check whether it finishes starting. Then the copy is stopped and deleted.
    
    Your real server stays untouched and can keep running. With large worlds the copying takes a few minutes.
upd-trial-tipp = Updates a copy in the temp folder and starts it on a different port
upd-trial-err = Trial run failed
upd-trial-ok-titel = Trial run successful
upd-trial-ok = The updated copy finished starting in { $sekunden } seconds (port { $port }).
upd-trial-timeout = The copy had not finished starting after 15 minutes.
upd-trial-exit = The copy quit during start (exit code { $code }).
upd-trial-meldungen = Notable messages:
upd-trial-zeilen = Last lines:
upd-trial-geloescht = The copy was deleted again.
upd-trial-nicht-geloescht = The copy could not be fully deleted: { $ordner }
upd-ev-trial-ok = Trial run successful
upd-ev-trial-fehler = Trial run failed
upd-jetzt = Now:
upd-erklaerung = Checks whether all mods exist for the chosen version. A backup is made automatically before every update.
upd-ziel = Target:
upd-versionen = Load versions
upd-pruefen = Check
upd-spalte-mod = Mod / datapack
upd-spalte-neu = New
upd-deaktivieren-fehlende = Disable mods/datapacks without a matching version (otherwise the server probably won't start)
upd-deaktivieren-unbekannte = Disable unknown mods/datapacks (not on Modrinth)
upd-nicht-unterstuetzt = The update assistant exists for Fabric, Paper, NeoForge and Vanilla.
upd-export-knopf = Export mods for friends (.mrpack)
upd-export-tipp = Only for Fabric and NeoForge servers (mods that friends need too)
upd-export-titel = Modpack saved
upd-export-mods = { $anzahl } mods from Modrinth
upd-export-eingepackt = { $anzahl } packed in directly
upd-export-hinweis = Your friends import the file in the Modrinth App (“Add file”). Server-only mods are not installed there.
upd-err-export = Export failed
