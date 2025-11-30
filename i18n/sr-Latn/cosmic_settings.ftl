app = COSMIC podešavanja
dbus-connection-error = Neuspešno povezivanje sa DBus-om
ok = U redu
unknown = Nepoznato
number = { $number }

## Network & Wireless

connections-and-profiles =
    { $variant ->
        [wired] Žičane
        [wifi] Wi-Fi
        [vpn] VPN
       *[other] Nepoznate
    } konekcije i profili konekcija.
add-network = Dodaj mrežu
    .profile = Dodaj profil
add-vpn = Dodaj VPN
airplane-on = Avionski režim je uključen.
cable-unplugged = Kabl je iskopčan
connect = Poveži se
connected = Povezano
connecting = Povezuje se…
disconnect = Prekini konekciju
forget = Zaboravi
known-networks = Poznate mreže
network-and-wireless = Mreža i bežična konekcija
no-networks = Nisu pronađene mreže.
no-vpn = Nema dostupnih VPN konekcija.
password = Lozinka
password-confirm = Potvrdi lozinku
remove = Ukloni
settings = Podešavanja
username = Korisničko ime
visible-networks = Vidljive mreže
identity = Identitet
auth-dialog = Potrebna autentifikacija
    .vpn-description = Unesite korisničko ime i lozinku potrebne za VPN servis.
    .wifi-description = Unesite lozinku ili ključ za šifrovanje. Možete se povezati i pritiskom na "WPS" dugme na ruteru.
forget-dialog = Zaboravi ovu Wi-Fi mrežu?
    .description = Moraćete ponovo da unesete lozinku da biste koristili ovu Wi-Fi mrežu u budućnosti.
network-device-state =
    .activated = Povezano
    .config = Povezuje se
    .deactivating = Prekida se konekcija
    .disconnected = Nije povezano
    .failed = Neuspešno povezivanje
    .ip-check = Proverava se konekcija
    .ip-config = Traže se IP i informacije rutiranja
    .need-auth = Potrebna autentifikacija
    .prepare = Priprema se za povezivanje
    .secondaries = Čeka se sekundarna konekcija
    .unavailable = Nedostupno
    .unknown = Nepoznato stanje
    .unmanaged = Neupravljano
    .unplugged = Kabl je iskopčan
remove-connection-dialog = Ukloni profil konekcije?
    .vpn-description = Moraćete ponovo da unesete lozinku da biste koristili ovu mrežu u budućnosti.
    .wired-description = Moraćete ponovo da kreirate ovaj profil da biste ga koristili u budućnosti.
vpn = VPN
    .connections = VPN konekcije
    .error = Neuspešno dodavanje VPN konfiguracije
    .remove = Ukloni profil konekcije
    .select-file = Izaberite VPN konfiguracionu datoteku
vpn-error = VPN greška
    .config = Neuspešno dodavanje VPN konfiguracije
    .connect = Neuspešno povezivanje sa VPN-om
    .connection-editor = Uređivač konekcije neuspešan
    .connection-settings = Neuspešno dobijanje podešavanja za aktivne konekcije
    .updating-state = Neuspešno ažuriranje stanja mrežnog menadžera
    .wireguard-config-path = Neispravna putanja datoteke za WireGuard konfiguraciju
    .wireguard-config-path-desc = Izabrana datoteka mora biti na lokalnom sistemu datoteka.
    .wireguard-device = Neuspešno kreiranje WireGuard uređaja
    .with-password =
        Neuspešno podešavanje VPN { $field ->
           *[username] korisničko ime
            [password] lozinka
            [password-flags] flag-ova lozinke
        } sa nmcli
wired = Žičana
    .adapter = Žičani adapter { $id }
    .connections = Žičane konekcije
    .devices = Žičani uređaji
    .remove = Ukloni profil konekcije
    .desc = Žičana konekcija, profili konekcije
wifi = Wi-Fi
    .adapter = Wi-Fi adapter { $id }
    .forget = Zaboravi ovu mrežu
wireguard-dialog = Dodaj WireGuard uređaj
    .description = Izaberite ime uređaja za WireGuard konfiguraciju.

## Networking: Online Accounts

online-accounts = Onlajn nalozi
    .desc = Dodaj naloge, IMAP i SMTP, prijavljivanje za preduzeća

# Bluetooth

activate = Aktiviraj
confirm = Potvrdi
enable = Omogući
bluetooth = Bluetooth
    .desc = Upravljaj Bluetooth uređajima
    .status = Ovaj sistem je vidljiv kao { $aliases } dok su Bluetooth podešavanja otvorena.
    .connected = Povezano
    .connecting = Povezuje se
    .disconnecting = Prekida se konekcija
    .connect = Poveži se
    .disconnect = Prekini konekciju
    .forget = Zaboravi
    .dbus-error = Došlo je do greške pri interakciji sa DBus-om: { $why }
    .disabled = Bluetooth servis je onemogućen
    .inactive = Bluetooth servis nije aktivan
    .unknown = Bluetooth servis se ne može aktivirati. Da li je BlueZ instaliran?
bluetooth-paired = Prethodno povezani uređaji
    .connect = Poveži se
    .battery = { $percentage }% baterije
bluetooth-confirm-pin = Potvrdi Bluetooth PIN
    .description = Molimo potvrdite da se sledeći PIN slaže sa onim prikazanim na { $device }
bluetooth-available = Obližnji uređaji
bluetooth-adapters = Bluetooth adapteri

## Accessibility

accessibility = Pristupačnost
    .vision = Vid
    .on = Uključeno
    .off = Isključeno
    .unavailable = Nedostupno
    .screen-reader = Čitač ekrana
    .high-contrast = Visok kontrast
    .invert-colors = Obrni boje
    .color-filters = Filteri boja
hearing = Sluh
    .mono = Reprodukuj stereo zvuk kao mono
default = Podrazumevano
magnifier = Lupa
    .controls =
        Ili koristi ove prečice: { $zoom_in ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_in } za uvećanje,
        }{ $zoom_out ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_out } za umanjenje,
        }
        Super + skrol mišem
    .scroll_controls = Omogući uvećavanje mišem ili dodirnom tablom sa Super + Skrol
    .show_overlay = Prikaži interfejs lupe
    .increment = Korak uvećanja
    .signin = Pokreni lupu pri prijavljivanju
    .applet = Uključi/isključi lupu u apletu na panelu
    .movement = Uvećani prikaz se pomera
    .continuous = Kontinuirano sa pokazivačem
    .onedge = Kada pokazivač dostigne ivicu
    .centered = Da zadrži pokazivač centriran
color-filter = Tip filtera boja
    .unknown = Nepoznat filter je aktivan
    .greyscale = Nijanse sive
    .deuteranopia = Zeleno/Crveno (slabost zelene, Deuteranopija)
    .protanopia = Crveno/Zeleno (slabost crvene, Protanopija)
    .tritanopia = Plavo/Žuto (slabost plave, Tritanopija)

## Desktop

desktop = Radna površina

## Desktop: Wallpaper

wallpaper = Pozadina
    .change = Promeni sliku svakih
    .desc = Pozadine radne površine, boje, i slajd-šou.
    .fit = Skaliranje pozadine
    .folder-dialog = Izaberite fasciklu za pozadine
    .image-dialog = Izaberite sliku pozadine
    .plural = Pozadine
    .same = Ista pozadina na svim ekranima
    .slide = Slajd-šou
add-color = Dodaj boju
add-image = Dodaj sliku
all-displays = Svi ekrani
colors = Boje
dialog-add = Dodaj
fill = Popuni
fit-to-screen = Uklopi u ekran
open-new-folder = Otvori novu fasciklu
recent-folders = Nedavne fascikle
x-minutes =
    { $number } { $number ->
        [one] minut
       *[other] minuta
    }
x-hours =
    { $number } { $number ->
        [one] sat
       *[other] sati
    }
never = Nikad

## Desktop: Appearance

appearance = Izgled
    .desc = Boje detalja i promena palete.
accent-color = Boja detalja
app-background = Pozadina aplikacija ili prozora
auto = Automatski
close = Zatvori
color-picker = Birač boja
copied-to-clipboard = Kopirano u privremenu memoriju
copy-to-clipboard = Kopiraj u privremenu memoriju
dark = Tamno
export = Izvezi
hex = Hex
import = Uvezi
light = Svetlo
mode-and-colors = Režim i boje
recent-colors = Nedavne boje
reset-to-default = Vrati na podrazumevano
rgb = RGB
window-hint-accent = Boja nagoveštaja aktivnog prozora
window-hint-accent-toggle = Koristi boju detalja iz teme kao nagoveštaj aktivnog prozora
auto-switch = Automatski prelaz između svetlog i tamnog režima
    .sunrise = Prelazi na svetli režim pri izlasku sunca
    .sunset = Prelazi na tamni režim pri zalasku sunca
    .next-sunrise = Prelazi na svetli režim pri sledećem izlasku sunca
    .next-sunset = Prelazi na tamni režim pri sledećem zalasku sunca
container-background = Pozadina kontejnera
    .desc-detail = Boja pozadine kontejnera se koristi za bočnu traku za navigaciju, bočni meni, dijaloške okvire i druge slične vidžete. Podrazumevano, automatski se izvodi iz pozadine aplikacija ili prozora.
    .reset = Vrati na automatsko
    .desc = Boja pozadine kontejnera se koristi za bočnu traku za navigaciju, bočni meni, dijaloške okvire i druge slične vidžete.
control-tint = Nijansa kontrolnih komponenti
    .desc = Koristi se za pozadinu standardnih dugmadi, unosa za pretragu, unosa teksta i sličnih komponenti.
frosted = Efekat mat stakla na interfejsu sistema
    .desc = Primenjuje zamućenje pozadine na panel, dok, aplete, pokretač i biblioteku aplikacija.
enable-export = Primeni ovu temu na GNOME aplikacije.
    .desc = Automatsku promenu teme ne podržavaju sve aplikacije. Ne-COSMIC aplikacije će možda morati da se ponovo pokrenu nakon promene teme.
icon-theme = Tema ikonica
    .desc = Primenjuje drugačiji skup ikonica na aplikacije.
text-tint = Nijansa teksta interfejsa
    .desc = Boja koja se koristi za dobijanje boja teksta interfejsa koje imaju dovoljan kontrast na različitim površinama.
style = Stil
    .round = Okrugli
    .slightly-round = Blago okrugli
    .square = Četvrtast
interface-density = Gustina interfejsa
    .comfortable = Udobno
    .compact = Kompaktno
    .spacious = Prostrano
window-management-appearance = Upravljanje prozorima
    .active-hint = Debljina nagoveštaja aktivnog prozora
    .gaps = Praznine oko složenih prozora

### Experimental

experimental-settings = Eksperimentalna podešavanja
icons-and-toolkit = Tema ikonica i toolkit-a
interface-font = Font sistema
monospace-font = Monoprostorni font

## Desktop: Notifications

notifications = Obaveštenja
    .desc = Ne uznemiravaj, obaveštenja na zaključanom ekranu i podešavanja aplikacija.

## Desktop: Panel

panel = Panel
    .desc = Glavna sistemska traka za menije i aplete.
add = Dodaj
add-applet = Dodaj aplet
all = Sve
applets = Apleti
center-segment = Centralni segment
place-here = Stavite aplete ovde
end-segment = Krajnji segment
large = Veliko
no-applets-found = Nisu pronađeni apleti...
panel-bottom = Dno
panel-left = Levo
panel-right = Desno
panel-top = Vrh
search-applets = Pretraži aplete...
small = Malo
start-segment = Početni segment
panel-appearance = Izgled
    .match = Kao sistem
    .light = Svetli
    .dark = Tamni
panel-behavior-and-position = Ponašanje i pozicija
    .autohide = Automatsko sakrivanje panela
    .dock-autohide = Automatsko sakrivanje dok-a
    .position = Pozicija na ekranu
    .display = Prikaži na ekranu
panel-style = Stil
    .anchor-gap = Razmak između panela i ivica ekrana
    .dock-anchor-gap = Razmak između dok-a i ivica ekrana
    .extend = Proširi panel do ivica ekrana
    .dock-extend = Proširi dok do ivica ekrana
    .appearance = Izgled
    .size = Veličina
    .background-opacity = Prozirnost pozadine
panel-applets = Konfiguracija
    .dock-desc = Podesi aplete na dok-u.
    .desc = Podesi aplete na panelu.
panel-missing = Nedostaje konfiguracija panela
    .desc = Konfiguraciona datoteka panela nedostaje zbog korišćenja prilagođene konfiguracije ili je oštećena.
    .fix = Vrati na podrazumevano

## Desktop: Dock

dock = Dok
    .desc = Opciona traka za aplikacije i aplete.

## Desktop: Window management

window-management = Upravljanje prozorima
    .desc = Akcija Super tastera, opcije kontrole prozora, i dodatne opcije slaganja prozora.
super-key = Super taster
    .launcher = Otvori Pokretač
    .workspaces = Otvori Radne prostore
    .applications = Otvori Aplikacije
    .disable = Onemogući
edge-gravity = Plutajući prozori se privlače ka ivicama ekrana
window-controls = Kontrole prozora
    .maximize = Prikaži dugme za maksimizovanje
    .minimize = Prikaži dugme za minimizovanje
    .active-window-hint = Prikaži nagoveštaj aktivnog prozora
focus-navigation = Navigacija fokusa
    .focus-follows-cursor = Fokus prati pokazivač
    .focus-follows-cursor-delay = Kašnjenje fokusa za pokazivačem u ms
    .cursor-follows-focus = Pokazivač prati fokus

## Desktop: Workspaces

workspaces = Radni prostori
    .desc = Orijentacija i ponašanje radnog prostora.
workspaces-behavior = Ponašanje radnih prostora
    .dynamic = Dinamični radni prostori
    .dynamic-desc = Automatski uklanja prazne radne prostore.
    .fixed = Fiksni broj radnih prostora
    .fixed-desc = Dodajte ili uklonite radne prostore u pregledu.
workspaces-multi-behavior = Ponašanje sa više monitora
    .span = Radni prostori su zajednički za sve ekrane
    .separate = Ekrani imaju odvojene radne prostore
workspaces-overview-thumbnails = Prikaz radnih prostora u pregledu
    .show-number = Prikaži broj radnog prostora
    .show-name = Prikaži ime radnog prostora
workspaces-orientation = Orijentacija radnih prostora
    .vertical = Vertikalni
    .horizontal = Horizontalni
hot-corner = Lepljivi ugao
    .top-left-corner = Omogući gornji levi lepljivi ugao za prikaz radnih prostora

## Displays

-requires-restart = Zahteva ponovno pokretanje
color = Boja
    .depth = Dubina boje
    .profile = Profil boje
    .sidebar = Profili boje
    .temperature = Temperatura boje
display = Ekrani
    .desc = Upravljajte ekranima i noćnim svetlom
    .arrangement = Raspored ekrana
    .arrangement-desc = Prevucite ekrane da biste ih preuredili.
    .enable = Omogući ekran
    .external = { $size } { $output } spoljašnji ekran
    .laptop = { $size } ekran laptopa
    .options = Opcije ekrana
    .refresh-rate = Osvežavanje
    .resolution = Rezolucija
    .scale = Razmera
    .additional-scale-options = Dodatne opcije razmere
mirroring = Preslikavanje
    .id = Preslikavanje { $id }
    .dont = Ne preslikavaj
    .mirror = Preslikaj { $display }
    .project =
        Projektuj na { $display ->
            [all] sve ekrane
           *[other] { $display }
        }
    .project-count =
        Projektovanje na još { $count } { $count ->
            [1] ekran
           *[other] ekrana
        }
night-light = Noćno svetlo
    .auto = Automatsko (od zalaska do izlaska sunca)
    .desc = Smanjite plavo svetlo toplijim bojama.
orientation = Orijentacija
    .standard = Standardna
    .rotate-90 = Rotirano 90°
    .rotate-180 = Rotirano 180°
    .rotate-270 = Rotirano 270°
vrr = Varijabilna brzina osvežavanja
    .enabled = Omogućeno
    .force = Uvek
    .auto = Automatski
    .disabled = Onemogućeno
scheduling = Raspored
    .manual = Ručni raspored
dialog = Dijalog
    .title = Zadrži ova podešavanja ekrana?
    .keep-changes = Zadrži promene
    .change-prompt = Promene podešavanja će se automatski vratiti za { $time } sekundi.
    .revert-settings = Vrati podešavanja

## Sound

sound = Zvuk
    .desc = N/A
sound-output = Izlaz
    .volume = Jačina izlaznog zvuka
    .device = Izlazni uređaj
    .level = Nivo izlaza
    .config = Konfiguracija
    .balance = Balans
    .left = Levo
    .right = Desno
sound-input = Ulaz
    .volume = Jačina ulaznog zvuka
    .device = Ulazni uređaj
    .level = Nivo ulaza
amplification = Pojačavanje
    .desc = Omogućava povećanje jačine zvuka do 150%.
sound-alerts = Upozorenja
    .volume = Jačina zvuka upozorenja
    .sound = Zvuk upozorenja
sound-applications = Aplikacije
    .desc = Jačina zvuka aplikacija i podešavanja

## Power

power = Napajanje i baterija
    .desc = Upravljajte postavkama napajanja.
battery = Baterija
    .minute =
        { $value } { $value ->
            [one] minut
           *[other] minuta
        }
    .hour =
        { $value } { $value ->
            [one] sat
           *[other] sati
        }
    .day =
        { $value } { $value ->
            [one] dan
           *[other] dana
        }
    .less-than-minute = Manje od minuta
    .and = i
    .remaining-time =
        { $time } do { $action ->
            [full] pune
           *[other] prazne
        }
connected-devices = Povezani uređaji
    .unknown = Nepoznat uređaj
power-mode = Režim napajanja
    .battery = Produženo trajanje baterije
    .battery-desc = Smanjena potrošnja energije i tihe performanse.
    .balanced = Balansirano
    .balanced-desc = Tihe performanse i umerena potrošnja energije.
    .performance = Visoke performanse
    .performance-desc = Najveće performanse i potrošnja energije.
    .no-backend = Podsistem nije pronađen. Instalirajte system76-power ili power-profiles-daemon.
power-saving = Opcije štednje energije
    .turn-off-screen-after = Ugasi ekran posle
    .auto-suspend = Automatsko spavanje
    .auto-suspend-ac = Automatsko spavanje priključeno za struju
    .auto-suspend-battery = Automatsko spavanje na bateriji

## Input

acceleration-desc = Automatski podešava osetljivost praćenja na osnovu brzine.
disable-while-typing = Onemogući tokom kucanja
input-devices = Ulazni uređaji
    .desc = Ulazni uređaji
primary-button = Primarno dugme
    .desc = Određuje redosled fizičkih dugmadi.
    .left = Levo
    .right = Desno
scrolling = Pomeranje
    .two-finger = Pomeranje sa dva prsta
    .edge = Pomeranje uz ivicu sa jednim prstom
    .speed = Brzina pomeranja
    .natural = Prirodno pomeranje
    .natural-desc = Okretanje točka pomera sadržaj umesto prikaza

## Input: Keyboard

slow = Sporo
fast = Brzo
short = Kratko
long = Dugo
keyboard = Tastatura
    .desc = Izvor unosa, prebacivanje, unos specijalnih karaktera, prečice.
keyboard-sources = Jezik unosa
    .desc = Jezik unosa se može menjati pomoću kombinacije tastera Super+Space. Ovo se može promeniti u podešavanjima prečica na tastaturi.
    .move-up = Pomeri gore
    .move-down = Pomeri dole
    .settings = Podešavanja
    .view-layout = Pogledaj raspored tastature
    .remove = Ukloni
    .add = Dodaj jezik unosa
keyboard-special-char = Unošenje specijalnih znakova
    .alternate = Taster za alternativne znakove
    .compose = Compose taster
    .compose-desc = Compose taster omogućava unos širokog spektra znakova. Da biste ga koristili, pritisnite Compose, a zatim sekvencu znakova. Na primer, Compose taster praćen tasterima C i o unosi ©, dok praćen tasterima a i ‘ unosi á.
    .caps = Caps Lock taster
keyboard-typing-assist = Kucanje
    .repeat-rate = Stopa ponavljanja
    .repeat-delay = Kašnjenje ponavljanja
keyboard-numlock-boot = Numlock
    .boot-state = Stanje pri pokretanju sistema
    .last-boot = Prethodno pokretanje
    .on = Uključeno
    .off = Isključeno
    .set = Podesi stanje numlock-a pri pokretanju
added = Dodato
type-to-search = Kucajte za pretragu...
show-extended-input-sources = Prikaži proširene izvore unosa

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Prečice na tastaturi
    .desc = Pregledajte i prilagodite prečice
add-another-keybinding = Dodaj drugu prečicu
cancel = Poništi
command = Komanda
custom = Prilagođene
debug = Debaguj
disabled = Onemogućeno
input-source-switch = Prebaci izvor unosa jezika tastature
migrate-workspace-prev = Premesti radni prostor na prethodni ekran
migrate-workspace-next = Premesti radni prostor na sledeći ekran
migrate-workspace =
    Premesti radni prostor na ekran { $direction ->
       *[down] dole
        [left] levo
        [right] desno
        [up] gore
    }
navigate = Navigacija
replace = Zameni
shortcut-name = Ime prečice
system-controls = Kontrole sistema
terminate = Prekini
toggle-stacking = Uključi grupisanje prozora
type-key-combination = Ukucajte kombinaciju tastera
custom-shortcuts = Prilagođene prečice
    .add = Dodaj prečicu
    .context = Dodaj prilagođenu prečicu
    .none = Nema prilagođenih prečica
modified = { $count } izmenjeno
nav-shortcuts = Navigacija
    .prev-output = Fokusiraj prethodni ekran
    .next-output = Fokusiraj sledeći ekran
    .last-workspace = Fokusiraj prošli radni prostor
    .prev-workspace = Fokusiraj prethodni radni prostor
    .next-workspace = Fokusiraj sledeći radni prostor
    .focus =
        Fokusiraj prozor { $direction ->
           *[down] dole
            [in] unutra
            [left] levo
            [out] spolja
            [right] desno
            [up] gore
        }
    .output =
        Prebaci se na ekran { $direction ->
           *[down] dole
            [left] levo
            [right] desno
            [up] gore
        }
    .workspace = Prebaci se na radni prostor { $num }
manage-windows = Upravljanje prozorima
    .close = Zatvori prozor
    .maximize = Maksimizuj prozor
    .fullscreen = Ceo ekran
    .minimize = Minimizuj prozor
    .resize-inwards = Smanji prozor
    .resize-outwards = Povećaj prozor
    .toggle-sticky = Uključi lepljivi prozor
move-windows = Pomeranje prozora
    .direction =
        Pomeri prozor { $direction ->
           *[down] dole
            [left] levo
            [right] desno
            [up] gore
        }
    .display =
        Pomeri prozor za jedan monitor { $direction ->
           *[down] dole
            [left] levo
            [right] desno
            [up] gore
        }
    .workspace =
        Pomeri prozor za jedan radni prostor { $direction ->
           *[below] ispod
            [left] levo
            [right] desno
            [above] iznad
        }
    .workspace-num = Pomeri prozor na radni prostor { $num }
    .prev-workspace = Pomeri prozor na prethodni radni prostor
    .next-workspace = Pomeri prozor na sledeći radni prostor
    .last-workspace = Pomeri prozor na prošli radni prostor
    .next-display = Pomeri prozor na sledeći ekran
    .prev-display = Pomeri prozor na prethodni ekran
    .send-to-prev-workspace = Pošalji prozor na prethodni radni prostor
    .send-to-next-workspace = Pošalji prozor na sledeći radni prostor
system-shortcut = Sistem
    .app-library = Otvori biblioteku aplikacija
    .brightness-down = Smanji osvetljenost ekrana
    .brightness-up = Povećaj osvetljenost ekrana
    .home-folder = Otvori početnu fasciklu
    .keyboard-brightness-down = Smanji osvetljenost tastature
    .keyboard-brightness-up = Povećaj osvetljenost tastature
    .launcher = Otvori pokretač
    .log-out = Odjavi se
    .lock-screen = Zaključaj ekran
    .mute = Isključi audio izlaz
    .mute-mic = Isključi ulaz mikrofona
    .play-pause = Pusti/Pauziraj
    .play-next = Sledeća numera
    .play-prev = Prethodna numera
    .poweroff = Isključi sistem
    .screenshot = Napravi snimak ekrana
    .terminal = Otvori terminal
    .volume-lower = Smanji jačinu audio izlaza
    .volume-raise = Povećaj jačinu audio izlaza
    .web-browser = Otvori veb pretraživač
    .window-switcher = Prebacivanje između otvorenih prozora
    .window-switcher-previous = Prebacivanje između otvorenih prozora unatrag
    .workspace-overview = Otvori pregled radnih prostora
window-tiling = Slaganje prozora
    .horizontal = Podesi horizontalnu orijentaciju
    .vertical = Podesi vertikalnu orijentaciju
    .swap-window = Zameni prozor
    .toggle-tiling = Uključi slaganje prozora
    .toggle-stacking = Uključi grupisanje prozora
    .toggle-floating = Uključi plutajući prozor
    .toggle-orientation = Promeni orijentaciju
replace-shortcut-dialog = Zameni prečicu?
    .desc = { $shortcut } se koristi od strane { $name }. Ako je zamenite, { $name } će biti onemogućeno.
zoom-in = Uvećaj
zoom-out = Umanji

## Input: Mouse

mouse = Miš
    .desc = Brzina miša, ubrzanje, prirodno pomeranje.
    .speed = Brzina miša
    .acceleration = Omogući ubrzanje miša

## Input: Touchpad

click-behavior = Ponašanje klika
    .click-finger = Sekundarni klik sa dva prsta i srednji klik sa tri prsta
    .button-areas = Sekundarni klik u donjem desnom uglu i srednji klik u donjem centru
pinch-to-zoom = Stisnite prste za zumiranje
    .desc = Koristite dva prsta za zumiranje sadržaja, za aplikacije koje podržavaju zumiranje.
tap-to-click = Dodir za klik
    .desc = Omogućava dodir jednim prstom za primarni klik, dva prsta za sekundarni klik i tri prsta za srednji klik.
touchpad = Dodirna tabla
    .acceleration = Omogući ubrzanje dodirne table
    .desc = Brzina dodirne table, opcije klika, pokreti.
    .speed = Brzina dodirne table

## Input: Gestures

gestures = Pokreti
    .four-finger-down = Prevuci prema dole sa četiri prsta
    .four-finger-left = Prevuci prema levo sa četiri prsta
    .four-finger-right = Prevuci prema desno sa četiri prsta
    .four-finger-up = Prevuci prema gore sa četiri prsta
    .three-finger-any = Prevuci sa tri prsta u bilo kom smeru
switch-workspaces = Promeni radni prostor
    .horizontal = Prevuci prema levo/desno sa četiri prsta
    .vertical = Prevuci prema gore/dole sa četiri prsta
switch-between-windows = Prebacivanje između prozora
open-application-library = Otvori biblioteku aplikacija
open-workspaces-view = Otvori pregled radnih prostora

## Time & Language

time = Vreme i jezik
    .desc = N/A
time-date = Datum i vreme
    .desc = Vremenska zona, automatska podešavanja sata i formatiranje vremena.
    .auto = Podesi automatski
    .auto-ntp = Datum i vreme će se automatski ažurirati kada se podesi vremenska zona.
time-zone = Vremenska zona
    .auto = Automatska vremenska zona
    .auto-info = Zahteva usluge lokacije i pristup internetu
time-format = Format datuma i vremena
    .twenty-four = 24-časovno vreme
    .show-seconds = Prikaži sekunde
    .first = Prvi dan nedelje
    .show-date = Prikaži datum u apletu za vreme
    .friday = Petak
    .saturday = Subota
    .sunday = Nedelja
    .monday = Ponedeljak
time-region = Region i jezik
    .desc = Format datuma, vremena i brojeva na osnovu regiona.
formatting = Formatiranje
    .dates = Datumi
    .time = Vreme
    .date-and-time = Datum i vreme
    .numbers = Brojevi
    .measurement = Merenja
    .paper = Papir
preferred-languages = Preferiran jezik
    .desc = Redosled jezika određuje koji se jezik koristi za korisnički interfejs. Promene se primenjuju pri sledećem prijavljivanju.
add-language = Dodaj jezik
    .context = Dodaj jezik
install-additional-languages = Instaliraj dodatne jezike
region = Region

## Applications

applications = Aplikacije

## Applications: Default Applications

default-apps = Podrazumevane aplikacije
    .desc = Podrazumevani veb pretraživač, imejl klijent, upravljač datoteka i druge aplikacije.
    .web-browser = Veb pretraživač
    .file-manager = Upravljač datoteka
    .mail-client = Imejl klijent
    .music = Muzika
    .video = Video
    .photos = Fotografije
    .calendar = Kalendar
    .terminal = Terminal
    .other-associations = Ostale asocijacije
    .text-editor = Uređivač teksta

## Applications: Startup Applications

startup-apps = Aplikacije pri pokretanju
    .desc = Konfigurišite aplikacije koje se pokreću pri prijavljivanju.
    .add = Dodaj aplikaciju
    .user = Aplikacije koje se pokreću kada se prijavite
    .none = Nisu dodate aplikacije za pokretanje
    .remove-dialog-title = Ukloni { $name }?
    .remove-dialog-description = Da li ste sigurni da želite da uklonite ovu aplikaciju za pokretanje?
    .add-startup-app = Dodaj aplikaciju za pokretanje

## Applications: Legacy Applications

legacy-applications = Kompatibilnost X11 aplikacija
    .desc = Skaliranje aplikacija X11 sistema prozora i globalne prečice.
legacy-app-global-shortcuts = Globalne prečice u X11 aplikacijama
    .desc = Globalne prečice omogućavaju da pritiske tastera i dugmadi miša izvedenih u aplikacijama budu prepoznati od strane drugih aplikacija za funkcije kao što su push-to-talk ili push-to-mute. Podrazumevano, ovo je onemogućeno u X11 aplikacijama da bi se osiguralo da druge aplikacije ne mogu da prate događaje tastature i miša koji sadrže osetljive informacije.
    .none = Nijedan taster
    .modifiers = Modifikatori (Super, Shift, Control, Alt)
    .combination = Svi tasteri dok se drže modifikatori Super, Control ili Alt
    .all = Svi tasteri
    .mouse = Događaji dugmadi miša u X11 aplikacijama
legacy-app-scaling = Skaliranje aplikacija X11 sistema prozora
    .scaled-gaming = Optimizuj za igre i aplikacije preko celog ekrana
    .gaming-description = X11 aplikacije mogu izgledati nešto veće/manje u poređenju sa Wayland aplikacijama.
    .scaled-applications = Optimizuj za aplikacije
    .applications-description = Igre i X11 aplikacije preko celog ekrana možda neće odgovarati vašoj rezoluciji ekrana.
    .scaled-compatibility = Režim maksimalne kompatibilnosti
    .compatibility-description = X11 aplikacije mogu izgledati zamućeno na HiDPI ekranima.
    .preferred-display = Preferirani ekran za igre i X11 aplikacije preko celog ekrana
    .no-display = Nijedan

## System

system = Sistem i nalozi

## System: About

about = O sistemu
    .desc = Ime uređaja, informacije o hardveru, podrazumevana podešavanja operativnog sistema.
about-device = Ime uređaja
    .desc = Ovo ime je vidljivo drugim mrežnim ili Bluetooth uređajima.
about-hardware = Hardver
    .model = Model hardvera
    .memory = Memorija
    .processor = Procesor
    .graphics = Grafika
    .disk-capacity = Kapacitet diska
about-os = Operativni sistem
    .os = Operativni sistem
    .os-architecture = Arhitektura operativnog sistema
    .desktop-environment = Okruženje radne površine
    .windowing-system = Sistem prozora
about-related = Povezana podešavanja
    .support = Podrška

## System: Firmware

firmware = Firmver
    .desc = Detalji firmvera.

## System: Users

users = Korisnici
    .desc = Autentifikacija i korisnički nalozi.
    .admin = Admin
    .standard = Standardan
    .profile-add = Izaberi sliku profila
administrator = Administrator
    .desc = Administratori mogu da menjaju podešavanja za sve korisnike, dodaju i uklanjaju druge korisnike.
add-user = Dodaj korisnika
change-password = Promeni lozinku
remove-user = Ukloni korisnika
full-name = Puno ime
invalid-username = Neispravno korisničko ime.
password-mismatch = Lozinka i potvrda moraju se poklapati.
save = Sačuvaj
