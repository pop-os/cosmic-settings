app = COSMIC Ayarları
dbus-connection-error = DBus'a bağlanılamadı
ok = Tamam
unknown = Bilinmiyor
number = { $number }

## Network & Wireless

add-network = Ağ ekle
    .profile = Profil ekle
add-vpn = VPN ekle
airplane-on = Uçak modu açık.
cable-unplugged = Kablo takılı değil
connect = Bağlan
connected = Bağlandı
connecting = Bağlanılıyor…
disconnect = Bağlantıyı kes
forget = Unut
known-networks = Bilinen ağlar
network-and-wireless = Ağ & kablosuz
no-networks = Herhangi bir ağ bulunamadı.
no-vpn = Herhangi bir VPN bağlantısı bulunmamaktadır.
password = Parola
password-confirm = Parolayı onayla
remove = Kaldır
settings = Ayarlar
username = Kullanıcı Adı
visible-networks = Görünen ağlar
identity = Kimlik
auth-dialog = Kimlik Doğrulama gerekli
    .vpn-description = VPN hizmeti tarafından istenilen kullanıcı adı ve parolayı girin.
    .wifi-description = Parola veya şifreleme anahtarını girin. Yönlendiricinin "WPS" düğmesine basarak da bağlanabilirsiniz.
forget-dialog = Bu Wi-Fi ağı unutulsun mu?
    .description = Gelecekte bu Wi-Fi ağını kullanmak için tekrar bir parola girmeniz gerekecektir.
network-device-state =
    .activated = Bağlanıldı
    .config = Bağlanılıyor
    .deactivating = Bağlantı kesiliyor
    .disconnected = Bağlantı kesildi
    .failed = Bağlantı başarısız
    .ip-check = Bağlantı kontrol ediliyor
    .ip-config = IP ve yönlendirme bilgisi isteniyor
    .need-auth = Kimlik doğrulama gerekiyor
    .prepare = Bağlanmaya hazırlanıyor
    .secondaries = İkincil bağlantı bekleniyor
    .unavailable = Mevcut değil
    .unknown = Bilinmeyen durum
    .unmanaged = Yönetilmiyor
    .unplugged = Kablo bağlı değil
remove-connection-dialog = Bağlantı profili silinsin mi?
    .vpn-description = Gelecekte bu ağı kullanabilmek için tekrardan parola girmeniz gerekecek.
    .wired-description = Gelecekte bu profili kullanabilmek için tekrardan oluşturmanız gerekecek.
vpn = VPN
    .connections = VPN bağlantıları
    .error = VPN yapılandırması eklenemedi
    .remove = Bağlantı profilini kaldır
    .select-file = Bir VPN yapılandırma dosyası seçin
vpn-error = VPN Hatası
    .config = VPN yapılandırması eklenemedi
    .connect = VPN'e bağlanılamadı
    .connection-editor = Bağlantı düzenleyici başarısız oldu
    .connection-settings = Aktif bağlantılar için ayarlar alınamadı
    .updating-state = Ağ yöneticisi durumu alınamadı
    .wireguard-config-path = WireGuard yapılandırması için geçersiz dosya yolu
    .wireguard-config-path-desc = Seçilen dosya yerel dosya sisteminde olmalı.
    .wireguard-device = WireGuard cihazı oluşturulamadı
    .with-password =
        VPN ayarlanamadı { $field ->
           *[username] kullanıcı adı
            [password] parola
            [password-flags] parola-bayrakları
        } nmcli ile
wired = Kablolu
    .adapter = Kablolu adaptör { $id }
    .connections = Kablolu bağlantılar
    .devices = Kablolu cihazlar
    .remove = Bağlantı profilini kaldır
wifi = Kablosuz
    .adapter = Kablosuz adaptör { $id }
    .forget = Ağı unut
wireguard-dialog = WireGuard cihazı ekle
    .description = WireGuard yapılandırması için cihaz adı belirleyin.

## Networking: Online Accounts

online-accounts = Çevrimiçi hesaplar
    .desc = Hesap, IMAP ve SMTP, kurumsal giriş ekleyin

# Bluetooth

activate = Aktifleştir
confirm = Onayla
enable = Etkinleştir
bluetooth = Bluetooth
    .status = Bu sistem bluetooth ayarları açık olduğu sürede { $aliases } olarak görünür.
    .connected = Bağlandı
    .connecting = Bağlanılıyor
    .disconnecting = Bağlantı kesiliyor
    .connect = Bağlan
    .disconnect = Bağlantıyı kes
    .forget = Unut
    .dbus-error = DBus ile etkileşirken bir hata meydana geldi: { $why }
    .disabled = Bluetooth hizmeti devre dışı
    .inactive = Bluetooth hizmeti aktif değil
    .unknown = Bluetooth hizmeti aktifleştirilemedi. BlueZ kurulu mu?
bluetooth-paired = Önceden bağlanılan cihazlar
    .connect = Bağlan
    .battery = %{ $percentage } şarj
bluetooth-confirm-pin = Bluetooth PIN'ini Onayla
    .description = Lütfen aşağıdaki PIN'in { $device } cihazında gözüken ile eşleştiğini onaylayın
bluetooth-available = Yakındaki cihazlar
bluetooth-adapters = Bluetooth adaptörleri

## Accessibility

accessibility = Erişilebilirlik
    .vision = Görme
    .on = Açık
    .off = Kapalı
    .unavailable = Mevcut değil
    .screen-reader = Ekran okuyucu
    .high-contrast = Yüksek kontrast modu
    .invert-colors = Renkleri ters çevir
    .color-filters = Renk filtreleri
hearing = Duyma
    .mono = Stereo sesi mono olarak çal
default = Varsayılan
magnifier = Büyüteç
    .controls =
        Veya bu kısayolları kullanın: { $zoom_in ->
            [zero] { "" }
           *[other]
                { "" }
                Yakınlaştırmak için { $zoom_in },
        }{ $zoom_out ->
            [zero] { "" }
           *[other]
                { "" }
                Uzaklaştırmak için { $zoom_out },
        }
        Fareniz ile Super + fare tekeri
    .scroll_controls = Super + Fare tekeri kullanarak fare veya dokunmatik yüzey ile kaydırmayı etkinleştir.
    .show_overlay = Büyüteç Overlay'ini göster.
    .increment = Yakınlaştırma miktarı
    .signin = Büyüteci açılışta başlat
    .applet = Büyüteci panel eklentisinden açıp kapat
    .movement = Büyütülmüş görünümde gezinim
    .continuous = İmleç ile beraber
    .onedge = İmleç kenara ulaştığında
    .centered = İmleci merkezde tutarak
color-filter = Renk filtresi tipi
    .unknown = Bilinmeyen filtre etkin
    .greyscale = Gri tonlama
    .deuteranopia = Yeşil/Kırmızı (yeşil zayıflığı, Döteranopi)
    .protanopia = Kırmızı/Yeşil (kırmızı zayıflığı, Protanopi)
    .tritanopia = Mavi/Sarı (mavi zayıflığı, Tritanopi)

## Desktop

desktop = Masaüstü

## Desktop: Wallpaper

wallpaper = Duvar Kağıdı
    .change = Belirli aralıklarda resmi değiştir
    .fit = Duvar kağıdı yerleşimi
    .folder-dialog = Duvar kağıdı klasörü seçin
    .image-dialog = Duvar kağıdı resmi seçin
    .plural = Duvar kağıtları
    .same = Bütün ekranlarda aynı duvar kağıdı
    .slide = Slayt gösterisi
add-color = Renk ekle
add-image = Resim ekle
all-displays = Tüm ekranlar
colors = Renkler
dialog-add = Ekle
fill = Doldur
fit-to-screen = Ekrana sığdır
open-new-folder = Yeni klasör aç
recent-folders = Son kullanılan klasörler
x-minutes =
    { $number } { $number ->
        [one] dakika
       *[other] dakika
    }
x-hours =
    { $number } { $number ->
        [one] saat
       *[other] saat
    }
never = Asla

## Desktop: Appearance

appearance = Görünüm
accent-color = Vurgu rengi
app-background = Pencere arka planı
auto = Otomatik
close = Kapat
color-picker = Renk Seçici
copied-to-clipboard = Panoya kopyalandı
copy-to-clipboard = Panoya kopyala
dark = Karanlık
export = Dışa aktar
hex = Hex
import = İçe aktar
light = Aydınlık
mode-and-colors = Mod ve renkler
recent-colors = Son kullanılan renkler
reset-to-default = Varsayılana dön
rgb = RGB
window-hint-accent = Etkin pencere vurgu rengi
window-hint-accent-toggle = Tema vurgu rengini etkin pencere vurgu rengi olarak kullan
auto-switch = Otomatik olarak Aydınlık ve Karanlık modlar arasında geçiş yap
    .sunrise = Gün doğumunda Aydınlık moda geçer
    .sunset = Gün batımında Karanlık moda geçer
    .next-sunrise = Bir dahaki gün doğumunda Aydınlık moda geçer
    .next-sunset = Bir dahaki gün doğumunda Karanlık moda geçer
container-background = Konteyner arka planı
    .desc-detail = Konteyner arka planı gezinti yan çubuğu, yan çekmece, diyaloglar ve benzeri araçlar için kullanılır. Varsayılanda otomatik olarak Uygulama veya pencere arka planından belirlenir.
    .reset = Otomatiğe dön
    .desc = Birincil konteyner rengi gezinti yan çubuğu, yan çekmece, diyaloglar ve benzeri araçlar için kullanılır
control-tint = Bileşen tonlamasını ayarla
    .desc = Standart düğmelerin, arama ve metin girdilerinin ve benzeri bileşenlerin arka planları için kullanılır
frosted = Sistem arayüzünde buzlu cam efekti
    .desc = Panel, dock, eklentiler, başlatıcı ve uygulama kütüphanesi arka planına flu ekler
enable-export = Bu temayı GNOME uygulamalarına uygula
    .desc = Bütün toolkitler otomatik tema değişimi desteklemez. COSMIC dışı uygulamaların tema değişimi için uygulamanın yeniden başlatılması gerekebilir.
icon-theme = Simge teması
    .desc = Uygulamalara farklı birtakım simgeler uygular
text-tint = Arayüz metin tonlaması
    .desc = Çeşitli yüzeylerde yeterli kontrast sağlayan arayüz metin renklerini elde etmek için kullanılır
style = Stil
    .round = Yuvarlak
    .slightly-round = Hafif yuvarlak
    .square = Kare
interface-density = Arayüz yoğunluğu
    .comfortable = Rahat
    .compact = Sıkışık
    .spacious = Geniş
window-management-appearance = Pencere Yönetimi
    .active-hint = Etkin pencere belirten boyutu
    .gaps = Dizili pencerelerin etrafındaki boşluk

### Experimental

experimental-settings = Deneysel ayarlar
icons-and-toolkit = Simge ve araç seti teması
interface-font = Sistem yazı tipi
monospace-font = Tek aralıklı yazı tipi

## Desktop: Notifications

notifications = Bildirimler

## Desktop: Panel

panel = Panel
add = Ekle
add-applet = Eklenti ekle
all = Tümü
applets = Eklentiler
center-segment = Orta bölüm
end-segment = Son bölüm
large = Büyük
no-applets-found = eklenti bulunamadı...
panel-bottom = Aşağı
panel-left = Sol
panel-right = Sağ
panel-top = Yukarı
search-applets = Eklenti ara...
small = Küçük
start-segment = Başlangıç Bölümü
panel-appearance = Görünüm
    .match = Masaüstü ile aynı
    .light = Aydınlık
    .dark = Karanlık
panel-behavior-and-position = Davranış ve Konumlar
    .autohide = Paneli otomatik gizle
    .dock-autohide = Docku otomatik gizle
    .position = Ekrandaki konum
    .display = Ekranda göster
panel-style = Stil
    .anchor-gap = Ekran kenarları ve panel arasındaki boşluk
    .dock-anchor-gap = Ekran kenarları ve dock arasındaki boşluk
    .extend = Paneli ekran kenarlarına genişlet
    .dock-extend = Docku ekran kenarlarına genişlet
    .appearance = Görünüm
    .size = Boyut
    .background-opacity = Arka plan opaklığı
panel-applets = Yapılandırma
    .dock-desc = Dock eklentilerini yapılandır
    .desc = Panel eklentilerini yapılandır
panel-missing = Panel Yapılandırması Eksik
    .desc = Panel yapılandırma dosyası özel bir yapılandırması kullanıldığından veya bozulduğundan dolayı eksik.
    .fix = Varsayılana dön

## Desktop: Dock

dock = Dock

## Desktop: Window management

window-management = Pencere yönetimi
super-key = Super tuşu davranışı
    .launcher = Başlatıcıyı Aç
    .workspaces = Çalışma Alanlarını Aç
    .applications = Uygulamaları Aç
    .disable = Devre dışı bırak
edge-gravity = Kayan pencereler kenarlara yapışır
window-controls = Pencere kontrolleri
    .minimize = Küçültme tuşunu göster
    .maximize = Büyütme tuşunu göster
    .active-window-hint = Etkin pencere belirtmesini göster
focus-navigation = Odak gezinimi
    .focus-follows-cursor = Odak imleci takip eder
    .focus-follows-cursor-delay = Odak imleci milisaniye gecikme ile takip eder
    .cursor-follows-focus = İmleç odağı takip eder

## Desktop: Workspaces

workspaces = Çalışma Alanları
workspaces-behavior = Çalışma alanı davranışı
    .dynamic = Dinamik çalışma alanları
    .dynamic-desc = Boş çalışma alanlarını otomatik olarak siler.
    .fixed = Sabit sayıda çalışma alanı
    .fixed-desc = Çalışma alanlarını genel görünümde ekle veya çıkar.
workspaces-multi-behavior = Çoklu-ekran davranışı
    .span = Çalışma alanları ekranlara yayılır
    .separate = Ekranların ayrı çalışma alanları olur
workspaces-overview-thumbnails = Çalışma alanları genel görünümü küçük resimleri
    .show-number = Çalışma alanı numarasını göster
    .show-name = Çalışma alanı adını göster
workspaces-orientation = Çalışma alanları yönelimi
    .vertical = Dikey
    .horizontal = Yatay
hot-corner = Hızlı Köşe
    .top-left-corner = Çalışma Alanları için sol üst hızlı köşeyi etkinleştir

## Displays

-requires-restart = Yeniden başlatma gerektirir
color = Renk
    .depth = Renk derinliği
    .profile = Renk profili
    .sidebar = Renk Profilleri
    .temperature = Renk sıcaklığı
display = Ekranlar
    .arrangement = Ekran Dizilimi
    .arrangement-desc = Ekranları yeniden dizmek için sürükleyin.
    .enable = Ekranı etkinleştir.
    .external = { $size } { $output } Harici Ekran
    .laptop = { $size } Dizüstü Ekranı
    .options = Ekran Seçenekleri
    .refresh-rate = Yenileme Hızı
    .resolution = Çözünürlük
    .scale = Ölçek
    .additional-scale-options = Ek ölçek seçenekleri
mirroring = Ekran Yansıtma
    .id = Yansıt { $id }
    .dont = Yansıtma
    .mirror = { $display } ekranını yansıt
    .project =
        Yansıt { $display ->
            [all] tüm ekranlar
           *[other] { $display } ekrana
        }
    .project-count =
        Yansıtılıyor { $count } diğer { $count ->
            [1] ekran
           *[other] ekranlara
        }
night-light = Gece Işığı
    .auto = Otomatik (gün doğumundan batışına)
    .desc = Daha sıcak renklerle mavi ışığı azalt
orientation = Yönelim
    .standard = Standart
    .rotate-90 = 90° çevir
    .rotate-180 = 180° çevir
    .rotate-270 = 270° çevir
vrr = Değişken yenileme hızı
    .enabled = Etkin
    .force = Her zaman
    .auto = Otomatik
    .disabled = Devre dışı
scheduling = Programlama
    .manual = Manuel Program
dialog = Diyalog
    .title = Görüntü ayarları korunsun mu?
    .keep-changes = Değişiklikleri koru
    .change-prompt = { $time } saniye içinde önceki görüntü ayarlarına dönülecek.
    .revert-settings = Ayarları geri al

## Sound

sound = Ses
sound-output = Çıkış
    .volume = Çıkış sesi
    .device = Çıkış cihazı
    .level = Çıkış düzeyi
    .config = Yapılandırma
    .balance = Denge
    .left = Sol
    .right = Sağ
sound-input = Giriş
    .volume = Giriş sesi
    .device = Giriş cihazı
    .level = Giriş seviyesi
sound-alerts = Uyarılar
    .volume = Uyarı seviyesi
    .sound = Uyarı sesi
sound-applications = Uygulamalar
    .desc = Uygulama sesleri ve ayarları

## Power

power = Güç & pil
battery = Pil
    .minute =
        { $value } { $value ->
            [one] dakika
           *[other] dakika
        }
    .hour =
        { $value } { $value ->
            [one] saat
           *[other] saat
        }
    .day =
        { $value } { $value ->
            [one] gün
           *[other] gün
        }
    .less-than-minute = Bir dakikadan az
    .and = ve
    .remaining-time =
        { $time } e kadar { $action ->
            [full] dolu
           *[other] boş
        }
connected-devices = Bağlı cihazlar
    .unknown = Bilinmeyen cihaz
power-mode = Güç modu
    .battery = Uzatılmış pil ömrü
    .battery-desc = Azaltılmış güç kullanımı ve performans
    .balanced = Dengeli
    .balanced-desc = Orta düzey performans ve güç kullanımı
    .performance = Yüksek performans
    .performance-desc = Maksimum performans ve güç kullanımı
    .no-backend = Arka uç bulunamadı. system76-power veya power-profiles-daemon kurun.
power-saving = Güç tasarrufu seçenekleri
    .turn-off-screen-after = Ekranı belirli süreden sonra kapat
    .auto-suspend = Otomatik askıya al
    .auto-suspend-ac = Şarjdayken otomatik askıya al
    .auto-suspend-battery = Pildeyken otomatik askıya al

## Input

acceleration-desc = Otomatik olarak takip hassasiyetini hıza bağlı olarak düzenle
disable-while-typing = Yazarken devre dışı bırak
input-devices = Giriş cihazları
primary-button = Birincil buton
    .desc = Fiziksel butonların sırasını belirler
    .left = Sol
    .right = Sağ
scrolling = Kaydırma
    .two-finger = İki parmak ile kaydır
    .edge = Kenardan tek parmak ile kaydır
    .speed = Kaydırma hızı
    .natural = Doğal kaydırma
    .natural-desc = Görüntü yerine içeriği kaydır

## Input: Keyboard

slow = Yavaş
fast = Hızlı
short = Kısa
long = Uzun
keyboard = Klavye
keyboard-sources = Giriş Kaynakları
    .desc = Giriş kaynakları arasında Super+Space tuş kombinasyonu ile geçiş yapılabilir. Bu klavye kısayolları ayarlarından kişileştirilebilir.
    .move-up = Yukarı çıkar
    .move-down = Aşağı indir
    .settings = Ayarlar
    .view-layout = Klavye düzenini görüntüle
    .remove = Kaldır
    .add = Giriş kaynağı ekle
keyboard-special-char = Özel Karakter Girişi
    .alternate = Alternatif karakter tuşu
    .compose = Özel karakter tuşu
    .compose-desc = Özel karakter tuşu, çok çeşitli karakterlerin girilmesine olanak tanır. Kullanmak için, özel karakter tuşuna ve ardından bir karakter dizisine basın. Örneğin, özel karakter tuşuna basıp ardından C ve o tuşlarına basmak © karakterini, özel karakter tuşuna basıp ardından a ve ‘ tuşlarına basmak ise á karakterini girecektir.
    .caps = Caps Lock tuşu
keyboard-typing-assist = Yazma
    .repeat-rate = Tekrar hızı
    .repeat-delay = Tekrar gecikme süresi
keyboard-numlock-boot = Numlock
    .boot-state = Başlangıç durumu
    .last-boot = Son başlangıç
    .on = Açık
    .off = Kapalı
    .set = Numlock başlangıç durumunu ayarla
added = Eklendi
type-to-search = Aramak için yazın...
show-extended-input-sources = Genişletilmiş giriş kaynaklarını göster

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Klavye kısayolları
    .desc = Kısayolları görüntüle ve kişileştir
cancel = Vazgeç
command = Komut
custom = Özel
debug = Hata ayıklama
disabled = Devre dışı
input-source-switch = Klavye dili giriş kaynağını değiştir
migrate-workspace-prev = Çalışma alanını önceki çıktıya taşı
migrate-workspace-next = Çalışma alanını sonraki çıktıya taşı
migrate-workspace =
    Çalışma alanını { $direction ->
       *[down] aşağı
        [left] sol
        [right] sağ
        [up] üst
    } çıkışa taşı
navigate = Gezin
replace = Değiştir
shortcut-name = Kısayol adı
system-controls = Sistem kontrolleri
terminate = Sonlandır
toggle-stacking = Pencere istiflemeyi aç/kapat
type-key-combination = Tuş kombinasyonu girin
custom-shortcuts = Özel kısayollar
    .add = Kısayol ekle
    .context = Özel kısayol ekle
    .none = Özel kısayol yok
modified = { $count } düzenlenmiş
nav-shortcuts = Gezinim
    .prev-output = Önceki çıktıya odaklan
    .next-output = Sonraki çıktıya odaklan
    .last-workspace = Son çalışma alanına odaklan
    .prev-workspace = Önceki çalışma alanına odaklan
    .next-workspace = Sonraki çalışma alanına odaklan
    .focus =
        { $direction ->
           *[down] Aşağıdaki
            [in] İçerideki
            [left] Soldaki
            [out] Dışarıdaki
            [right] Sağdaki
            [up] Üstteki
        } pencereye odaklan
    .output =
        { $direction ->
           *[down] Aşağıdaki
            [left] Soldaki
            [right] Sağdaki
            [up] Üstteki
        } çıktıya geç
    .workspace = { $num }. çalışma alanına geç
manage-windows = Pencereleri yönet
    .close = Pencereyi kapat
    .maximize = Pencereyi büyüt
    .fullscreen = Pencereyi tam ekrana al
    .minimize = Pencereyi küçült
    .resize-inwards = Pencereyi içe doğru yeniden boyutlandır
    .resize-outwards = Pencereyi dışa doğru yeniden boyutlandır
    .toggle-sticky = Yapışkan pencereyi aç/kapat
move-windows = Pencereyi Taşı
    .direction =
        Pencereyi { $direction ->
           *[down] aşağı
            [left] sola
            [right] sağa
            [up] yukarı
        } taşı
    .display =
        Pencereyi bir monitör { $direction ->
           *[down] aşağı
            [left] sola
            [right] sağa
            [up] yukarı
        } taşı
    .workspace =
        Pencereyi bir çalışma alanı { $direction ->
           *[below] aşağı
            [left] sola
            [right] sağa
            [above] yukarı
        } taşı
    .workspace-num = Pencereyi { $num }. çalışma alanına taşı
    .prev-workspace = Pencereyi önceki çalışma alanına taşı
    .next-workspace = Pencereyi sonraki çalışma alanına taşı
    .last-workspace = Pencereyi son çalışma alanına taşı
    .next-display = Pencereyi sonraki ekrana taşı
    .prev-display = Pencereyi önceki ekrana taşı
    .send-to-prev-workspace = Pencereyi önceki çalışma alanına taşı
    .send-to-next-workspace = Pencereyi sonraki çalışma alanına taşı
system-shortcut = Sistem
    .app-library = Uygulama kütüphanesini aç
    .brightness-down = Ekran parlaklığını azalt
    .brightness-up = Ekran parlaklığını arttır
    .display-toggle = Dahili ekranı aç/kapat
    .home-folder = Ev dizinini aç
    .keyboard-brightness-down = Klavye parlaklığını azalt
    .keyboard-brightness-up = Klavye parlaklığını arttır
    .launcher = Başlatıyıcı aç
    .log-out = Çıkış Yap
    .lock-screen = Ekranı kitle
    .mute = Ses çıkışını sustur
    .mute-mic = Mikrofon girişini sustur
    .play-pause = Oynat/Duraklat
    .play-next = Sonraki parça
    .play-prev = Önceki parça
    .poweroff = Gücü kapat
    .screenshot = Ekran görüntüsü al
    .suspend = Askıya al
    .terminal = Bir uçbirim aç
    .touchpad-toggle = Dokunmatik yüzeyi aç/kapat
    .volume-lower = Çıkış ses düzeyini azalt
    .volume-raise = Çıkış ses düzeyini arttır
    .web-browser = Bir web tarayıcısı aç
    .window-switcher = Açık pencereler arasında geçiş yap
    .window-switcher-previous = Açık pencereler arasında ters yönde geçiş yap
    .workspace-overview = Çalışma alanları genel görünümünü aç
window-tiling = Pencere döşeme
    .horizontal = Yatay yönelimi ayarla
    .vertical = Dikey yönelimi ayarla
    .swap-window = Pencereyi değiştir
    .toggle-tiling = Pencere döşemeyi aç/kapat
    .toggle-stacking = Pencere istiflemeyi aç/kapat
    .toggle-floating = Pencere kaydırmayı aç/kapat
    .toggle-orientation = Yönelimi değiştir
replace-shortcut-dialog = Kısayol değiştirilsin mi?
    .desc = { $shortcut }, { $name } tarafından kullanılmakta. Eğer değiştirirseniz, { $name } devre dışı bırakılacak.
zoom-in = Yakınlaştır
zoom-out = Uzaklaştır

## Input: Mouse

mouse = Fare
    .speed = Fare hızı
    .acceleration = Fare ivmesini etkinleştir

## Input: Touchpad

click-behavior = Tıklama davranışı
    .click-finger = İki parmak ile ikincil tıklama ve üç parmak ile orta tıklama
    .button-areas = Sağ alt köşede ikincil tıklama ve orta alt kenarında orta tıklama
pinch-to-zoom = İki parmakla yakınlaştırma
    .desc = Yakınlaştırma özelliği bulunan uygulamalarda içeriği yakınlaştırmak için iki parmağınızı kullanın
tap-to-click = Tıklamak için dokun
    .desc = Birincil tıklama için tek parmakla dokunmayı, ikincil tıklama için iki parmakla dokunmayı ve orta tıklama için üç parmakla dokunmayı sağlar
touchpad = Dokunmatik yüzey
    .acceleration = Dokunmatik yüzey ivmesini etkinleştir
    .speed = Dokunmatik yüzey hızı

## Input: Gestures

gestures = Hareketler
    .four-finger-down = Dört parmak ile aşağı kaydırma
    .four-finger-left = Dört parmak ile sola kaydırma
    .four-finger-right = Dört parmak ile sağa kaydırma
    .four-finger-up = Dört parmak ile yukarı kaydırma
    .three-finger-any = Üç parmak ile herhangi yöne kaydırma
switch-workspaces = Çalışma alanları arasında geçiş yapma
    .horizontal = Dört parmak ile sola/sağa kaydırma
    .vertical = Dört parmak ile yukarı/aşağı kaydırma
switch-between-windows = Pencereler arasında geçiş yapma
open-application-library = Uygulama Kütüphanesini Aç
open-workspaces-view = Çalışma Alanları Genel Görünümünü aç

## Time & Language

time = Zaman & dil
time-date = Tarih & saat
    .auto = Otomatik ayarla
    .auto-ntp = Tarih & saat, saat dilimi ayarlandığında otomatik olarak güncellenir
time-zone = Saat dilimi
    .auto = Otomatik saat dilimi
    .auto-info = Konum hizmetlerine ve internet erişimine gereksim duyar
time-format = Tarih & Saat Biçimi
    .twenty-four = 24-saat biçimi
    .show-seconds = Show seconds
    .first = Haftanın ilk günü
    .show-date = Zaman eklentisinde tarihi göster
    .friday = Cuma
    .saturday = Cumartesi
    .sunday = Pazar
    .monday = Pazartesi
time-region = Bölge & dil
formatting = Biçimlendirme
    .dates = Tarihler
    .time = Saat
    .date-and-time = Tarih & saat
    .numbers = Sayılar
    .measurement = Ölçüm
    .paper = Kağıt
preferred-languages = Tercih edilen diller
    .desc = Dillerin sırası masaüstünün çevirisinde hangi dilin kullanılacağını belirler. Değişiklikler bir dahaki girişinizde geçerli olur.
add-language = Dil ekle
    .context = Dil Ekle
install-additional-languages = Ek dil kur
region = Bölge

## Applications

applications = Uygulamalar

## Applications: Default Applications

default-apps = Varsayılan Uygulamalar
    .web-browser = İnternet tarayıcısı
    .file-manager = Dosya gezgini
    .mail-client = E-posta istemcisi
    .music = Müzik
    .video = Video
    .photos = Görseller
    .calendar = Takvim
    .terminal = Uçbirim
    .other-associations = Diğer ilişkiler
    .text-editor = Metin Düzenleyici
    .not-installed = kurulu değil

## Applications: Startup Applications

startup-apps = Başlangıç uygulamaları
    .add = Uygulama ekle
    .user = Oturum açtığınızda başlatılan uygulamalar
    .none = Başlangıç uygulaması eklenmedi
    .remove-dialog-title = { $name } ögesi kaldırılsın mı?
    .remove-dialog-description = Bu başlangıç uygulamasını kaldırmak ister misiniz?
    .add-startup-app = Başlangıç uygulaması ekle

## Applications: Legacy Applications

legacy-applications = X11 uygulamaları uyumluluğu
legacy-app-global-shortcuts = X11 Uygulamalarında genel kısayollar
    .desc = Genel kısayollar, uygulamalarda gerçekleştirilen klavye tuş vuruşlarının ve fare düğmesi olaylarının, bas konuş veya bas sessize al gibi özellikler için diğer uygulamalar tarafından tanınmasını sağlar. Varsayılan olarak, diğer uygulamaların hassas bilgiler içeren klavye ve fare olaylarını izlemesini engellemek için X11 uygulamalarında Genel kısayollar devre dışıdır.
    .none = Hiçbir tuş
    .modifiers = Modifierlar (Super, Shift, Control, Alt)
    .combination = Super, Control veya Alt tuşları basılı tutulduğu sürece tüm tuşlar
    .all = Tüm tuşlar
    .mouse = X11 uygulamalarında fare tuş olayları
legacy-app-scaling = X11 pencere sistemi uygulama ölçeklemesi
    .scaled-gaming = Oyun ve tam ekran uygulamalar için uyarla
    .gaming-description = X11 uygulamaları Wayland uygulamalarına göre biraz büyük veya küçük görünebilir
    .scaled-applications = Uygulamalar için uyarla
    .applications-description = Oyunlar ve tam ekran X11 uygulamaları ekran çözünürlüğünüze uymayabilir
    .scaled-compatibility = Maksimum uyumluluk modu
    .compatibility-description = X11 uygulamaları HiDPI ekranlarda bulanık gözükebilir
    .preferred-display = Oyunlar ve tam ekran X11 uygulamaları için tercih edilen ekran
    .no-display = Hiçbiri

## System

system = Sistem & hesaplar

## System: About

about = Hakkında
about-device = Cihaz adı
    .desc = Bu ad, Bluetooth cihazlarına ve ağdaki diğer cihazlara gözükür
about-hardware = Donanım
    .model = Donanım modeli
    .memory = Bellek
    .processor = İşlemci
    .graphics = Grafik
    .disk-capacity = Disk kapasitesi
about-os = İşletim sistemi
    .os = İşletim sistemi
    .os-architecture = İşletim sistemi mimarisi
    .kernel = Çekirdek sürümü
    .desktop-environment = Masaüstü ortamı
    .windowing-system = Pencere sistemi
about-related = İlgili ayarlar
    .support = Destek al

## System: Firmware

firmware = Cihaz yazılımı

## System: Users

users = Kullanıcılar
    .admin = Yönetici
    .standard = Standart
    .profile-add = Profil resmi seçin
administrator = Yönetici
    .desc = Yöneticiler tüm kullanıcılar için ayarları değiştirebilir, başka kullanıcılar ekleyebilir ve onları kaldırabilir
add-user = Kullanıcı ekle
change-password = Parolayı değiştir
remove-user = Kullanıcıyı kaldır
full-name = Tam ad
invalid-username = Geçersiz kullanıcı adı
password-mismatch = Parola ve doğrulama eşleşmelidir
save = Kaydet
amplification = Amplifikasyon
    .desc = Sesi %150'ye kadar yükseltmeye olanak tanır
add-another-keybinding = Başka bir tuş ataması ekle
xdg-entry-mouse-keywords = COSMIC;Fare; Hızlanma; Kaydırma;
xdg-entry-bluetooth-comment = Bluetooth cihazlarını yönetin
xdg-entry-notifications-keywords = COSMIC;Bildirim;Kilit;
xdg-entry-default-apps-comment = Varsayılan web tarayıcısı, e-posta istemcisi, dosya yöneticisi ve diğer uygulamalar
xdg-entry-keyboard-comment = Giriş kaynakları, geçiş, özel karakter girişi, kısayollar
xdg-entry-power = Güç & Pil
xdg-entry-appearance-keywords = COSMIC;Vurgu;Renk;Simge;Yazı Tipi;Tema
xdg-entry-mouse = Fare
xdg-entry-dock-keywords = COSMIC;Dock;Panel;Eklenti
xdg-entry-mouse-comment = Fare hızı, ivme ve doğal kaydırma
xdg-entry-panel-keywords = COSMIC;Panel;Eklenti
xdg-entry-a11y = Erişilebilirlik
xdg-entry-desktop-keywords = COSMIC;Masaüstü;
xdg-entry-displays-comment = Ekran yapılandırma ayarlarını yönetin
xdg-entry-default-apps = Varsayılan Uygulamalar
xdg-entry-applications-comment = Varsayılan uygulamalar, başlangıç uygulamaları ve X11 uygulama uyumluluk ayarları
xdg-entry-about-keywords = COSMIC;Hakkında
xdg-entry-panel = Panel
xdg-entry-notifications = Bildirimler
xdg-entry-a11y-keywords = COSMIC;Erişilebilirlik;A11y;Ekran;Okuyucu;Büyüteç;Kontrast;Renk;
xdg-entry-desktop = Masaüstü
xdg-entry-date-time-keywords = COSMIC;Saat;Zaman Dilimi;
xdg-entry-desktop-comment = Duvar kağıdı, görünüm, panel, dock, pencere yönetimi ve çalışma alanı ayarları
xdg-entry-keyboard = Klavye
xdg-entry-a11y-comment = Ekran okuyucu, büyüteç, yüksek kontrast ve renk filtreleri
xdg-entry-displays = Ekranlar
xdg-entry-network = Ağ & Kablosuz
xdg-entry-date-time = Tarih & Saat
xdg-entry-input-comment = Klavye ve fare ayarları
xdg-entry-bluetooth-keywords = COSMIC;Bluetooth;
xdg-entry-dock-comment = Uygulamalar ve eklentiler için isteğe bağlı bir çubuk
xdg-entry-network-keywords = COSMIC;Ağ;Kablosuz;WiFi;VPN;
xdg-entry-dock = Dock
xdg-entry-about-comment = Cihaz adı, donanım bilgileri, işletim sistemi varsayılan ayarları
xdg-entry-appearance-comment = Vurgu renkleri ve tema
xdg-entry-input-keywords = COSMIC;Giriş;Klavye;Fare;Fareler;
xdg-entry-comment = COSMIC masaüstü için ayarlar uygulaması
xdg-entry-notifications-comment = Rahatsız Etme modu, kilit ekranı bildirimleri ve uygulama bazlı ayarlar
xdg-entry-applications = Uygulamalar
xdg-entry-default-apps-keywords = COSMIC;Varsayılan;Uygulama
xdg-entry-keywords = COSMIC;Ayarlar;
xdg-entry-about = Hakkında
xdg-entry-input = Giriş Cihazları
xdg-entry-applications-keywords = COSMIC;Varsayılan;Uygulama;Başlangıç;X11;Uyumluluk
xdg-entry-keyboard-keywords = COSMIC;Klavye;Giriş;Kaynak;Kısayollar;
xdg-entry-appearance = Görünüm
xdg-entry-network-comment = Ağ bağlantılarını yönetin
xdg-entry-panel-comment = Menüler ve eklentiler için birincil sistem çubuğu
xdg-entry-power-comment = Güç modları ve güç tasarrufu seçenekleri
xdg-entry-displays-keywords = COSMIC;Ekran;
xdg-entry-date-time-comment = Saat dilimi, otomatik saat ayarları ve zaman biçimlendirme
xdg-entry-touchpad = Dokunmatik yüzey
xdg-entry-wired = Kablolu
xdg-entry-startup-apps-comment = Oturum açıldığında çalışan uygulamaları yapılandırın
xdg-entry-region-language = Bölge & Dil
xdg-entry-startup-apps-keywords = COSMIC;Başlangıç;Uygulama;
xdg-entry-wired-keywords = COSMIC;Kablolu; Yerel Ağ; Ağ; Bağlantı;
xdg-entry-system = Sistem & Hesaplar
xdg-entry-window-management = Pencere Yönetimi
xdg-entry-time-language-comment = Sistem tarihini, saatini, bölgesini ve dilini yönetin
xdg-entry-x11-applications-keywords = COSMIC;X11;Uygulama;Oyun;Uyumluluk;
xdg-entry-touchpad-keywords = COSMIC;Dokunmatik yüzey; Hareket;
xdg-entry-time-language = Zaman & Dil
xdg-entry-users = Kullanıcılar
xdg-entry-system-keywords = COSMIC;Sistem;Bilgi;Hesaplar;Cihaz Yazılımı;
xdg-entry-wireless-keywords = COSMIC;WiFi;Wi-Fi;Ağ; Bağlantı;
xdg-entry-wireless-comment = Wi-Fi bağlantıları ve bağlantı profilleri
xdg-entry-wallpaper = Duvar Kağıdı
xdg-entry-users-comment = Kimlik doğrulama ve kullanıcı hesapları
xdg-entry-wallpaper-comment = Duvar kağıdı resimleri, renkleri ve slayt gösterisi seçenekleri
xdg-entry-startup-apps = Başlangıç Uygulamaları
xdg-entry-wireless = Wi-Fi
xdg-entry-workspaces-keywords = COSMIC;Çalışma Alanı; Yönlendirme; Genel Bakış; Monitör;
xdg-entry-system-comment = Sistem bilgileri, hesaplar ve cihaz yazılımı güncellemeleri
xdg-entry-x11-applications-comment = X11 pencere sistemi uygulama ölçeklendirmesi, birincil ekran ve genel kısayollar
xdg-entry-region-language-comment = Tarihleri, saatleri ve sayıları bölgenize göre biçimlendirin
xdg-entry-wallpaper-keywords = COSMIC;Duvar kağıdı; Arka plan; Slayt gösterisi;
xdg-entry-users-keywords = COSMIC;Kullanıcı;Hesap;
xdg-entry-vpn-keywords = COSMIC;VPN;Ağ;Bağlantı;OpenVPN;OpenConnect;
xdg-entry-time-language-keywords = COSMIC;Sistem;Saat;Tarih;Bölge;Dil;
xdg-entry-sound-keywords = COSMIC;Ses;Sesli;Uyarı;Kablolu Sistem;
xdg-entry-power-keywords = COSMIC;Güç;Pil
xdg-entry-region-language-keywords = COSMIC;Bölge;Dil;Tarih;Biçim;Saat;Yerel Ayar;Yerelleştirme;
xdg-entry-sound-comment = Cihazlar, uyarılar ve uygulamalar için ses ayarları
xdg-entry-touchpad-comment = Dokunmatik yüzey hızı, tıklama seçenekleri ve hareketler
xdg-entry-vpn-comment = VPN bağlantıları ve bağlantı profilleri
xdg-entry-window-management-keywords = COSMIC;Pencere;Yönetim;Döşeme;Süper;Anahtar;
xdg-entry-workspaces-comment = Çalışma alanı yönlendirmesi, genel bakış ve çoklu monitör davranışı
xdg-entry-sound = Ses
xdg-entry-workspaces = Çalışma Alanları
xdg-entry-x11-applications = X11 Uygulamaları Uyumluluğu
xdg-entry-window-management-comment = Süper tuş işlevi, pencere kontrol seçenekleri ve ek pencere döşeme seçenekleri
xdg-entry-wired-comment = Kablolu bağlantılar ve bağlantı profilleri
place-here = Eklentileri buraya yerleştir
sound-usb-audio = USB Ses
sound-device-profiles = Cihaz profilleri
sound-hd-audio = HD Ses
qr-code-unavailable = QR kodu mevcut değil
shadow-and-corners = Pencere gölgesi ve köşeler
workspaces-overview = Çalışma alanlarına genel bakış
    .action-on-typing = Yazma işlemi
    .none = Hiçbiri
    .launcher = Başlatıcıyı Aç
    .applications = Uygulamaları Aç
network-name = Ağ Adı
shadows-tiling = Döşemeli pencereler
    .clip = Sistem köşelerini eşleştir
    .shadow = Gölgeleri uygula
shadows-floating = Kayan pencereler
    .clip = Sistem köşelerini eşleştirin ve gölgeleri uygulayın
share = Ağı paylaş
sound-device-port-unplugged = Takılı değil
scan-to-connect-description = Bu ağa bağlanmak için QR kodunu tarayın.
