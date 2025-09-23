app = COSMIC-instellingen

dbus-connection-error = Kon geen verbinding maken met DBus
ok = OK
unknown = Onbekend

number = { $number }

## Network & Wireless

connections-and-profiles = { $variant ->
    [wired] Kabelverbindingen
    [wifi] Wifi-verbindingen
    [vpn] VPN-verbindingen
    *[other] Onbekende verbindingen
} en verbindingsprofielen.

add-network = Netwerk toevoegen
    .profile = Profiel toevoegen
add-vpn = VPN toevoegen
airplane-on = Vliegtuigmodus staat aan.
cable-unplugged = Kabel losgekoppeld
connect = Verbinden
connected = Verbonden
connecting = Verbinding maken…
disconnect = Verbinden verbreken
forget = Vergeten
known-networks = Bekende netwerken
network-and-wireless = Netwerk en wifi
no-networks = Geen netwerken gevonden.
no-vpn = Geen VPN-verbindingen beschikbaar.
password = Wachtwoord
password-confirm = Wachtwoord bevestigen
remove = Verwijderen
settings = Instellingen
username = Gebruikersnaam
visible-networks = Zichtbare netwerken
identity = Identiteit

auth-dialog = Authenticatie vereist
    .vpn-description = Voer de gebruikersnaam en het wachtwoord van de VPN-dienst in.
    .wifi-description = Voer het wachtwoord of de encryptiesleutel in. U kunt ook verbinden door op de “WPS”-knop op de router te drukken.

forget-dialog = Dit wifi-netwerk vergeten?
    .description = U zal opnieuw een wachtwoord moeten invoeren als u dit wifi-netwerk in de toekomst gaat gebruiken.
    
network-device-state =
    .activated = Verbonden
    .config = Verbinding maken…
    .deactivating = Verbinding verbreken…
    .disconnected = Verbinding verbroken
    .failed = Verbinding mislukt
    .ip-check = Verbinding controleren…
    .ip-config =  IP- en routeringsinformatie opvragen…
    .need-auth = Vereist authenticatie
    .prepare = Verbinding voorbereiden…
    .secondaries = Wachten op secundaire verbinding…
    .unavailable = Niet beschikbaar
    .unknown = Status onbekend
    .unmanaged = Onbeheerd
    .unplugged = Kabel losgekoppeld

remove-connection-dialog = Verbindingsprofiel verwijderen?
    .vpn-description =  U zal opnieuw een wachtwoord moeten invoeren als u dit netwerk in de toekomst gaat gebruiken.
    .wired-description = U zal dit profiel opnieuw moeten aanmaken om het in de toekomst te kunnen gebruiken.

vpn = VPN
    .connections = VPN-verbindingen
    .error = Kon VPN-configuratie niet toevoegen
    .remove = Verbindingsprofiel verwijderen
    .select-file = Selecteer een VPN-configuratiebestand

vpn-error = VPN-fout
    .config = Kon VPN-configuratie niet toevoegen
    .connect = VPN-verbinding mislukt
    .connection-editor =  Verbindingseditor mislukt
    .connection-settings = Het ophalen van instellingen voor actieve verbindingen is mislukt
    .updating-state = Kan status van netwerkbeheerder niet bijwerken
    .wireguard-config-path = Ongeldig bestandspad voor de WireGuard-configuratie
    .wireguard-config-path-desc = Het gekozen bestand moet op een lokaal bestandssysteem staan.
    .wireguard-device = Kon WireGuard-apparaat niet aanmaken
    .with-password = Kon { $field ->
        *[username] de VPN-gebruikersnaam
        [password] het VPN-wachtwoord
        [password-flags] de VPN-wachtwoordvlaggen
    } niet met nmcli toevoegen

wired = Bedraad
    .adapter = Bedrade adapter { $id }
    .connections = Bedrade verbindingen
    .devices = Bedrade apparaten
    .remove = Verbindingsprofielen verwijderen

wifi = Wifi
    .adapter = Wifi-adapter { $id }
    .forget = Dit netwerk vergeten

wireguard-dialog = WireGuard-apparaat toevoegen
    .description = Kies een apparaatnaam voor de WireGuard-configuratie.

## Networking: Online Accounts

online-accounts = Online accounts
    .desc = Accounts toevoegen, IMAP en SMTP, bedrijfslogins

# Bluetooth

activate = Activeren
confirm = Bevestigen
enable = Inschakelen

bluetooth = Bluetooth
    .desc = Bluetooth-apparaten bewerken
    .status = Dit systeem is zichtbaar als { $aliases } zolang de bluetooth-instellingen open staan.
    .connected = Verbonden
    .connecting = Verbinding maken
    .disconnecting = Verbinding verbreken
    .connect = Verbinden
    .disconnect = Verbinding verbreken
    .forget = Vergeten
    .dbus-error = Er is een fout opgetreden tijdens de interactie met DBus: { $why }
    .disabled = De bluetooth-service is uitgeschakeld
    .inactive = De bluetooth-service is niet actief
    .unknown = De bluetooth-service kon niet worden geactiveerd. Is BlueZ geïnstalleerd?

bluetooth-paired = Eerder verbonden apparaten
    .connect = Verbinden
    .battery = { $percentage }% batterij

bluetooth-confirm-pin = Bluetooth-pincode bevestigen
    .description = Controleer of de volgende pincode overeenkomt met de pincode die op { $device } wordt weergegeven

bluetooth-available = Nabije bluetooth-apparaten

bluetooth-adapters = Bluetooth-adapters

## Accessibility

accessibility = Toegankelijkheid
    .vision = Zicht
    .on = Aan
    .off = Uit
    .unavailable = Niet beschikbaar
    .screen-reader = Schermverteller
    .high-contrast = Verhoogde contrast modus
    .invert-colors = Kleuren omkeren
    .color-filters = Kleurfilters

hearing = Gehoor
    .mono = Stereo audio als mono afspelen

default = Standaard
magnifier = Vergrootglas
    .controls = U kunt ook deze sneltoetsen gebruiken: { $zoom_in ->
            [zero] {""}
            *[other] {""}
                {$zoom_in} om in te zoomen,
        }{ $zoom_out ->
             [zero] {""}
            *[other] {""}
                {$zoom_out} om uit te zoomen,
        }
        'super + scrollen' om met de muis of touchpad te zoomen
    .scroll_controls = Zoomen met 'super + scrollen' (met de muis of touchpad) inschakelen
    .show_overlay = Het vergrootglasmenu tonen
    .increment = Zoom-stapgrootte
    .signin = Het vergrootglas direct gebruiken als u inlogt
    .applet = Met een panel-applet het vergrootglas in-/uitschakelen
    .movement = De vergroting beweegt mee
    .continuous = De vergroting volgt de muis
    .onedge = Verplaatst als de muis de randen raakt
    .centered = Houdt de muis steeds in het midden van de vergroting
color-filter = Type kleurfilter
    .unknown = Onbekend kleurfilter actief
    .greyscale = Grijstinten
    .deuteranopia = Groen/Rood (groen-zwakte, Deuteranopie)
    .protanopia = Rood/Groen (rood-zwakte, Protanopie)
    .tritanopia = Blauw/Geel (blauw-zwakte, Tritanopie)

## Desktop

desktop = Bureaublad

## Desktop: Wallpaper

wallpaper = Schermachtergrond
    .change = Wijzig alle afbeeldingen
    .desc = Opties voor schermachtergrond, kleuren en diavoorstellingen.
    .fit = Achtergrond aanpassen
    .folder-dialog = Selecteer map met achtergronden
    .image-dialog = Selecteer achtergrond
    .plural = Achtergronden
    .same = Dezelfde achtergrond op alle schermen
    .slide = Diavoorstelling

add-color = Kleur toevoegen
add-image = Afbeelding toevoegen
all-displays = Alle beeldschermen
colors = Kleuren
dialog-add = Toevoegen
fill = Vullen
fit-to-screen = Aan het scherm passend maken
open-new-folder = Nieuwe map openen
recent-folders = Recente mappen

x-minutes = { $number } { $number ->
        [one] minuut
        *[other] minuten
    }
x-hours = { $number } uur
# these three lines are intentionally left empty


never = Nooit

## Desktop: Appearance

appearance = Kleuren en thema's
    .desc = Bepaal hoe COSMIC eruit ziet.

accent-color = Accentkleur
app-background = Toepassings- of vensterachtergrond
auto = Automatisch
close = Sluiten
color-picker = Kleurkiezer
copied-to-clipboard = Naar klembord gekopieerd
copy-to-clipboard = Naar klembord kopiëren
dark = Donker
export = Exporteren
hex = Hex
import = Importeren
light = Licht
mode-and-colors = Kleuren en lichte/donkere modus
recent-colors = Recente kleuren
reset-to-default = Naar standaardinstellingen terugzetten
rgb = RGB
window-hint-accent = Accentkleur als visuele hint voor het actieve venster
window-hint-accent-toggle = Gebruik de accentkleur van het thema als visuele hint voor het actieve venster

auto-switch = Automatisch wisselen tussen lichte en donkere modus
    .sunrise = Wissel naar lichte modus bij zonsopgang
    .sunset = Wissel naar donkere modus bij zonsondergang
    .next-sunrise = Wissel naar lichte modus bij de volgende zonsopgang
    .next-sunset = Wissel naar donkere modus bij de volgende zonsondergang

container-background = Containerachtergrond
    .desc-detail = De containerachtergrondskleur wordt gebruikt voor de navigatiebalk, het zijpaneel, dialoogvensters en soortgelijke widgets. Standaard wordt deze automatisch afgeleid van de applicatie- of vensterachtergrondkleur.
    .reset = Naar automatisch terugzetten
    .desc = De primaire containerkleur wordt gebruikt voor de navigatiezijbalk, het zijpaneel, dialoogvensters en soortgelijke widgets.

control-tint = Kleuring van controlecomponenten
    .desc = Wordt gebruikt voor achtergronden van standaardknoppen, zoek- en tekstinvoervelden en soortgelijke onderdelen.

frosted =  Matglaseffect op de systeeminterface
    .desc = Past achtergrondvervaging toe op het paneel, de dock, applets, snelstarter en het appmenu.

enable-export = Pas dit thema toe op GNOME-apps.
    .desc = Niet alle toolkit-omgevingen ondersteunen automatische thema wisseling. Niet-COSMIC-apps moeten mogelijk opnieuw worden opgestart na een themawijziging.

icon-theme = Icoonthema
    .desc = Past een andere set pictogrammen toe op toepassingen.

text-tint = Interfacetekstkleur
    .desc = Kleur die wordt gebruikt om interfacetekstkleuren te bepalen, zodat er voldoende contrast is op verschillende oppervlakken.

style = Stijl
    .round = Rond
    .slightly-round = Licht afgerond
    .square = Rechthoekig

interface-density = Interfacedichtheid
    .comfortable = Comfortabel
    .compact = Compact
    .spacious = Ruim

window-management-appearance = Vensterbeheer
    .active-hint = Grootte visuele hint voor het actieve venster
    .gaps = Ruimte rondom getegelde vensters

### Experimental

experimental-settings = Experimentele instellingen
icons-and-toolkit = Thema's voor pictogrammen en de toolkit
interface-font = Standaardlettertype
monospace-font = Lettertype met gelijke letterbreedte

## Desktop: Notifications

notifications = Meldingsinstellingen
    .desc = 'Niet storen', meldingen op het vergrendelingsscherm en meldingsinstellingen per app.

## Desktop: Panel

panel = Paneel
    .desc = Bovenbalk met bureaubladapplets en menu's.

add = Toevoegen
add-applet = Applet toevoegen
all = Alle
applets = Applets
center-segment = Middenstuk
drop-here = Applets hier plaatsen
end-segment = Eindstuk
large = Groot
no-applets-found = Geen applets gevonden...
panel-bottom = Onder
panel-left = Links
panel-right = Rechts
panel-top = Boven
search-applets = Applets zoeken...
small = Klein
start-segment = Beginstuk

panel-appearance = Uiterlijk
    .match = Systeemstandaard
    .light = Licht
    .dark = Donker

panel-behavior-and-position = Paneel: werking en positie
    .autohide = Paneel automatisch verbergen
    .dock-autohide = Dock automatisch verbergen
    .position = Positie op het scherm
    .display = Weergeven op het scherm

panel-style = Stijl
    .anchor-gap = Ruimte tussen het paneel en de schermranden
    .dock-anchor-gap = Ruimte tussen de dock en de schermranden
    .extend = Paneel tot de schermranden uitbreiden
    .dock-extend = Dock tot de schermranden uitbreiden
    .appearance = Uiterlijk
    .size = Grootte
    .background-opacity = Doorzichtigheid van de achtergrond

panel-applets = Configuratie
    .dock-desc = Dock-applets configureren
    .desc = Paneel-applets configureren

panel-missing = Paneelconfiguratie ontbreekt
    .desc = Het paneelconfiguratiebestand ontbreekt door het gebruik van een aangepaste configuratie of door corruptie.
    .fix = Naar standaardinstellingen terugzetten

## Desktop: Dock

dock = Dock
    .desc = Paneel met vastgezetten toepassingen vanuit het appmenu en andere applets.

## Desktop: Window management

window-management = Vensterbeheer
    .desc = Opties voor de Supertoets, vensterbeheer en aanvullende opties voor het tegelen van vensters.

super-key = Actie van de supertoets
    .launcher = Snelstarter openen
    .workspaces = Werkbladoverzicht openen
    .applications = Appmenu openen
    .disable = Supertoets uitschakelen

edge-gravity = Zwevende vensters worden door de randen van het scherm aangetrokken

window-controls = Vensterbeheer
    .maximize = Maximaliseerknop tonen
    .minimize = Minimaliseerknop tonen
    .active-window-hint = Gebruik visuele hint voor het actieve venster

focus-navigation = Focusbesturing
    .focus-follows-cursor = Focus volgt de cursor
    .focus-follows-cursor-delay = Vertraging voor focus volgt de cursor in ms
    .cursor-follows-focus = De cursor volgt focus

## Desktop: Workspaces

workspaces = Virtuele werkbladen
    .desc = Werking en oriëntatie van virtuele werkbladen instellen.

workspaces-behavior = Werking virtuele werkbladen
    .dynamic = Dynamische werkbladen
    .dynamic-desc = Lege werkbladen automatisch verwijderen.
    .fixed = Vast aantal werkbladen
    .fixed-desc = Werkbladen aan het overzicht toevoegen of verwijderen.

workspaces-multi-behavior = Werking over meerdere beeldschermen
    .span = Virtuele werkbladen strekken zich uit over meerder beeldschermen
    .separate = Beeldschermen hebben afzonderlijke werkbladen

workspaces-overview-thumbnails = Miniatuurweergaven van het werkbladoverzicht
    .show-number = Toon het nummer van de werkbladen
    .show-name = Toon de naam van de werkbladen

workspaces-orientation = Oriëntatie van virtuele werkbladen
    .vertical = Verticaal
    .horizontal = Horizontaal

hot-corner = Slimme hoek
    .top-left-corner = Activeer slimme hoek linksboven voor het werkbladenoverzicht

## Displays

-requires-restart = Vereist een herstart

color = Kleur
    .depth = Kleurdiepte
    .profile = Kleurprofiel
    .sidebar = Kleurprofielen
    .temperature = Kleurtemperatuur

display = Beeldschermen
    .desc = Beeldschermbeheer, omschakeling tussen grafisch kaarten en nachtlichtbeheer
    .arrangement = Schermindeling
    .arrangement-desc = Sleep de schermen om ze te herschikken.
    .enable = Beeldscherm activeren
    .external = { $size } { $output } Extern beeldscherm
    .laptop = { $size } Laptop beeldscherm
    .options = Beeldschermopties
    .refresh-rate = Refresh rate
    .resolution = Schermresolutie
    .scale = Schaal
    .additional-scale-options = Extra schalingsopties

mirroring = Scherm dupliceren
    .id = { $id } dupliceren
    .dont = Niet dupliceren
    .mirror = { $display } dupliceren
    .project = Naar { $display ->
        [all] alle schermen
        *[other] { $display }
    } projecteren
    .project-count = Naar { $count} { $count ->
        [1] ander scherm
        *[other] andere schermen
    } projecteren

night-light = Nachtlichtbeheer
    .auto = Automatisch (van zonsondergang tot zonsopgang)
    .desc = Blauw licht verminderen met warmere lichtkleuren.

orientation = Oriëntatie
    .standard = Standaard
    .rotate-90 = 90° draaien
    .rotate-180 = 180° draaien
    .rotate-270 = 270° draaien

vrr = Variable refresh rate
    .enabled = Ingeschakeld
    .force = Ingeschakeld houden
    .auto = Automatisch
    .disabled = Uitgeschakeld

scheduling = Tijdsplanning
    .manual = Handmatig plannen

dialog = Dialoog
    .title = Deze beeldscherminstellingen behouden?
    .keep-changes = Wijzigingen behouden
    .change-prompt = Instellingen worden automatisch hersteld in { $time } seconden.
    .revert-settings = Instellingen herstellen

## Sound

sound = Geluid
    .desc = N/A

sound-output = Uitvoer
    .volume = Uitvoervolume
    .device = Uitvoerapparaat
    .level = Uitvoerniveau
    .config = Configuratie
    .balance = Geluidsbalans
    .left = Links
    .right = Rechts

sound-input = Invoer
    .volume = Invoervolume
    .device = Invoerapparaat
    .level = Invoerniveau

sound-alerts = Waarschuwingsgeluid
    .volume = Waarschuwingsvolume
    .sound = Type waarschuwingsgeluid

sound-applications = Toepassingen
    .desc = Volume-instellingen per toepassing

profile = Profiel

## Power

power = Enegrie en batterij
    .desc = Energieverbruik beheren

battery = Batterij
    .minute = { $value } { $value ->
        [one] minuut
        *[other] minuten
    }
    .hour = { $value } uur
    .day = { $value } { $value ->
        [one] dag
        *[other] dagen
    }
    .less-than-minute = In minder dan één minuut
    .and = en
    .remaining-time = De batterij is over { $time } { $action ->
        [full] opgeladen
        *[other] leeg
    }

connected-devices = Verbonden apparaten
  .unknown = Onbekend apparaat

power-mode = Energieverbruik
    .battery = Energiebesparing
    .battery-desc = Verminderd energieverbruik en stille prestaties.
    .balanced = Gebalanceerd
    .balanced-desc = Normale prestaties en batterieverbruik.
    .performance = Hoge Prestatie
    .performance-desc = Hoge prestatie en energieverbruik.
    .no-backend = Backend niet gevonden. Installeer system76-power of power-profiles-daemon.

power-saving = Energiebesparingsopties
    .turn-off-screen-after = Beeldscherm uitschakelen na
    .auto-suspend = Automatische slaapstand
    .auto-suspend-ac = Automatische slaapstand wanneer op het lichtnet aangesloten
    .auto-suspend-battery = Automatische slaapstand wanneer op batterijstroom

## Input

acceleration-desc =  De gevoeligheid wordt automatisch aangepast op basis van de snelheid.

disable-while-typing = Uitschakelen tijdens het typen

input-devices = Invoerapparaten
    .desc = Invoerapparaten

primary-button = Primaire knop
    .desc = Bepaalt de rangschikking van de fysieke toetsen.
    .left = Links
    .right = Rechts

scrolling = Scrollen
    .two-finger = Met twee vingers scrollen
    .edge = Scroll met een vinger langs de rand
    .speed = Scrollsnelheid
    .natural = Natuurlijk scrollen
    .natural-desc = Scroll de inhoud, niet het weergavevenster.

## Input: Keyboard

slow = Langzaam
fast = Snel
short = Kort
long = Lang
keyboard = Toetsenbord
    .desc = Invoermethodes, omschakelen, invoer van speciale tekens en sneltoetsen.

keyboard-sources = Invoermethodes
    .desc = Invoermethodes kunnen worden gewisseld met de sneltoetscombinatie Super + Spatie. Dit kan aangepast worden in de instellingen voor sneltoetsen.
    .move-up = Omhoog
    .move-down = Naar beneden
    .settings = Instellingen
    .view-layout = Toetsenbordindeling weergeven
    .remove = Verwijderen
    .add = Invoermethode toevoegen

keyboard-special-char = Invoer speciale tekens
    .alternate = Toets voor speciale tekens (Alt-Gr)
    .compose = Compose-toets
    .caps = Capslocktoets

keyboard-typing-assist = Typen
    .repeat-rate = Herhalingssnelheid
    .repeat-delay = Vertraging vóór herhaalde toetsaanslag

keyboard-numlock-boot = Numlock
    .boot-state = Status van de numlock bij het opstarten
    .last-boot = Ongewijzigd laten
    .on = Ingeschakeld
    .off = Uitgeschakeld
    .set = Numlock bij het opstarten

added = Toegevoegd
type-to-search = Typ om te zoeken...
show-extended-input-sources = Toon uitgebreide invoermethodes

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Sneltoetsen
    .desc = Sneltoetsen bekijken en aanpassen

add-keybinding = Sneltoets toevoegen
cancel = Annuleren
command = Commando
custom = Aangepast
debug = Fouten opsporen
disabled = Uitgeschakeld
input-source-switch = Tussen toetsenbordindeling wisselen
migrate-workspace-prev = Werkblad naar vorige uitvoer verplaatsen
migrate-workspace-next = Werkblad naar volgende uitvoer verplaatsen
migrate-workspace = Werkblad verplaatsen naar uitvoer { $direction ->
    *[down] hieronder
    [left] links
    [right] rechts
    [up] hierboven
}
navigate = Navigeren
replace = Vervangen
shortcut-name = Sneltoetsnaam
system-controls = Systeemaansturing
terminate = Beëindigen
toggle-stacking = Zet vensterstapelen aan/uit
type-key-combination = Toetsencombinatie invoeren

custom-shortcuts = Aangepaste sneltoetsen
    .add = Sneltoets toevoegen
    .context = Aangepaste sneltoets toevoegen
    .none = Gebruik geen aangepaste sneltoetsen

modified = { $count } aangepast

nav-shortcuts = Navigatie
    .prev-output = Focus op vorige uitvoer
    .next-output = Focus op volgende uitvoer
    .last-workspace = Focus op laatste werkblad
    .prev-workspace = Focus op vorige werkblad
    .next-workspace = Focus op volgende werkblad
    .focus = Vensterfocus naar { $direction ->
        *[down] beneden
        [in] binnen
        [left] links
        [out] buiten
        [right] rechts
        [up] boven
    }
    .output = Naar uitvoer { $direction ->
        *[down] hieronder
        [left] links
        [right] rechts
        [up] hierboven
    } omschakelen
    .workspace = Naar werkblad { $num } omschakelen

manage-windows = Vensterbeheer
    .close = Venster sluiten
    .maximize = Venster maximaliseren
    .fullscreen = Volledig scherm
    .minimize = Venster minimaliseren
    .resize-inwards = Venster verkleinen
    .resize-outwards = Venster vergroten
    .toggle-sticky = Venster vastzetten/losmaken

move-windows = Venster verplaatsen
    .direction = Venster naar { $direction ->
        *[down] beneden
        [left] links
        [right] rechts
        [up] boven
    } verplaatsen
    .display = Venster één beeldscherm naar { $direction ->
        *[down] beneden
        [left] links
        [right] rechts
        [up] boven
    } verplaatsen
    .workspace = Venster één werkblad naar { $direction ->
        *[below] beneden
        [left] links
        [right] rechts
        [above] boven
    } verplaatsen
    .workspace-num = Venster naar werkblad { $num } verplaatsen
    .prev-workspace = Venster naar vorig werkblad verplaatsen
    .next-workspace = Venster naar volgend werkblad verplaatsen
    .last-workspace = Venster naar laatst gebruikte werkblad verplaatsen
    .next-display = Venster naar volgend beeldscherm verplaatsen
    .prev-display = Venster naar vorig beeldscherm verplaatsen
    .send-to-prev-workspace = Venster naar vorig werkblad verplaatsen
    .send-to-next-workspace = Venster naar volgend werkblad verplaatsen

system-shortcut = Systeem
    .app-library = Het appmenu openen
    .brightness-down = Schermhelderheid verlagen
    .brightness-up = Schermhelderheid verhogen
    .home-folder = De gebruikersmap (/home) openen
    .keyboard-brightness-down = Toetsenbordverlichting verlagen
    .keyboard-brightness-up = Toetsenbordverlichting verhogen
    .launcher = Snelstarter openen
    .log-out = Afmelden
    .lock-screen = Scherm vergrendelen
    .mute = Geluid uit
    .mute-mic = Microfoon uitschakelen
    .play-pause = Afspelen/Pauzeren
    .play-next = Volgend nummer
    .play-prev = Vorig nummer
    .poweroff = Computer afsluiten
    .screenshot = Schermafbeelding maken
    .terminal = Open een terminal
    .volume-lower = Uitvoervolume lager
    .volume-raise = Uitvoervolume hoger
    .web-browser = Opent een webbrowser
    .window-switcher = Tussen openstaande vensters wisselen
    .window-switcher-previous = Omgekeerd tussen openstaande vensters wisselen
    .workspace-overview = Het werkbladoverzicht openen

window-tiling = Venstertegelen
    .horizontal = Kies de horizontale oriëntatie
    .vertical = Kies de verticale oriëntatie
    .swap-window = Vensters omwisselen
    .toggle-tiling = Zet venstertegelen aan/uit
    .toggle-stacking = Zet vensterstapelen aan/uit
    .toggle-floating = Zet zwevende vensters aan/uit
    .toggle-orientation = Verander de oriëntatie

replace-shortcut-dialog = Sneltoets overschrijven?
    .desc = { $shortcut } wordt al gebruikt door { $name }. Als u het overschrijft, wordt { $name } uitgeschakeld.

zoom-in = Inzoomen
zoom-out = Uitzoomen

## Input: Mouse

mouse = Muis
    .desc = Muissnelheid, muisversnelling en natuurlijk scrollen.
    .speed = Muissnelheid
    .acceleration = Muisversnelling inschakelen

## Input: Touchpad

click-behavior = Klikeigenschappen
    .click-finger = Secundaire klik met twee vingers en middelklik met drie vingers
    .button-areas = Secundaire klik in de rechterbenedenhoek en middelklik in het midden onderaan

pinch-to-zoom = Zoomen met twee vingers
    .desc = Gebruik twee vingers om in te zoomen, voor apps die zoom ondersteunen.

tap-to-click = Tikken om te klikken
    .desc = Met één vinger tikken voor de primaire klik, met twee vingers voor de secundaire klik en met drie vingers voor de middelklik.

touchpad = Touchpad
    .acceleration = Schakelt muisversnelling voor touchpad in
    .desc = Muissnelheid, klikopties en veeggebaren voor touchpad.
    .speed = Touchpad muissnelheid

## Input: Gestures

gestures = Veeggebaren
    .four-finger-down = Vier vingers naar beneden vegen
    .four-finger-left = Vier vingers naar links vegen
    .four-finger-right = Vier vingers naar rechts vegen
    .four-finger-up = Vier vingers naar boven vegen
    .three-finger-any = Veeg met drie vingers in een willekeurige richting

switch-workspaces = Werkbladen wisselen
    .horizontal = Met vier vingers naar links/recht vegen
    .vertical = Met vier vingers naar boven/beneden vegen

switch-between-windows = Tussen vensters wisselen
open-application-library = appmenu openen
open-workspaces-view = Werkbladoverzicht openen

## Time & Language

time = Tijd- en taalinstellingen
    .desc = N/A

time-date = Datum en tijd
    .desc = Tijdzones, automatisch klokinstellingen en weergave tijdsnotatie.
    .auto = Automatisch bijstellen
    .auto-ntp = Datum en tijd wordt automatisch bijgewerkt zodra de tijdzone is ingesteld.

time-zone = Tijdzone
    .auto = Tijdzone automatisch bepalen
    .auto-info = Vereist locatiebepaling en internettoegang

time-format = Datum- en tijdweergave
    .twenty-four = 24-uurs tijd
    .show-seconds = Laat seconden zien
    .first = De week begint op
    .show-date = Datum weergeven op het paneel boven
    .friday = Vrijdag
    .saturday = Zaterdag
    .sunday = Zondag
    .monday = Maandag

time-region = Taal en regio
    .desc = Regionale datum-, tijd- en getalweergave.

formatting = Opmaak
    .dates = Datum
    .time = Tijd
    .date-and-time = Datum en tijd
    .numbers = Getallen
    .measurement = Maateenheden
    .paper = Papiervoormaat

preferred-languages = Taalvoorkeur
    .desc = De volgorde van de talen bepaalt welke door COSMIC wordt gebruikt. Wijzigingen worden doorgevoerd nadat u zich opnieuw heeft aangemeld.

add-language = Taal toevoegen
    .context = Voeg een taal toe
install-additional-languages = Meer talen installeren
region = Regio

## Applications

applications = Toepassingen

## Applications: Default Applications

default-apps = Standaardtoepassingen
    .desc = Standaard webbrowser, e-mailprogramma, bestandsbeheerder, en andere toepassingen.
    .web-browser = Webbrowser
    .file-manager = Bestandsbeheerder
    .mail-client = E-mailprogramma
    .music = Muziek
    .video = Video
    .photos = Foto's
    .calendar = Agenda
    .terminal = Terminal
    .other-associations = Andere standaardtoepassingen
    .text-editor = Tekstbewerking

## Applications: Startup Applications

startup-apps = Autostart toepassingen
    .desc = Configureer toepassingen die automatisch worden gestart nadat u bent ingelogd.
    .add = Autostart toevoegen
    .user = Autostart toepassingen voor de huidige gebruiker
    .user-description = Deze toepassingen worden automatisch gestart als u met dit gebruikersaccount inlogt.
    .remove-dialog-title = Autostart { $name } verwijderen?
    .remove-dialog-description = Weet u zeker dat u { $name } niet meer automatisch wilt laten starten?
    .search-for-application = Autostart kiezen

## Applications: Legacy Applications

legacy-applications = Compatibiliteit met X11-toepassingen
    .desc = Compatibiliteit voor het schalen van X11-vensters en voor globale sneltoetsen.

legacy-app-global-shortcuts = Globale sneltoetsen in X11-toepassingen
    .desc = Met globale sneltoetsen kunnen toetsaanslagen en muisbewegingen die u maakte in de ene toepassing worden herkend door andere toepassingen voor functies als push-to-talk en push-to-mute. Voor X11-toepassingen is dit standaard uitgeschakeld zodat andere toepassingen uw mogelijk gevoelige toetsaanslagen en muisbewegingen niet kunnen afluisteren.
    .none = Toetsaanslagen niet met X11-toepassingen delen
    .modifiers = Alleen modifiers (Super, Shift, Control, Alt) delen
    .combination = Toetsaanslagen delen als u een modifier (Super, Shift, Control, Alt) ingedrukt houdt
    .all = Alle toetsaanslagen met X11-toepassingen delen
    .mouse = Muisknoppen met X11-toepassingen delen

legacy-app-scaling = Het schalen van vensters die het X11-venstersysteem gebruiken
    .scaled-by-system = Alle X11-vensters schalen
    .system-description = X11-vensters worden onscherp weergegeven op HiDPI-schermen.
    .scaled-applications = Voor toepassingen optimaliseren
    .applications-description = Games en X11-vensters in volledig scherm hebben mogelijk niet de juiste resolutie voor uw scherm.
    .scaled-compatibility = Modus voor maximale compatibiliteit
    .compatibility-description = X11-vensters kunnen wazig lijken op HiDPI-schermen.
    .preferred-display = Uw voorkeursscherm voor games en X11-vensters in volledig scherm
    .no-display = Geen

## System

system = Systeem en gebruikersaccounts

## System: About

about = Over dit apparaat
    .desc = Apparaatnaam, informatie over de hardware, standaardinstellingen van het besturingssysteem.

about-device = Apparaatnaam
    .desc = Deze naam is zichtbaar voor andere bluetooth- en netwerkapparaten.

about-hardware = Hardware
    .model = Hardwaremodel
    .memory = Geheugen
    .processor = Processor
    .graphics = Grafische kaart
    .disk-capacity = Opslagcapaciteit

about-os = Besturingssysteem
    .os = Besturingssysteem
    .os-architecture = Architectuur van het besturingssysteem
    .desktop-environment = Bureaubladomgeving
    .windowing-system = Vensterbeheerder

about-related = Gerelateerde instellingen
    .support = Om hulp vragen

## System: Firmware

firmware = Firmware
    .desc = Firmwaredetails.

## System: Users

users = Gebruikers
    .desc = Authenticatie en gebruikersinstellingen.
    .admin = Systeembeheerder
    .standard = Standaard
    .profile-add = Kies een profielafbeelding

administrator = Systeembeheerder (root)
    .desc = Beheerders kunnen instellingen voor alle gebruikers wijzigen, en andere gebruikers toevoegen of verwijderen.

add-user = Gebruiker toevoegen
change-password = Wachtwoord wijzigen
remove-user = Gebruiker verwijderen
full-name = Volledige naam
invalid-username = Ongeldige gebruikersnaam.
password-mismatch = De wachtwoorden komen niet overeen, probeer het opnieuw.
save = Opslaan
