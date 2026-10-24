# Vapafane – Deutsch (Quellsprache, von Hand gepflegt)
# Eine Zeile je Text: schluessel = Text. Platzhalter: { $name }

## Sprache
sprache-titel = Sprache
sprache-system = Systemsprache (empfohlen)
sprache-ki-hinweis = Diese Übersetzung wurde maschinell (per KI) erstellt – Fehler bitte über „Über Vapafane“ melden.
sprache-neustart = Die Oberfläche lädt beim Wechsel einmal kurz neu.

## Java
java-hinweis-windows = Installieren z. B. von https://adoptium.net (Temurin { $version }).
java-hinweis-linux = Installieren mit: sudo apt install openjdk-{ $version }-jre-headless
err-java-fehlt = Auf diesem PC ist Java nicht installiert – ohne Java startet kein Minecraft-Server. Dieser Server braucht Java { $version }. { $hinweis }
err-java-noetig = Minecraft { $mc } braucht Java { $version }, das ist nicht installiert. { $hinweis }

## Gemeinsame Knöpfe
gem-ok = OK
gem-schliessen = Schließen
gem-abbrechen = Abbrechen
gem-weiter = Weiter
gem-entfernen = Entfernen
gem-speichern = Speichern
task-abgebrochen = Abgebrochen

## Kopfzeile
hdr-untertitel = Minecraft-Werkbank für Server & Client
hdr-bereich = Bereich
hdr-client = Client
hdr-server = Server
hdr-aktiver-server = Aktiver Server (es läuft immer nur einer)
hdr-kein-server = (kein Server)
hdr-plus-erster = Hier neuen Server erstellen
hdr-plus = Server hinzufügen
hdr-menu-neu = Neuen Server erstellen (Fabric, Paper, NeoForge, Vanilla) …
hdr-menu-ordner = Vorhandenen Ordner hinzufügen …
hdr-menu-import = Server aus ZIP/Backup importieren …
hdr-menu-entfernen = Ausgewählten Server aus der Liste entfernen
hdr-einstellungen = Einstellungen
hdr-hell-dunkel = Hell/Dunkel umschalten
hdr-ev-gewechselt = Server gewechselt: { $name }
hdr-err-umschalten = Umschalten nicht möglich

## Seitenleiste
side-bereiche = Bereiche
side-betrieb = Betrieb
side-uebersicht = Übersicht
side-konsole = Konsole
side-spieler = Spieler
side-inhalte = Inhalte
side-mods = Mods/Plugins
side-updates = Version & Updates
side-backups = Backups
side-netzwerk = Netzwerk
side-erreichbarkeit = Erreichbarkeit

## Willkommen
willkommen-titel = Willkommen bei Vapafane
willkommen-text = Vapafane ist kein offizielles Minecraft-Produkt. Es ist nicht von Mojang oder Microsoft genehmigt und steht in keiner Verbindung zu ihnen.
    
    Fragen, Ideen oder Fehler: über die GitHub-Seite des Projekts (Einstellungen → Über Vapafane).

## Gemeinsam
gem-ordner = Ordner
gem-ordner-oeffnen = Ordner öffnen
gem-launcher = Offizieller Minecraft-Launcher
gem-modrinth-app = Modrinth-App

## Über Vapafane
about-titel = Über Vapafane
about-version = Version { $version }
about-testmodus = TESTMODUS
about-nicht-offiziell = Vapafane ist kein offizielles Minecraft-Produkt. Es ist nicht von Mojang oder Microsoft genehmigt und steht in keiner Verbindung zu ihnen.
about-marken = Minecraft ist eine Marke von Mojang/Microsoft. Mods und Plugins stammen von ihren jeweiligen Autoren (über Modrinth bzw. CurseForge).
about-kontakt = Kontakt & Fehler melden (GitHub) ↗
about-projektseite = Projektseite ↗

## Aussehen
skin-titel = Aussehen
skin-grass-sub = Hell oder dunkel (Knopf oben rechts)
skin-obsidian-sub = Immer dunkel

## Client-Einstellungen
cset-einstellung = Einstellung
cset-einrichten = Einrichten
cset-ziel = Standard-Ziel
cset-ziel-sub = Wo neue Server eingerichtet werden
cset-spielordner = Spielordner aller Clients
cset-spielordner-sub = Hier liegen je Server Mods, Optionen und Welten

## Backups
bak-restore-frage = „{ $name }“ auf { $datei } zurücksetzen?
bak-restore-text = Der Server wird dafür gestoppt. Der jetzige Stand wird nicht gelöscht, sondern bleibt als Ordner „…_vor_Wiederherstellung_<Datum>“ daneben liegen.
bak-wiederherstellen = Wiederherstellen
bak-ev-wiederhergestellt = Backup wiederhergestellt
bak-restore-fertig-titel = Wiederhergestellt
bak-restore-fertig = Der Server ist auf dem Stand von { $label }. Du kannst ihn jetzt starten.
bak-restore-daneben = Der vorherige Stand liegt in:
    { $ordner }
    (kann gelöscht werden, wenn alles passt)
bak-err-restore = Wiederherstellen fehlgeschlagen
bak-waehlen = Backup wählen
bak-erklaerung = Jedes Backup ist der komplette Server-Ordner als ZIP, mit Datum im Namen. Gespeichert in { $ordner }. Die neuesten { $anzahl } bleiben, ältere werden gelöscht.
bak-jetzt = Jetzt sichern
bak-ordner = Backup-Ordner öffnen
bak-andere = Andere ZIP wiederherstellen …
bak-aufheben = Aufheben:
bak-leer = Noch keine Backups für diesen Server.

## Erreichbarkeit
rch-fehlgeschlagen = Prüfung fehlgeschlagen
rch-noch-nicht = Noch nicht geprüft
rch-kein-server = Kein Server ausgewählt
rch-kein-server-text = Leg oben mit „+“ einen Server an oder wähl einen aus, dann wird geprüft.
rch-prueft = Wird geprüft …
rch-geprueft-um = Geprüft um { $zeit }
rch-ip-erklaerung = IPv4 ist die klassische Internet-Adresse, IPv6 die neue mit eigener Adresse für jedes Gerät. Bei DS-Lite teilt dein Anbieter die IPv4-Adresse, dann geht nur IPv6.
rch-erneut = Erneut prüfen
rch-portfreigabe = Anleitung Portfreigabe ↗
rch-dienst-hinweis = Zum Prüfen fragt Vapafane einen Prüfdienst im Internet (ifconfig.co), ob der Server-Port von außen offen ist. Abschalten: Einstellungen → Automatik.

## Freunde einladen
inv-titel = Freunde einladen
inv-erklaerung = Schick deinen Freunden einen Code – Vapafane richtet damit bei ihnen Minecraft mit den passenden Mods ein.
inv-adresse-label = Adresse, unter der deine Freunde den Server erreichen
inv-oeffentlich = Öffentliche Adresse ermitteln
inv-adresse-hinweis = Vorbelegt ist die Heimnetz-Adresse (nur im selben WLAN). Für Freunde von außen: öffentliche Adresse ermitteln und im Router den Server-Port freigeben.
inv-code-erzeugen = Code erzeugen
inv-als-datei = Als Datei speichern …
inv-kein-passwort = Der Code enthält keine Passwörter – nur Adresse, Version und Mods.
inv-datei-gespeichert = Gespeichert:
inv-datei-schicken = diese Datei deinen Freunden schicken.
inv-code-label = Einladungs-Code ({ $zeichen } Zeichen) – kopieren und z. B. per Messenger schicken:
inv-kopieren = Kopieren
inv-kopiert = Kopiert
inv-mods = { $anzahl } Mods für die Spieler
inv-mods-optional = (davon { $anzahl } optional)
inv-keine-mods = Keine Mods nötig – nur die Version { $version }.
inv-ipv6 = Enthält zusätzlich deine IPv6-Adresse { $adresse }.
inv-nur-ipv6 = – dein Server ist laut Prüfung nur über IPv6 erreichbar.
inv-nur-server = Nur auf dem Server (Spieler brauchen sie nicht): { $mods }
inv-datei-noetig = Diese Mods gibt es weder auf Modrinth noch auf CurseForge: { $mods }. Sie passen nicht in den Code – bitte stattdessen „Als Datei speichern“ nehmen, dann sind sie eingepackt.

## Spieler & Whitelist
spi-online-titel = Gerade online
spi-wl-titel = Whitelist (wer darf rein)
spi-wl-erklaerung = Ist die Whitelist an (Einstellungen → „Nur Whitelist darf rein“), kommen nur die Spieler hier auf den Server. Das schützt vor Fremden – trag dich selbst auch ein.
spi-name-platzhalter = Minecraft-Name
spi-hinzufuegen = Hinzufügen
spi-name-ungueltig = Bitte einen gültigen Minecraft-Namen eingeben (3–16 Zeichen).
spi-nur-laufend = Hinzufügen geht nur, während der Server läuft (er prüft den Namen bei Mojang).
spi-wl-leer = Die Whitelist ist leer.
spi-niemand = Niemand online
spi-kreativ = Kreativ
spi-ueberleben = Überleben
spi-op = OP geben
spi-op-weg = OP entziehen
spi-kick = Rauswerfen
spi-kick-frage = { $name } rauswerfen?
spi-kick-text = Ein erneuter Beitritt ist sofort möglich.
spi-err-befehl = Befehl fehlgeschlagen
wl-titel = Whitelist
wl-einschalten = Whitelist einschalten
wl-zu-spielern = Zu den Spielern
wl-warnung-offen = Achtung: Dein Server ist aus dem Internet erreichbar und die Whitelist ist aus – jeder, der die Adresse kennt oder zufällig findet, kann beitreten. Mit Whitelist kommen nur Spieler rein, die du einträgst.
wl-warnung-leer = Die Whitelist ist an, aber noch leer – gerade kommt niemand auf den Server, auch du nicht. Trag unter „Spieler“ deinen Minecraft-Namen und die deiner Freunde ein.
wl-tipp = Tipp: Die Whitelist ist aus – jeder mit der Adresse kann beitreten. Empfohlen ist sie, sobald Freunde von außen spielen: Dann kommen nur Spieler rein, die hier eingetragen sind.

## Konsole
kon-neues-log = neue Log-Datei (Server wurde neu gestartet)
kon-beigetreten = { $name } ist beigetreten
kon-verlassen = { $name } hat das Spiel verlassen
kon-bereit = Server bereit (Start in { $sekunden } s)
kon-fehler = Fehler: { $fehler }
kon-tag = Tag
kon-nacht = Nacht
kon-wetter = Klares Wetter
kon-wer-online = Wer ist online?
kon-platzhalter = Befehl eingeben, z. B. say Hallo  ·  time set day  ·  weather clear
kon-senden = Senden
kon-log-ordner = Log-Ordner öffnen

## Übersicht
ueb-laeuft = Läuft
ueb-startet = Startet …
ueb-stoppt = Stoppt …
ueb-neustart-absturz = Neustart nach Absturz …
ueb-gestoppt = Gestoppt
ueb-abgestuerzt = Abgestürzt
ueb-extern = Läuft außerhalb von Vapafane
ueb-anderer = Ein anderer Server läuft
ueb-starten = Starten
ueb-stoppen = Stoppen
ueb-neustart-knopf = Neustart
ueb-seit = seit { $dauer }
ueb-neustarts = { $anzahl }× automatisch neu gestartet
ueb-hinweis-extern = Der Server wurde nicht über Vapafane gestartet (z. B. altes Bedienfenster oder Terminal). Konsole und Spieler gehen trotzdem; „Stoppen“ schickt ihm den stop-Befehl.
ueb-hinweis-anderer = Der Hintergrund-Dienst betreibt gerade einen anderen Server. Bitte den zuerst stoppen.
ueb-hinweis-absturz = Der Server ist abgestürzt und wurde nicht mehr neu gestartet. Schau in der Konsole nach der letzten Fehlermeldung.
ueb-hinweis-kein-server = Füge oben über „＋“ einen Server hinzu.
ueb-ev-bereit = Server ist bereit
ueb-ev-gestoppt = Server gestoppt
ueb-ev-startet = Server wird gestartet
ueb-ev-neustart = Server startet neu
ueb-ev-stoppt = Server stoppt
ueb-ev-in-sek = in { $sekunden } Sek.
ueb-ev-absturz = Server ist abgestürzt
ueb-ev-absturz-neustart = Server ist abgestürzt – wird automatisch neu gestartet
ueb-notify-absturz = Minecraft-Server abgestürzt
ueb-notify-absturz-neustart = Er wird automatisch neu gestartet.
ueb-ev-backup-start = Backup gestartet
ueb-ev-backup-fertig = Backup fertig: { $datei } ({ $groesse })
ueb-err-start = Start fehlgeschlagen
ueb-err-stop = Stoppen fehlgeschlagen
ueb-err-backup = Backup fehlgeschlagen
ueb-eula-titel = EULA bestätigen
ueb-eula-text = Ohne Zustimmung zur Minecraft-EULA startet kein Server.
ueb-eula-akzeptiere = Ich akzeptiere die
ueb-eula-fuer = für „{ $name }“.
ueb-verb-stoppen = stoppen
ueb-verb-neustart = neu starten
ueb-stop-frage = Server { $verb }?
ueb-stop-extern-text = Der Server läuft außerhalb von Vapafane. Ich schicke ihm den Befehl „stop“ – er speichert die Welt und beendet sich.
ueb-stop-niemand = Gerade ist niemand online.
ueb-stop-spieler = { $anzahl } Spieler online: { $namen }
ueb-sofort = Sofort
ueb-mit-warnung = In { $sekunden } Sek. mit Warnung
ueb-karte-spieler = Spieler online
ueb-karte-tps = TPS (Ticks pro Sekunde)
ueb-karte-ram = Arbeitsspeicher
ueb-karte-ordner = Server-Ordner
ueb-karte-aussen = Von außen
ueb-karte-adresse = Adresse (Heimnetz)
ueb-karte-backup = Letztes Backup
ueb-karte-version = Version
ueb-server-aus = Server aus
ueb-keine-verbindung = keine Verbindung
ueb-niemand = niemand
ueb-ms-pro-tick = { $ms } ms pro Tick
ueb-ruckelt = ruckelt
ueb-fluessig = flüssig
ueb-von-kernen = von { $anzahl } Kernen
ueb-ram-belegt-limit = belegt (Limit { $limit } GB)
ueb-ram-belegt = belegt
ueb-ram-limit = Limit { $limit } GB
ueb-frei = { $groesse } frei
ueb-vor = vor { $dauer }
ueb-noch-keins = noch keins
ueb-backup-knopf = Knopf „Backup“ drücken
ueb-backup-sub = { $groesse } · { $anzahl } gespeichert
ueb-erreichbar = Erreichbar
ueb-nur-ipv6 = Nur IPv6
ueb-nicht-erreichbar = Nicht erreichbar
ueb-ipv4-und-6 = IPv4 und IPv6
ueb-ueber-ipv4 = über IPv4
ueb-portfreigabe = Portfreigabe prüfen
ueb-ereignisse = Letzte Ereignisse
ueb-nichts-passiert = Noch nichts passiert.
ueb-fenster-zu = Fenster schließen stoppt den Server nicht – er läuft im Hintergrund weiter.
ueb-kontakt = Kontakt & Fehler melden (GitHub)

## Einstellungen (Server)
set-spiel = Spiel (dieser Server)
set-motd = Servername (MOTD)
set-max-spieler = Max. Spieler
set-schwierigkeit = Schwierigkeit
set-wert-peaceful = Friedlich
set-wert-easy = Einfach
set-wert-normal = Normal
set-wert-hard = Schwer
set-spielmodus = Spielmodus für neue Spieler
set-wert-survival = Überleben
set-wert-creative = Kreativ
set-wert-adventure = Abenteuer
set-wert-spectator = Zuschauer
set-sichtweite = Sichtweite (Chunks)
set-simulationsweite = Simulationsweite (Chunks)
set-spawn-schutz = Spawn-Schutz (Blöcke)
set-port = Server-Port
set-whitelist = Nur Whitelist darf rein
set-whitelist-hilfe = Empfohlen für die Sicherheit: Nur Spieler auf der Whitelist (Seite „Spieler“) kommen rein. Ohne Whitelist kann jeder beitreten, der deine Adresse kennt.
set-leistung = Leistung (dieser Server)
set-ram = Max. Arbeitsspeicher für den Server (GB)
set-ram-hinweis = Der PC hat { $gb } GB. Lass etwa 3–4 GB für das System übrig.
set-speichern = Einstellungen speichern
set-gespeichert = Gespeichert.
set-gespeichert-neustart = Gespeichert. Wird beim nächsten Neustart aktiv.
set-err-speichern = Speichern fehlgeschlagen
set-automatik = Automatik
set-autostart = Aktiven Server beim PC-Start automatisch starten
set-auto-testmodus = Im Testmodus abgeschaltet.
set-auto-kein-systemd = Auf diesem System nicht verfügbar (kein systemd).
set-auto-windows = Startet beim PC-Start, auch ohne Anmeldung (Windows fragt beim Einschalten nach Administratorrechten).
set-auto-linger-fehlt = Achtung: startet erst, wenn du dich anmeldest. Klick hier, um den Start ohne Anmeldung zu erlauben.
set-auto-aktiv = Startet auch ohne Anmeldung und nach einem Absturz von selbst neu.
set-linger-frage = Start ohne Anmeldung erlauben?
set-linger-text = Dafür muss systemd deine Benutzer-Dienste schon beim Hochfahren starten dürfen („loginctl enable-linger“). Danach läuft der Server auch, wenn sich niemand anmeldet.
set-linger-erlauben = Erlauben
set-linger-abgelehnt = Nicht erlaubt
set-fenster-autostart = Dieses Fenster beim Anmelden öffnen
set-backup-um = Tägliches Backup um 
set-backup-uhr =  Uhr (ganzer Server als ZIP)
set-backup-sub = Macht der Hintergrund-Dienst. Holt es nach, falls der PC zu der Zeit aus war.
set-uhrzeit = Uhrzeit für das tägliche Backup
set-reach = Erreichbarkeit von außen prüfen
set-reach-sub = Beim Wechsel in den Server-Bereich. Fragt dafür einen Prüfdienst im Internet.
set-notify = Benachrichtigungen
set-notify-sub = Wenn jemand beitritt oder geht und wenn der Server abstürzt (solange das Fenster offen ist). Ohne Ton.
set-ordner = Ordner
set-ordner-server = Server-Ordner
set-an = an
set-aus = aus
set-panel-titel = Altes Bedienfenster noch eingerichtet.
set-panel-status = Dienst „minecraft.service“: { $dienst }, beim Start { $start }; tägliches Backup-Zeitprogramm { $backup }. Solange es läuft, kann Vapafane den Server nicht selbst starten (gleicher Port), und Backups würden doppelt gemacht.
set-panel-knopf = Altes Panel abschalten …
set-panel-frage = Altes Panel abschalten?
set-panel-text = Ich stoppe den Dienst „minecraft.service“ (der Server speichert dabei sauber und beendet sich) und schalte ihn und sein tägliches Backup beim PC-Start aus.
    
    Das alte Panel selbst und seine Dateien bleiben erhalten – man kann es jederzeit wieder einschalten. Danach startest du den Server hier mit „Starten“.
set-panel-abschalten = Abschalten
set-panel-aus = Altes Panel abgeschaltet
set-panel-err = Abschalten fehlgeschlagen

## CurseForge
cf-titel = CurseForge (optional)
cf-experimentell = Experimentell
cf-brauche = Brauche ich das?
cf-erklaerung = Nein – die meisten Mods gibt es auf Modrinth, das geht ohne Einrichtung. CurseForge findet zusätzlich Mods, die es nur dort gibt, und braucht dafür einen persönlichen API-Schlüssel (eine Art Zugangscode).
cf-anleitung-titel = So bekommst du einen Schlüssel (kostenlos)
cf-schritt-1 = Auf „Schlüssel beantragen“ klicken und das Formular ausfüllen (z. B. Zweck: „Mods für meinen eigenen Server“).
cf-schritt-2 = Auf die Antwort per E-Mail warten – CurseForge prüft von Hand, das kann ein paar Tage dauern.
cf-schritt-3 = Den Schlüssel unten einfügen, „Schlüssel testen“ und „Speichern“.
cf-persoenlich = Der Schlüssel ist persönlich
cf-persoenlich-text = – nicht weitergeben oder veröffentlichen. Er bleibt nur auf diesem PC. Es gelten die
cf-bedingungen = CurseForge-Bedingungen für Drittprogramme
cf-experimentell-text = Experimentell: Ob CurseForge Schlüssel für private Tools vergibt, entscheidet CurseForge.
cf-beantragen = Schlüssel beantragen ↗
cf-konsole = CurseForge-Konsole ↗
cf-mehr = Mehr zur CurseForge-API ↗
cf-platzhalter = API-Schlüssel hier einfügen
cf-testen = Schlüssel testen
cf-teste = Teste …
cf-funktioniert = Der Schlüssel funktioniert.
cf-schluessel-da = Gespeicherter Schlüssel: { $maskiert } – CurseForge wird bei der Suche mit gefragt.
cf-kein-schluessel = Kein Schlüssel gespeichert – gesucht wird nur auf Modrinth.
cf-gespeichert = CurseForge-Schlüssel gespeichert
cf-entfernen-frage = CurseForge-Schlüssel entfernen?
cf-entfernen-text = Danach sucht Vapafane wieder nur auf Modrinth.
