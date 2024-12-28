app = COSMIC-instellingen

dbus-connection-error = Kon geen verbinding maken met DBus
ok = Oké
unknown = Onbekend

number = { $number }

## Network & Wireless

connections-and-profiles = { $variant ->
    [wired] Bekabelde verbindingen
    [wifi] Wifi
    [vpn] VPN
    *[other] Onbekende verbindingen
} en verbindingsprofielen.

add-network = Netwerk toevoegen
    .profile = Profiel toevoegen
add-vpn = VPN toevoegen
airplane-on = Vliegtuigmodus is ingeschakeld.
cable-unplugged = Kabel losgekoppeld
connect = Verbinden
connected = Verbonden
connecting = Verbinding maken...
disconnect = Verbinden verbreken
forget = Vergeten
known-networks = Bekende netwerken
network-and-wireless = Netwerk en wifi
no-networks = Er zijn geen netwerken gevonden.
no-vpn = Geen VPN-verbindingen beschikbaar.
password = Wachtwoord
remove = Verwijderen
settings = Instellingen
username = Gebruikersnaam
visible-networks = Zichtbare netwerken

auth-dialog = Authenticatie vereist
    .vpn-description = Voer de gebruikersnaam en het wachtwoord van de VPN-dienst in.
    .wifi-description = Voer het wachtwoord of de encryptiesleutel in. U kunt ook verbinden door op de "WPS"-knop op de router te drukken.

forget-dialog = Wifi-netwerk vergeten?
    .description = U moet opnieuw een wachtwoord invoeren om dit wifi-netwerk in de toekomst te gebruiken.

network-device-state =
    .activated = Verbonden
    .config = Verbinding maken...
    .deactivating = Verbinding verbreken...
    .disconnected = Verbinding is verbroken
    .failed = Verbinding mislukt
    .ip-check = Verbinding wordt gecontroleerd...
    .ip-config =  IP- en routeringsinformatie wordt opgevraagd...
    .need-auth = Vereist authenticatie
    .prepare = Verbinding voorbereiden...
    .secondaries = Wachten op secundaire verbinding...
    .unavailable = Niet beschikbaar
    .unknown = Status onbekend
    .unmanaged = Onbeheerd
    .unplugged = Kabel losgekoppeld

remove-connection-dialog = Verbindingsprofiel verwijderen?
    .vpn-description =  Om dit netwerk in de toekomst te kunnen gebruiken, moet u opnieuw een wachtwoord invoeren.
    .wired-description = U moet dit profiel opnieuw aanmaken om het in de toekomst te kunnen gebruiken.

vpn = VPN
    .connections = VPN-verbindingen
    .error = Kon geen VPN-configuratie aanmaken
    .remove = Verbindingsprofiel verwijderen
    .select-file = Selecteer VPN-configuratiebestand

vpn-error = VPN-fout
    .config = Kon geen VPN-configuratie aanmaken
    .connect = VPN-verbinding mislukt
    .connection-editor =  Verbindingseditor mislukt
    .connection-settings = Het ophalen van instellingen voor actieve verbindingen is mistlukt
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

confirm = Bevestigen

bluetooth = Bluetooth
    .desc = Bluetooth-apparaten bewerken
    .status = Dit systeem is zichtbaar als { $aliases } zolang de Bluetooth-instellingen open staan.
    .connected = Verbonden
    .connecting = Verbinding maken...
    .disconnecting = Verbinding verbreken...
    .connect = Verbinden
    .disconnect = Verbinding verbreken
    .forget = Vergeten
    .dbus-error = Er is een fout opgetreden tijdens de interactie met DBus: { $why }

bluetooth-paired = Eerder verbonden apparaten
    .connect = Verbinden
    .battery = { $percentage }% batterij

bluetooth-confirm-pin = Bluetooth-pincode bevestigen
    .description = Controleer of de volgende pincode overeenkomt met de pincode die op { $device } wordt weergegeven

bluetooth-available = Nabije bluetooth-apparaten

bluetooth-adapters = Bluetooth-adapters

## Desktop

desktop = Bureaublad

## Desktop: Wallpaper

wallpaper = Schermachtergrond
    .change = Wijzig alle afbeeldingen
    .desc = Opties voor schermachtergrond, kleuren en diavoorstellingen
    .fit = Schermachtergrond aanpassen
    .folder-dialog = Selecteer map met achtergronden
    .image-dialog = Selecteer schermachtergrond
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

x-minutes = { $number } minuten
x-hours = { $number } uur

never = Nooit

## Desktop: Appearance

appearance = Uiterlijk
    .desc = Kleuren en thema's.

accent-color = Accentkleur
app-background = Achtergronden van applicaties of vensters
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

<#-- not sure about these translations, don't know what is meant with Container color -->
container-background = Containerachergrondskleur 
    .desc-detail = De containerachtergrondskleur wordt gebruikt voor de navigatiebalk, het zijpaneel, dialoogvensters en soortgelijke widgets. Standaard wordt deze automatisch afgeleid van de achtergrond van de toepassing of het venster.
    .reset = Naar automatisch terugzetten
    .desc = De primaire containerkleur wordt gebruikt voor de navigatiezijbalk, zijlade, dialoogvensters en soortgelijke widgets.

control-tint = Tinting van controlecomponenten
    .desc = Wordt gebruikt voor achtergronden van standaardknoppen, zoekingangen, tekstingangen en soortgelijke onderdelen.
frosted =  Matglaseffect op de systeeminterface
    .desc = Past achtergrondvervaging toe op het paneel, de dock, applets, de snelstarter en het startmenu

enable-export = Pas dit thema toe op GNOME-apps.
    .desc = Niet alle toolkit-omgevingen ondersteunen automatische wisseling. Niet-COSMIC-apps moeten mogelijk opnieuw worden opgestart na een themawijziging.

icon-theme = Icoonthema
    .desc = Past een andere set pictogrammen toe op applicaties.

text-tint = Interface tekstkleur
    .desc = Kleur die wordt gebruikt om interfacetekstkleuren te bepalen, zodat er voldoende contrast is op verschillende oppervlakken.

style = Stijl
    .round = Rond
    .slightly-round = Licht afgerond
    .square = Rechthoekig

interface-density = Interface-dichtheid
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

notifications = Meldingen
    .desc = Niet storen, meldingen op het vergrendelscherm en instellingen per applicatie.


## Desktop: Panel

panel = Paneel
    .desc = Balk bovenin met bureaublad-applets en menu's.

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
    .match = Aanpassen aan bureaublad
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
    .desc = Paneel met vastgezetten applicaties vanuit het startmenu en andere applets.

## Desktop: Window management

window-management = Vensterbeheer
    .desc = Actie van de supertoets, vensterbeheeropties en aanvullende opties voor het tegelen van vensters.

super-key = Actie van de supertoets
    .launcher = Snelstarter openen
    .workspaces = Werkbladoverzicht openen
    .applications = Startmenu openen
    .disable = Supertoets uitschakelen

window-controls = Vensterbeheer
    .maximize = Knop 'maximaliseren' tonen
    .minimize = Knop 'minimaliseren' tonen
    .active-window-hint = Gebruik visuele hint voor het actieve venster

focus-navigation = Focusbesturing
    .focus-follows-cursor = Focus volgt de cursor
    .focus-follows-cursor-delay = Vertraging voor focus volgt de cursor in ms
    .cursor-follows-focus = De cursor volgt de focus

## Desktop: Workspaces

workspaces = Virtuele werkbladen
    .desc = Aantal, werking en positie van virtuele werkbladen instellen

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

workspaces-orientation = Oriëntatie van de virtuele werkbladen
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
    .laptop = { $size } Laptop-beeldscherm
    .options = Beeldschermopties
    .refresh-rate = Refresh rate
    .resolution = Schermresolutie
    .scale = Schaal

mirroring = Scherm dubliceren
    .id = Dubliceren { $id }
    .dont = Don't mirror
    .mirror = { $display } dubliceren
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
    .force = Geforceerd ingeschakeld houden
    .auto = Automatisch
    .disabled = Uitgeschakeld

scheduling = Tijdsplanning
    .manual = Handmatig plannen

dialog = Dialoog
    .title = Deze beeldscherminstellingen behouden?
    .keep-changes = Wijzigingen behouden
    .change-prompt = Instellingen worden automatisch hersteld in { $time } seconden.
    .revert-settings = Instellingen herstellen

legacy-applications = Schaling van applicaties die het X11-venstersysteem gebruiken
    .scaled-by-system = Schaal alle X11-applicaties
    .system-description = X11-applicaties worden onscherp weergegeven op HiDPI-schermen.
    .scaled-natively = X11-applicaties in native resolutie weergeven
    .native-description = X11-applicaties die geen schaling ondersteunen worden verkleint op HiDPI-schermen weergegeven. Zet deze optie aan zodat games de volledige schermresolutie kunnen gebruiken.


## Sound

sound = Geluid
    .desc = N/A

sound-output = Uitvoer
    .volume = Uitvoervolume
    .device = Uitvoerapparaat
    .level = Uitvoerniveau
    .config = Configuratie
    .balance = Geluidsbalans

sound-input = Invoer
    .volume = Invoervolume
    .device = Invoerapparaat
    .level = Invoerniveau

sound-alerts = Waarschuwingsgeluid
    .volume = Waarschuwingsvolume
    .sound = Type waarschuwingsgeluid

sound-applications = Applicaties
    .desc = Applicatievolume en -instellingen

profile = Profiel

## Power

power = Energie- en batterijbeheer
    .desc = N/A

battery = Batterij
  .minute = { $value } { $value ->
        [one] minuut
       *[other] minuten
  }
  .hour = { $value } uur
  }
  .day = { $value } { $value ->
        [one] dag
       *[other] dagen
  }
  .less-than-minute = In minder dan één minuut
  .and = en
  .remaining-time = { $time } { $action ->
        [full] opgeladen
       *[other] leeg
   }

connected-devices = Verbonden apparaten
  .unknown = Onbekend apparaat

power-mode = Energieverbruik
    .battery = Energiebesparing
    .battery-desc = Verminderd stroomverbruik en geruisloze prestaties.
    .balanced = Gebalanceerd
    .balanced-desc = Gemiddeld stroomverbruik en stille prestaties.
    .performance = Hoge Prestatie
    .performance-desc = Hoge prestatie en batterijverbruik.
    .no-backend = Backend niet gevonden. Installeer system76-power of power-profiles-daemon.

power-saving = Enegriebesparingsopties
    .turn-off-screen-after = Beeldscherm uitschakelen na
    .auto-suspend = Automatische slaapstand
    .auto-suspend-ac = Automatische slaapstand wanneer op het lichtnet aangesloten
    .auto-suspend-battery = Automatische slaapstand wanneer op batterijstroom

## Input

acceleration-desc = De trackinggevoeligheid wordt automatisch aangepast op basis van de snelheid.

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
    .desc = Invoermethodes kunnen worden gewisseld met de sneltoetscombinatie Super+Spatie. Dit kan aangepast worden in de instellingen voor sneltoetsen.
    .move-up = Omhoog
    .move-down = Naar beneden
    .settings = Instellingen
    .view-layout = Toetsenbordindeling weergeven
    .remove = Verwijderen
    .add = Invoermethode toevoegen

keyboard-special-char = Invoer speciale tekens
    .alternate = Toets voor speciale tekens
    .compose = Compose-toets (voor speciale tekens)
    .caps = Caps Lock-toets

keyboard-typing-assist = Typen
    .repeat-rate = Snelheid van herhalen
    .repeat-delay = Herhalingsvertraging

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
toggle-stacking = Zet vensterstapelen (stacking) aan/uit
type-key-combination = Voer toetsencombinatie in

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

<#-- Realy confusing English sentence to translate: Focus window down / in / left ... What does it mean!? -->
    .focus = Focus op { $direction ->
        *[down] onderste
        [in] binnenste
        [left] linker
        [out] buitenste
        [right] rechter
        [up] bovenste
    } venster

    .output = Omschakelen naar uitvoer { $direction ->
        *[down] hieronder
        [left] links
        [right] rechts
        [up] hierboven
    }
    .workspace = Naar werkblad { $num } omschakelen

manage-windows = Manage windows
    .close = Venster sluiten
    .maximize = Venster maximaliseren
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
<#-- last workspace: last used workspace or the last in a row? -->
    .last-workspace = Venster naar laatst gebruikte werkblad verplaatsen
    .next-display = Venster naar volgend beeldscherm verplaatsen
    .prev-display = Venster naar vorig beeldscherm verplaatsen
    .send-to-prev-workspace = Venster naar vorig werkblad verplaatsen
    .send-to-next-workspace = Venster naar volgend werkblad verplaatsen

system-shortcut = Systeem
    .app-library = Het startmenu openen
    .brightness-down = Schermhelderheid dimmen
    .brightness-up = Schermhelderheid veller
    .home-folder = De gebruikersmap (/home) openen
    .keyboard-brightness-down = Toetsenbordverlichting dimmen
    .keyboard-brightness-up = Toetsenbordverlichting veller
    .launcher = Snelstarter openen
    .lock-screen = Scherm vergrendelen
    .mute = Geluid uit
    .mute-mic = Microfoon uitschakelen
    .play-pause = Afspelen/Pauzeren
    .play-next = Volgend nummer
    .play-prev = Vorig nummer
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
    .toggle-stacking = Zet vensterstapelen (stacking) aan/uit
    .toggle-floating = Zet zwevende vensters aan/uit
    .toggle-orientation = Verander de oriëntatie

replace-shortcut-dialog = Sneltoets overschrijven?
    .desc = { $shortcut } wordt al gebruikt door { $name }. Als u het overschrijft, wordt { $name } uitgeschakeld.

## Input: Mouse

mouse = Muis
    .desc = Muissnelheid, muisversnelling en natuurlijk scrollen
    .speed = Muissnelheid
    .acceleration = Muisversnelling inschakelen

## Input: Touchpad

click-behavior = Klik-eigenschappen
    .click-finger = Secundaire klik met twee vingers en middelklik met drie vingers
    .button-areas = Secundaire klik in de rechterbenedenhoek en middelklik in het midden onderaan

pinch-to-zoom = Twee-vinger-zoom
    .desc = Gebruik twee vingers om in te zoomen, voor apps die zoom ondersteunen.

tap-to-click = Tikken om te klikken
    .desc = Met één vinger tikken voor de primaire klik, met twee vingers voor de secundaire klik en met drie vingers voor de middelklik.

touchpad = Touchpad
    .acceleration = Schakelt touchpad-versnelling in
    .desc = Touchpad-snelheid, klik-opties, touchpad-gebaren.
    .speed = Touchpad-snelheid

## Input: Gestures

gestures = Touchpad-gebaren
    .four-finger-down = Vier vingers naar beneden vegen
    .four-finger-left = Vier vingers naar links vegen
    .four-finger-right = Vier vingers naar rechts vegen
    .four-finger-up = Vier vingers naar boven vegen
    .three-finger-any = Veeg met drie vingers in een willekeurige richting

switch-workspaces = Werkbladen wisselen
    .horizontal = Met vier vingers naar links/recht vegen
    .vertical = Met vier vingers naar boven/beneden vegen 

switch-between-windows = Tussen vensters wisselen
open-application-library = Startmenu openen
open-workspaces-view = Werkbladoverzicht openen

## Time & Language

time = Tijd- en taalinstellingen
    .desc = N/A

time-date = Tijd en datum
    .desc = Tijdzones, automatisch klokinstellingen en weergave tijdsnotatie.
    .auto = Automatisch bijstellen
    .auto-ntp = Tijd en datum wordt automatisch bijgewerkt zodra de tijdzone is ingesteld.

time-zone = Tijdzone
    .auto = Tijdzone automatisch bepalen
    .auto-info = Vereist locatiebepaling en internettoegang

time-format = Tijd- en datumweergave
    .twenty-four = 24-uurs tijd
    .show-seconds = Laat secondes zien
    .first = De week begint op
    .show-date = Datum weergeven op het paneel boven
    .friday = Vrijdag
    .saturday = Zaterdag
    .sunday = Zondag
    .monday = Maandag

time-region = Taal en regio
    .desc = Regionale tijd-, datum- en getalweergave.

formatting = Opmaak
    .dates = Datum
    .time = Tijd
    .date-and-time = Tijd en datum
    .numbers = Getallen
    .measurement = Maateenheden
    .paper = Papiervoormaat

preferred-languages = Taalvoorkeur
    .desc = The order of languages determines which language is used for the translation of the desktop. Changes take effect on next login.

add-language = Taal toevoegen
    .context = Voeg een taal toe
install-additional-languages = Meer talen installeren
region = Regio

## System

system = Systeem en accounts

## System: About

about = Over dit apparaat
    .desc = Apparaatnaam, informatie over de hardware, standaardinstellingen van het besturingssysteem.

about-device = Apparaatnaam
    .desc = Deze naam is zichtbaar voor andere bluetooth- en netwerkapparaten.

about-hardware = Hardware
    .model = Hardwaremodel
    .memory = Geheugen
    .processor = Processor
    .graphics = Grafische prestaties
    .disk-capacity = Opslagcapaciteit

about-os = Besturingssysteem
    .os = Besturingssysteem
    .os-architecture = Architectuur van het besturingssysteem
    .desktop-environment = Bureaubladomgeving
    .windowing-system = Vensterbeheerder

about-related = Gerelateerde instellingen
    .support = Hulp aanvragen

## System: Firmware

firmware = Firmware
    .desc = Firmwaredetails.

## System: Users

users = Gebruikers
    .desc = Authenticatie en gebruikersinstellingen
    .admin = Systeembeheerder
    .standard = Standaard
    .profile-add = Kies een profielafbeelding

administrator = Systeembeheerder (root)
    .desc = Beheerders kunnen instellingen voor alle gebruikers wijzigen, en andere gebruikers toevoegen of verwijderen.

add-user = Gebruiker toevoegen
remove-user = Gebruiker verwijderen
full-name = Volledige naam
username = Gebruikersnaam
password = Wachtwoord

## System: Default Applications

default-apps = Standaardapplicaties
    .desc = Standaard webbrowser, e-mailprogramma, bestandsbeheerder, en andere applicaties.
    .web-browser = Webbrowser
    .file-manager = Bestandsbeheerder
    .mail-client = e-mailprogramma
    .music = Muziek
    .video = Video
    .photos = Foto's
    .calendar = Agenda
    .terminal = Terminal
    .other-associations = Andere koppelingen
