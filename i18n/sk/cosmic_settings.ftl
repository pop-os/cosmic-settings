app = Nastavenia COSMIC

unknown = Neznáme

number = { $number }

## Network & Wireless

connections-and-profiles = { $variant ->
    [wired] Drôtové
    [wifi] Wi-Fi
    [vpn] VPN
    *[other] Neznáme
} pripojenia a profily.

add-network = Pridať sieť
    .profile = Pridať profil
add-vpn = Pridať VPN
airplane-on = Letový režim je aktívny.
cable-unplugged = Kábel je odpojený
connect = Pripojiť
connected = Pripojené
connecting = Pripája sa…
disconnect = Odpojiť
forget = Zabudnút
known-networks = Známe siete
network-and-wireless = Sieť a bezdrôtové pripojenie
no-networks = Žiadne siete neboli nájdené.
no-vpn = Žiadne dostupné pripojenia VPN.
password = Heslo
remove = Odstrániť
settings = Nastavenia
username = Prihlasovacie meno
visible-networks = Viditeľné siete

auth-dialog = Vyžaduje sa autorizácia
    .vpn-description = Zadajte používateľské meno a heslo požadované službou VPN.
    .wifi-description = Zadajte heslo alebo šifrovací kľúč. Pripojiť sa môžete aj stlačením tlačidla „WPS“ na smerovači.

forget-dialog = Zabudnúť túto Wi-Fi sieť?
    .description = Ak chcete túto sieť Wi-Fi v budúcnosti používať, budete musieť znova zadať heslo.

network-device-state =
    .activated = Pripojené
    .config = Pripája sa
    .deactivating = Odpája sa
    .disconnected = Odpojené
    .failed = Nepodarilo sa pripojiť
    .ip-check = Kontrolujem pripojenie
    .ip-config = Vyžiadanie IP a informácií o smerovaní
    .need-auth = Autorizácia je potrebná
    .prepare = Pripravuje sa pripojenie
    .secondaries = Čakám na sekundárne pripojenie
    .unavailable = Nedostupné
    .unknown = Neznámy stav
    .unmanaged = Nespravované
    .unplugged = Kábel odpojený

remove-connection-dialog = Odstrániť profil pripojenia?
    .vpn-description = Bude potrebné opätovne zadať heslo ak sa budete chcieť pripojiť.
    .wired-description = Bude potrebné znovu vytvoriť tento profil ak ho budete chcieť použiť v budúcnosti.

vpn = VPN
    .connections = VPN pripojenia
    .remove = Odstrániť profil pripojenia
    .select-file = Vybrať VPN konfiguračný súbor

wired = Drôtové
    .adapter = Adaptér drôtového pripojenia { $id }
    .connections = Drôtové pripojenia
    .devices = Drôtové zariadenia
    .remove = Odstrániť profil pripojenia

wifi = Wi-Fi
    .adapter = Wi-Fi adaptér { $id }
    .forget = Zabudnúť túto sieť

## Networking: Online Accounts

online-accounts = Online účty
    .desc = Pridať účty, IMAP a SMTP, podnikové prihlásenie

# Bluetooth

confirm = Potvrdiť

bluetooth = Bluetooth
    .desc = Spravovať zariadenia Bluetooth
    .status = Tento systém je viditeľný ako { $aliases } pokiaľ sú nastavenia Bluetooth otvorené.
    .connected = Pripojené
    .connecting = Pripája sa
    .disconnecting = Odpája sa
    .connect = Pripojiť
    .disconnect = Odpojiť
    .forget = Zabudnúť
    .dbus-error = Vyskytla sa chyba s DBus: { $why }
    .show-device-without-name = Zobraziť zariadenia bez názvu

bluetooth-paired = Spárované zariadenia
    .connect = Pripojiť
    .battery = { $percentage }% batérie

bluetooth-confirm-pin = Potvrďte Bluetooth PIN
    .description = Prosím potvrďte, že nasledujúci PIN je zhodný s tým, ktorý je zobrazený na { $device }

bluetooth-available = Zariadenia v blízkosti

bluetooth-adapters = Adaptéry Bluetooth

## Desktop

desktop = Plocha

## Desktop: Wallpaper

wallpaper = Pozadie
    .change = Zmeniť pozadie každých
    .desc = Obrázok pozadia, farby a nastavenia prezentácie.
    .fit = Prispôsobenie pozadia
    .folder-dialog = Vybrať priečinok s pozadiami
    .image-dialog = Vybrať obrázok pozadia
    .plural = Pozadia
    .same = Rovnaké pozadnie na všetkých obrazovkách
    .slide = Prezentácia

add-color = Pridať farbu
add-image = Pridať obrázok
all-displays = Všetky obrazovky
colors = Farby
dialog-add = Pridať
fill = Vyplniť
fit-to-screen = Prispôsobiť
open-new-folder = Otvoriť nový priečinok
recent-folders = Nedávne priečinky

x-minutes = { $number ->
    [1] { $number } minúta
    [few] { $number } minúty
    *[other] { $number } minút
}
x-hours = { $number ->
    [1] { $number } hodina
    [few] { $number } hodiny
    *[other] { $number } hodín
}

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

interface-density = Hustota rozhrania
    .comfortable = Pohodlné
    .compact = Kompaktné
    .spacious = Voľné

window-management-appearance = Správa okien
    .active-hint = Veľkosť orámovania aktívneho okna
    .gaps = Medzery okolo dlaždicových okien

### Experimental

experimental-settings = Experimentálne nastavenia
icons-and-toolkit = Ikony a téma toolkitu
interface-font = Systémový typ písma
monospace-font = Typ písma s pevnou šírkou

## Desktop: Notifications

notifications = Oznámenia
    .desc = Nerušiť, oznámenia na zamykacej obrazovke a nastavenia pre špecifické aplikácie.

## Desktop: Panel

panel = Panel
    .desc = Horný panel pre ovládanie plochy a menu.

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


## Desktop: Dock

dock = Dok
    .desc = Panel s pripnutými aplikáciami a inými apletmi.

## Desktop: Window management

window-management = Správa okien
    .desc = Správanie tlačidla Super, možnosti ovládania okien a iné nastavenia dláždenia okien.

super-key = Správanie tlačidla Super
    .launcher = Otvoriť Spúšťač
    .workspaces = Otvoriť Pracovné plochy
    .applications = Otvoriť Aplikácie
    .disable = Vypnúť

window-controls = Ovládače okien
    .maximize = Zobraziť tlačidlo maximalizovania
    .minimize = Zobraziť tlačidlo minimalizovania
    .active-window-hint = Zobraziť orámovanie aktívneho okna

focus-navigation = Zameranie okna
    .focus-follows-cursor = Zameranie sleduje kurzor
    .focus-follows-cursor-delay = Zameranie sleduje kurzor s opozdením
    .cursor-follows-focus = Kurzor sleduje zameranie

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
hot-corner = Rohy
    .top-left-corner = Povoliť použitie ľavého horného rohu pre otvorenie pracovných plôch

## Displays

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

legacy-applications = Škálovanie aplikácií typu X11
    .scaled-by-system = Škálovať všetky aplikácie typu X11
    .system-description = Aplikácie typu X11 budú rozmazané na HiDPI obrazovkách.
    .scaled-natively = Vykresliť aplikácie typu X11 v ich natívnom rozlíšení
    .native-description = Aplikácie typu X11, ktoré nepodporujú škálovanie budú malé v prípade, že používate HiDPI obrazovku. Aktivujte pre hry pre ich vykreslenie v plnom rozlíšení.

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
    .volume = Hlasitosť vstupu
    .device = Vstupné zariadenie
    .level = Úroveň vstupu

sound-alerts = Upozornenia
    .volume = Hlasitosť upozornení
    .sound = Zvuk upozornení

sound-applications = Aplikácie
    .desc = Nastavenia zvuku aplikácií

profile = Profil

## Power

power = Napájanie
    .desc = Spravujte nastavenia napájania

battery = Batéria
  .minute = { $value } { $value ->
        [one] minúta
        [few] minúty
       *[other] minút
  }
  .hour = { $value } { $value ->
        [one] hodina
        [few] hodiny
       *[other] hodín
  }
  .day = { $value } { $value ->
        [one] day
        [few] dni
       *[other] dní
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
    .performance = Vysoký výkon
    .balanced = Vyvážený
    .battery = Šetrenie energie
    .performance-desc = Maximálny výkon a vysoká spotreba energie.
    .balanced-desc = Tichá prevádzka a mierna spotreba energie.
    .battery-desc = Znížená spotreba energie a tichá prevádzka.
    .no-backend = Backend sa nenašiel. Nainštalujte démona system76-power alebo power-profiles-daemon.

## Input

acceleration-desc = Automaticky upraviť citlivosť sledovania na základe rýchlosti.

disable-while-typing = Zakázať počas písania

input-devices = Vstupné zariadenia
    .desc = Vstupné zariadenia

primary-button = Hlavné tlačidlo
    .desc = Nastaví poradie fyzických tlačidiel.
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
    .caps = Tlačidlo Caps Lock

keyboard-typing-assist = Písanie
    .repeat-rate = Rýchlosť opakovania
    .repeat-delay = Interval opozdenia

added = Pridané
type-to-search = Píšte pre vyhľadávanie...
show-extended-input-sources = Zobraziť rozšírené typy vstupu

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
    .play-pause = Prehrať/Pozastaviť
    .play-next = Nasledujúca skladba
    .play-prev = Predchádzajúca skladba
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

click-behavior = Správanie kliknutia
    .click-finger = Vedľajšie kliknutie dvoma prstami a kliknutie stredným tlačidlom troma prstami
    .button-areas = Vedľajšie kliknutie v pravom dolnom rohu a kliknutie stredným tlačidlom v strede dole

pinch-to-zoom = Priblíženie stiahnutím prstov
    .desc = Pri aplikáciách, ktoré podporujú priblíženie, použite dva prsty na priblíženie obsahu.

tap-to-click = Klepnutím kliknite
    .desc = Umožňuje klepnutie jedným prstom pre primárne kliknutie, klepnutie dvoma prstami pre sekundárne kliknutie a klepnutie tromi prstami pre kliknutie stredným tlačidlom.

touchpad = Touchpad
    .acceleration = Zapnúť akceleráciu touchpadu
    .desc = Citlivosť touchpadu, nastavenie klikania, gestá.
    .speed = Citlivosť touchpadu

## Input: Gestures

gestures = Gestá
    .four-finger-down = Potiahnite štyrmi prstami nadol
    .four-finger-left = Potiahnite štyrmi prstami vľavo
    .four-finger-right = Potiahnite štyrmi prstami vpravo
    .four-finger-up = Potiahnite štyrmi prstami hore
    .three-finger-any = Potiahnite troma prstami ľubovoľným smerom

switch-workspaces = Prepnúť pracovnú plochu
    .horizontal = Potiahnutie štyrmi prstami vľavo/vpravo
    .vertical = Potiahnutie štyrmi prstami hore/dole

switch-between-windows = Prepínanie medzi oknami
open-application-library = Otvoriť knižnicu aplikácií
open-workspaces-view = Otvoriť prehľad pracovných priestorov

## Time & Language

time = Čas a jazyk
    .desc = N/A

time-date = Dátum a čas
    .desc = Časová zóna, automatické nastavenie času, formátovanie času.
    .auto = Nastaviť automaticky
    .auto-ntp = Dátum a čas sa aktualizujú automaticky, keď je nastavené časové pásmo.

time-zone = Časová zóna
    .auto = Automatická časová zóna
    .auto-info = Vyžaduje službu polohy a internetu
time-format = Formát dátumu a času
    .twenty-four = 24 hodinový čas
    .show-seconds = Zobraziť sekundy
    .first = Prvý deň v týždni
    .show-date = Zobraziť dátum v paneli
    .friday = Piatok
    .saturday = Sobota
    .sunday = Nedeľa
    .monday = Pondelok

time-region = Oblasť a jazyk
    .desc = Formát dátumu, času, a čísel podľa oblasti

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
