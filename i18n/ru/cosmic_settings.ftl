app = Параметры COSMIC

unknown = Неизвестно

number = { $number }

## Desktop

desktop = Рабочий стол

## Desktop: Appearance

appearance = Внешний вид
    .desc = Акцентные цвета и оформление COSMIC.

import = Импорт
export = Экспорт

mode-and-colors = Цвета и оформление
auto-switch = Автоматически сменять оформление
    .desc = Сменяет оформление на Тёмное при закате и на Светлое при восходе солнца
accent-color = Акцентный цвет
app-background = Фон приложений или окон
auto = Автоматически
close = Закрыть
container-background = Фон контейнера
    .desc-detail = Цвет фона контейнера используется для боковой панели навигации, бокового меню, диалоговых окон и других подобных виджетов. По умолчанию он автоматически определяется на основе фона приложения или окна.
    .reset = Вернуть на автоматически
    .desc = Цвет фона контейнера используется для боковой панели навигации, бокового меню, диалоговых окон и других подобных виджетов.
text-tint = Оттенок текста интерфейса
    .desc = Цвет, используемый для выведения цветов текста интерфейса, обладающих достаточной контрастностью на различных поверхностях.
control-tint = Оттенок компонентов управления
    .desc = Используется для фонов стандартных кнопок, полей ввода текста и других подобных компонентов.
window-hint-accent-toggle = Использовать акцентный цвет в качестве подсветки активного окна
window-hint-accent = Цвет подсветки активного окна
dark = Тёмное
light = Светлое
color-picker = Выбор цвета
hex = Hex
rgb = RGB
recent-colors = Недавние цвета
reset-to-default = Вернуть по умолчанию
copy-to-clipboard = Копировать в буфер обмена
copied-to-clipboard = Скопировано в буфер обмена

style = Стиль
    .round = Округлый
    .slightly-round = Слегка округлый
    .square = Прямой
frosted = Эффект матового стекла на интерфейсе системы
    .desc = Размытие фона для Верхней панели, Дока, Апплетов, Панели запуска и библиотеки приложений.

reset-default = Вернуть по умолчанию
# interface density left out for now

window-management = Управление окнами
    .active-hint = Размер подсветки активного окна
    .gaps = Зазоры вокруг размещённых окон
## Desktop: Notifications

notifications = Уведомления
    .desc = Режим «Не беспокоить», уведомления на экране блокировки, настройка для приложений.


## Desktop: Options

desktop-panel-options = Рабочий стол и панель
    .desc = Действие кнопки Super, горячие углы, настройки управления окнами.

super-key-action = Действие кнопки Super
    .launcher = Панель запуска
    .workspaces = Рабочие места
    .applications = Приложения

hot-corner = Активные углы
    .top-left-corner = Открывать рабочие места при наведении в левый верхний угол

top-panel = Верхняя панель
    .workspaces = Отображать кнопку «Рабочие места»
    .applications = Отображать кнопку «Приложения»

window-controls = Кнопки управления окнами
    .minimize = Отображать кнопку «Свернуть»
    .maximize = Отображать кнопку «Развернуть»

desktop-panels-and-applets = Панели рабочего стола и апплеты


dock = Док
    .desc = Панель с закреплёнными приложениями.

## Desktop: Panel
panel = Верхняя панель
    .desc = Верхняя панель с кнопками управления рабочим столом и меню.

panel-behavior-and-position = Поведение и расположение
    .autohide = Автоматически скрывать панель
    .dock-autohide = Автоматически скрывать док
    .position = Расположение на экране
    .display = Отображать на экране

panel-top = Вверху
panel-bottom = Внизу
panel-left = Слева
panel-right = Справа

panel-appearance = Внешний вид
    .match = Как в системе
    .light = Светлый
    .dark = Тёмный

panel-style = Оформление
    .anchor-gap = Пробел между панелью и краями экрана
    .dock-anchor-gap = Пробел между доком и краями экрана
    .extend = Расширить панель до краёв экрана
    .dock-extend = Расширить док до краёв экрана
    .appearance = Внешний вид
    .size = Размер
    .background-opacity = Непрозрачность фона

small = Маленький
large = Большой

panel-applets = Конфигурация
    .dock-desc = Настройка апплетов дока.
    .desc = Настройка апплетов панели.

panel-missing = Отсутствует конфигурация панели
    .desc = Файл конфигурации панели повреждён или отсутствует в связи с использованием нестандартной конфигурации.
    .fix = Восстановить конфигурацию по умолчанию

applets = Апплеты
start-segment = Начальный сегмент
center-segment = Центральный сегмент
end-segment = Конечный сегмент

add = Добавить
add-applet = Добавить апплет
search-applets = Поиск апплетов...
no-applets-found = Апплеты не найдены...
all = Все

drop-here = Перетащите апплеты сюда

## Desktop: Wallpaper

wallpaper = Обои
    .desc = Фоновые изображения, цвета и параметры слайд-шоу.
    .same = Одинаковый фон на всех экранах
    .fit = Подгонять фон
    .slide = Слайд-шоу
    .change = Менять изображение каждые

all-displays = Все экраны
colors = Цвета
fit-to-screen = Подгонять под экран
stretch = Растянуть
system-backgrounds = Системные фоны
zoom = Увеличить

x-minutes = { $number } минут
x-hours = { $number ->
    [1] 60 минут
    *[other] { $number } часов
}

## Desktop: Workspaces

workspaces = Рабочие места
    .desc = Настроить порядок, поведение и расположение рабочих мест.

workspaces-behavior = Поведение рабочих мест
    .dynamic = Динамические рабочие места
    .fixed = Фиксированное число рабочих мест

workspaces-multi-behavior = Поведение для нескольких мониторов
    .span = Рабочие места охватывают все мониторы
    .separate = Отдельные рабочие места для каждого монитора

## Networking: Wired

wired = Проводная сеть
    .desc = Проводное подключение, профили подключения

## Networking: Online Accounts

online-accounts = Онлайн-аккаунты
    .desc = Добавление учётных записей, IMAP и SMTP, корпоративных логинов

## Time & Language

time = Время и язык
    .desc = Н/Д

time-date = Дата и время
    .desc = Часовой пояс, параметры автоматической настройки и форматирования времени.
    .auto = Устанавливать автоматически

time-zone = Часовой пояс
    .auto = Устанавливать автоматически
    .auto-info = Требуются службы определения местоположения и доступ в Интернет

time-format = Формат даты и времени
    .twenty-four = 24-часовой формат
    .first = Первый день недели

time-region = Регион и язык
    .desc = Форматирование даты, времени и чисел в зависимости от региона

## Sound

sound = Звук
    .desc = Н/Д

sound-output = Выход
    .volume = Выходная громкость
    .device = Выходное устройство
    .level = Выходной уровень
    .config = Конфигурация
    .balance = Баланс

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
    .desc = Имя устройства, информация об оборудовании, настройки ОС по умолчанию.

about-device = Имя устройства
    .desc = Это имя видно для других устройств по сети или Bluetooth.

about-hardware = Оборудование
    .model = Модель оборудования
    .memory = Память
    .processor = Процессор
    .graphics = Графика
    .disk-capacity = Ёмкость диска

about-os = Операционная система
    .os = Операционная система
    .os-architecture = Архитектура ОС
    .desktop-environment = Среда рабочего стола
    .windowing-system = Оконная система

about-related = Связанные настройки
    .support = Получить поддержку

## System: Firmware

firmware = Прошивка
    .desc = Сведения о прошивке.

## System: Users

users = Пользователи
    .desc = Аутентификация и вход в систему, экран блокировки.

## Input

input = Ввод
    .desc = Ввод

## Input: Keyboard

keyboard = Клавиатура
    .desc = Ввод с клавиатуры

keyboard-sources = Источники ввода
    .desc = Источники ввода можно переключать по комбинации клавиш Super+Space. Это поведение можно изменить в настройках сочетаний клавиш.
    .move-up = Переместить вверх
    .move-down = Переместить вниз
    .settings = Настройки
    .view-layout = Просмотреть раскладку
    .remove = Удалить

keyboard-special-char = Ввод специальных символов
    .alternate = Клавиша альтернативных символов
    .compose = Клавиша Compose

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Сочетания клавиш
    .desc = Просмотр и настройка сочетаний клавиш

## Input: Mouse
mouse = Мышь
    .desc = Скорость и ускорение мыши, естественная прокрутка.
    .primary-button = Основная кнопка
    .primary-button-left = Левая
    .primary-button-right = Правая
    .speed = Скорость мыши
    .acceleration = Включить ускорение мыши
    .acceleration-desc = Автоматически регулирует чувствительность мыши в зависимости от скорости.
    .double-click-speed = Скорость двойного нажатия
    .double-click-speed-desc = Изменить скорость регистрации двойных нажатий.

mouse-scrolling = Прокрутка
    .speed = Скорость прокрутки
    .natural = Естественная прокрутка
    .natural-desc = Прокручивать содержимое, а не представление

## Input: Touchpad

touchpad = Сенсорная панель
    .desc = Скорость сенсорной панели, параметры нажатия, жесты.
    .primary-button = Основная кнопка
    .primary-button-left = Левая
    .primary-button-right = Правая
    .speed = Скорость сенсорной панели
    .acceleration = Включить ускорение сенсорной панели
    .acceleration-desc = Автоматически регулирует чувствительность панели в зависимости от скорости.
    .double-click-speed = Скорость двойного нажатия
    .double-click-speed-desc = Изменить скорость регистрации двойных нажатий.