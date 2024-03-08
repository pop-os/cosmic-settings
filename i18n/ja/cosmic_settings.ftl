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
window-hint-accent-toggle = テーマのアクセントカラーをアクティブなウィンドウのヒントとして使う

auto-switch = 自動的にライトモードからダークモードに切り替える
    .desc = 日の出になるとライトモードに切り替えます。

container-background = コンテナー背景
    .desc-detail = サイドバー、ダイアログ、類似ウィジェットで使用されます。 規定では、アプリケーションまたはウィンドウの背景から自動的に選択されています。
    .reset = 自動的な選択に戻す
    .desc = Primary container color is used for navigation sidebar, side drawer, dialogs and similar widgets.

control-tint = コントロールコンポーネントティント
    .desc = スタンダードボタン、検索入力、テキスト入力、類似コンポーネントで使用されます。

frosted = システムインターフェイスのすりガラス効果
    .desc = パネル、ドック、アプレット、ランチャー、アプリケーションライブラリの背景をぼかします。

text-tint = Interface text tint
    .desc = Color used to derive interface text colors that have sufficient contrast on various surfaces.

style = スタイル
    .round = 丸い
    .slightly-round = 少し丸い
    .square = 四角い

# interface density left out for now
window-management = ウィンドウマネジメント
    .active-hint = Active window hint size
    .gaps = タイル型ウィンドウ

## Desktop: Display

-requires-restart = 再起動が必要です。

color = 色
    .depth = 色深度
    .profile = カラープロファイル
    .sidebar = カラープロファイル
    .temperature = 色温度

display = ディスプレイ
    .desc = Manage displays, graphics switching, and night light
    .arrangement = ディスプレイの配列
    .arrangement-desc = Drag displays to rearrange them.
    .enable = ディスプレイを有効にする
    .external = { $size } { $output } 外付けディスプレイ
    .laptop = { $size }ラップトップディスプレイ
    .options = ディスプレイの設定
    .refresh-rate = Refresh rate
    .resolution = Resolution
    .scale = スケーリング

graphics-mode = グラフィックスモード
    .mode = { $mode ->
        [compute] 計算
        *[hybrid] ハイブリッド
        [integrated] 内蔵
        [nvidia] NVIDIA
    } graphics
    .enable = { $mode ->
        [compute] 計算
        *[hybrid] ハイブリッド
        [integrated] 内蔵
        [nvidia] NVIDIA
    }グラフィックスを有効にする
    .desc = { $mode ->
        [compute] 計算の作業負荷だけで専用グラフィックスを使います。外付けディスプレイを無効にします。{ -requires-restart }
        *[hybrid] 専用グラフィックスを使用する要求がない限り、プログラムは統合グラフィックスを使用します。 { -requires-restart }
        [integrated] バッテリー寿命を強化してファン騒音を低減するように専用グラフィックスを無効にします。
        [nvidia] 改善されたグラッフィク体験と最高消費電力です。 { -requires-restart }.
    }
    .restart = 再起動して{ $mode }に切り替えますか？
    .restart-desc = { $mode }に切り替えると全てのアプリケーションを閉じます。

mirroring = ミラーリング
    .id = { $id }をミラーリングしています
    .dont = ミラーリングしない
    .mirror = { $display }をミラーリングする
    .project = { $display ->
        [all] 全てのディスプレイ
        *[other] { $display }
    }に投影する

    .project-count = 他の{ $count}つのディスプレイに投影している

night-light = ナイトライト
    .auto = 自動的（日の入りから日の出まで）
    .desc = 暖色で青い光を減らします。

orientation = 画面の向き
    .standard = 回さない
    .rotate-90 = 90度回転
    .rotate-180 = 180度回転
    .rotate-270 = 270度回転

scheduling = Scheduling
    .manual = Manual schedule

## Desktop: Notifications

notifications = 通知
    .desc = 通知ポップアップの表示、ロック画面の通知、アプリケーションごとの設定。

## Desktop: Options

desktop-panel-options = デスクトップとパネル
    .desc = スーパーキーの行動、ホットコーナー、ウィンドウコントロールの設定。

desktop-panels-and-applets = デスクトップパネルとアプレット

dock = ドック
    .desc = 固定されたアプリケーションありパネル。

hot-corner = ホットコーナー
    .top-left-corner = ワークスペースための左上のホットコーナーを有効にする

super-key-action = スーパーキーの行動
    .launcher = ランチャー
    .workspaces = ワークスペース
    .applications = アプリケーション

top-panel = トップパネル
    .workspaces = ワークスペースボタンを表示
    .applications = アプリケーションボタンを表示

window-controls = ウィンドウコントロール
    .minimize = 最小化ボタンを表示
    .maximize = 最大化ボタンを表示

## Desktop: Panel

panel = パネル
    .desc = デスクトップコントロールとメニューありトップパネル。

add = 追加
add-applet = アプレットを追加
all = すべて
applets = アプレット
center-segment = 中心の部分
drop-here = ここにアプレットを入れてください
end-segment = 最後の部分
large = 大きい
no-applets-found = アプレットは見つけられませんでした...
panel-bottom = 下
panel-left = 左
panel-right = 右
panel-top = 上
search-applets = アプレットを検索...
small = 小さい
start-segment = 最初の部分

panel-appearance = 外観
    .match = システム設定に従う
    .light = ライト
    .dark = ダーク

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
    .appearance = 外観
    .size = 大きさ
    .background-opacity = 背景の不透明度

panel-applets = 構成
    .dock-desc = ドックのアプレットを構成
    .desc = パネルのアプレットを構成

panel-missing = パネル構成は見つけられません
    .desc = パネル構成ファイルは壊れているか、カスタム構成の使用により見つけられません。
    .fix = デフォルトに戻す

## Desktop: Wallpaper

wallpaper = 背景
    .change = この期間ごとに画像を変える：
    .desc = 背景の画像、色、スライドショー設定。
    .fit = Wallpaper fit
    .folder-dialog = 背景のフォルダを選択
    .image-dialog = 背景の画像を選択
    .plural = 背景
    .same = すべてのディスプレイで同じ背景を使う
    .slide = スライドショー

add-color = 色を追加
add-image = 画像を追加
all-displays = 全てのディスプレイ
colors = 色
dialog-add = _追加
fill = Fill
fit-to-screen = 画面に合わせる
open-new-folder = 新しいフォルダを開く
recent-folders = 最近のフォルダ

x-minutes = { $number }分
x-hours = { $number }時間

## Desktop: Workspaces

workspaces = ワークスペース
    .desc = Set workspace number, behavior, and placement.

workspaces-behavior = ワークスペースの動作
    .dynamic = ダイナミックワークスペース
    .dynamic-desc = 自動的に空のワークスペースを削除します。
    .fixed = ワークスペース数を固定
    .fixed-desc = オーバービューでワークスペース数を変えられます。

workspaces-multi-behavior = マルチモニターの動作
    .span = Workspaces Span Displays
    .separate = Displays Have Separate Workspaces

workspaces-overview-thumbnails = ワークスペースオーバービューサムネール
    .show-number = ワークスペース番号を表示
    .show-name = ワークスペース名を表示

workspaces-orientation = Workspaces Orientation
    .vertical = Vertical
    .horizontal = Horizontal

## Networking: Wired

wired = 有線
    .desc = 有線接続、接続プロファイル

## Networking: Online Accounts

online-accounts = オンラインアカウント
    .desc = アカウント、IMAPとSMTP、エンタプライズログインを追加

## Time & Language

time = 日付と時刻
    .desc = N/A

time-date = 日付と時刻
    .desc = Time zone, automatic clock settings, and some time formatting.
    .auto = 自動的に設定する

time-zone = 等時帯
    .auto = Automatic time zone
    .auto-info = Requires location services and internet access

time-format = 日付と時刻
    .twenty-four = 24-hour time
    .first = First day of week

time-region = 地方と言語
    .desc = Format dates, times, and numbers based on your region

## Sound

sound = サウンド
    .desc = N/A

sound-output = 出力
    .volume = 出力音量
    .device = 出力デバイス
    .level = 出力レベル
    .config = 構成
    .balance = バランス

sound-input = 入力
    .volume = 入力音量
    .device = 入力デバイス
    .level = 入力レベル

sound-alerts = アラート
    .volume = アラート音量
    .sound = アラート音

sound-applications = アプリケーション
    .desc = Application volumes and settings

## System

system = システムとアカウント

## System: About

about = このデバイスについて
    .desc = デバイス名、ハードウェアの情報、OSのデフォルト。

about-device = デバイス名
    .desc = この名前は他のネットワークまたはBluetoothデバイスに表示されます。

about-hardware = ハードウェア
    .model = Hardware model
    .memory = メモリー
    .processor = プロセッサー
    .graphics = グラフィックス
    .disk-capacity = Disk Capacity

about-os = オペレーティングシステム
    .os = Operating system
    .os-architecture = Operating system architecture
    .desktop-environment = Desktop environment
    .windowing-system = Windowing system

about-related = 関係がある設定
    .support = 

## System: Firmware

firmware = ファームウェア
    .desc = ファームウェアの情報。

## System: Users

users = ユーザー
    .desc = 認証とログイン、ロック画面。

## Input

acceleration-desc = Automatically adjusts tracking sensitivity based on speed.

disable-while-typing = Disable while typing

input-devices = 入力デバイス
    .desc = 入力デバイス

primary-button = Primary button
    .left = 左
    .right = 右

scrolling = スクロール
    .two-finger = Scroll with two fingers
    .edge = Scroll along the edge with one finger
    .speed = Scrolling speed
    .natural = Natural scrolling
    .natural-desc = Scroll the content, instead of the view

## Input: Keyboard

keyboard = キーボード
    .desc = キーボードの入力

keyboard-sources = Input Sources
    .desc = Input sources can be switched using Super+Space key combination. This can be customized in the keyboard shortcut settings.
    .move-up = Move up
    .move-down = Move down
    .settings = 設定
    .view-layout = View keyboard layout
    .remove = 削除

keyboard-special-char = Special Character Entry
    .alternate = Alternate characters key
    .compose = Compose key

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Keyboard Shortcuts
    .desc = View and customize shortcuts

## Input: Mouse

mouse = マウス
    .desc = Mouse speed, acceleration, natural scrolling.
    .speed = Mouse speed
    .acceleration = Enable mouse acceleration

## Input: Touchpad

click-behavior = Click Behavior
    .click-finger = Secondary click with two fingers and middle-click with three fingers
    .button-areas = Secondary click in bottom right corner and middle-click in bottom center

pinch-to-zoom = Pinch to zoom
    .desc = Use two fingers to zoom into content, for applications that support zoom.

tap-to-click = タップでクリック
    .desc = Enables single-finger tap for primary click, two-finger tap for secondary click, and three-finger tap for middle click.

touchpad = Touchpad
    .acceleration = Enable touchpad acceleration
    .desc = Touchpad speed, click options, gestures.
    .speed = Touchpad speed

## Input: Gestures

swiping = スワイプ
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
