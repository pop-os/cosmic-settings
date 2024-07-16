app = COSMIC Inställningar

unknown = Okänd

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
    .desc = Växlar till ljustläge vid soluppgång

container-background = Behållarbakgrund
    .desc-detail = Behållarens bakgrundsfärg används för navigeringssidofält, sidolåda, dialogrutor och liknande widgets. Som standard härleds den automatiskt från applikationens eller fönstrets bakgrund.
    .reset = Återställ till automatiskt
    .desc = Primär behållarfärg används för navigeringssidofält, sidolåda, dialogrutor och liknande widgets.

control-tint = Kontrollkomponentens nyans
    .desc = Används för bakgrunder av standardknappar, sökinmatningar, textinmatningar och liknande komponenter.

frosted = Frostat glaseffekt på systemgränssnittet
    .desc = Tillämpar bakgrundsoskärpa på panel, docka, appletar, startprogram och programbibliotek.

text-tint = Gränssnittstextton
    .desc = Färg som används för att härleda gränssnittstextfärger som har tillräcklig kontrast på olika ytor.

style = Stil
    .round = Rund
    .slightly-round = Lite rund
    .square = Fyrkant

# gränssnittstäthet utelämnad för tillfället
window-management = Fönsterhantering
    .active-hint = Storlek på aktivt fönsterhinting
    .gaps = Springor runt kaklade fönster

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

scheduling = Schemaläggning
    .manual = Manuellt schema

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

## Nätverk: Trådbunden anslutning

wired = Trådbunden anslutning
    .desc = Trådbunden anslutning, anslutningsprofiler

## Nätverk: Onlinekonton

online-accounts = Onlinekonton
    .desc = Lägg till konton, IMAP och SMTP, företagsinloggningar

## Tid & språk

time = Tid & språk
    .desc = N/A

time-date = Datum & tid
    .desc = Tidszon, automatiska klockinställningar och viss tidsformatering.
    .auto = Ställ in automatiskt

time-zone = Tidszon
    .auto = Automatisk tidszon
    .auto-info = Kräver platstjänster och internetåtkomst

time-format = Datum & tidsformat
    .twenty-four = 24-timmars tid
    .first = Första dagen på veckan

time-region = Region & språk
    .desc = Formatera datum, tider och siffror baserat på din region

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
    .desc = Autentisering och login, låsskärm.

## Input

acceleration-desc = Justerar automatiskt spårningskänsligheten baserat på hastighet.

primary-button = Primär knapp
    .left = Vänster
    .right = Höger

scrolling = Rullning
    .speed = Rullningshastighet
    .natural = Naturlig rullning
    .natural-desc = Rulla igenom innehållet istället för vyn

## Input: Keyboard

keyboard = Tangentbord
    .desc = Tangentbordsinmatning

keyboard-sources = Inmatningskällor
    .desc = Ingångskällor kan växlas med tangentkombinationen Super+Mellanslag. Detta kan anpassas i inställningarna för kortkommandon.
    .move-up = Flytta upp
    .move-down = Flytta ned
    .settings = Inställningar
    .view-layout = Visa tangentbordslayout
    .remove = Ta bort

keyboard-special-char = Specialteckenssinmatning
    .alternate = Alternativa tecken-knapp
    .compose = Compose knapp

## Input: Keyboard shortcuts

keyboard-shortcuts = Tangentbordsgenvägar
    .desc = Visa och anpassa genvägar

## Input: Mouse

mouse = Mus
    .desc = Mushastighet, acceleration, naturlig rullning.
    .speed = Mushastighet
    .acceleration = Aktivera mus acceleration

## Input: Touchpad

touchpad = Pekplatta
    .desc = Pekplattans hastighet, klickalternativ, gester.
    .speed = Pekplattans hastighet
    .acceleration = Aktivera pekplattans acceleration