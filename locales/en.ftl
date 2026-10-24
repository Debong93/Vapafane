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
