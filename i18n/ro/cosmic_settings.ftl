app = Setări COSMIC
dbus-connection-error = Nu s-a putut conecta la DBus
ok = OK
unknown = Necunoscut
number = { $number }

## Network & Wireless

connections-and-profiles =
    { $variant ->
        [wired] Cablu
        [wifi] Wi-Fi
        [vpn] VPN
       *[other] Necunoscut
    } conexiuni și profile de conexiune.
add-network = Adaugă o rețea
    .profile = Adaugă un profil
add-vpn = Adaugă un VPN
airplane-on = Modul avion este activ.
cable-unplugged = Cablu deconectat
connect = Conectează
connected = Conectat
connecting = Se conectează…
disconnect = Deconectează
forget = Uită
known-networks = Rețele cunoscute
network-and-wireless = Rețea & Wireless
no-networks = Nu au fost găsite rețele.
no-vpn = Nu sunt disponibile conexiuni VPN.
password = Parolă
remove = Șterge
settings = Setări
username = Nume utilizator
visible-networks = Rețele vizibile
identity = Identitate
auth-dialog = Autentificare necesară
    .vpn-description = Introdu numele de utilizator și parola necesare pentru serviciul VPN.
    .wifi-description = Introdu parola sau cheia de criptare. De asemenea, te poți conecta apăsând butonul „WPS” de pe router.
forget-dialog = Uită această rețea Wi-Fi?
    .description = Va trebui să introduci parola din nou pentru a folosi această rețea Wi-Fi în viitor.
network-device-state =
    .activated = Conectat
    .config = Se conectează
    .deactivating = Se deconectează
    .disconnected = Deconectat
    .failed = Conexiune eșuată
    .ip-check = Verificare conexiune
    .ip-config = Solicitare IP și informații de rutare
    .need-auth = Necesită autentificare
    .prepare = Se pregătește conectarea
    .secondaries = Se așteaptă pentru o conexiune secundară
    .unavailable = Indisponibil
    .unknown = Stare necunoscută
    .unmanaged = Neadministrat
    .unplugged = Cablu deconectat
remove-connection-dialog = Șterge profilul de conexiune?
    .vpn-description = Va trebui să introduci parola din nou pentru a folosi această rețea în viitor.
    .wired-description = Va trebui să recreezi acest profil pentru a-l folosi în viitor.
vpn = VPN
    .connections = Conexiuni VPN
    .error = Eșec la adăugarea configurației VPN
    .remove = Șterge profilul de conexiune
    .select-file = Selectează un fișier de configurație VPN
vpn-error = Eroare VPN
    .config = Eșec la adăugarea configurației VPN
    .connect = Eșec la conectarea la VPN
    .connection-editor = Eșec la editorul de conexiuni
    .connection-settings = Eșec la obținerea setărilor pentru conexiunile active
    .updating-state = Eșec la actualizarea stării managerului de rețea
    .wireguard-config-path = Cale fișier invalidă pentru configurația WireGuard
    .wireguard-config-path-desc = Fișierul ales trebuie să fie pe un sistem de fișiere local.
    .wireguard-device = Eșec la crearea dispozitivului WireGuard
    .with-password =
        Eșec la setarea VPN { $field ->
           *[username] nume utilizator
            [password] parolă
            [password-flags] opțiuni parolă
        } cu nmcli
wired = Cablu
    .adapter = Adaptor cablu { $id }
    .connections = Conexiuni cablu
    .devices = Dispozitive cablu
    .remove = Șterge profilul de conexiune
wifi = Wi-Fi
    .adapter = Adaptor Wi-Fi { $id }
    .forget = Uită această rețea
wireguard-dialog = Adăugă dispozitiv WireGuard
    .description = Alege un nume de dispozitiv pentru configurația WireGuard.

## Networking: Online Accounts

online-accounts = Conturi Online
    .desc = Adaugă conturi, IMAP și SMTP, autentificări enterprise

# Bluetooth

confirm = Confirmă
bluetooth = Bluetooth
    .desc = Gestionează dispozitivele Bluetooth
    .status = Acest sistem este vizibil ca { $aliases } în timp ce setările Bluetooth sunt deschise.
    .connected = Conectat
    .connecting = Se conectează
    .disconnecting = Se deconectează
    .connect = Conectează
    .disconnect = Deconectează
    .forget = Uită
    .dbus-error = A apărut o eroare în interacțiunea cu DBus: { $why }
bluetooth-paired = Dispozitive conectate anterior
    .connect = Conectează
    .battery = { $percentage }% baterie
bluetooth-confirm-pin = Confirmă PIN-ul Bluetooth
    .description = Te rog să confirmi că acest PIN corespunde celui afișat pe { $device }
bluetooth-available = Dispozitive disponibile
bluetooth-adapters = Adaptoare Bluetooth

## Accessibility

accessibility = Accesibilitate
    .vision = Viziune
    .on = Activat
    .off = Dezactivat
    .unavailable = Indisponibil
    .high-contrast = Mod contrast înalt
    .invert-colors = Inversare culori
    .color-filters = Filtre de culori
hearing = Audiție
    .mono = Redă audio stereo ca mono
default = Implicit
magnifier = Lupă
    .controls =
        Sau utilizează aceste comenzi rapide: { $zoom_in ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_in } pentru a mări,
        }{ $zoom_out ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_out } pentru a micșora,
        }
        Super + rotește cu mouse-ul
    .scroll_controls = Activează comenzi rapide „Super + Scroll” (prin mouse sau touchpad)
    .show_overlay = Arată suprapunerea lupei
    .increment = Increment de zoom
    .signin = Pornește lupa la autentificare
    .applet = Activează/dezactivează lupa din aplicația de pe panou
    .movement = Viziune zoomată se mișcă
    .continuous = Continuu cu pointerul
    .onedge = Când pointerul ajunge la margine
    .centered = Pentru a păstra pointerul centrat
color-filter = Tip filtru de culoare
    .unknown = Filtru necunoscut activ
    .greyscale = Gri
    .deuteranopia = Verde/Roșu (slăbiciune verde, Deuteranopie)
    .protanopia = Roșu/Verde (slăbiciune roșie, Protanopie)
    .tritanopia = Albastru/Galben (slăbiciune albastră, Tritanopie)

## Desktop

desktop = Birou

## Desktop: Wallpaper

wallpaper = Tapet
    .change = Schimbă imaginea la fiecare
    .desc = Opțiuni de imagini pentru tapet, culori și slideshow.
    .fit = Ajustează tapetul
    .folder-dialog = Alege folderul pentru tapet
    .image-dialog = Alege imaginea pentru tapet
    .plural = Tapete
    .same = Același tapet pe toate ecranele
    .slide = Slideshow
add-color = Adaugă o culoare
add-image = Adaugă o imagine
all-displays = Toate ecranele
colors = Culori
dialog-add = Adaugă
fill = Umple
fit-to-screen = Se potrivește pe ecran
open-new-folder = Deschide un folder nou
recent-folders = Foldere recente
x-minutes = { $number } minute
x-hours =
    { $number ->
        [1] 1 oră
       *[other] { $number } ore
    }
never = Niciodată

## Desktop: Appearance

appearance = Aspect
    .desc = Culori de accent și teme.
accent-color = Culoare accent
app-background = Fundal aplicație sau fereastră
auto = Automat
close = Închide
color-picker = Selector de culoare
copied-to-clipboard = Copiat în clipboard
copy-to-clipboard = Copiază în clipboard
dark = Întunecat
export = Exportă
hex = Hex
import = Importă
light = Lumină
mode-and-colors = Mod și culori
recent-colors = Culori recente
reset-to-default = Resetează la valoarea implicită
rgb = RGB
window-hint-accent = Culoare de accent fereastră activă
window-hint-accent-toggle = Folosește culoarea de accent a temei ca indiciu pentru fereastra activă
auto-switch = Schimbă automat între modurile Lumină și Întunecat
    .sunrise = Schimbă la modul Lumină la răsărit
    .sunset = Schimbă la modul Întunecat la apus
    .next-sunrise = Schimbă la modul Lumină la următorul răsărit
    .next-sunset = Schimbă la modul Întunecat la următorul apus
container-background = Fundal container
    .desc-detail = Culoarea fundalului containerului este folosită pentru bara de navigare, sertarul lateral, dialoguri și widget-uri similare. Implicit, aceasta este derivată automat din fundalul aplicației sau ferestrei.
    .reset = Resetează la automat
    .desc = Culoarea principală a containerului este folosită pentru bara de navigare, sertarul lateral, dialoguri și widget-uri similare.
control-tint = Nuanță pentru componentele de control
    .desc = Utilizată pentru fundalurile butoanelor standard, câmpurilor de căutare, câmpurilor de text și altor componente similare.
frosted = Efect de sticlă mată pentru interfața sistemului
    .desc = Aplică estompare de fundal panoului, dock-ului, applet-urilor, lansatorului și bibliotecii de aplicații.
enable-export = Aplică această temă aplicațiilor GNOME
    .desc = Nu toate toolkit-urile suportă comutarea automată. Aplicațiile non-COSMIC pot necesita o repornire după schimbarea temei.
icon-theme = Temă de pictograme
    .desc = Aplică un set diferit de pictograme aplicațiilor.
text-tint = Nuanță a textului interfeței
    .desc = Culoare folosită pentru a deriva culori de text cu contrast suficient pe diverse suprafețe.
style = Stil
    .round = Rotunjit
    .slightly-round = Ușor rotunjit
    .square = Pătrat
interface-density = Densitatea interfeței
    .comfortable = Confortabil
    .compact = Compact
    .spacious = Spațios
window-management-appearance = Gestionarea aspectului ferestrelor
    .active-hint = Dimensiunea indicatorului ferestrei active
    .gaps = Spațiu între ferestrele aranjate

### Experimental

experimental-settings = Setări experimentale
icons-and-toolkit = Tematizarea pictogramelor și a interfeței
interface-font = Fontul sistemului
monospace-font = Fontul monospace

## Desktop: Notifications

notifications = Notificări
    .desc = Nu deranja, notificări pe ecranul de blocare și setări pe aplicație.

## Desktop: Panel

panel = Panou
    .desc = Bară de sus cu controale pentru desktop și meniuri.
add = Adaugă
add-applet = Adaugă applet
all = Toate
applets = Applet-uri
center-segment = Segment central
end-segment = Segment final
large = Mare
no-applets-found = Niciun applet găsit...
panel-bottom = Jos
panel-left = Stânga
panel-right = Dreapta
panel-top = Sus
search-applets = Caută applet-uri...
small = Mic
start-segment = Segment de început
panel-appearance = Aspect
    .match = Potrivește cu desktopul
    .light = Deschis
    .dark = Întunecat
panel-behavior-and-position = Comportament și poziție
    .autohide = Ascunde automat panoul
    .dock-autohide = Ascunde automat dock-ul
    .position = Poziția pe ecran
    .display = Afișează pe ecran
panel-style = Stil
    .anchor-gap = Distanța între panou și marginea ecranului
    .dock-anchor-gap = Distanța între dock și marginea ecranului
    .extend = Extinde panoul până la marginile ecranului
    .dock-extend = Extinde dock-ul până la marginile ecranului
    .appearance = Aspect
    .size = Dimensiune
    .background-opacity = Opacitate fundal
panel-applets = Configurare
    .dock-desc = Configurează applet-urile din dock
    .desc = Configurează applet-urile din panou
panel-missing = Configurația panoului lipsește
    .desc = Fișierul de configurare al panoului lipsește din cauza utilizării unei configurații personalizate sau este corupt.
    .fix = Resetează la valorile implicite

## Desktop: Dock

dock = Dock
    .desc = Panou cu aplicații fixate în bara de aplicații și alte applet-uri.

## Desktop: Window management

window-management = Gestionarea ferestrelor
    .desc = Acțiune pentru tasta Super, opțiuni de control ale ferestrelor și opțiuni suplimentare de aranjare.
super-key = Acțiune pentru tasta Super
    .launcher = Deschide lansatorul
    .workspaces = Deschide spațiile de lucru
    .applications = Deschide aplicațiile
    .disable = Dezactivează
edge-gravity = Ferestrele plutesc spre marginile apropiate
window-controls = Controale pentru fereastră
    .maximize = Afișează butonul de maximizare
    .minimize = Afișează butonul de minimizare
    .active-window-hint = Afișează indicatorul ferestrei active
focus-navigation = Navigare focalizare
    .focus-follows-cursor = Focalizarea urmează cursorul
    .focus-follows-cursor-delay = Întârziere focalizare după cursor (ms)
    .cursor-follows-focus = Cursorul urmează focalizarea

## Desktop: Workspaces

workspaces = Spații de lucru
    .desc = Orientarea și comportamentul spațiilor de lucru.
workspaces-behavior = Comportamentul spațiilor de lucru
    .dynamic = Spații de lucru dinamice
    .dynamic-desc = Elimină automat spațiile de lucru goale.
    .fixed = Număr fix de spații de lucru
    .fixed-desc = Adaugă sau elimină spații de lucru din prezentarea generală.
workspaces-multi-behavior = Comportamentul pe mai multe monitoare
    .span = Spațiile de lucru se extind pe mai multe ecrane
    .separate = Fiecare ecran are spații de lucru separate
workspaces-overview-thumbnails = Miniaturi prezentare generală spații de lucru
    .show-number = Afișează numărul spațiului de lucru
    .show-name = Afișează numele spațiului de lucru
workspaces-orientation = Orientarea spațiilor de lucru
    .vertical = Verticală
    .horizontal = Orizontală
hot-corner = Colț activ
    .top-left-corner = Activează colțul activ stânga-sus pentru Spațiile de lucru

## Displays

-requires-restart = Necesită o repornire
color = Culoare
    .depth = Adâncime de culoare
    .profile = Profil de culoare
    .sidebar = Profiluri de culoare
    .temperature = Temperatură de culoare
display = Ecrane
    .desc = Gestionează ecrane, comutare grafică și lumină de noapte
    .arrangement = Aranjarea ecranelor
    .arrangement-desc = Trage ecranele pentru a le rearanja.
    .enable = Activează ecranul
    .external = Ecran extern { $size } { $output }
    .laptop = Ecran laptop { $size }
    .options = Opțiuni ecran
    .refresh-rate = Rată de reîmprospătare
    .resolution = Rezoluție
    .scale = Scalare
    .additional-scale-options = Opțiuni suplimentare de scalare
mirroring = Oglindire
    .id = Oglindire { $id }
    .dont = Nu oglindi
    .mirror = Oglindește { $display }
    .project =
        Proiectează către { $display ->
            [all] toate ecranele
           *[other] { $display }
        }
    .project-count =
        Proiectează către alt(e) { $count } { $count ->
            [1] ecran
           *[other] ecrane
        }
night-light = Lumină de noapte
    .auto = Automat (de la apus la răsărit)
    .desc = Reduce lumina albastră cu culori mai calde.
orientation = Orientare
    .standard = Standard
    .rotate-90 = Rotește 90°
    .rotate-180 = Rotește 180°
    .rotate-270 = Rotește 270°
vrr = Rată de reîmprospătare variabilă
    .enabled = Activată
    .force = Întotdeauna
    .auto = Automat
    .disabled = Dezactivată
scheduling = Programare
    .manual = Program manual
dialog = Dialog
    .title = Păstrează aceste setări de afișare?
    .keep-changes = Păstrează modificările
    .change-prompt = Modificările vor fi anulate automat în { $time } secunde.
    .revert-settings = Reinstaurează setările anterioare

## Sound

sound = Sunet
    .desc = N/A
sound-output = Ieșire
    .volume = Volum ieșire
    .device = Dispozitiv de ieșire
    .level = Nivel ieșire
    .config = Configurație
    .balance = Echilibru
    .left = Stânga
    .right = Dreapta
sound-input = Intrare
    .volume = Volum intrare
    .device = Dispozitiv de intrare
    .level = Nivel intrare
sound-alerts = Alerte
    .volume = Volum alerte
    .sound = Sunet alertă
sound-applications = Aplicații
    .desc = Volum și setări pentru aplicații

## Power

power = Alimentare & Baterie
    .desc = Gestionează setările de alimentare
battery = Baterie
    .minute =
        { $value } { $value ->
            [one] minut
           *[other] minute
        }
    .hour =
        { $value } { $value ->
            [one] oră
           *[other] ore
        }
    .day =
        { $value } { $value ->
            [one] zi
           *[other] zile
        }
    .less-than-minute = Mai puțin de un minut
    .and = și
    .remaining-time =
        { $time } până când { $action ->
            [full] este complet încărcată
           *[other] se descarcă
        }
connected-devices = Dispozitive conectate
    .unknown = Dispozitiv necunoscut
power-mode = Mod de alimentare
    .battery = Durată extinsă a bateriei
    .battery-desc = Consum redus de energie și performanță relativ scăzută.
    .balanced = Echilibrat
    .balanced-desc = Performanță echilibrată și consum moderat de energie.
    .performance = Performanță ridicată
    .performance-desc = Performanță maximă și consum ridicat de energie.
    .no-backend = Backend-ul nu a fost găsit. Instalează system76-power sau power-profiles-daemon.
power-saving = Opțiuni de economisire a energiei
    .turn-off-screen-after = Oprește ecranul după
    .auto-suspend = Suspendare automată
    .auto-suspend-ac = Suspendare automată când este sistemul este conectat la priză
    .auto-suspend-battery = Suspendare automată pe baterie

## Input

acceleration-desc = Ajustează automat sensibilitatea de urmărire în funcție de viteză.
disable-while-typing = Dezactivează în timpul tastării
input-devices = Dispozitive de intrare
    .desc = Dispozitive de intrare
primary-button = Buton principal
    .desc = Setează ordinea butoanelor fizice.
    .left = Stânga
    .right = Dreapta
scrolling = Derulare
    .two-finger = Derulare cu două degete
    .edge = Derulare pe margine cu un deget
    .speed = Viteza de derulare
    .natural = Derulare naturală
    .natural-desc = Derulează conținutul, în locul vizualizării

## Input: Keyboard

slow = Încet
fast = Rapid
short = Scurt
long = Lung
keyboard = Tastatură
    .desc = Surse de intrare, comutare, introducerea de caractere speciale, scurtături.
keyboard-sources = Surse de intrare
    .desc = Sursele de intrare pot fi comutate folosind combinația Super+Space. Aceasta poate fi personalizată în setările pentru scurtături de tastatură.
    .move-up = Mută în sus
    .move-down = Mută în jos
    .settings = Setări
    .view-layout = Vezi aranjamentul tastaturii
    .remove = Elimină
    .add = Adaugă sursă de intrare
keyboard-special-char = Introducerea caracterelor speciale
    .alternate = Tasta pentru caractere alternative
    .compose = Tasta Compose
    .caps = Tasta Caps Lock
keyboard-typing-assist = Tastare
    .repeat-rate = Rată de repetare
    .repeat-delay = Întârzierea la repetare
keyboard-numlock-boot = Numlock
    .boot-state = Starea la pornire
    .last-boot = Ultima pornire
    .on = Activat
    .off = Dezactivat
    .set = Setează starea Numlock la pornire
added = Adăugat
type-to-search = Tastează pentru a căuta...
show-extended-input-sources = Afișează sursele de intrare extinse

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Scurtături de tastatură
    .desc = Vezi și personalizează scurtăturile
add-another-keybinding = Adaugă o altă combinație de taste
cancel = Anulează
command = Comandă
custom = Personalizat
debug = Depanare
disabled = Dezactivat
input-source-switch = Comută sursa de intrare a limbii tastaturii
migrate-workspace-prev = Migrează spațiul de lucru la ieșirea anterioară
migrate-workspace-next = Migrează spațiul de lucru la ieșirea următoare
migrate-workspace =
    Migrează spațiul de lucru la ieșirea { $direction ->
       *[down] jos
        [left] stânga
        [right] dreapta
        [up] sus
    }
navigate = Navighează
replace = Înlocuiește
shortcut-name = Nume scurtătură
system-controls = Controale de sistem
terminate = Termină
toggle-stacking = Comută stivuirea ferestrelor
type-key-combination = Tastează combinația de taste
custom-shortcuts = Scurtături personalizate
    .add = Adaugă o scurtătură
    .context = Adaugă scurtătură personalizată
    .none = Fără scurtături personalizate
modified = { $count } modificate
nav-shortcuts = Navigare
    .prev-output = Focalizează ieșirea anterioară
    .next-output = Focalizează ieșirea următoare
    .last-workspace = Focalizează ultimul spațiu de lucru
    .prev-workspace = Focalizează spațiul de lucru anterior
    .next-workspace = Focalizează spațiul de lucru următor
    .focus =
        Focalizează fereastra { $direction ->
           *[down] jos
            [in] în
            [left] stânga
            [out] afară
            [right] dreapta
            [up] sus
        }
    .output =
        Comută la ieșirea { $direction ->
           *[down] jos
            [left] stânga
            [right] dreapta
            [up] sus
        }
    .workspace = Comută la spațiul de lucru { $num }
manage-windows = Gestionarea ferestrelor
    .close = Închide fereastra
    .maximize = Maximizează fereastra
    .minimize = Minimizează fereastra
    .resize-inwards = Redimensionează fereastra înspre interior
    .resize-outwards = Redimensionează fereastra înspre exterior
    .toggle-sticky = Comută fereastra lipicioasă
move-windows = Mută ferestrele
    .direction =
        Mută fereastra în { $direction ->
           *[down] jos
            [left] stânga
            [right] dreapta
            [up] sus
        }
    .display =
        Mută fereastra la un monitor în { $direction ->
           *[down] jos
            [left] stânga
            [right] dreapta
            [up] sus
        }
    .workspace =
        Mută fereastra la un spațiu de lucru în { $direction ->
           *[below] sub
            [left] stânga
            [right] dreapta
            [above] deasupra
        }
    .workspace-num = Mută fereastra la spațiul de lucru { $num }
    .prev-workspace = Mută fereastra la spațiul de lucru anterior
    .next-workspace = Mută fereastra la spațiul de lucru următor
    .last-workspace = Mută fereastra la ultimul spațiu de lucru
    .next-display = Mută fereastra la următorul ecran
    .prev-display = Mută fereastra la ecranul anterior
    .send-to-prev-workspace = Mută fereastra la spațiul de lucru anterior
    .send-to-next-workspace = Mută fereastra la spațiul de lucru următor
system-shortcut = Sistem
    .app-library = Deschide biblioteca de aplicații
    .brightness-down = Scade luminozitatea ecranului
    .brightness-up = Crește luminozitatea ecranului
    .home-folder = Deschide folderul home
    .keyboard-brightness-down = Scade luminozitatea tastaturii
    .keyboard-brightness-up = Crește luminozitatea tastaturii
    .launcher = Deschide launcher-ul
    .log-out = Deconectează-te
    .lock-screen = Blochează ecranul
    .mute = Pune pe mut ieșirea audio
    .mute-mic = Pune pe mut intrarea microfonului
    .play-pause = Redă/Pauză
    .play-next = Următorul track
    .play-prev = Track anterior
    .screenshot = Fă o captură de ecran
    .terminal = Deschide un terminal
    .volume-lower = Scade volumul audio
    .volume-raise = Crește volumul audio
    .web-browser = Deschide un browser web
    .window-switcher = Comută între ferestrele deschise
    .window-switcher-previous = Comută între ferestrele deschise invers
    .workspace-overview = Deschide vizualizarea spațiilor de lucru
window-tiling = Aranjarea feronsterelor
    .horizontal = Setează orientarea orizontală
    .vertical = Setează orientarea verticală
    .swap-window = Schimbă fereastra
    .toggle-tiling = Comută aranjarea ferestrelor
    .toggle-stacking = Comută stivuirea ferestrelor
    .toggle-floating = Comută ferestrele plutitoare
    .toggle-orientation = Comută orientarea
replace-shortcut-dialog = Înlocuiește scurtătura?
    .desc = { $shortcut } este folosită de { $name }. Dacă o înlocuiești, { $name } va fi dezactivat.
zoom-in = Mărește
zoom-out = Micșorează

## Input: Mouse

mouse = Mouse
    .desc = Viteza mouse-ului, accelerare, derulare naturală.
    .speed = Viteza mouse-ului
    .acceleration = Activează accelerația mouse-ului

## Input: Touchpad

click-behavior = Comportament la click
    .click-finger = Click secundar cu două degete și click mijlociu cu trei degete
    .button-areas = Click secundar în colțul din dreapta jos și click mijlociu în mijlocul părții de jos
pinch-to-zoom = Zoom prin pinch
    .desc = Folosește două degete pentru a face zoom în conținut, pentru aplicațiile care suportă zoom.
tap-to-click = Tap pentru click
    .desc = Permite tap cu un deget pentru click primar, tap cu două degete pentru click secundar și tap cu trei degete pentru click mijlociu.
touchpad = Touchpad
    .acceleration = Activează accelerația touchpad-ului
    .desc = Viteza touchpad-ului, opțiuni de click, gesturi.
    .speed = Viteza touchpad-ului

## Input: Gestures

gestures = Gesturi
    .four-finger-down = Swipe cu patru degete în jos
    .four-finger-left = Swipe cu patru degete în stânga
    .four-finger-right = Swipe cu patru degete în dreapta
    .four-finger-up = Swipe cu patru degete în sus
    .three-finger-any = Swipe cu trei degete în orice direcție
switch-workspaces = Comută între spațiile de lucru
    .horizontal = Swipe cu patru degete stânga/dreapta
    .vertical = Swipe cu patru degete sus/jos
switch-between-windows = Comută între feronstere
open-application-library = Deschide biblioteca de aplicații
open-workspaces-view = Deschide vizualizarea spațiilor de lucru

## Time & Language

time = Timp & Limbă
    .desc = N/A
time-date = Dată & Timp
    .desc = Fus orar, setări automate ale ceasului și unele formate de timp.
    .auto = Setează automat
    .auto-ntp = Data și ora vor fi actualizate automat când fusul orar este setat.
time-zone = Fus orar
    .auto = Fus orar automat
    .auto-info = Necesită servicii de locație și acces la internet
time-format = Format dată & oră
    .twenty-four = Ora în format 24h
    .show-seconds = Afișează secunde
    .first = Prima zi a săptămânii
    .show-date = Afișează data pe panoul superior
    .friday = Vineri
    .saturday = Sâmbătă
    .sunday = Duminică
    .monday = Luni
time-region = Regiune & Limbă
    .desc = Formatează datele, orele și numerele în funcție de regiunea ta
formatting = Formatare
    .dates = Date
    .time = Oră
    .date-and-time = Dată și oră
    .numbers = Numere
    .measurement = Unități de măsură
    .paper = Format hârtie
preferred-languages = Limbi preferate
    .desc = Ordinea limbilor determină ce limbă este folosită pentru traducerea interfeței. Modificările au efect după următoarea autentificare.
add-language = Adaugă limbă
    .context = Adaugă limbă
install-additional-languages = Instalează limbi suplimentare
region = Regiune

## Applications

applications = Aplicații

## Applications: Default Applications

default-apps = Aplicații implicite
    .desc = Browser web, client de email, manager de fișiere și alte aplicații implicite.
    .web-browser = Browser web
    .file-manager = Manager de fișiere
    .mail-client = Client de email
    .music = Muzică
    .video = Video
    .photos = Fotografii
    .calendar = Calendar
    .terminal = Terminal
    .other-associations = Alte asocieri
    .text-editor = Editor de text

## Applications: Startup Applications

startup-apps = Aplicații la pornire
    .desc = Configurează aplicațiile care se deschid la autentificare.
    .add = Adaugă aplicație
    .user = Aplicații specifice utilizatorului
    .user-description = Aceste aplicații se lansează când te autentifici cu utilizatorul curent.
    .remove-dialog-title = Elimină { $name }?
    .remove-dialog-description = Sigur dorești să elimini această aplicație de la pornire?
    .search-for-application = Caută aplicație

## Applications: Legacy Applications

legacy-applications = Compatibilitate aplicații X11
    .desc = Scalare aplicații X11 și scurtături globale.
legacy-app-global-shortcuts = Scurtături globale în aplicații X11
    .desc = Scurtăturile globale permit ca apăsările de taste și butoane de mouse efectuate în aplicații să fie recunoscute de alte aplicații pentru funcții precum push-to-talk sau push-to-mute. Implicit, este dezactivat în aplicațiile X11 pentru a împiedica monitorizarea de către alte aplicații a evenimentelor sensibile de la tastatură și mouse.
    .none = Fără taste
    .modifiers = Modificatori (Super, Shift, Control, Alt)
    .combination = Toate tastele apăsate împreună cu Super, Control sau Alt
    .all = Toate tastele
    .mouse = Evenimente de butoane mouse în aplicații X11
legacy-app-scaling = Scalarea aplicațiilor în sistemul X11
    .scaled-by-system = Scalează toate aplicațiile X11
    .system-description = Aplicațiile X11 vor apărea neclare pe ecrane HiDPI.
    .scaled-natively = Randează aplicațiile X11 la rezoluția nativă
    .native-description = Aplicațiile X11 care nu suportă scalarea vor apărea mici pe ecranele HiDPI. Activează această opțiune pentru jocuri pentru a folosi întreaga rezoluție a monitorului.

## System

system = Sistem & Conturi

## System: About

about = Despre
    .desc = Numele dispozitivului, informații despre hardware și sistemul de operare.
about-device = Nume dispozitiv
    .desc = Acest nume este vizibil altor dispozitive din rețea sau prin Bluetooth.
about-hardware = Hardware
    .model = Model hardware
    .memory = Memorie
    .processor = Procesor
    .graphics = Grafică
    .disk-capacity = Capacitate disc
about-os = Sistem de operare
    .os = Sistem de operare
    .os-architecture = Arhitectura sistemului
    .desktop-environment = Mediu desktop
    .windowing-system = Sistem de ferestre
about-related = Setări conexe
    .support = Obține suport

## System: Firmware

firmware = Firmware
    .desc = Detalii despre firmware.

## System: Users

users = Utilizatori
    .desc = Autentificare și conturi de utilizatori.
    .admin = Administrator
    .standard = Standard
    .profile-add = Alege imaginea de profil
administrator = Administrator
    .desc = Administratorii pot modifica setări pentru toți utilizatorii, adăuga sau elimina utilizatori.
add-user = Adaugă utilizator
remove-user = Elimină utilizator
full-name = Nume complet
invalid-username = Nume de utilizator invalid
