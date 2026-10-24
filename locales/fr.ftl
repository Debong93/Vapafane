# Vapafane – Français (traduction automatique par IA, corrections bienvenues)

## Langue
sprache-titel = Langue
sprache-system = Langue du système (recommandé)
sprache-ki-hinweis = Cette traduction a été générée automatiquement (par IA) – merci de signaler les erreurs via « À propos de Vapafane ».
sprache-neustart = L’interface se recharge brièvement lors du changement.

## Einheiten und Zahlen
einheit-tag = j
einheit-std = h
einheit-min = min
einheit-sek = s
zahl-dezimal = ,
einheit-mio = M
einheit-tsd = k
zahl-tausender = {" "}

## Java
java-hinweis-windows = À installer p. ex. depuis https://adoptium.net (Temurin { $version }).
java-hinweis-linux = À installer avec : sudo apt install openjdk-{ $version }-jre-headless
err-java-fehlt = Java n’est pas installé sur ce PC – aucun serveur Minecraft ne peut démarrer sans lui. Ce serveur a besoin de Java { $version }. { $hinweis }
err-java-noetig = Minecraft { $mc } a besoin de Java { $version }, qui n’est pas installé. { $hinweis }

## Boutons communs
gem-ok = OK
gem-schliessen = Fermer
gem-abbrechen = Annuler
gem-weiter = Continuer
gem-entfernen = Supprimer
gem-speichern = Enregistrer
task-abgebrochen = Annulé

## En-tête
hdr-untertitel = Atelier Minecraft pour serveur & client
hdr-bereich = Zone
hdr-client = Client
hdr-server = Serveur
hdr-aktiver-server = Serveur actif (un seul tourne à la fois)
hdr-kein-server = (aucun serveur)
hdr-plus-erster = Créer ton premier serveur ici
hdr-plus = Ajouter un serveur
hdr-menu-neu = Créer un nouveau serveur (Fabric, Paper, NeoForge, Vanilla) …
hdr-menu-ordner = Ajouter un dossier existant …
hdr-menu-import = Importer un serveur depuis un ZIP/une sauvegarde …
hdr-menu-entfernen = Retirer le serveur sélectionné de la liste
hdr-einstellungen = Paramètres
hdr-hell-dunkel = Basculer clair/sombre
hdr-ev-gewechselt = Serveur changé : { $name }
hdr-err-umschalten = Changement impossible

## Barre latérale
side-bereiche = Sections
side-betrieb = Fonctionnement
side-uebersicht = Vue d’ensemble
side-konsole = Console
side-spieler = Joueurs
side-inhalte = Contenu
side-mods = Mods/Plugins
side-updates = Version & mises à jour
side-backups = Sauvegardes
side-netzwerk = Réseau
side-erreichbarkeit = Accessibilité

## Bienvenue
willkommen-titel = Bienvenue dans Vapafane
willkommen-text = Vapafane n’est pas un produit Minecraft officiel. Il n’est ni approuvé par Mojang ou Microsoft, ni associé à eux.

    Questions, idées ou bugs : via la page GitHub du projet (Paramètres → À propos de Vapafane).

## Commun
gem-ordner = Dossier
gem-ordner-oeffnen = Ouvrir le dossier
gem-launcher = Launcher Minecraft officiel
gem-modrinth-app = Application Modrinth

## À propos de Vapafane
about-titel = À propos de Vapafane
about-version = Version { $version }
about-testmodus = MODE TEST
about-nicht-offiziell = Vapafane n’est pas un produit Minecraft officiel. Il n’est ni approuvé par Mojang ou Microsoft, ni associé à eux.
about-marken = Minecraft est une marque de Mojang/Microsoft. Les mods et plugins proviennent de leurs auteurs respectifs (via Modrinth ou CurseForge).
about-kontakt = Contact & signaler un bug (GitHub) ↗
about-projektseite = Page du projet ↗

## Apparence
skin-titel = Apparence
skin-grass-sub = Clair ou sombre (bouton en haut à droite)
skin-obsidian-sub = Toujours sombre

## Paramètres client
cset-einstellung = Paramètre
cset-einrichten = Installation
cset-ziel = Cible par défaut
cset-ziel-sub = Où les nouveaux serveurs sont installés
cset-spielordner = Dossiers de jeu de tous les clients
cset-spielordner-sub = Les mods, options et mondes de chaque serveur sont ici

## Sauvegardes
bak-restore-frage = Remettre « { $name } » à l’état de { $datei } ?
bak-restore-text = Le serveur sera arrêté pour cela. L’état actuel n’est pas supprimé – il reste à côté dans un dossier nommé « …_vor_Wiederherstellung_<date> ».
bak-wiederherstellen = Restaurer
bak-ev-wiederhergestellt = Sauvegarde restaurée
bak-restore-fertig-titel = Restauré
bak-restore-fertig = Le serveur est revenu à l’état du { $label }. Tu peux le démarrer maintenant.
bak-restore-daneben = L’état précédent se trouve dans :
    { $ordner }
    (peut être supprimé si tout va bien)
bak-err-restore = Échec de la restauration
bak-waehlen = Choisir une sauvegarde
bak-erklaerung = Chaque sauvegarde est le dossier complet du serveur en ZIP, avec la date dans le nom. Enregistrée dans { $ordner }. Les { $anzahl } plus récentes sont gardées, les plus anciennes supprimées.
bak-jetzt = Sauvegarder maintenant
bak-ordner = Ouvrir le dossier des sauvegardes
bak-andere = Restaurer un autre ZIP …
bak-aufheben = Garder :
bak-leer = Pas encore de sauvegarde pour ce serveur.

## Accessibilité
rch-fehlgeschlagen = Échec de la vérification
rch-noch-nicht = Pas encore vérifié
rch-kein-server = Aucun serveur sélectionné
rch-kein-server-text = Crée un serveur avec « + » en haut ou choisis-en un, puis la vérification se lance.
rch-prueft = Vérification …
rch-geprueft-um = Vérifié à { $zeit }
rch-ip-erklaerung = IPv4 est l’adresse internet classique, IPv6 la nouvelle avec une adresse propre par appareil. Avec DS-Lite, ton fournisseur partage l’adresse IPv4 – seul IPv6 fonctionne alors.
rch-erneut = Vérifier à nouveau
rch-portfreigabe = Guide d’ouverture de port ↗
rch-dienst-hinweis = Pour vérifier, Vapafane demande à un service sur internet (ifconfig.co) si le port du serveur est ouvert de l’extérieur. Désactiver : Paramètres → Automatisation.

## Inviter des amis
inv-titel = Inviter des amis
inv-erklaerung = Envoie un code à tes amis – Vapafane configure leur Minecraft avec les mods adaptés.
inv-adresse-label = Adresse à laquelle tes amis joignent le serveur
inv-oeffentlich = Déterminer l’adresse publique
inv-adresse-hinweis = L’adresse du réseau local est pré-remplie (même Wi-Fi uniquement). Pour des amis à l’extérieur : déterminer l’adresse publique et ouvrir le port du serveur dans le routeur.
inv-code-erzeugen = Créer un code
inv-als-datei = Enregistrer comme fichier …
inv-kein-passwort = Le code ne contient aucun mot de passe – seulement l’adresse, la version et les mods.
inv-datei-gespeichert = Enregistré :
inv-datei-schicken = envoie ce fichier à tes amis.
inv-code-label = Code d’invitation ({ $zeichen } caractères) – à copier et envoyer p. ex. par messagerie :
inv-kopieren = Copier
inv-kopiert = Copié
inv-mods = { $anzahl } mods pour les joueurs
inv-mods-optional = (dont { $anzahl } optionnels)
inv-keine-mods = Aucun mod nécessaire – seulement la version { $version }.
inv-ipv6 = Contient aussi ton adresse IPv6 { $adresse }.
inv-nur-ipv6 = – d’après la dernière vérification, ton serveur n’est joignable que via IPv6.
inv-nur-server = Côté serveur uniquement (les joueurs n’en ont pas besoin) : { $mods }
inv-datei-noetig = Ces mods ne sont ni sur Modrinth ni sur CurseForge : { $mods }. Ils ne tiennent pas dans le code – utilise « Enregistrer comme fichier », ils y seront inclus.

## Joueurs & liste blanche
spi-online-titel = En ligne actuellement
spi-wl-titel = Liste blanche (qui peut entrer)
spi-wl-erklaerung = Avec la liste blanche activée (Paramètres → « Liste blanche uniquement »), seuls les joueurs listés ici peuvent rejoindre le serveur. Cela protège des inconnus – ajoute-toi aussi.
spi-name-platzhalter = Pseudo Minecraft
spi-hinzufuegen = Ajouter
spi-name-ungueltig = Merci d’entrer un pseudo Minecraft valide (3–16 caractères).
spi-nur-laufend = L’ajout ne fonctionne que pendant que le serveur tourne (il vérifie le pseudo chez Mojang).
spi-wl-leer = La liste blanche est vide.
spi-niemand = Personne en ligne
spi-kreativ = Créatif
spi-ueberleben = Survie
spi-op = Donner OP
spi-op-weg = Retirer OP
spi-kick = Expulser
spi-kick-frage = Expulser { $name } ?
spi-kick-text = Il peut revenir immédiatement.
spi-err-befehl = Échec de la commande
wl-titel = Liste blanche
wl-einschalten = Activer la liste blanche
wl-zu-spielern = Aller aux joueurs
wl-warnung-offen = Attention : ton serveur est accessible depuis internet et la liste blanche est désactivée – quiconque connaît ou trouve l’adresse peut entrer. Avec la liste blanche, seuls les joueurs que tu ajoutes peuvent entrer.
wl-warnung-leer = La liste blanche est activée mais encore vide – pour l’instant personne ne peut entrer, même pas toi. Ajoute ton pseudo Minecraft et ceux de tes amis sous « Joueurs ».
wl-tipp = Astuce : la liste blanche est désactivée – quiconque a l’adresse peut entrer. Elle est recommandée dès que des amis jouent depuis l’extérieur : seuls les joueurs listés ici peuvent alors entrer.

## Console
kon-neues-log = nouveau fichier journal (le serveur a redémarré)
kon-beigetreten = { $name } a rejoint
kon-verlassen = { $name } a quitté la partie
kon-bereit = Serveur prêt (démarré en { $sekunden } s)
kon-fehler = Erreur : { $fehler }
kon-tag = Jour
kon-nacht = Nuit
kon-wetter = Temps clair
kon-wer-online = Qui est en ligne ?
kon-platzhalter = Entrer une commande, p. ex. say Bonjour  ·  time set day  ·  weather clear
kon-senden = Envoyer
kon-log-ordner = Ouvrir le dossier des journaux

## Vue d’ensemble
ueb-laeuft = En marche
ueb-startet = Démarrage …
ueb-stoppt = Arrêt …
ueb-neustart-absturz = Redémarrage après plantage …
ueb-gestoppt = Arrêté
ueb-abgestuerzt = Planté
ueb-extern = Tourne en dehors de Vapafane
ueb-anderer = Un autre serveur tourne
ueb-starten = Démarrer
ueb-stoppen = Arrêter
ueb-neustart-knopf = Redémarrer
ueb-seit = depuis { $dauer }
ueb-neustarts = redémarré automatiquement { $anzahl }×
ueb-hinweis-extern = Le serveur n’a pas été démarré via Vapafane (p. ex. ancien panneau ou terminal). Console et joueurs fonctionnent quand même ; « Arrêter » lui envoie la commande stop.
ueb-hinweis-anderer = Le service d’arrière-plan fait tourner un autre serveur. Arrête-le d’abord.
ueb-hinweis-absturz = Le serveur a planté et n’a pas été redémarré. Regarde le dernier message d’erreur dans la console.
ueb-hinweis-kein-server = Ajoute un serveur avec « ＋ » en haut.
ueb-ev-bereit = Serveur prêt
ueb-ev-gestoppt = Serveur arrêté
ueb-ev-startet = Démarrage du serveur
ueb-ev-neustart = Le serveur redémarre
ueb-ev-stoppt = Le serveur s’arrête
ueb-ev-in-sek = dans { $sekunden } s
ueb-ev-absturz = Le serveur a planté
ueb-ev-absturz-neustart = Le serveur a planté – redémarrage automatique
ueb-notify-absturz = Serveur Minecraft planté
ueb-notify-absturz-neustart = Il redémarre automatiquement.
ueb-ev-backup-start = Sauvegarde lancée
ueb-ev-backup-fertig = Sauvegarde terminée : { $datei } ({ $groesse })
ueb-err-start = Échec du démarrage
ueb-err-stop = Échec de l’arrêt
ueb-err-backup = Échec de la sauvegarde
ueb-eula-titel = Confirmer l’EULA
ueb-eula-text = Aucun serveur ne démarre sans accepter l’EULA de Minecraft.
ueb-eula-akzeptiere = J’accepte l’
ueb-eula-fuer = pour « { $name } ».
ueb-verb-stoppen = arrêter
ueb-verb-neustart = redémarrer
ueb-stop-frage = { $verb } le serveur ?
ueb-stop-extern-text = Le serveur tourne en dehors de Vapafane. Je lui envoie la commande « stop » – il sauvegarde le monde et s’arrête.
ueb-stop-niemand = Personne n’est en ligne actuellement.
ueb-stop-spieler = { $anzahl } joueurs en ligne : { $namen }
ueb-sofort = Tout de suite
ueb-mit-warnung = Dans { $sekunden } s avec avertissement
ueb-karte-spieler = Joueurs en ligne
ueb-karte-tps = TPS (ticks par seconde)
ueb-karte-ram = Mémoire
ueb-karte-ordner = Dossier du serveur
ueb-karte-aussen = De l’extérieur
ueb-karte-adresse = Adresse (réseau local)
ueb-karte-backup = Dernière sauvegarde
ueb-karte-version = Version
ueb-server-aus = serveur éteint
ueb-keine-verbindung = pas de connexion
ueb-niemand = personne
ueb-ms-pro-tick = { $ms } ms par tick
ueb-ruckelt = saccade
ueb-fluessig = fluide
ueb-von-kernen = sur { $anzahl } cœurs
ueb-ram-belegt-limit = utilisé (limite { $limit } Go)
ueb-ram-belegt = utilisé
ueb-ram-limit = limite { $limit } Go
ueb-frei = { $groesse } libres
ueb-vor = il y a { $dauer }
ueb-noch-keins = aucune pour l’instant
ueb-backup-knopf = appuyer sur le bouton « Backup »
ueb-backup-sub = { $groesse } · { $anzahl } conservées
ueb-erreichbar = Accessible
ueb-nur-ipv6 = IPv6 uniquement
ueb-nicht-erreichbar = Inaccessible
ueb-ipv4-und-6 = IPv4 et IPv6
ueb-ueber-ipv4 = via IPv4
ueb-portfreigabe = vérifier l’ouverture de port
ueb-ereignisse = Événements récents
ueb-nichts-passiert = Rien ne s’est encore passé.
ueb-fenster-zu = Fermer la fenêtre n’arrête pas le serveur – il continue en arrière-plan.
ueb-kontakt = Contact & signaler un bug (GitHub)

## Paramètres (serveur)
set-spiel = Jeu (ce serveur)
set-motd = Nom du serveur (MOTD)
set-max-spieler = Joueurs max
set-schwierigkeit = Difficulté
set-wert-peaceful = Paisible
set-wert-easy = Facile
set-wert-normal = Normale
set-wert-hard = Difficile
set-spielmodus = Mode de jeu des nouveaux joueurs
set-wert-survival = Survie
set-wert-creative = Créatif
set-wert-adventure = Aventure
set-wert-spectator = Spectateur
set-sichtweite = Distance d’affichage (chunks)
set-simulationsweite = Distance de simulation (chunks)
set-spawn-schutz = Protection du spawn (blocs)
set-port = Port du serveur
set-whitelist = Liste blanche uniquement
set-whitelist-hilfe = Recommandé pour la sécurité : seuls les joueurs de la liste blanche (page « Joueurs ») peuvent entrer. Sans elle, quiconque connaît ton adresse peut rejoindre.
set-leistung = Performance (ce serveur)
set-ram = Mémoire max pour le serveur (Go)
set-ram-hinweis = Ce PC a { $gb } Go. Laisse environ 3–4 Go pour le système.
set-speichern = Enregistrer les paramètres
set-gespeichert = Enregistré.
set-gespeichert-neustart = Enregistré. Prendra effet au prochain redémarrage.
set-err-speichern = Échec de l’enregistrement
set-automatik = Automatisation
set-autostart = Démarrer automatiquement le serveur actif au démarrage du PC
set-auto-testmodus = Désactivé en mode test.
set-auto-kein-systemd = Indisponible sur ce système (pas de systemd).
set-auto-windows = Démarre au démarrage du PC, même sans connexion (Windows demande les droits administrateur à l’activation).
set-auto-linger-fehlt = Attention : il ne démarre qu’à ta connexion. Clique ici pour autoriser le démarrage sans connexion.
set-auto-aktiv = Démarre même sans connexion et redémarre seul après un plantage.
set-linger-frage = Autoriser le démarrage sans connexion ?
set-linger-text = Pour cela, systemd doit pouvoir lancer tes services utilisateur dès le démarrage (« loginctl enable-linger »). Ensuite le serveur tourne même si personne n’est connecté.
set-linger-erlauben = Autoriser
set-linger-abgelehnt = Non autorisé
set-fenster-autostart = Ouvrir cette fenêtre à la connexion
set-backup-um = Sauvegarde quotidienne à
set-backup-uhr =  (serveur complet en ZIP)
set-backup-sub = Effectuée par le service d’arrière-plan. Rattrapée si le PC était éteint à cette heure.
set-uhrzeit = Heure de la sauvegarde quotidienne
set-reach = Vérifier l’accessibilité depuis l’extérieur
set-reach-sub = En passant à la zone serveur. Interroge pour cela un service de vérification sur internet.
set-notify = Notifications
set-notify-sub = Quand quelqu’un arrive ou part et quand le serveur plante (tant que la fenêtre est ouverte). Sans son.
set-ordner = Dossiers
set-ordner-server = Dossier du serveur
set-an = activé
set-aus = désactivé
set-panel-titel = Ancien panneau de contrôle encore configuré.
set-panel-status = Service « minecraft.service » : { $dienst }, au démarrage { $start } ; programme de sauvegarde quotidienne { $backup }. Tant qu’il tourne, Vapafane ne peut pas démarrer le serveur lui-même (même port), et les sauvegardes seraient faites en double.
set-panel-knopf = Désactiver l’ancien panneau …
set-panel-frage = Désactiver l’ancien panneau ?
set-panel-text = J’arrête le service « minecraft.service » (le serveur sauvegarde proprement et s’arrête) et je le désactive, lui et sa sauvegarde quotidienne, au démarrage du PC.

    L’ancien panneau lui-même et ses fichiers restent – on peut le réactiver à tout moment. Ensuite tu démarres le serveur ici avec « Démarrer ».
set-panel-abschalten = Désactiver
set-panel-aus = Ancien panneau désactivé
set-panel-err = Échec de la désactivation

## CurseForge
cf-titel = CurseForge (optionnel)
cf-experimentell = Expérimental
cf-brauche = En ai-je besoin ?
cf-erklaerung = Non – la plupart des mods sont sur Modrinth, qui fonctionne sans configuration. CurseForge trouve en plus des mods qui n’existent que là-bas et nécessite pour cela une clé API personnelle (une sorte de code d’accès).
cf-anleitung-titel = Comment obtenir une clé (gratuit)
cf-schritt-1 = Clique sur « Demander une clé » et remplis le formulaire (p. ex. but : « Mods pour mon propre serveur »).
cf-schritt-2 = Attends la réponse par e-mail – CurseForge vérifie à la main, cela peut prendre quelques jours.
cf-schritt-3 = Colle la clé ci-dessous, « Tester la clé » puis « Enregistrer ».
cf-persoenlich = La clé est personnelle
cf-persoenlich-text = – ne pas la partager ni la publier. Elle reste uniquement sur ce PC. S’appliquent les
cf-bedingungen = conditions CurseForge pour programmes tiers
cf-experimentell-text = Expérimental : CurseForge décide s’il accorde des clés aux outils privés.
cf-beantragen = Demander une clé ↗
cf-konsole = Console CurseForge ↗
cf-mehr = En savoir plus sur l’API CurseForge ↗
cf-platzhalter = Coller la clé API ici
cf-testen = Tester la clé
cf-teste = Test …
cf-funktioniert = La clé fonctionne.
cf-schluessel-da = Clé enregistrée : { $maskiert } – CurseForge est inclus dans les recherches.
cf-kein-schluessel = Aucune clé enregistrée – recherche uniquement sur Modrinth.
cf-gespeichert = Clé CurseForge enregistrée
cf-entfernen-frage = Supprimer la clé CurseForge ?
cf-entfernen-text = Ensuite Vapafane ne cherche de nouveau que sur Modrinth.

## Ajouter un serveur
gem-anderer-ordner = Autre dossier …
srv-typ-fabric = Fabric (mods)
srv-typ-paper = Paper (plugins)
srv-typ-neoforge = NeoForge (mods)
srv-typ-vanilla = Vanilla (sans mods)
srv-name-label = Nom dans la liste des serveurs
srv-typ-label = Type de serveur
srv-version-label = Version de Minecraft
srv-laedt = chargement …
srv-keine-verbindung = Pas de connexion : { $fehler }
srv-ordner-waehlen = Choisir un dossier pour le serveur
srv-server-ordner-waehlen = Choisir le dossier du serveur
srv-backup-waehlen = Choisir une sauvegarde du serveur (ZIP ou tar)
srv-ordner-nicht-leer = Le dossier choisi n’est pas vide – le serveur ira dans un nouveau sous-dossier.
srv-ordner-ungueltig = Ce dossier ne peut pas être utilisé.
srv-eula-zusatz = (aucun serveur ne démarre sans)
srv-hinweis = Remarque
srv-ev-hinzu = Serveur ajouté : { $name }
srv-hinzu-titel = Ajouter un serveur
srv-hinzu-text = « { $name } » est dans la liste. Arrête le serveur en cours pour basculer.
srv-err-hinzu = Échec de l’ajout
srv-kein-server = Aucun serveur trouvé
srv-import-titel = Importer un serveur
srv-err-import = Échec de l’import
srv-neu-titel = Créer un nouveau serveur
srv-neu-name = Mon serveur
srv-err-erstellen = Échec de la création
srv-entfernen-frage = Retirer « { $name } » de la liste ?
srv-entfernen-text = Les fichiers dans { $ordner } sont conservés, seule l’entrée disparaît.
srv-err-entfernen = Échec du retrait

## Mods/Plugins
mods-erklaerung = Les mods désactivés restent dans le dossier mais ne sont pas chargés. Les changements prennent effet au prochain démarrage du serveur.
mods-ev-an = { $name } activé
mods-ev-aus = { $name } désactivé
mods-neustart-noetig = Le serveur tourne – le changement prendra effet après le prochain redémarrage.
mods-nur-client = client uniquement
mods-aus-grund = Désactivé : { $grund }
mods-an-tipp = Activé – cliquer pour désactiver
mods-aus-tipp = Désactivé – cliquer pour activer
mods-anzahl = { $an } sur { $gesamt } { $wort } activés
mods-datapacks-anzahl = { $anzahl } datapacks
mods-leer = Aucun { $wort } dans le dossier « { $ordner } ».
mods-datapacks-leer = Aucun datapack dans le monde.
mods-datapacks-titel = Datapacks dans le monde
mods-hinzufuegen = Ajouter des { $wort } …
mods-hinzufuegen-titel = Ajouter des { $wort }
mods-ordner-oeffnen = Ouvrir le dossier des mods
mods-aufraeumen = Nettoyer …
mods-aufraeumen-titel = Nettoyage
mods-aufraeumen-tipp = Supprimer les restes d’anciennes versions (le serveur doit être arrêté)
mods-alles-sauber = Tout est propre – il n’y a pas de restes d’anciennes versions.
mods-aufraeumen-text-1 = Ces restes d’anciennes versions seraient supprimés (au total
mods-aufraeumen-text-2 = Le monde, les paramètres et les mods désactivés restent. Ce dont le serveur a encore besoin, il le retélécharge au prochain démarrage.
mods-erst-stoppen = Merci d’arrêter d’abord le serveur pour supprimer.
mods-loeschen-knopf = Supprimer { $groesse }
mods-ev-aufgeraeumt = Nettoyé : { $groesse } libérés
mods-aufgeraeumt-titel = Nettoyé
mods-aufgeraeumt = { $anzahl } entrées supprimées, { $groesse } libérés.
mods-nicht-loeschbar = Impossible à supprimer :
mods-err-aufraeumen = Échec du nettoyage

## Catalogue (ajouter des mods)
kat-suche = Rechercher sur Modrinth … (vide = les plus populaires)
kat-dp-option = Datapacks (pour le monde)
kat-treffer = { $anzahl } { $wort } correspondants pour Minecraft { $mc }
kat-beliebtheit = triés par popularité
kat-sucht = Recherche …
kat-nichts = Rien trouvé.
kat-mehr = Charger plus
kat-cf-tipp = Astuce : avec une clé CurseForge (Paramètres), tu trouves en plus des mods qui n’existent que là-bas.
kat-nur-cf = Uniquement sur CurseForge
kat-cf-erklaerung = Ceux-ci ne sont pas sur Modrinth. Certains auteurs n’autorisent pas le téléchargement par d’autres programmes – tu télécharges alors le fichier dans le navigateur et le mets toi-même dans le dossier.
kat-beides = Serveur + client
kat-beides-tipp = Les joueurs en ont aussi besoin eux-mêmes – p. ex. via « Exporter les mods pour les amis (.mrpack) »
kat-nur-server = Serveur uniquement
kat-meta = { $downloads } téléchargements · par { $autor }
kat-laedt = Chargement …
kat-installiert = Installé
kat-installieren = Installer
kat-ev-installiert = Installé : { $name }
kat-err-install = L’installation n’a pas fonctionné : { $fehler }
kat-ist-installiert = est installé (dans « { $ordner } »)
kat-abhaengigkeiten = Ajouté comme dépendance : { $namen }.
kat-schon-da = Déjà présents : { $namen }.
kat-fehlende = Attention : cette dépendance n’existe pas pour cette version : { $namen } – le serveur risque de ne pas démarrer ainsi.
kat-client-noetig = Les joueurs ont aussi besoin de ce mod – le plus simple via « Exporter les mods pour les amis (.mrpack) » sous « Version & mises à jour ».
kat-manuell-1 = L’auteur n’autorise pas le téléchargement par d’autres programmes. Télécharge le fichier
kat-manuell-2 = dans le navigateur et mets-le dans le dossier « { $ordner } ».
kat-browser = Télécharger dans le navigateur ↗
kat-dp-ordner = Ouvrir le dossier des datapacks
kat-neustart = Le serveur tourne – cela prend effet après un redémarrage.
kat-jetzt-neustarten = Redémarrer maintenant

## Version & mises à jour
upd-st-aktuell = à jour
upd-st-fehlt = manquant
upd-st-unbekannt = inconnu
upd-st-neu = nouveau (nécessaire)
upd-st-manuell = mise à jour uniquement dans le navigateur (CurseForge)
upd-st-reenable = compatible à nouveau – sera activé
upd-st-doppelt = doublon (déjà présent comme mod)
upd-alles-aktuell = Tout est à jour.
upd-zusammenfassung = { $updates } mises à jour, { $aktuell } déjà à jour, { $fehlend } sans version compatible, { $unbekannt } inconnus
upd-neu-gebraucht = { $anzahl } nouvellement nécessaires
upd-passen-wieder = { $anzahl } compatibles à nouveau
upd-doppelt = { $anzahl } doublons (seront mis de côté)
upd-server-stand = Nouveau build serveur pour Minecraft { $mc } : { $label }.
upd-beta-hinweis = Remarque : il n’existe pour l’instant qu’une version test/bêta.
upd-wechsel = Serveur : Minecraft { $von } → { $nach } ({ $label }).
upd-welt-warnung = Attention : un monde qui a tourné une fois avec la nouvelle version ne peut pas être rétrogradé. Le seul retour possible passe par la sauvegarde faite automatiquement.
upd-java-da = Java { $version } nécessaire – présent ({ $pfad }).
upd-java-fehlt = Java { $version } nécessaire mais non installé. { $hinweis }
upd-manuell-hinweis = Pour les mods « mise à jour uniquement dans le navigateur », l’auteur n’autorise pas le téléchargement automatique – télécharge le nouveau fichier sur CurseForge et échange-le toi-même.
upd-platz = Pas assez d’espace disque pour la sauvegarde ({ $frei } libres, serveur { $server }).
upd-nur-mods = { $version } (mettre à jour uniquement mods/plugins)
upd-keine-neuere = Il n’y a pas de version stable de Minecraft plus récente.
upd-keine-verbindung = Pas de connexion
upd-err-pruefen = Échec de la vérification
upd-rollback-frage = Revenir à l’état d’avant la dernière mise à jour ?
upd-rollback-text = Le serveur est arrêté et restauré depuis la sauvegarde faite lors de la mise à jour. L’état actuel reste à côté dans un dossier « …_vor_Wiederherstellung_<date> ».
upd-zuruecksetzen = Réinitialiser
upd-ev-rollback = État d’avant la mise à jour restauré
upd-rollback-fertig-titel = Réinitialisé
upd-rollback-fertig = Le serveur est revenu à l’état du { $label }. Tu peux le démarrer maintenant.
upd-rollback-daneben = L’état d’après la mise à jour se trouve dans :
upd-err-rollback = Échec de la réinitialisation
upd-rollback-knopf = Revenir à l’état d’avant la dernière mise à jour
upd-was-mods = Mettre à jour les mods
upd-ablauf = Déroulement : arrêter le serveur (avec avertissement si quelqu’un est en ligne) → sauvegarde complète → télécharger et vérifier { $anzahl } fichiers → échanger → démarrer le serveur et vérifier.
upd-werden-deaktiviert = { $anzahl } mod(s)/datapack(s) sans version compatible seront désactivés.
upd-probelauf-tipp = Astuce : avec « Essai », tu peux tester cela sans risque sur une copie.
upd-start-frage = Lancer la mise à jour ? ({ $was })
upd-starten = Lancer la mise à jour
upd-ev-gestartet = Mise à jour lancée ({ $was })
upd-err-update = Échec de la mise à jour
upd-ev-fertig = Mise à jour terminée ({ $was })
upd-fertig-titel = Mise à jour terminée
upd-fertig-1 = Le serveur tourne à nouveau ({ $was }).
upd-fertig-backup = Sauvegarde de l’ancien état :
upd-fertig-aufgeraeumt = Nettoyé : { $anzahl } anciens fichiers/dossiers supprimés ({ $groesse }).
upd-fertig-2 = Les mods sans version compatible sont désactivés et visibles sous « Mods/Plugins ».
upd-fehlgeschlagen-titel = La mise à jour n’a pas fonctionné
upd-fehlgeschlagen-frage = Regarde le message d’erreur dans la console. Dois-je restaurer l’état d’avant la mise à jour depuis la sauvegarde ?
upd-trial = Essai
upd-trial-starten = Lancer l’essai
upd-trial-frage = Lancer un essai ?
upd-trial-text = Je copie le serveur dans le dossier temporaire, je mets à jour la copie, je la démarre sur un autre port et je vérifie qu’elle démarre complètement. Ensuite la copie est arrêtée et supprimée.

    Ton vrai serveur reste inchangé et peut continuer à tourner. Avec de grands mondes, la copie prend quelques minutes.
upd-trial-tipp = Met à jour une copie dans le dossier temporaire et la démarre sur un autre port
upd-trial-err = Échec de l’essai
upd-trial-ok-titel = Essai réussi
upd-trial-ok = La copie mise à jour a fini de démarrer en { $sekunden } secondes (port { $port }).
upd-trial-timeout = La copie n’avait pas fini de démarrer après 15 minutes.
upd-trial-exit = La copie s’est arrêtée au démarrage (code de sortie { $code }).
upd-trial-meldungen = Messages notables :
upd-trial-zeilen = Dernières lignes :
upd-trial-geloescht = La copie a été supprimée.
upd-trial-nicht-geloescht = La copie n’a pas pu être entièrement supprimée : { $ordner }
upd-ev-trial-ok = Essai réussi
upd-ev-trial-fehler = Échec de l’essai
upd-jetzt = Actuellement :
upd-erklaerung = Vérifie si tous les mods existent pour la version choisie. Une sauvegarde est faite automatiquement avant chaque mise à jour.
upd-ziel = Cible :
upd-versionen = Charger les versions
upd-pruefen = Vérifier
upd-spalte-mod = Mod / datapack
upd-spalte-neu = Nouveau
upd-deaktivieren-fehlende = Désactiver les mods/datapacks sans version compatible (sinon le serveur ne démarrera probablement pas)
upd-deaktivieren-unbekannte = Désactiver les mods/datapacks inconnus (pas sur Modrinth)
upd-nicht-unterstuetzt = L’assistant de mise à jour existe pour Fabric, Paper, NeoForge et Vanilla.
upd-export-knopf = Exporter les mods pour les amis (.mrpack)
upd-export-tipp = Uniquement pour les serveurs Fabric et NeoForge (mods dont les amis ont aussi besoin)
upd-export-titel = Modpack enregistré
upd-export-mods = { $anzahl } mods de Modrinth
upd-export-eingepackt = { $anzahl } inclus directement
upd-export-hinweis = Tes amis importent le fichier dans l’application Modrinth (« Ajouter un fichier »). Les mods côté serveur n’y sont pas installés.
upd-err-export = Échec de l’export

## Client
cli-titel = Utiliser une invitation
cli-code-erklaerung-1 = Colle le code (commence par
cli-code-erklaerung-2 = ) ou ouvre le fichier
cli-code-erklaerung-3 =  – Vapafane configure Minecraft avec tous les mods adaptés au serveur.
cli-code-platzhalter = Coller le code d’invitation ici (VPF1-…)
cli-pruefen = Vérifier l’invitation
cli-datei-oeffnen = Ouvrir un fichier d’invitation …
cli-pruefe-einladung = Vérification de l’invitation …
cli-launcher-da = Launcher Minecraft trouvé
cli-kein-launcher = Pas de launcher Minecraft
cli-modrinth-da = Application Modrinth trouvée
cli-meine-server = Mes serveurs
cli-leer = Aucun serveur configuré – utilise une invitation ci-dessus.
cli-kein-konto = Vapafane ne lance pas Minecraft lui-même et ne touche jamais à ton compte Microsoft – « Jouer » ouvre le launcher officiel ou l’application Modrinth.
cli-nicht-erreichbar = Serveur injoignable
cli-online = En ligne · { $version } · { $spieler }/{ $max } joueurs
cli-version-passt-nicht = En ligne, mais version { $version } – ne correspond pas
cli-aktualisiert = Le serveur a été mis à jour ({ $version }) – demande un nouveau code d’invitation à l’admin
cli-gerade-nicht = injoignable pour le moment
cli-modrinth-fehlt = Application Modrinth introuvable
cli-launcher-fehlt = Launcher Minecraft introuvable
cli-download-seite = Vers la page de téléchargement
cli-ev-eingerichtet = Client configuré : { $name }
cli-gefunden = trouvé
cli-nicht-gefunden = introuvable
cli-keine-mods = Aucun mod nécessaire – seulement Minecraft { $version }.
cli-optional = optionnel
cli-nur-browser = téléchargeable uniquement dans le navigateur (lien après l’installation)
cli-nur-datei = manquant – uniquement dans le fichier d’invitation
cli-launcher-fehlt-text = Le launcher Minecraft officiel est introuvable. Tu peux quand même configurer ; le profil sera créé dès que le launcher est installé et que tu appuies ici sur « Actualiser ».
cli-launcher-laden = Télécharger le launcher
cli-nie-gestartet = Le launcher n’a jamais été lancé. Lance-le une fois et ferme-le, puis configure ici.
cli-ordner-waehlen = Choisir un dossier pour ce Minecraft
cli-mods-zusammen = { $anzahl } mods, { $groesse } au total
cli-andere-version = Le serveur annonce une autre version ({ $version }). Il a probablement été mis à jour – demande un nouveau code d’invitation à l’admin.
cli-spalte-version = Version
cli-spalte-groesse = Taille
cli-einrichten-fuer = Configurer pour :
cli-in-modrinth = Dans l’application Modrinth :
cli-instanz-update = Mettre à jour l’instance existante « { $name } » (recommandé)
cli-instanz-update-sub = Tout reste dans la même instance : mondes, paramètres, shaders, packs de ressources et mods que tu as ajoutés toi-même (p. ex. Iris). Il te suffit de cliquer sur « Réinstaller le modpack » dans l’application Modrinth – Vapafane te montre comment.
cli-instanz-neu = Créer une nouvelle instance
cli-instanz-neu-sub = Fonctionne automatiquement via l’import. Tu peux reprendre ci-dessous paramètres, shaders et packs de ressources – pas les mods ajoutés toi-même. L’ancienne instance reste jusqu’à ce que tu la supprimes.
cli-update-bleibt = Lors de la mise à jour, tout reste dans l’instance : mondes, options, shaders, packs de ressources et ta liste de serveurs.
cli-import-bestaetigen = L’application Modrinth s’ouvre – confirme simplement l’import.
cli-ordner-label = Dossier :
cli-launcher-schliessen = Ferme le launcher Minecraft avant la configuration pour qu’il n’écrase pas le changement.
cli-einrichten = Configurer
cli-uebernehmen-von = Reprendre les paramètres de :
cli-nichts = Ne rien reprendre
cli-quelle-waehlen = Choisir le dossier Minecraft d’où reprendre
cli-nichts-uebernehmen = Rien à reprendre
cli-teil-options = Options & raccourcis clavier
cli-teil-config = Paramètres des mods
cli-teil-shaders = Shaders
cli-teil-resourcepacks = Packs de ressources
cli-teil-servers = Liste des serveurs
cli-teil-worlds = Mondes
cli-welten-warnung = Les mondes d’une version de Minecraft plus récente ne s’ouvrent pas dans une plus ancienne. Les grands mondes ralentissent la configuration.
cli-nur-fehlendes = Ce qui est déjà dans le dossier reste inchangé – seul ce qui manque est repris.
cli-uebernehmen-erklaerung = Par exemple depuis ton instance précédente : options, raccourcis, paramètres des mods, shaders, packs de ressources et liste des serveurs. Les mods viennent toujours du serveur, adaptés. La source est seulement lue, jamais modifiée.
cli-paket-update-1 = Le nouveau pack Modrinth est prêt. Voici comment mettre à jour ton instance
cli-paket-update-2 = mondes, options, shaders, packs de ressources et ta liste de serveurs sont tous conservés :
cli-schritt-instanz = Ouvre l’application Modrinth et clique sur l’instance
cli-schritt-anklicken = à gauche.
cli-schritt-zahnrad = Clique en haut sur l’engrenage « Paramètres de l’instance » et choisis l’onglet « Installation ».
cli-neu-installieren = Réinstaller le modpack
cli-schritt-datei = clique, confirme et choisis ce fichier :
cli-modrinth-oeffnen = Ouvrir l’application Modrinth
cli-paket-ordner = Ouvrir le dossier avec le pack
cli-update-sicher = L’application Modrinth sauvegarde d’abord l’instance elle-même et n’échange que les mods du serveur – ce que tu as ajouté toi-même (shaders, packs de ressources, mods perso) reste.
cli-paket-offen = Le pack Modrinth est prêt et l’application Modrinth a été ouverte avec. Confirme l’import là-bas – l’instance apparaîtra sous
cli-paket-fertig = Le pack Modrinth est prêt :
cli-paket-von-hand = L’application Modrinth n’a pas pu être ouverte automatiquement. À la main : clique sur « + » (créer une instance) dans l’application Modrinth, puis « Importer depuis un fichier » et choisis ce fichier.
cli-eingerichtet = Configuré : { $anzahl } mods téléchargés.
cli-profil = Le launcher Minecraft a maintenant le profil « { $name } ».
cli-uebernommen-paket = Les paramètres de « { $quelle } » sont dans le pack – l’application Modrinth les applique à l’import.
cli-uebernommen = Repris de « { $quelle } » : { $anzahl } fichiers.
cli-uebernommen-da = ({ $anzahl } étaient déjà là et sont restés inchangés)
cli-uebernommen-fehler = { $anzahl } fichiers n’ont pas pu être lus et manquent donc – Minecraft y est-il encore ouvert ? ({ $dateien })
cli-selbst-laden = À télécharger toi-même et à mettre dans le dossier « mods » :
cli-spielen = Jouer
cli-spielen-modrinth = Jouer (ouvrir l’application Modrinth)
cli-spielen-launcher = Jouer (ouvrir le launcher)
cli-prueft = vérification …
cli-unvollstaendig = La configuration n’a pas abouti – utilise à nouveau l’invitation ci-dessus (ou supprime).
cli-mods-kurz = { $anzahl } mods
cli-eingerichtet-am = { $ziel } · configuré le { $datum }
cli-aktualisieren = Actualiser
cli-aktualisieren-tipp = Reconfigurer avec l’invitation enregistrée – ou expliquer quoi faire après une mise à jour du serveur
cli-entfernen-frage = Supprimer « { $name } » ?
cli-entfernen-modrinth = Vapafane oublie ce client et supprime son pack. L’instance dans l’application Modrinth reste – supprime-la là-bas toi-même.
cli-entfernen-launcher = Le dossier avec les mods, paramètres et mondes solo de ce profil est supprimé, et le profil disparaît du launcher Minecraft.
cli-upd-titel = Le serveur a été mis à jour
cli-upd-text = « { $name } » tourne maintenant sous Minecraft { $version }. La version et les mods correspondants sont dans l’invitation – il te faut donc un nouveau code d’invitation de l’admin.
cli-upd-code = Colle-le en haut sous « Utiliser une invitation » et clique sur « Vérifier l’invitation ».
cli-upd-modrinth = Vapafane crée alors un nouveau pack et te montre comment mettre à jour ton instance – tes paramètres, mondes et shaders restent.
cli-upd-launcher = Vapafane échange alors les mods – tes paramètres et mondes restent.
cli-gespeicherte-einladung = Invitation enregistrée de « { $name } » chargée. « Configurer » remet tout en ordre – par exemple si des mods manquent ou si le profil a disparu. Si l’admin a mis à jour le serveur, il te faut plutôt un nouveau code d’invitation.
ext-knopf = Shaders & extras
ext-knopf-so-gehts = Shaders & extras – mode d’emploi

## Shaders & extras
ext-titel = Ajouter shaders & extras
ext-skizze-1-alt = Schéma de l’instance dans l’application Modrinth
ext-skizze-2-alt = Schéma de la recherche dans l’application Modrinth
ext-skizze-leer = Aucun contenu supplémentaire installé
ext-skizze-hochladen = Téléverser des fichiers
ext-durchsuchen = Parcourir le contenu
ext-skizze-shader = Un shader, p. ex. Complementary
ext-skizze-beschreibung = Description · téléchargements
ext-keine-shader = Les shaders nécessitent le mod Iris – qui ne tourne que sur les serveurs Fabric ou NeoForge. Ce serveur n’a pas de mod loader, donc les packs de ressources fonctionnent ici, mais pas les shaders.
ext-iris-modrinth = Pour les shaders, Minecraft a besoin du mod Iris : à l’étape 4, choisis d’abord « Mods » et installe « Iris Shaders » (Sodium vient automatiquement), puis choisis un shader sous « Shaders ».
ext-iris-launcher-1 = Pour les shaders, Minecraft a besoin du mod Iris (avec Sodium) : télécharge les deux pour la version Minecraft du serveur depuis
ext-iris-launcher-2 = et mets-les dans le dossier « mods ».
ext-mr-einleitung-1 = Dans ton instance
ext-mr-einleitung-2 = tu peux ajouter à tout moment tes propres shaders, packs de ressources et mods. Ils sont conservés lors des mises à jour du serveur – l’application Modrinth n’échange que les mods du serveur.
ext-schritt-content = Choisis l’onglet « Content » (contenu) en haut.
ext-schritt-durchsuchen = Clique sur le bouton vert « Parcourir le contenu ».
ext-schritt-art = Choisis le type en haut : « Shaders » pour les shaders, « Resource Packs » pour les packs de ressources ou « Mods ».
ext-schritt-installieren = Clique sur « Installer » à côté de l’entrée voulue – terminé.
ext-im-spiel = Dans le jeu : Options → Paramètres graphiques → Packs de shaders → choisir le shader.
ext-im-spiel-rp = Dans le jeu : Options → Packs de ressources → choisir le pack.
ext-mods-update-hinweis = Tes propres mods doivent être mis à jour toi-même dans l’application après une mise à jour de Minecraft ; les shaders et packs de ressources en général non.
ext-launcher-einleitung = Le dossier de jeu de ce serveur n’appartient qu’à lui : ce que tu y ajoutes est conservé lors des mises à jour du serveur.
ext-schritt-laden-shader = Télécharger un shader, p. ex. sur
ext-schritt-laden-rp = Télécharger un pack de ressources, p. ex. sur
ext-nicht-entpacken = ne pas décompresser le fichier .zip.
ext-schritt-shaderpacks = Clique en bas sur « Ouvrir le dossier » et mets le fichier dans le dossier « shaderpacks ». Les packs de ressources vont dans « resourcepacks ».
ext-schritt-resourcepacks = Clique en bas sur « Ouvrir le dossier » et mets le fichier dans le dossier « resourcepacks ».
ext-eigene-mods = Tu peux mettre tes propres mods dans le dossier « mods » – mais après une mise à jour de Minecraft, tu dois les maintenir compatibles toi-même.
err-antwort-unlesbar = Réponse de « { $befehl } » illisible : { $fehler }

## Rapport d’accessibilité
rch-w-erreichbar = accessible
rch-w-nicht = inaccessible
rch-w-nicht-dslite = inaccessible (DS-Lite)
rch-w-nicht-geprueft = non vérifié
rch-w-nicht-vorhanden = non disponible
rch-unmoeglich-titel = Vérification impossible pour le moment
rch-unmoeglich-text = Le service de vérification sur internet ne répond pas. Réessaie plus tard.
rch-gruen-beide = Accessible via IPv4 et IPv6
rch-gruen-v4 = Accessible via IPv4
rch-gruen-text = Tous les amis peuvent entrer.
rch-gelb-dslite-titel = DS-Lite : accessible uniquement via IPv6
rch-gelb-dslite-text = Ton fournisseur partage l’adresse IPv4. Les amis avec IPv6 peuvent quand même entrer – c’est la plupart.
rch-gelb-titel = Accessible uniquement via IPv6
rch-gelb-text = Personne n’entre via IPv4 : soit l’ouverture de port IPv4 (port { $port }) manque, soit ton fournisseur partage l’adresse IPv4 (DS-Lite). Les amis avec IPv6 peuvent quand même entrer.
rch-rot-titel = Inaccessible depuis l’extérieur
rch-rot-text = En général, il manque l’ouverture de port dans le routeur, ou le pare-feu du PC bloque le port { $port }.
rch-rot-dslite-v6 = Ton fournisseur partage l’adresse IPv4 (DS-Lite), donc seul IPv6 fonctionne. Ouvre le port { $port } pour IPv6 dans le routeur et vérifie le pare-feu du PC.
rch-rot-dslite = Ton fournisseur partage l’adresse IPv4 (DS-Lite) et ton PC n’a pas d’adresse IPv6. Personne de l’extérieur ne peut ainsi joindre le serveur.
rch-aus-titel = Le serveur ne tourne pas
rch-aus-text = La vérification se fera dès que le serveur tourne.
rch-port-belegt-titel = Le port { $port } est déjà utilisé
rch-port-belegt-text = Un autre programme – probablement un autre serveur Minecraft – utilise ce port. Ce serveur ne peut pas démarrer ainsi. Règle un port libre sous Paramètres → « Port du serveur » (p. ex. { $frei })
rch-hotspot-tipp = Le serveur n’est accessible que via IPv6, mais ton internet n’a pas d’IPv6. Astuce : connecte ton PC via le partage de connexion de ton téléphone – le réseau mobile a généralement IPv6.

## Erreurs du programme (général)
err-dienst-aus = Le service d’arrière-plan ne tourne pas
err-kein-systemd = systemd n’est pas disponible ici (ou mode test).
err-kein-server = Aucun serveur sélectionné.
err-schon-beschaeftigt = Quelque chose tourne déjà : { $was }
err-neustart-extern = Ce serveur tourne en dehors de Vapafane – le redémarrage ne fonctionne que via le programme qui l’a lancé.
err-stop-knopf = Merci d’utiliser le bouton Arrêter dans la vue d’ensemble (il sauvegarde proprement).
err-start-extern = Le serveur tourne en dehors de Vapafane. Arrête-le d’abord là-bas.
err-start-anderer = Un autre serveur tourne actuellement. Arrête-le d’abord.
err-stop-unmoeglich = Le serveur n’a pas pu être arrêté.
err-pfad = Chemin invalide
err-rcon-antwort = Réponse RCON invalide
err-adresse = Adresse invalide
cln-alte-neoforge = ancienne version de NeoForge
cln-alte-bibliothek = ancienne version d’une bibliothèque
cln-alte-mods = anciennes versions de mods (mises de côté par l’ancien panneau/launcher)
cln-cache = cache des mods (recréé au prochain démarrage)
cli-datei-titel = Ouvrir un fichier d’invitation
srv-rcon-an = RCON activé (Vapafane en a besoin pour les commandes).
err-link-verboten = Cette adresse ne peut pas être ouverte.
err-pruefsumme = La somme de contrôle ne correspond pas : { $datei }
err-kein-fabric = Aucun loader Fabric trouvé pour Minecraft { $mc }
cf-err-leer = Colle d’abord une clé.
cf-err-id = ID CurseForge invalide
cf-err-format = Cela ne ressemble pas à une clé API (colle-la sans espaces).
cf-err-antwort = Réponse invalide de CurseForge : { $fehler }
cf-err-abgelehnt = CurseForge rejette la clé API (invalide ou pas encore activée).
cf-err-fehlt = Aucune clé CurseForge enregistrée.
kat-err-typ = « Ajouter des mods » n’existe pas pour ce type de serveur.
kat-err-keine-version = « { $name } » n’existe pas pour Minecraft { $mc } ({ $loader }).
kat-err-cf-version = « { $name } » n’existe pas sur CurseForge pour Minecraft { $mc }.
fort-suche = Recherche de { $name } …
fort-suche-cf = Recherche de { $name } sur CurseForge …
inv-err-neuer = Ce code d’invitation est pour une version plus récente de Vapafane. Mets Vapafane à jour.
inv-err-unvollstaendig = Le code est incomplet ou endommagé – copie-le en entier.
inv-err-kaputt = Le code est endommagé – copie-le à nouveau.
inv-err-version = Cette invitation est pour une autre version de Vapafane.
inv-err-angaben = L’invitation contient des données invalides.
inv-err-mod = L’invitation contient une entrée de mod invalide.
inv-err-typ = Les invitations existent pour les serveurs Fabric, NeoForge, Paper et Vanilla.
inv-err-datei = Ce n’est pas un fichier d’invitation Vapafane valide.
inv-err-datei-kaputt = L’invitation dans le fichier est endommagée.
inv-err-oeffentlich = L’adresse publique n’a pas pu être déterminée.
flv-err-kein-server = Il n’y a pas de serveur pour Minecraft { $mc }.
flv-err-kein-paper = Pas de Paper pour { $mc }.
flv-err-kein-neoforge = Il n’y a pas de NeoForge pour Minecraft { $mc }.
fort-neoforge = L’installateur NeoForge configure le serveur (télécharge les bibliothèques) …
fort-suche-server = Recherche de { $was } pour Minecraft { $mc } …
fort-pruefe-java = Vérification de Java …
fort-frage-cf = Interrogation de CurseForge pour les mods restants …
fort-pruefe-deps = Vérification des dépendances …
fort-pruefe-dp = Vérification des datapacks …
fort-pruefe-java-platz = Vérification de Java et de l’espace disque …
upd-err-dateiname = Nom de fichier invalide de Modrinth : { $name }
upd-cf-fehler = CurseForge n’a pas pu être vérifié : { $fehler }
upd-abhaengigkeit = (dépendance)
upd-grund-fehlt = n’existe pas pour Minecraft { $mc }
cli-err-port = Port invalide dans l’adresse
cli-err-antwort = Réponse invalide du serveur
cli-err-profil-kaputt = { $datei } est endommagé – lance le launcher une fois.
cli-err-nicht-leer = Le dossier { $ordner } n’est pas vide et n’appartient pas à Vapafane.
cli-err-datei-kaputt = Le fichier { $datei } de l’invitation est endommagé.
cli-err-fabric-profil = Profil Fabric invalide.
cli-err-neoforge-java = NeoForge a besoin de Java { $version }. Lance le launcher Minecraft une fois (il apporte Java) ou installe Java.
cli-hinweis-launcher-starten = Lance le launcher Minecraft une fois et ferme-le, puis configure à nouveau.
cli-hinweis-kein-launcher = Le launcher Minecraft officiel est introuvable. Installe-le, lance-le une fois, puis appuie ici sur « Actualiser ».
cli-hinweis-nie-gestartet = Le launcher n’a jamais été lancé. Lance-le une fois et ferme-le, puis « Actualiser ».
cli-paket-summary = Configuré par Vapafane pour le serveur { $name } ({ $adresse })
fort-pruefe-mods = Vérification des mods …
fort-lade = Téléchargement de { $name } …
fort-launcher = Configuration du launcher …
fort-neoforge-install = Installation de NeoForge { $version } (peut prendre quelques minutes) …
fort-uebernehme = Reprise des paramètres …
fort-trage-ein = Ajout du serveur …
fort-sammle = Collecte des paramètres …
fort-packe = Création du pack Modrinth …
fort-fertig = Terminé.
task-update = Mise à jour
task-backup = Sauvegarde (serveur complet en ZIP)
task-restore = Restaurer la sauvegarde
task-client = Configurer le client
task-rollback = Retour à l’état d’avant la mise à jour
task-pruefe-mods = Vérification des mods pour { $mc }
upd-err-erst-pruefen = Appuie d’abord sur « Vérifier ».
upd-err-anderer-server = La vérification appartient à un autre serveur. Vérifie à nouveau.
upd-err-absturz = Le serveur a planté au démarrage. { $grund }
upd-err-beendet = Le serveur s’est arrêté aussitôt.
upd-err-kein-backup = Il n’y a pas de sauvegarde de la dernière mise à jour pour ce serveur.
upd-err-temp-platz = Pas assez de place dans le dossier temporaire pour une copie ({ $groesse } nécessaires).
cln-err-laeuft = Le nettoyage ne fonctionne que lorsque le serveur est arrêté.
fort-raeume-auf = Nettoyage des anciens fichiers …
fort-loesche = Suppression des anciens fichiers …
fort-loesche-kopie = Suppression de la copie …
nbt-err-unvollstaendig = servers.dat est incomplet.
nbt-err-laenge = servers.dat contient une longueur invalide.
nbt-err-eintrag = servers.dat contient une entrée inconnue.
nbt-err-tiefe = servers.dat est imbriqué trop profondément.
nbt-err-format = servers.dat a un format inconnu.
say-backup-start = Sauvegarde en cours …
say-backup-fertig = Sauvegarde terminée.
say-backup-fehler = Échec de la sauvegarde.
say-stoppt = s’arrête
say-neustart = redémarre
say-countdown = Le serveur { $was } dans { $sekunden } secondes !
svc-countdown = Serveur { $was } dans { $sekunden } s
svc-err-laeuft-nicht = Le serveur ne tourne pas actuellement.
svc-err-befehl = Commande inconnue : { $befehl }
svc-err-ordner-fehlt = Dossier du serveur manquant : { $ordner }
svc-err-eula = L’EULA de Minecraft n’a pas encore été acceptée pour ce serveur.
svc-err-fremder-server = Un serveur tourne déjà dans ce dossier (PID { $pid }) et n’a pas été lancé par Vapafane. Arrête-le d’abord (p. ex. ancien panneau).
svc-err-port = Le port { $port } ({ $art }) est déjà utilisé. Un autre serveur Minecraft tourne-t-il sur ce PC ?
svc-signal = terminé par signal
svc-beendet = terminé
svc-abgestuerzt = Planté ({ $grund }).
svc-taegliches-backup = Sauvegarde quotidienne
say-update-neustart = Le serveur redémarre pour une mise à jour !
svc-err-java-start = Java n’a pas pu être lancé ({ $java }) : { $fehler }

## Ligne de commande
ctl-hilfe = Utilisation : vapafane --ctl <commande>

      status            afficher l’état du serveur actif
      start             démarrer le serveur (lance le service d’arrière-plan si besoin)
      stop [SECONDES]   arrêter proprement le serveur, avec compte à rebours optionnel
      restart [SEC.]    redémarrer le serveur
      wait-ready [SEC.] attendre que le serveur soit prêt
      wait-stopped [S.] attendre que le serveur soit arrêté
      backup            sauvegarder tout le dossier du serveur en ZIP
      cmd <COMMANDE …>  envoyer une commande via RCON
      aktiv NOM         activer un autre serveur de la liste (seulement à l’arrêt)
      neu TYPE NOM VER [DOSSIER]  créer un nouveau serveur (fabric, paper, neoforge, vanilla) et l’activer
      update-check VER  vérifier mods/datapacks pour Minecraft VER via Modrinth
      trial VER         essai : mettre à jour une copie, tester sur un autre port, supprimer
      update VER        mise à jour : arrêter, sauvegarder, échanger, démarrer (seulement si la vérification est ok)
      mods              afficher mods et datapacks avec leur état activé/désactivé
      mod-an CHEMIN     activer un mod (CHEMIN comme dans « mods », p. ex. mods/x.jar)
      mod-aus CHEMIN    désactiver un mod (devient .jar.disabled)
      aufraeumen [--ja] montre les anciens restes ; avec --ja ils sont supprimés (serveur arrêté)
      mod-suche [TERME] [--datapack]    chercher sur Modrinth des mods/plugins (ou datapacks) adaptés
      mod-install PROJET [--datapack]   installer un mod/plugin/datapack avec ses dépendances requises
      curseforge-schluessel [--loeschen]  lire la clé CurseForge depuis l’entrée et l’enregistrer
      curseforge-test   essayer la clé CurseForge enregistrée
      einladung [ADRESSE]               afficher un code d’invitation pour les amis (adresse = hôte:port)
      einladung-datei FICHIER [ADRESSE] écrire un fichier d’invitation (.vapafane)
      einloesen CODE|FICHIER [--ordner O] [--alle-optionalen] [--ziel launcher|modrinth]
                … [--einstellungen-von DOSSIER] [--neue-instanz]
                                        configurer un client pour une invitation (profil launcher ou pack Modrinth) ;
                                        reprendre les paramètres (sans mondes) de DOSSIER ; avec --neue-instanz,
                                        une instance existante dans l’application Modrinth n’est pas mise à jour
      clients                           afficher les clients configurés
      client-entfernen DOSSIER          supprimer un client et son profil launcher
      ping ADRESSE                      interroger l’état du serveur (comme la liste des serveurs en jeu)
      erreichbarkeit    vérifier si le serveur est accessible de l’extérieur (IPv4/IPv6, DS-Lite)
      mrpack FICHIER    exporter les mods comme modpack Modrinth (.mrpack) pour les amis
      quit-service      arrêter le service d’arrière-plan (arrête d’abord le serveur)
ctl-fehler = Erreur : { $fehler }
