app = COSMIC設定
unknown = 不明

## Network & Wireless

connections-and-profiles =
    { $variant ->
        [wired] 有線の
        [wifi] Wi-Fiの
        [vpn] VPNの
       *[other] 不明な
    }接続と接続プロファイル。
add-network = ネットワークを追加
    .profile = プロファイルを追加
add-vpn = VPNを追加
airplane-on = 機内モードは有効です。
cable-unplugged = ケーブルは抜けています
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
forget-dialog = このWi-Fiネットワークを削除しますか？
    .description = 今後このWi-Fiネットワークを使用するには、再度パスワードを入力する必要があります。
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
    .error = VPN設定の追加失敗
    .remove = 接続プロファイルを削除
    .select-file = VPNの設定ファイルを選択
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
    .disabled = ブルートゥースは無効です
    .inactive = ブルートゥースは非アクティブです
    .unknown = ブルートゥースは有効にできません。BlueZはインストールされていますか？
bluetooth-paired = 前に接続したデバイス
    .connect = 接続
    .battery = バッテリーの{ $percentage }%
bluetooth-available = 隣のデバイス
bluetooth-adapters = Bluetoothアダプター

## Desktop

desktop = デスクトップ

## Desktop: 外観

appearance = 外観
    .desc = アクセントカラーとCOSMICのテーマ。
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
    .desc = スーパーキーの動作、ウィンドウコントロールのオプション、追加のウィンドウタイリングのオプション。

## Desktop: Display

-requires-restart = 再起動が必要です
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
    .additional-scale-options = 追加のスケーリングオプション
mirroring = ミラーリング
    .id = ミラーリング { $id }
    .dont = ミラーリングしない
    .mirror = { $display }をミラーリング
    .project =
        { $display ->
            [all] すべてのディスプレイに投影
           *[other] { $display }に投影
        }
    .project-count =
        { $count }台の{ $count ->
            [1] ディスプレイ
           *[other] ディスプレイ
        }に投影中
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
legacy-app-scaling = X11 ウィンドウシステム アプリケーションのスケーリング
    .scaled-gaming = ゲームとフルスクリーンアプリに最適化
    .gaming-description = X11 アプリケーションは、Wayland アプリと比較してわずかに大きく/小さく表示される場合があります。
    .scaled-applications = 一般的なアプリケーションに最適化
    .applications-description = ゲームやフルスクリーン X11 アプリは、ディスプレイ解像度に一致しない場合があります。
    .scaled-compatibility = 最大限の互換性モード
    .compatibility-description = X11 アプリケーションは、HiDPI 画面でぼやけて表示される場合があります。
    .preferred-display = ゲームとフルスクリーン X11 アプリケーションの優先ディスプレイ
    .no-display = なし

## Desktop: Notifications

notifications = 通知
    .desc = 通知ポップアップの表示、ロック画面の通知、アプリケーションごとの設定。

## Desktop: Options

dock = ドック
    .desc = 固定されたアプリケーションありパネル。
hot-corner = ホットコーナー
    .top-left-corner = ワークスペースための左上のホットコーナーを有効にする
window-controls = ウィンドウコントロール
    .maximize = 最大化ボタンを表示
    .minimize = 最小化ボタンを表示
    .active-window-hint = アクティブウィンドウヒントを表示

## Desktop: Panel

panel = パネル
    .desc = デスクトップコントロールとメニューありトップパネル。
add = 追加
add-applet = アプレットを追加
all = すべて
applets = アプレット
center-segment = 中心の部分
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
x-minutes =
    { $number } { $number ->
        [one] 分
       *[other] 分
    }
x-hours =
    { $number } { $number ->
        [one] 時間
       *[other] 時間
    }

## Desktop: Workspaces

workspaces = ワークスペース
    .desc = ワークスペースの数、行動、位置の設定。
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
    .auto = 自動設定
    .auto-ntp = タイムゾーンが設定時に日付と時刻を自動的に更新する。
time-zone = 時間帯
    .auto = 自動的に時間帯を設定
    .auto-info = 位置情報サービスとインターネット接続が必要です
time-format = 日付と時刻
    .twenty-four = 24時間制
    .show-seconds = 秒を表示
    .first = 週の初めの日
    .show-date = トップパネルに日付を表示
    .friday = 金
    .saturday = 土
    .sunday = 日
    .monday = 月
time-region = 地方と言語
    .desc = お住む地方によって日付、時刻、数値の形式を変える。

## Sound

sound = サウンド
    .desc = N/A
sound-output = 出力
    .volume = 出力音量
    .device = 出力デバイス
    .level = 出力レベル
    .config = 構成
    .balance = バランス.
    .left = 左
    .right = 右
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

users = Users
    .desc = 認証とユーザーアカウント。
    .admin = 管理者
    .standard = 標準ユーザー
    .profile-add = プロフィール画像選択

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
    .alternate = Altキー
    .compose = Composeキー
    .caps = Caps Lockキー
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
    .desc = 指二本のピンチで内容をズーム（サポートされた場合）。
tap-to-click = タップでクリック
    .desc = 指一本で一次クリック、指二本で二次クリック、指三本で中クリックを有効にします。
touchpad = タッチパッド
    .acceleration = タッチパッドの加速を有効にする
    .desc = タッチパッドの速度、クリックの設定、ジェスチャー。
    .speed = タッチパッドの速度

## Input: Gestures

switch-between-windows = ウィンドウを切り替える
open-application-library = アプリケーションライブラリを開く
open-workspaces-view = ワークスペースのか概要を開く

## Power

power = 電源とバッテリー
    .desc = 電源設定を管理
battery = バッテリー
    .minute =
        { $value } { $value ->
            [one] 分
           *[other] 分
        }
    .hour =
        { $value } { $value ->
            [one] 分
           *[other] 分
        }
    .day =
        { $value } { $value ->
            [one] 日
           *[other] 日
        }
    .less-than-minute = 1分未満
    .and = と
    .remaining-time =
        { $time }まで{ $action ->
            [full] 充電完了
           *[other] 電池切れ
        }
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
cancel = キャンセル
dbus-connection-error = DBus接続エラー
ok = OK
password-confirm = パスワードの確認
default = デフォルト
interface-font = システムフォント
monospace-font = 等幅フォント
command = コマンド
custom = カスタム
debug = デバッグ
replace = 置換
shortcut-name = ショートカット名
zoom-in = ズームイン
zoom-out = ズームアウト
install-additional-languages = 追加の言語をインストール
region = リージョン
applications = アプリケーション
add-user = ユーザー追加
change-password = パスワード変更
remove-user = ユーザー削除
full-name = フルネーム
invalid-username = 無効なユーザー名です。
password-mismatch = パスワードが一致しません。
save = 保存
identity = ID
wireguard-dialog = WireGuardデバイスの追加
    .description = WireGuard設定の選択。
activate = アクティベート
confirm = 確認する
enable = 有効にする
accessibility = アクセシビリティ
    .vision = 視覚
    .on = オン
    .off = オフ
    .unavailable = 利用不可
    .screen-reader = スクリーンリーダー
    .high-contrast = ハイコントラスト
    .invert-colors = 反転
    .color-filters = カラーフィルター
hearing = 聴く
    .mono = モノラルオーディオ
super-key = スーパーキーの動作
    .launcher = ランチャーを開く
    .workspaces = ワークスペースを開く
    .applications = アプリケーションを開く
    .disable = 無効
vrr = 可変リフレッシュレート
    .enabled = 有効
    .force = 強制
    .auto = 自動
    .disabled = 無効
amplification = 増幅
    .desc = ボリュームを150%まで上げることを許可する。
power-saving = パワーセーブオプション
    .turn-off-screen-after = 画面をオフにする
    .auto-suspend = 自動サスペンド
    .auto-suspend-ac = コンセント接続時の自動サスペンド
    .auto-suspend-battery = 低バッテリー時の自動サスペンド
add-another-keybinding = 他のキーバインドの追加
disabled = 無効
navigate = ナビゲート
terminate = 終了
toggle-stacking = ウィンドウスタッキングの切り替え
modified = { $count } 変更済み
add-language = 言語の追加
    .context = 言語の追加
administrator = 管理者
    .desc = 管理者は全てのユーザー設定を変更でき、ユーザーの追加と削除が出来ます。
vpn-error = VPNエラー
    .config = VPN設定の追加に失敗しました
    .connect = VPNへの接続に失敗しました
    .connection-editor = 接続エディタに失敗しました
    .connection-settings = アクティブな接続の設定取得に失敗しました
    .updating-state = ネットワークマネージャの状態更新に失敗しました
    .wireguard-config-path = WireGuard設定ファイルのパスが無効です
    .wireguard-config-path-desc = 選択したファイルはローカルファイルシステム上にある必要があります。
    .wireguard-device = WireGuardデバイスの作成に失敗しました
    .with-password =
        nmcliによるVPNの{ $field ->
           *[username] ユーザー名
            [password] パスワード
            [password-flags] パスワードフラグ
        }設定に失敗しました
bluetooth-confirm-pin = Bluetooth PINの確認
    .description = { $device }に表示されているPINと一致していることを確認してください
magnifier = 拡大鏡
    .controls =
        または、次のショートカットを使用します: { $zoom_in ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_in }で拡大、
        }{ $zoom_out ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_out }で縮小、
        }
        マウスでSuper + スクロール
    .scroll_controls = マウスまたはタッチパッドでSuper + スクロールを有効にする
    .show_overlay = 拡大鏡オーバーレイを表示
    .increment = ズームの増加量
    .signin = サインイン時に拡大鏡を開始
    .applet = パネルのアプレットで拡大鏡のオン/オフを切り替えます
    .movement = 拡大表示の移動
    .continuous = ポインターに合わせて継続的に
    .onedge = ポインターが端に達したとき
    .centered = ポインターを中心に保つ
color-filter = カラーフィルターのタイプ
    .unknown = 不明なフィルターがアクティブです
    .greyscale = グレースケール
    .deuteranopia = 緑/赤（緑色弱、Deuteranopia）
    .protanopia = 赤/緑（赤色弱、Protanopia）
    .tritanopia = 青/黄（青色弱、Tritanopia）
never = しない
window-management-appearance = ウィンドウ管理
    .active-hint = アクティブウィンドウのヒントのサイズ
    .gaps = タイル化されたウィンドウの周囲の隙間
icons-and-toolkit = アイコンとツールキットのテーマ設定
edge-gravity = フローティングウィンドウが近くの端に引き寄せられる
focus-navigation = フォーカスナビゲーション
    .focus-follows-cursor = フォーカスがカーソルに追従
    .focus-follows-cursor-delay = カーソル追従の遅延（ミリ秒）
    .cursor-follows-focus = カーソルがフォーカスに追従
keyboard-numlock-boot = Numlock
    .boot-state = 起動時の状態
    .last-boot = 最終起動
    .on = オン
    .off = オフ
    .set = Numlockの起動時の状態を設定
input-source-switch = キーボードの言語入力ソースを切り替えます
migrate-workspace-prev = ワークスペースを前の出力に移動
migrate-workspace-next = ワークスペースを次の出力に移動
migrate-workspace =
    ワークスペースを{ $direction ->
       *[down] 下
        [left] 左
        [right] 右
        [up] 上
    }の出力に移動
system-controls = システム操作
type-key-combination = キーの組み合わせを入力
custom-shortcuts = スタムショートカット
    .add = ショートカットを追加
    .context = カスタムショートカットを追加
    .none = カスタムショートカットはありません
nav-shortcuts = ナビゲーション
    .prev-output = 前の出力にフォーカス
    .next-output = 次の出力にフォーカス
    .last-workspace = 最後のワークスペースにフォーカス
    .prev-workspace = 前のワークスペースにフォーカス
    .next-workspace = 次のワークスペースにフォーカス
    .focus =
        ウィンドウにフォーカスを{ $direction ->
           *[down] 下
            [in] 内側
            [left] 左
            [out] 外側
            [right] 右
            [up] 上
        }に移動
    .output =
        出力を{ $direction ->
           *[down] 下
            [left] 左
            [right] 右
            [up] 上
        }に切り替え
    .workspace = ワークスペース { $num }に切り替え
manage-windows = ウィンドウの管理
    .close = ウィンドウを閉じる
    .maximize = ウィンドウを最大化
    .fullscreen = ウィンドウを全画面表示
    .minimize = ウィンドウを最小化
    .resize-inwards = ウィンドウを内側にリサイズ
    .resize-outwards = ウィンドウを外側にリサイズ
    .toggle-sticky = ウィンドウをスティッキーに切り替え
move-windows = ウィンドウの移動
    .direction =
        ウィンドウを{ $direction ->
           *[down] 下
            [left] 左
            [right] 右
            [up] 上
        }に移動
    .display =
        ウィンドウをモニター{ $direction ->
           *[down] 下
            [left] 左
            [right] 右
            [up] 上
        }に移動
    .workspace =
        ウィンドウをワークスペース{ $direction ->
           *[below] 下
            [left] 左
            [right] 右
            [above] 上
        }に移動
    .workspace-num = ウィンドウをワークスペース { $num }へ移動
    .prev-workspace = ウィンドウを前のワークスペースへ移動
    .next-workspace = ウィンドウを次のワークスペースへ移動
    .last-workspace = ウィンドウを最後のワークスペースへ移動
    .next-display = ウィンドウを次のディスプレイへ移動
    .prev-display = ウィンドウを前のディスプレイへ移動
    .send-to-prev-workspace = ウィンドウを前のワークスペースへ移動
    .send-to-next-workspace = ウィンドウを次のワークスペースへ移動
system-shortcut = システム
    .app-library = アプリケーションライブラリを開く
    .brightness-down = ディスプレイの輝度を下げる
    .brightness-up = ディスプレイの輝度を上げる
    .display-toggle = 内蔵ディスプレイを切り替え
    .home-folder = ホームフォルダを開く
    .keyboard-brightness-down = キーボードの輝度を下げる
    .keyboard-brightness-up = キーボードの輝度を上げる
    .launcher = ランチャーを開く
    .log-out = ログアウト
    .lock-screen = 画面をロック
    .mute = 音声出力をミュート
    .mute-mic = マイク入力をミュート
    .play-pause = 再生/一時停止
    .play-next = 次のトラック
    .play-prev = 前のトラック
    .poweroff = 電源オフ
    .screenshot = スクリーンショットを撮る
    .terminal = ターミナルを開く
    .touchpad-toggle = タッチパッドを切り替え
    .volume-lower = 音声出力の音量を下げる
    .volume-raise = 音声出力の音量を上げる
    .web-browser = ウェブブラウザを開く
    .window-switcher = 開いているウィンドウ間を切り替える
    .window-switcher-previous = 開いているウィンドウ間を逆順に切り替える
    .workspace-overview = ワークスペースの概要を開く
window-tiling = ウィンドウタイリング
    .horizontal = 水平方向を設定
    .vertical = 垂直方向を設定
    .swap-window = ウィンドウを入れ替える
    .toggle-tiling = ウィンドウタイリングを切り替える
    .toggle-stacking = ウィンドウスタッキングを切り替える
    .toggle-floating = ウィンドウフローティングを切り替える
    .toggle-orientation = 方向を切り替える
replace-shortcut-dialog = ショートカットを置き換えますか？
    .desc = { $shortcut }は{ $name }によって使用されています。置き換えた場合、{ $name }は無効になります。
gestures = ジェスチャー
    .four-finger-down = 4本指で下にスワイプ
    .four-finger-left = 4本指で左にスワイプ
    .four-finger-right = 4本指で右にスワイプ
    .four-finger-up = 4本指で上にスワイプ
    .three-finger-any = 3本指で任意の方向にスワイプ
switch-workspaces = ワークスペースを切り替え
    .horizontal = 4本指で左/右にスワイプ
    .vertical = 4本指で上/下にスワイプ
formatting = 書式設定
    .dates = 日付
    .time = 時刻
    .date-and-time = 日付と時刻
    .numbers = 数値
    .measurement = 測定単位
    .paper = 用紙
preferred-languages = 優先する言語
    .desc = 言語の順序によってユーザーインターフェースで使用される言語が決まります。変更は次回のログイン時に適用されます。
default-apps = 標準アプリケーション
    .desc = 標準のウェブブラウザ、メールクライアント、ファイルマネージャ、その他のアプリケーション。
    .web-browser = ウェブブラウザ
    .file-manager = ファイルマネージャ
    .mail-client = メールクライアント
    .music = 音楽
    .video = ビデオ
    .photos = 写真
    .calendar = カレンダー
    .terminal = ターミナル
    .other-associations = その他の関連付け
    .text-editor = テキストエディタ
startup-apps = スタートアップアプリケーション
    .desc = ログイン時に実行されるアプリケーションを設定します。
    .add = アプリを追加
    .user = ログイン時に起動されるアプリケーション
    .none = スタートアップアプリケーションは追加されていません
    .remove-dialog-title = { $name }を削除しますか？
    .remove-dialog-description = このスタートアップアプリケーションを削除してもよろしいですか？
    .search-for-application = アプリケーションを検索
legacy-applications = X11アプリケーション互換性
    .desc = X11ウィンドウシステムのアプリケーションスケーリングとグローバルショートカット。
legacy-app-global-shortcuts = X11アプリケーションでのグローバルショートカット
    .desc = グローバルショートカットにより、アプリケーション内で実行されたキーストロークやマウスボタンイベントを、プッシュツートークやプッシュツーミュートなどの機能のために、他のアプリケーションが認識できるようになります。デフォルトでは、機密情報を含むキーボードおよびマウスイベントを他のアプリケーションが監視できないようにするため、X11アプリケーションではこの機能は無効になっています。
    .none = キーなし
    .modifiers = 修飾キー (Super、Shift、Control、Alt)
    .combination = 修飾キー（Super、Control、Alt）が押されている間のすべてのキー
    .all = すべてのキー
    .mouse = X11アプリケーションでのマウスボタンイベント
number = { $number }
