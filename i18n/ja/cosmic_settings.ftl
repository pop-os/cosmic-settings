app = COSMIC設定

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
reset-to-default = デフォルトに戻す
rgb = RGB
window-hint-accent = アクティブなウィンドウのハイライトカラー
window-hint-accent-toggle = テーマのアクセントカラーをアクティブなウィンドウのヒントとして使う

auto-switch = 自動的にライトモードからダークモードに切り替える
    .desc = 日の出になるとライトモードに切り替えます。

container-background = コンテナー背景
    .desc-detail = サイドバー、ダイアログ、類似ウィジェットで使用されます。 規定では、アプリケーションまたはウィンドウの背景から自動的に選択されています。
    .reset = 自動的な選択に戻す
    .desc = サイドバー、ダイアログ、類似ウィジェットで使用されます。 規定では、アプリケーションまたはウィンドウの背景から自動的に選択されています。

control-tint = コントロールコンポーネントティント
    .desc = スタンダードボタン、検索入力、テキスト入力、類似コンポーネントで使用されます。

frosted = システムインターフェイスのすりガラス効果
    .desc = パネル、ドック、アプレット、ランチャー、アプリケーションライブラリの背景をぼかします。

text-tint = インターフェイステキストヒント
    .desc = 色々な表面に十分なコントラストがあるインターフェイスのテキストの色ために使用されます。

style = スタイル
    .round = 丸い
    .slightly-round = 少し丸い
    .square = 四角い

# interface density left out for now
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

    .project-count = 他の{ $count}つのディスプレイに投影している

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
    .desc = 時間帯、自動的なクロック設定、時刻形式
    .auto = 自動的に設定する

time-zone = 時間帯
    .auto = 自動的に時間帯を設定
    .auto-info = 位置情報サービスとインターネット接続が必要です
    
time-format = 日付と時刻
    .twenty-four = 24時間制
    .first = 週の初めの日

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
    .support = サポート

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
    .left = 左
    .right = 右

scrolling = スクロール
    .two-finger = 指二本でスクロール
    .edge = 指一本で橋に沿ってスクロール
    .speed = スクロールの速度
    .natural = 自然なスクロール
    .natural-desc = 表示の代わりに内容をスクロール

## Input: Keyboard

keyboard = キーボード
    .desc = キーボードの入力

keyboard-sources = 入力ソース
    .desc = 入力ソースはスーパー+スペースキーコンボで切り替えられます。これをキーボードショットカットの設定で構成できます。
    .move-up = 上に移動
    .move-down = 下に移動
    .settings = 設定
    .view-layout = キーボードレイアウトを表示
    .remove = 削除

keyboard-special-char = 特殊文字の入力
    .alternate = 代替文字キー
    .compose = コムポーズキー

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
