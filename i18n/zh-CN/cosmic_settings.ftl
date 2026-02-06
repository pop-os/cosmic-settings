app = COSMIC 设置
dbus-connection-error = 无法连接到 DBus
ok = 确定
unknown = 未知
number = { $number }

## Network & Wireless

connections-and-profiles =
    { $variant ->
        [wired] 有线
        [wifi] Wi-Fi
        [vpn] VPN
       *[other] 未知
    }连接和连接配置。
add-network = 添加网络
    .profile = 添加配置
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
password-confirm = 确认密码
remove = 移除
settings = 设置
username = 用户名
visible-networks = 可见网络
identity = 标识
auth-dialog = 需要认证
    .vpn-description = 请输入 VPN 服务要求的用户名和密码。
    .wifi-description = 输入密码或加密密钥。您也可以通过路由器上的“WPS”按钮来连接。
forget-dialog = 忘记此 Wi-Fi 网络？
    .description = 未来使用此 Wi-Fi 网络时需要重新输入密码。
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
    .secondaries = 等待次要连接
    .unavailable = 不可用
    .unknown = 未知状态
    .unmanaged = 未管理
    .unplugged = 网线已拔出
remove-connection-dialog = 确定移除连接配置文件？
    .vpn-description = 未来使用此网络时需要重新输入密码。
    .wired-description = 未来使用此网络时需要重新创建配置文件。
vpn = VPN
    .connections = VPN 连接
    .error = 添加 VPN 配置失败
    .remove = 移除连接配置文件
    .select-file = 选择 VPN 配置文件
vpn-error = VPN 错误
    .config = 添加 VPN 配置失败
    .connect = 连接 VPN 失败
    .connection-editor = 连接编辑器失败
    .connection-settings = 获取活跃连接设置失败
    .updating-state = 更新网络管理器状态失败
    .wireguard-config-path = WireGuard 配置文件路径无效
    .wireguard-config-path-desc = 所选文件必须在本地文件系统上。
    .wireguard-device = 创建 WireGuard 设备失败
    .with-password =
        使用 nmcli 设置 VPN { $field ->
           *[username] 用户名
            [password] 密码
            [password-flags] 密码标志
        }失败
wired = 有线网络
    .adapter = 有线网络适配器 { $id }
    .connections = 有线连接
    .devices = 有线设备
    .remove = 移除连接配置文件
wifi = Wi-Fi
    .adapter = Wi-Fi 适配器 { $id }
    .forget = 忘记此网络
wireguard-dialog = 添加 WireGuard 设备
    .description = 为 WireGuard 配置选择设备名称。

## Networking: Online Accounts

online-accounts = 在线账户
    .desc = 添加账户、IMAP与SMTP设置、企业登录

# Bluetooth

activate = 激活
confirm = 确认
enable = 启用
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
    .disabled = 蓝牙服务已禁用
    .inactive = 蓝牙服务未激活
    .unknown = 无法激活蓝牙服务。是否已安装 Bluez？
bluetooth-paired = 已连接过的设备
    .connect = 连接
    .battery = 电量 { $percentage }%
bluetooth-confirm-pin = 确认蓝牙 PIN 码
    .description = 请确认以下 PIN 码与 { $device } 上显示的是否匹配
bluetooth-available = 附近设备
bluetooth-adapters = 蓝牙适配器

## Accessibility

accessibility = 无障碍
    .vision = 视觉
    .on = 开启
    .off = 关闭
    .unavailable = 不可用
    .screen-reader = 屏幕阅读器
    .high-contrast = 高对比度模式
    .invert-colors = 颜色反转
    .color-filters = 色彩滤镜
hearing = 听觉
    .mono = 播放立体声音频为单声道
default = 默认
magnifier = 放大镜
    .controls =
        或使用这些快捷键：{ $zoom_in ->
            [zero] { "" }
           *[other]
                { "" }
                按 { $zoom_in } 放大，
        }{ $zoom_out ->
            [zero] { "" }
           *[other]
                { "" }
                按 { $zoom_out } 缩小，
        }
        按 Super 并滚动鼠标
    .scroll_controls = 按 Super 并使用鼠标或触摸板滚动来启用缩放
    .show_overlay = 显示放大镜叠加层
    .increment = 缩放增量
    .signin = 登录时启动放大镜
    .applet = 在面板上的小部件中切换放大镜开关
    .movement = 放大视图移动
    .continuous = 跟随指针连续移动
    .onedge = 当指针到达边缘时
    .centered = 保持指针居中
color-filter = 色彩滤镜类型
    .unknown = 未知滤镜已激活
    .greyscale = 灰度
    .deuteranopia = 绿色/红色（绿色弱视，绿色盲）
    .protanopia = 红色/绿色（红色弱视，红色盲）
    .tritanopia = 蓝色/黄色（蓝色弱视，蓝色盲）

## Desktop

desktop = 桌面

## Desktop: Wallpaper

wallpaper = 壁纸
    .change = 图片更换间隙
    .desc = 壁纸图片、颜色和幻灯片选项。
    .fit = 壁纸适应方式
    .folder-dialog = 选择壁纸文件夹
    .image-dialog = 选择壁纸图片
    .plural = 壁纸
    .same = 所有显示器使用相同壁纸
    .slide = 幻灯片
add-color = 添加颜色
add-image = 添加图片
all-displays = 所有显示器
colors = 颜色
dialog-add = 添加
fill = 填充
fit-to-screen = 适应屏幕
open-new-folder = 打开新文件夹
recent-folders = 最近文件夹
x-minutes =
    { $number } { $number ->
        [one] 分钟
       *[other] 分钟
    }
x-hours =
    { $number ->
        [one] 小时
       *[other] { $number } 小时
    }
never = 从不

## Desktop: Appearance

appearance = 外观
    .desc = 主题色和主题设置
accent-color = 主题色
app-background = 窗口背景
auto = 自动
close = 关闭
color-picker = 取色器
copied-to-clipboard = 已复制到剪切板
copy-to-clipboard = 复制到剪切板
dark = 暗色模式
export = 导出
hex = 十六进制
import = 导入
light = 亮色模式
mode-and-colors = 模式和颜色
recent-colors = 最近使用的颜色
reset-to-default = 重置为默认值
rgb = RGB
window-hint-accent = 活动窗口提示颜色
window-hint-accent-toggle = 使用主题色作为活动窗口提示色
auto-switch = 自动在亮色与暗色模式之间切换
    .sunrise = 日出时切换到亮色模式
    .sunset = 日落时切换到暗色模式
    .next-sunrise = 下次日出时切换到亮色模式
    .next-sunset = 下次日落时切换到暗色模式
container-background = 容器背景
    .desc-detail = 容器背景色用于导航侧栏、侧边抽屉、对话框等小部件。默认情况下，容器背景色会自动从窗口背景派生。
    .reset = 重置为自动
    .desc = 用于导航侧栏、侧边抽屉、对话框等小部件
control-tint = 控件色调
    .desc = 用于标准按钮、搜索输入框、文本输入框等组件的背景
frosted = 系统界面磨砂玻璃效果
    .desc = 对面板、程序坞、小部件、启动器和应用程序库应用背景模糊效果
enable-export = 将此主题应用到 GNOME 应用程序
    .desc = 并非所有工具包都支持自动切换。非 COSMIC 应用程序可能需要在主题更改后重新启动。
icon-theme = 图标主题
    .desc = 为应用程序应用另一套图标集
text-tint = 界面文字色调
    .desc = 用于派生在各种界面背景上都具有足够对比度的文字颜色
style = 样式
    .round = 圆角
    .slightly-round = 微圆角
    .square = 方角
interface-density = 界面密度
    .comfortable = 适中
    .compact = 紧凑
    .spacious = 宽裕
window-management-appearance = 窗口管理
    .active-hint = 活动窗口提示大小
    .gaps = 平铺窗口周围的间隙

### Experimental

experimental-settings = 实验性设置
icons-and-toolkit = 图标和工具包主题
interface-font = 系统字体
monospace-font = 等宽字体

## Desktop: Notifications

notifications = 通知
    .desc = 勿扰模式、锁屏通知以及各应用程序的通知设置

## Desktop: Panel

panel = 面板
    .desc = 用于菜单和小部件的主系统栏
add = 添加
add-applet = 添加小部件
all = 全部
applets = 小部件
center-segment = 中间区段
end-segment = 结束区段
large = 大
no-applets-found = 未找到小部件...
panel-bottom = 底部
panel-left = 左侧
panel-right = 右侧
panel-top = 顶部
search-applets = 搜索小部件...
small = 小
start-segment = 开始区段
panel-appearance = 外观
    .match = 匹配桌面
    .light = 亮色
    .dark = 暗色
panel-behavior-and-position = 行为和位置
    .autohide = 自动隐藏面板
    .dock-autohide = 自动隐藏程序坞
    .position = 屏幕位置
    .display = 在显示器上显示
panel-style = 样式
    .anchor-gap = 面板与屏幕边缘的间距
    .dock-anchor-gap = 程序坞与屏幕边缘的间距
    .extend = 延伸面板至屏幕边缘
    .dock-extend = 延伸程序坞至屏幕边缘
    .appearance = 外观
    .size = 大小
    .background-opacity = 背景不透明度
panel-applets = 配置
    .dock-desc = 配置程序坞部件
    .desc = 配置面板小部件
panel-missing = 缺少面板配置
    .desc = 由于使用自定义配置或配置文件已损坏，面板配置文件丢失。
    .fix = 重置为默认值

## Desktop: Dock

dock = 程序坞
    .desc = 用于应用程序和小部件的可选栏

## Desktop: Window management

window-management = 窗口管理
    .desc = Super 键动作、窗口控制选项和额外的窗口平铺选项
super-key = Super 键动作
    .launcher = 打开启动器
    .workspaces = 打开工作区
    .applications = 打开应用程序
    .disable = 禁用
edge-gravity = 悬浮窗口吸附到附近边缘
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
    .desc = 工作区方向和行为
workspaces-behavior = 工作区行为
    .dynamic = 动态工作区
    .dynamic-desc = 自动移除空的工作区
    .fixed = 固定数量工作区
    .fixed-desc = 在总览视图中添加或移除工作区。
workspaces-multi-behavior = 多显示器行为
    .span = 工作区跨显示器
    .separate = 显示器有独立工作区
workspaces-overview-thumbnails = 工作区总览缩略
    .show-number = 显示工作区编号
    .show-name = 显示工作区名称
workspaces-orientation = 工作区方向
    .vertical = 垂直
    .horizontal = 水平
hot-corner = 热角
    .top-left-corner = 启用左上角热角以打开工作区

## Display

-requires-restart = 需要重启
color = 颜色
    .depth = 色深
    .profile = 颜色配置文件
    .sidebar = 颜色配置文件
    .temperature = 色温
display = 显示
    .desc = 管理显示器和夜灯
    .arrangement = 显示器排列
    .arrangement-desc = 拖动显示器来重新排列
    .enable = 启用显示器
    .external = { $size } { $output } 的外部显示器
    .laptop = { $size } 的笔记本显示器
    .options = 显示选项
    .refresh-rate = 刷新率
    .resolution = 分辨率
    .scale = 缩放
    .additional-scale-options = 其他缩放选项
mirroring = 镜像
    .id = 镜像 { $id }
    .dont = 不镜像
    .mirror = 镜像到 { $display }
    .project =
        投影到 { $display ->
            [all] 所有显示器
           *[other] { $display }
        }
    .project-count =
        正在投影到 { $count } 个其他{ $count ->
            [1] 显示器
           *[other] 显示器
        }
night-light = 夜光
    .auto = 自动（日落到日出）
    .desc = 通过更暖的颜色减少蓝光
orientation = 方向
    .standard = 标准
    .rotate-90 = 旋转 90 度
    .rotate-180 = 旋转 180 度
    .rotate-270 = 旋转 270 度
vrr = 可变刷新率
    .enabled = 已启用
    .force = 始终启用
    .auto = 自动
    .disabled = 已禁用
scheduling = 日程
    .manual = 手动日程
dialog = 对话框
    .title = 保留这些显示设置？
    .keep-changes = 保留更改
    .change-prompt = 显示设置将在 { $time } 秒后自动还原。
    .revert-settings = 还原设置

## Sound

sound = 声音
    .desc = 音频设置
sound-output = 输出
    .volume = 输出音量
    .device = 输出设备
    .level = 输出电平
    .config = 配置
    .balance = 平衡
    .left = 左
    .right = 右
sound-input = 输入
    .volume = 输入音量
    .device = 输入设备
    .level = 输入电平
sound-alerts = 警告音
    .volume = 警告音音量
    .sound = 警告音效
sound-applications = 应用程序
    .desc = 应用程序音量和设置

## Power

power = 电源和电池
    .desc = 管理电源设置
battery = 电池
    .minute =
        { $value } { $value ->
            [1] 分钟
           *[other] 分钟
        }
    .hour =
        { $value } { $value ->
            [1] 小时
           *[other] 小时
        }
    .day =
        { $value } { $value ->
            [1] 天
           *[other] 天
        }
    .less-than-minute = 不到一分钟
    .and = 和
    .remaining-time =
        { $time }后{ $action ->
            [full] 充满
           *[other] 电量耗尽
        }
connected-devices = 已连接设备
    .unknown = 未知设备
power-mode = 电源模式
    .battery = 延长电池续航
    .battery-desc = 降低功耗并保持静音运行
    .balanced = 平衡
    .balanced-desc = 静音运行和适中的功耗
    .performance = 高性能
    .performance-desc = 最高性能和功耗
    .no-backend = 未找到后端服务。请安装 system76-power 或 power-profiles-daemon。
power-saving = 节能选项
    .turn-off-screen-after = 在此时间后关闭屏幕
    .auto-suspend = 自动待机
    .auto-suspend-ac = 接通电源时自动挂起
    .auto-suspend-battery = 使用电池时自动挂起

## Input

acceleration-desc = 根据速度自动调整跟踪灵敏度
disable-while-typing = 打字时禁用触摸板
input-devices = 输入设备
    .desc = 输入设备
primary-button = 主按键
    .desc = 设置物理按键的顺序
    .left = 左
    .right = 右
scrolling = 滚动
    .two-finger = 双指滚动
    .edge = 单指边缘滚动
    .speed = 滚动速度
    .natural = 自然滚动
    .natural-desc = 滚动内容而不是视图

## Input: Keyboard

slow = 慢
fast = 快
short = 短
long = 长
keyboard = 键盘
    .desc = 输入源、切换方式、特殊字符输入和快捷键
keyboard-sources = 输入源
    .desc = 可以使用 Super + 空格键组合切换输入源。可以在键盘快捷键设置中自定义。
    .move-up = 上移
    .move-down = 下移
    .settings = 设置
    .view-layout = 查看键盘布局
    .remove = 移除
    .add = 添加输入源
keyboard-special-char = 特殊字符输入
    .alternate = 备用字符键
    .compose = 组合键 (Compose Key)
    .compose-desc = 组合键允许输入种类繁多的字符。使用方法：按下组合键，然后依次输入字符序列。例如，按下组合键后依次输入 C 和 o 可输入 ©，而按下组合键后依次输入 a 和 ‘ 则可输入 á。
    .caps = 大写锁定键
keyboard-typing-assist = 打字
    .repeat-rate = 重复率
    .repeat-delay = 重复延迟
keyboard-numlock-boot = 数字锁定
    .boot-state = 启动时状态
    .last-boot = 上次启动
    .on = 开启
    .off = 关闭
    .set = 设置数字锁定启动状态
added = 已添加
type-to-search = 输入即可搜索...
show-extended-input-sources = 显示扩展输入源

## Input: Keyboard: Shortcuts

keyboard-shortcuts = 键盘快捷键
    .desc = 查看并自定义快捷键
add-another-keybinding = 添加另一个快捷键
cancel = 取消
command = 命令
custom = 自定义
debug = 调试
disabled = 已禁用
input-source-switch = 切换键盘语言输入源
migrate-workspace-prev = 将工作区移至上一个输出
migrate-workspace-next = 将工作区移至下一个输出
migrate-workspace =
    将工作区移至{ $direction ->
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
    .none = 暂无自定义快捷键
modified = 已修改 { $count } 项
nav-shortcuts = 导航快捷键
    .prev-output = 聚焦上一个输出
    .next-output = 聚焦下一个输出
    .last-workspace = 聚焦最后一个工作区
    .prev-workspace = 聚焦上一个工作区
    .next-workspace = 聚焦下一个工作区
    .focus =
        聚焦{ $direction ->
           *[down] 下方
            [in] 内部
            [left] 左侧
            [out] 外部
            [right] 右侧
            [up] 上方
        }窗口
    .output =
        切换到{ $direction ->
           *[down] 下方
            [left] 左侧
            [right] 右侧
            [up] 上方
        }输出
    .workspace = 切换到工作区 { $num }
manage-windows = 管理窗口
    .close = 关闭窗口
    .maximize = 最大化窗口
    .fullscreen = 窗口全屏
    .minimize = 最小化窗口
    .resize-inwards = 向内调整窗口大小
    .resize-outwards = 向外调整窗口大小
    .toggle-sticky = 切换窗口置顶
move-windows = 移动窗口
    .direction =
        将窗口向{ $direction ->
           *[down] 下
            [left] 左
            [right] 右
            [up] 上
        }移动
    .display =
        将窗口移动到{ $direction ->
           *[down] 下方
            [left] 左侧
            [right] 右侧
            [up] 上方
        }的显示器
    .workspace =
        将窗口移动到{ $direction ->
           *[below] 下方
            [left] 左侧
            [right] 右侧
            [above] 上方
        }的工作区
    .workspace-num = 将窗口移动到工作区 { $num }
    .prev-workspace = 将窗口移动到上一个工作区
    .next-workspace = 将窗口移动到下一个工作区
    .last-workspace = 将窗口移动到最后一个工作区
    .next-display = 将窗口移动到下一个显示器
    .prev-display = 将窗口移动到上一个显示器
    .send-to-prev-workspace = 将窗口移动到上一个工作区
    .send-to-next-workspace = 将窗口移动到下一个工作区
system-shortcut = 系统快捷键
    .app-library = 打开应用程序库
    .brightness-down = 降低显示器亮度
    .brightness-up = 提高显示器亮度
    .display-toggle = 切换内部显示
    .home-folder = 打开主目录
    .keyboard-brightness-down = 降低键盘背光亮度
    .keyboard-brightness-up = 提高键盘背光亮度
    .launcher = 打开启动器
    .log-out = 登出
    .lock-screen = 锁定屏幕
    .mute = 静音
    .mute-mic = 麦克风静音
    .play-pause = 播放/暂停
    .play-next = 下一曲
    .play-prev = 上一曲
    .poweroff = 关机
    .screenshot = 截图
    .suspend = 待机
    .terminal = 打开终端
    .touchpad-toggle = 切换触摸板
    .volume-lower = 降低音量
    .volume-raise = 提高音量
    .web-browser = 打开网络浏览器
    .window-switcher = 切换打开的窗口
    .window-switcher-previous = 反向切换打开的窗口
    .workspace-overview = 打开工作区概览
window-tiling = 窗口平铺
    .horizontal = 设置水平布局
    .vertical = 设置垂直布局
    .swap-window = 交换窗口
    .toggle-tiling = 开关窗口平铺
    .toggle-stacking = 开关窗口堆叠
    .toggle-floating = 开关窗口悬浮
    .toggle-orientation = 切换布局方向
replace-shortcut-dialog = 是否替换快捷键？
    .desc = { $shortcut } 已被 { $name } 占用。如果替换它，{ $name } 将被停用。
zoom-in = 放大
zoom-out = 缩小

## Input: Mouse

mouse = 鼠标
    .desc = 鼠标速度、加速度、自然滚动
    .speed = 鼠标速度
    .acceleration = 启用鼠标加速度

## Input: Touchpad

click-behavior = 点击行为
    .click-finger = 双指点击为右键点击，三指点击为中键点击
    .button-areas = 右下角点击为右键点击，底部中间点击为中键点击
pinch-to-zoom = 双指捏合缩放
    .desc = 使用双指捏合手势缩放内容（适用于支持缩放的应用程序）
tap-to-click = 轻触点击
    .desc = 启用单指轻触进行主点击，双指轻触进行次点击，三指轻触进行中键点击
touchpad = 触控板
    .acceleration = 启用触控板加速度
    .desc = 触控板速度、点击选项、手势
    .speed = 触控板速度

## Input: Gestures

gestures = 手势
    .four-finger-down = 四指向下滑动
    .four-finger-left = 四指向左滑动
    .four-finger-right = 四指向右滑动
    .four-finger-up = 四指向上滑动
    .three-finger-any = 三指任意方向滑动
switch-workspaces = 切换工作区
    .horizontal = 四指左右滑动
    .vertical = 四指上下滑动
switch-between-windows = 在窗口间切换
open-application-library = 打开应用程序库
open-workspaces-view = 打开工作区总览

## Time & Language

time = 时间和语言
    .desc = 不可用
time-date = 日期与时间
    .desc = 时区、自动时钟设置与时间格式
    .auto = 自动设置
    .auto-ntp = 当时区设置好后，日期与时间将自动更新
time-zone = 时区
    .auto = 自动设置时区
    .auto-info = 需要位置服务和互联网连接
time-format = 日期与时间格式
    .twenty-four = 24 小时制
    .show-seconds = 显示秒数
    .first = 每周第一天
    .show-date = 在时间小部件中显示日期
    .friday = 星期五
    .saturday = 星期六
    .sunday = 星期日
    .monday = 星期一
time-region = 区域与语言
    .desc = 根据您所在的区域设置日期、时间和数字的格式
formatting = 格式设置
    .dates = 日期
    .time = 时间
    .date-and-time = 日期和时间
    .numbers = 数字
    .measurement = 度量单位
    .paper = 纸张
preferred-languages = 首选语言
    .desc = 语言的排序决定了用户界面所使用的语言。更改将在下次登录后生效。
add-language = 添加语言
    .context = 添加语言
install-additional-languages = 安装其他语言
region = 区域

## Applications

applications = 应用程序

## Applications: Default Applications

default-apps = 默认应用程序
    .desc = 默认网络浏览器、邮件客户端、文件浏览器和其他应用程序。
    .web-browser = 网络浏览器
    .file-manager = 文件管理器
    .mail-client = 邮件客户端
    .music = 音乐
    .video = 视频
    .photos = 照片
    .calendar = 日历
    .terminal = 终端
    .other-associations = 其他关联
    .text-editor = 文本编辑器
    .not-installed = 未安装

## Applications: Startup Applications

startup-apps = 自动启动应用程序
    .desc = 配置登录时运行的应用程序。
    .add = 添加应用程序
    .user = 登录时启动的应用程序
    .none = 未添加自动启动应用程序
    .remove-dialog-title = 移除 { $name } 吗？
    .remove-dialog-description = 是否删除此自动启动应用程序？
    .add-startup-app = 添加自动启动应用程序

## Applications: Legacy Applications

legacy-applications = X11 应用程序兼容性
    .desc = X11 窗口系统应用程序缩放与全局快捷键
legacy-app-global-shortcuts = X11 应用程序中的全局快捷键
    .desc = 全局快捷键允许其他应用程序识别在应用程序中执行的击键和鼠标按钮事件，以实现按下通话或按下静音等功能。默认情况下，X11 应用程序中禁用全局快捷键，以确保其他应用程序无法监控包含敏感信息的键盘和鼠标事件。
    .none = 无按键
    .modifiers = 修饰键 (Super、Shift、Control、Alt)
    .combination = 当修饰键 Super、Control 或 Alt 被按下时的所有按键
    .all = 所有按键
    .mouse = X11 应用程序中的鼠标按钮事件
legacy-app-scaling = X11 窗口系统应用程序缩放
    .scaled-gaming = 为游戏和全屏应用优化
    .gaming-description = 与 Wayland 应用相比，X11 应用程序可能看起来稍大/稍小
    .scaled-applications = 为应用程序优化
    .applications-description = 游戏和全屏 X11 应用可能与您的显示分辨率不匹配
    .scaled-compatibility = 最大兼容性模式
    .compatibility-description = X11 应用程序在高分辨率屏幕上可能显示模糊
    .preferred-display = 游戏和全屏 X11 应用程序的首选显示器
    .no-display = 无

## System

system = 系统与账户

## System: About

about = 关于
    .desc = 设备名称、硬件信息、操作系统默认设置
about-device = 设备名称
    .desc = 此名称将显示给其他网络或蓝牙设备
about-hardware = 硬件
    .model = 硬件型号
    .memory = 内存
    .processor = 处理器
    .graphics = 图形处理器
    .disk-capacity = 磁盘容量
about-os = 操作系统
    .os = 操作系统
    .os-architecture = 操作系统架构
    .kernel = 内核版本
    .desktop-environment = 桌面环境
    .windowing-system = 窗口系统
about-related = 相关设置
    .support = 获取支持

## System: Firmware

firmware = 固件
    .desc = 固件详细信息

## System: Users

users = 用户
    .desc = 身份验证与用户账户
    .admin = 管理员
    .standard = 标准用户
    .profile-add = 选择个人资料图像
administrator = 管理员
    .desc = 管理员可以为所有用户更改设置、添加和移除其他用户
add-user = 添加用户
change-password = 修改密码
remove-user = 删除用户
full-name = 全名
invalid-username = 无效的用户名
password-mismatch = 密码与确认密码必须匹配
save = 保存
amplification = 音量增强
    .desc = 允许将音量提高到150%
network-name = 网络名称
place-here = 将小部件放置于此
qr-code-unavailable = 二维码不可用
share = 分享网络
scan-to-connect-description = 扫描二维码即可连接到该网络。
sound-device-port-unplugged = 未插入
sound-hd-audio = 高清音频
sound-usb-audio = USB 音频
sound-device-profiles = 设备配置文件
shadows-floating = 悬浮窗口
    .clip = 匹配系统角落并且应用阴影
shadows-tiling = 平铺窗口
    .clip = 匹配系统角落
    .shadow = 应用阴影
shadow-and-corners = 窗口阴影与角落
