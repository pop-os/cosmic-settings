app = COSMIC 設定
dbus-connection-error = 無法連線到 DBus
ok = 確定
unknown = 未知
number = { $number }

## Network & Wireless

add-network = 新增網路
    .profile = 新增設定檔
add-vpn = 新增 VPN
airplane-on = 飛航模式已開啟。
cable-unplugged = 網路線已拔除
connect = 連線
connected = 已經連線
connecting = 連線中…
disconnect = 中斷連線
forget = 忘記此網路
known-networks = 已知網路
network-and-wireless = 網路與無線
no-networks = 找不到網路。
no-vpn = 無可用的 VPN 連線。
password = 密碼
password-confirm = 確認密碼
remove = 移除
settings = 設定
username = 使用者名稱
visible-networks = 可見的網路
identity = 身分識別
auth-dialog = 需要驗證
    .vpn-description = 請輸入 VPN 服務所需的使用者名稱及密碼。
    .wifi-description = 請輸入密碼或加密金鑰。你也可以按下路由器上的「WPS」按鈕來連接。
forget-dialog = 忘記這個 Wi-Fi 網路？
    .description = 日後若要使用此 Wi-Fi 網路，將需重新輸入密碼。
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
    .vpn-description = 日後若要使用此網路，將需重新輸入密碼。
    .wired-description = 日後若要使用此設定檔，將需重新建立。
vpn = VPN
    .connections = VPN 連線
    .error = 新增 VPN 設定失敗
    .remove = 移除連線設定檔
    .select-file = 選擇 VPN 配置檔案
vpn-error = VPN 錯誤
    .config = 新增 VPN 設定失敗
    .connect = 連線至 VPN 失敗
    .connection-editor = 連線編輯器失敗
    .connection-settings = 取得活動連線的設定失敗
    .updating-state = 更新網路管理員狀態失敗
    .wireguard-config-path = WireGuard 設定的檔案路徑無效
    .wireguard-config-path-desc = 所選檔案必須位於本機檔案系統上。
    .wireguard-device = 建立 WireGuard 裝置失敗
    .with-password =
        使用 nmcli 設定 VPN { $field ->
           *[username] 使用者名稱
            [password] 密碼
            [password-flags] 密碼選項
        }失敗
wired = 有線網路
    .adapter = 有線網路卡 { $id }
    .connections = 有線網路連線
    .devices = 有線網路裝置
    .remove = 移除連線設定檔
wifi = Wi-Fi
    .adapter = Wi-Fi 網路卡 { $id }
    .forget = 忘記此網路
wireguard-dialog = 新增 WireGuard 裝置
    .description = 為 WireGuard 設定選擇裝置名稱。

## Networking: Online Accounts

online-accounts = 線上帳號
    .desc = 新增帳號、郵件與企業登入

# Bluetooth

activate = 啟用
confirm = 確認
enable = 啟用
bluetooth = 藍牙
    .status = 當藍牙設定開啟時，此系統顯示為 { $aliases }。
    .connected = 已經連線
    .connecting = 連線中
    .disconnecting = 中斷連線中
    .connect = 連線
    .disconnect = 中斷連線
    .forget = 忘記
    .dbus-error = 與 DBus 互動時發生錯誤：{ $why }
    .disabled = 藍牙服務已經停用
    .inactive = 藍牙服務尚未啟用
    .unknown = 無法啟用藍牙服務。是否已經安裝 BlueZ？
bluetooth-paired = 先前連接的裝置
    .connect = 連接
    .battery = 電量 { $percentage }%
bluetooth-confirm-pin = 確認藍牙 PIN 碼
    .description = 請確認以下 PIN 碼與 { $device } 上顯示的是否相符
bluetooth-available = 附近的裝置
bluetooth-adapters = 藍牙介面卡

## Accessibility

accessibility = 無障礙功能
    .vision = 視覺
    .on = 開啟
    .off = 關閉
    .unavailable = 無法使用
    .screen-reader = 螢幕閱讀器
    .high-contrast = 高對比模式
    .invert-colors = 反轉顏色
    .color-filters = 色彩濾鏡
hearing = 聽覺
    .mono = 將立體聲播放為單聲道
default = 預設
magnifier = 放大鏡
    .controls =
        或使用這些快速鍵： { $zoom_in ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_in } 放大,
        }{ $zoom_out ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_out } 縮小,
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
color-filter = 色彩濾鏡類型
    .unknown = 未知的濾鏡已啟用
    .greyscale = 灰階
    .deuteranopia = 綠/紅（綠色弱視，第二型色盲）
    .protanopia = 紅/綠（紅色弱視，第一型色盲）
    .tritanopia = 藍/黃（藍色弱視，第三型色盲）

## Desktop

desktop = 桌面

## Desktop: Wallpaper

wallpaper = 桌布
    .change = 更換圖片每隔
    .fit = 桌布調整
    .folder-dialog = 選擇桌布資料夾
    .image-dialog = 選擇桌布圖片
    .plural = 桌布
    .same = 全部顯示器使用相同的桌布
    .slide = 投影片放映
add-color = 新增色彩
add-image = 新增圖片
all-displays = 所有顯示器
colors = 色彩
dialog-add = 新增
fill = 填滿
fit-to-screen = 符合螢幕大小
open-new-folder = 開啟新資料夾
recent-folders = 最近使用資料夾
x-minutes =
    { $number } { $number ->
        [one] 分鐘
       *[other] 分鐘
    }
x-hours =
    { $number } { $number ->
        [one] 小時
       *[other] 小時
    }
never = 永不

## Desktop: Appearance

appearance = 外觀
accent-color = 強調色
app-background = 視窗背景
auto = 自動
close = 關閉
color-picker = 色彩選擇器
copied-to-clipboard = 已複製到剪貼簿
copy-to-clipboard = 複製到剪貼簿
dark = 深色
export = 匯出
hex = 十六進位
import = 匯入
light = 淺色
mode-and-colors = 色彩與模式
recent-colors = 最近使用的色彩
reset-to-default = 重新設定至預設
rgb = RGB
window-hint-accent = 作用中視窗提示色彩
window-hint-accent-toggle = 使用強調色作為作用中視窗提示色彩
auto-switch = 在淺色與深色模式間自動切換
    .sunrise = 日出時切換為淺色模式
    .sunset = 日落時切換為深色模式
    .next-sunrise = 下次日出時切換為淺色模式
    .next-sunset = 下次日落時切換為深色模式
container-background = 容器背景
    .desc-detail = 容器背景與導航列、側邊欄、對話方塊等元件相關，預設從應用程式或視窗背景進行衍生。
    .reset = 重設至自動
    .desc = 容器背景色與導航列、側邊欄、對話方塊等元件相關
control-tint = 控制元件色調
    .desc = 使用於標準按鈕、搜尋輸入、文字輸入和類似元件的背景
frosted = 系統介面呈現磨砂玻璃的透明效果
    .desc = 將磨砂玻璃的透明效果套用至面板、程式塢、工具程式、啟動器及程式庫
enable-export = 將目前主題套用至 GNOME 應用程式
    .desc = 並非所有工具組都支援自動切換。非 COSMIC 應用程式在主題變更後可能需要重新啟動。
icon-theme = 圖示主題
    .desc = 套用不同的圖示集至應用程式
text-tint = 介面文字色調
    .desc = 用來衍生出介面文字的顏色，其有足夠對比於各種表面
style = 樣式
    .round = 圓角
    .slightly-round = 略為圓角
    .square = 直角
interface-density = 介面密度
    .comfortable = 舒適
    .compact = 緊湊
    .spacious = 寬敞
window-management-appearance = 視窗管理
    .active-hint = 作用中視窗提示大小
    .gaps = 平鋪視窗周圍的間隙

### Experimental

experimental-settings = 實驗性設定
icons-and-toolkit = 圖示與工具組主題
interface-font = 系統字型
monospace-font = 等寬字型

## Desktop: Notifications

notifications = 通知

## Desktop: Panel

panel = 面板
add = 添增
add-applet = 新增工具程式
all = 全部
applets = 工具程式
center-segment = 中間位置
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
    .match = 符合桌面
    .light = 淺色
    .dark = 深色
panel-behavior-and-position = 行為和位置
    .autohide = 自動隱藏面板
    .dock-autohide = 自動隱藏程式塢
    .position = 螢幕上的位置
    .display = 顯示在螢幕上
panel-style = 樣式
    .anchor-gap = 面板與螢幕邊緣的間距
    .dock-anchor-gap = 程式塢與螢幕邊緣的間距
    .extend = 將面板延伸至螢幕邊緣
    .dock-extend = 將程式塢延伸至螢幕邊緣
    .appearance = 外觀
    .size = 尺寸
    .background-opacity = 背景透明度
panel-applets = 配置
    .dock-desc = 配置程式塢工具程式
    .desc = 配置工具程式面板
panel-missing = 面板設定遺失
    .desc = 面板設定檔案因使用自訂設定而遺失，或已損毀。
    .fix = 重設為預設值

## Desktop: Dock

dock = 程式塢

## Desktop: Window management

window-management = 視窗管理
super-key = Super 鍵動作
    .launcher = 開啟啟動器
    .workspaces = 開啟工作區
    .applications = 開啟應用程式
    .none = 停用
edge-gravity = 浮動視窗吸附至附近邊緣
window-controls = 視窗控制項
    .maximize = 顯示最大化按鈕
    .minimize = 顯示最小化按鈕
    .active-window-hint = 顯示作用中視窗提示
focus-navigation = 焦點導覽
    .focus-follows-cursor = 焦點跟隨游標
    .focus-follows-cursor-delay = 焦點跟隨游標延遲（毫秒）
    .cursor-follows-focus = 游標跟隨焦點

## Desktop: Workspaces

workspaces = 工作區
workspaces-behavior = 工作區行為
    .dynamic = 動態工作區
    .dynamic-desc = 自動移除空的工作區。
    .fixed = 固定數量的工作區
    .fixed-desc = 在概覽中新增或移除工作區。
workspaces-multi-behavior = 多螢幕行為
    .span = 工作區橫跨顯示器
    .separate = 顯示器有各自的工作區
workspaces-overview-thumbnails = 工作區總覽縮圖
    .show-number = 顯示工作區編號
    .show-name = 顯示工作區名稱
workspaces-orientation = 工作區方向
    .vertical = 垂直
    .horizontal = 水平
hot-corner = 熱區
    .top-left-corner = 啟用左上角熱區以開啟工作區

## Displays

-requires-restart = 需要重新啟動
color = 色彩
    .depth = 色彩深度
    .profile = 色彩設定檔
    .sidebar = 色彩設定檔
    .temperature = 色溫
display = 顯示螢幕
    .arrangement = 螢幕排列
    .arrangement-desc = 拖曳螢幕來更改排列方式
    .enable = 啟用顯示螢幕
    .external = { $size } { $output } 外接顯示螢幕
    .laptop = { $size } 筆電顯示螢幕
    .options = 顯示螢幕螢幕選項
    .refresh-rate = 螢幕更新頻率
    .resolution = 解析度
    .scale = 縮放大小
    .additional-scale-options = 其他縮放選項
mirroring = 投影
    .id = 正在投影 { $id }
    .dont = 不要投影
    .mirror = 投影 { $display }
    .project =
        投影至 { $display ->
            [all] 全部的螢幕
           *[other] { $display }
        }
    .project-count =
        投影了 { $count } 個 { $count ->
            [1] 螢幕
           *[other] 螢幕
        }
night-light = 夜間模式
    .auto = 自動（日落至日出）
    .desc = 以暖色調來減少藍光
orientation = 螢幕方向
    .standard = 橫式
    .rotate-90 = 旋轉 90 度
    .rotate-180 = 旋轉 180 度
    .rotate-270 = 旋轉 270 度
vrr = 可變刷新率
    .enabled = 已啟用
    .force = 總是
    .auto = 自動
    .disabled = 已停用
scheduling = 排程
    .manual = 手動排程
dialog = 對話框
    .title = 保留這些顯示器設定？
    .keep-changes = 保留變更
    .change-prompt = 設定變更將在 { $time } 秒後自動還原。
    .revert-settings = 還原設定

## Sound

sound = 音響
sound-output = 輸出
    .volume = 輸出音量
    .device = 輸出裝置
    .level = 輸出等級
    .config = 組態
    .balance = 平衡
    .left = 左
    .right = 右
sound-input = 輸入
    .volume = 輸入音量
    .device = 輸入裝置
    .level = 輸入等級
sound-alerts = 提示音
    .volume = 提示音音量
    .sound = 提示音音效
sound-applications = 應用程式
    .desc = 應用程式音量與設定

## Power

power = 電源與電池
battery = 電池
    .minute =
        { $value } { $value ->
            [one] 分鐘
           *[other] 分鐘
        }
    .hour =
        { $value } { $value ->
            [one] 小時
           *[other] 小時
        }
    .day =
        { $value } { $value ->
            [one] 天
           *[other] 天
        }
    .less-than-minute = 少於一分鐘
    .and = 與
    .remaining-time =
        { $time } 直到 { $action ->
            [full] 充滿
           *[other] 耗盡
        }
connected-devices = 已連接的裝置
    .unknown = 未知裝置
power-mode = 電源模式
    .battery = 延長電池續航力
    .battery-desc = 降低耗電量與靜音效能
    .balanced = 平衡
    .balanced-desc = 安靜的效能與適度的耗電量
    .performance = 高效能
    .performance-desc = 最高效能與高耗電量
    .no-backend = 找不到後端。請安裝 system76-power 或 power-profiles-daemon。
power-saving = 省電選項
    .turn-off-screen-after = 閒置多久後關閉螢幕
    .auto-suspend = 自動休眠
    .auto-suspend-ac = 插電時自動休眠
    .auto-suspend-battery = 使用電池時自動休眠

## Input

acceleration-desc = 自動調整追蹤靈敏度基於速度
disable-while-typing = 輸入時停用觸控板
input-devices = 輸入裝置
primary-button = 主要按鈕
    .desc = 設定實體按鈕的順序
    .left = 左
    .right = 右
scrolling = 捲動
    .two-finger = 使用兩指捲動
    .edge = 使用一指沿邊緣捲動
    .speed = 捲動速度
    .natural = 自然捲動
    .natural-desc = 捲動內容而非檢視畫面

## Input: Keyboard

slow = 慢
fast = 快
short = 短
long = 長
keyboard = 鍵盤
keyboard-sources = 輸入法
    .desc = 輸入法可使用 Super+Space 按鍵組合進行切換，此項設定可以在鍵盤快速鍵設定頁面進行修改
    .move-up = 往上
    .move-down = 往下
    .settings = 設定
    .view-layout = 檢視鍵盤布局
    .remove = 移除
    .add = 新增輸入法
keyboard-special-char = 特殊字輸入
    .alternate = 進階按鍵
    .compose = 組合按鍵
    .compose-desc = 組合按鍵允許輸入多樣字元。要使用它，請按組合按鍵，再按字元依序。例如，按組合按鍵接著按 C 和 o 鍵，將輸入 ©；按組合按鍵接著按 a 和 ‘ 鍵，將輸入 á。
    .caps = 大寫鎖定鍵
keyboard-typing-assist = 輸入
    .repeat-rate = 重複速率
    .repeat-delay = 重複延遲
keyboard-numlock-boot = 數字鍵盤鎖定
    .boot-state = 開機時的狀態
    .last-boot = 上次開機
    .on = 開啟
    .off = 關閉
    .set = 設定數字鍵盤鎖定開機狀態
added = 已新增
type-to-search = 輸入進行搜尋...
show-extended-input-sources = 顯示延伸輸入來源

## Input: Keyboard: Shortcuts

keyboard-shortcuts = 鍵盤快速鍵
    .desc = 顯示與修改快速鍵
add-another-keybinding = 添加另一個按鍵組合
cancel = 取消
command = 指令
custom = 自訂
debug = 偵錯
disabled = 已停用
input-source-switch = 切換鍵盤語言輸入來源
migrate-workspace-prev = 將工作區遷移至上一個輸出
migrate-workspace-next = 將工作區遷移至下一個輸出
migrate-workspace =
    將工作區遷移至 { $direction ->
       *[down] 下方
        [left] 左側
        [right] 右側
        [up] 上方
    }的輸出
navigate = 導覽
replace = 取代
shortcut-name = 快速鍵名稱
system-controls = 系統控制
terminate = 終止
toggle-stacking = 切換視窗堆疊
type-key-combination = 輸入按鍵組合
custom-shortcuts = 自訂快速鍵
    .add = 添增快速鍵
    .context = 添增自訂快速鍵
    .none = 無自訂快速鍵
modified = { $count } 項已經修改
nav-shortcuts = 導覽
    .prev-output = 聚焦上一個輸出
    .next-output = 聚焦下一個輸出
    .last-workspace = 聚焦最後一個工作區
    .prev-workspace = 聚焦上一個工作區
    .next-workspace = 聚焦下一個工作區
    .focus =
        聚焦 { $direction ->
           *[down] 下方
            [in] 內部
            [left] 左側
            [out] 外部
            [right] 右側
            [up] 上方
        }的視窗
    .output =
        切換至 { $direction ->
           *[down] 下方
            [left] 左側
            [right] 右側
            [up] 上方
        }的輸出
    .workspace = 切換至工作區 { $num }
manage-windows = 管理視窗
    .close = 關閉視窗
    .maximize = 最大化視窗
    .fullscreen = 全螢幕視窗
    .minimize = 最小化視窗
    .resize-inwards = 將視窗向內調整大小
    .resize-outwards = 將視窗向外調整大小
    .toggle-sticky = 切換置頂視窗
move-windows = 移動視窗
    .direction =
        向 { $direction ->
           *[down] 下
            [left] 左
            [right] 右
            [up] 上
        }移動視窗
    .display =
        將視窗移至 { $direction ->
           *[down] 下方
            [left] 左側
            [right] 右側
            [up] 上方
        }的一個螢幕
    .workspace =
        將視窗移至 { $direction ->
           *[below] 下方
            [left] 左側
            [right] 右側
            [above] 上方
        }的一個工作區
    .workspace-num = 將視窗移至工作區 { $num }
    .prev-workspace = 將視窗移至上一個工作區
    .next-workspace = 將視窗移至下一個工作區
    .last-workspace = 將視窗移至最後一個工作區
    .next-display = 將視窗移至下一個顯示器
    .prev-display = 將視窗移至上一個顯示器
    .send-to-prev-workspace = 將視窗移至上一個工作區
    .send-to-next-workspace = 將視窗移至下一個工作區
system-shortcut = 系統
    .app-library = 開啟應用程式庫
    .brightness-down = 降低螢幕亮度
    .brightness-up = 提高螢幕亮度
    .display-toggle = 切換內部顯示螢幕
    .home-folder = 開啟家目錄
    .keyboard-brightness-down = 降低鍵盤亮度
    .keyboard-brightness-up = 提高鍵盤亮度
    .launcher = 開啟啟動器
    .log-out = 登出
    .lock-screen = 鎖定螢幕
    .mute = 靜音
    .mute-mic = 麥克風靜音
    .play-pause = 播放/暫停
    .play-next = 下一首
    .play-prev = 上一首
    .poweroff = 關機
    .screenshot = 螢幕擷圖
    .suspend = 休眠
    .terminal = 開啟終端機
    .touchpad-toggle = 切換觸控板
    .volume-lower = 降低音量
    .volume-raise = 提高音量
    .web-browser = 開啟網頁瀏覽器
    .window-switcher = 切換視窗
    .window-switcher-previous = 反向切換視窗
    .workspace-overview = 開啟工作區總覽
window-tiling = 視窗平鋪
    .horizontal = 設定水平方向
    .vertical = 設定垂直方向
    .swap-window = 交換視窗
    .toggle-tiling = 切換視窗平鋪
    .toggle-stacking = 切換視窗堆疊
    .toggle-floating = 切換視窗浮動
    .toggle-orientation = 切換方向
replace-shortcut-dialog = 取代快速鍵？
    .desc = { $shortcut } 已被 { $name } 使用。如果您取代它， { $name } 將會被停用。
zoom-in = 放大
zoom-out = 縮小

## Input: Mouse

mouse = 滑鼠
    .speed = 滑鼠速度
    .acceleration = 啟用滑鼠加速

## Input: Touchpad

click-behavior = 點擊行為
    .click-finger = 兩指點擊為次要點擊，三指點擊為中鍵點擊
    .button-areas = 右下角點擊為次要點擊，中下角點擊為中鍵點擊
pinch-to-zoom = 雙指撥動縮放
    .desc = 可用雙指撥動來縮放內容，應用程式功能如有支援
tap-to-click = 滑觸點按
    .desc = 啟用單指滑觸視作點按滑鼠左鍵，雙指滑觸視作點按滑鼠右鍵，三指滑觸視作點按滑鼠中鍵
touchpad = 觸控板
    .acceleration = 啟用觸控板加速
    .speed = 觸控靈敏度

## Input: Gestures

gestures = 手勢
    .four-finger-down = 四指向下滑動
    .four-finger-left = 四指向左滑動
    .four-finger-right = 四指向右滑動
    .four-finger-up = 四指向上滑動
    .three-finger-any = 三指向任何方向滑動
switch-workspaces = 切換工作區
    .horizontal = 四指向左/右滑動
    .vertical = 四指向上/下滑動
switch-between-windows = 在視窗間切換
open-application-library = 開啟應用程式庫
open-workspaces-view = 開啟工作區總覽

## Time & Language

time = 時間與語言
time-date = 日期與時間
    .auto = 設定自動
    .auto-ntp = 設定時區後，日期與時間將會自動更新
time-zone = 時區
    .auto = 自動設定時區
    .auto-info = 需要定位服務與網際網路存取
time-format = 日期與時間格式
    .twenty-four = 24 小時制
    .show-seconds = 顯示秒數
    .first = 每週第一天
    .show-date = 在時間工具程式中顯示日期
    .friday = 星期五
    .saturday = 星期六
    .sunday = 星期日
    .monday = 星期一
time-region = 地區與語言
formatting = 格式設定
    .dates = 日期
    .time = 時間
    .date-and-time = 日期與時間
    .numbers = 數字
    .measurement = 度量衡
    .paper = 紙張
preferred-languages = 偏好語言
    .desc = 語言順序決定使用者介面使用的語言。變更將在下次登入時生效。
add-language = 新增語言
    .context = 新增語言
install-additional-languages = 安裝其他語言
region = 地區

## Applications

applications = 應用程式

## Applications: Default Applications

default-apps = 預設應用程式
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
    .not-installed = 尚未安裝

## Applications: Startup Applications

startup-apps = 啟動應用程式
    .add = 添增應用程式
    .user = 您登入時啟動的應用程式
    .none = 尚無添新增啟動應用程式
    .remove-dialog-title = 移除 { $name } 嗎？
    .remove-dialog-description = 是否移除此啟動應用程式？
    .add-startup-app = 添增啟動應用程式

## Applications: Legacy Applications

legacy-applications = X11 應用程式相容性
legacy-app-global-shortcuts = X11 應用程式中的整體快速鍵
    .desc = 整體快速鍵允許在應用程式中執行的按鍵和滑鼠按鈕事件被其他應用程式識別，用於「按鍵通話」或「按鍵靜音」等功能。預設情況下，此功能在 X11 應用程式中被停用，以確保其他應用程式無法監視包含敏感資訊的鍵盤和滑鼠事件。
    .none = 無按鍵
    .modifiers = 修飾鍵 (Super, Shift, Control, Alt)
    .combination = 當按下 Super、Control 或 Alt 修飾鍵時的全部按鍵
    .all = 全部按鍵
    .mouse = X11 應用程式中的滑鼠按鈕事件
legacy-app-scaling = X11 視窗系統應用程式縮放
    .scaled-gaming = 針對遊戲與全螢幕應用程式最佳化
    .gaming-description = X11 應用程式相較於 Wayland 應用程式可能顯得稍大/稍小
    .scaled-applications = 針對應用程式最佳化
    .applications-description = 遊戲與全螢幕 X11 應用程式可能無法符合您的顯示器解析度
    .scaled-compatibility = 最大相容性模式
    .compatibility-description = X11 應用程式在 HiDPI 螢幕上可能顯得模糊
    .preferred-display = 遊戲與全螢幕 X11 應用程式的偏好顯示器
    .no-display = 無

## System

system = 系統與帳號

## System: About

about = 關於
about-device = 裝置名稱
    .desc = 此名稱用於顯示給其他網路或藍牙裝置
about-hardware = 硬體
    .model = 硬體型號
    .memory = 記憶體
    .processor = 處理器
    .graphics = 顯示卡
    .disk-capacity = 磁碟容量
about-os = 作業系統
    .os = 系統
    .os-architecture = 系統架構
    .kernel = 核心版本
    .desktop-environment = 桌面環境
    .windowing-system = 視窗系統
about-related = 相關設定
    .support = 取得支援

## System: Firmware

firmware = 韌體

## System: Users

users = 使用者
    .admin = 管理員
    .standard = 標準
    .profile-add = 選擇個人資料圖片
administrator = 系統管理員
    .desc = 系統管理員可以變更全部使用者的設定，添增和移除其他使用者
add-user = 新增使用者
change-password = 變更密碼
remove-user = 移除使用者
full-name = 全名
invalid-username = 無效的使用者名稱
password-mismatch = 密碼與確認密碼必須相符
save = 儲存
xdg-entry-dock-comment = 可選的應用程式和工具程式的工具欄
xdg-entry-dock = 程式塢
xdg-entry-displays-keywords = COSMIC;顯示器;熒幕;
xdg-entry-displays-comment = 管理顯示器設定
xdg-entry-displays = 顯示器
xdg-entry-desktop-keywords = COSMIC;桌面;
xdg-entry-desktop = 桌面
xdg-entry-default-apps-keywords = COSMIC;預設;應用程式
xdg-entry-default-apps = 預設應用程式
xdg-entry-date-time-keywords = COSMIC;時間;時區;
xdg-entry-date-time-comment = 時區、自動時鐘設定和時間格式
xdg-entry-date-time = 日期與時間
xdg-entry-bluetooth-keywords = COSMIC;藍牙;
xdg-entry-bluetooth-comment = 管理藍牙裝置
xdg-entry-applications-keywords = COSMIC;預設;應用程式;啟動;X11;兼容性
xdg-entry-applications-comment = 預設應用程式、啟動應用程式和 X11 應用程式兼容性設定
xdg-entry-appearance-keywords = COSMIC;強調色;颜色;圖示;字體;主題
xdg-entry-appearance-comment = 強調色和主題
xdg-entry-applications = 應用程式
xdg-entry-appearance = 外觀
xdg-entry-a11y-keywords = COSMIC;無障礙;A11y;螢幕;閱讀器;放大鏡;對比;對比度;颜色;色彩;
xdg-entry-a11y-comment = 螢幕閱讀器、放大鏡、高對比以及濾色鏡
xdg-entry-a11y = 無障礙
xdg-entry-about-keywords = COSMIC;關於
xdg-entry-about-comment = 裝置名稱、硬體資訊、作業系統預設
xdg-entry-about = 關於
xdg-entry-keywords = COSMIC;設定;
xdg-entry-dock-keywords = COSMIC;程式塢;面板;工具程式
xdg-entry-region-language = 地區與語言
xdg-entry-notifications-keywords = COSMIC;通知;鎖定;
xdg-entry-power = 電源與電池
xdg-entry-panel-keywords = COSMIC;面板;工具程式
xdg-entry-panel = 面板
xdg-entry-notifications = 通知
xdg-entry-startup-apps = 開機啟動應用程式
xdg-entry-region-language-comment = 依據您所在的地區設定日期、時間和數字的格式
xdg-entry-notifications-comment = 勿擾模式、鎖定螢幕通知、及各個應用程式設定
xdg-entry-sound-keywords = COSMIC;音響;音訊;提醒;Pipewire;
xdg-entry-power-keywords = COSMIC;電源;電池
xdg-entry-region-language-keywords = COSMIC;地區;語言;日期;格式;時間;本地;本地化;
xdg-entry-sound-comment = 裝置、提醒和應用程式的音訊設定
xdg-entry-panel-comment = 主要系統列， 用於選單和工具程式
xdg-entry-sound = 音響
xdg-entry-power-comment = 電源模式和省電選項
xdg-entry-network-keywords = COSMIC;網路;無線;WiFi;VPN;
xdg-entry-network-comment = 管理網路連線
xdg-entry-network = 網路與無線
xdg-entry-mouse-keywords = COSMIC;滑鼠;加速度;滾動;
xdg-entry-mouse-comment = 滑鼠速度、加速度和自然滾動
xdg-entry-mouse = 滑鼠
xdg-entry-keyboard-keywords = COSMIC;鍵盤;輸入;輸入源;快速鍵;
xdg-entry-keyboard-comment = 輸入源、切換、特殊字符輸入、快速鍵
xdg-entry-keyboard = 鍵盤
xdg-entry-input-keywords = COSMIC;輸入;鍵盤;滑鼠;
xdg-entry-input-comment = 鍵盤與滑鼠設定
xdg-entry-input = 輸入裝置
xdg-entry-desktop-comment = 桌布、外觀、面板、程式塢、視窗管理和工作區設定
xdg-entry-default-apps-comment = 預設網頁瀏覽器、郵件用戶端、檔案管理器和其他應用程式
xdg-entry-touchpad = 觸控板
xdg-entry-wired = 有線連接
xdg-entry-startup-apps-comment = 當登入運行的應用程式進行調設
xdg-entry-startup-apps-keywords = COSMIC;開機啟動;應用程式;
xdg-entry-wired-keywords = COSMIC;有線連接;LAN;網路;連接;
xdg-entry-system = 系統與帳號
xdg-entry-window-management = 視窗管理
xdg-entry-time-language-comment = 管理系統日期、時間、區域及語言
xdg-entry-x11-applications-keywords = COSMIC;X11;應用程式;遊戲;相容性;
xdg-entry-time-language = 時間與語言
xdg-entry-users = 使用者
xdg-entry-system-keywords = COSMIC;系統;資訊;帳號;韌體;
xdg-entry-wireless-keywords = COSMIC;WiFi;Wi-Fi;網路;連接;
xdg-entry-wireless-comment = Wi-Fi 連接與連接設定檔案
xdg-entry-wallpaper = 背景圖片
xdg-entry-users-comment = 身份驗證與使用者帳號
xdg-entry-wallpaper-comment = 背景圖片、顏色及投影片選項
xdg-entry-wireless = Wi-Fi
xdg-entry-workspaces-keywords = COSMIC;工作區;方向;總覽;螢幕;
xdg-entry-system-comment = 系統資訊、帳號及韌體更新
xdg-entry-x11-applications-comment = X11 視窗系統應用程式縮放大小、主要顯示螢幕和整體快速鍵
xdg-entry-wallpaper-keywords = COSMIC;背景圖片;背景;投影片;
xdg-entry-users-keywords = COSMIC;使用者;帳戶;
xdg-entry-vpn-keywords = COSMIC;VPN;網路;連接;OpenVPN;OpenConnect;
xdg-entry-time-language-keywords = COSMIC;系統;時間;日期;地區;語言;
xdg-entry-touchpad-comment = 觸控板速度、點按選項及手勢
xdg-entry-vpn-comment = VPN 連接和連接設定檔案
xdg-entry-window-management-keywords = COSMIC;視窗;管理;平鋪;Super;鍵;
xdg-entry-workspaces-comment = 工作區方向、總覽及多重顯示器行為
xdg-entry-workspaces = 工作區
xdg-entry-x11-applications = X11 應用程式相容性
xdg-entry-window-management-comment = Super 鍵動作、視窗控制選項及附加視窗平鋪選項
xdg-entry-wired-comment = 有線連接和連接設定檔案
share = 分享網路
scan-to-connect-description = 掃描 QR 碼以連接至該網路。
qr-code-unavailable = QR 碼無可使用
network-name = 網路名稱
xdg-entry-touchpad-keywords = COSMIC;觸控板;手勢;
xdg-entry-comment = COSMIC 桌面設定應用程式
sound-device-profiles = 裝置設定檔案
sound-usb-audio = USB 音訊
sound-hd-audio = 高解析度音訊
sound-device-port-unplugged = 尚未插接
amplification = 音量擴大
    .desc = 允許將音量提高至 150%
workspaces-overview = 工作區總覽
    .action-on-typing = 輸入動作
    .none = 無
    .launcher = 開啟啟動器
    .applications = 開啟應用程式
place-here = 在此放置工具程式
shadow-and-corners = 視窗陰影與角型
shadows-tiling = 平鋪視窗
    .clip = 符合系統角型
    .shadow = 套用陰影
shadows-floating = 浮動視窗
    .clip = 符合系統角型和套用陰影
no-search-results = 沒有網路符合您所要搜尋的。
