app = Nastavenia COSMIC

dbus-connection-error = Nepodarilo sa pripojiť k DBus
ok = OK
unknown = Neznáme

number = { $number }

## Sieť a bezdrôtové pripojenie

connections-and-profiles = { $variant ->
    [wired] Káblové
    [wifi] Wi-Fi
    [vpn] VPN
    *[other] Neznáme
} pripojenia a profily pripojenia.

add-network = Pridať sieť
    .profile = Pridať profil
add-vpn = Pridať VPN
airplane-on = Režim Lietadlo je zapnutý.
cable-unplugged = Kábel odpojený
connect = Pripojiť
connected = Pripojené
connecting = Pripája sa…
disconnect = Odpojiť
forget = Zabudnúť
known-networks = Známe siete
network-and-wireless = Sieť a bezdrôtové pripojenie
no-networks = Neboli nájdené žiadne siete.
no-vpn = Žiadne VPN pripojenia nie sú dostupné.
password = Heslo
remove = Odstrániť
settings = Nastavenia
username = Používateľské meno
visible-networks = Viditeľné siete

auth-dialog = Vyžaduje sa autentifikácia
    .vpn-description = Zadajte používateľské meno a heslo požadované službou VPN.
    .wifi-description = Zadajte heslo alebo šifrovací kľúč. Môžete sa tiež pripojiť stlačením tlačidla „WPS“ na smerovači.

forget-dialog = Zabudnúť túto sieť Wi-Fi?
    .description = Ak chcete túto sieť Wi-Fi používať v budúcnosti, budete musieť znova zadať heslo.

network-device-state =
    .activated = Pripojené
    .config = Pripája sa
    .deactivating = Odpája sa
    .disconnected = Odpojené
    .failed = Nepodarilo sa pripojiť
    .ip-check = Kontroluje sa pripojenie
    .ip-config = Vyžadujú sa informácie o IP a smerovaní
    .need-auth = Vyžaduje sa autentifikácia
    .prepare = Pripravuje sa na pripojenie
    .secondaries = Čaká sa na sekundárne pripojenie
    .unavailable = Nedostupné
    .unknown = Neznámy stav
    .unmanaged = Nespravované
    .unplugged = Kábel odpojený

remove-connection-dialog = Odstrániť profil pripojenia?
    .vpn-description = Ak chcete túto sieť používať v budúcnosti, budete musieť znova zadať heslo.
    .wired-description = Ak ho chcete používať v budúcnosti, budete musieť tento profil znova vytvoriť.

vpn = VPN
    .connections = VPN pripojenia
    .error = Nepodarilo sa pridať konfiguráciu VPN
    .remove = Odstrániť profil pripojenia
    .select-file = Vyberte konfiguračný súbor VPN

vpn-error = Chyba VPN
    .config = Nepodarilo sa pridať konfiguráciu VPN
    .connect = Nepodarilo sa pripojiť k VPN
    .connection-editor = Editor pripojenia zlyhal
    .connection-settings = Nepodarilo sa získať nastavenia pre aktívne pripojenia
    .updating-state = Nepodarilo sa aktualizovať stav správcu siete
    .wireguard-config-path = Neplatná cesta k súboru pre konfiguráciu WireGuard
    .wireguard-config-path-desc = Vybratý súbor musí byť v lokálnom systéme súborov.
    .wireguard-device = Nepodarilo sa vytvoriť zariadenie WireGuard
    .with-password = Nepodarilo sa nastaviť VPN { $field ->
        *[username] používateľské meno
        [password] heslo
        [password-flags] príznaky hesla
    } pomocou nmcli

wired = Káblové pripojenie
    .adapter = Káblový adaptér { $id }
    .connections = Káblové pripojenia
    .devices = Káblové zariadenia
    .remove = Odstrániť profil pripojenia

wifi = Wi-Fi
    .adapter = Wi-Fi adaptér { $id }
    .forget = Zabudnúť túto sieť

wireguard-dialog = Pridať zariadenie WireGuard
    .description = Vyberte názov zariadenia pre konfiguráciu WireGuard.

## Siete: Online účty

online-accounts = Online účty
    .desc = Pridajte účty, IMAP a SMTP, podnikové prihlásenia

# Bluetooth

confirm = Potvrdiť

bluetooth = Bluetooth
    .desc = Spravovať zariadenia Bluetooth
    .status = Tento systém je viditeľný ako { $aliases }, keď sú otvorené nastavenia Bluetooth.
    .connected = Pripojené
    .connecting = Pripája sa
    .disconnecting = Odpája sa
    .connect = Pripojiť
    .disconnect = Odpojiť
    .forget = Zabudnúť
    .dbus-error = Pri interakcii s DBus sa vyskytla chyba: { $why }

bluetooth-paired = Spárované zariadenia
    .connect = Pripojiť
    .battery = { $percentage }% batérie

bluetooth-confirm-pin = Potvrďte PIN Bluetooth
    .description = Potvrďte, že nasledujúci PIN sa zhoduje s PIN zobrazeným na zariadení { $device }

bluetooth-available = Zariadenia v okolí

bluetooth-adapters = Adaptéry Bluetooth

## Prístupnosť

accessibility = Prístupnosť
    .vision = Zrak
    .on = Zapnuté
    .off = Vypnuté
    .unavailable = Nedostupné
magnifier = Lupa
    .controls =
        Super + posúvanie myšou

## Plocha

desktop = Plocha

## Plocha: Tapeta

wallpaper = Tapeta

add-color = Pridať farbu
add-image = Pridať obrázok
all-displays = Všetky displeje
colors = Farby
dialog-add = Pridať
fill = Vyplniť
fit-to-screen = Prispôsobiť obrazovke
open-new-folder = Otvoriť nový priečinok
recent-folders = Nedávne priečinky

x-minutes = { $number ->
    [one] { $number } minúta
    [few] { $number } minúty
    *[other] { $number } minút
}
x-hours = { $number ->
    [one] { $number } hodina
    [few] { $number } hodiny
    *[other] { $number } hodín
}
never = Nikdy

## Plocha: Vzhľad

appearance = Vzhľad
    .desc = Akcentové farby a témy.

accent-color = Akcentová farba
app-background = Pozadie aplikácie alebo okna
auto = Automaticky
close = Zatvoriť
color-picker = Výber farby
copied-to-clipboard = Skopírované do schránky
copy-to-clipboard = Skopírovať do schránky
dark = Tmavý
export = Exportovať
hex = Hex
import = Importovať
light = Svetlý
mode-and-colors = Režim a farby
recent-colors = Nedávne farby
reset-to-default = Obnoviť predvolené
rgb = RGB
window-hint-accent = Farba zvýraznenia aktívneho okna
window-hint-accent-toggle = Použiť farbu zvýraznenia témy ako farbu zvýraznenia aktívneho okna

auto-switch = Automaticky prepínať medzi svetlým a tmavým režimom
    .sunrise = Prepne na svetlý režim pri východe slnka
    .sunset = Prepne na tmavý režim pri západe slnka
    .next-sunrise = Prepne na svetlý režim pri nasledujúcom východe slnka
    .next-sunset = Prepne na tmavý režim pri nasledujúcom západe slnka

container-background = Pozadie kontajnera
    .desc-detail = Farba pozadia kontajnera sa používa pre navigačný bočný panel, bočnú zásuvku, dialógové okná a podobné widgety. Štandardne sa automaticky odvodzuje od pozadia aplikácie alebo okna.
    .reset = Obnoviť automatické
    .desc = Primárna farba kontajnera sa používa pre navigačný bočný panel, bočnú zásuvku, dialógové okná a podobné widgety.

control-tint = Odtieň ovládacieho prvku
    .desc = Používa sa pre pozadie štandardných tlačidiel, vstupov vyhľadávania, textových vstupov a podobných komponentov.

frosted = Efekt matného skla na systémovom rozhraní
    .desc = Používa rozmazanie pozadia na panel, dok, applety, spúšťač a knižnicu aplikácií.

enable-export = Použiť túto tému pre aplikácie GNOME.
    .desc = Nie všetky sady nástrojov podporujú automatické prepínanie. Aplikácie iné ako COSMIC možno bude potrebné reštartovať po zmene témy.

icon-theme = Téma ikon
    .desc = Použije inú sadu ikon pre aplikácie.

text-tint = Odtieň textu rozhrania
    .desc = Farba použitá na odvodenie farieb textu rozhrania, ktoré majú dostatočný kontrast na rôznych povrchoch.

style = Štýl
    .round = Okrúhly
    .slightly-round = Mierne okrúhly
    .square = Štvorcový

interface-density = Hustota rozhrania
    .comfortable = Pohodlné
    .compact = Kompaktné
    .spacious = Priestorné

window-management-appearance = Správa okien
    .active-hint = Veľkosť zvýraznenia aktívneho okna
    .gaps = Medzery okolo okien s dlaždicami

### Experimentálne

experimental-settings = Experimentálne nastavenia
icons-and-toolkit = Ikony a témy sady nástrojov
interface-font = Systémové písmo
monospace-font = Monospace písmo

## Plocha: Upozornenia

notifications = Upozornenia
    .desc = Nerušiť, upozornenia na uzamknutej obrazovke a nastavenia pre jednotlivé aplikácie.

## Plocha: Panel

panel = Panel
    .desc = Horný panel s ovládacími prvkami a ponukami plochy.

add = Pridať
add-applet = Pridať applet
all = Všetky
applets = Applety
center-segment = Stredný segment
drop-here = Presuňte applety sem
end-segment = Koncový segment
large = Veľký
no-applets-found = Nenašli sa žiadne applety...
panel-bottom = Dolný
panel-left = Ľavý
panel-right = Pravý
panel-top = Horný
search-applets = Hľadať applety...
small = Malý
start-segment = Počiatočný segment

panel-appearance = Vzhľad
    .match = Rovnaký ako systém
    .light = Svetlý
    .dark = Tmavý

panel-behavior-and-position = Správanie a pozície
    .autohide = Automaticky skryť panel
    .dock-autohide = Automaticky skryť dok
    .position = Pozícia na obrazovke
    .display = Zobraziť na displeji

panel-style = Štýl
    .anchor-gap = Medzera medzi panelom a okrajmi obrazovky
    .dock-anchor-gap = Medzera medzi dokom a okrajmi obrazovky
    .extend = Rozšíriť panel na okraje obrazovky
    .dock-extend = Rozšíriť dok na okraje obrazovky
    .appearance = Vzhľad
    .size = Veľkosť
    .background-opacity = Priehľadnosť pozadia

panel-applets = Konfigurácia
    .dock-desc = Konfigurovať applety doku
    .desc = Konfigurovať applety panela

panel-missing = Chýba konfigurácia panela
    .desc = Konfiguračný súbor panela chýba z dôvodu použitia vlastnej konfigurácie alebo je poškodený.
    .fix = Obnoviť predvolené

## Plocha: Dok

dock = Dok
    .desc = Panel s pripnutými aplikáciami v paneli aplikácií a ďalšími appletmi.

## Plocha: Správa okien

window-management = Správa okien
    .desc = Akcia klávesu Super, možnosti ovládania okien a ďalšie možnosti usporiadania okien.

super-key = Akcia klávesu Super
    .launcher = Otvoriť spúšťač
    .workspaces = Otvoriť pracovné priestory
    .applications = Otvoriť aplikácie
    .disable = Zakázať

edge-gravity = Plávajúce okná sa priťahujú k blízkym okrajom

window-controls = Ovládacie prvky okna
    .maximize = Zobraziť tlačidlo maximalizácie
    .minimize = Zobraziť tlačidlo minimalizácie
    .active-window-hint = Zobraziť zvýraznenie aktívneho okna

focus-navigation = Zameranie okna
    .focus-follows-cursor = Zameranie sleduje kurzor
    .focus-follows-cursor-delay = Zameranie sleduje kurzor s opozdením
    .cursor-follows-focus = Kurzor sleduje zameranie

## Plocha: Pracovné priestory

workspaces = Pracovné priestory
    .desc = Orientácia a správanie pracovného priestoru.

workspaces-behavior = Správanie pracovného priestoru
    .dynamic = Dynamické pracovné priestory
    .dynamic-desc = Automaticky odstraňuje prázdne pracovné priestory.
    .fixed = Pevný počet pracovných priestorov
    .fixed-desc = Pridajte alebo odstráňte pracovné priestory v prehľade.

workspaces-multi-behavior = Správanie viacerých monitorov
    .span = Pracovné priestory pokrývajú displeje
    .separate = Displeje majú samostatné pracovné priestory

workspaces-overview-thumbnails = Miniatúry prehľadu pracovného priestoru
    .show-number = Zobraziť číslo pracovného priestoru
    .show-name = Zobraziť názov pracovného priestoru

workspaces-orientation = Orientácia pracovného priestoru
    .vertical = Vertikálna
    .horizontal = Horizontálna

hot-corner = Rohy
    .top-left-corner = Povoliť horný ľavý horúci roh pre pracovné priestory

## Displeje

-requires-restart = Vyžaduje reštart

color = Farba
    .depth = Farebná hĺbka
    .profile = Farebný profil
    .sidebar = Farebné profily
    .temperature = Teplota farieb

display = Displeje
    .desc = Spravovať displeje, prepínanie grafiky a nočné svetlo
    .arrangement = Usporiadanie displejov
    .arrangement-desc = Presunutím displejov ich usporiadate.
    .enable = Povoliť displej
    .external = { $size } { $output } Externý displej
    .laptop = { $size } Displej notebooku
    .options = Možnosti displeja
    .refresh-rate = Obnovovacia frekvencia
    .resolution = Rozlíšenie
    .scale = Mierka
    .additional-scale-options = Ďalšie možnosti škálovania

mirroring = Zrkadlenie
    .id = Zrkadlenie { $id }
    .dont = Nezrkadliť
    .mirror = Zrkadliť { $display }
    .project = Premietať na { $display ->
        [all] všetky displeje
        *[other] { $display }
    }
    .project-count = Premieta sa na { $count} ďalší { $count ->
        [one] displej
        [few] displeje
        *[other] displejov
    }

night-light = Nočné svetlo
    .auto = Automaticky (západ slnka až východ slnka)
    .desc = Znížte modré svetlo teplejšími farbami.

orientation = Orientácia
    .standard = Štandardná
    .rotate-90 = Otočiť o 90
    .rotate-180 = Otočiť o 180
    .rotate-270 = Otočiť o 270

vrr = Variabilná obnovovacia frekvencia
    .enabled = Povolené
    .force = Vždy
    .auto = Automaticky
    .disabled = Zakázané

scheduling = Plánovanie
    .manual = Manuálny rozvrh

dialog = Dialóg
    .title = Zachovať tieto nastavenia displeja?
    .keep-changes = Zachovať zmeny
    .change-prompt = Zmeny nastavení sa automaticky vrátia o { $time } sekúnd.
    .revert-settings = Vrátiť nastavenia

legacy-app-scaling = Škálovanie aplikácií systému okien X11
    .scaled-by-system = Škálovať všetky aplikácie X11
    .system-description = Aplikácie X11 sa na obrazovkách HiDPI zobrazia rozmazane.
    .scaled-natively = Vykresľovať aplikácie X11 v natívnom rozlíšení
    .native-description = Aplikácie X11, ktoré nepodporujú škálovanie, budú pri používaní displejov HiDPI malé. Povoľte, aby hry využívali plné rozlíšenie monitora.

## Zvuk

sound = Zvuk
    .desc = N/A

sound-output = Výstup
    .volume = Hlasitosť výstupu
    .device = Výstupné zariadenie
    .level = Úroveň výstupu
    .config = Konfigurácia
    .balance = Vyváženie

sound-input = Vstup
    .volume = Hlasitosť vstupu
    .device = Vstupné zariadenie
    .level = Úroveň vstupu

sound-alerts = Upozornenia
    .volume = Hlasitosť upozornení
    .sound = Zvuk upozornení

sound-applications = Aplikácie
    .desc = Hlasitosti a nastavenia aplikácií

profile = Profil

## Napájanie

power = Napájanie a batéria
    .desc = Spravovať nastavenia napájania

battery = Batéria
  .minute = { $value } { $value ->
        [one] { $value } minúta
        [few] { $value } minúty
       *[other] { $value } minút
  }
  .hour = { $value } { $value ->
        [one] { $value } hodina
        [few] { $value } hodiny
       *[other] { $value } hodín
  }
  .day = { $value } { $value ->
        [one] { $value } deň
        [few] { $value } dni
       *[other] { $value } dní
  }
  .less-than-minute = Menej ako minúta
  .and = a
  .remaining-time = { $time } do { $action ->
        [full] nabitia
       *[other] vybitia
   }

connected-devices = Pripojené zariadenia
  .unknown = Neznáme zariadenie

power-mode = Režim napájania
    .battery = Predĺžená výdrž batérie
    .battery-desc = Znížené využitie energie a tichý výkon.
    .balanced = Vyvážený
    .balanced-desc = Tichý výkon a mierne využitie energie.
    .performance = Vysoký výkon
    .performance-desc = Špičkový výkon a využitie energie.
    .no-backend = Backend sa nenašiel. Nainštalujte system76-power alebo power-profiles-daemon.

power-saving = Možnosti úspory energie
    .turn-off-screen-after = Vypnúť obrazovku po
    .auto-suspend = Automatické pozastavenie
    .auto-suspend-ac = Automatické pozastavenie pri zapojení do siete
    .auto-suspend-battery = Automatické pozastavenie pri napájaní z batérie

## Vstup

acceleration-desc = Automaticky upravuje citlivosť sledovania na základe rýchlosti.

disable-while-typing = Zakázať počas písania

input-devices = Vstupné zariadenia
    .desc = Vstupné zariadenia

primary-button = Primárne tlačidlo
    .desc = Nastavuje poradie fyzických tlačidiel.
    .left = Vľavo
    .right = Vpravo

scrolling = Posúvanie
    .two-finger = Posúvať dvoma prstami
    .edge = Posúvať pozdĺž okraja jedným prstom
    .speed = Rýchlosť posúvania
    .natural = Prirodzené posúvanie
    .natural-desc = Posúvať obsah namiesto zobrazenia

## Vstup: Klávesnica

slow = Pomalé
fast = Rýchle
short = Krátke
long = Dlhé
keyboard = Klávesnica
    .desc = Vstupné zdroje, prepínanie, zadávanie špeciálnych znakov, skratky.

keyboard-sources = Vstupné zdroje
    .desc = Vstupné zdroje je možné prepínať pomocou kombinácie klávesov Super+Space. Toto je možné prispôsobiť v nastaveniach klávesových skratiek.
    .move-up = Presunúť nahor
    .move-down = Presunúť nadol
    .settings = Nastavenia
    .view-layout = Zobraziť rozloženie klávesnice
    .remove = Odstrániť
    .add = Pridať vstupný zdroj

keyboard-special-char = Zadanie špeciálneho znaku
    .alternate = Kláves alternatívnych znakov
    .compose = Kláves Compose
    .caps = Kláves Caps Lock

keyboard-typing-assist = Písanie
    .repeat-rate = Rýchlosť opakovania
    .repeat-delay = Oneskorenie opakovania

added = Pridané
type-to-search = Píšte pre vyhľadávanie...
show-extended-input-sources = Zobraziť rozšírené vstupné zdroje

## Vstup: Klávesnica: Skratky

keyboard-shortcuts = Klávesové skratky
    .desc = Zobraziť a prispôsobiť skratky

add-keybinding = Pridajte klávesovú skratku
cancel = Zrušiť
command = Príkaz
custom = Vlastné
debug = Debugovať
disabled = Zakázané
migrate-workspace-prev = Migrovať pracovný priestor na predchádzajúci výstup
migrate-workspace-next = Migrovať pracovný priestor na nasledujúci výstup
migrate-workspace = Migrovať pracovný priestor na výstup { $direction ->
    *[down] dole
    [left] vľavo
    [right] vpravo
    [up] hore
}
navigate = Navigovať
replace = Nahradiť
shortcut-name = Názov skratky
system-controls = Systémové ovládacie prvky
terminate = Ukončiť
toggle-stacking = Prepnúť usporiadanie okien
type-key-combination = Zadajte kombináciu klávesov

custom-shortcuts = Vlastné skratky
    .add = Pridať skratku
    .context = Pridať vlastnú skratku
    .none = Žiadne vlastné skratky

modified = { $count } upravené

nav-shortcuts = Navigácia
    .prev-output = Zamerať sa na predchádzajúci výstup
    .next-output = Zamerať sa na nasledujúci výstup
    .last-workspace = Zamerať sa na posledný pracovný priestor
    .prev-workspace = Zamerať sa na predchádzajúci pracovný priestor
    .next-workspace = Zamerať sa na nasledujúci pracovný priestor
    .focus = Zamerať sa na okno { $direction ->
        *[down] dole
        [in] dovnútra
        [left] vľavo
        [out] von
        [right] vpravo
        [up] hore
    }
    .output = Prepnúť na výstup { $direction ->
        *[down] dole
        [left] vľavo
        [right] vpravo
        [up] hore
    }
    .workspace = Prepnúť na pracovný priestor { $num }

manage-windows = Spravovať okná
    .close = Zatvoriť okno
    .maximize = Maximalizovať okno
    .minimize = Minimalizovať okno
    .resize-inwards = Zmenšiť okno dovnútra
    .resize-outwards = Zväčšiť okno von
    .toggle-sticky = Prepnúť prilepené okno

move-windows = Presunúť okná
    .direction = Presunúť okno { $direction ->
        *[down] dole
        [left] vľavo
        [right] vpravo
        [up] hore
    }
    .display = Presunúť okno o jeden monitor { $direction ->
        *[down] dole
        [left] vľavo
        [right] vpravo
        [up] hore
    }
    .workspace = Presunúť okno o jeden pracovný priestor { $direction ->
        *[below] dole
        [left] vľavo
        [right] vpravo
        [above] hore
    }
    .workspace-num = Presunúť okno do pracovného priestoru { $num }
    .prev-workspace = Presunúť okno do predchádzajúceho pracovného priestoru
    .next-workspace = Presunúť okno do nasledujúceho pracovného priestoru
    .last-workspace = Presunúť okno do posledného pracovného priestoru
    .next-display = Presunúť okno na nasledujúci displej
    .prev-display = Presunúť okno na predchádzajúci displej
    .send-to-prev-workspace = Presunúť okno do predchádzajúceho pracovného priestoru
    .send-to-next-workspace = Presunúť okno do nasledujúceho pracovného priestoru

system-shortcut = Systém
    .app-library = Otvoriť knižnicu aplikácií
    .brightness-down = Znížiť jas displeja
    .brightness-up = Zvýšiť jas displeja
    .home-folder = Otvoriť domovský priečinok
    .keyboard-brightness-down = Znížiť jas klávesnice
    .keyboard-brightness-up = Zvýšiť jas klávesnice
    .launcher = Otvoriť spúšťač
    .log-out = Odhlásiť sa
    .lock-screen = Zamknúť obrazovku
    .mute = Stlmiť zvukový výstup
    .mute-mic = Stlmiť vstup mikrofónu
    .play-pause = Prehrať/Pozastaviť
    .play-next = Nasledujúca skladba
    .play-prev = Predchádzajúca skladba
    .screenshot = Urobiť snímku obrazovky
    .terminal = Otvoriť terminál
    .volume-lower = Znížiť hlasitosť zvukového výstupu
    .volume-raise = Zvýšiť hlasitosť zvukového výstupu
    .web-browser = Otvoriť webový prehliadač
    .window-switcher = Prepínať medzi otvorenými oknami
    .window-switcher-previous = Prepínať medzi otvorenými oknami v opačnom poradí
    .workspace-overview = Otvoriť prehľad pracovných priestorov

window-tiling = Usporiadanie okien
    .horizontal = Nastaviť horizontálnu orientáciu
    .vertical = Nastaviť vertikálnu orientáciu
    .swap-window = Vymeniť okno
    .toggle-tiling = Prepnúť usporiadanie okien
    .toggle-stacking = Prepnúť usporiadanie okien
    .toggle-floating = Prepnúť plávajúce okno
    .toggle-orientation = Prepnúť orientáciu

replace-shortcut-dialog = Nahradiť skratku?
    .desc = { $shortcut } sa používa pre { $name }. Ak ju nahradíte, { $name } sa zakáže.

zoom-in = Priblížiť
zoom-out = Oddialiť

## Vstup: Myš

mouse = Myš
    .desc = Rýchlosť myši, zrýchlenie, prirodzené posúvanie.
    .speed = Rýchlosť myši
    .acceleration = Povoliť zrýchlenie myši

## Vstup: Touchpad

click-behavior = Správanie kliknutia
    .click-finger = Sekundárne kliknutie dvoma prstami a stredné kliknutie tromi prstami
    .button-areas = Sekundárne kliknutie v pravom dolnom rohu a stredné kliknutie v dolnej časti v strede

pinch-to-zoom = Priblíženie gestom
    .desc = Použite dva prsty na priblíženie obsahu pre aplikácie, ktoré podporujú priblíženie.

tap-to-click = Kliknutie ťuknutím
    .desc = Umožňuje ťuknutie jedným prstom pre primárne kliknutie, ťuknutie dvoma prstami pre sekundárne kliknutie a ťuknutie tromi prstami pre stredné kliknutie.

touchpad = Touchpad
    .acceleration = Povoliť zrýchlenie touchpadu
    .desc = Rýchlosť touchpadu, možnosti kliknutia, gestá.
    .speed = Rýchlosť touchpadu

## Vstup: Gestá

gestures = Gestá
    .four-finger-down = Potiahnutie štyrmi prstami nadol
    .four-finger-left = Potiahnutie štyrmi prstami doľava
    .four-finger-right = Potiahnutie štyrmi prstami doprava
    .four-finger-up = Potiahnutie štyrmi prstami nahor
    .three-finger-any = Potiahnutie tromi prstami ľubovoľným smerom

switch-workspaces = Prepínať pracovné priestory
    .horizontal = Potiahnutie štyrmi prstami doľava/doprava
    .vertical = Potiahnutie štyrmi prstami nahor/nadol

switch-between-windows = Prepínať medzi oknami
open-application-library = Otvoriť knižnicu aplikácií
open-workspaces-view = Otvoriť zobrazenie pracovných priestorov

## Čas a jazyk

time = Čas a jazyk
    .desc = N/A

time-date = Dátum a čas
    .desc = Časové pásmo, automatické nastavenia hodín a niektoré formátovania času.
    .auto = Nastaviť automaticky
    .auto-ntp = Dátum a čas sa automaticky aktualizujú, keď je nastavené časové pásmo.

time-zone = Časové pásmo
    .auto = Automatické časové pásmo
    .auto-info = Vyžaduje služby určovania polohy a prístup na internet

time-format = Formát dátumu a času
    .twenty-four = 24-hodinový čas
    .show-seconds = Zobraziť sekundy
    .first = Prvý deň v týždni
    .show-date = Zobraziť dátum na hornom paneli
    .friday = Piatok
    .saturday = Sobota
    .sunday = Nedeľa
    .monday = Pondelok

time-region = Oblasť a jazyk
    .desc = Formátovať dátumy, časy a čísla na základe vašej oblasti

formatting = Formátovanie
    .dates = Dátumy
    .time = Čas
    .date-and-time = Dátum a čas
    .numbers = Čísla
    .measurement = Meranie
    .paper = Papier

preferred-languages = Preferované jazyky
    .desc = Poradie jazykov určuje, ktorý jazyk sa použije na preklad pracovnej plochy. Zmeny sa prejavia pri nasledujúcom prihlásení.

add-language = Pridať jazyk
    .context = Pridať jazyk
install-additional-languages = Nainštalovať ďalšie jazyky
region = Oblasť

## Systém

system = Systém a účty

## Systém: Informácie

about = Informácie
    .desc = Názov zariadenia, informácie o hardvéri, predvolené nastavenia operačného systému.

about-device = Názov zariadenia
    .desc = Tento názov sa zobrazuje ostatným sieťovým zariadeniam alebo zariadeniam Bluetooth.

about-hardware = Hardvér
    .model = Model hardvéru
    .memory = Pamäť
    .processor = Procesor
    .graphics = Grafika
    .disk-capacity = Kapacita disku

about-os = Operačný systém
    .os = Operačný systém
    .os-architecture = Architektúra operačného systému
    .desktop-environment = Prostredie pracovnej plochy
    .windowing-system = Systém okien

about-related = Súvisiace nastavenia
    .support = Získať podporu

## Systém: Firmvér

firmware = Firmvér
    .desc = Podrobnosti o firmvéri.

## Systém: Používatelia

users = Používatelia
    .desc = Autentifikácia a používateľské účty.
    .admin = Správca
    .standard = Štandardný
    .profile-add = Vyberte profilový obrázok

administrator = Správca
    .desc = Správcovia môžu meniť nastavenia pre všetkých používateľov, pridávať a odstraňovať ostatných používateľov.

add-user = Pridať používateľa
remove-user = Odstrániť používateľa
full-name = Celé meno

## Systém: Predvolené aplikácie

default-apps = Predvolené aplikácie
    .desc = Predvolený webový prehliadač, poštový klient, prehliadač súborov a ďalšie aplikácie.
    .web-browser = Webový prehliadač
    .file-manager = Správca súborov
    .mail-client = Poštový klient
    .music = Hudba
    .video = Video
    .photos = Fotografie
    .calendar = Kalendár
    .terminal = Terminál
    .other-associations = Ďalšie priradenia
    .text-editor = Textový editor
