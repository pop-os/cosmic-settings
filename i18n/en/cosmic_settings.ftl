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
known-networks = Known Networks
network-and-wireless = Network & Wireless
no-networks = No networks have been found.
no-vpn = No VPN connections available.
password = Password
password-confirm = Confirm Password
remove = Remove
settings = Settings
username = Username
visible-networks = Visible Networks
identity = Identity

auth-dialog = Authentication Required
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

remove-connection-dialog = Remove Connection Profile?
    .vpn-description = You'll need to enter a password again to use this network in the future.
    .wired-description = You'll need to recreate this profile to use it in the future.

vpn = VPN
    .connections = VPN Connections
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
    .connections = Wired Connections
    .devices = Wired Devices
    .remove = Remove connection profile

wifi = Wi-Fi
    .adapter = Wi-Fi adapter { $id }
    .forget = Forget this network

wireguard-dialog = Add WireGuard device
    .description = Choose a device name for the WireGuard config.

## Networking: Online Accounts

online-accounts = Online Accounts
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

bluetooth-paired = Previously Connected Devices
    .connect = Connect
    .battery = { $percentage }% battery

bluetooth-confirm-pin = Confirm Bluetooth PIN
    .description = Please confirm that the following PIN matches the one displayed on { $device }

bluetooth-available = Nearby Devices

bluetooth-adapters = Bluetooth Adapters

## Accessibility

accessibility = Accessibility
    .vision = Vision
    .on = On
    .off = Off
    .unavailable = Unavailable
    .screen-reader = Screen reader
    .high-contrast = High contrast mode
    .invert-colors = Invert Colors
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
    .scroll_controls = Enable mouse or touchpad zoom with Super + Scroll
    .show_overlay = Show the Magnifier Overlay
    .increment = Zoom increment
    .signin = Start magnifier on sign in
    .applet = Toggle magnifier on/off in applet on the panel
    .movement = Zoomed view moves
    .continuous = Continuously with pointer
    .onedge = When pointer reaches edge
    .centered = To keep pointer centered
color-filter = Color filter type
    .unknown = Unknown Filter active
    .greyscale = Greyscale
    .deuteranopia = Green/Red (green weakness, Deuteranopia)
    .protanopia = Red/Green (red weakness, Protanopia)
    .tritanopia = Blue/Yellow (blue weakness, Tritanopia)

## Desktop

desktop = Desktop

## Desktop: Wallpaper

wallpaper = Wallpaper
    .change = Change image every
    .desc = Wallpaper images, colors, and slideshow options.
    .fit = Wallpaper fit
    .folder-dialog = Choose wallpaper folder
    .image-dialog = Choose wallpaper image
    .plural = Wallpapers
    .same = Same wallpaper on all displays
    .slide = Slideshow

add-color = Add color
add-image = Add image
all-displays = All Displays
colors = Colors
dialog-add = Add
fill = Fill
fit-to-screen = Fit to Screen
open-new-folder = Open new folder
recent-folders = Recent Folders

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
    .desc = Accent colors and theming.

accent-color = Accent color
app-background = Application or window background
auto = Auto
close = Close
color-picker = Color Picker
copied-to-clipboard = Copied to clipboard
copy-to-clipboard = Copy to clipboard
dark = Dark
export = Export
hex = Hex
import = Import
light = Light
mode-and-colors = Mode and Colors
recent-colors = Recent colors
reset-to-default = Reset to default
rgb = RGB
window-hint-accent = Active window hint color
window-hint-accent-toggle = Use theme accent color as active window hint

auto-switch = Automatically switch between Light and Dark modes
    .sunrise = Switches to Light mode at sunrise
    .sunset = Switches to Dark mode at sunset
    .next-sunrise = Switches to Light mode at next sunrise
    .next-sunset = Switches to Dark mode at next sunset

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
    .desc = Primary system bar for menus and applets.

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
    .desc = An optional bar for apps and applets.

## Desktop: Window management

window-management = Window management
    .desc = Super key action, window control options, and additional window tiling options.

super-key = Super key action
    .launcher = Open Launcher
    .workspaces = Open Workspaces
    .applications = Open Applications
    .disable = Disable

edge-gravity = Floating windows gravitate to nearby edges

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
    .desc = Manage displays and night light
    .arrangement = Display Arrangement
    .arrangement-desc = Drag displays to rearrange them.
    .enable = Enable display
    .external = { $size } { $output } External Display
    .laptop = { $size } Laptop Display
    .options = Display Options
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

night-light = Night Light
    .auto = Automatic (sunset to sunrise)
    .desc = Reduce blue light with warmer colors.

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
    .title = Keep These Display Settings?
    .keep-changes = Keep Changes
    .change-prompt = Settings changes will automatically revert in { $time } seconds.
    .revert-settings = Revert Settings

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
    .desc = Allows raising the volume to 150%.

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

keyboard-shortcuts = Keyboard Shortcuts
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
    .fullscreen = Fullscreen window
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
    .log-out = Log Out
    .lock-screen = Lock the screen
    .mute = Mute audio output
    .mute-mic = Mutes microphone input
    .play-pause = Play/Pause
    .play-next = Next track
    .play-prev = Previous track
    .poweroff = Power off
    .screenshot = Take a screenshot
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

replace-shortcut-dialog = Replace Shortcut?
    .desc = { $shortcut } is used by { $name }. If you replace it, { $name } will be disabled.

zoom-in = Zoom In
zoom-out = Zoom Out

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
    .show-date = Show date in the time applet
    .friday = Friday
    .saturday = Saturday
    .sunday = Sunday
    .monday = Monday

time-region = Region & Language
    .desc = Format dates, times, and numbers based on your region.

formatting = Formatting
    .dates = Dates
    .time = Time
    .date-and-time = Date & Time
    .numbers = Numbers
    .measurement = Measurement
    .paper = Paper

preferred-languages = Preferred Languages
    .desc = The order of languages determines which language is used for the user interface. Changes take effect on next login.

add-language = Add language
    .context = Add Language
install-additional-languages = Install additional languages
region = Region

## Applications

applications = Applications

## Applications: Default Applications

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
    .text-editor = Text Editor

## Applications: Startup Applications

startup-apps = Startup Applications
    .desc = Configure applications which run on login.
    .add = Add app
    .user = Applications launched when you log in
    .none = No startup applications added
    .remove-dialog-title = Remove { $name }?
    .remove-dialog-description = Are you sure you want to remove this startup application?
    .search-for-application = Search for application

## Applications: Legacy Applications

legacy-applications = X11 Applications Compatibility
    .desc = X11 Window system application scaling and Global shortcuts.

legacy-app-global-shortcuts = Global Shortcuts in X11 Applications
    .desc = Global shortcuts allows keystrokes and mouse button events performed in applications to be recognized by other applications for features like push-to-talk or push-to-mute. By default, this is disabled in X11 applications to ensure other applications can’t monitor for keyboard and mouse events containing sensitive information.
    .none = No keys
    .modifiers = Modifiers (Super, Shift, Control, Alt)
    .combination = All keys while modifiers Super, Control or Alt are being pressed
    .all = All keys
    .mouse = Mouse button events in X11 applications

legacy-app-scaling = X11 Window System Application Scaling
    .scaled-gaming = Optimize for gaming and full-screen apps
    .gaming-description = X11 applications may appear slightly larger/smaller compared to Wayland apps.
    .scaled-applications = Optimize for applications
    .applications-description = Games and full-screen X11 apps may not match your display resolution.
    .scaled-compatibility = Maximum compatibility mode
    .compatibility-description = X11 applications may appear blurry on HiDPI screens.
    .preferred-display = Preferred display for games and full screen X11 applications
    .no-display = None

## System

system = System & Accounts

## System: About

about = About
    .desc = Device name, hardware information, operating system defaults.

about-device = Device name
    .desc = This name appears to other network or Bluetooth devices.

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
change-password = Change password
remove-user = Remove user
full-name = Full name
invalid-username = Invalid username.
password-mismatch = Password and confirmation must match.
save = Save
