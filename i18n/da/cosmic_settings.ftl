app = COSMIC Indstillinger

dbus-connection-error = Failed to connect to DBus
ok = OK
unknown = Ukendt

number = { $number }

## Network & Wireless

connections-and-profiles = { $variant ->
    [wired] Kablet
    [wifi] Wi-Fi
    [vpn] VPN
    *[other] Unknown
} connections and connection profiles.

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
remove = Fjern
settings = Indstillinger
username = Brugernavn
visible-networks = Synlige Netværk

Auth-dialog = Godkendelse Påkrævet
    .vpn-description = Indtast brugernavnet og adgangskoden, der kræves af VPN-tjenesten.
    .wifi-description = Indtast adgangskoden eller krypteringsnøglen. Du kan også oprette forbindelse ved at trykke på "WPS"-knappen på routeren.

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
    .with-password = Failed to set VPN { $field ->
        *[username] username
        [password] password
        [password-flags] password-flags
    } with nmcli

wired = Kablet
    .adapter = Wired adapter { $id }
    .connections = Wired Connections
    .devices = Wired Devices
    .remove = Remove connection profile

wifi = Wi-Fi
    .adapter = Wi-Fi adapter { $id }
    .forget = Glem dette netværk
    
wireguard-dialog = Tilføj WireGuard enhed
    .description = Vælg et enhedsnavn til WireGuard konfigurationen.

## Networking: Online konti 

online-accounts = Online konti
    .desc = Tilføj konti, IMAP og SMTP, enterprise logins

# Bluetooth

confirm = Bekræft

bluetooth = Bluetooth
    .desc = Administrer Bluetooth-enheder
    .status = This system is visible as { $aliases } while Bluetooth settings are open.
    .connected = Tilsluttet
    .connecting = Tilslutter
    .disconnecting = Afbryder forbindelsen
    .connect = Tilslut
    .disconnect = Afbryd forbindelsen
    .forget = Glem
    .dbus-error = An error has occurred while interacting with DBus: { $why }
    .show-device-without-name = Vis enheder uden navn

bluetooth-paired = Tidligere forbundne enheder
    .connect = Tilslut
    .battery = { $percentage }% battery

bluetooth-confirm-pin = Bekræft Bluetooth PIN-kode
    .description = Bekræft venligst, at følgende PIN-kode svarer til den, der vises på { $device }

bluetooth-available = Enheder i nærheden

bluetooth-adapters = Bluetooth Adapters

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
fill =Fyld
fit-to-screen = Tilpas til Skærm
open-new-folder = Åben ny mappe
recent-folders = Recent Folders Seneste Mapper

x-minutes = { $number } minutter
x-hours = { $number ->
    [1] 1 time
    *[other] { $number } timer
}
never = Never

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

container-background = Container background
    .desc-detail = Container background color is used for navigation sidebar, side drawer, dialogs and similar widgets. By default, it is automatically derived from the Application or window background.
    .reset = Nulstil til auto
    .desc = Primary container color is used for navigation sidebar, side drawer, dialogs and similar widgets.

control-tint = Control component tint
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

interface-density = Interface Density
    .comfortable = Comfortable
    .compact = Compact
    .spacious = Spacious

window-management-appearance = Window Management
    .active-hint = Active window hint size
    .gaps = Gaps around tiled windows

### Experimental

experimental-settings = Experimental Settings
icons-and-toolkit = Icons and toolkit theming
interface-font = System font
monospace-font = Monospace font

## Desktop: Notifikationer

notifications = Notifikationer
    .desc = Forstyr ikke, meddelelser på låseskærm og indstillinger pr. applikation.

## Desktop: Panel

panel = Panel
    .desc = Øverste bjælke med skrivebordskontroller og menuer.

add = Tilføj
add-applet = Tilføj Applet
all = Alle
applets = Applets
center-segment = Center Segment
drop-here = Slip applets her
end-segment = End Segment
large = Stor
no-applets-found = Der blev ikke fundet nogen applets...
panel-bottom = Bund
panel-left = Venstre
panel-right = Højre
panel-top = Top
search-applets = Søg efter applets...
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
    .background-opacity = Background opacity

panel-applets = Konfiguration
    .dock-desc = Konfigurer dock applets
    .desc = Konfigurer panel applets

panel-missing = Panelkonfiguration mangler
    .desc = Panelkonfigurationsfilen mangler på grund af brug af en brugerdefineret konfiguration, eller den er beskadiget.
    .fix = Nulstil til standard

## Desktop: Dock

dock = Dock
    .desc = Panel med fastgjorte applikationer i app bakken og andre applets

## Desktop: Vinduesstyring

window-management = Vinduesstyring
    .desc = Super key action, window control options, and additional window tiling options.

super-key = Super tast handling
    .launcher = Åben Launcher
    .workspaces = Åben Workspaces
    .applications = Åben Applications
    .disable = Deaktiver

window-controls = Vinduesstyring
    .maximize = Vis maksimer knap
    .minimize = Vis minimer knap
    .active-window-hint = Vis aktive vinduestip

focus-navigation = Fokus navigation
    .focus-follows-cursor = Fokus følger markøren
    .cursor-follows-focus = Markøren følger fokus
    .cursor-follows-focus = Markøren følger fokus

## Desktop: Workspaces

workspaces = Workspaces
    .desc = Workspace orientation and behavior.

workspaces-behavior = Workspace Behavior
    .dynamic = Dynamic workspaces
    .dynamic-desc = Automatically removes empty workspaces.
    .fixed = Fixed Number of Workspaces
    .fixed-desc = Add or remove workspaces in the overview.

workspaces-multi-behavior = Multi-monitor Behavior
    .span = Workspaces Span Displays
    .separate = Displays Have Separate Workspaces

workspaces-overview-thumbnails = Workspace Overview Thumbnails
    .show-number = Show Workspace Number
    .show-name = Show Workspace Name

workspaces-orientation = Workspaces Orientation
    .vertical = Vertical
    .horizontal = Horizontal

hot-corner = Hot Corner
    .top-left-corner = Enable top-left hot corner for Workspaces

## Displays

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

mirroring = Spejling
    .id = Spejling { $id }
    .dont = Spejl ikke
    .mirror = Spejl { $display }
    .project = Projekter til { $display ->
        [all] alle skærme
        *[other] { $display }
    }
    .project-count = Projecting to { $count} other { $count ->
        [1] display
        *[other] displays
    }

night-light = Natlys
    .auto = Automatisk (solnedgang til solopgang)
    .desc = Reducer blåt lys med varmere farver.

orientation = Orientering
    .standard = Standard
    .rotate-90 = Roter 90
    .rotate-180 = Roter 180
    .rotate-270 = Roter 270

scheduling = Planlægning
    .manual = Manuel planlægning

dialog = Dialog
    .title = Behold disse skærmindstillinger?
    .keep-changes = Behold ændringer
    .change-prompt = Ændringer af indstillinger vil automatisk vende tilbage om { $time } sekunder.
    .revert-settings = Gendan indstillinger

legacy-applications = X11 Vinduessystem Applikationsskalering
    .scaled-by-system = Skalér alle X11 applikationer
    .system-description = X11 applikationer vil fremstå slørede på HiDPI-skærme.
    .scaled-natively = Render X11 applikationer i naturlig opløsning
    .native-description = X11 applikationer, der ikke understøtter skalering, vil være små, når HiDPI-skærme er i brug. Aktiver for spil at bruge den fulde skærmopløsning.

## Sound

sound = Sound
    .desc = N/A

sound-output = Udgang
    .volume = Udgangs Lydstyrke
    .device = Udgangsenhed
    .level = Udgangsniveau
    .config = Konfiguration
    .balance = Balance

sound-input = Input
    .volume = Input Lydstyrke
    .device = Input Enhed
    .level = Input Niveau

sound-alerts = Advarsler
    .volume = Lydstyrke for advarsler
    .sound = Alarmer lyde

sound-applications = Applikationer
    .desc = Applikation Lydstyrke og indstillinger

profile = Profil

## Power

power = Strøm & Batteri
    .desc = Administrer strømindstillinger

battery = Batteri
  .minute = { $value } { $value ->
        [one] minut
       *[other] minutter
  }
  .hour = { $value } { $value ->
        [one] time
       *[other] timer
  }
  .day = { $value } { $value ->
        [one] dag
       *[other] dage
  }
  .less-than-minute = Mindre end et minut
  .and = and
  .remaining-time = { $time } until { $action ->
        [full] full
       *[other] empty
   }

connected-devices = Tilsluttede enheder
  .unknown = Ukendt enhed

power-mode = Power Mode
    .battery = Forlænget batterilevetid
    .battery-desc = Reduceret strømforbrug og lydløs ydeevne.
    .balanced = Balanceret
    .balanced-desc = Støjsvag ydeevne og moderat strømforbrug.
    .performance = Høj ydeevne
    .performance-desc = Maksimal ydeevne og strømforbrug.
    .no-backend = Backend not found. Install system76-power or power-profiles-daemon.

power-saving = Power Saving Options
    .turn-off-screen-after = Turn off the screen after
    .auto-suspend = Automatic suspend
    .auto-suspend-ac = Automatic suspend when plugged in
    .auto-suspend-battery = Automatic suspend on battery power

## Input

acceleration-desc = Justerer automatisk sporingsfølsomhed baseret på hastighed.

disable-while-typing = Disable while typing

input-devices = Input Devices
    .desc = Input Devices

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
    .desc = Input sources, switching, special character entry, shortcuts.

keyboard-sources = Inputkilder
    .desc = Inputkilder kan skiftes ved hjælp af Super+Mellemrumstastekombinationen. Dette kan tilpasses i indstillingerne for tastaturgenveje.
    .move-up = Flyt op
    .move-down = Flyt ned
    .settings = Indstillinger
    .view-layout = Vis tastaturlayout
    .remove = Fjern
    .add = Tilføj inputkilde

keyboard-special-char = Special Character Entry
    .alternate = Alternate characters key
    .compose = Compose key
    .caps = Caps Lock key

keyboard-typing-assist = Typing
    .repeat-rate = Repeat rate
    .repeat-delay = Repeat delay
    
added = Added
type-to-search = Type to search...
show-extended-input-sources = Show extended input sources

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Tastaturgenveje
    .desc = Se og tilpas genveje

add-keybinding = Add keybinding
cancel = Annuller
command = Kommando
custom = Brugerdefineret
debug = Debug
disabled = Deaktiveret
migrate-workspace-prev = Migrer arbejdsområdet til tidligere output
migrate-workspace-next = Migrer arbejdsområdet til næste output
migrate-workspace = Migrer arbejdsområdet til output { $direction ->
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
    .prev-output = Focus previous output
    .next-output = Focus next output
    .last-workspace = Focus last workspace
    .prev-workspace = Focus previous workspace
    .next-workspace = Focus next workspace
    .focus = Focus window { $direction ->
        *[down] down
        [in] in
        [left] left
        [out] out
        [right] right
        [up] up
    }
    .output = Switch to output { $direction ->
        *[down] down
        [left] left
        [right] right
        [up] up
    }
    .workspace = Switch to workspace { $num }

manage-windows = Manage windows
    .close = Close window
    .maximize = Maximize window
    .minimize = Minimize window
    .resize-inwards = Resize window inwards
    .resize-outwards = Resize window outwards
    .toggle-sticky = Toggle sticky window

move-windows = Move Windows
    .direction = Move window { $direction ->
        *[down] down
        [left] left
        [right] right
        [up] up
    }
    .display = Move window one monitor { $direction ->
        *[down] down
        [left] left
        [right] right
        [up] up
    }
    .workspace = Move window one workspace { $direction ->
        *[below] below
        [left] left
        [right] right
        [above] above
    }
    .workspace-num = Move window to workspace { $num }
    .prev-workspace = Move window to prev workspace
    .next-workspace = Move window to next workspace
    .last-workspace = Move window to last workspace
    .next-display = Move window to next display
    .prev-display = Move window to prev display
    .send-to-prev-workspace = Move window to previous workspace
    .send-to-next-workspace = Move window to next workspace

system-shortcut = System
    .app-library = Åbn appbiblioteket
    .brightness-down = Reducer skærmens lysstyrke
    .brightness-up = Øg skærmens lysstyrke
    .home-folder = Åbn hjemmemappe
    .keyboard-brightness-down = Reducer tastaturets lysstyrke
    .keyboard-brightness-up = Øg tastaturets lysstyrke
    .launcher = Åbn launcher
    .lock-screen = Lås skærmen
    .mute = Slå lydudgang fra
    .mute-mic = Slår mikrofonindgangen fra
    .play-pause = Afspil/Pause
    .play-next = Næste nummer
    .play-prev = Forrige nummer
    .screenshot = Tag et skærmbillede
    .terminal = Åbn en terminal
    .volume-lower = Sænk lydudgangs lydstyrken
    .volume-raise = Øg lydudgangs lydstyrken
    .web-browser = Åbner en webbrowser
    .window-switcher = Skift mellem åbne vinduer
    .workspace-overview = Åbn oversigten over arbejdsområdet

window-tiling = Window tiling
    .horizontal = Indstil vandret orientering
    .vertical = Indstil lodret orientering
    .swap-window = Swap window
    .toggle-tiling = Toggle window tiling
    .toggle-stacking = Toggle window stacking
    .toggle-floating = Toggle window floating
    .toggle-orientation = Toggle orientation

replace-shortcut-dialog = Erstat Genvej?
    .desc = { $shortcut } bruges af { $name }. Hvis du erstatter det, { $name } vil blive deaktiveret.

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

time = Time & Language
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

## System

system = System & Accounts

## System: About

about = About
    .desc = Device name, hardware information, operating system defaults.

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
    .windowing-system = Windowing system

about-related = Relaterede indstillinger
    .support = Få support

## System: Firmware

firmware = Firmware
    .desc = Firmware detaljer.

## System: Brugere

users = Brugere
    .desc = Autentificering og brugerkonti.