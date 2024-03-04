app = COSMIC Settings

unknown = 不明

number = { $number }

## Desktop

desktop = デスクトップ

## Desktop: 外観

appearance = 外観
    .desc = アクセントカラーとCOSMICのテーマ

accent-color = アクセントカラー
app-background = アプリケーションおよびウィンドウの背景
auto = 自動的
close = 閉じる
color-picker = カラーピッカー
copied-to-clipboard = クリップボードにコピーされた
copy-to-clipboard = クリップボードにコピー
dark = ダーク
export = エクスポート
hex = 16進値
import = インポート
light = ライト
mode-and-colors = モードと色
recent-colors = 最近の色
reset-default = デフォルトに戻す
reset-to-default = デフォルトに戻す
rgb = RGB
window-hint-accent = アクティブなウィンドウのハイライトカラー
window-hint-accent-toggle = アクティブなUse theme accent color as active window hint

auto-switch = 自動的にライトモードからダークモードに切り替える
    .desc = 日の出になるとライトモードに切り替えます。

container-background = Container background
    .desc-detail = Container background color is used for navigation sidebar, side drawer, dialogs and similar widgets. By default, it is automatically derived from the Application or window background.
    .reset = Reset to auto
    .desc = Primary container color is used for navigation sidebar, side drawer, dialogs and similar widgets.

control-tint = Control component tint
    .desc = Used for backgrounds of standard buttons, search inputs, text inputs, and similar components.

frosted = Frosted glass effect on system interface
    .desc = Applies background blur to panel, dock, applets, launcher, and application library.

text-tint = Interface text tint
    .desc = Color used to derive interface text colors that have sufficient contrast on various surfaces.

style = Style
    .round = 丸い
    .slightly-round = 少し丸い
    .square = 四角い

# interface density left out for now
window-management = Window Management
    .active-hint = Active window hint size
    .gaps = Gaps around tiled windows

## Desktop: Display

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

graphics-mode = Graphics mode
    .mode = { $mode ->
        [compute] Compute
        *[hybrid] Hybrid
        [integrated] Integrated
        [nvidia] NVIDIA
    } graphics
    .enable = Enable { $mode ->
        [compute] compute
        *[hybrid] hybrid
        [integrated] integrated
        [nvidia] NVIDIA
    } graphics
    .desc = { $mode ->
        [compute] Uses dedicated graphics for computational workloads only. Disables external displays. { -requires-restart }.
        *[hybrid] Applications use integrated graphics unless explicitly requested to use dedicated graphics. { -requires-restart }.
        [integrated] Turns off dedicated graphics for a longer battery life and less fan noise.
        [nvidia] Better graphical experience and highest power usage. { -requires-restart }.
    }
    .restart = Restart and switch to { $mode }?
    .restart-desc = Switching to { $mode } will close all open applications

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

night-light = ナイトライト
    .auto = 自動的（日の入りから日の出まで）
    .desc = 暖色で青い光を減らします。

orientation = Orientation
    .standard = Standard
    .rotate-90 = Rotate 90
    .rotate-180 = Rotate 180
    .rotate-270 = Rotate 270

scheduling = Scheduling
    .manual = Manual schedule

## Desktop: Notifications

notifications = 通知
    .desc = Do Not Disturb, lockscreen notifications, and per-application settings.

## Desktop: Options

desktop-panel-options = デスクトップとパネル
    .desc = Super Key action, hot corners, window control options.

desktop-panels-and-applets = デスクトップパネルとアプレット

dock = ドック
    .desc = Panel with pinned applications.

hot-corner = ホットコーナー
    .top-left-corner = Enable top-left hot corner for Workspaces

super-key-action = スーパーキーの行動
    .launcher = Launcher
    .workspaces = Workspaces
    .applications = Applications

top-panel = トップパネル
    .workspaces = Show Workspaces Button
    .applications = Show Applications Button

window-controls = Window Controls
    .minimize = Show Minimize Button
    .maximize = Show Maximize Button

## Desktop: Panel

panel = パネル
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

panel-appearance = 外観
    .match = Match desktop
    .light = Light
    .dark = Dark

panel-behavior-and-position = Behavior and Positions
    .autohide = Automatically hide panel
    .dock-autohide = Automatically hide dock
    .position = Position on screen
    .display = Show on display

panel-style = パネルスタイル
    .anchor-gap = Gap between panel and screen edges
    .dock-anchor-gap = Gap between dock and screen edges
    .extend = Extend panel to screen edges
    .dock-extend = Extend dock to screen edges
    .appearance = Appearance
    .size = Size
    .background-opacity = Background opacity

panel-applets = 構成
    .dock-desc = Configure dock applets.
    .desc = Configure panel applets.

panel-missing = パネル構成は見つけられません
    .desc = The panel configuration file is missing due to use of a custom configuration or it is corrupted.
    .fix = Reset to default

## Desktop: Wallpaper

wallpaper = 背景
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
dialog-add = _Add
fill = Fill
fit-to-screen = Fit to Screen
open-new-folder = Open new folder
recent-folders = Recent Folders

x-minutes = { $number } minutes
x-hours = { $number ->
    [1] 1 hour
    *[other] { $number } hours
}

## Desktop: Workspaces

workspaces = ワークスペース
    .desc = Set workspace number, behavior, and placement.

workspaces-behavior = ワークスペースの動作
    .dynamic = Dynamic workspaces
    .dynamic-desc = Automatically removes empty workspaces.
    .fixed = Fixed Number of Workspaces
    .fixed-desc = Add or remove workspaces in the overview.

workspaces-multi-behavior = マルチモニターの動作
    .span = Workspaces Span Displays
    .separate = Displays Have Separate Workspaces

workspaces-overview-thumbnails = Workspace Overview Thumbnails
    .show-number = Show Workspace Number
    .show-name = Show Workspace Name

workspaces-orientation = Workspaces Orientation
    .vertical = Vertical
    .horizontal = Horizontal

## Networking: Wired

wired = Wired
    .desc = Wired connection, connection profiles

## Networking: Online Accounts

online-accounts = オンラインアカウント
    .desc = Add accounts, IMAP and SMTP, enterprise logins

## Time & Language

time = 日付と時刻
    .desc = N/A

time-date = 日付と時刻
    .desc = Time zone, automatic clock settings, and some time formatting.
    .auto = Set automatically

time-zone = 等時帯
    .auto = Automatic time zone
    .auto-info = Requires location services and internet access

time-format = Date & Time Format
    .twenty-four = 24-hour time
    .first = First day of week

time-region = 地方と言語
    .desc = Format dates, times, and numbers based on your region

## Sound

sound = サウンド
    .desc = N/A

sound-output = 出力
    .volume = Output volume
    .device = Output device
    .level = Output level
    .config = Configuration
    .balance = Balance

sound-input = 入力
    .volume = Input volume
    .device = Input device
    .level = Input level

sound-alerts = アラート
    .volume = Alerts volume
    .sound = Alerts sound

sound-applications = アプリケーション
    .desc = Application volumes and settings

## System

system = システムとアカウント

## System: About

about = このデバイスについて
    .desc = Device name, hardware information, operating system defaults.

about-device = デバイス名
    .desc = This name appears to other network or bluetooth devices.

about-hardware = ハードウェア
    .model = Hardware model
    .memory = Memory
    .processor = Processor
    .graphics = Graphics
    .disk-capacity = Disk Capacity

about-os = オペレーティングシステム
    .os = Operating system
    .os-architecture = Operating system architecture
    .desktop-environment = Desktop environment
    .windowing-system = Windowing system

about-related = 関係がある設定
    .support = Get support

## System: Firmware

firmware = ファームウェア
    .desc = Firmware details.

## System: Users

users = ユーザー
    .desc = Authentication and login, lock screen.

## Input

acceleration-desc = Automatically adjusts tracking sensitivity based on speed.

disable-while-typing = Disable while typing

input-devices = Input Devices
    .desc = Input Devices

primary-button = Primary button
    .left = Left
    .right = Right

scrolling = Scrolling
    .two-finger = Scroll with two fingers
    .edge = Scroll along the edge with one finger
    .speed = Scrolling speed
    .natural = Natural scrolling
    .natural-desc = Scroll the content, instead of the view

## Input: Keyboard

keyboard = Keyboard
    .desc = Keyboard input

keyboard-sources = Input Sources
    .desc = Input sources can be switched using Super+Space key combination. This can be customized in the keyboard shortcut settings.
    .move-up = Move up
    .move-down = Move down
    .settings = Settings
    .view-layout = View keyboard layout
    .remove = Remove

keyboard-special-char = Special Character Entry
    .alternate = Alternate characters key
    .compose = Compose key

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Keyboard Shortcuts
    .desc = View and customize shortcuts

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

swiping = Swiping
    .four-finger-down = Four-finger swipe down
    .four-finger-left = Four-finger swipe left
    .four-finger-right = Four-finger swipe right
    .four-finger-up = Four-finger swipe up
    .three-finger-any = Three-finger swipe any direction

switch-between-windows = Switch between windows
switch-to-next-workspace = Switch to next workspace
switch-to-prev-workspace = Switch to prev workspace
open-application-library = Open Application Library
open-workspaces-view = Open Workspaces Overview
