app = COSMIC Indstillinger
dbus-connection-error = Kunne ikke oprette forbindelse til DBus
ok = OK
unknown = Ukendt
number = { $number }

## Network & Wireless

connections-and-profiles =
    { $variant ->
        [wired] Kablet
        [wifi] Wi-Fi
        [vpn] VPN
       *[other] Ukendt
    } forbindelser og forbindelsesprofiler.
add-network = Tilføj Netværk
    .profile = Tilføj profil
add-vpn = Tilføj VPN
airplane-on = Airplane mode is on. Flytilstand er slået til
cable-unplugged = Kabel frakoblet
connect = Tilslut
connected = Tilsluttet
connecting = Tilslutter…
disconnect = Afbryd
forget = Glem
known-networks = Kendte Netværk
network-and-wireless = Netværk & trådløst
no-networks = Der blev ikke fundet nogen netværk.
no-vpn = Ingen VPN-forbindelser tilgængelige.
password = Adgangskode
password-confirm = Godkend Adgangskode
remove = Fjern
settings = Indstillinger
username = Brugernavn
visible-networks = Synlige Netværk
identity = Identitet
forget-dialog = Glem dette Wi-Fi-netværk?
    .description = Du skal indtaste en adgangskode igen for at bruge dette Wi-Fi-netværk i fremtiden.
network-device-state =
    .activated = Tilsluttet
    .config = Opretter forbindelse
    .deactivating = Afbryder
    .disconnected = Afbrudt
    .failed = Kunne ikke oprette forbindelse
    .ip-check = Kontrollerer forbindelsen
    .ip-config = Anmoder om IP- og routinginformation
    .need-auth = Har brug for godkendelse
    .prepare = Forbereder tilslutning
    .secondaries = Venter på sekundær forbindelse
    .unavailable = Ikke tilgængelig
    .unknown = Ukendt tilstand
    .unmanaged = Ikke administreret
    .unplugged = Kabel taget ud
remove-connection-dialog = Fjern forbindelsesprofil?
    .vpn-description = Du skal indtaste en adgangskode igen for at bruge dette netværk i fremtiden.
    .wired-description = Du skal oprette denne profil for at bruge den i fremtiden.
vpn = VPN
    .connections = VPN forbindelser
    .error = Det lykkedes ikke at tilføje VPN konfiguration
    .remove = Fjern forbindelsesprofilen
    .select-file = Vælg en VPN konfigurationsfil
vpn-error = VPN Fejl
    .config = Kunne ikke tilføje VPN konfiguration
    .connect = Kunne ikke oprette forbindelse til VPN
    .connection-editor = Forbindelsesredigering mislykkedes
    .connection-settings = Indstillingerne for aktive forbindelser kunne ikke hentes
    .updating-state = Kunne ikke opdatere netværksadministrators tilstand
    .wireguard-config-path = Ugyldig filsti for WireGuard-konfigurationen
    .wireguard-config-path-desc = Valgt fil skal være på et lokalt filsystem.
    .wireguard-device = Kunne ikke oprette WireGuard-enhed
    .with-password =
        Det lykkedes ikke at indstille VPN { $field ->
           *[username] brugernavn
            [password] adgangskode
            [password-flags] password-flags
        } with nmcli
wired = Kablet
    .adapter = Kablet adapter { $id }
    .connections = Kablede forbindelser
    .devices = Kablede enheder
    .remove = Fjern forbindelsesprofil
wifi = Wi-Fi
    .adapter = Wi-Fi adapter { $id }
    .forget = Glem dette netværk
wireguard-dialog = Tilføj WireGuard enhed
    .description = Vælg et enhedsnavn til WireGuard konfigurationen.

## Networking: Online konti

online-accounts = Online konti
    .desc = Tilføj konti, IMAP og SMTP, enterprise logins

# Bluetooth

activate = Aktiver
confirm = Bekræft
enable = Enable
bluetooth = Bluetooth
    .desc = Administrer Bluetooth enheder
    .status = This system is visible as { $aliases } while Bluetooth settings are open.
    .connected = Tilsluttet
    .connecting = Tilslutter
    .disconnecting = Afbryder forbindelsen
    .connect = Tilslut
    .disconnect = Afbryd forbindelsen
    .forget = Glem
    .dbus-error = Der opstod en fejl under interaktion med DBus: { $why }
    .disabled = Bluetooth-tjenesten er deaktiveret
    .inactive = Bluetooth-tjenesten er ikke aktiv
    .unknown = Bluetooth-tjenesten kunne ikke aktiveres. Er BlueZ installeret?
bluetooth-paired = Tidligere forbundne enheder
    .connect = Tilslut
    .battery = { $percentage }% batteri
bluetooth-confirm-pin = Bekræft Bluetooth PIN-kode
    .description = Bekræft venligst, at følgende PIN-kode svarer til den, der vises på { $device }
bluetooth-available = Enheder i nærheden
bluetooth-adapters = Bluetooth Enheder

## Accessibility

accessibility = Tilgængelighed
    .vision = Syn
    .on = Til
    .off = Fra
    .unavailable = Ikke tilgængelig
    .screen-reader = Skærmlæser
    .high-contrast = Høj kontrast
    .invert-colors = Inverter farver
    .color-filters = Farvefiltre
hearing = Hørelse
    .mono = Afspil stereolyd som mono
default = Standard
magnifier = Forstørrelsesglas
    .controls =
        Eller brug disse genveje: { $zoom_in ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_in } for at zoome ind,
        }{ $zoom_out ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_out } for at zoome ud,
        }
        Super + scroll med din mus
    .scroll_controls = Aktivér zoom med mus eller touchpad med Super + Scroll
    .show_overlay = Vis forstørrelsesglas-overlay
    .increment = Zoom-trin
    .signin = Start forstørrelsesglas ved login
    .applet = Skift forstørrelsesglas til/fra i panelets applet
    .movement = Forstørret visning bevæger sig
    .continuous = Kontinuerligt med markøren
    .onedge = Når markøren når kanten
    .centered = For at holde markøren centreret
color-filter = Farvefiltertype
    .unknown = Ukendt filter aktivt
    .greyscale = Gråtoner
    .deuteranopia = Grøn/Rød (grøn-svaghed, Deuteranopia)
    .protanopia = Rød/Grøn (rød-svaghed, Protanopia)
    .tritanopia = Blå/Gul (blå-svaghed, Tritanopia)

## Desktop

desktop = Desktop

## Desktop: Baggrundbillede

wallpaper = Baggrundbillede
    .change = Skift billede hvert
    .desc = Baggrundbilleder, farver, og slideshow indstillinger
    .fit = Tilpas baggrundbillede
    .folder-dialog = Vælg baggrundbillede mappe
    .image-dialog = Vælg baggrundbillede
    .plural = Baggrundbillede
    .same = Samme baggrundbillede på alle skærme
    .slide = Slideshow
add-color = Tilføj farve
add-image = Tilføj billede
all-displays = Alle Skærme
colors = Farver
dialog-add = Tilføj
fill = Fyld
fit-to-screen = Tilpas til Skærm
open-new-folder = Åben ny mappe
recent-folders = Seneste Mapper
x-minutes =
    { $number } { $number ->
        [one] minut
       *[other] minutter
    }
x-hours =
    { $number } { $number ->
        [one] time
       *[other] timer
    }
never = Aldrig

## Desktop: Udseende

appearance = Udseende
    .desc = Accentfarver og tema.
accent-color = Accentfarve
app-background = Applikation eller vinduesbaggrund
auto = Auto
close = Luk
color-picker = Farvevælger
copied-to-clipboard = Kopieret til udklipsholder
copy-to-clipboard = Kopiér til udklipsholder
dark = Mørk
export = Eksporter
hex = Hex
import = Importer
light = Lys
mode-and-colors = Tilstand og farver
recent-colors = Seneste farver
reset-to-default = Nulstil til standard
rgb = RGB
window-hint-accent = Aktivt vinduestip farve
window-hint-accent-toggle = Brug temaaccentfarve som aktivt vinduestip
auto-switch = Skift automatisk mellem Lys og Mørk tilstande
    .sunrise = Skifter til Lystilstand ved solopgang
    .sunset = Skifter til Mørk tilstand ved solnedgang
    .next-sunrise = Skifter til Lystilstand ved næste solopgang
    .next-sunset = Skifter til Mørk tilstand ved næste solnedgang
container-background = Baggrund for beholder
    .desc-detail = Beholderbaggrundsfarve bruges til navigationssidebjælke, sideskuffe, dialogbokse og lignende widgets. Som standard er det automatisk afledt fra programmet eller vinduets baggrund.
    .reset = Nulstil til auto
    .desc = Primær beholderfarve bruges til navigationssidebjælken, sideskuffe, dialogbokse og lignende widgets.
control-tint = Kontrolkomponent toning
    .desc = Anvendes til baggrunde af standardknapper, søgeinput, tekstinput og lignende komponenter.
frosted = Frosted glass effect on system interface
    .desc = Anvender baggrundssløring på panel, dock, applets, launcher og applikationsbibliotek.
enable-export = Anvend dette tema til GNOME apps.
    .desc = Ikke alle værktøjssæt understøtter automatisk skift. Ikke-COSMIC apps skal muligvis genstartes efter et temaskift.
icon-theme = Ikon tema
    .desc = Anvender et andet sæt ikoner til applikationer.
text-tint = Interface text tint
    .desc = Color used to derive interface text colors that have sufficient contrast on various surfaces.
style = Stil
    .round = Rund
    .slightly-round = Let afrundet
    .square = Firkant
interface-density = Grænsefladerdensitet
    .comfortable = Komfortabel
    .compact = Kompakt
    .spacious = Rummelig
window-management-appearance = Window Management
    .active-hint = Aktiv vinduestip størrelse
    .gaps = Mellemrum omkring flisevinduer

### Experimental

experimental-settings = Eksperimentelle Indstillinger
icons-and-toolkit = Ikoner og toolkit tema
interface-font = System skrifttype
monospace-font = Monospace skrifttype

## Desktop: Notifikationer

notifications = Notifikationer
    .desc = Forstyr ikke, meddelelser på låseskærm og indstillinger pr. applikation.

## Desktop: Panel

panel = Panel
    .desc = Øverste bjælke med skrivebordskontroller og menuer.
add = Tilføj
add-applet = Tilføj Applet
all = Alle
applets = Appletter
center-segment = Center segment
end-segment = Slut segment
large = Stor
no-applets-found = Der blev ikke fundet nogen applets...
panel-bottom = Bund
panel-left = Venstre
panel-right = Højre
panel-top = Top
search-applets = Søg efter appletter...
small = Lille
start-segment = Start Segment
panel-appearance = Udseende
    .match = Match skrivebordet
    .light = Lys
    .dark = Mørk
panel-behavior-and-position = Adfærd og Positioner
    .autohide = Skjul automatisk panelet
    .dock-autohide = Skjul automatisk dock
    .position = Position på skærmen
    .display = Vis på skærmen
panel-style = Stil
    .anchor-gap = Mellemrum mellem panel og skærmkanter
    .dock-anchor-gap = Mellemrum mellem dock og skærmkanter
    .extend = Udvid panelet til skærmens kanter
    .dock-extend = Forlæng docken til skærmens kanter
    .appearance = Udseende
    .size = Størelse
    .background-opacity = Baggrundens uigennemsigtighed
panel-applets = Konfiguration
    .dock-desc = Konfigurer dock appletter
    .desc = Konfigurer panel appletter
panel-missing = Panelkonfiguration mangler
    .desc = Panelkonfigurationsfilen mangler på grund af brug af en brugerdefineret konfiguration, eller den er beskadiget.
    .fix = Nulstil til standard

## Desktop: Dock

dock = Dock
    .desc = Panel med fastgjorte applikationer i app bakken og andre applets

## Desktop: Vinduesstyring

window-management = Vinduesstyring
    .desc = Super tast handling, vindue kontrolmuligheder og yderligere muligheder for fliser.
super-key = Super tast handling
    .launcher = Åben Starter
    .workspaces = Åben Arbejdsområder
    .applications = Åben Applikationer
    .disable = Deaktiver
edge-gravity = Flydende vinduer bevæger sig mod nærmeste kant
window-controls = Vinduesstyring
    .maximize = Vis maksimer knap
    .minimize = Vis minimer knap
    .active-window-hint = Vis aktive vinduestip
focus-navigation = Fokus navigation
    .focus-follows-cursor = Fokus følger markøren
    .cursor-follows-focus = Markøren følger fokus

## Desktop: Arbejdsområder

workspaces = Arbejdsområder
    .desc = Arbejdsområde orientering og adfærd.
workspaces-behavior = Opførsel for Arbejdsområder
    .dynamic = Dynamiske Arbejdsområder
    .dynamic-desc = Fjerner automatisk tomme Arbejdsområder.
    .fixed = Fast antal Arbejdsområder
    .fixed-desc = Tilføj eller fjern Arbejdsområder i oversigten.
workspaces-multi-behavior = Adfær for flere skærme
    .span = Arbejdsområder spræder sig over flere skærme
    .separate = Skærme har separate Arbejdsområder
workspaces-overview-thumbnails = Oversigt over Arbejdsområde Miniaturer
    .show-number = Vis Arbejdsområdenummer
    .show-name = Vis navnet på Arbejdsområdet
workspaces-orientation = Arbejdsområde Orientering
    .vertical = Lodret
    .horizontal = Vandret
hot-corner = Varmt Hjørne
    .top-left-corner = Aktiver det øverste venstre varme hjørne for Arbejdsområder

## Skærme

-requires-restart = Kræver genstart
color = Farve
    .depth = Farvedybde
    .profile = Farveprofil
    .sidebar = Farveprofiler
    .temperature = Farvetemperatur
display = Displays
    .desc = Administrer skærme, grafikskift og natlys
    .arrangement = Display Arrangement
    .arrangement-desc = Træk skærme for at omarrangere dem.
    .enable = Aktiver skærm
    .external = { $size } { $output } Ekstern skærm
    .laptop = { $size } Bærbar skærm
    .options = Skærm indstillinger
    .refresh-rate = Opdateringshastighed
    .resolution = Opløsning
    .scale = Skallér
    .additional-scale-options = Ydeligere skalerings indstillinger
mirroring = Spejling
    .id = Spejling { $id }
    .dont = Spejl ikke
    .mirror = Spejl { $display }
    .project =
        Projekter til { $display ->
            [all] alle skærme
           *[other] { $display }
        }
    .project-count =
        Projektere til { $count } andre { $count ->
            [1] skærm
           *[other] skærme
        }
night-light = Natlys
    .auto = Automatisk (solnedgang til solopgang)
    .desc = Reducer blåt lys med varmere farver.
orientation = Orientering
    .standard = Standard
    .rotate-90 = Roter 90°
    .rotate-180 = Roter 180°
    .rotate-270 = Roter 270°
vrr = Variable refresh rate
    .enabled = Slå til
    .force = Altid
    .auto = Automatisk
    .disabled = Deaktiveret
scheduling = Planlægning
    .manual = Manuel planlægning
dialog = Dialog
    .title = Behold disse skærmindstillinger?
    .keep-changes = Behold ændringer
    .change-prompt = Ændringer af indstillinger vil automatisk vende tilbage om { $time } sekunder.
    .revert-settings = Gendan indstillinger

## Sound

sound = Lyd
    .desc = N/A
sound-output = Udgang
    .volume = Udgangs Lydstyrke
    .device = Udgangsenhed
    .level = Udgangsniveau
    .config = Konfiguration
    .balance = Balance
    .left = Venstre
    .right = Højre
sound-input = Input
    .volume = Input Lydstyrke
    .device = Input Enhed
    .level = Input Niveau
sound-alerts = Advarsler
    .volume = Lydstyrke for advarsler
    .sound = Alarmer lyde
sound-applications = Applikationer
    .desc = Applikation Lydstyrke og indstillinger

## Power

power = Strøm & Batteri
    .desc = Administrer strømindstillinger
battery = Batteri
    .minute =
        { $value } { $value ->
            [one] minut
           *[other] minutter
        }
    .hour =
        { $value } { $value ->
            [one] time
           *[other] timer
        }
    .day =
        { $value } { $value ->
            [one] dag
           *[other] dage
        }
    .less-than-minute = Mindre end et minut
    .and = and
    .remaining-time =
        { $time } until { $action ->
            [full] fuld
           *[other] tom
        }
connected-devices = Tilsluttede enheder
    .unknown = Ukendt enhed
power-mode = Strømtilstand
    .battery = Forlænget batterilevetid
    .battery-desc = Reduceret strømforbrug og lydløs ydeevne.
    .balanced = Balanceret
    .balanced-desc = Støjsvag ydeevne og moderat strømforbrug.
    .performance = Høj ydeevne
    .performance-desc = Maksimal ydeevne og strømforbrug.
    .no-backend = Backend blev ikke fundet. Installer system76-power or power-profiles-daemon.
power-saving = Strømstyrings Indstillinger
    .turn-off-screen-after = Sluk skærmen efter
    .auto-suspend = Automatisk slumre
    .auto-suspend-ac = Automatisk slumre når den er tilsluttet
    .auto-suspend-battery = Automatisk slumre på batteri

## Input

acceleration-desc = Justerer automatisk sporingsfølsomhed baseret på hastighed.
disable-while-typing = Deaktiver under indtastning
input-devices = Indgangsenheder
    .desc = Indgangsenheder
primary-button = Primær knap
    .desc = Indstiller rækkefølgen af ​​fysiske knapper.
    .left = Venstre
    .right = Højre
scrolling = Rulning
    .two-finger = Rul med to fingre
    .edge = Rul langs kanten med en finger
    .speed = Rulningshastighed
    .natural = Naturlig rulning
    .natural-desc = Rul indholdet i stedet for visningen

## Input: Keyboard

slow = Langsom
fast = Hurtig
short = kort
long = Lang
keyboard = Tastatur
    .desc = Indtastningskilder, skift, indtastning af specialtegn, genveje.
keyboard-sources = Inputkilder
    .desc = Inputkilder kan skiftes ved hjælp af Super+Mellemrumstastekombinationen. Dette kan tilpasses i indstillingerne for tastaturgenveje.
    .move-up = Flyt op
    .move-down = Flyt ned
    .settings = Indstillinger
    .view-layout = Vis tastaturlayout
    .remove = Fjern
    .add = Tilføj inputkilde
keyboard-special-char = Indtastning af specialtegn
    .alternate = Tast for alternative tegn
    .compose = Skriv tast
    .caps = Caps Lock tast
keyboard-typing-assist = Indtastning
    .repeat-rate = Gentagelseshastighed
    .repeat-delay = Gentagelsesforsinkelse
keyboard-numlock-boot = Numlock
    .boot-state = Tilstand ved start
    .last-boot = Sidste start
    .on = Til
    .off = Fra
    .set = Set numlock start tilstand
added = Tilføjet
type-to-search = Skriv for at søge...
show-extended-input-sources = Vis udvidede inputkilder

## Input: Keyboard: Genveje

keyboard-shortcuts = Tastaturgenveje
    .desc = Se og tilpas genveje
cancel = Annuller
command = Kommando
custom = Brugerdefineret
debug = Debug
disabled = Deaktiveret
input-source-switch = Skift inputkilde for tastatursprog
migrate-workspace-prev = Migrer arbejdsområdet til tidligere output
migrate-workspace-next = Migrer arbejdsområdet til næste output
migrate-workspace =
    Migrer arbejdsområdet til output { $direction ->
       *[down] ned
        [left] venstre
        [right] højre
        [up] op
    }
navigate = Navigér
replace = Erstat
shortcut-name = Genvejsnavn
system-controls = System controls
terminate = Terminate
toggle-stacking = Slå vinduestabling til/fra
type-key-combination = Indtast tastekombination
custom-shortcuts = Brugerdefinerede genveje
    .add = Tilføj genvej
    .context = Tilføj Brugerdefineret Genvej
    .none = Ingen brugerdefinerede genveje
modified = { $count } modificeret
nav-shortcuts = Navigation
    .prev-output = Fokusér forrige output
    .next-output = Fokusér næste output
    .last-workspace = Fokusér sidste arbejdsområde
    .prev-workspace = Fokusér forrige arbejdsområde
    .next-workspace = Fokusér næste arbejdsområde
    .focus =
        Fokus vindue { $direction ->
           *[down] ned
            [in] ind
            [left] venstre
            [out] ud
            [right] højre
            [up] op
        }
    .output =
        Skift til output { $direction ->
           *[down] ned
            [left] venstre
            [right] højre
            [up] op
        }
    .workspace = Skift til arbejdsområde { $num }
manage-windows = Administrer vinduer
    .close = Luk vindue
    .maximize = Maksimér vindue
    .fullscreen = Fuldskærm vindue
    .minimize = Minimér vindue
    .resize-inwards = Ændr størrelsen på vinduet indad
    .resize-outwards = Ændr størrelsen på vinduet udad
    .toggle-sticky = Slå klæbrigt vindue til/fra
move-windows = Flyt vinduer
    .direction =
        Flyt vindue { $direction ->
           *[down] ned
            [left] venstre
            [right] højre
            [up] op
        }
    .display =
        Flyt vindue én skærm { $direction ->
           *[down] ned
            [left] venstre
            [right] højre
            [up] op
        }
    .workspace =
        Flyt vindue ét arbejdsområde { $direction ->
           *[below] under
            [left] venstre
            [right] højre
            [above] over
        }
    .workspace-num = Flyt vindue til arbejdsområde { $num }
    .prev-workspace = Flyt vindue til det forrige arbejdsområde
    .next-workspace = Flyt vindue til næste arbejdsområde
    .last-workspace = Flyt vindue til sidste arbejdsområde
    .next-display = Flyt vindue til næste skærm
    .prev-display = Flyt vindue til forrige skærm
    .send-to-prev-workspace = Flyt vindue til forrige arbejdsområde
    .send-to-next-workspace = Flyt vindue til næste arbejdsområde
system-shortcut = System
    .app-library = Åbn appbiblioteket
    .brightness-down = Reducer skærmens lysstyrke
    .brightness-up = Øg skærmens lysstyrke
    .home-folder = Åbn hjemmemappe
    .keyboard-brightness-down = Reducer tastaturets lysstyrke
    .keyboard-brightness-up = Øg tastaturets lysstyrke
    .launcher = Åbn launcher
    .log-out = Log ud
    .lock-screen = Lås skærmen
    .mute = Slå lydudgang fra
    .mute-mic = Slår mikrofonindgangen fra
    .play-pause = Afspil/Pause
    .play-next = Næste nummer
    .play-prev = Forrige nummer
    .poweroff = Luk ned
    .screenshot = Tag et skærmbillede
    .terminal = Åbn en terminal
    .volume-lower = Sænk lydudgangs lydstyrken
    .volume-raise = Øg lydudgangs lydstyrken
    .web-browser = Åbner en webbrowser
    .window-switcher = Skift mellem åbne vinduer
    .window-switcher-previous = Skift mellem åbne vinduer omvendt
    .workspace-overview = Åbn oversigten over arbejdsområdet
window-tiling = Vinduesfliser
    .horizontal = Indstil vandret orientering
    .vertical = Indstil lodret orientering
    .swap-window = Byt vindue
    .toggle-tiling = Slå vinduesfliser til/fra
    .toggle-stacking = Slå vinduestabling til/fra
    .toggle-floating = Slå svævende vindue til/fra
    .toggle-orientation = Slå orientering til/fra
replace-shortcut-dialog = Erstat Genvej?
    .desc = { $shortcut } bruges af { $name }. Hvis du erstatter det, { $name } vil blive deaktiveret.
zoom-in = Zoom Ind
zoom-out = Zoom Ud

## Input: Mouse

mouse = Mouse
    .desc = Musehastighed, acceleration, naturlig rulning.
    .speed = Musehastighed
    .acceleration = Aktiver museacceleration

## Input: Touchpad

click-behavior = Klik Adfærd
    .click-finger = Sekundært klik med to fingre og midterklik med tre fingre
    .button-areas = Sekundært klik i nederste højre hjørne og midterste klik nederst i midten
pinch-to-zoom = Knib sammen for at zoome
    .desc = Brug to fingre til at zoome ind på indhold, for applikationer, der understøtter zoom.
tap-to-click = Tryk for at klikke
    .desc = Aktiverer tryk med én finger for primært klik, tryk med to fingre for sekundært klik og tryk med tre fingre for midterste klik.
touchpad = Touchpad
    .acceleration = Aktiver acceleration af touchpad
    .desc = Touchpad hastighed, klikindstillinger, bevægelser.
    .speed = Touchpad hastighed

## Input: Gestures

gestures = Gestures
    .four-finger-down = Stryg ned med fire fingre
    .four-finger-left = Stryg til venstre med fire fingre
    .four-finger-right = Stryg til højre med fire fingre
    .four-finger-up = Stryg opad med fire fingre
    .three-finger-any = Stryg med tre fingre i enhver retning
switch-workspaces = Skift arbejdsområde
    .horizontal = Stryg til venstre/højre med fire fingre
    .vertical = Stryg op/ned med fire fingre
switch-between-windows = Skift mellem vinduer
open-application-library = Åbn applikationsbiblioteket
open-workspaces-view = Åbn oversigt over arbejdsområder

## Time & Language

time = Tid & Sprog
    .desc = N/A
time-date = Dato & Tid
    .desc = Tidszone, automatiske urindstillinger og noget tidsformatering.
    .auto = Indstil automatisk
    .auto-ntp = Dato og tid opdateres automatisk, når tidszonen er indstillet.
time-zone = Tidszone
    .auto = Automatisk tidszone
    .auto-info = Kræver lokationstjenester og internetadgang
time-format = Dato & tidsformat
    .twenty-four = 24 timers ur
    .show-seconds = Vis sekunder
    .first = Første dag i ugen
    .show-date = Vis Dato på Toppanelet
    .friday = Fredag
    .saturday = Lørdag
    .sunday = Søndag
    .monday = Mandag
time-region = Region & Sprog
    .desc = Formater datoer, klokkeslæt og tal baseret på din region
formatting = Formatering
    .dates = Dates
    .time = Tid
    .date-and-time = Dato & Tid
    .numbers = Tal
    .measurement = Mål
    .paper = Papir
preferred-languages = Foretrukne sprog
    .desc = Rækkefølgen af ​​sprog bestemmer, hvilket sprog der bruges til oversættelsen af ​​skrivebordet. Ændringer træder i kraft ved næste login.
add-language = Tilføj sprog
    .context = Tilføj Sprog
install-additional-languages = Installer yderligere sprog
region = Region

## Applications

applications = Applikationer

## Applications: Default Applications

default-apps = Standardapplikationer
    .desc = Standard webbrowser, mailklient, filhåndtering og andre applikationer.
    .web-browser = Webbrowser
    .file-manager = Filhåndtering
    .mail-client = Mailklient
    .music = Musik
    .video = Video
    .photos = Billeder
    .calendar = Kalender
    .terminal = Terminal
    .other-associations = Andre tilknytninger
    .text-editor = Tekstredigering

## Applications: Startup Applications

startup-apps = Startapplikationer
    .desc = Konfigurer applikationer der starter ved login.
    .add = Tilføj app
    .user = Applikationer der åbnes ved login
    .none = Ingen startapplikationer tilføjet
    .remove-dialog-title = Fjern { $name }?
    .remove-dialog-description = Er du sikker på, at du vil fjerne denne startapplikation?
    .search-for-application = Søg efter applikation

## Applications: Legacy Applications

legacy-applications = X11 applikationskompatibilitet
    .desc = Skalering af X11 vinduessystemapplikationer og globale genveje.
legacy-app-global-shortcuts = Globale genveje i X11-applikationer
    .desc = Globale genveje tillader tastetryk og museklik udført i applikationer at blive genkendt af andre applikationer, f.eks. til push-to-talk eller push-to-mute. Som standard er dette deaktiveret i X11-applikationer for at forhindre, at andre applikationer kan overvåge tastatur- og museevents, der indeholder følsomme oplysninger.
    .none = Ingen taster
    .modifiers = Modifikatortaster (Super, Shift, Control, Alt)
    .combination = Alle taster, mens modifikatortasterne Super, Control eller Alt holdes nede
    .all = Alle taster
    .mouse = Museklik i X11-applikationer
legacy-app-scaling = Skalering af X11 vinduessystemapplikationer
    .scaled-gaming = Optimer til spil og fuldskærmsapps
    .gaming-description = X11-applikationer kan fremstå lidt større eller mindre sammenlignet med Wayland-apps.
    .scaled-applications = Optimer til applikationer
    .applications-description = Spil og fuldskærms X11-apps matcher måske ikke din skærmopløsning.
    .scaled-compatibility = Maksimal kompatibilitetstilstand
    .compatibility-description = X11-applikationer kan se slørede ud på HiDPI-skærme.
    .preferred-display = Foretrukken skærm til spil og fuldskærms X11-applikationer
    .no-display = Ingen

## System

system = System & Konti

## System: Om

about = Om
    .desc = Enhedsnavn, hardwareoplysninger, standardindstillinger for operativsystem.
about-device = Enhed navn
    .desc = Dette navn vises til andre netværks eller bluetooth enheder.
about-hardware = Hardware
    .model = Hardware model
    .memory = Hukommelse
    .processor = Processor
    .graphics = Grafik
    .disk-capacity = Diskkapacitet
about-os = Operativsystem
    .os = Operativsystem
    .os-architecture = Operativsystems arkitektur
    .desktop-environment = Skrivebordsmiljø
    .windowing-system = Vinduessystem
about-related = Relaterede indstillinger
    .support = Få support

## System: Firmware

firmware = Firmware
    .desc = Firmware detaljer.

## System: Brugere

users = Brugere
    .desc = Godkendelse og brugerkonti.
    .admin = Administrator
    .standard = Standard
    .profile-add = Vælg profilbillede
administrator = Administrator
    .desc = Administratorer kan ændre indstillinger for alle brugere samt tilføje og fjerne andre brugere.
add-user = Tilføj bruger
change-password = Skift adgangskode
remove-user = Fjern bruger
full-name = Fulde navn
invalid-username = Ugyldigt brugernavn.
password-mismatch = Adgangskode og bekræftelse skal være ens.
save = Gem
