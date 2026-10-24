# Vapafane – Italiano (traduzione automatica tramite IA, correzioni benvenute)

## Lingua
sprache-titel = Lingua
sprache-system = Lingua di sistema (consigliato)
sprache-ki-hinweis = Questa traduzione è stata generata automaticamente (tramite IA) – segnala gli errori in «Informazioni su Vapafane».
sprache-neustart = Al cambio l’interfaccia si ricarica brevemente.

## Einheiten und Zahlen
einheit-tag = g
einheit-std = h
einheit-min = min
einheit-sek = s
zahl-dezimal = ,
einheit-mio = mln
einheit-tsd = mila
zahl-tausender = .

## Java
java-hinweis-windows = Installalo ad es. da https://adoptium.net (Temurin { $version }).
java-hinweis-linux = Installalo con: sudo apt install openjdk-{ $version }-jre-headless
err-java-fehlt = Java non è installato su questo PC – senza di esso nessun server Minecraft può avviarsi. Questo server richiede Java { $version }. { $hinweis }
err-java-noetig = Minecraft { $mc } richiede Java { $version }, che non è installato. { $hinweis }

## Pulsanti comuni
gem-ok = OK
gem-schliessen = Chiudi
gem-abbrechen = Annulla
gem-weiter = Continua
gem-entfernen = Rimuovi
gem-speichern = Salva
task-abgebrochen = Annullato

## Intestazione
hdr-untertitel = Officina Minecraft per server e client
hdr-bereich = Area
hdr-client = Client
hdr-server = Server
hdr-aktiver-server = Server attivo (ne funziona solo uno alla volta)
hdr-kein-server = (nessun server)
hdr-plus-erster = Crea qui il tuo primo server
hdr-plus = Aggiungi server
hdr-menu-neu = Crea un nuovo server (Fabric, Paper, NeoForge, Vanilla) …
hdr-menu-ordner = Aggiungi una cartella esistente …
hdr-menu-import = Importa un server da ZIP/backup …
hdr-menu-entfernen = Rimuovi dall’elenco il server selezionato
hdr-einstellungen = Impostazioni
hdr-hell-dunkel = Cambia chiaro/scuro
hdr-ev-gewechselt = Server cambiato: { $name }
hdr-err-umschalten = Cambio non riuscito

## Barra laterale
side-bereiche = Sezioni
side-betrieb = Funzionamento
side-uebersicht = Panoramica
side-konsole = Console
side-spieler = Giocatori
side-inhalte = Contenuti
side-mods = Mod/Plugin
side-updates = Versione e aggiornamenti
side-backups = Backup
side-netzwerk = Rete
side-erreichbarkeit = Raggiungibilità

## Benvenuto
willkommen-titel = Benvenuto in Vapafane
willkommen-text = Vapafane non è un prodotto Minecraft ufficiale. Non è approvato da Mojang o Microsoft né associato a loro.

    Domande, idee o errori: sulla pagina GitHub del progetto (Impostazioni → Informazioni su Vapafane).

## Comune
gem-ordner = Cartella
gem-ordner-oeffnen = Apri cartella
gem-launcher = Launcher ufficiale di Minecraft
gem-modrinth-app = App Modrinth

## Informazioni su Vapafane
about-titel = Informazioni su Vapafane
about-version = Versione { $version }
about-testmodus = MODALITÀ TEST
about-nicht-offiziell = Vapafane non è un prodotto Minecraft ufficiale. Non è approvato da Mojang o Microsoft né associato a loro.
about-marken = Minecraft è un marchio di Mojang/Microsoft. Mod e plugin provengono dai rispettivi autori (tramite Modrinth o CurseForge).
about-kontakt = Contatti e segnalazione errori (GitHub) ↗
about-projektseite = Pagina del progetto ↗

## Aspetto
skin-titel = Aspetto
skin-grass-sub = Chiaro o scuro (pulsante in alto a destra)
skin-obsidian-sub = Sempre scuro

## Impostazioni client
cset-einstellung = Impostazione
cset-einrichten = Installazione
cset-ziel = Destinazione predefinita
cset-ziel-sub = Dove vengono installati i nuovi server
cset-spielordner = Cartelle di gioco di tutti i client
cset-spielordner-sub = Qui si trovano mod, opzioni e mondi di ogni server

## Backup
bak-restore-frage = Riportare «{ $name }» allo stato di { $datei }?
bak-restore-text = Per farlo il server verrà fermato. Lo stato attuale non viene cancellato: resta accanto in una cartella chiamata «…_vor_Wiederherstellung_<data>».
bak-wiederherstellen = Ripristina
bak-ev-wiederhergestellt = Backup ripristinato
bak-restore-fertig-titel = Ripristinato
bak-restore-fertig = Il server è tornato allo stato del { $label }. Ora puoi avviarlo.
bak-restore-daneben = Lo stato precedente si trova in:
    { $ordner }
    (puoi cancellarlo se va tutto bene)
bak-err-restore = Ripristino non riuscito
bak-waehlen = Scegli un backup
bak-erklaerung = Ogni backup è l’intera cartella del server in ZIP, con la data nel nome. Salvato in { $ordner }. Si conservano i { $anzahl } più recenti, i più vecchi vengono cancellati.
bak-jetzt = Esegui backup adesso
bak-ordner = Apri cartella dei backup
bak-andere = Ripristina un altro ZIP …
bak-aufheben = Conserva:
bak-leer = Ancora nessun backup per questo server.

## Raggiungibilità
rch-fehlgeschlagen = Verifica non riuscita
rch-noch-nicht = Non ancora verificato
rch-kein-server = Nessun server selezionato
rch-kein-server-text = Crea un server con «+» in alto o scegline uno, poi parte la verifica.
rch-prueft = Verifica in corso …
rch-geprueft-um = Verificato alle { $zeit }
rch-ip-erklaerung = IPv4 è l’indirizzo internet classico, IPv6 quello nuovo con un indirizzo proprio per dispositivo. Con DS-Lite il tuo provider condivide l’indirizzo IPv4 – allora funziona solo IPv6.
rch-erneut = Verifica di nuovo
rch-portfreigabe = Guida all’apertura della porta ↗
rch-dienst-hinweis = Per la verifica Vapafane chiede a un servizio su internet (ifconfig.co) se la porta del server è aperta dall’esterno. Disattivare: Impostazioni → Automazione.

## Invitare amici
inv-titel = Invita amici
inv-erklaerung = Manda un codice ai tuoi amici – Vapafane configura il loro Minecraft con le mod giuste.
inv-adresse-label = Indirizzo con cui i tuoi amici raggiungono il server
inv-oeffentlich = Rileva indirizzo pubblico
inv-adresse-hinweis = È precompilato l’indirizzo della rete locale (solo stessa rete Wi-Fi). Per amici da fuori: rilevare l’indirizzo pubblico e aprire la porta del server nel router.
inv-code-erzeugen = Crea codice
inv-als-datei = Salva come file …
inv-kein-passwort = Il codice non contiene password – solo indirizzo, versione e mod.
inv-datei-gespeichert = Salvato:
inv-datei-schicken = manda questo file ai tuoi amici.
inv-code-label = Codice di invito ({ $zeichen } caratteri) – copialo e mandalo ad es. in chat:
inv-kopieren = Copia
inv-kopiert = Copiato
inv-mods = { $anzahl } mod per i giocatori
inv-mods-optional = (di cui { $anzahl } facoltative)
inv-keine-mods = Nessuna mod necessaria – solo la versione { $version }.
inv-ipv6 = Contiene anche il tuo indirizzo IPv6 { $adresse }.
inv-nur-ipv6 = – secondo l’ultima verifica il tuo server è raggiungibile solo via IPv6.
inv-nur-server = Solo lato server (ai giocatori non servono): { $mods }
inv-datei-noetig = Queste mod non sono né su Modrinth né su CurseForge: { $mods }. Non entrano nel codice – usa «Salva come file», lì vengono incluse.

## Giocatori e whitelist
spi-online-titel = Online adesso
spi-wl-titel = Whitelist (chi può entrare)
spi-wl-erklaerung = Con la whitelist attiva (Impostazioni → «Solo whitelist») possono entrare solo i giocatori elencati qui. Protegge dagli sconosciuti – aggiungi anche te stesso.
spi-name-platzhalter = Nome Minecraft
spi-hinzufuegen = Aggiungi
spi-name-ungueltig = Inserisci un nome Minecraft valido (3–16 caratteri).
spi-nur-laufend = Aggiungere funziona solo a server avviato (verifica il nome presso Mojang).
spi-wl-leer = La whitelist è vuota.
spi-niemand = Nessuno online
spi-kreativ = Creativa
spi-ueberleben = Sopravvivenza
spi-op = Dai OP
spi-op-weg = Togli OP
spi-kick = Espelli
spi-kick-frage = Espellere { $name }?
spi-kick-text = Può rientrare subito.
spi-err-befehl = Comando non riuscito
wl-titel = Whitelist
wl-einschalten = Attiva whitelist
wl-zu-spielern = Vai ai giocatori
wl-warnung-offen = Attenzione: il tuo server è raggiungibile da internet e la whitelist è disattivata – chiunque conosca o trovi l’indirizzo può entrare. Con la whitelist entrano solo i giocatori che aggiungi tu.
wl-warnung-leer = La whitelist è attiva ma ancora vuota – per ora non può entrare nessuno, nemmeno tu. Aggiungi il tuo nome Minecraft e quelli dei tuoi amici in «Giocatori».
wl-tipp = Consiglio: la whitelist è disattivata – chiunque abbia l’indirizzo può entrare. È consigliata appena giocano amici da fuori: allora entrano solo i giocatori elencati qui.

## Console
kon-neues-log = nuovo file di log (il server è stato riavviato)
kon-beigetreten = { $name } è entrato
kon-verlassen = { $name } ha lasciato la partita
kon-bereit = Server pronto (avviato in { $sekunden } s)
kon-fehler = Errore: { $fehler }
kon-tag = Giorno
kon-nacht = Notte
kon-wetter = Tempo sereno
kon-wer-online = Chi è online?
kon-platzhalter = Scrivi un comando, ad es. say Ciao  ·  time set day  ·  weather clear
kon-senden = Invia
kon-log-ordner = Apri cartella dei log

## Panoramica
ueb-laeuft = In funzione
ueb-startet = Avvio …
ueb-stoppt = Arresto …
ueb-neustart-absturz = Riavvio dopo crash …
ueb-gestoppt = Fermo
ueb-abgestuerzt = In crash
ueb-extern = Funziona fuori da Vapafane
ueb-anderer = Un altro server è in funzione
ueb-starten = Avvia
ueb-stoppen = Ferma
ueb-neustart-knopf = Riavvia
ueb-seit = da { $dauer }
ueb-neustarts = riavviato automaticamente { $anzahl }×
ueb-hinweis-extern = Il server non è stato avviato con Vapafane (ad es. vecchio pannello o terminale). Console e giocatori funzionano comunque; «Ferma» gli invia il comando stop.
ueb-hinweis-anderer = Il servizio in background esegue un altro server. Fermalo prima.
ueb-hinweis-absturz = Il server è andato in crash e non è stato riavviato. Guarda l’ultimo messaggio di errore nella console.
ueb-hinweis-kein-server = Aggiungi un server con «＋» in alto.
ueb-ev-bereit = Server pronto
ueb-ev-gestoppt = Server fermato
ueb-ev-startet = Avvio del server
ueb-ev-neustart = Il server si riavvia
ueb-ev-stoppt = Il server si ferma
ueb-ev-in-sek = tra { $sekunden } s
ueb-ev-absturz = Il server è andato in crash
ueb-ev-absturz-neustart = Il server è andato in crash – riavvio automatico
ueb-notify-absturz = Server Minecraft in crash
ueb-notify-absturz-neustart = Si riavvia automaticamente.
ueb-ev-backup-start = Backup avviato
ueb-ev-backup-fertig = Backup completato: { $datei } ({ $groesse })
ueb-err-start = Avvio non riuscito
ueb-err-stop = Arresto non riuscito
ueb-err-backup = Backup non riuscito
ueb-eula-titel = Conferma EULA
ueb-eula-text = Nessun server si avvia senza accettare l’EULA di Minecraft.
ueb-eula-akzeptiere = Accetto l’
ueb-eula-fuer = per «{ $name }».
ueb-verb-stoppen = fermare
ueb-verb-neustart = riavviare
ueb-stop-frage = { $verb } il server?
ueb-stop-extern-text = Il server funziona fuori da Vapafane. Gli invio il comando «stop» – salva il mondo e si ferma.
ueb-stop-niemand = Al momento non c’è nessuno online.
ueb-stop-spieler = { $anzahl } giocatori online: { $namen }
ueb-sofort = Subito
ueb-mit-warnung = Tra { $sekunden } s con avviso
ueb-karte-spieler = Giocatori online
ueb-karte-tps = TPS (tick al secondo)
ueb-karte-ram = Memoria
ueb-karte-ordner = Cartella del server
ueb-karte-aussen = Dall’esterno
ueb-karte-adresse = Indirizzo (rete locale)
ueb-karte-backup = Ultimo backup
ueb-karte-version = Versione
ueb-server-aus = server spento
ueb-keine-verbindung = nessuna connessione
ueb-niemand = nessuno
ueb-ms-pro-tick = { $ms } ms per tick
ueb-ruckelt = a scatti
ueb-fluessig = fluido
ueb-von-kernen = di { $anzahl } core
ueb-ram-belegt-limit = in uso (limite { $limit } GB)
ueb-ram-belegt = in uso
ueb-ram-limit = limite { $limit } GB
ueb-frei = { $groesse } liberi
ueb-vor = { $dauer } fa
ueb-noch-keins = ancora nessuno
ueb-backup-knopf = premi il pulsante «Backup»
ueb-backup-sub = { $groesse } · { $anzahl } conservati
ueb-erreichbar = Raggiungibile
ueb-nur-ipv6 = Solo IPv6
ueb-nicht-erreichbar = Non raggiungibile
ueb-ipv4-und-6 = IPv4 e IPv6
ueb-ueber-ipv4 = via IPv4
ueb-portfreigabe = verifica apertura porta
ueb-ereignisse = Eventi recenti
ueb-nichts-passiert = Non è ancora successo nulla.
ueb-fenster-zu = Chiudere la finestra non ferma il server – continua in background.
ueb-kontakt = Contatti e segnalazione errori (GitHub)

## Impostazioni (server)
set-spiel = Gioco (questo server)
set-motd = Nome del server (MOTD)
set-max-spieler = Giocatori max
set-schwierigkeit = Difficoltà
set-wert-peaceful = Pacifica
set-wert-easy = Facile
set-wert-normal = Normale
set-wert-hard = Difficile
set-spielmodus = Modalità di gioco dei nuovi giocatori
set-wert-survival = Sopravvivenza
set-wert-creative = Creativa
set-wert-adventure = Avventura
set-wert-spectator = Spettatore
set-sichtweite = Distanza di visualizzazione (chunk)
set-simulationsweite = Distanza di simulazione (chunk)
set-spawn-schutz = Protezione dello spawn (blocchi)
set-port = Porta del server
set-whitelist = Solo whitelist
set-whitelist-hilfe = Consigliato per sicurezza: entrano solo i giocatori della whitelist (pagina «Giocatori»). Senza, può entrare chiunque conosca il tuo indirizzo.
set-leistung = Prestazioni (questo server)
set-ram = Memoria max per il server (GB)
set-ram-hinweis = Questo PC ha { $gb } GB. Lascia circa 3–4 GB per il sistema.
set-speichern = Salva impostazioni
set-gespeichert = Salvato.
set-gespeichert-neustart = Salvato. Avrà effetto al prossimo riavvio.
set-err-speichern = Salvataggio non riuscito
set-automatik = Automazione
set-autostart = Avvia automaticamente il server attivo all’accensione del PC
set-auto-testmodus = Disattivato in modalità test.
set-auto-kein-systemd = Non disponibile su questo sistema (niente systemd).
set-auto-windows = Si avvia all’accensione del PC, anche senza accesso (Windows chiede i diritti di amministratore all’attivazione).
set-auto-linger-fehlt = Attenzione: si avvia solo al tuo accesso. Clicca qui per consentire l’avvio senza accesso.
set-auto-aktiv = Si avvia anche senza accesso e si riavvia da solo dopo un crash.
set-linger-frage = Consentire l’avvio senza accesso?
set-linger-text = Per questo systemd deve poter avviare i tuoi servizi utente già all’accensione («loginctl enable-linger»). Dopo, il server funziona anche se nessuno ha effettuato l’accesso.
set-linger-erlauben = Consenti
set-linger-abgelehnt = Non consentito
set-fenster-autostart = Apri questa finestra all’accesso
set-backup-um = Backup giornaliero alle
set-backup-uhr =  (server completo in ZIP)
set-backup-sub = Lo esegue il servizio in background. Viene recuperato se il PC era spento a quell’ora.
set-uhrzeit = Orario del backup giornaliero
set-reach = Verifica la raggiungibilità dall’esterno
set-reach-sub = Passando all’area server. Per questo interroga un servizio di verifica su internet.
set-notify = Notifiche
set-notify-sub = Quando qualcuno entra o esce e quando il server va in crash (finché la finestra è aperta). Senza suono.
set-ordner = Cartelle
set-ordner-server = Cartella del server
set-an = attivo
set-aus = disattivato
set-panel-titel = È ancora configurato un vecchio pannello di controllo.
set-panel-status = Servizio «minecraft.service»: { $dienst }, all’avvio { $start }; programma di backup giornaliero { $backup }. Finché funziona, Vapafane non può avviare il server da solo (stessa porta), e i backup verrebbero fatti due volte.
set-panel-knopf = Disattiva il vecchio pannello …
set-panel-frage = Disattivare il vecchio pannello?
set-panel-text = Fermo il servizio «minecraft.service» (il server salva e si spegne correttamente) e disattivo lui e il suo backup giornaliero all’accensione del PC.

    Il vecchio pannello e i suoi file restano – si può riattivare in qualsiasi momento. Poi avvii il server qui con «Avvia».
set-panel-abschalten = Disattiva
set-panel-aus = Vecchio pannello disattivato
set-panel-err = Disattivazione non riuscita

## CurseForge
cf-titel = CurseForge (facoltativo)
cf-experimentell = Sperimentale
cf-brauche = Mi serve?
cf-erklaerung = No – la maggior parte delle mod è su Modrinth, che funziona senza configurazione. CurseForge trova in più mod che esistono solo lì e richiede per questo una chiave API personale (una specie di codice di accesso).
cf-anleitung-titel = Come ottenere una chiave (gratis)
cf-schritt-1 = Clicca su «Richiedi chiave» e compila il modulo (ad es. scopo: «Mod per il mio server»).
cf-schritt-2 = Aspetta la risposta via e-mail – CurseForge controlla a mano, può volerci qualche giorno.
cf-schritt-3 = Incolla la chiave qui sotto, «Prova chiave» e poi «Salva».
cf-persoenlich = La chiave è personale
cf-persoenlich-text = – non condividerla né pubblicarla. Resta solo su questo PC. Valgono le
cf-bedingungen = condizioni CurseForge per programmi di terzi
cf-experimentell-text = Sperimentale: CurseForge decide se concedere chiavi agli strumenti privati.
cf-beantragen = Richiedi chiave ↗
cf-konsole = Console CurseForge ↗
cf-mehr = Altro sull’API CurseForge ↗
cf-platzhalter = Incolla qui la chiave API
cf-testen = Prova chiave
cf-teste = Prova in corso …
cf-funktioniert = La chiave funziona.
cf-schluessel-da = Chiave salvata: { $maskiert } – CurseForge è incluso nelle ricerche.
cf-kein-schluessel = Nessuna chiave salvata – si cerca solo su Modrinth.
cf-gespeichert = Chiave CurseForge salvata
cf-entfernen-frage = Eliminare la chiave CurseForge?
cf-entfernen-text = Dopo Vapafane cerca di nuovo solo su Modrinth.

## Aggiungere un server
gem-anderer-ordner = Altra cartella …
srv-typ-fabric = Fabric (mod)
srv-typ-paper = Paper (plugin)
srv-typ-neoforge = NeoForge (mod)
srv-typ-vanilla = Vanilla (senza mod)
srv-name-label = Nome nell’elenco dei server
srv-typ-label = Tipo di server
srv-version-label = Versione di Minecraft
srv-laedt = caricamento …
srv-keine-verbindung = Nessuna connessione: { $fehler }
srv-ordner-waehlen = Scegli una cartella per il server
srv-server-ordner-waehlen = Scegli la cartella del server
srv-backup-waehlen = Scegli un backup del server (ZIP o tar)
srv-ordner-nicht-leer = La cartella scelta non è vuota – il server andrà in una nuova sottocartella.
srv-ordner-ungueltig = Questa cartella non può essere usata.
srv-eula-zusatz = (senza, nessun server si avvia)
srv-hinweis = Nota
srv-ev-hinzu = Server aggiunto: { $name }
srv-hinzu-titel = Aggiungi server
srv-hinzu-text = «{ $name }» è nell’elenco. Ferma il server in funzione per passare.
srv-err-hinzu = Aggiunta non riuscita
srv-kein-server = Nessun server trovato
srv-import-titel = Importa server
srv-err-import = Importazione non riuscita
srv-neu-titel = Crea nuovo server
srv-neu-name = Il mio server
srv-err-erstellen = Creazione non riuscita
srv-entfernen-frage = Rimuovere «{ $name }» dall’elenco?
srv-entfernen-text = I file in { $ordner } restano, sparisce solo la voce.
srv-err-entfernen = Rimozione non riuscita

## Mod/Plugin
mods-erklaerung = Le mod disattivate restano nella cartella ma non vengono caricate. Le modifiche hanno effetto al prossimo avvio del server.
mods-ev-an = { $name } attivata
mods-ev-aus = { $name } disattivata
mods-neustart-noetig = Il server è in funzione – la modifica avrà effetto dopo il prossimo riavvio.
mods-nur-client = solo client
mods-aus-grund = Disattivata: { $grund }
mods-an-tipp = Attivata – clic per disattivare
mods-aus-tipp = Disattivata – clic per attivare
mods-anzahl = { $an } su { $gesamt } { $wort } attivati
mods-datapacks-anzahl = { $anzahl } datapack
mods-leer = Nessun { $wort } nella cartella «{ $ordner }».
mods-datapacks-leer = Nessun datapack nel mondo.
mods-datapacks-titel = Datapack nel mondo
mods-hinzufuegen = Aggiungi { $wort } …
mods-hinzufuegen-titel = Aggiungi { $wort }
mods-ordner-oeffnen = Apri cartella delle mod
mods-aufraeumen = Pulizia …
mods-aufraeumen-titel = Pulizia
mods-aufraeumen-tipp = Elimina i resti di versioni vecchie (il server deve essere fermo)
mods-alles-sauber = Tutto pulito – non ci sono resti di versioni vecchie.
mods-aufraeumen-text-1 = Verrebbero eliminati questi resti di versioni vecchie (in totale
mods-aufraeumen-text-2 = Mondo, impostazioni e mod disattivate restano. Ciò che serve ancora al server viene riscaricato al prossimo avvio.
mods-erst-stoppen = Ferma prima il server per eliminare.
mods-loeschen-knopf = Elimina { $groesse }
mods-ev-aufgeraeumt = Pulizia fatta: { $groesse } liberati
mods-aufgeraeumt-titel = Pulizia fatta
mods-aufgeraeumt = { $anzahl } voci eliminate, { $groesse } liberati.
mods-nicht-loeschbar = Non eliminabile:
mods-err-aufraeumen = Pulizia non riuscita

## Catalogo (aggiungere mod)
kat-suche = Cerca su Modrinth … (vuoto = i più popolari)
kat-dp-option = Datapack (per il mondo)
kat-treffer = { $anzahl } { $wort } compatibili per Minecraft { $mc }
kat-beliebtheit = ordinati per popolarità
kat-sucht = Ricerca …
kat-nichts = Nessun risultato.
kat-mehr = Carica altri
kat-cf-tipp = Consiglio: con una chiave CurseForge (Impostazioni) trovi in più mod che esistono solo lì.
kat-nur-cf = Solo su CurseForge
kat-cf-erklaerung = Questi non sono su Modrinth. Alcuni autori non permettono il download tramite altri programmi – allora scarichi il file nel browser e lo metti tu nella cartella.
kat-beides = Server + client
kat-beides-tipp = Serve anche ai giocatori – ad es. tramite «Esporta mod per gli amici (.mrpack)»
kat-nur-server = Solo server
kat-meta = { $downloads } download · di { $autor }
kat-laedt = Caricamento …
kat-installiert = Installato
kat-installieren = Installa
kat-ev-installiert = Installato: { $name }
kat-err-install = L’installazione non ha funzionato: { $fehler }
kat-ist-installiert = è installato (in «{ $ordner }»)
kat-abhaengigkeiten = Aggiunto come dipendenza: { $namen }.
kat-schon-da = Già presenti: { $namen }.
kat-fehlende = Attenzione: questa dipendenza non esiste per questa versione: { $namen } – così il server potrebbe non avviarsi.
kat-client-noetig = Anche i giocatori hanno bisogno di questa mod – il modo più semplice è «Esporta mod per gli amici (.mrpack)» in «Versione e aggiornamenti».
kat-manuell-1 = L’autore non permette il download tramite altri programmi. Scarica il file
kat-manuell-2 = nel browser e mettilo nella cartella «{ $ordner }».
kat-browser = Scarica nel browser ↗
kat-dp-ordner = Apri cartella dei datapack
kat-neustart = Il server è in funzione – avrà effetto dopo un riavvio.
kat-jetzt-neustarten = Riavvia adesso

## Versione e aggiornamenti
upd-st-aktuell = aggiornato
upd-st-fehlt = mancante
upd-st-unbekannt = sconosciuto
upd-st-neu = nuovo (necessario)
upd-st-manuell = aggiornabile solo nel browser (CurseForge)
upd-st-reenable = di nuovo compatibile – verrà attivato
upd-st-doppelt = doppione (già presente come mod)
upd-alles-aktuell = Tutto aggiornato.
upd-zusammenfassung = { $updates } aggiornamenti, { $aktuell } già aggiornati, { $fehlend } senza versione compatibile, { $unbekannt } sconosciuti
upd-neu-gebraucht = { $anzahl } ora necessari
upd-passen-wieder = { $anzahl } di nuovo compatibili
upd-doppelt = { $anzahl } doppioni (verranno messi da parte)
upd-server-stand = Nuova build del server per Minecraft { $mc }: { $label }.
upd-beta-hinweis = Nota: per ora esiste solo una versione di prova/beta.
upd-wechsel = Server: Minecraft { $von } → { $nach } ({ $label }).
upd-welt-warnung = Attenzione: un mondo avviato una volta con la nuova versione non può tornare indietro. L’unico ritorno è il backup fatto automaticamente.
upd-java-da = Serve Java { $version } – presente ({ $pfad }).
upd-java-fehlt = Serve Java { $version }, ma non è installato. { $hinweis }
upd-manuell-hinweis = Per le mod «aggiornabile solo nel browser» l’autore non permette il download automatico – scarica il nuovo file su CurseForge e sostituiscilo tu.
upd-platz = Spazio su disco insufficiente per il backup ({ $frei } liberi, server { $server }).
upd-nur-mods = { $version } (aggiorna solo mod/plugin)
upd-keine-neuere = Non c’è una versione stabile di Minecraft più recente.
upd-keine-verbindung = Nessuna connessione
upd-err-pruefen = Verifica non riuscita
upd-rollback-frage = Tornare allo stato precedente all’ultimo aggiornamento?
upd-rollback-text = Il server viene fermato e ripristinato dal backup fatto durante l’aggiornamento. Lo stato attuale resta accanto in una cartella «…_vor_Wiederherstellung_<data>».
upd-zuruecksetzen = Ripristina
upd-ev-rollback = Ripristinato lo stato precedente all’aggiornamento
upd-rollback-fertig-titel = Ripristinato
upd-rollback-fertig = Il server è tornato allo stato del { $label }. Ora puoi avviarlo.
upd-rollback-daneben = Lo stato successivo all’aggiornamento si trova in:
upd-err-rollback = Ripristino non riuscito
upd-rollback-knopf = Torna allo stato precedente all’ultimo aggiornamento
upd-was-mods = Aggiorna mod
upd-ablauf = Procedura: fermare il server (con avviso se c’è qualcuno online) → backup completo → scaricare e verificare { $anzahl } file → sostituire → avviare il server e controllare.
upd-werden-deaktiviert = { $anzahl } mod/datapack senza versione compatibile verranno disattivati.
upd-probelauf-tipp = Consiglio: con «Prova» puoi testarlo senza rischi su una copia.
upd-start-frage = Avviare l’aggiornamento? ({ $was })
upd-starten = Avvia aggiornamento
upd-ev-gestartet = Aggiornamento avviato ({ $was })
upd-err-update = Aggiornamento non riuscito
upd-ev-fertig = Aggiornamento completato ({ $was })
upd-fertig-titel = Aggiornamento completato
upd-fertig-1 = Il server funziona di nuovo ({ $was }).
upd-fertig-backup = Backup dello stato precedente:
upd-fertig-aufgeraeumt = Pulizia: { $anzahl } vecchi file/cartelle eliminati ({ $groesse }).
upd-fertig-2 = Le mod senza versione compatibile sono disattivate e visibili in «Mod/Plugin».
upd-fehlgeschlagen-titel = L’aggiornamento non ha funzionato
upd-fehlgeschlagen-frage = Guarda il messaggio di errore nella console. Ripristino lo stato precedente all’aggiornamento dal backup?
upd-trial = Prova
upd-trial-starten = Avvia prova
upd-trial-frage = Fare una prova?
upd-trial-text = Copio il server nella cartella temporanea, aggiorno la copia, la avvio su un’altra porta e verifico che si avvii del tutto. Poi la copia viene fermata ed eliminata.

    Il tuo vero server resta intatto e può continuare a funzionare. Con mondi grandi la copia richiede qualche minuto.
upd-trial-tipp = Aggiorna una copia nella cartella temporanea e la avvia su un’altra porta
upd-trial-err = Prova non riuscita
upd-trial-ok-titel = Prova superata
upd-trial-ok = La copia aggiornata ha completato l’avvio in { $sekunden } secondi (porta { $port }).
upd-trial-timeout = La copia non aveva completato l’avvio dopo 15 minuti.
upd-trial-exit = La copia si è chiusa all’avvio (codice di uscita { $code }).
upd-trial-meldungen = Messaggi rilevanti:
upd-trial-zeilen = Ultime righe:
upd-trial-geloescht = La copia è stata eliminata.
upd-trial-nicht-geloescht = La copia non si è potuta eliminare del tutto: { $ordner }
upd-ev-trial-ok = Prova superata
upd-ev-trial-fehler = Prova non riuscita
upd-jetzt = Adesso:
upd-erklaerung = Verifica se tutte le mod esistono per la versione scelta. Prima di ogni aggiornamento viene fatto automaticamente un backup.
upd-ziel = Obiettivo:
upd-versionen = Carica versioni
upd-pruefen = Verifica
upd-spalte-mod = Mod / datapack
upd-spalte-neu = Nuovo
upd-deaktivieren-fehlende = Disattiva mod/datapack senza versione compatibile (altrimenti il server probabilmente non si avvierà)
upd-deaktivieren-unbekannte = Disattiva mod/datapack sconosciuti (non su Modrinth)
upd-nicht-unterstuetzt = L’assistente di aggiornamento esiste per Fabric, Paper, NeoForge e Vanilla.
upd-export-knopf = Esporta mod per gli amici (.mrpack)
upd-export-tipp = Solo per server Fabric e NeoForge (mod che servono anche agli amici)
upd-export-titel = Modpack salvato
upd-export-mods = { $anzahl } mod da Modrinth
upd-export-eingepackt = { $anzahl } incluse direttamente
upd-export-hinweis = I tuoi amici importano il file nell’app Modrinth («Aggiungi file»). Le mod solo server non vengono installate lì.
upd-err-export = Esportazione non riuscita

## Client
cli-titel = Usa un invito
cli-code-erklaerung-1 = Incolla il codice (inizia con
cli-code-erklaerung-2 = ) o apri il file
cli-code-erklaerung-3 =  – Vapafane configura Minecraft con tutte le mod adatte al server.
cli-code-platzhalter = Incolla qui il codice di invito (VPF1-…)
cli-pruefen = Verifica invito
cli-datei-oeffnen = Apri file di invito …
cli-pruefe-einladung = Verifica dell’invito …
cli-launcher-da = Launcher Minecraft trovato
cli-kein-launcher = Nessun launcher Minecraft
cli-modrinth-da = App Modrinth trovata
cli-meine-server = I miei server
cli-leer = Nessun client configurato – usa un invito qui sopra.
cli-kein-konto = Vapafane non avvia Minecraft da solo e non tocca mai il tuo account Microsoft – «Gioca» apre il launcher ufficiale o l’app Modrinth.
cli-nicht-erreichbar = Server non raggiungibile
cli-online = Online · { $version } · { $spieler }/{ $max } giocatori
cli-version-passt-nicht = Online, ma versione { $version } – non corrisponde
cli-aktualisiert = Il server è stato aggiornato ({ $version }) – chiedi all’admin un nuovo codice di invito
cli-gerade-nicht = al momento non raggiungibile
cli-modrinth-fehlt = App Modrinth non trovata
cli-launcher-fehlt = Launcher Minecraft non trovato
cli-download-seite = Vai alla pagina di download
cli-ev-eingerichtet = Client configurato: { $name }
cli-gefunden = trovato
cli-nicht-gefunden = non trovato
cli-keine-mods = Nessuna mod necessaria – solo Minecraft { $version }.
cli-optional = facoltativa
cli-nur-browser = scaricabile solo nel browser (link dopo l’installazione)
cli-nur-datei = mancante – solo nel file di invito
cli-launcher-fehlt-text = Il launcher Minecraft ufficiale non si trova. Puoi comunque configurare; il profilo verrà creato appena il launcher è installato e premi qui «Aggiorna».
cli-launcher-laden = Scarica launcher
cli-nie-gestartet = Il launcher non è mai stato avviato. Avvialo una volta e chiudilo, poi configura qui.
cli-ordner-waehlen = Scegli una cartella per questo Minecraft
cli-mods-zusammen = { $anzahl } mod, { $groesse } in totale
cli-andere-version = Il server annuncia un’altra versione ({ $version }). Probabilmente è stato aggiornato – chiedi all’admin un nuovo codice di invito.
cli-spalte-version = Versione
cli-spalte-groesse = Dimensione
cli-einrichten-fuer = Configura per:
cli-in-modrinth = Nell’app Modrinth:
cli-instanz-update = Aggiorna l’istanza esistente «{ $name }» (consigliato)
cli-instanz-update-sub = Tutto resta nella stessa istanza: mondi, impostazioni, shader, resource pack e le mod aggiunte da te (ad es. Iris). Basta cliccare «Reinstalla modpack» nell’app Modrinth – Vapafane ti mostra come.
cli-instanz-neu = Crea nuova istanza
cli-instanz-neu-sub = Funziona automaticamente tramite l’importazione. Qui sotto puoi riprendere impostazioni, shader e resource pack – non le mod aggiunte da te. La vecchia istanza resta finché non la elimini.
cli-update-bleibt = Con l’aggiornamento tutto resta nell’istanza: mondi, opzioni, shader, resource pack e il tuo elenco server.
cli-import-bestaetigen = Si apre l’app Modrinth – conferma solo l’importazione.
cli-ordner-label = Cartella:
cli-launcher-schliessen = Chiudi il launcher Minecraft prima della configurazione, così non sovrascrive la modifica.
cli-einrichten = Configura
cli-uebernehmen-von = Riprendi impostazioni da:
cli-nichts = Non riprendere nulla
cli-quelle-waehlen = Scegli la cartella Minecraft da cui riprendere
cli-nichts-uebernehmen = Nulla da riprendere
cli-teil-options = Opzioni e tasti
cli-teil-config = Impostazioni delle mod
cli-teil-shaders = Shader
cli-teil-resourcepacks = Resource pack
cli-teil-servers = Elenco server
cli-teil-worlds = Mondi
cli-welten-warnung = I mondi di una versione di Minecraft più nuova non si aprono in una più vecchia. I mondi grandi rallentano la configurazione.
cli-nur-fehlendes = Ciò che è già nella cartella resta intatto – si riprende solo ciò che manca.
cli-uebernehmen-erklaerung = Ad esempio dalla tua istanza precedente: opzioni, tasti, impostazioni delle mod, shader, resource pack ed elenco server. Le mod vengono sempre dal server, abbinate. La fonte viene solo letta, mai modificata.
cli-paket-update-1 = Il nuovo pacchetto Modrinth è pronto. Ecco come aggiornare la tua istanza
cli-paket-update-2 = mondi, opzioni, shader, resource pack e il tuo elenco server restano tutti:
cli-schritt-instanz = Apri l’app Modrinth e clicca sull’istanza
cli-schritt-anklicken = a sinistra.
cli-schritt-zahnrad = Clicca in alto sull’ingranaggio «Impostazioni istanza» e scegli la scheda «Installazione».
cli-neu-installieren = Reinstalla modpack
cli-schritt-datei = clicca, conferma e scegli questo file:
cli-modrinth-oeffnen = Apri l’app Modrinth
cli-paket-ordner = Apri la cartella con il pacchetto
cli-update-sicher = L’app Modrinth prima mette al sicuro l’istanza e sostituisce solo le mod del server – ciò che hai aggiunto tu (shader, resource pack, mod proprie) resta.
cli-paket-offen = Il pacchetto Modrinth è pronto e l’app Modrinth è stata aperta con esso. Conferma lì l’importazione – l’istanza apparirà come
cli-paket-fertig = Il pacchetto Modrinth è pronto:
cli-paket-von-hand = L’app Modrinth non si è potuta aprire automaticamente. A mano: nell’app Modrinth clicca su «+» (crea istanza), poi «Importa da file» e scegli questo file.
cli-eingerichtet = Configurato: { $anzahl } mod scaricate.
cli-profil = Il launcher Minecraft ha ora il profilo «{ $name }».
cli-uebernommen-paket = Le impostazioni di «{ $quelle }» sono nel pacchetto – l’app Modrinth le applica all’importazione.
cli-uebernommen = Ripreso da «{ $quelle }»: { $anzahl } file.
cli-uebernommen-da = ({ $anzahl } erano già presenti e sono rimasti intatti)
cli-uebernommen-fehler = { $anzahl } file non si sono potuti leggere e quindi mancano – Minecraft è ancora aperto lì? ({ $dateien })
cli-selbst-laden = Da scaricare tu stesso e mettere nella cartella «mods»:
cli-spielen = Gioca
cli-spielen-modrinth = Gioca (apri l’app Modrinth)
cli-spielen-launcher = Gioca (apri il launcher)
cli-prueft = verifica …
cli-unvollstaendig = La configurazione non è andata a buon fine – usa di nuovo l’invito qui sopra (o elimina).
cli-mods-kurz = { $anzahl } mod
cli-eingerichtet-am = { $ziel } · configurato il { $datum }
cli-aktualisieren = Aggiorna
cli-aktualisieren-tipp = Riconfigura con l’invito salvato – o spiega cosa fare dopo un aggiornamento del server
cli-entfernen-frage = Rimuovere «{ $name }»?
cli-entfernen-modrinth = Vapafane dimentica questo client ed elimina il suo pacchetto. L’istanza nell’app Modrinth resta – eliminala lì tu stesso.
cli-entfernen-launcher = Viene eliminata la cartella con mod, impostazioni e mondi in solitaria di questo profilo, e il profilo sparisce dal launcher Minecraft.
cli-upd-titel = Il server è stato aggiornato
cli-upd-text = «{ $name }» ora funziona con Minecraft { $version }. Versione e mod adatte sono nell’invito – ti serve quindi un nuovo codice di invito dall’admin.
cli-upd-code = Incollalo in alto in «Usa un invito» e clicca «Verifica invito».
cli-upd-modrinth = Vapafane crea allora un nuovo pacchetto e ti mostra come aggiornare la tua istanza – impostazioni, mondi e shader restano.
cli-upd-launcher = Vapafane sostituisce allora le mod – impostazioni e mondi restano.
cli-gespeicherte-einladung = Caricato l’invito salvato di «{ $name }». «Configura» rimette tutto a posto – ad esempio se mancano mod o il profilo è sparito. Se l’admin ha aggiornato il server, ti serve invece un nuovo codice di invito.
ext-knopf = Shader ed extra
ext-knopf-so-gehts = Shader ed extra – come si fa

## Shader ed extra
ext-titel = Aggiungi shader ed extra
ext-skizze-1-alt = Schema dell’istanza nell’app Modrinth
ext-skizze-2-alt = Schema della ricerca nell’app Modrinth
ext-skizze-leer = Nessun contenuto aggiuntivo installato
ext-skizze-hochladen = Carica file
ext-durchsuchen = Sfoglia contenuti
ext-skizze-shader = Uno shader, ad es. Complementary
ext-skizze-beschreibung = Descrizione · download
ext-keine-shader = Gli shader richiedono la mod Iris – che funziona solo su server Fabric o NeoForge. Questo server non ha un mod loader, quindi qui funzionano i resource pack, ma non gli shader.
ext-iris-modrinth = Per gli shader Minecraft ha bisogno della mod Iris: al passo 4 scegli prima «Mods» e installa «Iris Shaders» (Sodium arriva automaticamente), poi scegli uno shader in «Shaders».
ext-iris-launcher-1 = Per gli shader Minecraft ha bisogno della mod Iris (con Sodium): scarica entrambe per la versione Minecraft del server da
ext-iris-launcher-2 = e mettile nella cartella «mods».
ext-mr-einleitung-1 = Nella tua istanza
ext-mr-einleitung-2 = puoi aggiungere in qualsiasi momento shader, resource pack e mod tuoi. Restano con gli aggiornamenti del server – l’app Modrinth sostituisce solo le mod del server.
ext-schritt-content = Scegli in alto la scheda «Content» (contenuti).
ext-schritt-durchsuchen = Clicca sul pulsante verde «Sfoglia contenuti».
ext-schritt-art = Scegli in alto il tipo: «Shaders» per gli shader, «Resource Packs» per i resource pack o «Mods».
ext-schritt-installieren = Clicca «Installa» accanto alla voce desiderata – fatto.
ext-im-spiel = Nel gioco: Opzioni → Impostazioni grafiche → Shader → scegli lo shader.
ext-im-spiel-rp = Nel gioco: Opzioni → Resource pack → scegli il pacchetto.
ext-mods-update-hinweis = Le tue mod devi aggiornarle tu nell’app dopo un aggiornamento di Minecraft; shader e resource pack in genere no.
ext-launcher-einleitung = La cartella di gioco di questo server è solo sua: ciò che aggiungi lì resta con gli aggiornamenti del server.
ext-schritt-laden-shader = Scarica uno shader, ad es. su
ext-schritt-laden-rp = Scarica un resource pack, ad es. su
ext-nicht-entpacken = non estrarre il file .zip.
ext-schritt-shaderpacks = Clicca in basso su «Apri cartella» e metti il file nella cartella «shaderpacks». I resource pack vanno in «resourcepacks».
ext-schritt-resourcepacks = Clicca in basso su «Apri cartella» e metti il file nella cartella «resourcepacks».
ext-eigene-mods = Puoi mettere mod tue nella cartella «mods» – ma dopo un aggiornamento di Minecraft devi mantenerle compatibili tu stesso.
err-antwort-unlesbar = Risposta di «{ $befehl }» illeggibile: { $fehler }

## Rapporto di raggiungibilità
rch-w-erreichbar = raggiungibile
rch-w-nicht = non raggiungibile
rch-w-nicht-dslite = non raggiungibile (DS-Lite)
rch-w-nicht-geprueft = non verificato
rch-w-nicht-vorhanden = non presente
rch-unmoeglich-titel = Verifica al momento impossibile
rch-unmoeglich-text = Il servizio di verifica su internet non risponde. Riprova più tardi.
rch-gruen-beide = Raggiungibile via IPv4 e IPv6
rch-gruen-v4 = Raggiungibile via IPv4
rch-gruen-text = Tutti gli amici possono entrare.
rch-gelb-dslite-titel = DS-Lite: raggiungibile solo via IPv6
rch-gelb-dslite-text = Il tuo provider condivide l’indirizzo IPv4. Gli amici con IPv6 possono comunque entrare – sono la maggioranza.
rch-gelb-titel = Raggiungibile solo via IPv6
rch-gelb-text = Via IPv4 non entra nessuno: o manca l’apertura della porta IPv4 (porta { $port }) o il tuo provider condivide l’indirizzo IPv4 (DS-Lite). Gli amici con IPv6 possono comunque entrare.
rch-rot-titel = Non raggiungibile dall’esterno
rch-rot-text = Di solito manca l’apertura della porta nel router, oppure il firewall del PC blocca la porta { $port }.
rch-rot-dslite-v6 = Il tuo provider condivide l’indirizzo IPv4 (DS-Lite), quindi funziona solo IPv6. Apri la porta { $port } per IPv6 nel router e controlla il firewall del PC.
rch-rot-dslite = Il tuo provider condivide l’indirizzo IPv4 (DS-Lite) e il tuo PC non ha un indirizzo IPv6. Così nessuno da fuori può raggiungere il server.
rch-aus-titel = Il server non è in funzione
rch-aus-text = La verifica avverrà appena il server funziona.
rch-port-belegt-titel = La porta { $port } è già in uso
rch-port-belegt-text = Un altro programma – probabilmente un altro server Minecraft – usa questa porta. Così questo server non può avviarsi. Imposta una porta libera in Impostazioni → «Porta del server» (ad es. { $frei })
rch-hotspot-tipp = Il server è raggiungibile solo via IPv6, ma la tua internet non ha IPv6. Consiglio: collega il PC all’hotspot del telefono – la rete mobile di solito ha IPv6.

## Errori del programma (generale)
err-dienst-aus = Il servizio in background non è in funzione
err-kein-systemd = systemd non è disponibile qui (o modalità test).
err-kein-server = Nessun server selezionato.
err-schon-beschaeftigt = C’è già qualcosa in corso: { $was }
err-neustart-extern = Questo server funziona fuori da Vapafane – il riavvio funziona solo dal programma che l’ha avviato.
err-stop-knopf = Usa il pulsante Ferma nella panoramica (salva correttamente).
err-start-extern = Il server funziona fuori da Vapafane. Fermalo prima lì.
err-start-anderer = Al momento è in funzione un altro server. Fermalo prima.
err-stop-unmoeglich = Non è stato possibile fermare il server.
err-pfad = Percorso non valido
err-rcon-antwort = Risposta RCON non valida
err-adresse = Indirizzo non valido
cln-alte-neoforge = vecchia versione di NeoForge
cln-alte-bibliothek = vecchia versione di una libreria
cln-alte-mods = vecchie versioni di mod (messe da parte dal vecchio pannello/launcher)
cln-cache = cache delle mod (ricreata al prossimo avvio)
cli-datei-titel = Apri file di invito
srv-rcon-an = RCON attivo (serve a Vapafane per i comandi).
err-link-verboten = Questo indirizzo non può essere aperto.
err-pruefsumme = La somma di controllo non corrisponde: { $datei }
err-kein-fabric = Nessun loader Fabric trovato per Minecraft { $mc }
cf-err-leer = Incolla prima una chiave.
cf-err-id = ID CurseForge non valido
cf-err-format = Non sembra una chiave API (incollala senza spazi).
cf-err-antwort = Risposta non valida da CurseForge: { $fehler }
cf-err-abgelehnt = CurseForge rifiuta la chiave API (non valida o non ancora attivata).
cf-err-fehlt = Nessuna chiave CurseForge salvata.
kat-err-typ = «Aggiungi mod» non esiste per questo tipo di server.
kat-err-keine-version = «{ $name }» non esiste per Minecraft { $mc } ({ $loader }).
kat-err-cf-version = «{ $name }» non esiste su CurseForge per Minecraft { $mc }.
fort-suche = Ricerca di { $name } …
fort-suche-cf = Ricerca di { $name } su CurseForge …
inv-err-neuer = Questo codice di invito è di una versione più nuova di Vapafane. Aggiorna Vapafane.
inv-err-unvollstaendig = Il codice è incompleto o danneggiato – copialo per intero.
inv-err-kaputt = Il codice è danneggiato – copialo di nuovo.
inv-err-version = Questo invito è di un’altra versione di Vapafane.
inv-err-angaben = L’invito contiene dati non validi.
inv-err-mod = L’invito contiene una voce di mod non valida.
inv-err-typ = Gli inviti esistono per server Fabric, NeoForge, Paper e Vanilla.
inv-err-datei = Questo non è un file di invito Vapafane valido.
inv-err-datei-kaputt = L’invito nel file è danneggiato.
inv-err-oeffentlich = Non è stato possibile rilevare l’indirizzo pubblico.
flv-err-kein-server = Non esiste un server per Minecraft { $mc }.
flv-err-kein-paper = Nessun Paper per { $mc }.
flv-err-kein-neoforge = Non esiste NeoForge per Minecraft { $mc }.
fort-neoforge = L’installer di NeoForge prepara il server (scarica le librerie) …
fort-suche-server = Ricerca di { $was } per Minecraft { $mc } …
fort-pruefe-java = Verifica di Java …
fort-frage-cf = Interrogazione di CurseForge per le mod restanti …
fort-pruefe-deps = Verifica delle dipendenze …
fort-pruefe-dp = Verifica dei datapack …
fort-pruefe-java-platz = Verifica di Java e dello spazio su disco …
upd-err-dateiname = Nome file non valido da Modrinth: { $name }
upd-cf-fehler = Non è stato possibile verificare CurseForge: { $fehler }
upd-abhaengigkeit = (dipendenza)
upd-grund-fehlt = non esiste per Minecraft { $mc }
cli-err-port = Porta non valida nell’indirizzo
cli-err-antwort = Risposta non valida dal server
cli-err-profil-kaputt = { $datei } è danneggiato – avvia il launcher una volta.
cli-err-nicht-leer = La cartella { $ordner } non è vuota e non appartiene a Vapafane.
cli-err-datei-kaputt = Il file { $datei } dell’invito è danneggiato.
cli-err-fabric-profil = Profilo Fabric non valido.
cli-err-neoforge-java = NeoForge richiede Java { $version }. Avvia una volta il launcher Minecraft (porta Java) o installa Java.
cli-hinweis-launcher-starten = Avvia una volta il launcher Minecraft e chiudilo, poi configura di nuovo.
cli-hinweis-kein-launcher = Il launcher Minecraft ufficiale non si trova. Installalo, avvialo una volta, poi premi qui «Aggiorna».
cli-hinweis-nie-gestartet = Il launcher non è mai stato avviato. Avvialo una volta e chiudilo, poi «Aggiorna».
cli-paket-summary = Configurato da Vapafane per il server { $name } ({ $adresse })
fort-pruefe-mods = Verifica delle mod …
fort-lade = Download di { $name } …
fort-launcher = Configurazione del launcher …
fort-neoforge-install = Installazione di NeoForge { $version } (può richiedere qualche minuto) …
fort-uebernehme = Ripresa delle impostazioni …
fort-trage-ein = Aggiunta del server …
fort-sammle = Raccolta delle impostazioni …
fort-packe = Creazione del pacchetto Modrinth …
fort-fertig = Fatto.
task-update = Aggiornamento
task-backup = Backup (server completo in ZIP)
task-restore = Ripristina backup
task-client = Configura client
task-rollback = Ritorno allo stato precedente all’aggiornamento
task-pruefe-mods = Verifica delle mod per { $mc }
upd-err-erst-pruefen = Premi prima «Verifica».
upd-err-anderer-server = La verifica appartiene a un altro server. Verifica di nuovo.
upd-err-absturz = Il server è andato in crash all’avvio. { $grund }
upd-err-beendet = Il server si è chiuso subito.
upd-err-kein-backup = Non c’è un backup dell’ultimo aggiornamento per questo server.
upd-err-temp-platz = Spazio insufficiente nella cartella temporanea per una copia (servono { $groesse }).
cln-err-laeuft = La pulizia funziona solo a server fermo.
fort-raeume-auf = Pulizia dei vecchi file …
fort-loesche = Eliminazione dei vecchi file …
fort-loesche-kopie = Eliminazione della copia …
nbt-err-unvollstaendig = servers.dat è incompleto.
nbt-err-laenge = servers.dat contiene una lunghezza non valida.
nbt-err-eintrag = servers.dat contiene una voce sconosciuta.
nbt-err-tiefe = servers.dat è annidato troppo in profondità.
nbt-err-format = servers.dat ha un formato sconosciuto.
say-backup-start = Backup in corso …
say-backup-fertig = Backup completato.
say-backup-fehler = Backup non riuscito.
say-stoppt = si ferma
say-neustart = si riavvia
say-countdown = Il server { $was } tra { $sekunden } secondi!
svc-countdown = Server { $was } tra { $sekunden } s
svc-err-laeuft-nicht = Il server non è in funzione al momento.
svc-err-befehl = Comando sconosciuto: { $befehl }
svc-err-ordner-fehlt = Manca la cartella del server: { $ordner }
svc-err-eula = L’EULA di Minecraft non è ancora stata accettata per questo server.
svc-err-fremder-server = In questa cartella funziona già un server (PID { $pid }) non avviato da Vapafane. Fermalo prima (ad es. vecchio pannello).
svc-err-port = La porta { $port } ({ $art }) è già in uso. Su questo PC funziona un altro server Minecraft?
svc-signal = terminato da segnale
svc-beendet = terminato
svc-abgestuerzt = In crash ({ $grund }).
svc-taegliches-backup = Backup giornaliero
say-update-neustart = Il server si riavvia per un aggiornamento!
svc-err-java-start = Non è stato possibile avviare Java ({ $java }): { $fehler }

## Riga di comando
ctl-hilfe = Uso: vapafane --ctl <comando>

      status            mostra lo stato del server attivo
      start             avvia il server (avvia il servizio in background se serve)
      stop [SECONDI]    ferma il server correttamente, con conto alla rovescia facoltativo
      restart [SEC.]    riavvia il server
      wait-ready [SEC.] aspetta che il server sia pronto
      wait-stopped [S.] aspetta che il server sia fermo
      backup            salva l’intera cartella del server in ZIP
      cmd <COMANDO …>   invia un comando via RCON
      aktiv NOME        attiva un altro server dell’elenco (solo da fermo)
      neu TIPO NOME VER [CARTELLA]  crea un nuovo server (fabric, paper, neoforge, vanilla) e lo attiva
      update-check VER  verifica mod/datapack per Minecraft VER via Modrinth
      trial VER         prova: aggiorna una copia, la testa su un’altra porta, la elimina
      update VER        aggiornamento: ferma, backup, sostituisce, avvia (solo se la verifica è ok)
      mods              mostra mod e datapack con stato attivo/disattivo
      mod-an PERCORSO   attiva una mod (PERCORSO come in «mods», ad es. mods/x.jar)
      mod-aus PERCORSO  disattiva una mod (diventa .jar.disabled)
      aufraeumen [--ja] mostra i vecchi resti; con --ja vengono eliminati (server fermo)
      mod-suche [TERMINE] [--datapack]  cerca su Modrinth mod/plugin (o datapack) compatibili
      mod-install PROGETTO [--datapack] installa una mod/plugin/datapack con le dipendenze necessarie
      curseforge-schluessel [--loeschen]  legge la chiave CurseForge dall’input e la salva
      curseforge-test   prova la chiave CurseForge salvata
      einladung [INDIRIZZO]             mostra un codice di invito per gli amici (indirizzo = host:porta)
      einladung-datei FILE [INDIRIZZO]  scrive un file di invito (.vapafane)
      einloesen CODICE|FILE [--ordner C] [--alle-optionalen] [--ziel launcher|modrinth]
                … [--einstellungen-von CARTELLA] [--neue-instanz]
                                        configura un client per un invito (profilo launcher o pacchetto Modrinth);
                                        riprende le impostazioni (senza mondi) da CARTELLA; con --neue-instanz
                                        un’istanza esistente nell’app Modrinth non viene aggiornata
      clients                           mostra i client configurati
      client-entfernen CARTELLA         elimina un client e il suo profilo launcher
      ping INDIRIZZO                    interroga lo stato del server (come l’elenco server nel gioco)
      erreichbarkeit    verifica se il server è raggiungibile dall’esterno (IPv4/IPv6, DS-Lite)
      mrpack FILE       esporta le mod come modpack Modrinth (.mrpack) per gli amici
      quit-service      chiude il servizio in background (prima ferma il server)
ctl-fehler = Errore: { $fehler }
