app = Налады COSMIC
dbus-connection-error = Не ўдалося падключыцца да DBus
ok = OK
unknown = Невядома
number = { $number }

## Network & Wireless

connections-and-profiles =
    { $variant ->
        [wired] Правадныя
        [wifi] Wi-Fi
        [vpn] VPN
       *[other] Невядомыя
    } злучэнні і профілі злучэнняў.
add-network = Дадаць сетку
    .profile = Дадаць профіль
add-vpn = Дадаць VPN
airplane-on = Уключаны рэжым палёту.
cable-unplugged = Кабель адключаны
connect = Падключыцца
connected = Падключана
connecting = Падключэнне…
disconnect = Адключыцца
forget = Забыць
known-networks = Вядомыя сеткі
network-and-wireless = Сетка і бесправадная сувязь
no-networks = Сетак не знойдзена.
no-vpn = Няма даступных VPN-злучэнняў.
password = Пароль
password-confirm = Пацвердзіць пароль
remove = Выдаліць
settings = Налады
username = Імя карыстальніка
visible-networks = Бачныя сеткі
identity = Ідэнтыфікатар
auth-dialog = Патрабуецца аўтэнтыфікацыя
    .vpn-description = Увядзіце імя карыстальніка і пароль, неабходныя для службы VPN.
    .wifi-description = Увядзіце пароль або ключ шыфравання. Вы таксама можаце падключыцца, націснуўшы кнопку «WPS» на маршрутызатары.
forget-dialog = Забыць гэтую сетку Wi-Fi?
    .description = Вам трэба будзе зноў увесці пароль, каб выкарыстоўваць гэтую сетку Wi-Fi у будучыні.
network-device-state =
    .activated = Падключана да сеткі
    .config = Падключэнне да сеткі
    .deactivating = Адключэнне ад сеткі
    .disconnected = Адключана
    .failed = Не ўдалося падключыцца
    .ip-check = Праверка падключэння
    .ip-config = Запыт IP-адраса і інфармацыі аб маршрутызацыі
    .need-auth = Патрабуецца аўтэнтыфікацыя
    .prepare = Падрыхтоўка да падключэння да сеткі
    .secondaries = Чаканне другаснага падключэння
    .unavailable = Недаступна
    .unknown = Невядомы стан
    .unmanaged = Некіруемы
    .unplugged = Кабель адключаны
remove-connection-dialog = Выдаліць профіль злучэння?
    .vpn-description = Вам трэба будзе ўвесці пароль яшчэ раз, каб выкарыстоўваць гэтую сетку ў будучыні.
    .wired-description = Вам трэба будзе аднавіць гэты профіль, каб выкарыстоўваць яго ў будучыні.
vpn = VPN
    .connections = VPN-падключэнні
    .error = Не ўдалося дадаць канфігурацыю VPN
    .remove = Выдаліць профіль падключэння
    .select-file = Выберыце файл канфігурацыі VPN
vpn-error = Памылка VPN
    .config = Не ўдалося дадаць канфігурацыю VPN
    .connect = Не ўдалося падключэння да VPN
    .connection-editor = Памылка рэдактара злучэнняў
    .connection-settings = Не ўдалося атрымаць налады для актыўных злучэнняў
    .updating-state = Не ўдалося абнавіць стан сеткавага менеджара
    .wireguard-config-path = Недапушчальны шлях да файла канфігурацыі WireGuard
    .wireguard-config-path-desc = Выбраны файл мусіць знаходзіцца ў лакальнай файлавай сістэме.
    .wireguard-device = Не ўдалося стварыць прыладу WireGuard
    .with-password =
        Не ўдалося задаць { $field ->
           *[username] імя карыстальніка
            [password] пароль
            [password-flags] пазнакі пароля
        } VPN праз nmcli
wired = Правадная
    .adapter = Правадны адаптар { $id }
    .connections = Правадныя злучэнні
    .devices = Правадныя прылады
    .remove = Выдаліць профіль злучэння
wifi = Wi-Fi
    .adapter = Wi-Fi адаптар { $id }
    .forget = Забыць гэтую сетку
wireguard-dialog = Дадаць WireGuard прыладу
    .description = Выберыце імя прылады для канфігурацыі WireGuard.

## Networking: Online Accounts

online-accounts = Анлайн акаўнты
    .desc = Дадаць акаўнты, IMAP і SMTP, карпаратыўныя лагіны

# Bluetooth

activate = Актываваць
confirm = Пацвердзіць
enable = Уключыць
bluetooth = Bluetooth
    .desc = Кіраваць прыладамі Bluetooth
    .status = Гэтая сістэма бачная як { $aliases } калі налады Bluetooth адкрытыя.
    .connected = Падключана
    .connecting = Падключэнне
    .disconnecting = Адключэнне
    .connect = Падключыцца
    .disconnect = Адключыцца
    .forget = Забыць
    .dbus-error = Адбылася памылка падчас узаемадзеяння з DBus: { $why }
    .disabled = Служба Bluetooth адключана
    .inactive = Служба Bluetooth неактыўная
    .unknown = Не ўдалося актываваць службу Bluetooth. Ці ўсталяваны BlueZ?
bluetooth-paired = Раней падключаныя прылады
    .connect = Падключыцца
    .battery = { $percentage }% батарэі
bluetooth-confirm-pin = Пацвердзіць PIN-код Bluetooth
    .description = Пацвердзіце, што наступны PIN-код супадае з тым, што адлюстроўваецца на прыладзе { $device }
bluetooth-available = Прылады паблізу
bluetooth-adapters = Bluetooth Адаптары

## Accessibility

accessibility = Спецыяльныя магчымасці
    .vision = Зрок
    .on = Уключана
    .off = Выключана
    .unavailable = Недаступна
    .screen-reader = Праграма чытання з экрана
    .high-contrast = Рэжым высокай кантраснасці
    .invert-colors = Інвертаваць колеры
    .color-filters = Колеравыя фільтры
hearing = Слых
    .mono = Прайграваць стэрэа-аўдыё як мона
default = Па змаўчанні
magnifier = Лупа
    .controls =
        Ці выкарыстоўвайце гэтыя спалучэнні: { $zoom_in ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_in } для павялічэння,
        }{ $zoom_out ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_out } для памяншэння,
        }
        Super + пракрутка мышкай
    .scroll_controls = Уключыць маштабаванне мышкай ці тачпадам праз Super + Пракрутка
    .show_overlay = Паказваць накладку лупы
    .increment = Крок маштабавання
    .signin = Запускаць лупу пры ўваходзе
    .applet = Уключыць/выключыць лупу ў аплеце на панэлі
    .movement = Маштабаваны выгляд рухаецца
    .continuous = Бесперапынна за указальнікам
    .onedge = Калі указальнік дасягае краю
    .centered = Каб утрымліваць указальнік у цэнтры
color-filter = Тып колеравага фільтра
    .unknown = Невядомы фільтр актыўны
    .greyscale = Шэры
    .deuteranopia = Зялёны/Чырвоны (зялёная слабасць, дэйтэранапія)
    .protanopia = Чырвоны/Зялёны (чырвоная слабасць, пратанапія)
    .tritanopia = Сіні/Жоўты (сіняя слабасць, трытанапія)

## Desktop

desktop = Рабочы стол

## Desktop: Wallpaper

wallpaper = Шпалеры
    .change = Змяняць малюнак кожныя
    .desc = Малюнкі шпалераў, колеры і опцыі слайд-шоў.
    .fit = Падганяць шпалеры
    .folder-dialog = Выбраць папку з шпалерамі
    .image-dialog = Выбраць малюнак шпалераў
    .plural = Шпалеры
    .same = Адны і тыя ж шпалеры на ўсіх дысплэях
    .slide = Слайд-шоў
add-color = Дадаць колер
add-image = Дадаць малюнак
all-displays = Усе дысплэі
colors = Колеры
dialog-add = _Дадаць
fill = Запоўніць
fit-to-screen = Падганяць да экрана
open-new-folder = Адкрыць новую папку
recent-folders = Апошнія папкі
x-minutes =
    { $number } { $number ->
        [one] мінута
        [few] мінуты
       *[other] мінут
    }
x-hours =
    { $number ->
        [1] 1 гадзіну
        [few] { $number } гадзіны
       *[other] { $number } гадзін
    }
never = Ніколі

## Desktop: Appearance

appearance = Выгляд
    .desc = Дамінуючыя колеры і тэмы COSMIC.
accent-color = Дамінуючы колер
app-background = Фон праграмы ці акна
auto = Аўтаматычна
close = Закрыць
color-picker = Сродак выбару колеру
copied-to-clipboard = Скапіравана ў буфер абмену
copy-to-clipboard = Скапіяваць у буфер абмену
dark = Цёмная
export = Экспарт
hex = Hex
import = Імпартаваць
light = Светлая
mode-and-colors = Рэжым і колеры
recent-colors = Апошнія колеры
reset-to-default = Скінуць да стандартных налад
rgb = RGB
window-hint-accent = Колер падказкі актыўнага акна
window-hint-accent-toggle = Выкарыстоўваць дамінуючы колер тэмы як колер падказкі актыўнага акна
auto-switch = Аўтаматычна пераключацца з Светлага на Цёмны рэжым
    .sunrise = Пераключаецца на Светлы рэжым на ўсходзе
    .sunset = Пераключаецца на Цёмны рэжым на захадзе
    .next-sunrise = Пераключаецца на Светлы рэжым на наступным усходзе
    .next-sunset = Пераключаецца на Цёмны рэжым на наступным захадзе
container-background = Фон кантэйнера
    .desc-detail = Колер фону кантэйнера выкарыстоўваецца для навігацыйных панэляў, бакавых скрынь, дыялогаў і падобных элементаў. Па змаўчанні, ён аўтаматычна вызначаецца з фону праграмы ці акна.
    .reset = Скід да аўтаматычнага рэжыму
    .desc = Асноўны колер кантэйнера выкарыстоўваецца для навігацыйных панэляў, бакавых скрынь, дыялогаў і падобных элементаў.
control-tint = Афарбоўка кампанентаў кіравання
    .desc = Выкарыстоўваецца для фонаў стандартных кнопак, палёў пошуку, палёў уводу тэксту і падобных кампанентаў.
frosted = Эфект замарожанага шкла ў сістэмным інтэрфейсе
    .desc = Прымяняе размыццё фону да панэлі, дока, віджэтаў, праграмы запуску і бібліятэкі праграм.
enable-export = Прымяніць гэтую тэму да праграм GNOME.
    .desc = Не ўсе інструменты падтрымліваюць аўтаматычнае пераключэнне. Для праграм, якія не належаць да COSMIC, можа спатрэбіцца перазапуск пасля змены тэмы.
icon-theme = Тэма значкоў
    .desc = Прымяняе іншы набор значкоў да праграм.
text-tint = Афарбоўка тэксту інтэрфейсу
    .desc = Колер, які выкарыстоўваецца для вызначэння колераў тэксту інтэрфейсу, што мае дастатковы кантраст на розных паверхнях.
style = Стыль
    .round = Круглы
    .slightly-round = Злёгку круглы
    .square = Квадратны
interface-density = Шчыльнасць інтэрфейсу
    .comfortable = Зручна
    .compact = Кампактна
    .spacious = Прасторна
window-management-appearance = Кіраванне вокнамі
    .active-hint = Памер падказкі актыўнага акна
    .gaps = Прамежкі вакол вакон, размешчаных у сетцы

### Experimental

experimental-settings = Эксперыментальныя налады
icons-and-toolkit = Тэмы значкоў і набору інструментаў
interface-font = Сістэмны шрыфт
monospace-font = Манашырынны шрыфт

## Desktop: Notifications

notifications = Апавяшчэнні
    .desc = Не турбаваць, апавяшчэнні на экране блакіроўкі і налады для кожнай праграмы.

## Desktop: Panel

panel = Панэль
    .desc = Верхняя панэль з элементамі кіравання і меню.
add = Дадаць
add-applet = Дадаць віджэт
all = Усе
applets = Аплеты
center-segment = Цэнтральны сегмент
end-segment = Канчатковы сегмент
large = Вялікі
no-applets-found = Віджэты не знойдзены...
panel-bottom = Унізе
panel-left = Злева
panel-right = Справа
panel-top = Зверху
search-applets = Пошук віджэтаў...
small = Маленькі
start-segment = Пачатковы сегмент
panel-appearance = Выгляд
    .match = Як у сістэме
    .light = Светлая
    .dark = Цёмная
panel-behavior-and-position = Паводзіны і пазіцыі
    .autohide = Аўтаматычна хаваць панэль
    .dock-autohide = Аўтаматычна хаваць док
    .position = Пазіцыя на экране
    .display = Паказаць на дысплэі
panel-style = Стыль
    .anchor-gap = Прамежак паміж панэллю і краямі экрана
    .dock-anchor-gap = Прамежак паміж докам і краямі экрана
    .extend = Пашыраць панэль да краёў экрана
    .dock-extend = Пашыраць док да краёў экрана
    .appearance = Выгляд
    .size = Памер
    .background-opacity = Непразрыстасць фону
panel-applets = Налады
    .dock-desc = Налады віджэтаў дока
    .desc = Налады віджэтаў панэлі
panel-missing = Налады панэлі адсутнічаюць
    .desc = Файл налад панэлі адсутнічае з-за выкарыстання карыстальніцкай канфігурацыі або ён пашкоджаны.
    .fix = Скід да стандартных налад

## Desktop: Dock

dock = Док
    .desc = Панэль з замацаванымі праграмамі.

## Desktop: Window management

window-management = Кіраванне вокнамі
    .desc = Дзеянне клавішы Super, параметры кіравання вокнамі і дадатковыя налады размяшчэння вокнаў.
super-key = Дзеянне клавішы Super
    .launcher = Адкрыць праграму запуску
    .workspaces = Адкрыць працоўныя прасторы
    .applications = Адкрыць праграмы
    .disable = Адключыць
edge-gravity = Плаваючыя вокны прыцягваюцца да бліжэйшых краёў
window-controls = Кіраванне вокнамі
    .minimize = Паказаць кнопку згортвання
    .maximize = Паказаць кнопку разгортвання
    .active-window-hint = Паказваць падказку актыўнага акна
focus-navigation = Кіраванне фокусам
    .focus-follows-cursor = Фокус ідзе за курсорам
    .focus-follows-cursor-delay = Затрымка фокусу пасля курсору (мс.)
    .cursor-follows-focus = Курсор ідзе за фокусам

## Desktop: Workspaces

workspaces = Працоўныя прасторы
    .desc = Усталяванне колькасці працоўных прастораў, іх паводзіны і размяшчэнне.
workspaces-behavior = Паводзіны працоўных прастораў
    .dynamic = Дынамічныя працоўныя прасторы
    .dynamic-desc = Аўтаматычна выдаляе пустыя працоўныя прасторы.
    .fixed = Фіксаваная колькасць працоўных прастораў
    .fixed-desc = Дадаваць або выдаляць працоўныя прасторы ў аглядзе.
workspaces-multi-behavior = Паводзіны на некалькіх маніторах
    .span = Працоўныя прасторы ахопліваюць усе дысплэі
    .separate = Кожны дысплэй мае свае працоўныя прасторы
workspaces-overview-thumbnails = Мініяцюры агляду працоўных прастораў
    .show-number = Паказваць нумар працоўнай прасторы
    .show-name = Паказваць назву працоўнай прасторы
workspaces-orientation = Арыентацыя працоўных прастораў
    .vertical = Вертыкальная
    .horizontal = Гарызантальная
hot-corner = Гарачы кут
    .top-left-corner = Уключыць верхні левы гарачы кут для працоўных прастораў

## Displays

-requires-restart = Патрабуецца перазапуск
color = Колер
    .depth = Глыбіня колеру
    .profile = Профіль колеру
    .sidebar = Профілі колераў
    .temperature = Тэмпература колеру
display = Дысплэі
    .desc = Кіраванне дысплэямі, пераключэнне графікі і начное святло
    .arrangement = Размяшчэнне дысплэяў
    .arrangement-desc = Перацягвайце дысплэі для іх перапарадкавання.
    .enable = Уключыць дысплэй
    .external = { $size } { $output } Знешні дысплэй
    .laptop = { $size } Дысплэй ноўтбука
    .options = Опцыі дысплэяў
    .refresh-rate = Частата абнаўлення
    .resolution = Раздзяляльнасць
    .scale = Масштаб
    .additional-scale-options = Дадатковыя параметры маштабавання
mirroring = Адлюстраванне
    .id = Адлюстраванне { $id }
    .dont = Не люстраваць
    .mirror = Люстраваць { $display }
    .project =
        Праекцыя на { $display ->
            [all] усе дысплэі
           *[other] { $display }
        }
    .project-count =
        Праецыюецца на { $count } іншыя { $count ->
            [1] дысплэй
           *[other] дысплэі
        }
night-light = Начное святло
    .auto = Аўтаматычна (з заходу да ўсходу)
    .desc = Паменшыць сіняе святло, выкарыстоўваць больш цёплыя колеры.
orientation = Арыентацыя
    .standard = Стандартная
    .rotate-90 = Павярнуць на 90
    .rotate-180 = Павярнуць на 180
    .rotate-270 = Павярнуць на 270
vrr = Пераменная частата абнаўлення
    .enabled = Уключана
    .force = Заўсёды
    .auto = Аўтаматычна
    .disabled = Адключана
scheduling = Планаванне
    .manual = Ручное планаванне
dialog = Дыялог
    .title = Захаваць гэтыя налады адлюстравання?
    .keep-changes = Захаваць змены
    .change-prompt = Змены налад будуць аўтаматычна вернуты праз { $time } секунд.
    .revert-settings = Вярнуць налады

## Sound

sound = Гук
    .desc = N/A
sound-output = Выхад
    .volume = Выхадная гучнасць
    .device = Выхадная прылада
    .level = Выхадны ўзровень
    .config = Налады
    .balance = Баланс
    .left = Лева
    .right = Права
sound-input = Уваход
    .volume = Уваходная гучнасць
    .device = Уваходная прылада
    .level = Уваходны ўзровень
sound-alerts = Абвесткі
    .volume = Гучнасць абвестак
    .sound = Гук абвестак
sound-applications = Праграмы
    .desc = Гучнасць праграмы і налады

## Power

power = Сілкаванне і батарэя
    .desc = Кіраванне наладамі сілкавання
battery = Батарэя
    .minute =
        { $value } { $value ->
            [one] мінута
            [few] мінуты
           *[other] мінут
        }
    .hour =
        { $value } { $value ->
            [one] гадзіна
            [few] гадзіны
           *[other] гадзін
        }
    .day =
        { $value } { $value ->
            [one] дзень
            [few] дні
           *[other] дзён
        }
    .less-than-minute = Менш за мінуту
    .and = і
    .remaining-time =
        { $time } да { $action ->
            [full] запаўнення
           *[other] разрадкі
        }
connected-devices = Падключаныя прылады
    .unknown = Невядомая прылада
power-mode = Рэжым сілкавання
    .battery = Павялічаны тэрмін службы батарэі
    .battery-desc = Зніжэнне спажывання энергіі і бясшумная праца.
    .balanced = Збалансаваны
    .balanced-desc = Ціхая праца і ўмеранае энергаспажыванне.
    .performance = Высокая прадукцыйнасць
    .performance-desc = Пікавая прадукцыйнасць і энергаспажыванне.
    .no-backend = Бэкенд не знойдзены. Усталюйце system76-power або power-profiles-daemon.
power-saving = Параметры эканоміі энергіі
    .turn-off-screen-after = Выключыць экран праз
    .auto-suspend = Аўтаматычнае прыпыненне
    .auto-suspend-ac = Аўтаматычнае прыпыненне пры падключэнні да электрасеткі
    .auto-suspend-battery = Аўтаматычнае прыпыненне ад батарэі

## Input

acceleration-desc = Аўтаматычна рэгулюе адчувальнасць адсочвання ў залежнасці ад хуткасці.
disable-while-typing = Адключаць падчас набора тэксту
input-devices = Прылады ўводу
    .desc = Прылады ўводу
primary-button = Асноўная кнопка
    .desc = Задае парадак фізічных кнопак.
    .left = Левая
    .right = Правая
scrolling = Прагортка
    .two-finger = Прагортка двума пальцамі
    .edge = Прагортка ўздоўж краю адным пальцам
    .speed = Хуткасць прагорткі
    .natural = Натуральная прагортка
    .natural-desc = Прагортка змесціва, а не выгляду

## Input: Keyboard

slow = Павольна
fast = Хутка
short = Коратка
long = Доўга
keyboard = Клавіятура
    .desc = Крыніцы ўводу, пераключэнне, увод спецыяльных сімвалаў, спалучэнні клавіш.
keyboard-sources = Крыніцы ўводу
    .desc = Крыніцы ўводу могуць пераключацца з дапамогай камбінацыі клавіш Super+Space. Гэта можа быць наладжана ў наладах клавішавых спалучэнняў.
    .move-up = Перасунуць уверх
    .move-down = Перасунуць уніз
    .settings = Налады
    .view-layout = Паказаць раскладку клавіятуры
    .remove = Выдаліць
    .add = Дадаць крыніцу ўводу
keyboard-special-char = Увод спецыяльных сімвалаў
    .alternate = Клавіша альтэрнатыўных сімвалаў
    .compose = Клавіша састаўлення
    .caps = Клавіша Caps Lock
keyboard-typing-assist = Увод
    .repeat-rate = Хуткасць паўтарэння
    .repeat-delay = Затрымка паўтарэння
keyboard-numlock-boot = Numlock
    .boot-state = Стан пры загрузцы
    .last-boot = Апошняя загрузка
    .on = Уключаны
    .off = Выключаны
    .set = Задаць стан NumLock пры загрузцы
added = Дададзена
type-to-search = Набярыце для пошуку...
show-extended-input-sources = Паказаць пашыраныя крыніцу ўводу

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Спалучэнні клавіш
    .desc = Паказаць і наладзіць спалучэнні
cancel = Скасаваць
command = Каманда
custom = Карыстальніцкія
debug = Адладка
disabled = Адключана
input-source-switch = Пераключыць крыніцу ўводу мовы клавіятуры
migrate-workspace-prev = Перанесці працоўную прастору на папярэдні вывад
migrate-workspace-next = Перанесці працоўную прастору на наступны вывад
migrate-workspace =
    Перанесці працоўную прастору на вывад { $direction ->
       *[down] знізу
        [left] злева
        [right] справа
        [up] зверху
    }
navigate = Перайсці
replace = Замяніць
shortcut-name = Назва спалучэння
system-controls = Элементы кантроля сістэмай
terminate = Завяршыць
toggle-stacking = Пераключыць стасаванне акна
type-key-combination = Увядзіце камбінацыю клавіш
custom-shortcuts = Карыстальніцкія спалучэнні
    .add = Дадаць спалучэнне
    .context = Дадаць карыстальніцкае спалучэнне
    .none = Няма карыстальніцкіх спалучэнняў
modified = { $count } зменена
nav-shortcuts = Навігацыя
    .prev-output = Фокус на папярэдні вывад
    .next-output = Фокус на наступны вывад
    .last-workspace = Фокус на апошнюю працоўную прастору
    .prev-workspace = Фокус на папярэднюю працоўную прастору
    .next-workspace = Фокус на наступную працоўную прастору
    .focus =
        Фокус на акно { $direction ->
           *[down] знізу
            [in] ўнутры
            [left] злева
            [out] вонку
            [right] справа
            [up] зверху
        }
    .output =
        Фокус на вывад { $direction ->
           *[down] знізу
            [left] злева
            [right] справа
            [up] зверху
        }
    .workspace = Перайсці на працоўную прастору { $num }
manage-windows = Кіраванне вокнамі
    .close = Закрыць акно
    .maximize = Разгарнуць акно
    .fullscreen = Акно на ўвесь экран
    .minimize = Згарнуць акно
    .resize-inwards = Змяніць памер акна ўнутр
    .resize-outwards = Змяніць памер акна вонкі
    .toggle-sticky = Пераключыць ліпкае акно
move-windows = Перамяшчэнне вакон
    .direction =
        Перамясціць акно { $direction ->
           *[down] ўніз
            [left] ўлева
            [right] ўправа
            [up] ўверх
        }
    .display =
        Перамясціць акно на адзін манітор { $direction ->
           *[down] ніжэй
            [left] лявей
            [right] правей
            [up] вышэй
        }
    .workspace =
        Перамясіць акно на адну працоўную прастору { $direction ->
           *[below] ніжэй
            [left] лявей
            [right] правей
            [above] вышэй
        }
    .workspace-num = Перамясціць акно на працоўную прастору { $num }
    .prev-workspace = Перамясціць акно на папярэднюю працоўную прастору
    .next-workspace = Перамясціць акно на наступную працоўную прастору
    .last-workspace = Перамясціць акно на апошнюю працоўную прастору
    .next-display = Перамясціць акно на наступнэ дысплэй
    .prev-display = Перамясціць акно на папярэдні дысплэй
    .send-to-prev-workspace = Перамясціць акно на папярэднюю працоўную прастору
    .send-to-next-workspace = Перамясціць акно на наступную працоўную прастору
system-shortcut = Сістэма
    .app-library = Адкрыць бібліятэку праграм
    .brightness-down = Паменшыць яркасць дысплэя
    .brightness-up = Павялічыць яркасць дысплэя
    .display-toggle = Пераключыць унутраны дысплэй
    .home-folder = Адкрыць дамашнюю папку
    .keyboard-brightness-down = Паменшыць яркасць клавіятуры
    .keyboard-brightness-up = Павялічыць яркасць клавіятуры
    .launcher = Адкрыць праграму запуску
    .log-out = Выйсці
    .lock-screen = Заблакіраваць экран
    .mute = Выключыць вывад гуку
    .mute-mic = Выключыць мікрафон
    .play-pause = Прайграванне/Паўза
    .play-next = Наступны трэк
    .play-prev = Папярэдні трэк
    .poweroff = Выключэнне камп'ютара
    .screenshot = Зрабіць здымак экрана
    .terminal = Адкрыць тэрмінал
    .touchpad-toggle = Уключыць або выключыць сэнсарную панэль
    .volume-lower = Паменшыць гучнасць
    .volume-raise = Павялічыць гучнасць
    .web-browser = Адкрыць вэб-браўзер
    .window-switcher = Пераключыцца паміж адкрытымі вокнамі
    .window-switcher-previous = Пераключыцца паміж адкрытымі вокнамі ў зваротным парадку
    .workspace-overview = Адкрыць агляд працоўных прастор
window-tiling = Укладанне вокнаў
    .horizontal = Задаць гарызантальную арыентацыю
    .vertical = Задаць вертыкальную арыентацыю
    .swap-window = Памяняць акно
    .toggle-tiling = Пераключыць укладанне акна
    .toggle-stacking = Пераключыць стасаванне акна
    .toggle-floating = Пераключыць плавучасць акна
    .toggle-orientation = Пераключыць арыентацыю
replace-shortcut-dialog = Замянць спалучэнне?
    .desc = { $shortcut } выкарыстоўваецца для { $name }. Калі вы заменіце яго, { $name } будзе адаключана.
zoom-in = Павялічыць
zoom-out = Паменшыць

## Input: Mouse

mouse = Мыш
    .desc = Хуткасць мышы, паскарэнне, натуральная прагортка.
    .speed = Хуткасць мышы
    .acceleration = Уключыць паскарэнне мышы

## Input: Touchpad

click-behavior = Паводзіны пры націску
    .click-finger = Другасны клік актывуецца двума пальцамі, а сярэдні клік — трыма пальцамі
    .button-areas = Другасны клік актывуецца ў ніжнім правым вугле, а сярэдні клік — у ніжнім цэнтры
pinch-to-zoom = Шчыпок для маштабавання
    .desc = Выкарыстоўвайце два пальцы для маштабавання змесціва праграм, якія падтрымліваюць маштабаванне.
tap-to-click = Націск для кліку
    .desc = Уключае націск адным пальцам для асноўнага кліку, двума пальцамі — для другаснага кліку і трыма пальцамі — для сярэдняга кліку.
touchpad = Сэнсарная панэль
    .acceleration = Уключыць паскарэнне сэнсарнай панэлі
    .desc = Хуткасць сэнсарнай панэлі, параметры націскаў, жэсты.
    .speed = Хуткасць сэнсарнай панэлі

## Input: Gestures

gestures = Жэсты
    .four-finger-down = Правядзенне чатырох пальцаў уніз
    .four-finger-left = Правядзенне чатырох пальцаў улева
    .four-finger-right = Правядзенне чатырох пальцаў управа
    .four-finger-up = Правядзенне чатырох пальцаў уверх
    .three-finger-any = Правядзенне трох пальцаў у любым напрамку
switch-workspaces = Пераключыць працоўныя прасторы
    .horizontal = Правядзіце чатырма пальцамі ўлева/ўправа
    .vertical = Правядзіце чатырма пальцамі ўверх/ўніз
switch-between-windows = Пераключэнне паміж вокнамі
open-application-library = Адкрыць бібліятэку праграм
open-workspaces-view = Адкрыць агляд працоўных прастораў

## Time & Language

time = Час і мова
    .desc = N/A
time-date = Дата і час
    .desc = Часавы пояс, аўтаматычныя налады гадзінніка і фарматаванне часу.
    .auto = Задаваць аўтаматычна
    .auto-ntp = Дата і час будуць абнаўляцца аўтаматычка, калі заданы часавы пояс.
time-zone = Часовы пояс
    .auto = Аўтаматычны часовы пояс
    .auto-info = Патрабуецца доступ да месцазнаходжання і інтэрнэту
time-format = Фармат даты і часу
    .twenty-four = 24-гадзінны фармат
    .show-seconds = Паказваць секунды
    .first = Першы дзень тыдня
    .show-date = Паказваць дату на верхняй панэлі
    .friday = Пятніца
    .saturday = Субота
    .sunday = Нядзеля
    .monday = Панядзелак
time-region = Рэгіён і мова
    .desc = Фармат даты, часу і лікаў у адпаведнасці з рэгіёнам.
formatting = Фарматаванне
    .dates = Даты
    .time = Час
    .date-and-time = Дата і час
    .numbers = Лічбы
    .measurement = Адзінкі вымярэння
    .paper = Папера
preferred-languages = Выбраныя мовы
    .desc = Парадак моў вызначае, якая мова будзе выкарыстоўвацца для перакладу асяроддзя працоўнага стала. Змены ўступяць у сілу пасля наступнага ўваходу.
add-language = Дадаць мову
    .context = Дадаць Мову
install-additional-languages = Усталяваць дадатковыя мовы
region = Рэгіён

## Applications

applications = Праграмы

## Applications: Default Applications

default-apps = Праграмы па змаўчанні
    .desc = Вэб-браўзер па змаўчанні, паштовы кліент, файлавы мэнэджар і іншыя праграмы.
    .web-browser = Вэб-браўзер
    .file-manager = Файлавы мэнэджар
    .mail-client = Пошта
    .music = Музыка
    .video = Відэа
    .photos = Фота
    .calendar = Календар
    .terminal = Тэрмінал
    .other-associations = Іншыя асацыяцыі
    .text-editor = Тэкставы рэдактар

## Applications: Startup Applications

startup-apps = Праграмы пры запуску
    .desc = Наладзьце праграмы, якія запускаюцца пры ўваходзе ў сістэму.
    .add = Дадаць праграму
    .user = Праграмы, якія запускаюцца падчас вашага ўвахода ў сістэму
    .none = У аўтазапуску няма праграм
    .remove-dialog-title = Выдаліць { $name }?
    .remove-dialog-description = Вы сапраўды жадаеце выдаліць гэта як праграму для запуску?
    .search-for-application = Пошук праграмы

## Applications: Legacy Applications

legacy-applications = Сумяшчальнасць праграм X11
    .desc = Маштабаванне праграм аконнай сістэмы X11 і глабальныя спалучэнні клавіш.
legacy-app-global-shortcuts = Глабальныя спалучэнні клавіш у праграмах X11
    .desc = Глабальныя спалучэнні клавіш дазваляюць распазнаваць націсканні клавіш і падзеі кнопак мышы ў праграмах іншымі праграмамі для такіх функцый, як "націсні і кажы" або "націсні і адключы гук". Па змаўчанні гэта адключана ў праграмах X11, каб іншыя праграмы не маглі маніторыць падзеі клавіятуры і мышы, якія змяшчаюць канфідэнцыйную інфармацыю.
    .none = Без клавіш
    .modifiers = Мадыфікатары (Super, Shift, Control, Alt)
    .combination = Усе клавішы, пакуль націснутыя мадыфікатары Super, Control або Alt
    .all = Усе клавішы
    .mouse = Падзеі кнопак мышы ў праграмах X11
legacy-app-scaling = Маштабаванне праграм аконнай сістэмы X11
    .scaled-gaming = Аптымізаваць для гульняў і поўнаэкранных праграм
    .gaming-description = Праграмы X11 могуць выглядаць крыху большымі/меншымі ў параўнанні з праграмамі Wayland.
    .scaled-applications = Аптымізаваць для праграм
    .applications-description = Гульні і поўнаэкранныя праграмы X11 могуць не адпавядаць разрозненню вашага дысплея.
    .scaled-compatibility = Рэжым максімальнай сумяшчальнасці
    .compatibility-description = Праграмы X11 могуць выглядаць размытымі на HiDPI-экранах.
    .preferred-display = Выбраны дысплей для гульняў і поўнаэкранных праграм X11
    .no-display = Няма

## System

system = Сістэма і акаўнты

## System: About

about = Пра сістэму
    .desc = Назва прылады, інфармацыя аб абсталяванні, стандартныя налады аперацыйнай сістэмы.
about-device = Назва прылады
    .desc = Гэтая назва адлюстроўваецца іншым сеткавым або блютуз прыладам.
about-hardware = Абсталяванне
    .model = Мадэль абсталявання
    .memory = Памяць
    .processor = Працэсар
    .graphics = Графіка
    .disk-capacity = Ёмістасць дыску
about-os = Аперацыйная сістэма
    .os = Аперацыйная сістэма
    .os-architecture = Архітэктура аперацыйнай сістэмы
    .desktop-environment = Асяроддзе працоўнага стала
    .windowing-system = Сістэма вокнаў
about-related = Спадарожныя налады
    .support = Атрымаць падтрымку

## System: Firmware

firmware = Праграмнае забеспячэнне
    .desc = Дэталі праграмнага забеспячэння.

## System: Users

users = Карыстальнікі
    .desc = Аўтэнтыфікацыя і ўваход, экран блакіроўкі.
    .admin = Адміністратар
    .standard = Звычайны
    .profile-add = Выбраць выяву профілю
administrator = Адміністратар
    .desc = Адміністратары могуць змяняць налады для ўсіх карыстальнікаў, дадаваць і выдаляць іншых карыстальнікаў.
add-user = Дадаць карыстальніка
change-password = Змяніць пароль
remove-user = Выдаліць карыстальніка
full-name = Поўнае імя
invalid-username = Недапушчальнае імя карыстальніка.
password-mismatch = Пароль і пацвярджэнне мусяць супадаць.
save = Захаваць
amplification = Узмацненне
    .desc = Дазваляе павялічваць гучнасць да 150%.
add-another-keybinding = Дадаць спалучэнне
