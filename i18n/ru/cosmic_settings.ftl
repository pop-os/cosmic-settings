app = Параметры
unknown = Неизвестно
number = { $number }

## Desktop

desktop = Рабочий стол

## Desktop: Appearance

appearance = Внешний вид
accent-color = Акцентный цвет
app-background = Фон приложений или окон
auto = Автоматически
close = Закрыть
color-picker = Выбор цвета
copied-to-clipboard = Скопировано в буфер обмена
copy-to-clipboard = Копировать в буфер обмена
dark = Тёмное
export = Экспорт
hex = Hex
import = Импорт
light = Светлое
mode-and-colors = Цвета и оформление
recent-colors = Недавние цвета
reset-to-default = Вернуть по умолчанию
rgb = RGB
window-hint-accent = Цвет подсветки активного окна
window-hint-accent-toggle = Использовать акцентный цвет в качестве подсветки активного окна
auto-switch = Автоматически переключать оформление
    .sunrise = Сменится на светлое при восходе
    .sunset = Сменится на тёмное при закате
    .next-sunrise = Сменится на светлое при следующем восходе
    .next-sunset = Сменится на тёмное при следующем закате
container-background = Фон контейнера
    .desc-detail = Цвет фона контейнера используется для боковой панели навигации, бокового меню, диалоговых окон и других подобных виджетов. По умолчанию он автоматически определяется на основе фона окна.
    .reset = Вернуть автоматический
    .desc = Используется для боковой панели навигации, бокового меню, диалоговых окон и других подобных виджетов
control-tint = Оттенок компонентов управления
    .desc = Используется для фонов стандартных кнопок, полей ввода текста и других подобных компонентов
frosted = Эффект матового стекла на интерфейсе системы
    .desc = Размытие фона для верхней панели, дока, апплетов, панели запуска и библиотеки приложений
experimental-settings = Экспериментальные настройки
enable-export = Применить текущее оформление к приложениям GNOME.
    .desc = Не все наборы инструментов поддерживают автоматическое переключение. После смены темы может потребоваться перезапуск приложений, не относящихся к COSMIC.
icon-theme = Тема значков
    .desc = Применить другой набор значков для приложений
text-tint = Оттенок текста интерфейса
    .desc = Цвет, используемый для выведения цветов текста интерфейса, обладающих достаточной контрастностью на различных поверхностях
style = Стиль
    .round = Округлый
    .slightly-round = Слегка округлый
    .square = Прямой
# interface density left out for now
window-management = Управление окнами

## Desktop: Display

-requires-restart = Требует перезагрузки
color = Цвет
    .depth = Глубина цвета
    .profile = Цветовой профиль
    .sidebar = Цветовые профили
    .temperature = Цветовая температура
display = Экраны
    .arrangement = Расположение экранов
    .arrangement-desc = Перетаскивайте экраны, чтобы изменить их расположение.
    .enable = Включить экран
    .external = Внешний экран { $size } { $output }
    .laptop = Экран ноутбука { $size }
    .options = Параметры экрана
    .refresh-rate = Частота обновления
    .resolution = Разрешение
    .scale = Масштаб
    .additional-scale-options = Дополнительная настройка масштаба
mirroring = Зеркальное отображение
    .id = Зеркальное отображение { $id }
    .dont = Не зеркалить
    .mirror = Отзеркалить { $display }
    .project =
        Отображение на { $display ->
            [all] всех мониторах
           *[other] { $display }
        }
    .project-count =
        Отображение на { $count } др. { $count ->
            [1] мониторе
           *[other] мониторах
        }
night-light = Ночной свет
    .auto = Автоматически (от заката до рассвета)
    .desc = Уменьшите синий свет, используя более тёплые цвета
orientation = Ориентация
    .standard = Стандартная
    .rotate-90 = Поворот на 90°
    .rotate-180 = Поворот на 180°
    .rotate-270 = Поворот на 270°
scheduling = Расписание
    .manual = Ручное расписание
dialog = Диалог
    .title = Сохранить эти настройки монитора?
    .keep-changes = Сохранить изменения
    .change-prompt = Изменения настроек будут автоматически отменены через { $time } сек.
    .revert-settings = Вернуть прежние настройки

## Desktop: Notifications

notifications = Уведомления

## Desktop: Options

dock = Док
hot-corner = Активные углы
    .top-left-corner = Открывать рабочие столы при наведении в левый верхний угол
super-key = Клавиша Super
    .launcher = Открывает панель запуска
    .workspaces = Открывает обзор рабочих столов
    .applications = Открывает приложения
    .disable = Отключена
window-controls = Элементы управления окнами
    .minimize = Отображать кнопку «Свернуть»
    .maximize = Отображать кнопку «Развернуть»
    .active-window-hint = Отображать обводку активного окна

## Desktop: Panel

panel = Панель
add = Добавить
add-applet = Добавить апплет
all = Все
applets = Апплеты
center-segment = Центральный сегмент
end-segment = Конечный сегмент
large = Крупный
no-applets-found = Апплеты не найдены…
panel-bottom = Внизу
panel-left = Слева
panel-right = Справа
panel-top = Вверху
search-applets = Поиск апплетов…
small = Малый
start-segment = Начальный сегмент
panel-appearance = Внешний вид
    .match = Как в системе
    .light = Светлый
    .dark = Тёмный
panel-behavior-and-position = Поведение и расположение
    .autohide = Автоматически скрывать панель
    .dock-autohide = Автоматически скрывать док
    .position = Расположение на экране
    .display = Отображать на экране
panel-style = Оформление
    .anchor-gap = Пробел между панелью и краями экрана
    .dock-anchor-gap = Пробел между доком и краями экрана
    .extend = Расширить панель до краёв экрана
    .dock-extend = Расширить док до краёв экрана
    .appearance = Внешний вид
    .size = Размер
    .background-opacity = Непрозрачность фона
panel-applets = Конфигурация
    .dock-desc = Настройка апплетов дока
    .desc = Настройка апплетов панели
panel-missing = Отсутствует конфигурация панели
    .desc = Файл конфигурации панели повреждён или отсутствует в связи с использованием нестандартной конфигурации.
    .fix = Восстановить конфигурацию по умолчанию

## Desktop: Wallpaper

wallpaper = Фон
    .change = Сменять изображение каждые
    .fit = Подгонять фон
    .folder-dialog = Выбрать папку с изображениями
    .image-dialog = Выбрать изображение
    .plural = Обои
    .same = Одинаковый фон на всех экранах
    .slide = Слайд-шоу
add-color = Добавить цвет
add-image = Добавить изображение
all-displays = Все экраны
colors = Цвета
dialog-add = Добавить
fill = Заполнить
fit-to-screen = Подгонять под экран
open-new-folder = Открыть новую папку
recent-folders = Недавние папки
x-minutes =
    { $number ->
        [one] 1 минута
       *[other] { $number } мин.
    }
x-hours =
    { $number ->
        [one] 1 час
       *[other] { $number } ч.
    }

## Desktop: Workspaces

workspaces = Рабочие столы
workspaces-behavior = Поведение рабочих столов
    .dynamic = Динамические рабочие столы
    .dynamic-desc = Пустые рабочие столы удаляются автоматически.
    .fixed = Фиксированное число рабочих столов
    .fixed-desc = Добавляйте и удаляйте рабочие столы в Обзоре.
workspaces-multi-behavior = Поведение для нескольких мониторов
    .span = Рабочие столы охватывают все мониторы
    .separate = Отдельные рабочие столы для каждого монитора
workspaces-overview-thumbnails = Миниатюры рабочих столов в Обзоре
    .show-number = Отображать номер рабочего стола
    .show-name = Отображать имя рабочего стола
workspaces-orientation = Ориентация рабочих столов
    .vertical = Вертикальная
    .horizontal = Горизонтальная

## Networking: Wired

wired = Проводная сеть
    .adapter = Проводной адаптер { $id }
    .connections = Проводные соединения
    .devices = Проводные устройства
    .remove = Удалить профиль соединения

## Networking: Online Accounts

online-accounts = Онлайн-аккаунты
    .desc = Добавление учётных записей, IMAP и SMTP, корпоративных логинов

## Time & Language

time = Время и язык
time-date = Дата и время
    .auto = Устанавливать автоматически
    .auto-ntp = Дата и время будут автоматически обновлены после установки часового пояса
time-zone = Часовой пояс
    .auto = Устанавливать автоматически
    .auto-info = Требуются службы определения местоположения и доступ в Интернет
time-format = Формат даты и времени
    .twenty-four = 24-часовой формат
    .show-seconds = Отображать секунды
    .first = Первый день недели
    .show-date = Отображать дату в апплете времени
    .friday = Пятница
    .saturday = Суббота
    .sunday = Воскресенье
    .monday = Понедельник
time-region = Регион и язык

## Sound

sound = Звук
sound-output = Выход
    .volume = Выходная громкость
    .device = Выходное устройство
    .level = Выходной уровень
    .config = Конфигурация
    .balance = Баланс
    .left = Лево
    .right = Право
sound-input = Вход
    .volume = Входная громкость
    .device = Входное устройство
    .level = Входной уровень
sound-alerts = Предупреждения
    .volume = Громкость предупреждений
    .sound = Звук предупреждений
sound-applications = Приложения
    .desc = Громкость и настройки приложений

## System

system = Система и учётные записи

## System: About

about = О системе
about-device = Имя устройства
    .desc = Это имя видно для других устройств по сети или Bluetooth
about-hardware = Оборудование
    .model = Модель оборудования
    .memory = Память
    .processor = Процессор
    .graphics = Графика
    .disk-capacity = Ёмкость диска
about-os = Операционная система
    .os = Операционная система
    .os-architecture = Архитектура ОС
    .kernel = Версия ядра
    .desktop-environment = Среда рабочего стола
    .windowing-system = Оконная система
about-related = Связанные настройки
    .support = Получить поддержку

## System: Firmware

firmware = Прошивка

## System: Users

users = Пользователи
    .admin = Администратор
    .standard = Обычный
    .profile-add = Выберите изображение профиля

## Input

acceleration-desc = Автоматически регулирует чувствительность панели в зависимости от скорости
disable-while-typing = Отключать при наборе текста
input-devices = Устройства ввода
primary-button = Основная кнопка
    .desc = Настраивает порядок физических кнопок.
    .left = Левая
    .right = Правая
scrolling = Прокрутка
    .two-finger = Прокрутка двумя пальцами
    .edge = Прокрутка по краю одним пальцем
    .speed = Скорость прокрутки
    .natural = Естественная прокрутка
    .natural-desc = Прокручивать содержимое, а не представление

## Input: Keyboard

slow = Медленно
fast = Быстро
short = Короткая
long = Длинная
keyboard = Клавиатура
keyboard-sources = Источники ввода
    .desc = Источники ввода можно переключать по комбинации клавиш Super+Space. Это поведение можно изменить в настройках сочетаний клавиш.
    .move-up = Переместить вверх
    .move-down = Переместить вниз
    .settings = Настройки
    .view-layout = Просмотреть раскладку
    .remove = Удалить
    .add = Добавить источник ввода
keyboard-special-char = Ввод специальных символов
    .alternate = Клавиша альтернативных символов
    .compose = Клавиша Compose
    .compose-desc = Клавиша Compose позволяет вводить широкое множество символов. Для этого нужно нажать на неё, а затем ввести последовательность «составляющих» символов. Например, нажатие Compose, латинских «o» и «c» введёт «©», а нажатие Compose, «з» и «=» — «₽».
    .caps = Клавиша Caps Lock
keyboard-typing-assist = Ввод текста
    .repeat-rate = Скорость повторения
    .repeat-delay = Задержка повторения
added = Добавлено
type-to-search = Введите для поиска…

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Сочетания клавиш
    .desc = Просмотр и настройка сочетаний клавиш
cancel = Отмена
command = Команда
custom = Пользовательские
debug = Отладка
disabled = Отключено
migrate-workspace-prev = Перенести рабочий стол на пред. выход
migrate-workspace-next = Перенести рабочий стол на след. выход
migrate-workspace =
    Перенести рабочий стол на выход { $direction ->
       *[down] снизу
        [left] слева
        [right] справа
        [up] сверху
    }
navigate = Перейти
replace = Заменить
shortcut-name = Название сочетания клавиш
system-controls = Системные элементы управления
terminate = Завершить
toggle-stacking = Собрать/распустить стопку окон
type-key-combination = Введите комбинацию клавиш
custom-shortcuts = Пользовательские сочетания клавиш
    .add = Добавить сочетание клавиш
    .context = Добавить пользовательское сочетание клавиш
    .none = Нет пользовательских сочетаний клавиш
modified = { $count } изменено
nav-shortcuts = Навигация
    .prev-output = Фокус на предыдущем выходе
    .next-output = Фокус на следующем выходе
    .last-workspace = Фокус на последнем рабочем столе
    .prev-workspace = Фокус на предыдущем рабочем столе
    .next-workspace = Фокус на следующем рабочем столе
    .focus =
        Фокус на окне { $direction ->
           *[down] снизу
            [in] установить
            [left] слева
            [out] убрать
            [right] справа
            [up] сверху
        }
    .output =
        Переключиться на выход { $direction ->
           *[down] снизу
            [left] слева
            [right] справа
            [up] сверху
        }
    .workspace = Переключиться на рабочий стол { $num }
manage-windows = Управление окнами
    .close = Закрыть окно
    .maximize = Развернуть окно
    .fullscreen = Окно во весь экран
    .minimize = Свернуть окно
    .resize-inwards = Изменить размер изнутри
    .resize-outwards = Изменить размер снаружи
    .toggle-sticky = Вкл./выкл закрепление окна
move-windows = Перемещение окон
    .direction =
        Переместить окно { $direction ->
           *[down] вниз
            [left] влево
            [right] вправо
            [up] вверх
        }
    .display =
        Переместить окно на монитор { $direction ->
           *[down] снизу
            [left] слева
            [right] справа
            [up] сверху
        }
    .workspace =
        Переместить окно на рабочий стол { $direction ->
           *[below] ниже
            [left] левее
            [right] правее
            [above] выше
        }
    .workspace-num = Переместить окно на рабочий стол { $num }
    .prev-workspace = Переместить окно на пред. рабочий стол
    .next-workspace = Переместить окно на след. рабочий стол
    .last-workspace = Переместить окно на посл. рабочий стол
    .next-display = Переместить окно на след. монитор
    .prev-display = Переместить окно на пред. монитор
    .send-to-prev-workspace = Переместить окно на предыдущий рабочий стол
    .send-to-next-workspace = Переместить окно на следующий рабочий стол
system-shortcut = Системные
    .app-library = Открыть библиотеку приложений
    .brightness-down = Уменьшить яркость экрана
    .brightness-up = Увеличить яркость экрана
    .display-toggle = Вкл/выкл встроенный экран
    .home-folder = Открыть домашнюю папку
    .keyboard-brightness-down = Уменьшить яркость клавиатуры
    .keyboard-brightness-up = Увеличить яркость клавиатуры
    .launcher = Открыть панель запуска
    .log-out = Выход из системы
    .lock-screen = Заблокировать экран
    .mute = Приглушить аудиовыход
    .mute-mic = Приглушить аудиовход микрофона
    .play-pause = Воспроизвести/Приостановить
    .play-next = Следующий трек
    .play-prev = Предыдущий трек
    .poweroff = Завершение работы
    .screenshot = Сделать снимок экрана
    .suspend = Спящий режим
    .terminal = Открыть терминал
    .touchpad-toggle = Вкл/выкл сенсорную панель
    .volume-lower = Уменьшить громкость аудиовыхода
    .volume-raise = Увеличить громкость аудиовыхода
    .web-browser = Открыть веб-браузер
    .window-switcher = Переключение между открытыми окнами
    .window-switcher-previous = Переключение между откр. окнами в обр. порядке
    .workspace-overview = Открыть обзор рабочих столов
window-tiling = Размещение окон
    .horizontal = Установить горизонтальную ориентацию
    .vertical = Установить вертикальную ориентацию
    .swap-window = Сменить окно
    .toggle-tiling = Переключить размещение окна
    .toggle-stacking = Переключить стопку окна
    .toggle-floating = Переключить плавающее окно
    .toggle-orientation = Переключить ориентацию
replace-shortcut-dialog = Заменить сочетание клавиш?
    .desc = { $shortcut } уже используется { $name }. Если вы замените его, { $name } будет отключено.

## Input: Mouse

mouse = Мышь
    .speed = Скорость мыши
    .acceleration = Включить ускорение мыши

## Input: Touchpad

click-behavior = Поведение при нажатии
    .click-finger = Вторичное нажатие двумя пальцами и среднее нажатие тремя пальцами
    .button-areas = Вторичное нажатие в правом нижнем углу и среднее нажатие в центре внизу
pinch-to-zoom = Сведение и разведение для масштабирования
    .desc = Используйте два пальца для увеличения содержимого в приложениях, поддерживающих масштабирование
tap-to-click = Касание для нажатия
    .desc = Включает касание одним пальцем для основного нажатия, двумя пальцами для вторичного и тремя пальцами для среднего
touchpad = Сенсорная панель
    .acceleration = Включить ускорение сенсорной панели
    .speed = Скорость сенсорной панели

## Input: Gestures

gestures = Жесты
    .four-finger-down = Проведение четырьмя пальцами вниз
    .four-finger-left = Проведение четырьмя пальцами влево
    .four-finger-right = Проведение четырьмя пальцами вправо
    .four-finger-up = Проведение четырьмя пальцами вверх
    .three-finger-any = Проведение тремя пальцами в любом направлении
switch-between-windows = Переключение между окнами
open-application-library = Открыть библиотеку приложений
open-workspaces-view = Открыть обзор рабочих столов

## Power

power = Питание и аккумулятор
power-mode = Режим питания
    .performance = Высокая производительность
    .balanced = Сбалансированный
    .battery = Энергосбережение
    .performance-desc = Повышенная производительность и энергопотребление.
    .balanced-desc = Невысокая производительность и умеренное энергопотребление.
    .battery-desc = Сниженное энергопотребление и тихая работа.
    .no-backend = Службы электропитания не найдены. Установите system76-power или power-profiles-daemon.
remove = Убрать
save = Сохранить
connect = Подключиться
dbus-connection-error = Не удалось подключиться к DBus
ok = ОК
add-network = Добавить сеть
    .profile = Добавить профиль
add-vpn = Добавить VPN
airplane-on = Включён режим полёта.
cable-unplugged = Кабель не подключён
connected = Подключено
connecting = Подключение…
disconnect = Отключить
forget = Забыть
known-networks = Известные сети
network-and-wireless = Сеть и Wi-Fi
no-networks = Сети не обнаружены.
no-vpn = Нет доступных VPN-соединений.
password = Пароль
password-confirm = Подтверждение пароля
settings = Параметры
username = Имя пользователя
visible-networks = Видимые сети
identity = Идентификатор
auth-dialog = Требуется авторизация
    .vpn-description = Введите требуемые сервисом VPN имя пользователя и пароль.
    .wifi-description = Введите пароль или ключ шифрования. Вы также можете подключиться, нажав кнопку «WPS» на роутере.
forget-dialog = Забыть эту сеть Wi-Fi?
    .description = Вам нужно будет ввести пароль ещё раз, чтобы использовать эту сеть Wi-Fi в будущем.
network-device-state =
    .activated = Подключено
    .config = Подключение
    .deactivating = Отключение
    .disconnected = Отключено
    .failed = Не удалось подключиться
    .ip-check = Проверка подключения
    .ip-config = Получение IP и маршрутов
    .need-auth = Требуется авторизация
    .prepare = Подготовка к подключению
    .secondaries = Ожидание вторичного подключения
    .unavailable = Недоступно
    .unknown = Состояние неизвестно
    .unmanaged = Не управляется
    .unplugged = Кабель отключён
remove-connection-dialog = Удалить профиль подключения?
    .vpn-description = Чтобы использовать эту сеть в дальнейшем, вам снова потребуется ввести пароль.
    .wired-description = Чтобы использовать эту сеть в дальнейшем, вам потребуется пересоздать этот профиль.
vpn = VPN
    .connections = VPN-соединения
    .error = Не удалось добавить конфигурацию VPN
    .remove = Удалить профиль соединения
    .select-file = Выберите файл конфигурации VPN
vpn-error = Ошибка VPN
    .config = Не удалось добавить конфигурацию VPN
    .connect = Не удалось подключиться к VPN
    .connection-editor = Сбой редактора соединений
    .connection-settings = Не удалось получить настройки активных подключений
    .updating-state = Не удалось обновить состояние сетевого менеджера
    .wireguard-config-path = Некорректный путь до файла конфигурации WireGuard
    .wireguard-config-path-desc = Выбранный файл должен находиться на локальной файловой системе.
    .wireguard-device = Не удалось создать устройство WireGuard
    .with-password =
        Не удалось настроить { $field ->
           *[username] имя пользователя
            [password] пароль
            [password-flags] флаги пароля
        } VPN через nmcli
wifi = Wi-Fi
    .adapter = Wi-Fi-адаптер { $id }
    .forget = Забыть эту сеть
wireguard-dialog = Добавить устройство WireGuard
    .description = Выберите имя устройства для конфигурации WireGuard.
activate = Активировать
confirm = Подтвердить
enable = Включить
bluetooth = Bluetooth
    .status = Эта система доступна как { $aliases }, пока открыты настройки Bluetooth.
    .connected = Подключено
    .connecting = Подключение
    .disconnecting = Отключение
    .connect = Подключиться
    .disconnect = Отключиться
    .forget = Забыть
    .dbus-error = Произошла ошибка во время взаимодействия с DBus: { $why }
    .disabled = Служба Bluetooth отключена
    .inactive = Служба Bluetooth не активна
    .unknown = Не удаётся активировать службу Bluetooth. Установлен ли BlueZ?
bluetooth-paired = Ранее подключённые устройства
    .connect = Подключиться
    .battery = { $percentage } % заряда
bluetooth-confirm-pin = Подтверждение PIN-кода Bluetooth
    .description = Подтвердите, что следующий PIN-код совпадает с отображаемым на { $device }
bluetooth-available = Устройства поблизости
bluetooth-adapters = Bluetooth-адаптеры
accessibility = Специальные возможности
    .vision = Зрение
    .on = Вкл
    .off = Выкл
    .unavailable = Недоступно
    .screen-reader = Экранный диктор
    .high-contrast = Режим высокой контрастности
    .invert-colors = Инвертирование цвета
    .color-filters = Цветовые фильтры
hearing = Слух
    .mono = Проигрывать стереозвук как монозвук
default = По умолчанию
magnifier = Экранная лупа
    .controls =
        Либо используйте сочетания клавиш: { $zoom_in ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_in } для приближения,
        }{ $zoom_out ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_out } для отдаления,
        }
        Super + прокрутка мышью
    .scroll_controls = Включить масштабирование по Super + прокрутка мышью или сенсорной панелью
    .show_overlay = Показывать всплывающее окно экранной лупы
    .increment = Шаг масштабирования
    .signin = Запускать экранную лупу при входе в систему
    .applet = Вы можете переключать экранную лупу через апплет на панели
    .movement = Перемещение приближения
    .continuous = Последовательно за курсором
    .onedge = Когда курсор достигает края
    .centered = Обеспечение положения курсора по центру
color-filter = Тип цветового фильтра
    .unknown = Неизвестный фильтр
    .greyscale = Оттенки серого
    .deuteranopia = Зелёный/Красный (невосприимчивость зелёного, дейтеранопия)
    .protanopia = Красный/Зелёный (невосприимчивость красного, протанопия)
    .tritanopia = Синий/Жёлтый (невосприимчивость синего, тританопия)
never = Никогда
interface-density = Плотность интерфейса
    .comfortable = Комфортная
    .compact = Компактная
    .spacious = Просторная
window-management-appearance = Управление окнами
    .active-hint = Толщина обводки активного окна
    .gaps = Отступы между размещёнными окнами
icons-and-toolkit = Стилизация значков и графики
interface-font = Системный шрифт
monospace-font = Моноширинный шрифт
edge-gravity = Плавающие окна притягиваются к ближайшим краям
focus-navigation = Управление фокусировкой
    .focus-follows-cursor = Фокусировка следует за курсором
    .focus-follows-cursor-delay = Задержка фокусировки вслед за курсором, мс
    .cursor-follows-focus = Курсор следует за фокусировкой
vrr = Динамическая частота обновления
    .enabled = Включена
    .force = Всегда
    .auto = Автоматически
    .disabled = Отключена
battery = Аккумулятор
    .minute =
        { $value } { $value ->
            [one] минута
           *[other] мин.
        }
    .hour =
        { $value } { $value ->
            [one] час
           *[other] ч.
        }
    .day =
        { $value } { $value ->
            [one] день
           *[other] д.
        }
    .less-than-minute = меньше минуты
    .and = и
    .remaining-time =
        { $time } до полной { $action ->
            [full] зарядки
           *[other] разрядки
        }
connected-devices = Подключённые устройства
    .unknown = Неизвестное устройство
amplification = Усиление звука
    .desc = Разрешить увеличение звука до 150%
power-saving = Настройки энергосбережения
    .turn-off-screen-after = Выключать экран через
    .auto-suspend = Переход в режим ожидания
    .auto-suspend-ac = Переход в режим ожидания при питании от сети
    .auto-suspend-battery = Переход в режим ожидания при питании от батареи
keyboard-numlock-boot = Num Lock
    .boot-state = Состояние при загрузке
    .last-boot = По последней загрузке
    .on = Вкл
    .off = Выкл
    .set = Настройка состояния Num Lock по загрузке системы
show-extended-input-sources = Показать дополнительные источники ввода
add-another-keybinding = Добавить сочетание клавиш
input-source-switch = Перекл. источник ввода клавиатуры
zoom-in = Приблизить
zoom-out = Отдалить
switch-workspaces = Переключение рабочих столов
    .horizontal = Проведение четырьмя пальцами влево-вправо
    .vertical = Проведение четырьмя пальцами вверх-вниз
formatting = Форматирование
    .dates = Даты
    .time = Время
    .date-and-time = Дата и время
    .numbers = Числа
    .measurement = Система мер
    .paper = Бумажный формат
preferred-languages = Предпочитаемые языки
    .desc = Порядок языков определяет, какой язык будет использоваться в интерфейсе. Для применения изменений требуется перезайти в систему.
add-language = Добавить язык
    .context = Добавить язык
install-additional-languages = Установить дополнительные языки
region = Регион
applications = Приложения
default-apps = Приложения по умолчанию
    .web-browser = Веб-браузер
    .file-manager = Файловый менеджер
    .mail-client = Почтовый клиент
    .music = Музыка
    .video = Видео
    .photos = Фотографии
    .calendar = Календарь
    .terminal = Терминал
    .other-associations = Прочие ассоциации
    .text-editor = Текстовый редактор
    .not-installed = Не установлено
startup-apps = Автозапуск приложений
    .add = Добавить приложение
    .user = Приложения, которые запускаются при входе в систему
    .none = Нет автозапускаемых приложений
    .remove-dialog-title = Убрать { $name }?
    .remove-dialog-description = Вы уверены, что хотите убрать это приложение из автозапуска?
    .add-startup-app = Добавить приложение в автозапуск
legacy-applications = Совместимость с приложениями для X11
legacy-app-global-shortcuts = Глобальные сочетания клавиш в приложениях для X11
    .desc = Глобальные сочетания клавиш позволяют нажатиям клавиатуры и мыши, выполненных в одних приложениях, распознаваться другими приложениями для таких функций, как push-to-talk и push-to-mute. По умолчанию они отключены для приложений X11, чтобы другие приложения не могли перехватывать события клавиатуры и мыши, содержащие конфиденциальную информацию.
    .none = Отключены
    .modifiers = Клавиши-модификаторы (Super, Shift, Control, Alt)
    .combination = Все клавиши, когда клавиши-модификаторы Super, Control или Alt нажаты
    .all = Все клавиши
    .mouse = События кнопок мыши в приложениях для X11
legacy-app-scaling = Масштабирование приложений для оконной системы X11
    .scaled-gaming = Оптимизировать для игр и полноэкранных приложений
    .gaming-description = Приложения для X11 могут выглядеть чуть больше или меньше по сравнению с приложениями для Wayland.
    .scaled-applications = Оптимизировать для приложений
    .applications-description = Игры и полноэкранные приложения для X11 могут не совпадать с разрешением вашего экрана.
    .scaled-compatibility = Режим максимальной совместимости
    .compatibility-description = Приложения для X11 могут выглядеть размытыми на экранах с высоким разрешением.
    .preferred-display = Предпочитаемый экран для игр и полноэкранных приложений для X11
    .no-display = Нет
administrator = Администратор
    .desc = Администраторы могут изменять настройки для всех пользователей, добавлять и удалять другие учётные записи
add-user = Добавить пользователя
change-password = Изменить пароль
remove-user = Удалить пользователя
full-name = Полное имя
invalid-username = Некорректное имя пользователя
password-mismatch = Пароль и подтверждение пароля должны совпадать
network-name = Имя сети
qr-code-unavailable = QR-код недоступен
scan-to-connect-description = Отсканируйте QR-код, чтобы подключиться к этой сети.
share = Поделиться сетью
shadows-floating = Плавающие окна
    .clip = Соответствовать системным углам и применять тени
shadows-tiling = Размещённые окна
    .clip = Углы в соответствии с системными
    .shadow = Добавлять тени
shadow-and-corners = Тень и углы окон
place-here = Поместите апплеты сюда
sound-device-port-unplugged = Не подключено
sound-hd-audio = Звук в HD
sound-usb-audio = Звук по USB
sound-device-profiles = Профили устройств
workspaces-overview = Режим обзора рабочих столов
    .action-on-typing = При вводе текста
    .none = Ничего не делать
    .launcher = Открыть панель запуска
    .applications = Открыть приложения
xdg-entry-touchpad = Сенсорная панель
xdg-entry-wired = Проводная сеть
xdg-entry-startup-apps-comment = Настройка приложений, запускаемых при входе в систему
xdg-entry-region-language = Язык и регион
xdg-entry-mouse-keywords = COSMIC;Мышь;Ускорение;Прокрутка;Скроллинг;
xdg-entry-bluetooth-comment = Управление Bluetooth-устройствами
xdg-entry-notifications-keywords = COSMIC;Уведомления;Блокировка;
xdg-entry-startup-apps-keywords = COSMIC;Автозапуск;Автозагрузка;Приложение;
xdg-entry-default-apps-comment = Веб-браузер, почтовый клиент, файловый менеджер и другие приложения, выбираемые по умолчанию
xdg-entry-keyboard-comment = Источники ввода, переключение, ввод специальных символов, сочетания клавиш
xdg-entry-wired-keywords = COSMIC;Проводная;LAN;Сеть;Подключение;Соединение;
xdg-entry-power = Питание и аккумулятор
xdg-entry-appearance-keywords = COSMIC;Акцентный;Цвет;Иконки;Значки;Шрифт;Тема;Оформление
xdg-entry-mouse = Мышь
xdg-entry-dock-keywords = COSMIC;Док;Панель;Апплет
xdg-entry-mouse-comment = Скорость мыши, ускорение и естественная прокрутка
xdg-entry-system = Система и учётные записи
xdg-entry-panel-keywords = COSMIC;Панель;Апплет
xdg-entry-window-management = Управление окнами
xdg-entry-a11y = Специальные возможности
xdg-entry-time-language-comment = Управление системной датой, временем, регионом и языком
xdg-entry-desktop-keywords = COSMIC;Рабочий;Стол;
xdg-entry-x11-applications-keywords = COSMIC;X11;Иксы;Приложение;Игра;Совместимость;
xdg-entry-displays-comment = Управление параметрами конфигурации экранов
xdg-entry-touchpad-keywords = COSMIC;Сенсорная;Панель;Тачпад;Жесты;
xdg-entry-time-language = Время и язык
xdg-entry-users = Пользователи
xdg-entry-system-keywords = COSMIC;Система;Информация;Сведения;Учетные;Записи;Аккаунты;Прошивка;
xdg-entry-default-apps = Приложения по умолчанию
xdg-entry-wireless-keywords = COSMIC;WiFi;Wi-Fi;Вайфай;Сеть;Подключение;Соединение;
xdg-entry-applications-comment = Приложения по умолчанию, автозагрузка и совместимость приложений для X11
xdg-entry-about-keywords = COSMIC;О:Системе;Сведения
xdg-entry-panel = Панель
xdg-entry-notifications = Уведомления
xdg-entry-a11y-keywords = COSMIC;Доступность;A11y;Экранный;Диктор;Лупа;Контраст;Цвет;Специальные;Возможности;
xdg-entry-wireless-comment = Wi-Fi-соединения и профили
xdg-entry-desktop = Рабочий стол
xdg-entry-date-time-keywords = COSMIC;Время;Часовой;Пояс;Зона;
xdg-entry-desktop-comment = Фон, оформление, панель, док, управление окнами и параметры рабочих столов
xdg-entry-wallpaper = Фон
xdg-entry-keyboard = Клавиатура
xdg-entry-a11y-comment = Экранный диктор, лупа, высокая контрастность и цветовые фильтры
xdg-entry-displays = Экраны
xdg-entry-network = Сеть и Wi-Fi
xdg-entry-date-time = Дата и время
xdg-entry-users-comment = Аутентификация и уч. записи пользователей
xdg-entry-input-comment = Параметры клавиатуры и мыши
xdg-entry-bluetooth-keywords = COSMIC;Bluetooth;Блютуз;
xdg-entry-dock-comment = Дополнительная панель для приложений и апплетов
xdg-entry-network-keywords = COSMIC;Сеть;Беспроводная;Связь;WiFi;VPN;Вайфай;ВПН;
xdg-entry-wallpaper-comment = Фоновое изображение, цвета и параметры слайд-шоу
xdg-entry-dock = Док
xdg-entry-startup-apps = Автозапуск приложений
xdg-entry-wireless = Wi-Fi
xdg-entry-workspaces-keywords = COSMIC;Рабочий;Стол;Ориентация;Обзор;Монитор;
xdg-entry-system-comment = Сведения о системе, учётные записи и обновления прошивки
xdg-entry-x11-applications-comment = Масштабирование приложений X11, основной дисплей и глобальные сочетания клавиш
xdg-entry-region-language-comment = Форматирование дат, времени и чисел в соответствии с вашим регионом
xdg-entry-wallpaper-keywords = COSMIC;Обои;Фон;Слайд-шоу;
xdg-entry-users-keywords = COSMIC;Пользователь;Учетная;Учётная;Запись;Аккаунт;
xdg-entry-about-comment = Имя устройства, информация об оборудовании, параметры операционной системы по умолчанию
xdg-entry-vpn-keywords = COSMIC;VPN;ВПН;Сеть;Подключение;OpenVPN;OpenConnect;
xdg-entry-appearance-comment = Акцентные цвета и оформление
xdg-entry-time-language-keywords = COSMIC;Система;Время;Дата;Регион;Язык;
xdg-entry-input-keywords = COSMIC;Ввод;Клавиатура;Мышь;Мышка;
xdg-entry-comment = Приложение параметров для рабочей среды COSMIC
xdg-entry-notifications-comment = Режим «Не беспокоить», уведомления на экране блокировки и параметры по приложениям
xdg-entry-applications = Приложения
xdg-entry-sound-keywords = COSMIC;Звук;Аудио;Оповещение;Pipewire;
xdg-entry-power-keywords = COSMIC;Питание;Аккумулятор;Батарея
xdg-entry-default-apps-keywords = COSMIC;По;умолчанию;Приложения
xdg-entry-keywords = COSMIC;Параметры;Настройки;
xdg-entry-region-language-keywords = COSMIC;Регион;Язык;Дата;Формат;Время;Языковая;Локаль;Локализация;
xdg-entry-about = Сведения
xdg-entry-input = Устройства ввода
xdg-entry-applications-keywords = COSMIC;По;умолчанию;Приложения;Запуск;X11;Иксы;Совместимость;Автозапуск;Автозагрузка
xdg-entry-sound-comment = Настройки звука для устройств, оповещений и приложений
xdg-entry-touchpad-comment = Скорость сенсорной панели, параметры нажатия и жесты
xdg-entry-keyboard-keywords = COSMIC;Клавиатура;Ввод;Источник;Горячие;Клавиши;Клавиш;Сочетания;Раскладки;
xdg-entry-vpn-comment = VPN-соединения и профили
xdg-entry-appearance = Внешний вид
xdg-entry-window-management-keywords = COSMIC;Окно;Управление;Размещение;Тайлинг;Супер;Super;Клавиша;
xdg-entry-workspaces-comment = Ориентация рабочих столов, режим Обзора и поведение на нескольких мониторах
xdg-entry-network-comment = Управление сетевыми подключениями
xdg-entry-panel-comment = Основная системная панель для меню и апплетов
xdg-entry-sound = Звук
xdg-entry-power-comment = Режимы питания и параметры энергосбережения
xdg-entry-displays-keywords = COSMIC;Экран;Монитор;Дисплей;
xdg-entry-workspaces = Рабочие столы
xdg-entry-date-time-comment = Часовой пояс, автонастройка часов и форматирование времени
xdg-entry-x11-applications = Совместимость с приложениями для X11
xdg-entry-window-management-comment = Действие кнопки Super, параметры размещения и управления окнами
xdg-entry-wired-comment = Проводные соединения и профили
