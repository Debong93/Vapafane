# Vapafane – English (maintained by hand)
# One line per text: key = text. Placeholders: { $name }

## Language
sprache-titel = Language
sprache-system = System language (recommended)
sprache-ki-hinweis = This translation was machine-generated (by AI) – please report mistakes via "About Vapafane".
sprache-neustart = The interface briefly reloads when switching.

## Einheiten und Zahlen
einheit-tag = d
einheit-std = h
einheit-min = min
einheit-sek = s
zahl-dezimal = .
einheit-mio = M
einheit-tsd = K
zahl-tausender = ,

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

## Client
cli-titel = Redeem invite
cli-code-erklaerung-1 = Paste the code (starts with 
cli-code-erklaerung-2 = ) or open the 
cli-code-erklaerung-3 =  file – Vapafane sets up Minecraft with all mods matching the server.
cli-code-platzhalter = Paste invite code here (VPF1-…)
cli-pruefen = Check invite
cli-datei-oeffnen = Open invite file …
cli-pruefe-einladung = Checking the invite …
cli-launcher-da = Minecraft Launcher found
cli-kein-launcher = No Minecraft Launcher
cli-modrinth-da = Modrinth App found
cli-meine-server = My servers
cli-leer = No server set up yet – redeem an invite above.
cli-kein-konto = Vapafane does not start Minecraft itself and never touches your Microsoft account – “Play” opens the official launcher or the Modrinth App.
cli-nicht-erreichbar = Server not reachable
cli-online = Online · { $version } · { $spieler }/{ $max } players
cli-version-passt-nicht = Online, but version { $version } – does not match
cli-aktualisiert = Server was updated ({ $version }) – get a new invite code from the admin
cli-gerade-nicht = not reachable right now
cli-modrinth-fehlt = Modrinth App not found
cli-launcher-fehlt = Minecraft Launcher not found
cli-download-seite = Go to download page
cli-ev-eingerichtet = Client set up: { $name }
cli-gefunden = found
cli-nicht-gefunden = not found
cli-keine-mods = No mods needed – just Minecraft { $version }.
cli-optional = optional
cli-nur-browser = downloadable only in the browser (link after setup)
cli-nur-datei = missing – only in the invite file
cli-launcher-fehlt-text = The official Minecraft Launcher was not found. You can set up anyway; the profile is created as soon as the launcher is installed and you press “Refresh” here.
cli-launcher-laden = Download launcher
cli-nie-gestartet = The launcher has never been started. Please start it once and close it, then set up here.
cli-ordner-waehlen = Choose a folder for this Minecraft
cli-mods-zusammen = { $anzahl } mods, { $groesse } in total
cli-andere-version = The server reports a different version ({ $version }). It was probably updated – please get a new invite code from the admin.
cli-spalte-version = Version
cli-spalte-groesse = Size
cli-einrichten-fuer = Set up for:
cli-in-modrinth = In the Modrinth App:
cli-instanz-update = Update existing instance “{ $name }” (recommended)
cli-instanz-update-sub = Everything stays in the same instance: worlds, settings, shaders, resource packs and mods you added yourself (e.g. Iris). You just click “Reinstall modpack” in the Modrinth App – Vapafane shows you how.
cli-instanz-neu = Create a new instance
cli-instanz-neu-sub = Works automatically via import. Settings, shaders and resource packs can be taken over below – mods you added yourself cannot. The old instance stays until you delete it.
cli-update-bleibt = When updating, everything in the instance is kept: worlds, options, shaders, resource packs and your server list.
cli-import-bestaetigen = The Modrinth App opens – just confirm the import there.
cli-ordner-label = Folder:
cli-launcher-schliessen = Please close the Minecraft Launcher before setting up so it does not overwrite the change.
cli-einrichten = Set up
cli-uebernehmen-von = Take settings from:
cli-nichts = Take nothing
cli-quelle-waehlen = Choose the Minecraft folder to take settings from
cli-nichts-uebernehmen = Nothing to take over
cli-teil-options = Options & key bindings
cli-teil-config = Mod settings
cli-teil-shaders = Shaders
cli-teil-resourcepacks = Resource packs
cli-teil-servers = Server list
cli-teil-worlds = Worlds
cli-welten-warnung = Worlds from a newer Minecraft version cannot be opened in an older one. Large worlds make the setup slower.
cli-nur-fehlendes = Whatever is already in the folder stays unchanged – only what is missing is taken over.
cli-uebernehmen-erklaerung = For example from your previous instance: options, key bindings, mod settings, shaders, resource packs and server list. Mods always come matching from the server. The source is only read, never changed.
cli-paket-update-1 = The new Modrinth pack is ready. Here is how to update your instance
cli-paket-update-2 = worlds, options, shaders, resource packs and your server list are all kept:
cli-schritt-instanz = Open the Modrinth App and click the instance
cli-schritt-anklicken = on the left.
cli-schritt-zahnrad = Click the gear “Instance settings” at the top and choose the “Installation” tab.
cli-neu-installieren = Reinstall modpack
cli-schritt-datei = click it, confirm and choose this file:
cli-modrinth-oeffnen = Open Modrinth App
cli-paket-ordner = Open folder with the pack
cli-update-sicher = The Modrinth App backs up the instance itself first and only swaps the server's mods – whatever you added yourself (shaders, resource packs, own mods) stays.
cli-paket-offen = The Modrinth pack is ready and the Modrinth App was opened with it. Confirm the import there – then the instance appears as
cli-paket-fertig = The Modrinth pack is ready:
cli-paket-von-hand = The Modrinth App could not be opened automatically. To do it by hand: click “+” (create instance) in the Modrinth App, then “Import from file” and choose this file.
cli-eingerichtet = Set up: { $anzahl } mods downloaded.
cli-profil = The Minecraft Launcher now has the profile “{ $name }”.
cli-uebernommen-paket = The settings from “{ $quelle }” are in the pack – the Modrinth App applies them during import.
cli-uebernommen = Taken from “{ $quelle }”: { $anzahl } files.
cli-uebernommen-da = ({ $anzahl } were already there and stayed unchanged)
cli-uebernommen-fehler = { $anzahl } files could not be read and are therefore missing – is Minecraft still open there? ({ $dateien })
cli-selbst-laden = Please download yourself and put into the “mods” folder:
cli-spielen = Play
cli-spielen-modrinth = Play (open Modrinth App)
cli-spielen-launcher = Play (open launcher)
cli-prueft = checking …
cli-unvollstaendig = Setup did not finish – please redeem the invite above again (or remove).
cli-mods-kurz = { $anzahl } mods
cli-eingerichtet-am = { $ziel } · set up { $datum }
cli-aktualisieren = Refresh
cli-aktualisieren-tipp = Set up again with the stored invite – or explain what to do after a server update
cli-entfernen-frage = Remove “{ $name }”?
cli-entfernen-modrinth = Vapafane forgets this client and deletes its pack. The instance in the Modrinth App stays – please delete it there yourself.
cli-entfernen-launcher = The folder with mods, settings and singleplayer worlds of this profile is deleted, and the profile disappears from the Minecraft Launcher.
cli-upd-titel = Server was updated
cli-upd-text = “{ $name }” now runs Minecraft { $version }. Which version and mods belong to it is stored in the invite – you need a new invite code from the admin for that.
cli-upd-code = Paste it above under “Redeem invite” and click “Check invite”.
cli-upd-modrinth = Vapafane then builds a new pack and shows you how to update your instance with it – your settings, worlds and shaders stay.
cli-upd-launcher = Vapafane then swaps the mods – your settings and worlds stay.
cli-gespeicherte-einladung = Loaded the stored invite of “{ $name }”. “Set up” makes everything right again – for example when mods are missing or the profile disappeared. If the admin updated the server, you need a new invite code instead.
ext-knopf = Shaders & extras
ext-knopf-so-gehts = Shaders & extras – how to

## Shaders & extras
ext-titel = Add shaders & extras
ext-skizze-1-alt = Sketch of the instance in the Modrinth App
ext-skizze-2-alt = Sketch of the search in the Modrinth App
ext-skizze-leer = No additional content installed
ext-skizze-hochladen = Upload files
ext-durchsuchen = Browse content
ext-skizze-shader = A shader, e.g. Complementary
ext-skizze-beschreibung = Description · downloads
ext-keine-shader = Shaders need the Iris mod – and it only runs on servers with Fabric or NeoForge. This server has no mod loader, so resource packs work here, but shaders don't.
ext-iris-modrinth = For shaders, Minecraft needs the Iris mod: in step 4 choose “Mods” first and install “Iris Shaders” (Sodium comes along automatically), then pick a shader under “Shaders”.
ext-iris-launcher-1 = For shaders, Minecraft needs the Iris mod (with Sodium): download both matching the server's Minecraft version from
ext-iris-launcher-2 = and put them into the “mods” folder.
ext-mr-einleitung-1 = In your instance
ext-mr-einleitung-2 = you can add your own shaders, resource packs and mods at any time. They are kept across server updates – the Modrinth App only swaps the server's mods.
ext-schritt-content = Choose the “Content” tab at the top.
ext-schritt-durchsuchen = Click the green “Browse content” button.
ext-schritt-art = Choose the type at the top: “Shaders” for shaders, “Resource Packs” for resource packs or “Mods”.
ext-schritt-installieren = Click “Install” on the entry you want – done.
ext-im-spiel = In the game: Options → Video Settings → Shader Packs → select the shader.
ext-im-spiel-rp = In the game: Options → Resource Packs → select the pack.
ext-mods-update-hinweis = Your own mods must be updated in the app yourself after a Minecraft update; shaders and resource packs usually not.
ext-launcher-einleitung = This server's game folder belongs to it alone: whatever you add there is kept across server updates.
ext-schritt-laden-shader = Download a shader, e.g. from
ext-schritt-laden-rp = Download a resource pack, e.g. from
ext-nicht-entpacken = do not unzip the .zip file.
ext-schritt-shaderpacks = Click “Open folder” below and put the file into the “shaderpacks” folder. Resource packs go into “resourcepacks”.
ext-schritt-resourcepacks = Click “Open folder” below and put the file into the “resourcepacks” folder.
ext-eigene-mods = You can put your own mods into the “mods” folder – but after a Minecraft update you must keep them matching yourself.
err-antwort-unlesbar = Response from “{ $befehl }” unreadable: { $fehler }

## Reachability report
rch-w-erreichbar = reachable
rch-w-nicht = not reachable
rch-w-nicht-dslite = not reachable (DS-Lite)
rch-w-nicht-geprueft = not checked
rch-w-nicht-vorhanden = not available
rch-unmoeglich-titel = Check not possible right now
rch-unmoeglich-text = The checking service on the internet is not responding. Try again later.
rch-gruen-beide = Reachable via IPv4 and IPv6
rch-gruen-v4 = Reachable via IPv4
rch-gruen-text = All friends can get in.
rch-gelb-dslite-titel = DS-Lite: reachable via IPv6 only
rch-gelb-dslite-text = Your provider shares the IPv4 address. Friends with IPv6 can still get in – that's most of them.
rch-gelb-titel = Reachable via IPv6 only
rch-gelb-text = Nobody gets in via IPv4: either the IPv4 port forwarding (port { $port }) is missing, or your provider shares the IPv4 address (DS-Lite). Friends with IPv6 can still get in.
rch-rot-titel = Not reachable from outside
rch-rot-text = Usually the port forwarding in the router is missing, or the PC's firewall blocks port { $port }.
rch-rot-dslite-v6 = Your provider shares the IPv4 address (DS-Lite), so only IPv6 works. Forward port { $port } for IPv6 in your router and check the PC's firewall.
rch-rot-dslite = Your provider shares the IPv4 address (DS-Lite) and your PC has no IPv6 address. Nobody from outside can reach the server like this.
rch-aus-titel = Server is not running
rch-aus-text = It will be checked as soon as the server runs.
rch-port-belegt-titel = Port { $port } is already in use
rch-port-belegt-text = Another program – probably another Minecraft server – is using this port. This server cannot start like this. Set a free port under Settings → “Server port” (e.g. { $frei })
rch-hotspot-tipp = The server is only reachable via IPv6, but your internet has no IPv6. Tip: connect your PC through your phone's hotspot – mobile networks usually have IPv6.

## Program errors (general)
err-dienst-aus = Background service is not running
err-kein-systemd = systemd is not available here (or test mode).
err-kein-server = No server selected.
err-schon-beschaeftigt = Something is already running: { $was }
err-neustart-extern = This server runs outside Vapafane – restarting only works through the program that started it.
err-stop-knopf = Please use the Stop button in the overview (it saves safely).
err-start-extern = The server runs outside Vapafane. Please stop it there first.
err-start-anderer = Another server is currently running. Please stop it first.
err-stop-unmoeglich = The server could not be stopped.
err-pfad = Invalid path
err-rcon-antwort = Invalid RCON response
err-adresse = Invalid address
cln-alte-neoforge = older NeoForge version
cln-alte-bibliothek = older version of a library
cln-alte-mods = old mod versions (set aside by the previous panel/launcher)
cln-cache = mod cache (recreated on the next start)
cli-datei-titel = Open invite file
srv-rcon-an = RCON enabled (Vapafane needs it for commands).
err-link-verboten = This address may not be opened.
err-pruefsumme = Checksum does not match: { $datei }
err-kein-fabric = No Fabric loader found for Minecraft { $mc }
cf-err-leer = Please paste a key first.
cf-err-id = Invalid CurseForge ID
cf-err-format = That does not look like an API key (please paste without spaces).
cf-err-antwort = Invalid response from CurseForge: { $fehler }
cf-err-abgelehnt = CurseForge rejects the API key (invalid or not yet approved).
cf-err-fehlt = No CurseForge key stored.
kat-err-typ = “Add mods” is not available for this server type.
kat-err-keine-version = “{ $name }” does not exist for Minecraft { $mc } ({ $loader }).
kat-err-cf-version = “{ $name }” does not exist on CurseForge for Minecraft { $mc }.
fort-suche = Searching { $name } …
fort-suche-cf = Searching { $name } on CurseForge …
inv-err-neuer = This invite code is for a newer Vapafane version. Please update Vapafane.
inv-err-unvollstaendig = The code is incomplete or damaged – please copy all of it.
inv-err-kaputt = The code is damaged – please copy it again.
inv-err-version = This invite is for a different Vapafane version.
inv-err-angaben = The invite contains invalid data.
inv-err-mod = The invite contains an invalid mod entry.
inv-err-typ = Invites exist for Fabric, NeoForge, Paper and Vanilla servers.
inv-err-datei = This is not a valid Vapafane invite file.
inv-err-datei-kaputt = The invite in the file is damaged.
inv-err-oeffentlich = The public address could not be determined.
flv-err-kein-server = There is no server for Minecraft { $mc }.
flv-err-kein-paper = No Paper for { $mc }.
flv-err-kein-neoforge = There is no NeoForge for Minecraft { $mc }.
fort-neoforge = NeoForge installer is setting up the server (downloading libraries) …
fort-suche-server = Looking for { $was } for Minecraft { $mc } …
fort-pruefe-java = Checking Java …
fort-frage-cf = Asking CurseForge about the remaining mods …
fort-pruefe-deps = Checking dependencies …
fort-pruefe-dp = Checking datapacks …
fort-pruefe-java-platz = Checking Java and disk space …
upd-err-dateiname = Invalid file name from Modrinth: { $name }
upd-cf-fehler = CurseForge could not be checked: { $fehler }
upd-abhaengigkeit = (dependency)
upd-grund-fehlt = does not exist for Minecraft { $mc }
cli-err-port = Invalid port in the address
cli-err-antwort = Invalid response from the server
cli-err-profil-kaputt = { $datei } is damaged – please start the launcher once.
cli-err-nicht-leer = The folder { $ordner } is not empty and does not belong to Vapafane.
cli-err-datei-kaputt = File { $datei } from the invite is damaged.
cli-err-fabric-profil = Fabric profile invalid.
cli-err-neoforge-java = NeoForge needs Java { $version }. Please start the Minecraft Launcher once (it brings Java along) or install Java.
cli-hinweis-launcher-starten = Please start the Minecraft Launcher once and close it, then set up again.
cli-hinweis-kein-launcher = The official Minecraft Launcher was not found. Please install it, start it once, then press “Refresh” here.
cli-hinweis-nie-gestartet = The launcher has never been started. Please start it once and close it, then “Refresh”.
cli-paket-summary = Set up by Vapafane for the server { $name } ({ $adresse })
fort-pruefe-mods = Checking the mods …
fort-lade = Downloading { $name } …
fort-launcher = Setting up the launcher …
fort-neoforge-install = Installing NeoForge { $version } (can take a few minutes) …
fort-uebernehme = Taking over settings …
fort-trage-ein = Adding the server …
fort-sammle = Collecting the settings …
fort-packe = Packing the Modrinth pack …
fort-fertig = Done.
task-update = Update
task-backup = Backup (whole server as ZIP)
task-restore = Restore backup
task-client = Set up client
task-rollback = Back to the state before the update
task-pruefe-mods = Checking mods for { $mc }
upd-err-erst-pruefen = Please press “Check” first.
upd-err-anderer-server = The check belongs to a different server. Please check again.
upd-err-absturz = The server crashed during start. { $grund }
upd-err-beendet = The server quit right away again.
upd-err-kein-backup = There is no backup from the last update for this server.
upd-err-temp-platz = Not enough space in the temp folder for a copy ({ $groesse } needed).
cln-err-laeuft = Cleaning up only works while the server is stopped.
fort-raeume-auf = Cleaning up old files …
fort-loesche = Deleting old files …
fort-loesche-kopie = Deleting the copy …
nbt-err-unvollstaendig = servers.dat is incomplete.
nbt-err-laenge = servers.dat contains an invalid length.
nbt-err-eintrag = servers.dat contains an unknown entry.
nbt-err-tiefe = servers.dat is nested too deeply.
nbt-err-format = servers.dat has an unknown format.
say-backup-start = Backup running …
say-backup-fertig = Backup done.
say-backup-fehler = Backup failed.
say-stoppt = stopping
say-neustart = restarting
say-countdown = Server { $was } in { $sekunden } seconds!
svc-countdown = Server { $was } in { $sekunden } s
svc-err-laeuft-nicht = The server is not running right now.
svc-err-befehl = Unknown command: { $befehl }
svc-err-ordner-fehlt = Server folder missing: { $ordner }
svc-err-eula = The Minecraft EULA has not been accepted for this server yet.
svc-err-fremder-server = A server is already running in this folder (PID { $pid }) that was not started by Vapafane. Please stop it first (e.g. old control panel).
svc-err-port = Port { $port } ({ $art }) is already in use. Is another Minecraft server running on this PC?
svc-signal = ended by signal
svc-beendet = ended
svc-abgestuerzt = Crashed ({ $grund }).
svc-taegliches-backup = Daily backup
say-update-neustart = Server is restarting for an update!
svc-err-java-start = Java could not be started ({ $java }): { $fehler }

## Command line
ctl-hilfe = Usage: vapafane --ctl <command>
    
      status            show the state of the active server
      start             start the server (starts the background service if needed)
      stop [SECONDS]    stop the server cleanly, optionally with a countdown warning
      restart [SEC.]    restart the server
      wait-ready [SEC.] wait until the server is ready
      wait-stopped [S.] wait until the server is off
      backup            back up the whole server folder as a ZIP
      cmd <COMMAND …>   send a command via RCON
      aktiv NAME        make another server from the list active (only while stopped)
      neu TYPE NAME VER [FOLDER]  create a new server (fabric, paper, neoforge, vanilla) and make it active
      update-check VER  check mods/datapacks for Minecraft VER via Modrinth
      trial VER         trial run: update a copy, test on another port, delete it
      update VER        update: stop, backup, swap, start (only if the check is ok)
      mods              show mods and datapacks with their on/off state
      mod-an PATH       enable a mod (PATH as in “mods”, e.g. mods/x.jar)
      mod-aus PATH      disable a mod (becomes .jar.disabled)
      mod-entfernen PATH  delete a mod/datapack for good
      aufraeumen [--ja] shows old leftovers; with --ja they are deleted (server must be off)
      mod-suche [TERM] [--datapack]     search Modrinth for matching mods/plugins (or datapacks)
      mod-install PROJECT [--datapack]  install a mod/plugin/datapack with required dependencies
      curseforge-schluessel [--loeschen]  read the CurseForge key from input and store it
      curseforge-test   try the stored CurseForge key
      einladung [ADDRESS]               print an invite code for friends (address = host:port)
      einladung-datei FILE [ADDRESS]    write an invite file (.vapafane)
      einloesen CODE|FILE [--ordner O] [--alle-optionalen] [--ziel launcher|modrinth]
                … [--einstellungen-von FOLDER] [--neue-instanz]
                                        set up a client for an invite (launcher profile or Modrinth pack);
                                        take settings (without worlds) from FOLDER; with --neue-instanz an
                                        existing instance in the Modrinth App is not updated
      clients                           show the set-up clients
      client-entfernen FOLDER           remove a client and its launcher profile
      ping ADDRESS                      query the server status (like the in-game server list)
      erreichbarkeit    check whether the server is reachable from outside (IPv4/IPv6, DS-Lite)
      mrpack FILE       export mods as a Modrinth modpack (.mrpack) for friends
      quit-service      stop the background service (stops the server first)
ctl-fehler = Error: { $fehler }

## Programm-Meldungen (Teil 3)
err-archivformat = Unknown archive format (allowed: .zip, .tar, .tar.gz, .tar.zst)
fort-entpacke = Extracting { $datei }
err-ordner-nicht-leer = The folder { $ordner } is not empty.
err-aufgabenplanung = Setting up the Task Scheduler failed (cancelled or no permission). { $details }
err-mc-unbekannt = The server's Minecraft version is unknown – please start the server once.
kat-err-projekt = The project “{ $projekt }” doesn't exist on Modrinth.
kat-err-schon-da = “{ $name }” is already installed.
kat-err-keine-datei = “{ $name }” has no downloadable file.
fort-sortiere = Putting the files in place …
kat-err-installiert = That's already installed.
fort-lade-cf = Downloading { $name } from CurseForge …
cln-alte-mc-dateien = Files of an old Minecraft version
cln-alte-server-datei = Server file of an old version
cln-alte-mc-datei = Minecraft file of an old version
cln-vor-update = Files from before an update (their ZIP backup stays)
cln-ersetzt = files replaced by this update
cli-err-keine-adresse = No server address given.
cli-err-adresse = Address “{ $adresse }” not found.
cli-err-nicht-erreichbar = Server not reachable (off, wrong address or port not forwarded).
cli-err-mod-weg = A mod from the invite no longer exists on Modrinth ({ $id }).
cli-err-hash = A mod file doesn't match the invite (checksum mismatch).
cli-nur-in-datei = only included in the invite file
err-neoforge-installer = The NeoForge installer failed:
    { $details }
cli-err-kein-client = That's not a Vapafane client.
cli-err-launcher-fehlt = The official Minecraft Launcher was not found. Download: { $link }
cli-keine-app = (no app)
cli-test-oeffnen = Test mode – would open: { $wie } { $pfad }
inv-datei-titel = Save invite file
inv-datei-name = Invite
inv-datei-filter = Vapafane invite
err-ziel-unbekannt = Unknown target
cf-err-404 = Not found on CurseForge.
cf-err-429 = CurseForge reports too many requests – please wait a moment.
cf-err-code = CurseForge replied with error { $code }.
cf-err-bereich = CurseForge category “{ $bereich }” not found.
err-mc-gibt-es-nicht = Minecraft { $mc } doesn't exist at Mojang.
err-paper-download = There's no Paper download for this version.
err-neoforge-start = The NeoForge installer couldn't start ({ $java }): { $fehler }
err-rcon-weg = No RCON connection to the server.
fort-stoppe = Stopping the server (saving the world) …
err-ordner-unbekannt = Unknown folder
err-fenster = The window couldn't be started: { $fehler }
inv-err-kein-code = That's not a Vapafane invite code (it starts with “VPF1-”).
inv-err-typ-unbekannt = Unknown server type in the invite.
inv-err-datei-leer = The file doesn't contain an invite (vapafane.json).
err-dienst-antwort = No answer from the background service.
err-dienst-fehler = Error in the background service.
err-dienst-start = The background service couldn't be started: { $details }
err-dienst-stumm = The background service isn't responding. Details are in { $datei }
err-umschalten-laeuft = Switching only works while the current server is stopped.
err-aufgabe-laeuft = Please wait until the running task is finished.
err-ordner-fehlt = Folder missing: { $ordner }
err-kein-server-ordner = { $ordner } contains neither a server jar nor a server.properties.
err-ordner-doppelt = This folder is already in the list.
err-name-fehlt = Please enter a name.
task-import = Importing “{ $name }”
err-typ-unbekannt = Unknown server type
task-erstelle = Creating { $typ } server “{ $name }”
err-sprache-unbekannt = Unknown language
err-erst-stoppen = Please stop the server first.
err-port-zahl = The port must be a number.
err-port-bereich = The port must be between 1024 and 65535.
err-uhrzeit = Please enter a time like 05:00.
err-noch-kein-server = No server created yet.
err-http-code = The server replied with error { $code }.
err-keine-verbindung = No connection: { $fehler }
err-zu-viele-anfragen = { $host } reports too many requests. Please wait a few minutes and try again.
err-fabric-installer = No Fabric installer found.
err-andere-aufgabe = another task
fort-speichert = The server is saving the world …
err-port-belegt = Port { $port } is already in use – probably another Minecraft server is running there. Set a free port under Settings → “Server port”, e.g. { $frei }.
err-rcon-aus = RCON is switched off in server.properties.
err-rcon-nicht-verbunden = RCON is not connected.
err-rcon-passwort = The RCON password is wrong.
err-rcon-erreichbar = RCON is not reachable.
err-keine-jar = No server jar found. Please put the jar into the folder.
err-keine-jar-name = No server jar found (MC_JAR={ $jar }).
err-nichts-zu-uebernehmen = There's nothing to take over in this folder. Choose a Minecraft game folder – it contains e.g. “options.txt”, “config”, “shaderpacks” or “saves”.
cli-quelle-ordner = Folder: { $ordner }
err-ordner-weg = The folder { $ordner } no longer exists.
err-teil-unbekannt = Unknown part: { $teil }
fort-lese-mods = Reading the installed mods and plugins …
fort-lade-namen = Loading the mod names …
fort-lade-version = Downloading { $name } { $version } …
fort-tausche = Swapping the files …
fort-richte-ein = Setting up { $name } …
fort-getauscht = Files swapped.
err-start-timeout = The server still hadn't finished starting after 15 minutes.
fort-warne = Warning the players (30 seconds) …
fort-backup = Creating the backup …
fort-starte = Starting the server (the first start can take a few minutes) …
fort-kopiere = Copying { $datei }
fort-starte-kopie = Starting the copy on port { $port } …
fort-stoppe-kopie = Stopping the copy …
ctl-timeout = Timed out (state: { $zustand })
ctl-startet = Starting the server.
ctl-stoppt = Stopping the server.
ctl-neustart = Restarting the server.
ctl-fehlt-name = Please give a name.
ctl-nicht-in-liste = “{ $name }” is not in the list.
ctl-neu-benutzung = Usage: --ctl neu TYPE NAME VERSION [FOLDER]
ctl-fehlt-version = Please give the target version.
ctl-update-plan = Version { $von } → { $nach }, Java needed: { $java } (ok: { $javaok }), space ok: { $platzok }, work to do: { $tun }
ctl-java-platz = Java or disk space doesn't fit.
ctl-fehlt-pfad = Please give a path.
ctl-geloescht = Deleted: { $groesse } (not possible: { $fehler })
ctl-treffer = { $anzahl } results for Minecraft { $mc } ({ $typ }):
ctl-fehlt-projekt = Please give a project.
ctl-cf-geloescht = CurseForge key deleted.
ctl-cf-ok = The CurseForge key works ({ $schluessel }).
ctl-nur-server = Server only (not in the invite): { $mods }
ctl-ohne-quelle = Without a source (only possible in the invite file): { $mods }
ctl-fehlt-code = Please give a code or a file.
ctl-fehlt-ordner = Please give a folder.
ctl-fehlt-adresse = Please give an address.
ctl-fehlt-zieldatei = Please give a target file.
ctl-dienst-ende = Stopping the background service.
gem-zitat = “{ $text }”
spi-entfernt = { $name } removed.

## Mods: entfernen, Abhängigkeiten, Infos
mods-err-name-belegt = There's already an enabled file with this name.
mods-err-ordner-aus = Datapack folders can't be switched off – only ZIP datapacks. You can remove them, though.
mods-err-weg = The file no longer exists.
mods-grund-hand = switched off by hand
mods-grund-unbekannt = not on Modrinth – matching version unclear
mods-grund-doppelt = duplicate – “{ $name }” is already installed as a mod
mods-ev-entfernt = { $name } removed
mods-entfernen-tipp = Remove for good
mods-entfernen-frage = Remove “{ $name }”?
mods-entfernen-text = The file will be deleted for good. It's still contained in the last backup.
mods-gebraucht-warnung = Careful – still needed by: { $namen }. Without this file the server probably won't start.
mods-waisen-frage = No other mod needs these helpers afterwards – remove them too?
mods-aus-frage = Switch off “{ $name }”?
mods-aus-knopf = Switch off anyway
mods-err-entfernen = Removing didn't work
mods-gebraucht-von = Needed by: { $namen }
mods-bibliothek = Library
mods-bibliothek-tipp = A helper that other mods need
mods-seite = Page
mods-seite-tipp = Open the mod's page in the browser – it explains what the mod does
mods-suche = Search the list …

## Empfohlene Leistungs-Mods
empf-badge = Recommended
empf-badge-tipp = Recommended by Vapafane: proven for better performance, doesn't change gameplay
empf-titel = Recommended for better performance
empf-text = These mods make the server faster and lighter without changing gameplay. They only run on the server – your friends don't need to install anything.
empf-alle = Install all recommended
empf-mitinstallieren = Also install recommended performance mods
empf-sucht = Looking for matching performance mods …
empf-paper = Paper already comes with its own performance optimizations – no extra plugins needed.
empf-lithium = Faster game logic (mobs, redstone, blocks)
empf-ferritecore = Much lower memory usage
empf-krypton = Lighter network traffic to the players
empf-c2me = Loads and generates the world faster (uses several CPU cores)
empf-scalablelux = Faster lighting calculation, especially while exploring
empf-modernfix = Faster startup, less memory
empf-ev = Recommended mods installed: { $namen }
empf-installiert = Installed (in “{ $ordner }”): { $namen }.
empf-err-titel = Not all recommended mods were installed
empf-err-text = The server is ready, but these recommended mods could not be installed: { $namen }. You can add them later under “Mods” → “Add Mods”.
fort-suche-empf = Looking up the recommended mods …
task-empf = Installing recommended mods

## Programm-Meldungen (Teil 4)
task-installiere = Installing { $name }
fort-frage-modrinth = Asking Modrinth about the mods …
upd-export-speichern = Save modpack
upd-export-filter = Modrinth modpack
task-backup-ctl = Backup (command line)
task-mod-installieren = Installing a mod
cli-grund-browser = Download only in the browser (CurseForge)
err-archiv-pfad = Unsafe path in the archive: { $pfad }
ctl-einladung-gespeichert = { $datei }: { $mods } mods ({ $dateien } packed in directly)
fort-sichere = Backing up { $datei }
zeit-datum = { $jahr }-{ $monat }-{ $tag }  { $stunde }:{ $minute }
autostart-kommentar = Minecraft server manager
fort-bereite-vor = Preparing { $name } …
cf-mod-ersatz = CurseForge mod { $id }
ctl-aktiv = Active: { $name } ({ $pfad })
ctl-zusammen = Total: { $groesse }
ctl-gespeichert = Saved ({ $schluessel }).
ctl-code-laenge = { $mods } mods, { $zeichen } characters
ctl-entfernt = Removed.
err-unsichere-adresse = Unsafe download address: { $url }
err-nicht-erlaubt = Not allowed
quelle-modrinth = Modrinth App: { $name }
quelle-launcher-standard = Minecraft Launcher (default)
quelle-launcher = Minecraft Launcher: { $name }
upd-pack-zusammenfassung = Mods from the server { $name }
fort-server-startet = Server is starting …
aufr-fabric-mappings = Fabric mappings of an old Minecraft version
aufr-fabric-cache = Fabric cache of an old version
skin-grass = Grass Block
skin-obsidian = Crying Obsidian
