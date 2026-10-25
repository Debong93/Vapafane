# Vapafane – Deutsch (Quellsprache, von Hand gepflegt)
# Eine Zeile je Text: schluessel = Text. Platzhalter: { $name }

## Sprache
sprache-titel = Sprache
sprache-system = Systemsprache (empfohlen)
sprache-ki-hinweis = Diese Übersetzung wurde maschinell (per KI) erstellt – Fehler bitte über „Über Vapafane“ melden.
sprache-neustart = Die Oberfläche lädt beim Wechsel einmal kurz neu.

## Einheiten und Zahlen
einheit-tag = T
einheit-std = Std
einheit-min = Min
einheit-sek = Sek
zahl-dezimal = ,
einheit-mio = Mio.
einheit-tsd = Tsd.
zahl-tausender = .

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

## Server hinzufügen
gem-anderer-ordner = Anderer Ordner …
srv-typ-fabric = Fabric (Mods)
srv-typ-paper = Paper (Plugins)
srv-typ-neoforge = NeoForge (Mods)
srv-typ-vanilla = Vanilla (ohne Mods)
srv-name-label = Name in der Server-Liste
srv-typ-label = Server-Typ
srv-version-label = Minecraft-Version
srv-laedt = lädt …
srv-keine-verbindung = Keine Verbindung: { $fehler }
srv-ordner-waehlen = Ordner für den Server wählen
srv-server-ordner-waehlen = Server-Ordner wählen
srv-backup-waehlen = Server-Backup (ZIP oder tar) wählen
srv-ordner-nicht-leer = Der gewählte Ordner ist nicht leer – der Server kommt in einen neuen Unterordner.
srv-ordner-ungueltig = Dieser Ordner lässt sich nicht verwenden.
srv-eula-zusatz = (ohne startet kein Server)
srv-hinweis = Hinweis
srv-ev-hinzu = Server hinzugefügt: { $name }
srv-hinzu-titel = Server hinzufügen
srv-hinzu-text = „{ $name }“ ist in der Liste. Stoppe den laufenden Server, um umzuschalten.
srv-err-hinzu = Hinzufügen fehlgeschlagen
srv-kein-server = Kein Server gefunden
srv-import-titel = Server importieren
srv-err-import = Import fehlgeschlagen
srv-neu-titel = Neuen Server erstellen
srv-neu-name = Mein Server
srv-err-erstellen = Erstellen fehlgeschlagen
srv-entfernen-frage = „{ $name }“ aus der Liste entfernen?
srv-entfernen-text = Die Dateien in { $ordner } bleiben erhalten, nur der Eintrag verschwindet.
srv-err-entfernen = Entfernen fehlgeschlagen

## Mods/Plugins
mods-erklaerung = Ausgeschaltete Mods bleiben im Ordner, werden aber nicht geladen. Änderungen wirken beim nächsten Serverstart.
mods-ev-an = { $name } eingeschaltet
mods-ev-aus = { $name } ausgeschaltet
mods-neustart-noetig = Der Server läuft – die Änderung wirkt nach dem nächsten Neustart.
mods-nur-client = nur Client
mods-aus-grund = Aus: { $grund }
mods-an-tipp = Eingeschaltet – klicken zum Ausschalten
mods-aus-tipp = Ausgeschaltet – klicken zum Einschalten
mods-anzahl = { $an } von { $gesamt } { $wort } eingeschaltet
mods-datapacks-anzahl = { $anzahl } Datapacks
mods-leer = Keine { $wort } im Ordner „{ $ordner }“.
mods-datapacks-leer = Keine Datapacks in der Welt.
mods-datapacks-titel = Datapacks in der Welt
mods-hinzufuegen = { $wort } hinzufügen …
mods-hinzufuegen-titel = { $wort } hinzufügen
mods-ordner-oeffnen = Mods-Ordner öffnen
mods-aufraeumen = Aufräumen …
mods-aufraeumen-titel = Aufräumen
mods-aufraeumen-tipp = Reste alter Versionen löschen (Server muss gestoppt sein)
mods-alles-sauber = Alles sauber – es gibt keine Reste alter Versionen.
mods-aufraeumen-text-1 = Diese Reste alter Versionen würden gelöscht (zusammen
mods-aufraeumen-text-2 = Welt, Einstellungen und ausgeschaltete Mods bleiben. Was der Server doch noch braucht, lädt er beim nächsten Start selbst nach.
mods-erst-stoppen = Zum Löschen bitte zuerst den Server stoppen.
mods-loeschen-knopf = { $groesse } löschen
mods-ev-aufgeraeumt = Aufgeräumt: { $groesse } frei
mods-aufgeraeumt-titel = Aufgeräumt
mods-aufgeraeumt = { $anzahl } Einträge gelöscht, { $groesse } frei.
mods-nicht-loeschbar = Nicht löschbar:
mods-err-aufraeumen = Aufräumen fehlgeschlagen

## Katalog (Mods hinzufügen)
kat-suche = Suchen auf Modrinth … (leer = die beliebtesten)
kat-dp-option = Datapacks (für die Welt)
kat-treffer = { $anzahl } passende { $wort } für Minecraft { $mc }
kat-beliebtheit = sortiert nach Beliebtheit
kat-sucht = Suche …
kat-nichts = Nichts gefunden.
kat-mehr = Mehr laden
kat-cf-tipp = Tipp: Mit einem CurseForge-Schlüssel (Einstellungen) findest du zusätzlich Mods, die es nur dort gibt.
kat-nur-cf = Nur auf CurseForge
kat-cf-erklaerung = Diese gibt es nicht auf Modrinth. Manche Autoren erlauben keinen Download über andere Programme – dann lädst du die Datei im Browser und legst sie selbst in den Ordner.
kat-beides = Server + Client
kat-beides-tipp = Die Spieler brauchen das auch selbst – z. B. über „Mods für Freunde exportieren (.mrpack)“
kat-nur-server = Nur Server
kat-meta = { $downloads } Downloads · von { $autor }
kat-laedt = Lädt …
kat-installiert = Installiert
kat-installieren = Installieren
kat-ev-installiert = Installiert: { $name }
kat-err-install = Installieren hat nicht geklappt: { $fehler }
kat-ist-installiert = ist installiert (in „{ $ordner }“)
kat-abhaengigkeiten = Dazu als Abhängigkeit: { $namen }.
kat-schon-da = Schon vorhanden: { $namen }.
kat-fehlende = Achtung: Diese Abhängigkeit gibt es nicht für diese Version: { $namen } – der Server startet so evtl. nicht.
kat-client-noetig = Die Spieler brauchen diese Mod auch – am einfachsten über „Mods für Freunde exportieren (.mrpack)“ unter „Version & Updates“.
kat-manuell-1 = Der Autor erlaubt keinen Download über andere Programme. Lade die Datei
kat-manuell-2 = im Browser herunter und lege sie in den Ordner „{ $ordner }“.
kat-browser = Im Browser herunterladen ↗
kat-dp-ordner = Datapack-Ordner öffnen
kat-neustart = Der Server läuft – das wirkt erst nach einem Neustart.
kat-jetzt-neustarten = Jetzt neu starten

## Version & Updates
upd-st-aktuell = aktuell
upd-st-fehlt = fehlt
upd-st-unbekannt = unbekannt
upd-st-neu = neu (wird gebraucht)
upd-st-manuell = Update nur im Browser (CurseForge)
upd-st-reenable = passt wieder – wird eingeschaltet
upd-st-doppelt = doppelt (ist schon als Mod da)
upd-alles-aktuell = Alles ist auf dem neuesten Stand.
upd-zusammenfassung = { $updates } Updates, { $aktuell } schon aktuell, { $fehlend } ohne passende Version, { $unbekannt } unbekannt
upd-neu-gebraucht = { $anzahl } werden neu gebraucht
upd-passen-wieder = { $anzahl } passen wieder
upd-doppelt = { $anzahl } doppelt (wird beiseitegelegt)
upd-server-stand = Neuer Server-Stand für Minecraft { $mc }: { $label }.
upd-beta-hinweis = Hinweis: Dafür gibt es bisher nur eine Test-/Beta-Version.
upd-wechsel = Server: Minecraft { $von } → { $nach } ({ $label }).
upd-welt-warnung = Achtung: Eine Welt, die einmal mit der neuen Version lief, kann man nicht zurückstufen. Zurück geht nur über das Backup, das automatisch gemacht wird.
upd-java-da = Java { $version } nötig – vorhanden ({ $pfad }).
upd-java-fehlt = Java { $version } nötig, aber nicht installiert. { $hinweis }
upd-manuell-hinweis = Für Mods mit „Update nur im Browser“ erlaubt der Autor kein automatisches Laden – neue Datei auf CurseForge herunterladen und selbst austauschen.
upd-platz = Zu wenig Speicherplatz für das Backup ({ $frei } frei, Server { $server }).
upd-nur-mods = { $version } (nur Mods/Plugins aktualisieren)
upd-keine-neuere = Es gibt keine neuere stabile Minecraft-Version.
upd-keine-verbindung = Keine Verbindung
upd-err-pruefen = Prüfen fehlgeschlagen
upd-rollback-frage = Zurück zum Stand vor dem letzten Update?
upd-rollback-text = Der Server wird gestoppt und aus dem Backup wiederhergestellt, das beim Update gemacht wurde. Der jetzige Stand bleibt als Ordner „…_vor_Wiederherstellung_<Datum>“ daneben liegen.
upd-zuruecksetzen = Zurücksetzen
upd-ev-rollback = Stand vor dem Update wiederhergestellt
upd-rollback-fertig-titel = Zurückgesetzt
upd-rollback-fertig = Der Server ist wieder auf dem Stand von { $label }. Du kannst ihn jetzt starten.
upd-rollback-daneben = Der Stand nach dem Update liegt in:
upd-err-rollback = Zurücksetzen fehlgeschlagen
upd-rollback-knopf = Zurück zum Stand vor dem letzten Update
upd-was-mods = Mods aktualisieren
upd-ablauf = Ablauf: Server stoppen (mit Warnung, falls jemand online ist) → komplettes Backup → { $anzahl } Dateien laden und prüfen → tauschen → Server starten und prüfen.
upd-werden-deaktiviert = { $anzahl } Mod(s)/Datapack(s) ohne passende Version werden deaktiviert.
upd-probelauf-tipp = Tipp: Mit „Probelauf“ kannst du das vorher gefahrlos an einer Kopie testen.
upd-start-frage = Update starten? ({ $was })
upd-starten = Update starten
upd-ev-gestartet = Update gestartet ({ $was })
upd-err-update = Update fehlgeschlagen
upd-ev-fertig = Update fertig ({ $was })
upd-fertig-titel = Update fertig
upd-fertig-1 = Der Server läuft wieder ({ $was }).
upd-fertig-backup = Backup vom alten Stand:
upd-fertig-aufgeraeumt = Aufgeräumt: { $anzahl } alte Dateien/Ordner gelöscht ({ $groesse }).
upd-fertig-2 = Mods ohne passende Version sind ausgeschaltet und unter „Mods/Plugins“ zu sehen.
upd-fehlgeschlagen-titel = Update hat nicht geklappt
upd-fehlgeschlagen-frage = Schau in der Konsole nach der Fehlermeldung. Soll ich den Stand vor dem Update aus dem Backup wiederherstellen?
upd-trial = Probelauf
upd-trial-starten = Probelauf starten
upd-trial-frage = Probelauf starten?
upd-trial-text = Ich kopiere den Server in den Temp-Ordner, aktualisiere die Kopie, starte sie auf einem anderen Port und schaue, ob sie fertig hochfährt. Danach wird die Kopie gestoppt und gelöscht.
    
    Dein echter Server bleibt dabei unverändert und kann weiterlaufen. Bei großen Welten dauert das Kopieren ein paar Minuten.
upd-trial-tipp = Aktualisiert eine Kopie im Temp-Ordner und startet sie auf einem anderen Port
upd-trial-err = Probelauf fehlgeschlagen
upd-trial-ok-titel = Probelauf erfolgreich
upd-trial-ok = Die aktualisierte Kopie ist in { $sekunden } Sekunden fertig gestartet (Port { $port }).
upd-trial-timeout = Die Kopie war nach 15 Minuten noch nicht fertig gestartet.
upd-trial-exit = Die Kopie hat sich beim Start beendet (Exit-Code { $code }).
upd-trial-meldungen = Auffällige Meldungen:
upd-trial-zeilen = Letzte Zeilen:
upd-trial-geloescht = Die Kopie wurde wieder gelöscht.
upd-trial-nicht-geloescht = Die Kopie konnte nicht ganz gelöscht werden: { $ordner }
upd-ev-trial-ok = Probelauf erfolgreich
upd-ev-trial-fehler = Probelauf fehlgeschlagen
upd-jetzt = Jetzt:
upd-erklaerung = Prüft, ob es alle Mods für die gewählte Version gibt. Vor jedem Update wird automatisch ein Backup gemacht.
upd-ziel = Ziel:
upd-versionen = Versionen laden
upd-pruefen = Prüfen
upd-spalte-mod = Mod / Datapack
upd-spalte-neu = Neu
upd-deaktivieren-fehlende = Mods/Datapacks ohne passende Version deaktivieren (sonst startet der Server wahrscheinlich nicht)
upd-deaktivieren-unbekannte = Unbekannte Mods/Datapacks (nicht auf Modrinth) deaktivieren
upd-nicht-unterstuetzt = Den Update-Assistenten gibt es für Fabric, Paper, NeoForge und Vanilla.
upd-export-knopf = Mods für Freunde exportieren (.mrpack)
upd-export-tipp = Nur für Fabric- und NeoForge-Server (Mods, die auch die Freunde brauchen)
upd-export-titel = Modpack gespeichert
upd-export-mods = { $anzahl } Mods von Modrinth
upd-export-eingepackt = { $anzahl } direkt eingepackt
upd-export-hinweis = Deine Freunde importieren die Datei in der Modrinth-App („Datei hinzufügen“). Reine Server-Mods werden dort nicht installiert.
upd-err-export = Export fehlgeschlagen

## Client
cli-titel = Einladung einlösen
cli-code-erklaerung-1 = Code (beginnt mit 
cli-code-erklaerung-2 = ) einfügen oder die 
cli-code-erklaerung-3 = -Datei öffnen – Vapafane richtet Minecraft mit allen Mods passend zum Server ein.
cli-code-platzhalter = Einladungs-Code hier einfügen (VPF1-…)
cli-pruefen = Einladung prüfen
cli-datei-oeffnen = Einladungs-Datei öffnen …
cli-pruefe-einladung = Prüfe die Einladung …
cli-launcher-da = Minecraft-Launcher gefunden
cli-kein-launcher = Kein Minecraft-Launcher
cli-modrinth-da = Modrinth-App gefunden
cli-meine-server = Meine Server
cli-leer = Noch kein Server eingerichtet – oben eine Einladung einlösen.
cli-kein-konto = Vapafane startet Minecraft nicht selbst und fasst kein Microsoft-Konto an – „Spielen“ öffnet den offiziellen Launcher bzw. die Modrinth-App.
cli-nicht-erreichbar = Server nicht erreichbar
cli-online = Online · { $version } · { $spieler }/{ $max } Spieler
cli-version-passt-nicht = Online, aber Version { $version } – passt nicht
cli-aktualisiert = Server wurde aktualisiert ({ $version }) – neuen Einladungs-Code beim Admin holen
cli-gerade-nicht = gerade nicht erreichbar
cli-modrinth-fehlt = Modrinth-App nicht gefunden
cli-launcher-fehlt = Minecraft-Launcher nicht gefunden
cli-download-seite = Zur Download-Seite
cli-ev-eingerichtet = Client eingerichtet: { $name }
cli-gefunden = gefunden
cli-nicht-gefunden = nicht gefunden
cli-keine-mods = Keine Mods nötig – nur Minecraft { $version }.
cli-optional = optional
cli-nur-browser = nur im Browser ladbar (Link nach dem Einrichten)
cli-nur-datei = fehlt – nur in der Einladungs-Datei
cli-launcher-fehlt-text = Der offizielle Minecraft-Launcher wurde nicht gefunden. Du kannst trotzdem einrichten; das Profil wird angelegt, sobald der Launcher installiert ist und du hier „Aktualisieren“ drückst.
cli-launcher-laden = Launcher herunterladen
cli-nie-gestartet = Der Launcher wurde noch nie gestartet. Bitte einmal starten und wieder schließen, dann hier einrichten.
cli-ordner-waehlen = Ordner für dieses Minecraft wählen
cli-mods-zusammen = { $anzahl } Mods, zusammen { $groesse }
cli-andere-version = Der Server meldet eine andere Version ({ $version }). Wahrscheinlich wurde er aktualisiert – bitte beim Admin einen neuen Einladungs-Code holen.
cli-spalte-version = Version
cli-spalte-groesse = Größe
cli-einrichten-fuer = Einrichten für:
cli-in-modrinth = In der Modrinth-App:
cli-instanz-update = Vorhandene Instanz „{ $name }“ aktualisieren (empfohlen)
cli-instanz-update-sub = Alles bleibt in derselben Instanz: Welten, Einstellungen, Shader, Ressourcenpakete und Mods, die du selbst hinzugefügt hast (z. B. Iris). Dafür klickst du in der Modrinth-App kurz „Modpack neu installieren“ – Vapafane zeigt, wie.
cli-instanz-neu = Neue Instanz anlegen
cli-instanz-neu-sub = Geht automatisch über den Import. Einstellungen, Shader und Ressourcenpakete kannst du unten übernehmen – selbst hinzugefügte Mods nicht. Die alte Instanz bleibt, bis du sie löschst.
cli-update-bleibt = Beim Aktualisieren bleibt in der Instanz alles erhalten: Welten, Optionen, Shader, Ressourcenpakete und deine Server-Liste.
cli-import-bestaetigen = Die Modrinth-App öffnet sich – dort nur noch den Import bestätigen.
cli-ordner-label = Ordner:
cli-launcher-schliessen = Bitte den Minecraft-Launcher vor dem Einrichten schließen, damit er die Änderung nicht überschreibt.
cli-einrichten = Einrichten
cli-uebernehmen-von = Einstellungen übernehmen von:
cli-nichts = Nichts übernehmen
cli-quelle-waehlen = Minecraft-Ordner wählen, aus dem übernommen wird
cli-nichts-uebernehmen = Nichts zu übernehmen
cli-teil-options = Optionen & Tastenbelegung
cli-teil-config = Mod-Einstellungen
cli-teil-shaders = Shader
cli-teil-resourcepacks = Ressourcenpakete
cli-teil-servers = Server-Liste
cli-teil-worlds = Welten
cli-welten-warnung = Welten aus einer neueren Minecraft-Version lassen sich in einer älteren nicht öffnen. Große Welten machen das Einrichten langsamer.
cli-nur-fehlendes = Was im Ordner schon liegt, bleibt unverändert – übernommen wird nur, was fehlt.
cli-uebernehmen-erklaerung = Zum Beispiel aus deiner bisherigen Instanz: Optionen, Tastenbelegung, Mod-Einstellungen, Shader, Ressourcenpakete und Server-Liste. Mods kommen immer passend vom Server. Die Quelle wird nur gelesen, nicht verändert.
cli-paket-update-1 = Das neue Modrinth-Paket ist fertig. So aktualisierst du deine Instanz
cli-paket-update-2 = Welten, Optionen, Shader, Ressourcenpakete und deine Server-Liste bleiben dabei erhalten:
cli-schritt-instanz = Modrinth-App öffnen und links die Instanz
cli-schritt-anklicken = anklicken.
cli-schritt-zahnrad = Oben auf das Zahnrad „Instanz-Einstellungen“ klicken und den Reiter „Installation“ wählen.
cli-neu-installieren = Modpack neu installieren
cli-schritt-datei = klicken, bestätigen und diese Datei wählen:
cli-modrinth-oeffnen = Modrinth-App öffnen
cli-paket-ordner = Ordner mit dem Paket öffnen
cli-update-sicher = Die Modrinth-App sichert die Instanz vorher selbst und tauscht nur die Mods des Servers aus – was du selbst hinzugefügt hast (Shader, Ressourcenpakete, eigene Mods), bleibt.
cli-paket-offen = Das Modrinth-Paket ist fertig, und die Modrinth-App wurde damit geöffnet. Bestätige dort den Import – dann erscheint die Instanz
cli-paket-fertig = Das Modrinth-Paket ist fertig:
cli-paket-von-hand = Die Modrinth-App ließ sich nicht automatisch öffnen. So geht es von Hand: in der Modrinth-App auf „+“ (Instanz erstellen) klicken, dann „Aus Datei importieren“ (englisch „Import from file“) und diese Datei wählen.
cli-eingerichtet = Eingerichtet: { $anzahl } Mods geladen.
cli-profil = Im Minecraft-Launcher gibt es jetzt das Profil „{ $name }“.
cli-uebernommen-paket = Die Einstellungen aus „{ $quelle }“ sind im Paket – die Modrinth-App übernimmt sie beim Import.
cli-uebernommen = Aus „{ $quelle }“ übernommen: { $anzahl } Dateien.
cli-uebernommen-da = ({ $anzahl } waren schon da und blieben unverändert)
cli-uebernommen-fehler = { $anzahl } Dateien ließen sich nicht lesen und fehlen deshalb – ist Minecraft dort noch offen? ({ $dateien })
cli-selbst-laden = Bitte selbst herunterladen und in den Ordner „mods“ legen:
cli-spielen = Spielen
cli-spielen-modrinth = Spielen (Modrinth-App öffnen)
cli-spielen-launcher = Spielen (Launcher öffnen)
cli-prueft = prüfe …
cli-unvollstaendig = Einrichten wurde nicht fertig – bitte die Einladung oben nochmal einlösen (oder entfernen).
cli-mods-kurz = { $anzahl } Mods
cli-eingerichtet-am = { $ziel } · eingerichtet { $datum }
cli-aktualisieren = Aktualisieren
cli-aktualisieren-tipp = Mit der gespeicherten Einladung neu einrichten – oder erklären, was nach einem Server-Update zu tun ist
cli-entfernen-frage = „{ $name }“ entfernen?
cli-entfernen-modrinth = Vapafane vergisst diesen Client und löscht sein Paket. Die Instanz in der Modrinth-App bleibt – die bitte dort selbst löschen.
cli-entfernen-launcher = Der Ordner mit Mods, Einstellungen und Einzelspieler-Welten dieses Profils wird gelöscht, und das Profil verschwindet aus dem Minecraft-Launcher.
cli-upd-titel = Server wurde aktualisiert
cli-upd-text = „{ $name }“ läuft jetzt mit Minecraft { $version }. Welche Version und welche Mods dazugehören, steht in der Einladung – dafür brauchst du einen neuen Einladungs-Code vom Admin.
cli-upd-code = Füg ihn oben unter „Einladung einlösen“ ein und klick „Einladung prüfen“.
cli-upd-modrinth = Vapafane baut dann ein neues Paket und zeigt, wie du deine Instanz damit aktualisierst – deine Einstellungen, Welten und Shader bleiben.
cli-upd-launcher = Vapafane tauscht dann die Mods aus – deine Einstellungen und Welten bleiben.
cli-gespeicherte-einladung = Gespeicherte Einladung von „{ $name }“ geladen. Mit „Einrichten“ wird alles nochmal passend eingerichtet – zum Beispiel, wenn Mods fehlen oder das Profil verschwunden ist. Hat der Admin den Server aktualisiert, brauchst du stattdessen einen neuen Einladungs-Code.
ext-knopf = Shader & Extras
ext-knopf-so-gehts = Shader & Extras – so geht's

## Shader & Extras
ext-titel = Shader & Extras hinzufügen
ext-skizze-1-alt = Skizze der Instanz in der Modrinth-App
ext-skizze-2-alt = Skizze der Suche in der Modrinth-App
ext-skizze-leer = Keine zusätzlichen Inhalte installiert
ext-skizze-hochladen = Dateien hochladen
ext-durchsuchen = Inhalte durchsuchen
ext-skizze-shader = Ein Shader, z. B. Complementary
ext-skizze-beschreibung = Beschreibung · Downloads
ext-keine-shader = Shader brauchen die Mod Iris – und die läuft nur auf Servern mit Fabric oder NeoForge. Dieser Server ist ohne Mod-Loader, deshalb gehen hier Ressourcenpakete, aber keine Shader.
ext-iris-modrinth = Für Shader braucht Minecraft die Mod Iris: In Schritt 4 zuerst „Mods“ wählen und „Iris Shaders“ installieren (Sodium kommt automatisch mit), danach unter „Shaders“ einen Shader.
ext-iris-launcher-1 = Für Shader braucht Minecraft die Mod Iris (mit Sodium): Beide passend zur Minecraft-Version des Servers auf
ext-iris-launcher-2 = herunterladen und in den Ordner „mods“ legen.
ext-mr-einleitung-1 = In deiner Instanz
ext-mr-einleitung-2 = kannst du jederzeit eigene Shader, Ressourcenpakete und Mods hinzufügen. Sie bleiben bei Server-Updates erhalten – die Modrinth-App tauscht dabei nur die Mods des Servers aus.
ext-schritt-content = Oben den Reiter „Content“ (Inhalte) wählen.
ext-schritt-durchsuchen = Auf den grünen Knopf „Inhalte durchsuchen“ klicken.
ext-schritt-art = Oben die Art wählen: „Shaders“ für Shader, „Resource Packs“ für Ressourcenpakete oder „Mods“.
ext-schritt-installieren = Beim gewünschten Eintrag auf „Installieren“ klicken – fertig.
ext-im-spiel = Im Spiel: Optionen → Grafikeinstellungen → Shader-Pakete → Shader auswählen.
ext-im-spiel-rp = Im Spiel: Optionen → Ressourcenpakete → Paket auswählen.
ext-mods-update-hinweis = Eigene Mods musst du nach einem Minecraft-Update in der App selbst aktualisieren; Shader und Ressourcenpakete meist nicht.
ext-launcher-einleitung = Der Spielordner dieses Servers gehört nur ihm: Was du dort hinzufügst, bleibt bei Server-Updates erhalten.
ext-schritt-laden-shader = Shader herunterladen, z. B. auf
ext-schritt-laden-rp = Ressourcenpaket herunterladen, z. B. auf
ext-nicht-entpacken = die .zip-Datei nicht entpacken.
ext-schritt-shaderpacks = Unten auf „Ordner öffnen“ klicken und die Datei in den Ordner „shaderpacks“ legen. Ressourcenpakete gehören in „resourcepacks“.
ext-schritt-resourcepacks = Unten auf „Ordner öffnen“ klicken und die Datei in den Ordner „resourcepacks“ legen.
ext-eigene-mods = Eigene Mods kannst du in den Ordner „mods“ legen – nach einem Minecraft-Update musst du sie aber selbst passend halten.
err-antwort-unlesbar = Antwort von „{ $befehl }“ unlesbar: { $fehler }

## Erreichbarkeits-Bericht
rch-w-erreichbar = erreichbar
rch-w-nicht = nicht erreichbar
rch-w-nicht-dslite = nicht erreichbar (DS-Lite)
rch-w-nicht-geprueft = nicht geprüft
rch-w-nicht-vorhanden = nicht vorhanden
rch-unmoeglich-titel = Prüfung gerade nicht möglich
rch-unmoeglich-text = Der Prüfdienst im Internet antwortet nicht. Später erneut prüfen.
rch-gruen-beide = Über IPv4 und IPv6 erreichbar
rch-gruen-v4 = Über IPv4 erreichbar
rch-gruen-text = Alle Freunde kommen rein.
rch-gelb-dslite-titel = DS-Lite: nur über IPv6 erreichbar
rch-gelb-dslite-text = Dein Anbieter teilt die IPv4-Adresse. Freunde mit IPv6 kommen trotzdem rein, das sind die meisten.
rch-gelb-titel = Nur über IPv6 erreichbar
rch-gelb-text = Über IPv4 kommt niemand rein: Entweder fehlt die Portfreigabe für IPv4 (Port { $port }), oder dein Anbieter teilt die IPv4-Adresse (DS-Lite). Freunde mit IPv6 kommen trotzdem rein.
rch-rot-titel = Von außen nicht erreichbar
rch-rot-text = Meist fehlt noch die Portfreigabe im Router, oder die Firewall am PC blockt Port { $port }.
rch-rot-dslite-v6 = Dein Anbieter teilt die IPv4-Adresse (DS-Lite), es geht also nur über IPv6. Gib im Router die IPv6-Freigabe für Port { $port } frei, und prüf die Firewall am PC.
rch-rot-dslite = Dein Anbieter teilt die IPv4-Adresse (DS-Lite), und dein PC hat keine IPv6-Adresse. So kommt von außen niemand an den Server.
rch-aus-titel = Server läuft nicht
rch-aus-text = Geprüft wird, sobald der Server läuft.
rch-port-belegt-titel = Port { $port } ist schon belegt
rch-port-belegt-text = Ein anderes Programm – wahrscheinlich ein anderer Minecraft-Server – benutzt diesen Port. Dieser Server kann so nicht starten. Stell unter Einstellungen → „Server-Port“ einen freien Port ein (z. B. { $frei })
rch-hotspot-tipp = Der Server ist nur über IPv6 erreichbar, dein Internet hat aber kein IPv6. Tipp: Verbinde deinen PC über den Hotspot deines Handys – Mobilfunk hat meistens IPv6.

## Programm-Fehler (allgemein)
err-dienst-aus = Hintergrund-Dienst läuft nicht
err-kein-systemd = systemd ist hier nicht verfügbar (oder Testmodus).
err-kein-server = Kein Server ausgewählt.
err-schon-beschaeftigt = Es läuft gerade schon: { $was }
err-neustart-extern = Dieser Server läuft außerhalb von Vapafane – Neustart geht nur über das Programm, das ihn gestartet hat.
err-stop-knopf = Bitte den Stopp-Knopf in der Übersicht benutzen (der speichert sicher).
err-start-extern = Der Server läuft außerhalb von Vapafane. Bitte zuerst dort stoppen.
err-start-anderer = Ein anderer Server läuft gerade. Bitte zuerst stoppen.
err-stop-unmoeglich = Der Server ließ sich nicht stoppen.
err-pfad = Ungültiger Pfad
err-rcon-antwort = Ungültige RCON-Antwort
err-adresse = Adresse ungültig
cln-alte-neoforge = ältere NeoForge-Version
cln-alte-bibliothek = ältere Version einer Bibliothek
cln-alte-mods = alte Mod-Versionen (vom früheren Panel/Launcher beiseitegelegt)
cln-cache = Zwischenspeicher der Mods (wird beim nächsten Start neu erzeugt)
cli-datei-titel = Einladungs-Datei öffnen
srv-rcon-an = RCON eingeschaltet (braucht Vapafane für Befehle).
err-link-verboten = Diese Adresse darf nicht geöffnet werden.
err-pruefsumme = Prüfsumme stimmt nicht: { $datei }
err-kein-fabric = Kein Fabric-Loader für Minecraft { $mc } gefunden
cf-err-leer = Bitte zuerst einen Schlüssel einfügen.
cf-err-id = Ungültige CurseForge-ID
cf-err-format = Das sieht nicht wie ein API-Schlüssel aus (bitte ohne Leerzeichen einfügen).
cf-err-antwort = Ungültige Antwort von CurseForge: { $fehler }
cf-err-abgelehnt = CurseForge lehnt den API-Schlüssel ab (ungültig oder noch nicht freigeschaltet).
cf-err-fehlt = Kein CurseForge-Schlüssel hinterlegt.
kat-err-typ = Für diesen Server-Typ gibt es „Mods hinzufügen“ nicht.
kat-err-keine-version = „{ $name }“ gibt es nicht für Minecraft { $mc } ({ $loader }).
kat-err-cf-version = „{ $name }“ gibt es auf CurseForge nicht für Minecraft { $mc }.
fort-suche = Suche { $name } …
fort-suche-cf = Suche { $name } auf CurseForge …
inv-err-neuer = Dieser Einladungs-Code ist für eine neuere Vapafane-Version. Bitte Vapafane aktualisieren.
inv-err-unvollstaendig = Der Code ist unvollständig oder beschädigt – bitte ganz kopieren.
inv-err-kaputt = Der Code ist beschädigt – bitte nochmal kopieren.
inv-err-version = Diese Einladung ist für eine andere Vapafane-Version.
inv-err-angaben = Die Einladung enthält ungültige Angaben.
inv-err-mod = Die Einladung enthält eine ungültige Mod-Angabe.
inv-err-typ = Einladungen gibt es für Fabric-, NeoForge-, Paper- und Vanilla-Server.
inv-err-datei = Das ist keine gültige Vapafane-Einladungs-Datei.
inv-err-datei-kaputt = Die Einladung in der Datei ist beschädigt.
inv-err-oeffentlich = Die öffentliche Adresse ließ sich nicht ermitteln.
flv-err-kein-server = Für Minecraft { $mc } gibt es keinen Server.
flv-err-kein-paper = Kein Paper für { $mc }.
flv-err-kein-neoforge = Für Minecraft { $mc } gibt es kein NeoForge.
fort-neoforge = NeoForge-Installer richtet den Server ein (lädt Bibliotheken) …
fort-suche-server = Suche { $was } für Minecraft { $mc } …
fort-pruefe-java = Prüfe Java …
fort-frage-cf = Frage CurseForge nach den übrigen Mods …
fort-pruefe-deps = Prüfe Abhängigkeiten …
fort-pruefe-dp = Prüfe Datapacks …
fort-pruefe-java-platz = Prüfe Java und Speicherplatz …
upd-err-dateiname = Ungültiger Dateiname von Modrinth: { $name }
upd-cf-fehler = CurseForge konnte nicht geprüft werden: { $fehler }
upd-abhaengigkeit = (Abhängigkeit)
upd-grund-fehlt = gibt es nicht für Minecraft { $mc }
cli-err-port = Ungültiger Port in der Adresse
cli-err-antwort = Ungültige Antwort vom Server
cli-err-profil-kaputt = { $datei } ist beschädigt – bitte den Launcher einmal starten.
cli-err-nicht-leer = Der Ordner { $ordner } ist nicht leer und gehört nicht zu Vapafane.
cli-err-datei-kaputt = Datei { $datei } aus der Einladung ist beschädigt.
cli-err-fabric-profil = Fabric-Profil ungültig.
cli-err-neoforge-java = Für NeoForge wird Java { $version } gebraucht. Bitte den Minecraft-Launcher einmal starten (er bringt Java mit) oder Java installieren.
cli-hinweis-launcher-starten = Bitte den Minecraft-Launcher einmal starten und wieder schließen, dann nochmal einrichten.
cli-hinweis-kein-launcher = Der offizielle Minecraft-Launcher wurde nicht gefunden. Bitte installieren, einmal starten und dann hier „Aktualisieren“ drücken.
cli-hinweis-nie-gestartet = Der Launcher wurde noch nie gestartet. Bitte einmal starten und wieder schließen, dann „Aktualisieren“.
cli-paket-summary = Eingerichtet von Vapafane für den Server { $name } ({ $adresse })
fort-pruefe-mods = Prüfe die Mods …
fort-lade = Lade { $name } …
fort-launcher = Richte den Launcher ein …
fort-neoforge-install = Installiere NeoForge { $version } (kann ein paar Minuten dauern) …
fort-uebernehme = Übernehme Einstellungen …
fort-trage-ein = Trage den Server ein …
fort-sammle = Sammle die Einstellungen …
fort-packe = Packe das Modrinth-Paket …
fort-fertig = Fertig.
task-update = Update
task-backup = Backup (ganzer Server als ZIP)
task-restore = Backup wiederherstellen
task-client = Client einrichten
task-rollback = Zurück zum Stand vor dem Update
task-pruefe-mods = Prüfe Mods für { $mc }
upd-err-erst-pruefen = Bitte zuerst „Prüfen“ drücken.
upd-err-anderer-server = Die Prüfung gehört zu einem anderen Server. Bitte neu prüfen.
upd-err-absturz = Der Server ist beim Start abgestürzt. { $grund }
upd-err-beendet = Der Server hat sich direkt wieder beendet.
upd-err-kein-backup = Für diesen Server gibt es kein Backup vom letzten Update.
upd-err-temp-platz = Zu wenig Platz im Temp-Ordner für eine Kopie ({ $groesse } nötig).
cln-err-laeuft = Aufräumen geht nur, wenn der Server gestoppt ist.
fort-raeume-auf = Räume alte Dateien auf …
fort-loesche = Lösche alte Dateien …
fort-loesche-kopie = Lösche die Kopie …
nbt-err-unvollstaendig = servers.dat ist unvollständig.
nbt-err-laenge = servers.dat enthält eine ungültige Länge.
nbt-err-eintrag = servers.dat enthält einen unbekannten Eintrag.
nbt-err-tiefe = servers.dat ist zu tief verschachtelt.
nbt-err-format = servers.dat hat ein unbekanntes Format.
say-backup-start = Backup läuft …
say-backup-fertig = Backup fertig.
say-backup-fehler = Backup fehlgeschlagen.
say-stoppt = stoppt
say-neustart = startet neu
say-countdown = Server { $was } in { $sekunden } Sekunden!
svc-countdown = Server { $was } in { $sekunden } s
svc-err-laeuft-nicht = Der Server läuft gerade nicht.
svc-err-befehl = Unbekannter Befehl: { $befehl }
svc-err-ordner-fehlt = Server-Ordner fehlt: { $ordner }
svc-err-eula = Die Minecraft-EULA ist für diesen Server noch nicht akzeptiert.
svc-err-fremder-server = In diesem Ordner läuft schon ein Server (PID { $pid }), der nicht von Vapafane gestartet wurde. Bitte den zuerst beenden (z. B. altes Bedienfenster).
svc-err-port = Port { $port } ({ $art }) ist schon belegt. Läuft noch ein anderer Minecraft-Server auf diesem PC?
svc-signal = durch Signal beendet
svc-beendet = beendet
svc-abgestuerzt = Abgestürzt ({ $grund }).
svc-taegliches-backup = Tägliches Backup
say-update-neustart = Server startet für ein Update neu!
svc-err-java-start = Java konnte nicht gestartet werden ({ $java }): { $fehler }

## Befehlszeile
ctl-hilfe = Benutzung: vapafane --ctl <befehl>
    
      status            Zustand des aktiven Servers anzeigen
      start             Server starten (startet bei Bedarf den Hintergrund-Dienst)
      stop [SEKUNDEN]   Server sauber stoppen, optional mit Countdown-Warnung
      restart [SEK.]    Server neu starten
      wait-ready [SEK.] warten, bis der Server bereit ist
      wait-stopped [S.] warten, bis der Server aus ist
      backup            ganzen Server-Ordner als ZIP sichern
      cmd <BEFEHL …>    Befehl per RCON schicken
      aktiv NAME        anderen Server aus der Liste aktiv machen (nur wenn gestoppt)
      neu TYP NAME VER [ORDNER]  neuen Server anlegen (fabric, paper, neoforge, vanilla) und aktiv machen
      update-check VER  Mods/Datapacks für Minecraft VER über Modrinth prüfen
      trial VER         Probelauf: Kopie aktualisieren, auf anderem Port testen, löschen
      update VER        Update: stoppen, Backup, tauschen, starten (nur wenn Prüfung ok)
      mods              Mods und Datapacks mit An/Aus-Zustand anzeigen
      mod-an PFAD       Mod einschalten (PFAD wie in „mods“, z. B. mods/x.jar)
      mod-aus PFAD      Mod ausschalten (wird zu .jar.disabled)
      mod-entfernen PFAD  Mod/Datapack endgültig löschen
      aufraeumen [--ja] zeigt alte Reste; mit --ja werden sie gelöscht (Server muss aus sein)
      mod-suche [BEGRIFF] [--datapack]   Modrinth nach passenden Mods/Plugins (bzw. Datapacks) durchsuchen
      mod-install PROJEKT [--datapack]  Mod/Plugin/Datapack samt Pflicht-Abhängigkeiten installieren
      curseforge-schluessel [--loeschen]  CurseForge-Schlüssel von der Eingabe lesen und speichern
      curseforge-test   gespeicherten CurseForge-Schlüssel ausprobieren
      einladung [ADRESSE]               Einladungs-Code für Freunde ausgeben (Adresse = host:port)
      einladung-datei DATEI [ADRESSE]   Einladungs-Datei (.vapafane) schreiben
      einloesen CODE|DATEI [--ordner O] [--alle-optionalen] [--ziel launcher|modrinth]
                … [--einstellungen-von ORDNER] [--neue-instanz]
                                        Client für eine Einladung einrichten (Launcher-Profil oder Modrinth-Paket);
                                        Einstellungen (ohne Welten) aus ORDNER übernehmen; mit --neue-instanz wird
                                        eine vorhandene Instanz in der Modrinth-App nicht aktualisiert
      clients                           eingerichtete Clients anzeigen
      client-entfernen ORDNER           Client und sein Launcher-Profil entfernen
      ping ADRESSE                      Server-Status abfragen (wie die Serverliste im Spiel)
      erreichbarkeit    prüfen, ob der Server von außen erreichbar ist (IPv4/IPv6, DS-Lite)
      mrpack DATEI      Mods als Modrinth-Modpack (.mrpack) für Freunde exportieren
      quit-service      Hintergrund-Dienst beenden (stoppt vorher den Server)
ctl-fehler = Fehler: { $fehler }

## Programm-Meldungen (Teil 3)
err-archivformat = Unbekanntes Archivformat (erlaubt: .zip, .tar, .tar.gz, .tar.zst)
fort-entpacke = Entpacke { $datei }
err-ordner-nicht-leer = Der Ordner { $ordner } ist nicht leer.
err-aufgabenplanung = Die Aufgabenplanung hat nicht geklappt (abgebrochen oder keine Berechtigung). { $details }
err-mc-unbekannt = Die Minecraft-Version des Servers ist unbekannt – bitte den Server einmal starten.
kat-err-projekt = Das Projekt „{ $projekt }“ gibt es auf Modrinth nicht.
kat-err-schon-da = „{ $name }“ ist schon installiert.
kat-err-keine-datei = „{ $name }“ hat keine herunterladbare Datei.
fort-sortiere = Sortiere ein …
kat-err-installiert = Das ist schon installiert.
fort-lade-cf = Lade { $name } von CurseForge …
cln-alte-mc-dateien = Dateien einer alten Minecraft-Version
cln-alte-server-datei = Server-Datei einer alten Version
cln-alte-mc-datei = Minecraft-Datei einer alten Version
cln-vor-update = Dateien von vor einem Update (das ZIP-Backup davon bleibt)
cln-ersetzt = ersetzte Dateien aus diesem Update
cli-err-keine-adresse = Keine Serveradresse angegeben.
cli-err-adresse = Adresse „{ $adresse }“ nicht gefunden.
cli-err-nicht-erreichbar = Server nicht erreichbar (aus, falsche Adresse oder Port nicht freigegeben).
cli-err-mod-weg = Eine Mod aus der Einladung gibt es auf Modrinth nicht mehr ({ $id }).
cli-err-hash = Eine Mod-Datei passt nicht zur Einladung (die Prüfsumme stimmt nicht).
cli-nur-in-datei = nur in der Einladungs-Datei enthalten
err-neoforge-installer = Der NeoForge-Installer ist fehlgeschlagen:
    { $details }
cli-err-kein-client = Das ist kein Vapafane-Client.
cli-err-launcher-fehlt = Der offizielle Minecraft-Launcher wurde nicht gefunden. Download: { $link }
cli-keine-app = (keine App)
cli-test-oeffnen = Testmodus – würde öffnen: { $wie } { $pfad }
inv-datei-titel = Einladungs-Datei speichern
inv-datei-name = Einladung
inv-datei-filter = Vapafane-Einladung
err-ziel-unbekannt = Unbekanntes Ziel
cf-err-404 = Bei CurseForge nicht gefunden.
cf-err-429 = CurseForge meldet zu viele Anfragen – bitte kurz warten.
cf-err-code = CurseForge antwortet mit Fehler { $code }.
cf-err-bereich = CurseForge-Bereich „{ $bereich }“ nicht gefunden.
err-mc-gibt-es-nicht = Minecraft { $mc } gibt es bei Mojang nicht.
err-paper-download = Für diese Version gibt es keinen Paper-Download.
err-neoforge-start = Der NeoForge-Installer konnte nicht starten ({ $java }): { $fehler }
err-rcon-weg = Keine RCON-Verbindung zum Server.
fort-stoppe = Stoppe den Server (speichert die Welt) …
err-ordner-unbekannt = Unbekannter Ordner
err-fenster = Das Fenster konnte nicht gestartet werden: { $fehler }
inv-err-kein-code = Das ist kein Vapafane-Einladungs-Code (er beginnt mit „VPF1-“).
inv-err-typ-unbekannt = Unbekannter Server-Typ in der Einladung.
inv-err-datei-leer = In der Datei fehlt die Einladung (vapafane.json).
err-dienst-antwort = Keine Antwort vom Hintergrund-Dienst.
err-dienst-fehler = Fehler im Hintergrund-Dienst.
err-dienst-start = Der Hintergrund-Dienst konnte nicht gestartet werden: { $details }
err-dienst-stumm = Der Hintergrund-Dienst antwortet nicht. Details stehen in { $datei }
err-umschalten-laeuft = Umschalten geht nur, wenn der aktuelle Server gestoppt ist.
err-aufgabe-laeuft = Bitte warten, bis die laufende Aufgabe fertig ist.
err-ordner-fehlt = Ordner fehlt: { $ordner }
err-kein-server-ordner = In { $ordner } liegt weder eine Server-Jar noch eine server.properties.
err-ordner-doppelt = Dieser Ordner ist schon in der Liste.
err-name-fehlt = Bitte einen Namen eingeben.
task-import = Importiere „{ $name }“
err-typ-unbekannt = Unbekannter Server-Typ
task-erstelle = Erstelle { $typ }-Server „{ $name }“
err-sprache-unbekannt = Unbekannte Sprache
err-erst-stoppen = Bitte den Server erst stoppen.
err-port-zahl = Der Port muss eine Zahl sein.
err-port-bereich = Der Port muss zwischen 1024 und 65535 liegen.
err-uhrzeit = Bitte eine Uhrzeit wie 05:00 angeben.
err-noch-kein-server = Noch kein Server angelegt.
err-http-code = Der Server antwortet mit Fehler { $code }.
err-keine-verbindung = Keine Verbindung: { $fehler }
err-zu-viele-anfragen = { $host } meldet zu viele Anfragen. Bitte ein paar Minuten warten und es dann nochmal versuchen.
err-fabric-installer = Kein Fabric-Installer gefunden.
err-andere-aufgabe = eine andere Aufgabe
fort-speichert = Der Server speichert die Welt …
err-port-belegt = Port { $port } ist schon belegt – wahrscheinlich läuft dort ein anderer Minecraft-Server. Stell unter Einstellungen → „Server-Port“ einen freien Port ein, z. B. { $frei }.
err-rcon-aus = RCON ist in server.properties ausgeschaltet.
err-rcon-nicht-verbunden = RCON ist nicht verbunden.
err-rcon-passwort = Das RCON-Passwort ist falsch.
err-rcon-erreichbar = RCON ist nicht erreichbar.
err-keine-jar = Keine Server-Jar gefunden. Bitte die Jar in den Ordner legen.
err-keine-jar-name = Keine Server-Jar gefunden (MC_JAR={ $jar }).
err-nichts-zu-uebernehmen = In diesem Ordner gibt es nichts zu übernehmen. Gesucht wird ein Minecraft-Spielordner – darin liegen z. B. „options.txt“, „config“, „shaderpacks“ oder „saves“.
cli-quelle-ordner = Ordner: { $ordner }
err-ordner-weg = Der Ordner { $ordner } ist nicht mehr da.
err-teil-unbekannt = Unbekannter Teil: { $teil }
fort-lese-mods = Lese die installierten Mods und Plugins …
fort-lade-namen = Lade die Namen der Mods …
fort-lade-version = Lade { $name } { $version } …
fort-tausche = Tausche die Dateien …
fort-richte-ein = Richte { $name } ein …
fort-getauscht = Dateien getauscht.
err-start-timeout = Der Server war nach 15 Minuten noch nicht fertig gestartet.
fort-warne = Warne die Spieler (30 Sekunden) …
fort-backup = Erstelle das Backup …
fort-starte = Starte den Server (der erste Start kann ein paar Minuten dauern) …
fort-kopiere = Kopiere { $datei }
fort-starte-kopie = Starte die Kopie auf Port { $port } …
fort-stoppe-kopie = Stoppe die Kopie …
ctl-timeout = Zeitüberschreitung (Zustand: { $zustand })
ctl-startet = Server wird gestartet.
ctl-stoppt = Server wird gestoppt.
ctl-neustart = Server wird neu gestartet.
ctl-fehlt-name = Bitte einen Namen angeben.
ctl-nicht-in-liste = „{ $name }“ ist nicht in der Liste.
ctl-neu-benutzung = Benutzung: --ctl neu TYP NAME VERSION [ORDNER]
ctl-fehlt-version = Bitte die Zielversion angeben.
ctl-update-plan = Version { $von } → { $nach }, Java nötig: { $java } (ok: { $javaok }), Platz ok: { $platzok }, etwas zu tun: { $tun }
ctl-java-platz = Java oder Speicherplatz passt nicht.
ctl-fehlt-pfad = Bitte einen Pfad angeben.
ctl-geloescht = Gelöscht: { $groesse } (nicht möglich: { $fehler })
ctl-treffer = { $anzahl } Treffer für Minecraft { $mc } ({ $typ }):
ctl-fehlt-projekt = Bitte ein Projekt angeben.
ctl-cf-geloescht = CurseForge-Schlüssel gelöscht.
ctl-cf-ok = Der CurseForge-Schlüssel funktioniert ({ $schluessel }).
ctl-nur-server = Nur auf dem Server (nicht in der Einladung): { $mods }
ctl-ohne-quelle = Ohne Quelle (nur in der Einladungs-Datei möglich): { $mods }
ctl-fehlt-code = Bitte einen Code oder eine Datei angeben.
ctl-fehlt-ordner = Bitte einen Ordner angeben.
ctl-fehlt-adresse = Bitte eine Adresse angeben.
ctl-fehlt-zieldatei = Bitte eine Zieldatei angeben.
ctl-dienst-ende = Der Hintergrund-Dienst wird beendet.
gem-zitat = „{ $text }“
spi-entfernt = { $name } entfernt.

## Mods: entfernen, Abhängigkeiten, Infos
mods-err-name-belegt = Es gibt schon eine eingeschaltete Datei mit diesem Namen.
mods-err-ordner-aus = Datapack-Ordner lassen sich nicht ausschalten – nur ZIP-Datapacks. Entfernen geht aber.
mods-err-weg = Die Datei gibt es nicht mehr.
mods-grund-hand = von Hand ausgeschaltet
mods-grund-unbekannt = nicht auf Modrinth – passende Version unklar
mods-grund-doppelt = doppelt – „{ $name }“ ist schon als Mod installiert
mods-ev-entfernt = { $name } entfernt
mods-entfernen-tipp = Endgültig entfernen
mods-entfernen-frage = „{ $name }“ entfernen?
mods-entfernen-text = Die Datei wird endgültig gelöscht. Im letzten Backup ist sie noch enthalten.
mods-gebraucht-warnung = Achtung, das brauchen noch: { $namen }. Ohne diese Datei startet der Server vermutlich nicht.
mods-waisen-frage = Diese Helfer braucht danach kein anderer Mod mehr – auch entfernen?
mods-aus-frage = „{ $name }“ ausschalten?
mods-aus-knopf = Trotzdem ausschalten
mods-err-entfernen = Entfernen hat nicht geklappt
mods-gebraucht-von = Gebraucht von: { $namen }
mods-bibliothek = Bibliothek
mods-bibliothek-tipp = Ein Helfer, den andere Mods brauchen
mods-seite = Seite
mods-seite-tipp = Seite der Mod im Browser öffnen – dort steht, was sie macht
mods-suche = In der Liste suchen …

## Empfohlene Leistungs-Mods
empf-badge = Empfohlen
empf-badge-tipp = Von Vapafane empfohlen: bewährt für bessere Leistung, ändert nichts am Spielablauf
empf-titel = Empfohlen für bessere Leistung
empf-text = Diese Mods machen den Server schneller und sparsamer, ohne am Spielablauf etwas zu ändern. Sie laufen nur auf dem Server – deine Freunde müssen dafür nichts installieren.
empf-alle = Alle empfohlenen installieren
empf-mitinstallieren = Empfohlene Leistungs-Mods mitinstallieren
empf-sucht = Suche passende Leistungs-Mods …
empf-paper = Paper bringt schon eigene Leistungs-Optimierungen mit – dafür sind keine zusätzlichen Plugins nötig.
empf-lithium = Schnellere Spiellogik (Mobs, Redstone, Blöcke)
empf-ferritecore = Deutlich weniger Arbeitsspeicher
empf-krypton = Entlastet die Netzwerkverbindung zu den Spielern
empf-c2me = Lädt und erzeugt die Welt schneller (nutzt mehrere Prozessorkerne)
empf-scalablelux = Schnellere Lichtberechnung, besonders beim Erkunden
empf-modernfix = Schnellerer Start, weniger Arbeitsspeicher
empf-ev = Empfohlene Mods installiert: { $namen }
empf-installiert = Installiert (in „{ $ordner }“): { $namen }.
empf-err-titel = Nicht alle empfohlenen Mods installiert
empf-err-text = Der Server ist fertig, nur diese empfohlenen Mods ließen sich nicht installieren: { $namen }. Du kannst sie später unter „Mods“ → „Mods hinzufügen“ nachholen.
fort-suche-empf = Suche die empfohlenen Mods …
task-empf = Installiere empfohlene Mods

## Programm-Meldungen (Teil 4)
task-installiere = Installiere { $name }
fort-frage-modrinth = Frage Modrinth nach den Mods …
upd-export-speichern = Modpack speichern
upd-export-filter = Modrinth-Modpack
task-backup-ctl = Backup (Befehlszeile)
task-mod-installieren = Mod installieren
cli-grund-browser = Download nur im Browser (CurseForge)
err-archiv-pfad = Unsichere Pfadangabe im Archiv: { $pfad }
ctl-einladung-gespeichert = { $datei }: { $mods } Mods ({ $dateien } direkt eingepackt)
