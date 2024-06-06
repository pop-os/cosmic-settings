app = Setări COSMIC 

unknown = Necunoscut

number = { $number }

## Desktop

desktop = Desktop

## Desktop: Appearance

appearance = Aspect 
    .desc = Culori de accent și tematică COSMIC.

accent-color = Culoare de accent
app-background = Fundalul ferestrei sau al aplicației 
auto = Automat
close = Închideți
color-picker = Selector de culoare
copied-to-clipboard = Copiat în clipboard
copy-to-clipboard = Copiați în clipboard
dark = Întunecat
export = Exportați
hex = Hex
import = Importați
light = Luminos
mode-and-colors = Mod și Culori
recent-colors = Culori recente
reset-to-default = Resetare la valorile implicite
rgb = RGB
window-hint-accent = Culoarea marginii ferestrei active
window-hint-accent-toggle = Folosiți culoarea de accent a tematicii pentru marginea ferestrelor

auto-switch = Treceți automat de la modul luminos la modul întuncat
    .sunrise = Această opțiune va trece la modul luminos la răsărit 
    .sunset = Această opțiune va trece la modul luminos la apus
    .next-sunrise = Această opțiune va trece la modul luminos la răsăritul următor
    .next-sunset = Această opțiune va trece la modul luminos la apusul următor

container-background = Fundal container
    .desc-detail = Culoarea fundalului containerului este folosită pentru bara laterală de navigare, sertarul lateral, dialoguri și widget-uri asemănătoare. În mod implicit, este derivată automat din fundalul aplicației sau al ferestrei.
    .reset = Resetați la modul automat
    .desc = Culoarea containerului principal este folosită pentru bara laterală de navigare, sertarul lateral, dialoguri și widget-uri asemănătoare. 

control-tint = Nuanța componentei de control
    .desc = Aceasta este folosită pentru fundalele butoanelor standard, căsuțelor de căutare, căsuțelor de text și a componentelor similare.

frosted = Efect de sticlă mată pe interfața de sistem
    .desc = Această opțiune aplică o ceață de fundal pe panou, dock, applet-uri și librăria de aplicații. 

experimental-settings = Setări experimentale

enable-export = Aplicați această tematică pe aplicațiile GNOME.
    .desc = Nu toate toolkit-urile suportă schimbarea automată a tematicii. Aplicațiile non-COSMIC ar putea avea nevoie de o repornire după schimbarea tematicii.

icon-theme = Tematica pictogramelor
    .desc = Această opțiune aplică un set diferit de pictograme asupra aplicațiilor. 

text-tint = Nuanța textului interfaței
    .desc = Această culoare este folosită pentru a deriva culori care au suficient contrast pe diferite suprafețe.

style = Stil
    .round = Rotund
    .slightly-round = Puțin rotund
    .square = Pătrat

# interface density left out for now
window-management = Gestionarea ferestrelor
    .active-hint = Mărimea marginii ferestrei active 
    .gaps = Distanța dintre ferestrele spațiate

## Desktop: Display

-requires-restart = Necesită repornire

color = Culoare
    .depth = Adâncime de culoare
    .profile = Profil de culoare 
    .sidebar = Profile de culoare 
    .temperature = Temperatura culorii

display = Ecrane
    .desc = Gestionați ecranele, schimbările de grafică, și lumina de noapte
    .arrangement = Aranjarea ecranelor
    .arrangement-desc = Mutați ecranele pentru a le rearanja
    .enable = Activați ecranul
    .external = { $size } { $output } Ecran Extern
    .laptop = { $size } Ecran de laptop
    .options = Opțiunile ecranului
    .refresh-rate = Rata de reîmprospătare
    .resolution = Rezoluție
    .scale = Scală

mirroring = Oglindire
    .id = Oglindirea { $id }
    .dont = Nu oglindiți
    .mirror = Oglindiți { $display }
    .project = Proiectează către { $display ->
        [all] toate ecranele
        *[other] { $display }
    }
    .project-count = Se proiectează către { $count} { $count ->
        [1] ecran
        *[other] ecrane
    }

night-light = Lumină de noapte
    .auto = Automată (de la apus la răsărit)
    .desc = Aceasta reduce lumina albastră folosind culori mai calde.

orientation = Orientare
    .standard = Standard
    .rotate-90 = Rotire 90
    .rotate-180 = Rotire 180
    .rotate-270 = Rotire 270

scheduling = Programare
    .manual = Programare manuală

## Desktop: Notifications

notifications = Notificări
    .desc = Nu deranja, notificările ecranului de blocare și setări pentru fiecare aplicație.

## Desktop: Options

desktop-panel-options = Desktop și Panou
    .desc = Acțiunea butonului Super, marginile ecranului, opțiuni de control a ferestrelor.

desktop-panels-and-applets = Panouri Desktop și Applet-uri

dock = Dock
    .desc = Panou cu aplicații fixate.

hot-corner = Marginea ecranului
    .top-left-corner = Activați marginea din stânga sus pentru Spații de lucru

top-panel = Panoul de sus
    .workspaces = Afișați butonul pentru Spații de lucru
    .applications = Afișați butonul pentru aplicații


window-controls = Controalele ferestrelor
    .minimize = Afișați butonul de minimizare
    .maximize = Afișați butonul de maximizare

## Desktop: Panel

panel = Panou
    .desc = Bară superioară cu controalele desktop-ului și meniuri.

add = Adăugați
add-applet = Adăugați un applet
all = Toate
applets = Applet-uri
center-segment = Segmentul de centru
drop-here = Puneți applet-uri aici
end-segment = Sfârșitul segmentului
large = Mare
no-applets-found = Nu a fost găsit niciun applet...
panel-bottom = Jos
panel-left = Stânga
panel-right = Dreapta
panel-top = Sus
search-applets = Căutați applet-uri...
small = Mică
start-segment = Segmentul de început

panel-appearance = Aspect
    .match = Asemenea desktop-ului
    .light = Luminos
    .dark = Întunecat

panel-behavior-and-position = Comportament și Poziții
    .autohide = Ascundeți automat panoul
    .dock-autohide = Ascundeți automat dockul
    .position = Poziția pe ecran
    .display = Arătați pe ecran

panel-style = Stil
    .anchor-gap = Afișați o distanță dintre panou și marginile ecranului
    .dock-anchor-gap = Afișați o distanță dintre dock și marginile ecranului
    .extend = Întindeți panoul până la marginile ecranului
    .dock-extend = Întindeți dockul până la marginile ecranului
    .appearance = Aspect
    .size = Mărime
    .background-opacity = Opacitatea fundalului

panel-applets = Configurare
    .dock-desc = Configurați applet-urile dock-ului.
    .desc = Configurați applet-urile panoului.

panel-missing = Configurația panoului lipsește
    .desc = Fișierul cu configurația panoului lipsește datorită unei configurații diferite sau acesta este corupt.
    .fix = Resetați la valorile implicite

## Desktop: Wallpaper

wallpaper = Imagine de fundal
    .change = Schimbați în fiecare zi
    .desc = Opțiuni pentru imaginea de fundal, culori și liste de fundale.
    .fit = Încadrarea imaginii de fundal
    .folder-dialog = Alegeți dosarul pentru imagini de fundal
    .image-dialog = Alegeți imaginea de fundal
    .plural = Imagini de fundal
    .same = Aceeași imagine de fundal pe toate ecranele
    .slide = Listă de fundale

add-color = Adăugați o culoare
add-image = Adăugați o imagine
all-displays = Toate ecranele
colors = Culori
dialog-add = _Adaugă
fill = Umplere
fit-to-screen = Încadrare pe ecran
open-new-folder = Deschideți un dosar nou
recent-folders = Dosare recente

x-minutes = { $number } minute
x-hours = { $number ->
    [1] o oră
    *[other] { $number } ore
}

## Desktop: Workspaces

workspaces = Spații de lucru
    .desc = Configurați plasamentul, comportamentul și numărul spațiilor de lucru.

workspaces-behavior = Comportamentul spațiilor de lucru
    .dynamic = Spații de lucru dinamice
    .dynamic-desc = Această opțiune șterge automat spațiile de lucru goale.
    .fixed = Număr fix de spații de lucru
    .fixed-desc = Adaugă sau șterge spații de lucru în Prezentarea generală.

workspaces-multi-behavior = Comportamentul spațiilor de lucru cu mai multe monitoare
    .span = Spațiile de lucru sunt identice pe toate ecranele
    .separate = Ecranele au spații de lucru diferite

workspaces-overview-thumbnails = Previzualizarea spațiilor de lucru în Prezentarea generală
    .show-number = Afișați numărul spațiului de lucru
    .show-name = Afișați numele spațiului de lucru

workspaces-orientation = Orientarea spațiilor de lucru
    .vertical = Verticală
    .horizontal = Orizontală

## Networking: Wired

wired = Cu fir
    .desc = Conexiune cu fir, profile de conexiune

## Networking: Online Accounts

online-accounts = Conturi online
    .desc = Adăugați conturi, IMAP sau SMTP, conectări enterprise

## Time & Language

time = Timp & Limbă
    .desc = N/A

time-date = Dată & Timp
    .desc = Zona de timp, setări automate ale ceasului și formatarea timpului.
    .auto = Setare automată

time-zone = Zonă de timp
    .auto = Setare automată
    .auto-info = Necesită serviciile de locație și acces la internet.

time-format = Formatul timpului și al datei 
    .twenty-four = Tip de 24 de ore 
    .first = Prima zi a săptămânii
    .show-date = Afișați data pe panoul superior
    .friday = Vineri
    .saturday = Sâmbătă
    .sunday = Duminică
    .monday = Luni

time-region = Regiune & Limbă
    .desc = Formatarea datei, timpului și a numerelor bazată pe regiunea dvs.

## Sound

sound = Sunet
    .desc = N/A

sound-output = Sunetul de ieșire
    .volume = Volumul de ieșire
    .device = Dispozitivul de ieșire
    .level = Nivelul de ieșire
    .config = Configurare
    .balance = Balanță

sound-input = Sunetul de intrare
    .volume = Volumul de intrare
    .device = Dispozitivul de intrare
    .level = Nivelul de intrare

sound-alerts = Alerte
    .volume = Alerte de volum
    .sound = Alerte de sunet

sound-applications = Aplicații
    .desc = Volumele și setările aplicațiilor

## System

system = Sistem & Conturi

## System: About

about = Despre
    .desc = Numele dispozitivului, informații hardware, setările implicite ale sistemului de operare.

about-device = Numele dispozitivului
    .desc = Acest nume este vizibil pentru alte dispozitive de rețea sau Bluetooth.

about-hardware = Hardware
    .model = Model hardware
    .memory = Memorie
    .processor = Procesor
    .graphics = Grafică
    .disk-capacity = Capacitatea discului

about-os = Sistem de operare
    .os = Sistem de operare
    .os-architecture = Arhitectura sistemului de operare
    .desktop-environment = Tip de desktop
    .windowing-system = Sistem de afișare al ferestrelor

about-related = Setări relatate
    .support = Primiți suport

## System: Firmware

firmware = Firmware
    .desc = Detalii firmware.

## System: Users

users = Utilizatori
    .desc = Autentificare și conectare, ecran de blocare.

## Input

acceleration-desc = Această opțiune modifică automat sensitivitatea de urmărire bazată pe viteză.

disable-while-typing = Dezactivați în timpul scrisului

input-devices = Dispozitivele de intrare
    .desc = Dispozitivele de intrare

primary-button = Butonul principal
    .left = Stânga
    .right = Dreapta

scrolling = Derulare
    .two-finger = Derulați cu două degete
    .edge = Derulați la margine cu un deget 
    .speed = Viteza de derulare
    .natural = Derulare naturală
    .natural-desc = Derulați conținutul în schimbul spațiului de derulare

## Input: Keyboard

keyboard = Tastatură
    .desc = Opțiuni tastatură

keyboard-sources = Surse de intrare
    .desc = Sursele de intrare se pot schimba cu combinația Super+Space. Aceasta poate fi schimbată în setările de scurtături ale tastaturii.
    .move-up = Mișcați în sus 
    .move-down = Mișcați în jos 
    .settings = Setări
    .view-layout = Vizualizați sursa de intrare a tastaturii
    .remove = Ștergeți
    .add = Adăugați sursă de intrare

keyboard-special-char = Caractere speciale
    .alternate = Butonul pentru caractere alternative
    .compose = Butonul de compunere

added = Adăugat
type-to-search = Scrieți pentru a căuta...

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Scurtături ale tastaturii
    .desc = Vizualizați și modificați scurtăturile

## Input: Mouse

mouse = Mouse
    .desc = Viteza mouse-ului, accelerația, derularea naturală.
    .speed = Viteza mouse-ului
    .acceleration = Activați accelerația mouse-ului

## Input: Touchpad

click-behavior = Comportamentul de apăsare
    .click-finger = Click secundar cu două degete și middle-click cu trei degete
    .button-areas = Click secundar în colțul din dreapta jos și middle-click în zona de centru jos

pinch-to-zoom = Ciupiți pentru a face click
    .desc = Folosiți două degete pentru a mări conținutul aplicațiilor care suportă această funcționalitate.

tap-to-click = Apasă pentru click
    .desc = Această opțiune activează click-ul cu un deget pentru un click principal, cel cu două degete pentru click-ul secundar și cel cu trei degete pentru middle-click.

touchpad = Touchpad
    .acceleration = Activați accelerația touchpad-ului
    .desc = Viteza touchpad-ului, opțiunile de click, gesturi.
    .speed = Viteza touchpad-ului

## Input: Gestures

swiping = Glisare
    .four-finger-down = Glisare în jos cu patru degete
    .four-finger-left = Glisare în stânga cu patru degete
    .four-finger-right = Glisare în dreapta cu patru degete
    .four-finger-up = Glisare în sus cu patru degete
    .three-finger-any = Glisare în orice direcție cu trei degete

switch-between-windows = Schimbați ferestrele
switch-to-next-workspace = Schimbați la spațiul de lucru următor
switch-to-prev-workspace = Schimbați la spațiul de lucru anterior
open-application-library = Deschideți Librăria cu aplicații
open-workspaces-view = Deschideți Prezentarea generală a spațiilor de lucru

