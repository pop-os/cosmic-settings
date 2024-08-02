app = Nastavenia COSMIC

unknown = Neznáme

number = { $number }

## Desktop

desktop = Plocha

## Desktop: Appearance

appearance = Vzhľad
    .desc = Farby a COSMIC témy.

accent-color = Farba prvkov
app-background = Pozadie aplikácie alebo okna
auto = Auto
close = Zatvoriť
color-picker = Farebná paleta
copied-to-clipboard = Skopírované do schránky
copy-to-clipboard = Kopírovať do schránky
dark = Tmavý
export = Exportovať
hex = Hex
import = Importovať
light = Svetlý
mode-and-colors = Režim a farby
recent-colors = Posledné farby
reset-to-default = Predvolené nastavenie
rgb = RGB
window-hint-accent = Farba orámovania aktívneho okna
window-hint-accent-toggle = Použiť farbu prvkov pre orámovanie aktívneho okna

auto-switch = Automaticky prepnúť medzi svetlým a tmavým režimom
    .sunrise = Prepnúť na svetlý režim pri východe slnka
    .sunset = Prepnúť na tmavý režim pri západe slnka
    .next-sunrise = Prepne na svetlý režim pri východe slnka
    .next-sunset = Prepne na tmavý režim pri západe slnka

container-background = Pozadie kontajnera
    .desc-detail = Farba pozadia kontajnera sa používa pre bočný panel navigácie, bočné panely, dialógové okná a podobné widgety. Štandardne sa automaticky odvodzuje od pozadia aplikácie alebo okna.
    .reset = Obnoviť automatické
    .desc = Primárna farba kontajnera sa používa pre bočný navigačný panel, bočný panel, dialógové okná a podobné widgety.

control-tint = Odtieň ovládacích komponentov
    .desc = Používa sa pre pozadie štandardných tlačidiel, vyhľadávacích vstupov, textových vstupov a podobných komponentov.

frosted = Efekt matného skla na rozhraní systému
    .desc = Aplikuje rozostrenie pozadia na panel, dok, applety, spúšťač a knižnicu aplikácií.

experimental-settings = Experimentálne nastavenia

enable-export = Použite túto tému na aplikácie GNOME.
    .desc = Nie všetky toolkity podporujú automatické prepínanie. Po zmene motívu môže byť potrebné reštartovať aplikácie, ktoré nie sú navrhnuté pre COSMIC.

icon-theme = Téma ikon
    .desc = Aplikuje na aplikácie inú sadu ikon.

text-tint = Odtieň textu rozhrania
    .desc = Farba používaná na odvodenie farieb textu rozhrania, ktoré majú dostatočný kontrast na rôznych povrchoch.

style = Štýl
    .round = Oblý
    .slightly-round = Mierne zaoblený
    .square = Hranatý

# interface density left out for now
window-management = Správa okien
    .active-hint = Veľkosť orámovania aktívneho okna
    .gaps = Medzery okolo dlaždicových okien

## Desktop: Display

-requires-restart = Vyžaduje reštart

color = Farba
    .depth = Hĺbka farieb
    .profile = Farebný profil
    .sidebar = Farebné profily
    .temperature = Teplota farieb

display = Obrazovka
    .desc = Správa obrazoviek, prepínania grafiky a nočného osvetlenia
    .arrangement = Usporiadanie obrazoviek
    .arrangement-desc = Usporiadajte obrazovky ich presunutím.
    .enable = Povoliť obrazovku
    .external = { $size } { $output } Externá obrazovka
    .laptop = { $size } Obrazovka notebooku
    .options = Možnosti obrazovky
    .refresh-rate = Obnovovacia frekvencia
    .resolution = Rozlíšenie
    .scale = Škálovanie

mirroring = Zrkadlenie
    .id = Zrkadlenie { $id }
    .dont = Nezrkadliť
    .mirror = Zrkadliť { $display }
    .project = Premietať na { $display ->
        [all] všetky obrazovky
        *[other] { $display }
    }
    .project-count = Premietam na { $count} { $count ->
        [1] obrazovku
        [few] obrazovky
        *[other] obrazoviek
    }

night-light = Nočné osvetlenie
    .auto = Automaticky (od západu po východ slnka)
    .desc = Redukuje množstvo modrého svetla teplejšími farbami.

orientation = Orientácia
    .standard = Štandardná
    .rotate-90 = Otočiť o 90 stupňov
    .rotate-180 = Otočiť o 180 stupňov
    .rotate-270 = Otočiť o 90 stupňov

scheduling = Plánovanie
    .manual = Manuálne plánovanie

dialog = Dialog
    .title = Ponechať tieto nastavenia?
    .keep-changes = Ponechať nastavenia
    .change-prompt = Pôvodné nastavenia sa automaticky obnovia o { $time } sekúnd.
    .revert-settings = Obnoviť pôvodné nastavenia

## Desktop: Notifications

notifications = Oznámenia
    .desc = Nerušiť, oznámenia na zamykacej obrazovke a nastavenia pre špecifické aplikácie.

## Desktop: Options

desktop-panel-options = Plocha a panel
    .desc = Správanie klávesy Super, roh obrazovky, nastavenia ovládania okien.

desktop-panels-and-applets = Panely na ploche a applety

dock = Dok
    .desc = Panel s pripnutými aplikáciami.

hot-corner = Rohy
    .top-left-corner = Povoliť použitie ľavého horného rohu pre otvorenie pracovných plôch

super-key = Klávesa Super
    .launcher = Otvoriť spúšťač
    .workspaces = Otvoriť prehľad pracovných plôch
    .applications = Otvoriť aplikácie

top-panel = Horný panel
    .workspaces = Tlačidlo pre zobrazenie pracovných plôch
    .applications = Tlačidlo pre zobrazenie aplikácií

window-controls = Ovládanie okien
    .minimize = Zobraziť tlačidlo minimalizovania
    .maximize = Zobraziť tlačidlo maximalizovania

## Desktop: Panel

panel = Panel
    .desc = Horné pole pre ovládanie plochy a menu.

panel-behavior-and-position = Správanie a pozícia
    .autohide = Automaticky skryť panel
    .dock-autohide = Automaticky skryť dok
    .position = Pozícia na obrazovke
    .display = Zobraziť na displeji

add = Pridať
add-applet = Pridať applet
all = Všetko
applets = Applety
center-segment = Prostredný segment
drop-here = Umiestnite applety sem
end-segment = Koncový segment
large = Veľký
no-applets-found = Neboli nájdené žiadne...
panel-bottom = Dole
panel-left = Vľavo
panel-right = Vpravo
panel-top = Hore
search-applets = Vyhľadať applety...
small = Malý
start-segment = Začiatočný segment

panel-appearance = Vzhľad
    .match = Rovnaký ako plocha
    .light = Svetlý
    .dark = Tmavý


panel-behavior-and-position = Správanie a poloha
    .autohide = Automaticky schovať panel
    .dock-autohide = Automaticky schovať dok
    .position = Pozícia na obrazovke
    .display = Zobraziť na obrazovke

panel-style = Štýl
    .anchor-gap = Miesto medzi panelom a okrajom obrazovky
    .dock-anchor-gap = Miesto medzi dokom a okrajom obrazovky
    .extend = Roztiahnuť panel až k okrajom obrazovky.
    .dock-extend = Roztiahnuť panel až k okrajom obrazovky.
    .appearance = Vzhľad
    .size = Veľkosť
    .background-opacity = Priehľadnosť pozadia

panel-applets = Konfigurácia
    .dock-desc = Konfigurovať applety doku.
    .desc = Konfigurovať applety panelu.

panel-missing = Nastavenia panelu chýbajú
    .desc = Nastavenia panelu chýbajú kvôli použitiu vlastnej témy alebo poškodenému súboru.
    .fix = Použiť predvolené nastavenia

## Desktop: Wallpaper

wallpaper = Pozadie
    .change = Zmeniť pozadie každých
    .desc = Obrázky pozadia, farby a nastavenia prezentácie.
    .fit = Prispôsobenie pozadia
    .folder-dialog = Vybrať priečinok s pozadiami
    .image-dialog = Vybrať obrázok pozadia
    .plural = Pozadia
    .same = Použiť rovnaké pozadie na všetkých obrazovkách
    .slide = Prezentácia

add-color = Pridať farbu
add-image = Pridať obrázok
all-displays = Všetky obrazovky
colors = Farby
dialog-add = Pridať
fill = Vyplniť
fit-to-screen = Prispôsobiť obrazovke
open-new-folder = Otvoriť nový priečinok
recent-folders = Posledné priečinky

x-minutes = { $number } minút
x-hours = { $number ->
    [1] 1 hodinu
    [few] {$number} hodiny
    *[other] { $number } hodín
}

## Desktop: Workspaces

workspaces = Pracovné plochy
    .desc = Nastaviť počet pracovných plôch, správanie a ich pozíciu.

workspaces-behavior = Správanie pracovných plôch
    .dynamic = Dynamické pracovné plochy
    .dynamic-desc = Automaticky odstráni prázdne pracovné plochy.
    .fixed = Pevný počet pracovných plôch
    .fixed-desc = Pridá alebo odstráni počet pracovných plôch.

workspaces-multi-behavior = Správanie pri viacerých monitoroch
    .span = Pracovné plochy obsadia viac monitorov
    .separate = Každý monitor má svoju pracovnú plochu

workspaces-overview-thumbnails = Ukážky pracovných plôch
    .show-number = Zobraziť číslo pracovnej plochy
    .show-name = Zobraziť nazov pracovnej plochy

workspaces-orientation = Orientácia pracovných plôch
    .vertical = Vertikálna
    .horizontal = Horizontálna

## Networking: Wired

wired = Drôtové
    .desc = Drôtové pripojenie, profily pripojenia

## Networking: Online Accounts

online-accounts = Online účty
    .desc = Pridať účty, IMAP a SMTP, firemné prihlásenie

## Time & Language

time = Čas a jazyk
    .desc = N/A

time-date = Dátum a čas
    .desc = Časová zóna, automatické nastavenie času, formátovanie času.
    .auto = Nastaviť automaticky

time-zone = Časová zóna
    .auto = Automatická časová zóna
    .auto-info = Vyžaduje službu polohy a internetu
time-format = Formát dátumu a času
    .twenty-four = 24 hodinový čas
    .first = Prvý deň v týždni
    .show-date = Zobraziť dátum v paneli
    .friday = Piatok
    .saturday = Sobota
    .sunday = Nedeľa
    .monday = Pondelok

time-region = Oblasť a jazyk
    .desc = Formát dátumu, času, a čísel podľa oblasti

## Sound

sound = Zvuk
    .desc = N/A

sound-output = Výstup
    .volume = Výstupná hlasitosť
    .device = Výstupné zariadenia
    .level = Výstupná úroveň
    .config = Nastavenia
    .balance = Vyváženie

sound-input = Vstup
    .volume = Vstupná hlasitosť
    .device = Vstupné zariadenie
    .level = Vstupná úroveň

sound-alerts = Upozornenia
    .volume = Hlasitosť upozornenia
    .sound = Zvuk pre upozornenie

sound-applications = Aplikácie
    .desc = Hlasitosti aplikácií a nastavení

## System

system = Systém a účty

## System: About

about = O systéme
    .desc = Názov zariadenia, hardwarové informácie, predvolené nastavenia operačného systému.

about-device = Názov počítača
    .desc = Toto meno sa zobrazí ostatným sieťovým a bluetooth zariadeniam.

about-hardware = Hardware
    .model = Model hardwaru
    .memory = Pamäť
    .processor = Procesor
    .graphics = Grafika
    .disk-capacity = Kapacita disku

about-os = Operačný systém
    .os = Operačný systém
    .os-architecture = Architektúra OS
    .desktop-environment = Pracovné prostredie
    .windowing-system = Systém na správu okien

about-related = Podobné nastavenia
    .support = Získať podporu

## System: Firmware

firmware = Firmware
    .desc = Podrobnosti o firmware.

## System: Users

users = Používatelia
    .desc = Overenia a prihlásenie, zamknutá obrazovka.

## Input

acceleration-desc = Automaticky upraviť citlivosť sledovania na základe rýchlosti.

disable-while-typing = Zakázať počas písania

input-devices = Vstupné zariadenia
    .desc = Vstupné zariadenia

primary-button = Hlavné tlačidlo
    .left = Ľavé
    .right = Pravé

scrolling = Rolovanie
    .two-finger = Rolovať dvoma prstami
    .edge = Rolovať pozdĺž hrany jedným prstom
    .speed = Rýchlosť rolovania
    .natural = Prirodzené rolovania
    .natural-desc = Prevráti smer rolovania

## Input: Keyboard

slow = Pomaly
fast = Rýchlo
short = Krátko
long = Dlho
keyboard = Klávesnica
    .desc = Vstup z klávesnice

keyboard-sources = Metódy zadávania
    .desc = Metódy zadávania sa dajú prepínať pomocou Super + Medzerník. Toto sa dá zmeniť v nastaveniach klávesových skratiek.
    .move-up = Posunúť hore
    .move-down = Posunúť dole
    .settings = Nastavenia
    .view-layout = Zobraziť rozloženie klávesnice
    .remove = Odstrániť
    .add = Pridať zdroj vstupu
keyboard-special-char = Zadávanie špeciálnych znakov
    .alternate = Klávesa pre náhradné znaky
    .compose = Compose klávesa

keyboard-typing-assist = Písanie
    .repeat-rate = Rýchlosť opakovania
    .repeat-delay = Interval opozdenia

added = Pridané
type-to-search = Píšte pre vyhľadávanie...

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Klávesové skratky
    .desc = Zobraziť a zmeniť klávesové skratky

add-keybinding = Pridajte klávesovú skratku
cancel = Zrušiť
command = Príkaz
custom = Vlastné
debug = Ladenie
disabled = Vypnuté
migrate-workspace-prev = Migrujte pracovný priestor na predchádzajúci výstup
migrate-workspace-next = Migrujte pracovný priestor na ďalší výstup
migrate-workspace = Migrujte pracovný priestor { $direction ->
    *[down] dole
    [left] vľavo
    [right] vpravo
    [up] hore
}
navigate = Navigovať
replace = Nahradiť
shortcut-name = Názov skratky
system-controls = Ovládanie systému
terminate = Zrušiť
toggle-stacking = Prepnúť kaskádovanie okien
type-key-combination = Vložte kombináciu klávesovej skratky
unknown = Neznáme

custom-shortcuts = Vlastná skratka
    .add = Pridať skratku
    .context = Pridať vlastnú skratku
    .none = Žiadne vlastné skratky

modified = { $count } zmenené

nav-shortcuts = Navigácia
    .prev-output = Zamerať sa na predchádzajúci výstup
    .next-output = Zamerať sa na ďalší výstup
    .last-workspace = Zamerať sa na poslednú pracovnú plochu
    .prev-workspace = Zamerať sa na predchádzajúcu pracovnú plochu
    .next-workspace = Zamerať sa na ďalšiu pracovnú plochu
    .focus = Zamerať okno { $direction ->
        *[down] dole
        [in] v
        [left] vľavo
        [out] von
        [right] vpravo
        [up] hore
    }
    .output = Zmeniť na výstup { $direction ->
        *[down] dole
        [left] vľavo
        [right] vpravo
        [up] hore
    }
    .workspace = Prepnúť na pracovnú plochu { $num }

manage-windows = Spravovať okná
    .close = Zatvoriť okno
    .maximize = Maximalizovať okno
    .minimize = Minimalizovať okno
    .resize-inwards = Zmeniť veľkosť okna dovnútra
    .resize-outwards = Zmeniť veľkosť okna von
    .toggle-sticky = Zapnúť lepkavé okná

move-windows = Premiestniť okno
    .direction = Presunúť okno { $direction ->
        *[down] dole
        [left] vľavo
        [right] vpravo
        [up] hore
    }
    .display = Presunúť okno jednu obrazovku { $direction ->
        *[down] dole
        [left] vľavo
        [right] vpravo
        [up] hore
    }
    .workspace = Presunúť okno jednu pracovnú plochu { $direction ->
        *[below] dole
        [left] vľavo
        [right] vpravo
        [above] hore
    }
    .workspace-num = Presunúť okno na pracovnú plochu { $num }
    .prev-workspace = Presunúť okno na predchádzajúcu pracovnú plochu
    .next-workspace = Presunúť okno na ďalšiu pracovnú plochu
    .last-workspace = Presunúť okno na poslednú pracovnú plochu
    .next-display = Presunúť okno na ďalšiu obrazovku
    .prev-display = Presunúť okno na predchádzajúcu obrazovku
    .send-to-prev-workspace = Presunúť okno na predchádzajúcu pracovnú plochu
    .send-to-next-workspace = Presunúť okno na ďalšiu pracovnú plochu

system-shortcut = Systém
    .app-library = Otvoriť knižnicu aplikácií
    .brightness-down = Znížiť jas obrazovky
    .brightness-up = Zvýšiť jas obrazovky
    .home-folder = Otvoriť domovský priečinok
    .keyboard-brightness-down = Znížiť jas klávesnice
    .keyboard-brightness-up = Zvýšiť jas klávesnice
    .launcher = Otvoriť spúšťač
    .lock-screen = Zamknúť obrazovku
    .mute = Ztlmiť zvukový výstup
    .mute-mic = Ztlmiť vstup mikrofónu
    .screenshot = Urobiť snímku obrazovky
    .terminal = Otvoriť terminál
    .volume-lower = Znížiť hlasitosť zvukového výstupu
    .volume-raise = Zvýšiť hlasitosť zvukového výstupu
    .web-browser = Otvoriť webový prehliadač
    .window-switcher = Prepnúť medzi oknami
    .workspace-overview = Otvoriť prehľad pracovných plôch

window-tiling = Dláždenie okien
    .horizontal = Nastaviť horizontálnu orientáciu
    .vertical = Nastaviť vertikálnu orientáciu
    .swap-window = Vymeniť okná
    .toggle-tiling = Zapnúť dláždenie okien
    .toggle-stacking = Zapnúť ukladanie okien
    .toggle-floating = Zapnúť plávanie okien
    .toggle-orientation = Prepnúť orientáciu

replace-shortcut-dialog = Nahradiť klávesovú skratku?
    .desc = { $shortcut } je použitá v skratke { $name }. Ak ju nahradíte, { $name } bude vypnutá.

## Input: Mouse

mouse = Myš
    .desc = Citlivosť myši, akcelerácia, prirodzené rolovanie.
    .speed = Citlivosť myši
    .acceleration = Zapnúť akceleráciu myši

## Input: Touchpad

click-behavior = Správanie klikania
    .click-finger = Vedľajšie kliknutie dvoma prstami a kliknutie stredným tlačidlom troma prstami
    .button-areas = Vedľajšie kliknutie v pravom dolnom rohu a kliknutie stredným tlačidlom v strede dole

pinch-to-zoom = Priblíženie stiahnutím prstov
    .desc = Pri aplikáciách, ktoré podporujú priblíženie, použite dva prsty na priblíženie obsahu.

tap-to-click = Klepnutím kliknite
    .desc = Umožňuje klepnutie jedným prstom pre primárne kliknutie, klepnutie dvoma prstami pre sekundárne kliknutie a klepnutie tromi prstami pre kliknutie stredným tlačidlom.

touchpad = Touchpad
    .desc = Citlivosť touchpadu, nastavenie klikania, gestá.
    .speed = Citlivosť touchpadu
    .acceleration = Zapnúť akceleráciu touchpadu

## Input: Gestures

gestures = Gestá
    .four-finger-down = Potiahnite štyrmi prstami nadol
    .four-finger-left = Potiahnite štyrmi prstami vľavo
    .four-finger-right = Potiahnite štyrmi prstami vpravo
    .four-finger-up = Potiahnite štyrmi prstami hore
    .three-finger-any = Potiahnite troma prstami ľubovoľným smerom

switch-between-windows = Prepínanie medzi oknami
switch-to-next-workspace = Prepnúť na ďalší pracovný priestor
switch-to-prev-workspace = Prepnúť na predchádzajúci pracovný priestor
open-application-library = Otvoriť knižnicu aplikácií
open-workspaces-view = Otvoriť prehľad pracovných priestorov

## Power 

power = Napájanie
    .desc = Spravujte nastavenia napájania

power-mode = Režim napájania
    .performance = Vysoký výkon
    .balanced = Vyvážený
    .battery = Šetrenie energie
    .performance-desc = Maximálny výkon a vysoká spotreba energie.
    .balanced-desc = Tichá prevádzka a mierna spotreba energie.
    .battery-desc = Znížená spotreba energie a tichá prevádzka.
    .nobackend = Backend sa nenašiel. Nainštalujte démona system76-power alebo power-profiles-daemon.
