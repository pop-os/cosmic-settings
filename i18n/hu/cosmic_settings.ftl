app = COSMIC Beállítások
dbus-connection-error = Nem sikerült csatlakozni a DBus-hoz
ok = OK
unknown = Ismeretlen
number = { $number }

## Network & Wireless

connections-and-profiles =
    { $variant ->
        [wired] Vezetékes
        [wifi] Wi-Fi
        [vpn] VPN
       *[other] Ismeretlen
    } kapcsolatok és profilok
add-network = Hálózat hozzáadása
    .profile = Profil hozzáadása
add-vpn = VPN hozzáadása
airplane-on = A repülőgép üzemmód be van kapcsolva
cable-unplugged = Kábel kihúzva
connect = Csatlakozás
connected = Csatlakozva
connecting = Csatlakozás…
disconnect = Leválasztás
forget = Elfelejtés
known-networks = Ismert hálózatok
network-and-wireless = Hálózat és Wi-Fi
no-networks = Nem található hálózat
no-vpn = Nincs elérhető VPN-kapcsolat
password = Jelszó
password-confirm = Jelszó megerősítése
remove = Törlés
settings = Beállítások
username = Felhasználónév
visible-networks = Látható hálózatok
identity = Azonosító
auth-dialog = Azonosítás szükséges
    .vpn-description = Írd be a VPN szolgáltatás által követelt felhasználónevet és jelszót.
    .wifi-description = Írd be a jelszót vagy a titkosítókulcsot. A router „WPS” gombjának megnyomásával is csatlakozhatsz.
forget-dialog = El akarod felejteni ezt a Wi-Fi hálózatot?
    .description = A jelszót ismét meg kell adnod, ha újra csatlakozni szeretnél.
network-device-state =
    .activated = Csatlakozva
    .config = Csatlakozás…
    .deactivating = Leválasztás…
    .disconnected = Leválasztva
    .failed = Csatlakozás sikertelen
    .ip-check = Kapcsolat ellenőrzése
    .ip-config = IP és router információk lekérése
    .need-auth = Azonosítás szükséges
    .prepare = Felkészülés a kapcsolódáshoz
    .secondaries = Várakozás egy másodlagos kapcsolatra
    .unavailable = Nem elérhető
    .unknown = Ismeretlen állapot
    .unmanaged = Kezeletlen
    .unplugged = A kábel ki van húzva
remove-connection-dialog = Törlöd a kapcsolati profilt?
    .vpn-description = A jelszót újra be kell írnod a jövőbeli használathoz.
    .wired-description = A profilt újból létre kell hoznod a jövőbeli használathoz.
vpn = VPN
    .connections = VPN-kapcsolatok
    .error = Nem sikerült hozzáadni a VPN-konfigurációt
    .remove = Kapcsolati profil törlése
    .select-file = VPN-konfigurációs fájl kiválasztása
vpn-error = VPN hiba
    .config = Nem sikerült hozzáadni a VPN-konfigurációt
    .connect = Nem sikerült csatlakozni a VPN-hez
    .connection-editor = A hálózati beállító nem működik
    .connection-settings = Nem sikerült lekérni az aktív kapcsolatok beállításait
    .updating-state = Nem sikerült frissíteni a hálózatkezelő állapotát
    .wireguard-config-path = Érvénytelen fájlelérési út a WireGuard konfigurációhoz
    .wireguard-config-path-desc = A kiválasztott fájlnak helyi fájlrendszeren kell lennie.
    .wireguard-device = Nem sikerült létrehozni a WireGuard eszközt
    .with-password =
        Nem sikerült beállítani a VPN { $field ->
           *[username] felhasználónevét
            [password] jelszavát
            [password-flags] jelszóbeállításait
        } az nmclivel
wired = Vezetékes
    .adapter = Vezetékes adapter { $id }
    .connections = Vezetékes kapcsolatok
    .devices = Vezetékes eszközök
    .remove = Kapcsolati profil törlése
wifi = Wi-Fi
    .adapter = Wi-Fi adapter { $id }
    .forget = Hálózat elfelejtése
wireguard-dialog = WireGuard eszköz hozzáadása
    .description = Válassz egy eszköznevet a WireGuard-konfigurációhoz.

## Networking: Online Accounts

online-accounts = Online fiókok
    .desc = Fiókok hozzáadása, IMAP és SMTP, vállalati bejelentkezések

# Bluetooth

activate = Aktiválás
confirm = Megerősítés
enable = Engedélyezés
bluetooth = Bluetooth
    .desc = Bluetooth-eszközök kezelése
    .status = Ez a rendszer { $aliases } néven látható, amíg a Bluetooth beállítások vannak megnyitva.
    .connected = Csatlakozva
    .connecting = Csatlakozás…
    .disconnecting = Leválasztás…
    .connect = Csatlakozás
    .disconnect = Leválasztás
    .forget = Elfelejtés
    .dbus-error = Hiba történt a DBus-szal való kommunikáció során: { $why }
    .disabled = A Bluetooth szolgáltatás ki van kapcsolva
    .inactive = A Bluetooth szolgáltatás nem aktív
    .unknown = A Bluetooth szolgáltatást nem sikerült aktiválni. A BlueZ telepítve van?
bluetooth-paired = Korábban párosított eszközök
    .connect = Csatlakozás
    .battery = { $percentage }% töltöttség
bluetooth-confirm-pin = Bluetooth PIN megerősítése
    .description = Erősítsd meg, hogy a következő PIN megegyezik a(z) { $device } eszközön megjelenített PIN-kóddal
bluetooth-available = Közeli eszközök
bluetooth-adapters = Bluetooth adapterek

## Accessibility

accessibility = Akadálymentesség
    .vision = Látás
    .on = Bekapcsolva
    .off = Kikapcsolva
    .unavailable = Nem elérhető
    .screen-reader = Képernyőolvasó
    .high-contrast = Magas kontraszt mód
    .invert-colors = Színek invertálása
    .color-filters = Színszűrők
hearing = Hallás
    .mono = Sztereó hang lejátszása monóként
default = Alapértelmezett
magnifier = Nagyító
    .controls =
        Vagy használd ezeket a gyorsbillentyűket: { $zoom_in ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_in } a nagyításhoz,
        }{ $zoom_out ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_out } a kicsinyítéshez,
        }
        Super + egérgörgő
    .scroll_controls = Egérrel vagy érintőpárnával történő nagyítás Super + görgetéssel
    .show_overlay = Nagyító megjelenítése
    .increment = Nagyítás mértéke
    .signin = Nagyító indítása bejelentkezéskor
    .applet = A nagyítót be- és kikapcsolhatod a panelen található kisalkalmazásban
    .movement = Nagyított nézet mozgása
    .continuous = A mutatóval
    .onedge = Amikor a mutató eléri a szélét
    .centered = A mutató középen tartásához
color-filter = Színszűrő típusa
    .unknown = Ismeretlen szűrő aktív
    .greyscale = Szürkeárnyalatos
    .deuteranopia = Zöld/Vörös (zöld színvakság, Deuteranópia)
    .protanopia = Vörös/Zöld (vörös színvakság, Protanópia)
    .tritanopia = Kék/Sárga (kék színvakság, Tritanópia)

## Desktop

desktop = Asztal

## Desktop: Wallpaper

wallpaper = Háttérkép
    .change = Háttérkép váltásának gyakorisága
    .desc = Háttérképek, színek és diavetítési beállítások
    .fit = Háttérkép illeszkedése
    .folder-dialog = Válassz háttérkép mappát
    .image-dialog = Válassz háttérkép képet
    .plural = Háttérképek
    .same = Ugyanaz a háttérkép legyen minden kijelzőn
    .slide = Diavetítés
add-color = Szín hozzáadása
add-image = Kép hozzáadása
all-displays = Minden kijelző
colors = Színek
dialog-add = Hozzáadás
fill = Kitöltés
fit-to-screen = Képernyőhöz igazítás
open-new-folder = Új mappa megnyitása
recent-folders = Legutóbbi mappák
x-minutes =
    { $number } { $number ->
        [one] perc
       *[other] perc
    }
x-hours =
    { $number } { $number ->
        [one] óra
       *[other] óra
    }
never = Soha

## Desktop: Appearance

appearance = Megjelenés
    .desc = Kiemelőszínek és COSMIC témák
accent-color = Kiemelőszín
app-background = Ablakháttér
auto = Automatikus
close = Bezárás
color-picker = Színválasztó
copied-to-clipboard = Vágólapra másolva
copy-to-clipboard = Másolás a vágólapra
dark = Sötét
export = Exportálás
hex = Hex
import = Importálás
light = Világos
mode-and-colors = Mód és színek
recent-colors = Korábbi színek
reset-to-default = Visszaállítás alapértelmezettre
rgb = RGB
window-hint-accent = Az Aktív ablak kiemelésének színe
window-hint-accent-toggle = A témaszín használata az aktív ablak kiemelésére
auto-switch = Automatikus váltás világos és sötét mód között
    .sunrise = Napkeltekor világos módra vált
    .sunset = Napnyugtakor sötét módra vált
    .next-sunrise = Következő napfelkeltekor világos módra vált
    .next-sunset = Következő napnyugtakor sötét módra vált
container-background = Felületi elemek háttere
    .desc-detail = A felületi elemek háttérszínét a navigációs oldalsáv, az oldalsó menü, a párbeszédablakok és hasonló widgetek használják. Alapértelmezés szerint a felületi elemek háttérszíne automatikusan az ablak hátteréből származik.
    .reset = Visszaállítás automatikusra
    .desc = Navigációs oldalsávhoz, oldalsó menühöz, párbeszédablakhoz és hasonló widgetekhez használatos
control-tint = Vezérlőelemek színezése
    .desc = Szabványos gombok hátterére, keresési bemenetekre, szövegbevitelre és hasonló összetevőkre használatos
frosted = Elmosódott üveg hatás a rendszerfelületen
    .desc = A háttér elmosását alkalmazza a panelre, a dokkra, a kisalkalmazásokra, az indítóra és az alkalmazáskönyvtárra
enable-export = Jelenlegi téma alkalmazása a GNOME-alkalmazásokra
    .desc = Nem minden eszközkészlet támogatja az automatikus váltást. Előfordulhat, hogy a téma módosítása után újra kell indítani a nem COSMIC-alapú alkalmazásokat.
icon-theme = Ikontéma
    .desc = Más ikonkészletet alkalmaz az alkalmazásokra
text-tint = Felületi szöveg árnyalata
    .desc = A felület szövegszíneinek meghatározására szolgál, hogy azok megfelelő kontrasztot biztosítsanak különböző felületeken
style = Stílus
    .round = Lekerekített
    .slightly-round = Némileg lekerekített
    .square = Négyzetes
interface-density = Felületsűrűség
    .comfortable = Kényelmes
    .compact = Kompakt
    .spacious = Tágas
window-management-appearance = Ablakkezelés
    .active-hint = Aktív ablak kiemelésének mérete
    .gaps = Rések a csempézett ablakok körül

### Experimental

experimental-settings = Kísérleti beállítások
icons-and-toolkit = Ikonok és eszközkészlet témázása
interface-font = Rendszer betűtípusa
monospace-font = Rögzített szélességű betűtípus

## Desktop: Notifications

notifications = Értesítések
    .desc = Ne zavarjanak, zárolási képernyő értesítések és alkalmazásonkénti beállítások

## Desktop: Panel

panel = Panel
    .desc = Fő rendszersáv menükhöz és kisalkalmazásokhoz
add = Hozzáadás
add-applet = Kisalkalmazás hozzáadása
all = Összes
applets = Kisalkalmazások
center-segment = Szegmens közepe
end-segment = Szegmens vége
large = Nagy
no-applets-found = Nem található kisalkalmazás…
panel-bottom = Alul
panel-left = Balra
panel-right = Jobbra
panel-top = Felül
search-applets = Kisalkalmazások keresése…
small = Kicsi
start-segment = Szegmens eleje
panel-appearance = Megjelenés
    .match = Rendszertéma
    .light = Világos
    .dark = Sötét
panel-behavior-and-position = Viselkedés és pozíció
    .autohide = A panel automatikus elrejtése
    .dock-autohide = A dokk automatikus elrejtése
    .position = Elhelyezkedés a képernyőn
    .display = Megjelenítés a kijelzőn
panel-style = Stílus
    .anchor-gap = Hézag a panel és a képernyő szélei között
    .dock-anchor-gap = Hézag a dokk és a képernyő szélei között
    .extend = Panel kinyújtása a képernyő széléig
    .dock-extend = Dokk kinyújtása a képernyő széléig
    .appearance = Megjelenés
    .size = Méret
    .background-opacity = Háttér átlátszósága
panel-applets = Konfiguráció
    .dock-desc = A dokk kisalkalmazásainak konfigurálása
    .desc = A panel kisalkalmazásainak konfigurálása
panel-missing = A panel konfigurációja hiányzik
    .desc = A panel konfigurációs fájlja hiányzik egyéni konfiguráció használata miatt vagy mert a fájl megsérült.
    .fix = Visszaállítás alapértelmezettre

## Desktop: Dock

dock = Dokk
    .desc = Egy opcionális sáv alkalmazásokhoz és kisalkalmazásokhoz

## Desktop: Window management

window-management = Ablakkezelés
    .desc = Super billentyű funkció, ablakkezelési és csempézési beállítások
super-key = Super billentyű
    .launcher = Indító megnyitása
    .workspaces = Munkaterületek megnyitása
    .applications = Alkalmazások megnyitása
    .disable = Kikapcsolás
edge-gravity = A lebegő ablakok a közeli szélekhez igazodnak
window-controls = Ablakvezérlés
    .maximize = Maximalizálás gomb mutatása
    .minimize = Minimalizálás gomb mutatása
    .active-window-hint = Aktív ablak kiemelése
focus-navigation = Fókusznavigáció
    .focus-follows-cursor = Fókusz követi az egeret
    .focus-follows-cursor-delay = Fókuszálás késleltetése (ms)
    .cursor-follows-focus = Mutató követi a fókuszt

## Desktop: Workspaces

workspaces = Munkaterületek
    .desc = Munkaterületek tájolása és viselkedése
workspaces-behavior = Munkaterületek viselkedése
    .dynamic = Dinamikus munkaterületek
    .dynamic-desc = Az üres munkamenetek automatikus eltávolítása
    .fixed = Megadott számú munkaterület
    .fixed-desc = Munkaterületek hozzáadása vagy eltávolítása az áttekintésben
workspaces-multi-behavior = Többmonitoros viselkedés
    .span = A munkaterületek kiterjednek a kijelzőkre
    .separate = A kijelzők külön munkaterülettel rendelkeznek
workspaces-overview-thumbnails = Munkaterület-áttekintő bélyegképek
    .show-number = Munkaterület számának megjelenítése
    .show-name = Munkaterület nevének megjelenítése
workspaces-orientation = Munkaterületek tájolása
    .vertical = Függőleges
    .horizontal = Vízszintes
hot-corner = Aktív sarok
    .top-left-corner = A munkaterületek bal felső aktív sarkának engedélyezése

## Displays

-requires-restart = Újraindítást igényel
color = Szín
    .depth = Színmélység
    .profile = Színprofil
    .sidebar = Színprofilok
    .temperature = Színhőmérséklet
display = Kijelzők
    .desc = Kijelzők kezelése és éjszakai fény
    .arrangement = Kijelző elrendezése
    .arrangement-desc = Húzd a kijelzőket az átrendezésükhöz
    .enable = Kijelző engedélyezése
    .external = { $size } { $output } külső kijelző
    .laptop = { $size } laptop kijelző
    .options = Kijelző lehetőségek
    .refresh-rate = Frissítési gyakoriság
    .resolution = Felbontás
    .scale = Skálázás
    .additional-scale-options = További skálázás
mirroring = Tükrözés
    .id = Tükrözés { $id }
    .dont = Ne tükrözzön
    .mirror = Tükrözze a(z) { $display } kijelzőt
    .project =
        Kivetítés { $display ->
            [all] az összes kijelzőre
           *[other] a(z) { $display } kijelzőre
        }
    .project-count =
        Kivetítés { $count } további { $count ->
            [1] kijelzőre
           *[other] kijelzőkre
        }
night-light = Éjszakai fény
    .auto = Automatikus (naplementétől napkeltéig)
    .desc = Csökkentse a kék fényt melegebb színekkel
orientation = Tájolás
    .standard = Normál
    .rotate-90 = 90 fokos elforgatás
    .rotate-180 = 180 fokos elforgatás
    .rotate-270 = 270 fokos elforgatás
vrr = Változó frissítési gyakoriság
    .enabled = Engedélyezve
    .force = Mindig
    .auto = Automatikus
    .disabled = Letiltva
scheduling = Ütemezés
    .manual = Kézi ütemezés
dialog = Párbeszédablak
    .title = Megtartod ezeket a kijelzőbeállításokat?
    .keep-changes = Változtatások megtartása
    .change-prompt = A beállítások automatikusan visszaállnak { $time } másodperc múlva.
    .revert-settings = Beállítások visszaállítása

## Sound

sound = Hang
    .desc = N/A
sound-output = Kimenet
    .volume = Kimeneti hangerő
    .device = Kimeneti eszköz
    .level = Kimeneti szint
    .config = Konfiguráció
    .balance = Balansz
    .left = Bal
    .right = Jobb
sound-input = Bemenet
    .volume = Bemeneti hangerő
    .device = Bemeneti eszköz
    .level = Bemeneti szint
sound-alerts = Figyelmeztetések
    .volume = Figyelmeztetések hangereje
    .sound = Figyelmeztető hang
sound-applications = Alkalmazások
    .desc = Alkalmazások hangerejei és beállításai

## Power

power = Energia és akkumulátor
    .desc = Energiabeállítások kezelése
battery = Akkumulátor
    .minute =
        { $value } { $value ->
            [one] perc
           *[other] perc
        }
    .hour =
        { $value } { $value ->
            [one] óra
           *[other] óra
        }
    .day =
        { $value } { $value ->
            [one] nap
           *[other] nap
        }
    .less-than-minute = Kevesebb, mint egy perc
    .and = és
    .remaining-time =
        { $time } { $action ->
            [full] a teljes töltöttségig
           *[other] a lemerülésig
        }
connected-devices = Csatlakoztatott eszközök
    .unknown = Ismeretlen eszköz
power-mode = Energiagazdálkodási mód
    .battery = Meghosszabbított akkumulátor-üzemidő
    .battery-desc = Csökkentett energiafogyasztás és csendes teljesítmény
    .balanced = Kiegyensúlyozott
    .balanced-desc = Csendes teljesítmény és mérsékelt energiafogyasztás
    .performance = Nagy teljesítmény
    .performance-desc = Csúcsteljesítmény és energiafelhasználás
    .no-backend = A háttérprogram nem található. Telepítsd a system76-power vagy a power-profiles-daemon csomagot.
power-saving = Energiagazdálkodási beállítások
    .turn-off-screen-after = Képernyő kikapcsolása
    .auto-suspend = Automatikus felfüggesztés
    .auto-suspend-ac = Automatikus felfüggesztés hálózati áramforráson
    .auto-suspend-battery = Automatikus felfüggesztés akkumulátoron

## Input

acceleration-desc = Automatikusan beállítja a követési érzékenységet a sebesség alapján
disable-while-typing = Letiltás gépelés közben
input-devices = Beviteli eszközök
    .desc = Beviteli eszközök
primary-button = Elsődleges gomb
    .desc = A gombok sorrendjének beállítása
    .left = Bal
    .right = Jobb
scrolling = Görgetés
    .two-finger = Görgetés két ujjal
    .edge = Egy ujjal görgetés az érintőpárna szélén
    .speed = Görgetési sebesség
    .natural = Természetes görgetés
    .natural-desc = A görgetés a tartalmat mozgatja, nem a nézetet

## Input: Keyboard

slow = Lassú
fast = Gyors
short = Rövid
long = Hosszú
keyboard = Billentyűzet
    .desc = Bemeneti források, speciális karakterek, gyorsbillentyűk
keyboard-sources = Bemeneti források
    .desc = A bemeneti források a Super+Space billentyűkombinációval válthatók. Ez testreszabható a gyorsbillentyűk beállításaiban.
    .move-up = Mozgatás feljebb
    .move-down = Mozgatás lejjebb
    .settings = Beállítások
    .view-layout = Billentyűkiosztás megtekintése
    .remove = Eltávolítás
    .add = Bemeneti forrás hozzáadása
keyboard-special-char = Speciális karakter beírása
    .alternate = Alternatív karakterek billentyűje
    .compose = Kombináló billentyű
    .compose-desc = A kombináló billentyű lehetővé teszi számos karakter bevitelét. Használatához nyomd meg a billentyűt, majd gépeld be a karakterek sorozatát. Például a kombináló billentyű, majd C és o lenyomásával a © jelet kapod, míg a és ‘ után á jelenik meg.
    .caps = Caps Lock billentyű
keyboard-typing-assist = Gépelés
    .repeat-rate = Ismétlési sebesség
    .repeat-delay = Ismétlési késleltetés
keyboard-numlock-boot = Num Lock
    .boot-state = Állapot rendszerindításkor
    .last-boot = Utolsó rendszerindítás
    .on = Bekapcsolva
    .off = Kikapcsolva
    .set = Num Lock rendszerindítási állapotának beállítása
added = Hozzáadva
type-to-search = Írj a kereséshez…
show-extended-input-sources = Bővített bemeneti források megjelenítése

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Gyorsbillentyűk
    .desc = Gyorsbillentyűk megtekintése és testreszabása
add-another-keybinding = Új gyorsbillentyű hozzáadása
cancel = Mégse
command = Parancs
custom = Egyedi
debug = Hibakeresés
disabled = Tiltva
input-source-switch = Billentyűzetkiosztás váltása
migrate-workspace-prev = Munkaterület áthelyezése az előző kimenetre
migrate-workspace-next = Munkaterület áthelyezése a következő kimenetre
migrate-workspace =
    Munkaterület áthelyezése { $direction ->
       *[down] az alsó
        [left] a bal
        [right] a jobb
        [up] a felső
    } kimenetre
navigate = Navigálás
replace = Csere
shortcut-name = Gyorsbillentyű neve
system-controls = Rendszervezérlés
terminate = Befejezés
toggle-stacking = Az ablakok egymásra helyezésének be- és kikapcsolása
type-key-combination = Billentyűkombináció
custom-shortcuts = Egyéni gyorsbillentyűk
    .add = Gyorsbillentyű hozzáadása
    .context = Egyéni gyorsbillentyű hozzáadása
    .none = Nincsenek egyéni gyorsbillentyűk
modified = { $count } módosítva
nav-shortcuts = Navigáció
    .prev-output = Az előző kimenet fókuszálása
    .next-output = A következő kimenet fókuszálása
    .last-workspace = Az utolsó munkaterület fókuszálása
    .prev-workspace = Az előző munkaterület fókuszálása
    .next-workspace = A következő munkaterület fókuszálása
    .focus =
        Ablak fókuszálása { $direction ->
           *[down] le
            [in] be
            [left] balra
            [out] ki
            [right] jobbra
            [up] fel
        }
    .output =
        Váltás { $direction ->
           *[down] lenti
            [left] bal
            [right] jobb
            [up] fenti
        } kimenetre
    .workspace = Váltás a(z) { $num }. munkaterületre
manage-windows = Ablakok kezelése
    .close = Ablak bezárása
    .maximize = Ablak maximalizálása
    .fullscreen = Teljes képernyő
    .minimize = Ablak minimalizálása
    .resize-inwards = Ablak átméretezése befelé
    .resize-outwards = Ablak átméretezése kifelé
    .toggle-sticky = Ragadós ablak be- és kikapcsolása
move-windows = Ablakok mozgatása
    .direction =
        Ablakok mozgatása { $direction ->
           *[down] lefelé
            [left] balra
            [right] jobbra
            [up] felfelé
        }
    .display =
        Ablak mozgatása egy monitorral { $direction ->
           *[down] lefelé
            [left] balra
            [right] jobbra
            [up] felfelé
        }
    .workspace =
        Ablak mozgatása egy munkaterülettel { $direction ->
           *[below] lefelé
            [left] balra
            [right] jobbra
            [above] felfelé
        }
    .workspace-num = Ablak áthelyezése a(z) { $num }. munkaterületre
    .prev-workspace = Ablak áthelyezése az előző munkaterületre
    .next-workspace = Ablak áthelyezése a következő munkaterületre
    .last-workspace = Ablak áthelyezése az utolsó munkaterületre
    .next-display = Ablak áthelyezése a következő monitorra
    .prev-display = Ablak áthelyezése az előző monitorra
    .send-to-prev-workspace = Ablak áthelyezése az előző munkaterületre
    .send-to-next-workspace = Ablak áthelyezése a következő munkaterületre
system-shortcut = Rendszer
    .app-library = Alkalmazáskönyvtár megnyitása
    .brightness-down = Kijelző fényerejének csökkentése
    .brightness-up = Kijelző fényerejének növelése
    .display-toggle = Belső kijelző be- vagy kikapcsolása
    .home-folder = Saját mappa megnyitása
    .keyboard-brightness-down = Billentyűzet fényerejének csökkentése
    .keyboard-brightness-up = Billentyűzet fényerejének növelése
    .launcher = Indító megnyitása
    .log-out = Kijelentkezés
    .lock-screen = Képernyő zárolása
    .mute = Hangkimenet némítása
    .mute-mic = Mikrofonbemenet némítása
    .play-pause = Lejátszás/Szünet
    .play-next = Következő szám
    .play-prev = Előző szám
    .poweroff = Leállítás
    .screenshot = Képernyőkép készítése
    .suspend = Felfüggesztés
    .terminal = Terminál megnyitása
    .touchpad-toggle = Érintőpárna be- vagy kikapcsolása
    .volume-lower = Hangkimenet hangerejének csökkentése
    .volume-raise = Hangkimenet hangerejének növelése
    .web-browser = Böngésző megnyitása
    .window-switcher = Váltás a nyitott ablakok között
    .window-switcher-previous = Váltás a nyitott ablakok között fordított sorrendben
    .workspace-overview = Munkaterület áttekintésének megnyitása
window-tiling = Ablakcsempézés
    .horizontal = Vízszintes tájolás beállítása
    .vertical = Függőleges tájolás beállítása
    .swap-window = Ablak felcserélése
    .toggle-tiling = Az ablakok csempézésének be- és kikapcsolása
    .toggle-stacking = Az ablakok egymásra helyezésének be- és kikapcsolása
    .toggle-floating = Az ablakok lebegtetésének be- és kikapcsolása
    .toggle-orientation = Tájolás váltása
replace-shortcut-dialog = Lecseréled a billentyűt?
    .desc = A(z) { $shortcut } billentyűt a(z) { $name } használja. Ha lecseréled, a(z) { $name } le lesz tiltva.
zoom-in = Nagyítás
zoom-out = Kicsinyítés

## Input: Mouse

mouse = Egér
    .desc = Egérsebesség, gyorsítás és természetes görgetés
    .speed = Egér sebessége
    .acceleration = Egérgyorsítás engedélyezése

## Input: Touchpad

click-behavior = Kattintási viselkedés
    .click-finger = Másodlagos kattintás két ujjal és középső kattintás három ujjal
    .button-areas = Másodlagos kattintás a jobb alsó sarokban, középső kattintás az alsó középső sarokban
pinch-to-zoom = Csippentéses nagyítás
    .desc = Két ujjal belenagyíthatsz a tartalomba azokban az alkalmazásokban, amik támogatják a nagyítást
tap-to-click = Koppints a kattintáshoz
    .desc = Engedélyezi az egyujjas koppintást az elsődleges kattintáshoz, a kétujjas koppintást a másodlagos kattintáshoz és a háromujjas érintést a középső kattintáshoz
touchpad = Érintőpárna
    .acceleration = Az Érintőpárna gyorsításának engedélyezése
    .desc = Érintőpárna sebessége, kattintási lehetőségek, gesztusok
    .speed = Érintőpárna sebessége

## Input: Gestures

gestures = Gesztusok
    .four-finger-down = Négy ujjas csúsztatás lefelé
    .four-finger-left = Négy ujjas csúsztatás balra
    .four-finger-right = Négy ujjas csúsztatás jobbra
    .four-finger-up = Négy ujjas csúsztatás felfelé
    .three-finger-any = Három ujjas csúsztatás bármelyik irányba
switch-workspaces = Munkaterület váltása
    .horizontal = Négy ujjas csúsztatás balra/jobbra
    .vertical = Négy ujjas csúsztatás felfelé/lefelé
switch-between-windows = Váltás az ablakok között
open-application-library = Alkalmazáskönyvtár megnyitása
open-workspaces-view = Munkaterületek áttekintésének megnyitása

## Time & Language

time = Idő és nyelv
    .desc = N/A
time-date = Dátum és idő
    .desc = Időzóna, automatikus órabeállítások és időformátum
    .auto = Beállítás automatikusan
    .auto-ntp = A dátum és idő automatikusan frissül, ha az időzóna be van állítva
time-zone = Időzóna
    .auto = Automatikus időzóna
    .auto-info = Helymeghatározási szolgáltatások és internetkapcsolat szükséges
time-format = Dátum- és időformátum
    .twenty-four = 24 órás időformátum
    .show-seconds = Másodpercek megjelenítése
    .first = A hét első napja
    .show-date = Dátum megjelenítése az idő kisalkalmazásban
    .friday = Péntek
    .saturday = Szombat
    .sunday = Vasárnap
    .monday = Hétfő
time-region = Régió és nyelv
    .desc = Dátumok, időpontok és számok formázása a régió alapján
formatting = Formátum
    .dates = Dátum
    .time = Idő
    .date-and-time = Dátum és idő
    .numbers = Számok
    .measurement = Mértékegységek
    .paper = Papír
preferred-languages = Preferált nyelvek
    .desc = A nyelvek sorrendje határozza meg, hogy melyik nyelvet használja a felhasználói felület. A módosítások a következő bejelentkezéskor lépnek életbe.
add-language = Nyelv hozzáadása
    .context = Nyelv hozzáadása
install-additional-languages = További nyelvek telepítése
region = Régió

## Applications

applications = Alkalmazások

## Applications: Default Applications

default-apps = Alapértelmezett alkalmazások
    .desc = Alapértelmezett böngésző, levelezőprogram, fájlkezelő és egyéb alkalmazások
    .web-browser = Böngésző
    .file-manager = Fájlkezelő
    .mail-client = Levelezőprogram
    .music = Zene
    .video = Videó
    .photos = Fényképek
    .calendar = Naptár
    .terminal = Terminál
    .other-associations = Egyéb társítások
    .text-editor = Szövegszerkesztő

## Applications: Startup Applications

startup-apps = Indítási alkalmazások
    .desc = Azoknak az alkalmazásoknak a beállítása, amelyek bejelentkezéskor elindulnak
    .add = Alkalmazás hozzáadása
    .user = Bejelentkezéskor indított alkalmazások
    .none = Nincs indítási alkalmazás hozzáadva
    .remove-dialog-title = { $name } eltávolítása?
    .remove-dialog-description = Eltávolítod ezt az indítási alkalmazást?
    .add-startup-app = Indítási alkalmazás hozzáadása

## Applications: Legacy Applications

legacy-applications = X11 alkalmazások támogatása
    .desc = X11 ablakkezelő rendszer alkalmazásméretezés és globális gyorsbillentyűk
legacy-app-global-shortcuts = Globális gyorsbillentyűk X11 alkalmazásokban
    .desc = A globális gyorsbillentyűk lehetővé teszik, hogy az alkalmazásokban a billentyűleütéseket és egérkattintásokat más alkalmazások is felismerjék, például a „push-to-talk” vagy a „push-to-mute” funkciókhoz. Alapértelmezés szerint a globális gyorsbillentyűk az X11 alkalmazásokban le van tiltva, hogy más alkalmazások ne figyelhessék a billentyű- és egéreseményeket, amelyek érzékeny információkat tartalmazhatnak.
    .none = Nincs billentyű
    .modifiers = Módosítók (Super, Shift, Control, Alt)
    .combination = Minden billentyű, miközben a Super, Control vagy Alt módosítók lenyomva vannak
    .all = Minden billentyű
    .mouse = Egérgomb-események X11 alkalmazásokban
legacy-app-scaling = X11-ablakrendszer alkalmazásméretezés
    .scaled-gaming = Optimalizálás játékokra és teljes képernyős alkalmazásokra
    .gaming-description = Az X11 alkalmazások kicsit nagyobbnak/kisebbnek tűnhetnek a Wayland alkalmazásokhoz képest
    .scaled-applications = Optimalizálás alkalmazásokra
    .applications-description = A játékok és a teljes képernyős X11 alkalmazások felbontása eltérhet a kijelző natív felbontásától
    .scaled-compatibility = Maximális kompatibilitási mód
    .compatibility-description = Az X11 alkalmazások elmosódottan jelenhetnek meg HiDPI képernyőkön
    .preferred-display = Preferált kijelző játékokhoz és teljes képernyős X11-alkalmazásokhoz
    .no-display = Nincs

## System

system = Rendszer és fiókok

## System: About

about = Rendszerinformáció
    .desc = Eszköznév, hardverinformációk és az operációs rendszer alapértelmezett beállításai
about-device = Eszköz neve
    .desc = Ez a név más hálózati vagy Bluetooth-eszközök számára látható
about-hardware = Hardver
    .model = Hardver modell
    .memory = Memória
    .processor = Processzor
    .graphics = Grafika
    .disk-capacity = Tárhely
about-os = Operációs rendszer
    .os = Operációs rendszer
    .os-architecture = Operációs rendszer architektúra
    .kernel = Kernel verzió
    .desktop-environment = Asztali környezet
    .windowing-system = Ablakrendszer
about-related = Kapcsolódó beállítások
    .support = Támogatás kérése

## System: Firmware

firmware = Firmware
    .desc = Firmware részletei

## System: Users

users = Felhasználók
    .desc = Hitelesítés és felhasználói fiókok
    .admin = Rendszergazda
    .standard = Normál
    .profile-add = Profilkép kiválasztása
administrator = Rendszergazda
    .desc = A rendszergazdák megváltoztathatják az összes felhasználó beállításait, új felhasználókat adhatnak hozzá és távolíthatnak el
add-user = Felhasználó hozzáadása
change-password = Jelszó megváltoztatása
remove-user = Felhasználó eltávolítása
full-name = Teljes név
invalid-username = Érvénytelen felhasználónév
password-mismatch = A jelszónak és a megerősítésének meg kell egyeznie
save = Mentés
amplification = Erősítés
    .desc = Lehetővé teszi a hangerő 150%-ra emelését
qr-code-unavailable = QR-kód nem elérhető
network-name = Hálózat neve
share = Hálózat megosztása
scan-to-connect-description = Olvasd be a QR-kódot a hálózathoz való csatlakozáshoz.
place-here = Ide helyezd a kisalkalmazásokat
sound-device-port-unplugged = Nincs csatlakoztatva
sound-hd-audio = HD-hang
sound-usb-audio = USB-hang
sound-device-profiles = Eszközprofilok
