app = COSMIC設定

unknown = 不明

number = { $number }

## Network & Wireless

connections-and-profiles = { $variant ->
    [wired] 有線の
    [wifi] Wi-Fiの
    [vpn] VPNの
    *[other] 不明な
}接続と接続プロファイル

add-network = ネットワークを追加
    .profile = プロファイルを追加
add-vpn = VPNを追加
airplane-on = 機内モードは有効です。
cable-unplugged = ケーブルは抜けています。
connect = 接続する
connected = 接続済み
connecting = 接続中…
disconnect = 切断する
forget = 忘れる
known-networks = 既知のネットワーク
network-and-wireless = ネットワークとワイヤレス
no-networks = ネットワークは見つけられませんでした。
no-vpn = VPN接続はありません。
password = パスワード
remove = 削除
settings = 設定
username = ユーザー名
visible-networks = ネットワークの一覧

auth-dialog = 認証は必要です。
    .vpn-description = VPNサービスにより必要とさせるユーザー名とパスワードを入力して下さい。
    .wifi-description = パスワードまたは暗号化キーを入力して下さい。 ルーターののWPSボタンを押すことでも接続できます。

forget-dialog = このWi-Fiネットワークを忘れてもよろしいですか？
    .description = 将来でこのWi-Fiネットワーク使用するために、もう一度パスワードを入力は必要です。

network-device-state =
    .activated = 接続済み
    .config = 接続中
    .deactivating = 切断中
    .disconnected = 接続されていません
    .failed = 接続失敗
    .ip-check = 接続を確認中
    .ip-config = IPとルーティング情報を要求中
    .need-auth = 認証が必要
    .prepare = 接続の準備中
    .secondaries = 二次の接続を待っています
    .unavailable = 無効
    .unknown = 不明状態
    .unmanaged = 管理外
    .unplugged = 抜けています

remove-connection-dialog = 接続プロファイルを削除してもよろしいですか？
    .vpn-description = 将来でこのネットワーク使用するために、もう一度パスワードを入力は必要です。
    .wired-description = 将来でこのプロファイルを使用するために、再作成することは必要です。

vpn = VPN
    .connections = VPN接続
    .remove = 接続プロファイルを削除する
    .select-file = VPNの構成ファイルを選択

wired = 有線
    .adapter = { $id }という有線のアダプター
    .connections = 有線の接続
    .devices = 有線のデバイス
    .remove = 接続プロファイルを削除する

wifi = Wi-Fi
    .adapter = { $id }というWi-Fiアダプター
    .forget = このネットワークを忘れる

## Networking: Online Accounts

online-accounts = オンラインアカウント
    .desc = アカウント、IMAPとSMTP、エンタプライズログインを追加

# Bluetooth

bluetooth = Bluetooth
    .desc = Bluetoothデバイスの操作
    .status = このデバイスはBluetooth設定が表示されながら{ $aliases }として見えます。
    .connected = 接続済み
    .connecting = 接続中
    .disconnecting = 切断中
    .connect = 接続する
    .disconnect = 切断する
    .forget = 忘れる
    .dbus-error = DBusと相互作用しながらエラーが発生しました： { $why }
    .show-device-without-name = 名前無しのデバイスを表示

bluetooth-paired = 前に接続したデバイス
    .connect = 接続
    .battery = バッテリーの{ $percentage }%

bluetooth-available = 隣のデバイス

bluetooth-adapters = Bluetoothアダプター


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
reset-to-default = デフォルトに戻す
rgb = RGB
window-hint-accent = アクティブなウィンドウのハイライトカラー
window-hint-accent-toggle = テーマのアクセントカラーをアクティブなウィンドウのヒントとして使う

auto-switch = 自動的にライトモードからダークモードに切り替える
    .sunrise = 日の出にライトモードに切り替えます
    .sunset = 日の入りにライトモードに切り替えます
    .next-sunrise = 次の日の出にライトモードに切り替えます
    .next-sunset = 次の日の入りにライトモードに切り替えます

container-background = コンテナー背景
    .desc-detail = サイドバー、ダイアログ、類似ウィジェットで使用されます。 規定では、アプリケーションまたはウィンドウの背景から自動的に選択されています。
    .reset = 自動的な選択に戻す
    .desc = サイドバー、ダイアログ、類似ウィジェットで使用されます。 規定では、アプリケーションまたはウィンドウの背景から自動的に選択されています。

control-tint = コントロールコンポーネントティント
    .desc = スタンダードボタン、検索入力、テキスト入力、類似コンポーネントで使用されます。

frosted = システムインターフェイスのすりガラス効果
    .desc = パネル、ドック、アプレット、ランチャー、アプリケーションライブラリの背景をぼかします。

experimental-settings = 試験的な設定

enable-export = このテーマをGNOMEアプリケーションに適用
    .desc = あるツールキットは自動的な切り替えをサポートしません。 テーマを切り替えると、非COSMICのアプリケーションは再起動が必要の可能性があります。

icon-theme = アイコンテーマ
    .desc = アプリケーションに他のテーマを適用します。

text-tint = インターフェイステキストヒント
    .desc = 色々な表面に十分なコントラストがあるインターフェイスのテキストの色ために使用されます。

style = スタイル
    .round = 丸い
    .slightly-round = 少し丸い
    .square = 四角い

interface-density = インタフェース密度
    .comfortable = 普通
    .compact = 狭い
    .spacious = 広い

window-management = ウィンドウマネジメント
    .active-hint = アクティブウィンドウヒントの大きさ
    .gaps = タイル型ウィンドウ

## Desktop: Display

-requires-restart = 再起動が必要です。

color = 色
    .depth = 色深度
    .profile = カラープロファイル
    .sidebar = カラープロファイル
    .temperature = 色温度

display = ディスプレイ
    .desc = ディスプレイ、グラフィックス切り替え、ナイトライトの操作
    .arrangement = ディスプレイの配列
    .arrangement-desc = ディスプレイを並べ替えるためにドラッグできます。
    .enable = ディスプレイを有効にする
    .external = { $size } { $output } 外付けディスプレイ
    .laptop = { $size }ラップトップディスプレイ
    .options = ディスプレイの設定
    .refresh-rate = リフレッシュレート
    .resolution = 解像度
    .scale = スケーリング

mirroring = ミラーリング
    .id = { $id }をミラーリングしています
    .dont = ミラーリングしない
    .mirror = { $display }をミラーリングする
    .project = { $display ->
        [all] 全てのディスプレイ
        *[other] { $display }
    }に投影する

    .project-count = 他の{ $count}つのディスプレイに投影しています

night-light = ナイトライト
    .auto = 自動的（日の入りから日の出まで）
    .desc = 暖色で青い光を減らします。

orientation = 画面の向き
    .standard = 回さない
    .rotate-90 = 90度回転
    .rotate-180 = 180度回転
    .rotate-270 = 270度回転

scheduling = 計画
    .manual = 手動計画

dialog = ダイヤログ
    .title = このディスプレイの設定を保存しますか？
    .keep-changes = 保存
    .change-prompt = { $time }秒後に設定は自動的に元に戻ります。
    .revert-settings = 元に戻す

legacy-app-scaling = X11アプリケーションのスケーリング
    .scaled-by-system = X11アプリを全てスケーリングする
    .system-description = HiDPI画面はX11アプリケーションがボケに見えます。
    .scaled-natively = X11アプリをネーティブの解像度で表現する
    .native-description = スケーリングをサポートしていないX11アプリケーションはHiDPI画面に小さく見えます。ゲームが完全に解像度を使用できるように有効にして下さい。

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

panel-behavior-and-position = 行動と位置
    .autohide = パネルを自動的にかくす
    .dock-autohide = ドックを自動的に隠す
    .position = 画面上の位置
    .display = ディスプレイに表示

panel-style = パネルスタイル
    .anchor-gap = パネルと画面の端の間の隙間
    .dock-anchor-gap = ドックと画面の端の間の隙間
    .extend = パネルを画面の端まで拡張
    .dock-extend = ドックを画面の端まで拡張
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
    .fit = 背景フィット
    .folder-dialog = 背景のフォルダを選択
    .image-dialog = 背景の画像を選択
    .plural = 背景
    .same = すべてのディスプレイで同じ背景を使う
    .slide = スライドショー

add-color = 色を追加
add-image = 画像を追加
all-displays = 全てのディスプレイ
colors = 色
dialog-add = 追加
fill = フィル
fit-to-screen = 画面に合わせる
open-new-folder = 新しいフォルダを開く
recent-folders = 最近のフォルダ

x-minutes = { $number }分
x-hours = { $number }時間

## Desktop: Workspaces

workspaces = ワークスペース
    .desc = ワークスペースの数、行動、位置の設定

workspaces-behavior = ワークスペースの動作
    .dynamic = ダイナミックワークスペース
    .dynamic-desc = 自動的に空のワークスペースを削除します。
    .fixed = ワークスペース数を固定
    .fixed-desc = オーバービューでワークスペース数を変えられます。

workspaces-multi-behavior = マルチモニターの動作
    .span = ワークスペースはディスプレイを超える
    .separate = ディスプレイはそれぞれのワークスペースがある

workspaces-overview-thumbnails = ワークスペースオーバービューサムネール
    .show-number = ワークスペース番号を表示
    .show-name = ワークスペース名を表示

workspaces-orientation = ワークスペース方向
    .vertical = 垂直
    .horizontal = 水平

## Time & Language

time = 日付と時刻
    .desc = N/A

time-date = 日付と時刻
    .desc = 時間帯、自動的なクロック設定、時刻形式
    .auto = 自動的に設定する

time-zone = 時間帯
    .auto = 自動的に時間帯を設定
    .auto-info = 位置情報サービスとインターネット接続が必要です

time-format = 日付と時刻
    .twenty-four = 24時間制
    .first = 週の初めの日
    .show-date = トップパネルに日付を表示
    .friday = 金
    .saturday = 土
    .sunday = 日
    .monday = 月

time-region = 地方と言語
    .desc = お住む地方によって日付、時刻、数値の形式を変える

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
    .desc = アプリケーションの音量と設定

## System

system = システムとアカウント

## System: About

about = このデバイスについて
    .desc = デバイス名、ハードウェアの情報、オペレーティングシステムのデフォルト。

about-device = デバイス名
    .desc = この名前は他のネットワークまたはBluetoothデバイスに表示されます。

about-hardware = ハードウェア
    .model = ハードウェアモデル
    .memory = メモリー
    .processor = プロセッサー
    .graphics = グラフィックス
    .disk-capacity = ディスク容量

about-os = オペレーティングシステム
    .os = オペレーティングシステム
    .os-architecture = オペレーティングシステムアーキテクチャ
    .desktop-environment = デスクトップ環境
    .windowing-system = ウィンドウシステム

about-related = 関係がある設定
    .support = サポートを受ける

## System: Firmware

firmware = ファームウェア
    .desc = ファームウェアの情報。

## System: Users

users = ユーザー
    .desc = 認証とログイン、ロック画面。

## Input

acceleration-desc = 速度に基づいて自動的にトラッキング感動を調節します。

disable-while-typing = 入力中に無効にする

input-devices = 入力デバイス
    .desc = 入力デバイス

primary-button = 一次内ボタン
    .desc = 物理ボタンの順を設定します。
    .left = 左
    .right = 右

scrolling = スクロール
    .two-finger = 指二本でスクロール
    .edge = 指一本で橋に沿ってスクロール
    .speed = スクロールの速度
    .natural = 自然なスクロール
    .natural-desc = 表示の代わりに内容をスクロール

## Input: Keyboard

slow = 遅い
fast = 速い
short = 短い
long = 長い
keyboard = キーボード
    .desc = 入力のソース、切り替え、特殊文字の入り、ショートカット。

keyboard-sources = 入力ソース
    .desc = 入力ソースはスーパー+スペースキーコンボで切り替えられます。これをキーボードショットカットの設定で構成できます。
    .move-up = 上に移動
    .move-down = 下に移動
    .settings = 設定
    .view-layout = キーボードレイアウトを表示
    .remove = 削除
    .add = 入力ソースを追加

keyboard-special-char = 特殊文字の入り
    .alternate = 代替文字キー
    .compose = コムポーズキー

keyboard-typing-assist = タイピング
    .repeat-rate = リピートレート
    .repeat-delay = リピート遅延

added = 追加された
type-to-search = 入力して検索...
show-extended-input-sources = 拡張の入力ソースを表示

## Input: Keyboard: Shortcuts

keyboard-shortcuts = キーボードショットカット
    .desc = キーボードショットカットの表示と構成

## Input: Mouse

mouse = マウス
    .desc = マウスの速度、アクセラレーション、自然なスクロールリング
    .speed = マウスの速度
    .acceleration = マウスアクセラレーションを有効にする

## Input: Touchpad

click-behavior = クリックの行動
    .click-finger = 指二本で二次クリックと指三本で中クリック
    .button-areas = 右下で二次クリックと下中央で中クリック

pinch-to-zoom = ピンチでズーム
    .desc = 指二本のピンチで内容をズーム（サポートされた場合）

tap-to-click = タップでクリック
    .desc = 指一本で一次クリック、指二本で二次クリック、指三本で中クリックを有効にします。

touchpad = タッチパッド
    .acceleration = タッチパッドの加速を有効にする
    .desc = タッチパッドの速度、クリックの設定、ジェスチャー。
    .speed = タッチパッドの速度

## Input: Gestures

swiping = スワイプ
    .four-finger-down = 指四本で下向きのスワイプ
    .four-finger-left = 指四本で左向きのスワイプ
    .four-finger-right = 指四本で右向きのスワイプ
    .four-finger-up = 指四本で上向きのスワイプ
    .three-finger-any = 指三本で好きな方向のスワイプ

switch-between-windows = ウィンドウを切り替える
switch-to-next-workspace = 次のワークスペースに切り替える
switch-to-prev-workspace = 前のワークスペースに切り替える
open-application-library = アプリケーションライブラリを開く
open-workspaces-view = ワークスペースのか概要を開く

## Power

power = 電源とバッテリー
  .desc = 電源設定を管理

battery = Battery
    .minute = { $value }分
    .hour = { $value }時間
    .day = { $value }日
    .less-than-minute = 一分以下
    .remaining-time = { $action ->
        [full] 充電完了
       *[other] 電池切れ
    }まで{ $time }

connected-devices = 接続されたデバイス
  .unknown = 不明なデバイス

power-mode = 電源モード
  .performance = ハイパフォーマンス
  .balanced = バランス
  .battery = 長いバッテリー寿命
  .performance-desc = 一番高い性能と電力消費量。
  .balanced-desc = 静かな性能と程度な電力消費量。
  .battery-desc = 低い電力消費量と静粛な性能。
  .no-backend = バックエンドは見つかれませんでした。 system76-powerまたはpower-profiles-daemonをインストールして下さい。
