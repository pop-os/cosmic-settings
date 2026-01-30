app = Ustawienia COSMIC
dbus-connection-error = Nieudane połączenie z DBus
ok = OK
unknown = Nieznane
number = { $number }

## Network & Wireless

connections-and-profiles =
    { $variant ->
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
password-confirm = Potwierdź hasło
remove = Usuń
settings = Ustawienia
username = Nazwa użytkownika
visible-networks = Widoczne sieci
identity = Tożsamość
auth-dialog = Konieczne uwierzytelnienie
    .vpn-description = Wprowadź nazwę użytkownika i hasło wymagane przez usługę VPN.
    .wifi-description = Wprowadź hasło lub klucz szyfrowania. Możesz też połączyć się, używając przycisku „WPS” na routerze.
forget-dialog = Chcesz zapomnieć o tej sieci Wi-Fi?
    .description = By znów używać tej sieci Wi-Fi będzie konieczne ponowne wprowadzenie hasła.
network-device-state =
    .activated = Połączono
    .config = Łączenie
    .deactivating = Rozłączanie
    .disconnected = Rozłączono
    .failed = Nie udało się połączyć
    .ip-check = Sprawdzanie połączenia
    .ip-config = Uzyskiwanie informacji o IP i routingu
    .need-auth = Konieczne uwierzytelnienie
    .prepare = Przygotowywanie połączenia
    .secondaries = Oczekiwanie na dodatkowe połączenie
    .unavailable = Niedostępne
    .unknown = Nieznany stan
    .unmanaged = Niezarządzane
    .unplugged = Odłączono kabel
remove-connection-dialog = Usunąć profil połączenia?
    .vpn-description = By znów używać tej sieci konieczne będzie ponowne wprowadzenie hasła..
    .wired-description = Będzie konieczne ponowne stworzenie profilu by go w przyszłości używać.
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
    .with-password =
        Nie udało się ustawić { $field ->
           *[username] nazwy użytkownika
            [password] hasła
            [password-flags] flag hasła
        } VPN przez nmcli
wired = Przewodowe
    .adapter = Adapter przewodowy { $id }
    .connections = Połączenia przewodowe
    .devices = Urządzenia przewodowe
    .remove = Usuń profil połączenia
wifi = Wi-Fi
    .adapter = Adapter Wi-Fi { $id }
    .forget = Zapomnij tę sieć
wireguard-dialog = Dodaj urządzenie WireGuard
    .description = Wybierz nazwę urządzenia dla tej konfiguracji WireGuard.

## Networking: Online Accounts

online-accounts = Konta online
    .desc = Dodaj konta, IMAP, SMTP i logowanie firmowe

# Bluetooth

activate = Aktywuj
confirm = Potwierdź
enable = Włącz
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
    .disabled = Usługa bluetooth jest wyłączona
    .inactive = Usługa bluetooth nie jest aktywna
    .unknown = Nie można aktywować usługi Bluetooth. Czy BlueZ jest zainstalowany?
bluetooth-paired = Wcześniej połączone urządzenia
    .connect = Połącz
    .battery = { $percentage }% baterii
bluetooth-confirm-pin = Potwierdź PIN Bluetooth
    .description = Potwierdź, że ten kod PIN i wyświetlany na { $device } są jednakowe
bluetooth-available = Pobliskie urządzenia
bluetooth-adapters = Adaptery bluetooth

## Accessibility

accessibility = Dostępność
    .vision = Widoczność
    .on = Włączona
    .off = Wyłączona
    .unavailable = Niedostępna
    .screen-reader = Czytnik ekranu
    .high-contrast = Tryb wysokiego kontrastu
    .invert-colors = Odwróć kolory
    .color-filters = Filtry kolorów
hearing = Słuchanie
    .mono = Odtwarzaj dźwięk stereo jako mono
default = Domyślne
magnifier = Lupa
    .controls =
        Lub użyj skrótów klawiszowych:{ $zoom_in ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_in } aby zbliżyć,
        }{ $zoom_out ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_out } aby oddalić,
        }
        Super+Scroll
    .scroll_controls = Aktywuj zbliżanie myszą lub touchpadem za pomocą Super + scroll
    .show_overlay = Pokaż nakładkę lupy
    .increment = Inkrementacja przybliżenia
    .signin = Uruchom lupę przy logowaniu
    .applet = Przełącznik lupy w aplecie na panelu
    .movement = Widok powiększenia porusza się
    .continuous = Stale ze wskaźnikiem
    .onedge = Gdy wskaźnik osiągnie krawędź
    .centered = Jest stale wycentrowany
color-filter = Typ filtra kolorów
    .unknown = Aktywny nieznany filtr
    .greyscale = Odcienie szarości
    .deuteranopia = Zielony/Czerwony (nierozpoznawanie zielonego, Deuteranopia)
    .protanopia = Czerwony/Zielony (nierozpoznawanie czerwonego, Protanopia)
    .tritanopia = Niebieski/Żółty (nierozpoznawanie niebieskiego, Tritanopia)

## Desktop

desktop = Pulpit

## Desktop: Wallpaper

wallpaper = Tapeta
    .change = Zmieniaj obraz co
    .desc = Obrazy tapety, kolory i opcje pokazu slajdów.
    .fit = Dopasuj tapetę
    .folder-dialog = Wybierz katalog z tapetami
    .image-dialog = Wybierz obraz tapety
    .plural = Tapety
    .same = Ta sama tapeta na wszystkich wyświetlaczach
    .slide = Pokaz slajdów
add-color = Dodaj kolor
add-image = Dodaj obraz
all-displays = Wszystkie wyświetlacze
colors = Kolory
dialog-add = Dodaj
fill = Wypełnij
fit-to-screen = Dopasuj do rozmiaru ekranu
open-new-folder = Otwórz nowy katalog
recent-folders = Ostatnie katalogi
x-minutes =
    { $number } { $number ->
        [one] minuta
        [few] minuty
       *[other] minut
    }
x-hours =
    { $number } { $number ->
        [one] godzina
        [few] godziny
       *[other] godzin
    }
never = Nigdy

## Desktop: Appearance

appearance = Wygląd
    .desc = Kolory akcentów i motywy
accent-color = Kolory akcentów
app-background = Tło okien
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
window-hint-accent-toggle = Użyj koloru motywu jako wyróżnienia aktywnego okna
auto-switch = Automatycznie zmieniaj między trybem jasnym a ciemnym
    .sunrise = Zmienia na jasny tryb o świcie
    .sunset = Zmienia na ciemny tryb o zmierzchu
    .next-sunrise = Zmienia na jasny tryb podczas następnego świtu
    .next-sunset = Zmienia na ciemny tryb podczas następnego zmierzchu
container-background = Tło kontenera
    .desc-detail = Kolor tła kontenera jest używany do panelu bocznego nawigacji, bocznego szkicownika, dialogów i podobnych widżetów. Domyślnie wywodzi się on z Aplikacji lub tła okna.
    .reset = Resetuj do automatycznych
    .desc = Jest używany do bocznego panelu nawigacji, bocznego szkicownika, dialogów i podobnych widżetów
control-tint = Sterowanie odcieniami komponentów
    .desc = Używany do tła standardowych przycisków, wprowadzania wyszukiwania, wprowadzania tekstu i podobnych komponentów
frosted = Efekt zmrożonego szkła na interfejsie systemowym
    .desc = Nakłada efekt rozmycia na panel, dok, aplety, program startowy oraz bibliotekę aplikacji
enable-export = Używaj tego motywu w aplikacjach GNOME.
    .desc = Nie wszystkie toolkity wspierają automatyczne zmiany. Aplikacje spoza COSMIC mogą wymagać restartu po zmianie motywu.
icon-theme = Motyw Ikon
    .desc = Zastosuj inny zbiór ikon do aplikacji
text-tint = Odcień tekstu interfejsu
    .desc = Kolor używany do uzyskania odcienia tekstu interfejsu, który ma wystarczający kontrast na różnych powierzchniach
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
    .gaps = Przerwy między ramkami okien w trybie kafelków

### Experimental

experimental-settings = Ustawienia eksperymentalne
icons-and-toolkit = Motyw ikon i toolkitu
interface-font = Czcionka systemowa
monospace-font = Czcionka o stałej szerokości

## Desktop: Notifications

notifications = Powiadomienia
    .desc = Nie Przeszkadzać, powiadomienia ekranu blokady oraz ustawienia konkretnej aplikacji

## Desktop: Panel

panel = Panel
    .desc = Główna belka z menu i apletami
add = Dodaj
add-applet = Dodaj aplet
all = Wszystkie
applets = Aplety
center-segment = Człon środkowy
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
    .dock-autohide = Automatycznie chowaj dok
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
    .desc = Konfiguracja apletów panelu
panel-missing = Brakuje Konfiguracji Panelu
    .desc = Brakuje pliku konfiguracji panelu z powodu użycia spersonalizowanej konfiguracji lub ten plik jest uszkodzony.
    .fix = Przywróć ustawienia domyślne

## Desktop: Dock

dock = Dok
    .desc = Dodatkowa belka z aplikacjami i apletami

## Desktop: Window management

window-management = Zarządzanie oknami
    .desc = Akcje klawisza super, ustawienia kontroli okien i dodatkowe ustawienia kafelkowania okien
super-key = Klawisz Super
    .launcher = Otwórz program startowy
    .workspaces = Otwórz obszary robocze
    .applications = Otwórz aplikacje
    .disable = Wyłącz
edge-gravity = Pływające okna są przyciągane do pobliskich krawędzi
window-controls = Sterowanie oknami
    .maximize = Pokaż przycisk maksymalizacji
    .minimize = Pokaż przycisk minimalizacji
    .active-window-hint = Wyróżnij aktywne okno
focus-navigation = Nawigacja aktywnym oknem
    .focus-follows-cursor = Aktywuje okno nad kursorem
    .focus-follows-cursor-delay = Opóźnienie kursora aktywującego okno w milisekundach
    .cursor-follows-focus = Przenosi kursor nad aktywne okno

## Desktop: Workspaces

workspaces = Obszary robocze
    .desc = Zachowanie i orientacja obszaru roboczego
workspaces-behavior = Zachowanie obszaru roboczego
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
workspaces-orientation = Orientacja obszarów roboczych
    .vertical = Pionowy
    .horizontal = Poziomy
hot-corner = Narożnik funkcyjny
    .top-left-corner = Włącz lewy górny narożnik funkcyjny w obszarach roboczych

## Displays

-requires-restart = Wymagany restart
color = Kolor
    .depth = Głębia koloru
    .profile = Profil koloru
    .sidebar = Profile Koloru
    .temperature = Temperatura koloru
display = Wyświetlacz
    .desc = Zarządzaj wyświetlaczami i nocnym światłem
    .arrangement = Układ wyświetlaczy
    .arrangement-desc = Przeciągaj wyświetlacze, aby zmienić układ
    .enable = Włącz wyświetlacz
    .external = { $size } { $output } Zewnętrzny Wyświetlacz
    .laptop = { $size } Wyświetlacz Laptopa
    .options = Opcje Wyświetlacza
    .refresh-rate = Prędkość odświeżania ekranu
    .resolution = Rozdzielczość
    .scale = Skala
    .additional-scale-options = Dodatkowe opcje skalowania
mirroring = Lustrzane Odbicie
    .id = Lustrzane Odbicie { $id }
    .dont = Nie stosuj Lustrzanego Odbicia
    .mirror = Zastosuj Lustrzane Odbicie na { $display }
    .project =
        Rzutuj na { $display ->
            [all] wszystkie wyświetlacze
           *[other] { $display }
        }
    .project-count =
        Rzutuj na { $count } { $count ->
            [1] inny
            [few] inne
           *[other] innych
        } { $count ->
            [1] wyświetlacz
            [few] wyświetlacze
           *[other] wyświetlaczy
        }
night-light = Nocne Światło
    .auto = Automatyczne (od świtu do zmierzchu)
    .desc = Zmniejsza ilość niebieskiego światła i ociepla kolory
orientation = Orientacja
    .standard = Standardowa
    .rotate-90 = Obróć o 90°
    .rotate-180 = Obróć o 180°
    .rotate-270 = Obróć o 270°
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
    .change-prompt =
        Ustawienia automatycznie powrócą do poprzednich za { $time ->
            [1] sekundę
            [few] { $time } sekundy
           *[other] { $time } sekund
        }.
    .revert-settings = Przywróć poprzednie ustawienia

## Sound

sound = Dźwięk
    .desc = Nie dotyczy
sound-output = Wyjście
    .volume = Głośność wyjścia
    .device = Urządzenie wyjścia
    .level = Poziom wyjścia
    .config = Konfiguracja
    .balance = Balans
    .left = Lewy
    .right = Prawy
sound-input = Wejście
    .volume = Głośność wejścia
    .device = Urządzenie wejścia
    .level = Poziom wejścia
amplification = Wzmocnienie
    .desc = Pozwala zwiększać głośność do 150%
sound-alerts = Alerty
    .volume = Głośność alertów
    .sound = Dźwięk alertów
sound-applications = Aplikacje
    .desc = Ustawienia i głośność aplikacji

## Power

power = Zasilanie i bateria
    .desc = Zarządzaj ustawieniami zasilania
battery = Bateria
    .minute =
        { $value } { $value ->
            [one] minuta
            [few] minuty
           *[other] minut
        }
    .hour =
        { $value } { $value ->
            [one] godzina
            [few] godziny
           *[other] godzin
        }
    .day =
        { $value } { $value ->
            [one] dzień
           *[other] dni
        }
    .less-than-minute = Mniej niż minuta
    .and = i
    .remaining-time =
        { $time } do { $action ->
            [full] naładowania
           *[other] rozładowania
        }
connected-devices = Podłączone urządzenia
    .unknown = Nierozpoznane urządzenie
power-mode = Profil zasilania
    .battery = Tryb oszczędzania energii
    .battery-desc = Zmniejszone zużycie energii i ciche działanie.
    .balanced = Tryb zrównoważony
    .balanced-desc = Wysoka wydajność i średnie zużycie baterii.
    .performance = Tryb wysokiej wydajności
    .performance-desc = Najwyższa wydajność i zwiększone zużycie energii.
    .no-backend = Nie znaleziono silnika. Zainstaluj system76-power lub power-profiles-daemon.
power-saving = Opcje Oszczędzania Energii
    .turn-off-screen-after = Wyłącz ekran po
    .auto-suspend = Automatycznie wstrzymaj
    .auto-suspend-ac = Automatycznie wstrzymuj na zasilaniu
    .auto-suspend-battery = Automatycznie wstrzymuj na baterii

## Input

acceleration-desc = Automatycznie dostosuj dokładność śledzenia do prędkości ruchu
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
short = Krótko
long = Długo
keyboard = Klawiatura
    .desc = Wprowadzanie danych, przełączanie, wprowadzanie znaków specjalnych i skróty na klawiaturze
keyboard-sources = Źródła wprowadzania danych
    .desc = Źródła wprowadzania danych można zmienić, naciskając kombinację klawiszy Super i Spacja. Może być to spersonalizowane w ustawieniach skrótów klawiaturowych.
    .move-up = Przesuń w górę
    .move-down = Przesuń w dół
    .settings = Ustawienia
    .view-layout = Pokaż układ klawiatury
    .remove = Usuń
    .add = Dodaj źródło wprowadzania danych
keyboard-special-char = Wpis znaków specjalnych
    .alternate = Alternatywne klawisze znaków
    .compose = Klawisz komponujący
    .compose-desc = Klawisz komponujący umożliwia wprowadzanie różnorodnych liter. By z niego skorzystać, naciśnij go a następnie kombinację znaków. Na przykład, po naciśnięciu klawisza komponującego a następnie C i o wyjdzie ©, lub klawisz komponujący a następnie a i ' wyjdzie á.
    .caps = Klawisz Caps Lock
keyboard-typing-assist = Pisanie
    .repeat-rate = Tempo powtarzania
    .repeat-delay = Opóźnienie powtarzania
keyboard-numlock-boot = Numlock
    .boot-state = Stan podczas uruchamiania
    .last-boot = Taki jak ostatnio
    .on = Włącz
    .off = Wyłącz
    .set = Ustaw stan Numlocka podczas uruchamiania
added = Dodano
type-to-search = Zacznij pisać by wyszukać…
show-extended-input-sources = Pokaż rozszerzone źródła wprowadzania

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Skróty klawiszowe
    .desc = Obejrzyj i spersonalizuj skróty
add-another-keybinding = Dodaj kolejny skrót klawiszowy
cancel = Anuluj
command = Komenda
custom = Własne
debug = Debuguj
disabled = Wyłączone
input-source-switch = Zmień język wprowadzania danych na klawiaturze
migrate-workspace-prev = Migruj obszar roboczy do poprzedniego ekranu
migrate-workspace-next = Migruj obszar roboczy do następnego ekranu
migrate-workspace =
    Migruj obszar roboczy do { $direction ->
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
    .none = Nie masz własnych skrótów klawiszowych
nav-shortcuts = Nawigacja
    .prev-output = Przełącz na poprzedni ekran
    .next-output = Przełącz na następny ekran
    .last-workspace = Przełącz na ostatni obszar roboczy
    .prev-workspace = Przełącz na poprzedni obszar roboczy
    .next-workspace = Przełącz na następny obszar roboczy
    .focus =
        Przełącz { $direction ->
           *[down] na dolne okno
            [in] na okno
            [left] na lewe okno
            [out] poza oknem
            [right] na prawe okno
            [up] na górne okno
        }
    .output =
        Przełącz na  { $direction ->
           *[down] dolny
            [left] lewy
            [right] prawy
            [up] górny
        } ekran
    .workspace = Przełącz na obszar roboczy { $num }
manage-windows = Zarządzanie oknami
    .close = Zamknij okno
    .maximize = Maksymalizuj okno
    .fullscreen = Pełny ekran
    .minimize = Minimalizuj okno
    .resize-inwards = Zmniejsz okno
    .resize-outwards = Powiększ okno
    .toggle-sticky = Przełącznik zakrywających okien
move-windows = Przemieszczanie okien
    .direction =
        Przemieść okno na { $direction ->
           *[down] dół
            [left] lewo
            [right] prawo
            [up] górę
        }
    .display =
        Przenieś okno o jeden monitor w { $direction ->
           *[down] dół
            [left] lewo
            [right] prawo
            [up] górę
        }
    .workspace =
        Przenieś okno o jeden obszar roboczy { $direction ->
           *[below] niżej
            [left] w lewo
            [right] w prawo
            [above] wyżej
        }
    .workspace-num = Przenieś okno do obszaru roboczego { $num }
    .prev-workspace = Przenieś okno do poprzedniego obszaru roboczego
    .next-workspace = Przenieś okno do następnego obszaru roboczego
    .last-workspace = Przenieś okno do ostatniego obszaru roboczego
    .next-display = Przenieś okno na następny wyświetlacz
    .prev-display = Przenieś okno na poprzedni wyświetlacz
    .send-to-prev-workspace = Przenieś okno do poprzedniego obszaru roboczego
    .send-to-next-workspace = Przenieś okno do następnego obszaru roboczego
system-shortcut = System
    .app-library = Otwórz bibliotekę aplikacji
    .brightness-down = Zmniejsz jasność wyświetlacza
    .brightness-up = Zwiększ jasność wyświetlacza
    .display-toggle = Przełącznik wewnętrznego wyświetlacza
    .home-folder = Otwórz katalog domowy
    .keyboard-brightness-down = Zmniejsz jasność klawiatury
    .keyboard-brightness-up = Zwiększ jasność klawiatury
    .launcher = Otwórz program startowy
    .log-out = Wyloguj się
    .lock-screen = Zablokuj ekran
    .mute = Wycisz dźwięk wyjścia
    .mute-mic = Wycisz wejście mikrofonu
    .play-pause = Odtwarzaj/Zatrzymaj
    .play-next = Następny utwór
    .play-prev = Poprzedni utwór
    .poweroff = Wyłącz
    .screenshot = Zrób zrzut ekranu
    .suspend = Wstrzymaj
    .terminal = Otwórz terminal
    .touchpad-toggle = Włączanie/wyłączanie gładzika
    .volume-lower = Zmniejsz głośność wyjścia dźwięku
    .volume-raise = Zwiększ głośność wyjścia dźwięku
    .web-browser = Otwórz przeglądarkę
    .window-switcher = Przełącz między otwartymi oknami
    .window-switcher-previous = Przełącz między otwartymi oknami w odwróconej kolejności
    .workspace-overview = Otwórz podgląd obszaru roboczego
window-tiling = Kafelkowanie okien
    .horizontal = Ustaw orientację poziomą
    .vertical = Ustaw orientację pionową
    .swap-window = Zamień okna
    .toggle-tiling = Przełącz kafelkowanie okien
    .toggle-stacking = Przełącz grupowanie okien
    .toggle-floating = Przełącz pływające okna
    .toggle-orientation = Przełącz orientację
replace-shortcut-dialog = Zamienić skrót?
    .desc = { $shortcut } jest używany przez { $name }. Jeśli go zamienisz, skrót do { $name } będzie wyłączony.
zoom-in = Zbliż
zoom-out = Oddal

## Input: Mouse

mouse = Myszka
    .desc = Prędkość, przyśpieszenie i naturalne przewijanie myszki
    .speed = Prędkość myszki
    .acceleration = Włącz akcelerację myszki

## Input: Touchpad

click-behavior = Funkcjonowanie kliknięć
    .click-finger = Drugi przycisk uzyskujemy kliknięciem dwoma palcami, a środkowy przycisk trzema
    .button-areas = Drugi przycisk uzyskujemy kliknięciem prawy dolny róg, a środkowy przycisk kliknięciem środek dołu
pinch-to-zoom = Uszczypnij, by przybliżyć
    .desc = Zbliż dwa palce do siebie, by przybliżyć zawartość, w aplikacjach używających przybliżenia
tap-to-click = Dotknij, aby kliknąć
    .desc = Włącza klikanie przez dotknięcie jednym palcem dla głównego przycisku, dwoma palcami dla drugiego przycisku i trzema palcami dla środkowego przycisku
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
    .vertical = Przesunięcie czterema palcami w górę/dół
switch-between-windows = Przełączenie pomiędzy oknami
open-application-library = Otwarcie Biblioteki Aplikacji
open-workspaces-view = Otwarcie podglądu obszarów roboczych

## Time & Language

time = Czas i język
    .desc = Nie dostępne
time-date = Data i godzina
    .desc = Strefa czasowa, automatyczne ustawienia zegara oraz formatowanie czasu.
    .auto = Ustaw automatycznie
    .auto-ntp = Czas i data ustawią się automatycznie po wybraniu strefy czasowej
time-zone = Strefa czasowa
    .auto = Automatyczna strefa czasowa
    .auto-info = Wymaga usług lokalizacji oraz połączenia internetowego
time-format = Format daty i godziny
    .twenty-four = Czas 24-godzinny
    .show-seconds = Pokaż sekundy
    .first = Pierwszy dzień tygodnia
    .show-date = Pokaż datę w aplecie czasu
    .friday = Piątek
    .saturday = Sobota
    .sunday = Niedziela
    .monday = Poniedziałek
time-region = Region i język
    .desc = Format dat, czasu i numerów odpowiadający wybranemu regionowi
formatting = Formatowanie
    .dates = Daty
    .time = Godzina
    .date-and-time = Data i godzina
    .numbers = Liczby
    .measurement = Miara
    .paper = Papier
preferred-languages = Preferowane języki
    .desc = Kolejność języków określa, który język jest używany podczas tłumaczenia. Zmiany wejdą w życie podczas kolejnego logowania.
add-language = Dodaj język
    .context = Dodaj Język
install-additional-languages = Zainstaluj dodatkowe języki
region = Region

## Applications

applications = Aplikacje

## Applications: Default Applications

default-apps = Domyślne Aplikacje
    .desc = Domyślna przeglądarka, klient e-mail, przeglądarka plików i inne aplikacje.
    .web-browser = Przeglądarka
    .file-manager = Przeglądarka plików
    .mail-client = Klient e-mail
    .music = Muzyka
    .video = Filmy
    .photos = Obrazy
    .calendar = Kalendarz
    .terminal = Konsola
    .other-associations = Inne powiązania
    .text-editor = Edytor tekstu

## Applications: Startup Applications

startup-apps = Aplikacje Startowe
    .desc = Konfiguracja aplikacji uruchamianych po zalogowaniu.
    .add = Dodaj aplikacje
    .user = Aplikacje uruchamiane po zalogowaniu
    .none = Brak dodanych aplikacji startowych
    .remove-dialog-title = Czy usunąć { $name }?
    .remove-dialog-description = Czy jesteś pewien, że chcesz usunąć tę aplikację startową?
    .add-startup-app = Dodaj aplikację startową

## Applications: Legacy Applications

legacy-applications = Kompatybilność Aplikacji X11
    .desc = Skalowanie i globalne skróty klawiszowe w aplikacjach systemu okien X11
legacy-app-global-shortcuts = Globalne skróty klawiszowe aplikacji X11
    .desc = Globalne skróty umożliwiają by naciśnięcia klawiszy lub przycisków myszy w aplikacjach były rozpoznawane przez inne aplikacje do między innymi funkcjonalności jak naciśnij-by-rozmawiać lub naciśnij-by-wyciszyć. Domyślnie jest to zablokowane w aplikacjach X11, by nie miały dostępu do śledzenia naciśnięć klawiszy i przyciśnięć myszy w aplikacjach mogących zawierać poufne informacje.
    .none = Żadne przyciski
    .modifiers = Modyfikatory (Super, Shift, Control, Alt)
    .combination = Wszystkie przyciski, kiedy modyfikatory Super, Control lub Alt są wciśnięte
    .all = Wszystkie przyciski
    .mouse = Naciśnięcia przycisków myszy w aplikacjach X11
legacy-app-scaling = Skalowanie aplikacji systemu okien X11
    .scaled-gaming = Zoptymalizuj do grania i aplikacji pełnoekranowych
    .gaming-description = Aplikacje X11 mogą być nieco mniejsze/większe w porównaniu z aplikacjami Wayland
    .scaled-applications = Zoptymalizuj dla aplikacji
    .applications-description = Gry i aplikacje pełnoekranowe X11 mogą nie zgadzać się z rozdzielczością twojego ekranu
    .scaled-compatibility = Tryb maksymalnej kompatybilności
    .compatibility-description = Aplikacje X11 mogą być rozmazane na wyświetlaczach HiDPI.
    .preferred-display = Preferowany wyświetlacz dla gier i aplikacji pełnoekranowych X11
    .no-display = Brak

## System

system = System i konta

## System: About

about = O systemie
    .desc = Nazwa urządzenia, informacje o sprzęcie, domyślne ustawienia systemu
about-device = Nazwa urządzenia
    .desc = Ta nazwa wyświetla się innym w sieci i na urządzeniach bluetooth
about-hardware = Sprzęt
    .model = Model sprzętu
    .memory = Pamięć
    .processor = Procesor
    .graphics = Grafika
    .disk-capacity = Pojemność dysku
about-os = System operacyjny
    .os = System operacyjny
    .os-architecture = Architektura systemu operacyjnego
    .kernel = Wersja kernela
    .desktop-environment = Środowisko graficzne
    .windowing-system = System okien
about-related = Pokrewne ustawienia
    .support = Uzyskaj wsparcie

## System: Firmware

firmware = Firmware
    .desc = Szczegóły firmware

## System: Users

users = Użytkownicy
    .desc = Uwierzytelnianie, logowanie i ekran blokady.
    .admin = Administrator
    .standard = Standardowy
    .profile-add = Wybierz obraz profilu
administrator = Administrator
    .desc = Administratorzy mogą zmieniać ustawienia wszystkich użytkowników, dodawać i usuwać innych użytkowników
add-user = Dodaj użytkownika
change-password = Zmień hasło
remove-user = Usuń użytkownika
full-name = Pełna nazwa
invalid-username = Nieprawidłowa nazwa użytkownika
password-mismatch = Hasło i jego potwierdzenie muszą być jednakowe
save = Zapisz
modified = Zmodyfikowano { $count }
place-here = Umieść tu applety
qr-code-unavailable = Kod QR jest niedostępny
network-name = Nazwa Sieci
share = Udostępnij sieć
scan-to-connect-description = Zeskanuj kod QR, aby połączyć się z tą siecią.
sound-device-port-unplugged = Odłączone
sound-hd-audio = Dźwięk HD
sound-usb-audio = Dźwięk USB
sound-device-profiles = Profile urządzeń
shadows-floating = Pływające okna
    .clip = Dopasuj do rogów systemowych i zaaplikuj cienie
shadows-tiling = Pokafelkowane okna
    .clip = Dopasuj do rogów systemowych
    .shadow = Zaaplikuj cienie
shadow-and-corners = Cienie i rogi okna
