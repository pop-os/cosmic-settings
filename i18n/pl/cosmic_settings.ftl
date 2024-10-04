app = Ustawienia COSMIC

unknown = Nieznane

number = { $number }

## Networking: Wired

connections-and-profiles = { $variant ->
    [wired] Przewodowe
    [wifi] Wi-Fi
    [vpn] VPN
    *[other] Nieznane
} połączenia i profile połączeń.

add-network = Dodaj sieć
    .profile = Dodaj profil
add-vpn = Dodaj VPN
airplane-on = Tryb samolotowy jest włączony.
cable-unplugged = Rozłączono połączenie przewodowe
connect = Połącz
connected = Połączono
connecting = Łączę…
disconnect = Rozłącz
forget = Zapomnij
known-networks = Znane Sieci
network-and-wireless = Sieci i Połączenia Bezprzewodowe
no-networks = Nie znaleziono żadnej sieci.
no-vpn = Brak dostępnych połączeń VPN.
password = Hasło
remove = Usuń
settings = Ustawienia
username = Użytkownik
visible-networks = Widoczne Sieci

auth-dialog = Wymagane Uwierzytelnienie
    .vpn-description = Wprowadź nazwę i hasło wymagane przez usługę VPN.
    .wifi-description = Wprowadź hasło lub klucz odszyfrowania lub połącz się wciskając przycisk "WPS" na routerze.

forget-dialog = Chcesz zapomnieć hasło tej sieci Wi-Fi?
    .description = Będziesz musiał ponownie wprowadzać hasło do tej sieci Wi-Fi, by jej użyć.

network-device-state =
    .activated = Połączono
    .config = Konfiguracja
    .deactivating = Rozłączanie
    .disconnected = Rozłączono
    .failed = Nieudane połączenie
    .ip-check = Sprawdzam połączenie
    .ip-config = Wymagane IP i informacje o routingu
    .need-auth = Wymagane uwierzytelnienie
    .prepare = Przygotowuję do połączenia
    .secondaries = Oczekuję na dodatkowe połączenie
    .unavailable = Niedostępne
    .unknown = Nieznany stan
    .unmanaged = Niezarządzane
    .unplugged = Wypięty kabel

remove-connection-dialog = Usunąć Profil Połączenia?
    .vpn-description = Będziesz musiał ponownie wprowadzać hasło do tej sieci, by jej użyć.
    .wired-description = Będziesz musiał odtworzyć ten profil, by jej użyć.

vpn = VPN
    .connections = Połączenia VPN
    .remove = Usuń profil połączenia
    .select-file = Wybierz plik konfiguracyjny VPN

wired = Przewodowa
    .adapter = Adapter przewodowy { $id }
    .connections = Połączenia Przewodowe
    .devices = Urządzenia Przewodowe
    .remove = Usuń profil połączenia

wifi = Wi-Fi
    .adapter = Adapter Wi-Fi { $id }
    .forget = Zapomnij tą sieć

## Networking: Online Accounts

online-accounts = Konta Online
    .desc = Dodaj konta, IMAP, SMTP i logowanie firmowe

# Bluetooth

confirm = Potwierdź

bluetooth = Bluetooth
    .desc = Zarządzaj urządzeniami Bluetooth
    .status = Ten system jest widoczny jako { $aliases }, dla urządzeń Bluetooth.
    .connected = Połączono
    .connecting = Łączenie
    .disconnecting = Rozłączanie
    .connect = Połącz
    .disconnect = Rozłącz
    .forget = Zapomnij
    .dbus-error = Wystąpił błąd podczas interakcji z DBus: { $why }
    .show-device-without-name = Pokaż urządzenia bez nazw

bluetooth-paired = Poprzednio Połączone Urządzenia
    .connect = Połącz
    .battery = { $percentage }% baterii

bluetooth-confirm-pin = Potwierdź PINem Bluetooth
    .description = Upewnij się, że PIN jest taki sam jak na urządzeniu { $device }

bluetooth-available = Pobliskie Urządzenia

bluetooth-adapters = Adaptery Bluetooth

## Desktop

desktop = Pulpit

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
dialog-add = Dodaj
fill = Wypełnij
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
    .sunset = Zmienia na Ciemny tryb o zmierzchu
    .next-sunrise = Zmienia na Jasny tryb podczas następnego świtu
    .next-sunset = Zmienia na Ciemny tryb podczas następnego zmierzchu

container-background = Tło kontenera
    .desc-detail = Kolor tła kontenera jest używany do nawigacji panelem bocznym, bocznym szkicownikiem, dialogami i podobnymi widżetami. Domyślnie wywodzi się on z Aplikacji lub tła okna.
    .reset = Resetuj do automatycznych
    .desc = Główny kolor kontenera jest używany do bocznego panelu nawigacji, bocznego szkicownika, dialogów i podobnych widżetów.

control-tint = Sterowanie odcieniami komponentów
    .desc = Używany do tła standardowych przycisków, wprowadzania wyszukiwania, wprowadzania tekstu i podobnych komponentów.

frosted = Efekt zmrożonego szkła na interfejsie systemowym
    .desc = Nakłada efekt rozmycia na panel, dok, aplety, program startowy oraz bibliotekę aplikacji.

experimental-settings = Ustawienia eksperymentalne

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

interface-density = Zagęszczenie Interfejsu
    .comfortable = Wygodne
    .compact = Zwarte
    .spacious = Swobodne

window-management-appearance = Zarządzanie Oknami
    .active-hint = Rozmiar wyróżnienia aktywnego okna
    .gaps = Przerwa między ramkami okien w trybie kafelków

## Desktop: Notifications

notifications = Powiadomienia
    .desc = Nie przeszkadzać, powiadomienia ekranu blokady oraz ustawienia konkretnej aplikacji.

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

## Desktop: Dock

dock = Dok
    .desc = Panel z przypiętymi aplikacjami.

## Desktop: Window management

window-management = Zarządzanie oknami
    .desc = Akcje klawisza super, ustawienia kontroli okien i dodatkowe ustawienia kafelkowania okien.

super-key = Klawisz Super
    .launcher = Otwórz Program Startowy
    .workspaces = Otwórz Obszary Robocze
    .applications = Otwórz Aplikacje
    .disable = Wyłącz

window-controls = Sterowanie Oknem
    .maximize = Pokaż Przycisk Maksymalizacji
    .minimize = Pokaż Przycisk Minimalizacji

focus-navigation = Nawigacja Aktywnym Oknem
    .focus-follows-cursor = Aktywuje okno nad kursorem
    .cursor-follows-focus = Przenosi kursor nad aktywne okno

## Desktop: Workspaces

workspaces = Obszary Robocze
    .desc = Zachowanie i kierunek obszaru roboczego.

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

hot-corner = Hot Corner
    .top-left-corner = Włącz lewy górny narożnik funkcyjny w Obszarach Roboczych

## Displays

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

dialog = Dialog
    .title = Czy Zachować Te Ustawienia Wyświetlacza?
    .keep-changes = Zachowaj Zmiany
    .change-prompt = Ustawienia automatycznie powrócą do poprzednich za { $time ->
        [1] sekundę.
        [few] {$time} sekundy.
        *[other] {$time} sekund.
    }
    .revert-settings = Powróć Do Poprzednich Ustawień

legacy-applications = Skalowanie Aplikacji z Systemu Okien X11
    .scaled-by-system = Skaluj wszystkie Aplikacje X11
    .system-description = Aplikacje X11 będą rozmyte na wyświetlaczach z wysokim DPI.
    .scaled-natively = Renderuj Aplikacje X11 w pierwotnej rozdzielczości.
    .native-description = Aplikacje X11 które nie wspierają skalowania będą małe na wyświetlaczach z wysokim DPI. Włącz do gier by wykorzystywały pełną rozdzielczość monitora.

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

profile = Profil

## Power

power = Zasilanie
  .desc = Zarządzaj ustawieniami zasilania

battery = Bateria
    .minute = { $value } { $value ->
        [one] minuta
        [few] minuty
       *[other] minut
  }
  .hour = { $value } { $value ->
        [one] godzina
        [few] godziny
       *[other] godzin
  }
  .day = { $value } { $value ->
        [one] dzień
       *[other] dni
  }
  .less-than-minute = Mniej niż minuta
  .and = i
  .remaining-time = { $time } do { $action ->
        [full] naładowania
       *[other] rozładowania
   }

connected-devices = Podłączone Urządzenia
  .unknown = Nierozpoznane urządzenie

power-mode = Profile Zasilania
  .performance = Tryb Wysokowydajny
  .balanced = Tryb Zbalansowany
  .battery = Tryb Oszczędzania Energii
  .performance-desc = Najwyższa wydajność i zwiększone zużycie energii.
  .balanced-desc = Standardowa wydajność i zużycie baterii.
  .battery-desc = Zmniejszone zużycie energii i zmniejszona wydajność.
  .no-backend = Nie znaleziono backendu. Zainstaluj system76-power lub power-profiles-daemon.

## Input

acceleration-desc = Automatycznie dostosuj dokładność śledzenia do prędkości ruchu.

disable-while-typing = Wyłącz podczas pisania

input-devices = Urządzenia Wprowadzania Danych
    .desc = Urządzenia Wprowadzania Danych

primary-button = Główny przycisk
    .desc = Ustawia kolejność fizycznych przycisków
    .left = Lewy
    .right = Prawy

scrolling = Przewijanie
    .two-finger = Przewijaj dwoma palcami
    .edge = Przewijaj jednym palcem przy krawędzi
    .speed = Prędkość przewijania
    .natural = Naturalne przewijanie
    .natural-desc = Przewijaj zawartość, nie widok

## Input: Keyboard

slow = Wolno
fast = Szybko
short = Któtko
long = Długo
keyboard = Klawiatura
    .desc = Wprowadzanie danych, zmienianie, wprowadzanie znaków specjalnych i skróty na klawiaturze.

keyboard-sources = Żródła Wprowadzania Danych
    .desc = Żródła Wprowadzania Danych można zmienić naciskając kombinację klawiszy Super i Spację. Może być to spersonalizowane w ustawieniach skrótów klawiaturowych.
    .move-up = Przesuń w górę
    .move-down = Przesuń w dół
    .settings = Ustawienia
    .view-layout = Pokaż układ klawiatury
    .remove = Usuń
    .add = Dodaj źródło wprowadzania danych

keyboard-special-char = Wpis Znaków Specjalnych
    .alternate = Alternatywne klawisze znaków
    .compose = Ustaw klawisz

keyboard-typing-assist = Pisanie
    .repeat-rate = Tempo powtarzania
    .repeat-delay = Opóżnienie powtarzania

added = Dodany
type-to-search = Naciśnij by wyszukać...

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Skróty Klawiaturowe
    .desc = Obejrzyj i spersonalizuj skróty

add-keybinding = Dodaj skrót klawiszowy
cancel = Anuluj
command = Komenda
custom = Własne
debug = Debuguj
disabled = Wyłączone
migrate-workspace-prev = Migruj obszar roboczy do następnego ekranu
migrate-workspace-next = Migruj obszar roboczy do poprzedniego ekranu
migrate-workspace = Migruj obszar roboczy do { $direction ->
    *[down] dolnego
    [left] lewego
    [right] prawego
    [up] górnego
} ekranu
navigate = Nawiguj
replace = Zastąp
shortcut-name = Nazwa skrótu klawiszowego
system-controls = Panel kontrolny systemu
terminate = Zakończ sesję
toggle-stacking = Przełącznik grupowania w kartach
type-key-combination = Wpisz kombinację klawiszy

custom-shortcuts = Własne skróty klawiszowe
    .add = Dodaj skrót klawiszowy
    .context = Dodaj Własny skrót klawiszowy
    .none = Nie masz własych skrótów klawiszowych

modified = { $count } zmodyfikowano

nav-shortcuts = Nawigacja
    .prev-output = Przełącz na poprzedni ekran
    .next-output = Przełącz na następny ekran
    .last-workspace = Przełącz na ostatni obszar roboczy
    .prev-workspace = Przełącz na poprzedni obszar roboczy
    .next-workspace = Przełącz na następny obszar roboczy
    .focus = Przełącz { $direction ->
        *[down] na dolne okno
        [in] na okno
        [left] na lewe okno
        [out] z okna
        [right] na prawe okno
        [up] na górne okno
    }
    .output = Zmień na  { $direction ->
        *[down] dolny
        [left] lewy
        [right] prawy
        [up] górny
    } ekran
    .workspace = Zmień na obszar roboczy { $num }

manage-windows = Zarządzanie oknami
    .close = Zamknij okno
    .maximize = Maksymalizuj okno
    .minimize = Minimalizuj okno
    .resize-inwards = Powiększ okno
    .resize-outwards = Powiększ okno
    .toggle-sticky = Przełącznik zakrywających okien

move-windows = Przemieszczanie okien
    .direction = Przemieść okno { $direction ->
        *[down] w dół
        [left] w lewo
        [right] na prawo
        [up] w górę
    }
    .display = Przenieś okno o jeden monitor { $direction ->
        *[down] w dół
        [left] w lewo
        [right] na prawo
        [up] w górę
    }
    .workspace = Przenieś okno o jeden obszar roboczy { $direction ->
        *[below] niżej
        [left] w lewo
        [right] w prawo
        [above] wyżej
    }
    .workspace-num = Przenieś okno do obszaru roboczego { $num }
    .prev-workspace = Przenieś okno do poprzedeniego obszaru roboczego
    .next-workspace = Przenieś okno do następnego obszaru roboczego
    .last-workspace = Przenieś okno do ostatniego obszaru roboczego
    .next-display = Przenieś okno do następnego wyświetlacza
    .prev-display = Przenieś okno do poprzedniego wyświetlacza
    .send-to-prev-workspace = Przenieś okno do poprzedeniego obszaru roboczego
    .send-to-next-workspace = Przenieś okno do następnego obszaru roboczego

system-shortcut = System
    .app-library = Otwórz bibliotekę apek
    .brightness-down = Zmniejsz jasność wyświetlacza
    .brightness-up = Zwiększ jasność wyświetlacza
    .home-folder = Otwórz katalog domowy
    .keyboard-brightness-down = Zmniejsz jasność klawiatury
    .keyboard-brightness-up = Zwiększ jasność klawiatury
    .launcher = Otwórz program startowy
    .lock-screen = Zablokuj ekran
    .mute = Wycisz dzwięk wyjścia
    .mute-mic = Wycisz wejście mikrofonu
    .play-pause = Odtwarzaj/Zatrzymaj
    .play-next = Następny utwór
    .play-prev = Poprzedni utwór
    .screenshot = Zrób zrzut ekranu
    .terminal = Otwórz terminal
    .volume-lower = Zmniejsz głośność wyjścia dźwięku
    .volume-raise = Zwiększ głośność wyjścia dźwięku
    .web-browser = Otwórz przeglądarkę
    .window-switcher = Przełącz między otwartymi oknami
    .workspace-overview = Otwórz podgląd obszaru roboczego

window-tiling = Kafelkowanie okien
    .horizontal = Ustaw kierunek poziomy
    .vertical = Ustaw kierunek pionowy
    .swap-window = Zamień okna
    .toggle-tiling = Przełącz na kafelkowanie okien
    .toggle-stacking = Przełącznik grupowania w kartach
    .toggle-floating = Przełącz na pływające okna
    .toggle-orientation = Przełącznik kierunku

replace-shortcut-dialog = Zmienić skrót?
    .desc = { $shortcut } jest używany przez { $name }. Jeśli go zamienisz, skrót do { $name } będzie wyłączony.

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

gestures = Gesty
    .four-finger-down = Przesunięcie czterema palcami w dół
    .four-finger-left = Przesunięcie czterema palcami w lewo
    .four-finger-right = Przesunięcie czterema palcami w prawo
    .four-finger-up = Przesunięcie czterema palcami w górę
    .three-finger-any = Przesunięcie trzema palcami w dowolnym kierunku

switch-workspaces = Przełączenie pomiędzy obszarami roboczymi
    .horizontal = Przesunięcie czterema palcami w lewo/prawo
    .vertical = Przesunięcie czterema palcami w góra/dół
switch-between-windows = Przełączenie pomiędzy oknami
open-application-library = Otwarcie biblioteki aplikacji
open-workspaces-view = Otwarcie podglądu obszarów roboczych

## Time & Language

time = Czas i Język
    .desc = Niedostępne

time-date = Czas i Data
    .desc = Strefa czasowa, automatyczne ustawienia zegara oraz formatowanie czasu.
    .auto = Ustaw automatycznie
    .auto-ntp = Czas i data ustawią się automatycznie po wybraniu strefy czasowej.

time-zone = Strefa Czasowa
    .auto = Automatyczna strefa czasowa
    .auto-info = Wymaga usług lokalizacji oraz połączenia internetowego

time-format = Format Daty i Czasu
    .twenty-four = Czas 24-godzinny
    .show-seconds = Pokaż sekundy
    .first = Pierwszy dzień tygodnia
    .show-date = Pokaż Datę w Górnym Panelu
    .friday = Piątek
    .saturday = Sobota
    .sunday = Niedziela
    .monday = Poniedziałek

time-region = Region i Język
    .desc = Format dat, czasu i numerów na podstawie wybranego regionu

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
