app = Ustawienia COSMIC

dbus-connection-error = Nieudane połączenie do DBus
ok = OK
unknown = Nieznane

number = { $number }

## Network & Wireless

connections-and-profiles = { $variant ->
    [wired] Połączenia przewodowe
    [wifi] Połączenia Wi-Fi
    [vpn] Połączenia VPN
    *[other] Nieznane połączenia
} i profile połączeń.

add-network = Dodaj sieć
    .profile = Dodaj profil
add-vpn = Dodaj VPN
airplane-on = Tryb samolotowy jest włączony.
cable-unplugged = Odłączono kabel
connect = Połącz
connected = Połączono
connecting = Łączenie…
disconnect = Rozłącz
forget = Zapomnij
known-networks = Znane sieci
network-and-wireless = Połączenia sieciowe
no-networks = Nie odnaleziono sieci.
no-vpn = Brak dostępnych połączeń VPN.
password = Hasło
remove = Usuń
settings = Ustawienia
username = Nazwa użytkownika
visible-networks = Widoczne sieci

auth-dialog = Wymagane jest uwierzytelnianie
    .vpn-description = Wprowadź nazwę użytkownika i hasło wymagane przez usługę VPN.
    .wifi-description = Wprowadź hasło lub klucz szyfrowania. Możesz też połączyć się, używajac przycisku „WPS” na routerze.

forget-dialog = Chcesz zapomnieć o tej sieci Wi-Fi?
    .description = Będziesz musiał(a) wprowadzić hasło jeszcze raz, aby ponownie użyć tej sieci Wi-Fi.

network-device-state =
    .activated = Połączono
    .config = Łączenie
    .deactivating = Rozłączanie
    .disconnected = Rozłączono
    .failed = Nie udało się połąćzyć
    .ip-check = Sprawdzanie połączenia
    .ip-config = Uzyskiwanie informacji o IP i routowaniu
    .need-auth = Wymagane jest uwierzytelnienie
    .prepare = Przygotowywanie połączenia
    .secondaries = Oczekiwanie na dodatkowe połączenie
    .unavailable = Niedostępne
    .unknown = Nieznany stan
    .unmanaged = Niezarządzane
    .unplugged = Odłączono kabel

remove-connection-dialog = Usunąć profil połączenia?
    .vpn-description = Będziesz musiał(a) wprowadzić hasło jeszcze raz, aby ponownie użyć tej siecii.
    .wired-description = Będziesz musiał(a) utworzyć profil jeszcze raz, aby ponownie użyć tej sieci Wi-Fi.

vpn = VPN
    .connections = Połączenia VPN
    .error = Nie udało się dodać konfiguracji VPN
    .remove = Usuń profil połączenia
    .select-file = Wybierz plik konfiguracji VPN

vpn-error = Błąd VPN
    .config = Nie udało się dodać konfiguracji VPN
    .connect = Nie udało się połączyć z VPN
    .connection-editor = Błąd edytora połączenia
    .connection-settings = Nie udało się uzyskać ustawień dla aktywnych połączeń
    .updating-state = Nie udało się zaktualizować stanu menedżera sieci
    .wireguard-config-path = Nieprawidłowa ścieżka pliku konfiguracji WireGuard
    .wireguard-config-path-desc = Wybrany plik musi znajdować się w lokalnym systemie plików system.
    .wireguard-device = Nie udało się utworzyć urządzenia WireGuard
    .with-password = Nie udało się ustawić { $field ->
        *[username] nazwy użytkownika
        [password] hasła
        [password-flags] flag hasła
    } VPN przez nmcli

wired = Przewodowe
    .adapter = Adapter przewodowy { $id }
    .connections = Połączenia przewodowe
    .devices = Urządzenia przeowodowe
    .remove = Remove connection profile

wifi = Wi-Fi
    .adapter = Adapter Wi-Fi { $id }
    .forget = Zapomnij o tym urządzeniu

wireguard-dialog = Dodaj urządzenie WireGuard
    .description = Wybierz nazwę urządzenia dla tej konfiguracji WireGuard.

## Networking: Online Accounts

online-accounts = Konta online
    .desc = Dodaj konta, IMAP, SMTP i logowanie firmowe

# Bluetooth

confirm = Potwierdź

bluetooth = Bluetooth
    .desc = Zarządzaj urządzeniami Bluetooth
    .status = Ten system jest widoczny jako { $aliases }, gdy ustawienia Bluetooth są otwarte.
    .connected = Połączono
    .connecting = Łączenie
    .disconnecting = Rozłączanie
    .connect = Połącz
    .disconnect = Rozłącz
    .forget = Zapomnij
    .dbus-error = Błąd interakcji z DBus: { $why }

bluetooth-paired = Wcześniej połączone urządzenia
    .connect = Połącz
    .battery = { $percentage }% baterii

bluetooth-confirm-pin = Potwierdź PIN Bluetooth
    .description = Potwierdź, że następujący PIN zgadza się z wyświetlanym na { $device }

bluetooth-available = Urządzenia w pobliżu

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
all-displays = Wszystkie wyświetlacze
colors = Kolory
dialog-add = Dodaj
fill = Wypełnij
fit-to-screen = Dopasuj do rozmiaru ekranu
open-new-folder = Otwórz nowy katalog
recent-folders = Ostatnie katalogi

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
never = Nigdy

## Desktop: Appearance

appearance = Wygląd
    .desc = Kolory akcentów i motywy COSMIC.

accent-color = Kolory akcentów
app-background = Tło aplikacji oraz okien
auto = Automatyczne
close = Zamknij
color-picker = Wybór koloru
copied-to-clipboard = Skopiowano do schowka
copy-to-clipboard = Skopiuj do schowka
dark = Ciemny
export = Eksportuj
hex = Hex
import = Importuj
light = Jasny
mode-and-colors = Tryb i kolory
recent-colors = Ostatnie kolory
reset-to-default = Przywróć do ustawień domyślnych
rgb = RGB
window-hint-accent = Kolor wyróżnienia aktywnego okna
window-hint-accent-toggle = Użycie koloru z motywu jako wyróżnienia aktywnego okna

auto-switch = Automatycznie zmieniaj między trybem jasnym a ciemnym
    .sunrise = Zmienia na jasny tryb o świcie
    .sunset = Zmienia na ciemny tryb o zmierzchu
    .next-sunrise = Zmienia na jasny tryb podczas następnego świtu
    .next-sunset = Zmienia na ciemny tryb podczas następnego zmierzchu

container-background = Tło kontenera
    .desc-detail = Kolor tła kontenera jest używany do nawigacji panelem bocznym, bocznym szkicownikiem, dialogami i podobnymi widżetami. Domyślnie wywodzi się on z Aplikacji lub tła okna.
    .reset = Resetuj do automatycznych
    .desc = Główny kolor kontenera jest używany do bocznego panelu nawigacji, bocznego szkicownika, dialogów i podobnych widżetów.

control-tint = Sterowanie odcieniami komponentów
    .desc = Używany do tła standardowych przycisków, wprowadzania wyszukiwania, wprowadzania tekstu i podobnych komponentów.

frosted = Efekt zmrożonego szkła na interfejsie systemowym
    .desc = Nakłada efekt rozmycia na panel, dok, aplety, program startowy oraz bibliotekę aplikacji.

experimental-settings = Ustawienia eksperymentalne

enable-export = Używaj tego motywu w aplikacjach GNOME.
    .desc = Nie wszystkie toolkity wspierają automatyczne zmiany. Aplikacje spoza COSMIC mogą wymagać restartu po zmianie motywu.

icon-theme = Motyw Ikon
    .desc = Zastosuj inny zbiór ikon do aplikacji.

text-tint = Odcień tekstu interfejsu
    .desc = Kolor używany do uzyskania odcienia tekstu interfejsu, który ma wystarczający kontrast na różnych powierzchniach.

style = Styl
    .round = Okrągły
    .slightly-round = Lekko zaokrąglony
    .square = Kwadratowy

interface-density = Zagęszczenie interfejsu
    .comfortable = Wygodne
    .compact = Zwarte
    .spacious = Przestronne

window-management-appearance = Zarządzanie oknami
    .active-hint = Rozmiar wyróżnienia aktywnego okna
    .gaps = Przerwa między ramkami okien w trybie kafelków

### Experimental

experimental-settings = Ustawienia eksperymentalne
icons-and-toolkit = Motyw ikon i toolkita
interface-font = Font systemowy
monospace-font = Font o stałej szerokości

## Desktop: Notifications

notifications = Powiadomienia
    .desc = Nie przeszkadzać, powiadomienia ekranu blokady oraz ustawienia konkretnej aplikacji.

## Desktop: Panel

panel = Panel
    .desc = Górna belka ze sterowaniem pulpitem i menu.

add = Dodaj
add-applet = Dodaj aplet
all = Wszystkie
applets = Aplety
center-segment = Człon środkowy
drop-here = Tutaj upuść aplety
end-segment = Człon końcowy
large = Duży
no-applets-found = Nie znaleziono apletów…
panel-bottom = Dolny
panel-left = Lewy
panel-right = Prawy
panel-top = Górny
search-applets = Wyszukaj aplety…
small = Mały
start-segment = Człon początkowy

panel-appearance = Wygląd
    .match = Dopasuj do pulpitu
    .light = Jasny
    .dark = Ciemny

panel-behavior-and-position = Funkcjonowanie i położenie
    .autohide = Automatycznie chowaj panel
    .dock-autohide = Automatycznie chowaj panel
    .position = Położenie na ekranie
    .display = Pokaż na wyświetlaczu

panel-style = Styl
    .anchor-gap = Przerwa pomiędzy panelem a krawędziami ekranu
    .dock-anchor-gap = Przerwa pomiędzy dokiem a krawędziami ekranu
    .extend = Rozszerz panel do krawędzi ekranu
    .dock-extend = Rozszerz dok do krawędzi ekranu
    .appearance = Wygląd
    .size = Rozmiar
    .background-opacity = Przejrzystość tła

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
    .launcher = Otwórz program startowy
    .workspaces = Otwórz obszary robocze
    .applications = Otwórz aplikacje
    .disable = Wyłącz

window-controls = Sterowanie oknami
    .maximize = Pokaż przycisk maksymalizacji
    .minimize = Pokaż przycisk minimalizacji

focus-navigation = Nawigacja aktywnym oknem
    .focus-follows-cursor = Aktywuje okno nad kursorem
    .focus-follows-cursor-delay = Opóźnienie kursora aktywującego okno w milisekundach
    .cursor-follows-focus = Przenosi kursor nad aktywne okno

## Desktop: Workspaces

workspaces = Obszary robocze
    .desc = Zachowanie i kierunek obszaru roboczego.

workspaces-behavior = Funkcjonowanie obszaru roboczego
    .dynamic = Dynamiczne obszary robocze
    .dynamic-desc = Puste obszary robocze są automatycznie usuwane.
    .fixed = Stała liczba obszarów roboczych
    .fixed-desc = Dodaj lub usuń obszar roboczy w podglądzie.

workspaces-multi-behavior = Funkcjonowanie przy wielu monitorach
    .span = Obszary robocze wspólne dla wyświetlaczy
    .separate = Wyświetlacze mają osobne obszary robocze

workspaces-overview-thumbnails = Miniatury podglądu obszaru roboczego
    .show-number = Pokaż numer obszaru roboczego
    .show-name = Pokaż nazwę obszaru roboczego

workspaces-orientation = Kierunek obszarów roboczych
    .vertical = Pionowy
    .horizontal = Poziomy

hot-corner = Hot Corner
    .top-left-corner = Włącz lewy górny narożnik funkcyjny w obszarach roboczych

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

vrr = Zmienna częstotliwość odświeżania
    .enabled = Włączona
    .force = Zawsze
    .auto = Automatycznie
    .disabled = Wyłączona

scheduling = Harmonogram
    .manual = Ręcznie ustawiony harmonogram

dialog = Dialog
    .title = Czy zachować te ustawienia wyświetlacza?
    .keep-changes = Zachowaj zmiany
    .change-prompt = Ustawienia automatycznie powrócą do poprzednich za { $time ->
        [1] sekundę.
        [few] {$time} sekundy.
        *[other] {$time} sekund.
    }
    .revert-settings = Powróć do poprzednich ustawień

legacy-applications = Skalowanie aplikacji systemu okien X11
    .scaled-by-system = Skaluj wszystkie aplikacje X11
    .system-description = Aplikacje X11 będą rozmyte na wyświetlaczach z wysokim DPI.
    .scaled-natively = Renderuj aplikacje X11 w pierwotnej rozdzielczości.
    .native-description = Aplikacje X11 które nie wspierają skalowania będą małe na wyświetlaczach z wysokim DPI. Włącz do gier by wykorzystywały pełną rozdzielczość monitora.

## Sound

sound = Dźwięk
    .desc = nd.

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

connected-devices = Podłączone urządzenia
  .unknown = Nierozpoznane urządzenie

power-mode = Profile zasilania
  .battery = Tryb oszczędzania energii
  .battery-desc = Zmniejszone zużycie energii i ciche działanie.
  .balanced = Tryb zbalansowany
  .balanced-desc = Standardowa wydajność i zużycie baterii.
  .performance = Tryb wysokiej wydajności
  .performance-desc = Najwyższa wydajność i zwiększone zużycie energii.
  .no-backend = Nie znaleziono backendu. Zainstaluj system76-power lub power-profiles-daemon.

power-saving = Opcje oszczędzania energii
    .turn-off-screen-after = Wygaszaj ekran po
    .auto-suspend = Automatycznie wstrzymuj
    .auto-suspend-ac = Automatycznie wstrzymuj gdy podłączono
    .auto-suspend-battery = Automatycznie wstrzymuj na baterii

## Input

acceleration-desc = Automatycznie dostosuj dokładność śledzenia do prędkości ruchu.

disable-while-typing = Wyłącz podczas pisania

input-devices = Urządzenia wprowadzania danych
    .desc = Urządzenia wprowadzania danych

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

keyboard-sources = Żródła wprowadzania danych
    .desc = Żródła wprowadzania danych można zmienić naciskając kombinację klawiszy Super i Spacja. Może być to spersonalizowane w ustawieniach skrótów klawiaturowych.
    .move-up = Przesuń w górę
    .move-down = Przesuń w dół
    .settings = Ustawienia
    .view-layout = Pokaż układ klawiatury
    .remove = Usuń
    .add = Dodaj źródło wprowadzania danych

keyboard-special-char = Wpis znaków specjalnych
    .alternate = Alternatywne klawisze znaków
    .compose = Klawisz komponujący
    .caps = Klawisz Caps Lock

keyboard-typing-assist = Pisanie
    .repeat-rate = Tempo powtarzania
    .repeat-delay = Opóźnienie powtarzania

added = Dodany
type-to-search = Naciśnij by wyszukać…
show-extended-input-sources = Pokaż rozszerzone źródła wprowadzania

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Skróty klawiaturowe
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
    .context = Dodaj własny skrót klawiszowy
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
    .window-switcher-previous = Przełącz między otwartymi oknami w odwróconej kolejności
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

click-behavior = Funkcjonowanie kliknięć
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

time = Czas i język
    .desc = Niedostępne

time-date = Data i godzina
    .desc = Strefa czasowa, automatyczne ustawienia zegara oraz formatowanie czasu.
    .auto = Ustaw automatycznie
    .auto-ntp = Czas i data ustawią się automatycznie po wybraniu strefy czasowej.

time-zone = Strefa czasowa
    .auto = Automatyczna strefa czasowa
    .auto-info = Wymaga usług lokalizacji oraz połączenia internetowego

time-format = Format daty i godziny
    .twenty-four = Czas 24-godzinny
    .show-seconds = Pokaż sekundy
    .first = Pierwszy dzień tygodnia
    .show-date = Pokaż datę w górnym panelu
    .friday = Piątek
    .saturday = Sobota
    .sunday = Niedziela
    .monday = Poniedziałek

time-region = Region i język
    .desc = Format dat, czasu i numerów na podstawie wybranego regionu

formatting = Formatowanie
    .dates = Daty
    .time = Godzina
    .date-and-time = Data i godzina
    .numbers = Liczby
    .measurement = Miara
    .paper = Papier

preferred-languages = Preferowane języki
    .desc = Kolejność języków określa który język jest używany podczas tłumaczenia. Zmiany wejdą w życie podczas kolejnego logowania.

add-language = Dodaj język
    .context = Dodaj język
install-additional-languages = Zainstaluj dodatkowe języki
region = Region

## System

system = System i Konta

## System: About

about = O systemie
    .desc = Nazwa urządzenia, informacje o sprzęcie, domyślne ustawienia systemu.

about-device = Nazwa urządzenia
    .desc = Ta nazwa wyświetla się innym w sieci i na urządzeniach bluetooth.

about-hardware = Sprzęt
    .model = Model sprzętu
    .memory = Pamięć
    .processor = Procesor
    .graphics = Grafika
    .disk-capacity = Pojemność dysku

about-os = System operacyjny
    .os = System operacyjny
    .os-architecture = Architektura systemu operacyjnego
    .desktop-environment = Środowisko graficzne
    .windowing-system = System okien

about-related = Pokrewne ustawienia
    .support = Uzyskaj wsparcie

## System: Firmware

firmware = Mikrooprogramowanie
    .desc = Szczegóły mikrooprogramowania

## System: Users

users = Użytkownicy
    .desc = Uwierzytelnianie, logowanie i ekran blokady.
    .admin = Administrator
    .standard = Standardowy
    .profile-add = Wybierz obraz profilu

administrator = Administrator
    .desc = Administratorzy mogą zmieniać ustawienia wszystkich użytkowników, dodawać i usuwać innych użytkowników.

add-user = Dodaj użytkownika
remove-user = Usuń użytkownika
full-name = Pełna nazwa
username = Nazwa użytkownika
password = Hasło

## System: Domyślne aplikacje

default-apps = Domyślne aplikacje
    .desc = Domyślna przeglądarka, klient e-mail, przeglądarka plików i inne aplikacje.
    .web-browser = Przeglądarka internetowa
    .file-manager = Menedżer plików
    .mail-client = Klient poczty e-mail
    .music = Muzyka
    .video = Filmy
    .photos = Zdjęcia
    .calendar = Kalendarz
    .terminal = Konsola
    .other-associations = Inne powiązania
