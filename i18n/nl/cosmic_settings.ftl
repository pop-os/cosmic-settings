app = COSMIC-instellingen

dbus-connection-error = Kon geen verbinding maken met DBus
ok = Oké
unknown = Onbekend

number = { $number }

## Network & Wireless

connections-and-profiles = { $variant ->
    [wired] Bekabelde verbindingen
    [wifi] Wifi
    [vpn] VPN
    *[other] Onbekende verbindingen
} en verbindingsprofielen.

add-network = Netwerk toevoegen
    .profile = Profiel toevoegen
add-vpn = VPN toevoegen
airplane-on = Vliegtuigmodus is ingeschakeld.
cable-unplugged = Kabel losgekoppeld
connect = Verbinden
connected = Verbonden
connecting = Verbinding maken…
disconnect = Verbinden verbreken
forget = Vergeten
known-networks = Bekende netwerken
network-and-wireless = Netwerk en wifi
no-networks = Er zijn geen netwerken gevonden.
no-vpn = Geen VPN-verbindingen beschikbaar.
password = Wachtwoord
remove = Verwijderen
settings = Instellingen
username = Gebruikersnaam
visible-networks = Zichtbare netwerken

auth-dialog = Authenticatie vereist
    .vpn-description = Voer de gebruikersnaam en het wachtwoord van de VPN-dienst in.
    .wifi-description = Voer het wachtwoord of de coderingssleutel in. U kunt ook verbinding maken door op de “WPS”-knop op de router te drukken.

forget-dialog = Wifi-netwerk vergeten?
    .description = U moet opnieuw een wachtwoord invoeren om dit wifi-netwerk in de toekomst te gebruiken.

network-device-state =
    .activated = Verbonden
    .config = Verbinding maken…
    .deactivating = Verbinding verbreken…
    .disconnected = Verbinding is verbroken
    .failed = Verbinding mislukt
    .ip-check = Verbinding wordt gecontroleerd…
    .ip-config =  IP- en routeringsinformatie wordt opgevraagd…
    .need-auth = Vereist authenticatie
    .prepare = Verbinding voorbereiden…
    .secondaries = Wachten op secundaire verbinding…
    .unavailable = Niet beschikbaar
    .unknown = Status onbekend
    .unmanaged = Onbeheerd
    .unplugged = Kabel losgekoppeld

remove-connection-dialog = Verbindingsprofiel verwijderen?
    .vpn-description =  Om dit netwerk in de toekomst te kunnen gebruiken, moet u opnieuw een wachtwoord invoeren.
    .wired-description = U moet dit profiel opnieuw aanmaken om het in de toekomst te kunnen gebruiken.

vpn = VPN
    .connections = VPN-verbindingen
    .error = Kon geen VPN-configuratie aanmaken
    .remove = Verbindingsprofiel verwijderen
    .select-file = Selecteer VPN-configuratiebestand

vpn-error = VPN-fout
    .config = Kon geen VPN-configuratie aanmaken
    .connect = VPN-verbinding mislukt
    .connection-editor =  Verbindingseditor mislukt
    .connection-settings = Het ophalen van instellingen voor actieve verbindingen is mistlukt
    .updating-state = Kan status van netwerkbeheerder niet bijwerken
    .wireguard-config-path = Ongeldig bestandspad voor de WireGuard-configuratie
    .wireguard-config-path-desc = Het gekozen bestand moet op een lokaal bestandssysteem staan.
    .wireguard-device = Kon WireGuard-apparaat niet aanmaken
    .with-password = Kon { $field ->
        *[username] de VPN-gebruikersnaam
        [password] het VPN-wachtwoord
        [password-flags] de VPN-wachtwoordvlaggen
    } niet met nmcli toevoegen

wired = Bedraad
    .adapter = Bedrade adapter { $id }
    .connections = Bedrade verbindingen
    .devices = Bedrade apparaten
    .remove = Verbindingsprofielen verwijderen

wifi = Wifi
    .adapter = Wifi-adapter { $id }
    .forget = Dit netwerk vergeten

wireguard-dialog = WireGuard-apparaat toevoegen
    .description = Kies een apparaatnaam voor de WireGuard-configuratie.

## Networking: Online Accounts

online-accounts = Online accounts
    .desc = Accounts toevoegen, IMAP en SMTP, bedrijfslogins

# Bluetooth

confirm = Bevestigen

bluetooth = Bluetooth
    .desc = Bluetooth-apparaten bewerken
    .status = Dit systeem is zichtbaar als { $aliases } zolang de Bluetooth-instellingen open staan.
    .connected = Verbonden
    .connecting = Verbinding maken…
    .disconnecting = Verbinding verbreken…
    .connect = Verbinden
    .disconnect = Verbinding verbreken
    .forget = Vergeten
    .dbus-error = Er is een fout opgetreden tijdens de interactie met DBus: { $why }

bluetooth-paired = Eerder verbonden apparaten
    .connect = Verbinden
    .battery = { $percentage }% batterij

bluetooth-confirm-pin = Bluetooth-pincode bevestigen
    .description = Controleer of de volgende pincode overeenkomt met de pincode die op { $device } wordt weergegeven

bluetooth-available = Nabije bluetooth-apparaten

bluetooth-adapters = Bluetooth-adapters

## Desktop

desktop = Bureaublad

## Desktop: Wallpaper

wallpaper = Schermachtergrond
    .change = Wijzig alle afbeeldingen
        <#-- Wijzig alle afbeeldingen: Change every image -->
    .desc = Opties voor schermachtergrond, kleuren en diavoorstellingen
    .fit = Schermachtergrond aanpassen
    .folder-dialog = Selecteer map met achtergronden
    .image-dialog = Selecteer schermachtergrond
    .plural = Achtergronden
    .same = Dezelfde achtergrond op alle schermen
    .slide = Diavoorstelling

add-color = Kleur toevoegen
add-image = Afbeelding toevoegen
all-displays = Alle beeldschermen
colors = Kleuren
dialog-add = Toevoegen
fill = Vullen
fit-to-screen = Aan het scherm passend maken
open-new-folder = Nieuwe map openen
recent-folders = Recente mappen

x-minutes = { $number } minuten
x-hours = { $number } uur

never = Nooit

## Desktop: Appearance

appearance = Uiterlijk
    .desc = Kleuren en thema's.

accent-color = Accentkleur
app-background = Achtergronden van applicaties of vensters
auto = Automatisch
close = Sluiten
color-picker = Kleurkiezer
copied-to-clipboard = Naar klembord gekopieerd
copy-to-clipboard = Naar klembord kopiëren
dark = Donker
export = Exporteren
hex = Hex
import = Importeren
light = Licht
mode-and-colors = Modus en Kleuren
<#-- Mode and color: "Modus" doesn't refer to anything specific in Dutch: people might not assosiate it with dark/light theme -->
recent-colors = Recente kleuren
reset-to-default = Naar standaardinstellingen terugzetten
rgb = RGB
window-hint-accent = Accentkleur als visuele hint voor het actieve venster
window-hint-accent-toggle = Gebruik de accentkleur van het thema als visuele hint voor het actieve venster

auto-switch = Automatisch wisselen tussen lichte en donkere modus
    .sunrise = Wissel naar lichte modus bij zonsopgang
    .sunset = Wissel naar donkere modus bij zonsondergang
    .next-sunrise = Wissel naar lichte modus bij de volgende zonsopgang
    .next-sunset = Wissel naar donkere modus bij de volgende zonsondergang

<#-- not sure about these translations, don't know what is meant with Container color -->
container-background = Containerachergrondskleur 
    .desc-detail = De containerachtergrondskleur wordt gebruikt voor de navigatiebalk, het zijpaneel, dialoogvensters en soortgelijke widgets. Standaard wordt deze automatisch afgeleid van de achtergrond van de toepassing of het venster.
    .reset = Naar automatisch terugzetten
    .desc = De primaire containerkleur wordt gebruikt voor de navigatiezijbalk, zijlade, dialoogvensters en soortgelijke widgets.

control-tint = Tinting van controlecomponenten
    .desc = Wordt gebruikt voor achtergronden van standaardknoppen, zoekingangen, tekstingangen en soortgelijke onderdelen.
frosted =  Matglaseffect op de systeeminterface
    .desc = Past achtergrondvervaging toe op het paneel, de dock, applets, het zoekmenu en het startmenu

enable-export = Pas dit thema toe op GNOME-apps.
    .desc = Niet alle toolkit-omgevingen ondersteunen automatische wisseling. Niet-COSMIC-apps moeten mogelijk opnieuw worden opgestart na een themawijziging.

icon-theme = Icoonthema
    .desc = Past een andere set pictogrammen toe op applicaties.

text-tint = Interface tekstkleur
    .desc = Kleur die wordt gebruikt om interfacetekstkleuren te bepalen, zodat er voldoende contrast is op verschillende oppervlakken.

style = Stijl
    .round = Rond
    .slightly-round = Licht afgerond
    .square = Rechthoekig

interface-density = Interface-dichtheid
    .comfortable = Comfortabel
    .compact = Compact
    .spacious = Ruim

window-management-appearance = Vensterbeheer
    .active-hint = Grootte visuele hint voor het actieve venster
    .gaps = Ruimte rondom getegelde vensters

### Experimental

experimental-settings = Experimentele instellingen
icons-and-toolkit = Thema's voor pictogrammen en de toolkit
interface-font = Standaardlettertype
monospace-font = Lettertype met gelijke letterbreedte

## Desktop: Notifications

notifications = Meldingen
    .desc = Niet storen, meldingen op het vergrendelscherm en instellingen per applicatie.


## Desktop: Panel

panel = Paneel
    .desc = Bovenste balk met bureaubladbesturingselementen en menu's

add = Toevoegen
add-applet = Applet toevoegen
all = Alle
applets = Applets
center-segment = Middenstuk
drop-here = Applets hier plaatsen
end-segment = Eindstuk
large = Groot
no-applets-found = Geen applets gevonden…
panel-bottom = Onder
panel-left = Links
panel-right = Rechts
panel-top = Boven
search-applets = Applets zoeken…
small = Klein
start-segment = Beginstuk

panel-appearance = Uiterlijk
    .match = Aanpassen aan bureaublad
    .light = Licht
    .dark = Donker

panel-behavior-and-position = Paneel: werking en positie
    .autohide = Paneel automatisch verbergen
    .dock-autohide = Dock automatisch verbergen
    .position = Positie op het scherm
    .display = Weergeven op het scherm

panel-style = Stijl
    .anchor-gap = Ruimte tussen het paneel en de schermranden
    .dock-anchor-gap = Ruimte tussen de dock en de schermranden
    .extend = Paneel tot de schermranden uitbreiden
    .dock-extend = Dock tot de schermranden uitbreiden
    .appearance = Uiterlijk
    .size = Grootte
    .background-opacity = Doorzichtigheid van de achtergrond

panel-applets = Configuratie
    .dock-desc = Dock-applets configureren
    .desc = Paneel-applets configureren

panel-missing = Paneelconfiguratie ontbreekt
    .desc = Het paneelconfiguratiebestand ontbreekt door het gebruik van een aangepaste configuratie of door corruptie.
    .fix = Naar standaardinstellingen terugzetten

## Desktop: Dock

dock = Dock
    .desc = Paneel met vastgezetten applicaties vanuit het startmenu en andere applets.

## Desktop: Window management

window-management = Vensterbeheer
    .desc = Actie van de supertoets, vensterbeheeropties en aanvullende opties voor het tegelen van vensters.

super-key = Actie van de supertoets
    .launcher = Snelstarter openen
    .workspaces = Werkbladoverzicht openen
    .applications = Startmenu openen
    .disable = Supertoets uitschakelen

window-controls = Vensterbeheer
    .maximize = Knop 'maximaliseren' tonen
    .minimize = Knop 'minimaliseren' tonen
    .active-window-hint = Gebruik visuele hint voor het actieve venster

focus-navigation = Focusbesturing
    .focus-follows-cursor = Focus volgt de cursor
    .focus-follows-cursor-delay = Vertraging voor focus volgt de cursor in ms
    .cursor-follows-focus = De cursor volgt de focus

## Desktop: Workspaces

workspaces = Virtuele werkbladen
    .desc = Aantal, werking en positie van virtuele werkbladen instellen

workspaces-behavior = Werking virtuele werkbladen
    .dynamic = Dynamische virtuele werkbladen
    .dynamic-desc = Lege werkbladen automatisch verwijderen.
    .fixed = Vast aantal virtuele werkbladen
    .fixed-desc = Werkbladen aan het overzicht toevoegen of verwijderen.

workspaces-multi-behavior = Werking over meerdere beeldschermen
    .span = Virtuele werkbladen strekken zich uit over meerder beeldschermen
    .separate = Beeldschermen hebben afzonderlijke werkbladen

workspaces-overview-thumbnails = Miniatuurweergaven van het werkbladoverzicht
    .show-number = Toon het nummer van de werkbladen
    .show-name = Toon de naam van de werkbladen

workspaces-orientation = Oriëntatie van de virtuele werkbladen
    .vertical = Verticaal
    .horizontal = Horizontaal

hot-corner = Slimme hoek
    .top-left-corner = Activeer slimme hoek linksboven voor het werkbladenoverzicht

## Displays

-requires-restart = Vereist een herstart

color = Kleur
    .depth = Kleurdiepte
    .profile = Kleurprofiel
    .sidebar = Kleurprofielen
    .temperature = Kleurtemperatuur

display = Beeldschermen
    .desc = Beeldschermbeheer, omschakeling tussen grafisch kaarten en nachtlichtbeheer
    .arrangement = Schermindeling
    .arrangement-desc = Sleep de schermen om ze te herschikken.
    .enable = Beeldscherm activeren
    .external = { $size } { $output } Extern beeldscherm
    .laptop = { $size } Laptop-beeldscherm
    .options = Beeldschermopties
    .refresh-rate = Refresh rate
    .resolution = Schermresolutie
    .scale = Schaal

mirroring = Scherm dubliceren
    .id = Dubliceren { $id }
    .dont = Don't mirror
    .mirror = { $display } dubliceren
    .project = Naar { $display ->
        [all] alle schermen
        *[other] { $display }
    } projecteren
    .project-count = Naar { $count} { $count ->
        [1] ander scherm
        *[other] andere schermen
    } projecteren

night-light = Nachtlichtbeheer
    .auto = Automatisch (van zonsondergang tot zonsopgang)
    .desc = Blauw licht verminderen met warmere lichtkleuren.

orientation = Oriëntatie
    .standard = Standaard
    .rotate-90 = 90° draaien
    .rotate-180 = 180° draaien
    .rotate-270 = 270° draaien

vrr = Variable refresh rate
    .enabled = Ingeschakeld
    .force = Geforceerd ingeschakeld houden
    .auto = Automatisch
    .disabled = Uitgeschakeld

scheduling = Tijdsplanning
    .manual = Handmatig plannen

dialog = Dialoog
    .title = Deze beeldscherminstellingen behouden?
    .keep-changes = Wijzigingen behouden
    .change-prompt = Instellingen worden automatisch hersteld in { $time } seconden.
    .revert-settings = Instellingen herstellen

legacy-applications = Schaling van applicaties die het X11-venstersysteem gebruiken
    .scaled-by-system = Schaal alle X11-applicaties
    .system-description = X11-applicaties worden onscherp weergegeven op HiDPI-schermen.
    .scaled-natively = X11-applicaties in native resolutie weergeven
    .native-description = X11-applicaties die geen schaling ondersteunen worden verkleint op HiDPI-schermen weergegeven. Zet deze optie aan zodat games de volledige schermresolutie kunnen gebruiken.


## Sound

sound = Sound
    .desc = N/A

sound-output = Output
    .volume = Output volume
    .device = Output device
    .level = Output level
    .config = Configuration
    .balance = Balance

sound-input = Input
    .volume = Input volume
    .device = Input device
    .level = Input level

sound-alerts = Alerts
    .volume = Alerts volume
    .sound = Alerts sound

sound-applications = Applications
    .desc = Application volumes and settings

profile = Profile

## Power

power = Power & Battery
    .desc = Manage power settings

battery = Battery
  .minute = { $value } { $value ->
        [one] minute
       *[other] minutes
  }
  .hour = { $value } { $value ->
        [one] hour
       *[other] hours
  }
  .day = { $value } { $value ->
        [one] day
       *[other] days
  }
  .less-than-minute = Less than a minute
  .and = and
  .remaining-time = { $time } until { $action ->
        [full] full
       *[other] empty
   }

connected-devices = Connected Devices
  .unknown = Unknown device

power-mode = Power Mode
    .battery = Extended battery life
    .battery-desc = Reduced power usage and silent performance.
    .balanced = Balanced
    .balanced-desc = Quiet performance and moderate power usage.
    .performance = High performance
    .performance-desc = Peak performance and power usage.
    .no-backend = Backend not found. Install system76-power or power-profiles-daemon.

power-saving = Power Saving Options
    .turn-off-screen-after = Turn off the screen after
    .auto-suspend = Automatic suspend
    .auto-suspend-ac = Automatic suspend when plugged in
    .auto-suspend-battery = Automatic suspend on battery power

## Input

acceleration-desc = Automatically adjusts tracking sensitivity based on speed.

disable-while-typing = Disable while typing

input-devices = Input Devices
    .desc = Input Devices

primary-button = Primary button
    .desc = Sets the order of physical buttons.
    .left = Left
    .right = Right

scrolling = Scrolling
    .two-finger = Scroll with two fingers
    .edge = Scroll along the edge with one finger
    .speed = Scrolling speed
    .natural = Natural scrolling
    .natural-desc = Scroll the content, instead of the view

## Input: Keyboard

slow = Slow
fast = Fast
short = Short
long = Long
keyboard = Keyboard
    .desc = Input sources, switching, special character entry, shortcuts.

keyboard-sources = Input Sources
    .desc = Input sources can be switched using Super+Space key combination. This can be customized in the keyboard shortcut settings.
    .move-up = Move up
    .move-down = Move down
    .settings = Settings
    .view-layout = View keyboard layout
    .remove = Remove
    .add = Add input source

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

keyboard-shortcuts = Keyboard Shortcuts
    .desc = View and customize shortcuts

add-keybinding = Add keybinding
cancel = Cancel
command = Command
custom = Custom
debug = Debug
disabled = Disabled
migrate-workspace-prev = Migrate workspace to previous output
migrate-workspace-next = Migrate workspace to next output
migrate-workspace = Migrate workspace to output { $direction ->
    *[down] down
    [left] left
    [right] right
    [up] up
}
navigate = Navigate
replace = Replace
shortcut-name = Shortcut name
system-controls = System controls
terminate = Terminate
toggle-stacking = Toggle window stacking
type-key-combination = Type key combination

custom-shortcuts = Custom Shortcuts
    .add = Add shortcut
    .context = Add Custom Shortcut
    .none = No custom shortcuts

modified = { $count } modified

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
    .app-library = Open the app library
    .brightness-down = Decrease display brightness
    .brightness-up = Increase display brightness
    .home-folder = Open home folder
    .keyboard-brightness-down = Decrease keyboard brightness
    .keyboard-brightness-up = Increase keyboard brightness
    .launcher = Open the launcher
    .lock-screen = Lock the screen
    .mute = Mute audio output
    .mute-mic = Mutes microphone input
    .play-pause = Play/Pause
    .play-next = Next track
    .play-prev = Previous track
    .screenshot = Take a screenshot
    .terminal = Open a terminal
    .volume-lower = Decrease audio output volume
    .volume-raise = Increase audio output volume
    .web-browser = Opens a web browser
    .window-switcher = Switch between open windows
    .window-switcher-previous = Switch between open windows reversed
    .workspace-overview = Open the workspace overview

window-tiling = Window tiling
    .horizontal = Set horizontal orientation
    .vertical = Set vertical orientation
    .swap-window = Swap window
    .toggle-tiling = Toggle window tiling
    .toggle-stacking = Toggle window stacking
    .toggle-floating = Toggle window floating
    .toggle-orientation = Toggle orientation

replace-shortcut-dialog = Replace Shortcut?
    .desc = { $shortcut } is used by { $name }. If you replace it, { $name } will be disabled.

## Input: Mouse

mouse = Mouse
    .desc = Mouse speed, acceleration, natural scrolling.
    .speed = Mouse speed
    .acceleration = Enable mouse acceleration

## Input: Touchpad

click-behavior = Click Behavior
    .click-finger = Secondary click with two fingers and middle-click with three fingers
    .button-areas = Secondary click in bottom right corner and middle-click in bottom center

pinch-to-zoom = Pinch to zoom
    .desc = Use two fingers to zoom into content, for applications that support zoom.

tap-to-click = Tap to click
    .desc = Enables single-finger tap for primary click, two-finger tap for secondary click, and three-finger tap for middle click.

touchpad = Touchpad
    .acceleration = Enable touchpad acceleration
    .desc = Touchpad speed, click options, gestures.
    .speed = Touchpad speed

## Input: Gestures

gestures = Gestures
    .four-finger-down = Four-finger swipe down
    .four-finger-left = Four-finger swipe left
    .four-finger-right = Four-finger swipe right
    .four-finger-up = Four-finger swipe up
    .three-finger-any = Three-finger swipe any direction

switch-workspaces = Switch workspaces
    .horizontal = Four-finger swipe left/right
    .vertical = Four-finger swipe up/down

switch-between-windows = Switch between windows
open-application-library = Open Application Library
open-workspaces-view = Open Workspaces Overview

## Time & Language

time = Time & Language
    .desc = N/A

time-date = Date & Time
    .desc = Time zone, automatic clock settings, and some time formatting.
    .auto = Set automatically
    .auto-ntp = Date & time will update automatically when the time zone is set.

time-zone = Time Zone
    .auto = Automatic time zone
    .auto-info = Requires location services and internet access

time-format = Date & Time Format
    .twenty-four = 24-hour time
    .show-seconds = Show seconds
    .first = First day of week
    .show-date = Show Date on Top Panel
    .friday = Friday
    .saturday = Saturday
    .sunday = Sunday
    .monday = Monday

time-region = Region & Language
    .desc = Format dates, times, and numbers based on your region

formatting = Formatting
    .dates = Dates
    .time = Time
    .date-and-time = Date & Time
    .numbers = Numbers
    .measurement = Measurement
    .paper = Paper

preferred-languages = Preferred Languages
    .desc = The order of languages determines which language is used for the translation of the desktop. Changes take effect on next login.

add-language = Add language
    .context = Add Language
install-additional-languages = Install additional languages
region = Region

## System

system = System & Accounts

## System: About

about = About
    .desc = Device name, hardware information, operating system defaults.

about-device = Device name
    .desc = This name appears to other network or bluetooth devices.

about-hardware = Hardware
    .model = Hardware model
    .memory = Memory
    .processor = Processor
    .graphics = Graphics
    .disk-capacity = Disk Capacity

about-os = Operating System
    .os = Operating system
    .os-architecture = Operating system architecture
    .desktop-environment = Desktop environment
    .windowing-system = Windowing system

about-related = Related settings
    .support = Get support

## System: Firmware

firmware = Firmware
    .desc = Firmware details.

## System: Users

users = Users
    .desc = Authentication and user accounts.
    .admin = Admin
    .standard = Standard
    .profile-add = Choose profile image

administrator = Administrator
    .desc = Administrators can change settings for all users, add and remove other users.

add-user = Add user
remove-user = Remove user
full-name = Full name
username = Username
password = Password

## System: Default Applications

default-apps = Default Applications
    .desc = Default web browser, mail client, file browser, and other applications.
    .web-browser = Web browser
    .file-manager = File manager
    .mail-client = Mail client
    .music = Music
    .video = Video
    .photos = Photos
    .calendar = Calendar
    .terminal = Terminal
    .other-associations = Other Associations
