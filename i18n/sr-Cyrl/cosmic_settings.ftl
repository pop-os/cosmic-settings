app = COSMIC подешавања

unknown = Непознато

number = { $number }

## Desktop

desktop = Радна површина

## Desktop: Appearance

appearance = Изглед
    .desc = Боје детаља и промена COSMIC палете.
	
accent-color = Боја детаља
app-background = Позадина апликација или прозора
auto = Аутоматски
close = Затвори
color-picker = Бирач боја
copied-to-clipboard = Копирано у привремену меморију
copy-to-clipboard = Копирај у привремену меморију
dark = Тамно
export = Извези
hex = Hex
import = Увези
light = Светло
mode-and-colors = Режим и боје
recent-colors = Недавне боје
reset-default = Врати на подразумевано
reset-to-default = Врати на подразумевано
rgb = RGB
window-hint-accent = Боја наговештаја активног прозора
window-hint-accent-toggle = Користи боју детаља из теме као наговештај активног прозора

auto-switch = Аутоматско мењање режима
    .desc = Светли режим се укључује при изласку, а тамни при заласку сунца

container-background = Позадина контејнера
    .desc-detail = Боја позадине контејнера се користи за бочну траку за навигацију, бочни мени, дијалошке оквире и друге сличне виџете. Подразумевано, аутоматски се изводи из позадине апликација или прозора.
    .reset = Врати на аутоматско
    .desc = Боја позадине контејнера се користи за бочну траку за навигацију, бочни мени, дијалошке оквире и друге сличне виџете.
	
control-tint = Нијанса контролних компоненти
    .desc = Користи се за позадину стандардних дугмади, уноса за претрагу, уноса текста и сличних компоненти.

frosted = Ефекат мат стакла на интерфејсу система
    .desc = Примењује замућење позадине на панел, док, аплете, покретач и библиотеку апликација.

text-tint = Нијанса текста интерфејса
    .desc = Боја која се користи за добијање боја текста интерфејса које имају довољан контраст на различитим површинама.

style = Стил
    .round = Округли
    .slightly-round = Благо округли
    .square = Четвртаст

# interface density left out for now
window-management = Управљање прозорима
    .active-hint = Величина наговештаја активног прозора
    .gaps = Празнине око сложених прозора

## Desktop: Display

-requires-restart = Захтева поновно покретање

color = Боја
    .depth = Дубина боје
    .profile = Профил боје
    .sidebar = Профили боје
    .temperature = Температура боје

display = Екран
    .desc = Управљајте екранима, пребацивањем графике и ноћним светлом
    .arrangement = Распоред екрана
    .arrangement-desc = Превуците екране да бисте их преуредили.
    .enable = Омогући екран
    .external = { $size } { $output } спољашњи екран
    .laptop = { $size } екран лаптопа
    .options = Опције екрана
    .refresh-rate = Освежавање
    .resolution = Резолуција
    .scale = Размера

graphics-mode = Графички режим
    .mode = { $mode ->
        [compute] Рачунска
        *[hybrid] Хибридна
        [integrated] Интегрисана
        [nvidia] NVIDIA
    } графика
    .enable = Омогући { $mode ->
        [compute] рачунску
        *[hybrid] хибридну
        [integrated] интегрисану
        [nvidia] NVIDIA
    } графику
    .desc = { $mode ->
        [compute] Користи наменску графику само за рачунска оптерећења. Исључује спољне екране. { -requires-restart }.
        *[hybrid] Апликације користе интегрисану графику осим ако се изричито не захтева коришћење наменске графике. { -requires-restart }.
        [integrated] Искључује наменску графику ради дужег трајања батерије и мање буке вентилатора.
        [nvidia] Боље графичко искуство и највећа потрошња енергије. { -requires-restart }.
    }
    .restart = Поново покрени и пребаци на { $mode }?
    .restart-desc = Пребацивање на { $mode } ће затворити све отворене апликације

mirroring = Пресликавање
    .id = Пресликавање { $id }
    .dont = Не пресликавај
    .mirror = Пресликај { $display }
    .project = Пројектуј на { $display ->
        [all] све екране
        *[other] { $display }
    }
    .project-count = Пројектовање на још { $count} { $count ->
        [1] екран
        *[other] екрана
    }

night-light = Ноћно светло
    .auto = Аутоматско (од заласка до изласка сунца)
    .desc = Смањите плаво светло топлијим бојама.

orientation = Оријентација
    .landscape = Положено
	.landscape-flipped = Положено (преокренуто)
    .portrait = Усправно
	.portrait-flipped = Усправно (преокренуто)

scheduling = Распоред
    .manual = Ручни распоред

## Desktop: Notifications

notifications = Обавештења
    .desc = Не узнемиравај, обавештења на закључаном екрану и подешавања апликација.

## Desktop: Options

desktop-panel-options = Радна површина и панел
    .desc = Улога Super тастера, лепљиви углови, контрола прозора.
	
desktop-panels-and-applets = Панели радне површине и аплети

dock = Док
    .desc = Трака са закаченим апликацијама.
	
hot-corner = Лепљиви углови
    .top-left-corner = Укључити горњи леви лепљиви угао за радне просторе

super-key-action = Улога Super тастера
    .launcher = Покретач апликација
    .workspaces = Радни простори
    .applications = Апликације

top-panel = Горњи панел
    .workspaces = Дугме за приказивање радних простора
    .applications = Дугме за приказивање апликација

window-controls = Контрола прозора
    .minimize = Прикажи дугме за минимизовање
    .maximize = Прикажи дугме за максимизовање

## Desktop: Panel

panel = Горњи панел
    .desc = Горња трака са контролама радне површине и менијима.
	
add = Додај
add-applet = Додај аплет
all = Све
applets = Аплети
center-segment = Централни сегмент
drop-here = Спустите аплете овде
end-segment = Крајњи сегмент
large = Велико
no-applets-found = Нису пронађени аплети...
panel-bottom = Дно
panel-left = Лево
panel-right = Десно
panel-top = Врх
search-applets = Претражи аплете...
small = Мало
start-segment = Почетни сегмент

panel-appearance = Изглед
    .match = Као систем
    .light = Светли
    .dark = Тамни

panel-behavior-and-position = Понашање и позиција
    .autohide = Аутоматско сакривање панела
    .dock-autohide = Аутоматско сакривање док-а
    .position = Позиција на екрану
    .display = Прикажи на екрану

panel-style = Стил
    .anchor-gap = Размак између панела и ивица екрана
    .dock-anchor-gap = Размак између док-а и ивица екрана
    .extend = Прошири панел до ивица екрана
    .dock-extend = Прошири док до ивица екрана
    .appearance = Изглед
    .size = Величина
    .background-opacity = Прозирност позадине

panel-applets = Конфигурација
    .dock-desc = Подеси аплете на док-у.
    .desc = Подеси аплете на панелу.

panel-missing = Недостаје конфигурација панела
    .desc = Конфигурациона датотека панела недостаје због коришћења прилагођене конфигурације или је оштећена.
    .fix = Врати на подразумевано

## Desktop: Wallpaper

wallpaper = Позадина
    .change = Промени слику сваких
    .desc = Позадине радне површине, боје, и слајд-шоу.
	.fit = Скалирање позадине
	.folder-dialog = Изаберите фасциклу за позадине
    .image-dialog = Изаберите слику позадине
    .plural = Позадине
    .same = Иста позадина на свим екранима
    .slide = Слајд-шоу

add-color = Додај боју
add-image = Додај слику
all-displays = Сви екрани
colors = Боје
dialog-add = _Add
fit-to-screen = Уклопи у екран
open-new-folder = Отвори нову фасциклу
recent-folders = Недавне фасцикле
stretch = Развуци
zoom = Увећај

x-minutes = { $number } мин.
x-hours = { $number ->
    [1] 60 минута
    *[other] { $number } сат.
}

## Desktop: Workspaces

workspaces = Радни простори
    .desc = Постави број радних простора, понашање, и позицију.

workspaces-behavior = Понашање радних простора
    .dynamic = Динамични радни простори
	.dynamic-desc = Аутоматски уклања празне радне просторе.
    .fixed = Фиксни број радних простора
	.fixed-desc = Додајте или уклоните радне просторе у прегледу.

workspaces-multi-behavior = Понашање са више монитора
    .span = Радни простори су заједнички за све екране
    .separate = Екрани имају одвојене радне просторе
	
workspaces-overview-thumbnails = Приказ радних простора у прегледу
    .show-number = Прикажи број радног простора
    .show-name = Прикажи име радног простора

workspaces-orientation = Оријентација радних простора
    .vertical = Вертикални
    .horizontal = Хоризонтални

## Networking: Wired

wired = Жичана
    .desc = Жичана конекција, профили конекције

## Networking: Online Accounts

online-accounts = Онлајн налози
    .desc = Додај налоге, IMAP и SMTP, пријављивање за предузећа

## Time & Language

time = Време и језик
    .desc = N/A

time-date = Датум и време
    .desc = Временска зона, аутоматско подешавање сата, формат сата.
    .auto = Подеси аутоматски

time-zone = Временска зона
    .auto = Аутоматска временска зона
    .auto-info = Захтева приступ локацији и интернету

time-format = Формат датума и времена
    .twenty-four = 24-часа
    .first = Први дан недеље

time-region = Регион и језик
    .desc = Формат датума, времена и бројева добијен на основу Вашег региона

## Sound

sound = Звук
    .desc = N/A

sound-output = Излаз
    .volume = Јачина излазног звука
    .device = Излазни уређај
    .level = Поравнање излазног звука
    .config = Конфигурација
    .balance = Баланс

sound-input = Улаз
    .volume = Јачина улазног звука
    .device = Улазни уређај
    .level = Поравнање улазног звука

sound-alerts = Упозорења
    .volume = Јачина звука упозорења
    .sound = Звук упозорења

sound-applications = Апликације
    .desc = Јачина звука апликација и подешавања

## System

system = Систем и налози

## System: About

about = О систему
    .desc = Име уређаја, информације о хардверу, подразумевана подешавања оперативног система.

about-device = Име уређаја
    .desc = Ово име је видљиво другим мрежним или Bluetooth уређајима.

about-hardware = Хардвер
    .model = Модел хардвера
    .memory = Меморија
    .processor = Процесор
    .graphics = Графика
    .disk-capacity = Капацитет диска

about-os = Оперативни систем
    .os = Оперативни систем
    .os-architecture = Архитектура оперативног система
    .desktop-environment = Окружење радне површине
    .windowing-system = Систем прозора

about-related = Додатна подешавања
    .support = Подршка

## System: Firmware

firmware = Фирмвер
    .desc = Детаљи фирмвера.

## System: Users

users = Корисници
    .desc = Аутентификација и пријављивање, закључан екран.

## Input

input = Унос
    .desc = Унос

## Input: Keyboard

keyboard = Тастатура
    .desc = Унос са тастатуре

keyboard-sources = Језик уноса
    .desc = Језик уноса се може мењати помоћу комбинације тастера Super+Space. Ово понашање се може променити у подешавањима пречица на тастатури.
    .move-up = Помери горе
    .move-down = Помери доле
    .settings = Подешавања
    .view-layout = Погледај распоред тастатуре
    .remove = Уклони

keyboard-special-char = Уношење специјалних знакова
    .alternate = Тастер за алтернативне знакове
    .compose = Compose тастер

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Пречице на тастатури
    .desc = Прегледајте и прилагодите пречице

## Input: Mouse
mouse = Миш
    .desc = Брзина миша, убрзање, природно померање.
    .primary-button = Примарно дугме
    .primary-button-left = Лево
    .primary-button-right = Десно
    .speed = Брзина миша
    .acceleration = Омогући убрзање миша
    .acceleration-desc = Аутоматски подешава осетљивост праћења на основу брзине.
    .double-click-speed = Брзина двоструког клика
    .double-click-speed-desc = Мења потребну брзину за регистровање двоструких кликова.

mouse-scrolling = Померање
    .speed = Брзина померања
    .natural = Природно померање
    .natural-desc = Окретање точка помера садржај уместо приказа.

## Input: Touchpad

touchpad = Додирна табла
    .desc = Брзина додирне табле, опције клика, покрети.
    .primary-button = Примарно дугме
    .primary-button-left = Лево
    .primary-button-right = Десно
    .speed = Брзина додирне табле
    .acceleration = Омогући убрзање додирне табле
    .acceleration-desc = Аутоматски подешава осетљивост праћења на основу брзине.
    .double-click-speed = Брзина двоструког клика
    .double-click-speed-desc = Мења потребну брзину за регистровање двоструких кликова.
