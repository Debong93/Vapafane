# Vapafane – Español (traducción automática por IA, se agradecen correcciones)

## Idioma
sprache-titel = Idioma
sprache-system = Idioma del sistema (recomendado)
sprache-ki-hinweis = Esta traducción se generó automáticamente (por IA); informa de errores en «Acerca de Vapafane».
sprache-neustart = La interfaz se recarga brevemente al cambiar.

## Einheiten und Zahlen
einheit-tag = d
einheit-std = h
einheit-min = min
einheit-sek = s
zahl-dezimal = ,
einheit-mio = M
einheit-tsd = mil
zahl-tausender = .

## Java
java-hinweis-windows = Instálalo p. ej. desde https://adoptium.net (Temurin { $version }).
java-hinweis-linux = Instálalo con: sudo apt install openjdk-{ $version }-jre-headless
err-java-fehlt = Java no está instalado en este PC; sin él no puede arrancar ningún servidor de Minecraft. Este servidor necesita Java { $version }. { $hinweis }
err-java-noetig = Minecraft { $mc } necesita Java { $version }, que no está instalado. { $hinweis }

## Botones comunes
gem-ok = Aceptar
gem-schliessen = Cerrar
gem-abbrechen = Cancelar
gem-weiter = Continuar
gem-entfernen = Quitar
gem-speichern = Guardar
task-abgebrochen = Cancelado

## Encabezado
hdr-untertitel = Taller de Minecraft para servidor y cliente
hdr-bereich = Zona
hdr-client = Cliente
hdr-server = Servidor
hdr-aktiver-server = Servidor activo (solo uno funciona a la vez)
hdr-kein-server = (ningún servidor)
hdr-plus-erster = Crea aquí tu primer servidor
hdr-plus = Añadir servidor
hdr-menu-neu = Crear un servidor nuevo (Fabric, Paper, NeoForge, Vanilla) …
hdr-menu-ordner = Añadir una carpeta existente …
hdr-menu-import = Importar un servidor desde ZIP/copia de seguridad …
hdr-menu-entfernen = Quitar de la lista el servidor seleccionado
hdr-einstellungen = Ajustes
hdr-hell-dunkel = Cambiar claro/oscuro
hdr-ev-gewechselt = Servidor cambiado: { $name }
hdr-err-umschalten = No se pudo cambiar

## Barra lateral
side-bereiche = Secciones
side-betrieb = Funcionamiento
side-uebersicht = Resumen
side-konsole = Consola
side-spieler = Jugadores
side-inhalte = Contenido
side-mods = Mods/Plugins
side-updates = Versión y actualizaciones
side-backups = Copias de seguridad
side-netzwerk = Red
side-erreichbarkeit = Accesibilidad

## Bienvenida
willkommen-titel = Bienvenido a Vapafane
willkommen-text = Vapafane no es un producto oficial de Minecraft. No está aprobado por Mojang o Microsoft ni asociado a ellos.

    Preguntas, ideas o fallos: en la página de GitHub del proyecto (Ajustes → Acerca de Vapafane).

## Común
gem-ordner = Carpeta
gem-ordner-oeffnen = Abrir carpeta
gem-launcher = Launcher oficial de Minecraft
gem-modrinth-app = Aplicación Modrinth

## Acerca de Vapafane
about-titel = Acerca de Vapafane
about-version = Versión { $version }
about-testmodus = MODO DE PRUEBA
about-nicht-offiziell = Vapafane no es un producto oficial de Minecraft. No está aprobado por Mojang o Microsoft ni asociado a ellos.
about-marken = Minecraft es una marca de Mojang/Microsoft. Los mods y plugins proceden de sus respectivos autores (vía Modrinth o CurseForge).
about-kontakt = Contacto e informar de fallos (GitHub) ↗
about-projektseite = Página del proyecto ↗

## Apariencia
skin-titel = Apariencia
skin-grass-sub = Claro u oscuro (botón arriba a la derecha)
skin-obsidian-sub = Siempre oscuro

## Ajustes del cliente
cset-einstellung = Ajuste
cset-einrichten = Instalación
cset-ziel = Destino predeterminado
cset-ziel-sub = Dónde se instalan los servidores nuevos
cset-spielordner = Carpetas de juego de todos los clientes
cset-spielordner-sub = Aquí están los mods, opciones y mundos de cada servidor

## Copias de seguridad
bak-restore-frage = ¿Devolver «{ $name }» al estado de { $datei }?
bak-restore-text = Para ello se detendrá el servidor. El estado actual no se borra: queda al lado en una carpeta llamada «…_vor_Wiederherstellung_<fecha>».
bak-wiederherstellen = Restaurar
bak-ev-wiederhergestellt = Copia de seguridad restaurada
bak-restore-fertig-titel = Restaurado
bak-restore-fertig = El servidor vuelve al estado del { $label }. Ya puedes iniciarlo.
bak-restore-daneben = El estado anterior está en:
    { $ordner }
    (puedes borrarlo si todo va bien)
bak-err-restore = No se pudo restaurar
bak-waehlen = Elegir copia de seguridad
bak-erklaerung = Cada copia es la carpeta completa del servidor en ZIP, con la fecha en el nombre. Se guarda en { $ordner }. Se conservan las { $anzahl } más recientes; las más antiguas se borran.
bak-jetzt = Hacer copia ahora
bak-ordner = Abrir carpeta de copias
bak-andere = Restaurar otro ZIP …
bak-aufheben = Conservar:
bak-leer = Aún no hay copias de seguridad de este servidor.

## Accesibilidad
rch-fehlgeschlagen = Falló la comprobación
rch-noch-nicht = Aún sin comprobar
rch-kein-server = Ningún servidor seleccionado
rch-kein-server-text = Crea un servidor con «+» arriba o elige uno; entonces se iniciará la comprobación.
rch-prueft = Comprobando …
rch-geprueft-um = Comprobado a las { $zeit }
rch-ip-erklaerung = IPv4 es la dirección clásica de internet; IPv6 la nueva, con dirección propia por dispositivo. Con DS-Lite tu proveedor comparte la dirección IPv4: entonces solo funciona IPv6.
rch-erneut = Comprobar de nuevo
rch-portfreigabe = Guía para abrir el puerto ↗
rch-dienst-hinweis = Para comprobar, Vapafane pregunta a un servicio en internet (ifconfig.co) si el puerto del servidor está abierto desde fuera. Desactivar: Ajustes → Automatización.

## Invitar amigos
inv-titel = Invitar amigos
inv-erklaerung = Envía un código a tus amigos: Vapafane les configura Minecraft con los mods adecuados.
inv-adresse-label = Dirección con la que tus amigos llegan al servidor
inv-oeffentlich = Averiguar dirección pública
inv-adresse-hinweis = Viene rellenada la dirección de la red local (solo misma red Wi-Fi). Para amigos de fuera: averiguar la dirección pública y abrir el puerto del servidor en el router.
inv-code-erzeugen = Crear código
inv-als-datei = Guardar como archivo …
inv-kein-passwort = El código no contiene contraseñas, solo dirección, versión y mods.
inv-datei-gespeichert = Guardado:
inv-datei-schicken = envía este archivo a tus amigos.
inv-code-label = Código de invitación ({ $zeichen } caracteres): cópialo y envíalo p. ej. por mensajería:
inv-kopieren = Copiar
inv-kopiert = Copiado
inv-mods = { $anzahl } mods para los jugadores
inv-mods-optional = (de ellos { $anzahl } opcionales)
inv-keine-mods = No hacen falta mods, solo la versión { $version }.
inv-ipv6 = Incluye también tu dirección IPv6 { $adresse }.
inv-nur-ipv6 = – según la última comprobación, tu servidor solo es accesible por IPv6.
inv-nur-server = Solo en el servidor (los jugadores no los necesitan): { $mods }
inv-datei-noetig = Estos mods no están ni en Modrinth ni en CurseForge: { $mods }. No caben en el código: usa «Guardar como archivo», ahí van incluidos.

## Jugadores y lista blanca
spi-online-titel = Conectados ahora
spi-wl-titel = Lista blanca (quién puede entrar)
spi-wl-erklaerung = Con la lista blanca activada (Ajustes → «Solo lista blanca»), solo pueden entrar los jugadores aquí listados. Protege de desconocidos; añádete también a ti.
spi-name-platzhalter = Nombre de Minecraft
spi-hinzufuegen = Añadir
spi-name-ungueltig = Introduce un nombre de Minecraft válido (3–16 caracteres).
spi-nur-laufend = Añadir solo funciona con el servidor en marcha (comprueba el nombre con Mojang).
spi-wl-leer = La lista blanca está vacía.
spi-niemand = Nadie conectado
spi-kreativ = Creativo
spi-ueberleben = Supervivencia
spi-op = Dar OP
spi-op-weg = Quitar OP
spi-kick = Expulsar
spi-kick-frage = ¿Expulsar a { $name }?
spi-kick-text = Puede volver a entrar enseguida.
spi-err-befehl = Falló el comando
wl-titel = Lista blanca
wl-einschalten = Activar lista blanca
wl-zu-spielern = Ir a jugadores
wl-warnung-offen = Atención: tu servidor es accesible desde internet y la lista blanca está desactivada; cualquiera que conozca o encuentre la dirección puede entrar. Con la lista blanca solo entran los jugadores que tú añadas.
wl-warnung-leer = La lista blanca está activada pero aún vacía: de momento no puede entrar nadie, ni siquiera tú. Añade tu nombre de Minecraft y los de tus amigos en «Jugadores».
wl-tipp = Consejo: la lista blanca está desactivada; cualquiera con la dirección puede entrar. Se recomienda en cuanto jueguen amigos desde fuera: entonces solo entran los jugadores aquí listados.

## Consola
kon-neues-log = archivo de registro nuevo (el servidor se reinició)
kon-beigetreten = { $name } ha entrado
kon-verlassen = { $name } ha salido de la partida
kon-bereit = Servidor listo (arrancó en { $sekunden } s)
kon-fehler = Error: { $fehler }
kon-tag = Día
kon-nacht = Noche
kon-wetter = Tiempo despejado
kon-wer-online = ¿Quién está conectado?
kon-platzhalter = Escribe un comando, p. ej. say Hola  ·  time set day  ·  weather clear
kon-senden = Enviar
kon-log-ordner = Abrir carpeta de registros

## Resumen
ueb-laeuft = En marcha
ueb-startet = Arrancando …
ueb-stoppt = Deteniendo …
ueb-neustart-absturz = Reinicio tras fallo …
ueb-gestoppt = Detenido
ueb-abgestuerzt = Falló
ueb-extern = Funciona fuera de Vapafane
ueb-anderer = Otro servidor está en marcha
ueb-starten = Iniciar
ueb-stoppen = Detener
ueb-neustart-knopf = Reiniciar
ueb-seit = desde hace { $dauer }
ueb-neustarts = reiniciado automáticamente { $anzahl }×
ueb-hinweis-extern = El servidor no se inició con Vapafane (p. ej. panel antiguo o terminal). La consola y los jugadores funcionan igualmente; «Detener» le envía el comando stop.
ueb-hinweis-anderer = El servicio en segundo plano ejecuta otro servidor. Detenlo primero.
ueb-hinweis-absturz = El servidor falló y no se reinició. Mira el último mensaje de error en la consola.
ueb-hinweis-kein-server = Añade un servidor con «＋» arriba.
ueb-ev-bereit = Servidor listo
ueb-ev-gestoppt = Servidor detenido
ueb-ev-startet = Arrancando el servidor
ueb-ev-neustart = El servidor se reinicia
ueb-ev-stoppt = El servidor se detiene
ueb-ev-in-sek = en { $sekunden } s
ueb-ev-absturz = El servidor falló
ueb-ev-absturz-neustart = El servidor falló; reinicio automático
ueb-notify-absturz = Servidor de Minecraft caído
ueb-notify-absturz-neustart = Se reinicia automáticamente.
ueb-ev-backup-start = Copia de seguridad iniciada
ueb-ev-backup-fertig = Copia terminada: { $datei } ({ $groesse })
ueb-err-start = No se pudo iniciar
ueb-err-stop = No se pudo detener
ueb-err-backup = Falló la copia de seguridad
ueb-eula-titel = Confirmar el EULA
ueb-eula-text = Ningún servidor arranca sin aceptar el EULA de Minecraft.
ueb-eula-akzeptiere = Acepto el
ueb-eula-fuer = para «{ $name }».
ueb-verb-stoppen = detener
ueb-verb-neustart = reiniciar
ueb-stop-frage = ¿{ $verb } el servidor?
ueb-stop-extern-text = El servidor funciona fuera de Vapafane. Le envío el comando «stop»: guarda el mundo y se detiene.
ueb-stop-niemand = Ahora mismo no hay nadie conectado.
ueb-stop-spieler = { $anzahl } jugadores conectados: { $namen }
ueb-sofort = Ahora mismo
ueb-mit-warnung = En { $sekunden } s con aviso
ueb-karte-spieler = Jugadores conectados
ueb-karte-tps = TPS (ticks por segundo)
ueb-karte-ram = Memoria
ueb-karte-ordner = Carpeta del servidor
ueb-karte-aussen = Desde fuera
ueb-karte-adresse = Dirección (red local)
ueb-karte-backup = Última copia
ueb-karte-version = Versión
ueb-server-aus = servidor apagado
ueb-keine-verbindung = sin conexión
ueb-niemand = nadie
ueb-ms-pro-tick = { $ms } ms por tick
ueb-ruckelt = va a tirones
ueb-fluessig = fluido
ueb-von-kernen = de { $anzahl } núcleos
ueb-ram-belegt-limit = en uso (límite { $limit } GB)
ueb-ram-belegt = en uso
ueb-ram-limit = límite { $limit } GB
ueb-frei = { $groesse } libres
ueb-vor = hace { $dauer }
ueb-noch-keins = ninguna todavía
ueb-backup-knopf = pulsa el botón «Backup»
ueb-backup-sub = { $groesse } · { $anzahl } conservadas
ueb-erreichbar = Accesible
ueb-nur-ipv6 = Solo IPv6
ueb-nicht-erreichbar = No accesible
ueb-ipv4-und-6 = IPv4 e IPv6
ueb-ueber-ipv4 = por IPv4
ueb-portfreigabe = comprobar apertura de puerto
ueb-ereignisse = Eventos recientes
ueb-nichts-passiert = Aún no ha pasado nada.
ueb-fenster-zu = Cerrar la ventana no detiene el servidor: sigue en segundo plano.
ueb-kontakt = Contacto e informar de fallos (GitHub)

## Ajustes (servidor)
set-spiel = Juego (este servidor)
set-motd = Nombre del servidor (MOTD)
set-max-spieler = Jugadores máx.
set-schwierigkeit = Dificultad
set-wert-peaceful = Pacífica
set-wert-easy = Fácil
set-wert-normal = Normal
set-wert-hard = Difícil
set-spielmodus = Modo de juego de los jugadores nuevos
set-wert-survival = Supervivencia
set-wert-creative = Creativo
set-wert-adventure = Aventura
set-wert-spectator = Espectador
set-sichtweite = Distancia de visión (chunks)
set-simulationsweite = Distancia de simulación (chunks)
set-spawn-schutz = Protección del spawn (bloques)
set-port = Puerto del servidor
set-whitelist = Solo lista blanca
set-whitelist-hilfe = Recomendado por seguridad: solo entran los jugadores de la lista blanca (página «Jugadores»). Sin ella puede entrar cualquiera que conozca tu dirección.
set-leistung = Rendimiento (este servidor)
set-ram = Memoria máx. para el servidor (GB)
set-ram-hinweis = Este PC tiene { $gb } GB. Deja unos 3–4 GB para el sistema.
set-speichern = Guardar ajustes
set-gespeichert = Guardado.
set-gespeichert-neustart = Guardado. Se aplicará en el próximo reinicio.
set-err-speichern = No se pudo guardar
set-automatik = Automatización
set-autostart = Iniciar automáticamente el servidor activo al encender el PC
set-auto-testmodus = Desactivado en modo de prueba.
set-auto-kein-systemd = No disponible en este sistema (sin systemd).
set-auto-windows = Arranca al encender el PC, incluso sin iniciar sesión (Windows pide permisos de administrador al activarlo).
set-auto-linger-fehlt = Atención: solo arranca cuando inicias sesión. Haz clic aquí para permitir el arranque sin iniciar sesión.
set-auto-aktiv = Arranca incluso sin iniciar sesión y se reinicia solo tras un fallo.
set-linger-frage = ¿Permitir el arranque sin iniciar sesión?
set-linger-text = Para ello, systemd debe poder iniciar tus servicios de usuario ya al encender («loginctl enable-linger»). Después el servidor funciona aunque nadie haya iniciado sesión.
set-linger-erlauben = Permitir
set-linger-abgelehnt = No permitido
set-fenster-autostart = Abrir esta ventana al iniciar sesión
set-backup-um = Copia de seguridad diaria a las
set-backup-uhr =  (servidor completo en ZIP)
set-backup-sub = La hace el servicio en segundo plano. Se recupera si el PC estaba apagado a esa hora.
set-uhrzeit = Hora de la copia diaria
set-reach = Comprobar accesibilidad desde fuera
set-reach-sub = Al cambiar a la zona del servidor. Para ello consulta un servicio de comprobación en internet.
set-notify = Notificaciones
set-notify-sub = Cuando alguien entra o sale y cuando el servidor falla (mientras la ventana esté abierta). Sin sonido.
set-ordner = Carpetas
set-ordner-server = Carpeta del servidor
set-an = activado
set-aus = desactivado
set-panel-titel = Aún hay un panel de control antiguo configurado.
set-panel-status = Servicio «minecraft.service»: { $dienst }, al arrancar { $start }; programa de copia diaria { $backup }. Mientras funcione, Vapafane no puede iniciar el servidor por sí mismo (mismo puerto), y las copias se harían por duplicado.
set-panel-knopf = Desactivar el panel antiguo …
set-panel-frage = ¿Desactivar el panel antiguo?
set-panel-text = Detengo el servicio «minecraft.service» (el servidor guarda y se apaga correctamente) y lo desactivo, junto con su copia diaria, al encender el PC.

    El panel antiguo y sus archivos se conservan; se puede reactivar en cualquier momento. Después inicias el servidor aquí con «Iniciar».
set-panel-abschalten = Desactivar
set-panel-aus = Panel antiguo desactivado
set-panel-err = No se pudo desactivar

## CurseForge
cf-titel = CurseForge (opcional)
cf-experimentell = Experimental
cf-brauche = ¿Lo necesito?
cf-erklaerung = No: la mayoría de los mods están en Modrinth, que funciona sin configurar nada. CurseForge encuentra además mods que solo existen allí y requiere para ello una clave API personal (una especie de código de acceso).
cf-anleitung-titel = Cómo conseguir una clave (gratis)
cf-schritt-1 = Haz clic en «Solicitar clave» y rellena el formulario (p. ej. propósito: «Mods para mi propio servidor»).
cf-schritt-2 = Espera la respuesta por correo: CurseForge revisa a mano, puede tardar unos días.
cf-schritt-3 = Pega la clave abajo, «Probar clave» y luego «Guardar».
cf-persoenlich = La clave es personal
cf-persoenlich-text = : no la compartas ni la publiques. Se queda solo en este PC. Se aplican las
cf-bedingungen = condiciones de CurseForge para programas de terceros
cf-experimentell-text = Experimental: CurseForge decide si concede claves a herramientas privadas.
cf-beantragen = Solicitar clave ↗
cf-konsole = Consola de CurseForge ↗
cf-mehr = Más sobre la API de CurseForge ↗
cf-platzhalter = Pega aquí la clave API
cf-testen = Probar clave
cf-teste = Probando …
cf-funktioniert = La clave funciona.
cf-schluessel-da = Clave guardada: { $maskiert } – CurseForge se incluye en las búsquedas.
cf-kein-schluessel = Sin clave guardada: se busca solo en Modrinth.
cf-gespeichert = Clave de CurseForge guardada
cf-entfernen-frage = ¿Borrar la clave de CurseForge?
cf-entfernen-text = Después Vapafane vuelve a buscar solo en Modrinth.

## Añadir servidor
gem-anderer-ordner = Otra carpeta …
srv-typ-fabric = Fabric (mods)
srv-typ-paper = Paper (plugins)
srv-typ-neoforge = NeoForge (mods)
srv-typ-vanilla = Vanilla (sin mods)
srv-name-label = Nombre en la lista de servidores
srv-typ-label = Tipo de servidor
srv-version-label = Versión de Minecraft
srv-laedt = cargando …
srv-keine-verbindung = Sin conexión: { $fehler }
srv-ordner-waehlen = Elegir carpeta para el servidor
srv-server-ordner-waehlen = Elegir la carpeta del servidor
srv-backup-waehlen = Elegir copia de seguridad del servidor (ZIP o tar)
srv-ordner-nicht-leer = La carpeta elegida no está vacía: el servidor irá a una subcarpeta nueva.
srv-ordner-ungueltig = Esta carpeta no se puede usar.
srv-eula-zusatz = (sin esto no arranca ningún servidor)
srv-hinweis = Nota
srv-ev-hinzu = Servidor añadido: { $name }
srv-hinzu-titel = Añadir servidor
srv-hinzu-text = «{ $name }» está en la lista. Detén el servidor en marcha para cambiar.
srv-err-hinzu = No se pudo añadir
srv-kein-server = No se encontró ningún servidor
srv-import-titel = Importar servidor
srv-err-import = No se pudo importar
srv-neu-titel = Crear servidor nuevo
srv-neu-name = Mi servidor
srv-err-erstellen = No se pudo crear
srv-entfernen-frage = ¿Quitar «{ $name }» de la lista?
srv-entfernen-text = Los archivos en { $ordner } se conservan; solo desaparece la entrada.
srv-err-entfernen = No se pudo quitar

## Mods/Plugins
mods-erklaerung = Los mods desactivados quedan en la carpeta pero no se cargan. Los cambios se aplican en el próximo arranque del servidor.
mods-ev-an = { $name } activado
mods-ev-aus = { $name } desactivado
mods-neustart-noetig = El servidor está en marcha: el cambio se aplicará tras el próximo reinicio.
mods-nur-client = solo cliente
mods-aus-grund = Desactivado: { $grund }
mods-an-tipp = Activado – clic para desactivar
mods-aus-tipp = Desactivado – clic para activar
mods-anzahl = { $an } de { $gesamt } { $wort } activados
mods-datapacks-anzahl = { $anzahl } datapacks
mods-leer = No hay { $wort } en la carpeta «{ $ordner }».
mods-datapacks-leer = No hay datapacks en el mundo.
mods-datapacks-titel = Datapacks del mundo
mods-hinzufuegen = Añadir { $wort } …
mods-hinzufuegen-titel = Añadir { $wort }
mods-ordner-oeffnen = Abrir carpeta de mods
mods-aufraeumen = Limpiar …
mods-aufraeumen-titel = Limpieza
mods-aufraeumen-tipp = Borrar restos de versiones antiguas (el servidor debe estar detenido)
mods-alles-sauber = Todo limpio: no hay restos de versiones antiguas.
mods-aufraeumen-text-1 = Se borrarían estos restos de versiones antiguas (en total
mods-aufraeumen-text-2 = El mundo, los ajustes y los mods desactivados se conservan. Lo que el servidor aún necesite lo vuelve a descargar en el próximo arranque.
mods-erst-stoppen = Detén primero el servidor para borrar.
mods-loeschen-knopf = Borrar { $groesse }
mods-ev-aufgeraeumt = Limpieza hecha: { $groesse } liberados
mods-aufgeraeumt-titel = Limpieza hecha
mods-aufgeraeumt = { $anzahl } entradas borradas, { $groesse } liberados.
mods-nicht-loeschbar = No se pudo borrar:
mods-err-aufraeumen = Falló la limpieza

## Catálogo (añadir mods)
kat-suche = Buscar en Modrinth … (vacío = los más populares)
kat-dp-option = Datapacks (para el mundo)
kat-treffer = { $anzahl } { $wort } compatibles para Minecraft { $mc }
kat-beliebtheit = ordenados por popularidad
kat-sucht = Buscando …
kat-nichts = No se encontró nada.
kat-mehr = Cargar más
kat-cf-tipp = Consejo: con una clave de CurseForge (Ajustes) encuentras además mods que solo existen allí.
kat-nur-cf = Solo en CurseForge
kat-cf-erklaerung = Estos no están en Modrinth. Algunos autores no permiten la descarga mediante otros programas: entonces descargas el archivo en el navegador y lo pones tú mismo en la carpeta.
kat-beides = Servidor + cliente
kat-beides-tipp = Los jugadores también lo necesitan, p. ej. vía «Exportar mods para amigos (.mrpack)»
kat-nur-server = Solo servidor
kat-meta = { $downloads } descargas · de { $autor }
kat-laedt = Cargando …
kat-installiert = Instalado
kat-installieren = Instalar
kat-ev-installiert = Instalado: { $name }
kat-err-install = La instalación no funcionó: { $fehler }
kat-ist-installiert = está instalado (en «{ $ordner }»)
kat-abhaengigkeiten = Añadido como dependencia: { $namen }.
kat-schon-da = Ya estaban: { $namen }.
kat-fehlende = Atención: esta dependencia no existe para esta versión: { $namen }. Así el servidor podría no arrancar.
kat-client-noetig = Los jugadores también necesitan este mod; lo más fácil es «Exportar mods para amigos (.mrpack)» en «Versión y actualizaciones».
kat-manuell-1 = El autor no permite la descarga mediante otros programas. Descarga el archivo
kat-manuell-2 = en el navegador y ponlo en la carpeta «{ $ordner }».
kat-browser = Descargar en el navegador ↗
kat-dp-ordner = Abrir carpeta de datapacks
kat-neustart = El servidor está en marcha: esto se aplica tras un reinicio.
kat-jetzt-neustarten = Reiniciar ahora

## Versión y actualizaciones
upd-st-aktuell = al día
upd-st-fehlt = falta
upd-st-unbekannt = desconocido
upd-st-neu = nuevo (necesario)
upd-st-manuell = actualizar solo en el navegador (CurseForge)
upd-st-reenable = vuelve a ser compatible – se activará
upd-st-doppelt = duplicado (ya existe como mod)
upd-alles-aktuell = Todo está al día.
upd-zusammenfassung = { $updates } actualizaciones, { $aktuell } ya al día, { $fehlend } sin versión compatible, { $unbekannt } desconocidos
upd-neu-gebraucht = { $anzahl } necesarios ahora
upd-passen-wieder = { $anzahl } vuelven a ser compatibles
upd-doppelt = { $anzahl } duplicados (se apartarán)
upd-server-stand = Nuevo build del servidor para Minecraft { $mc }: { $label }.
upd-beta-hinweis = Nota: por ahora solo existe una versión de prueba/beta.
upd-wechsel = Servidor: Minecraft { $von } → { $nach } ({ $label }).
upd-welt-warnung = Atención: un mundo que haya funcionado una vez con la versión nueva no puede volver atrás. El único retorno es la copia de seguridad hecha automáticamente.
upd-java-da = Se necesita Java { $version } – presente ({ $pfad }).
upd-java-fehlt = Se necesita Java { $version }, pero no está instalado. { $hinweis }
upd-manuell-hinweis = En los mods de «actualizar solo en el navegador», el autor no permite la descarga automática: descarga el archivo nuevo en CurseForge y cámbialo tú mismo.
upd-platz = No hay espacio suficiente para la copia de seguridad ({ $frei } libres, servidor { $server }).
upd-nur-mods = { $version } (actualizar solo mods/plugins)
upd-keine-neuere = No hay ninguna versión estable de Minecraft más reciente.
upd-keine-verbindung = Sin conexión
upd-err-pruefen = Falló la comprobación
upd-rollback-frage = ¿Volver al estado de antes de la última actualización?
upd-rollback-text = El servidor se detiene y se restaura desde la copia hecha en la actualización. El estado actual queda al lado en una carpeta «…_vor_Wiederherstellung_<fecha>».
upd-zuruecksetzen = Restablecer
upd-ev-rollback = Restaurado el estado de antes de la actualización
upd-rollback-fertig-titel = Restablecido
upd-rollback-fertig = El servidor vuelve al estado del { $label }. Ya puedes iniciarlo.
upd-rollback-daneben = El estado posterior a la actualización está en:
upd-err-rollback = No se pudo restablecer
upd-rollback-knopf = Volver al estado de antes de la última actualización
upd-was-mods = Actualizar mods
upd-ablauf = Proceso: detener el servidor (con aviso si hay alguien conectado) → copia completa → descargar y verificar { $anzahl } archivos → cambiar → iniciar el servidor y comprobar.
upd-werden-deaktiviert = Se desactivarán { $anzahl } mod(s)/datapack(s) sin versión compatible.
upd-probelauf-tipp = Consejo: con «Prueba» puedes probarlo sin riesgo en una copia.
upd-start-frage = ¿Iniciar la actualización? ({ $was })
upd-starten = Iniciar actualización
upd-ev-gestartet = Actualización iniciada ({ $was })
upd-err-update = Falló la actualización
upd-ev-fertig = Actualización terminada ({ $was })
upd-fertig-titel = Actualización terminada
upd-fertig-1 = El servidor vuelve a funcionar ({ $was }).
upd-fertig-backup = Copia del estado anterior:
upd-fertig-aufgeraeumt = Limpieza: { $anzahl } archivos/carpetas antiguos borrados ({ $groesse }).
upd-fertig-2 = Los mods sin versión compatible quedan desactivados y visibles en «Mods/Plugins».
upd-fehlgeschlagen-titel = La actualización no funcionó
upd-fehlgeschlagen-frage = Mira el mensaje de error en la consola. ¿Restauro el estado de antes de la actualización desde la copia?
upd-trial = Prueba
upd-trial-starten = Iniciar prueba
upd-trial-frage = ¿Hacer una prueba?
upd-trial-text = Copio el servidor a la carpeta temporal, actualizo la copia, la arranco en otro puerto y compruebo que arranca del todo. Después la copia se detiene y se borra.

    Tu servidor real queda intacto y puede seguir funcionando. Con mundos grandes, la copia tarda unos minutos.
upd-trial-tipp = Actualiza una copia en la carpeta temporal y la arranca en otro puerto
upd-trial-err = Falló la prueba
upd-trial-ok-titel = Prueba superada
upd-trial-ok = La copia actualizada terminó de arrancar en { $sekunden } segundos (puerto { $port }).
upd-trial-timeout = La copia no había terminado de arrancar tras 15 minutos.
upd-trial-exit = La copia se cerró al arrancar (código de salida { $code }).
upd-trial-meldungen = Mensajes destacados:
upd-trial-zeilen = Últimas líneas:
upd-trial-geloescht = La copia se ha borrado.
upd-trial-nicht-geloescht = La copia no se pudo borrar del todo: { $ordner }
upd-ev-trial-ok = Prueba superada
upd-ev-trial-fehler = Falló la prueba
upd-jetzt = Ahora:
upd-erklaerung = Comprueba si todos los mods existen para la versión elegida. Antes de cada actualización se hace una copia automáticamente.
upd-ziel = Objetivo:
upd-versionen = Cargar versiones
upd-pruefen = Comprobar
upd-spalte-mod = Mod / datapack
upd-spalte-neu = Nuevo
upd-deaktivieren-fehlende = Desactivar mods/datapacks sin versión compatible (si no, el servidor probablemente no arrancará)
upd-deaktivieren-unbekannte = Desactivar mods/datapacks desconocidos (no están en Modrinth)
upd-nicht-unterstuetzt = El asistente de actualización existe para Fabric, Paper, NeoForge y Vanilla.
upd-export-knopf = Exportar mods para amigos (.mrpack)
upd-export-tipp = Solo para servidores Fabric y NeoForge (mods que los amigos también necesitan)
upd-export-titel = Modpack guardado
upd-export-mods = { $anzahl } mods de Modrinth
upd-export-eingepackt = { $anzahl } incluidos directamente
upd-export-hinweis = Tus amigos importan el archivo en la aplicación Modrinth («Añadir archivo»). Los mods solo de servidor no se instalan ahí.
upd-err-export = No se pudo exportar

## Cliente
cli-titel = Usar invitación
cli-code-erklaerung-1 = Pega el código (empieza por
cli-code-erklaerung-2 = ) o abre el archivo
cli-code-erklaerung-3 =  – Vapafane configura Minecraft con todos los mods adecuados al servidor.
cli-code-platzhalter = Pega aquí el código de invitación (VPF1-…)
cli-pruefen = Comprobar invitación
cli-datei-oeffnen = Abrir archivo de invitación …
cli-pruefe-einladung = Comprobando la invitación …
cli-launcher-da = Launcher de Minecraft encontrado
cli-kein-launcher = No hay launcher de Minecraft
cli-modrinth-da = Aplicación Modrinth encontrada
cli-meine-server = Mis servidores
cli-leer = Ningún cliente configurado: usa una invitación arriba.
cli-kein-konto = Vapafane no inicia Minecraft por sí mismo y nunca toca tu cuenta de Microsoft: «Jugar» abre el launcher oficial o la aplicación Modrinth.
cli-nicht-erreichbar = Servidor no accesible
cli-online = En línea · { $version } · { $spieler }/{ $max } jugadores
cli-version-passt-nicht = En línea, pero versión { $version }: no coincide
cli-aktualisiert = El servidor se actualizó ({ $version }): pide al administrador un código de invitación nuevo
cli-gerade-nicht = ahora mismo no accesible
cli-modrinth-fehlt = No se encontró la aplicación Modrinth
cli-launcher-fehlt = No se encontró el launcher de Minecraft
cli-download-seite = Ir a la página de descarga
cli-ev-eingerichtet = Cliente configurado: { $name }
cli-gefunden = encontrado
cli-nicht-gefunden = no encontrado
cli-keine-mods = No hacen falta mods, solo Minecraft { $version }.
cli-optional = opcional
cli-nur-browser = solo descargable en el navegador (enlace tras la instalación)
cli-nur-datei = falta: solo está en el archivo de invitación
cli-launcher-fehlt-text = No se encuentra el launcher oficial de Minecraft. Puedes configurar igualmente; el perfil se creará en cuanto el launcher esté instalado y pulses aquí «Actualizar».
cli-launcher-laden = Descargar launcher
cli-nie-gestartet = El launcher nunca se ha abierto. Ábrelo una vez y ciérralo; luego configura aquí.
cli-ordner-waehlen = Elegir carpeta para este Minecraft
cli-mods-zusammen = { $anzahl } mods, { $groesse } en total
cli-andere-version = El servidor anuncia otra versión ({ $version }). Probablemente se actualizó: pide al administrador un código de invitación nuevo.
cli-spalte-version = Versión
cli-spalte-groesse = Tamaño
cli-einrichten-fuer = Configurar para:
cli-in-modrinth = En la aplicación Modrinth:
cli-instanz-update = Actualizar la instancia existente «{ $name }» (recomendado)
cli-instanz-update-sub = Todo queda en la misma instancia: mundos, ajustes, shaders, paquetes de recursos y los mods que añadiste tú (p. ej. Iris). Solo tienes que pulsar «Reinstalar modpack» en la aplicación Modrinth; Vapafane te enseña cómo.
cli-instanz-neu = Crear instancia nueva
cli-instanz-neu-sub = Funciona automáticamente mediante la importación. Abajo puedes traer ajustes, shaders y paquetes de recursos, pero no los mods añadidos por ti. La instancia antigua se conserva hasta que la borres.
cli-update-bleibt = Al actualizar, todo queda en la instancia: mundos, opciones, shaders, paquetes de recursos y tu lista de servidores.
cli-import-bestaetigen = Se abre la aplicación Modrinth: solo confirma la importación.
cli-ordner-label = Carpeta:
cli-launcher-schliessen = Cierra el launcher de Minecraft antes de configurar, para que no sobrescriba el cambio.
cli-einrichten = Configurar
cli-uebernehmen-von = Traer ajustes de:
cli-nichts = No traer nada
cli-quelle-waehlen = Elegir la carpeta de Minecraft de la que traer
cli-nichts-uebernehmen = Nada que traer
cli-teil-options = Opciones y teclas
cli-teil-config = Ajustes de los mods
cli-teil-shaders = Shaders
cli-teil-resourcepacks = Paquetes de recursos
cli-teil-servers = Lista de servidores
cli-teil-worlds = Mundos
cli-welten-warnung = Los mundos de una versión de Minecraft más nueva no se abren en una más antigua. Los mundos grandes ralentizan la configuración.
cli-nur-fehlendes = Lo que ya esté en la carpeta no se toca: solo se trae lo que falta.
cli-uebernehmen-erklaerung = Por ejemplo de tu instancia anterior: opciones, teclas, ajustes de mods, shaders, paquetes de recursos y lista de servidores. Los mods vienen siempre del servidor, a juego. La fuente solo se lee, nunca se modifica.
cli-paket-update-1 = El paquete nuevo de Modrinth está listo. Así actualizas tu instancia
cli-paket-update-2 = mundos, opciones, shaders, paquetes de recursos y tu lista de servidores se conservan todos:
cli-schritt-instanz = Abre la aplicación Modrinth y haz clic en la instancia
cli-schritt-anklicken = a la izquierda.
cli-schritt-zahnrad = Haz clic arriba en el engranaje «Ajustes de la instancia» y elige la pestaña «Instalación».
cli-neu-installieren = Reinstalar modpack
cli-schritt-datei = haz clic, confirma y elige este archivo:
cli-modrinth-oeffnen = Abrir la aplicación Modrinth
cli-paket-ordner = Abrir la carpeta con el paquete
cli-update-sicher = La aplicación Modrinth primero respalda la instancia y solo cambia los mods del servidor: lo que añadiste tú (shaders, paquetes de recursos, mods propios) se queda.
cli-paket-offen = El paquete de Modrinth está listo y la aplicación Modrinth se ha abierto con él. Confirma allí la importación; la instancia aparecerá como
cli-paket-fertig = El paquete de Modrinth está listo:
cli-paket-von-hand = La aplicación Modrinth no se pudo abrir automáticamente. A mano: en la aplicación Modrinth haz clic en «+» (crear instancia), luego «Importar desde archivo» y elige este archivo.
cli-eingerichtet = Configurado: { $anzahl } mods descargados.
cli-profil = El launcher de Minecraft tiene ahora el perfil «{ $name }».
cli-uebernommen-paket = Los ajustes de «{ $quelle }» van en el paquete; la aplicación Modrinth los aplica al importar.
cli-uebernommen = Traído de «{ $quelle }»: { $anzahl } archivos.
cli-uebernommen-da = ({ $anzahl } ya estaban y no se tocaron)
cli-uebernommen-fehler = { $anzahl } archivos no se pudieron leer y por eso faltan. ¿Sigue Minecraft abierto ahí? ({ $dateien })
cli-selbst-laden = Para descargar tú mismo y poner en la carpeta «mods»:
cli-spielen = Jugar
cli-spielen-modrinth = Jugar (abrir la aplicación Modrinth)
cli-spielen-launcher = Jugar (abrir el launcher)
cli-prueft = comprobando …
cli-unvollstaendig = La configuración no terminó: usa de nuevo la invitación de arriba (o borra).
cli-mods-kurz = { $anzahl } mods
cli-eingerichtet-am = { $ziel } · configurado el { $datum }
cli-aktualisieren = Actualizar
cli-aktualisieren-tipp = Reconfigurar con la invitación guardada, o explicar qué hacer tras una actualización del servidor
cli-entfernen-frage = ¿Quitar «{ $name }»?
cli-entfernen-modrinth = Vapafane olvida este cliente y borra su paquete. La instancia en la aplicación Modrinth se queda: bórrala allí tú mismo.
cli-entfernen-launcher = Se borra la carpeta con los mods, ajustes y mundos de un jugador de este perfil, y el perfil desaparece del launcher de Minecraft.
cli-upd-titel = El servidor se ha actualizado
cli-upd-text = «{ $name }» funciona ahora con Minecraft { $version }. La versión y los mods adecuados van en la invitación, así que necesitas un código de invitación nuevo del administrador.
cli-upd-code = Pégalo arriba en «Usar invitación» y pulsa «Comprobar invitación».
cli-upd-modrinth = Vapafane crea entonces un paquete nuevo y te enseña cómo actualizar tu instancia: tus ajustes, mundos y shaders se conservan.
cli-upd-launcher = Vapafane cambia entonces los mods: tus ajustes y mundos se conservan.
cli-gespeicherte-einladung = Cargada la invitación guardada de «{ $name }». «Configurar» lo deja todo en orden, por ejemplo si faltan mods o desapareció el perfil. Si el administrador actualizó el servidor, necesitas más bien un código de invitación nuevo.
ext-knopf = Shaders y extras
ext-knopf-so-gehts = Shaders y extras: cómo se hace

## Shaders y extras
ext-titel = Añadir shaders y extras
ext-skizze-1-alt = Esquema de la instancia en la aplicación Modrinth
ext-skizze-2-alt = Esquema de la búsqueda en la aplicación Modrinth
ext-skizze-leer = Sin contenido adicional instalado
ext-skizze-hochladen = Subir archivos
ext-durchsuchen = Explorar contenido
ext-skizze-shader = Un shader, p. ej. Complementary
ext-skizze-beschreibung = Descripción · descargas
ext-keine-shader = Los shaders necesitan el mod Iris, que solo funciona en servidores Fabric o NeoForge. Este servidor no tiene mod loader, así que aquí funcionan los paquetes de recursos, pero no los shaders.
ext-iris-modrinth = Para shaders, Minecraft necesita el mod Iris: en el paso 4 elige primero «Mods» e instala «Iris Shaders» (Sodium viene automáticamente); luego elige un shader en «Shaders».
ext-iris-launcher-1 = Para shaders, Minecraft necesita el mod Iris (con Sodium): descarga ambos para la versión de Minecraft del servidor desde
ext-iris-launcher-2 = y ponlos en la carpeta «mods».
ext-mr-einleitung-1 = En tu instancia
ext-mr-einleitung-2 = puedes añadir en cualquier momento shaders, paquetes de recursos y mods propios. Se conservan en las actualizaciones del servidor: la aplicación Modrinth solo cambia los mods del servidor.
ext-schritt-content = Elige arriba la pestaña «Content» (contenido).
ext-schritt-durchsuchen = Haz clic en el botón verde «Explorar contenido».
ext-schritt-art = Elige arriba el tipo: «Shaders» para shaders, «Resource Packs» para paquetes de recursos o «Mods».
ext-schritt-installieren = Haz clic en «Instalar» junto a la entrada deseada. Listo.
ext-im-spiel = En el juego: Opciones → Ajustes gráficos → Paquetes de shaders → elegir el shader.
ext-im-spiel-rp = En el juego: Opciones → Paquetes de recursos → elegir el paquete.
ext-mods-update-hinweis = Tus propios mods debes actualizarlos tú en la aplicación tras una actualización de Minecraft; los shaders y paquetes de recursos normalmente no.
ext-launcher-einleitung = La carpeta de juego de este servidor es solo suya: lo que añadas ahí se conserva en las actualizaciones del servidor.
ext-schritt-laden-shader = Descarga un shader, p. ej. en
ext-schritt-laden-rp = Descarga un paquete de recursos, p. ej. en
ext-nicht-entpacken = no descomprimas el archivo .zip.
ext-schritt-shaderpacks = Haz clic abajo en «Abrir carpeta» y pon el archivo en la carpeta «shaderpacks». Los paquetes de recursos van en «resourcepacks».
ext-schritt-resourcepacks = Haz clic abajo en «Abrir carpeta» y pon el archivo en la carpeta «resourcepacks».
ext-eigene-mods = Puedes poner mods propios en la carpeta «mods», pero tras una actualización de Minecraft debes mantenerlos compatibles tú mismo.
err-antwort-unlesbar = Respuesta de «{ $befehl }» ilegible: { $fehler }

## Informe de accesibilidad
rch-w-erreichbar = accesible
rch-w-nicht = no accesible
rch-w-nicht-dslite = no accesible (DS-Lite)
rch-w-nicht-geprueft = sin comprobar
rch-w-nicht-vorhanden = no disponible
rch-unmoeglich-titel = Ahora mismo no se puede comprobar
rch-unmoeglich-text = El servicio de comprobación en internet no responde. Inténtalo más tarde.
rch-gruen-beide = Accesible por IPv4 e IPv6
rch-gruen-v4 = Accesible por IPv4
rch-gruen-text = Todos los amigos pueden entrar.
rch-gelb-dslite-titel = DS-Lite: accesible solo por IPv6
rch-gelb-dslite-text = Tu proveedor comparte la dirección IPv4. Los amigos con IPv6 pueden entrar igualmente; son la mayoría.
rch-gelb-titel = Accesible solo por IPv6
rch-gelb-text = Por IPv4 no entra nadie: o falta la apertura del puerto IPv4 (puerto { $port }) o tu proveedor comparte la dirección IPv4 (DS-Lite). Los amigos con IPv6 pueden entrar igualmente.
rch-rot-titel = No accesible desde fuera
rch-rot-text = Normalmente falta la apertura del puerto en el router, o el cortafuegos del PC bloquea el puerto { $port }.
rch-rot-dslite-v6 = Tu proveedor comparte la dirección IPv4 (DS-Lite), así que solo funciona IPv6. Abre el puerto { $port } para IPv6 en el router y revisa el cortafuegos del PC.
rch-rot-dslite = Tu proveedor comparte la dirección IPv4 (DS-Lite) y tu PC no tiene dirección IPv6. Así nadie de fuera puede llegar al servidor.
rch-aus-titel = El servidor no está en marcha
rch-aus-text = La comprobación se hará en cuanto el servidor funcione.
rch-port-belegt-titel = El puerto { $port } ya está en uso
rch-port-belegt-text = Otro programa –probablemente otro servidor de Minecraft– usa este puerto. Así este servidor no puede arrancar. Configura un puerto libre en Ajustes → «Puerto del servidor» (p. ej. { $frei })
rch-hotspot-tipp = El servidor solo es accesible por IPv6, pero tu internet no tiene IPv6. Consejo: conecta el PC al punto de acceso de tu móvil; la red móvil suele tener IPv6.

## Errores del programa (general)
err-dienst-aus = El servicio en segundo plano no está en marcha
err-kein-systemd = systemd no está disponible aquí (o modo de prueba).
err-kein-server = Ningún servidor seleccionado.
err-schon-beschaeftigt = Ya hay algo en marcha: { $was }
err-neustart-extern = Este servidor funciona fuera de Vapafane: el reinicio solo funciona desde el programa que lo inició.
err-stop-knopf = Usa el botón Detener del resumen (guarda correctamente).
err-start-extern = El servidor funciona fuera de Vapafane. Detenlo primero allí.
err-start-anderer = Ahora mismo funciona otro servidor. Detenlo primero.
err-stop-unmoeglich = No se pudo detener el servidor.
err-pfad = Ruta no válida
err-rcon-antwort = Respuesta RCON no válida
err-adresse = Dirección no válida
cln-alte-neoforge = versión antigua de NeoForge
cln-alte-bibliothek = versión antigua de una biblioteca
cln-alte-mods = versiones antiguas de mods (apartadas por el panel/launcher antiguo)
cln-cache = caché de mods (se recrea en el próximo arranque)
cli-datei-titel = Abrir archivo de invitación
srv-rcon-an = RCON activado (Vapafane lo necesita para los comandos).
err-link-verboten = Esta dirección no se puede abrir.
err-pruefsumme = La suma de comprobación no coincide: { $datei }
err-kein-fabric = No se encontró un loader de Fabric para Minecraft { $mc }
cf-err-leer = Pega primero una clave.
cf-err-id = ID de CurseForge no válido
cf-err-format = Esto no parece una clave API (pégala sin espacios).
cf-err-antwort = Respuesta no válida de CurseForge: { $fehler }
cf-err-abgelehnt = CurseForge rechaza la clave API (no válida o aún no activada).
cf-err-fehlt = No hay clave de CurseForge guardada.
kat-err-typ = «Añadir mods» no existe para este tipo de servidor.
kat-err-keine-version = «{ $name }» no existe para Minecraft { $mc } ({ $loader }).
kat-err-cf-version = «{ $name }» no existe en CurseForge para Minecraft { $mc }.
fort-suche = Buscando { $name } …
fort-suche-cf = Buscando { $name } en CurseForge …
inv-err-neuer = Este código de invitación es de una versión más nueva de Vapafane. Actualiza Vapafane.
inv-err-unvollstaendig = El código está incompleto o dañado: cópialo entero.
inv-err-kaputt = El código está dañado: cópialo de nuevo.
inv-err-version = Esta invitación es de otra versión de Vapafane.
inv-err-angaben = La invitación contiene datos no válidos.
inv-err-mod = La invitación contiene una entrada de mod no válida.
inv-err-typ = Hay invitaciones para servidores Fabric, NeoForge, Paper y Vanilla.
inv-err-datei = Esto no es un archivo de invitación de Vapafane válido.
inv-err-datei-kaputt = La invitación del archivo está dañada.
inv-err-oeffentlich = No se pudo averiguar la dirección pública.
flv-err-kein-server = No hay servidor para Minecraft { $mc }.
flv-err-kein-paper = No hay Paper para { $mc }.
flv-err-kein-neoforge = No hay NeoForge para Minecraft { $mc }.
fort-neoforge = El instalador de NeoForge prepara el servidor (descarga las bibliotecas) …
fort-suche-server = Buscando { $was } para Minecraft { $mc } …
fort-pruefe-java = Comprobando Java …
fort-frage-cf = Consultando a CurseForge por los mods restantes …
fort-pruefe-deps = Comprobando dependencias …
fort-pruefe-dp = Comprobando datapacks …
fort-pruefe-java-platz = Comprobando Java y el espacio en disco …
upd-err-dateiname = Nombre de archivo no válido de Modrinth: { $name }
upd-cf-fehler = No se pudo comprobar CurseForge: { $fehler }
upd-abhaengigkeit = (dependencia)
upd-grund-fehlt = no existe para Minecraft { $mc }
cli-err-port = Puerto no válido en la dirección
cli-err-antwort = Respuesta no válida del servidor
cli-err-profil-kaputt = { $datei } está dañado: abre el launcher una vez.
cli-err-nicht-leer = La carpeta { $ordner } no está vacía y no pertenece a Vapafane.
cli-err-datei-kaputt = El archivo { $datei } de la invitación está dañado.
cli-err-fabric-profil = Perfil de Fabric no válido.
cli-err-neoforge-java = NeoForge necesita Java { $version }. Abre el launcher de Minecraft una vez (trae Java) o instala Java.
cli-hinweis-launcher-starten = Abre el launcher de Minecraft una vez y ciérralo; luego configura de nuevo.
cli-hinweis-kein-launcher = No se encuentra el launcher oficial de Minecraft. Instálalo, ábrelo una vez y pulsa aquí «Actualizar».
cli-hinweis-nie-gestartet = El launcher nunca se ha abierto. Ábrelo una vez y ciérralo; luego «Actualizar».
cli-paket-summary = Configurado por Vapafane para el servidor { $name } ({ $adresse })
fort-pruefe-mods = Comprobando los mods …
fort-lade = Descargando { $name } …
fort-launcher = Configurando el launcher …
fort-neoforge-install = Instalando NeoForge { $version } (puede tardar unos minutos) …
fort-uebernehme = Trayendo los ajustes …
fort-trage-ein = Añadiendo el servidor …
fort-sammle = Recopilando los ajustes …
fort-packe = Empaquetando el paquete de Modrinth …
fort-fertig = Listo.
task-update = Actualización
task-backup = Copia de seguridad (servidor completo en ZIP)
task-restore = Restaurar copia de seguridad
task-client = Configurar cliente
task-rollback = Volver al estado de antes de la actualización
task-pruefe-mods = Comprobando mods para { $mc }
upd-err-erst-pruefen = Pulsa primero «Comprobar».
upd-err-anderer-server = La comprobación es de otro servidor. Comprueba de nuevo.
upd-err-absturz = El servidor falló al arrancar. { $grund }
upd-err-beendet = El servidor se cerró enseguida.
upd-err-kein-backup = No hay copia de la última actualización de este servidor.
upd-err-temp-platz = No hay espacio suficiente en la carpeta temporal para una copia (se necesitan { $groesse }).
cln-err-laeuft = La limpieza solo funciona con el servidor detenido.
fort-raeume-auf = Limpiando archivos antiguos …
fort-loesche = Borrando archivos antiguos …
fort-loesche-kopie = Borrando la copia …
nbt-err-unvollstaendig = servers.dat está incompleto.
nbt-err-laenge = servers.dat contiene una longitud no válida.
nbt-err-eintrag = servers.dat contiene una entrada desconocida.
nbt-err-tiefe = servers.dat está anidado demasiado profundo.
nbt-err-format = servers.dat tiene un formato desconocido.
say-backup-start = Haciendo copia de seguridad …
say-backup-fertig = Copia de seguridad terminada.
say-backup-fehler = Falló la copia de seguridad.
say-stoppt = se detiene
say-neustart = se reinicia
say-countdown = ¡El servidor { $was } en { $sekunden } segundos!
svc-countdown = Servidor { $was } en { $sekunden } s
svc-err-laeuft-nicht = El servidor no está en marcha ahora.
svc-err-befehl = Comando desconocido: { $befehl }
svc-err-ordner-fehlt = Falta la carpeta del servidor: { $ordner }
svc-err-eula = El EULA de Minecraft aún no se ha aceptado para este servidor.
svc-err-fremder-server = En esta carpeta ya funciona un servidor (PID { $pid }) que no fue iniciado por Vapafane. Detenlo primero (p. ej. panel antiguo).
svc-err-port = El puerto { $port } ({ $art }) ya está en uso. ¿Funciona otro servidor de Minecraft en este PC?
svc-signal = terminado por señal
svc-beendet = terminado
svc-abgestuerzt = Falló ({ $grund }).
svc-taegliches-backup = Copia de seguridad diaria
say-update-neustart = ¡El servidor se reinicia por una actualización!
svc-err-java-start = No se pudo iniciar Java ({ $java }): { $fehler }

## Línea de comandos
ctl-hilfe = Uso: vapafane --ctl <comando>

      status            mostrar el estado del servidor activo
      start             iniciar el servidor (arranca el servicio en segundo plano si hace falta)
      stop [SEGUNDOS]   detener el servidor correctamente, con cuenta atrás opcional
      restart [SEG.]    reiniciar el servidor
      wait-ready [SEG.] esperar a que el servidor esté listo
      wait-stopped [S.] esperar a que el servidor esté detenido
      backup            guardar toda la carpeta del servidor en ZIP
      cmd <COMANDO …>   enviar un comando por RCON
      aktiv NOMBRE      activar otro servidor de la lista (solo detenido)
      neu TIPO NOMBRE VER [CARPETA]  crear un servidor nuevo (fabric, paper, neoforge, vanilla) y activarlo
      update-check VER  comprobar mods/datapacks para Minecraft VER vía Modrinth
      trial VER         prueba: actualizar una copia, probarla en otro puerto, borrarla
      update VER        actualización: detener, copiar, cambiar, iniciar (solo si la comprobación está bien)
      mods              mostrar mods y datapacks con su estado activado/desactivado
      mod-an RUTA       activar un mod (RUTA como en «mods», p. ej. mods/x.jar)
      mod-aus RUTA      desactivar un mod (pasa a .jar.disabled)
      mod-entfernen RUTA  borrar definitivamente un mod/datapack
      aufraeumen [--ja] muestra restos antiguos; con --ja se borran (servidor detenido)
      mod-suche [TÉRMINO] [--datapack]  buscar en Modrinth mods/plugins (o datapacks) compatibles
      mod-install PROYECTO [--datapack] instalar un mod/plugin/datapack con sus dependencias necesarias
      curseforge-schluessel [--loeschen]  leer la clave de CurseForge de la entrada y guardarla
      curseforge-test   probar la clave de CurseForge guardada
      einladung [DIRECCIÓN]             mostrar un código de invitación para amigos (dirección = host:puerto)
      einladung-datei ARCHIVO [DIRECCIÓN] escribir un archivo de invitación (.vapafane)
      einloesen CÓDIGO|ARCHIVO [--ordner C] [--alle-optionalen] [--ziel launcher|modrinth]
                … [--einstellungen-von CARPETA] [--neue-instanz]
                                        configurar un cliente para una invitación (perfil del launcher o paquete Modrinth);
                                        traer los ajustes (sin mundos) de CARPETA; con --neue-instanz
                                        no se actualiza una instancia existente de la aplicación Modrinth
      clients                           mostrar los clientes configurados
      client-entfernen CARPETA          borrar un cliente y su perfil del launcher
      ping DIRECCIÓN                    consultar el estado del servidor (como la lista de servidores del juego)
      erreichbarkeit    comprobar si el servidor es accesible desde fuera (IPv4/IPv6, DS-Lite)
      mrpack ARCHIVO    exportar los mods como modpack de Modrinth (.mrpack) para amigos
      quit-service      cerrar el servicio en segundo plano (primero detiene el servidor)
ctl-fehler = Error: { $fehler }

## Programm-Meldungen (Teil 3)
err-archivformat = Formato de archivo desconocido (permitidos: .zip, .tar, .tar.gz, .tar.zst)
fort-entpacke = Extrayendo { $datei }
err-ordner-nicht-leer = La carpeta { $ordner } no está vacía.
err-aufgabenplanung = No se pudo configurar el Programador de tareas (cancelado o sin permiso). { $details }
err-mc-unbekannt = La versión de Minecraft del servidor es desconocida: inicia el servidor una vez.
kat-err-projekt = El proyecto «{ $projekt }» no existe en Modrinth.
kat-err-schon-da = «{ $name }» ya está instalado.
kat-err-keine-datei = «{ $name }» no tiene ningún archivo descargable.
fort-sortiere = Colocando los archivos …
kat-err-installiert = Eso ya está instalado.
fort-lade-cf = Descargando { $name } de CurseForge …
cln-alte-mc-dateien = Archivos de una versión antigua de Minecraft
cln-alte-server-datei = Archivo de servidor de una versión antigua
cln-alte-mc-datei = Archivo de Minecraft de una versión antigua
cln-vor-update = Archivos de antes de una actualización (su copia ZIP se conserva)
cln-ersetzt = archivos reemplazados por esta actualización
cli-err-keine-adresse = No se indicó ninguna dirección de servidor.
cli-err-adresse = No se encontró la dirección «{ $adresse }».
cli-err-nicht-erreichbar = Servidor no accesible (apagado, dirección incorrecta o puerto no abierto).
cli-err-mod-weg = Un mod de la invitación ya no existe en Modrinth ({ $id }).
cli-err-hash = Un archivo de mod no coincide con la invitación (la suma de comprobación no coincide).
cli-nur-in-datei = incluido solo en el archivo de invitación
err-neoforge-installer = El instalador de NeoForge falló:
    { $details }
cli-err-kein-client = Esto no es un cliente de Vapafane.
cli-err-launcher-fehlt = No se encontró el launcher oficial de Minecraft. Descarga: { $link }
cli-keine-app = (ninguna app)
cli-test-oeffnen = Modo de prueba – abriría: { $wie } { $pfad }
inv-datei-titel = Guardar archivo de invitación
inv-datei-name = Invitacion
inv-datei-filter = Invitación de Vapafane
err-ziel-unbekannt = Destino desconocido
cf-err-404 = No encontrado en CurseForge.
cf-err-429 = CurseForge informa de demasiadas solicitudes: espera un momento.
cf-err-code = CurseForge respondió con el error { $code }.
cf-err-bereich = No se encontró la categoría de CurseForge «{ $bereich }».
err-mc-gibt-es-nicht = Minecraft { $mc } no existe en Mojang.
err-paper-download = No hay descarga de Paper para esta versión.
err-neoforge-start = El instalador de NeoForge no pudo iniciarse ({ $java }): { $fehler }
err-rcon-weg = No hay conexión RCON con el servidor.
fort-stoppe = Deteniendo el servidor (guardando el mundo) …
err-ordner-unbekannt = Carpeta desconocida
err-fenster = No se pudo abrir la ventana: { $fehler }
inv-err-kein-code = Esto no es un código de invitación de Vapafane (empieza por «VPF1-»).
inv-err-typ-unbekannt = Tipo de servidor desconocido en la invitación.
inv-err-datei-leer = El archivo no contiene ninguna invitación (vapafane.json).
err-dienst-antwort = No hay respuesta del servicio en segundo plano.
err-dienst-fehler = Error en el servicio en segundo plano.
err-dienst-start = No se pudo iniciar el servicio en segundo plano: { $details }
err-dienst-stumm = El servicio en segundo plano no responde. Los detalles están en { $datei }
err-umschalten-laeuft = Solo se puede cambiar cuando el servidor actual está detenido.
err-aufgabe-laeuft = Espera a que termine la tarea en curso.
err-ordner-fehlt = Falta la carpeta: { $ordner }
err-kein-server-ordner = { $ordner } no contiene ni un jar de servidor ni un server.properties.
err-ordner-doppelt = Esta carpeta ya está en la lista.
err-name-fehlt = Introduce un nombre.
task-import = Importando «{ $name }»
err-typ-unbekannt = Tipo de servidor desconocido
task-erstelle = Creando el servidor { $typ } «{ $name }»
err-sprache-unbekannt = Idioma desconocido
err-erst-stoppen = Detén primero el servidor.
err-port-zahl = El puerto debe ser un número.
err-port-bereich = El puerto debe estar entre 1024 y 65535.
err-uhrzeit = Indica una hora como 05:00.
err-noch-kein-server = Aún no se ha creado ningún servidor.
err-http-code = El servidor respondió con el error { $code }.
err-keine-verbindung = Sin conexión: { $fehler }
err-zu-viele-anfragen = { $host } informa de demasiadas solicitudes. Espera unos minutos y vuelve a intentarlo.
err-fabric-installer = No se encontró ningún instalador de Fabric.
err-andere-aufgabe = otra tarea
fort-speichert = El servidor está guardando el mundo …
err-port-belegt = El puerto { $port } ya está en uso: probablemente hay otro servidor de Minecraft ahí. Configura un puerto libre en Ajustes → «Puerto del servidor», p. ej. { $frei }.
err-rcon-aus = RCON está desactivado en server.properties.
err-rcon-nicht-verbunden = RCON no está conectado.
err-rcon-passwort = La contraseña de RCON es incorrecta.
err-rcon-erreichbar = RCON no es accesible.
err-keine-jar = No se encontró ningún jar de servidor. Pon el jar en la carpeta.
err-keine-jar-name = No se encontró ningún jar de servidor (MC_JAR={ $jar }).
err-nichts-zu-uebernehmen = En esta carpeta no hay nada que traer. Elige una carpeta de juego de Minecraft: contiene p. ej. «options.txt», «config», «shaderpacks» o «saves».
cli-quelle-ordner = Carpeta: { $ordner }
err-ordner-weg = La carpeta { $ordner } ya no existe.
err-teil-unbekannt = Parte desconocida: { $teil }
fort-lese-mods = Leyendo los mods y plugins instalados …
fort-lade-namen = Cargando los nombres de los mods …
fort-lade-version = Descargando { $name } { $version } …
fort-tausche = Cambiando los archivos …
fort-richte-ein = Configurando { $name } …
fort-getauscht = Archivos cambiados.
err-start-timeout = El servidor aún no había terminado de arrancar tras 15 minutos.
fort-warne = Avisando a los jugadores (30 segundos) …
fort-backup = Creando la copia de seguridad …
fort-starte = Iniciando el servidor (el primer arranque puede tardar unos minutos) …
fort-kopiere = Copiando { $datei }
fort-starte-kopie = Iniciando la copia en el puerto { $port } …
fort-stoppe-kopie = Deteniendo la copia …
ctl-timeout = Tiempo agotado (estado: { $zustand })
ctl-startet = Iniciando el servidor.
ctl-stoppt = Deteniendo el servidor.
ctl-neustart = Reiniciando el servidor.
ctl-fehlt-name = Indica un nombre.
ctl-nicht-in-liste = «{ $name }» no está en la lista.
ctl-neu-benutzung = Uso: --ctl neu TIPO NOMBRE VERSIÓN [CARPETA]
ctl-fehlt-version = Indica la versión de destino.
ctl-update-plan = Versión { $von } → { $nach }, Java necesario: { $java } (ok: { $javaok }), espacio ok: { $platzok }, trabajo pendiente: { $tun }
ctl-java-platz = Java o el espacio en disco no son suficientes.
ctl-fehlt-pfad = Indica una ruta.
ctl-geloescht = Borrado: { $groesse } (no posible: { $fehler })
ctl-treffer = { $anzahl } resultados para Minecraft { $mc } ({ $typ }):
ctl-fehlt-projekt = Indica un proyecto.
ctl-cf-geloescht = Clave de CurseForge borrada.
ctl-cf-ok = La clave de CurseForge funciona ({ $schluessel }).
ctl-nur-server = Solo en el servidor (no en la invitación): { $mods }
ctl-ohne-quelle = Sin origen (solo posible en el archivo de invitación): { $mods }
ctl-fehlt-code = Indica un código o un archivo.
ctl-fehlt-ordner = Indica una carpeta.
ctl-fehlt-adresse = Indica una dirección.
ctl-fehlt-zieldatei = Indica un archivo de destino.
ctl-dienst-ende = Cerrando el servicio en segundo plano.
gem-zitat = «{ $text }»
spi-entfernt = { $name } quitado.

## Mods: entfernen, Abhängigkeiten, Infos
mods-err-name-belegt = Ya existe un archivo activado con este nombre.
mods-err-ordner-aus = Las carpetas de datapack no se pueden desactivar, solo los datapacks ZIP. Pero sí se pueden quitar.
mods-err-weg = El archivo ya no existe.
mods-grund-hand = desactivado a mano
mods-grund-unbekannt = no está en Modrinth – versión compatible incierta
mods-grund-doppelt = duplicado – «{ $name }» ya está instalado como mod
mods-ev-entfernt = { $name } quitado
mods-entfernen-tipp = Quitar definitivamente
mods-entfernen-frage = ¿Quitar «{ $name }»?
mods-entfernen-text = El archivo se borrará definitivamente. Todavía está en la última copia de seguridad.
mods-gebraucht-warnung = Atención, todavía lo necesitan: { $namen }. Sin este archivo, el servidor probablemente no arrancará.
mods-waisen-frage = Ningún otro mod necesitará después estos ayudantes. ¿Quitarlos también?
mods-aus-frage = ¿Desactivar «{ $name }»?
mods-aus-knopf = Desactivar igualmente
mods-err-entfernen = No se pudo quitar
mods-gebraucht-von = Lo necesitan: { $namen }
mods-bibliothek = Biblioteca
mods-bibliothek-tipp = Un ayudante que otros mods necesitan
mods-seite = Página
mods-seite-tipp = Abrir la página del mod en el navegador: explica lo que hace
mods-suche = Buscar en la lista …
