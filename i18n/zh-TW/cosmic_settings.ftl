app = COSMIC 控制中心

unknown = 未知

number = { $number }

## Desktop

desktop = 桌面

## Desktop: Appearance

appearance = 外觀
    .desc = 介面主題與色調

accent-color = 強調色
app-background = 應用程式背景
auto = 自動
close = 關閉
color-picker = 調色盤
copy-to-clipboard = 複製到剪貼簿
copied-to-clipboard = 已複製到剪貼簿
dark = 夜色
export = 匯出
hex = Hex
import = 匯入
light = 明亮色
mode-and-colors = 色彩與模式
recent-colors = 近似色
reset-to-default = 重設至預設值
rgb = RGB
window-hint-accent = 使用中視窗的提示色
window-hint-accent-toggle = 使用主題強調色作為使用中視窗的提示色

auto-switch = 自動切換至夜色模式
    .sunrise = 清晨時切換為亮色調
    .sunset = 黃昏時切換為夜色調
    .next-sunrise = 下次清晨切換為亮色調
    .next-sunset = 下次黃昏切換為亮色調

container-background = 容器背景
    .desc-detail = 容器背景與導航列、側邊欄、對話方塊等元件相關，預設從應用程式或視窗背景進行衍生
    .reset = 重設至自動
    .desc = 容器背景色與導航列、側邊欄、對話方塊等元件相關

control-tint = 控制元件色調
    .desc = 標準按鈕、搜尋輸入、文字輸入和類似元件的背景

frosted = 系統介面呈現磨砂玻璃的透明效果
    .desc = 將磨砂玻璃的透明效果套用至面板、容器、Dock、工具程式、啟動器及程式庫

experimental-settings = 實驗性功能

enable-export = 將此主題套用於 GNOME 應用程式
    .desc = 並非所有圖形介面程式庫都支援自動切換，非 Cosmic 原生圖形介面程式庫所開發的程式可能需要在重啟啟動後才會套用新的主題

icon-theme = 圖示主題
    .desc = 為應用程式選用不同的圖示集合

text-tint = 介面文字色調
    .desc = 在各種背景中衍生出足夠對比色的顏色

style = 風格
    .round = 圓角
    .slightly-round = 半圓角
    .square = 正角

# interface density left out for now
window-management = 視窗管理
    .active-hint = 使用中視窗提示尺寸
    .gaps = 平鋪視窗模式下的間隔距離

## Desktop: Notifications

-requires-restart = 需要重啟

color = 顏色
    .depth = 色彩深度
    .profile = 色彩特性
    .sidebar = 色彩特性
    .temperature = 色彩溫度

display = 螢幕
    .desc = 管理螢幕、影像切換及夜色模式
    .arrangement = 螢幕排列
    .arrangement-desc = 拖曳螢幕來更改排列方式
    .enable = 啟用螢幕
    .external = { $size } { $output } 外接螢幕
    .laptop = { $size } 筆記型螢幕
    .options = 螢幕選項
    .refresh-rate = 螢幕更新率
    .resolution = 解析度
    .scale = 縮放比例

graphics-mode = 顯卡運作模式
    .mode = { $mode ->
        [compute] 高效能模式
        *[hybrid] 混合模式
        [integrated] 省電模式
        [nvidia] NVIDIA 模式
    } graphics
    .enable = Enable { $mode ->
        [compute] 高效能模式
        *[hybrid] 混合模式
        [integrated] 省電模式
        [nvidia] NVIDIA 模式
    } graphics
    .desc = { $mode ->
        [compute] 只啟用獨立顯示卡獲得最好的運算效能，可能會關閉外接螢幕。{ -requires-restart }.
        *[hybrid] 應用程式預設使用整合式顯示卡，除非程式主動要求使用獨立顯示卡。{ -requires-restart }.
        [integrated] 關閉獨立顯示卡，只啟用整合式顯示卡以獲得更好的省電效果及更低的風扇噪音。
        [nvidia] Nvidia 顯示卡將消耗更多的電源來獲得更好的運算與顯示體驗。 { -requires-restart }.
    }
    .restart = 重啟並切換至 { $mode }?
    .restart-desc = 切換至 { $mode } 將關閉全部的應用程式

mirroring = 投影
    .id = 正在投影 { $id }
    .dont = 不要投影
    .mirror = 投影 { $display }
    .project = 投影至 { $display ->
        [all] 全部的螢幕
        *[other] { $display }
    }
    .project-count = 投影了 { $count} 個 { $count ->
        [1] 螢幕
        *[other] 螢幕
    }

night-light = 夜間模式
    .auto = 自動 (黃昏後到清晨前期間啟用)
    .desc = 以暖色調來減少藍光

orientation = 螢幕方向
    .standard = 橫式
    .rotate-90 = 旋轉 90
    .rotate-180 = 旋轉 180
    .rotate-270 = 旋轉 270

scheduling = 排程
    .manual = 手動排程

## Desktop: Notifications

notifications = 通知
    .desc = 勿擾模式，鎖定畫面通知以及個別應用程式設定

## Desktop: Options

desktop-panel-options = 桌面與面板
    .desc = Super 按鍵行為, 螢幕角落熱點, 視窗控制選項

desktop-panels-and-applets = 桌面面板與工具程式

dock = Dock
    .desc = 用於釘選常用應用程式的面板

hot-corner = 螢幕角落熱點
    .top-left-corner = 為工作區啟用位於左上方的螢幕角落熱點

super-key-action = Super 按鍵行為
    .launcher = 啟動器
    .workspaces = 工作區
    .applications = 應用程式

top-panel = 頂部面板
    .workspaces = 顯示工作區按鈕
    .applications = 顯示應用程式按鈕

window-controls = 視窗控制
    .minimize = 顯示最小話按鈕
    .maximize = 顯示最大話按鈕

## Desktop: Panel

panel = 面板
    .desc = 頂部條狀面板用於提供桌面控制和選單

add = 新增
add-applet = 新增工具程式
all = 全部
applets = 工具程式
center-segment = 中間位置
drop-here = 將工具程式放到此處
end-segment = 末尾位置
large = 大
no-applets-found = 找不到工具程式...
panel-bottom = 下
panel-left = 左
panel-right = 右
panel-top = 上
search-applets = 搜尋工具程式...
small = 小
start-segment = 起始位置

panel-appearance = 外觀
    .match = 配合桌面
    .light = 亮
    .dark = 暗

panel-behavior-and-position = 呈現方式與位置
    .autohide = 自動隱藏面板
    .dock-autohide = 自動隱藏 Dock
    .position = 螢幕上的位置
    .display = 顯示於顯示器

panel-style = 樣式
    .anchor-gap = 面板與螢幕邊緣的間隙
    .dock-anchor-gap = Dock 與螢幕邊緣的間隙
    .extend = 面板延伸至螢幕邊緣
    .dock-extend = Dock 延伸至螢幕邊緣
    .appearance = 外觀
    .size = 尺寸
    .background-opacity = 背景透明度

panel-applets = 設定
    .dock-desc = 設定 Dock 工具
    .desc = 設定面板工具

panel-missing = 面板設定遺失
    .desc = 由於使用了自定義設定或設定檔毀損以致面板設定檔遺失
    .fix = 重設至預設值

## Desktop: Wallpaper

wallpaper = 桌布
    .change = 圖片切換
    .desc = 背景圖片、顏色和幻燈片選項
    .fit = 延伸桌布
    .folder-dialog = 選擇桌布資料夾
    .image-dialog = 選擇桌布
    .plural = 桌布
    .same = 所有螢幕使用相同的背景
    .slide = 幻燈片

add-color = 新增顏色
add-image = 新增圖片
all-displays = 所有顯示器
colors = 顏色
dialog-add = 新增
fill = 填充
fit-to-screen = 延伸至全螢幕
open-new-folder = 開啟新資料夾
recent-folders = 最近使用的資料夾

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
    .dynamic-desc = 自動移除空的工作區
    .fixed = 固定工作區數量
    .fixed-desc = 概觀模式可新增或移除工作區

workspaces-multi-behavior = 多顯示器行為
    .span = 工作區橫跨顯示器
    .separate = 顯示器各自擁有工作區

workspaces-overview-thumbnails = 工作區概觀縮圖
    .show-number = 顯示工作區編號
    .show-name = 顯示工作區名稱

workspaces-orientation = 工作區螢幕方向
    .vertical = 垂直
    .horizontal = 水平

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
    .show-date = 在條狀面板顯示日期
    .friday = 星期五
    .saturday = 星期六
    .sunday = 星期日
    .monday = 星期一

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

disable-while-typing = 鍵盤輸入時關閉觸控板

input-devices = 輸入裝置
    .desc = 輸入裝置

primary-button = 主要按鈕
    .left = 左
    .right = 右

scrolling = 捲動
    .two-finger = 以雙指捲動
    .edge = 在觸控板邊緣以單指捲動
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
    .add = 新增輸入法

keyboard-special-char = 特殊輸入
    .alternate = 進階按鍵
    .compose = 組合鍵

added = 新增
type-to-search = 輸入關鍵字以進行搜尋

## Input: Keyboard: Shortcuts

keyboard-shortcuts = 鍵盤快捷鍵
    .desc = 顯示與修改快捷鍵

## Input: Mouse

mouse = 滑鼠
    .desc = 滑鼠靈敏度、加速及捲動設定
    .speed = 滑鼠速度
    .acceleration = 啟用滑鼠加速

## Input: Touchpad

click-behavior = 觸控模式
    .click-finger = 觸控板雙指點擊視為滑鼠右鍵，三指點擊視為滑鼠中鍵
    .button-areas = 觸控板右下方點擊視為滑鼠右鍵，於中間下方點擊視為滑鼠中鍵

pinch-to-zoom = 雙指撥動縮放
    .desc = 若應用程式支援，可用雙指撥動來縮放內容

tap-to-click = 觸擊
    .desc = 啟用單指觸擊視作滑鼠左鍵，雙指觸擊視作滑鼠右鍵，三指觸擊視作滑鼠中鍵

touchpad = 觸控板
    .desc = 觸控靈敏度、點擊和手勢
    .speed = 觸控靈敏度
    .acceleration = 啟用觸控板加速

## Input: Gestures

swiping = 觸控滑動
    .four-finger-down = 四指向下滑動
    .four-finger-left = 四指向左滑動
    .four-finger-right = 四指向右滑動
    .four-finger-up = 四指向上滑動
    .three-finger-any = 三指向任何方向滑動

switch-between-windows = 視窗間切換
switch-to-next-workspace = 切換至下一個工作區
switch-to-prev-workspace = 切換至上一個工作區
open-application-library = 開啟應用程式庫
open-workspaces-view = 開啟工作區概觀
