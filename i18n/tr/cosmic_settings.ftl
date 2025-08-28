app = COSMIC Ayarları

dbus-connection-error = DBus'a bağlanılamadı
ok = Tamam
unknown = Bilinmiyor

number = { $number }

## Network & Wireless

connections-and-profiles = { $variant ->
    [wired] Kablolu
    [wifi] Kablosuz
    [vpn] VPN
    *[other] Bilinmeyen
} bağlantılar ve bağlantı profilleri.

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
known-networks = Bilinen Ağlar
network-and-wireless = Ağ & Kablosuz
no-networks = Herhangi bir ağ bulunamadı.
no-vpn = Herhangi bir VPN bağlantısı bulunmamaktadır.
password = Parola
password-confirm = Parolayı Onayla
remove = Kaldır
settings = Ayarlar
username = Kullanıcı Adı
visible-networks = Görünen Ağlar
identity = Kimlik

auth-dialog = Kimlik Doğrulama Gerekli
    .vpn-description = VPN hizmeti tarafından istenilen kullanıcı adı ve parolayı girin.
    .wifi-description = Parola veya şifreleme anahtarını girin. Yönlendiricinin "WPS" düğmesine basarak da bağlanabilirsiniz.

forget-dialog = Bu kablosuz ağını unut?
    .description = Gelecekte bu kablosuz ağı kullanabilmek için tekrardan parola girmeniz gerekecek.

network-device-state =
    .activated = Bağlanıldı
    .config = Bağlanılıyor
    .deactivating = Bağlantı kesiliyor
    .disconnected = Bağlantı kesildi
    .failed = Bağlantı başarısız
    .ip-check = Bağlantı kontrol ediliyor
    .ip-config = IP ve yönlendirme bilgisi isteniyor
    .need-auth = Kimlik doğrulama gerekiyor
    .prepare = Preparing to connect
    .secondaries = İkincil bağlantı bekleniyor
    .unavailable = Mevcut değil
    .unknown = Bilinmeyen durum
    .unmanaged = Yönetilmiyor
    .unplugged = Kablo bağlı değil

remove-connection-dialog = Bağlantı profilini kaldır?
    .vpn-description = Gelecekte bu ağı kullanabilmek için tekrardan parola girmeniz gerekecek.
    .wired-description = Gelecekte bu profili kullanabilmek için tekrardan oluşturmanız gerekecek.

vpn = VPN
    .connections = VPN Bağlantıları
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
    .wireguard-device = WireGuard aygıtı oluşturulamadı
    .with-password = VPN ayarlanamadı { $field ->
        *[username] kullanıcı adı
        [password] parola
        [password-flags] password-flags
    } nmcli ile

wired = Kablolu
    .adapter = Kablolu adaptör { $id }
    .connections = Kablolu bağlantılar
    .devices = Kablolu aygıtlar
    .remove = Bağlantı profili kaldır

wifi = Kablosuz
    .adapter = Kablosuz adaptör { $id }
    .forget = Ağı unut

wireguard-dialog = WireGuard aygıtı ekle
    .description = WireGuard yapılandırması için aygıt adı belirleyin.

## Networking: Online Accounts

online-accounts = Çevrim İçi Hesaplar
    .desc = Hesap, IMAP ve SMTP, kurumsal giriş ekleyin

# Bluetooth

activate = Aktifleştir
confirm = Onayla
enable = Etkinleştir

bluetooth = Bluetooth
    .desc = Bluetooth aygıtlarını yönet
    .status = Bu sistem bluetooth ayarları açık olduğu sürede { $aliases } olarak görünür.
    .connected = Bağlandı
    .connecting = Bağlanılıyor
    .disconnecting = Bağlantı kesiliyor
    .connect = Bağlan
    .disconnect = Bağlantıyı kes
    .forget = Unut
    .dbus-error = DBus ile etkileşirken bir hata meydana geldi: { $why }
    .disabled = Bluetooth hizmeti devre dışı
    .inactive = Bluetooth hizmeti inaktif
    .unknown = Bluetooth hizmeti aktifleştirilemedi. bluez kurulu mu?

bluetooth-paired = Önceden Bağlanılan Aygıtlar
    .connect = Bağlan
    .battery = %{ $percentage } şarj

bluetooth-confirm-pin = Bluetooth PIN'ini Onayla
    .description = Lütfen aşağıdaki PIN'in { $device } aygıtında gözüken ile eşleştiğini onaylayın

bluetooth-available = Yakındaki Aygıtlar

bluetooth-adapters = Bluetooth Adaptörleri

## Accessibility

accessibility = Erişilebilirlik
    .vision = Görme
    .on = Açık
    .off = Kapalı
    .unavailable = Mevcut değil
    .high-contrast = Yüksek kontrast modu
    .invert-colors = Renkleri Ters Çevir
    .color-filters = Renk filtreleri


hearing = Duyma
    .mono = Stereo sesi mono olarak çal

default = Varsayılan
magnifier = Büyüteç
    .controls = Veya bu kısayolları kullanın: { $zoom_in ->
             [zero] {""}
            *[other] {""}
                Yakınlaştırmak için {$zoom_in},
        }{ $zoom_out ->
             [zero] {""}
            *[other] {""}
                Uzaklaştırmak için {$zoom_out},
        }
        Fareniz ile Super + fare tekeri
    .scroll_controls = Super + Fare tekeri kullanarak fare veya dokunmatik yüzey ile kaydırmayı etkinleştir.
    .show_overlay = Büyüteç Overlay'ini göster.
    .increment = Yakınlaştırma miktarı
    .signin = Büyüteci açılışta başlat
    .applet = Büyüteci panel bileşeninden açıp kapat
    .movement = Büyütülmüş görünümde gezinim
    .continuous = İmleç ile beraber
    .onedge = İmleç kenara ulaştığında
    .centered = İmleci merkezde tutarak
color-filter = Renk filtresi biçimi
    .unknown = Bilinmeyen Filtre etkin
    .greyscale = Gri tonlama
    .deuteranopia = Yeşil/Kırmızı (yeşil zayıflığı, Döteranopi)
    .protanopia = Kırmızı/Yeşil (kırmızı zayıflığı, Protanopi)
    .tritanopia = Blue/Yellow (mavi zayıflığı, Tritanopi)

## Desktop

desktop = Masaüstü

## Desktop: Wallpaper

wallpaper = Duvar Kağıdı
    .change = Belirli aralıklarda resmi değiştir
    .desc = Duvar kağıdı resimleri, renkleri ve slayt gösterisi seçenekleri.
    .fit = Duvar kağıdı yerleşimi
    .folder-dialog = Duvar kağıdı klasörü seçin
    .image-dialog = Duvar kağıdı resmi seçin
    .plural = Duvar kağıtları
    .same = Bütün ekranlarda aynı duvar kağıdı
    .slide = Slayt gösterisi

add-color = Renk ekle
add-image = Resim ekle
all-displays = Tüm Ekranlar
colors = Renkler
dialog-add = Ekle
fill = Doldur
fit-to-screen = Ekrana Sığdır
open-new-folder = Yeni klasör aç
recent-folders = Son Kullanılan Klasörler

x-minutes = { $number } dakika
x-hours = { $number } saat



never = Asla

## Desktop: Appearance

appearance = Görünüm
    .desc = Vurgu renkleri ve temalar.

accent-color = Vurgu rengi
app-background = Uygulama veya pencere arka planı
auto = Otomatik
close = Kapat
color-picker = Renk Seçici
copied-to-clipboard = Panoya kopyalandı
copy-to-clipboard = Panoya kopyala
dark = Koyu
export = Dışa aktar
hex = Hex
import = İçe aktar
light = Açık
mode-and-colors = Mod ve Renkler
recent-colors = Son kullanılan renkler
reset-to-default = Varsayılana dön
rgb = RGB
window-hint-accent = Etkin pencere belirten rengi
window-hint-accent-toggle = Etkin pencereyi belirtmek için tema vurgu rengini kullan

auto-switch = Otomatik olarak Açık ve Koyu modlar arasında geçiş yap
    .sunrise = Gün doğumunda Açık moda geçer
    .sunset = Gün batımında Koyu moda geçer
    .next-sunrise = Bir dahaki gün doğumunda Açık moda geçer
    .next-sunset = Bir dahaki gün doğumunda Koyu moda geçer

container-background = Konteyner arka planı
    .desc-detail = Konteyner arka planı gezinti yan çubuğu, yan çekmece, diyaloglar ve benzeri araçlar için kullanılır. Varsayılanda otomatik olarak Uygulama veya pencere arka planından belirlenir.
    .reset = Otomatiğe dön
    .desc = Birincil konteyner rengi gezinti yan çubuğu, yan çekmece, diyaloglar ve benzeri araçlar için kullanılır.

control-tint = Bileşen tonlamasını ayarla
    .desc = Standart düğmelerin, arama ve metin girdilerinin ve benzeri bileşenlerin arka planları için kullanılır.

frosted = Sistem arayüzünde buzlu cam efekti.
    .desc = Panel, dock, kabuk bileşenleri, başlatıcı ve uygulama kütüphanesi arka planına flu ekler

enable-export = Bu temayı GNOME uygulamalarına uygula.
    .desc = Bütün toolkitler otomatik tema değişimi desteklemez. COSMIC dışı uygulamaların tema değişimi için uygulamanın yeniden başlatılması gerekebilir.

icon-theme = Simge Teması
    .desc = Uygulamalara farklı birtakım simgeler uygular.

text-tint = Arayüz metin tonlaması
    .desc = Çeşitli yüzeylerde yeterli kontrasta sahip metinlerin rengini belirlemekte kullanılan renk

style = Stil
    .round = Yuvarlak
    .slightly-round = Yuvarlağımsı
    .square = Kare

interface-density = Arayüz Yoğunluğu
    .comfortable = Rahat
    .compact = Sıkışık
    .spacious = Geniş

window-management-appearance = Pencere Yönetimi
    .active-hint = Etkin pencere belirten boyutu
    .gaps = Dizili pencerelerin etrafındaki boşluk

### Experimental

experimental-settings = Deneysel Ayarlar
icons-and-toolkit = Simge ve toolkit temaları
interface-font = Sistem yazı tipi
monospace-font = Tek aralıklı yazı tipi

## Desktop: Notifications

notifications = Bildirimler
    .desc = Rahatsız Etme, kilit ekranı bildirimleri, ve uygulama başına ayarlar.

## Desktop: Panel

panel = Panel
    .desc = Masaüstü seçeneklerin ve menülerin bulunduğu üst panel.

add = Ekle
add-applet = Kabuk Bileşeni ekle
all = Tümü
applets = Kabuk Bileşenleri
center-segment = Orta Bölüm
drop-here = Bileşenleri buraya bırakın
end-segment = Son Bölüm
large = Büyük
no-applets-found = Kabuk bileşeni bulunamadı...
panel-bottom = Aşağı
panel-left = Sol
panel-right = Sağ
panel-top = Yukarı
search-applets = Bileşen ara...
small = Küçük
start-segment = Baş Bölüm

panel-appearance = Görünüm
    .match = Masaüstü ile aynı
    .light = Açık
    .dark = Koyu

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
    .dock-desc = Dock kabul bileşenlerini yapılandır.
    .desc = Panel kabuk bileşenlerini yapılandır.

panel-missing = Panel Yapılandırması Eksik
    .desc = Panel yapılandırma dosyası özel bir yapılandırması kullanıldığından veya bozulduğundan dolayı eksik.
    .fix = Varsayılana dön

## Desktop: Dock

dock = Görev Çubuğu
    .desc = Sabitlenmiş uygulamaların bulunduğu panel.

## Desktop: Window management

window-management = Pencere yönetimi
    .desc = Super tuşu davranışı, pencere kontrolü seçenekleri ve daha fazla pencere dizme seçenekleri.

super-key = Super tuşu davranışı
    .launcher = Başlatıcıyı aç
    .workspaces = Çalışma Alanlarını aç
    .applications = Uygulamaları aç
    .disable = Devre dışı bırak

edge-gravity = Yüzen pencereler kenarlara yapışır

window-controls = Pencere Kontrolleri
    .minimize = Küçültme tuşunu göster
    .maximize = Büyültme tuşunu göster
    .active-window-hint = Etkin pencere belirtmesini göster

focus-navigation = Odak gezinimi
    .focus-follows-cursor = Odak imleci takip eder
    .focus-follows-cursor-delay = Odak imleci milisaniye gecikme ile takip eder
    .cursor-follows-focus = İmleç odağı takip eder

## Desktop: Workspaces

workspaces = Çalışma Alanları
    .desc = Çalışma alan sayısını, davranışını, ve yerini değiştir.

workspaces-behavior = Çalışma Alanı Davranışı
    .dynamic = Dinamik çalışma alanları
    .dynamic-desc = Boş çalışma alanlarını otomatik olarak siler.
    .fixed = Sabit sayıda çalışma alanı
    .fixed-desc = Çalışma alanlarını genel görünümde kendiniz ekeyip çıkarın.

workspaces-multi-behavior = Çoklu-ekran Davranışı
    .span = Çalışma Alanları Ekranlara Yayılır
    .separate = Her Ekranın Kendi Çalışma Alanı Olur

workspaces-overview-thumbnails = Çalışma Alanları Genel Görünümü Küçük Resimleri
    .show-number = Çalışma Alanı Numarasını Göster
    .show-name = Çalışma Alanı Adını Göster

workspaces-orientation = Çalışma Alanları Yönelimi
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
    .desc = Ekranları, harici ekran kartı kullanımı ve gece ışığını yönet
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
    .id = Yansıtma { $id }
    .dont = Yansıtma
    .mirror = { $display } ekranını aynala
    .project = { $display ->
        [all] tüm ekranlara
        *[other] { $display } ekranına
    } yansıt
    .project-count = { $count } diğer ekrana yansıtılıyor




night-light = Gece Işığı
    .auto = Otomatik (gün doğumundan batışına)
    .desc = Daha sıcak renklerle mavi ışığı azalt.

orientation = Yönelim
    .standard = Standart
    .rotate-90 = 90 çevir
    .rotate-180 = 180 çevir
    .rotate-270 = 270 çevir

vrr = Değişken yenileme hızı
    .enabled = Etkin
    .force = Her zaman
    .auto = Otomatik
    .disabled = Devre dışı

scheduling = Programlama
    .manual = Manuel Program

dialog = Diyalog
    .title = Görüntü Ayarları Korunsun mu?
    .keep-changes = Değişiklikleri Koru
    .change-prompt = { $time } saniye içinde önceki görüntü ayarlarına dönülecek.
    .revert-settings = Ayarları Geri Al

## Sound

sound = Ses
    .desc = N/A

sound-output = Çıkış
    .volume = Çıkış sesi
    .device = Çıkış aygıtı
    .level = Çıkış düzeyi
    .config = Yapılandırma
    .balance = Denge
    .left = Sol
    .right = Sağ

sound-input = Giriş
    .volume = Giriş sesi
    .device = Giriş aygıtı
    .level = Giriş seviyesi

sound-alerts = Uyarılar
    .volume = Uyarı seviyesi
    .sound = Uyarı sesi

sound-applications = Uygulamalar
    .desc = Uygulama sesleri ve ayarları

profile = Profil

## Power

power = Güç & Pil
    .desc = Güç seçeneklerini yönet

battery = Pil
  .minute = { $value } dakika



  .hour = { $value } saat



  .day = { $value } gün



  .less-than-minute = Bir dakikadan az
  .and = ve
  .remaining-time = { $action ->
        [full] Dolmasına
       *[other] Bitmesine
   } { $time }

connected-devices = Bağlı Aygıtlar
  .unknown = Bilinmeyen aygıt

power-mode = Güç Modu
    .battery =  Uzatılmış pil ömrü
    .battery-desc = Azaltılmış güç kullanımı ve performans.
    .balanced = Dengeli
    .balanced-desc = Orta düzey performans ve güç kullanımı.
    .performance = Yüksek performans
    .performance-desc = Maksimum performans ve güç kullanımı.
    .no-backend = Arka uç bulunamadı. system76-power veya power-profiles-daemon kurun.

power-saving = Güç Tasarrufu Seçenekleri
    .turn-off-screen-after = Ekranı belirli süreden sonra kapat
    .auto-suspend = Otomatik askıya al
    .auto-suspend-ac = Şarjdayken otomatik askıya al
    .auto-suspend-battery = Pildeyken otomatik askıya al

## Input

acceleration-desc = Otomatik olarak takip hassasiyetini hıza bağlı olarak düzenle.

disable-while-typing = Yazarken devre dışı bırak.

input-devices = Giriş Aygıtları
    .desc = Giriş Aygıtları

primary-button = Birincil buton
    .desc = Fiziksel butonların sırasını belirler.
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
    .desc = Giriş kaynakları, aralarında geçiş, özel karakter girişi, kısayollar.

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
    .compose = Compose tuşu
    .caps = Caps Lock tuşu

keyboard-typing-assist = Yazma
    .repeat-rate = Tekrar hızı
    .repeat-delay = Tekrar gecikme süresi

keyboard-numlock-boot = Numlock
    .boot-state = Başlangıç durumu
    .last-boot = Son başlangıç
    .on = Açık
    .off = Kapalı
    .set = Numlock başlangıç durumunu ayarlayın

added = Eklendi
type-to-search = Aramak için yazın...
show-extended-input-sources = Genişletilmiş giriş kaynaklarını göster

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Klavye Kısayolları
    .desc = Kısayolları görüntüle ve kişileştir

add-keybinding = Tuş ataması ekle
cancel = İptal
command = Komut
custom = Özel
debug = Hata Ayıklama
disabled = Devre Dışı
input-source-switch = Klavye düzenini değiştir
migrate-workspace-prev = Çalışma alanını önceki çıktıya taşı
migrate-workspace-next = Çalışma alanını sonraki çıktıya taşı
migrate-workspace = Çalışma alanını { $direction ->
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
toggle-stacking = Pencere istiflemeyi kapat aç
type-key-combination = Tuş kombinasyonu girin

custom-shortcuts = Özel Kısayollar
    .add = Kısayol Ekle
    .context = Özel Kısayol Ekle
    .none = Özel kısayol yok

modified = { $count } düzenlenmiş

nav-shortcuts = Gezinim
    .prev-output = Önceki çıktıya odaklan
    .next-output = Sonraki çıktıya odaklan
    .last-workspace = Son çalışma alanına odaklan
    .prev-workspace = Önceki çalışma alanına odaklan
    .next-workspace = Sonraki çalışma alanına odaklan
    .focus = { $direction ->
        *[down] Aşağıdaki
        [in] İçerideki
        [left] Soldaki
        [out] Dışarıdaki
        [right] Sağdaki
        [up] Üstteki
    } pencereye odaklan
    .output = { $direction ->
        *[down] Aşağıdaki
        [left] Soldaki
        [right] Sağdaki
        [up] Üstteki
    } çıktıya geç
    .workspace = { $num }. çalışma alanına geç

manage-windows = Pencereleri yönet
    .close = Pencereyi kapat
    .maximize = Pencereyi büyüt
    .minimize = Pencereyi küçült
    .resize-inwards = Pencereyi içe doğru yeniden boyutlandır
    .resize-outwards = Pencereyi dışa doğru yeniden boyutlandır
    .toggle-sticky = Yapışkan pencereyi aç kapat

move-windows = Pencereyi Taşı
    .direction = Pencereyi { $direction ->
        *[down] aşağı
        [left] sola
        [right] sağa
        [up] yukarı
    } taşı
    .display = Pencereyi bir monitör { $direction ->
        *[down] aşağı
        [left] sola
        [right] sağa
        [up] yukarı
    } taşı
    .workspace = Pencereyi bir çalışma alanı { $direction ->
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
    .screenshot = Ekran görüntüsü al
    .terminal = Bir uçbirim aç
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
    .toggle-tiling = Pencere döşemeyi kapat aç
    .toggle-stacking = Pencere istiflemeyi kapat aç
    .toggle-floating = Pencere yüzdürmeyi kapat aç
    .toggle-orientation = Yönelimi değiştir

replace-shortcut-dialog = Kısayolu Değiştir?
    .desc = { $shortcut }, { $name } tarafından kullanılmakta. Eğer değiştirirseniz, { $name } devre dışı bırakılacak.

zoom-in = Yakınlaştır
zoom-out = Uzaklaştır

## Input: Mouse

mouse = Fare
    .desc = Fare hızı, ivmesi, doğal kaydırma.
    .speed = Fare hızı
    .acceleration = Fare ivmesini etkinleştir

## Input: Touchpad

click-behavior = Tıklama davranışı
    .click-finger = İki parmak ile ikincil tıklama ve üç parmak ile orta tıklama
    .button-areas = Sağ alt köşede ikincil tıklama ve orta alt kenarında orta tıklama

pinch-to-zoom = İki parmakla yakınlaştırma
    .desc = Yakınlaştırma destekleyen uygulamalar için iki parmağınıla sıkıştırarak ve genişleterek yaklaştırın ve uzaklaştırın.

tap-to-click = Tıklamak için dokun
    .desc = Tek parmak ile dokunuşta birincil, iki parmakla dokunuşta ikincil ve üç parmakla dokunuşta orta tıklamayı etkinleştir.

touchpad = Dokunmatik yüzey
    .acceleration = Dokunmatik yüzey ivmesini etkinleştir
    .desc = Dokunmatik yüzey hızı, tıklama seçenekleri ve hareketleri.
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

time = Zaman & Dil
    .desc = N/A

time-date = Tarih & Saat
    .desc = Saat dilimi, otomatik saat ayarları ve bazı zaman biçimleri.
    .auto = Otomatik ayarla
    .auto-ntp = Tarih & saat, saat dilimi ayarlandığında otomatik olarak güncellenir.

time-zone = Saat Dilimi
    .auto = Otomatik saat dilimi
    .auto-info = Konum hizmetlerine ve internet erişimine gereksim duyar

time-format = Tarih & Saat Biçimi
    .twenty-four = 24-saat biçimi
    .show-seconds = Show seconds
    .first = Haftanın ilk günü
    .show-date = Üst Panelde Tarihi Göster
    .friday = Cuma
    .saturday = Cumartesi
    .sunday = Pazar
    .monday = Pazartesi

time-region = Bölge & Dil
    .desc = Bölgenize göre tarih, saat ve sayıların biçimlendirin

formatting = Biçimlendirme
    .dates = Tarihler
    .time = Saat
    .date-and-time = Tarih & Saat
    .numbers = Sayılar
    .measurement = Ölçüm
    .paper = Kağıt

preferred-languages = Tercih Edilen Diller
    .desc = Dillerin sırası masaüstünün çevirisinde hangi dilin kullanılacağını belirler. Değişiklikler bir dahaki girişinizde geçerli olur.

add-language = Dil ekle
    .context = Dil Ekle
install-additional-languages = Ek dil kurun
region = Bölge

## Applications

applications = Uygulamalar

## Applications: Default Applications

default-apps = Varsayılan Uygulamalar
    .desc = Varsayılan web tarayıcısı, e-posta istemcisi, dosya gezgini ve diğer uygulamalar.
    .web-browser = Web tarayıcısı
    .file-manager = Dosya gezgini
    .mail-client = E-posta istemcisi
    .music = Müzik
    .video = Video
    .photos = Görseller
    .calendar = Takvim
    .terminal = Uçbirim
    .other-associations = Diğer ilişkiler
    .text-editor = Metin Düzenleyici

## Applications: Startup Applications

startup-apps = Başlangıç Uygulamaları
    .desc = Girişte çalıştırılacak uygulamaları ayarlayın.
    .add = Uygulama ekle
    .user = Kullanıcıya özgü uygulamalar
    .user-description = Bu uygulamalar mevcut kullanıcınıza giriş yaptığınızda başlatılır.
    .remove-dialog-title = { $name } ögesini kaldır?
    .remove-dialog-description = Bunu başlangıç uygulamalarından kaldırmak istediğinize emin misiniz?
    .search-for-application = Uygulama ara

## Applications: Legacy Applications

legacy-applications = X11 Uygulamaları Uyumluluğu
    .desc = X11 Pencere sistemi uygulama ölçeklemesi ve Genel kısayollar.

legacy-app-global-shortcuts = X11 Uygulamalarında Genel Kısayollar
    .desc = Genel kısayollar bir  uygulamda gerçekleştirilen fare ve klavye olaylarının diğer uygulamalar tarafından bas-konuş ve bas-sustur gibi özellikler için algılanmasını sağlar. Varsayılan olarak bu X11 uygulamalarında diğer uygulamaların hassas bilgiler içerebilen klavye ve fare olaylarını gözetlemesini engellemek için devre dışıdır.
    .none = Hiçbir tuş
    .modifiers = Modifierlar (Super, Shift, Control, Alt)
    .combination = Super, Control veya Alt tuşları basılı tutulduğu sürece tüm tuşlar
    .all = Tüm tuşlar
    .mouse = X11 uygulamalarında fare tuş olayları

legacy-app-scaling = X11 Pencere Sistemi Uygulama Ölçeklemesi
    .scaled-gaming = Oyun ve tam ekran uygulamalar için uyarla
    .gaming-description = X11 uygulamaları Wayland uygulamalarına göre biraz büyük veya küçük görünebilir.
    .scaled-applications = Uygulamalar için uyarla
    .applications-description = Oyunlar ve tam ekran X11 uygulamaları ekran çözünürlüğünüze uymayabilir.
    .scaled-compatibility = Maksimum uyumluluk modu
    .compatibility-description = X11 uygulamaları HiDPI ekranlarda bulanık gözükebilir.
    .preferred-display = Oyunlar ve tam ekran X11 uygulamaları için tercih edilen ekran
    .no-display = Hiçbiri

## System

system = Sistem & Hesaplar

## System: About

about = Hakkında
    .desc = Aygıt adı, donanım bilgisi, işletim sistemi varsayılanları.

about-device = Aygıt adı
    .desc = Bu isim, bluetooth aygıtlara ve ağdaki diğer aygıtlara gözükür.

about-hardware = Donanım
    .model = Donanım modeli
    .memory = Bellek
    .processor = İşlemci
    .graphics = Grafik
    .disk-capacity = Disk Kapasitesi

about-os = İşletim Sistemi
    .os = İşletim sistemi
    .os-architecture = İşletim sistemi mimarisi
    .desktop-environment = Masaüstü ortamı
    .windowing-system = Pencere sistemi

about-related = İlgili ayarlar
    .support = Destek al

## System: Firmware

firmware = Bellenim
    .desc = Bellenim detayları.

## System: Users

users = Kullanıcılar
    .desc = Kimlik doğrulama ve kullanıcı hesapları.
    .admin = Yönetici
    .standard = Standart
    .profile-add = Profil resmi seçin

administrator = Yönetici
    .desc = Yöneticiler tüm kullanıcılar için ayarları değiştirebilir, başka kullanıcılar ekleyebilir ve onları kaldırabilir.

add-user = Kullanıcı ekle
change-password = Parolayı değiştir
remove-user = Kullanıcıyı kaldır
full-name = Tam ad
invalid-username = Geçersiz kullanıcı adı.
password-mismatch = Parola ve doğrulama birbirine uymalıdır.
save = Kaydet
