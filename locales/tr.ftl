# Vapafane – Türkçe (yapay zekâ ile otomatik çeviri, düzeltmeler memnuniyetle karşılanır)

## Dil
sprache-titel = Dil
sprache-system = Sistem dili (önerilir)
sprache-ki-hinweis = Bu çeviri otomatik olarak (yapay zekâ ile) oluşturuldu – hataları „Vapafane Hakkında” üzerinden bildir.
sprache-neustart = Değiştirince arayüz kısaca yeniden yüklenir.

## Java
java-hinweis-windows = Örn. https://adoptium.net adresinden kur (Temurin { $version }).
java-hinweis-linux = Şu komutla kur: sudo apt install openjdk-{ $version }-jre-headless
err-java-fehlt = Bu bilgisayarda Java kurulu değil – Java olmadan hiçbir Minecraft sunucusu başlayamaz. Bu sunucu Java { $version } gerektiriyor. { $hinweis }
err-java-noetig = Minecraft { $mc }, kurulu olmayan Java { $version } sürümünü gerektiriyor. { $hinweis }

## Ortak düğmeler
gem-ok = Tamam
gem-schliessen = Kapat
gem-abbrechen = İptal
gem-weiter = Devam
gem-entfernen = Kaldır
gem-speichern = Kaydet
task-abgebrochen = İptal edildi

## Başlık
hdr-untertitel = Sunucu ve istemci için Minecraft atölyesi
hdr-bereich = Alan
hdr-client = İstemci
hdr-server = Sunucu
hdr-aktiver-server = Etkin sunucu (aynı anda yalnızca biri çalışır)
hdr-kein-server = (sunucu yok)
hdr-plus-erster = İlk sunucunu burada oluştur
hdr-plus = Sunucu ekle
hdr-menu-neu = Yeni sunucu oluştur (Fabric, Paper, NeoForge, Vanilla) …
hdr-menu-ordner = Var olan bir klasörü ekle …
hdr-menu-import = ZIP/yedekten sunucu içe aktar …
hdr-menu-entfernen = Seçili sunucuyu listeden kaldır
hdr-einstellungen = Ayarlar
hdr-hell-dunkel = Açık/koyu arasında geçiş yap
hdr-ev-gewechselt = Sunucu değiştirildi: { $name }
hdr-err-umschalten = Geçiş yapılamadı

## Kenar çubuğu
side-bereiche = Bölümler
side-betrieb = İşletim
side-uebersicht = Genel bakış
side-konsole = Konsol
side-spieler = Oyuncular
side-inhalte = İçerik
side-mods = Modlar/Eklentiler
side-updates = Sürüm ve güncellemeler
side-backups = Yedekler
side-netzwerk = Ağ
side-erreichbarkeit = Erişilebilirlik

## Hoş geldin
willkommen-titel = Vapafane’e hoş geldin
willkommen-text = Vapafane resmî bir Minecraft ürünü değildir. Mojang veya Microsoft tarafından onaylanmamıştır ve onlarla bağlantılı değildir.

    Sorular, fikirler veya hatalar: projenin GitHub sayfası üzerinden (Ayarlar → Vapafane Hakkında).

## Ortak
gem-ordner = Klasör
gem-ordner-oeffnen = Klasörü aç
gem-launcher = Resmî Minecraft Launcher
gem-modrinth-app = Modrinth uygulaması

## Vapafane Hakkında
about-titel = Vapafane Hakkında
about-version = Sürüm { $version }
about-testmodus = TEST MODU
about-nicht-offiziell = Vapafane resmî bir Minecraft ürünü değildir. Mojang veya Microsoft tarafından onaylanmamıştır ve onlarla bağlantılı değildir.
about-marken = Minecraft, Mojang/Microsoft’un markasıdır. Modlar ve eklentiler kendi yapımcılarından gelir (Modrinth veya CurseForge üzerinden).
about-kontakt = İletişim ve hata bildirme (GitHub) ↗
about-projektseite = Proje sayfası ↗

## Görünüm
skin-titel = Görünüm
skin-grass-sub = Açık veya koyu (sağ üstteki düğme)
skin-obsidian-sub = Her zaman koyu

## İstemci ayarları
cset-einstellung = Ayar
cset-einrichten = Kurulum
cset-ziel = Varsayılan hedef
cset-ziel-sub = Yeni sunucuların kurulduğu yer
cset-spielordner = Tüm istemcilerin oyun klasörleri
cset-spielordner-sub = Her sunucunun modları, seçenekleri ve dünyaları burada

## Yedekler
bak-restore-frage = „{ $name }” { $datei } tarihindeki durumuna geri alınsın mı?
bak-restore-text = Bunun için sunucu durdurulacak. Şu anki durum silinmez – yanında „…_vor_Wiederherstellung_<tarih>” adlı bir klasörde kalır.
bak-wiederherstellen = Geri yükle
bak-ev-wiederhergestellt = Yedek geri yüklendi
bak-restore-fertig-titel = Geri yüklendi
bak-restore-fertig = Sunucu { $label } tarihindeki durumuna döndü. Şimdi başlatabilirsin.
bak-restore-daneben = Önceki durum şurada:
    { $ordner }
    (her şey yolundaysa silinebilir)
bak-err-restore = Geri yükleme başarısız
bak-waehlen = Yedek seç
bak-erklaerung = Her yedek, adında tarih bulunan, sunucu klasörünün tamamının ZIP’idir. { $ordner } içine kaydedilir. En yeni { $anzahl } yedek saklanır, eskiler silinir.
bak-jetzt = Şimdi yedek al
bak-ordner = Yedek klasörünü aç
bak-andere = Başka bir ZIP geri yükle …
bak-aufheben = Sakla:
bak-leer = Bu sunucunun henüz yedeği yok.

## Erişilebilirlik
rch-fehlgeschlagen = Denetim başarısız
rch-noch-nicht = Henüz denetlenmedi
rch-kein-server = Sunucu seçilmedi
rch-kein-server-text = Üstteki „+” ile bir sunucu oluştur veya birini seç; denetim o zaman başlar.
rch-prueft = Denetleniyor …
rch-geprueft-um = { $zeit } saatinde denetlendi
rch-ip-erklaerung = IPv4 klasik internet adresidir; IPv6, her cihaza kendi adresini veren yenisidir. DS-Lite’ta sağlayıcın IPv4 adresini paylaştırır – o zaman yalnızca IPv6 çalışır.
rch-erneut = Yeniden denetle
rch-portfreigabe = Port açma rehberi ↗
rch-dienst-hinweis = Denetim için Vapafane, internetteki bir hizmete (ifconfig.co) sunucu portunun dışarıdan açık olup olmadığını sorar. Kapatmak için: Ayarlar → Otomasyon.

## Arkadaşları davet et
inv-titel = Arkadaşları davet et
inv-erklaerung = Arkadaşlarına bir kod gönder – Vapafane onların Minecraft’ını uygun modlarla kurar.
inv-adresse-label = Arkadaşlarının sunucuya ulaştığı adres
inv-oeffentlich = Genel adresi bul
inv-adresse-hinweis = Yerel ağ adresi önceden doldurulmuştur (yalnızca aynı Wi-Fi). Dışarıdaki arkadaşlar için: genel adresi bul ve yönlendiricide sunucu portunu aç.
inv-code-erzeugen = Kod oluştur
inv-als-datei = Dosya olarak kaydet …
inv-kein-passwort = Kod parola içermez – yalnızca adres, sürüm ve modlar.
inv-datei-gespeichert = Kaydedildi:
inv-datei-schicken = bu dosyayı arkadaşlarına gönder.
inv-code-label = Davet kodu ({ $zeichen } karakter) – kopyala ve örn. mesajla gönder:
inv-kopieren = Kopyala
inv-kopiert = Kopyalandı
inv-mods = Oyuncular için { $anzahl } mod
inv-mods-optional = (bunların { $anzahl } tanesi isteğe bağlı)
inv-keine-mods = Mod gerekmiyor – yalnızca { $version } sürümü.
inv-ipv6 = IPv6 adresin { $adresse } de içinde.
inv-nur-ipv6 = – son denetime göre sunucuna yalnızca IPv6 üzerinden erişilebiliyor.
inv-nur-server = Yalnızca sunucu tarafında (oyunculara gerekmez): { $mods }
inv-datei-noetig = Bu modlar ne Modrinth’te ne CurseForge’da var: { $mods }. Koda sığmazlar – „Dosya olarak kaydet” seçeneğini kullan, orada dahil edilirler.

## Oyuncular ve beyaz liste
spi-online-titel = Şu anda çevrim içi
spi-wl-titel = Beyaz liste (kim girebilir)
spi-wl-erklaerung = Beyaz liste açıkken (Ayarlar → „Yalnızca beyaz liste”) sunucuya yalnızca burada listelenen oyuncular girebilir. Bu, yabancılardan korur – kendini de ekle.
spi-name-platzhalter = Minecraft adı
spi-hinzufuegen = Ekle
spi-name-ungueltig = Geçerli bir Minecraft adı gir (3–16 karakter).
spi-nur-laufend = Ekleme yalnızca sunucu çalışırken olur (adı Mojang’dan doğrular).
spi-wl-leer = Beyaz liste boş.
spi-niemand = Kimse çevrim içi değil
spi-kreativ = Yaratıcı
spi-ueberleben = Hayatta kalma
spi-op = OP ver
spi-op-weg = OP’yi al
spi-kick = At
spi-kick-frage = { $name } atılsın mı?
spi-kick-text = Hemen geri girebilir.
spi-err-befehl = Komut başarısız
wl-titel = Beyaz liste
wl-einschalten = Beyaz listeyi aç
wl-zu-spielern = Oyunculara git
wl-warnung-offen = Dikkat: sunucuna internetten erişilebiliyor ve beyaz liste kapalı – adresi bilen veya bulan herkes girebilir. Beyaz listeyle yalnızca senin eklediğin oyuncular girebilir.
wl-warnung-leer = Beyaz liste açık ama henüz boş – şu anda kimse giremez, sen bile. „Oyuncular” bölümünde kendi Minecraft adını ve arkadaşlarınınkileri ekle.
wl-tipp = İpucu: beyaz liste kapalı – adresi olan herkes girebilir. Arkadaşlar dışarıdan oynamaya başlar başlamaz önerilir: o zaman yalnızca buradaki oyuncular girebilir.

## Konsol
kon-neues-log = yeni günlük dosyası (sunucu yeniden başlatıldı)
kon-beigetreten = { $name } katıldı
kon-verlassen = { $name } oyundan ayrıldı
kon-bereit = Sunucu hazır ({ $sekunden } sn’de başladı)
kon-fehler = Hata: { $fehler }
kon-tag = Gündüz
kon-nacht = Gece
kon-wetter = Açık hava
kon-wer-online = Kim çevrim içi?
kon-platzhalter = Komut yaz, örn. say Merhaba  ·  time set day  ·  weather clear
kon-senden = Gönder
kon-log-ordner = Günlük klasörünü aç

## Genel bakış
ueb-laeuft = Çalışıyor
ueb-startet = Başlatılıyor …
ueb-stoppt = Durduruluyor …
ueb-neustart-absturz = Çökme sonrası yeniden başlatma …
ueb-gestoppt = Durduruldu
ueb-abgestuerzt = Çöktü
ueb-extern = Vapafane dışında çalışıyor
ueb-anderer = Başka bir sunucu çalışıyor
ueb-starten = Başlat
ueb-stoppen = Durdur
ueb-neustart-knopf = Yeniden başlat
ueb-seit = { $dauer } süredir
ueb-neustarts = otomatik olarak { $anzahl }× yeniden başlatıldı
ueb-hinweis-extern = Sunucu Vapafane üzerinden başlatılmadı (örn. eski panel veya terminal). Konsol ve oyuncular yine de çalışır; „Durdur” ona stop komutunu gönderir.
ueb-hinweis-anderer = Arka plan hizmeti başka bir sunucu çalıştırıyor. Önce onu durdur.
ueb-hinweis-absturz = Sunucu çöktü ve yeniden başlatılmadı. Konsoldaki son hata iletisine bak.
ueb-hinweis-kein-server = Üstteki „＋” ile bir sunucu ekle.
ueb-ev-bereit = Sunucu hazır
ueb-ev-gestoppt = Sunucu durduruldu
ueb-ev-startet = Sunucu başlatılıyor
ueb-ev-neustart = Sunucu yeniden başlıyor
ueb-ev-stoppt = Sunucu duruyor
ueb-ev-in-sek = { $sekunden } sn içinde
ueb-ev-absturz = Sunucu çöktü
ueb-ev-absturz-neustart = Sunucu çöktü – otomatik yeniden başlatma
ueb-notify-absturz = Minecraft sunucusu çöktü
ueb-notify-absturz-neustart = Otomatik olarak yeniden başlıyor.
ueb-ev-backup-start = Yedekleme başladı
ueb-ev-backup-fertig = Yedek tamamlandı: { $datei } ({ $groesse })
ueb-err-start = Başlatılamadı
ueb-err-stop = Durdurulamadı
ueb-err-backup = Yedekleme başarısız
ueb-eula-titel = EULA’yı onayla
ueb-eula-text = Minecraft EULA’sı kabul edilmeden hiçbir sunucu başlamaz.
ueb-eula-akzeptiere = Kabul ediyorum:
ueb-eula-fuer = („{ $name }” için).
ueb-verb-stoppen = durdurulsun
ueb-verb-neustart = yeniden başlatılsın
ueb-stop-frage = Sunucu { $verb } mu?
ueb-stop-extern-text = Sunucu Vapafane dışında çalışıyor. Ona „stop” komutunu gönderiyorum – dünyayı kaydedip durur.
ueb-stop-niemand = Şu anda kimse çevrim içi değil.
ueb-stop-spieler = { $anzahl } oyuncu çevrim içi: { $namen }
ueb-sofort = Hemen
ueb-mit-warnung = { $sekunden } sn sonra uyarıyla
ueb-karte-spieler = Çevrim içi oyuncular
ueb-karte-tps = TPS (saniyedeki tick)
ueb-karte-ram = Bellek
ueb-karte-ordner = Sunucu klasörü
ueb-karte-aussen = Dışarıdan
ueb-karte-adresse = Adres (yerel ağ)
ueb-karte-backup = Son yedek
ueb-karte-version = Sürüm
ueb-server-aus = sunucu kapalı
ueb-keine-verbindung = bağlantı yok
ueb-niemand = kimse yok
ueb-ms-pro-tick = tick başına { $ms } ms
ueb-ruckelt = takılıyor
ueb-fluessig = akıcı
ueb-von-kernen = / { $anzahl } çekirdek
ueb-ram-belegt-limit = kullanımda (sınır { $limit } GB)
ueb-ram-belegt = kullanımda
ueb-ram-limit = sınır { $limit } GB
ueb-frei = { $groesse } boş
ueb-vor = { $dauer } önce
ueb-noch-keins = henüz yok
ueb-backup-knopf = „Backup” düğmesine bas
ueb-backup-sub = { $groesse } · { $anzahl } saklanıyor
ueb-erreichbar = Erişilebilir
ueb-nur-ipv6 = Yalnızca IPv6
ueb-nicht-erreichbar = Erişilemiyor
ueb-ipv4-und-6 = IPv4 ve IPv6
ueb-ueber-ipv4 = IPv4 üzerinden
ueb-portfreigabe = port açılımını denetle
ueb-ereignisse = Son olaylar
ueb-nichts-passiert = Henüz bir şey olmadı.
ueb-fenster-zu = Pencereyi kapatmak sunucuyu durdurmaz – arka planda çalışmaya devam eder.
ueb-kontakt = İletişim ve hata bildirme (GitHub)

## Ayarlar (sunucu)
set-spiel = Oyun (bu sunucu)
set-motd = Sunucu adı (MOTD)
set-max-spieler = Azami oyuncu
set-schwierigkeit = Zorluk
set-wert-peaceful = Barışçıl
set-wert-easy = Kolay
set-wert-normal = Normal
set-wert-hard = Zor
set-spielmodus = Yeni oyuncuların oyun modu
set-wert-survival = Hayatta kalma
set-wert-creative = Yaratıcı
set-wert-adventure = Macera
set-wert-spectator = İzleyici
set-sichtweite = Görüş mesafesi (chunk)
set-simulationsweite = Simülasyon mesafesi (chunk)
set-spawn-schutz = Doğma noktası koruması (blok)
set-port = Sunucu portu
set-whitelist = Yalnızca beyaz liste
set-whitelist-hilfe = Güvenlik için önerilir: yalnızca beyaz listedeki oyuncular girebilir („Oyuncular” sayfası). Onsuz, adresini bilen herkes girebilir.
set-leistung = Performans (bu sunucu)
set-ram = Sunucu için azami bellek (GB)
set-ram-hinweis = Bu bilgisayarda { $gb } GB var. Sisteme yaklaşık 3–4 GB bırak.
set-speichern = Ayarları kaydet
set-gespeichert = Kaydedildi.
set-gespeichert-neustart = Kaydedildi. Bir sonraki yeniden başlatmada geçerli olur.
set-err-speichern = Kaydedilemedi
set-automatik = Otomasyon
set-autostart = Bilgisayar açılınca etkin sunucuyu otomatik başlat
set-auto-testmodus = Test modunda devre dışı.
set-auto-kein-systemd = Bu sistemde kullanılamıyor (systemd yok).
set-auto-windows = Bilgisayar açılınca, oturum açılmasa bile başlar (Windows etkinleştirirken yönetici izni ister).
set-auto-linger-fehlt = Dikkat: yalnızca sen oturum açınca başlar. Oturum açmadan başlatmaya izin vermek için buraya tıkla.
set-auto-aktiv = Oturum açılmasa bile başlar ve çökmeden sonra kendini yeniden başlatır.
set-linger-frage = Oturum açmadan başlatmaya izin verilsin mi?
set-linger-text = Bunun için systemd, kullanıcı hizmetlerini açılışta başlatabilmeli („loginctl enable-linger”). Sonrasında sunucu kimse oturum açmasa da çalışır.
set-linger-erlauben = İzin ver
set-linger-abgelehnt = İzin verilmedi
set-fenster-autostart = Oturum açınca bu pencereyi aç
set-backup-um = Günlük yedek saati:
set-backup-uhr =  (sunucunun tamamı ZIP olarak)
set-backup-sub = Arka plan hizmeti yapar. O saatte bilgisayar kapalıysa sonradan telafi edilir.
set-uhrzeit = Günlük yedeğin saati
set-reach = Dışarıdan erişilebilirliği denetle
set-reach-sub = Sunucu alanına geçerken. Bunun için internetteki bir denetim hizmetine sorar.
set-notify = Bildirimler
set-notify-sub = Biri gelip gittiğinde ve sunucu çöktüğünde (pencere açık olduğu sürece). Sessiz.
set-ordner = Klasörler
set-ordner-server = Sunucu klasörü
set-an = açık
set-aus = kapalı
set-panel-titel = Hâlâ eski bir denetim paneli kurulu.
set-panel-status = „minecraft.service” hizmeti: { $dienst }, açılışta { $start }; günlük yedek programı { $backup }. O çalıştıkça Vapafane sunucuyu kendisi başlatamaz (aynı port) ve yedekler iki kez alınırdı.
set-panel-knopf = Eski paneli kapat …
set-panel-frage = Eski panel kapatılsın mı?
set-panel-text = „minecraft.service” hizmetini durduruyorum (sunucu düzgünce kaydedip kapanır) ve onu ve günlük yedeğini bilgisayar açılışında devre dışı bırakıyorum.

    Eski panelin kendisi ve dosyaları kalır – her zaman yeniden açılabilir. Sonra sunucuyu burada „Başlat” ile başlatırsın.
set-panel-abschalten = Kapat
set-panel-aus = Eski panel kapatıldı
set-panel-err = Kapatılamadı

## CurseForge
cf-titel = CurseForge (isteğe bağlı)
cf-experimentell = Deneysel
cf-brauche = Buna ihtiyacım var mı?
cf-erklaerung = Hayır – modların çoğu, hiçbir ayar gerektirmeden çalışan Modrinth’te. CurseForge ek olarak yalnızca orada bulunan modları bulur ve bunun için kişisel bir API anahtarı (bir tür erişim kodu) gerektirir.
cf-anleitung-titel = Anahtar nasıl alınır (ücretsiz)
cf-schritt-1 = „Anahtar başvurusu” düğmesine tıkla ve formu doldur (örn. amaç: „Kendi sunucum için modlar”).
cf-schritt-2 = E-posta yanıtını bekle – CurseForge elle denetler, birkaç gün sürebilir.
cf-schritt-3 = Anahtarı aşağıya yapıştır, „Anahtarı dene” ve ardından „Kaydet”.
cf-persoenlich = Anahtar kişiseldir
cf-persoenlich-text = – paylaşma ve yayımlama. Yalnızca bu bilgisayarda kalır. Şunlar geçerlidir:
cf-bedingungen = üçüncü taraf programlar için CurseForge koşulları
cf-experimentell-text = Deneysel: CurseForge, özel araçlara anahtar verip vermemeye kendisi karar verir.
cf-beantragen = Anahtar başvurusu ↗
cf-konsole = CurseForge konsolu ↗
cf-mehr = CurseForge API’si hakkında daha fazla ↗
cf-platzhalter = API anahtarını buraya yapıştır
cf-testen = Anahtarı dene
cf-teste = Deneniyor …
cf-funktioniert = Anahtar çalışıyor.
cf-schluessel-da = Anahtar kayıtlı: { $maskiert } – aramalara CurseForge da dahil.
cf-kein-schluessel = Kayıtlı anahtar yok – yalnızca Modrinth’te aranır.
cf-gespeichert = CurseForge anahtarı kaydedildi
cf-entfernen-frage = CurseForge anahtarı silinsin mi?
cf-entfernen-text = Sonrasında Vapafane yine yalnızca Modrinth’te arar.

## Sunucu ekleme
gem-anderer-ordner = Başka klasör …
srv-typ-fabric = Fabric (modlar)
srv-typ-paper = Paper (eklentiler)
srv-typ-neoforge = NeoForge (modlar)
srv-typ-vanilla = Vanilla (modsuz)
srv-name-label = Sunucu listesindeki ad
srv-typ-label = Sunucu türü
srv-version-label = Minecraft sürümü
srv-laedt = yükleniyor …
srv-keine-verbindung = Bağlantı yok: { $fehler }
srv-ordner-waehlen = Sunucu için klasör seç
srv-server-ordner-waehlen = Sunucu klasörünü seç
srv-backup-waehlen = Sunucu yedeği seç (ZIP veya tar)
srv-ordner-nicht-leer = Seçilen klasör boş değil – sunucu yeni bir alt klasöre kurulacak.
srv-ordner-ungueltig = Bu klasör kullanılamıyor.
srv-eula-zusatz = (bunsuz hiçbir sunucu başlamaz)
srv-hinweis = Not
srv-ev-hinzu = Sunucu eklendi: { $name }
srv-hinzu-titel = Sunucu ekle
srv-hinzu-text = „{ $name }” listede. Geçiş yapmak için çalışan sunucuyu durdur.
srv-err-hinzu = Eklenemedi
srv-kein-server = Sunucu bulunamadı
srv-import-titel = Sunucu içe aktar
srv-err-import = İçe aktarılamadı
srv-neu-titel = Yeni sunucu oluştur
srv-neu-name = Sunucum
srv-err-erstellen = Oluşturulamadı
srv-entfernen-frage = „{ $name }” listeden kaldırılsın mı?
srv-entfernen-text = { $ordner } içindeki dosyalar kalır, yalnızca kayıt silinir.
srv-err-entfernen = Kaldırılamadı

## Modlar/Eklentiler
mods-erklaerung = Kapatılan modlar klasörde kalır ama yüklenmez. Değişiklikler sunucunun bir sonraki başlangıcında geçerli olur.
mods-ev-an = { $name } açıldı
mods-ev-aus = { $name } kapatıldı
mods-neustart-noetig = Sunucu çalışıyor – değişiklik bir sonraki yeniden başlatmadan sonra geçerli olur.
mods-nur-client = yalnızca istemci
mods-aus-grund = Kapalı: { $grund }
mods-an-tipp = Açık – kapatmak için tıkla
mods-aus-tipp = Kapalı – açmak için tıkla
mods-anzahl = { $gesamt } { $wort } içinden { $an } tanesi açık
mods-datapacks-anzahl = { $anzahl } datapack
mods-leer = „{ $ordner }” klasöründe { $wort } yok.
mods-datapacks-leer = Dünyada datapack yok.
mods-datapacks-titel = Dünyadaki datapack’ler
mods-hinzufuegen = { $wort } ekle …
mods-hinzufuegen-titel = { $wort } ekle
mods-ordner-oeffnen = Mod klasörünü aç
mods-aufraeumen = Temizle …
mods-aufraeumen-titel = Temizlik
mods-aufraeumen-tipp = Eski sürüm kalıntılarını sil (sunucu durdurulmuş olmalı)
mods-alles-sauber = Her şey temiz – eski sürüm kalıntısı yok.
mods-aufraeumen-text-1 = Şu eski sürüm kalıntıları silinecek (toplam
mods-aufraeumen-text-2 = Dünya, ayarlar ve kapalı modlar kalır. Sunucunun hâlâ ihtiyaç duyduklarını bir sonraki başlangıçta yeniden indirir.
mods-erst-stoppen = Silmek için önce sunucuyu durdur.
mods-loeschen-knopf = { $groesse } sil
mods-ev-aufgeraeumt = Temizlendi: { $groesse } boşaltıldı
mods-aufgeraeumt-titel = Temizlendi
mods-aufgeraeumt = { $anzahl } öğe silindi, { $groesse } boşaltıldı.
mods-nicht-loeschbar = Silinemedi:
mods-err-aufraeumen = Temizlik başarısız

## Katalog (mod ekleme)
kat-suche = Modrinth’te ara … (boş = en popülerler)
kat-dp-option = Datapack’ler (dünya için)
kat-treffer = Minecraft { $mc } için { $anzahl } uygun { $wort }
kat-beliebtheit = popülerliğe göre sıralı
kat-sucht = Aranıyor …
kat-nichts = Bir şey bulunamadı.
kat-mehr = Daha fazla yükle
kat-cf-tipp = İpucu: CurseForge anahtarıyla (Ayarlar) ek olarak yalnızca orada bulunan modları da bulursun.
kat-nur-cf = Yalnızca CurseForge’da
kat-cf-erklaerung = Bunlar Modrinth’te yok. Bazı yapımcılar başka programlarla indirmeye izin vermez – o zaman dosyayı tarayıcıda indirir ve klasöre kendin koyarsın.
kat-beides = Sunucu + istemci
kat-beides-tipp = Oyuncuların da buna ihtiyacı var – örn. „Arkadaşlar için modları dışa aktar (.mrpack)” üzerinden
kat-nur-server = Yalnızca sunucu
kat-meta = { $downloads } indirme · yapan: { $autor }
kat-laedt = Yükleniyor …
kat-installiert = Kurulu
kat-installieren = Kur
kat-ev-installiert = Kuruldu: { $name }
kat-err-install = Kurulum olmadı: { $fehler }
kat-ist-installiert = kurulu („{ $ordner }” içinde)
kat-abhaengigkeiten = Bağımlılık olarak eklendi: { $namen }.
kat-schon-da = Zaten vardı: { $namen }.
kat-fehlende = Dikkat: bu bağımlılık bu sürüm için yok: { $namen } – sunucu böyle başlamayabilir.
kat-client-noetig = Oyuncuların da bu moda ihtiyacı var – en kolayı „Sürüm ve güncellemeler” altındaki „Arkadaşlar için modları dışa aktar (.mrpack)”.
kat-manuell-1 = Yapımcı başka programlarla indirmeye izin vermiyor. Dosyayı
kat-manuell-2 = tarayıcıda indir ve „{ $ordner }” klasörüne koy.
kat-browser = Tarayıcıda indir ↗
kat-dp-ordner = Datapack klasörünü aç
kat-neustart = Sunucu çalışıyor – bu, bir yeniden başlatmadan sonra geçerli olur.
kat-jetzt-neustarten = Şimdi yeniden başlat

## Sürüm ve güncellemeler
upd-st-aktuell = güncel
upd-st-fehlt = eksik
upd-st-unbekannt = bilinmiyor
upd-st-neu = yeni (gerekli)
upd-st-manuell = yalnızca tarayıcıda güncellenir (CurseForge)
upd-st-reenable = yeniden uyumlu – açılacak
upd-st-doppelt = yinelenen (mod olarak zaten var)
upd-alles-aktuell = Her şey güncel.
upd-zusammenfassung = { $updates } güncelleme, { $aktuell } zaten güncel, { $fehlend } uygun sürümü yok, { $unbekannt } bilinmiyor
upd-neu-gebraucht = { $anzahl } yeni gerekli
upd-passen-wieder = { $anzahl } yeniden uyumlu
upd-doppelt = { $anzahl } yinelenen (kenara alınacak)
upd-server-stand = Minecraft { $mc } için yeni sunucu derlemesi: { $label }.
upd-beta-hinweis = Not: şimdilik yalnızca test/beta sürümü var.
upd-wechsel = Sunucu: Minecraft { $von } → { $nach } ({ $label }).
upd-welt-warnung = Dikkat: yeni sürümle bir kez çalışan dünya geri alınamaz. Tek geri dönüş, otomatik alınan yedektir.
upd-java-da = Java { $version } gerekli – mevcut ({ $pfad }).
upd-java-fehlt = Java { $version } gerekli ama kurulu değil. { $hinweis }
upd-manuell-hinweis = „Yalnızca tarayıcıda güncellenir” modlarında yapımcı otomatik indirmeye izin vermiyor – yeni dosyayı CurseForge’dan indir ve kendin değiştir.
upd-platz = Yedek için disk alanı yetersiz ({ $frei } boş, sunucu { $server }).
upd-nur-mods = { $version } (yalnızca modları/eklentileri güncelle)
upd-keine-neuere = Daha yeni kararlı bir Minecraft sürümü yok.
upd-keine-verbindung = Bağlantı yok
upd-err-pruefen = Denetim başarısız
upd-rollback-frage = Son güncellemeden önceki duruma dönülsün mü?
upd-rollback-text = Sunucu durdurulur ve güncellemede alınan yedekten geri yüklenir. Şu anki durum yanında „…_vor_Wiederherstellung_<tarih>” klasöründe kalır.
upd-zuruecksetzen = Geri al
upd-ev-rollback = Güncelleme öncesi durum geri yüklendi
upd-rollback-fertig-titel = Geri alındı
upd-rollback-fertig = Sunucu { $label } tarihindeki durumuna döndü. Şimdi başlatabilirsin.
upd-rollback-daneben = Güncelleme sonrası durum şurada:
upd-err-rollback = Geri alınamadı
upd-rollback-knopf = Son güncellemeden önceki duruma dön
upd-was-mods = Modları güncelle
upd-ablauf = Akış: sunucuyu durdur (biri çevrim içiyse uyarıyla) → tam yedek → { $anzahl } dosyayı indir ve doğrula → değiştir → sunucuyu başlat ve denetle.
upd-werden-deaktiviert = Uygun sürümü olmayan { $anzahl } mod/datapack kapatılacak.
upd-probelauf-tipp = İpucu: „Deneme” ile bunu bir kopyada risksizce test edersin.
upd-start-frage = Güncelleme başlatılsın mı? ({ $was })
upd-starten = Güncellemeyi başlat
upd-ev-gestartet = Güncelleme başlatıldı ({ $was })
upd-err-update = Güncelleme başarısız
upd-ev-fertig = Güncelleme tamamlandı ({ $was })
upd-fertig-titel = Güncelleme tamamlandı
upd-fertig-1 = Sunucu yeniden çalışıyor ({ $was }).
upd-fertig-backup = Eski durumun yedeği:
upd-fertig-aufgeraeumt = Temizlendi: { $anzahl } eski dosya/klasör silindi ({ $groesse }).
upd-fertig-2 = Uygun sürümü olmayan modlar kapalı ve „Modlar/Eklentiler” altında görünür.
upd-fehlgeschlagen-titel = Güncelleme olmadı
upd-fehlgeschlagen-frage = Konsoldaki hata iletisine bak. Güncelleme öncesi durumu yedekten geri yükleyeyim mi?
upd-trial = Deneme
upd-trial-starten = Denemeyi başlat
upd-trial-frage = Deneme yapılsın mı?
upd-trial-text = Sunucuyu geçici klasöre kopyalıyorum, kopyayı güncelliyorum, başka bir portta başlatıyorum ve tamamen açıldığını denetliyorum. Sonra kopya durdurulup silinir.

    Gerçek sunucun el değmeden kalır ve çalışmaya devam edebilir. Büyük dünyalarda kopyalama birkaç dakika sürer.
upd-trial-tipp = Geçici klasördeki bir kopyayı günceller ve başka bir portta başlatır
upd-trial-err = Deneme başarısız
upd-trial-ok-titel = Deneme başarılı
upd-trial-ok = Güncellenen kopya { $sekunden } saniyede açıldı (port { $port }).
upd-trial-timeout = Kopya 15 dakika sonra hâlâ açılmamıştı.
upd-trial-exit = Kopya başlarken kapandı (çıkış kodu { $code }).
upd-trial-meldungen = Dikkat çeken iletiler:
upd-trial-zeilen = Son satırlar:
upd-trial-geloescht = Kopya silindi.
upd-trial-nicht-geloescht = Kopya tamamen silinemedi: { $ordner }
upd-ev-trial-ok = Deneme başarılı
upd-ev-trial-fehler = Deneme başarısız
upd-jetzt = Şu an:
upd-erklaerung = Tüm modların seçilen sürüm için var olup olmadığını denetler. Her güncellemeden önce otomatik yedek alınır.
upd-ziel = Hedef:
upd-versionen = Sürümleri yükle
upd-pruefen = Denetle
upd-spalte-mod = Mod / datapack
upd-spalte-neu = Yeni
upd-deaktivieren-fehlende = Uygun sürümü olmayan modları/datapack’leri kapat (yoksa sunucu büyük olasılıkla başlamaz)
upd-deaktivieren-unbekannte = Bilinmeyen modları/datapack’leri kapat (Modrinth’te yok)
upd-nicht-unterstuetzt = Güncelleme yardımcısı Fabric, Paper, NeoForge ve Vanilla için var.
upd-export-knopf = Arkadaşlar için modları dışa aktar (.mrpack)
upd-export-tipp = Yalnızca Fabric ve NeoForge sunucuları için (arkadaşların da ihtiyaç duyduğu modlar)
upd-export-titel = Mod paketi kaydedildi
upd-export-mods = Modrinth’ten { $anzahl } mod
upd-export-eingepackt = { $anzahl } doğrudan pakete kondu
upd-export-hinweis = Arkadaşların dosyayı Modrinth uygulamasında içe aktarır („Dosya ekle”). Yalnızca sunucu tarafı modlar orada kurulmaz.
upd-err-export = Dışa aktarılamadı

## İstemci
cli-titel = Davet kullan
cli-code-erklaerung-1 = Kodu yapıştır (
cli-code-erklaerung-2 =  ile başlar) veya dosyayı aç (
cli-code-erklaerung-3 = ) – Vapafane, Minecraft’ı sunucuya uyan tüm modlarla kurar.
cli-code-platzhalter = Davet kodunu buraya yapıştır (VPF1-…)
cli-pruefen = Daveti denetle
cli-datei-oeffnen = Davet dosyasını aç …
cli-pruefe-einladung = Davet denetleniyor …
cli-launcher-da = Minecraft Launcher bulundu
cli-kein-launcher = Minecraft Launcher yok
cli-modrinth-da = Modrinth uygulaması bulundu
cli-meine-server = Sunucularım
cli-leer = Kurulu istemci yok – yukarıdan bir davet kullan.
cli-kein-konto = Vapafane, Minecraft’ı kendisi başlatmaz ve Microsoft hesabına asla dokunmaz – „Oyna”, resmî launcher’ı veya Modrinth uygulamasını açar.
cli-nicht-erreichbar = Sunucuya erişilemiyor
cli-online = Çevrim içi · { $version } · { $spieler }/{ $max } oyuncu
cli-version-passt-nicht = Çevrim içi ama sürüm { $version } – uymuyor
cli-aktualisiert = Sunucu güncellendi ({ $version }) – yöneticiden yeni bir davet kodu iste
cli-gerade-nicht = şu anda erişilemiyor
cli-modrinth-fehlt = Modrinth uygulaması bulunamadı
cli-launcher-fehlt = Minecraft Launcher bulunamadı
cli-download-seite = İndirme sayfasına git
cli-ev-eingerichtet = İstemci kuruldu: { $name }
cli-gefunden = bulundu
cli-nicht-gefunden = bulunamadı
cli-keine-mods = Mod gerekmiyor – yalnızca Minecraft { $version }.
cli-optional = isteğe bağlı
cli-nur-browser = yalnızca tarayıcıda indirilebilir (kurulumdan sonra bağlantı)
cli-nur-datei = eksik – yalnızca davet dosyasında var
cli-launcher-fehlt-text = Resmî Minecraft Launcher bulunamıyor. Yine de kurabilirsin; launcher kurulduğunda ve burada „Yenile” düğmesine bastığında profil oluşturulur.
cli-launcher-laden = Launcher’ı indir
cli-nie-gestartet = Launcher hiç başlatılmamış. Bir kez başlatıp kapat, sonra burada kur.
cli-ordner-waehlen = Bu Minecraft için klasör seç
cli-mods-zusammen = { $anzahl } mod, toplam { $groesse }
cli-andere-version = Sunucu başka bir sürüm bildiriyor ({ $version }). Büyük olasılıkla güncellendi – yöneticiden yeni bir davet kodu iste.
cli-spalte-version = Sürüm
cli-spalte-groesse = Boyut
cli-einrichten-fuer = Şunun için kur:
cli-in-modrinth = Modrinth uygulamasında:
cli-instanz-update = Var olan „{ $name }” örneğini güncelle (önerilir)
cli-instanz-update-sub = Her şey aynı örnekte kalır: dünyalar, ayarlar, shader’lar, kaynak paketleri ve kendi eklediğin modlar (örn. Iris). Modrinth uygulamasında „Mod paketini yeniden kur” düğmesine tıklaman yeter – Vapafane nasıl olduğunu gösterir.
cli-instanz-neu = Yeni örnek oluştur
cli-instanz-neu-sub = İçe aktarma üzerinden otomatik çalışır. Aşağıda ayarları, shader’ları ve kaynak paketlerini devralabilirsin – kendi eklediğin modları değil. Eski örnek sen silene dek kalır.
cli-update-bleibt = Güncellemede her şey örnekte kalır: dünyalar, seçenekler, shader’lar, kaynak paketleri ve sunucu listen.
cli-import-bestaetigen = Modrinth uygulaması açılır – içe aktarmayı onaylaman yeter.
cli-ordner-label = Klasör:
cli-launcher-schliessen = Değişikliğin üzerine yazmaması için kurulumdan önce Minecraft Launcher’ı kapat.
cli-einrichten = Kur
cli-uebernehmen-von = Ayarları şuradan devral:
cli-nichts = Hiçbir şey devralma
cli-quelle-waehlen = Devralınacak Minecraft klasörünü seç
cli-nichts-uebernehmen = Devralınacak bir şey yok
cli-teil-options = Seçenekler ve tuşlar
cli-teil-config = Mod ayarları
cli-teil-shaders = Shader’lar
cli-teil-resourcepacks = Kaynak paketleri
cli-teil-servers = Sunucu listesi
cli-teil-worlds = Dünyalar
cli-welten-warnung = Daha yeni bir Minecraft sürümünün dünyaları eski sürümde açılmaz. Büyük dünyalar kurulumu yavaşlatır.
cli-nur-fehlendes = Klasörde zaten olanlar el değmeden kalır – yalnızca eksik olanlar devralınır.
cli-uebernehmen-erklaerung = Örneğin önceki örneğinden: seçenekler, tuşlar, mod ayarları, shader’lar, kaynak paketleri ve sunucu listesi. Modlar her zaman sunucudan, uyumlu olarak gelir. Kaynak yalnızca okunur, asla değiştirilmez.
cli-paket-update-1 = Yeni Modrinth paketi hazır. Örneğini şöyle güncellersin
cli-paket-update-2 = dünyalar, seçenekler, shader’lar, kaynak paketleri ve sunucu listen – hepsi kalır:
cli-schritt-instanz = Modrinth uygulamasını aç ve örneğe tıkla
cli-schritt-anklicken = (solda).
cli-schritt-zahnrad = Üstteki „Örnek ayarları” dişlisine tıkla ve „Kurulum” sekmesini seç.
cli-neu-installieren = Mod paketini yeniden kur
cli-schritt-datei = tıkla, onayla ve bu dosyayı seç:
cli-modrinth-oeffnen = Modrinth uygulamasını aç
cli-paket-ordner = Paketin bulunduğu klasörü aç
cli-update-sicher = Modrinth uygulaması önce örneğin kendisini güvenceye alır ve yalnızca sunucu modlarını değiştirir – kendi eklediklerin (shader’lar, kaynak paketleri, kendi modların) kalır.
cli-paket-offen = Modrinth paketi hazır ve Modrinth uygulaması onunla açıldı. Orada içe aktarmayı onayla – örnek şu adla görünür:
cli-paket-fertig = Modrinth paketi hazır:
cli-paket-von-hand = Modrinth uygulaması otomatik açılamadı. Elle: Modrinth uygulamasında „+” (örnek oluştur) düğmesine tıkla, sonra „Dosyadan içe aktar” ve bu dosyayı seç.
cli-eingerichtet = Kuruldu: { $anzahl } mod indirildi.
cli-profil = Minecraft Launcher’da artık „{ $name }” profili var.
cli-uebernommen-paket = „{ $quelle }” ayarları pakette – Modrinth uygulaması içe aktarırken uygular.
cli-uebernommen = „{ $quelle }” kaynağından devralındı: { $anzahl } dosya.
cli-uebernommen-da = ({ $anzahl } tanesi zaten vardı ve el değmeden kaldı)
cli-uebernommen-fehler = { $anzahl } dosya okunamadı ve bu yüzden eksik – Minecraft orada hâlâ açık mı? ({ $dateien })
cli-selbst-laden = Kendin indirip „mods” klasörüne koyman gerekenler:
cli-spielen = Oyna
cli-spielen-modrinth = Oyna (Modrinth uygulamasını aç)
cli-spielen-launcher = Oyna (launcher’ı aç)
cli-prueft = denetleniyor …
cli-unvollstaendig = Kurulum tamamlanmadı – yukarıdaki daveti yeniden kullan (veya sil).
cli-mods-kurz = { $anzahl } mod
cli-eingerichtet-am = { $ziel } · { $datum } tarihinde kuruldu
cli-aktualisieren = Yenile
cli-aktualisieren-tipp = Kayıtlı davetle yeniden kur – veya sunucu güncellemesinden sonra ne yapılacağını açıkla
cli-entfernen-frage = „{ $name }” silinsin mi?
cli-entfernen-modrinth = Vapafane bu istemciyi unutur ve paketini siler. Modrinth uygulamasındaki örnek kalır – onu orada kendin sil.
cli-entfernen-launcher = Bu profilin modları, ayarları ve tek kişilik dünyalarını içeren klasör silinir ve profil Minecraft Launcher’dan kaybolur.
cli-upd-titel = Sunucu güncellendi
cli-upd-text = „{ $name }” artık Minecraft { $version } ile çalışıyor. Uygun sürüm ve modlar davette – bu yüzden yöneticiden yeni bir davet kodu gerekiyor.
cli-upd-code = Onu yukarıda „Davet kullan” alanına yapıştır ve „Daveti denetle” düğmesine tıkla.
cli-upd-modrinth = Vapafane o zaman yeni bir paket oluşturur ve örneğini nasıl güncelleyeceğini gösterir – ayarların, dünyaların ve shader’ların kalır.
cli-upd-launcher = Vapafane o zaman modları değiştirir – ayarların ve dünyaların kalır.
cli-gespeicherte-einladung = „{ $name }” için kayıtlı davet yüklendi. „Kur” her şeyi yeniden düzene sokar – örneğin modlar eksikse veya profil kaybolduysa. Yönetici sunucuyu güncellediyse, asıl gereken yeni bir davet kodudur.
ext-knopf = Shader’lar ve ekstralar
ext-knopf-so-gehts = Shader’lar ve ekstralar – nasıl yapılır

## Shader’lar ve ekstralar
ext-titel = Shader ve ekstra ekle
ext-skizze-1-alt = Modrinth uygulamasındaki örneğin taslağı
ext-skizze-2-alt = Modrinth uygulamasındaki aramanın taslağı
ext-skizze-leer = Ek içerik kurulu değil
ext-skizze-hochladen = Dosya yükle
ext-durchsuchen = İçeriğe göz at
ext-skizze-shader = Bir shader, örn. Complementary
ext-skizze-beschreibung = Açıklama · indirmeler
ext-keine-shader = Shader’lar Iris modunu gerektirir – o da yalnızca Fabric veya NeoForge sunucularında çalışır. Bu sunucunun mod yükleyicisi yok; bu yüzden kaynak paketleri burada çalışır ama shader’lar çalışmaz.
ext-iris-modrinth = Shader’lar için Minecraft’ın Iris moduna ihtiyacı var: 4. adımda önce „Mods” seçip „Iris Shaders” kur (Sodium otomatik gelir), sonra „Shaders” altında bir shader seç.
ext-iris-launcher-1 = Shader’lar için Minecraft’ın Iris moduna (Sodium ile) ihtiyacı var: ikisini de sunucunun Minecraft sürümü için şuradan indir:
ext-iris-launcher-2 = ve „mods” klasörüne koy.
ext-mr-einleitung-1 = Örneğinde
ext-mr-einleitung-2 = istediğin zaman kendi shader’larını, kaynak paketlerini ve modlarını ekleyebilirsin. Sunucu güncellemelerinde kalırlar – Modrinth uygulaması yalnızca sunucu modlarını değiştirir.
ext-schritt-content = Üstte „Content” (içerik) sekmesini seç.
ext-schritt-durchsuchen = Yeşil „İçeriğe göz at” düğmesine tıkla.
ext-schritt-art = Üstte türü seç: shader’lar için „Shaders”, kaynak paketleri için „Resource Packs” veya „Mods”.
ext-schritt-installieren = İstediğin öğenin yanındaki „Kur” düğmesine tıkla – bitti.
ext-im-spiel = Oyunda: Seçenekler → Grafik ayarları → Shader paketleri → shader’ı seç.
ext-im-spiel-rp = Oyunda: Seçenekler → Kaynak paketleri → paketi seç.
ext-mods-update-hinweis = Kendi modlarını Minecraft güncellemesinden sonra uygulamada kendin güncellemelisin; shader’lar ve kaynak paketleri genelde gerekmez.
ext-launcher-einleitung = Bu sunucunun oyun klasörü yalnızca ona aittir: oraya eklediklerin sunucu güncellemelerinde kalır.
ext-schritt-laden-shader = Bir shader indir, örn. şuradan:
ext-schritt-laden-rp = Bir kaynak paketi indir, örn. şuradan:
ext-nicht-entpacken = .zip dosyasını açma (çıkartma).
ext-schritt-shaderpacks = Altta „Klasörü aç” düğmesine tıkla ve dosyayı „shaderpacks” klasörüne koy. Kaynak paketleri „resourcepacks” klasörüne gider.
ext-schritt-resourcepacks = Altta „Klasörü aç” düğmesine tıkla ve dosyayı „resourcepacks” klasörüne koy.
ext-eigene-mods = „mods” klasörüne kendi modlarını koyabilirsin – ama Minecraft güncellemesinden sonra uyumlarını kendin sağlamalısın.
err-antwort-unlesbar = „{ $befehl }” yanıtı okunamıyor: { $fehler }

## Erişilebilirlik raporu
rch-w-erreichbar = erişilebilir
rch-w-nicht = erişilemiyor
rch-w-nicht-dslite = erişilemiyor (DS-Lite)
rch-w-nicht-geprueft = denetlenmedi
rch-w-nicht-vorhanden = yok
rch-unmoeglich-titel = Denetim şu anda yapılamıyor
rch-unmoeglich-text = İnternetteki denetim hizmeti yanıt vermiyor. Daha sonra yeniden dene.
rch-gruen-beide = IPv4 ve IPv6 üzerinden erişilebilir
rch-gruen-v4 = IPv4 üzerinden erişilebilir
rch-gruen-text = Tüm arkadaşlar girebilir.
rch-gelb-dslite-titel = DS-Lite: yalnızca IPv6 üzerinden erişilebilir
rch-gelb-dslite-text = Sağlayıcın IPv4 adresini paylaştırıyor. IPv6’lı arkadaşlar yine de girebilir – çoğunluk onlar.
rch-gelb-titel = Yalnızca IPv6 üzerinden erişilebilir
rch-gelb-text = IPv4 üzerinden kimse giremiyor: ya IPv4 port açılımı (port { $port }) eksik ya da sağlayıcın IPv4 adresini paylaştırıyor (DS-Lite). IPv6’lı arkadaşlar yine de girebilir.
rch-rot-titel = Dışarıdan erişilemiyor
rch-rot-text = Genelde yönlendiricide port açılımı eksiktir veya bilgisayarın güvenlik duvarı { $port } portunu engelliyordur.
rch-rot-dslite-v6 = Sağlayıcın IPv4 adresini paylaştırıyor (DS-Lite), bu yüzden yalnızca IPv6 çalışır. Yönlendiricide { $port } portunu IPv6 için aç ve bilgisayarın güvenlik duvarını denetle.
rch-rot-dslite = Sağlayıcın IPv4 adresini paylaştırıyor (DS-Lite) ve bilgisayarının IPv6 adresi yok. Böylece dışarıdan kimse sunucuya ulaşamaz.
rch-aus-titel = Sunucu çalışmıyor
rch-aus-text = Denetim, sunucu çalışır çalışmaz yapılır.
rch-port-belegt-titel = { $port } portu zaten kullanımda
rch-port-belegt-text = Başka bir program – büyük olasılıkla başka bir Minecraft sunucusu – bu portu kullanıyor. Bu sunucu böyle başlayamaz. Ayarlar → „Sunucu portu” altında boş bir port ayarla (örn. { $frei })
rch-hotspot-tipp = Sunucuya yalnızca IPv6 üzerinden erişilebiliyor ama internetinde IPv6 yok. İpucu: bilgisayarını telefonunun erişim noktası üzerinden bağla – mobil ağda genelde IPv6 vardır.

## Program hataları (genel)
err-dienst-aus = Arka plan hizmeti çalışmıyor
err-kein-systemd = systemd burada kullanılamıyor (veya test modu).
err-kein-server = Sunucu seçilmedi.
err-schon-beschaeftigt = Zaten bir şey sürüyor: { $was }
err-neustart-extern = Bu sunucu Vapafane dışında çalışıyor – yeniden başlatma yalnızca onu başlatan programla olur.
err-stop-knopf = Genel bakıştaki Durdur düğmesini kullan (düzgünce kaydeder).
err-start-extern = Sunucu Vapafane dışında çalışıyor. Önce orada durdur.
err-start-anderer = Şu anda başka bir sunucu çalışıyor. Önce onu durdur.
err-stop-unmoeglich = Sunucu durdurulamadı.
err-pfad = Geçersiz yol
err-rcon-antwort = Geçersiz RCON yanıtı
err-adresse = Geçersiz adres
cln-alte-neoforge = eski NeoForge sürümü
cln-alte-bibliothek = bir kitaplığın eski sürümü
cln-alte-mods = eski mod sürümleri (eski panel/launcher tarafından kenara alınmış)
cln-cache = mod önbelleği (bir sonraki başlangıçta yeniden oluşturulur)
cli-datei-titel = Davet dosyasını aç
srv-rcon-an = RCON açık (Vapafane komutlar için buna ihtiyaç duyar).
err-link-verboten = Bu adres açılamıyor.
err-pruefsumme = Sağlama toplamı uyuşmuyor: { $datei }
err-kein-fabric = Minecraft { $mc } için Fabric yükleyicisi bulunamadı
cf-err-leer = Önce bir anahtar yapıştır.
cf-err-id = Geçersiz CurseForge kimliği
cf-err-format = Bu bir API anahtarına benzemiyor (boşluksuz yapıştır).
cf-err-antwort = CurseForge’dan geçersiz yanıt: { $fehler }
cf-err-abgelehnt = CurseForge, API anahtarını reddediyor (geçersiz veya henüz etkinleştirilmemiş).
cf-err-fehlt = Kayıtlı CurseForge anahtarı yok.
kat-err-typ = „Mod ekle” bu sunucu türü için yok.
kat-err-keine-version = „{ $name }”, Minecraft { $mc } ({ $loader }) için yok.
kat-err-cf-version = „{ $name }”, CurseForge’da Minecraft { $mc } için yok.
fort-suche = { $name } aranıyor …
fort-suche-cf = { $name } CurseForge’da aranıyor …
inv-err-neuer = Bu davet kodu daha yeni bir Vapafane sürümünden. Vapafane’i güncelle.
inv-err-unvollstaendig = Kod eksik veya bozuk – tamamını kopyala.
inv-err-kaputt = Kod bozuk – yeniden kopyala.
inv-err-version = Bu davet başka bir Vapafane sürümü için.
inv-err-angaben = Davet geçersiz veriler içeriyor.
inv-err-mod = Davet geçersiz bir mod kaydı içeriyor.
inv-err-typ = Davetler Fabric, NeoForge, Paper ve Vanilla sunucuları için var.
inv-err-datei = Bu geçerli bir Vapafane davet dosyası değil.
inv-err-datei-kaputt = Dosyadaki davet bozuk.
inv-err-oeffentlich = Genel adres belirlenemedi.
flv-err-kein-server = Minecraft { $mc } için sunucu yok.
flv-err-kein-paper = { $mc } için Paper yok.
flv-err-kein-neoforge = Minecraft { $mc } için NeoForge yok.
fort-neoforge = NeoForge kurucusu sunucuyu hazırlıyor (kitaplıkları indiriyor) …
fort-suche-server = Minecraft { $mc } için { $was } aranıyor …
fort-pruefe-java = Java denetleniyor …
fort-frage-cf = Kalan modlar için CurseForge’a soruluyor …
fort-pruefe-deps = Bağımlılıklar denetleniyor …
fort-pruefe-dp = Datapack’ler denetleniyor …
fort-pruefe-java-platz = Java ve disk alanı denetleniyor …
upd-err-dateiname = Modrinth’ten geçersiz dosya adı: { $name }
upd-cf-fehler = CurseForge denetlenemedi: { $fehler }
upd-abhaengigkeit = (bağımlılık)
upd-grund-fehlt = Minecraft { $mc } için yok
cli-err-port = Adreste geçersiz port
cli-err-antwort = Sunucudan geçersiz yanıt
cli-err-profil-kaputt = { $datei } bozuk – launcher’ı bir kez başlat.
cli-err-nicht-leer = { $ordner } klasörü boş değil ve Vapafane’e ait değil.
cli-err-datei-kaputt = Davetin { $datei } dosyası bozuk.
cli-err-fabric-profil = Geçersiz Fabric profili.
cli-err-neoforge-java = NeoForge, Java { $version } gerektiriyor. Minecraft Launcher’ı bir kez başlat (Java’yı getirir) veya Java kur.
cli-hinweis-launcher-starten = Minecraft Launcher’ı bir kez başlatıp kapat, sonra yeniden kur.
cli-hinweis-kein-launcher = Resmî Minecraft Launcher bulunamıyor. Kur, bir kez başlat, sonra burada „Yenile” düğmesine bas.
cli-hinweis-nie-gestartet = Launcher hiç başlatılmamış. Bir kez başlatıp kapat, sonra „Yenile”.
cli-paket-summary = Vapafane tarafından { $name } sunucusu için kuruldu ({ $adresse })
fort-pruefe-mods = Modlar denetleniyor …
fort-lade = { $name } indiriliyor …
fort-launcher = Launcher kuruluyor …
fort-neoforge-install = NeoForge { $version } kuruluyor (birkaç dakika sürebilir) …
fort-uebernehme = Ayarlar devralınıyor …
fort-trage-ein = Sunucu ekleniyor …
fort-sammle = Ayarlar toplanıyor …
fort-packe = Modrinth paketi paketleniyor …
fort-fertig = Bitti.
task-update = Güncelleme
task-backup = Yedek (sunucunun tamamı ZIP olarak)
task-restore = Yedeği geri yükle
task-client = İstemciyi kur
task-rollback = Güncelleme öncesi duruma dönüş
task-pruefe-mods = { $mc } için modlar denetleniyor
upd-err-erst-pruefen = Önce „Denetle” düğmesine bas.
upd-err-anderer-server = Denetim başka bir sunucuya ait. Yeniden denetle.
upd-err-absturz = Sunucu başlarken çöktü. { $grund }
upd-err-beendet = Sunucu hemen kapandı.
upd-err-kein-backup = Bu sunucunun son güncellemesinden yedek yok.
upd-err-temp-platz = Geçici klasörde kopya için yeterli yer yok ({ $groesse } gerekli).
cln-err-laeuft = Temizlik yalnızca sunucu durdurulmuşken çalışır.
fort-raeume-auf = Eski dosyalar temizleniyor …
fort-loesche = Eski dosyalar siliniyor …
fort-loesche-kopie = Kopya siliniyor …
nbt-err-unvollstaendig = servers.dat eksik.
nbt-err-laenge = servers.dat geçersiz bir uzunluk içeriyor.
nbt-err-eintrag = servers.dat bilinmeyen bir kayıt içeriyor.
nbt-err-tiefe = servers.dat çok derin iç içe.
nbt-err-format = servers.dat bilinmeyen bir biçimde.
say-backup-start = Yedek alınıyor …
say-backup-fertig = Yedek tamamlandı.
say-backup-fehler = Yedekleme başarısız.
say-stoppt = duracak
say-neustart = yeniden başlayacak
say-countdown = Sunucu { $sekunden } saniye içinde { $was }!
svc-countdown = Sunucu { $sekunden } sn içinde { $was }
svc-err-laeuft-nicht = Sunucu şu anda çalışmıyor.
svc-err-befehl = Bilinmeyen komut: { $befehl }
svc-err-ordner-fehlt = Sunucu klasörü eksik: { $ordner }
svc-err-eula = Bu sunucu için Minecraft EULA’sı henüz kabul edilmedi.
svc-err-fremder-server = Bu klasörde zaten Vapafane tarafından başlatılmamış bir sunucu çalışıyor (PID { $pid }). Önce onu durdur (örn. eski panel).
svc-err-port = { $port } portu ({ $art }) zaten kullanımda. Bu bilgisayarda başka bir Minecraft sunucusu mu çalışıyor?
svc-signal = sinyalle sonlandırıldı
svc-beendet = sonlandı
svc-abgestuerzt = Çöktü ({ $grund }).
svc-taegliches-backup = Günlük yedek
say-update-neustart = Sunucu bir güncelleme için yeniden başlıyor!
svc-err-java-start = Java başlatılamadı ({ $java }): { $fehler }

## Komut satırı
ctl-hilfe = Kullanım: vapafane --ctl <komut>

      status            etkin sunucunun durumunu göster
      start             sunucuyu başlat (gerekirse arka plan hizmetini de başlatır)
      stop [SANİYE]     sunucuyu düzgünce durdur, isteğe bağlı geri sayımla
      restart [SN.]     sunucuyu yeniden başlat
      wait-ready [SN.]  sunucu hazır olana dek bekle
      wait-stopped [SN.] sunucu durana dek bekle
      backup            sunucu klasörünün tamamını ZIP olarak kaydet
      cmd <KOMUT …>     RCON üzerinden komut gönder
      aktiv AD          listeden başka bir sunucuyu etkinleştir (yalnızca durmuşken)
      neu TÜR AD SÜR [KLASÖR]  yeni sunucu oluştur (fabric, paper, neoforge, vanilla) ve etkinleştir
      update-check SÜR  Minecraft SÜR için modları/datapack’leri Modrinth’ten denetle
      trial SÜR         deneme: kopyayı güncelle, başka portta test et, sil
      update SÜR        güncelleme: durdur, yedekle, değiştir, başlat (yalnızca denetim olumluysa)
      mods              modları ve datapack’leri açık/kapalı durumuyla göster
      mod-an YOL        bir modu aç (YOL „mods” içindeki gibi, örn. mods/x.jar)
      mod-aus YOL       bir modu kapat (.jar.disabled olur)
      aufraeumen [--ja] eski kalıntıları gösterir; --ja ile silinir (sunucu durmuş olmalı)
      mod-suche [TERİM] [--datapack]    Modrinth’te uygun modları/eklentileri (veya datapack’leri) ara
      mod-install PROJE [--datapack]    bir modu/eklentiyi/datapack’i gerekli bağımlılıklarıyla kur
      curseforge-schluessel [--loeschen]  CurseForge anahtarını girişten oku ve kaydet
      curseforge-test   kayıtlı CurseForge anahtarını dene
      einladung [ADRES]                 arkadaşlar için davet kodu göster (adres = sunucu:port)
      einladung-datei DOSYA [ADRES]     davet dosyası yaz (.vapafane)
      einloesen KOD|DOSYA [--ordner K] [--alle-optionalen] [--ziel launcher|modrinth]
                … [--einstellungen-von KLASÖR] [--neue-instanz]
                                        bir davet için istemci kur (launcher profili veya Modrinth paketi);
                                        ayarları (dünyalar hariç) KLASÖR’den devral; --neue-instanz ile
                                        Modrinth uygulamasındaki var olan örnek güncellenmez
      clients                           kurulu istemcileri göster
      client-entfernen KLASÖR           bir istemciyi ve launcher profilini sil
      ping ADRES                        sunucu durumunu sorgula (oyundaki sunucu listesi gibi)
      erreichbarkeit    sunucuya dışarıdan erişilip erişilemediğini denetle (IPv4/IPv6, DS-Lite)
      mrpack DOSYA      modları arkadaşlar için Modrinth mod paketi (.mrpack) olarak dışa aktar
      quit-service      arka plan hizmetini kapat (önce sunucuyu durdurur)
ctl-fehler = Hata: { $fehler }
