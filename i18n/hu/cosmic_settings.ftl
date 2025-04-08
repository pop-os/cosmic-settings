app = COSMIC Beállítások

dbus-connection-error = Nem sikerült csatlakozni a DBus-hoz
ok = OK
unknown = Ismeretlen

number = { $number }

## Network & Wireless

connections-and-profiles = { $variant ->
    [wired] Vezetékes
    [wifi] Wi-Fi
    [vpn] VPN
    *[other] Ismeretlen
} kapcsolatok és profilok.

add-network = Hálózat hozzáadása
    .profile = Profil hozzáadása
add-vpn = VPN hozzáadása
airplane-on = Repülőgép üzemmód be van kapcsolva.
cable-unplugged = Kábel kihúzva
connect = Kapcsolódás
connected = Kapcsolódva
connecting = Kapcsolódás...
disconnect = Szétkapcsolás
forget = Elfelejtés
known-networks = Ismert hálózatok
network-and-wireless = Hálózat és Wi-Fi
no-networks = Nem található hálózat.
no-vpn = Nincsenek elérhető VPN-kapcsolatok.
password = Jelszó
remove = Törlés
settings = Beállítások
username = Felhasználónév
visible-networks = Látható hálózatok
identity = Azonosító

auth-dialog = Azonosítás szükséges
    .vpn-description = Írja be a VPN szolgáltatás által követelt felhasználónevet és jelszót.
    .wifi-description = Írja be a jelszót vagy a titkosítókulcsot. A router "WPS" gombjának megnyomásával is csatlakozhat.

forget-dialog = El akarja felejteni ezt a Wi-Fi hálózatot?
    .description = A jelszót ismét meg kell adnia, ha újra csatlakozni szeretne.

network-device-state =
    .activated = Csatlakozva a hálózathoz
    .config = Csatlakozás a hálózathoz
    .deactivating = Hálózat szétkapcsolása
    .disconnected = Szétkapcsolva
    .failed = Csatlakozás sikertelen
    .ip-check = Kapcsolat ellenőrzése
    .ip-config = IP és router információk lekérése
    .need-auth = Azonosítás szükséges
    .prepare = Felkészülés a hálózatra csatlakozáshoz
    .secondaries = Várakozás egy másodlagos kapcsolatra
    .unavailable = Elérhetetlen
    .unknown = Ismeretlen állapot
    .unmanaged = Kezeletlen
    .unplugged = Kábel kihúzva

remove-connection-dialog = Törli a kapcsolati profilt?
    .vpn-description = A jelszót újra be kell írnia a jövőbeli használathoz.
    .wired-description = A profilt újból létre kell hoznia a jövőbeli használathoz.

vpn = VPN
    .connections = VPN-kapcsolatok
    .error = Nem sikerült hozzáadni a VPN-konfigurációt
    .remove = Kapcsolati profil törlése
    .select-file = Válasszon ki egy VPN konfigurációs fájlt

vpn-error = VPN Hiba
    .config = Nem sikerült hozzáadni a VPN-konfigurációt
    .connect = Nem sikerült csatlakozni a VPN-hez
    .connection-editor = A hálózati beállító nem működik
    .connection-settings = Nem sikerült lekérni az aktív kapcsolatok beállításait
    .updating-state = Nem sikerült frissíteni a hálózatkezelő állapotát
    .wireguard-config-path = Érvénytelen fájlelérési út a WireGuard konfigurációhoz
    .wireguard-config-path-desc = A kiválasztott fájlnak helyi fájlrendszeren kell lennie.
    .wireguard-device = Nem sikerült létrehozni a WireGuard eszközt
    .with-password = Nem sikerült beállítani a VPN { $field ->
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
    .description = Válasszon egy eszköz nevet a WireGuard konfigurációhoz.

## Networking: Online Accounts

online-accounts = Online fiókok
    .desc = fiókok hozzáadása, IMAP és SMTP, vállalati bejelentkezések

# Bluetooth

confirm = Megerősítés

bluetooth = Bluetooth
    .desc = Bluetooth-eszközök kezelése
    .status = Ez a rendszer { $aliases } néven látható, amíg a Bluetooth beállítások vannak megnyitva.
    .connected = Kapcsolódva
    .connecting = Kapcsolódás
    .disconnecting = Szétkapcsolás
    .connect = Kapcsolódás
    .disconnect = Szétkapcsolás
    .forget = Elfelejt
    .dbus-error = Hiba történt a DBus-szal való kommunikáció során: { $why }

bluetooth-paired = Korábban párosított eszközök
    .connect = Kapcsolódás
    .battery = { $percentage }% töltöttség

bluetooth-confirm-pin = Bluetooth PIN megerősítése
    .description = Kérjük, erősítse meg, hogy a következő PIN megegyezik a(z) { $device } eszközön megjelenített PIN-kóddal

bluetooth-available = Közeli eszközök

bluetooth-adapters = Bluetooth adapterek

## Accessibility

accessibility = Akadálymentesség
    .vision = Látás
    .on = Bekapcsolva
    .off = Kikapcsolva
    .unavailable = Nem elérhető
    .high-contrast = Kontrasztos mód
    .invert-colors = Színek invertálása
    .color-filters = Színszűrők

hearing = Hallás
    .mono = Sztereó hang lejátszása monóként

default = Alapértelmezett
magnifier = Nagyító
    .controls = Vagy használja ezeket a billentyűparancsokat: { $zoom_in ->
             [zero] {""}
            *[other] {""}
                {$zoom_in} a nagyításhoz,
        }{ $zoom_out ->
             [zero] {""}
            *[other] {""}
                {$zoom_out} a kicsinyítéshez,
        }
        Super + görgetés az egérrel
    .scroll_controls = "Super + Görgetés" (egérrel vagy érintőpárnával) billentyűparancsok engedélyezése
    .show_overlay = Nagyító megjelenítése
    .increment = Nagyítás mértéke
    .signin = Nagyító indítása bejelentkezéskor
    .applet = A nagyítót be- és kikapcsolhatja a panelen található kisalkalmazásban
    .movement = Nagyított nézet mozgása
    .continuous = A mutatóval
    .onedge = Amikor a mutató eléri a szélet
    .centered = Hogy a mutató középen maradjon
color-filter = Színszűrő típusa
    .unknown = Ismeretlen szűrő aktív
    .greyscale = Szürkeárnyalatos
    .deuteranopia = Zöld/Piros (zöld színre vak, Deuteranopia)
    .protanopia = Piros/Zöld (vörös színre vak, Protanopia)
    .tritanopia = Kék/Sárga (kék színre vak, Tritanopia)

## Desktop

desktop = Asztal

## Desktop: Wallpaper

wallpaper = Háttérkép
    .change = Képcserék gyakorisága
    .desc = Háttérképek, színek és diavetítési beállítások.
    .fit = Háttérkép illeszkedése
    .folder-dialog = Válasszon háttérkép mappát
    .image-dialog = Válasszon háttérkép képet
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

x-minutes = { $number } perc
x-hours = { $number ->
    [1] 1 óra
    *[other] { $number } óra
}
never = Soha

## Desktop: Appearance

appearance = Megjelenés
    .desc = Kiemelő színek és COSMIC témák.

accent-color = Kiemelő színek
app-background = Alkalmazás vagy ablak háttér
auto = Auto
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

auto-switch = Automatikus váltás a világos és sötét módok között
    .sunrise = Napkeltekor Világos módra vált
    .sunset = Napnyugtakor Sötét módra vált
    .next-sunrise = Következő napfelkeltekor Világos módra vált
    .next-sunset = Következő napnyugtakor Sötét módra vált

container-background = Tároló háttér
    .desc-detail = A tároló háttérszínét a navigációs oldalsáv, az oldalsó fiók, a párbeszédpanelek és hasonló widgetek használják. Alapértelmezés szerint automatikusan az alkalmazás vagy ablak hátteréből származik.
    .reset = Visszaállítás automatikusra
    .desc = Az elsődleges tárolószín a navigációs oldalsávhoz, az oldalsó fiókhoz, a párbeszédablakhoz és hasonló widgetekhez használatos.

control-tint = Irányító komponens színezése
    .desc = Szabványos gombok hátterére, keresési bemenetekre, szövegbevitelre és hasonló összetevőkre használatos.

frosted = Elmosódott üveg hatás a rendszerfelületen
    .desc = A háttér elmosását alkalmazza a panelre, a dokkolóra, a kisalkalmazásokra, az indítóra és az alkalmazáskönyvtárra.

enable-export = Alkalmazza ezt a témát a GNOME-alkalmazásokra.
    .desc = Nem minden eszközkészlet támogatja az automatikus váltást. Előfordulhat, hogy a téma módosítása után újra kell indítani a nem COSMIC alkalmazásokat.

icon-theme = Ikon téma
    .desc = Más ikonkészletet alkalmaz az alkalmazásokra.

text-tint = A felület szövegének árnyalata
    .desc = Színek, amelyek a felület szövegének színeinek származtatására szolgálnak, amelyek megfelelő kontrasztot mutatnak a különböző felületeken.

style = Stílus
    .round = Lekerekített
    .slightly-round = Némileg lekerekített
    .square = Négyzetes

interface-density = Felület sűrűsége
    .comfortable = Kényelmes
    .compact = Kompakt
    .spacious = Tágas

window-management = Ablakkezelés
    .active-hint = Aktív ablak kiemelésének mérete
    .gaps = Rések a csempézett ablakok körül

### Experimental

experimental-settings = Kísérleti beállítások
icons-and-toolkit = Ikonok és eszközkészlet témázása
interface-font = Rendszer betűtípusa
monospace-font = Állandó szélességű betűtípus

## Desktop: Notifications

notifications = Értesítések
    .desc = Ne zavarjanak, zárolási képernyő értesítések és alkalmazásonkénti beállítások.

## Desktop: Panel

panel = Panel
    .desc = Felső sáv asztali vezérlőkkel és menükkel

add = Hozzáadás
add-applet = Kisalkalmazás hozzáadása
all = Összes
applets = Kisalkalmazások
center-segment = Szegmens közepe
drop-here = Húzza ide a kisalkalmazásokat
end-segment = Szegmens vége
large = Nagy
no-applets-found = Nem található kisalkalmazás...
panel-bottom = Alul
panel-left = Balra
panel-right = Jobbra
panel-top = Felül
search-applets = Kisalkalmazások keresése...
small = Kicsi
start-segment = Szegmens eleje

panel-appearance = Megjelenés
    .match = Rendszertéma
    .light = Világos
    .dark = Sötét

panel-behavior-and-position = Viselkedés és pozíció
    .autohide = A panel automatikus elrejtése
    .dock-autohide = A dokkoló automatikus elrejtése
    .position = Elhelyezkedés a képernyőn
    .display = Megjelenítés a kijelzőn

panel-style = Stílus
    .anchor-gap = Hézag a panel és a képernyő szélei között
    .dock-anchor-gap = Hézag a dokkoló és a képernyő szélei között
    .extend = Panel kinyújtása a képernyő széléig
    .dock-extend = Dokkoló kinyújtása a képernyő széléig
    .appearance = Megjelenés
    .size = Méret
    .background-opacity = Háttér átlátszósága

panel-applets = Konfiguráció
    .dock-desc = Konfigurálja a dokkoló kisalkalmazásait.
    .desc = Konfigurálja a panel kisalkalmazásait.

panel-missing = A panel konfigurációja hiányzik
    .desc = A panel konfigurációs fájlja hiányzik, mert egyéni konfigurációt használtak, vagy sérült.
    .fix = Visszaállítás alapértelmezettre

## Desktop: Dock

dock = Dokkoló
    .desc = Panel kitűzött alkalmazásokkal és kisalkalmazásokkal az alkalmazástálcán.

## Desktop: Window management

window-management = Ablakkezelés
    .desc = Super billentyű funkció, ablakkezelési és csempézési beállítások.

super-key = Super billentyű
    .launcher = Indító megnyitása
    .workspaces = Munkaterületek megnyitása
    .applications = Alkalmazások megnyitása
    .disable = Kikapcsolás

edge-gravity = A lebegő ablakok a közeli szélekhez igazodnak

window-controls = Ablakkezelés
    .maximize = Maximalizálás gomb mutatása
    .minimize = Minimalizálás gomb mutatása
    .active-window-hint = Aktív ablak kiemelése

focus-navigation = Fókusznavigáció
    .focus-follows-cursor = Fókusz követi az egeret
    .focus-follows-cursor-delay = Fókuszálás késleltetése (ms)
    .cursor-follows-focus = Mutató követi a fókuszt

## Desktop: Workspaces

workspaces = Munkaterületek
    .desc = Munkaterületek tájolása és viselkedése.

workspaces-behavior = Munkaterületek viselkedése
    .dynamic = Dinamikus munkaterületek
    .dynamic-desc = Az üres munkamenetek automatikus eltávolítása.
    .fixed = Megadott számú munkaterület
    .fixed-desc = Munkaterületek hozzáadása vagy eltávolítása az áttekintésben.

workspaces-multi-behavior = Többmonitoros viselkedés
    .span = A munkaterületek kiterjednek a kijelzőkre
    .separate = A kijelzők külön munkaterülettel rendelkeznek

workspaces-overview-thumbnails = Munkaterület-áttekintő bélyegképek
    .show-number = Munkaterület számának megjelenítése
    .show-name = Munkaterület nevének megjelenítése

workspaces-orientation = Munkaterületek tájolása
    .vertical = Függőleges
    .horizontal = Vízszintes

hot-corner = Forró sarok
    .top-left-corner = A munkaterületek bal felső forró sarkának engedélyezése

## Displays

-requires-restart = Újraindítást igényel

color = Szín
    .depth = Színmélység
    .profile = Szín profil
    .sidebar = Szín profilok
    .temperature = Színhőmérséklet

display = Kijelzők
    .desc = Kijelzők kezelése, grafikus módváltás és éjszakai fény.
    .arrangement = Kijelző elrendezése
    .arrangement-desc = Húzza a kijelzőket az átrendezésükhöz.
    .enable = Kijelző engedélyezése
    .external = { $size } { $output } külső kijelző
    .laptop = { $size } laptop kijelző
    .options = Kijelző lehetőségek
    .refresh-rate = Frissítési gyakoriság
    .resolution = Felbontás
    .scale = Skálázás
    .additional-scale-options = További skálázási beállítások

mirroring = Tükrözés
    .id = Tükrözés { $id }
    .dont = Ne tükrözzön
    .mirror = Tükrözze a(z) { $display } kijelzőt
    .project = Kivetítés { $display ->
        [all] az összes kijelzőre
        *[other] a(z) { $display } kijelzőre
    }
    .project-count = Kivetítés { $count} további { $count ->
        [1] kijelzőre
        *[other] kijelzőkre
    }

night-light = Éjszakai fény
    .auto = Automatikus (naplementétől napkeltéig)
    .desc = Csökkentse a kék fényt melegebb színekkel.

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
    .title = Megtartja ezeket a kijelzőbeállításokat?
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

profile = Profilok

## Power

power = Energia és akkumulátor
    .desc = Energia beállítások kezelése

battery = Akkumulátor
  .minute = { $value } { $value ->
        [one] perc
       *[other] perc
  }
  .hour = { $value } { $value ->
        [one] óra
       *[other] óra
  }
  .day = { $value } { $value ->
        [one] nap
       *[other] nap
  }
  .less-than-minute = Kevesebb, mint egy perc
  .and = és
  .remaining-time = { $time } { $action ->
        [full] a teljes töltöttségig
       *[other] a lemerülésig
   }

connected-devices = Csatlakoztatott eszközök
  .unknown = Ismeretlen eszköz

power-mode = Energiagazdálkodási mód
    .battery = Meghosszabbított akkumulátor-üzemidő
    .battery-desc = Csökkentett energiafogyasztás és csendes teljesítmény.
    .balanced = Kiegyensúlyozott
    .balanced-desc = Csendes teljesítmény és mérsékelt energiafogyasztás.
    .performance = Nagy teljesítmény
    .performance-desc = Csúcsteljesítmény és energiafelhasználás.
    .no-backend = A háttérprogram nem található. Telepítse a system76-power vagy a power-profiles-daemont.

power-saving = Energiagazdálkodási beállítások
    .turn-off-screen-after = A képernyő kikapcsolása
    .auto-suspend = Automatikus felfüggesztés
    .auto-suspend-ac = Automatikus felfüggesztés hálózati áramforráson
    .auto-suspend-battery = Automatikus felfüggesztés akkumulátoron

## Input

acceleration-desc = Automatikusan beállítja a követési érzékenységet a sebesség alapján.

disable-while-typing = Letiltás gépelés közben

input-devices = Beviteli eszközök
    .desc = Beviteli eszközök

primary-button = Elsődleges gomb
    .desc = A gombok sorrendjének beállítása.
    .left = Bal
    .right = Jobb

scrolling = Görgetés
    .two-finger = Görgetés két ujjal
    .edge = Egy ujjal görgetés az érintőpárna szélén
    .speed = Görgetési sebesség
    .natural = Természetes görgetés
    .natural-desc = A görgetés a tartalmat mozgatja, nem a nézetet.

## Input: Keyboard

slow = Lassú
fast = Gyors
short = Rövid
long = Hosszú
keyboard = Billentyűzet
    .desc = Bemeneti források, váltásuk, speciális karakterek, billentyűparancsok

keyboard-sources = Bemeneti források
    .desc = A bemeneti források a Super+Space billentyűkombinációval válthatók. Ez testreszabható a billentyűparancsok beállításaiban.
    .move-up = Mozgatás feljebb
    .move-down = Mozgatás lejjebb
    .settings = Beállítások
    .view-layout = Billentyűkiosztás megtekintése
    .remove = Eltávolítás
    .add = Bemeneti forrás hozzáadása

keyboard-special-char = Speciális karakter beírása
    .alternate = Alternatív karakterek billentyűje
    .compose = Kombináló billentyű
    .caps = Caps Lock billentyű

keyboard-typing-assist = Gépelés
    .repeat-rate = Ismétlési sebesség
    .repeat-delay = Ismétlési késleltetés

keyboard-numlock-boot = Num Lock
    .boot-state = Állapot indításkor
    .last-boot = Utolsó indítás
    .on = Bekapcsolva
    .off = Kikapcsolva
    .set = Num Lock indítási állapotának beállítása

added = Hozzáadva
type-to-search = Gépeljen a kereséshez...
show-extended-input-sources = Bővített bemeneti források megjelenítése

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Billentyűparancsok
    .desc = Billentyűparancsok megtekintése és testreszabása

add-another-keybinding = Új billentyűparancs hozzáadása
cancel = Mégse
command = Parancs
custom = Egyedi
debug = Hibakeresés
disabled = Tiltva
input-source-switch = Billentyűzetkiosztás váltása
migrate-workspace-prev = Munkaterület áthelyezése az előző kimenetre
migrate-workspace-next = Munkaterület áthelyezése a következő kimenetre
migrate-workspace = Munkaterület áthelyezése { $direction ->
    *[down] az alsó
    [left] a bal
    [right] a jobb
    [up] a felső
} kimenetre
navigate = Navigálás
replace = Kicserélés
shortcut-name = Billentyűparancs neve
system-controls = Rendszervezérlés
terminate = Befejezés
toggle-stacking = Az ablakok egymásra helyezésének be- és kikapcsolása
type-key-combination = Billentyűkombináció

custom-shortcuts = Egyedi billentyűkombinációk
    .add = Billentyűkombináció hozzáadása
    .context = Egyedi billentyűkombináció hozzáadása
    .none = Nincsenek egyéni billentyűkombinációk

modified = { $count } módosítva

nav-shortcuts = Navigáció
    .prev-output = Az előző kimenet fókuszálása
    .next-output = A következő kimenet fókuszálása
    .last-workspace = Az utolsó munkaterület fókuszálása
    .prev-workspace = Az előző munkaterület fókuszálása
    .next-workspace = Az következő munkaterület fókuszálása
    .focus = Ablak fókuszálása { $direction ->
        *[down] le
        [in] be
        [left] balra
        [out] ki
        [right] jobbra
        [up] fel
    }
    .output = Váltás { $direction ->
        *[down] lenti
        [left] bal
        [right] jobb
        [up] fenti
    } kimenetre
    .workspace = Váltás a(z) { $num }. munkaterületre

manage-windows = Ablakok kezelése
    .close = Ablak bezárása
    .maximize = Ablak maximalizálása
    .minimize = Ablak minimalizálása
    .resize-inwards = Ablak átméretezése befelé
    .resize-outwards = Ablak átméretezése kifelé
    .toggle-sticky = Ragadós ablak be- és kikapcsolása

move-windows = Ablakok mozgatása
    .direction = Ablakok mozgatása { $direction ->
        *[down] le
        [left] balra
        [right] jobra
        [up] fel
    }
    .display = Ablak mozgatása egy monitort { $direction ->
        *[down] le
        [left] balra
        [right] jobra
        [up] fel
    }
    .workspace = Ablak mozgatása egy munkaterülettel { $direction ->
        *[below] lejjebb
        [left] balra
        [right] jobbra
        [above] feljebb
    }
    .workspace-num = Ablak áthelyezése a(z) { $num }. munkaterületre
    .prev-workspace = Ablak áthelyezése az előző munkaterületre
    .next-workspace = Ablak áthelyezése a következő munkaterületre
    .last-workspace = Ablak áthelyezése az utolsó munkaterületre
    .next-display = Ablak áthelyezése a következő képernyőre
    .prev-display = Ablak áthelyezése az előző képernyőre
    .send-to-prev-workspace = Ablak küldése az előző munkaterületre
    .send-to-next-workspace = Ablak küldése a következő munkaterületre

system-shortcut = Rendszer
    .app-library = Alkalmazáskönyvtár megnyitása
    .brightness-down = Kijelző fényerejének csökkentése
    .brightness-up = Kijelző fényerejének növelése
    .home-folder = Saját mappa megnyitása
    .keyboard-brightness-down = Billentyűzet fényerejének csökkentése
    .keyboard-brightness-up = Billentyűzet fényerejének növelése
    .launcher = Indító megnyitása
    .log-out = Kijelentkezés
    .lock-screen = Képernyő zárolása
    .mute = Hangkimenet némítása
    .mute-mic = Elnémítja a mikrofon bemenetet
    .play-pause = Lejátszás/Szünet
    .play-next = Következő szám
    .play-prev = Előző szám
    .screenshot = Képernyőkép készítése
    .terminal = Egy terminál megnyitása
    .volume-lower = Hangkimenet hangerejének csökkentése
    .volume-raise = Hangkimenet hangerejének növelése
    .web-browser = Megnyit egy webböngészőt
    .window-switcher = Váltás a nyitott ablakok között
    .window-switcher-previous = Váltás a nyitott ablakok között fordított sorrendben
    .workspace-overview = Munkaterület áttekintésének megnyitása

window-tiling = Ablak csempézés
    .horizontal = Vízszintes tájolás beállítása
    .vertical = Függőleges tájolás beállítása
    .swap-window = Ablak cserélése
    .toggle-tiling = Az Ablakok csempézésének be- és kikapcsolása
    .toggle-stacking = Az Ablakok egymásra helyezésének be- és kikapcsolása
    .toggle-floating = Az ablakok lebegtetésének be- és kikapcsolása
    .toggle-orientation = Tájolás váltása

replace-shortcut-dialog = Lecseréli a parancsikont?
    .desc = A(z) { $shortcut } használja a(z) { $name }. Ha lecseréli, a(z) { $name } le lesz tiltva.

zoom-in = Nagyítás
zoom-out = Kicsinyítés

## Input: Mouse

mouse = Egér
    .desc = Egérsebesség, gyorsítás és természetes görgetés.
    .speed = Egér sebessége
    .acceleration = Egérgyorsítás engedélyezése

## Input: Touchpad

click-behavior = Kattintási viselkedés
    .click-finger = Másodlagos kattintás két ujjal és középső kattintás három ujjal
    .button-areas = Másodlagos kattintás a jobb alsó sarokban, középső kattintás az alsó középső sarokban

pinch-to-zoom = Csippentéses nagyítás
    .desc = Használja két ujját a tartalom nagyításához a nagyítást támogató alkalmazásokhoz.

tap-to-click = Koppintson a kattintáshoz
    .desc = Engedélyezi az egyujjas koppintást az elsődleges kattintáshoz, a kétujjas koppintást a másodlagos kattintáshoz és a háromujjas érintést a középső kattintáshoz.

touchpad = Érintőpárna
    .acceleration = Az Érintőpárna gyorsításának engedélyezése
    .desc = Érintőpárna sebessége, kattintási lehetőségek, gesztusok.
    .speed = Érintőpárna sebessége

## Input: Gestures

gestures = Gesztusok
    .four-finger-down = Négy ujjas csúsztatás lefelé
    .four-finger-left = Négy ujjas csúsztatás balra
    .four-finger-right = Négy ujjas csúsztatás jobbra
    .four-finger-up = Négy ujjas csúsztatás felfelé
    .three-finger-any = Három ujjas csúztatás bármelyik irányba

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
    .desc = Időzóna, automatikus órabeállítások és időformátum.
    .auto = Beállítás automatikusan
    .auto-ntp = A dátum és idő automatikusan frissül, ha az időzóna be van állítva.

time-zone = Időzóna
    .auto = Automatikus időzóna
    .auto-info = Helymeghatározási szolgáltatások és internetkapcsolat szükséges

time-format = Dátum- és időformátum
    .twenty-four = 24 órás időformátum
    .show-seconds = Másodpercek mutatása
    .first = A hét első napja
    .show-date = Dátum megjelenítése a felső panelen
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
    .desc = A nyelvek sorrendje határozza meg, hogy melyik nyelvet használja az asztali környezet. A módosítások a következő bejelentkezéskor lépnek életbe.

add-language = Nyelv hozzáadása
    .context = Nyelv hozzáadása
install-additional-languages = További nyelvek telepítése
region = Régió

## Applications

applications = Alkalmazások

## Applications: Default Applications

default-apps = Alapértelmezett alkalmazások
    .desc = Alapértelmezett böngésző, levelezőprogram, fájlkezelő és egyéb alkalmazások.
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
    .desc = Azoknak az alkalmazásoknak a beállítása, amelyek bejelentkezéskor elindulnak.
    .add = Alkalmazás hozzáadása
    .user = Felhasználói alkalmazások
    .user-description = Ezek az alkalmazások a jelenlegi felhasználó bejelentkezésekor indulnak el.
    .remove-dialog-title = { $name } eltávolítása?
    .remove-dialog-description = Biztosan el akarja távolítani ezt az indítási alkalmazások közül?
    .search-for-application = Alkalmazás keresése

## Applications: Legacy Applications

legacy-applications = X11 alkalmazások támogatása
    .desc = X11 ablakkezelő rendszer alkalmazásméretezése és globális billentyűparancsok.

legacy-app-global-shortcuts = Globális billentyűparancsok X11 alkalmazásokban
    .desc = A globális parancsikonok lehetővé teszik, hogy az alkalmazásokban a billentyű- és egérkattintás eseményeket más alkalmazások is felismerjék, például a "nyomd-hogy-beszélj" vagy a "nyomd a némításhoz" funkciókhoz. Alapértelmezés szerint ez az X11 alkalmazásokban le van tiltva, hogy más alkalmazások ne figyelhessék a billentyűzet és az egér eseményeit, amelyek érzékeny információkat tartalmazhatnak.
    .none = Nincsnek billentyűk
    .modifiers = Módosítók (Super, Shift, Control, Alt)
    .combination = Minden billentyű, miközben a Super, Control vagy Alt módosítók lenyomva vannak
    .all = Minden billentyű
    .mouse = Egérgomb-események X11 alkalmazásokban

legacy-app-scaling = X11-ablakrendszer alkalmazásméretezés
    .scaled-by-system = Minden X11-alkalmazás méretezése
    .system-description = Az X11-alkalmazások elmosódottak lehetnek HiDPI kijelzőkön.
    .scaled-natively = X11-alkalmazások renderelése natív felbontásban
    .native-description = Azok az X11-alkalmazások, amelyek nem támogatják a méretezést, kisméretűek lesznek HiDPI kijelzőkön. Engedélyezd a teljes monitorfelbontás kihasználásához játékok esetén.

## System

system = Rendszer és fiókok

## System: About

about = Rendszerinformáció
    .desc = Eszköznév, hardverinformációk és az operációs rendszer alapértelmezett beállításai.

about-device = Eszköz neve
    .desc = Ez a név más hálózati vagy Bluetooth-eszközök számára látható.

about-hardware = Hardver
    .model = Hardver modell
    .memory = Memória
    .processor = Processzor
    .graphics = Grafika
    .disk-capacity = Tárhely

about-os = Operációs rendszer
    .os = Operációs rendszer
    .os-architecture = Operációs rendszer architektúra
    .desktop-environment = Asztali környezet
    .windowing-system = Ablakrendszer

about-related = Kapcsolódó beállítások
    .support = Támogatás kérése

## System: Firmware

firmware = Firmware
    .desc = Firmware részletei.

## System: Users

users = Felhasználók
    .desc = Hitelesítés és felhasználói fiókok.
    .admin = Rendszergazda
    .standard = Normál
    .profile-add = Profilkép kiválasztása

administrator = Rendszergazda
    .desc = A rendszergazdák megváltoztathatják az összes felhasználó beállításait, új felhasználókat adhatnak hozzá és távolíthatnak el

add-user = Felhasználó hozzáadása
remove-user = Felhasználó eltávolítása
full-name = Teljes név
invalid-username = Érvénytelen felhasználónév
