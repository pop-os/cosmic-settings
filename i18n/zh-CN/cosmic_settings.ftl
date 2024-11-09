app = COSMIC 设置

dbus-connection-error = 无法连接到 DBus
ok = 确定
unknown = 未知

number = { $number }

## Network & Wireless

connections-and-profiles = { $variant ->
    [wired] 有线网络
    [wifi] Wi-Fi
    [vpn] VPN
    *[other] 未知
} 连接和连接配置文件。

add-network = 添加网络
    .profile = 添加配置文件
add-vpn = 添加 VPN
airplane-on = 飞行模式已开启。
cable-unplugged = 网线已拔出
connect = 连接
connected = 已连接
connecting = 正在连接…
disconnect = 断开连接
forget = 忘记
known-networks = 已知网络
network-and-wireless = 网络和无线
no-networks = 未找到任何网络。
no-vpn = 没有可用的 VPN 连接。
password = 密码
remove = 移除
settings = 设置
username = 用户名
visible-networks = 可见网络

auth-dialog = 需要认证
    .vpn-description = 请输入 VPN 服务所需的用户名和密码。
    .wifi-description = 请输入密码或加密密钥。您也可以通过按路由器上的"WPS"按钮来连接。

forget-dialog = 忘记此 Wi-Fi 网络？
    .description = 将来要使用此 Wi-Fi 网络时，您需要重新输入密码。

network-device-state =
    .activated = 已连接
    .config = 正在连接
    .deactivating = 正在断开连接
    .disconnected = 已断开连接
    .failed = 连接失败
    .ip-check = 正在检查连接
    .ip-config = 正在请求 IP 和路由信息
    .need-auth = 需要认证
    .prepare = 正在准备连接
    .secondaries = 正在等待次要连接
    .unavailable = 不可用
    .unknown = 未知状态
    .unmanaged = 未管理
    .unplugged = 网线已拔出

remove-connection-dialog = 移除连接配置文件？
    .vpn-description = 将来要使用此网络时，您需要重新输入密码。
    .wired-description = 将来要使用此配置文件时，您需要重新创建。

vpn = VPN
    .connections = VPN 连接
    .error = 添加 VPN 配置失败
    .remove = 移除连接配置文件
    .select-file = 选择 VPN 配置文件

vpn-error = VPN 错误
    .config = 添加 VPN 配置失败
    .connect = VPN 连接失败
    .connection-editor = 连接编辑器失败
    .connection-settings = 获取活动连接设置失败
    .updating-state = 更新网络管理器状态失败
    .wireguard-config-path = WireGuard 配置文件路径无效
    .wireguard-config-path-desc = 所选文件必须位于本地文件系统上。
    .wireguard-device = 创建 WireGuard 设备失败
    .with-password = 使用 nmcli 设置 VPN { $field ->
        *[username] 用户名
        [password] 密码
        [password-flags] 密码标志
    } 失败

wired = 有线网络
    .adapter = 有线网络适配器 { $id }
    .connections = 有线网络连接
    .devices = 有线网络设备
    .remove = 移除连接配置文件

wifi = Wi-Fi
    .adapter = Wi-Fi 适配器 { $id }
    .forget = 忘记此网络

wireguard-dialog = 添加 WireGuard 设备
    .description = 为 WireGuard 配置选择设备名称。

## Networking: Online Accounts

online-accounts = 在线账户
    .desc = 添加账户、IMAP 和 SMTP、企业登录

# Bluetooth

confirm = 确认

bluetooth = 蓝牙
    .desc = 管理蓝牙设备
    .status = 当蓝牙设置打开时，此系统显示为 { $aliases }。
    .connected = 已连接
    .connecting = 正在连接
    .disconnecting = 正在断开连接
    .connect = 连接
    .disconnect = 断开连接
    .forget = 忘记
    .dbus-error = 与 DBus 交互时发生错误：{ $why }
    .show-device-without-name = 显示无名称设备

bluetooth-paired = 之前连接的设备
    .connect = 连接
    .battery = 电池电量 { $percentage }%

bluetooth-confirm-pin = 确认蓝牙 PIN 码
    .description = 请确认以下 PIN 码与 { $device } 上显示的是否匹配

bluetooth-available = 附近的设备

bluetooth-adapters = 蓝牙适配器

## Desktop

desktop = 桌面

## Desktop: Wallpaper

wallpaper = 壁纸
  .change = 每隔
  .desc = 壁纸图片、颜色和幻灯片选项。
  .fit = 适合屏幕
  .folder-dialog = 选择壁纸文件夹
  .image-dialog = 选择壁纸图片
  .plural = 壁纸
  .same = 在所有显示器上使用相同的壁纸
  .slide = 幻灯片

add-color = 添加颜色
add-image = 添加图片
all-displays = 所有显示器
colors = 颜色
dialog-add = 添加
fill = 填充
fit-to-screen = 适合屏幕
open-new-folder = 打开新文件夹
recent-folders = 最近的文件夹

x-minutes = { $number } 分钟
x-hours = { $number ->
    [1] 1 小时
    *[other] { $number } 小时
}
never = 从不

## Desktop: Appearance

appearance = 外观
    .desc = 强调色和 COSMIC 主题。

accent-color = 强调色
app-background = 应用程序或窗口背景
auto = 自动
close = 关闭
color-picker = 颜色选择器
copied-to-clipboard = 已复制到剪贴板
copy-to-clipboard = 复制到剪贴板
dark = 深色
export = 导出
hex = 十六进制颜色代码
import = 导入
light = 浅色
mode-and-colors = 模式和颜色
recent-colors = 最近使用的颜色
reset-to-default = 恢复默认设置
rgb = RGB
window-hint-accent = 聚焦窗口边框颜色
window-hint-accent-toggle = 使用强调色作为聚焦窗口边框颜色

auto-switch = 在浅色和深色模式之间自动切换
    .sunrise = 日出时切换到浅色模式
    .sunset = 日落时切换到浅色模式
    .next-sunrise = 下一次日出时切换到浅色模式
    .next-sunset = 下一次日落时切换到浅色模式

container-background = 容器背景
    .desc-detail = 容器背景颜色用于导航侧边栏、侧边抽屉、对话框和类似的组件。默认情况下，它会自动通过应用程序或窗口背景颜色衍生。
    .reset = 恢复自动
    .desc = 主要容器颜色用于导航侧边栏、侧边抽屉、对话框和类似的组件。

control-tint = 控制组件颜色
    .desc = 用于按钮、搜索框、文本输入框和类似组件的背景。

frosted = 系统界面上的磨砂玻璃效果
    .desc = 将模糊效果应用于面板、程序坞、面板小部件、启动器和应用程序库。

enable-export = 将此主题应用于 GNOME 应用程序
    .desc = 并非所有toolkit都支持自动切换。非 COSMIC 应用程序可能需要在主题更改后重新启动以应用主题。

icon-theme = 图标主题
    .desc = 更改应用程序图标包。

text-tint = 界面文本颜色
    .desc = 用于在各种背景上具有足够对比度的界面文本颜色。

style = 样式
    .round = 大圆角
    .slightly-round = 中等圆角
    .square = 小圆角

interface-density = 界面密度
    .comfortable = 舒适
    .compact = 紧凑
    .spacious = 宽敞

window-management-appearance = 窗口管理
    .active-hint = 聚焦窗口边框大小
    .gaps = 平铺窗口的间距

### Experimental

experimental-settings = 实验性设置
icons-and-toolkit = 图标和工具包主题
interface-font = 系统字体
monospace-font = 等宽字体

## Desktop: Notifications

notifications = 通知
    .desc = 免打扰、锁屏通知和每个应用程序的通知设置。

## Desktop: Panel

panel = 面板
    .desc = 顶部面板，带有各种控件和菜单。

add = 添加
add-applet = 添加小部件
all = 全部
applets = 小部件
center-segment = 中间部分
drop-here = 将小部件拖放到此处
end-segment = 末尾部分
large = 大
no-applets-found = 未找到小部件...
panel-bottom = 底部
panel-left = 左侧
panel-right = 右侧
panel-top = 顶部
search-applets = 搜索小部件...
small = 小
start-segment = 开始部分

panel-appearance = 外观
    .match = 匹配主题颜色
    .light = 浅色
    .dark = 深色

panel-behavior-and-position = 行为和位置
    .autohide = 自动隐藏面板
    .dock-autohide = 自动隐藏程序坞
    .position = 屏幕上的位置
    .display = 显示在显示器上

panel-style = 样式
    .anchor-gap = 面板和屏幕边缘之间的间隙
    .dock-anchor-gap = 程序坞和屏幕边缘之间的间隙
    .extend = 将面板扩展到屏幕边缘
    .dock-extend = 将程序坞扩展到屏幕边缘
    .appearance = 外观
    .size = 大小
    .background-opacity = 背景不透明度

panel-applets = 配置
    .dock-desc = 配置程序坞小部件。
    .desc = 配置面板小部件。

panel-missing = 缺少面板配置
    .desc = 由于使用了自定义配置或配置已损坏，缺少面板配置文件。
    .fix = 恢复默认设置

## Desktop: Dock

dock = 程序坞
    .desc = 带有固定应用程序托盘和其他小部件的面板。

## Desktop: Window management

window-management = 窗口管理
    .desc = Super 键动作、窗口控制选项和其他窗口平铺选项。

super-key = Super 键动作
    .launcher = 打开启动器
    .workspaces = 打开工作区
    .applications = 打开应用程序
    .disable = 禁用

window-controls = 窗口控制
    .maximize = 显示最大化按钮
    .minimize = 显示最小化按钮
    .active-window-hint = 显示活动窗口提示

focus-navigation = 焦点导航
    .focus-follows-cursor = 焦点跟随光标
    .focus-follows-cursor-delay = 焦点跟随光标延迟（毫秒）
    .cursor-follows-focus = 光标跟随焦点

## Desktop: Workspaces

workspaces = 工作区
    .desc = 设置工作区数量、行为和排放方向。

workspaces-behavior = 工作区行为
    .dynamic = 动态工作区
    .dynamic-desc = 自动删除空工作区。
    .fixed = 固定数量的工作区
    .fixed-desc = 在概览中添加或删除工作区。

workspaces-multi-behavior = 多显示器行为
    .span = 工作区跨越显示器
    .separate = 显示器具有单独的工作区

workspaces-overview-thumbnails = 工作区概览缩略图
    .show-number = 显示工作区编号
    .show-name = 显示工作区名称

workspaces-orientation = 工作区排放方向
    .vertical = 垂直排放
    .horizontal = 水平排放

hot-corner = 热角
    .top-left-corner = 启用左上角热角以打开工作区
    
## Desktop: Display

-requires-restart = 需要重启

color = 颜色
    .depth = 色深
    .profile = 颜色配置文件
    .sidebar = 颜色配置文件
    .temperature = 色温

display = 显示器
    .desc = 管理显示器、显示切换和夜灯
    .arrangement = 显示器排列
    .arrangement-desc = 拖动显示器以重新排列它们。
    .enable = 启用显示器
    .external = { $size } { $output } 外部显示器
    .laptop = { $size } 笔记本电脑显示器
    .options = 显示器选项
    .refresh-rate = 刷新率
    .resolution = 分辨率
    .scale = 缩放

mirroring = 镜像显示
    .id = 镜像 { $id }
    .dont = 不镜像显示
    .mirror = 镜像 { $display }
    .project = 投射到 { $display ->
        [all] 所有显示器
        *[other] { $display }
    }
    .project-count = 投射到 { $count} 个其他 { $count ->
        [1] 显示器
        *[other] 显示器
    }

night-light = 夜灯
    .auto = 自动（日落到日出）
    .desc = 使用较暖的颜色以减少蓝光。

orientation = 显示方向
    .standard = 标准
    .rotate-90 = 旋转 90 度
    .rotate-180 = 旋转 180 度
    .rotate-270 = 旋转 270 度

scheduling = 计划
    .manual = 手动计划

dialog = 对话框
    .title = 保留这些显示设置？
    .keep-changes = 保留更改
    .change-prompt = 设置更改将在 { $time } 秒后自动恢复。
    .revert-settings = 恢复设置

legacy-applications = X11 窗口系统应用程序缩放
    .scaled-by-system = 缩放所有 X11 应用程序
    .system-description = X11 应用程序在 HiDPI 屏幕上会显示模糊。
    .scaled-natively = 以原生分辨率渲染 X11 应用程序
    .native-description = 不支持缩放的 X11 应用程序在使用 HiDPI 显示器时会显示很小。启用此选项可让游戏使用完整的显示器分辨率。

## Sound

sound = 声音
    .desc = N/A

sound-output = 声音输出
    .volume = 输出音量
    .device = 输出设备
    .level = 输出等级
    .config = 配置
    .balance = 平衡

sound-input = 声音输入
    .volume = 输入音量
    .device = 输入设备
    .level = 输入等级

sound-alerts = 提醒
    .volume = 提醒音量
    .sound = 提醒声音

sound-applications = 应用程序
    .desc = 应用程序音量和设置

profile = 配置文件

## Power

power = 电源和电池
    .desc = 管理电源设置

battery = 电池
  .minute = { $value } { $value ->
        [one] 分钟
       *[other] 分钟
  }
  .hour = { $value } { $value ->
        [one] 小时
       *[other] 小时
  }
  .day = { $value } { $value ->
        [one] 天
       *[other] 天
  }
  .less-than-minute = 不到一分钟
  .and = 和
  .remaining-time = 距离{ $action ->
        [full] 充满
       *[other] 耗尽
   }还有 { $time }

connected-devices = 已连接的设备
  .unknown = 未知设备

power-mode = 电源模式
    .battery = 延长电池寿命
    .battery-desc = 降低功耗并保持静音运行。
    .balanced = 平衡
    .balanced-desc = 安静运行并适度使用电源。
    .performance = 高性能
    .performance-desc = 最高性能和功耗。
    .no-backend = 未找到后端。请安装 system76-power 或 power-profiles-daemon。

power-saving = 省电选项
    .turn-off-screen-after = 在此时间后关闭屏幕
    .auto-suspend = 自动挂起
    .auto-suspend-ac = 接通电源时自动挂起
    .auto-suspend-battery = 使用电池时自动挂起

## Input

acceleration-desc = 根据速度自动调整跟踪灵敏度。

disable-while-typing = 键盘输入时禁用

input-devices = 输入设备
    .desc = 输入设备

primary-button = 主要按键
    .desc = 设置物理按键的顺序。
    .left = 左键
    .right = 右键

scrolling = 滚动
    .two-finger = 用两根手指滚动
    .edge = 用一根手指沿边缘滚动
    .speed = 滚动速度
    .natural = 自然滚动
    .natural-desc = 滚动内容，而不是视图

## Input: Keyboard

slow = 慢
fast = 快
short = 短
long = 长
keyboard = 键盘
    .desc = 键盘输入

keyboard-sources = 输入法
    .desc = 可以使用 Super+空格键切换输入法。这可以在键盘快捷键设置中自定义。
    .move-up = 向上移动
    .move-down = 向下移动
    .settings = 设置
    .view-layout = 查看键盘布局
    .remove = 移除
    .add = 添加输入法

keyboard-special-char = 特殊字符输入
    .alternate = 备用字符键
    .compose = 组合键
    .caps = Caps Lock 键

keyboard-typing-assist = 打字
    .repeat-rate = 重复速率
    .repeat-delay = 重复延迟

added = 已添加
type-to-search = 输入以搜索...
show-extended-input-sources = 显示扩展输入法

## Input: Keyboard: Shortcuts

keyboard-shortcuts = 键盘快捷键
    .desc = 查看和自定义快捷键

add-keybinding = 添加快捷键
cancel = 取消
command = 命令
custom = 自定义
debug = 调试
disabled = 已禁用
migrate-workspace-prev = 将工作区移至上一个输出
migrate-workspace-next = 将工作区移至下一个输出
migrate-workspace = 将工作区移至{ $direction ->
    *[down] 下方
    [left] 左侧
    [right] 右侧
    [up] 上方
}输出
navigate = 导航
replace = 替换
shortcut-name = 快捷键名称
system-controls = 系统控制
terminate = 终止
toggle-stacking = 切换窗口堆叠
type-key-combination = 输入按键组合

custom-shortcuts = 自定义快捷键
    .add = 添加快捷键
    .context = 添加自定义快捷键
    .none = 无自定义快捷键

modified = 已修改 { $count } 项

nav-shortcuts = 导航
    .prev-output = 聚焦上一个输出
    .next-output = 聚焦下一个输出
    .last-workspace = 聚焦最后一个工作区
    .prev-workspace = 聚焦上一个工作区
    .next-workspace = 聚焦下一个工作区
    .focus = 聚焦{ $direction ->
        *[down] 下方
        [in] 内部
        [left] 左侧
        [out] 外部
        [right] 右侧
        [up] 上方
    }窗口
    .output = 切换到{ $direction ->
        *[down] 下方
        [left] 左侧
        [right] 右侧
        [up] 上方
    }输出
    .workspace = 切换到工作区 { $num }

manage-windows = 管理窗口
    .close = 关闭窗口
    .maximize = 最大化窗口
    .minimize = 最小化窗口
    .resize-inwards = 向内调整窗口大小
    .resize-outwards = 向外调整窗口大小
    .toggle-sticky = 切换窗口置顶

move-windows = 移动窗口
    .direction = 将窗口移至{ $direction ->
        *[down] 下方
        [left] 左侧
        [right] 右侧
        [up] 上方
    }
    .display = 将窗口移至{ $direction ->
        *[down] 下方
        [left] 左侧
        [right] 右侧
        [up] 上方
    }显示器
    .workspace = 将窗口移至{ $direction ->
        *[below] 下方
        [left] 左侧
        [right] 右侧
        [above] 上方
    }工作区
    .workspace-num = 将窗口移至工作区 { $num }
    .prev-workspace = 将窗口移至上一个工作区
    .next-workspace = 将窗口移至下一个工作区
    .last-workspace = 将窗口移至最后一个工作区
    .next-display = 将窗口移至下一个显示器
    .prev-display = 将窗口移至上一个显示器
    .send-to-prev-workspace = 将窗口移至上一个工作区
    .send-to-next-workspace = 将窗口移至下一个工作区

system-shortcut = 系统
    .app-library = 打开应用程序库
    .brightness-down = 降低显示器亮度
    .brightness-up = 提高显示器亮度
    .home-folder = 打开主文件夹
    .keyboard-brightness-down = 降低键盘亮度
    .keyboard-brightness-up = 提高键盘亮度
    .launcher = 打开启动器
    .lock-screen = 锁定屏幕
    .mute = 静音
    .mute-mic = 麦克风静音
    .play-pause = 播放/暂停
    .play-next = 下一曲
    .play-prev = 上一曲
    .screenshot = 截图
    .terminal = 打开终端
    .volume-lower = 降低音量
    .volume-raise = 提高音量
    .web-browser = 打开网络浏览器
    .window-switcher = 在打开的窗口间切换
    .workspace-overview = 打开工作区概览

window-tiling = 窗口平铺
    .horizontal = 设置水平方向
    .vertical = 设置垂直方向
    .swap-window = 交换窗口
    .toggle-tiling = 切换窗口平铺
    .toggle-stacking = 切换窗口堆叠
    .toggle-floating = 切换窗口浮动
    .toggle-orientation = 切换方向

replace-shortcut-dialog = 替换快捷键？
    .desc = { $shortcut } 已被 { $name } 使用。如果替换它，{ $name } 将被禁用。

## Input: Mouse

mouse = 鼠标
    .desc = 鼠标速度、加速、自然滚动。
    .speed = 鼠标速度
    .acceleration = 启用鼠标加速

## Input: Touchpad

click-behavior = 点击行为
    .click-finger = 用两根手指进行右键点击，用三根手指进行中键点击
    .button-areas = 在右下角进行右键点击，在底部中心进行中键点击

pinch-to-zoom = 捏合缩放
    .desc = 用两根手指缩放内容，仅适用于支持缩放的应用程序。

tap-to-click = 轻触点击
    .desc = 启用单指轻触进行点击，双指轻触进行右键点击，三指轻触进行中键点击。

touchpad = 触摸板
    .acceleration = 启用触摸板加速
    .desc = 触摸板速度、点击选项、手势。
    .speed = 触摸板速度

## Input: Gestures

swiping = 滑动
    .four-finger-down = 四指向下滑动
    .four-finger-left = 四指向左滑动
    .four-finger-right = 四指向右滑动
    .four-finger-up = 四指向上滑动
    .three-finger-any = 三指向任意方向滑动

switch-workspaces = 切换工作区
    .horizontal = 四指左右滑动
    .vertical = 四指上下滑动

switch-between-windows = 在窗口之间切换
open-application-library = 打开应用程序库
open-workspaces-view = 打开工作区概览

## Time & Language

time = 时间和语言
    .desc = N/A

time-date = 日期和时间
    .desc = 时区、自动设置时间和时间格式。
    .auto = 自动设置时间
    .auto-ntp = 当设置时区时，日期和时间将自动更新。

time-zone = 时区
    .auto = 自动确定时区
    .auto-info = 需要定位服务和互联网访问

time-format = 日期和时间格式
    .twenty-four = 24 小时制
    .show-seconds = 显示秒
    .first = 一周的第一天
    .show-date = 在顶部面板上显示日期
    .friday = 星期五
    .saturday = 星期六
    .sunday = 星期日
    .monday = 星期一

time-region = 区域和语言
    .desc = 根据您的区域确定日期、时间和数字格式

## System

system = 系统和帐户

## System: About

about = 关于
    .desc = 设备名称、硬件信息、操作系统信息。

about-device = 设备名称
    .desc = 此名称将显示给其他网络或蓝牙设备。

about-hardware = 硬件
    .model = 硬件型号
    .memory = 内存
    .processor = 处理器
    .graphics = 显卡
    .disk-capacity = 磁盘容量

about-os = 操作系统
    .os = 操作系统
    .os-architecture = 操作系统架构
    .desktop-environment = 桌面环境
    .windowing-system = 窗口系统

about-related = 相关设置
    .support = 获取支持

## System: Firmware

firmware = 固件
    .desc = 固件详细信息。

## System: Users

users = 用户
    .desc = 身份验证和登录、锁屏。