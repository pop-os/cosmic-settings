app = COSMIC Beállítások

unknown = Ismeretlen

number = { $number }


## Network & Wireless

connections-and-profiles = { $variant ->
    [wired] Vezetékes
    [wifi] Wi-Fi
    [vpn] VPN
    *[other] Ismeretlen
} Kapcsolatok és kapcsolati profilok.

add-network = Hálozat hozzáadása
add-vpn = VPN hozzáadása
airplane-on = Repülőgép üzemmód be van kapcsolva.
cable-unplugged = Kábel kihúzva
connect = Kapcsolódás
connected = Kapcsolódva
connecting = Kapcsolódás...
disconnect = Szétkapcsolás
forget = Elfelejtés
known-networks = Ismert hálózatok
network-and-wireless = Internet és Wi-fi
no-networks = Nem található hálózat.
no-vpn = Nincs elérhető VPN kapcsolat.
password = Jelszó
remove = Törlés
settings = Beállítások
username = Felhasználónév
visible-networks = Látható hálózatok

auth-dialog = Azonosítás szükséges
    .vpn-description = Írja be a VPN szolgáltatás által követelt felhasználónevet és jelszót.
    .wifi-description = Írja be a jelszót vagy a titkosítókulcsot. A rúter "WPS" gombjának megnyomásával is csatlakozhat.

forget-dialog = El akarja felejteni ez a Wi-Fi hálózatot?
    .description = A jelszót újra be kell írnia a jővőbeli használathoz.

network-device-state =
    .activated = Csatlakozva a hálózathoz
    .config = Csatlakozás a hálózathoz
    .deactivating = Hálózat szétkapcsolása
    .disconnected = Szétkapcsolva
    .failed = Csatlakozás sikertelen
    .ip-check = Kapcsolat ellenőrzése
    .ip-config = IP és rúter információk lekérése
    .need-auth = Azonosítás szükséges
    .prepare = Felkészülés a hálózatra csatlkozáshoz
    .secondaries = Várakozás egy másodlagos kapcsolatra
    .unavailable = Elérhetetlen
    .unknown = Ismeretlen állapot
    .unmanaged = Kezeletlen
    .unplugged = Kábel kihúzva

remove-connection-dialog = Törli a kapcsolódási profilt?
    .vpn-description = A jelszót újra be kell írnia a jővőbeli használathoz.
    .wired-description = A profil újból létre kell hoznia a jővőbeli használathoz.

vpn = VPN
    .connections = VPN kapcsolatok
    .remove = Kapcsolódási profil törlése
    .select-file = Válassz ki egy VPN konfigurációs fájlt

wired = Vezetékes
    .connections = Vezetékes kapcsolatok
    .devices = Vezetékes eszközök
    .remove = Kapcsolódási profil törlése

wifi = Wi-Fi
    .forget = Hálózat elfelejtése

## Networking: Online Accounts

online-accounts = Online fiókok
    .desc = fiókok hozzáadása, IMAP és SMTP, vállalati bejelentkezések


## Desktop

desktop = Asztal

## Desktop: Appearance

appearance = Kinézet
    .desc = Kiemelő színék és COSMIC témák.

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
recent-colors = Előző színek
reset-to-default = Visszaállítás alapértelmezettre
rgb = RGB
window-hint-accent = Aktív ablak kiemelő szín
window-hint-accent-toggle = Használja a téma kiemelő színét aktív ablak tippként

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

frosted = Tetüveg hatás a rendszer interfészén
    .desc = A háttér elmosását alkalmazza a panelre, a dokkolóra, a kisalkalmazásokra, az indítóra és az alkalmazáskönyvtárra.

experimental-settings = Kísérleti beállítások

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

# interface density left out for now
window-management = Ablakkezelés
    .active-hint = Aktív ablak tipp mérete
    .gaps = Rések a csempézett ablakok körül

## Desktop: Display

-requires-restart = Újraindítást igényel

color = Szín
    .depth = Színmélység
    .profile = Szín profil
    .sidebar = Szín profilok
    .temperature = Színhőmérséklet

display = Kijelzők
    .desc = Kezelje a kijelzőket, a grafika váltást és az éjszakai fényt
    .arrangement = Kijelző elrendezése
    .arrangement-desc = Húzza a kijelzőket az átrendezésükhöz.
    .enable = Kijelző engedélyezése
    .external = { $size } { $output } Külső kijelző
    .laptop = { $size } Laptop kijelző
    .options = Kijelző lehetőségek
    .refresh-rate = Frissítési ráta
    .resolution = Felbontás
    .scale = Skála

mirroring = Tükrözés
    .id = Tükrözés { $id }
    .dont = Ne tükrözzön
    .mirror = Tükrözze { $display }
    .project = Kivetítés { $display ->
        [all] minden kijelzőre
        *[other] { $display }
    }
    .project-count = Kivetítés { $count} egyéb { $count ->
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

scheduling = Ütemezés
    .manual = Kézi ütemezés

dialog = Párbeszédablak
    .title = Megtartja ezeket a kijelzőbeállításokat?
    .keep-changes = Változtatások megtartása
    .change-prompt = A beállítások automatikusan visszaállnak { $time } másodperc múlva.
    .revert-settings = Beállítások visszaállítása

## Desktop: Notifications

notifications = Értesítések
    .desc = Ne zavarjanak, zárolt képernyő értesítések, és alkalmazásonkénti beállításokat.

## Desktop: Options

desktop-panel-options = Asztal és Panel
    .desc = Super billentyű művelet, forró sarkok, ablakvezérlési lehetőségek.

desktop-panels-and-applets = Asztal panelek és kisalkalmazások

dock = Dokkoló
    .desc = Panel rögzített alkalmazásokkal.

hot-corner = Forró sarok
    .top-left-corner = A munkaterületek bal felső forró sarkának engedélyezése

super-key = Super billentyű
    .launcher = Indító megnyitása
    .workspaces = Munkaterületek megnyitása
    .applications = Applikációk menü megnyitása

top-panel = Felső panel
    .workspaces = Munkaterületek gomb mutatása
    .applications = Applikációk gomb mutatása

window-controls = Ablakvezérlők
    .minimize = Minimalizálás gomb mutatása
    .maximize = Maximalizálás gomb mutatása

## Desktop: Panel

panel = Panel
    .desc = Felső sáv asztali vezérlőkkel és menükkel.

add = Hozzáadás
add-applet = Kisalkalmazás hozzáadása
all = Összes
applets = Kisalkalmazások
center-segment = Középső szegmens
drop-here = Dobja ide a Kisalkalmazásokat
end-segment = Szegmens vége
large = Nagy
no-applets-found = Nem található kisalkalmazás...
panel-bottom = Alul
panel-left = Bal
panel-right = Jobb
panel-top = Felül
search-applets = Kisalkalmazások keresése...
small = Kicsi
start-segment = Szegmens indítása

panel-appearance = Kinézet
    .match = Asztallal egyező
    .light = Világos
    .dark = Sötét

panel-behavior-and-position = Viselkedés és pozíciók
    .autohide = Panel automatikus elrejtése
    .dock-autohide = A dokkoló automatikus elrejtése
    .position = Elhelyezés a képernyőn
    .display = Megjelenítés a kijelzőn

panel-style = Stílus
    .anchor-gap = Hézag a panel és a képernyő szélei között
    .dock-anchor-gap = Hézag a dokkoló és a képernyő szélei között
    .extend = Panel kinyújtása a képernyő széléig
    .dock-extend = Dokkoló kinyújtása a képernyő széléig
    .appearance = Kinézet
    .size = Méret
    .background-opacity = Háttér átlátszósága

panel-applets = Konfiguráció
    .dock-desc = Konfigurálja a dokkolókisalkalmazásait.
    .desc = Konfigurálja a panelkisalkalmazásait.

panel-missing = A panel konfigurációja hiányzik
    .desc = A panel konfigurációs fájlja hiányzik, mert egyéni konfigurációt használtak, vagy sérült.
    .fix = Visszaállítás alapértelmezettre

## Desktop: Wallpaper

wallpaper = Háttérkép
    .change = Háttérkép megváltoztatása minden
    .desc = Háttérképek, színek és diavetítési lehetőségek.
    .fit = Háttérkép illeszkedése
    .folder-dialog = Válasszon háttérkép mappát
    .image-dialog = Válasszon háttérképet
    .plural = Háttérképek
    .same = Ugyanaz a háttérkép minden kijelzőn
    .slide = Diavetítés

add-color = Szín hozzáadása
add-image = Kép hozzáadása
all-displays = Minden kijelző
colors = Színek
dialog-add = Hozzáadás
fill = Kitöltés
fit-to-screen = Képernyőhöz igazítva
open-new-folder = Új mappa megnyitása
recent-folders = Legutóbbi mappák

x-minutes = { $number } perc
x-hours = { $number ->
    [1] 1 óra
    *[other] { $number } óra
}

## Desktop: Workspaces

workspaces = Munkaterületek
    .desc = Állítsa be a munkaterület számát, viselkedését és elhelyezését.

workspaces-behavior = Munkaterületek viselkedése
    .dynamic = Dinamikus munkaterületek
    .dynamic-desc = Automatikusan eltávolítja az üres munkaterületeket.
    .fixed = Megadott számú munkaterület
    .fixed-desc = Munkaterületek hozzáadása vagy eltávolítása az áttekintésben.

workspaces-multi-behavior = Többmonitoros viselkedés
    .span = Munkaterületek megosztottak az összes kijelzőn
    .separate = A kijelzőknek külön munkaterületük van

workspaces-overview-thumbnails = A munkaterület áttekintési miniatűrök
    .show-number = Munkaterület számának megjelenítése
    .show-name = Munkaterület nevének megjelenítése

workspaces-orientation = Munkaterületek tájolása
    .vertical = Függőleges
    .horizontal = Vízszintes

## Networking: Wired

wired = Vezetékes
    .desc = Vezetékes kapcsolat, csatlakozási profilok

## Networking: Online Accounts

online-accounts = Online fiókok
    .desc = Fiókok hozzáadása, IMAP és SMTP, vállalati bejelentkezés

## Time & Language

time = Idő és nyelv
    .desc = N/A

time-date = Dátum és idő
    .desc = Időzóna, automatikus órabeállítások és bizonyos időformázás.
    .auto = Automatikus beállítás

time-zone = Időzóna
    .auto = Automatikus időzóna
    .auto-info = Helymeghatározási szolgáltatást és internet-hozzáférést igényel

time-format = Dátum és idő formátum
    .twenty-four = 24 órás idő
    .first = A hét első napja
    .show-date = Dátum megjelenítése a felső panelen
    .friday = Péntek
    .saturday = Szombat
    .sunday = Vasárnap
    .monday = Hétfő

time-region = Régió és nyelv
    .desc = Formázza a dátumokat, időpontokat és számokat a régiója alapján

## Sound

sound = Hang
    .desc = N/A

sound-output = Kimenet
    .volume = Kimeneti hangerő
    .device = Kimeneti eszköz
    .level = Kimeneti szint
    .config = Konfiguráció
    .balance = Egyensúlyozás

sound-input = Bemenet
    .volume = Bemeneti hangerő
    .device = Bemeneti eszköz
    .level = Bemeneti szint

sound-alerts = Figyelmeztetések
    .volume = Figyelmeztetések hangereje
    .sound = Figyelmeztető hang

sound-applications = Alkalmazások
    .desc = Alkalmazások hangerejei és beállításai

## System

system = Rendszer és fiókok

## System: About

about = Névjegy
    .desc = Eszköznév, hardverinformációk, operációs rendszer alapértelmezett beállításai.

about-device = Eszköz neve
    .desc = Ez a név más hálózati vagy bluetooth-eszközökön jelenik meg.

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
    .support = Támogatás

## System: Firmware

firmware = Firmware
    .desc = Firmware részletei.

## System: Users

users = Felhasználók
    .desc = Hitelesítés és bejelentkezés, zár képernyő.

## Input

acceleration-desc = Automatikusan beállítja a követési érzékenységet a sebesség alapján.

disable-while-typing = Letiltás gépelés közben

input-devices = Beviteli eszközök
    .desc = Beviteli eszközök

primary-button = Elsődleges gomb
    .left = Bal
    .right = Jobb

scrolling = Görgetés
    .two-finger = Görgetés két ujjal
    .edge = Egy ujjal görgetés az érintőtábla szélén
    .speed = Görgetési sebesség
    .natural = Természetes görgetés
    .natural-desc = A görgetés a tartalmat mozgatja, nem a nézetet.

## Input: Keyboard

slow = Lassú
fast = Gyors
short = Rövid
long = Hosszú
keyboard = Billentyűzet
    .desc = Billentyűzet bemenet

keyboard-sources = Bemeneti források
    .desc = A bemeneti források a Super+Space billentyűkombinációval válthatók. Ez testreszabható a billentyűparancsok beállításaiban.
    .move-up = Mozgatás feljebb
    .move-down = Mozgatás lejjebb
    .settings = Beállítások
    .view-layout = Billentyűkiosztás megtekintése
    .remove = Eltávolítás
    .add = Bemeneti forrás hozzáadása

keyboard-special-char = Speciális karakter bejegyzés
    .alternate = Alternatív karakterek gomb
    .compose = Íráskulcs

keyboard-typing-assist = Gépelés
    .repeat-rate = Ismétlési arány
    .repeat-delay = Ismétlési késleltetés

added = Hozzáadva
type-to-search = Gépeljen a kereséshez...

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Gyorsbillentyűk
    .desc = Gyorsbillentyűk megtekintése és testreszabása

add-keybinding = Billentyűzetkötés hozzáadása
cancel = Mégse
command = Parancs
custom = Egyedi
debug = Hibakeresés
disabled = Tiltva
migrate-workspace-prev = Munkaterület áttelepítése az előző kimenetre
migrate-workspace-next = Munkaterület áttelepítése az Következő kimenetre
migrate-workspace = Munkaterület áttelepítése a { $direction ->
    *[down] alsó
    [left] bal
    [right] jobb
    [up] felsó
} kimenetre
navigate = Navigálás
replace = Kicserélés
shortcut-name = Gyorsbillentyűk neve
system-controls = System controls
terminate = Befejezés
toggle-stacking = Az ablakok egymásra helyezésének váltása
type-key-combination = Billentyűkombináció

custom-shortcuts = Egyedi billentyűkombinációk
    .add = Billentyűkombináció hozzáadása
    .context = Egyedi billentyűkombináció hozzáadása
    .none = Nincsenek egyéni billentyűkombinációk

modified = { $count }-szer szerkeszte

nav-shortcuts = Navigáció
    .prev-output = Az előző kimenet fókuszálása
    .next-output = A következő kimenet fókuszálása
    .last-workspace = Az utolsó munkaterület fókuszálása
    .prev-workspace = Az előző munkaterület fókuszálása
    .next-workspace = Az következő munkaterület fókuszálása
    .focus = Ablak fokuszálása { $direction ->
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
    .workspace = Cserélés az { $num }. munkaterületre

manage-windows = Ablakok kezelése
    .close = Ablak bezárása
    .maximize = Ablak maximalizálása
    .minimize = Ablak minimalizálása
    .resize-inwards = Ablak átméretezése befelé
    .resize-outwards = Ablak átméretezése kifelé
    .toggle-sticky = Ragadós ablak váltása

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
    .workspace-num = Ablak áthelyezése az { $num }. munkaterületre
    .prev-workspace = Ablak áthelyezése az előző munkaterületre
    .next-workspace = Ablak áthelyezése a következő munkaterületre
    .last-workspace = Ablak áthelyezése az utolsó munkaterületre
    .next-display = Ablak áthelyezése a következő képernyőre
    .prev-display = Ablak áthelyezése a előző képernyőre
    .send-to-prev-workspace = Ablak küldése az előző munkaterületre
    .send-to-next-workspace = Ablak küldése a következő munkaterületre

system-shortcut = Rendszer
    .app-library = Alkalmazáskönytár megnyitása
    .brightness-down = Kijelző fényerejének csökkentése
    .brightness-up = Kijelző fényerejének növelése
    .home-folder = Saját mappa megnyitása
    .keyboard-brightness-down = Billentyűzet fényerejének csökkentése
    .keyboard-brightness-up = Billentyűzet fényerejének növelése
    .launcher = Indító megnyitása
    .lock-screen = Képernyő zárolása
    .mute = Hangkimenet némítása
    .mute-mic = Elnémítja a mikrofon bemenetet
    .screenshot = Képernyőkép készítése
    .terminal = Egy terminál megnyitása
    .volume-lower = Hangkimenet hangerejének csökkentése
    .volume-raise = Hangkimenet hangerejének növelése
    .web-browser = Megynyit egy webböngészőt
    .window-switcher = Váltás a nyitott ablakok között
    .workspace-overview = Munkaterület áttekintésének megnyitása

window-tiling = Ablak csempézés
    .horizontal = Vízszintes tájolás beállítása
    .vertical = Függőleges tájolás beállítása
    .swap-window = Ablak cserélése
    .toggle-tiling = Ablak csempézésének váltása
    .toggle-stacking = Ablakok egymásra helyezésének váltása
    .toggle-floating = Lebegő ablak váltása
    .toggle-orientation = Tájolás váltása

replace-shortcut-dialog = Lecseréli a parancsikont?
    .desc = A { $shortcut } használja a { $name }. Ha lecseréli, a { $name } le lesz tiltva.

## Input: Mouse

mouse = Egér
    .desc = Egérsebesség, gyorsulás, természetes görgetés.
    .speed = Egér sebessége
    .acceleration = Egérgyorsítás engedélyezése

## Input: Touchpad

click-behavior = Klikk viselkedése
    .click-finger = Másodlagos kattintás két ujjal és középső kattintás három ujjal
    .button-areas = Másodlagos kattintás a jobb alsó sarokban, középső kattintás az alsó középső sarokban

pinch-to-zoom = Csípés a nagyításhoz
    .desc = Használja két ujját a tartalom nagyításához a nagyítást támogató alkalmazásokhoz.

tap-to-click = Koppintson a kattintáshoz
    .desc = Engedélyezi az egyujjas koppintást az elsődleges kattintáshoz, a kétujjas koppintást a másodlagos kattintáshoz és a háromujjas érintést a középső kattintáshoz.

touchpad = Érintőtábla
    .acceleration = Az Érintőtábla gyorsításának engedélyezése
    .desc = Érintőtábla sebessége, kattintási lehetőségek, gesztusok.
    .speed = Érintőtábla sebessége

## Input: Gestures

gestures = Gesztusok
    .four-finger-down = Négy ujjas csúsztatás lefelé
    .four-finger-left = Négy ujjas csúsztatás balra
    .four-finger-right = Négy ujjas csúsztatás jobbra
    .four-finger-up = Három ujjas csúsztatás minden irányba

switch-between-windows = Váltás az ablakok között
switch-to-next-workspace = Váltás a következő munkaterületre
switch-to-prev-workspace = Váltás a előző munkaterületre
open-application-library = Alkalmazáskönytár megnyitása
open-workspaces-view = Munkaterületek áttekintésének megnyitása

## Power

power = Energia
    .desc = Az energiabeállítások kezelése

power-mode = Energiagazdálkodási mód
    .performance = Nagy teljesítmény
    .balanced = Kiegyensúlyozott
    .battery = Meghosszabbított akkumulátor-élettartam
    .performance-desc = Csúcsteljesítmény és energiafelhasználás.
    .balanced-desc = Csendes teljesítmény és mérsékelt energiafogyasztás.
    .battery-desc = Csökkentett energiafogyasztás és csendes teljesítmény.
    .no-backend = A háttérprogram nem található. Telepítse a system76-power vagy a power-profiles-daemont.
