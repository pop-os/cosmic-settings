app = COSMIC Settings

dbus-connection-error = Failed to connect to DBus
ok = OK
unknown = Unknown

number = { $number }

## Network & Wireless

connections-and-profiles = { $variant ->
    [wired] Wired
    [wifi] Wi-Fi
    [vpn] VPN
    *[other] Unknown
} connections and connection profiles.

add-network = Add network
    .profile = Add profile
add-vpn = Add VPN
airplane-on = Airplane mode is on.
cable-unplugged = Cable unplugged
connect = Connect
connected = Connected
connecting = Connecting…
disconnect = Disconnect
forget = Forget
known-networks = Known networks
network-and-wireless = Network & wireless
network-name = Network Name
no-networks = No networks have been found.
no-vpn = No VPN connections available.
password = Password
password-confirm = Confirm password
qr-code-unavailable = QR code not available
remove = Remove
scan-to-connect-description = Scan the QR code to connect to this network.
settings = Settings
share = Share network
username = Username
visible-networks = Visible networks
identity = Identity

auth-dialog = Authentication required
    .vpn-description = Enter the username and password required by the VPN service.
    .wifi-description = Enter the password or encryption key. You can also connect by pressing the “WPS” button on the router.

forget-dialog = Forget this Wi-Fi network?
    .description = You'll need to enter a password again to use this Wi-Fi network in the future.

network-device-state =
    .activated = Connected
    .config = Connecting
    .deactivating = Disconnecting
    .disconnected = Disconnected
    .failed = Failed to connect
    .ip-check = Checking connection
    .ip-config = Requesting IP and routing info
    .need-auth = Needs authentication
    .prepare = Preparing to connect
    .secondaries = Waiting for secondary connection
    .unavailable = Unavailable
    .unknown = Unknown state
    .unmanaged = Unmanaged
    .unplugged = Cable unplugged

remove-connection-dialog = Remove connection profile?
    .vpn-description = You'll need to enter a password again to use this network in the future.
    .wired-description = You'll need to recreate this profile to use it in the future.

vpn = VPN
    .connections = VPN connections
    .error = Failed to add VPN config
    .remove = Remove connection profile
    .select-file = Select a VPN configuration file

vpn-error = VPN Error
    .config = Failed to add VPN config
    .connect = Failed to connect to VPN
    .connection-editor = Connection editor failed
    .connection-settings = Failed to get settings for active connections
    .updating-state = Failed to update network manager state
    .wireguard-config-path = Invalid file path for WireGuard config
    .wireguard-config-path-desc = Chosen file must be on a local file system.
    .wireguard-device = Failed to create WireGuard device
    .with-password = Failed to set VPN { $field ->
        *[username] username
        [password] password
        [password-flags] password-flags
    } with nmcli

wired = Wired
    .adapter = Wired adapter { $id }
    .connections = Wired connections
    .devices = Wired devices
    .remove = Remove connection profile

wifi = Wi-Fi
    .adapter = Wi-Fi adapter { $id }
    .forget = Forget this network

wireguard-dialog = Add WireGuard device
    .description = Choose a device name for the WireGuard config.

## Networking: Online accounts

online-accounts = Online accounts
    .desc = Add accounts, IMAP and SMTP, enterprise logins

# Bluetooth

activate = Activate
confirm = Confirm
enable = Enable

bluetooth = Bluetooth
    .desc = Manage Bluetooth devices
    .status = This system is visible as { $aliases } while Bluetooth settings are open.
    .connected = Connected
    .connecting = Connecting
    .disconnecting = Disconnecting
    .connect = Connect
    .disconnect = Disconnect
    .forget = Forget
    .dbus-error = An error has occurred while interacting with DBus: { $why }
    .disabled = The Bluetooth service is disabled
    .inactive = The Bluetooth service is not active
    .unknown = The Bluetooth service could not be activated. Is BlueZ installed?

bluetooth-paired = Previously connected devices
    .connect = Connect
    .battery = { $percentage }% battery

bluetooth-confirm-pin = Confirm Bluetooth PIN
    .description = Please confirm that the following PIN matches the one displayed on { $device }

bluetooth-available = Nearby devices

bluetooth-adapters = Bluetooth adapters

## Accessibility

accessibility = Accessibility
    .vision = Vision
    .on = On
    .off = Off
    .unavailable = Unavailable
    .screen-reader = Screen reader
    .high-contrast = High contrast mode
    .invert-colors = Invert colors
    .color-filters = Color filters

hearing = Hearing
    .mono = Play stereo audio as mono

default = Default
magnifier = Magnifier
    .controls = Or use these shortcuts: { $zoom_in ->
             [zero] {""}
            *[other] {""}
                {$zoom_in} to zoom in,
        }{ $zoom_out ->
             [zero] {""}
            *[other] {""}
                {$zoom_out} to zoom out,
        }
        Super + scroll with your mouse
    .scroll_controls = Enable mouse or touchpad zoom with Super + scroll
    .show_overlay = Show the magnifier overlay
    .increment = Zoom increment
    .signin = Start magnifier on sign in
    .applet = Toggle magnifier on/off in applet on the panel
    .movement = Zoomed view moves
    .continuous = Continuously with pointer
    .onedge = When pointer reaches edge
    .centered = To keep pointer centered
color-filter = Color filter type
    .unknown = Unknown filter active
    .greyscale = Greyscale
    .deuteranopia = Green/Red (green weakness, Deuteranopia)
    .protanopia = Red/Green (red weakness, Protanopia)
    .tritanopia = Blue/Yellow (blue weakness, Tritanopia)

## Desktop

desktop = Desktop

## Desktop: Wallpaper

wallpaper = Wallpaper
    .change = Change image every
    .desc = Wallpaper images, colors, and slideshow options
    .fit = Wallpaper fit
    .folder-dialog = Choose wallpaper folder
    .image-dialog = Choose wallpaper image
    .plural = Wallpapers
    .same = Same wallpaper on all displays
    .slide = Slideshow

add-color = Add color
add-image = Add image
all-displays = All displays
colors = Colors
dialog-add = Add
fill = Fill
fit-to-screen = Fit to screen
open-new-folder = Open new folder
recent-folders = Recent folders

x-minutes = { $number } { $number ->
    [one] minute
    *[other] minutes
}
x-hours = { $number } { $number ->
    [one] hour
    *[other] hours
}
never = Never

## Desktop: Appearance

appearance = Appearance
    .desc = Accent colors and theming

accent-color = Accent color
app-background = Window background
auto = Auto
close = Close
color-picker = Color picker
copied-to-clipboard = Copied to clipboard
copy-to-clipboard = Copy to clipboard
dark = Dark
export = Export
hex = Hex
import = Import
light = Light
mode-and-colors = Mode and colors
recent-colors = Recent colors
reset-to-default = Reset to default
rgb = RGB
window-hint-accent = Active window hint color
window-hint-accent-toggle = Use theme accent color as active window hint

auto-switch = Automatically switch between light and dark modes
    .sunrise = Switches to light mode at sunrise
    .sunset = Switches to dark mode at sunset
    .next-sunrise = Switches to light mode at next sunrise
    .next-sunset = Switches to dark mode at next sunset

shadows-floating = Floating windows
    .clip = Match system corners and apply shadows
shadows-tiling = Tiled windows
    .clip = Match system corners
    .shadow = Apply shadows

container-background = Container background
    .desc-detail = Container background color is used for navigation sidebar, side drawer, dialogs and similar widgets. By default, container background color is automatically derived from the window background.
    .reset = Reset to auto
    .desc = Used for navigation sidebar, side drawer, dialogs and similar widgets

control-tint = Control component tint
    .desc = Used for backgrounds of standard buttons, search inputs, text inputs, and similar components

frosted = Frosted glass effect on system interface
    .desc = Applies background blur to panel, dock, applets, launcher, and application library

enable-export = Apply current theme to GNOME apps
    .desc = Not all toolkits support auto-switching. Non-COSMIC apps may need to be restarted after a theme change.

icon-theme = Icon theme
    .desc = Applies a different set of icons to applications

text-tint = Interface text tint
    .desc = Used to derive interface text colors that have sufficient contrast on various surfaces

style = Style
    .round = Round
    .slightly-round = Slightly round
    .square = Square

interface-density = Interface density
    .comfortable = Comfortable
    .compact = Compact
    .spacious = Spacious

window-management-appearance = Window management
    .active-hint = Active window hint size
    .gaps = Gaps around tiled windows

### Experimental

experimental-settings = Experimental settings
icons-and-toolkit = Icons and toolkit theming
interface-font = System font
monospace-font = Monospace font
shadow-and-corners = Window shadow and corners

## Desktop: Notifications

notifications = Notifications
    .desc = Do Not Disturb, lockscreen notifications, and per-application settings

## Desktop: Panel

panel = Panel
    .desc = Primary system bar for menus and applets

add = Add
add-applet = Add applet
all = All
applets = Applets
center-segment = Center segment
place-here = Place applets here
end-segment = End segment
large = Large
no-applets-found = No applets found...
panel-bottom = Bottom
panel-left = Left
panel-right = Right
panel-top = Top
search-applets = Search applets...
small = Small
start-segment = Start segment

panel-appearance = Appearance
    .match = Match desktop
    .light = Light
    .dark = Dark

panel-behavior-and-position = Behavior and positions
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

panel-missing = Panel configuration is missing
    .desc = The panel configuration file is missing due to use of a custom configuration or it is corrupted.
    .fix = Reset to default

## Desktop: Dock

dock = Dock
    .desc = An optional bar for apps and applets

## Desktop: Window management

window-management = Window management
    .desc = Super key action, window control options, and additional window tiling options

super-key = Super key action
    .launcher = Open Launcher
    .workspaces = Open Workspaces
    .applications = Open Applications
    .disable = Disable

edge-gravity = Floating windows gravitate to nearby edges

window-controls = Window controls
    .maximize = Show maximize button
    .minimize = Show minimize button
    .active-window-hint = Show active window hint

focus-navigation = Focus navigation
    .focus-follows-cursor = Focus follows cursor
    .focus-follows-cursor-delay = Focus follows cursor delay in ms
    .cursor-follows-focus = Cursor follows focus

## Desktop: Workspaces

workspaces = Workspaces
    .desc = Workspace orientation and behavior

workspaces-behavior = Workspace behavior
    .dynamic = Dynamic workspaces
    .dynamic-desc = Automatically removes empty workspaces.
    .fixed = Fixed number of Workspaces
    .fixed-desc = Add or remove workspaces in the overview.

workspaces-multi-behavior = Multi-monitor behavior
    .span = Workspaces span displays
    .separate = Displays have separate workspaces

workspaces-overview-thumbnails = Workspace overview thumbnails
    .show-number = Show workspace number
    .show-name = Show workspace name

workspaces-orientation = Workspaces orientation
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
    .desc = Manage displays and night light
    .arrangement = Display arrangement
    .arrangement-desc = Drag displays to rearrange them
    .enable = Enable display
    .external = { $size } { $output } external display
    .laptop = { $size } laptop display
    .options = Display options
    .refresh-rate = Refresh rate
    .resolution = Resolution
    .scale = Scale
    .additional-scale-options = Additional scale options

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

night-light = Night light
    .auto = Automatic (sunset to sunrise)
    .desc = Reduce blue light with warmer colors

orientation = Orientation
    .standard = Standard
    .rotate-90 = Rotate 90°
    .rotate-180 = Rotate 180°
    .rotate-270 = Rotate 270°

vrr = Variable refresh rate
    .enabled = Enabled
    .force = Always
    .auto = Automatic
    .disabled = Disabled

scheduling = Scheduling
    .manual = Manual schedule

dialog = Dialog
    .title = Keep these display settings?
    .keep-changes = Keep changes
    .change-prompt = Settings changes will automatically revert in { $time } seconds.
    .revert-settings = Revert settings

## Sound

sound = Sound
    .desc = N/A

sound-output = Output
    .volume = Output volume
    .device = Output device
    .level = Output level
    .config = Configuration
    .balance = Balance
    .left = Left
    .right = Right

sound-input = Input
    .volume = Input volume
    .device = Input device
    .level = Input level

amplification = Amplification
    .desc = Allows raising the volume to 150%

sound-alerts = Alerts
    .volume = Alerts volume
    .sound = Alerts sound

sound-applications = Applications
    .desc = Application volumes and settings

# No speaker, headphones, or microphone plugged into sound card port
sound-device-port-unplugged = Unplugged
sound-hd-audio = HD Audio
sound-usb-audio = USB Audio

# Profiles for sound card devices
sound-device-profiles = Device profiles

# Power & Battery settings page
power = Power & battery
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

connected-devices = Connected devices
  .unknown = Unknown device

power-mode = Power mode
    .battery = Extended battery life
    .battery-desc = Reduced power usage and silent performance
    .balanced = Balanced
    .balanced-desc = Quiet performance and moderate power usage
    .performance = High performance
    .performance-desc = Peak performance and power usage
    .no-backend = Backend not found. Install system76-power or power-profiles-daemon.

power-saving = Power saving options
    .turn-off-screen-after = Turn off the screen after
    .auto-suspend = Automatic suspend
    .auto-suspend-ac = Automatic suspend when plugged in
    .auto-suspend-battery = Automatic suspend on battery power

## Input

acceleration-desc = Automatically adjusts tracking sensitivity based on speed

disable-while-typing = Disable while typing

input-devices = Input devices
    .desc = Input devices

primary-button = Primary button
    .desc = Sets the order of physical buttons
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
    .desc = Input sources, switching, special character entry, shortcuts

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
    .compose-desc = The compose key allows a wide variety of characters to be entered. To use it, press compose and then a sequence of characters. For example, compose key followed by C and o will enter ©, while compose key followed by a and ‘ will enter á.
    .caps = Caps Lock key

keyboard-typing-assist = Typing
    .repeat-rate = Repeat rate
    .repeat-delay = Repeat delay

keyboard-numlock-boot = Numlock
    .boot-state = State on boot
    .last-boot = Last boot
    .on = On
    .off = Off
    .set = Set numlock boot state

added = Added
type-to-search = Type to search...
show-extended-input-sources = Show extended input sources

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Keyboard shortcuts
    .desc = View and customize shortcuts

add-another-keybinding = Add another keybinding
cancel = Cancel
command = Command
custom = Custom
debug = Debug
disabled = Disabled
input-source-switch = Switch keyboard language input source
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

custom-shortcuts = Custom shortcuts
    .add = Add shortcut
    .context = Add custom shortcut
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
    .fullscreen = Fullscreen window
    .minimize = Minimize window
    .resize-inwards = Resize window inwards
    .resize-outwards = Resize window outwards
    .toggle-sticky = Toggle sticky window

move-windows = Move windows
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
    .display-toggle = Toggle internal display
    .home-folder = Open home folder
    .keyboard-brightness-down = Decrease keyboard brightness
    .keyboard-brightness-up = Increase keyboard brightness
    .launcher = Open the Launcher
    .log-out = Log Out
    .lock-screen = Lock the screen
    .mute = Mute audio output
    .mute-mic = Mutes microphone input
    .play-pause = Play/pause
    .play-next = Next track
    .play-prev = Previous track
    .poweroff = Power off
    .screenshot = Take a screenshot
    .suspend = Suspend
    .terminal = Open a terminal
    .touchpad-toggle = Toggle touchpad
    .volume-lower = Decrease audio output volume
    .volume-raise = Increase audio output volume
    .web-browser = Open a web browser
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

replace-shortcut-dialog = Replace shortcut?
    .desc = { $shortcut } is used by { $name }. If you replace it, { $name } will be disabled.

zoom-in = Zoom In
zoom-out = Zoom Out

## Input: Mouse

mouse = Mouse
    .desc = Mouse speed, acceleration, natural scrolling
    .speed = Mouse speed
    .acceleration = Enable mouse acceleration

## Input: Touchpad

click-behavior = Click Behavior
    .click-finger = Secondary click with two fingers and middle-click with three fingers
    .button-areas = Secondary click in bottom right corner and middle-click in bottom center

pinch-to-zoom = Pinch to zoom
    .desc = Use two fingers to zoom into content, for applications that support zoom

tap-to-click = Tap to click
    .desc = Enables single-finger tap for primary click, two-finger tap for secondary click, and three-finger tap for middle click

touchpad = Touchpad
    .acceleration = Enable touchpad acceleration
    .desc = Touchpad speed, click options, gestures
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

## Time & language

time = Time & language
    .desc = N/A

time-date = Date & time
    .desc = Time zone, automatic clock settings, and time formatting
    .auto = Set automatically
    .auto-ntp = Date & time will update automatically when the time zone is set

time-zone = Time zone
    .auto = Automatic time zone
    .auto-info = Requires location services and internet access

time-format = Date & time format
    .twenty-four = 24-hour time
    .show-seconds = Show seconds
    .first = First day of week
    .show-date = Show date in the time applet
    .friday = Friday
    .saturday = Saturday
    .sunday = Sunday
    .monday = Monday

time-region = Region & language
    .desc = Format dates, times, and numbers based on your region

formatting = Formatting
    .dates = Dates
    .time = Time
    .date-and-time = Date & time
    .numbers = Numbers
    .measurement = Measurement
    .paper = Paper

preferred-languages = Preferred languages
    .desc = The order of languages determines which language is used for the user interface. Changes take effect on next login.

add-language = Add language
    .context = Add Language
install-additional-languages = Install additional languages
region = Region

## Applications

applications = Applications

## Applications: Default applications

default-apps = Default Applications
    .desc = Default web browser, mail client, file browser, and other applications
    .web-browser = Web browser
    .file-manager = File manager
    .mail-client = Mail client
    .music = Music
    .video = Video
    .photos = Photos
    .calendar = Calendar
    .terminal = Terminal
    .other-associations = Other associations
    .text-editor = Text Editor
    .not-installed = Not installed


## Applications: Startup applications

startup-apps = Startup applications
    .desc = Configure applications which run on login
    .add = Add app
    .user = Applications launched when you log in
    .none = No startup applications added
    .remove-dialog-title = Remove { $name }?
    .remove-dialog-description = Remove this startup application?
    .add-startup-app = Add startup application

## Applications: Legacy applications

legacy-applications = X11 applications compatibility
    .desc = X11 Window system application scaling and Global shortcuts

legacy-app-global-shortcuts = Global shortcuts in X11 applications
    .desc = Global shortcuts allows keystrokes and mouse button events performed in applications to be recognized by other applications for features like push-to-talk or push-to-mute. By default, Global shortcuts is disabled in X11 applications to ensure other applications can’t monitor for keyboard and mouse events containing sensitive information.
    .none = No keys
    .modifiers = Modifiers (Super, Shift, Control, Alt)
    .combination = All keys while modifiers Super, Control or Alt are being pressed
    .all = All keys
    .mouse = Mouse button events in X11 applications

legacy-app-scaling = X11 window system application scaling
    .scaled-gaming = Optimize for gaming and full-screen apps
    .gaming-description = X11 applications may appear slightly larger/smaller compared to Wayland apps
    .scaled-applications = Optimize for applications
    .applications-description = Games and full-screen X11 apps may not match your display resolution
    .scaled-compatibility = Maximum compatibility mode
    .compatibility-description = X11 applications may appear blurry on HiDPI screens
    .preferred-display = Preferred display for games and full screen X11 applications
    .no-display = None

## System

system = System & accounts

## System: About

about = About
    .desc = Device name, hardware information, operating system defaults

about-device = Device name
    .desc = This name appears to other network or Bluetooth devices

about-hardware = Hardware
    .model = Hardware model
    .memory = Memory
    .processor = Processor
    .graphics = Graphics
    .disk-capacity = Disk capacity

about-os = Operating system
    .os = Operating system
    .os-architecture = Operating system architecture
    .kernel = Kernel version
    .desktop-environment = Desktop environment
    .windowing-system = Windowing system

about-related = Related settings
    .support = Get support

## System: Firmware

firmware = Firmware
    .desc = Firmware details

## System: Users

users = Users
    .desc = Authentication and user accounts
    .admin = Admin
    .standard = Standard
    .profile-add = Choose profile image

administrator = Administrator
    .desc = Administrators can change settings for all users, add and remove other users

add-user = Add user
change-password = Change password
remove-user = Remove user
full-name = Full name
invalid-username = Invalid username
password-mismatch = Password and confirmation must match
save = Save
