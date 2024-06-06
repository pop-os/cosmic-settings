app = COSMIC 设置

unknown = 未知

number = { $number }

## Desktop

desktop = 桌面

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

experimental-settings = 实验性设置

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

# interface density left out for now
window-management = 窗口管理
    .active-hint = 聚焦窗口边框大小
    .gaps = 平铺窗口的间距

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

## Desktop: Notifications

notifications = 通知
    .desc = 免打扰、锁屏通知和每个应用程序的通知设置。

## Desktop: Options

desktop-panel-options = 桌面和面板选项
    .desc = Super 键行为、热区、窗口控件。

desktop-panels-and-applets = 桌面面板和小部件

dock = 程序坞
    .desc = 带有固定应用程序的面板。

hot-corner = 热区
    .top-left-corner = 启用左上角热区来查看工作区概览

top-panel = 顶部面板
    .workspaces = 显示工作区概览按钮
    .applications = 显示应用程序库按钮

window-controls = 窗口控件
    .minimize = 显示最小化按钮
    .maximize = 显示最大化按钮

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
dialog-add = _添加
fill = 填充
fit-to-screen = 适合屏幕
open-new-folder = 打开新文件夹
recent-folders = 最近的文件夹

x-minutes = { $number } 分钟
x-hours = { $number ->
    [1] 1 小时
    *[other] { $number } 小时
}

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

## Networking: Wired

wired = 有线
    .desc = 有线连接、连接配置文件

## Networking: Online Accounts

online-accounts = 在线帐户
    .desc = 添加帐户、IMAP 和 SMTP、企业登录

## Time & Language

time = 时间和语言
    .desc = N/A

time-date = 日期和时间
    .desc = 时区、自动设置时间和时间格式。
    .auto = 自动设置时间

time-zone = 时区
    .auto = 自动确定时区
    .auto-info = 需要定位服务和互联网访问

time-format = 日期和时间格式
    .twenty-four = 24 小时制
    .first = 一周的第一天
    .show-date = 在顶部面板上显示日期
    .friday = 星期五
    .saturday = 星期六
    .sunday = 星期日
    .monday = 星期一

time-region = 区域和语言
    .desc = 根据您的区域确定日期、时间和数字格式

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

## Input

acceleration-desc = 根据速度自动调整跟踪灵敏度。

disable-while-typing = 键盘输入时禁用

input-devices = 输入设备
    .desc = 输入设备

primary-button = 主要按键
    .left = 左键
    .right = 右键

scrolling = 滚动
    .two-finger = 用两根手指滚动
    .edge = 用一根手指沿边缘滚动
    .speed = 滚动速度
    .natural = 自然滚动
    .natural-desc = 滚动内容，而不是视图

## Input: Keyboard

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

added = 已添加
type-to-search = 输入以搜索...

## Input: Keyboard: Shortcuts

keyboard-shortcuts = 键盘快捷键
    .desc = 查看和自定义快捷键

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

switch-between-windows = 在窗口之间切换
switch-to-next-workspace = 切换到下一个工作区
switch-to-prev-workspace = 切换到上一个工作区
open-application-library = 打开应用程序库
open-workspaces-view = 打开工作区概览
