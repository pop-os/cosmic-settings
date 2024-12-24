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
network-and-wireless = Netwerk en draadloze verbindingen
        <#-- May also be "Netwerk en Wifi" -->
no-networks = Er zijn geen netwerken gevonden.
no-vpn = Geen VPN-verbindingen beschikbaar.
password = Wachtwoord
remove = Verwijderen
settings = Instellingen
username = Gebruikersnaam
visible-networks = Zichtbare netwerken

auth-dialog = Authenticatie Vereist
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
recent-colors = Recente Kleuren
reset-to-default = Naar standaardinstellingen terugzetten
rgb = RGB
window-hint-accent = Actief venster accentkleur
window-hint-accent-toggle = Gebruik de accentkleur van het thema als actief venster accentkleur

auto-switch = Automatisch wisselen tussen lichte en donkere modus
    .sunrise = Wissel naar lichte modus bij zonsopgang
    .sunset = Wissel naar donkere modus bij zonsondergang
    .next-sunrise = Wissel naar lichte modus bij de volgende zonsopgang
    .next-sunset = Wissel naar donkere modus bij de volgende zonsondergang

<#-- CAME TO HERE, CONTINEU TRANSLATION HERE! -->

container-background = Container background
    .desc-detail = Container background color is used for navigation sidebar, side drawer, dialogs and similar widgets. By default, it is automatically derived from the Application or window background.
    .reset = Reset to auto
    .desc = Primary container color is used for navigation sidebar, side drawer, dialogs and similar widgets.

control-tint = Control component tint
    .desc = Used for backgrounds of standard buttons, search inputs, text inputs, and similar components.

frosted = Frosted glass effect on system interface
    .desc = Applies background blur to panel, dock, applets, launcher, and application library.

enable-export = Apply this theme to GNOME apps.
    .desc = Not all toolkits support auto-switching. Non-COSMIC apps may need to be restarted after a theme change.

icon-theme = Icon Theme
    .desc = Applies a different set of icons to applications.

text-tint = Interface text tint
    .desc = Color used to derive interface text colors that have sufficient contrast on various surfaces.

style = Style
    .round = Round
    .slightly-round = Slightly round
    .square = Square

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

## Desktop: Notifications

notifications = Notifications
    .desc = Do Not Disturb, lockscreen notifications, and per-application settings.

## Desktop: Panel

panel = Panel
    .desc = Top bar with desktop controls and menus.

add = Add
add-applet = Add Applet
all = All
applets = Applets
center-segment = Center Segment
drop-here = Drop applets here
end-segment = End Segment
large = Large
no-applets-found = No applets found...
panel-bottom = Bottom
panel-left = Left
panel-right = Right
panel-top = Top
search-applets = Search applets...
small = Small
start-segment = Start Segment

panel-appearance = Appearance
    .match = Match desktop
    .light = Light
    .dark = Dark

panel-behavior-and-position = Behavior and Positions
    .autohide = Automatically hide panel
    .dock-autohide = Automatically hide dock
    .position = Position on screen
    .display = Show on display

panel-style = Style
    .anchor-gap = Gap between panel and screen edges
    .dock-anchor-gap = Gap between dock and screen edges
    .extend = Extend panel to screen edges
    .dock-extend = Extend dock to screen edges
    .appearance = Appearance
    .size = Size
    .background-opacity = Background opacity

panel-applets = Configuration
    .dock-desc = Configure dock applets
    .desc = Configure panel applets

panel-missing = Panel Configuration is Missing
    .desc = The panel configuration file is missing due to use of a custom configuration or it is corrupted.
    .fix = Reset to default

## Desktop: Dock

dock = Dock
    .desc = Panel with pinned applications in the app tray and other applets.

## Desktop: Window management

window-management = Window management
    .desc = Super key action, window control options, and additional window tiling options.

super-key = Super key action
    .launcher = Open Launcher
    .workspaces = Open Workspaces
    .applications = Open Applications
    .disable = Disable

window-controls = Window Controls
    .maximize = Show maximize button
    .minimize = Show minimize button
    .active-window-hint = Show active window hint

focus-navigation = Focus Navigation
    .focus-follows-cursor = Focus follows cursor
    .focus-follows-cursor-delay = Focus follows cursor delay in ms
    .cursor-follows-focus = Cursor follows focus

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

-requires-restart = Requires restart

color = Color
    .depth = Color depth
    .profile = Color profile
    .sidebar = Color Profiles
    .temperature = Color temperature

display = Displays
    .desc = Manage displays, graphics switching, and night light
    .arrangement = Display Arrangement
    .arrangement-desc = Drag displays to rearrange them.
    .enable = Enable display
    .external = { $size } { $output } External Display
    .laptop = { $size } Laptop Display
    .options = Display Options
    .refresh-rate = Refresh rate
    .resolution = Resolution
    .scale = Scale

mirroring = Mirroring
    .id = Mirroring { $id }
    .dont = Don't mirror
    .mirror = Mirror { $display }
    .project = Project to { $display ->
        [all] all displays
        *[other] { $display }
    }
    .project-count = Projecting to { $count} other { $count ->
        [1] display
        *[other] displays
    }

night-light = Night Light
    .auto = Automatic (sunset to sunrise)
    .desc = Reduce blue light with warmer colors.

orientation = Orientation
    .standard = Standard
    .rotate-90 = Rotate 90
    .rotate-180 = Rotate 180
    .rotate-270 = Rotate 270

vrr = Variable refresh rate
    .enabled = Enabled
    .force = Always
    .auto = Automatic
    .disabled = Disabled

scheduling = Scheduling
    .manual = Manual schedule

dialog = Dialog
    .title = Keep These Display Settings?
    .keep-changes = Keep Changes
    .change-prompt = Settings changes will automatically revert in { $time } seconds.
    .revert-settings = Revert Settings

legacy-applications = X11 Window System Application Scaling
    .scaled-by-system = Scale all X11 Applications
    .system-description = X11 applications will appear blurry on HiDPI screens.
    .scaled-natively = Render X11 Applications at native resolution
    .native-description = X11 applications that don't support scaling will be small when HiDPI displays are in use. Enable for games to utilize the full monitor resolution.

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
