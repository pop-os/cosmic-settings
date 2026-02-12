app = COSMIC баптаулары
dbus-connection-error = DBus-қа қосылу сәтсіз аяқталды
ok = ОК
unknown = Белгісіз
number = { $number }
connections-and-profiles =
    { $variant ->
        [wired] Сымды
        [wifi] Wi-Fi
        [vpn] VPN
       *[other] Белгісіз
    } байланыстар мен байланыс профильдері.
add-network = Желіні қосу
    .profile = Профильді қосу
add-vpn = VPN қосу
airplane-on = Ұшақ режимі іске қосулы.
cable-unplugged = Кабель ажыратылған
connect = Қосылу
connected = Қосылды
connecting = Қосылуда…
disconnect = Ажырату
forget = Ұмыту
known-networks = Белгілі желілер
network-and-wireless = Желі және сымсыз байланыс
network-name = Желі атауы
no-networks = Желілер табылмады.
no-vpn = Қолжетімді VPN байланыстары жоқ.
password = Пароль
password-confirm = Парольді растау
qr-code-unavailable = QR коды қолжетімсіз
remove = Өшіру
scan-to-connect-description = Осы желіге қосылу үшін QR кодын сканерлеңіз.
settings = Баптаулар
share = Желімен бөлісу
username = Пайдаланушы аты
visible-networks = Көрінетін желілер
identity = Сәйкестендіру
auth-dialog = Аутентификация қажет
    .vpn-description = VPN қызметіне қажетті пайдаланушы атын және парольді енгізіңіз.
    .wifi-description = Парольді немесе шифрлеу кілтін енгізіңіз. Сондай-ақ роутердегі «WPS» батырмасын басу арқылы қосылуға болады.
forget-dialog = Осы Wi-Fi желісін ұмыту керек пе?
    .description = Болашақта осы Wi-Fi желісін пайдалану үшін парольді қайта енгізу қажет болады.
network-device-state =
    .activated = Қосылды
    .config = Қосылуда
    .deactivating = Ажыратылуда
    .disconnected = Ажыратылған
    .failed = Қосылу сәтсіз аяқталды
    .ip-check = Байланысты тексеру
    .ip-config = IP және маршруттау ақпаратын сұрау
    .need-auth = Аутентификация қажет
    .prepare = Қосылуға дайындалу
    .secondaries = Қосымша байланысты күту
    .unavailable = Қолжетімсіз
    .unknown = Белгісіз күй
    .unmanaged = Басқарылмайтын
    .unplugged = Кабель ажыратылған
remove-connection-dialog = Байланыс профилін өшіру керек пе?
    .vpn-description = Болашақта осы желіні пайдалану үшін парольді қайта енгізу қажет болады.
    .wired-description = Болашақта оны пайдалану үшін осы профильді қайта жасау қажет болады.
vpn = VPN
    .connections = VPN байланыстары
    .error = VPN конфигурациясын қосу сәтсіз аяқталды
    .remove = Байланыс профилін өшіру
    .select-file = VPN конфигурациялық файлын таңдаңыз
vpn-error = VPN қатесі
    .config = VPN конфигурациясын қосу сәтсіз аяқталды
    .connect = VPN-ге қосылу сәтсіз аяқталды
    .connection-editor = Байланыс редакторының жұмысы сәтсіз аяқталды
    .connection-settings = Белсенді байланыстар үшін баптауларды алу сәтсіз аяқталды
    .updating-state = Желі менеджерінің күйін жаңарту сәтсіз аяқталды
    .wireguard-config-path = WireGuard конфигурациясы үшін файл жолы жарамсыз
    .wireguard-config-path-desc = Таңдалған файл жергілікті файлдық жүйеде болуы тиіс.
    .wireguard-device = WireGuard құрылғысын жасау сәтсіз аяқталды
    .with-password =
        VPN { $field ->
           *[username] пайдаланушы атын
            [password] паролін
            [password-flags] пароль жалаушаларын
        } nmcli арқылы орнату сәтсіз аяқталды
wired = Сымды
    .adapter = Сымды адаптер { $id }
    .connections = Сымды байланыстар
    .devices = Сымды құрылғылар
    .remove = Байланыс профилін өшіру
wifi = Wi-Fi
    .adapter = Wi-Fi адаптері { $id }
    .forget = Осы желіні ұмыту
wireguard-dialog = WireGuard құрылғысын қосу
    .description = WireGuard конфигурациясы үшін құрылғы атауын таңдаңыз.
online-accounts = Желілік тіркелгілер
    .desc = Тіркелгілерді, IMAP және SMTP, кәсіпорындық кіру мәліметтерін қосу
activate = Белсендіру
confirm = Растау
enable = Іске қосу
bluetooth = Bluetooth
    .desc = Bluetooth құрылғыларын басқару
    .status = Bluetooth баптаулары ашық болғанда, бұл жүйе { $aliases } ретінде көрінеді.
    .connected = Қосылды
    .connecting = Қосылуда
    .disconnecting = Ажыратылуда
    .connect = Қосылу
    .disconnect = Ажырату
    .forget = Ұмыту
    .dbus-error = DBus-пен әрекеттесу кезінде қате орын алды: { $why }
    .disabled = Bluetooth қызметі сөндірулі
    .inactive = Bluetooth қызметі белсенді емес
    .unknown = Bluetooth қызметін іске қосу мүмкін болмады. BlueZ орнатылған ба?
bluetooth-paired = Бұрын қосылған құрылғылар
    .connect = Қосылу
    .battery = Батарея заряды { $percentage }%
bluetooth-confirm-pin = Bluetooth PIN кодын растау
    .description = Келесі PIN коды { $device } құрылғысында көрсетілген кодқа сәйкес келетінін растаңыз
bluetooth-available = Жақын маңдағы құрылғылар
bluetooth-adapters = Bluetooth адаптерлері
accessibility = Арнайы мүмкіндіктер
    .vision = Көру қабілеті
    .on = Іске қосулы
    .off = Өшірулі
    .unavailable = Қолжетімсіз
    .screen-reader = Экрандық диктор
    .high-contrast = Жоғары контраст режимі
    .invert-colors = Түстерді инверсиялау
    .color-filters = Түс сүзгілері
hearing = Есту қабілеті
    .mono = Стерео аудионы моно ретінде ойнату
default = Әдепкі
magnifier = Үлкейткіш
    .controls =
        Немесе келесі жарлықтарды қолданыңыз: { $zoom_in ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_in } үлкейту үшін,
        }{ $zoom_out ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_out } кішірейту үшін,
        }
        Super + тышқанмен айналдыру
    .scroll_controls = Super + айналдыру арқылы тышқан немесе тачпадпен үлкейтуді іске қосу
    .show_overlay = Үлкейткіш қабатын көрсету
    .increment = Масштаб қадамы
    .signin = Жүйеге кіргенде үлкейткішті іске қосу
    .applet = Панельдегі апплетте үлкейткішті қосу/сөндіру
    .movement = Үлкейтілген көріністің жылжуы
    .continuous = Курсормен бірге үздіксіз
    .onedge = Курсор жиекке жеткенде
    .centered = Курсорды ортада ұстау үшін
color-filter = Түс фильтрінің түрі
    .unknown = Белгісіз фильтр белсенді
    .greyscale = Сұр түстер шкаласы
    .deuteranopia = Жасыл/Қызыл (жасыл түсті нашар қабылдау, дейтеранопия)
    .protanopia = Қызыл/Жасыл (қызыл түсті нашар қабылдау, протанопия)
    .tritanopia = Көк/Сары (көк түсті нашар қабылдау, тританопия)
desktop = Жұмыс үстелі
wallpaper = Тұсқағаз
    .change = Суретті келесі уақыт сайын өзгерту
    .desc = Тұсқағаз суреттері, түстері және слайдшоу опциялары
    .fit = Тұсқағазды сәйкестендіру
    .folder-dialog = Тұсқағаз бумасын таңдау
    .image-dialog = Тұсқағаз суретін таңдау
    .plural = Тұсқағаздар
    .same = Барлық дисплейлерде бірдей тұсқағаз
    .slide = Слайдшоу
add-color = Түс қосу
add-image = Сурет қосу
all-displays = Барлық дисплейлер
colors = Түстер
dialog-add = Қосу
fill = Толтыру
fit-to-screen = Экранға сыйдыру
open-new-folder = Жаңа буманы ашу
recent-folders = Соңғы бумалар
x-minutes =
    { $number } { $number ->
        [one] минут
       *[other] минут
    }
x-hours =
    { $number } { $number ->
        [one] сағат
       *[other] сағат
    }
never = Ешқашан
appearance = Сыртқы түрі
    .desc = Акцентті түстер мен темалар
accent-color = Акцент түсі
app-background = Терезе фоны
auto = Автоматты түрде
close = Жабу
color-picker = Түс таңдаушы
copied-to-clipboard = Алмасу буферіне көшірілді
copy-to-clipboard = Алмасу буферіне көшіру
dark = Қараңғы
export = Экспорттау
hex = Он алтылық
import = Импорттау
light = Жарық
mode-and-colors = Режим мен түстер
recent-colors = Соңғы түстер
reset-to-default = Әдепкі күйге қайтару
rgb = RGB
window-hint-accent = Белсенді терезенің тұспал түсі
window-hint-accent-toggle = Теманың акцентті түсін белсенді терезенің тұспалы ретінде қолдану
auto-switch = Жарық және қараңғы режимдер арасында автоматты түрде ауысу
    .sunrise = Күн шыққанда жарық режиміне ауысады
    .sunset = Күн батқанда қараңғы режиміне ауысады
    .next-sunrise = Келесі күн шыққанда жарық режиміне ауысады
    .next-sunset = Келесі күн батқанда қараңғы режиміне ауысады
shadows-floating = Қалқымалы терезелер
    .clip = Жүйелік бұрыштарға сәйкестендіру және көлеңкелерді іске асыру
shadows-tiling = Плиткалы терезелер
    .clip = Жүйелік бұрыштарға сәйкестендіру
    .shadow = Көлеңкелерді іске асыру
container-background = Контейнер фоны
    .desc-detail = Контейнердің фон түсі навигацияның бүйірлік панелі, бүйірлік тартпа, сұхбат терезелері және ұқсас виджеттер үшін қолданылады. Әдепкі бойынша, контейнердің фон түсі терезе фонынан автоматты түрде алынады.
    .reset = Авто күйіне қайтару
    .desc = Навигацияның бүйірлік панелі, бүйірлік тартпа, сұхбат терезелері және ұқсас виджеттер үшін қолданылады
control-tint = Басқару компоненттерінің реңкі
    .desc = Стандартты батырмалардың фоны, іздеу өрістері, мәтін енгізу өрістері және ұқсас компоненттер үшін қолданылады
frosted = Жүйелік интерфейстегі күңгірт шыны эффектісі
    .desc = Панель, док, апплеттер, іске қосқыш және қолданбалар кітапханасына фонның бұлыңғырлығын іске асырады
enable-export = Ағымдағы тақырыпты GNOME қолданбаларына іске асыру
    .desc = Барлық интерфейс жиынтықтары автоматты түрде ауысуды қолдамайды. COSMIC-ке жатпайтын қолданбаларды тақырып өзгергеннен кейін қайта іске қосу қажет болуы мүмкін.
icon-theme = Таңбашалар тақырыбы
    .desc = Қолданбаларға басқа таңбашалар жиынтығын іске асырады
text-tint = Интерфейс мәтінінің реңкі
    .desc = Әртүрлі беттерде жеткілікті контрасты бар интерфейс мәтінінің түстерін алу үшін қолданылады
style = Стиль
    .round = Дөңгелек
    .slightly-round = Сәл дөңгелектелген
    .square = Шаршы
interface-density = Интерфейс тығыздығы
    .comfortable = Ыңғайлы
    .compact = Ықшам
    .spacious = Кең
window-management-appearance = Терезелерді басқару
    .active-hint = Белсенді терезенің тұспал өлшемі
    .gaps = Плиткалы терезелердің айналасындағы аралықтар
experimental-settings = Эксперименттік баптаулар
icons-and-toolkit = Таңбашалар мен интерфейс жиынтығының темалары
interface-font = Жүйелік қаріп
monospace-font = Ені біркелкі қаріп
shadow-and-corners = Терезе көлеңкесі мен бұрыштары
notifications = Хабарландырулар
    .desc = "Мазаламау" режимі, экран құлыптаулы кездегі хабарландырулар және әр қолданбаға арналған баптаулар
panel = Панель
    .desc = Мәзірлер мен апплеттерге арналған негізгі жүйелік жолақ
add = Қосу
add-applet = Апплет қосу
all = Барлығы
applets = Апплеттер
center-segment = Ортаңғы сегмент
place-here = Апплеттерді осында орналастырыңыз
end-segment = Соңғы сегмент
large = Үлкен
no-applets-found = Апплеттер табылмады...
panel-bottom = Төменде
panel-left = Сол жақта
panel-right = Оң жақта
panel-top = Жоғарыда
search-applets = Апплеттерді іздеу...
small = Кіші
start-segment = Бастапқы сегмент
panel-appearance = Көрініс
    .match = Жұмыс үстеліне сәйкестендіру
    .light = Жарық
    .dark = Күңгірт
panel-behavior-and-position = Мінез-құлқы және орындары
    .autohide = Панельді автоматты түрде жасыру
    .dock-autohide = Докты автоматты түрде жасыру
    .position = Экрандағы орны
    .display = Дисплейде көрсету
panel-style = Стиль
    .anchor-gap = Панель мен экран шеттерінің арасындағы аралық
    .dock-anchor-gap = Док пен экран шеттерінің арасындағы аралық
    .extend = Панельді экран шеттеріне дейін созу
    .dock-extend = Докты экран шеттеріне дейін созу
    .appearance = Көрініс
    .size = Өлшемі
    .background-opacity = Фон мөлдірлігі
panel-applets = Конфигурация
    .dock-desc = Док апплеттерін баптау
    .desc = Панель апплеттерін баптау
panel-missing = Панель конфигурациясы табылмады
    .desc = Арнайы баптауды қолдану немесе бүліну салдарынан панельдің баптау файлы табылмады.
    .fix = Әдепкі күйге қайтару
dock = Док
    .desc = Қолданбалар мен апплеттерге арналған қосымша жолақ
window-management = Терезелерді басқару
    .desc = Super пернесінің әрекеті, терезені басқару опциялары және терезелерді қатарластырудың қосымша опциялары
super-key = Super пернесінің әрекеті
    .launcher = Іске қосқышты ашу
    .workspaces = Жұмыс орындарын ашу
    .applications = Қолданбаларды ашу
    .disable = Сөндіру
edge-gravity = Қалқымалы терезелер жақын маңдағы шеттерге тартылады
window-controls = Терезе басқару элементтері
    .maximize = Жазық қылу батырмасын көрсету
    .minimize = Қайыру батырмасын көрсету
    .active-window-hint = Белсенді терезе нұсқауын көрсету
focus-navigation = Фокус навигациясы
    .focus-follows-cursor = Фокус курсорға ілеседі
    .focus-follows-cursor-delay = Фокустың курсорға ілесу кідірісі (мс)
    .cursor-follows-focus = Курсор фокусқа ілеседі
workspaces = Жұмыс орындары
    .desc = Жұмыс орындарының бағыты мен мінез-құлқы
workspaces-behavior = Жұмыс орындарының мінез-құлқы
    .dynamic = Динамикалық жұмыс орындары
    .dynamic-desc = Бос жұмыс орындарын автоматты түрде өшіреді.
    .fixed = Жұмыс орындарының бекітілген саны
    .fixed-desc = Шолу кезінде жұмыс орындарын қосу немесе өшіру.
workspaces-multi-behavior = Көпмониторлы режимнің мінез-құлқы
    .span = Жұмыс орындары дисплейлерге жайылады
    .separate = Дисплейлерде жеке жұмыс орындары болады
workspaces-overview-thumbnails = Жұмыс орындарына шолу нобайлары
    .show-number = Жұмыс орнының нөмірін көрсету
    .show-name = Жұмыс орнының атын көрсету
workspaces-orientation = Жұмыс орындарының бағыты
    .vertical = Вертикалды
    .horizontal = Горизонталды
hot-corner = Белсенді бұрыш
    .top-left-corner = Жұмыс орындары үшін жоғарғы сол жақ белсенді бұрышты іске қосу
-requires-restart = Қайта іске қосуды талап етеді
color = Түс
    .depth = Түс тереңдігі
    .profile = Түс профилі
    .sidebar = Түс профильдері
    .temperature = Түс температурасы
display = Дисплейлер
    .desc = Дисплейлерді және түнгі режимді басқару
    .arrangement = Дисплейлердің орналасуы
    .arrangement-desc = Дисплейлердің орнын ауыстыру үшін оларды сүйреңіз
    .enable = Дисплейді іске қосу
    .external = { $size } { $output } сыртқы дисплейі
    .laptop = { $size } ноутбук дисплейі
    .options = Дисплей опциялары
    .refresh-rate = Жаңарту жиілігі
    .resolution = Ажыратымдылығы
    .scale = Масштаб
    .additional-scale-options = Масштабтың қосымша опциялары
mirroring = Айналау
    .id = Айналау { $id }
    .dont = Айналамау
    .mirror = { $display } айналау
    .project =
        { $display ->
            [all] барлық дисплейлерге
           *[other] { $display } дисплейіне
        } көрсету
    .project-count =
        Басқа { $count } { $count ->
            [1] дисплейге
           *[other] дисплейлерге
        } көрсетуде
night-light = Түнгі жарық
    .auto = Автоматты (күн батқаннан күн шыққанға дейін)
    .desc = Жылырақ түстер арқылы көк жарықты азайту
orientation = Бағыты
    .standard = Стандартты
    .rotate-90 = 90° бұру
    .rotate-180 = 180° бұру
    .rotate-270 = 270° бұру
vrr = Өзгермелі жаңарту жиілігі
    .enabled = Іске қосылған
    .force = Әрқашан
    .auto = Автоматты
    .disabled = Сөндірілген
scheduling = Кесте
    .manual = Қолмен орнатылған кесте
dialog = Сұхбат терезесі
    .title = Дисплейдің осы баптауларын сақтау керек пе?
    .keep-changes = Өзгерістерді сақтау
    .change-prompt = Баптаулардағы өзгерістер { $time } секундтан кейін автоматты түрде кері қайтарылады.
    .revert-settings = Баптауларды кері қайтару
sound = Дыбыс
    .desc = Қолжетімсіз
sound-output = Шығыс
    .volume = Шығыс дыбыс деңгейі
    .device = Шығыс құрылғысы
    .level = Шығыс деңгейі
    .config = Конфигурация
    .balance = Баланс
    .left = Сол жақ
    .right = Оң жақ
sound-input = Кіріс
    .volume = Кіріс дыбыс деңгейі
    .device = Кіріс құрылғысы
    .level = Кіріс деңгейі
amplification = Күшейту
    .desc = Дыбыс деңгейін 150%-ға дейін көтеруге мүмкіндік береді
sound-alerts = Ескертулер
    .volume = Ескертулердің дыбыс деңгейі
    .sound = Ескертулер дыбысы
sound-applications = Қолданбалар
    .desc = Қолданбалардың дыбыс деңгейі мен баптаулары
sound-device-port-unplugged = Ажыратылған
sound-hd-audio = HD аудио
sound-usb-audio = USB аудио
sound-device-profiles = Құрылғы профильдері
power = Қуат және батарея
    .desc = Қуат баптауларын басқару
battery = Батарея
    .minute =
        { $value } { $value ->
            [one] минут
           *[other] минут
        }
    .hour =
        { $value } { $value ->
            [one] сағат
           *[other] сағат
        }
    .day =
        { $value } { $value ->
            [one] күн
           *[other] күн
        }
    .less-than-minute = Бір минуттан аз
    .and = және
    .remaining-time =
        { $time } { $action ->
            [full] толық толғанға
           *[other] таусылғанға
        } дейін
connected-devices = Қосылған құрылғылар
    .unknown = Белгісіз құрылғы
power-mode = Қуат режимі
    .battery = Батареяның жұмыс уақытын ұзарту
    .battery-desc = Қуат тұтынуды азайту және дыбыссыз жұмыс істеу
    .balanced = Теңгерілген
    .balanced-desc = Тыныш жұмыс және қалыпты қуат тұтыну
    .performance = Жоғары өнімділік
    .performance-desc = Ең жоғары өнімділік пен қуат тұтыну
    .no-backend = Бэкенд табылмады. system76-power немесе power-profiles-daemon бағдарламасын орнатыңыз.
power-saving = Қуатты үнемдеу опциялары
    .turn-off-screen-after = Экранды келесі уақыттан кейін сөндіру
    .auto-suspend = Автоматты түрде күту режиміне өту
    .auto-suspend-ac = Желіге қосулы кезде автоматты түрде күту режиміне өту
    .auto-suspend-battery = Батареядан жұмыс істегенде автоматты түрде күту режиміне өту
acceleration-desc = Жылдамдыққа байланысты бақылау сезімталдығын автоматты түрде реттейді
disable-while-typing = Теру кезінде сөндіру
input-devices = Енгізу құрылғылары
    .desc = Енгізу құрылғылары
primary-button = Негізгі батырма
    .desc = Физикалық батырмалардың ретін орнатады
    .left = Сол жақ
    .right = Оң жақ
scrolling = Айналдыру
    .two-finger = Екі саусақпен айналдыру
    .edge = Бір саусақпен жиек бойымен айналдыру
    .speed = Айналдыру жылдамдығы
    .natural = Табиғи айналдыру
    .natural-desc = Көріністің орнына мазмұнды айналдыру
slow = Баяу
fast = Жылдам
short = Қысқа
long = Ұзақ
keyboard = Пернетақта
    .desc = Енгізу көздері, ауысу, арнайы таңбаларды енгізу, жарлықтар
keyboard-sources = Енгізу көздері
    .desc = Енгізу көздерін Super+Space пернелер жарлығы арқылы ауыстыруға болады. Бұл баптауды пернетақта жарлықтарының баптауларында өзгертуге болады.
    .move-up = Жоғары жылжыту
    .move-down = Төмен жылжыту
    .settings = Баптаулар
    .view-layout = Пернетақта жаймасын қарау
    .remove = Өшіру
    .add = Енгізу көзін қосу
keyboard-special-char = Арнайы таңбаларды енгізу
    .alternate = Балама таңбалар пернесі
    .compose = Compose пернесі
    .compose-desc = Compose пернесі таңбалардың алуан түрін енгізуге мүмкіндік береді. Оны пайдалану үшін, compose пернесін, содан кейін таңбалар тізбегін басыңыз. Мысалы, compose пернесінен кейін C және o басылса, © таңбасы енгізіледі, ал compose пернесінен кейін a және ‘ басылса, á таңбасы енгізіледі.
    .caps = Caps Lock пернесі
keyboard-typing-assist = Теру
    .repeat-rate = Қайталау жиілігі
    .repeat-delay = Қайталау кідірісі
keyboard-numlock-boot = Numlock
    .boot-state = Жүктелген кездегі күйі
    .last-boot = Соңғы жүктелу
    .on = Қосулы
    .off = Сөндірулі
    .set = Numlock-тың жүктелу кезіндегі күйін орнату
added = Қосылды
type-to-search = Іздеу үшін теріңіз...
show-extended-input-sources = Кеңейтілген енгізу көздерін көрсету
keyboard-shortcuts = Пернетақта жарлықтары
    .desc = Жарлықтарды көру және теңшеу
add-another-keybinding = Басқа пернелер жарлығын қосу
cancel = Бас тарту
command = Команда
custom = Таңдауыңызша
debug = Жөндеу
disabled = Сөндірулі
input-source-switch = Пернетақта тілін енгізу көзін ауыстыру
migrate-workspace-prev = Жұмыс орнын алдыңғы шығысқа көшіру
migrate-workspace-next = Жұмыс орнын келесі шығысқа көшіру
migrate-workspace =
    Жұмыс орнын шығысқа көшіру: { $direction ->
       *[down] төмен
        [left] солға
        [right] оңға
        [up] жоғары
    }
navigate = Навигация
replace = Алмастыру
shortcut-name = Жарлық атауы
system-controls = Жүйені басқару элементтері
terminate = Аяқтау
toggle-stacking = Терезелерді жинақтауды ауыстыру
type-key-combination = Пернелер жарлығын теріңіз
custom-shortcuts = Арнайы жарлықтар
    .add = Жарлық қосу
    .context = Арнайы жарлық қосу
    .none = Арнайы жарлықтар жоқ
modified = { $count } өзгертілді
nav-shortcuts = Навигация
    .prev-output = Алдыңғы шығысқа фокустау
    .next-output = Келесі шығысқа фокустау
    .last-workspace = Соңғы жұмыс орнына фокустау
    .prev-workspace = Алдыңғы жұмыс орнына фокустау
    .next-workspace = Келесі жұмыс орнына фокустау
    .focus =
        Терезеге фокустау: { $direction ->
           *[down] төменгі
            [in] ішкі
            [left] сол жақтағы
            [out] сыртқы
            [right] оң жақтағы
            [up] жоғарыдағы
        }
    .output =
        Шығысқа ауысу: { $direction ->
           *[down] төменгі
            [left] сол жақтағы
            [right] оң жақтағы
            [up] жоғарғы
        }
    .workspace = Жұмыс орнына ауысу: { $num }
manage-windows = Терезелерді басқару
    .close = Терезені жабу
    .maximize = Терезені жаймалау
    .fullscreen = Терезені толық экранға шығару
    .minimize = Терезені жию
    .resize-inwards = Терезе өлшемін ішке қарай өзгерту
    .resize-outwards = Терезе өлшемін сыртқа қарай өзгерту
    .toggle-sticky = Жабысқақ терезе режимін ауыстыру
move-windows = Терезелерді жылжыту
    .direction =
        Терезені { $direction ->
           *[down] төмен
            [left] солға
            [right] оңға
            [up] жоғары
        } жылжыту
    .display =
        Терезені бір мониторға { $direction ->
           *[down] төмен
            [left] солға
            [right] оңға
            [up] жоғары
        } жылжыту
    .workspace =
        Терезені бір жұмыс орнына { $direction ->
           *[below] төмен
            [left] солға
            [right] оңға
            [above] жоғары
        } жылжыту
    .workspace-num = Терезені { $num } жұмыс орнына жылжыту
    .prev-workspace = Терезені алдыңғы жұмыс орнына жылжыту
    .next-workspace = Терезені келесі жұмыс орнына жылжыту
    .last-workspace = Терезені соңғы жұмыс орнына жылжыту
    .next-display = Терезені келесі экранға жылжыту
    .prev-display = Терезені алдыңғы экранға жылжыту
    .send-to-prev-workspace = Терезені алдыңғы жұмыс орнына жіберу
    .send-to-next-workspace = Терезені келесі жұмыс орнына жіберу
system-shortcut = Жүйе
    .app-library = Қолданбалар кітапханасын ашу
    .brightness-down = Экран жарықтығын азайту
    .brightness-up = Экран жарықтығын арттыру
    .display-toggle = Ішкі экранды ауыстырып қосу
    .home-folder = Үй бумасын ашу
    .keyboard-brightness-down = Пернетақта жарықтығын азайту
    .keyboard-brightness-up = Пернетақта жарықтығын арттыру
    .launcher = Launcher-ді ашу
    .log-out = Шығу
    .lock-screen = Экранды бұғаттау
    .mute = Дыбысты өшіру
    .mute-mic = Микрофон дыбысын өшіру
    .play-pause = Ойнату/кідірту
    .play-next = Келесі трек
    .play-prev = Алдыңғы трек
    .poweroff = Қуатты өшіру
    .screenshot = Скриншот түсіру
    .suspend = Күту режимі
    .terminal = Терминалды ашу
    .touchpad-toggle = Тачпадты ауыстырып қосу
    .volume-lower = Дыбыс деңгейін азайту
    .volume-raise = Дыбыс деңгейін арттыру
    .web-browser = Веб-браузерді ашу
    .window-switcher = Ашық терезелер арасында ауысу
    .window-switcher-previous = Ашық терезелер арасында кері ретпен ауысу
    .workspace-overview = Жұмыс орындары шолуын ашу
window-tiling = Терезелерді қатар орналастыру
    .horizontal = Горизонталды бағытты орнату
    .vertical = Вертикалды бағытты орнату
    .swap-window = Терезелердің орнын ауыстыру
    .toggle-tiling = Терезелерді қатар орналастыруды ауыстырып қосу
    .toggle-stacking = Терезелерді жинақтауды ауыстырып қосу
    .toggle-floating = Терезелердің еркін қозғалуын ауыстырып қосу
    .toggle-orientation = Бағытты ауыстырып қосу
replace-shortcut-dialog = Жарлықты алмастыру керек пе?
    .desc = { $shortcut } жарлығын { $name } қолдануда. Оны алмастырсаңыз, { $name } сөндіріледі.
zoom-in = Үлкейту
zoom-out = Кішірейту
mouse = Тышқан
    .desc = Тышқан жылдамдығы, үдеуі, табиғи айналдыру
    .speed = Тышқан жылдамдығы
    .acceleration = Тышқан үдеуін іске қосу
click-behavior = Шерту әрекеті
    .click-finger = Екі саусақпен қосымша шерту және үш саусақпен ортаңғы шерту
    .button-areas = Төменгі оң жақ бұрышта қосымша шерту және төменгі ортада ортаңғы шерту
pinch-to-zoom = Үлкейту үшін жақындату
    .desc = Мазмұнды үлкейту үшін екі саусақты қолданыңыз (үлкейтуді қолдайтын қолданбалар үшін)
tap-to-click = Шерту үшін тигізу
    .desc = Негізгі шерту үшін бір саусақпен, қосымша шерту үшін екі саусақпен және ортаңғы шерту үшін үш саусақпен тигізуді іске қосады
touchpad = Тачпад
    .acceleration = Тачпад үдеуін іске қосу
    .desc = Тачпад жылдамдығы, шерту опциялары, қимылдар
    .speed = Тачпад жылдамдығы
gestures = Қимылдар
    .four-finger-down = Төрт саусақпен төмен қарай сырғыту
    .four-finger-left = Төрт саусақпен солға қарай сырғыту
    .four-finger-right = Төрт саусақпен оңға қарай сырғыту
    .four-finger-up = Төрт саусақпен жоғары қарай сырғыту
    .three-finger-any = Үш саусақпен кез келген бағытқа сырғыту
switch-workspaces = Жұмыс орындарын ауыстыру
    .horizontal = Төрт саусақпен солға/оңға сырғыту
    .vertical = Төрт саусақпен жоғары/төмен сырғыту
switch-between-windows = Терезелер арасында ауысу
open-application-library = Қолданбалар кітапханасын ашу
open-workspaces-view = Жұмыс орындары шолуын ашу
time = Уақыт және тіл
    .desc = Мәлімет жоқ
time-date = Күн және уақыт
    .desc = Уақыт белдеуі, сағатты автоматты баптау және уақыт пішімі
    .auto = Автоматты түрде орнату
    .auto-ntp = Уақыт белдеуі орнатылған кезде күн мен уақыт автоматты түрде жаңартылады
time-zone = Уақыт белдеуі
    .auto = Автоматты уақыт белдеуі
    .auto-info = Орналасқан жер қызметтері мен интернетке қол жеткізуді қажет етеді
time-format = Күн және уақыт пішімі
    .twenty-four = 24 сағаттық уақыт
    .show-seconds = Секундтарды көрсету
    .first = Аптаның бірінші күні
    .show-date = Уақыт апплетінде күнді көрсету
    .friday = Жұма
    .saturday = Сенбі
    .sunday = Жексенбі
    .monday = Дүйсенбі
time-region = Аймақ және тіл
    .desc = Аймағыңызға байланысты күндерді, уақытты және сандарды пішімдеу
formatting = Пішімдеу
    .dates = Күндер
    .time = Уақыт
    .date-and-time = Күн және уақыт
    .numbers = Сандар
    .measurement = Өлшем бірліктері
    .paper = Қағаз
preferred-languages = Таңдаулы тілдер
    .desc = Тілдердің реті пайдаланушы интерфейсі үшін қай тіл қолданылатынын анықтайды. Өзгерістер келесі рет жүйеге кіргенде іске асады.
add-language = Тілді қосу
    .context = Тілді қосу
install-additional-languages = Қосымша тілдерді орнату
region = Аймақ
applications = Қолданбалар
default-apps = Әдепкі қолданбалар
    .desc = Әдепкі веб-браузер, пошта клиенті, файлдық браузер және басқа қолданбалар
    .web-browser = Веб-браузер
    .file-manager = Файлдар басқарушысы
    .mail-client = Пошта клиенті
    .music = Музыка
    .video = Видео
    .photos = Фотосуреттер
    .calendar = Күнтізбе
    .terminal = Терминал
    .other-associations = Басқа сәйкестіктер
    .text-editor = Мәтіндік түзеткіш
    .not-installed = Орнатылмаған
startup-apps = Автожегізілетін қолданбалар
    .desc = Жүйеге кіргенде іске қосылатын қолданбаларды баптау
    .add = Қолданбаны қосу
    .user = Жүйеге кіргенде іске қосылатын қолданбалар
    .none = Автоқосылатын қолданбалар қосылмаған
    .remove-dialog-title = { $name } өшіру керек пе?
    .remove-dialog-description = Бұл автоқосылатын қолданбаны өшіру керек пе?
    .add-startup-app = Автоқосылатын қолданбаны қосу
legacy-applications = X11 қолданбаларының үйлесімділігі
    .desc = X11 терезелер жүйесі қолданбаларының масштабталуы және глобалды жарлықтар
legacy-app-global-shortcuts = X11 қолданбаларындағы глобалды жарлықтар
    .desc = Глобалды жарлықтар қолданбаларда орындалатын перне басу және тышқан батырмасы оқиғаларын «сөйлеу үшін басу» немесе «дыбысты өшіру үшін басу» сияқты функциялар үшін басқа қолданбалардың тануына мүмкіндік береді. Әдепкі бойынша, басқа қолданбалар құпия ақпараты бар пернетақта мен тышқан оқиғаларын бақылай алмауы үшін X11 қолданбаларында глобалды жарлықтар сөндірілген.
    .none = Пернелер жоқ
    .modifiers = Түрлендіргіштер (Super, Shift, Control, Alt)
    .combination = Super, Control немесе Alt түрлендіргіштері басылып тұрғандағы барлық пернелер
    .all = Барлық пернелер
    .mouse = X11 қолданбаларындағы тышқан батырмасының оқиғалары
legacy-app-scaling = X11 терезелер жүйесі қолданбаларының масштабталуы
    .scaled-gaming = Ойындар мен толық экранды қолданбалар үшін оңтайландыру
    .gaming-description = X11 қолданбалары Wayland қолданбаларымен салыстырғанда сәл үлкенірек/кішірек көрінуі мүмкін
    .scaled-applications = Қолданбалар үшін оңтайландыру
    .applications-description = Ойындар мен толық экранды X11 қолданбалары экран ажыратымдылығына сәйкес келмеуі мүмкін
    .scaled-compatibility = Максималды үйлесімділік режимі
    .compatibility-description = X11 қолданбалары HiDPI экрандарында бұлдыр болып көрінуі мүмкін
    .preferred-display = Ойындар мен толық экранды X11 қолданбалары үшін таңдаулы экран
    .no-display = Жоқ
system = Жүйе және тіркелгілер
about = Осы туралы
    .desc = Құрылғы атауы, аппараттық қамтама туралы ақпарат, операциялық жүйенің әдепкі параметрлері
about-device = Құрылғы атауы
    .desc = Бұл атау басқа желілік немесе Bluetooth құрылғыларына көрінеді
about-hardware = Аппараттық қамтама
    .model = Аппараттық модель
    .memory = Жад
    .processor = Процессор
    .graphics = Графика
    .disk-capacity = Диск сыйымдылығы
about-os = Операциялық жүйе
    .os = Операциялық жүйе
    .os-architecture = Операциялық жүйе архитектурасы
    .kernel = Ядро нұсқасы
    .desktop-environment = Жұмыс үстелі ортасы
    .windowing-system = Терезелер жүйесі
about-related = Қатысты параметрлер
    .support = Қолдау алу
firmware = Микробағдарлама
    .desc = Микробағдарлама мәліметтері
users = Пайдаланушылар
    .desc = Аутентификация және пайдаланушы тіркелгілері
    .admin = Әкімші
    .standard = Стандартты
    .profile-add = Профиль суретін таңдау
administrator = Әкімші
    .desc = Әкімшілер барлық пайдаланушылар үшін параметрлерді өзгерте алады, басқа пайдаланушыларды қоса алады және өшіре алады
add-user = Пайдаланушыны қосу
change-password = Парольді өзгерту
remove-user = Пайдаланушыны өшіру
full-name = Толық аты-жөні
invalid-username = Пайдаланушы аты жарамсыз
password-mismatch = Пароль мен оны растау сәйкес келуі керек
save = Сақтау
