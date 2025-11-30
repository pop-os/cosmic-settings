app = COSMIC подешавања
dbus-connection-error = Неуспешно повезивање са DBus-ом
ok = У реду
unknown = Непознато
number = { $number }

## Network & Wireless

connections-and-profiles =
    { $variant ->
        [wired] Жичане
        [wifi] Wi-Fi
        [vpn] VPN
       *[other] Непознате
    } конекције и профили конекција.
add-network = Додај мрежу
    .profile = Додај профил
add-vpn = Додај VPN
airplane-on = Авионски режим је укључен.
cable-unplugged = Кабл је ископчан
connect = Повежи се
connected = Повезано
connecting = Повезује се…
disconnect = Прекини конекцију
forget = Заборави
known-networks = Познате мреже
network-and-wireless = Мрежа и бежична конекција
no-networks = Нису пронађене мреже.
no-vpn = Нема доступних VPN конекција.
password = Шифра
password-confirm = Потврди лозинку
remove = Уклони
settings = Подешавања
username = Корисничко име
visible-networks = Видљиве мреже
identity = Идентитет
auth-dialog = Потребна аутентификација
    .vpn-description = Унесите корисничко име и лозинку потребне за VPN сервис.
    .wifi-description = Унесите лозинку или кључ за шифровање. Можете се повезати и притиском на "WPS" дугме на рутеру.
forget-dialog = Заборави ову Wi-Fi мрежу?
    .description = Мораћете поново да унесете лозинку да бисте користили ову Wi-Fi мрежу у будућности.
network-device-state =
    .activated = Повезано
    .config = Повезује се
    .deactivating = Прекида се конекција
    .disconnected = Није повезано
    .failed = Неуспешно повезивање
    .ip-check = Проверава се конекција
    .ip-config = Траже се IP и информације рутирања
    .need-auth = Потребна аутентификација
    .prepare = Припрема се за повезивање
    .secondaries = Чека се секундарна конекција
    .unavailable = Недоступно
    .unknown = Непознато стање
    .unmanaged = Неуправљано
    .unplugged = Кабл је ископчан
remove-connection-dialog = Уклони профил конекције?
    .vpn-description = Мораћете поново да унесете лозинку да бисте користили ову мрежу у будућности.
    .wired-description = Мораћете поново да креирате овај профил да бисте га користили у будућности.
vpn = VPN
    .connections = VPN конекције
    .error = Неуспешно додавање VPN конфигурације
    .remove = Уклони профил конекције
    .select-file = Изаберите VPN конфигурациону датотеку
vpn-error = VPN грешка
    .config = Неуспешно додавање VPN конфигурације
    .connect = Неуспешно повезивање са VPN-ом
    .connection-editor = Уређивач конекције неуспешан
    .connection-settings = Неуспешно добијање подешавања за активне конекције
    .updating-state = Неуспешно ажурирање стања мрежног менаџера
    .wireguard-config-path = Неисправна путања датотеке за WireGuard конфигурацију
    .wireguard-config-path-desc = Изабрана датотека мора бити на локалном систему датотека.
    .wireguard-device = Неуспешно креирање WireGuard уређаја
    .with-password =
        Неуспешно подешавање VPN { $field ->
           *[username] корисничко име
            [password] лозинка
            [password-flags] flag-ова лозинке
        } са nmcli
wired = Жичана
    .adapter = Жичани адаптер { $id }
    .connections = Жичане конекције
    .devices = Жичани уређаји
    .remove = Уклони профил конекције
    .desc = Жичана конекција, профили конекције
wifi = Wi-Fi
    .adapter = Wi-Fi адаптер { $id }
    .forget = Заборави ову мрежу
wireguard-dialog = Додај WireGuard уређај
    .description = Изаберите име уређаја за WireGuard конфигурацију.

## Networking: Online Accounts

online-accounts = Онлајн налози
    .desc = Додај налоге, IMAP и SMTP, пријављивање за предузећа

# Bluetooth

activate = Активирај
confirm = Потврди
enable = Омогући
bluetooth = Bluetooth
    .desc = Управљај Bluetooth уређајима
    .status = Овај систем је видљив као { $aliases } док су Bluetooth подешавања отворена.
    .connected = Повезано
    .connecting = Повезује се
    .disconnecting = Прекида се конекција
    .connect = Повежи се
    .disconnect = Прекини конекцију
    .forget = Заборави
    .dbus-error = Дошло је до грешке при интеракцији са DBus-ом: { $why }
    .disabled = Bluetooth сервис је онемогућен
    .inactive = Bluetooth сервис није активан
    .unknown = Bluetooth сервис се не може активирати. Да ли је BlueZ инсталиран?
bluetooth-paired = Претходно повезани уређаји
    .connect = Повежи се
    .battery = { $percentage }% батерије
bluetooth-confirm-pin = Потврди Bluetooth PIN
    .description = Молимо потврдите да се следећи PIN слаже са оним приказаним на { $device }
bluetooth-available = Оближњи уређаји
bluetooth-adapters = Bluetooth адаптери

## Accessibility

accessibility = Приступачност
    .vision = Вид
    .on = Укључено
    .off = Искључено
    .unavailable = Недоступно
    .screen-reader = Читач екрана
    .high-contrast = Висок контраст
    .invert-colors = Обрни боје
    .color-filters = Филтери боја
hearing = Слух
    .mono = Репродукуј стерео звук као моно
default = Подразумевано
magnifier = Лупа
    .controls =
        Или користи ове пречице: { $zoom_in ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_in } за увећање,
        }{ $zoom_out ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_out } за умањење,
        }
        Super + скрол мишем
    .scroll_controls = Омогући увећавање мишем или додирном таблом са Super + Скрол
    .show_overlay = Прикажи интерфејс лупе
    .increment = Корак увећања
    .signin = Покрени лупу при пријављивању
    .applet = Укључи/искључи лупу у аплету на панелу
    .movement = Увећани приказ се помера
    .continuous = Континуирано са показивачем
    .onedge = Када показивач достигне ивицу
    .centered = Да задржи показивач центриран
color-filter = Тип филтра боја
    .unknown = Непознат филтер је активан
    .greyscale = Нијансе сиве
    .deuteranopia = Зелено/Црвено (слабост зелене, Деутеранопија)
    .protanopia = Црвено/Зелено (слабост црвене, Протанопија)
    .tritanopia = Плаво/Жуто (слабост плаве, Тританопија)

## Desktop

desktop = Радна површина

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
dialog-add = Додај
fill = Попуни
fit-to-screen = Уклопи у екран
open-new-folder = Отвори нову фасциклу
recent-folders = Недавне фасцикле
x-minutes =
    { $number } { $number ->
        [one] минут
       *[other] минута
    }
x-hours =
    { $number } { $number ->
        [one] сат
       *[other] сати
    }
never = Никад

## Desktop: Appearance

appearance = Изглед
    .desc = Боје детаља и промена палете.
accent-color = Боја детаља
app-background = Позадина апликација или прозора
auto = Аутоматски
close = Затвори
color-picker = Бирач боја
copied-to-clipboard = Копирано у привремену меморију
copy-to-clipboard = Копирај у привремену меморију
dark = Тамна
export = Извези
hex = Hex
import = Увези
light = Светла
mode-and-colors = Режим и боје
recent-colors = Недавне боје
reset-to-default = Врати на подразумевано
rgb = RGB
window-hint-accent = Боја наговештаја активног прозора
window-hint-accent-toggle = Користи боју детаља из теме као наговештај активног прозора
auto-switch = Аутоматски прелаз између светлог и тамног режима
    .sunrise = Прелази на светли режим при изласку сунца
    .sunset = Прелази на тамни режим при заласку сунца
    .next-sunrise = Прелази на светли режим при следећем изласку сунца
    .next-sunset = Прелази на тамни режим при следећем заласку сунца
container-background = Позадина контејнера
    .desc-detail = Боја позадине контејнера се користи за бочну траку за навигацију, бочни мени, дијалошке оквире и друге сличне виџете. Подразумевано, аутоматски се изводи из позадине апликација или прозора.
    .reset = Врати на аутоматско
    .desc = Боја позадине контејнера се користи за бочну траку за навигацију, бочни мени, дијалошке оквире и друге сличне виџете.
control-tint = Нијанса контролних компоненти
    .desc = Користи се за позадину стандардних дугмади, уноса за претрагу, уноса текста и сличних компоненти.
frosted = Ефекат мат стакла на интерфејсу система
    .desc = Примењује замућење позадине на панел, док, аплете, покретач и библиотеку апликација.
enable-export = Примени ову тему на GNOME апликације.
    .desc = Аутоматску промену теме не подржавају све апликације. Не-COSMIC апликације ће можда морати да се поново покрену након промене теме.
icon-theme = Тема иконица
    .desc = Примењује другачији скуп иконица на апликације.
text-tint = Нијанса текста интерфејса
    .desc = Боја која се користи за добијање боја текста интерфејса које имају довољан контраст на различитим површинама.
style = Стил
    .round = Округли
    .slightly-round = Благо округли
    .square = Четвртаст
interface-density = Густина интерфејса
    .comfortable = Удобно
    .compact = Компактно
    .spacious = Просторно
window-management-appearance = Управљање прозорима
    .active-hint = Дебљина наговештаја активног прозора
    .gaps = Празнине око сложених прозора

### Experimental

experimental-settings = Експериментална подешавања
icons-and-toolkit = Тема иконица и toolkit-а
interface-font = Фонт система
monospace-font = Монопросторни фонт

## Desktop: Notifications

notifications = Обавештења
    .desc = Не узнемиравај, обавештења на закључаном екрану и подешавања апликација.

## Desktop: Panel

panel = Панел
    .desc = Главна системска трака за меније и аплете.
add = Додај
add-applet = Додај аплет
all = Све
applets = Аплети
center-segment = Централни сегмент
place-here = Ставите аплете овде
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

## Desktop: Dock

dock = Док
    .desc = Опциона трака за апликације и аплете.

## Desktop: Window management

window-management = Управљање прозорима
    .desc = Акција Super тастера, опције контроле прозора, и додатне опције слагања прозора.
super-key = Super тастер
    .launcher = Отвори Покретач
    .workspaces = Отвори Радне просторе
    .applications = Отвори Апликације
    .disable = Онемогући
edge-gravity = Плутајући прозори се привлаче ка ивицама екрана
window-controls = Контроле прозора
    .maximize = Прикажи дугме за максимизовање
    .minimize = Прикажи дугме за минимизовање
    .active-window-hint = Прикажи наговештај активног прозора
focus-navigation = Навигација фокуса
    .focus-follows-cursor = Фокус прати показивач
    .focus-follows-cursor-delay = Кашњење фокуса за показивачем у ms
    .cursor-follows-focus = Показивач прати фокус

## Desktop: Workspaces

workspaces = Радни простори
    .desc = Оријентација и понашање радног простора.
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
hot-corner = Лепљиви угао
    .top-left-corner = Омогући горњи леви лепљиви угао за приказ радних простора

## Displays

-requires-restart = Захтева поновно покретање
color = Боја
    .depth = Дубина боје
    .profile = Профил боје
    .sidebar = Профили боје
    .temperature = Температура боје
display = Екрани
    .desc = Управљајте екранима и ноћним светлом
    .arrangement = Распоред екрана
    .arrangement-desc = Превуците екране да бисте их преуредили.
    .enable = Омогући екран
    .external = { $size } { $output } спољашњи екран
    .laptop = { $size } екран лаптопа
    .options = Опције екрана
    .refresh-rate = Освежавање
    .resolution = Резолуција
    .scale = Размера
    .additional-scale-options = Додатне опције размере
mirroring = Пресликавање
    .id = Пресликавање { $id }
    .dont = Не пресликавај
    .mirror = Пресликај { $display }
    .project =
        Пројектуј на { $display ->
            [all] све екране
           *[other] { $display }
        }
    .project-count =
        Пројектовање на још { $count } { $count ->
            [1] екран
           *[other] екрана
        }
night-light = Ноћно светло
    .auto = Аутоматско (од заласка до изласка сунца)
    .desc = Смањите плаво светло топлијим бојама.
orientation = Оријентација
    .standard = Стандардна
    .rotate-90 = Ротирано 90°
    .rotate-180 = Ротирано 180°
    .rotate-270 = Ротирано 270°
vrr = Варијабилна брзина освежавања
    .enabled = Омогућено
    .force = Увек
    .auto = Аутоматски
    .disabled = Онемогућено
scheduling = Распоред
    .manual = Ручни распоред
dialog = Дијалог
    .title = Задржи ова подешавања екрана?
    .keep-changes = Задржи промене
    .change-prompt = Промене подешавања ће се аутоматски вратити за { $time } секунди.
    .revert-settings = Врати подешавања

## Sound

sound = Звук
    .desc = N/A
sound-output = Излаз
    .volume = Јачина излазног звука
    .device = Излазни уређај
    .level = Ниво излаза
    .config = Конфигурација
    .balance = Баланс
    .left = Лево
    .right = Десно
sound-input = Улаз
    .volume = Јачина улазног звука
    .device = Улазни уређај
    .level = Ниво улаза
amplification = Појачавање
    .desc = Омогућава повећање јачине звука до 150%.
sound-alerts = Упозорења
    .volume = Јачина звука упозорења
    .sound = Звук упозорења
sound-applications = Апликације
    .desc = Јачина звука апликација и подешавања

## Power

power = Напајање и батерија
    .desc = Управљајте поставкама напајања.
battery = Батерија
    .minute =
        { $value } { $value ->
            [one] минут
           *[other] минута
        }
    .hour =
        { $value } { $value ->
            [one] сат
           *[other] сати
        }
    .day =
        { $value } { $value ->
            [one] дан
           *[other] дана
        }
    .less-than-minute = Мање од минута
    .and = и
    .remaining-time =
        { $time } до { $action ->
            [full] пуне
           *[other] празне
        }
connected-devices = Повезани уређаји
    .unknown = Непознат уређај
power-mode = Режим напајања
    .battery = Продужено трајање батерије
    .battery-desc = Смањена потрошња енергије и тихе перформансе.
    .balanced = Балансирано
    .balanced-desc = Тихе перформансе и умерена потрошња енергије.
    .performance = Високе перформансе
    .performance-desc = Највеће перформансе и потрошња енергије.
    .no-backend = Подсистем није пронађен. Инсталирајте system76-power или power-profiles-daemon.
power-saving = Опције штедње енергије
    .turn-off-screen-after = Угаси екран после
    .auto-suspend = Аутоматско спавање
    .auto-suspend-ac = Аутоматско спавање приључено за струју
    .auto-suspend-battery = Аутоматско спавање на батерији

## Input

acceleration-desc = Аутоматски подешава осетљивост праћења на основу брзине.
disable-while-typing = Онемогући током куцања
input-devices = Улазни уређаји
    .desc = Улазни уређаји
primary-button = Примарно дугме
    .desc = Одређује редослед физичких дугмади.
    .left = Лево
    .right = Десно
scrolling = Померање
    .two-finger = Померање са два прста
    .edge = Померање уз ивицу са једним прстом
    .speed = Брзина померања
    .natural = Природно померање
    .natural-desc = Окретање точка помера садржај уместо приказа

## Input: Keyboard

slow = Споро
fast = Брзо
short = Кратко
long = Дуго
keyboard = Тастатура
    .desc = Извор уноса, пребацивање, унос специјалних карактера, пречице.
keyboard-sources = Језик уноса
    .desc = Језик уноса се може мењати помоћу комбинације тастера Super+Space. Ово се може променити у подешавањима пречица на тастатури.
    .move-up = Помери горе
    .move-down = Помери доле
    .settings = Подешавања
    .view-layout = Погледај распоред тастатуре
    .remove = Уклони
    .add = Додај језик уноса
keyboard-special-char = Уношење специјалних знакова
    .alternate = Тастер за алтернативне знакове
    .compose = Compose тастер
    .compose-desc = Compose тастер омогућава унос широког спектра знакова. Да бисте га користили, притисните Compose, а затим секвенцу знакова. На пример, Compose тастер праћен тастерима C и o уноси ©, док праћен тастерима а и ‘ уноси á.
    .caps = Caps Lock тастер
keyboard-typing-assist = Куцање
    .repeat-rate = Стопа понављања
    .repeat-delay = Кашњење понављања
keyboard-numlock-boot = Numlock
    .boot-state = Стање при покретању система
    .last-boot = Претходно покретање
    .on = Укључено
    .off = Искључено
    .set = Подеси стање numlock-а при покретању
added = Додато
type-to-search = Куцајте за претрагу...
show-extended-input-sources = Прикажи проширене изворе уноса

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Пречице на тастатури
    .desc = Прегледајте и прилагодите пречице
add-another-keybinding = Додај другу пречицу
cancel = Прекини
command = Команда
custom = Прилагођене
debug = Дебагуј
disabled = Онемогућено
input-source-switch = Пребаци извор уноса језика тастатуре
migrate-workspace-prev = Премести радни простор на претходни екран
migrate-workspace-next = Премести радни простор на следећи екран
migrate-workspace =
    Премести радни простор на екран { $direction ->
       *[down] доле
        [left] лево
        [right] десно
        [up] горе
    }
navigate = Навигација
replace = Замени
shortcut-name = Име пречице
system-controls = Контроле система
terminate = Прекини
toggle-stacking = Укључи груписање прозора
type-key-combination = Укуцајте комбинацију тастера
custom-shortcuts = Прилагођене пречице
    .add = Додај пречицу
    .context = Додај прилагођену пречицу
    .none = Нема прилагођених пречица
modified = { $count } измењено
nav-shortcuts = Навигација
    .prev-output = Фокусирај претходни екран
    .next-output = Фокусирај следећи екран
    .last-workspace = Фокусирај прошли радни простор
    .prev-workspace = Фокусирај претходни радни простор
    .next-workspace = Фокусирај следећи радни простор
    .focus =
        Фокусирај прозор { $direction ->
           *[down] доле
            [in] унутра
            [left] лево
            [out] споља
            [right] десно
            [up] горе
        }
    .output =
        Пребаци се на екран { $direction ->
           *[down] доле
            [left] лево
            [right] десно
            [up] горе
        }
    .workspace = Пребаци се на радни простор { $num }
manage-windows = Управљање прозорима
    .close = Затвори прозор
    .maximize = Максимизуј прозор
    .fullscreen = Цео екран
    .minimize = Минимизуј прозор
    .resize-inwards = Смањи прозор
    .resize-outwards = Повећај прозор
    .toggle-sticky = Укључи лепљиви прозор
move-windows = Померање прозора
    .direction =
        Помери прозор { $direction ->
           *[down] доле
            [left] лево
            [right] десно
            [up] горе
        }
    .display =
        Помери прозор за један монитор { $direction ->
           *[down] доле
            [left] лево
            [right] десно
            [up] горе
        }
    .workspace =
        Помери прозор за један радни простор { $direction ->
           *[below] испод
            [left] лево
            [right] десно
            [above] изнад
        }
    .workspace-num = Помери прозор на радни простор { $num }
    .prev-workspace = Помери прозор на претходни радни простор
    .next-workspace = Помери прозор на следећи радни простор
    .last-workspace = Помери прозор на прошли радни простор
    .next-display = Помери прозор на следећи екран
    .prev-display = Помери прозор на претходни екран
    .send-to-prev-workspace = Пошаљи прозор на претходни радни простор
    .send-to-next-workspace = Пошаљи прозор на следећи радни простор
system-shortcut = Систем
    .app-library = Отвори библиотеку апликација
    .brightness-down = Смањи осветљеност екрана
    .brightness-up = Повећај осветљеност екрана
    .home-folder = Отвори почетну фасциклу
    .keyboard-brightness-down = Смањи осветљеност тастатуре
    .keyboard-brightness-up = Повећај осветљеност тастатуре
    .launcher = Отвори покретач
    .log-out = Одјави се
    .lock-screen = Закључај екран
    .mute = Искључи аудио излаз
    .mute-mic = Искључи улаз микрофона
    .play-pause = Пусти/Паузирај
    .play-next = Следећа нумера
    .play-prev = Претходна нумера
    .poweroff = Искључи систем
    .screenshot = Направи снимак екрана
    .terminal = Отвори терминал
    .volume-lower = Смањи јачину аудио излаза
    .volume-raise = Повећај јачину аудио излаза
    .web-browser = Отвори веб претраживач
    .window-switcher = Пребацивање између отворених прозора
    .window-switcher-previous = Пребацивање између отворених прозора унатраг
    .workspace-overview = Отвори преглед радних простора
window-tiling = Слагање прозора
    .horizontal = Подеси хоризонталну оријентацију
    .vertical = Подеси вертикалну оријентацију
    .swap-window = Замени прозор
    .toggle-tiling = Укључи слагање прозора
    .toggle-stacking = Укључи груписање прозора
    .toggle-floating = Укључи плутајући прозор
    .toggle-orientation = Промени оријентацију
replace-shortcut-dialog = Замени пречицу?
    .desc = { $shortcut } се користи од стране { $name }. Ако је замените, { $name } ће бити онемогућено.
zoom-in = Увећај
zoom-out = Умањи

## Input: Mouse

mouse = Миш
    .desc = Брзина миша, убрзање, природно померање.
    .speed = Брзина миша
    .acceleration = Омогући убрзање миша

## Input: Touchpad

click-behavior = Понашање клика
    .click-finger = Секундарни клик са два прста и средњи клик са три прста
    .button-areas = Секундарни клик у доњем десном углу и средњи клик у доњем центру
pinch-to-zoom = Стисните прсте за зумирање
    .desc = Користите два прста за зумирање садржаја, за апликације које подржавају зумирање.
tap-to-click = Додир за клик
    .desc = Омогућава додир једним прстом за примарни клик, два прста за секундарни клик и три прста за средњи клик.
touchpad = Додирна табла
    .acceleration = Омогући убрзање додирне табле
    .desc = Брзина додирне табле, опције клика, покрети.
    .speed = Брзина додирне табле

## Input: Gestures

gestures = Покрети
    .four-finger-down = Превуци према доле са четири прста
    .four-finger-left = Превуци према лево са четири прста
    .four-finger-right = Превуци према десно са четири прста
    .four-finger-up = Превуци према горе са четири прста
    .three-finger-any = Превуци са три прста у било ком смеру
switch-workspaces = Промени радни простор
    .horizontal = Превуци према лево/десно са четири прста
    .vertical = Превуци према горе/доле са четири прста
switch-between-windows = Пребацивање између прозора
open-application-library = Отвори библиотеку апликација
open-workspaces-view = Отвори преглед радних простора

## Time & Language

time = Време и језик
    .desc = N/A
time-date = Датум и време
    .desc = Временска зона, аутоматска подешавања сата и форматирање времена.
    .auto = Подеси аутоматски
    .auto-ntp = Датум и време ће се аутоматски ажурирати када се подеси временска зона.
time-zone = Временска зона
    .auto = Аутоматска временска зона
    .auto-info = Захтева услуге локације и приступ интернету
time-format = Формат датума и времена
    .twenty-four = 24-часовно време
    .show-seconds = Прикажи секунде
    .first = Први дан недеље
    .show-date = Прикажи датум у аплету за време
    .friday = Петак
    .saturday = Субота
    .sunday = Недеља
    .monday = Понедељак
time-region = Регион и језик
    .desc = Формат датума, времена и бројева на основу региона.
formatting = Форматирање
    .dates = Датуми
    .time = Време
    .date-and-time = Датум и време
    .numbers = Бројеви
    .measurement = Мерења
    .paper = Папир
preferred-languages = Преферирани језик
    .desc = Редослед језика одређује који се језик користи за кориснички интерфејс. Промене се примењују при следећем пријављивању.
add-language = Додај језик
    .context = Додај језик
install-additional-languages = Инсталирај додатне језике
region = Регион

## Applications

applications = Апликације

## Applications: Default Applications

default-apps = Подразумеване апликације
    .desc = Подразумевани веб претраживач, имејл клијент, управљач датотека и друге апликације.
    .web-browser = Веб претраживач
    .file-manager = Управљач датотека
    .mail-client = Имејл клијент
    .music = Музика
    .video = Видео
    .photos = Фотографије
    .calendar = Календар
    .terminal = Терминал
    .other-associations = Остале асоцијације
    .text-editor = Уређивач текста

## Applications: Startup Applications

startup-apps = Апликације при покретању
    .desc = Конфигуришите апликације које се покрећу при пријављивању.
    .add = Додај апликацију
    .user = Апликације које се покрећу када се пријавите
    .none = Нису додате апликације за покретање
    .remove-dialog-title = Уклони { $name }?
    .remove-dialog-description = Да ли сте сигурни да желите да уклоните ову апликацију за покретање?
    .add-startup-app = Додај апликацију за покретање

## Applications: Legacy Applications

legacy-applications = Компатибилност X11 апликација
    .desc = Скалирање апликација X11 система прозора и глобалне пречице.
legacy-app-global-shortcuts = Глобалне пречице у X11 апликацијама
    .desc = Глобалне пречице омогућавају да притисци тастера и дугмади миша изведених у апликацијама буду препознати од стране других апликација за функције као што су push-to-talk или push-to-mute. Подразумевано, ово је онемогућено у X11 апликацијама да би се осигурало да друге апликације не могу да прате догађаје тастатуре и миша који садрже осетљиве информације.
    .none = Ниједан тастер
    .modifiers = Модификатори (Super, Shift, Control, Alt)
    .combination = Сви тастери док се држе модификатори Super, Control или Alt
    .all = Сви тастери
    .mouse = Догађаји дугмади миша у X11 апликацијама
legacy-app-scaling = Скалирање апликација X11 система прозора
    .scaled-gaming = Оптимизуј за игре и апликације преко целог екрана
    .gaming-description = X11 апликације могу изгледати нешто веће/мање у поређењу са Wayland апликацијама.
    .scaled-applications = Оптимизуј за апликације
    .applications-description = Игре и X11 апликације преко целог екрана можда неће одговарати вашој резолуцији екрана.
    .scaled-compatibility = Режим максималне компатибилности
    .compatibility-description = X11 апликације могу изгледати замућено на HiDPI екранима.
    .preferred-display = Префериран екран за игре и X11 апликације преко целог екрана
    .no-display = Ниједан

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
about-related = Повезана подешавања
    .support = Подршка

## System: Firmware

firmware = Фирмвер
    .desc = Детаљи фирмвера.

## System: Users

users = Корисници
    .desc = Аутентификација и кориснички налози.
    .admin = Админ
    .standard = Стандардан
    .profile-add = Изабери слику профила
administrator = Администратор
    .desc = Администратори могу да мењају подешавања за све кориснике, додају и уклањају друге кориснике.
add-user = Додај корисника
change-password = Промени лозинку
remove-user = Уклони корисника
full-name = Пуно име
invalid-username = Неисправно корисничко име.
password-mismatch = Лозинка и потврда морају се поклапати.
save = Сачувај
