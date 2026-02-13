app = Pengaturan COSMIC
ok = OKE
dbus-connection-error = Gagal menyambungkan ke DBus
unknown = Tidak diketahui
number = { $number }
add-network = Tambahkan jaringan
    .profile = Tambahkan profil
add-vpn = Tambahkan VPN
connections-and-profiles =
    Sambungan { $variant ->
        [wired] berkabel
        [wifi] Wi-Fi
        [vpn] VPN
       *[other] yang tidak diketahui
    } dan profil sambungan.
airplane-on = Mode pesawat nyala.
connect = Sambungkan
cable-unplugged = Kabel tercabut
connected = Tersambung
connecting = Menyambungkan…
disconnect = Putuskan
forget = Lupakan
known-networks = Jaringan yang diketahui
network-and-wireless = Jaringan & nirkabel
network-name = Nama Jaringan
no-networks = Tidak ada jaringan yang ditemukan.
no-vpn = Tidak ada sambungan VPN yang tersedia.
password = Kata sandi
password-confirm = Konfirmasikan kata sandi
qr-code-unavailable = Kode QR tidak tersedia
remove = Hapus
scan-to-connect-description = Pindai kode QR untuk menyambungkan ke jaringan ini.
settings = Pengaturan
share = Bagikan jaringan
username = Nama pengguna
visible-networks = Jaringan yang terlihat
identity = Identitas
auth-dialog = Autentikasi diperlukan
    .vpn-description = Masukkan nama pengguna dan kata sandi yang dibutuhkan oleh layanan VPN.
    .wifi-description = Masukkan kata sandi atau kunci enkripsi. Anda juga dapat menyambungkan dengan menekan tombol “WPS” pada router.
forget-dialog = Lupakan jaringan Wi-Fi ini?
    .description = Anda akan perlu memasukkan kata sandi lagi untuk menggunakan jaringan Wi-Fi ini di masa mendatang.
network-device-state =
    .activated = Tersambung
    .config = Menyambungkan
    .deactivating = Memutuskan
    .disconnected = Terputus
    .failed = Gagal tersambung
    .ip-check = Memeriksa sambungan
    .ip-config = Meminta info IP dan routing
    .need-auth = Memerlukan autentikasi
    .prepare = Bersiap tersambung
    .secondaries = Menunggu untuk sambungan kedua
    .unavailable = Tidak tersedia
    .unknown = Status tidak diketahui
    .unmanaged = Tidak terkelola
    .unplugged = Kabel tercabut
remove-connection-dialog = Hapus profil sambungan?
    .vpn-description = Anda akan perlu memasukkan kata sandi lagi untuk menggunakan jaringan ini di masa mendatang.
    .wired-description = Anda akan perlu membuat ulang profil ini untuk menggunakannya di masa mendantang.
vpn = VPN
    .connections = Sambungan VPN
    .error = Gagal menambahkan konfigurasi VPN
    .remove = Hapus profil sambungan
    .select-file = Pilih berkas konfigurasi VPN
vpn-error = Galat VPN
    .config = Gagal menambahkan konfigurasi VPN
    .connect = Gagal menyambungkan ke VPN
    .connection-editor = Penyunting sambungan gagal
    .connection-settings = Gagal mendapatkan pengaturan untuk sambungan aktif
    .updating-state = Gagal memperbarui status pengelola jaringan
    .wireguard-config-path = Jalur berkas tidak valid untuk konfigurasi WireGuard
    .wireguard-config-path-desc = Berkas yang dipiih harus di sistem berkas lokal.
    .wireguard-device = Gagal membuat perangkat WireGuard
    .with-password =
        Gagal mengatur { $field ->
           *[username] nama pengguna
            [password] kata sandi
            [password-flags] penanda kata sandi
        } VPN dengan nmcli
wired = Berkabel
    .adapter = Adaptor berkabel { $id }
    .connections = Sambungan berkabel
    .devices = Perangkat berkabel
    .remove = Hapus profil sambungan
wifi = Wi-Fi
    .adapter = Adaptor Wi-Fi { $id }
    .forget = Lupakan jaringan ini
wireguard-dialog = Tambahkan perangkat WireGuard
    .description = Pilih nama perangkat untuk konfigurasi WireGuard.
online-accounts = Akun daring
    .desc = Tambahkan akun, IMAP dan SMTP, masuk perusahaan
activate = Aktifkan
confirm = Konfirmasikan
enable = Aktifkan
bluetooth = Bluetooth
    .desc = Kelola perangkat Bluetooth
    .status = Sistem ini terlihat sebagai { $aliases } saat pengaturan Bluetooth dibuka.
    .connected = Tersambung
    .connecting = Menyambungkan
    .disconnecting = Memutuskan
    .connect = Sambungkan
    .disconnect = Putuskan
    .forget = Lupakan
    .dbus-error = Terjadi galat saat berinteraksi dengan DBus: { $why }
    .disabled = Layanan Bluetooth dinonaktifkan
    .inactive = Layanan Bluetooh tidak aktif
    .unknown = Layanan Bluetooth tidak dapat diaktifkan. Apakah BlueZ terpasang?
bluetooth-paired = Perangkat yang tersambung sebelumnya
    .connect = Sambungkan
    .battery = Baterai { $percentage }%
bluetooth-confirm-pin = Konfirmasikan PIN Bluetooth
    .description = Silakan konfirmasikan bahwa PIN berikut cocok dengan PIN yang ditampilkan pada { $device }
bluetooth-available = Perangkat terdekat
bluetooth-adapters = Adaptor Bluetooth
accessibility = Aksesibilitas
    .vision = Penglihatan
    .on = Nyala
    .off = Mati
    .unavailable = Tidak tersedia
    .screen-reader = Pembaca layar
    .high-contrast = Mode kontras tinggi
    .invert-colors = Balikkan warna
    .color-filters = Filter warna
hearing = Pendengaran
    .mono = Mainkan audio stereo sebagai mono
default = Bawaan
magnifier = Kaca Pembesar
    .controls =
        Atau gunakan pintasan ini: { $zoom_in ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_in } untuk perbesar,
        }{ $zoom_out ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_out } untuk perkecil,
        }
        Super + gulir menggunakan tetikus anda
    .scroll_controls = Aktifkan tetikus atau zum papan sentuh dengan Super + gulir
    .show_overlay = Tampilkan lapisan kaca pembesar
    .increment = Zum tambahan
    .signin = Mulai kaca pembesar saat masuk
    .applet = Ubah kaca pembesar nyala/mati dalam applet di panel
    .movement = Tampilan yang dizum bergerak
    .continuous = Terus menerus dengan penunjuk
    .onedge = Saat penunjuk mencapai tepi
    .centered = Untuk mempertahankan penunjuk berada di tengah
workspaces-behavior = Perilaku ruang kerja
    .dynamic = Ruang kerja dinamis
    .dynamic-desc = Menghapus ruang kerja kosong secara otomatis.
    .fixed = Jumlah ruang kerja tetap
    .fixed-desc = Tambahkan atau hapus ruang kerja di ikhtisar.
color-filter = Jenis filter warna
    .unknown = Filter aktif tidak diketahui
    .greyscale = Skala abu-abu
    .deuteranopia = Hijau/Merah (kelemahan hijau, Deuteranopia)
    .protanopia = Merah/Hijau (kelemahan merah, Protanopia)
    .tritanopia = Biru/Kuning (kelemahan biru, Tritanopia)
desktop = Desktop
wallpaper = Wallpaper
    .change = Ubah gambar setiap
    .desc = Opsi gambar wallpaper, warna, dan tayangan salindia
    .fit = Sesuaikan wallpaper
    .folder-dialog = Pilih map wallpaper
    .image-dialog = Pilih gambar wallpaper
    .plural = Wallpaper
    .same = Wallpaper yang sama di semua layar
    .slide = Tayangan salindia
add-color = Tambahkan warna
add-image = Tambahkan gambar
all-displays = Semua layar
colors = Warna
dialog-add = Tambahkan
fill = Isi
fit-to-screen = Sesuaikan dengan layar
open-new-folder = Buka map baru
recent-folders = Map terbaru
x-minutes =
    { $number } { $number ->
        [one] menit
       *[other] menit
    }
x-hours =
    { $number } { $number ->
        [one] jam
       *[other] jam
    }
never = Tidak pernah
appearance = Tampilan
    .desc = Warna aksen dan tema
accent-color = Warna aksen
app-background = Latar belakang jendela
auto = Otomatis
close = Tutup
color-picker = Pemilih warna
copied-to-clipboard = Disalin ke papan klip
copy-to-clipboard = Salin ke papan klip
dark = Gelap
export = Ekspor
hex = Heks
import = Impor
light = Terang
mode-and-colors = Mode dan warna
recent-colors = Warna terbaru
reset-to-default = Atur ulang ke bawaan
rgb = RGB
window-hint-accent = Warna petunjuk jendela aktif
window-hint-accent-toggle = Gunakan warna aksen tema sebagai petunjuk jendela aktif
auto-switch = Mengalihkan antara mode terang dan gelap secara otomatis
    .sunrise = Mengalihkan ke mode terang saat matahari terbit
    .sunset = Mengalihkan ke mode gelap saat matahari terbenam
    .next-sunrise = Mengalihkan ke mode terang saat matahari terbit selanjutnya
    .next-sunset = Mengalihkan ke mode gelap saat matahari terbenam selanjutnya
shadows-floating = Jendela mengapung
    .clip = Cocokkan sudut sistem dan terapkan bayangan
shadows-tiling = Jendela berubin
    .clip = Cocokkan sudut sistem
    .shadow = Terapkan bayangan
container-background = Latar belakang kontainer
    .desc-detail = Warna latar belakang kontainer digunakan untuk bilah sisi navigasi, laci samping, dialog dan widget serupa. Secara bawaan, warna latar belakang kontainer diambil dari latar belakang jendela secara otomatis.
    .reset = Atur ulang ke otomatis
    .desc = Digunakan untuk bilah sisi navigasi, laci samping, dialog dan widget serupa
control-tint = Warna komponen kontrol
    .desc = Digunakan untuk latar belakang tombol standar, masukan pencarian, masukan teks, dan komponen serupa
frosted = Efek kaca buram pada antarmuka sistem
    .desc = Menerapkan blur latar belakang ke panel, dok, applet, peluncur, dan pustaka aplikasi
enable-export = Terapkan tema saat ini ke aplikasi GNOME
    .desc = Tidak semua alat kit mendukung peralihan otomatis. Aplikasi non-COSMIC mungkin perlu dimulai ulang seletah perubahan tema.
icon-theme = Tema ikon
    .desc = Menerapkan serangkaian ikon yang berbeda ke aplikasi
text-tint = Warna teks antarmuka
    .desc = Digunakan untuk mendapatkan warna teks antarmuka yang memiliki kontras yang cukup pada berbagai permukaan
style = Gaya
    .round = Bulat
    .slightly-round = Sedikit bulat
    .square = Persegi
interface-density = Kepadatan antarmuka
    .comfortable = Nyaman
    .compact = Kompak
    .spacious = Luas
window-management-appearance = Pengelolaan jendela
    .active-hint = Ukuran petunjuk jendela aktif
    .gaps = Jarak sekitar jendela berubin
experimental-settings = Pengaturan eksperimental
icons-and-toolkit = Tema ikon dan alat kit
interface-font = Huruf sistem
monospace-font = Huruf monospace
shadow-and-corners = Bayangan dan sudut jendela
notifications = Notifikasi
    .desc = Jangan Ganggu, notifikasi layar kunci, dan pengaturan per aplikasi
panel = Panel
    .desc = Bar sistem utama untuk menu dan applet
add = Tambahkan
add-applet = Tambahkan applet
all = Semua
applets = Applet
center-segment = Segmen tengah
place-here = Letakkan applet di sini
replace = Ganti
replace-shortcut-dialog = Ganti pintasan?
    .desc = { $shortcut } digunakan oleh { $name }. Jika anda menggantikannya, { $name } akan dinonaktifkan.
end-segment = Segmen akhir
large = Besar
no-applets-found = Tidak ada applet yang ditemukan...
panel-bottom = Bawah
panel-left = Kiri
panel-right = Kanan
panel-top = Atas
search-applets = Cari applet...
small = Kecil
start-segment = Segmen awal
panel-appearance = Tampilan
    .match = Cocokkan dekstop
    .light = Terang
    .dark = Gelap
panel-behavior-and-position = Perilaku dan posisi
    .autohide = Sembunyikan panel secara otomatis
    .dock-autohide = Sembunyikan dok secara otomatis
    .position = Posisi di layar
    .display = Tampilkan di layar
panel-style = Gaya
    .anchor-gap = Jarak antara panel dan tepi layar
    .dock-anchor-gap = Jarak antara dok dan tepi layar
    .extend = Luaskan panel ke tepi layar
    .dock-extend = Luaskan dok ke tepi layar
    .appearance = Tampilan
    .size = Ukuran
    .background-opacity = Opasitas latar belakang
panel-applets = Konfigurasi
    .dock-desc = Konfigurasikan applet dok
    .desc = Konfigurasikan applet panel
panel-missing = Konfigurasi panel hilang
    .desc = Berkas konfigurasi panel hilang karena penggunaan konfigurasi kustom atau berkas tersebut rusak.
    .fix = Atur ulang ke bawaan
dock = Dok
    .desc = Bar opsional untuk aplikasi dan applet
window-management = Pengelolaan jendela
    .desc = Tindakan tombol super, opsi kontrol jendela, dan opsi pengubinan jendela tambahan
super-key = Tindakan tombol super
    .launcher = Buka Peluncur
    .workspaces = Buka Ruang Kerja
    .applications = Buka Aplikasi
    .disable = Nonaktifkan
edge-gravity = Jendela mengapung bergerak ke tepi terdekat
window-controls = Kontrol jendela
    .maximize = Tampilkan tombol maksimalkan
    .minimize = Tampilkan tombol minimalkan
    .active-window-hint = Tampilkan petunjuk jendela aktif
focus-navigation = Navigasi fokus
    .focus-follows-cursor = Fokus mengikuti kursor
    .focus-follows-cursor-delay = Fokus mengikuti penundaan kursor dalam md
    .cursor-follows-focus = Kursor mengikuti fokus
workspaces = Ruang kerja
    .desc = Orientasi dan perilaku ruang kerja
workspaces-multi-behavior = Perilaku multi-monitor
    .span = Ruang kerja mencakup layar
    .separate = Layar memiliki ruang kerja terpisah
workspaces-overview-thumbnails = Gambaran ikhtisar ruang kerja
    .show-number = Tampilkan angka ruang kerja
    .show-name = Tampilkan nama ruang kerja
workspaces-orientation = Orientasi ruang kerja
    .vertical = Vertikal
    .horizontal = Horisontal
hot-corner = Sudut Panas
    .top-left-corner = Aktifkan sudut panas kiri atas untuk Ruang kerja
-requires-restart = Memerlukan mulai ulang
color = Warna
    .depth = Kedalaman warna
    .profile = Profil warna
    .sidebar = Profil Warna
    .temperature = Suhu warna
display = Layar
    .desc = Kelola layar dan cahaya malam
    .arrangement = Susunan layar
    .arrangement-desc = Seret layar untuk menyusun ulang posisinya
    .enable = Aktifkan layar
    .external = Layar eksternal { $size } { $output }
    .laptop = Layar laptop { $size }
    .options = Opsi layar
    .refresh-rate = Kecepatan penyegaran
    .resolution = Resolusi
    .scale = Skala
    .additional-scale-options = Opsi skala tambahan
mirroring = Pencerminan
    .id = Mencerminkan { $id }
    .dont = Jangan cerminkan
    .mirror = Cerminkan { $display }
    .project =
        Proyekkan ke { $display ->
            [all] semua layar
           *[other] { $display }
        }
    .project-count =
        Memproyekkan ke { $count } { $count ->
            [1] layar
           *[other] layar
        } lainnya
night-light = Cahaya malam
    .auto = Otomatis (terbenam ke terbit)
    .desc = Mengurangi cahaya biru dengan warna hangat
orientation = Orientasi
    .standard = Standar
    .rotate-90 = Putar 90°
    .rotate-180 = Putar 180°
    .rotate-270 = Putar 270°
vrr = Kecepatan penyegaran variabel
    .enabled = Diaktifkan
    .force = Selalu
    .auto = Otomatis
    .disabled = Dinonaktifkan
scheduling = Penjadwalan
    .manual = Jadwal manual
dialog = Dialog
    .title = Pertahankan pengaturan layar ini?
    .keep-changes = Pertahankan perubahan
    .change-prompt = Pengaturan perubahan akan dikembalikan secara otomatis dalam { $time } detik.
    .revert-settings = Pengaturan pengembalian
sound = Suara
    .desc = T/A
sound-output = Keluaran
    .volume = Volume keluaran
    .device = Perangkat keluaran
    .level = Tingkat keluaran
    .config = Konfigurasi
    .balance = Seimbang
    .left = Kiri
    .right = Kanan
sound-input = Masukan
    .volume = Volume masukan
    .device = Perangkat masukan
    .level = Tingkat masukan
amplification = Amplifikasi
    .desc = Memungkinkan meningkatkan volume ke 150%
sound-alerts = Peringatan
    .volume = Volume peringatan
    .sound = Suara peringatan
sound-applications = Aplikasi
    .desc = Volume dan pengaturan aplikasi
sound-device-port-unplugged = Tercabut
sound-hd-audio = Audio HD
sound-usb-audio = Audio USB
sound-device-profiles = Profil perangkat
power = Daya & baterai
    .desc = Kelola pengaturan daya
battery = Baterai
    .minute =
        { $value } { $value ->
            [one] menit
           *[other] menit
        }
    .hour =
        { $value } { $value ->
            [one] jam
           *[other] jam
        }
    .day =
        { $value } { $value ->
            [one] hari
           *[other] hari
        }
    .less-than-minute = Kurang dari satu menit
    .and = dan
    .remaining-time =
        { $time } hingga { $action ->
            [full] penuh
           *[other] kosong
        }
connected-devices = Perangkat yang tersambung
    .unknown = Perangkat yang tidak diketahui
power-mode = Mode daya
    .battery = Masa pakai baterai diperluas
    .battery-desc = Penggunaan daya berkurang dan kinerja senyap
    .balanced = Seimbang
    .balanced-desc = Kinerja senyap dan penggunaan daya sedang
    .performance = Kinerja tinggi
    .performance-desc = Kinerja puncak dan penggunaan daya
    .no-backend = Backend tidak ditemukan. Pasang system76-power atau power-profiles-daemon.
acceleration-desc = Menyesuaikan sensitivitas pelacakan berdasarkan kecepatan secara otomatis
disable-while-typing = Nonaktifkan saat mengetik
slow = Lambat
fast = Cepat
short = Pendek
long = Panjang
added = Ditambahkan
type-to-search = Ketik untuk mencari...
show-extended-input-sources = Tampilkan sumber masukan yang diperluaskan
add-another-keybinding = Tambahkan pintasan tombol lainnya
cancel = Batalkan
command = Perintah
custom = Kustom
debug = Awakutu
disabled = Dinonaktifkan
input-source-switch = Alihkan sumber masukan bahasa papan ketik
migrate-workspace-prev = Migrasikan ruang kerja ke keluaran sebelumnya
migrate-workspace-next = Migrasikan ruang kerja ke keluaran selanjutnya
navigate = Navigasikan
shortcut-name = Nama pintasan
system-controls = Kontrol sistem
terminate = Akhiri
toggle-stacking = Ubah penumpukan jendela
type-key-combination = Ketik kombinasi tombol
modified = { $count } dimodifikasi
zoom-in = Perbesar
zoom-out = Perkecil
switch-between-windows = Alihkan antar jendela
open-application-library = Buka Pustaka Aplikasi
open-workspaces-view = Buka Ikhtisar Ruang Kerja
install-additional-languages = Pasang bahasa tambahan
region = Wilayah
applications = Aplikasi
system = Sistem & akun
add-user = Tambahkan pengguna
change-password = Ubah kata sandi
remove-user = Hapus pengguna
full-name = Nama lengkap
invalid-username = Nama pengguna tidak valid
password-mismatch = Kata sandi dan konfirmasi harus cocok
save = Simpan
administrator = Administrator
    .desc = Administrator dapat mengubah pengaturan untuk semua pengguna, menambahkan atau menghapus pengguna lainnya
users = Pengguna
    .desc = Autentikasi dan akun pengguna
    .admin = Admin
    .standard = Standar
    .profile-add = Pilih gambar profil
firmware = Firmware
    .desc = Rincian firmware
about-related = Pengaturan terkait
    .support = Dapatkan dukungan
about-os = Sistem operasi
    .os = Sistem operasi
    .os-architecture = Arsitektur sistem operasi
    .kernel = Versi kernel
    .desktop-environment = Lingkungan desktop
    .windowing-system = Sistem penjendelaan
about-hardware = Perangkat keras
    .model = Model perangkat keras
    .memory = Memori
    .processor = Prosesor
    .graphics = Grafis
    .disk-capacity = Kapasitas cakram
about-device = Nama perangkat
    .desc = Nama ini akan muncul ke jaringan lainnya atau perangkat Bluetooth
about = Tentang
    .desc = Nama perangkat, informasi perangkat keras, bawaan sistem operasi
legacy-applications = Kompatibilitas aplikasi X11
    .desc = Penskalaan aplikasi sistem Jendela X11 dan Pintasan global
add-language = Tambahkan bahasa
    .context = Tambahkan bahasa
preferred-languages = Bahasa pilihan
    .desc = Urutan bahasa menentukan bahasa mana yang digunakan untuk antarmuka pengguna. Perubahan akan berlaku saat masuk selanjutnya.
formatting = Pemformatan
    .dates = Tanggal
    .time = Waktu
    .date-and-time = Tanggal & waktu
    .numbers = Angka
    .measurement = Pengukuran
    .paper = Kertas
time-region = Wilayah & bahasa
    .desc = Format tanggal, waktu, dan angka berdasarkan wilayah anda
time-zone = Zona waktu
    .auto = Zona waktu otomatis
    .auto-info = Memerlukan layanan lokasi dan akses internet
time-date = Tanggal & waktu
    .desc = Zona waktu, pengaturan jam otomatis, dan pemformatan waktu
    .auto = Atur secara otomatis
    .auto-ntp = Tanggal & waktu akan diperbarui secara otomatis saat zona waktu diatur
time = Waktu & bahasa
    .desc = T/A
switch-workspaces = Alihkan ruang kerja
    .horizontal = Geser empat jari ke kiri/kanan
    .vertical = Geser empat jari ke atas/bawah
input-devices = Perangkat masukan
    .desc = Perangkat masukan
primary-button = Tombol utama
    .desc = Mengatur urutan tombol fisik
    .left = Kiri
    .right = Kanan
scrolling = Pengguliran
    .two-finger = Gulir dengan dua jari
    .edge = Gulir di sepanjang tepi dengan satu jari
    .speed = Kecepatan pengguliran
    .natural = Pengguliran natural
    .natural-desc = Gulir konten, bukan tampilan
keyboard = Papan Ketik
    .desc = Sumber masukan, pengalihan, entri karakter spesial, pintasan
keyboard-sources = Sumber Masukan
    .desc = Sumber masukan dapat dialihkan menggunakan kombinasi tombol Super+Spasi. Ini dapat disesuaikan di pengaturan pintasan papan ketik.
    .move-up = Pindahkan ke atas
    .move-down = Pindahkan ke bawah
    .settings = Pengaturan
    .view-layout = Tampilkan tata letak papan ketik
    .remove = Hapus
    .add = Tambahkan sumber masukan
keyboard-special-char = Entri Karakter Spesial
    .alternate = Ganti tombol karakter
    .compose = Tombol compose
    .compose-desc = Tombol compose memungkinkan berbagai macam karakter untuk memasukkan. Untuk menggunakannya, tekan compose dan kemudian serangkaian karakter. Misalnya, tombol compose diikuti dengan C dan o akan memasukkan ©, sedangkan tombol compose diikuti dengan a dan ‘ akan memasukkan á.
    .caps = Tombol Caps Lock
keyboard-typing-assist = Pengetikan
    .repeat-rate = Kecepatan pengulangan
    .repeat-delay = Penundaan pengulangan
keyboard-numlock-boot = Numlock
    .boot-state = Status saat boot
    .last-boot = Boot terakhir
    .on = Nyala
    .off = Mati
    .set = Atur status boot numlock
keyboard-shortcuts = Pintasan papan ketik
    .desc = Tampilkan dan sesuaikan pintasan
power-saving = Opsi penghematan daya
    .turn-off-screen-after = Matikan layar setelah
    .auto-suspend = Hentikan otomatis
    .auto-suspend-ac = Hentikan otomatis saat dicolok
    .auto-suspend-battery = Hentikan otomatis saat daya baterai
custom-shortcuts = Pintasan kustom
    .add = Tambahkan pintasan
    .context = Tambahkan pintasan kustom
    .none = Tidak ada pintasan kustom
manage-windows = Kelola jendela
    .close = Tutup jendela
    .maximize = Maksimalkan jendela
    .fullscreen = Layar penuh jendela
    .minimize = Minimalkan jendela
    .resize-inwards = Ukur ulang jendela ke dalam
    .resize-outwards = Ukur ulang jendela ke luar
    .toggle-sticky = Ubah jendela lengket
tap-to-click = Ketuk untuk klik
    .desc = Mengaktifkan ketukan satu jari untuk klik utama, ketukan dua jari untuk klik kedua, dan ketukan tiga jari untuk klik tengah
touchpad = Papan Sentuh
    .acceleration = Aktifkan akselerasi papan sentuh
    .desc = Kecepatan papan sentuh, opsi klik, gerakan
    .speed = Kecepatan papan sentuh
mouse = Tetikus
    .desc = Kecepatan tetikus, akselerasi, pengguliran natural
    .speed = Kecepatan tetikus
    .acceleration = Aktifkan akselerasi tetikus
click-behavior = Perilaku Klik
    .click-finger = Klik kedua dengan dua jari dan klik tengah dengan tiga jari
    .button-areas = Klik kedua di sudut kanan bawah dan klik tengah di tengah bawah
pinch-to-zoom = Jepit untuk memperbesar
    .desc = Gunakan dua jari untuk memperbesar konten, untuk aplikasi yang mendukung zum
gestures = Gerakan
    .four-finger-down = Geser empat jari ke bawah
    .four-finger-left = Geser empat jari ke kiri
    .four-finger-right = Geser empat jari ke kanan
    .four-finger-up = Geser empat jari ke atas
    .three-finger-any = Geser tiga jari ke segala arah
migrate-workspace =
    Migrasikan ruang kerja ke keluaran { $direction ->
       *[down] bawah
        [left] kiri
        [right] kanan
        [up] atas
    }
window-tiling = Pengubinan jendela
    .horizontal = Atur orientasi horisontal
    .vertical = Atur orientasi vertikal
    .swap-window = Tukar jendela
    .toggle-tiling = Ubah pengubinan jendela
    .toggle-stacking = Ubah penumpukan jendela
    .toggle-floating = Ubah pengapungan jendela
    .toggle-orientation = Ubah orientasi
time-format = Format tanggal & waktu
    .twenty-four = Waktu 24 jam
    .show-seconds = Tampilkan detik
    .first = Hari pertama dalam mingguan
    .show-date = Tampilkan tanggal di applet waktu
    .friday = Jum'at
    .saturday = Sabtu
    .sunday = Minggu
    .monday = Senin
default-apps = Aplikasi Bawaan
    .desc = Peramban web bawaan, klien surel, peramban berkas, dan aplikasi lainnya
    .web-browser = Peramban web
    .file-manager = Pengelola berkas
    .mail-client = Klien surel
    .music = Musik
    .video = Video
    .photos = Foto
    .calendar = Kalender
    .terminal = Terminal
    .other-associations = Asosiasi lainnya
    .text-editor = Penyunting Teks
    .not-installed = Tidak dipasang
startup-apps = Aplikasi pemulaian
    .desc = Konfigurasikan aplikasi yang berjalan saat masuk
    .add = Tambahkan aplikasi
    .user = Aplikasi yang diluncurkan saat anda masuk
    .none = Tidak ada aplikasi pemulaian yang ditambahkan
    .remove-dialog-title = Hapus { $name }?
    .remove-dialog-description = Hapus aplikasi pemulaian ini?
    .add-startup-app = Tambahkan aplikasi pemulaian
legacy-app-global-shortcuts = Pintasan global di aplikasi X11
    .desc = Pintasan global memungkinkan penekanan tombol papan ketik dan tetikus yang dilakukan dalam aplikasi dikenali oleh aplikasi lain untuk fitur seperti tekan untuk bicara atau tekan untuk membisukan. Secara bawaan, pintasan global dinonaktifkan di aplikasi X11 untuk memastikan aplikasi lain tidak dapat memantau peristiwa papan ketik dan tetikus yang berisi informasi sensitif.
    .none = Tidak ada tombol
    .modifiers = Pengubah (Super, Shift, Control, Alt)
    .combination = Semua tombol beserta pengubah Super, Control atau Alt ditekan
    .all = Semua tombol
    .mouse = Peristiwa tombol tetikus di aplikasi X11
legacy-app-scaling = Penskalaan aplikasi sistem jendela X11
    .scaled-gaming = Optimalkan untuk permainan dan aplikasi layar penuh
    .gaming-description = Aplikasi X11 mungkin tampak sedikit lebih besar/kecil dibandingkan dengan aplikasi Wayland
    .scaled-applications = Optimalkan untuk aplikasi
    .applications-description = Permainan dan aplikasi X11 layar penuh mungkin tidak cocok dengan resolusi layar anda
    .scaled-compatibility = Mode kompatibilitas maksimum
    .compatibility-description = Aplikasi X11 mungkin tampak buram pada layar HiDPI
    .preferred-display = Layar pilihan untuk permainan dan aplikasi X11 layar penuh
    .no-display = Tidak ada
system-shortcut = Sistem
    .app-library = Buka pustaka aplikasi
    .brightness-down = Kurangi kecerahan layar
    .brightness-up = Tingkatkan kecerahan layar
    .display-toggle = Ubah layar internal
    .home-folder = Buka map beranda
    .keyboard-brightness-down = Kurangi kecerahan papan ketik
    .keyboard-brightness-up = Tingkatkan kecerahan papan ketik
    .launcher = Buka Peluncur
    .log-out = Keluar
    .lock-screen = Kunci layar
    .mute = Matikan keluaran audio
    .mute-mic = Matikan masukan mikrofon
    .play-pause = Putar/jeda
    .play-next = Lagu selanjutnya
    .play-prev = Lagu sebelumnya
    .poweroff = Matikan daya
    .screenshot = Ambil tangkapan layar
    .suspend = Hentikan
    .terminal = Buka terminal
    .touchpad-toggle = Ubah papan sentuh
    .volume-lower = Kurangi volume keluaran audio
    .volume-raise = Tingkatkan volume keluaran audio
    .web-browser = Buka peramban web
    .window-switcher = Alihkan antar jendela yang dibuka
    .window-switcher-previous = Alihkan antar jendela yang dibuka secara terbalik
    .workspace-overview = Buka ikhtisar ruang kerja
nav-shortcuts = Navigasi
    .prev-output = Fokus keluaran sebelumnya
    .next-output = Fokus keluaran selanjutnya
    .last-workspace = Fokus ruang kerja terakhir
    .prev-workspace = Fokus ruang kerja sebelumnya
    .next-workspace = Fokus ruang kerja selanjutnya
    .focus =
        Fokus jendela { $direction ->
           *[down] bawah
            [in] dalam
            [left] kiri
            [out] luar
            [right] kanan
            [up] atas
        }
    .output =
        Alihkan ke keluaran { $direction ->
           *[down] bawah
            [left] kiri
            [right] kanan
            [up] atas
        }
    .workspace = Alihkan ke ruang kerja { $num }
move-windows = Pindahkan jendela
    .direction =
        Pindahkan jendela { $direction ->
           *[down] bawah
            [left] kiri
            [right] kanan
            [up] atas
        }
    .display =
        Pindahkan jendela satu monitor { $direction ->
           *[down] bawah
            [left] kiri
            [right] kanan
            [up] atas
        }
    .workspace =
        Pindahkan jendela satu ruang kerja { $direction ->
           *[below] bawah
            [left] kiri
            [right] kanan
            [above] atas
        }
    .workspace-num = Pindahkan jendela ke ruang kerja { $num }
    .prev-workspace = Pindahkan jendela ke ruang kerja sebelumnya
    .next-workspace = Pindahkan jendela ke ruang kerja selanjutnya
    .last-workspace = Pindahkan jendela ke ruang kerja terakhir
    .next-display = Pindahkan jendela ke layar selanjutnya
    .prev-display = Pindahkan jendela ke layar sebelumnya
    .send-to-prev-workspace = Pindahkan jendela ke ruang kerja sebelumnya
    .send-to-next-workspace = Pindahkan jendela ke ruang kerja selanjutnya
