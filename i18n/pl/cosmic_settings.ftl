app = Ustawienia COSMIC

unknown = Nieznany

number = { $number }

## Desktop

desktop = Pulpit

## Desktop: Appearance

appearance = Wygląd
    .desc = Kolory akcentów i motywy COSMIC.

accent-color = Kolory akcentów
app-background = Tło aplikacji oraz okien
auto = Automatyczne
close = Zamknij
color-picker = Wybór koloru
copied-to-clipboard = Skopiowane do schowka
copy-to-clipboard = Skopiuj do schowka
dark = Ciemny
export = Eksport
hex = Hex
import = Import
light = Jasny
mode-and-colors = Tryb i Kolor
recent-colors = Ostatnie kolory
reset-to-default = Resetuj do ustawień domyślnych
rgb = RGB
window-hint-accent = Kolor wyróżnienia aktywnego okna
window-hint-accent-toggle = Użycie koloru z motywu jako wyróżnienia aktywnego okna

auto-switch = Automatycznie zmieniaj z trybu Jasnego na Ciemny
    .sunrise = Zmienia na Jasny tryb o świcie
    .sunset = Zmienia na Jasny tryb o zmierzchu
    .next-sunrise = Zmienia na Jasny tryb podczas następnego świtu
    .next-sunset = Zmienia na Jasny tryb podczas następnego zmierzchu

container-background = Tło kontenera
    .desc-detail = Kolor tła kontenera jest używany do nawigacji panelem bocznym, bocznym szkicownikiem, dialogami i podobnymi widżetami. Domyślnie wywodzi się on z Aplikacji lub tła okna.
    .reset = Resetuj do automatycznych
    .desc = Główny kolor kontenera jest używany do bocznego panelu nawigacji, bocznego szkicownika, dialogów i podobnych widżetów.

control-tint = Sterowanie odcieniami komponentów
    .desc = Używany do tła standardowych przycisków, wprowadzania wyszukiwania, wprowadzania tekstu i podobnych komponentów.

frosted = Efekt zmrożonego szkła na interfejsie systemowym
    .desc = Nakłada efekt rozmycia na panel, dok, aplety, program startowy oraz bibliotekę aplikacji.
    
enable-export = Użyj tego motywu do apek GNOME.
    .desc = Nie wszystkie toolkity wspierają automatyczne zmiany. Apki inne niż COSMIC mogą wymagać restartu do zmiany motywu.
    
icon-theme = Motyw ikon
    .desc = Zastosuj inny zbiór ikon do aplikacji.
    
text-tint = Odcień tekstu interfejsu
    .desc = Kolor używany do uzyskania odcienia tekstu interfejsu, który ma wystarczający kontrast na różnych powierzchniach.

style = Styl
    .round = Okrągły
    .slightly-round = Lekko zaokrąglony
    .square = Kwadratowy

# interface density left out for now
window-management = Zarządzanie Oknami
    .active-hint = Rozmiar wyróżnienia aktywnego okna
    .gaps = Przerwa między ramkami okien w trybie kafelków

## Desktop: Display

-requires-restart = Wymagany restart

color = Kolor
    .depth = Głębia koloru
    .profile = Profil koloru
    .sidebar = Profile Koloru
    .temperature = Temperatura koloru

display = Wyświetlacz
    .desc = Zarządzaj wyświetlaczami, zmianami karty graficznej i nocnym światłem
    .arrangement = Układ wyświetlaczy
    .arrangement-desc = Przeciągaj wyświetlacze by zmienić układ.
    .enable = Włącz wyświetlacz
    .external = { $size } { $output } Zewnętrzny Wyświetlacz
    .laptop = { $size } Wyświetlacz Laptopa
    .options = Opcje Wyświetlacza
    .refresh-rate = Prędkość odświeżania ekranu
    .resolution = Rozdzielczość
    .scale = Skala

graphics-mode = Tryb Graficzny
    .mode = { $mode ->
        [compute] Obliczeniowy
        *[hybrid] Hybrydowy
        [integrated] Zintegrowany
        [nvidia] NVIDIA
    } graphics
    .enable = Włącz { $mode ->
        [compute] Obliczeniowy
        *[hybrid] Hybrydowy
        [integrated] Zintegrowany
        [nvidia] NVIDIA
    } graphics
    .desc = { $mode ->
        [compute] Używa dedykowanej karty graficznej tylko do pracy obliczeniowej. Wyłącza zewnętrzne ekrany. { -requires-restart }.
        *[hybrid] Aplikacje używają zintegrowanej karty graficznej, chyba że bezpośredio kazano im używać dedykowanej karty graficznej. { -requires-restart }.
        [integrated] Wyłącz dedykowaną kartę graficzną by zwiększyć czas pracy na baterii i zmniejszyć głośność wentylatorów.
        [nvidia] Najwydajniejszy tryb graficzny wiążący się z najwyższym zapotrzebowaniem na prąd. { -requires-restart }.
    }
    .restart = Uruchomić ponownie i zmienić na { $mode }?
    .restart-desc = Zmiana na { $mode } zamknie wszystkie otwarte aplikacje

mirroring = Lustrzane Odbicie
    .id = Lustrzane Odbicie { $id }
    .dont = Nie stosuj Lustrzanego Odbicia
    .mirror = Zastosuj Lustrzene Odbicie { $display }
    .project = Rzutuj na { $display ->
        [all] wszystkie wyświetlacze
        *[other] { $display }
    }
    .project-count = Rzutuj na { $count} { $count ->
        [1] inny
        [Few] inne
        *[other] innych
    } { $count ->
        [1] wyświetlacz
        [Few] wyświetlacze
        *[other] wyświetlaczy
    }

night-light = Nocne Światło
    .auto = Automatyczne (od świtu do zmierzchu)
    .desc = Zmniejsza ilość niebieskiego światła i ociepla kolory.

orientation = Kierunek
    .standard = Standardowy
    .rotate-90 = Obróć o 90
    .rotate-180 = Obróć o 180
    .rotate-270 = Obróć o 270

scheduling = Harmonogram
    .manual = Ręcznie ustawiony harmonogram

## Desktop: Notifications

notifications = Powiadomienia
    .desc = Nie przeszkadzać, powiadomienia ekranu blokady oraz ustawienia konkretnej aplikacji.

## Desktop: Options

desktop-panel-options = Pulpit i Panel
    .desc = Akcje Klawisza Super, narożnik funkcyjny, opcje sterowania oknem.

desktop-panels-and-applets = Panele Pulpitu i Aplety

dock = Dok
    .desc = Panel z przypiętymi aplikacjami.

hot-corner = Narożniki Funkcyjne
    .top-left-corner = Włącz Obszary Robocze w lewym górnym narożniku funkcyjnym.

super-key-action = Akcje Klawisza Super
    .launcher = Program Startowy
    .workspaces = Obszary Robocze
    .applications = Aplikacje

top-panel = Górny Panel
    .workspaces = Pokaż Przycisk Obszarów Roboczych
    .applications = Pokaż Przycisk Aplikacji

window-controls = Sterowanie Oknem
    .minimize = Pokaż Przycisk Minimalizacji
    .maximize = Pokaż Przycisk Maksymalizacji

## Desktop: Panel

panel = Panel
    .desc = Górna belka ze sterowaniem pulpitem i menu.

add = Dodaj
add-applet = Dodaj Aplet
all = Wszystkie
applets = Aplety
center-segment = Człon Środkowy
drop-here = Tutaj upuść aplety
end-segment = Człon Końcowy
large = Duży
no-applets-found = Nie znaleziono apletów...
panel-bottom = Dolny
panel-left = Lewy
panel-right = Prawy
panel-top = Górny
search-applets = Wyszukaj aplety...
small = Mały
start-segment = Człon Początkowy

panel-appearance = Wygląd
    .match = Dopasuj do Pulpitu
    .light = Jasny
    .dark = Ciemny

panel-behavior-and-position = Funkcjonowanie i Pozycja
    .autohide = Automatycznie chowaj panel
    .dock-autohide = Automatycznie chowaj panel
    .position = Pozycja na ekranie
    .display = Pokaż na wyświetlaczu

panel-style = Styl
    .anchor-gap = Przerwa pomiędzy panelem a krawędziami ekranu
    .dock-anchor-gap = Przerwa pomiędzy dokiem a krawędziami ekranu
    .extend = Rozszerz panel do krawędzi ekranu
    .dock-extend = Rozszerz dok do krawędzi ekranu
    .appearance = Wygląd
    .size = Rozmiar
    .background-opacity = Przejrzystość Tła

panel-applets = Konfiguracja
    .dock-desc = Konfiguracja apletów doku.
    .desc = Konfiguracja apletów panelu.

panel-missing = Brakuje Konfiguracji Panelu
    .desc = Brakuje pliku konfiguracji panelu, albo z powodu użycia spersonalizowanej konfiguracji, albo plik jest uszkodzony.
    .fix = Przywróć ustawienia domyślne

## Desktop: Wallpaper

wallpaper = Tapeta
    .change = Zmieniaj obraz co
    .desc = Obraz tapety, kolory i opcje pokazu slajdów.
    .fit = Dopasuj tapetę
    .folder-dialog = Wybierz katalog z tapetami
    .image-dialog = Wybierz obraz tapety
    .plural = Tapety
    .same = Ta sama tapeta na wszystkich wyświetlaczach
    .slide = Pokaz Slajdów

add-color = Dodaj kolor
add-image = Dodaj obraz
all-displays = Wszystkie Wyświetlacze
colors = Kolory
dialog-add = _Add
fit-to-screen = Dopasuj do rozmiaru ekranu
open-new-folder = Otwórz nowy katalog
recent-folders = Ostatnie Katalogi

x-minutes = { $number ->
    [1] 1 minuta
    [few] { $number } minuty
    *[other] { $number } minut
}
x-hours = { $number ->
    [1] 1 godzina
    [few] { $number } godziny
    *[other] { $number } godzin
}

## Desktop: Workspaces

workspaces = Obszary Robocze
    .desc = Ustaw numer, zachowanie i rozmieszczenie obszaru roboczego.

workspaces-behavior = Funkcjonowanie Obszaru Roboczego
    .dynamic = Dynamiczne obszary robocze
    .dynamic-desc = Puste obszary robocze są automatycznie usuwane.
    .fixed = Stała liczba obszarów roboczych
    .fixed-desc = Dodaj lub usuń obszar roboczy w podglądzie.

workspaces-multi-behavior = Funkcjonowanie Przy Wielu Monitorach
    .span = Obszary Robocze Wspólne Dla Wyświetlaczy
    .separate = Wyświetlacze Mają Osobne Obszary Robocze

workspaces-overview-thumbnails = Miniatury Podglądu Obszaru Roboczego
    .show-number = Pokaż Numer Obszaru Roboczego
    .show-name = Pokaż Nazwę Obszaru Roboczego

workspaces-orientation = Kierunek Obszarów Roboczych
    .vertical = Pionowy
    .horizontal = Poziomy

## Networking: Wired

wired = Przewodowe
    .desc = Połączenie przewodowe, profil połączenia

## Networking: Online Accounts

online-accounts = Konta Online
    .desc = Dodaj konta, IMAP, SMTP i logowanie firmowe

## Time & Language

time = Czas i Język
    .desc = N/A

time-date = Czas i Data
    .desc = Strefa czasowa, automatyczne ustawienia zegara oraz formatowanie czasu.
    .auto = Ustaw automatycznie

time-zone = Strefa Czasowa
    .auto = Automatyczna strefa czasowa
    .auto-info = Wymaga usług lokalizacji oraz połączenia internetowego

time-format = Format Daty i Czasu
    .twenty-four = Czas 24-godzinny
    .first = Pierwszy dzień tygodnia
    .show-date = Pokaż Datę w Górnym Panelu
    .show-day-name = Pokaż nazwe dnia w górnym panelu
    .friday = Piątek
    .saturday = Sobota
    .sunday = Niedziela
    .monday = Poniedziałek

time-region = Region i Język
    .desc = Format dat, czasu i numerów na podstawie wybranego regionu

## Sound

sound = Dźwięk
    .desc = N/A

sound-output = Wyjście
    .volume = Głośność wyjścia
    .device = Urządzenie wyjścia
    .level = Poziom wyjścia
    .config = Konfiguracja
    .balance = Balans

sound-input = Wejście
    .volume = Głośność wejścia
    .device = Urządzenie wejścia
    .level = Poziom wejścia

sound-alerts = Alarmy
    .volume = Głośność alarmów
    .sound = Dźwięk alarmów

sound-applications = Aplikacje
    .desc = Ustawienia i głośność aplikacji

## System

system = System i Konta

## System: About

about = O Systemie
    .desc = Nazwa urządzenia, informacje o sprzęcie, domyślne ustawienia systemu.

about-device = Nazwa urządzenia
    .desc = Ta nazwa wyświetla się innym w sieci i na urządzeniach bluetooth.

about-hardware = Sprzęt
    .model = Model Sprzętu
    .memory = Pamięć
    .processor = Procesor
    .graphics = Grafika
    .disk-capacity = Pojemność Dysku

about-os = System Operacyjny
    .os = System Operacyjny
    .os-architecture = Architektura Systemu Operacyjnego
    .desktop-environment = Środowisko Graficzne
    .windowing-system = System Okien

about-related = Pokrewne Ustawienia
    .support = Uzyskaj Wsparcie

## System: Firmware

firmware = Mikrooprogramowanie
    .desc = Szczegóły Mikrooprogramowania

## System: Users

users = Użytkownicy
    .desc = Uwierzytelnianie, logowanie i ekran blokady.

## Input

acceleration-desc = Automatycznie dostosuj dokładność śledzenia do prędkości ruchu.

disable-while-typing = Wyłącz podczas pisania

input = Urządzenia do Wprowadzania Danych
    .desc = Urządzenia do Wprowadzania Danych
    
primary-button = Główny przycisk
    .left = Lewy
    .right = Prawy
    
scrolling = Przewijanie
    .two-finger = Przewijaj dwoma palcami
    .edge = Przewijaj jednym palcem przy krawędzi
    .speed = Prędkość przewijania
    .natural = Naturalne przewijanie
    .natural-desc = Przewijaj zawartość, nie widok

## Input: Keyboard

keyboard = Klawiatura
    .desc = Wprowadzanie danych na klawiaturze

keyboard-sources = Żródła Wprowadzania Danych
    .desc = Żródła Wprowadzania Danych można zmienić naciskając kombinację klawiszy Super i Spację. Może być to spersonalizowane w ustawieniach skrótów klawiaturowych.
    .move-up = Przesuń w górę
    .move-down = Przesuń w dół
    .settings = Ustawienia
    .view-layout = Pokaż układ klawiatury
    .remove = Usuń
    .add = Dodaj źródło przewijania

keyboard-special-char = Wpis Znaków Specjalnych
    .alternate = Alternatywne klawisze znaków
    .compose = Ustaw klawisz
    
added = Dodany
type-to-search = Naciśnij by wyszukać...

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Skróty Klawiaturowe
    .desc = Obejrzyj i spersonalizuj skróty

## Input: Mouse

mouse = Myszka
    .desc = Prędkość, przyśpieszenie i naturalne przewijanie myszki
    .speed = Prędkość myszki
    .acceleration = Włącz przyśpieszenie myszki

## Input: Touchpad

click-behavior = Funkcjonowanie Kliknięć
    .click-finger = Drugi przycisk uzyskujemy kliknięciem dwoma palcami, a środkowy przycisk trzema
    .button-areas = Drugi przycisk uzyskujemy kliknięciem w prawy dolny róg, a środkowy przycisk kliknięciem w środek dołu
    
pinch-to-zoom = Uszczypnij by przybliżyć
    .desc = Zbliż dwa palce do siebie by przybliżyć zawartość, w aplikacjach używających przybliżenia.
    
tap-to-click = Tap to click
    .desc = Enables single-finger tap for primary click, two-finger tap for secondary click, and three-finger tap for middle click.

touchpad = Gładzik
    .acceleration = Włącz przyśpieszenie gładzika
    .desc = Prędkość, opcje klikania i gesty gładzikiem.
    .speed = Prędkość Gładzika
    
## Input: Gestures

swiping = Przesuwanie
    .four-finger-down = Przesunięcie czterema palcami w dół
    .four-finger-left = Przesunięcie czterema palcami w lewo
    .four-finger-right = Przesunięcie czterema palcami w prawo
    .four-finger-up = Przesunięcie czterema palcami w górę
    .three-finger-any = Przesunięcie trzema palcami w dowolnym kierunku 

switch-between-windows = Przełączenie pomiędzy oknami
switch-to-next-workspace = Przełączenie do następnego obszaru roboczego
switch-to-prev-workspace = Przełączenie do poprzedniego obszaru roboczego
open-application-library = Otwarcie biblioteki aplikacji
open-workspaces-view = Otwarcie podglądu obszarów roboczych
