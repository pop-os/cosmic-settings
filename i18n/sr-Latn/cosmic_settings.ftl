app = COSMIC podešavanja

unknown = Nepoznato

number = { $number }

## Desktop

desktop = Radna površina

## Desktop: Appearance

appearance = Izgled
    .desc = Boje detalja i promena COSMIC palete.

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
reset-default = Vrati na podrazumevano
reset-to-default = Vrati na podrazumevano
rgb = RGB
window-hint-accent = Boja nagoveštaja aktivnog prozora
window-hint-accent-toggle = Koristi boju detalja iz teme kao nagoveštaj aktivnog prozora

auto-switch = Automatsko menjanje režima
    .desc = Svetli režim se uključuje pri izlasku, a tamni pri zalasku sunca

container-background = Pozadina kontejnera
    .desc-detail = Boja pozadine kontejnera se koristi za bočnu traku za navigaciju, bočni meni, dijaloške okvire i druge slične vidžete. Podrazumevano, automatski se izvodi iz pozadine aplikacija ili prozora.
    .reset = Vrati na automatsko
    .desc = Boja pozadine kontejnera se koristi za bočnu traku za navigaciju, bočni meni, dijaloške okvire i druge slične vidžete.
	
control-tint = Nijansa kontrolnih komponenti
    .desc = Koristi se za pozadinu standardnih dugmadi, unosa za pretragu, unosa teksta i sličnih komponenti.
	
frosted = Efekat mat stakla na interfejsu sistema
    .desc = Primenjuje zamućenje pozadine na panel, dok, aplete, pokretač i biblioteku aplikacija.
	
text-tint = Nijansa teksta interfejsa
    .desc = Boja koja se koristi za dobijanje boja teksta interfejsa koje imaju dovoljan kontrast na različitim površinama.
	
style = Stil
    .round = Okrugli
    .slightly-round = Blago okrugli
    .square = Četvrtast

# interface density left out for now
window-management = Upravljanje prozorima
    .active-hint = Veličina nagoveštaja aktivnog prozora
    .gaps = Praznine oko složenih prozora

## Desktop: Display

-requires-restart = Zahteva ponovno pokretanje

color = Boja
    .depth = Dubina boje
    .profile = Profil boje
    .sidebar = Profili boje
    .temperature = Temperatura boje

display = Ekran
    .desc = Upravljajte ekranima, prebacivanjem grafike i noćnim svetlom
    .arrangement = Raspored ekrana
    .arrangement-desc = Prevucite ekrane da biste ih preuredili.
    .enable = Omogući ekran
    .external = { $size } { $output } spoljašnji ekran
    .laptop = { $size } ekran laptopa
    .options = Opcije ekrana
    .refresh-rate = Osvežavanje
    .resolution = Rezolucija
    .scale = Razmera

graphics-mode = Grafički režim
    .mode = { $mode ->
        [compute] Računska
        *[hybrid] Hibridna
        [integrated] Integrisana
        [nvidia] NVIDIA
    } grafika
    .enable = Omogući { $mode ->
        [compute] računsku
        *[hybrid] hibridnu
        [integrated] integrisanu
        [nvidia] NVIDIA
    } grafiku
    .desc = { $mode ->
        [compute] Koristi namensku grafiku samo za računska opterećenja. Isljučuje spoljne ekrane. { -requires-restart }.
        *[hybrid] Aplikacije koriste integrisanu grafiku osim ako se izričito ne zahteva korišćenje namenske grafike. { -requires-restart }.
        [integrated] Isključuje namensku grafiku radi dužeg trajanja baterije i manje buke ventilatora.
        [nvidia] Bolje grafičko iskustvo i najveća potrošnja energije. { -requires-restart }.
    }
    .restart = Ponovo pokreni i prebaci na { $mode }?
    .restart-desc = Prebacivanje na { $mode } će zatvoriti sve otvorene aplikacije

mirroring = Preslikavanje
    .id = Preslikavanje { $id }
    .dont = Ne preslikavaj
    .mirror = Preslikaj { $display }
    .project = Projektuj na { $display ->
        [all] sve ekrane
        *[other] { $display }
    }
    .project-count = Projektovanje na još { $count} { $count ->
        [1] ekran
        *[other] ekrana
    }

night-light = Noćno svetlo
    .auto = Automatsko (od zalaska do izlaska sunca)
    .desc = Smanjite plavo svetlo toplijim bojama.

orientation = Orijentacija
    .landscape = Položeno
	.landscape-flipped = Položeno (preokrenuto)
    .portrait = Uspravno
	.portrait-flipped = Uspravno (preokrenuto)

scheduling = Planiranje
    .manual = Ručni raspored

## Desktop: Notifications

notifications = Obaveštenja
    .desc = Ne uznemiravaj, obaveštenja na zaključanom ekranu i podešavanja aplikacija.

## Desktop: Options

desktop-panel-options = Radna površina i panel
    .desc = Uloga Super tastera, lepljivi uglovi, kontrola prozora.
	
desktop-panels-and-applets = Paneli radne površine i apleti

dock = Dok
    .desc = Traka sa zakačenim aplikacijama.
	
hot-corner = Lepljivi uglovi
    .top-left-corner = Uključiti gornji levi lepljivi ugao za radne prostore

super-key-action = Uloga Super tastera
    .launcher = Pokretač aplikacija
    .workspaces = Radni prostori
    .applications = Aplikacije

top-panel = Gornji panel
    .workspaces = Dugme za prikazivanje radnih prostora
    .applications = Dugme za prikazivanje aplikacija

window-controls = Kontrola prozora
    .minimize = Prikaži dugme za minimizovanje
    .maximize = Prikaži dugme za maksimizovanje

## Desktop: Panel

panel = Gornji panel
    .desc = Gornja traka sa kontrolama radne površine i menijima.
	
add = Dodaj
add-applet = Dodaj aplet
all = Sve
applets = Apleti
center-segment = Centralni segment
drop-here = Spustite aplete ovde
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
    .desc = Konfiguraciona datoteka panela nedostaje zbog korišćenja prilagođene konfiguracije ili je oštećena.
    .fix = Vrati na podrazumevano

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
dialog-add = _Add
fit-to-screen = Uklopi u ekran
open-new-folder = Otvori novu fasciklu
recent-folders = Nedavne fascikle
stretch = Razvuci
zoom = Uvećaj

x-minutes = { $number } min.
x-hours = { $number ->
    [1] 60 minuta
    *[other] { $number } sat.
}

## Desktop: Workspaces

workspaces = Radni prostori
    .desc = Postavi broj radnih prostora, ponašanje, i poziciju.

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

## Networking: Wired

wired = Žičana
    .desc = Žičana konekcija, profili konekcije

## Networking: Online Accounts

online-accounts = Onlajn nalozi
    .desc = Dodaj naloge, IMAP i SMTP, prijavljivanje za preduzeća

## Time & Language

time = Vreme i jezik
    .desc = N/A

time-date = Datum i vreme
    .desc = Vremenska zona, automatsko podešavanje sata, format sata.
    .auto = Podesi automatski

time-zone = Vremenska zona
    .auto = Automatska vremenska zona
    .auto-info = Zahteva pristup lokaciji i internetu

time-format = Format datuma i vremena
    .twenty-four = 24-časa
    .first = Prvi dan nedelje

time-region = Region i jezik
    .desc = Format datuma, vremena i brojeva dobijen na osnovu Vašeg regiona

## Sound

sound = Zvuk
    .desc = N/A

sound-output = Izlaz
    .volume = Jačina izlaznog zvuka
    .device = Izlazni uređaj
    .level = Poravnanje izlaznog zvuka
    .config = Konfiguracija
    .balance = Balans

sound-input = Ulaz
    .volume = Jačina ulaznog zvuka
    .device = Ulazni uređaj
    .level = Poravnanje ulaznog zvuka

sound-alerts = Upozorenja
    .volume = Jačina zvuka upozorenja
    .sound = Zvuk upozorenja

sound-applications = Aplikacije
    .desc = Jačina zvuka aplikacija i podešavanja

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

about-related = Dodatna podešavanja
    .support = Podrška

## System: Firmware

firmware = Firmver
    .desc = Detalji firmvera.

## System: Users

users = Korisnici
    .desc = Autentifikacija i prijavljivanje, zaključan ekran.

## Input

input = Unos
    .desc = Unos

## Input: Keyboard

keyboard = Tastatura
    .desc = Unos sa tastature

keyboard-sources = Jezik unosa
    .desc = Jezik unosa se može menjati pomoću kombinacije tastera Super+Space. Ovo ponašanje se može promeniti u podešavanjima prečica na tastaturi.
    .move-up = Pomeri gore
    .move-down = Pomeri dole
    .settings = Podešavanja
    .view-layout = Pogledaj raspored tastature
    .remove = Ukloni

keyboard-special-char = Unošenje specijalnih znakova
    .alternate = Taster za alternativne znakove
    .compose = Compose taster

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Prečice na tastaturi
    .desc = Pregledajte i prilagodite prečice

## Input: Mouse
mouse = Miš
    .desc = Brzina miša, ubrzanje, prirodno pomeranje.
    .primary-button = Primarno dugme
    .primary-button-left = Levo
    .primary-button-right = Desno
    .speed = Brzina miša
    .acceleration = Omogući ubrzanje miša
    .acceleration-desc = Automatski podešava osetljivost praćenja na osnovu brzine.
    .double-click-speed = Brzina dvostrukog klika
    .double-click-speed-desc = Menja potrebnu brzinu za registrovanje dvostrukih klikova.

mouse-scrolling = Pomeranje
    .speed = Brzina pomeranja
    .natural = Prirodno pomeranje
    .natural-desc = Okretanje točka pomera sadržaj umesto prikaza.

## Input: Touchpad

touchpad = Dodirna tabla
    .desc = Brzina dodirne table, opcije klika, pokreti.
    .primary-button = Primarno dugme
    .primary-button-left = Levo
    .primary-button-right = Desno
    .speed = Brzina dodirne table
    .acceleration = Omogući ubrzanje dodirne table
    .acceleration-desc = Automatski podešava osetljivost praćenja na osnovu brzine.
    .double-click-speed = Brzina dvostrukog klika
    .double-click-speed-desc = Menja potrebnu brzinu za registrovanje dvostrukih klikova.
