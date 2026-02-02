app = Налаштування COSMIC
unknown = Невідомо
number = { $number }

## Networking: Wired

wired = Дротове
    .adapter = Дротовий адаптер { $id }
    .connections = Дротові з'єднання
    .devices = Дротові пристрої
    .remove = Видалити профіль з'єднання

## Networking: Online Accounts

online-accounts = Онлайн-облікові записи
    .desc = Додати облікові записи, IMAP і SMTP, облікові записи організацій

## Desktop

desktop = Стільниця

## Desktop: Wallpaper

wallpaper = Зображення тла
    .change = Змінювати зображення що
    .desc = Налаштування для зображення тла, кольорів та показу слайдів.
    .fit = Припасувати зображення тла
    .folder-dialog = Виберіть теку із зображеннями
    .image-dialog = Виберіть зображення
    .plural = Зображення
    .same = Однакове зображення на всіх дисплеях
    .slide = Показ слайдів
add-color = Додати колір
add-image = Додати зображення
all-displays = Всі дисплеї
colors = Кольори
dialog-add = Додати
fill = Заповнити
fit-to-screen = Припасувати до екрана
open-new-folder = Відкрити нову теку
recent-folders = Нещодавні теки
x-minutes =
    { $number } { $number ->
        [one] хвилину
        [few] хвилини
       *[other] хвилин
    }
x-hours =
    { $number } { $number ->
        [one] годину
        [few] години
       *[other] годин
    }

## Desktop: Appearance

appearance = Зовнішній вигляд
    .desc = Акцентні кольори та оформлення
accent-color = Колір акценту
app-background = Тло вікон
auto = Автоматичне
close = Закрити
color-picker = Вибір кольору
copied-to-clipboard = Скопійовано до буфера обміну
copy-to-clipboard = Копіювати до буфера обміну
dark = Темна
export = Експорт
hex = Hex
import = Імпортувати
light = Світла
mode-and-colors = Режим та кольори
recent-colors = Останні кольори
reset-to-default = Типові значення
rgb = RGB
window-hint-accent = Колір підказки щодо активного вікна
window-hint-accent-toggle = Використовувати колір акценту з теми для підказки щодо активного вікна
auto-switch = Автоматичне перемикання між світлим і темним режимами
    .sunrise = Перемикатися на світлий режим на світанку
    .sunset = Перемикатися на темний режим на смерканні
    .next-sunrise = Перемикатися на світлий режим на наступному світанку
    .next-sunset = Перемикатися на темний режим на наступному смерканні
container-background = Тло контейнера
    .desc-detail = Колір тла контейнера використовується для навігаційної бічної панелі, бокових панелей, діалогових вікон та подібних елементів. Усталено він автоматично визначається на основі тла вікна.
    .reset = Скинути до автоматичного
    .desc = Використовується для навігаційної бічної панелі, бокових панелей, діалогових вікон та подібних елементів
control-tint = Відтінок елементів керування
    .desc = Використовується для тла стандартних кнопок, пошукових полів, текстових полів та подібних компонентів
frosted = → Ефект матового скла для системного інтерфейсу
    .desc = Застосовує розмиття тла до панелі, дока, аплетів, запускача та бібліотеки застосунків
experimental-settings = Експериментальні налаштування
enable-export = Застосувати цю тему до застосунків GNOME
    .desc = Не всі інструментарії підтримують автоперемикання. Після зміни теми деякі застосунки може знадобитися перезапустити.
icon-theme = Тема значків
    .desc = Застосовує інший набір значків для застосунків
text-tint = Відтінок тексту інтерфейсу
    .desc = Колір, з якого формуються кольори тексту інтерфейсу з достатнім контрастом на різних поверхнях
style = Стиль
    .round = Заокруглений
    .slightly-round = Трохи заокруглений
    .square = Квадратний
# interface density left out for now
window-management-appearance = Керування вікнами
    .active-hint = Розмір підказки активного вікна
    .gaps = Відступи навколо укладених вікон

## Desktop: Notifications

notifications = Сповіщення
    .desc = Не турбувати, сповіщення на екрані блокування, налаштування програм

## Desktop: Panel

panel = Панель
    .desc = Головна панель з елементами керування та аплетами
add = Додати
add-applet = Додати аплет
all = На всіх
applets = Аплети
center-segment = Центральний сегмент
end-segment = Кінцевий сегмент
large = Великий
no-applets-found = Аплетів не знайдено...
panel-bottom = Внизу
panel-left = Ліворуч
panel-right = Праворуч
panel-top = Вгорі
search-applets = Шукати аплети...
small = Малий
start-segment = Початковий сегмент
panel-appearance = Зовнішній вигляд
    .match = Системний
    .light = Світлий
    .dark = Темний
panel-behavior-and-position = Поведінка та положення
    .autohide = Автоматично приховувати панель
    .dock-autohide = Автоматично приховувати док
    .position = Положення на екрані
    .display = Показувати на дисплеї
panel-style = Стиль
    .anchor-gap = Відступ між панеллю та краями екрана
    .dock-anchor-gap = Відступ між доком та краями екрана
    .extend = Розширити панель до країв екрана
    .dock-extend = Розширити док до країв екрана
    .appearance = Зовнішній вигляд
    .size = Розмір
    .background-opacity = Непрозорість тла
panel-applets = Налаштування
    .dock-desc = Налаштуйте аплети в доці
    .desc = Налаштуйте аплети на панелі
panel-missing = Не знайдено налаштувань панелі
    .desc = Не знайдено файлу конфігурації панелі через використання власних налаштувань або через його пошкодження.
    .fix = Типові значення

## Desktop: Dock

dock = Док
    .desc = Панель з пришпиленими застосунками в лотку застосунків та іншими аплетами.

## Desktop: Window management

window-management = Керування вікнами
    .desc = Дія клавіші Super, параметри керування вікнами та додаткові налаштування укладання вікон
super-key = Клавіша Super
    .launcher = Відкриває запускач
    .workspaces = Відкриває меню робочих просторів
    .applications = Відкриває меню застосунків
    .disable = Вимкнути
window-controls = Керування вікнами
    .minimize = Показувати кнопку згортання
    .maximize = Показувати кнопку розгортання
    .active-window-hint = Підсвічувати активне вікно

## Desktop: Workspaces

workspaces = Робочі простори
    .desc = Орієнтація та поведінка робочих просторів
workspaces-behavior = Поведінка робочих просторів
    .dynamic = Динамічні робочі простори
    .dynamic-desc = Порожні робочі простори автоматично вилучаються
    .fixed = Фіксована кількість робочих просторів
    .fixed-desc = Додавайте або вилучайте робочі простори в режимі огляду.
workspaces-multi-behavior = Поведінка за наявності декількох моніторів
    .span = Робочий простір охоплює кілька дисплеїв
    .separate = Окремий робочий простір для кожного дисплея
workspaces-overview-thumbnails = Мініатюри в огляді робочих просторів
    .show-number = Показувати номер робочого простору
    .show-name = Показувати назву робочого простору
workspaces-orientation = Орієнтація робочих просторів
    .vertical = Вертикальна
    .horizontal = Горизонтальна
hot-corner = Гарячий кут
    .top-left-corner = Використати гарячий кут вгорі ліворуч для робочих просторів

## Desktop: Display

-requires-restart = Потрібен перезапуск
color = Колір
    .depth = Глибина кольору
    .profile = Профіль кольору
    .sidebar = Профілі кольорів
    .temperature = Температура кольору
display = Дисплеї
    .desc = Керування дисплеями та нічним режимом
    .arrangement = Розташування дисплеїв
    .arrangement-desc = Перетягніть дисплеї, щоб змінити їх порядок
    .enable = Увімкнути дисплей
    .external = { $size } { $output } Зовнішній дисплей
    .laptop = { $size } Дисплей ноутбука
    .options = Параметри дисплея
    .refresh-rate = Частота оновлення
    .resolution = Роздільна здатність
    .scale = Масштаб
    .additional-scale-options = Додаткові параметри масштабу
mirroring = Віддзеркалення
    .id = Віддзеркалення { $id }
    .dont = Не віддзеркалювати
    .mirror = Віддзеркалювати { $display }
    .project =
        Проектувати на { $display ->
            [all] всі дисплеї
           *[other] { $display }
        }
    .project-count =
        Проектування на { $count } { $count ->
            [one] дисплей
            [few] дисплеї
           *[other] дисплеїв
        }
night-light = Нічне світло
    .auto = Автоматично (від смеркання до світання)
    .desc = Приглушити синє світло теплими кольорами
orientation = Орієнтація
    .standard = Стандартна
    .rotate-90 = Обернути на 90°
    .rotate-180 = Обернути на 180°
    .rotate-270 = Обернути на 270°
scheduling = Планування
    .manual = Планування вручну
dialog = Діалог
    .title = Залишити ці налаштування дисплея?
    .keep-changes = Зберегти
    .change-prompt =
        Повернення до попередніх налаштувань буде виконано за { $time } { $time ->
            [one] секунду
            [few] секунди
           *[other] секунд
        }.
    .revert-settings = Повернути

## Sound

sound = Звук
    .desc = Н/Д
sound-output = Виведення
    .volume = Гучність
    .device = Пристрій виведення
    .level = Рівень
    .config = Конфігурація
    .balance = Баланс
    .left = Ліво
    .right = Право
sound-input = Введення
    .volume = Гучність запису
    .device = Пристрій введення
    .level = Рівень введення
sound-alerts = Попередження
    .volume = Гучність попереджень
    .sound = Звук попереджень
sound-applications = Застосунки
    .desc = Гучність та налаштування для застосунків

## Power

power = Живлення та акумулятор
    .desc = Керування енергоспоживанням
power-mode = Режим живлення
    .battery = Подовжений час роботи акумулятора
    .battery-desc = Зменшене енергоспоживання та тиха робота
    .balanced = Збалансований
    .balanced-desc = Тиха робота та помірне енергоспоживання
    .performance = Висока швидкодія
    .performance-desc = Максимальна швидкодія та енергоспоживання
    .no-backend = Бекенд не знайдено. Встановіть system76-power або power-profiles-daemon.

## Input

acceleration-desc = Автоматично регулює чутливість відстеження руху залежно від швидкості
disable-while-typing = Вимикати під час введення тексту
input-devices = Пристрої введення
    .desc = Пристрої введення
primary-button = Основна кнопка
    .desc = Задає порядок фізичних кнопок.
    .left = Ліва
    .right = Права
scrolling = Гортання
    .two-finger = Гортання двома пальцями
    .edge = Гортання одним пальцем вздовж краю
    .speed = Швидкість гортання
    .natural = Природне гортання
    .natural-desc = Переміщення вмісту замість переміщення перегляду

## Input: Keyboard

slow = Повільна
fast = Швидка
short = Коротка
long = Довга
keyboard = Клавіатура
    .desc = Джерела введення, перемикання, введення спеціальних символів, скорочення
keyboard-sources = Джерела введення
    .desc = Джерела введення можна перемикати за допомогою сполучення клавіш Super+Space. Це можна налаштувати в параметрах клавіатурних скорочень.
    .move-up = Перемістити вгору
    .move-down = Перемістити вниз
    .settings = Налаштування
    .view-layout = Показати розкладку клавіатури
    .remove = Вилучити
    .add = Додати джерело введення
keyboard-special-char = Введення спеціальних символів
    .alternate = Клавіша альтернативних символів
    .compose = Клавіша компонування
    .caps = Клавіша Caps Lock
keyboard-typing-assist = Введення
    .repeat-rate = Частота повторів
    .repeat-delay = Затримка повторів
added = Додана
type-to-search = Введіть текст для пошуку...

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Сполучення клавіш
    .desc = Перегляд і налаштування сполучень
cancel = Скасувати
command = Команда
custom = Власні
debug = Зневадження
disabled = Вимкнено
migrate-workspace-prev = Перемістити робочий простір до попереднього джерела виведення
migrate-workspace-next = Перемістити робочий простір до наступного джерела виведення
migrate-workspace =
    Перемістити робочий простір до джерела виведення { $direction ->
       *[down] вниз
        [left] ліворуч
        [right] праворуч
        [up] вгору
    }
navigate = Перейти
replace = Замінити
shortcut-name = Назва скорочення
system-controls = Керування системою
terminate = Зупинити
toggle-stacking = Перемкнути групування вікон
type-key-combination = Введіть комбінацію клавіш
custom-shortcuts = Власні сполучення
    .add = Додати сполучення
    .context = Додати користувацькі сполучення
    .none = Немає користувацьких сполучень
modified = { $count } змінено
nav-shortcuts = Навігація
    .prev-output = Фокус на попереднє джерело виведення
    .next-output = Фокус на наступне джерело виведення
    .last-workspace = Фокус на останній робочий простір
    .prev-workspace = Фокус на попередній робочий простір
    .next-workspace = Фокус на наступний робочий простір
    .focus =
        Фокус на вікно { $direction ->
           *[down] нижче
            [in] всередині
            [left] ліворуч
            [out] зовні
            [right] праворуч
            [up] вище
        }
    .output =
        Перейти до джерела виведення { $direction ->
           *[down] нижче
            [left] ліворуч
            [right] праворуч
            [up] вище
        }
    .workspace = Перейти до робочого простору { $num }
manage-windows = Керування вікнами
    .close = Закрити вікно
    .maximize = Розгорнути вікно
    .fullscreen = Розгорнути вікно на повний екран
    .minimize = Згорнути вікно
    .resize-inwards = Зміна розмірів вікна всередину
    .resize-outwards = Зміна розмірів вікна назовні
    .toggle-sticky = Перемкнути прилипання вікна
move-windows = Переміщення вікон
    .direction =
        Перемістити вікно { $direction ->
           *[down] нижче
            [left] ліворуч
            [right] праворуч
            [up] вище
        }
    .display =
        Перемістити вікно на монітор { $direction ->
           *[down] нижче
            [left] ліворуч
            [right] праворуч
            [up] вище
        }
    .workspace =
        Перемістити вікно на робочий простір { $direction ->
           *[below] нижче
            [left] ліворуч
            [right] праворуч
            [above] вище
        }
    .workspace-num = Перемістити вікно на робочий простір { $num }
    .prev-workspace = Перемістити вікно на попередній робочий простір
    .next-workspace = Перемістити вікно на наступний робочий простір
    .last-workspace = Перемістити вікно на останній робочий простір
    .next-display = Перемістити вікно на наступний дисплей
    .prev-display = Перемістити вікно на попередній дисплей
    .send-to-prev-workspace = Перемістити вікно на попередній робочий простір
    .send-to-next-workspace = Перемістити вікно на наступний робочий простір
system-shortcut = Система
    .app-library = Відкрити бібліотеку застосунків
    .brightness-down = Зменшити яскравість дисплея
    .brightness-up = Збільшити яскравість дисплея
    .display-toggle = Перемкнути вбудований дисплей
    .home-folder = Відкрити домашню теку
    .keyboard-brightness-down = Зменшити яскравість клавіатури
    .keyboard-brightness-up = Збільшити яскравість клавіатури
    .launcher = Відкрити запускач
    .log-out = Вийти
    .lock-screen = Заблокувати екран
    .mute = Вимкнути виведення звуку
    .mute-mic = Вимкнути мікрофон
    .play-pause = Відтворення/Пауза
    .play-next = Наступна доріжка
    .play-prev = Попередня доріжка
    .poweroff = Вимкнути
    .screenshot = Зробити знімок екрана
    .terminal = Відкрити термінал
    .touchpad-toggle = Перемкнути тачпад
    .volume-lower = Зменшити гучність виведення звуку
    .volume-raise = Збільшити гучність виведення звуку
    .web-browser = Відкрити браузер
    .window-switcher = Перемикати відкриті вікна
    .window-switcher-previous = Перемикати відкриті вікна у зворотньому порядку
    .workspace-overview = Відкрити огляд робочих просторів
window-tiling = Укладання вікон
    .horizontal = Встановити горизонтальну орієнтацію
    .vertical = Встановити вертикальну орієнтацію
    .swap-window = Поміняти місцями вікна
    .toggle-tiling = Перемкнути укладання вікон
    .toggle-stacking = Перемкнути групування вікон
    .toggle-floating = Перемкнути плавучість вікон
    .toggle-orientation = Перемкнути орієнтацію
replace-shortcut-dialog = Замінити скорочення?
    .desc = { $shortcut } вже використовується для { $name }. Якщо замінити його, { $name } буде вимкнено.

## Input: Mouse

mouse = Миша
    .desc = Швидкість миші, прискорення, природне гортання
    .speed = Швидкість миші
    .acceleration = Увімкнути прискорення миші

## Input: Touchpad

click-behavior = Поведінка при натисканні
    .click-finger = Другорядне клацання активується двома пальцями, а клацання середньою кнопкою — трьома пальцями
    .button-areas = Другорядне клацання активується в нижньому правому куті, а клацання середньою кнопкою — внизу по центру
pinch-to-zoom = Масштабування щипком
    .desc = У застосунках, які підтримують масштабування, використовуйте два пальці для збільшення вмісту
tap-to-click = Натискання дотиком
    .desc = Дотик одним пальцем активує основне клацання, дотик двома пальцями — другорядне клацання, дотик трьома пальцями — клацання середньою кнопкою
touchpad = Сенсорна панель
    .acceleration = Увімкнути прискорення сенсорної панелі
    .desc = Швидкість сенсорної панелі, варіанти натискання, жести
    .speed = Швидкість сенсорної панелі

## Input: Gestures

gestures = Жести
    .four-finger-down = Провести чотирма пальцями вниз
    .four-finger-left = Провести чотирма пальцями ліворуч
    .four-finger-right = Провести чотирма пальцями праворуч
    .four-finger-up = Провести чотирма пальцями вгору
    .three-finger-any = Провести трьома пальцями в будь-якому напрямку
switch-between-windows = Перемикання між вікнами
open-application-library = Відкрити бібліотеку застосунків
open-workspaces-view = Відкрити огляд робочих просторів

## Time & Language

time = Час і мова
    .desc = Н/Д
time-date = Дата та час
    .desc = Часовий пояс, автоматичне налаштування годинника та формат часу
    .auto = Встановлювати автоматично
    .auto-ntp = Дата та час будуть оновлюватися автоматично при встановленні часового поясу
time-zone = Часовий пояс
    .auto = Автоматичне визначення часового поясу
    .auto-info = Необхідні служби визначення місцезнаходження та доступ до Інтернету
time-format = Формат дати та часу
    .twenty-four = 24-годинний формат часу
    .show-seconds = Показувати секунди
    .first = Перший день тижня
    .show-date = Показувати дату на панелі
    .friday = Пʼятниця
    .saturday = Субота
    .sunday = Неділя
    .monday = Понеділок
time-region = Регіон та мова
    .desc = Формат дати, часу та чисел на основі вашого регіону

## System

system = Система і обліковки

## System: About

about = Про систему
    .desc = Назва пристрою, інформація щодо обладнання, типові значення операційної системи
about-device = Назва пристрою
    .desc = Назва, яку бачать інші мережі та пристрої Bluetooth
about-hardware = Обладнання
    .model = Модель обладнання
    .memory = Памʼять
    .processor = Процесор
    .graphics = Графіка
    .disk-capacity = Місткість диска
about-os = Операційна система
    .os = Операційна система
    .os-architecture = Архітектура операційної системи
    .kernel = Версія ядра
    .desktop-environment = Робоче оточення
    .windowing-system = Система керування вікнами
about-related = Повʼязані налаштування
    .support = Підтримка

## System: Firmware

firmware = Мікропрограма
    .desc = Подробиці щодо мікропрограми

## System: Users

users = Користувачі
    .desc = Автентифікація та облікові записи користувачів
    .admin = Адміністратор
    .standard = Звичайний
    .profile-add = Вибрати зображення профілю
remove = Вилучити
connect = Під'єднати
password = Пароль
username = Ім'я користувача
settings = Налаштування
ok = Гаразд
connections-and-profiles =
    { $variant ->
        [wired] Дротове
        [wifi] Wi-Fi
        [vpn] VPN
       *[other] Невідоме
    } з'єднання і профілі підключень.
add-network = Додати мережу
    .profile = Додати профіль
add-vpn = Додати VPN
airplane-on = Увімкнено режим польоту.
cable-unplugged = Дріт від'єднано
connected = Підключено
connecting = З'єднання…
disconnect = Від'єднатися
forget = Забути
known-networks = Відомі мережі
network-and-wireless = Мережа та Wi‑Fi
no-networks = Мереж не знайдено.
no-vpn = Немає доступних VPN-з'єднань.
password-confirm = Підтвердити пароль
visible-networks = Видимі мережі
auth-dialog = Авторизація обов'язкова
    .vpn-description = Введіть ім’я користувача та пароль, необхідні для служби VPN.
    .wifi-description = Введіть пароль або ключ шифрування. Також можна підключитися через кнопку «WPS» на маршрутизаторі.
forget-dialog = Забути цю Wi-Fi мережу?
    .description = Вам потрібно буде знову ввести пароль, щоб використовувати цю мережу в майбутньому.
network-device-state =
    .activated = З'єднано
    .config = З'єднання
    .deactivating = Від'єднання
    .disconnected = Від'єднано
    .failed = Не вдалося з'єднатися
    .ip-check = Перевірка з'єднання
    .ip-config = Отримання IP та даних маршрутизації
    .need-auth = Потрібна авторизація
    .prepare = Підготовка до з'єднання
    .secondaries = Очікування вторинного з'єднання
    .unavailable = Недоступно
    .unknown = Невідомий стан
    .unmanaged = Некерований
    .unplugged = Дріт від'єднано
remove-connection-dialog = Вилучити профіль з'єднання?
    .vpn-description = Щоб скористатися цією мережею знову, вам потрібно буде повторно ввести пароль.
    .wired-description = Щоб скористатися цим підключенням знову, вам потрібно буде створити цей профіль повторно.
vpn = VPN
    .connections = VPN з'єднання
    .error = Не вдалося додати налаштування VPN
    .remove = Видалити профіль з'єднання
    .select-file = Обрати файл налаштувань VPN
vpn-error = Помилка VPN
    .config = Не вдалося додати налаштування VPN
    .connect = Не вдалося з'єднатися до VPN
    .connection-editor = Помилка редактора з'єднань
    .connection-settings = Не вдалося отримати налаштування активних з'єднань
    .updating-state = Не вдалося оновити стан мережевого менеджера
    .wireguard-config-path = Неправильний шлях до профілю WireGuard
    .wireguard-config-path-desc = Обраний файл має бути локальним.
    .wireguard-device = Не вдалося створити пристрій WireGuard
    .with-password =
        nmcli не зміг встановити { $field ->
           *[username] ім'я користувача
            [password] пароль
            [password-flags] флаги читання паролю
        } для VPN
wifi = Wi-Fi
    .adapter = Бездротовий адаптер { $id }
    .forget = Забути цю мережу
wireguard-dialog = Додати пристрій WireGuard
    .description = Оберіть ім'я пристрою для профілю WireGuard.
activate = Активувати
confirm = Підтвердити
enable = Увімкнути
bluetooth = Bluetooth
    .desc = Керувати Bluetooth пристроями
    .status = Ця система відображається як { $aliases }, поки відкриті налаштування Bluetooth.
    .connected = З'єднано
    .connecting = З'єднання
    .disconnecting = Від'єднання
    .connect = З'єднатися
    .disconnect = Від'єднатися
    .forget = Забути
    .dbus-error = Сталося помилка при взаємодії з DBus: { $why }
    .disabled = Служба Bluetooth вимкнена
    .inactive = Служба Bluetooth неактивна
    .unknown = Неможливо активувати службу Bluetooth. Можливо, у вас не встановлено пакет BlueZ?
bluetooth-paired = З'єднані у минулому пристрої
    .connect = Під'єднатися
    .battery = { $percentage }% заряду
bluetooth-confirm-pin = Перевірка PIN-коду між Bluetooth-пристроями
    .description = Будь ласка, підтвердіть, що цей PIN-код зівпадає з тим, який відображається на іншому пристрої: { $device }
bluetooth-available = Пристрої поблизу
bluetooth-adapters = Адаптери Bluetooth
accessibility = Доступність
    .vision = Зір
    .on = Увімкнено
    .off = Вимкнено
    .unavailable = Недоступно
    .screen-reader = Засіб читання з екрану
    .high-contrast = Режим високої контрастності
    .invert-colors = Інвертація кольорів
    .color-filters = Виправлення кольорової сліпоти
hearing = Слух
    .mono = Відтворювати стерео звук у моно
default = Типовий
dbus-connection-error = Не вдалося з'єднатися з DBus
identity = Ідентичність
magnifier = Лупа
    .controls =
        Також можна використати: { $zoom_in ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_in } для наближення,
        }{ $zoom_out ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_out } для віддалення,
        }
        Super + гортання коліщатком миші
    .scroll_controls = Увімкнути зміну масштабу мишею або сенсорною панеллю з Super + гортання
    .show_overlay = Показувати накладання лупи
    .increment = Крок зміни масштабу
    .signin = Запускати лупу під час входу в систему
    .applet = Ви можете увімкнути/вимкнути лупу за допомогою аплету на панелі
    .movement = Наближений вид рухається
    .continuous = Слідувати за вказівником постійно
    .onedge = Коли вказівник торкається краю
    .centered = Тримати вказівник у центрі
color-filter = Режим виправлення кольорової сліпоти
    .unknown = Активовано невідомий режим
    .greyscale = Відтінки сірого
    .deuteranopia = Зелено-червоний (нечутливість до зеленого, Дейтеранопія)
    .protanopia = Червоно-зелений (нечутливість до червоного, Протанопія)
    .tritanopia = Блакитно-жовтий (нечутливість до синього, Тританопія)
never = Ніколи
interface-density = Щільність інтерфейсу
    .comfortable = Комфортна
    .compact = Компактна
    .spacious = Простора
icons-and-toolkit = Тема піктограм
interface-font = Системний шрифт
monospace-font = Моноширинний шрифт
edge-gravity = Плавучі вікна чіпляються до країв екрану
focus-navigation = Навігація фокусом
    .focus-follows-cursor = Фокус слідкує за вказівником
    .focus-follows-cursor-delay = Затримка слідкування фокусу за вказівником у мс
    .cursor-follows-focus = Вказівник слідкує за фокусом
vrr = Адаптивна синхронізація
    .enabled = Увімкнена
    .force = Завжди
    .auto = Автоматично
    .disabled = Вимкнена
amplification = Посилення звуку
    .desc = Дозволяє встановити гучність до 150%
battery = Акумулятор
    .minute =
        { $value } { $value ->
            [one] хвилина
            [few] хвилини
            [many] хвилин
           *[other] хвилин
        }
    .hour =
        { $value } { $value ->
            [one] година
            [few] години
            [many] годин
           *[other] годин
        }
    .day =
        { $value } { $value ->
            [one] день
            [few] дні
            [many] днів
           *[other] днів
        }
    .less-than-minute = Менше ніж хвилина
    .and = і
    .remaining-time =
        { $time } до { $action ->
            [full] наснаження
           *[other] виснаження
        }
connected-devices = Під'єднані пристрої
    .unknown = Невідомний пристрій
power-saving = Налаштування енергозбереження
    .turn-off-screen-after = Вимкнути екран через
    .auto-suspend = Автоматичне призупинення
    .auto-suspend-ac = Автоматичне призупинення під час роботи від мережі
    .auto-suspend-battery = Автоматичне призупинення під час роботи від акумулятора
keyboard-numlock-boot = Numlock
    .boot-state = Стан при запуску
    .last-boot = Зберігати стан минулого запуску
    .on = Завжди увімкнений
    .off = Завжди вимкнений
    .set = Задати стан Numlock при запуску системи
show-extended-input-sources = Показувати розширені джерела введення
add-another-keybinding = Додати сполучення
input-source-switch = Змінити розкладку клавіатури
zoom-in = Наблизити
zoom-out = Віддалити
switch-workspaces = Змінити робочі простори
    .horizontal = Проведення чотрима пальцями вліво/вправо
    .vertical = Проведення чотрима пальцями вгору/вниз
formatting = Формати
    .dates = Дати
    .time = Час
    .date-and-time = Дата і час
    .numbers = Числа
    .measurement = Одиниця виміру
    .paper = Папір
preferred-languages = Обрані мови
    .desc = Порядок мов визначає, яка мова буде використовуватися як мова інтерфейсу. Зміни набудуть чинності під час наступного входу в систему.
add-language = Додати мову
    .context = Додати мову
install-additional-languages = Встановити додаткові мови
region = Регіон
applications = Застосунки
default-apps = Типові застосунки
    .desc = Типовий браузер, поштовий клієнт, файловий менеджер та інші застосунки
    .web-browser = Браузер
    .file-manager = Файловий менеджер
    .mail-client = Поштовий клієнт
    .music = Музика
    .video = Відео
    .photos = Зображення
    .calendar = Календар
    .terminal = Термінал
    .other-associations = Інші застосунки
    .text-editor = Текстовий редактор
startup-apps = Автозапуск
    .desc = Налаштування застосунків, які запускаються при вході у систему
    .add = Додати застосунок
    .user = Застосунки, які запускаються при вході у систему
    .none = В автозапуску немає ніяких застосунків
    .remove-dialog-title = Вилучити { $name }?
    .remove-dialog-description = Вилучити цей застосунок з автозапуску?
    .add-startup-app = Додати застосунок до автозапуску
legacy-applications = Сумісність застосунків X11
    .desc = Масштабування застосунків X11 і глобальні сполучення клавіш віконного менеджера
legacy-app-global-shortcuts = Глобальні сполучення клавіш у застосунках X11
    .desc = Глобальні сполучення дозволяють іншим застосункам реагувати на натискання клавіш і кнопок миші, навіть якщо вони виконані в іншому застосунку, наприклад для функцій активації або вимкнення голосу кнопкою. За замовчуванням ця функція вимкнена у застосунках X11, щоб уникнути перехоплення чутливої інформації.
    .none = Ніякі клавіші
    .modifiers = Модифікатори (Super, Shift, Control, Alt)
    .combination = Усі клавіши у комбінації з клавішами Super, Control або Alt
    .all = Усі клавіші
    .mouse = Події натискання кнопок миші у застосунках X11
legacy-app-scaling = Масштабування застосунків X11
    .scaled-gaming = Оптимізовано для ігор та повноекранних застосунків
    .gaming-description = Застосунки X11 можуть виглядати трохи більшими або меншими, ніж застосунки Wayland.
    .scaled-applications = Оптимізовано для звичайних застосунків
    .applications-description = Ігри та повноекранні застосунки X11 можуть не відповідати роздільній здатності вашого екрана.
    .scaled-compatibility = Режим максимальної сумісності
    .compatibility-description = Застосунки X11 можуть виглядати нечітко на HiDPI екранах.
    .preferred-display = Надавати перевагу цьому дисплею для ігор та повноекранних застосунків X11
    .no-display = Жодного
administrator = Адміністратор
    .desc = Адміністратори можуть змінювати налаштування для всіх користувачів, додавати та видаляти інших користувачів
add-user = Додати користувача
change-password = Змінити пароль
remove-user = Видалити користувача
full-name = Повне ім'я
invalid-username = Неприпустиме ім'я користувача
password-mismatch = Обидва паролі мусять збігатися
save = Зберегти
qr-code-unavailable = QR-код недоступний
network-name = Ім'я мережі
share = Ширити мережу
scan-to-connect-description = Відскануйте QR-код, щоб з'єднатися з цією мережею.
sound-hd-audio = HD Аудіо
sound-usb-audio = USB Аудіо
sound-device-profiles = Профілі пристроїв
shadows-floating = Плавучі вікна
    .clip = Вирівняти до кутів системи та застосувати тіні
shadows-tiling = Укладені вікна
    .clip = Вирівняти до системних кутів
    .shadow = Застосувати тіні
shadow-and-corners = Тіні й кути вікон
sound-device-port-unplugged = Відключено
