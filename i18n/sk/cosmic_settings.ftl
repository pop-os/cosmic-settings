app = Nastavenia COSMIC
dbus-connection-error = Nepodarilo sa pripojiť k DBus
ok = OK
unknown = Neznáme
number = { $number }

## Sieť & Bezdrôtové

connections-and-profiles =
    { $variant ->
        [wired] Káblové
        [wifi] Wi-Fi
        [vpn] VPN
       *[other] Neznáme
    } pripojenia a profily pripojení.
add-network = Pridať sieť
    .profile = Pridať profil
add-vpn = Pridať VPN
airplane-on = Režim lietadla je zapnutý.
cable-unplugged = Kábel odpojený
connect = Pripojiť
connected = Pripojené
connecting = Pripája sa…
disconnect = Odpojiť
forget = Zabudnúť
known-networks = Známe siete
network-and-wireless = Sieť & Bezdrôtové
no-networks = Neboli nájdené žiadne siete.
no-vpn = Nie sú dostupné žiadne VPN pripojenia.
password = Heslo
password-confirm = Potvrdiť heslo
remove = Odstrániť
settings = Nastavenia
username = Používateľské meno
visible-networks = Viditeľné siete
identity = Identita
auth-dialog = Vyžaduje sa autentifikácia
    .vpn-description = Zadajte používateľské meno a heslo vyžadované službou VPN.
    .wifi-description = Zadajte heslo alebo šifrovací kľúč. Môžete sa tiež pripojiť stlačením tlačidla „WPS“ na routeri.
forget-dialog = Zabudnúť túto Wi-Fi sieť?
    .description = Ak budete chcieť túto Wi-Fi sieť v budúcnosti používať, budete musieť znova zadať heslo.
network-device-state =
    .activated = Pripojené
    .config = Pripája sa
    .deactivating = Odpája sa
    .disconnected = Odpojené
    .failed = Nepodarilo sa pripojiť
    .ip-check = Kontroluje sa pripojenie
    .ip-config = Vyžiadanie IP a smerovacích informácií
    .need-auth = Vyžaduje autentifikáciu
    .prepare = Pripravuje sa pripojenie
    .secondaries = Čaká sa na sekundárne pripojenie
    .unavailable = Nedostupné
    .unknown = Neznámy stav
    .unmanaged = Nespravované
    .unplugged = Kábel odpojený
remove-connection-dialog = Odstrániť profil pripojenia?
    .vpn-description = Ak budete chcieť túto sieť v budúcnosti používať, budete musieť znova zadať heslo.
    .wired-description = Ak budete chcieť tento profil v budúcnosti používať, budete ho musieť znova vytvoriť.
vpn = VPN
    .connections = VPN pripojenia
    .error = Nepodarilo sa pridať VPN konfiguráciu
    .remove = Odstrániť profil pripojenia
    .select-file = Vybrať konfiguračný súbor VPN
vpn-error = Chyba VPN
    .config = Nepodarilo sa pridať VPN konfiguráciu
    .connect = Nepodarilo sa pripojiť k VPN
    .connection-editor = Editor pripojenia zlyhal
    .connection-settings = Nepodarilo sa získať nastavenia aktívnych pripojení
    .updating-state = Nepodarilo sa aktualizovať stav správcu siete
    .wireguard-config-path = Neplatná cesta k súboru WireGuard konfigurácie
    .wireguard-config-path-desc = Vybraný súbor musí byť na lokálnom súborovom systéme.
    .wireguard-device = Nepodarilo sa vytvoriť zariadenie WireGuard
    .with-password =
        Nepodarilo sa nastaviť VPN { $field ->
           *[username] používateľské meno
            [password] heslo
            [password-flags] príznaky hesla
        } pomocou nmcli
wired = Káblové
    .adapter = Káblový adaptér { $id }
    .connections = Káblové pripojenia
    .devices = Káblové zariadenia
    .remove = Odstrániť profil pripojenia
wifi = Wi-Fi
    .adapter = Wi-Fi adaptér { $id }
    .forget = Zabudnúť túto sieť
wireguard-dialog = Pridať zariadenie WireGuard
    .description = Vyberte názov zariadenia pre konfiguráciu WireGuard.

## Sieť: Online účty

online-accounts = Online účty
    .desc = Pridať účty, IMAP a SMTP, podnikové prihlásenia

# Bluetooth

activate = Aktivovať
confirm = Potvrdiť
enable = Povoliť
bluetooth = Bluetooth
    .desc = Spravovať Bluetooth zariadenia
    .status = Tento systém je viditeľný ako { $aliases } keď sú nastavenia Bluetooth otvorené.
    .connected = Pripojené
    .connecting = Pripája sa
    .disconnecting = Odpája sa
    .connect = Pripojiť
    .disconnect = Odpojiť
    .forget = Zabudnúť
    .dbus-error = Vyskytla sa chyba pri komunikácii s DBus: { $why }
    .disabled = Služba Bluetooth je zakázaná
    .inactive = Služba Bluetooth nie je aktívna
    .unknown = Službu Bluetooth sa nepodarilo aktivovať. Je nainštalovaný BlueZ?
bluetooth-paired = Predtým pripojené zariadenia
    .connect = Pripojiť
    .battery = { $percentage }% batéria
bluetooth-confirm-pin = Potvrdiť Bluetooth PIN
    .description = Potvrďte, že nasledujúci PIN zodpovedá tomu, ktorý je zobrazený na { $device }
bluetooth-available = Blízke zariadenia
bluetooth-adapters = Bluetooth adaptéry

## Prístupnosť

accessibility = Prístupnosť
    .vision = Zrak
    .on = Zapnuté
    .off = Vypnuté
    .unavailable = Nedostupné
    .screen-reader = Čítačka obrazovky
    .high-contrast = Vysoký kontrast
    .invert-colors = Inverzia farieb
    .color-filters = Farebné filtre
hearing = Sluch
    .mono = Prehrávať stereo zvuk ako mono
default = Predvolené
magnifier = Lupa
    .controls =
        Alebo použite tieto skratky: { $zoom_in ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_in } na priblíženie,
        }{ $zoom_out ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_out } na oddialenie,
        }
        Super + rolovanie myšou
    .scroll_controls = Povoliť priblíženie myšou alebo touchpadom pomocou Super + Rolovanie
    .show_overlay = Zobraziť prekrytie lupy
    .increment = Prírastok priblíženia
    .signin = Spustiť lupu pri prihlásení
    .applet = Prepínať lupu v applete na paneli
    .movement = Pohyb zväčšeného zobrazenia
    .continuous = Neustále s kurzorom
    .onedge = Keď kurzor dosiahne okraj
    .centered = Udržiavať kurzor v strede
color-filter = Typ farebného filtra
    .unknown = Neznámy filter aktívny
    .greyscale = Stupne šedej
    .deuteranopia = Zelená/Červená (slabosť zelenej, Deuteranopia)
    .protanopia = Červená/Zelená (slabosť červenej, Protanopia)
    .tritanopia = Modrá/Žltá (slabosť modrej, Tritanopia)

## Pracovná plocha

desktop = Pracovná plocha

## Pracovná plocha: Tapeta

wallpaper = Tapeta
    .change = Zmeniť obrázok každých
    .desc = Obrázky tapiet, farby a možnosti prezentácie.
    .fit = Prispôsobenie tapety
    .folder-dialog = Vybrať priečinok s tapetami
    .image-dialog = Vybrať obrázok tapety
    .plural = Tapety
    .same = Rovnaká tapeta na všetkých displejoch
    .slide = Prezentácia
add-color = Pridať farbu
add-image = Pridať obrázok
all-displays = Všetky displeje
colors = Farby
dialog-add = Pridať
fill = Vyplniť
fit-to-screen = Prispôsobiť obrazovke
open-new-folder = Otvoriť nový priečinok
recent-folders = Nedávne priečinky
x-minutes =
    { $number } { $number ->
        [one] minúta
        [two] minúty
        [few] minúty
       *[other] minút
    }
x-hours =
    { $number } { $number ->
        [one] hodina
        [two] hodiny
        [few] hodiny
       *[other] hodín
    }
never = Nikdy

## Pracovná plocha: Vzhľad

appearance = Vzhľad
    .desc = Akcentové farby a témy.
accent-color = Akcentová farba
app-background = Pozadie aplikácie alebo okna
auto = Automaticky
close = Zavrieť
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
window-hint-accent-toggle = Použiť akcentovú farbu témy ako zvýraznenie aktívneho okna
auto-switch = Automaticky prepínať medzi svetlým a tmavým režimom
    .sunrise = Prepne na svetlý režim pri východe slnka
    .sunset = Prepne na tmavý režim pri západe slnka
    .next-sunrise = Prepne na svetlý režim pri ďalšom východe slnka
    .next-sunset = Prepne na tmavý režim pri ďalšom západe slnka
container-background = Pozadie kontajnera
    .desc-detail = Farba pozadia kontajnera sa používa pre navigačný bočný panel, bočný zásuvný panel, dialógy a podobné widgety. Predvolene sa automaticky odvádza z pozadia aplikácie alebo okna.
    .reset = Obnoviť na automaticky
    .desc = Primárna farba kontajnera sa používa pre navigačný bočný panel, bočný zásuvný panel, dialógy a podobné widgety.
control-tint = Odtieň ovládacích prvkov
    .desc = Používa sa pre pozadia štandardných tlačidiel, vyhľadávacích polí, textových polí a podobných komponentov.
frosted = Efekt matného skla na systémovom rozhraní
    .desc = Aplikuje rozmazanie pozadia na panel, dok, applety, launcher a knižnicu aplikácií.
enable-export = Použiť túto tému v GNOME aplikáciách.
    .desc = Nie všetky toolkit-y podporujú automatické prepínanie. Ne-COSMIC aplikácie je potrebné po zmene témy reštartovať.
icon-theme = Téma ikon
    .desc = Použije inú sadu ikon pre aplikácie.
text-tint = Odtieň textu rozhrania
    .desc = Farba použitá na odvodenie farieb textu rozhrania s dostatočným kontrastom na rôznych povrchoch.
style = Štýl
    .round = Okrúhly
    .slightly-round = Mierne okrúhly
    .square = Štvorcový
interface-density = Hustota rozhrania
    .comfortable = Pohodlné
    .compact = Kompaktné
    .spacious = Priestranné
window-management-appearance = Správa okien
    .active-hint = Veľkosť zvýraznenia aktívneho okna
    .gaps = Medzery okolo dlaždicových okien

### Experimentálne

experimental-settings = Experimentálne nastavenia
icons-and-toolkit = Ikony a témy toolkitov
interface-font = Systémové písmo
monospace-font = Monospace písmo

## Pracovná plocha: Oznámenia

notifications = Oznámenia
    .desc = Nerušiť, oznámenia na uzamknutej obrazovke a nastavenia pre aplikácie.

## Pracovná plocha: Panel

panel = Panel
    .desc = Hlavný systémový panel pre menu a applety.
add = Pridať
add-applet = Pridať applet
all = Všetko
applets = Applety
center-segment = Stredný segment
drop-here = Sem presuňte applety
end-segment = Koncový segment
large = Veľký
no-applets-found = Neboli nájdené žiadne applety...
panel-bottom = Dole
panel-left = Vľavo
panel-right = Vpravo
panel-top = Hore
search-applets = Hľadať applety...
small = Malý
start-segment = Začiatočný segment
panel-appearance = Vzhľad
    .match = Zhodovať s pracovnou plochou
    .light = Svetlý
    .dark = Tmavý
panel-behavior-and-position = Správanie a pozície
    .autohide = Automaticky skrývať panel
    .dock-autohide = Automaticky skrývať dok
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
    .desc = Konfigurovať applety panelu
panel-missing = Konfigurácia panelu chýba
    .desc = Konfiguračný súbor panelu chýba kvôli vlastnej konfigurácii alebo je poškodený.
    .fix = Obnoviť predvolené

## Pracovná plocha: Dok

dock = Dok
    .desc = Voliteľný panel pre aplikácie a applety.

## Pracovná plocha: Správa okien

window-management = Správa okien
    .desc = Akcia klávesu Super, možnosti ovládania okien a ďalšie možnosti dlaždicovania okien.
super-key = Akcia klávesu Super
    .launcher = Otvoriť launcher
    .workspaces = Otvoriť pracovné priestory
    .applications = Otvoriť aplikácie
    .disable = Zakázať
edge-gravity = Plávajúce okná sa presúvajú k najbližším okrajom
window-controls = Ovládanie okien
    .maximize = Zobraziť tlačidlo maximalizácie
    .minimize = Zobraziť tlačidlo minimalizácie
    .active-window-hint = Zobraziť zvýraznenie aktívneho okna
focus-navigation = Navigácia zamerania
    .focus-follows-cursor = Zameranie sleduje kurzor
    .focus-follows-cursor-delay = Oneskorenie zamerania sledujúceho kurzor v ms
    .cursor-follows-focus = Kurzor sleduje zameranie

## Pracovná plocha: Pracovné priestory

workspaces = Pracovné priestory
    .desc = Orientácia a správanie pracovných priestorov.
workspaces-behavior = Správanie pracovných priestorov
    .dynamic = Dynamické pracovné priestory
    .dynamic-desc = Automaticky odstraňuje prázdne pracovné priestory.
    .fixed = Pevný počet pracovných priestorov
    .fixed-desc = Pridajte alebo odstráňte pracovné priestory v prehľade.
workspaces-multi-behavior = Správanie na viacerých monitoroch
    .span = Pracovné priestory na všetkých displejoch
    .separate = Displeje majú samostatné pracovné priestory
workspaces-overview-thumbnails = Náhľady pracovných priestorov
    .show-number = Zobraziť číslo pracovného priestoru
    .show-name = Zobraziť názov pracovného priestoru
workspaces-orientation = Orientácia pracovných priestorov
    .vertical = Vertikálna
    .horizontal = Horizontálna
hot-corner = Horúci roh
    .top-left-corner = Povoliť horúci roh vľavo hore pre pracovné priestory

## Displeje

-requires-restart = Vyžaduje reštart
color = Farba
    .depth = Hĺbka farieb
    .profile = Farebný profil
    .sidebar = Farebné profily
    .temperature = Teplota farieb
display = Displeje
    .desc = Spravovať displeje a nočný režim
    .arrangement = Usporiadanie displejov
    .arrangement-desc = Presuňte displeje na zmenu usporiadania.
    .enable = Povoliť displej
    .external = { $size } { $output } externý displej
    .laptop = { $size } displej notebooku
    .options = Možnosti displeja
    .refresh-rate = Obnovovacia frekvencia
    .resolution = Rozlíšenie
    .scale = Mierka
    .additional-scale-options = Ďalšie možnosti mierky
mirroring = Zrkadlenie
    .id = Zrkadlenie { $id }
    .dont = Nezrkadliť
    .mirror = Zrkadliť { $display }
    .project =
        Projekcia na { $display ->
            [all] všetky displeje
           *[other] { $display }
        }
    .project-count =
        Projekcia na { $count } ďalšie { $count ->
            [one] displej
            [two] displeje
            [few] displeje
           *[other] displejov
        }
night-light = Nočné svetlo
    .auto = Automaticky (od západu do východu slnka)
    .desc = Znížte modré svetlo teplejšími farbami.
orientation = Orientácia
    .standard = Štandardná
    .rotate-90 = Otočiť o 90°
    .rotate-180 = Otočiť o 180°
    .rotate-270 = Otočiť o 270°
vrr = Premenlivá obnovovacia frekvencia
    .enabled = Povolené
    .force = Vždy
    .auto = Automaticky
    .disabled = Zakázané
scheduling = Plánovanie
    .manual = Manuálny plán
dialog = Dialóg
    .title = Ponechať tieto nastavenia displeja?
    .keep-changes = Ponechať zmeny
    .change-prompt = Zmeny nastavení sa automaticky vrátia za { $time } sekúnd.
    .revert-settings = Vrátiť nastavenia

## Zvuk

sound = Zvuk
    .desc = N/A
sound-output = Výstup
    .volume = Hlasitosť výstupu
    .device = Výstupné zariadenie
    .level = Úroveň výstupu
    .config = Konfigurácia
    .balance = Balancia
    .left = Vľavo
    .right = Vpravo
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

power = Napájanie & Batéria
    .desc = Spravovať nastavenia napájania
battery = Batéria
    .minute =
        { $value } { $value ->
            [one] minúta
            [two] minúty
            [few] minúty
           *[other] minút
        }
    .hour =
        { $value } { $value ->
            [one] hodina
            [two] hodiny
            [few] hodiny
           *[other] hodín
        }
    .day =
        { $value } { $value ->
            [one] deň
            [two] dni
            [few] dni
           *[other] dní
        }
    .less-than-minute = Menej ako minúta
    .and = a
    .remaining-time =
        { $time } do { $action ->
            [full] plného nabitia
           *[other] vybitia
        }
connected-devices = Pripojené zariadenia
    .unknown = Neznáme zariadenie
power-mode = Režim napájania
    .battery = Predĺžená výdrž batérie
    .battery-desc = Znížená spotreba energie a tichý výkon.
    .balanced = Vyvážený
    .balanced-desc = Tichý výkon a stredná spotreba energie.
    .performance = Vysoký výkon
    .performance-desc = Maximálny výkon a spotreba energie.
    .no-backend = Backend nenájdený. Nainštalujte system76-power alebo power-profiles-daemon.
power-saving = Možnosti úspory energie
    .turn-off-screen-after = Vypnúť obrazovku po
    .auto-suspend = Automatické uspatie
    .auto-suspend-ac = Automatické uspatie pri pripojení do siete
    .auto-suspend-battery = Automatické uspatie na batériu

## Vstup

acceleration-desc = Automaticky upravuje citlivosť sledovania podľa rýchlosti.
disable-while-typing = Deaktivovať počas písania
input-devices = Vstupné zariadenia
    .desc = Vstupné zariadenia
primary-button = Primárne tlačidlo
    .desc = Nastavuje poradie fyzických tlačidiel.
    .left = Vľavo
    .right = Vpravo
scrolling = Rolovanie
    .two-finger = Rolovať dvoma prstami
    .edge = Rolovať po okraji jedným prstom
    .speed = Rýchlosť rolovania
    .natural = Prirodzené rolovanie
    .natural-desc = Roluje obsah, namiesto pohľadu

## Vstup: Klávesnica

slow = Pomalé
fast = Rýchle
short = Krátke
long = Dlhé
keyboard = Klávesnica
    .desc = Vstupné zdroje, prepínanie, zadávanie špeciálnych znakov, skratky.
keyboard-sources = Vstupné zdroje
    .desc = Vstupné zdroje je možné prepínať kombináciou klávesov Super+Medzerník. Toto je možné upraviť v nastaveniach klávesových skratiek.
    .move-up = Posunúť vyššie
    .move-down = Posunúť nižšie
    .settings = Nastavenia
    .view-layout = Zobraziť rozloženie klávesnice
    .remove = Odstrániť
    .add = Pridať vstupný zdroj
keyboard-special-char = Zadávanie špeciálnych znakov
    .alternate = Kláves alternatívnych znakov
    .compose = Kompozičný kláves
    .caps = Kláves Caps Lock
keyboard-typing-assist = Písanie
    .repeat-rate = Rýchlosť opakovania
    .repeat-delay = Oneskorenie opakovania
keyboard-numlock-boot = Numlock
    .boot-state = Stav pri štarte
    .last-boot = Posledný štart
    .on = Zapnuté
    .off = Vypnuté
    .set = Nastaviť stav numlock pri štarte
added = Pridané
type-to-search = Začnite písať pre vyhľadávanie...
show-extended-input-sources = Zobraziť rozšírené vstupné zdroje

## Vstup: Klávesnica: Skratky

keyboard-shortcuts = Klávesové skratky
    .desc = Zobraziť a upraviť skratky
add-another-keybinding = Pridať ďalšiu klávesovú skratku
cancel = Zrušiť
command = Príkaz
custom = Vlastné
debug = Debug
disabled = Zakázané
input-source-switch = Prepínať jazyk vstupného zdroja klávesnice
migrate-workspace-prev = Presunúť pracovný priestor na predchádzajúci výstup
migrate-workspace-next = Presunúť pracovný priestor na ďalší výstup
migrate-workspace =
    Presunúť pracovný priestor na výstup { $direction ->
       *[down] dole
        [left] vľavo
        [right] vpravo
        [up] hore
    }
navigate = Navigovať
replace = Nahradiť
shortcut-name = Názov skratky
system-controls = Systémové ovládanie
terminate = Ukončiť
toggle-stacking = Prepínať vrstvenie okien
type-key-combination = Zadajte kombináciu klávesov
custom-shortcuts = Vlastné skratky
    .add = Pridať skratku
    .context = Pridať vlastnú skratku
    .none = Žiadne vlastné skratky
modified = { $count } upravené
nav-shortcuts = Navigácia
    .prev-output = Zamerať predchádzajúci výstup
    .next-output = Zamerať ďalší výstup
    .last-workspace = Zamerať posledný pracovný priestor
    .prev-workspace = Zamerať predchádzajúci pracovný priestor
    .next-workspace = Zamerať ďalší pracovný priestor
    .focus =
        Zamerať okno { $direction ->
           *[down] dole
            [in] dovnútra
            [left] vľavo
            [out] von
            [right] vpravo
            [up] hore
        }
    .output =
        Prepínať na výstup { $direction ->
           *[down] dole
            [left] vľavo
            [right] vpravo
            [up] hore
        }
    .workspace = Prepínať na pracovný priestor { $num }
manage-windows = Spravovať okná
    .close = Zavrieť okno
    .maximize = Maximalizovať okno
    .fullscreen = Celá obrazovka
    .minimize = Minimalizovať okno
    .resize-inwards = Zmeniť veľkosť okna dovnútra
    .resize-outwards = Zmeniť veľkosť okna von
    .toggle-sticky = Prepínať pripnutie okna
move-windows = Presunúť okná
    .direction =
        Presunúť okno { $direction ->
           *[down] dole
            [left] vľavo
            [right] vpravo
            [up] hore
        }
    .display =
        Presunúť okno o jeden monitor { $direction ->
           *[down] dole
            [left] vľavo
            [right] vpravo
            [up] hore
        }
    .workspace =
        Presunúť okno o jeden pracovný priestor { $direction ->
           *[below] nižšie
            [left] vľavo
            [right] vpravo
            [above] vyššie
        }
    .workspace-num = Presunúť okno na pracovný priestor { $num }
    .prev-workspace = Presunúť okno na predchádzajúci pracovný priestor
    .next-workspace = Presunúť okno na ďalší pracovný priestor
    .last-workspace = Presunúť okno na posledný pracovný priestor
    .next-display = Presunúť okno na ďalší displej
    .prev-display = Presunúť okno na predchádzajúci displej
    .send-to-prev-workspace = Presunúť okno na predchádzajúci pracovný priestor
    .send-to-next-workspace = Presunúť okno na ďalší pracovný priestor
system-shortcut = Systém
    .app-library = Otvoriť knižnicu aplikácií
    .brightness-down = Znížiť jas displeja
    .brightness-up = Zvýšiť jas displeja
    .home-folder = Otvoriť domovský priečinok
    .keyboard-brightness-down = Znížiť jas klávesnice
    .keyboard-brightness-up = Zvýšiť jas klávesnice
    .launcher = Otvoriť launcher
    .log-out = Odhlásiť sa
    .lock-screen = Uzamknúť obrazovku
    .mute = Stlmiť zvuk
    .mute-mic = Stlmiť mikrofón
    .play-pause = Prehrať/Pozastaviť
    .play-next = Ďalšia skladba
    .play-prev = Predchádzajúca skladba
    .poweroff = Vypnúť
    .screenshot = Urobiť snímku obrazovky
    .terminal = Otvoriť terminál
    .volume-lower = Znížiť hlasitosť
    .volume-raise = Zvýšiť hlasitosť
    .web-browser = Otvoriť webový prehliadač
    .window-switcher = Prepínať medzi otvorenými oknami
    .window-switcher-previous = Prepínať medzi otvorenými oknami opačne
    .workspace-overview = Otvoriť prehľad pracovných priestorov
window-tiling = Dlaždicovanie okien
    .horizontal = Nastaviť horizontálnu orientáciu
    .vertical = Nastaviť vertikálnu orientáciu
    .swap-window = Vymeniť okno
    .toggle-tiling = Prepínať dlaždicovanie okien
    .toggle-stacking = Prepínať vrstvenie okien
    .toggle-floating = Prepínať plávajúce okno
    .toggle-orientation = Prepínať orientáciu
replace-shortcut-dialog = Nahradiť skratku?
    .desc = { $shortcut } je používaná { $name }. Ak ju nahradíte, { $name } bude deaktivovaná.
zoom-in = Priblížiť
zoom-out = Oddialiť

## Vstup: Myš

mouse = Myš
    .desc = Rýchlosť myši, akcelerácia, prirodzené rolovanie.
    .speed = Rýchlosť myši
    .acceleration = Povoliť akceleráciu myši

## Vstup: Touchpad

click-behavior = Správanie kliknutia
    .click-finger = Sekundárne kliknutie dvoma prstami a stredné kliknutie tromi prstami
    .button-areas = Sekundárne kliknutie v pravom dolnom rohu a stredné kliknutie v strede dole
pinch-to-zoom = Priblíženie gestom
    .desc = Použite dva prsty na priblíženie obsahu, ak to aplikácia podporuje.
tap-to-click = Kliknutie ťuknutím
    .desc = Povolené jedno-prstové ťuknutie pre primárne kliknutie, dvoj-prstové pre sekundárne a troj-prstové pre stredné kliknutie.
touchpad = Touchpad
    .acceleration = Povoliť akceleráciu touchpadu
    .desc = Rýchlosť touchpadu, možnosti kliknutia, gestá.
    .speed = Rýchlosť touchpadu

## Vstup: Gestá

gestures = Gestá
    .four-finger-down = Švih štyrmi prstami dole
    .four-finger-left = Švih štyrmi prstami vľavo
    .four-finger-right = Švih štyrmi prstami vpravo
    .four-finger-up = Švih štyrmi prstami hore
    .three-finger-any = Švih tromi prstami akýmkoľvek smerom
switch-workspaces = Prepínať pracovné priestory
    .horizontal = Švih štyrmi prstami vľavo/vpravo
    .vertical = Švih štyrmi prstami hore/dole
switch-between-windows = Prepínať medzi oknami
open-application-library = Otvoriť knižnicu aplikácií
open-workspaces-view = Otvoriť prehľad pracovných priestorov

## Čas & Jazyk

time = Čas & Jazyk
    .desc = N/A
time-date = Dátum & Čas
    .desc = Časové pásmo, automatické nastavenie hodín a niektoré formáty času.
    .auto = Nastaviť automaticky
    .auto-ntp = Dátum a čas sa automaticky aktualizujú po nastavení časového pásma.
time-zone = Časové pásmo
    .auto = Automatické časové pásmo
    .auto-info = Vyžaduje lokalizačné služby a prístup na internet
time-format = Formát dátumu & času
    .twenty-four = 24-hodinový čas
    .show-seconds = Zobraziť sekundy
    .first = Prvý deň v týždni
    .show-date = Zobraziť dátum v applete času
    .friday = Piatok
    .saturday = Sobota
    .sunday = Nedeľa
    .monday = Pondelok
time-region = Región & Jazyk
    .desc = Formátovanie dátumov, časov a čísel podľa vášho regiónu.
formatting = Formátovanie
    .dates = Dátumy
    .time = Čas
    .date-and-time = Dátum & Čas
    .numbers = Čísla
    .measurement = Meranie
    .paper = Papier
preferred-languages = Preferované jazyky
    .desc = Poradie jazykov určuje, ktorý jazyk sa použije pre používateľské rozhranie. Zmeny sa prejavia po ďalšom prihlásení.
add-language = Pridať jazyk
    .context = Pridať jazyk
install-additional-languages = Inštalovať ďalšie jazyky
region = Región

## Aplikácie

applications = Aplikácie

## Aplikácie: Predvolené aplikácie

default-apps = Predvolené aplikácie
    .desc = Predvolený webový prehliadač, e-mailový klient, správca súborov a ďalšie aplikácie.
    .web-browser = Webový prehliadač
    .file-manager = Správca súborov
    .mail-client = E-mailový klient
    .music = Hudba
    .video = Video
    .photos = Fotky
    .calendar = Kalendár
    .terminal = Terminál
    .other-associations = Ďalšie asociácie
    .text-editor = Textový editor

## Aplikácie: Spúšťané aplikácie

startup-apps = Spúšťané aplikácie
    .desc = Nastaviť aplikácie, ktoré sa spúšťajú pri prihlásení.
    .add = Pridať aplikáciu
    .user = Aplikácie spustené pri prihlásení
    .none = Neboli pridané žiadne spúšťané aplikácie
    .remove-dialog-title = Odstrániť { $name }?
    .remove-dialog-description = Naozaj chcete odstrániť túto spúšťanú aplikáciu?
    .search-for-application = Hľadať aplikáciu

## Aplikácie: Legacy aplikácie

legacy-applications = Kompatibilita X11 aplikácií
    .desc = Škálovanie X11 aplikácií a globálne skratky.
legacy-app-global-shortcuts = Globálne skratky v X11 aplikáciách
    .desc = Globálne skratky umožňujú, aby stlačenia klávesov a tlačidiel myši vykonané v aplikáciách boli rozpoznané inými aplikáciami pre funkcie ako push-to-talk alebo push-to-mute. Predvolene je to v X11 aplikáciách zakázané, aby iné aplikácie nemohli monitorovať klávesové a myšové udalosti obsahujúce citlivé informácie.
    .none = Žiadne klávesy
    .modifiers = Modifikátory (Super, Shift, Control, Alt)
    .combination = Všetky klávesy pri stlačených modifikátoroch Super, Control alebo Alt
    .all = Všetky klávesy
    .mouse = Udalosti tlačidiel myši v X11 aplikáciách
legacy-app-scaling = Škálovanie X11 aplikácií
    .scaled-gaming = Optimalizovať pre hry a aplikácie na celú obrazovku
    .gaming-description = X11 aplikácie sa môžu zdať o niečo väčšie/menšie v porovnaní s Wayland aplikáciami.
    .scaled-applications = Optimalizovať pre aplikácie
    .applications-description = Hry a aplikácie na celú obrazovku X11 nemusia zodpovedať rozlíšeniu vášho displeja.
    .scaled-compatibility = Maximálny režim kompatibility
    .compatibility-description = X11 aplikácie môžu byť na HiDPI obrazovkách rozmazané.
    .preferred-display = Preferovaný displej pre hry a aplikácie na celú obrazovku X11
    .no-display = Žiadny

## Systém

system = Systém & Účty

## Systém: O aplikácii

about = O aplikácii
    .desc = Názov zariadenia, hardvérové informácie, predvolené nastavenia systému.
about-device = Názov zariadenia
    .desc = Tento názov sa zobrazí ostatným sieťovým alebo Bluetooth zariadeniam.
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

## Systém: Firmware

firmware = Firmware
    .desc = Podrobnosti o firmvéri.

## Systém: Používatelia

users = Používatelia
    .desc = Autentifikácia a používateľské účty.
    .admin = Admin
    .standard = Štandardný
    .profile-add = Vybrať profilový obrázok
administrator = Administrátor
    .desc = Administrátori môžu meniť nastavenia pre všetkých používateľov, pridávať a odstraňovať ďalších používateľov.
add-user = Pridať používateľa
change-password = Zmeniť heslo
remove-user = Odstrániť používateľa
full-name = Celé meno
invalid-username = Neplatné používateľské meno.
password-mismatch = Heslo a potvrdenie sa musia zhodovať.
save = Uložiť
amplification = Zosilnenie
    .desc = Umožňuje zvýšiť hlasitosť až na 150 %.
