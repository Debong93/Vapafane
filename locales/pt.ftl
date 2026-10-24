# Vapafane – Português (tradução automática por IA, correções são bem-vindas)

## Idioma
sprache-titel = Idioma
sprache-system = Idioma do sistema (recomendado)
sprache-ki-hinweis = Esta tradução foi gerada automaticamente (por IA) – comunique erros em «Sobre o Vapafane».
sprache-neustart = Ao mudar, a interface recarrega por um instante.

## Java
java-hinweis-windows = Instale p. ex. em https://adoptium.net (Temurin { $version }).
java-hinweis-linux = Instale com: sudo apt install openjdk-{ $version }-jre-headless
err-java-fehlt = O Java não está instalado neste PC – sem ele nenhum servidor de Minecraft consegue iniciar. Este servidor precisa do Java { $version }. { $hinweis }
err-java-noetig = O Minecraft { $mc } precisa do Java { $version }, que não está instalado. { $hinweis }

## Botões comuns
gem-ok = OK
gem-schliessen = Fechar
gem-abbrechen = Cancelar
gem-weiter = Continuar
gem-entfernen = Remover
gem-speichern = Salvar
task-abgebrochen = Cancelado

## Cabeçalho
hdr-untertitel = Oficina de Minecraft para servidor e cliente
hdr-bereich = Área
hdr-client = Cliente
hdr-server = Servidor
hdr-aktiver-server = Servidor ativo (só um funciona por vez)
hdr-kein-server = (nenhum servidor)
hdr-plus-erster = Crie aqui o seu primeiro servidor
hdr-plus = Adicionar servidor
hdr-menu-neu = Criar um servidor novo (Fabric, Paper, NeoForge, Vanilla) …
hdr-menu-ordner = Adicionar uma pasta existente …
hdr-menu-import = Importar um servidor de ZIP/backup …
hdr-menu-entfernen = Remover da lista o servidor selecionado
hdr-einstellungen = Configurações
hdr-hell-dunkel = Alternar claro/escuro
hdr-ev-gewechselt = Servidor trocado: { $name }
hdr-err-umschalten = Não foi possível trocar

## Barra lateral
side-bereiche = Seções
side-betrieb = Operação
side-uebersicht = Visão geral
side-konsole = Console
side-spieler = Jogadores
side-inhalte = Conteúdo
side-mods = Mods/Plugins
side-updates = Versão e atualizações
side-backups = Backups
side-netzwerk = Rede
side-erreichbarkeit = Acessibilidade

## Boas-vindas
willkommen-titel = Bem-vindo ao Vapafane
willkommen-text = O Vapafane não é um produto oficial do Minecraft. Não é aprovado pela Mojang ou Microsoft nem associado a elas.

    Perguntas, ideias ou erros: na página do projeto no GitHub (Configurações → Sobre o Vapafane).

## Comum
gem-ordner = Pasta
gem-ordner-oeffnen = Abrir pasta
gem-launcher = Launcher oficial do Minecraft
gem-modrinth-app = App Modrinth

## Sobre o Vapafane
about-titel = Sobre o Vapafane
about-version = Versão { $version }
about-testmodus = MODO DE TESTE
about-nicht-offiziell = O Vapafane não é um produto oficial do Minecraft. Não é aprovado pela Mojang ou Microsoft nem associado a elas.
about-marken = Minecraft é uma marca da Mojang/Microsoft. Mods e plugins vêm dos seus respectivos autores (via Modrinth ou CurseForge).
about-kontakt = Contato e comunicar erros (GitHub) ↗
about-projektseite = Página do projeto ↗

## Aparência
skin-titel = Aparência
skin-grass-sub = Claro ou escuro (botão no canto superior direito)
skin-obsidian-sub = Sempre escuro

## Configurações do cliente
cset-einstellung = Configuração
cset-einrichten = Instalação
cset-ziel = Destino padrão
cset-ziel-sub = Onde os servidores novos são instalados
cset-spielordner = Pastas de jogo de todos os clientes
cset-spielordner-sub = Aqui ficam os mods, opções e mundos de cada servidor

## Backups
bak-restore-frage = Devolver «{ $name }» ao estado de { $datei }?
bak-restore-text = Para isso o servidor será parado. O estado atual não é apagado – fica ao lado numa pasta chamada «…_vor_Wiederherstellung_<data>».
bak-wiederherstellen = Restaurar
bak-ev-wiederhergestellt = Backup restaurado
bak-restore-fertig-titel = Restaurado
bak-restore-fertig = O servidor voltou ao estado de { $label }. Você já pode iniciá-lo.
bak-restore-daneben = O estado anterior está em:
    { $ordner }
    (pode ser apagado se estiver tudo bem)
bak-err-restore = Não foi possível restaurar
bak-waehlen = Escolher backup
bak-erklaerung = Cada backup é a pasta completa do servidor em ZIP, com a data no nome. Salvo em { $ordner }. Os { $anzahl } mais recentes são mantidos, os mais antigos apagados.
bak-jetzt = Fazer backup agora
bak-ordner = Abrir pasta de backups
bak-andere = Restaurar outro ZIP …
bak-aufheben = Manter:
bak-leer = Ainda não há backups deste servidor.

## Acessibilidade
rch-fehlgeschlagen = Falha na verificação
rch-noch-nicht = Ainda não verificado
rch-kein-server = Nenhum servidor selecionado
rch-kein-server-text = Crie um servidor com «+» no topo ou escolha um; a verificação começa então.
rch-prueft = Verificando …
rch-geprueft-um = Verificado às { $zeit }
rch-ip-erklaerung = IPv4 é o endereço clássico da internet; IPv6 é o novo, com endereço próprio por aparelho. Com DS-Lite o seu provedor compartilha o endereço IPv4 – então só o IPv6 funciona.
rch-erneut = Verificar de novo
rch-portfreigabe = Guia de abertura de porta ↗
rch-dienst-hinweis = Para verificar, o Vapafane pergunta a um serviço na internet (ifconfig.co) se a porta do servidor está aberta de fora. Desativar: Configurações → Automação.

## Convidar amigos
inv-titel = Convidar amigos
inv-erklaerung = Envie um código aos seus amigos – o Vapafane configura o Minecraft deles com os mods certos.
inv-adresse-label = Endereço pelo qual os seus amigos chegam ao servidor
inv-oeffentlich = Descobrir endereço público
inv-adresse-hinweis = O endereço da rede local já vem preenchido (só na mesma rede Wi-Fi). Para amigos de fora: descobrir o endereço público e abrir a porta do servidor no roteador.
inv-code-erzeugen = Criar código
inv-als-datei = Salvar como arquivo …
inv-kein-passwort = O código não contém senhas – só endereço, versão e mods.
inv-datei-gespeichert = Salvo:
inv-datei-schicken = envie este arquivo aos seus amigos.
inv-code-label = Código de convite ({ $zeichen } caracteres) – copie e envie p. ex. por mensagem:
inv-kopieren = Copiar
inv-kopiert = Copiado
inv-mods = { $anzahl } mods para os jogadores
inv-mods-optional = (sendo { $anzahl } opcionais)
inv-keine-mods = Nenhum mod necessário – só a versão { $version }.
inv-ipv6 = Contém também o seu endereço IPv6 { $adresse }.
inv-nur-ipv6 = – segundo a última verificação, o seu servidor só é acessível via IPv6.
inv-nur-server = Só no servidor (os jogadores não precisam): { $mods }
inv-datei-noetig = Estes mods não estão nem no Modrinth nem no CurseForge: { $mods }. Não cabem no código – use «Salvar como arquivo», que eles vão incluídos.

## Jogadores e whitelist
spi-online-titel = Online agora
spi-wl-titel = Whitelist (quem pode entrar)
spi-wl-erklaerung = Com a whitelist ativa (Configurações → «Só whitelist»), só entram os jogadores listados aqui. Protege de desconhecidos – adicione você também.
spi-name-platzhalter = Nome no Minecraft
spi-hinzufuegen = Adicionar
spi-name-ungueltig = Digite um nome de Minecraft válido (3–16 caracteres).
spi-nur-laufend = Adicionar só funciona com o servidor em execução (ele verifica o nome na Mojang).
spi-wl-leer = A whitelist está vazia.
spi-niemand = Ninguém online
spi-kreativ = Criativo
spi-ueberleben = Sobrevivência
spi-op = Dar OP
spi-op-weg = Tirar OP
spi-kick = Expulsar
spi-kick-frage = Expulsar { $name }?
spi-kick-text = A pessoa pode voltar logo em seguida.
spi-err-befehl = Falha no comando
wl-titel = Whitelist
wl-einschalten = Ativar whitelist
wl-zu-spielern = Ir para jogadores
wl-warnung-offen = Atenção: o seu servidor está acessível pela internet e a whitelist está desativada – qualquer pessoa que conheça ou descubra o endereço pode entrar. Com a whitelist só entram os jogadores que você adicionar.
wl-warnung-leer = A whitelist está ativa mas ainda vazia – por enquanto ninguém consegue entrar, nem você. Adicione em «Jogadores» o seu nome de Minecraft e os dos seus amigos.
wl-tipp = Dica: a whitelist está desativada – qualquer pessoa com o endereço pode entrar. É recomendada assim que amigos jogarem de fora: então só entram os jogadores listados aqui.

## Console
kon-neues-log = novo arquivo de log (o servidor foi reiniciado)
kon-beigetreten = { $name } entrou
kon-verlassen = { $name } saiu do jogo
kon-bereit = Servidor pronto (iniciou em { $sekunden } s)
kon-fehler = Erro: { $fehler }
kon-tag = Dia
kon-nacht = Noite
kon-wetter = Tempo limpo
kon-wer-online = Quem está online?
kon-platzhalter = Digite um comando, p. ex. say Olá  ·  time set day  ·  weather clear
kon-senden = Enviar
kon-log-ordner = Abrir pasta de logs

## Visão geral
ueb-laeuft = Em execução
ueb-startet = Iniciando …
ueb-stoppt = Parando …
ueb-neustart-absturz = Reinício após falha …
ueb-gestoppt = Parado
ueb-abgestuerzt = Travou
ueb-extern = Funciona fora do Vapafane
ueb-anderer = Outro servidor está em execução
ueb-starten = Iniciar
ueb-stoppen = Parar
ueb-neustart-knopf = Reiniciar
ueb-seit = há { $dauer }
ueb-neustarts = reiniciado automaticamente { $anzahl }×
ueb-hinweis-extern = O servidor não foi iniciado pelo Vapafane (p. ex. painel antigo ou terminal). Console e jogadores funcionam mesmo assim; «Parar» envia o comando stop.
ueb-hinweis-anderer = O serviço em segundo plano executa outro servidor. Pare-o primeiro.
ueb-hinweis-absturz = O servidor travou e não foi reiniciado. Veja a última mensagem de erro no console.
ueb-hinweis-kein-server = Adicione um servidor com «＋» no topo.
ueb-ev-bereit = Servidor pronto
ueb-ev-gestoppt = Servidor parado
ueb-ev-startet = Iniciando o servidor
ueb-ev-neustart = O servidor reinicia
ueb-ev-stoppt = O servidor está parando
ueb-ev-in-sek = em { $sekunden } s
ueb-ev-absturz = O servidor travou
ueb-ev-absturz-neustart = O servidor travou – reinício automático
ueb-notify-absturz = Servidor de Minecraft travou
ueb-notify-absturz-neustart = Ele reinicia automaticamente.
ueb-ev-backup-start = Backup iniciado
ueb-ev-backup-fertig = Backup concluído: { $datei } ({ $groesse })
ueb-err-start = Não foi possível iniciar
ueb-err-stop = Não foi possível parar
ueb-err-backup = Falha no backup
ueb-eula-titel = Confirmar o EULA
ueb-eula-text = Nenhum servidor inicia sem aceitar o EULA do Minecraft.
ueb-eula-akzeptiere = Aceito o
ueb-eula-fuer = para «{ $name }».
ueb-verb-stoppen = parar
ueb-verb-neustart = reiniciar
ueb-stop-frage = { $verb } o servidor?
ueb-stop-extern-text = O servidor funciona fora do Vapafane. Envio a ele o comando «stop» – ele salva o mundo e para.
ueb-stop-niemand = Ninguém está online agora.
ueb-stop-spieler = { $anzahl } jogadores online: { $namen }
ueb-sofort = Agora mesmo
ueb-mit-warnung = Em { $sekunden } s com aviso
ueb-karte-spieler = Jogadores online
ueb-karte-tps = TPS (ticks por segundo)
ueb-karte-ram = Memória
ueb-karte-ordner = Pasta do servidor
ueb-karte-aussen = De fora
ueb-karte-adresse = Endereço (rede local)
ueb-karte-backup = Último backup
ueb-karte-version = Versão
ueb-server-aus = servidor desligado
ueb-keine-verbindung = sem conexão
ueb-niemand = ninguém
ueb-ms-pro-tick = { $ms } ms por tick
ueb-ruckelt = engasgando
ueb-fluessig = fluido
ueb-von-kernen = de { $anzahl } núcleos
ueb-ram-belegt-limit = em uso (limite { $limit } GB)
ueb-ram-belegt = em uso
ueb-ram-limit = limite { $limit } GB
ueb-frei = { $groesse } livres
ueb-vor = há { $dauer }
ueb-noch-keins = nenhum ainda
ueb-backup-knopf = aperte o botão «Backup»
ueb-backup-sub = { $groesse } · { $anzahl } mantidos
ueb-erreichbar = Acessível
ueb-nur-ipv6 = Só IPv6
ueb-nicht-erreichbar = Inacessível
ueb-ipv4-und-6 = IPv4 e IPv6
ueb-ueber-ipv4 = via IPv4
ueb-portfreigabe = verificar abertura de porta
ueb-ereignisse = Eventos recentes
ueb-nichts-passiert = Ainda não aconteceu nada.
ueb-fenster-zu = Fechar a janela não para o servidor – ele continua em segundo plano.
ueb-kontakt = Contato e comunicar erros (GitHub)

## Configurações (servidor)
set-spiel = Jogo (este servidor)
set-motd = Nome do servidor (MOTD)
set-max-spieler = Jogadores máx.
set-schwierigkeit = Dificuldade
set-wert-peaceful = Pacífico
set-wert-easy = Fácil
set-wert-normal = Normal
set-wert-hard = Difícil
set-spielmodus = Modo de jogo dos jogadores novos
set-wert-survival = Sobrevivência
set-wert-creative = Criativo
set-wert-adventure = Aventura
set-wert-spectator = Espectador
set-sichtweite = Distância de visão (chunks)
set-simulationsweite = Distância de simulação (chunks)
set-spawn-schutz = Proteção do spawn (blocos)
set-port = Porta do servidor
set-whitelist = Só whitelist
set-whitelist-hilfe = Recomendado por segurança: só entram os jogadores da whitelist (página «Jogadores»). Sem ela, qualquer pessoa que conheça o seu endereço pode entrar.
set-leistung = Desempenho (este servidor)
set-ram = Memória máx. para o servidor (GB)
set-ram-hinweis = Este PC tem { $gb } GB. Deixe uns 3–4 GB para o sistema.
set-speichern = Salvar configurações
set-gespeichert = Salvo.
set-gespeichert-neustart = Salvo. Vale a partir do próximo reinício.
set-err-speichern = Não foi possível salvar
set-automatik = Automação
set-autostart = Iniciar automaticamente o servidor ativo ao ligar o PC
set-auto-testmodus = Desativado no modo de teste.
set-auto-kein-systemd = Indisponível neste sistema (sem systemd).
set-auto-windows = Inicia ao ligar o PC, mesmo sem login (o Windows pede direitos de administrador ao ativar).
set-auto-linger-fehlt = Atenção: ele só inicia quando você faz login. Clique aqui para permitir iniciar sem login.
set-auto-aktiv = Inicia mesmo sem login e reinicia sozinho após uma falha.
set-linger-frage = Permitir iniciar sem login?
set-linger-text = Para isso o systemd precisa poder iniciar os seus serviços de usuário já na inicialização («loginctl enable-linger»). Depois o servidor funciona mesmo sem ninguém logado.
set-linger-erlauben = Permitir
set-linger-abgelehnt = Não permitido
set-fenster-autostart = Abrir esta janela ao fazer login
set-backup-um = Backup diário às
set-backup-uhr =  (servidor completo em ZIP)
set-backup-sub = Feito pelo serviço em segundo plano. É recuperado se o PC estava desligado nessa hora.
set-uhrzeit = Hora do backup diário
set-reach = Verificar acessibilidade de fora
set-reach-sub = Ao mudar para a área do servidor. Para isso consulta um serviço de verificação na internet.
set-notify = Notificações
set-notify-sub = Quando alguém entra ou sai e quando o servidor trava (enquanto a janela estiver aberta). Sem som.
set-ordner = Pastas
set-ordner-server = Pasta do servidor
set-an = ligado
set-aus = desligado
set-panel-titel = Ainda há um painel de controle antigo configurado.
set-panel-status = Serviço «minecraft.service»: { $dienst }, na inicialização { $start }; programa de backup diário { $backup }. Enquanto ele funcionar, o Vapafane não pode iniciar o servidor sozinho (mesma porta), e os backups seriam feitos em dobro.
set-panel-knopf = Desativar o painel antigo …
set-panel-frage = Desativar o painel antigo?
set-panel-text = Paro o serviço «minecraft.service» (o servidor salva e desliga direito) e desativo ele e o seu backup diário na inicialização do PC.

    O painel antigo em si e os seus arquivos ficam – dá para reativar a qualquer momento. Depois você inicia o servidor aqui com «Iniciar».
set-panel-abschalten = Desativar
set-panel-aus = Painel antigo desativado
set-panel-err = Não foi possível desativar

## CurseForge
cf-titel = CurseForge (opcional)
cf-experimentell = Experimental
cf-brauche = Preciso disso?
cf-erklaerung = Não – a maioria dos mods está no Modrinth, que funciona sem configurar nada. O CurseForge encontra ainda mods que só existem lá e precisa para isso de uma chave API pessoal (uma espécie de código de acesso).
cf-anleitung-titel = Como conseguir uma chave (grátis)
cf-schritt-1 = Clique em «Solicitar chave» e preencha o formulário (p. ex. finalidade: «Mods para o meu próprio servidor»).
cf-schritt-2 = Aguarde a resposta por e-mail – o CurseForge revisa manualmente, pode levar alguns dias.
cf-schritt-3 = Cole a chave abaixo, «Testar chave» e depois «Salvar».
cf-persoenlich = A chave é pessoal
cf-persoenlich-text = – não compartilhe nem publique. Ela fica só neste PC. Valem as
cf-bedingungen = condições do CurseForge para programas de terceiros
cf-experimentell-text = Experimental: o CurseForge decide se concede chaves a ferramentas privadas.
cf-beantragen = Solicitar chave ↗
cf-konsole = Console do CurseForge ↗
cf-mehr = Mais sobre a API do CurseForge ↗
cf-platzhalter = Cole a chave API aqui
cf-testen = Testar chave
cf-teste = Testando …
cf-funktioniert = A chave funciona.
cf-schluessel-da = Chave salva: { $maskiert } – o CurseForge entra nas buscas.
cf-kein-schluessel = Nenhuma chave salva – busca só no Modrinth.
cf-gespeichert = Chave do CurseForge salva
cf-entfernen-frage = Apagar a chave do CurseForge?
cf-entfernen-text = Depois o Vapafane volta a buscar só no Modrinth.

## Adicionar servidor
gem-anderer-ordner = Outra pasta …
srv-typ-fabric = Fabric (mods)
srv-typ-paper = Paper (plugins)
srv-typ-neoforge = NeoForge (mods)
srv-typ-vanilla = Vanilla (sem mods)
srv-name-label = Nome na lista de servidores
srv-typ-label = Tipo de servidor
srv-version-label = Versão do Minecraft
srv-laedt = carregando …
srv-keine-verbindung = Sem conexão: { $fehler }
srv-ordner-waehlen = Escolher pasta para o servidor
srv-server-ordner-waehlen = Escolher a pasta do servidor
srv-backup-waehlen = Escolher backup do servidor (ZIP ou tar)
srv-ordner-nicht-leer = A pasta escolhida não está vazia – o servidor irá para uma subpasta nova.
srv-ordner-ungueltig = Esta pasta não pode ser usada.
srv-eula-zusatz = (sem isso nenhum servidor inicia)
srv-hinweis = Observação
srv-ev-hinzu = Servidor adicionado: { $name }
srv-hinzu-titel = Adicionar servidor
srv-hinzu-text = «{ $name }» está na lista. Pare o servidor em execução para trocar.
srv-err-hinzu = Não foi possível adicionar
srv-kein-server = Nenhum servidor encontrado
srv-import-titel = Importar servidor
srv-err-import = Não foi possível importar
srv-neu-titel = Criar servidor novo
srv-neu-name = Meu servidor
srv-err-erstellen = Não foi possível criar
srv-entfernen-frage = Remover «{ $name }» da lista?
srv-entfernen-text = Os arquivos em { $ordner } ficam; só a entrada some.
srv-err-entfernen = Não foi possível remover

## Mods/Plugins
mods-erklaerung = Mods desativados ficam na pasta mas não são carregados. As mudanças valem a partir do próximo início do servidor.
mods-ev-an = { $name } ativado
mods-ev-aus = { $name } desativado
mods-neustart-noetig = O servidor está em execução – a mudança vale após o próximo reinício.
mods-nur-client = só cliente
mods-aus-grund = Desativado: { $grund }
mods-an-tipp = Ativado – clique para desativar
mods-aus-tipp = Desativado – clique para ativar
mods-anzahl = { $an } de { $gesamt } { $wort } ativados
mods-datapacks-anzahl = { $anzahl } datapacks
mods-leer = Nenhum { $wort } na pasta «{ $ordner }».
mods-datapacks-leer = Nenhum datapack no mundo.
mods-datapacks-titel = Datapacks no mundo
mods-hinzufuegen = Adicionar { $wort } …
mods-hinzufuegen-titel = Adicionar { $wort }
mods-ordner-oeffnen = Abrir pasta de mods
mods-aufraeumen = Limpar …
mods-aufraeumen-titel = Limpeza
mods-aufraeumen-tipp = Apagar restos de versões antigas (o servidor precisa estar parado)
mods-alles-sauber = Tudo limpo – não há restos de versões antigas.
mods-aufraeumen-text-1 = Estes restos de versões antigas seriam apagados (no total
mods-aufraeumen-text-2 = Mundo, configurações e mods desativados ficam. O que o servidor ainda precisar, ele baixa de novo no próximo início.
mods-erst-stoppen = Pare primeiro o servidor para apagar.
mods-loeschen-knopf = Apagar { $groesse }
mods-ev-aufgeraeumt = Limpeza feita: { $groesse } liberados
mods-aufgeraeumt-titel = Limpeza feita
mods-aufgeraeumt = { $anzahl } itens apagados, { $groesse } liberados.
mods-nicht-loeschbar = Não foi possível apagar:
mods-err-aufraeumen = Falha na limpeza

## Catálogo (adicionar mods)
kat-suche = Buscar no Modrinth … (vazio = os mais populares)
kat-dp-option = Datapacks (para o mundo)
kat-treffer = { $anzahl } { $wort } compatíveis para Minecraft { $mc }
kat-beliebtheit = ordenados por popularidade
kat-sucht = Buscando …
kat-nichts = Nada encontrado.
kat-mehr = Carregar mais
kat-cf-tipp = Dica: com uma chave do CurseForge (Configurações) você encontra também mods que só existem lá.
kat-nur-cf = Só no CurseForge
kat-cf-erklaerung = Estes não estão no Modrinth. Alguns autores não permitem o download por outros programas – então você baixa o arquivo no navegador e coloca na pasta você mesmo.
kat-beides = Servidor + cliente
kat-beides-tipp = Os jogadores também precisam dele – p. ex. via «Exportar mods para amigos (.mrpack)»
kat-nur-server = Só servidor
kat-meta = { $downloads } downloads · de { $autor }
kat-laedt = Carregando …
kat-installiert = Instalado
kat-installieren = Instalar
kat-ev-installiert = Instalado: { $name }
kat-err-install = A instalação não funcionou: { $fehler }
kat-ist-installiert = está instalado (em «{ $ordner }»)
kat-abhaengigkeiten = Adicionado como dependência: { $namen }.
kat-schon-da = Já estavam: { $namen }.
kat-fehlende = Atenção: esta dependência não existe para esta versão: { $namen } – assim o servidor pode não iniciar.
kat-client-noetig = Os jogadores também precisam deste mod – o mais fácil é «Exportar mods para amigos (.mrpack)» em «Versão e atualizações».
kat-manuell-1 = O autor não permite o download por outros programas. Baixe o arquivo
kat-manuell-2 = no navegador e coloque na pasta «{ $ordner }».
kat-browser = Baixar no navegador ↗
kat-dp-ordner = Abrir pasta de datapacks
kat-neustart = O servidor está em execução – isto vale após um reinício.
kat-jetzt-neustarten = Reiniciar agora

## Versão e atualizações
upd-st-aktuell = atualizado
upd-st-fehlt = faltando
upd-st-unbekannt = desconhecido
upd-st-neu = novo (necessário)
upd-st-manuell = atualizar só no navegador (CurseForge)
upd-st-reenable = compatível de novo – será ativado
upd-st-doppelt = duplicado (já existe como mod)
upd-alles-aktuell = Tudo atualizado.
upd-zusammenfassung = { $updates } atualizações, { $aktuell } já atualizados, { $fehlend } sem versão compatível, { $unbekannt } desconhecidos
upd-neu-gebraucht = { $anzahl } agora necessários
upd-passen-wieder = { $anzahl } compatíveis de novo
upd-doppelt = { $anzahl } duplicados (serão postos de lado)
upd-server-stand = Novo build do servidor para Minecraft { $mc }: { $label }.
upd-beta-hinweis = Observação: por enquanto só existe uma versão de teste/beta.
upd-wechsel = Servidor: Minecraft { $von } → { $nach } ({ $label }).
upd-welt-warnung = Atenção: um mundo que rodou uma vez com a versão nova não pode voltar. O único retorno é pelo backup feito automaticamente.
upd-java-da = Java { $version } necessário – presente ({ $pfad }).
upd-java-fehlt = Java { $version } necessário mas não instalado. { $hinweis }
upd-manuell-hinweis = Nos mods «atualizar só no navegador», o autor não permite o download automático – baixe o arquivo novo no CurseForge e troque você mesmo.
upd-platz = Espaço em disco insuficiente para o backup ({ $frei } livres, servidor { $server }).
upd-nur-mods = { $version } (atualizar só mods/plugins)
upd-keine-neuere = Não há versão estável do Minecraft mais recente.
upd-keine-verbindung = Sem conexão
upd-err-pruefen = Falha na verificação
upd-rollback-frage = Voltar ao estado de antes da última atualização?
upd-rollback-text = O servidor é parado e restaurado do backup feito na atualização. O estado atual fica ao lado numa pasta «…_vor_Wiederherstellung_<data>».
upd-zuruecksetzen = Reverter
upd-ev-rollback = Estado de antes da atualização restaurado
upd-rollback-fertig-titel = Revertido
upd-rollback-fertig = O servidor voltou ao estado de { $label }. Você já pode iniciá-lo.
upd-rollback-daneben = O estado de depois da atualização está em:
upd-err-rollback = Não foi possível reverter
upd-rollback-knopf = Voltar ao estado de antes da última atualização
upd-was-mods = Atualizar mods
upd-ablauf = Etapas: parar o servidor (com aviso se alguém estiver online) → backup completo → baixar e verificar { $anzahl } arquivos → trocar → iniciar o servidor e conferir.
upd-werden-deaktiviert = { $anzahl } mod(s)/datapack(s) sem versão compatível serão desativados.
upd-probelauf-tipp = Dica: com «Ensaio» você testa isso sem risco numa cópia.
upd-start-frage = Iniciar a atualização? ({ $was })
upd-starten = Iniciar atualização
upd-ev-gestartet = Atualização iniciada ({ $was })
upd-err-update = Falha na atualização
upd-ev-fertig = Atualização concluída ({ $was })
upd-fertig-titel = Atualização concluída
upd-fertig-1 = O servidor está funcionando de novo ({ $was }).
upd-fertig-backup = Backup do estado antigo:
upd-fertig-aufgeraeumt = Limpeza: { $anzahl } arquivos/pastas antigos apagados ({ $groesse }).
upd-fertig-2 = Mods sem versão compatível ficam desativados e visíveis em «Mods/Plugins».
upd-fehlgeschlagen-titel = A atualização não funcionou
upd-fehlgeschlagen-frage = Veja a mensagem de erro no console. Restauro o estado de antes da atualização a partir do backup?
upd-trial = Ensaio
upd-trial-starten = Iniciar ensaio
upd-trial-frage = Fazer um ensaio?
upd-trial-text = Copio o servidor para a pasta temporária, atualizo a cópia, inicio-a em outra porta e confiro se ela inicia por completo. Depois a cópia é parada e apagada.

    O seu servidor de verdade fica intacto e pode continuar funcionando. Com mundos grandes, a cópia leva alguns minutos.
upd-trial-tipp = Atualiza uma cópia na pasta temporária e a inicia em outra porta
upd-trial-err = Falha no ensaio
upd-trial-ok-titel = Ensaio aprovado
upd-trial-ok = A cópia atualizada terminou de iniciar em { $sekunden } segundos (porta { $port }).
upd-trial-timeout = A cópia não tinha terminado de iniciar após 15 minutos.
upd-trial-exit = A cópia fechou ao iniciar (código de saída { $code }).
upd-trial-meldungen = Mensagens relevantes:
upd-trial-zeilen = Últimas linhas:
upd-trial-geloescht = A cópia foi apagada.
upd-trial-nicht-geloescht = A cópia não pôde ser apagada por completo: { $ordner }
upd-ev-trial-ok = Ensaio aprovado
upd-ev-trial-fehler = Falha no ensaio
upd-jetzt = Agora:
upd-erklaerung = Verifica se todos os mods existem para a versão escolhida. Antes de cada atualização é feito um backup automaticamente.
upd-ziel = Alvo:
upd-versionen = Carregar versões
upd-pruefen = Verificar
upd-spalte-mod = Mod / datapack
upd-spalte-neu = Novo
upd-deaktivieren-fehlende = Desativar mods/datapacks sem versão compatível (senão o servidor provavelmente não inicia)
upd-deaktivieren-unbekannte = Desativar mods/datapacks desconhecidos (não estão no Modrinth)
upd-nicht-unterstuetzt = O assistente de atualização existe para Fabric, Paper, NeoForge e Vanilla.
upd-export-knopf = Exportar mods para amigos (.mrpack)
upd-export-tipp = Só para servidores Fabric e NeoForge (mods que os amigos também precisam)
upd-export-titel = Modpack salvo
upd-export-mods = { $anzahl } mods do Modrinth
upd-export-eingepackt = { $anzahl } incluídos diretamente
upd-export-hinweis = Os seus amigos importam o arquivo no app Modrinth («Adicionar arquivo»). Mods só de servidor não são instalados lá.
upd-err-export = Não foi possível exportar

## Cliente
cli-titel = Usar convite
cli-code-erklaerung-1 = Cole o código (começa com
cli-code-erklaerung-2 = ) ou abra o arquivo
cli-code-erklaerung-3 =  – o Vapafane configura o Minecraft com todos os mods adequados ao servidor.
cli-code-platzhalter = Cole o código de convite aqui (VPF1-…)
cli-pruefen = Verificar convite
cli-datei-oeffnen = Abrir arquivo de convite …
cli-pruefe-einladung = Verificando o convite …
cli-launcher-da = Launcher do Minecraft encontrado
cli-kein-launcher = Nenhum launcher do Minecraft
cli-modrinth-da = App Modrinth encontrado
cli-meine-server = Meus servidores
cli-leer = Nenhum cliente configurado – use um convite acima.
cli-kein-konto = O Vapafane não inicia o Minecraft sozinho e nunca mexe na sua conta Microsoft – «Jogar» abre o launcher oficial ou o app Modrinth.
cli-nicht-erreichbar = Servidor inacessível
cli-online = Online · { $version } · { $spieler }/{ $max } jogadores
cli-version-passt-nicht = Online, mas versão { $version } – não corresponde
cli-aktualisiert = O servidor foi atualizado ({ $version }) – peça ao administrador um código de convite novo
cli-gerade-nicht = inacessível no momento
cli-modrinth-fehlt = App Modrinth não encontrado
cli-launcher-fehlt = Launcher do Minecraft não encontrado
cli-download-seite = Ir à página de download
cli-ev-eingerichtet = Cliente configurado: { $name }
cli-gefunden = encontrado
cli-nicht-gefunden = não encontrado
cli-keine-mods = Nenhum mod necessário – só o Minecraft { $version }.
cli-optional = opcional
cli-nur-browser = só baixável no navegador (link após a instalação)
cli-nur-datei = faltando – só está no arquivo de convite
cli-launcher-fehlt-text = O launcher oficial do Minecraft não foi encontrado. Você pode configurar mesmo assim; o perfil será criado assim que o launcher estiver instalado e você apertar aqui «Atualizar».
cli-launcher-laden = Baixar launcher
cli-nie-gestartet = O launcher nunca foi aberto. Abra-o uma vez e feche; depois configure aqui.
cli-ordner-waehlen = Escolher pasta para este Minecraft
cli-mods-zusammen = { $anzahl } mods, { $groesse } no total
cli-andere-version = O servidor anuncia outra versão ({ $version }). Provavelmente foi atualizado – peça ao administrador um código de convite novo.
cli-spalte-version = Versão
cli-spalte-groesse = Tamanho
cli-einrichten-fuer = Configurar para:
cli-in-modrinth = No app Modrinth:
cli-instanz-update = Atualizar a instância existente «{ $name }» (recomendado)
cli-instanz-update-sub = Tudo fica na mesma instância: mundos, configurações, shaders, pacotes de recursos e os mods que você adicionou (p. ex. Iris). Basta clicar em «Reinstalar modpack» no app Modrinth – o Vapafane mostra como.
cli-instanz-neu = Criar instância nova
cli-instanz-neu-sub = Funciona automaticamente pela importação. Abaixo você pode trazer configurações, shaders e pacotes de recursos – mas não os mods adicionados por você. A instância antiga fica até você apagá-la.
cli-update-bleibt = Na atualização tudo fica na instância: mundos, opções, shaders, pacotes de recursos e a sua lista de servidores.
cli-import-bestaetigen = O app Modrinth abre – só confirme a importação.
cli-ordner-label = Pasta:
cli-launcher-schliessen = Feche o launcher do Minecraft antes de configurar, para que ele não sobrescreva a mudança.
cli-einrichten = Configurar
cli-uebernehmen-von = Trazer configurações de:
cli-nichts = Não trazer nada
cli-quelle-waehlen = Escolher a pasta do Minecraft de onde trazer
cli-nichts-uebernehmen = Nada a trazer
cli-teil-options = Opções e teclas
cli-teil-config = Configurações dos mods
cli-teil-shaders = Shaders
cli-teil-resourcepacks = Pacotes de recursos
cli-teil-servers = Lista de servidores
cli-teil-worlds = Mundos
cli-welten-warnung = Mundos de uma versão mais nova do Minecraft não abrem numa mais antiga. Mundos grandes deixam a configuração mais lenta.
cli-nur-fehlendes = O que já está na pasta fica intacto – só o que falta é trazido.
cli-uebernehmen-erklaerung = Por exemplo da sua instância anterior: opções, teclas, configurações de mods, shaders, pacotes de recursos e lista de servidores. Os mods vêm sempre do servidor, combinando. A origem é só lida, nunca alterada.
cli-paket-update-1 = O novo pacote Modrinth está pronto. Veja como atualizar a sua instância
cli-paket-update-2 = mundos, opções, shaders, pacotes de recursos e a sua lista de servidores ficam todos:
cli-schritt-instanz = Abra o app Modrinth e clique na instância
cli-schritt-anklicken = à esquerda.
cli-schritt-zahnrad = Clique em cima na engrenagem «Configurações da instância» e escolha a aba «Instalação».
cli-neu-installieren = Reinstalar modpack
cli-schritt-datei = clique, confirme e escolha este arquivo:
cli-modrinth-oeffnen = Abrir o app Modrinth
cli-paket-ordner = Abrir a pasta com o pacote
cli-update-sicher = O app Modrinth primeiro protege a própria instância e só troca os mods do servidor – o que você adicionou (shaders, pacotes de recursos, mods próprios) fica.
cli-paket-offen = O pacote Modrinth está pronto e o app Modrinth foi aberto com ele. Confirme lá a importação – a instância aparecerá como
cli-paket-fertig = O pacote Modrinth está pronto:
cli-paket-von-hand = O app Modrinth não pôde ser aberto automaticamente. Manualmente: no app Modrinth clique em «+» (criar instância), depois «Importar de arquivo» e escolha este arquivo.
cli-eingerichtet = Configurado: { $anzahl } mods baixados.
cli-profil = O launcher do Minecraft agora tem o perfil «{ $name }».
cli-uebernommen-paket = As configurações de «{ $quelle }» estão no pacote – o app Modrinth as aplica na importação.
cli-uebernommen = Trazido de «{ $quelle }»: { $anzahl } arquivos.
cli-uebernommen-da = ({ $anzahl } já estavam lá e ficaram intactos)
cli-uebernommen-fehler = { $anzahl } arquivos não puderam ser lidos e por isso faltam – o Minecraft ainda está aberto lá? ({ $dateien })
cli-selbst-laden = Para baixar você mesmo e colocar na pasta «mods»:
cli-spielen = Jogar
cli-spielen-modrinth = Jogar (abrir o app Modrinth)
cli-spielen-launcher = Jogar (abrir o launcher)
cli-prueft = verificando …
cli-unvollstaendig = A configuração não terminou – use de novo o convite acima (ou apague).
cli-mods-kurz = { $anzahl } mods
cli-eingerichtet-am = { $ziel } · configurado em { $datum }
cli-aktualisieren = Atualizar
cli-aktualisieren-tipp = Reconfigurar com o convite salvo – ou explicar o que fazer após uma atualização do servidor
cli-entfernen-frage = Remover «{ $name }»?
cli-entfernen-modrinth = O Vapafane esquece este cliente e apaga o seu pacote. A instância no app Modrinth fica – apague-a lá você mesmo.
cli-entfernen-launcher = A pasta com mods, configurações e mundos solo deste perfil é apagada, e o perfil some do launcher do Minecraft.
cli-upd-titel = O servidor foi atualizado
cli-upd-text = «{ $name }» agora roda Minecraft { $version }. A versão e os mods adequados vão no convite – você precisa, portanto, de um código de convite novo do administrador.
cli-upd-code = Cole-o em cima em «Usar convite» e clique em «Verificar convite».
cli-upd-modrinth = O Vapafane cria então um pacote novo e mostra como atualizar a sua instância – as suas configurações, mundos e shaders ficam.
cli-upd-launcher = O Vapafane troca então os mods – as suas configurações e mundos ficam.
cli-gespeicherte-einladung = Convite salvo de «{ $name }» carregado. «Configurar» põe tudo em ordem de novo – por exemplo se faltam mods ou o perfil sumiu. Se o administrador atualizou o servidor, você precisa é de um código de convite novo.
ext-knopf = Shaders e extras
ext-knopf-so-gehts = Shaders e extras – como fazer

## Shaders e extras
ext-titel = Adicionar shaders e extras
ext-skizze-1-alt = Esquema da instância no app Modrinth
ext-skizze-2-alt = Esquema da busca no app Modrinth
ext-skizze-leer = Nenhum conteúdo extra instalado
ext-skizze-hochladen = Enviar arquivos
ext-durchsuchen = Explorar conteúdo
ext-skizze-shader = Um shader, p. ex. Complementary
ext-skizze-beschreibung = Descrição · downloads
ext-keine-shader = Shaders precisam do mod Iris – que só roda em servidores Fabric ou NeoForge. Este servidor não tem mod loader, então pacotes de recursos funcionam aqui, shaders não.
ext-iris-modrinth = Para shaders o Minecraft precisa do mod Iris: no passo 4 escolha primeiro «Mods» e instale «Iris Shaders» (o Sodium vem junto automaticamente); depois escolha um shader em «Shaders».
ext-iris-launcher-1 = Para shaders o Minecraft precisa do mod Iris (com Sodium): baixe os dois para a versão do Minecraft do servidor em
ext-iris-launcher-2 = e coloque-os na pasta «mods».
ext-mr-einleitung-1 = Na sua instância
ext-mr-einleitung-2 = você pode adicionar a qualquer momento shaders, pacotes de recursos e mods próprios. Eles ficam nas atualizações do servidor – o app Modrinth só troca os mods do servidor.
ext-schritt-content = Escolha em cima a aba «Content» (conteúdo).
ext-schritt-durchsuchen = Clique no botão verde «Explorar conteúdo».
ext-schritt-art = Escolha em cima o tipo: «Shaders» para shaders, «Resource Packs» para pacotes de recursos ou «Mods».
ext-schritt-installieren = Clique em «Instalar» ao lado do item desejado – pronto.
ext-im-spiel = No jogo: Opções → Configurações gráficas → Pacotes de shader → escolher o shader.
ext-im-spiel-rp = No jogo: Opções → Pacotes de recursos → escolher o pacote.
ext-mods-update-hinweis = Os seus próprios mods você precisa atualizar no app após uma atualização do Minecraft; shaders e pacotes de recursos em geral não.
ext-launcher-einleitung = A pasta de jogo deste servidor é só dele: o que você adicionar lá fica nas atualizações do servidor.
ext-schritt-laden-shader = Baixe um shader, p. ex. em
ext-schritt-laden-rp = Baixe um pacote de recursos, p. ex. em
ext-nicht-entpacken = não descompacte o arquivo .zip.
ext-schritt-shaderpacks = Clique embaixo em «Abrir pasta» e coloque o arquivo na pasta «shaderpacks». Pacotes de recursos vão em «resourcepacks».
ext-schritt-resourcepacks = Clique embaixo em «Abrir pasta» e coloque o arquivo na pasta «resourcepacks».
ext-eigene-mods = Você pode colocar mods próprios na pasta «mods» – mas após uma atualização do Minecraft precisa mantê-los compatíveis você mesmo.
err-antwort-unlesbar = Resposta de «{ $befehl }» ilegível: { $fehler }

## Relatório de acessibilidade
rch-w-erreichbar = acessível
rch-w-nicht = inacessível
rch-w-nicht-dslite = inacessível (DS-Lite)
rch-w-nicht-geprueft = não verificado
rch-w-nicht-vorhanden = inexistente
rch-unmoeglich-titel = Verificação impossível no momento
rch-unmoeglich-text = O serviço de verificação na internet não responde. Tente de novo mais tarde.
rch-gruen-beide = Acessível via IPv4 e IPv6
rch-gruen-v4 = Acessível via IPv4
rch-gruen-text = Todos os amigos podem entrar.
rch-gelb-dslite-titel = DS-Lite: acessível só via IPv6
rch-gelb-dslite-text = O seu provedor compartilha o endereço IPv4. Amigos com IPv6 conseguem entrar mesmo assim – são a maioria.
rch-gelb-titel = Acessível só via IPv6
rch-gelb-text = Via IPv4 ninguém entra: ou falta a abertura da porta IPv4 (porta { $port }) ou o seu provedor compartilha o endereço IPv4 (DS-Lite). Amigos com IPv6 conseguem entrar mesmo assim.
rch-rot-titel = Inacessível de fora
rch-rot-text = Em geral falta a abertura da porta no roteador, ou o firewall do PC bloqueia a porta { $port }.
rch-rot-dslite-v6 = O seu provedor compartilha o endereço IPv4 (DS-Lite), então só o IPv6 funciona. Abra a porta { $port } para IPv6 no roteador e confira o firewall do PC.
rch-rot-dslite = O seu provedor compartilha o endereço IPv4 (DS-Lite) e o seu PC não tem endereço IPv6. Assim ninguém de fora consegue chegar ao servidor.
rch-aus-titel = O servidor não está em execução
rch-aus-text = A verificação acontece assim que o servidor estiver funcionando.
rch-port-belegt-titel = A porta { $port } já está em uso
rch-port-belegt-text = Outro programa – provavelmente outro servidor de Minecraft – usa esta porta. Assim este servidor não consegue iniciar. Defina uma porta livre em Configurações → «Porta do servidor» (p. ex. { $frei })
rch-hotspot-tipp = O servidor só é acessível via IPv6, mas a sua internet não tem IPv6. Dica: conecte o PC pelo roteador do celular (hotspot) – a rede móvel costuma ter IPv6.

## Erros do programa (geral)
err-dienst-aus = O serviço em segundo plano não está em execução
err-kein-systemd = systemd não está disponível aqui (ou modo de teste).
err-kein-server = Nenhum servidor selecionado.
err-schon-beschaeftigt = Já há algo em andamento: { $was }
err-neustart-extern = Este servidor funciona fora do Vapafane – o reinício só funciona pelo programa que o iniciou.
err-stop-knopf = Use o botão Parar na visão geral (ele salva direito).
err-start-extern = O servidor funciona fora do Vapafane. Pare-o primeiro lá.
err-start-anderer = Outro servidor está em execução agora. Pare-o primeiro.
err-stop-unmoeglich = Não foi possível parar o servidor.
err-pfad = Caminho inválido
err-rcon-antwort = Resposta RCON inválida
err-adresse = Endereço inválido
cln-alte-neoforge = versão antiga do NeoForge
cln-alte-bibliothek = versão antiga de uma biblioteca
cln-alte-mods = versões antigas de mods (postas de lado pelo painel/launcher antigo)
cln-cache = cache de mods (recriado no próximo início)
cli-datei-titel = Abrir arquivo de convite
srv-rcon-an = RCON ativado (o Vapafane precisa dele para os comandos).
err-link-verboten = Este endereço não pode ser aberto.
err-pruefsumme = A soma de verificação não confere: { $datei }
err-kein-fabric = Nenhum loader Fabric encontrado para Minecraft { $mc }
cf-err-leer = Cole primeiro uma chave.
cf-err-id = ID do CurseForge inválido
cf-err-format = Isto não parece uma chave API (cole sem espaços).
cf-err-antwort = Resposta inválida do CurseForge: { $fehler }
cf-err-abgelehnt = O CurseForge rejeita a chave API (inválida ou ainda não ativada).
cf-err-fehlt = Nenhuma chave do CurseForge salva.
kat-err-typ = «Adicionar mods» não existe para este tipo de servidor.
kat-err-keine-version = «{ $name }» não existe para Minecraft { $mc } ({ $loader }).
kat-err-cf-version = «{ $name }» não existe no CurseForge para Minecraft { $mc }.
fort-suche = Buscando { $name } …
fort-suche-cf = Buscando { $name } no CurseForge …
inv-err-neuer = Este código de convite é de uma versão mais nova do Vapafane. Atualize o Vapafane.
inv-err-unvollstaendig = O código está incompleto ou danificado – copie-o inteiro.
inv-err-kaputt = O código está danificado – copie-o de novo.
inv-err-version = Este convite é de outra versão do Vapafane.
inv-err-angaben = O convite contém dados inválidos.
inv-err-mod = O convite contém uma entrada de mod inválida.
inv-err-typ = Há convites para servidores Fabric, NeoForge, Paper e Vanilla.
inv-err-datei = Este não é um arquivo de convite Vapafane válido.
inv-err-datei-kaputt = O convite no arquivo está danificado.
inv-err-oeffentlich = Não foi possível descobrir o endereço público.
flv-err-kein-server = Não há servidor para Minecraft { $mc }.
flv-err-kein-paper = Não há Paper para { $mc }.
flv-err-kein-neoforge = Não há NeoForge para Minecraft { $mc }.
fort-neoforge = O instalador do NeoForge prepara o servidor (baixa as bibliotecas) …
fort-suche-server = Buscando { $was } para Minecraft { $mc } …
fort-pruefe-java = Verificando o Java …
fort-frage-cf = Consultando o CurseForge pelos mods restantes …
fort-pruefe-deps = Verificando dependências …
fort-pruefe-dp = Verificando datapacks …
fort-pruefe-java-platz = Verificando Java e espaço em disco …
upd-err-dateiname = Nome de arquivo inválido do Modrinth: { $name }
upd-cf-fehler = Não foi possível verificar o CurseForge: { $fehler }
upd-abhaengigkeit = (dependência)
upd-grund-fehlt = não existe para Minecraft { $mc }
cli-err-port = Porta inválida no endereço
cli-err-antwort = Resposta inválida do servidor
cli-err-profil-kaputt = { $datei } está danificado – abra o launcher uma vez.
cli-err-nicht-leer = A pasta { $ordner } não está vazia e não pertence ao Vapafane.
cli-err-datei-kaputt = O arquivo { $datei } do convite está danificado.
cli-err-fabric-profil = Perfil Fabric inválido.
cli-err-neoforge-java = O NeoForge precisa do Java { $version }. Abra o launcher do Minecraft uma vez (ele traz o Java) ou instale o Java.
cli-hinweis-launcher-starten = Abra o launcher do Minecraft uma vez e feche; depois configure de novo.
cli-hinweis-kein-launcher = O launcher oficial do Minecraft não foi encontrado. Instale-o, abra-o uma vez e depois aperte aqui «Atualizar».
cli-hinweis-nie-gestartet = O launcher nunca foi aberto. Abra-o uma vez e feche; depois «Atualizar».
cli-paket-summary = Configurado pelo Vapafane para o servidor { $name } ({ $adresse })
fort-pruefe-mods = Verificando os mods …
fort-lade = Baixando { $name } …
fort-launcher = Configurando o launcher …
fort-neoforge-install = Instalando o NeoForge { $version } (pode levar alguns minutos) …
fort-uebernehme = Trazendo as configurações …
fort-trage-ein = Adicionando o servidor …
fort-sammle = Reunindo as configurações …
fort-packe = Empacotando o pacote Modrinth …
fort-fertig = Pronto.
task-update = Atualização
task-backup = Backup (servidor completo em ZIP)
task-restore = Restaurar backup
task-client = Configurar cliente
task-rollback = Voltar ao estado de antes da atualização
task-pruefe-mods = Verificando mods para { $mc }
upd-err-erst-pruefen = Aperte primeiro «Verificar».
upd-err-anderer-server = A verificação pertence a outro servidor. Verifique de novo.
upd-err-absturz = O servidor travou ao iniciar. { $grund }
upd-err-beendet = O servidor fechou logo em seguida.
upd-err-kein-backup = Não há backup da última atualização deste servidor.
upd-err-temp-platz = Espaço insuficiente na pasta temporária para uma cópia (necessários { $groesse }).
cln-err-laeuft = A limpeza só funciona com o servidor parado.
fort-raeume-auf = Limpando arquivos antigos …
fort-loesche = Apagando arquivos antigos …
fort-loesche-kopie = Apagando a cópia …
nbt-err-unvollstaendig = servers.dat está incompleto.
nbt-err-laenge = servers.dat contém um comprimento inválido.
nbt-err-eintrag = servers.dat contém uma entrada desconhecida.
nbt-err-tiefe = servers.dat está aninhado fundo demais.
nbt-err-format = servers.dat tem um formato desconhecido.
say-backup-start = Fazendo backup …
say-backup-fertig = Backup concluído.
say-backup-fehler = Falha no backup.
say-stoppt = para
say-neustart = reinicia
say-countdown = O servidor { $was } em { $sekunden } segundos!
svc-countdown = Servidor { $was } em { $sekunden } s
svc-err-laeuft-nicht = O servidor não está em execução no momento.
svc-err-befehl = Comando desconhecido: { $befehl }
svc-err-ordner-fehlt = Pasta do servidor ausente: { $ordner }
svc-err-eula = O EULA do Minecraft ainda não foi aceito para este servidor.
svc-err-fremder-server = Nesta pasta já roda um servidor (PID { $pid }) que não foi iniciado pelo Vapafane. Pare-o primeiro (p. ex. painel antigo).
svc-err-port = A porta { $port } ({ $art }) já está em uso. Há outro servidor de Minecraft rodando neste PC?
svc-signal = encerrado por sinal
svc-beendet = encerrado
svc-abgestuerzt = Travou ({ $grund }).
svc-taegliches-backup = Backup diário
say-update-neustart = O servidor reinicia para uma atualização!
svc-err-java-start = Não foi possível iniciar o Java ({ $java }): { $fehler }

## Linha de comando
ctl-hilfe = Uso: vapafane --ctl <comando>

      status            mostrar o estado do servidor ativo
      start             iniciar o servidor (inicia o serviço em segundo plano se preciso)
      stop [SEGUNDOS]   parar o servidor direito, com contagem regressiva opcional
      restart [SEG.]    reiniciar o servidor
      wait-ready [SEG.] esperar o servidor ficar pronto
      wait-stopped [S.] esperar o servidor parar
      backup            salvar a pasta completa do servidor em ZIP
      cmd <COMANDO …>   enviar um comando via RCON
      aktiv NOME        ativar outro servidor da lista (só parado)
      neu TIPO NOME VER [PASTA]  criar um servidor novo (fabric, paper, neoforge, vanilla) e ativá-lo
      update-check VER  verificar mods/datapacks para Minecraft VER via Modrinth
      trial VER         ensaio: atualizar uma cópia, testar em outra porta, apagar
      update VER        atualização: parar, backup, trocar, iniciar (só com verificação ok)
      mods              mostrar mods e datapacks com estado ativado/desativado
      mod-an CAMINHO    ativar um mod (CAMINHO como em «mods», p. ex. mods/x.jar)
      mod-aus CAMINHO   desativar um mod (vira .jar.disabled)
      aufraeumen [--ja] mostra restos antigos; com --ja eles são apagados (servidor parado)
      mod-suche [TERMO] [--datapack]    buscar no Modrinth mods/plugins (ou datapacks) compatíveis
      mod-install PROJETO [--datapack]  instalar um mod/plugin/datapack com as dependências necessárias
      curseforge-schluessel [--loeschen]  ler a chave do CurseForge da entrada e salvá-la
      curseforge-test   testar a chave do CurseForge salva
      einladung [ENDEREÇO]              mostrar um código de convite para amigos (endereço = host:porta)
      einladung-datei ARQUIVO [ENDEREÇO] escrever um arquivo de convite (.vapafane)
      einloesen CÓDIGO|ARQUIVO [--ordner P] [--alle-optionalen] [--ziel launcher|modrinth]
                … [--einstellungen-von PASTA] [--neue-instanz]
                                        configurar um cliente para um convite (perfil do launcher ou pacote Modrinth);
                                        trazer as configurações (sem mundos) de PASTA; com --neue-instanz
                                        uma instância existente no app Modrinth não é atualizada
      clients                           mostrar os clientes configurados
      client-entfernen PASTA            apagar um cliente e o seu perfil do launcher
      ping ENDEREÇO                     consultar o estado do servidor (como a lista de servidores no jogo)
      erreichbarkeit    verificar se o servidor é acessível de fora (IPv4/IPv6, DS-Lite)
      mrpack ARQUIVO    exportar os mods como modpack Modrinth (.mrpack) para amigos
      quit-service      encerrar o serviço em segundo plano (para o servidor antes)
ctl-fehler = Erro: { $fehler }
