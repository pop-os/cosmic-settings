app = COSMIC 控制中心

unknown = 未知

number = { $number }

## Desktop

desktop = 桌面

## Desktop: Appearance

appearance = 外觀
    .desc = 介面主題與色調

mode-and-colors = 色彩與模式
auto-switch = 自動切換至夜色模式
    .desc = 在晚上時自動切換至夜色模式
accent-color = 強調色
app-background = 應用程式背景
container-background = 容器背景
    .desc-detail = 容器背景與導航列、側邊欄、對話方塊等元件相關，預設從應用程式或視窗背景進行衍生
    .reset = 重設至自動
    .desc = 容器背景色與導航列、側邊欄、對話方塊等元件相關
text-tint = 介面文字色調
    .desc = 在各種背景中衍生出足夠對比色的顏色
control-tint = 控制元件色調
    .desc = 按鈕、搜尋欄、文字輸入或相似元件的背景色
window-hint-accent = 使用主題強調色作為使用中視窗的提示色
dark = 夜色
light = 明亮色
color-picker = 色盤
hex = Hex
rgb = RGB
recent-colors = 近似色
reset-to-default = 重設至默認值
copy-to-clipboard = 複製到剪貼簿
copied-to-clipboard = 已複製到剪貼簿
style = 風格
    .round = 圓角
    .slightly-round = 半圓角
    .square = 正角
frosted = 系統介面呈現磨砂玻璃的透明效果
    .desc = 將磨砂玻璃的透明效果套用至面板、容器、Dock、啟動器及程式庫
reset-default = 重設至預設值
# interface density left out for now

window-management = 視窗管理
    .active-hint = 使用中視窗提示尺寸
    .gaps = 平鋪視窗模式下的間隔距離

## Desktop: Notifications

notifications = 通知
    .desc = 勿擾模式，鎖定畫面通知以及個別應用程式設定


## Desktop: Options

desktop-panel-options = 桌面與面板
    .desc = Super 按鍵行為, 螢幕角落熱點, 視窗控制選項

super-key-action = Super 按鍵行為
    .launcher = 啟動器
    .workspaces = 工作區
    .applications = 應用程式

hot-corner = 螢幕角落熱點
    .top-left-corner = 為工作區啟用位於左上方的螢幕角落熱點

top-panel = 頂部面板
    .workspaces = 顯示工作區按鈕
    .applications = 顯示應用程式按鈕

window-controls = 視窗控制
    .minimize = 顯示最小話按鈕
    .maximize = 顯示最大話按鈕

desktop-panels-and-applets = 桌面面板與工具程式


dock = Dock
    .desc = 用於釘選常用應用程式的面板

## Desktop: Panel
panel = 面板
    .desc = 頂部條狀面板用於提供桌面控制和選單

panel-behavior-and-position = 呈現方式與位置
    .autohide = 自動隱藏面板
    .dock-autohide = 自動隱藏 Dock
    .position = 螢幕上的位置
    .display = 顯示於顯示器

panel-top = 上
panel-bottom = 下
panel-left = 左
panel-right = 右

panel-appearance = 外觀
    .match = 配合桌面
    .light = 亮
    .dark = 暗

panel-style = 樣式
    .anchor-gap = 面板與螢幕邊緣的間隙
    .dock-anchor-gap = Dock 與螢幕邊緣的間隙
    .extend = 面板延伸至螢幕邊緣
    .dock-extend = Dock 延伸至螢幕邊緣
    .appearance = 外觀
    .size = 尺寸
    .background-opacity = 背景透明度

small = 小
large = 大

panel-applets = 設定
    .dock-desc = 設定 Dock 工具
    .desc = 設定面板工具

panel-missing = 面板設定遺失
    .desc = 由於使用了自定義設定或設定檔毀損以致面板設定檔遺失
    .fix = 重設至預設值

applets = 工具程式
start-segment = 起始位置
center-segment = 中間位置
end-segment = 末尾位置

add = 新增
add-applet = 新增工具程式
search-applets = 搜尋工具程式...
no-applets-found = 找不到工具程式...
all = 全部

drop-here = 將工具程式放到此處

## Desktop: Wallpaper

wallpaper = 桌布
    .desc = 背景圖片、顏色和幻燈片選項
    .same = 所有螢幕使用相同的背景
    .fit = 延伸背景
    .slide = 幻燈片
    .change = 圖片切換

all-displays = 所有顯示器
colors = 顏色
fit-to-screen = 延伸至全螢幕
stretch = 延伸
zoom = 縮放

x-minutes = { $number } 分鐘
x-hours = { $number ->
    [1] 1 小時
    *[other] { $number } 小時
}

## Desktop: Workspaces

workspaces = 工作區
    .desc = 設定工作區編號、行為、擺放位置

workspaces-behavior = 工作區行為
    .dynamic = 動態工作區數量
    .fixed = 固定工作區數量

workspaces-multi-behavior = 多顯示器行為
    .span = 工作區橫跨顯示器
    .separate = 顯示器各自擁有工作區

## Networking: Wired

wired = 網路
    .desc = 網路連線與設定檔

## Networking: Online Accounts

online-accounts = 線上帳號
    .desc = 新增帳號、郵件（IMAP、SMTP）或企業登入

## Time & Language

time = 時間和語言
    .desc = N/A

time-date = 日期和時間
    .desc = 時區、自動時間校正、時間格式設定
    .auto = 自動設定

time-zone = 時區
    .auto = 自動設定時區
    .auto-info = 需要定位服務和網路存取能力

time-format = 日期和時間格式
    .twenty-four = 24 小時制
    .first = 每週的第一天

time-region = 地區和語言
    .desc = 基於地區來格式化日期、時間及數字

## Sound

sound = 音效
    .desc = N/A

sound-output = 輸出
    .volume = 音量
    .device = 輸出裝置
    .level = 音量大小
    .config = 設定
    .balance = 設定

sound-input = 輸入
    .volume = 音量
    .device = 輸入裝置
    .level = 音量大小

sound-alerts = 警示音
    .volume = 警示音量
    .sound = 警示聲音

sound-applications = 程式音效
    .desc = 程式音效設定

## System

system = 系統與帳戶

## System: About

about = 關於
    .desc = 裝置名稱、硬體資訊、作業系統

about-device = 裝置名稱
    .desc = 此名稱用於顯示給其他網路或藍芽裝置

about-hardware = 硬體
    .model = 硬體型號
    .memory = 記憶體
    .processor = 處理器
    .graphics = 顯示卡
    .disk-capacity = 硬碟空間

about-os = 作業系統
    .os = 系統
    .os-architecture = 系統架構
    .desktop-environment = 桌面環境
    .windowing-system = 視窗系統

about-related = 相關設定
    .support = 取得幫助

## System: Firmware

firmware = 韌體
    .desc = 韌體資訊

## System: Users

users = 使用者
    .desc = 驗證、登入、畫面鎖定

## Input

acceleration-desc = 自動依照觸控板移動加速度即時變更滑鼠速度

primary-button = 主要按鈕
    .left = 左
    .right = 右

scrolling = 捲動
    .speed = 捲動速度
    .natural = 自然捲動
    .natural-desc = 捲動內容而非視界

## Input: Keyboard

keyboard = 鍵盤
    .desc = 鍵盤輸入

keyboard-sources = 輸入法
    .desc = 輸入法可使用 Super+Space 按鍵組合進行切換，此項設定可以在鍵盤快捷鍵設定頁面進行修改
    .move-up = 往上
    .move-down = 往下
    .settings = 設定
    .view-layout = 檢視鍵盤布局
    .remove = 移除

keyboard-special-char = 特殊輸入
    .alternate = 進階按鍵
    .compose = 組合鍵

## Input: Keyboard: Shortcuts

keyboard-shortcuts = 鍵盤快捷鍵
    .desc = 顯示與修改快捷鍵

## Input: Mouse

mouse = 滑鼠
    .desc = 滑鼠靈敏度、加速及捲動設定
    .speed = 滑鼠速度
    .acceleration = 啟用滑鼠加速

## Input: Touchpad

touchpad = 觸控板
    .desc = 觸控靈敏度、點擊和手勢
    .speed = 觸控靈敏度
    .acceleration = 啟用觸控板加速