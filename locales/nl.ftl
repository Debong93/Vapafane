# Vapafane – Nederlands (automatische vertaling door AI, verbeteringen welkom)

## Taal
sprache-titel = Taal
sprache-system = Systeemtaal (aanbevolen)
sprache-ki-hinweis = Deze vertaling is automatisch gemaakt (door AI) – meld fouten via „Over Vapafane”.
sprache-neustart = Bij het wisselen wordt het venster kort opnieuw geladen.

## Einheiten und Zahlen
einheit-tag = d
einheit-std = u
einheit-min = min
einheit-sek = s
zahl-dezimal = ,
einheit-mio = mln.
einheit-tsd = k
zahl-tausender = .

## Java
java-hinweis-windows = Installeer het bijv. via https://adoptium.net (Temurin { $version }).
java-hinweis-linux = Installeer het met: sudo apt install openjdk-{ $version }-jre-headless
err-java-fehlt = Java is niet geïnstalleerd op deze pc – zonder Java kan geen enkele Minecraft-server starten. Deze server heeft Java { $version } nodig. { $hinweis }
err-java-noetig = Minecraft { $mc } heeft Java { $version } nodig, maar dat is niet geïnstalleerd. { $hinweis }

## Algemene knoppen
gem-ok = OK
gem-schliessen = Sluiten
gem-abbrechen = Annuleren
gem-weiter = Verder
gem-entfernen = Verwijderen
gem-speichern = Opslaan
task-abgebrochen = Geannuleerd

## Koptekst
hdr-untertitel = Minecraft-werkplaats voor server & client
hdr-bereich = Gebied
hdr-client = Client
hdr-server = Server
hdr-aktiver-server = Actieve server (er draait er maar één tegelijk)
hdr-kein-server = (geen server)
hdr-plus-erster = Maak hier je eerste server
hdr-plus = Server toevoegen
hdr-menu-neu = Nieuwe server maken (Fabric, Paper, NeoForge, Vanilla) …
hdr-menu-ordner = Bestaande map toevoegen …
hdr-menu-import = Server importeren uit ZIP/back-up …
hdr-menu-entfernen = Geselecteerde server uit de lijst halen
hdr-einstellungen = Instellingen
hdr-hell-dunkel = Licht/donker wisselen
hdr-ev-gewechselt = Server gewisseld: { $name }
hdr-err-umschalten = Wisselen mislukt

## Zijbalk
side-bereiche = Onderdelen
side-betrieb = Beheer
side-uebersicht = Overzicht
side-konsole = Console
side-spieler = Spelers
side-inhalte = Inhoud
side-mods = Mods/Plugins
side-updates = Versie & updates
side-backups = Back-ups
side-netzwerk = Netwerk
side-erreichbarkeit = Bereikbaarheid

## Welkom
willkommen-titel = Welkom bij Vapafane
willkommen-text = Vapafane is geen officieel Minecraft-product. Het is niet goedgekeurd door of verbonden met Mojang of Microsoft.

    Vragen, ideeën of fouten: via de GitHub-pagina van het project (Instellingen → Over Vapafane).

## Algemeen
gem-ordner = Map
gem-ordner-oeffnen = Map openen
gem-launcher = Officiële Minecraft Launcher
gem-modrinth-app = Modrinth-app

## Over Vapafane
about-titel = Over Vapafane
about-version = Versie { $version }
about-testmodus = TESTMODUS
about-nicht-offiziell = Vapafane is geen officieel Minecraft-product. Het is niet goedgekeurd door of verbonden met Mojang of Microsoft.
about-marken = Minecraft is een merk van Mojang/Microsoft. Mods en plugins komen van hun makers (via Modrinth of CurseForge).
about-kontakt = Contact & fouten melden (GitHub) ↗
about-projektseite = Projectpagina ↗

## Uiterlijk
skin-titel = Uiterlijk
skin-grass-sub = Licht of donker (knop rechtsboven)
skin-obsidian-sub = Altijd donker

## Client-instellingen
cset-einstellung = Instelling
cset-einrichten = Installatie
cset-ziel = Standaarddoel
cset-ziel-sub = Waar nieuwe servers worden geïnstalleerd
cset-spielordner = Spelmappen van alle clients
cset-spielordner-sub = Hier staan de mods, opties en werelden van elke server

## Back-ups
bak-restore-frage = „{ $name }” terugzetten naar de staat van { $datei }?
bak-restore-text = Daarvoor wordt de server gestopt. De huidige staat wordt niet gewist – die blijft ernaast staan in een map met de naam „…_vor_Wiederherstellung_<datum>”.
bak-wiederherstellen = Terugzetten
bak-ev-wiederhergestellt = Back-up teruggezet
bak-restore-fertig-titel = Teruggezet
bak-restore-fertig = De server staat weer op de staat van { $label }. Je kunt hem nu starten.
bak-restore-daneben = De vorige staat staat in:
    { $ordner }
    (mag weg als alles in orde is)
bak-err-restore = Terugzetten mislukt
bak-waehlen = Back-up kiezen
bak-erklaerung = Elke back-up is de complete servermap als ZIP, met de datum in de naam. Opgeslagen in { $ordner }. De nieuwste { $anzahl } blijven bewaard, oudere worden gewist.
bak-jetzt = Nu back-up maken
bak-ordner = Back-upmap openen
bak-andere = Andere ZIP terugzetten …
bak-aufheben = Bewaren:
bak-leer = Nog geen back-up van deze server.

## Bereikbaarheid
rch-fehlgeschlagen = Controle mislukt
rch-noch-nicht = Nog niet gecontroleerd
rch-kein-server = Geen server geselecteerd
rch-kein-server-text = Maak bovenaan met „+” een server of kies er een, dan start de controle.
rch-prueft = Controleren …
rch-geprueft-um = Gecontroleerd om { $zeit }
rch-ip-erklaerung = IPv4 is het klassieke internetadres, IPv6 het nieuwe met een eigen adres per apparaat. Bij DS-Lite deelt je provider het IPv4-adres – dan werkt alleen IPv6.
rch-erneut = Opnieuw controleren
rch-portfreigabe = Handleiding poort openen ↗
rch-dienst-hinweis = Voor de controle vraagt Vapafane aan een dienst op internet (ifconfig.co) of de serverpoort van buitenaf open is. Uitschakelen: Instellingen → Automatisering.

## Vrienden uitnodigen
inv-titel = Vrienden uitnodigen
inv-erklaerung = Stuur je vrienden een code – Vapafane richt hun Minecraft in met de passende mods.
inv-adresse-label = Adres waarmee je vrienden de server bereiken
inv-oeffentlich = Publiek adres opzoeken
inv-adresse-hinweis = Het adres van het thuisnetwerk is al ingevuld (alleen zelfde wifi). Voor vrienden van buiten: publiek adres opzoeken en de serverpoort in de router openzetten.
inv-code-erzeugen = Code maken
inv-als-datei = Als bestand opslaan …
inv-kein-passwort = De code bevat geen wachtwoorden – alleen adres, versie en mods.
inv-datei-gespeichert = Opgeslagen:
inv-datei-schicken = stuur dit bestand naar je vrienden.
inv-code-label = Uitnodigingscode ({ $zeichen } tekens) – kopiëren en bijv. via chat versturen:
inv-kopieren = Kopiëren
inv-kopiert = Gekopieerd
inv-mods = { $anzahl } mods voor de spelers
inv-mods-optional = (waarvan { $anzahl } optioneel)
inv-keine-mods = Geen mods nodig – alleen versie { $version }.
inv-ipv6 = Bevat ook je IPv6-adres { $adresse }.
inv-nur-ipv6 = – volgens de laatste controle is je server alleen via IPv6 bereikbaar.
inv-nur-server = Alleen op de server (spelers hebben ze niet nodig): { $mods }
inv-datei-noetig = Deze mods staan niet op Modrinth of CurseForge: { $mods }. Ze passen niet in de code – gebruik „Als bestand opslaan”, dan gaan ze mee.

## Spelers & whitelist
spi-online-titel = Nu online
spi-wl-titel = Whitelist (wie mag binnenkomen)
spi-wl-erklaerung = Met de whitelist aan (Instellingen → „Alleen whitelist”) mogen alleen de spelers hier in de lijst de server op. Dat beschermt tegen vreemden – zet ook jezelf erbij.
spi-name-platzhalter = Minecraft-naam
spi-hinzufuegen = Toevoegen
spi-name-ungueltig = Vul een geldige Minecraft-naam in (3–16 tekens).
spi-nur-laufend = Toevoegen werkt alleen terwijl de server draait (die controleert de naam bij Mojang).
spi-wl-leer = De whitelist is leeg.
spi-niemand = Niemand online
spi-kreativ = Creatief
spi-ueberleben = Overleven
spi-op = OP geven
spi-op-weg = OP afnemen
spi-kick = Eruit zetten
spi-kick-frage = { $name } eruit zetten?
spi-kick-text = Diegene kan meteen weer binnenkomen.
spi-err-befehl = Commando mislukt
wl-titel = Whitelist
wl-einschalten = Whitelist aanzetten
wl-zu-spielern = Naar spelers
wl-warnung-offen = Let op: je server is vanaf internet bereikbaar en de whitelist staat uit – iedereen die het adres kent of vindt, kan binnenkomen. Met de whitelist mogen alleen spelers die jij toevoegt erin.
wl-warnung-leer = De whitelist staat aan maar is nog leeg – op dit moment kan niemand binnenkomen, ook jij niet. Voeg bij „Spelers” je Minecraft-naam en die van je vrienden toe.
wl-tipp = Tip: de whitelist staat uit – iedereen met het adres kan binnenkomen. Aangeraden zodra vrienden van buiten meespelen: dan mogen alleen de spelers hier in de lijst erin.

## Console
kon-neues-log = nieuw logbestand (de server is opnieuw gestart)
kon-beigetreten = { $name } is erbij gekomen
kon-verlassen = { $name } heeft het spel verlaten
kon-bereit = Server klaar (gestart in { $sekunden } s)
kon-fehler = Fout: { $fehler }
kon-tag = Dag
kon-nacht = Nacht
kon-wetter = Helder weer
kon-wer-online = Wie is er online?
kon-platzhalter = Typ een commando, bijv. say Hallo  ·  time set day  ·  weather clear
kon-senden = Versturen
kon-log-ordner = Logmap openen

## Overzicht
ueb-laeuft = Draait
ueb-startet = Start …
ueb-stoppt = Stopt …
ueb-neustart-absturz = Herstart na crash …
ueb-gestoppt = Gestopt
ueb-abgestuerzt = Gecrasht
ueb-extern = Draait buiten Vapafane om
ueb-anderer = Er draait een andere server
ueb-starten = Starten
ueb-stoppen = Stoppen
ueb-neustart-knopf = Herstarten
ueb-seit = sinds { $dauer }
ueb-neustarts = automatisch herstart { $anzahl }×
ueb-hinweis-extern = De server is niet via Vapafane gestart (bijv. oud paneel of terminal). Console en spelers werken gewoon; „Stoppen” stuurt hem het stop-commando.
ueb-hinweis-anderer = De achtergronddienst draait een andere server. Stop die eerst.
ueb-hinweis-absturz = De server is gecrasht en niet herstart. Bekijk de laatste foutmelding in de console.
ueb-hinweis-kein-server = Voeg bovenaan met „＋” een server toe.
ueb-ev-bereit = Server klaar
ueb-ev-gestoppt = Server gestopt
ueb-ev-startet = Server start
ueb-ev-neustart = Server herstart
ueb-ev-stoppt = Server stopt
ueb-ev-in-sek = over { $sekunden } s
ueb-ev-absturz = Server gecrasht
ueb-ev-absturz-neustart = Server gecrasht – automatische herstart
ueb-notify-absturz = Minecraft-server gecrasht
ueb-notify-absturz-neustart = Hij start automatisch opnieuw.
ueb-ev-backup-start = Back-up gestart
ueb-ev-backup-fertig = Back-up klaar: { $datei } ({ $groesse })
ueb-err-start = Starten mislukt
ueb-err-stop = Stoppen mislukt
ueb-err-backup = Back-up mislukt
ueb-eula-titel = EULA bevestigen
ueb-eula-text = Zonder akkoord met de Minecraft-EULA start geen enkele server.
ueb-eula-akzeptiere = Ik accepteer de
ueb-eula-fuer = voor „{ $name }”.
ueb-verb-stoppen = stoppen
ueb-verb-neustart = herstarten
ueb-stop-frage = Server { $verb }?
ueb-stop-extern-text = De server draait buiten Vapafane om. Ik stuur hem het commando „stop” – hij slaat de wereld op en stopt.
ueb-stop-niemand = Er is nu niemand online.
ueb-stop-spieler = { $anzahl } spelers online: { $namen }
ueb-sofort = Meteen
ueb-mit-warnung = Over { $sekunden } s met waarschuwing
ueb-karte-spieler = Spelers online
ueb-karte-tps = TPS (ticks per seconde)
ueb-karte-ram = Geheugen
ueb-karte-ordner = Servermap
ueb-karte-aussen = Van buitenaf
ueb-karte-adresse = Adres (thuisnetwerk)
ueb-karte-backup = Laatste back-up
ueb-karte-version = Versie
ueb-server-aus = server uit
ueb-keine-verbindung = geen verbinding
ueb-niemand = niemand
ueb-ms-pro-tick = { $ms } ms per tick
ueb-ruckelt = hapert
ueb-fluessig = soepel
ueb-von-kernen = van { $anzahl } kernen
ueb-ram-belegt-limit = in gebruik (limiet { $limit } GB)
ueb-ram-belegt = in gebruik
ueb-ram-limit = limiet { $limit } GB
ueb-frei = { $groesse } vrij
ueb-vor = { $dauer } geleden
ueb-noch-keins = nog geen
ueb-backup-knopf = druk op de knop „Backup”
ueb-backup-sub = { $groesse } · { $anzahl } bewaard
ueb-erreichbar = Bereikbaar
ueb-nur-ipv6 = Alleen IPv6
ueb-nicht-erreichbar = Niet bereikbaar
ueb-ipv4-und-6 = IPv4 en IPv6
ueb-ueber-ipv4 = via IPv4
ueb-portfreigabe = poortinstelling controleren
ueb-ereignisse = Recente gebeurtenissen
ueb-nichts-passiert = Er is nog niets gebeurd.
ueb-fenster-zu = Het venster sluiten stopt de server niet – hij draait door op de achtergrond.
ueb-kontakt = Contact & fouten melden (GitHub)

## Instellingen (server)
set-spiel = Spel (deze server)
set-motd = Servernaam (MOTD)
set-max-spieler = Max. spelers
set-schwierigkeit = Moeilijkheid
set-wert-peaceful = Vreedzaam
set-wert-easy = Makkelijk
set-wert-normal = Normaal
set-wert-hard = Moeilijk
set-spielmodus = Spelmodus van nieuwe spelers
set-wert-survival = Overleven
set-wert-creative = Creatief
set-wert-adventure = Avontuur
set-wert-spectator = Toeschouwer
set-sichtweite = Zichtafstand (chunks)
set-simulationsweite = Simulatieafstand (chunks)
set-spawn-schutz = Spawnbescherming (blokken)
set-port = Serverpoort
set-whitelist = Alleen whitelist
set-whitelist-hilfe = Aangeraden voor de veiligheid: alleen spelers op de whitelist (pagina „Spelers”) mogen erin. Zonder kan iedereen die je adres kent binnenkomen.
set-leistung = Prestaties (deze server)
set-ram = Max. geheugen voor de server (GB)
set-ram-hinweis = Deze pc heeft { $gb } GB. Laat ongeveer 3–4 GB over voor het systeem.
set-speichern = Instellingen opslaan
set-gespeichert = Opgeslagen.
set-gespeichert-neustart = Opgeslagen. Geldt vanaf de volgende herstart.
set-err-speichern = Opslaan mislukt
set-automatik = Automatisering
set-autostart = Actieve server automatisch starten bij het aanzetten van de pc
set-auto-testmodus = Uitgeschakeld in testmodus.
set-auto-kein-systemd = Niet beschikbaar op dit systeem (geen systemd).
set-auto-windows = Start bij het aanzetten van de pc, ook zonder aanmelden (Windows vraagt bij het inschakelen om beheerdersrechten).
set-auto-linger-fehlt = Let op: hij start pas als jij je aanmeldt. Klik hier om starten zonder aanmelden toe te staan.
set-auto-aktiv = Start ook zonder aanmelden en herstart zichzelf na een crash.
set-linger-frage = Starten zonder aanmelden toestaan?
set-linger-text = Daarvoor moet systemd je gebruikersdiensten al bij het opstarten kunnen starten („loginctl enable-linger”). Daarna draait de server ook als niemand is aangemeld.
set-linger-erlauben = Toestaan
set-linger-abgelehnt = Niet toegestaan
set-fenster-autostart = Dit venster openen bij aanmelden
set-backup-um = Dagelijkse back-up om
set-backup-uhr =  uur (complete server als ZIP)
set-backup-sub = Wordt gedaan door de achtergronddienst. Wordt ingehaald als de pc op dat moment uit stond.
set-uhrzeit = Tijd van de dagelijkse back-up
set-reach = Bereikbaarheid van buitenaf controleren
set-reach-sub = Bij het wisselen naar het servergebied. Vraagt daarvoor een controledienst op internet.
set-notify = Meldingen
set-notify-sub = Als iemand komt of gaat en als de server crasht (zolang het venster open is). Zonder geluid.
set-ordner = Mappen
set-ordner-server = Servermap
set-an = aan
set-aus = uit
set-panel-titel = Er is nog een oud beheerpaneel ingericht.
set-panel-status = Dienst „minecraft.service”: { $dienst }, bij het opstarten { $start }; dagelijks back-upprogramma { $backup }. Zolang die draait, kan Vapafane de server niet zelf starten (zelfde poort), en back-ups zouden dubbel worden gemaakt.
set-panel-knopf = Oud paneel uitschakelen …
set-panel-frage = Oud paneel uitschakelen?
set-panel-text = Ik stop de dienst „minecraft.service” (de server slaat netjes op en stopt) en schakel hem en zijn dagelijkse back-up bij het opstarten uit.

    Het oude paneel zelf en zijn bestanden blijven – het kan altijd weer aan. Daarna start je de server hier met „Starten”.
set-panel-abschalten = Uitschakelen
set-panel-aus = Oud paneel uitgeschakeld
set-panel-err = Uitschakelen mislukt

## CurseForge
cf-titel = CurseForge (optioneel)
cf-experimentell = Experimenteel
cf-brauche = Heb ik dit nodig?
cf-erklaerung = Nee – de meeste mods staan op Modrinth, dat zonder instellen werkt. CurseForge vindt daarnaast mods die alleen daar bestaan en heeft daarvoor een persoonlijke API-sleutel nodig (een soort toegangscode).
cf-anleitung-titel = Zo krijg je een sleutel (gratis)
cf-schritt-1 = Klik op „Sleutel aanvragen” en vul het formulier in (bijv. doel: „Mods voor mijn eigen server”).
cf-schritt-2 = Wacht op het antwoord per e-mail – CurseForge controleert handmatig, dat kan een paar dagen duren.
cf-schritt-3 = Plak de sleutel hieronder, „Sleutel testen” en dan „Opslaan”.
cf-persoenlich = De sleutel is persoonlijk
cf-persoenlich-text = – niet delen of publiceren. Hij blijft alleen op deze pc. Van toepassing zijn de
cf-bedingungen = CurseForge-voorwaarden voor programma’s van derden
cf-experimentell-text = Experimenteel: CurseForge beslist of privétools een sleutel krijgen.
cf-beantragen = Sleutel aanvragen ↗
cf-konsole = CurseForge-console ↗
cf-mehr = Meer over de CurseForge-API ↗
cf-platzhalter = API-sleutel hier plakken
cf-testen = Sleutel testen
cf-teste = Testen …
cf-funktioniert = De sleutel werkt.
cf-schluessel-da = Sleutel opgeslagen: { $maskiert } – CurseForge doet mee in zoekopdrachten.
cf-kein-schluessel = Geen sleutel opgeslagen – er wordt alleen op Modrinth gezocht.
cf-gespeichert = CurseForge-sleutel opgeslagen
cf-entfernen-frage = CurseForge-sleutel wissen?
cf-entfernen-text = Daarna zoekt Vapafane weer alleen op Modrinth.

## Server toevoegen
gem-anderer-ordner = Andere map …
srv-typ-fabric = Fabric (mods)
srv-typ-paper = Paper (plugins)
srv-typ-neoforge = NeoForge (mods)
srv-typ-vanilla = Vanilla (zonder mods)
srv-name-label = Naam in de serverlijst
srv-typ-label = Servertype
srv-version-label = Minecraft-versie
srv-laedt = laden …
srv-keine-verbindung = Geen verbinding: { $fehler }
srv-ordner-waehlen = Map voor de server kiezen
srv-server-ordner-waehlen = Servermap kiezen
srv-backup-waehlen = Back-up van de server kiezen (ZIP of tar)
srv-ordner-nicht-leer = De gekozen map is niet leeg – de server komt in een nieuwe submap.
srv-ordner-ungueltig = Deze map kan niet worden gebruikt.
srv-eula-zusatz = (zonder start geen enkele server)
srv-hinweis = Opmerking
srv-ev-hinzu = Server toegevoegd: { $name }
srv-hinzu-titel = Server toevoegen
srv-hinzu-text = „{ $name }” staat in de lijst. Stop de draaiende server om te wisselen.
srv-err-hinzu = Toevoegen mislukt
srv-kein-server = Geen server gevonden
srv-import-titel = Server importeren
srv-err-import = Importeren mislukt
srv-neu-titel = Nieuwe server maken
srv-neu-name = Mijn server
srv-err-erstellen = Maken mislukt
srv-entfernen-frage = „{ $name }” uit de lijst halen?
srv-entfernen-text = De bestanden in { $ordner } blijven staan, alleen de vermelding verdwijnt.
srv-err-entfernen = Verwijderen mislukt

## Mods/Plugins
mods-erklaerung = Uitgeschakelde mods blijven in de map maar worden niet geladen. Wijzigingen gelden vanaf de volgende start van de server.
mods-ev-an = { $name } ingeschakeld
mods-ev-aus = { $name } uitgeschakeld
mods-neustart-noetig = De server draait – de wijziging geldt na de volgende herstart.
mods-nur-client = alleen client
mods-aus-grund = Uitgeschakeld: { $grund }
mods-an-tipp = Ingeschakeld – klik om uit te schakelen
mods-aus-tipp = Uitgeschakeld – klik om in te schakelen
mods-anzahl = { $an } van { $gesamt } { $wort } ingeschakeld
mods-datapacks-anzahl = { $anzahl } datapacks
mods-leer = Geen { $wort } in de map „{ $ordner }”.
mods-datapacks-leer = Geen datapacks in de wereld.
mods-datapacks-titel = Datapacks in de wereld
mods-hinzufuegen = { $wort } toevoegen …
mods-hinzufuegen-titel = { $wort } toevoegen
mods-ordner-oeffnen = Modmap openen
mods-aufraeumen = Opruimen …
mods-aufraeumen-titel = Opruimen
mods-aufraeumen-tipp = Resten van oude versies wissen (de server moet gestopt zijn)
mods-alles-sauber = Alles schoon – er zijn geen resten van oude versies.
mods-aufraeumen-text-1 = Deze resten van oude versies zouden worden gewist (in totaal
mods-aufraeumen-text-2 = Wereld, instellingen en uitgeschakelde mods blijven. Wat de server nog nodig heeft, downloadt hij bij de volgende start opnieuw.
mods-erst-stoppen = Stop eerst de server om te wissen.
mods-loeschen-knopf = { $groesse } wissen
mods-ev-aufgeraeumt = Opgeruimd: { $groesse } vrijgemaakt
mods-aufgeraeumt-titel = Opgeruimd
mods-aufgeraeumt = { $anzahl } items gewist, { $groesse } vrijgemaakt.
mods-nicht-loeschbar = Niet te wissen:
mods-err-aufraeumen = Opruimen mislukt

## Catalogus (mods toevoegen)
kat-suche = Zoeken op Modrinth … (leeg = populairste)
kat-dp-option = Datapacks (voor de wereld)
kat-treffer = { $anzahl } passende { $wort } voor Minecraft { $mc }
kat-beliebtheit = gesorteerd op populariteit
kat-sucht = Zoeken …
kat-nichts = Niets gevonden.
kat-mehr = Meer laden
kat-cf-tipp = Tip: met een CurseForge-sleutel (Instellingen) vind je ook mods die alleen daar bestaan.
kat-nur-cf = Alleen op CurseForge
kat-cf-erklaerung = Deze staan niet op Modrinth. Sommige makers staan downloaden via andere programma’s niet toe – dan download je het bestand in de browser en zet je het zelf in de map.
kat-beides = Server + client
kat-beides-tipp = Spelers hebben hem zelf ook nodig – bijv. via „Mods voor vrienden exporteren (.mrpack)”
kat-nur-server = Alleen server
kat-meta = { $downloads } downloads · door { $autor }
kat-laedt = Laden …
kat-installiert = Geïnstalleerd
kat-installieren = Installeren
kat-ev-installiert = Geïnstalleerd: { $name }
kat-err-install = De installatie is niet gelukt: { $fehler }
kat-ist-installiert = is geïnstalleerd (in „{ $ordner }”)
kat-abhaengigkeiten = Als afhankelijkheid toegevoegd: { $namen }.
kat-schon-da = Waren er al: { $namen }.
kat-fehlende = Let op: deze afhankelijkheid bestaat niet voor deze versie: { $namen } – zo start de server mogelijk niet.
kat-client-noetig = Spelers hebben deze mod zelf ook nodig – het makkelijkst via „Mods voor vrienden exporteren (.mrpack)” bij „Versie & updates”.
kat-manuell-1 = De maker staat downloaden via andere programma’s niet toe. Download het bestand
kat-manuell-2 = in de browser en zet het in de map „{ $ordner }”.
kat-browser = In de browser downloaden ↗
kat-dp-ordner = Datapackmap openen
kat-neustart = De server draait – dit geldt na een herstart.
kat-jetzt-neustarten = Nu herstarten

## Versie & updates
upd-st-aktuell = actueel
upd-st-fehlt = ontbreekt
upd-st-unbekannt = onbekend
upd-st-neu = nieuw (nodig)
upd-st-manuell = alleen in de browser bij te werken (CurseForge)
upd-st-reenable = past weer – wordt ingeschakeld
upd-st-doppelt = dubbel (bestaat al als mod)
upd-alles-aktuell = Alles is actueel.
upd-zusammenfassung = { $updates } updates, { $aktuell } al actueel, { $fehlend } zonder passende versie, { $unbekannt } onbekend
upd-neu-gebraucht = { $anzahl } nieuw nodig
upd-passen-wieder = { $anzahl } passen weer
upd-doppelt = { $anzahl } dubbel (worden opzijgezet)
upd-server-stand = Nieuwe serverbuild voor Minecraft { $mc }: { $label }.
upd-beta-hinweis = Opmerking: er is voorlopig alleen een test-/bètaversie.
upd-wechsel = Server: Minecraft { $von } → { $nach } ({ $label }).
upd-welt-warnung = Let op: een wereld die eenmaal met de nieuwe versie heeft gedraaid, kan niet terug. De enige weg terug is de automatisch gemaakte back-up.
upd-java-da = Java { $version } nodig – aanwezig ({ $pfad }).
upd-java-fehlt = Java { $version } nodig maar niet geïnstalleerd. { $hinweis }
upd-manuell-hinweis = Bij mods met „alleen in de browser bij te werken” staat de maker automatisch downloaden niet toe – haal het nieuwe bestand op CurseForge en vervang het zelf.
upd-platz = Te weinig schijfruimte voor de back-up ({ $frei } vrij, server { $server }).
upd-nur-mods = { $version } (alleen mods/plugins bijwerken)
upd-keine-neuere = Er is geen nieuwere stabiele Minecraft-versie.
upd-keine-verbindung = Geen verbinding
upd-err-pruefen = Controle mislukt
upd-rollback-frage = Terug naar de staat van vóór de laatste update?
upd-rollback-text = De server wordt gestopt en teruggezet vanaf de back-up die bij de update is gemaakt. De huidige staat blijft ernaast staan in een map „…_vor_Wiederherstellung_<datum>”.
upd-zuruecksetzen = Terugzetten
upd-ev-rollback = Staat van vóór de update teruggezet
upd-rollback-fertig-titel = Teruggezet
upd-rollback-fertig = De server staat weer op de staat van { $label }. Je kunt hem nu starten.
upd-rollback-daneben = De staat van ná de update staat in:
upd-err-rollback = Terugzetten mislukt
upd-rollback-knopf = Terug naar de staat van vóór de laatste update
upd-was-mods = Mods bijwerken
upd-ablauf = Stappen: server stoppen (met waarschuwing als er iemand online is) → complete back-up → { $anzahl } bestanden downloaden en controleren → vervangen → server starten en controleren.
upd-werden-deaktiviert = { $anzahl } mod(s)/datapack(s) zonder passende versie worden uitgeschakeld.
upd-probelauf-tipp = Tip: met „Proefdraai” test je dit zonder risico op een kopie.
upd-start-frage = Update starten? ({ $was })
upd-starten = Update starten
upd-ev-gestartet = Update gestart ({ $was })
upd-err-update = Update mislukt
upd-ev-fertig = Update klaar ({ $was })
upd-fertig-titel = Update klaar
upd-fertig-1 = De server draait weer ({ $was }).
upd-fertig-backup = Back-up van de oude staat:
upd-fertig-aufgeraeumt = Opgeruimd: { $anzahl } oude bestanden/mappen gewist ({ $groesse }).
upd-fertig-2 = Mods zonder passende versie zijn uitgeschakeld en te zien bij „Mods/Plugins”.
upd-fehlgeschlagen-titel = De update is niet gelukt
upd-fehlgeschlagen-frage = Bekijk de foutmelding in de console. Zal ik de staat van vóór de update terugzetten vanaf de back-up?
upd-trial = Proefdraai
upd-trial-starten = Proefdraai starten
upd-trial-frage = Proefdraai doen?
upd-trial-text = Ik kopieer de server naar de tijdelijke map, werk de kopie bij, start hem op een andere poort en controleer of hij helemaal opstart. Daarna wordt de kopie gestopt en gewist.

    Je echte server blijft onaangeroerd en kan gewoon doordraaien. Bij grote werelden duurt het kopiëren een paar minuten.
upd-trial-tipp = Werkt een kopie in de tijdelijke map bij en start hem op een andere poort
upd-trial-err = Proefdraai mislukt
upd-trial-ok-titel = Proefdraai geslaagd
upd-trial-ok = De bijgewerkte kopie was in { $sekunden } seconden opgestart (poort { $port }).
upd-trial-timeout = De kopie was na 15 minuten nog niet opgestart.
upd-trial-exit = De kopie stopte bij het opstarten (afsluitcode { $code }).
upd-trial-meldungen = Opvallende meldingen:
upd-trial-zeilen = Laatste regels:
upd-trial-geloescht = De kopie is gewist.
upd-trial-nicht-geloescht = De kopie kon niet helemaal worden gewist: { $ordner }
upd-ev-trial-ok = Proefdraai geslaagd
upd-ev-trial-fehler = Proefdraai mislukt
upd-jetzt = Nu:
upd-erklaerung = Controleert of alle mods er voor de gekozen versie zijn. Vóór elke update wordt automatisch een back-up gemaakt.
upd-ziel = Doel:
upd-versionen = Versies laden
upd-pruefen = Controleren
upd-spalte-mod = Mod / datapack
upd-spalte-neu = Nieuw
upd-deaktivieren-fehlende = Mods/datapacks zonder passende versie uitschakelen (anders start de server waarschijnlijk niet)
upd-deaktivieren-unbekannte = Onbekende mods/datapacks uitschakelen (niet op Modrinth)
upd-nicht-unterstuetzt = De update-assistent is er voor Fabric, Paper, NeoForge en Vanilla.
upd-export-knopf = Mods voor vrienden exporteren (.mrpack)
upd-export-tipp = Alleen voor Fabric- en NeoForge-servers (mods die vrienden ook nodig hebben)
upd-export-titel = Modpack opgeslagen
upd-export-mods = { $anzahl } mods van Modrinth
upd-export-eingepackt = { $anzahl } direct ingepakt
upd-export-hinweis = Je vrienden importeren het bestand in de Modrinth-app („Bestand toevoegen”). Server-only mods worden daar niet geïnstalleerd.
upd-err-export = Exporteren mislukt

## Client
cli-titel = Uitnodiging gebruiken
cli-code-erklaerung-1 = Plak de code (begint met
cli-code-erklaerung-2 = ) of open het bestand
cli-code-erklaerung-3 =  – Vapafane richt Minecraft in met alle mods die bij de server passen.
cli-code-platzhalter = Uitnodigingscode hier plakken (VPF1-…)
cli-pruefen = Uitnodiging controleren
cli-datei-oeffnen = Uitnodigingsbestand openen …
cli-pruefe-einladung = Uitnodiging controleren …
cli-launcher-da = Minecraft Launcher gevonden
cli-kein-launcher = Geen Minecraft Launcher
cli-modrinth-da = Modrinth-app gevonden
cli-meine-server = Mijn servers
cli-leer = Nog geen client ingericht – gebruik hierboven een uitnodiging.
cli-kein-konto = Vapafane start Minecraft niet zelf en komt nooit aan je Microsoft-account – „Spelen” opent de officiële launcher of de Modrinth-app.
cli-nicht-erreichbar = Server niet bereikbaar
cli-online = Online · { $version } · { $spieler }/{ $max } spelers
cli-version-passt-nicht = Online, maar versie { $version } – past niet
cli-aktualisiert = De server is bijgewerkt ({ $version }) – vraag de beheerder om een nieuwe uitnodigingscode
cli-gerade-nicht = nu even niet bereikbaar
cli-modrinth-fehlt = Modrinth-app niet gevonden
cli-launcher-fehlt = Minecraft Launcher niet gevonden
cli-download-seite = Naar de downloadpagina
cli-ev-eingerichtet = Client ingericht: { $name }
cli-gefunden = gevonden
cli-nicht-gefunden = niet gevonden
cli-keine-mods = Geen mods nodig – alleen Minecraft { $version }.
cli-optional = optioneel
cli-nur-browser = alleen in de browser te downloaden (link na de installatie)
cli-nur-datei = ontbreekt – alleen in het uitnodigingsbestand
cli-launcher-fehlt-text = De officiële Minecraft Launcher is niet te vinden. Je kunt toch inrichten; het profiel wordt gemaakt zodra de launcher is geïnstalleerd en je hier op „Vernieuwen” drukt.
cli-launcher-laden = Launcher downloaden
cli-nie-gestartet = De launcher is nog nooit gestart. Start hem één keer en sluit hem, richt daarna hier in.
cli-ordner-waehlen = Map voor deze Minecraft kiezen
cli-mods-zusammen = { $anzahl } mods, samen { $groesse }
cli-andere-version = De server meldt een andere versie ({ $version }). Hij is waarschijnlijk bijgewerkt – vraag de beheerder om een nieuwe uitnodigingscode.
cli-spalte-version = Versie
cli-spalte-groesse = Grootte
cli-einrichten-fuer = Inrichten voor:
cli-in-modrinth = In de Modrinth-app:
cli-instanz-update = Bestaande instantie „{ $name }” bijwerken (aanbevolen)
cli-instanz-update-sub = Alles blijft in dezelfde instantie: werelden, instellingen, shaders, resource packs en mods die je zelf hebt toegevoegd (bijv. Iris). Je hoeft in de Modrinth-app alleen op „Modpack opnieuw installeren” te klikken – Vapafane laat zien hoe.
cli-instanz-neu = Nieuwe instantie maken
cli-instanz-neu-sub = Werkt automatisch via de import. Hieronder kun je instellingen, shaders en resource packs overnemen – zelf toegevoegde mods niet. De oude instantie blijft tot je hem wist.
cli-update-bleibt = Bij het bijwerken blijft alles in de instantie: werelden, opties, shaders, resource packs en je serverlijst.
cli-import-bestaetigen = De Modrinth-app gaat open – bevestig daar alleen de import.
cli-ordner-label = Map:
cli-launcher-schliessen = Sluit de Minecraft Launcher vóór het inrichten, zodat hij de wijziging niet overschrijft.
cli-einrichten = Inrichten
cli-uebernehmen-von = Instellingen overnemen van:
cli-nichts = Niets overnemen
cli-quelle-waehlen = Minecraft-map kiezen om van over te nemen
cli-nichts-uebernehmen = Niets over te nemen
cli-teil-options = Opties & toetsen
cli-teil-config = Modinstellingen
cli-teil-shaders = Shaders
cli-teil-resourcepacks = Resource packs
cli-teil-servers = Serverlijst
cli-teil-worlds = Werelden
cli-welten-warnung = Werelden van een nieuwere Minecraft-versie openen niet in een oudere. Grote werelden vertragen het inrichten.
cli-nur-fehlendes = Wat al in de map staat blijft onaangeroerd – alleen wat ontbreekt wordt overgenomen.
cli-uebernehmen-erklaerung = Bijvoorbeeld uit je vorige instantie: opties, toetsen, modinstellingen, shaders, resource packs en serverlijst. De mods komen altijd passend van de server. De bron wordt alleen gelezen, nooit gewijzigd.
cli-paket-update-1 = Het nieuwe Modrinth-pakket is klaar. Zo werk je je instantie bij
cli-paket-update-2 = werelden, opties, shaders, resource packs en je serverlijst blijven allemaal behouden:
cli-schritt-instanz = Open de Modrinth-app en klik op de instantie
cli-schritt-anklicken = links.
cli-schritt-zahnrad = Klik bovenaan op het tandwiel „Instantie-instellingen” en kies het tabblad „Installatie”.
cli-neu-installieren = Modpack opnieuw installeren
cli-schritt-datei = klik, bevestig en kies dit bestand:
cli-modrinth-oeffnen = Modrinth-app openen
cli-paket-ordner = Map met het pakket openen
cli-update-sicher = De Modrinth-app stelt eerst de instantie zelf veilig en vervangt alleen de servermods – wat je zelf hebt toegevoegd (shaders, resource packs, eigen mods) blijft.
cli-paket-offen = Het Modrinth-pakket is klaar en de Modrinth-app is ermee geopend. Bevestig daar de import – de instantie verschijnt als
cli-paket-fertig = Het Modrinth-pakket is klaar:
cli-paket-von-hand = De Modrinth-app kon niet automatisch worden geopend. Handmatig: klik in de Modrinth-app op „+” (instantie maken), dan „Importeren uit bestand” en kies dit bestand.
cli-eingerichtet = Ingericht: { $anzahl } mods gedownload.
cli-profil = De Minecraft Launcher heeft nu het profiel „{ $name }”.
cli-uebernommen-paket = De instellingen van „{ $quelle }” zitten in het pakket – de Modrinth-app past ze bij de import toe.
cli-uebernommen = Overgenomen van „{ $quelle }”: { $anzahl } bestanden.
cli-uebernommen-da = ({ $anzahl } waren er al en zijn onaangeroerd gebleven)
cli-uebernommen-fehler = { $anzahl } bestanden konden niet worden gelezen en ontbreken daarom – is Minecraft daar nog open? ({ $dateien })
cli-selbst-laden = Zelf downloaden en in de map „mods” zetten:
cli-spielen = Spelen
cli-spielen-modrinth = Spelen (Modrinth-app openen)
cli-spielen-launcher = Spelen (launcher openen)
cli-prueft = controleren …
cli-unvollstaendig = Het inrichten is niet afgerond – gebruik hierboven opnieuw de uitnodiging (of verwijder).
cli-mods-kurz = { $anzahl } mods
cli-eingerichtet-am = { $ziel } · ingericht op { $datum }
cli-aktualisieren = Vernieuwen
cli-aktualisieren-tipp = Opnieuw inrichten met de opgeslagen uitnodiging – of uitleggen wat te doen na een serverupdate
cli-entfernen-frage = „{ $name }” verwijderen?
cli-entfernen-modrinth = Vapafane vergeet deze client en wist zijn pakket. De instantie in de Modrinth-app blijft – wis die daar zelf.
cli-entfernen-launcher = De map met mods, instellingen en solowerelden van dit profiel wordt gewist, en het profiel verdwijnt uit de Minecraft Launcher.
cli-upd-titel = De server is bijgewerkt
cli-upd-text = „{ $name }” draait nu op Minecraft { $version }. De passende versie en mods zitten in de uitnodiging – je hebt dus een nieuwe uitnodigingscode van de beheerder nodig.
cli-upd-code = Plak die bovenaan bij „Uitnodiging gebruiken” en klik op „Uitnodiging controleren”.
cli-upd-modrinth = Vapafane maakt dan een nieuw pakket en laat zien hoe je je instantie bijwerkt – je instellingen, werelden en shaders blijven.
cli-upd-launcher = Vapafane vervangt dan de mods – je instellingen en werelden blijven.
cli-gespeicherte-einladung = Opgeslagen uitnodiging van „{ $name }” geladen. „Inrichten” zet alles weer goed – bijvoorbeeld als mods ontbreken of het profiel weg is. Heeft de beheerder de server bijgewerkt, dan heb je juist een nieuwe uitnodigingscode nodig.
ext-knopf = Shaders & extra’s
ext-knopf-so-gehts = Shaders & extra’s – zo werkt het

## Shaders & extra’s
ext-titel = Shaders & extra’s toevoegen
ext-skizze-1-alt = Schets van de instantie in de Modrinth-app
ext-skizze-2-alt = Schets van het zoeken in de Modrinth-app
ext-skizze-leer = Geen extra inhoud geïnstalleerd
ext-skizze-hochladen = Bestanden uploaden
ext-durchsuchen = Inhoud doorzoeken
ext-skizze-shader = Een shader, bijv. Complementary
ext-skizze-beschreibung = Beschrijving · downloads
ext-keine-shader = Shaders hebben de mod Iris nodig – die draait alleen op Fabric- of NeoForge-servers. Deze server heeft geen modloader, dus resource packs werken hier wel, shaders niet.
ext-iris-modrinth = Voor shaders heeft Minecraft de mod Iris nodig: kies bij stap 4 eerst „Mods” en installeer „Iris Shaders” (Sodium komt automatisch mee), kies daarna bij „Shaders” een shader.
ext-iris-launcher-1 = Voor shaders heeft Minecraft de mod Iris nodig (met Sodium): download beide voor de Minecraft-versie van de server van
ext-iris-launcher-2 = en zet ze in de map „mods”.
ext-mr-einleitung-1 = In je instantie
ext-mr-einleitung-2 = kun je altijd eigen shaders, resource packs en mods toevoegen. Ze blijven behouden bij serverupdates – de Modrinth-app vervangt alleen de servermods.
ext-schritt-content = Kies bovenaan het tabblad „Content” (inhoud).
ext-schritt-durchsuchen = Klik op de groene knop „Inhoud doorzoeken”.
ext-schritt-art = Kies bovenaan het soort: „Shaders” voor shaders, „Resource Packs” voor resource packs of „Mods”.
ext-schritt-installieren = Klik op „Installeren” naast het gewenste item – klaar.
ext-im-spiel = In het spel: Opties → Grafische instellingen → Shaderpacks → shader kiezen.
ext-im-spiel-rp = In het spel: Opties → Resource packs → pack kiezen.
ext-mods-update-hinweis = Eigen mods moet je na een Minecraft-update zelf in de app bijwerken; shaders en resource packs meestal niet.
ext-launcher-einleitung = De spelmap van deze server is alleen van hem: wat je daar toevoegt, blijft behouden bij serverupdates.
ext-schritt-laden-shader = Download een shader, bijv. op
ext-schritt-laden-rp = Download een resource pack, bijv. op
ext-nicht-entpacken = het .zip-bestand niet uitpakken.
ext-schritt-shaderpacks = Klik onderaan op „Map openen” en zet het bestand in de map „shaderpacks”. Resource packs gaan in „resourcepacks”.
ext-schritt-resourcepacks = Klik onderaan op „Map openen” en zet het bestand in de map „resourcepacks”.
ext-eigene-mods = Je mag eigen mods in de map „mods” zetten – maar na een Minecraft-update moet je ze zelf passend houden.
err-antwort-unlesbar = Antwoord van „{ $befehl }” onleesbaar: { $fehler }

## Bereikbaarheidsrapport
rch-w-erreichbar = bereikbaar
rch-w-nicht = niet bereikbaar
rch-w-nicht-dslite = niet bereikbaar (DS-Lite)
rch-w-nicht-geprueft = niet gecontroleerd
rch-w-nicht-vorhanden = niet aanwezig
rch-unmoeglich-titel = Controle is nu niet mogelijk
rch-unmoeglich-text = De controledienst op internet antwoordt niet. Probeer het later opnieuw.
rch-gruen-beide = Bereikbaar via IPv4 en IPv6
rch-gruen-v4 = Bereikbaar via IPv4
rch-gruen-text = Alle vrienden kunnen binnenkomen.
rch-gelb-dslite-titel = DS-Lite: alleen via IPv6 bereikbaar
rch-gelb-dslite-text = Je provider deelt het IPv4-adres. Vrienden met IPv6 kunnen toch binnenkomen – dat zijn de meesten.
rch-gelb-titel = Alleen via IPv6 bereikbaar
rch-gelb-text = Via IPv4 komt niemand binnen: óf de IPv4-poortinstelling (poort { $port }) ontbreekt, óf je provider deelt het IPv4-adres (DS-Lite). Vrienden met IPv6 kunnen toch binnenkomen.
rch-rot-titel = Niet bereikbaar van buitenaf
rch-rot-text = Meestal ontbreekt de poortinstelling in de router, of de firewall van de pc blokkeert poort { $port }.
rch-rot-dslite-v6 = Je provider deelt het IPv4-adres (DS-Lite), dus alleen IPv6 werkt. Zet poort { $port } voor IPv6 open in de router en controleer de firewall van de pc.
rch-rot-dslite = Je provider deelt het IPv4-adres (DS-Lite) en je pc heeft geen IPv6-adres. Zo kan niemand van buiten de server bereiken.
rch-aus-titel = De server draait niet
rch-aus-text = De controle volgt zodra de server draait.
rch-port-belegt-titel = Poort { $port } is al in gebruik
rch-port-belegt-text = Een ander programma – waarschijnlijk een andere Minecraft-server – gebruikt deze poort. Zo kan deze server niet starten. Stel bij Instellingen → „Serverpoort” een vrije poort in (bijv. { $frei })
rch-hotspot-tipp = De server is alleen via IPv6 bereikbaar, maar jouw internet heeft geen IPv6. Tip: verbind je pc via de hotspot van je telefoon – mobiel internet heeft meestal IPv6.

## Programmafouten (algemeen)
err-dienst-aus = De achtergronddienst draait niet
err-kein-systemd = systemd is hier niet beschikbaar (of testmodus).
err-kein-server = Geen server geselecteerd.
err-schon-beschaeftigt = Er loopt al iets: { $was }
err-neustart-extern = Deze server draait buiten Vapafane om – herstarten kan alleen via het programma dat hem heeft gestart.
err-stop-knopf = Gebruik de knop Stoppen in het overzicht (die slaat netjes op).
err-start-extern = De server draait buiten Vapafane om. Stop hem eerst daar.
err-start-anderer = Er draait nu een andere server. Stop die eerst.
err-stop-unmoeglich = De server kon niet worden gestopt.
err-pfad = Ongeldig pad
err-rcon-antwort = Ongeldig RCON-antwoord
err-adresse = Ongeldig adres
cln-alte-neoforge = oude NeoForge-versie
cln-alte-bibliothek = oude versie van een bibliotheek
cln-alte-mods = oude modversies (opzijgezet door het oude paneel/de launcher)
cln-cache = modcache (wordt bij de volgende start opnieuw gemaakt)
cli-datei-titel = Uitnodigingsbestand openen
srv-rcon-an = RCON aangezet (Vapafane heeft het nodig voor commando’s).
err-link-verboten = Dit adres kan niet worden geopend.
err-pruefsumme = De controlesom klopt niet: { $datei }
err-kein-fabric = Geen Fabric-loader gevonden voor Minecraft { $mc }
cf-err-leer = Plak eerst een sleutel.
cf-err-id = Ongeldig CurseForge-ID
cf-err-format = Dit ziet er niet uit als een API-sleutel (plak hem zonder spaties).
cf-err-antwort = Ongeldig antwoord van CurseForge: { $fehler }
cf-err-abgelehnt = CurseForge wijst de API-sleutel af (ongeldig of nog niet geactiveerd).
cf-err-fehlt = Geen CurseForge-sleutel opgeslagen.
kat-err-typ = „Mods toevoegen” bestaat niet voor dit servertype.
kat-err-keine-version = „{ $name }” bestaat niet voor Minecraft { $mc } ({ $loader }).
kat-err-cf-version = „{ $name }” bestaat op CurseForge niet voor Minecraft { $mc }.
fort-suche = Zoeken naar { $name } …
fort-suche-cf = Zoeken naar { $name } op CurseForge …
inv-err-neuer = Deze uitnodigingscode is voor een nieuwere versie van Vapafane. Werk Vapafane bij.
inv-err-unvollstaendig = De code is onvolledig of beschadigd – kopieer hem helemaal.
inv-err-kaputt = De code is beschadigd – kopieer hem opnieuw.
inv-err-version = Deze uitnodiging is voor een andere versie van Vapafane.
inv-err-angaben = De uitnodiging bevat ongeldige gegevens.
inv-err-mod = De uitnodiging bevat een ongeldige modvermelding.
inv-err-typ = Uitnodigingen zijn er voor Fabric-, NeoForge-, Paper- en Vanilla-servers.
inv-err-datei = Dit is geen geldig Vapafane-uitnodigingsbestand.
inv-err-datei-kaputt = De uitnodiging in het bestand is beschadigd.
inv-err-oeffentlich = Het publieke adres kon niet worden bepaald.
flv-err-kein-server = Er is geen server voor Minecraft { $mc }.
flv-err-kein-paper = Geen Paper voor { $mc }.
flv-err-kein-neoforge = Er is geen NeoForge voor Minecraft { $mc }.
fort-neoforge = De NeoForge-installer richt de server in (downloadt bibliotheken) …
fort-suche-server = Zoeken naar { $was } voor Minecraft { $mc } …
fort-pruefe-java = Java controleren …
fort-frage-cf = CurseForge raadplegen voor de overige mods …
fort-pruefe-deps = Afhankelijkheden controleren …
fort-pruefe-dp = Datapacks controleren …
fort-pruefe-java-platz = Java en schijfruimte controleren …
upd-err-dateiname = Ongeldige bestandsnaam van Modrinth: { $name }
upd-cf-fehler = CurseForge kon niet worden gecontroleerd: { $fehler }
upd-abhaengigkeit = (afhankelijkheid)
upd-grund-fehlt = bestaat niet voor Minecraft { $mc }
cli-err-port = Ongeldige poort in het adres
cli-err-antwort = Ongeldig antwoord van de server
cli-err-profil-kaputt = { $datei } is beschadigd – start de launcher één keer.
cli-err-nicht-leer = De map { $ordner } is niet leeg en hoort niet bij Vapafane.
cli-err-datei-kaputt = Het bestand { $datei } van de uitnodiging is beschadigd.
cli-err-fabric-profil = Ongeldig Fabric-profiel.
cli-err-neoforge-java = NeoForge heeft Java { $version } nodig. Start de Minecraft Launcher één keer (die brengt Java mee) of installeer Java.
cli-hinweis-launcher-starten = Start de Minecraft Launcher één keer en sluit hem, richt daarna opnieuw in.
cli-hinweis-kein-launcher = De officiële Minecraft Launcher is niet te vinden. Installeer hem, start hem één keer en druk daarna hier op „Vernieuwen”.
cli-hinweis-nie-gestartet = De launcher is nog nooit gestart. Start hem één keer en sluit hem, daarna „Vernieuwen”.
cli-paket-summary = Ingericht door Vapafane voor de server { $name } ({ $adresse })
fort-pruefe-mods = Mods controleren …
fort-lade = { $name } downloaden …
fort-launcher = Launcher inrichten …
fort-neoforge-install = NeoForge { $version } installeren (kan een paar minuten duren) …
fort-uebernehme = Instellingen overnemen …
fort-trage-ein = Server toevoegen …
fort-sammle = Instellingen verzamelen …
fort-packe = Modrinth-pakket inpakken …
fort-fertig = Klaar.
task-update = Update
task-backup = Back-up (complete server als ZIP)
task-restore = Back-up terugzetten
task-client = Client inrichten
task-rollback = Terug naar de staat van vóór de update
task-pruefe-mods = Mods controleren voor { $mc }
upd-err-erst-pruefen = Druk eerst op „Controleren”.
upd-err-anderer-server = De controle hoort bij een andere server. Controleer opnieuw.
upd-err-absturz = De server is bij het starten gecrasht. { $grund }
upd-err-beendet = De server stopte meteen weer.
upd-err-kein-backup = Er is geen back-up van de laatste update voor deze server.
upd-err-temp-platz = Te weinig ruimte in de tijdelijke map voor een kopie ({ $groesse } nodig).
cln-err-laeuft = Opruimen werkt alleen als de server gestopt is.
fort-raeume-auf = Oude bestanden opruimen …
fort-loesche = Oude bestanden wissen …
fort-loesche-kopie = Kopie wissen …
nbt-err-unvollstaendig = servers.dat is onvolledig.
nbt-err-laenge = servers.dat bevat een ongeldige lengte.
nbt-err-eintrag = servers.dat bevat een onbekende vermelding.
nbt-err-tiefe = servers.dat is te diep genest.
nbt-err-format = servers.dat heeft een onbekend formaat.
say-backup-start = Back-up wordt gemaakt …
say-backup-fertig = Back-up klaar.
say-backup-fehler = Back-up mislukt.
say-stoppt = stopt
say-neustart = herstart
say-countdown = De server { $was } over { $sekunden } seconden!
svc-countdown = Server { $was } over { $sekunden } s
svc-err-laeuft-nicht = De server draait op dit moment niet.
svc-err-befehl = Onbekend commando: { $befehl }
svc-err-ordner-fehlt = Servermap ontbreekt: { $ordner }
svc-err-eula = De Minecraft-EULA is voor deze server nog niet geaccepteerd.
svc-err-fremder-server = In deze map draait al een server (PID { $pid }) die niet door Vapafane is gestart. Stop die eerst (bijv. oud paneel).
svc-err-port = Poort { $port } ({ $art }) is al in gebruik. Draait er nog een Minecraft-server op deze pc?
svc-signal = beëindigd door signaal
svc-beendet = beëindigd
svc-abgestuerzt = Gecrasht ({ $grund }).
svc-taegliches-backup = Dagelijkse back-up
say-update-neustart = De server herstart voor een update!
svc-err-java-start = Java kon niet worden gestart ({ $java }): { $fehler }

## Opdrachtregel
ctl-hilfe = Gebruik: vapafane --ctl <commando>

      status            toestand van de actieve server tonen
      start             server starten (start zo nodig de achtergronddienst)
      stop [SECONDEN]   server netjes stoppen, met optioneel aftellen
      restart [SEC.]    server herstarten
      wait-ready [SEC.] wachten tot de server klaar is
      wait-stopped [S.] wachten tot de server gestopt is
      backup            complete servermap als ZIP opslaan
      cmd <COMMANDO …>  commando via RCON sturen
      aktiv NAAM        andere server uit de lijst activeren (alleen gestopt)
      neu TYPE NAAM VER [MAP]  nieuwe server maken (fabric, paper, neoforge, vanilla) en activeren
      update-check VER  mods/datapacks voor Minecraft VER controleren via Modrinth
      trial VER         proefdraai: kopie bijwerken, op andere poort testen, wissen
      update VER        update: stoppen, back-up, vervangen, starten (alleen bij goede controle)
      mods              mods en datapacks tonen met status aan/uit
      mod-an PAD        mod inschakelen (PAD zoals bij „mods”, bijv. mods/x.jar)
      mod-aus PAD       mod uitschakelen (wordt .jar.disabled)
      aufraeumen [--ja] toont oude resten; met --ja worden ze gewist (server gestopt)
      mod-suche [TERM] [--datapack]     op Modrinth passende mods/plugins (of datapacks) zoeken
      mod-install PROJECT [--datapack]  mod/plugin/datapack met vereiste afhankelijkheden installeren
      curseforge-schluessel [--loeschen]  CurseForge-sleutel van de invoer lezen en opslaan
      curseforge-test   opgeslagen CurseForge-sleutel proberen
      einladung [ADRES]                 uitnodigingscode voor vrienden tonen (adres = host:poort)
      einladung-datei BESTAND [ADRES]   uitnodigingsbestand schrijven (.vapafane)
      einloesen CODE|BESTAND [--ordner M] [--alle-optionalen] [--ziel launcher|modrinth]
                … [--einstellungen-von MAP] [--neue-instanz]
                                        client voor een uitnodiging inrichten (launcherprofiel of Modrinth-pakket);
                                        instellingen (zonder werelden) overnemen uit MAP; met --neue-instanz
                                        wordt een bestaande instantie in de Modrinth-app niet bijgewerkt
      clients                           ingerichte clients tonen
      client-entfernen MAP              client en zijn launcherprofiel wissen
      ping ADRES                        servertoestand opvragen (zoals de serverlijst in het spel)
      erreichbarkeit    controleren of de server van buitenaf bereikbaar is (IPv4/IPv6, DS-Lite)
      mrpack BESTAND    mods als Modrinth-modpack (.mrpack) voor vrienden exporteren
      quit-service      achtergronddienst afsluiten (stopt eerst de server)
ctl-fehler = Fout: { $fehler }
