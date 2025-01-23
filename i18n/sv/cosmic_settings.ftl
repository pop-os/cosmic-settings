app = COSMIC Inställningar

dbus-connection-error = Misslyckades med att ansluta till DBus
ok = OK
unknown = Okänd

number = { $number }

## Skrivbord

desktop = Skrivbord

## Skrivbord: Utseende

appearance = Utseende
    .desc = Accentfärger och COSMIC tema.

accent-color = Accentfärg
app-background = Applikation- eller fönsterbakgrund
auto = Automatisk
close = Stäng
color-picker = Färgväljare
copied-to-clipboard = Kopierat till urklipp
copy-to-clipboard = Kopiera till urklipp
dark = Mörkt
export = Exportera
hex = Hex
import = Importera
light = Ljust
mode-and-colors = Läge och färger
recent-colors = Senaste färger
reset-to-default = Återställ till standard
rgb = RGB
window-hint-accent = Aktivt fönsterhinting färg
window-hint-accent-toggle =Använd temaaccentfärg som aktiv fönsterhinting

auto-switch = Växla automatiskt från ljust till mörkt läge
    .sunrise = Växlar till ljust läge vid soluppgång
    .sunset = Växlar till mörkt läge vid solnedgång
    .next-sunrise = Växlar till ljust läge vid nästa soluppgång
    .next-sunset = Växlar till mörkt läge vid nästa solnedgång

container-background = Behållarbakgrund
    .desc-detail = Behållarens bakgrundsfärg används för navigeringssidofält, sidolåda, dialogrutor och liknande widgets. Som standard härleds den automatiskt från applikationens eller fönstrets bakgrund.
    .reset = Återställ till automatiskt
    .desc = Primär behållarfärg används för navigeringssidofält, sidolåda, dialogrutor och liknande widgets.

control-tint = Kontrollkomponentens nyans
    .desc = Används för bakgrunder av standardknappar, sökinmatningar, textinmatningar och liknande komponenter.

frosted = Frostat glaseffekt på systemgränssnittet
    .desc = Tillämpar bakgrundsoskärpa på panel, docka, appletar, startprogram och programbibliotek.

enable-export = Använd detta tema för GNOME program.
    .desc = Alla verktygssatser har inte stöd för automatisk växling. Icke COSMIC program kan behöva startas om efter en temaändring.

icon-theme = Ikontema
    .desc = Tillämpar en annan uppsättning ikoner till program.

text-tint = Gränssnittstextton
    .desc = Färg som används för att härleda gränssnittstextfärger som har tillräcklig kontrast på olika ytor.

style = Stil
    .round = Rund
    .slightly-round = Lite rund
    .square = Fyrkant

interface-density = Gränssnittstäthet
    .comfortable = Bekväm
    .compact = Tät
    .spacious = Vidsträckt

## Gränssnittstäthet

window-management = Fönsterhantering
window-management-appearance = Fönsterhantering
    .active-hint = Storlek på aktivt fönsterhinting
    .gaps = Springor runt kaklade fönster

super-key = Super key åtgärd
    .launcher = Öppna programstartare
    .workspaces = Öppna arbetsytor
    .applications = Öppna applikationer
    .disable = Inaktivera




## Testinställningar

experimental-settings = Testinställningar
icons-and-toolkit = Utformning av ikoner och verktyg
interface-font = Standardteckensnitt
monospace-font = Teckensnitt med fast bredd

## Skrivbord: Skärm

-requires-restart = Kräver omstart

color = Färg
    .depth = Färgdjup
    .profile = Färgprofiler
    .sidebar = Färgprofiler
    .temperature = Färgtemperatur
display = Skärmar
    .desc = Hantera skärmar, grafikväxling och nattljus
    .arrangement = Visningsarrangemang
    .arrangement-desc = Dra skärmar för att ordna om dem.
    .enable = Aktivera skärm
    .external = { $size } { $output } Extern skärm
    .laptop = { $size } Laptop skärm
    .options = Skärmalternativ
    .refresh-rate = Uppdateringsfrekvens
    .resolution = Upplösning
    .scale = Skala

mirroring = Spegling
    .id = Spegling { $id }
    .dont = Spegla inte
    .mirror = Spegla { $display }
    .project = Projicera till { $display ->
        [all] alla skärmar
        *[other] { $display }
    }
    .project-count = Projicerar till { $count} andra { $count ->
        [1] skärm
        *[other] skärmar
    }

night-light = Nattljus
    .auto = Automatiskt (solnedgång till soluppgång)
    .desc = Reducera blått ljus med varmare färger.

orientation = Orientering
    .standard = Standard
    .rotate-90 = Rotera 90
    .rotate-180 = Rotera 180
    .rotate-270 = Rotera 270

scheduling = Schemaläggning
    .manual = Manuellt schema
    
dialog = Dialogruta
    .title = Behåll dessa skärminställningar?
    .keep-changes = Behåll ändringar
    .change-prompt = Inställningsändringar återställs automatiskt om { $time } sekunder.
    .revert-settings = Återställ inställningar
    
legacy-applications = X11 fönstersystem applikationsskalning
    .scaled-by-system = Skala alla X11 applikationer
    .system-description = X11-applikationer kommer att se suddiga ut på HiDPI-skärmar.
    .scaled-natively = Rendera X11-applikationer med inbyggd upplösning
    .native-description = X11-applikationer som inte stöder skalning kommer att vara små när HiDPI-skärmar används. Aktivera för att spel ska kunna använda hela skärmen

## Skrivbord: Aviseringar

notifications = Aviseringar
    .desc = Stör ej, aviseringar på låsskärmen och inställningar per applikation.

## Skrivbord: Alternativ

desktop-panel-options = Skrivbord och Panel
    .desc = Supertangentåtgärd, varma hörn, fönsterkontrollalternativ.

desktop-panels-and-applets = Skrivbordspaneler och applets

dock = Docka
    .desc = Panel med fästa applikationer.

hot-corner = Het hörn
    .top-left-corner = Aktivera det övre vänstra hörnet för arbetsytor

top-panel = Övre Panel
    .workspaces = Visa knappen arbetsytor
    .applications = Visa knappen applikationer

window-controls = Fönsterkontroller
    .minimize = Visa minimera knapp
    .maximize = Visa maximera knapp
    
focus-navigation = Fokusnavigering
    .focus-follows-cursor = Fokus följer markören
    .focus-follows-cursor-delay = Fokus följer markörens fördröjning i ms
    .cursor-follows-focus = Markören följer fokus


## Skrivbord: Panel

panel = Panel
    .desc = Översta fältet med skrivbordskontroller och menyer.

add = Lägg till
add-applet = Lägg till Applet
all = Alla
applets = Applets
center-segment = Mittsegment
drop-here = Släpp applets här
end-segment = Slutsegment
large = Stor
no-applets-found = Inga applets hittades...
panel-bottom = Längst ned
panel-left = Vänster
panel-right = Höger
panel-top = Högst upp
search-applets = Sök efter applets...
small = Liten
start-segment = Startsegment

panel-appearance = Utseende
    .match = Matcha skrivbordet
    .light = Ljust
    .dark = Mörkt

panel-behavior-and-position = Beteende och positioner
    .autohide = Automatiskt dölj panel
    .dock-autohide = Automatiskt dölj docka
    .position = Position på skärm
    .display = Visa på skärm

panel-style = Stil
    .anchor-gap = Glapp mellan panel och skärmkanter
    .dock-anchor-gap = Glapp mellan docka och skärmkanter
    .extend = Förläng panelen till skärmkanterna
    .dock-extend = Förläng docka till skärmkanterna
    .appearance = Utseende
    .size = Storlek
    .background-opacity = Bakgrundens opacitet

panel-applets = Konfiguera
    .dock-desc = Konfigurera dockans applets.
    .desc = Konfigurera panelens applets.

panel-missing = Panelkonfiguration saknas
    .desc = Panelkonfigurationsfilen saknas på grund av användning av en anpassad konfiguration eller så är den skadad.
    .fix = Återställ till standard

## Desktop: Bakgrundsbild

wallpaper = Bakgrundsbild
    .change = Byt bild var
    .desc = Bakgrundsbilder, färger, och bildspelsalternativ.
    .fit = Bakgrundsbildspassning
    .folder-dialog = Välj katalog med bakgrundsbilder
    .image-dialog = Välj bakgrundsbild
    .plural = Bakgrundsbilder
    .same = Samma bakgrundsbild på alla skärmar
    .slide = Bildspel

add-color = Lägg till färg
add-image = Lägg till bild
all-displays = Alla skärmar
colors = Färger
dialog-add = Lägg till
fill = Fyll
fit-to-screen = Anpassa till skärm
open-new-folder = Öppna ny katalog
recent-folders = Senaste kataloger

x-minutes = { $number } minuter
x-hours = { $number ->
    [1] 1 timme
    *[other] { $number } timmar
}

## Skrivbord: Arbetsytor

workspaces = Arbetsytor
    .desc = Ställ in nummer av arbetsytor, beteende och placering.

workspaces-behavior = Beteende för arbetsytor
    .dynamic = Dynamiska arbetsytor
    .dynamic-desc = Tar automatiskt bort tomma arbetsytor.
    .fixed = Fast antal arbetsytor
    .fixed-desc = Lägg till eller ta bort arbetsytor i översikten.

workspaces-multi-behavior = Beteende med flera skärmar
    .span = Arbetsytor spänner skärmar
    .separate = Skärmar har separata arbetsytor

workspaces-overview-thumbnails = Arbetsyta Översikt Miniatyrer
    .show-number = Visa arbetsytans nummer
    .show-name = Visa arbetsytans namn

workspaces-orientation = Arbetsytor Orientering
    .vertical = Vertikal
    .horizontal = Horisontell
    
hot-corner = Heta hörn
    .top-left-corner = Aktivera det övre vänstra heta hörnet för arbetsytor

## Nätverk: Trådbunden anslutning

wired = Trådbunden anslutning
    .adapter = Trådbunden adapter { $id }
    .desc = Trådbunden anslutning, anslutningsprofiler
    .connections = Trådbundna anslutningar
    .devices = Trådbundna enheter
    .remove = Ta bort anslutningsprofil
    
## Nätverk: WiFi

wifi = Wi-Fi
    .adapter = WiFi adapter { $id }
    .forget = Glöm detta nätverket
    
## Nätverk: Wireguard

wireguard-dialog = Lägg till WireGuard enhet
    .description = Välj ett enhetsnamn för WireGuard-konfigurationen.
    
    

## Nätverksanslutningar

add-network = Lägg till nätverk
add-vpn = Lägg till VPN
airplane-on = Flygplansläge aktiverat.
cable-unplugged = Kabel bortkopplad
connect = Anslut
connected = Ansluten
connecting = Ansluter…
disconnect = Koppla från
forget = Glöm
forget-dialog = Glöm detta Wi-Fi nätverk?
    .description = Du måste ange ett lösenord igen för att använda detta Wi-Fi-nätverk i framtiden.
known-networks = Kända nätverk
network-and-wireless = Nätverksanslutningar
no-networks = Inga nätverk har hittats.
no-vpn = Inga VPN-anslutningar tillgängliga.
password = Lösenord
remove = Ta bort
settings = Inställningar
username = Användarnamn
visible-networks = Tillgängliga nätverk

## Nätverksanslutningar: Beskrivningar

connections-and-profiles = { $variant ->
    [wired] Trådbundna
    [wifi] Trådlösa
    [vpn] VPN
    *[other] Okända
} anslutningar och anslutningsprofiler.

## Virtuellt privat nätverk (VPN)

add-network = Lägg till nätverk
add-vpn = Lägg till VPN

auth-dialog = Autentisering krävs
    .vpn-description = Skriv användarnamn och lösenod som krävs av VPN-tjänsten.
    .wifi-description = Skriv lösenord eller krypterad nyckel. Du kan även ansluta genom att trycka på "WPS"-knappen på routern.

remove-connection-dialog = Ta bort ansluten profil?
    .vpn-description = Du behöver i framtiden skriva ditt lösenord igen för att använda den här funktoinen.
    .wired-description = Du behöver i framtiden återskapa den här profilen för att kunna använda den.

vpn = VPN
    .connections = VPN-anslutningar
    .error = Misslyckades med att lägga till en VPN-konfiguration
    .remove = Ta bort ansluten profil
    .select-file = Välj en VPN-konfigurationsfil
    
    vpn-error = VPN fel
    .config = Misslyckades med att lägga till en VPN-konfiguration
    .connect = Misslyckades att ansluta till VPN
    .connection-editor = Anslutningsredigerare misslyckades
    .connection-settings = Misslyckades att hämta inställningar för aktiva anslutningar
    .updating-state = Misslyckades att uppdatera nätverkshanterarens status
    .wireguard-config-path = Ogiltig filsökväg för Wireguard konfiguration
    .wireguard-config-path-desc = Den valda filen måste vara på ett lokalt filsystem.
    .wireguard-device = Misslyckades med att skapa WireGuard enhet
    .with-password = Misslyckades att ställa in VPN { $field ->
        *[username] användarnamn
        [password] lösenord
        [password-flags] lösenordsflaggor
    } med nmcli

## Nätverk: Onlinekonton

online-accounts = Onlinekonton
    .desc = Lägg till konton, IMAP och SMTP, företagsinloggningar

# Bluetooth

confirm = Bekräfta

bluetooth = Bluetooth
    .desc = Hantera Bluetooth enheter
    .status = Detta system är synligt som { $aliases } medan Bluetooth-inställningarna är öppna.
    .connected = Ansluten
    .connecting = Ansluter
    .disconnecting = Kopplar från
    .connect = Anslut
    .disconnect = Koppla från
    .forget = Glöm bort
    .dbus-error = Ett fel har uppstått under interaktion med DBus: { $why }
    .show-device-without-name = Visa enheter utan namn
    
bluetooth-paired = Tidigare anslutna enheter
    .connect = Anslut
    .battery = { $percentage }% batteri
    
bluetooth-confirm-pin = Bekräfta Bluetooth PIN
    .description = Kontrollera att följande PIN-kod stämmer överens med den som visas på { $device }
    
bluetooth-available = Enheter i närheten

bluetooth-adapters = Bluetooth Adapters

## Datum, tid & språk

time = Tid & språk
    .desc = N/A

time-date = Datum & tid
    .desc = Tidszon, automatiska klockinställningar och viss tidsformatering.
    .auto = Ställ in automatiskt
    .auto-ntp = Datum och tid uppdateras automatisk när tidszon är satt.

time-zone = Tidszon
    .auto = Automatisk tidszon
    .auto-info = Kräver platstjänster och internetåtkomst

time-format = Datum & tidsformat
    .twenty-four = 24-timmars tid
    .show-seconds = Visa sekunder
    .first = Första dagen på veckan
    .show-date = Visa datum på övre panel.
    .friday = Fredag
    .saturday = Lördag
    .sunday = Söndag
    .monday = Måndag

time-region = Region & språk
    .desc = Formatera datum, tider och siffror baserat på din region
    
    formatting = Formatering
    .dates = Datum
    .time = Tid
    .date-and-time = Datum & tid
    .numbers = Nummer
    .measurement = Mått
    .paper = Papper

preferred-languages = Föredragna språk
    .desc = Ordningen på språken avgör vilket språk som används för översättningen av skrivbordsmiljön. Ändringar träder i kraft vid nästa inloggning.
    
add-language = Lägg till språk
    .context = Lägg till språk
install-additional-languages = Installera ytterligare språk
region = Region

## Ljud

sound = Ljud
    .desc = N/A

sound-output = Utgång
    .volume = Utgångsvolym
    .device = Utgångsenhet
    .level = Utgångsnivå
    .config = Konfiguration
    .balance = Balans

sound-input = Ingång
    .volume = Ingångsvolym
    .device = Ingångsenhet
    .level = Ingångsnivå

sound-alerts = Larm
    .volume = Larmvolym
    .sound = Larmljud

sound-applications = Applikationer
    .desc = Applikationvolym och inställningar

profile = Profil

## System

system = System & konton

## System: Om

about = Om
    .desc = Enhetsnamn, hårvaruinformation, operativsystemstandarder.

about-device = Enhetsnamn
    .desc = Detta namn visas för andra nätverks eller bluetooth-enheter.

about-hardware = Hårdvara
    .model = Hårdvarumodell
    .memory = Minne
    .processor = Processor
    .graphics = Grafik
    .disk-capacity = Diskkapacitet

about-os = Operativsystem
    .os = Operativsystem
    .os-architecture = Operativsystemarkitektur
    .desktop-environment = Skrivbordsmiljö
    .windowing-system = Fönstersystem

about-related = Relaterade inställningar
    .support = Få support

## System: Fast programvara

firmware = Fast programvara
    .desc = Fastprogramvara detaljer.

## System: Användare

users = Användare
    .desc = Autentisering och användarkonton.
    .admin = Administratör
    .standard = Standard
    .profile-add = Välj profilbild

administrator = Administratör
    .desc = Administratörer kan ändra inställningar för alla användare, lägga till och ta bort andra användare.

add-user = Lägg till användare
remove-user = Ta bort användare
full-name = Fullständigt namn
username = Användarnamn
password = Lösenord

users = Användare
    .desc = Autentisering och login, låsskärm.

## Ström

power = Ström & Batteri
    .desc = Hantera ströminställningar

battery = Batteri
  .minute = { $value } { $value ->
        [one] minut
       *[other] minuter
  }
  .hour = { $value } { $value ->
        [one] timme
       *[other] timmar
  }
  .day = { $value } { $value ->
        [one] dag
       *[other] dagar
  }
  .less-than-minute = Mindre än en minut
  .and = och
  .remaining-time = { $time } tills { $action ->
        [full] fullt
       *[other] urladdat
   }

connected-devices = Kopplade enheter
  .unknown = Okänd enhet

power-mode = Strömalternativ
    .battery = Förläng batteriets livslängd
    .battery-desc = Låg strömförbrukning och tyst prestanda.
    .balanced = Balanserad
    .balanced-desc = Lågmäld prestanda och måttlig strömförbrukning.
    .performance = Hög prestanda
    .performance-desc = Hög prestanda och strömförbrukning.
    .no-backend = Basbearbetning ej funnen. Installera system76-power eller power-profiles-daemon.

power-saving = Energisparalternativ
    .turn-off-screen-after = Stäng av skärmen efter
    .auto-suspend = Automatisk suspendering
    .auto-suspend-ac = Automatisk suspendering vid inkoppling
    .auto-suspend-battery = Automatisk suspendering på batteridrift

## Input

acceleration-desc = Justerar automatiskt spårningskänsligheten baserat på hastighet.

disable-while-typing = Inaktivera medans du skriver

input-devices = Inmatningsenheter
    .desc = Inmatningsenheter

primary-button = Primär knapp
.desc = Ställer in ordningen på fysiska knappar.
    .left = Vänster
    .right = Höger

scrolling = Rullning
.two-finger = Rulla med två fingrar
    .edge = Rulla längs kanten med ett finger
    .speed = Rullningshastighet
    .natural = Naturlig rullning
    .natural-desc = Rulla igenom innehållet istället för vyn

## Input: Tangentbord

slow = Långsam
fast = Snabb
short = Kort
long = Lång
keyboard = Tangentbord
    .desc = Ingångskällor, växling, inmatning av specialtecken, genvägar.

keyboard-sources = Inmatningskällor
    .desc = Ingångskällor kan växlas med tangentkombinationen Super+Mellanslag. Detta kan anpassas i inställningarna för kortkommandon.
    .move-up = Flytta upp
    .move-down = Flytta ned
    .settings = Inställningar
    .view-layout = Visa tangentbordslayout
    .remove = Ta bort
    .add = Lägg till källa
    
keyboard-special-char = Specialkaraktärsinmatning
    .alternate = Knapp för alternativa tecken
    .compose = Compose knapp
    .caps = Caps Lock knapp

keyboard-typing-assist = Tangenttrycksrespons
    .repeat-rate = Upprepningshastighet
    .repeat-delay = Upprepningsfördröjning

added = Tillagd
type-to-search = Skriv för att söka...
show-extended-input-sources = Visa utökade ingångskällor

## Input: Keyboard shortcuts

keyboard-shortcuts = Tangentbordsgenvägar
    .desc = Visa och anpassa genvägar
   
add-keybinding = Lägg till tangentbindning
cancel = Avbryt
command = Kommando
custom = Anpassat
debug = Avlusa
disabled = Inaktiverat
migrate-workspace-prev = Migrera arbetsytan till tidigare utdata
migrate-workspace-next = Migrera arbetsytan till nästa utdata
migrate-workspace = Migrera arbetsytan till utdata { $direction ->
    *[down] ned
    [left] vänster
    [right] höger
    [up] upp
    
navigate = Navigera
replace = Byt ut
shortcut-name = Genvägsnamn
system-controls = Systemkontroller
terminate = Terminera
toggle-stacking = Växla fönsterstapling
type-key-combination = Skriv knappkombination

custom-shortcuts = Anpassade genvägar
    .add = Lägg till genväg
    .context = Lägg till anpassad genväg
    .none = Inga anpassade genvägar
    
modified = { $count } modiferad

nav-shortcuts = Navigation
    .prev-output = Fokusera föregående utdata
    .next-output = Fokusera nästa utdata
    .last-workspace = Fokusera sista arbetsyta
    .prev-workspace = Fokusera föregående arbetsyta
    .next-workspace = Fokusera nästa arbetsyta
    .focus = Fokusera fönster { $direction ->
        *[down] ned
        [in] i
        [left] vänster
        [out] ot
        [right] höger
        [up] upp
    }
    .output = Ändra till utdata { $direction ->
        *[down] ned
        [left] vänster
        [right] höger
        [up] upp
    }
    .workspace = Ändra till arbetsyta { $num }
    
manage-windows = Hantera fönster
    .close = Stäng fönster
    .maximize = Maximera fönster
    .minimize = Minimera fönster
    .resize-inwards = Ändra storlek på fönstret inåt
    .resize-outwards = Ändra storlek på fönstret utåt
    .toggle-sticky = Växla klibbigt fönster
    
move-windows = Flytta fönster
    .direction = Flytta fönster { $direction ->
        *[down] ned
        [left] vänster
        [right] höger
        [up] upp
    }
    .display = Flytta fönster en skärm { $direction ->
        *[down] ned
        [left] vänster
        [right] höger
        [up] upp
    }
    .workspace = Flytta fönster en arbetsyta { $direction ->
        *[below] nedan
        [left] vänster
        [right] höger
        [above] över
    }
    .workspace-num = Flytta fönster till arbetsyta { $num }
    .prev-workspace = Flytta fönster till föregående arbetsyta
    .next-workspace = Flytta fönster till nästa arbetsyta
    .last-workspace = Flytta fönster till sista arbetsyta
    .next-display = Flytta fönster till nästa skärm
    .prev-display = Flytta fönster till föregående skärm
    .send-to-prev-workspace = Flytta fönster till föregående arbetsyta
    .send-to-next-workspace = Flytta fönster till nästa arbetsyta
    
system-shortcut = System
    .app-library = Öppna app bibliotek
    .brightness-down = Minska skärmens ljusstyrka
    .brightness-up = Öka skärmens ljusstyrka
    .home-folder = Öppna hemkatalog
    .keyboard-brightness-down = Minska tangentbordets ljusstyrka
    .keyboard-brightness-up = Öka tangentbordets ljusstyrka
    .launcher = Öppna programstartaren
    .lock-screen = Lås skärmen
    .mute = Dämpa ljudutgången
    .mute-mic = Dämpa mikrofoningång
    .play-pause = Spela/Pausa
    .play-next = Nästa spår
    .play-prev = Föregående spår
    .screenshot = Ta en skärmdump
    .terminal = Öppna en terminal
    .volume-lower = Sänk ljudvolymen
    .volume-raise = Öka ljudvolymen
    .web-browser = Öppna en webbläsare
    .window-switcher = Växla mellan öppna fönster
    .workspace-overview = Öppna översikten över arbetsytor
    
window-tiling = Kakelsättning av fönster
    .horizontal = Ställ in horisontell orientering
    .vertical = Ställ in vertikal orientering
    .swap-window = Växla fönster
    .toggle-tiling = Växla kakelsättning av fönster
    .toggle-stacking = Växla fönsterstapling
    .toggle-floating = Växla flytande fönster
    .toggle-orientation = Växla orientering
    
replace-shortcut-dialog = Byt ut genväg?
    .desc = { $shortcut } används av { $name }. Om du byter ut den så kommer, { $name } att inaktiveras.

## Input: Mus

mouse = Mus
    .desc = Mushastighet, acceleration, naturlig rullning.
    .speed = Mushastighet
    .acceleration = Aktivera mus acceleration

## Input: Touchpad

click-behavior = Klickbeteende
    .click-finger = Sekundärklicka med två fingrar och mellanklicka med tre fingrar
    .button-areas = Sekundärklicka i det nedre högra hörnet och mittenklicka längst ner i mitten
    
 pinch-to-zoom = Nyp för att zooma
    .desc = Använd två fingrar för att zooma in på innehåll, för applikationer som stöder zoom.
    
tap-to-click = Tryck för att klicka
    .desc = Aktiverar tryck med ett finger för primärt klick, tryck med två fingrar för sekundärt klick och tryck med tre fingrar för mittenklick.

touchpad = Pekplatta
    .desc = Pekplattans hastighet, klickalternativ, gester.
    .speed = Pekplattans hastighet
    .acceleration = Aktivera pekplattans acceleration

## Input: Gester

gestures = Gester
    .four-finger-down = Fyra fingrar svep ned
    .four-finger-left = Fyra fingrar svep vänster
    .four-finger-right = Fyra fingrar svep höger
    .four-finger-up = Fyra fingrar svep upp
    .three-finger-any = Svep med tre fingrar i valfri riktning

switch-workspaces = Ändra arbetsyta
    .horizontal = Fyra fingrar svep vänster/höger
    .vertical = Fyra fingrar svep upp/ned
    
switch-between-windows = Ändra mellan fönster
open-application-library = Öppna Applikationsbibliotek
open-workspaces-view = Öppna Översikt över arbetsytor

## System: Standardprogram

default-apps = Standardprogram
    .desc = Standard webbläsare, e-postklient, filhanteringsprogram och andra program.
    .web-browser = Webbläsare
    .file-manager = Filhanteringsprogram
    .mail-client = E-postklient
    .music = Musik
    .video = Video
    .photos = Foton
    .calendar = Kalender
    .terminal = Terminal
    .other-associations = Andra associeringar
