app = COSMIC 控制中心

dbus-connection-error = 無法連線到 DBus
ok = OK
unknown = 未知

number = { $number }

## Network & Wireless

connections-and-profiles = { $variant ->
    [wired] 有線網路
    [wifi] 無線網路
    [vpn] VPN
    *[other] 未知
} 連線及連線設定檔。

add-network = 新增網路
    .profile = 新增設定檔
add-vpn = 新增 VPN
airplane-on = 飛航模式已開啟
cable-unplugged = 網路線已拔除
connect = 連線
connected = 已連線
connecting = 連線中…
disconnect = 中斷連線
forget = 忘記此網路
known-networks = 已知網路
network-and-wireless = 網路與無線
no-networks = 找不到網路
no-vpn = 無可用的 VPN 連線
password = 密碼
password-confirm = 確認密碼
remove = 移除
settings = 設定
username = 使用者名稱
visible-networks = 可見的網路
identity = 身分

auth-dialog = 需要驗證
    .vpn-description = 請輸入 VPN 服務所需的使用者名稱及密碼。
    .wifi-description = 請輸入密碼或加密金鑰。你也可以按下路由器上的「WPS」按鈕來連接。

forget-dialog = 忘記這個 Wi-Fi 網路？
    .description = 日後如要使用此 Wi-Fi 網路，將需重新輸入密碼。

network-device-state =
    .activated = 已連線
    .config = 連線中
    .deactivating = 中斷連線中
    .disconnected = 已中斷連線
    .failed = 連線失敗
    .ip-check = 檢查連線
    .ip-config = 請求 IP 和路由資訊
    .need-auth = 需要驗證
    .prepare = 準備連線
    .secondaries = 等待次要連線
    .unavailable = 無法使用
    .unknown = 狀態未知
    .unmanaged = 未管理
    .unplugged = 網路線已拔除

remove-connection-dialog = 移除連線設定檔？
    .vpn-description = 日後如要使用此 VPN，將需重新輸入密碼。
    .wired-description = 若要再次使用此設定檔，需重新建立。

vpn = VPN
    .connections = VPN 連線
    .error = 新增 VPN 設定失敗
    .remove = 移除連線設定檔
    .select-file = 選擇 VPN 配置檔案

vpn-error = VPN 錯誤
    .config = 新增 VPN 設定失敗
    .connect = 無法連線到 VPN
    .connection-editor = 連線編輯器失敗
    .connection-settings = 無法取得使用中連線的設定
    .updating-state = 網路管理員狀態更新失敗
    .wireguard-config-path = WireGuard 配置檔案路徑無效
    .wireguard-config-path-desc = 選擇的檔案必須位於本機檔案系統。
    .wireguard-device = 無法建立 WireGuard 裝置
    .with-password = 使用 nmcli 指令設定 VPN { $field ->
        *[username] 使用者名稱
        [password] 密碼
        [password-flags] password-flags
    } 失敗

wired = 有線網路
    .adapter = 有線網路卡 { $id }
    .connections = 有線網路連線
    .devices = 有線網路設備
    .remove = 移除連線設定檔

wifi = 無線網路
    .adapter = 無線網路卡 { $id }
    .forget = 忘記此網路

wireguard-dialog = 新增 WireGuard 設備
    .description = 為 WireGuard 配置選擇裝置名稱。

## Networking: Online Accounts

online-accounts = 線上帳號
    .desc = 新增帳號、郵件（IMAP、SMTP）或企業登入

# Bluetooth

activate = 啟用
confirm = 確認
enable = 啟用

bluetooth = 藍牙
    .desc = 管理藍牙設備
    .status = 當藍牙設定開啟時，此系統顯示為 { $aliases }。
    .connected = 已連線
    .connecting = 連線中
    .disconnecting = 中斷連線中
    .connect = 連線
    .disconnect = 中斷連線
    .forget = 忘記
    .dbus-error = 與 DBus 互動時發生錯誤：{ $why }
    .disabled = 藍牙服務已停用
    .inactive = 藍牙服務未啟用
    .unknown = 無法啟用藍牙服務。是否已安裝 BlueZ？

bluetooth-paired = 先前連線過的裝置
    .connect = 連線
    .battery = { $percentage }% 電池電量

bluetooth-confirm-pin = 確認藍牙PIN碼
    .description = 請確認以下 PIN 碼與顯示在 { $device } 上的 PIN 碼相符

bluetooth-available = 附近的裝置

bluetooth-adapters = 藍牙接收器

## Accessibility

accessibility = 無障礙功能
    .vision = 視覺
    .on = 開啟
    .off = 關閉
    .unavailable = 無法使用
    .screen-reader = 螢幕閱讀器
    .high-contrast = 高對比模式
    .invert-colors = 反相顏色
    .color-filters = 濾色鏡

hearing = 聽覺
    .mono = 將立體聲音訊作為單聲道播放

default = 預設
magnifier = 放大鏡
    .controls = 或使用這些快捷鍵: { $zoom_in ->
             [zero] {""}
            *[other] {""}
                {$zoom_in} 放大,
        }{ $zoom_out ->
             [zero] {""}
            *[other] {""}
                {$zoom_out} 縮小,
        }
        Super + 滾動滑鼠
    .scroll_controls = 使用 Super + 滾動啟用滑鼠或觸控板縮放
    .show_overlay = 顯示放大鏡浮層
    .increment = 放大增量
    .signin = 登入時啟動放大鏡
    .applet = 在面板的小程式中切換放大鏡開/關
    .movement = 縮放檢視移動方式
    .continuous = 跟隨指標連續移動
    .onedge = 指標到達邊緣時移動
    .centered = 保持指標置中
color-filter = 濾色鏡類型
    .unknown = 未知的濾鏡已啟用
    .greyscale = 灰階
    .deuteranopia = 綠/紅（綠色弱，Deuteranopia）
    .protanopia = 紅/綠（紅色弱，Protanopia）
    .tritanopia = 藍/黃（藍色弱，Tritanopia）

## Desktop

desktop = 桌面

## Desktop: Wallpaper

wallpaper = 桌布
    .change = 更換圖片每隔
    .desc = 桌布圖片、顏色和幻燈片選項。
    .fit = 桌布調整
    .folder-dialog = 選擇桌布資料夾
    .image-dialog = 選擇桌布圖片
    .plural = 桌布
    .same = 所有顯示器使用相同的桌布
    .slide = 幻燈片放映

add-color = 新增顏色
add-image = 新增圖片
all-displays = 所有顯示器
colors = 顏色
dialog-add = 新增
fill = 填滿
fit-to-screen = 適應螢幕
open-new-folder = 開啟新的資料夾
recent-folders = 最近使用的資料夾

x-minutes = { $number } { $number ->
    [one] 分鐘
    *[other] 分鐘
}
x-hours = { $number } { $number ->
    [one] 小時
    *[other] 小時
}
never = 從不

## Desktop: Appearance

appearance = 外觀
    .desc = 介面主題與色調

accent-color = 強調色
app-background = 應用程式背景
auto = 自動
close = 關閉
color-picker = 調色盤
copied-to-clipboard = 已複製到剪貼簿
copy-to-clipboard = 複製到剪貼簿
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

auto-switch = 自動在明亮和夜色模式之間切換
    .sunrise = 清晨時切換為亮色調
    .sunset = 黃昏時切換為夜色調
    .next-sunrise = 下次清晨切換為亮色調
    .next-sunset = 下次黃昏切換為夜色調

container-background = 容器背景
    .desc-detail = 容器背景與導航列、側邊欄、對話方塊等元件相關，預設從應用程式或視窗背景進行衍生
    .reset = 重設至自動
    .desc = 容器背景色與導航列、側邊欄、對話方塊等元件相關

control-tint = 控制元件色調
    .desc = 標準按鈕、搜尋輸入、文字輸入和類似元件的背景

frosted = 系統介面呈現磨砂玻璃的透明效果
    .desc = 將磨砂玻璃的透明效果套用至面板、容器、基座、工具程式、啟動器及程式庫

enable-export = 將此主題套用於 GNOME 應用程式
    .desc = 並非所有圖形介面程式庫都支援自動切換，非 COSMIC 原生圖形介面程式庫所開發的程式可能需要在重啟啟動後才會套用新的主題

icon-theme = 圖示佈景主題
    .desc = 將不同的圖示集套用至應用程式

text-tint = 介面文字色調
    .desc = 在各種背景中衍生出足夠對比色的顏色

style = 風格
    .round = 圓角
    .slightly-round = 半圓角
    .square = 正角

interface-density = 介面密度
    .comfortable = 舒適
    .compact = 緊湊
    .spacious = 寬敞

window-management-appearance = 視窗管理
    .active-hint = 作用中視窗提示尺寸
    .gaps = 平鋪視窗模式下的間隔距離

### Experimental

experimental-settings = 實驗性設定
icons-and-toolkit = 圖示和工具組佈景主題
interface-font = 系統字型
monospace-font = 等寬字型

## Desktop: Notifications

notifications = 通知
    .desc = 勿擾模式，鎖定畫面通知以及個別應用程式設定

## Desktop: Panel

panel = 面板
    .desc = 桌面控制項和選單的頂部欄。

add = 新增
add-applet = 新增工具程式
all = 全部
applets = 工具程式
center-segment = 中間位置
drop-here = 將工具程式拖曳至此處
end-segment = 末尾位置
large = 大
no-applets-found = 找不到工具程式...
panel-bottom = 底部
panel-left = 左側
panel-right = 右側
panel-top = 頂部
search-applets = 搜尋工具程式...
small = 小
start-segment = 起始位置

panel-appearance = 外觀
    .match = 符合桌布主題
    .light = 淺色
    .dark = 深色

panel-behavior-and-position = 行為和位置
    .autohide = 自動隱藏面板
    .dock-autohide = 自動隱藏基座
    .position = 螢幕上的位置
    .display = 顯示在螢幕上

panel-style = 樣式
    .anchor-gap = 面板與螢幕邊緣的間距
    .dock-anchor-gap = 基座與螢幕邊緣的間距
    .extend = 將面板延伸至螢幕邊緣
    .dock-extend = 將基座延伸至螢幕邊緣
    .appearance = 外觀
    .size = 尺寸
    .background-opacity = 背景透明度

panel-applets = 配置
    .dock-desc = 配置基座工具程式
    .desc = 配置工具程式面板

panel-missing = 面板配置遺失
    .desc = 由於使用自訂設定或設定檔損毀，面板配置遺失。
    .fix = 重設為預設值

## Desktop: Dock

dock = 基座
    .desc = 應用程式匣中釘選的應用程式面板和其他工具程式。

## Desktop: Window management

window-management = 視窗管理
    .desc = 超級鍵動作、視窗控制選項，以及額外的視窗平鋪選項。

super-key = 超級鍵動作
    .launcher = 開啟啟動器
    .workspaces = 開啟工作區
    .applications = 開啟應用程式
    .disable = 關閉

edge-gravity = 浮動視窗吸附至鄰近邊緣

window-controls = 視窗控制項
    .maximize = 顯示最大化按鈕
    .minimize = 顯示最小化按鈕
    .active-window-hint = 顯示活動視窗提示

focus-navigation = 焦點導航
    .focus-follows-cursor = 焦點跟隨滑鼠游標
    .focus-follows-cursor-delay = 焦點跟隨滑鼠游標延遲（毫秒）
    .cursor-follows-focus = 滑鼠游標跟隨焦點

## Desktop: Workspaces

workspaces = 工作區
    .desc = 工作區方向和行為。

workspaces-behavior = 工作區行為
    .dynamic = 動態工作區
    .dynamic-desc = 自動移除空白工作區。
    .fixed = 固定數量的工作區
    .fixed-desc = 在總覽中新增或移除工作區。

workspaces-multi-behavior = 多螢幕行為
    .span = 工作區跨螢幕顯示
    .separate = 螢幕擁有各自的工作區

workspaces-overview-thumbnails = 工作區概觀縮圖
    .show-number = 顯示工作區編號
    .show-name = 顯示工作區名稱

workspaces-orientation = 工作區方向
    .vertical = 垂直
    .horizontal = 水平

hot-corner = 觸發角
    .top-left-corner = 啟用左上角觸發角以切換工作區

## Displays

-requires-restart = 需要重新啟動

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
    .additional-scale-options = 其他縮放選項

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
    .rotate-90 = 旋轉 90°
    .rotate-180 = 旋轉 180°
    .rotate-270 = 旋轉 270°

vrr = 可變刷新率
    .enabled = 已啟用
    .force = 總是
    .auto = 自動
    .disabled = 已停用

scheduling = 排程
    .manual = 手動排程

dialog = 對話框
    .title = 保留這些顯示設定？
    .keep-changes = 保留變更
    .change-prompt = 設定變更將在 { $time } 秒後自動還原。
    .revert-settings = 還原設定

## Sound

sound = 音效
    .desc = N/A

sound-output = 輸出
    .volume = 音量
    .device = 輸出裝置
    .level = 音量大小
    .config = 設定
    .balance = 平衡
    .left = 左
    .right = 右

sound-input = 輸入
    .volume = 音量
    .device = 輸入裝置
    .level = 音量大小

sound-alerts = 警示音
    .volume = 音量
    .sound = 警示聲音

sound-applications = 程式音效
    .desc = 程式音效設定

profile = 設定檔

## Power

power = 電源 & 電池
    .desc = 管理電源設定

battery = 電池
  .minute = { $value } { $value ->
        [one] 分鐘
       *[other] 分鐘
  }
  .hour = { $value } { $value ->
        [one] 小時
       *[other] 小時
  }
  .day = { $value } { $value ->
        [one] 天
       *[other] 天
  }
  .less-than-minute = 小於一分鐘
  .and = 和
  .remaining-time = { $time } 直到 { $action ->
        [full] 充滿
       *[other] 電量耗盡
   }

connected-devices = 已連線裝置
  .unknown = 未知裝置

power-mode = 電源模式
    .battery = 延長電池續航力
    .battery-desc = 降低耗電量並以靜音模式運作。
    .balanced = 平衡
    .balanced-desc = 安靜的效能和適中的耗電量。
    .performance = 高效能
    .performance-desc = 最高效能和耗電量。
    .no-backend = 找不到後端。請安裝 system76-power 或 power-profiles-daemon。

power-saving = 省電選項
    .turn-off-screen-after = 閒置多久後關閉螢幕
    .auto-suspend = 自動休眠
    .auto-suspend-ac = 插電時自動休眠
    .auto-suspend-battery = 使用電池時自動休眠

## Input

acceleration-desc = 自動依照觸控板移動加速度即時變更滑鼠速度

disable-while-typing = 鍵盤輸入時關閉觸控板

input-devices = 輸入裝置
    .desc = 輸入裝置

primary-button = 主要按鈕
    .desc = 設定實體按鈕的順序。
    .left = 左
    .right = 右

scrolling = 捲動
    .two-finger = 以雙指捲動
    .edge = 在觸控板邊緣以單指捲動
    .speed = 捲動速度
    .natural = 自然捲動
    .natural-desc = 捲動內容而非視界

## Input: Keyboard

slow = 慢
fast = 快
short = 短
long = 長
keyboard = 鍵盤
    .desc = 鍵盤輸入來源、切換、特殊字元輸入、捷徑。

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
    .caps = 大寫鎖定鍵

keyboard-typing-assist = 鍵盤輸入
    .repeat-rate = 重複率
    .repeat-delay = 重複延遲

keyboard-numlock-boot = 鍵盤數字鎖定開機
    .boot-state = 開機狀態
    .last-boot = 上次開機狀態
    .on = 開
    .off = 關
    .set = 設定數字鎖定鍵開機狀態

added = 新增
type-to-search = 輸入關鍵字以進行搜尋
show-extended-input-sources = 顯示延伸輸入來源

## Input: Keyboard: Shortcuts

keyboard-shortcuts = 鍵盤快捷鍵
    .desc = 顯示與修改快捷鍵

add-another-keybinding = 新增其他按鍵綁定
cancel = 取消
command = 命令
custom = 自訂
debug = 除錯
disabled = 已停用
input-source-switch = 切換鍵盤輸入法
migrate-workspace-prev = 將工作區移至上一個輸出
migrate-workspace-next = 將工作區移至下一個輸出
migrate-workspace = 將工作區移至輸出 { $direction ->
    *[down] 下方
    [left] 左方
    [right] 右方
    [up] 上方
}
navigate = 導覽
replace = 取代
shortcut-name = 捷徑名稱
system-controls = 系統控制
terminate = 終止
toggle-stacking = 切換視窗堆疊
type-key-combination = 輸入按鍵組合

custom-shortcuts = 自訂捷徑
    .add = 新增捷徑
    .context = 新增自訂捷徑
    .none = 沒有自訂捷徑

modified = { $count } 項已修改

nav-shortcuts = 導覽
    .prev-output = 聚焦到前一個輸出
    .next-output = 聚焦到下一個輸出
    .last-workspace = 聚焦到上一個工作區
    .prev-workspace = 聚焦到前一個工作區
    .next-workspace = 聚焦到下一個工作區
    .focus = 聚焦視窗 { $direction ->
        *[down] 下方
        [in] 內
        [left] 左方
        [out] 外
        [right] 右方
        [up] 上方
    }
    .output = 切換到輸出 { $direction ->
        *[down] 下方
        [left] 左方
        [right] 右方
        [up] 上方
    }
    .workspace = 切換到工作區 { $num }

manage-windows = 管理視窗
    .close = 關閉視窗
    .maximize = 最大化視窗
    .fullscreen = 全螢幕視窗
    .minimize = 最小化視窗
    .resize-inwards = 向內調整視窗大小
    .resize-outwards = 向外調整視窗大小
    .toggle-sticky = 切換置頂視窗

move-windows = 移動視窗
    .direction = 移動視窗 { $direction ->
        *[down] 下
        [left] 左
        [right] 右
        [up] 上
    }
    .display = 將視窗移動到另一個螢幕 { $direction ->
        *[down] 下
        [left] 左
        [right] 右
        [up] 上
    }
    .workspace = 將視窗移動到另一個工作區 { $direction ->
        *[below] 下方
        [left] 左方
        [right] 右方
        [above] 上方
    }
    .workspace-num = 將視窗移動到工作區 { $num }
    .prev-workspace = 將視窗移動到上一個工作區
    .next-workspace = 將視窗移動到下一個工作區
    .last-workspace = 將視窗移動到最後一個工作區
    .next-display = 將視窗移動到下一個顯示器
    .prev-display = 將視窗移動到上一個顯示器
    .send-to-prev-workspace = 將視窗移動到上一個工作區
    .send-to-next-workspace = 將視窗移動到下一個工作區

system-shortcut = 系統
    .app-library = 開啟應用程式庫
    .brightness-down = 降低螢幕亮度
    .brightness-up = 增加螢幕亮度
    .home-folder = 開啟主目錄
    .keyboard-brightness-down = 降低鍵盤亮度
    .keyboard-brightness-up = 增加鍵盤亮度
    .launcher = 開啟啟動器
    .log-out = 登出
    .lock-screen = 鎖定螢幕
    .mute = 靜音
    .mute-mic = 麥克風靜音
    .play-pause = 播放/暫停
    .play-next = 下一首
    .play-prev = 上一首
    .poweroff = 關機
    .screenshot = 截圖
    .terminal = 開啟終端機
    .volume-lower = 降低音量
    .volume-raise = 增加音量
    .web-browser = 開啟網頁瀏覽器
    .window-switcher = 切換視窗
    .window-switcher-previous = 反向切換視窗
    .workspace-overview = 開啟工作區概覽

window-tiling = 視窗平鋪
    .horizontal = 設定水平方向
    .vertical = 設定垂直方向
    .swap-window = 交換視窗
    .toggle-tiling = 切換視窗平鋪
    .toggle-stacking = 切換視窗堆疊
    .toggle-floating = 切換視窗浮動
    .toggle-orientation = 切換方向

replace-shortcut-dialog = 取代捷徑？
    .desc = { $shortcut } 已被 { $name } 使用。如果您取代它， { $name } 將會被停用。

zoom-in = 放大
zoom-out = 縮小

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
    .acceleration = 啟用觸控板加速
    .desc = 觸控靈敏度、點擊和手勢
    .speed = 觸控靈敏度

## Input: Gestures

gestures = 手勢
    .four-finger-down = 四指向下滑動
    .four-finger-left = 四指向左滑動
    .four-finger-right = 四指向右滑動
    .four-finger-up = 四指向上滑動
    .three-finger-any = 三指向任何方向滑動

switch-workspaces = 切換工作區
    .horizontal = 四指左右滑動
    .vertical = 四指上下滑動

switch-between-windows = 視窗間切換
open-application-library = 開啟應用程式庫
open-workspaces-view = 開啟工作區概觀

## Time & Language

time = 時間和語言
    .desc = N/A

time-date = 日期和時間
    .desc = 時區、自動時間校正、時間格式設定
    .auto = 自動設定
    .auto-ntp = 設定時區後，日期和時間將會自動更新。

time-zone = 時區
    .auto = 自動設定時區
    .auto-info = 需要定位服務和網路存取能力

time-format = 日期和時間格式
    .twenty-four = 24 小時制
    .show-seconds = 顯示秒數
    .first = 每週的第一天
    .show-date = 在條狀面板顯示日期
    .friday = 星期五
    .saturday = 星期六
    .sunday = 星期日
    .monday = 星期一

time-region = 地區和語言
    .desc = 基於地區來格式化日期、時間及數字

formatting = 格式設定
    .dates = 日期
    .time = 時間
    .date-and-time = 日期 & 時間
    .numbers = 數字
    .measurement = 度量衡
    .paper = 紙張

preferred-languages = 偏好語言
    .desc = 語言的順序決定桌面的翻譯語言，變更將在下次登入時生效。

add-language = 新增語言
    .context = 新增語言
install-additional-languages = 安裝其他語言
region = 地區

## Applications

applications = 應用程式

## Applications: Default Applications

default-apps = 預設應用程式
    .desc = 預設網頁瀏覽器、郵件用戶端、檔案瀏覽器和其他應用程式。
    .web-browser = 網頁瀏覽器
    .file-manager = 檔案管理員
    .mail-client = 郵件用戶端
    .music = 音樂
    .video = 影片
    .photos = 照片
    .calendar = 行事曆
    .terminal = 終端機
    .other-associations = 其他關聯
    .text-editor = 文字編輯器

## Applications: Startup Applications

startup-apps = 啟動應用程式
    .desc = 設定登入時執行的應用程式。
    .add = 新增應用程式
    .user = 您登入時啟動的應用程式
    .none = 未新增啟動應用程式
    .remove-dialog-title = 移除 { $name }？
    .remove-dialog-description = 您確定要移除此啟動應用程式嗎？
    .search-for-application = 搜尋應用程式

## Applications: Legacy Applications

legacy-applications = X11 應用程式相容性
    .desc = X11 視窗系統應用程式縮放和全域快捷鍵。

legacy-app-global-shortcuts = X11 應用程式中的全域快捷鍵
    .desc = 全域快捷鍵允許在應用程式中執行的按鍵和滑鼠按鈕事件被其他應用程式識別，用於「按鍵通話」或「按鍵靜音」等功能。預設情況下，此功能在 X11 應用程式中被停用，以確保其他應用程式無法監視包含敏感資訊的鍵盤和滑鼠事件。
    .none = 無按鍵
    .modifiers = 修飾鍵 (Super, Shift, Control, Alt)
    .combination = 當按下 Super、Control 或 Alt 修飾鍵時的所有按鍵
    .all = 所有按鍵
    .mouse = X11 應用程式中的滑鼠按鈕事件

legacy-app-scaling = X11 視窗系統應用程式縮放
    .scaled-gaming = 為遊戲和全螢幕應用程式最佳化
    .gaming-description = X11 應用程式可能比 Wayland 應用程式稍大/稍小。
    .scaled-applications = 為應用程式最佳化
    .applications-description = 遊戲和全螢幕 X11 應用程式可能不符合您的顯示器解析度。
    .scaled-compatibility = 最大相容性模式
    .compatibility-description = X11 應用程式在 HiDPI 螢幕上可能會出現模糊。
    .preferred-display = 遊戲和全螢幕 X11 應用程式的偏好顯示器
    .no-display = 無

## System

system = 系統與帳戶

## System: About

about = 關於
    .desc = 裝置名稱、硬體資訊、作業系統

about-device = 裝置名稱
    .desc = 此名稱用於顯示給其他網路或藍牙裝置

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
    .admin = 管理員
    .standard = 標準
    .profile-add = 選擇個人資料圖片

administrator = 系統管理員
    .desc = 系統管理員可以變更所有使用者的設定，新增和移除其他使用者。

add-user = 新增使用者
change-password = 變更密碼
remove-user = 移除使用者
full-name = 姓名
invalid-username = 無效的使用者名稱。
password-mismatch = 密碼與確認密碼必須相符。
save = 儲存