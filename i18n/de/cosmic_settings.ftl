app = COSMIC Einstellungen

unknown = Unbekannt

number = { $number }

## Netzwerk & Kabellos

connections-and-profiles = { $variant ->
[wired] Kabelgebundene Verbindungen
[wifi] Wi-Fi-Verbindungen
[vpn] VPN-Verbindungen
*[other] Unbekannte Verbindungen
} und Verbindungsprofile.

add-network = Netzwerk hinzufügen
    .profile = Profil hinzufügen
add-vpn = VPN hinzufügen
airplane-on = Flugzeugmodus ist eingeschaltet.
cable-unplugged = Kabel ausgesteckt
connect = Verbinden
connected = Verbunden
connecting = Wird verbunden…
disconnect = Trennen
forget = Vergessen
known-networks = Bekannte Netzwerke
network-and-wireless = Netzwerk & Kabellos
no-networks = Es wurden keine Netzwerke gefunden.
no-vpn = Keine VPN-Verbindungen verfügbar.
password = Passwort
remove = Entfernen
settings = Einstellungen
username = Benutzername
visible-networks = Sichtbare Netzwerke

auth-dialog = Authentifizierung erforderlich
    .vpn-description = Gib den vom VPN-Dienst geforderten Benutzernamen und das Passwort ein.
    .wifi-description = Gib das Passwort oder den Verschlüsselungscode ein. Du kannst die Verbindung auch durch Drücken der „WPS“-Taste am Router herstellen.

forget-dialog = Dieses Wi-Fi-Netzwerk vergessen?
    .description = Du musst erneut ein Passwort eingeben, um dieses Wi-Fi-Netzwerk in der Zukunft zu verwenden.

network-device-state =
    .activated = Verbunden
    .config = Wird verbunden
    .deactivating = Wird getrennt
    .disconnected = Getrennt
    .failed = Verbindung fehlgeschlagen
    .ip-check = Verbindung wird überprüft
    .ip-config = IP- und Routing-Informationen werden angefordert
    .need-auth = Erfordert Authentifizierung
    .prepare = Verbinden wird vorbereitet
    .secondaries = Warten auf sekundäre Verbindung
    .unavailable = Nicht verfügbar
    .unknown = Unbekannter Status
    .unmanaged = Unverwaltet
    .unplugged = Kabel ausgesteckt

remove-connection-dialog = Verbindungsprofil entfernen?
    .vpn-description = Um dieses Netzwerk in der Zukunft nutzen zu können, musst du erneut ein Passwort eingeben.
    .wired-description = Du musst dieses Profil neu erstellen, um es in Zukunft verwenden zu können.

vpn = VPN
    .connections = VPN-Verbindungen
    .remove = Verbindungsprofil entfernen
    .select-file = VPN-Konfigurationsdatei auswählen

wired = Kabelgebunden
    .adapter = Kabelgebundener Adapter { $id }
    .connections = Kabelgebundene Verbindungen
    .devices = Kabelgebundene Geräte
    .remove = Verbindungsprofil entfernen

wifi = Wi-Fi
    .adapter = Wi-Fi-Adapter { $id }
    .forget = Dieses Netzwerk vergessen

## Vernetzung: Online-Konten

online-accounts = Online-Konten
    .desc = Hinzufügen von Konten, IMAP und SMTP, Unternehmensanmeldungen

# Bluetooth

bluetooth = Bluetooth
    .desc = Bluetooth-Geräte verwalten
    .status = Dieses System ist als { $aliases } sichtbar, während die Bluetooth-Einstellungen geöffnet sind.
    .connected = Verbunden
    .connecting = Wird verbunden
    .disconnecting = Wird getrennt
    .connect = Verbinden
    .disconnect = Trennen
    .forget = Vergessen
    .dbus-error = Bei der Interaktion mit DBus ist ein Fehler aufgetreten: { $why }
    .show-device-without-name = Geräte ohne Namen anzeigen

bluetooth-paired = Zuvor verbundene Geräte
    .connect = Verbinden
    .battery = { $percentage } % Akku

bluetooth-available = Geräte in der Nähe

bluetooth-adapters = Bluetooth-Adapter

## Desktop

desktop = Desktop

## Desktop: Hintergrundbild

wallpaper = Hintergrundbild
    .change = Bild ändern alle
    .desc = Hintergrundbilder, Farben und Diashow-Optionen.
    .fit = Hintergrundbild anpassen
    .folder-dialog = Ordner für Hintergrundbilder auswählen
    .image-dialog = Hintergrundbild auswählen
    .plural = Hintergrundbilder
    .same = Gleiches Hintergrundbild auf allen Bildschirmen
    .slide = Diashow

add-color = Farbe hinzufügen
add-image = Bild hinzufügen
all-displays = Alle Bildschirme
colors = Farben
dialog-add = Hinzufügen
fill = Füllen
fit-to-screen = An Bildschirm anpassen
open-new-folder = Neuen Ordner öffnen
recent-folders = Letzte Ordner

x-minutes = { $number } Minuten
x-hours = { $number ->
    [1] 1 Stunde
    *[other] { $number } Stunden
}

## Desktop: Aussehen

appearance = Aussehen
    .desc = Akzentfarben und Themen.

accent-color = Akzentfarbe
app-background = Anwendungs- oder Fensterhintergrund
auto = Automatisch
close = Schließen
color-picker = Farbwähler
copied-to-clipboard = In Zwischenablage kopiert
copy-to-clipboard = In Zwischenablage kopieren
dark = Dunkel
export = Exportieren
hex = Hex
import = Importieren
light = Hell
mode-and-colors = Modus und Farben
recent-colors = Letzte Farben
reset-to-default = Auf Standardwerte zurücksetzen
rgb = RGB
window-hint-accent = Hinweisfarbe für aktives Fenster
window-hint-accent-toggle = Akzentfarbe des Themas als Hinweisfarbe für aktives Fenster verwenden

auto-switch = Automatischer Wechsel zwischen hellem und dunklem Modus
    .sunrise = Wechselt bei Sonnenaufgang in den hellen Modus
    .sunset = Wechselt bei Sonnenuntergang in den dunklen Modus
    .next-sunrise = Wechselt beim nächsten Sonnenaufgang in den hellen Modus
    .next-sunset = Wechselt beim nächsten Sonnenuntergang in den dunklen Modus

container-background = Container-Hintergrund
    .desc-detail = Die Hintergrundfarbe des Containers wird für die Navigationsseitenleiste, die Seitenschublade, Dialoge und ähnliche Widgets verwendet. Standardmäßig wird sie automatisch vom Anwendungs- oder Fensterhintergrund abgeleitet.
    .reset = Auf Automatisch zurücksetzen
    .desc = Die Farbe des primären Containers wird für die Navigationsseitenleiste, die Seitenschublade, Dialoge und ähnliche Widgets verwendet.

control-tint = Komponententönung steuern
    .desc = Wird für Hintergründe von Standardschaltflächen, Sucheingaben, Texteingaben und ähnlichen Komponenten verwendet.

frosted = Milchglaseffekt bei der Systemoberfläche
    .desc = Wendet eine Hintergrundunschärfe auf Panel, Dock, Applets, Starter und Anwendungsbibliothek an.

experimental-settings = Experimentelle Einstellungen

enable-export = Dieses Thema auf GNOME-Apps anwenden.
    .desc = Nicht alle Toolkits unterstützen den automatischen Wechsel. Nicht-COSMIC-Apps müssen nach einem Themenwechsel möglicherweise neu gestartet werden.

icon-theme = Symbolthema
    .desc = Wendet einen anderen Satz von Symbolen auf Anwendungen an.

text-tint = Texttönung der Oberfläche
    .desc = Farbe zur Ableitung von Oberflächentextfarben, die auf verschiedenen Oberflächen einen ausreichenden Kontrast aufweisen.

style = Stil
    .round = Rund
    .slightly-round = Leicht rund
    .square = Quadratisch

interface-density = Dichte der Benutzeroberfläche
    .comfortable = Komfortabel
    .compact = Kompakt
    .spacious = Geräumig

window-management-appearance = Fensterverwaltung
    .active-hint = Größe des Hinweises für aktives Fenster
    .gaps = Lücken um gekachelte Fenster

## Desktop: Benachrichtigungen

notifications = Benachrichtigungen
    .desc = Nicht stören, Sperrbildschirm-Benachrichtigungen und Einstellungen pro Anwendung.

## Desktop: Panel

panel = Panel
    .desc = Obere Leiste mit Desktop-Steuerelementen und Menüs.

add = Hinzufügen
add-applet = Applet hinzufügen
all = Alle
applets = Applets
center-segment = Mittelsegment
drop-here = Applets hier ablegen
end-segment = Endsegment
large = Groß
no-applets-found = Keine Applets gefunden...
panel-bottom = Unten
panel-left = Links
panel-right = Rechts
panel-top = Oben
search-applets = Applets suchen...
small = Klein
start-segment = Anfangssegment

panel-appearance = Aussehen
    .match = An Desktop anpassen
    .light = Hell
    .dark = Dunkel

panel-behavior-and-position = Verhalten und Positionen
    .autohide = Panel automatisch ausblenden
    .dock-autohide = Dock automatisch ausblenden
    .position = Position auf Bildschirm
    .display = Anzeigen auf Bildschirm

panel-style = Stil
    .anchor-gap = Lücke zwischen Panel und Bildschirmrändern
    .dock-anchor-gap = Lücke zwischen Dock und Bildschirmrändern
    .extend = Panel bis zu den Bildschirmrändern ausdehnen
    .dock-extend = Dock bis zu den Bildschirmrändern ausdehnen
    .appearance = Aussehen
    .size = Größe
    .background-opacity = Deckkraft des Hintergrunds

panel-applets = Konfiguration
    .dock-desc = Dock-Applets konfigurieren.
    .desc = Panel-Applets konfigurieren.

panel-missing = Panel-Konfiguration fehlt
    .desc = Die Konfigurationsdatei des Panels fehlt aufgrund der Verwendung einer benutzerdefinierten Konfiguration oder sie ist beschädigt.
    .fix = Auf Standardwerte zurücksetzen

## Desktop: Dock

dock = Dock
    .desc = Panel mit angehefteten Anwendungen in der App-Ablage und anderen Applets.

## Desktop: Fensterverwaltung

window-management = Fensterverwaltung
    .desc = Super-Tasten-Aktion, Optionen für die Fenstersteuerung und zusätzliche Optionen für die Fensterkachelung.

super-key = Super-Taste
    .launcher = Starter öffnen
    .workspaces = Arbeitsflächen öffnen
    .applications = Anwendungen öffnen
    .disable = Deaktivieren

window-controls = Fenstersteuerung
    .minimize = Minimieren-Schaltfläche anzeigen
    .maximize = Maximieren-Schaltfläche anzeigen

focus-navigation = Fokus-Navigation
    .focus-follows-cursor = Fokus folgt dem Cursor
    .focus-follows-cursor-delay = Verzögerung für Fokus folgt dem Cursor in ms
    .cursor-follows-focus = Cursor folgt dem Fokus

## Desktop: Arbeitsflächen

workspaces = Arbeitsflächen
    .desc = Anzahl, Verhalten und Platzierung der Arbeitsflächen festlegen.

workspaces-behavior = Verhaltern der Arbeitsfläche
    .dynamic = Dynamische Arbeitsflächen
    .dynamic-desc = Entfernt automatisch leere Arbeitsflächen.
    .fixed = Feste Anzahl an Arbeitsflächen
    .fixed-desc = Arbeitsflächen in der Übersicht hinzufügen oder entfernen.

workspaces-multi-behavior = Multi-Monitor-Verhalten
    .span = Arbeitsflächen erstrecken sich über mehrere Bildschirme
    .separate = Bildschirme haben separate Arbeitsflächen

workspaces-overview-thumbnails = Miniaturansichten der Arbeitsflächenübersicht
    .show-number = Nummer der Arbeitsfläche anzeigen
    .show-name = Name der Arbeitsfläche anzeigen

workspaces-orientation = Ausrichtung der Arbeitsflächen
    .vertical = Vertikal
    .horizontal = Horizontal

hot-corner = Aktive Ecken
    .top-left-corner = Aktive Ecke oben links für Arbeitsflächen aktivieren

## Bildschirme

-requires-restart = Erfordert Neustart

color = Farbe
    .depth = Farbtiefe
    .profile = Farbprofil
    .sidebar = Farbprofile
    .temperature = Farbtemperatur

display = Bildschirme
    .desc = Bildschirme, Grafikumschaltung und Nachtlicht verwalten
    .arrangement = Bildschirmanordnung
    .arrangement-desc = Bildschirme ziehen, um sie neu anzuordnen.
    .enable = Bildschirm aktivieren
    .external = { $size } { $output } Externer Bildschirm
    .laptop = { $size } Laptop-Bildschirm
    .options = Anzeigeoptionen
    .refresh-rate = Bildwiederholrate
    .resolution = Auflösung
    .scale = Skalierung

mirroring = Spiegelung
    .id = Spiegelung { $id }
    .dont = Nicht spiegeln
    .mirror = { $display } spiegeln
    .project = Auf { $display ->
        [all] alle Bildschirme
        *[other] { $display }
    } projizieren
    .project-count = Projektion auf { $count} { $count ->
        [1] anderen Bildschirm
        *[other] andere Bildschirme
    }

night-light = Nachtlicht
    .auto = Automatisch (Sonnenuntergang bis Sonnenaufgang)
    .desc = Blaues Licht mittels wärmerer Farben reduzieren.

orientation = Ausrichtung
    .standard = Standard
    .rotate-90 = Um 90 Grad drehen
    .rotate-180 = Um 180 Grad drehen
    .rotate-270 = Um 270 Grad drehen

scheduling = Zeitplanung
    .manual = Manueller Zeitplan

dialog = Dialog
    .title = Diese Bildschirmeinstellungen beibehalten?
    .keep-changes = Änderungen beibehalten
    .change-prompt = Änderungen an den Einstellungen werden in { $time } Sekunden automatisch rückgängig gemacht.
    .revert-settings = Einstellungen rückgängig machen

legacy-applications = Anwendungsskalierung des X11-Fenstersystems
    .scaled-by-system = Alle X11-Anwendungen skalieren
    .system-description = X11-Anwendungen werden auf HiDPI-Bildschirmen unscharf dargestellt.
    .scaled-natively = X11-Anwendungen in nativer Auflösung rendern
    .native-description = X11-Anwendungen, die keine Skalierung unterstützen, werden klein dargestellt, wenn HiDPI-Bildschirme verwendet werden. Für Spiele aktivieren, um die volle Monitorauflösung auszunutzen.

## Klang

sound = Klang
    .desc = N/V

sound-output = Ausgabe
    .volume = Ausgabelautstärke
    .device = Ausgabegerät
    .level = Ausgangspegel
    .config = Konfiguration
    .balance = Balance

sound-input = Eingang
    .volume = Eingangslautstärke
    .device = Eingabegerät
    .level = Eingangspegel

sound-alerts = Alarmsignale
    .volume = Lautstärke von Alarmsignalen.
    .sound = Alarmsignalklang.

sound-applications = Anwendungen
    .desc =  Lautstärken und Einstellungen von Anwendungen

profile = Profil

## Energie

power = Energie & Akku
    .desc = Energieeinstellungen verwalten

battery = Akku
    .minute = { $value } { $value ->
    [one] Minute
    *[other] Minuten
      }
    .hour = { $value } { $value ->
    [one] Stunde
    *[other] Stunden
      }
    .day = { $value } { $value ->
    [one] Tag
    *[other] Tage
      }
    .less-than-minute = Weniger als eine Minute
    .and = und
    .remaining-time = { $time } bis { $action ->
    [full] voll
    *[other] leer
       }

connected-devices = Verbundene Geräte
    .unknown = Unbekannte Geräte

power-mode = Energiemodus
    .battery = Verlängerte Akkulaufzeit
    .battery-desc = Geringerer Stromverbrauch und leise Leistung.
    .balanced = Ausgeglichen
    .balanced-desc = Geräuscharme Leistung und moderater Stromverbrauch.
    .performance = Hohe Leistung
    .performance-desc = Spitzenleistung und höchster Stromverbrauch.
    .no-backend = Backend nicht gefunden. Installiere system76-power oder power-profiles-daemon.

## Eingabe

acceleration-desc = Passt die Tracking-Empfindlichkeit automatisch an die Geschwindigkeit an.

disable-while-typing = Während dem Tippen deaktivieren

input-devices = Eingabegeräte
    .desc = Eingabegeräte

primary-button = Primäre Taste
    .desc = Legt die Reihenfolge der physischen Tasten fest.
    .left = Links
    .right = Rechts

scrolling = Scrollen
    .two-finger = Scrollen mit zwei Fingern
    .edge = Mit einem Finger an der Kante entlang scrollen
    .speed = Scrollgeschwindigkeit
    .natural = Natürliches Scrollen
    .natural-desc = Den Inhalt scrollen, anstatt der Ansicht

## Eingabe: Tastatur

slow = Langsam
fast = Schnell
short = Kurz
long = Lang
keyboard = Tastatur
    .desc = Eingabequellen, Umschaltung, Eingabe von Sonderzeichen, Tastenkombinationen.

keyboard-sources = Eingabequellen
    .desc = Die Eingabequellen können mit der Tastenkombination Super+Leertaste umgeschaltet werden. Dies kann in den Einstellungen für Tastenkombinationen angepasst werden.
    .move-up = Nach oben verschieben
    .move-down = Nach unten verschieben
    .settings = Einstellungen
    .view-layout = Tastaturbelegung anzeigen
    .remove = Entfernen
    .add = Eingabequelle hinzufügen

keyboard-special-char = Eingabe von Sonderzeichen
    .alternate = Taste für alternative Zeichen
    .compose = Compose-Taste
    .caps = Feststelltaste

keyboard-typing-assist = Tippen
    .repeat-rate = Wiederholungsrate
    .repeat-delay = Wiederholungsverzögerung

added = Hinzugefügt
type-to-search = Zum Suchen tippen...
show-extended-input-sources = Erweiterte Eingabequellen anzeigen

## Eingabe: Tastatur: Tastenkombinationen

keyboard-shortcuts = Tastenkombinationen
    .desc = Tastenkombinationen anzeigen und anpassen

add-keybinding = Tastenbelegung hinzufügen
cancel = Abbrechen
command = Befehl
custom = Benutzerdefiniert
debug = Debug
disabled = Deaktiviert
migrate-workspace-prev = Arbeitsfläche zur vorherigen Ausgabe migrieren
migrate-workspace-next = Arbeitsfläche zur nächsten Ausgabe migrieren
migrate-workspace = Arbeitsfläche zur { $direction ->
    *[down] unteren
    [left] linken
    [right] rechten
    [up] oberen
} Ausgabe migrieren
navigate = Navigieren
replace = Ersetzen
shortcut-name = Name der Tastenkombination
system-controls = Systemsteuerung
terminate = Beenden
toggle-stacking = Fensterstapelung umschalten
type-key-combination = Tastenkombination eintippen

custom-shortcuts = Benutzerdefinierte Tastenkombinationen
    .add = Tastenkombination hinzufügen
    .context = Benutzerdefinierte Tastenkombination hinzufügen
    .none = Keine benutzerdefinierten Tastenkombinationen

modified = { $count } geändert

nav-shortcuts = Navigation
    .prev-output = Vorherige Ausgabe fokussieren
    .next-output = Nächste Ausgabe fokussieren
    .last-workspace = Letzte Arbeitsfläche fokussieren
    .prev-workspace = Vorherige Arbeitsfläche fokussieren
    .next-workspace = Nächste Arbeitsfläche fokussieren
    .focus = { $direction ->
        *[down] Unteres
        [in] Inneres
        [left] Linkes
        [out] Äußeres
        [right] Rechtes
        [up] Oberes
    } Fenster fokussieren
    .output = Auf { $direction ->
        *[down] untere
        [left] linke
        [right] rechte
        [up] obere
    } Ausgabe umschalten
    .workspace = Auf Arbeitsfläche { $num } umschalten

manage-windows = Fenster verwalten
    .close = Fenster schließen
    .maximize = Fenster maximieren
    .minimize = Fenster minimieren
    .resize-inwards = Fenstergröße nach innen ändern
    .resize-outwards = Fenstergröße nach außen ändern
    .toggle-sticky = Anhaftendes Fenster umschalten

move-windows = Fenster verschieben
    .direction = Fenster nach { $direction ->
        *[down] unten
        [left] links
        [right] rechts
        [up] oben
    } verschieben
    .display = Fenster um einen Monitor nach { $direction ->
        *[down] unten
        [left] links
        [right] rechts
        [up] oben
    } verschieben
    .workspace = Fenster um eine Arbeitsfläche nach { $direction ->
        *[down] unten
        [left] links
        [right] rechts
        [up] oben
    } verschieben
    .workspace-num = Fenster auf Arbeitsfläche { $num } verschieben
    .prev-workspace = Fenster auf vorherige Arbeitsfläche verschieben
    .next-workspace = Fenster auf nächste Arbeitsfläche verschieben
    .last-workspace = Fenster auf letzte Arbeitsfläche verschieben
    .next-display = Fenster auf nächsten Bildschirm verschieben
    .prev-display = Fenster auf vorherigen Bildschirm verschieben
    .send-to-prev-workspace = Fenster auf vorherige Arbeitsfläche verschieben
    .send-to-next-workspace = Fenster auf nächste Arbeitsfläche verschieben

system-shortcut = System
    .app-library = App-Bibliothek öffnen
    .brightness-down = Bildschirmhelligkeit verringern
    .brightness-up = Bildschirmhelligkeit erhöhen
    .home-folder = Persönlichen Ordner öffnen
    .keyboard-brightness-down = Tastaturhelligkeit verringern
    .keyboard-brightness-up = Tastaturhelligkeit erhöhen
    .launcher = Starter öffnen
    .lock-screen = Bildschirm sperren
    .mute = Audioausgabe stummschalten
    .mute-mic = Mikrofoneingang stummschalten
    .play-pause = Wiedergabe/Pause
    .play-next = Nächster Titel
    .play-prev = Vorheriger Titel
    .screenshot = Bildschirmfoto machen
    .terminal = Terminal öffnen
    .volume-lower = Lautstärke der Audioausgabe verringern
    .volume-raise = Lautstärke der Audioausgabe erhöhen
    .web-browser = Öffnet einen Webbrowser
    .window-switcher = Zwischen geöffneten Fenstern wechseln
    .workspace-overview = Arbeitsflächenübersicht öffnen

window-tiling = Fensterkachelung
    .horizontal = Horizontale Ausrichtung festlegen
    .vertical = Vertikale Ausrichtung festlegen
    .swap-window = Fenster tauschen
    .toggle-tiling = Fensterkachelung umschalten
    .toggle-stacking = Fensterstapelung umschalten
    .toggle-floating = Fensterschweben umschalten
    .toggle-orientation = Ausrichtung umschalten

replace-shortcut-dialog = Tastenkombination ersetzen?
    .desc = { $shortcut } wird von { $name } verwendet. Wenn du sie ersetzt, wird { $name } deaktiviert.

## Eingabe: Maus

mouse = Maus
    .desc = Mausgeschwindigkeit, -beschleunigung, natürliches Scrollen.
    .speed = Mausgeschwindigkeit
    .acceleration = Mausbeschleunigung aktivieren

## Eingabe: Touchpad

click-behavior = Klickverhalten
    .click-finger = Sekundärklick mit zwei Fingern und Mittelklick mit drei Fingern
    .button-areas = Sekundärklick in der rechten unteren Ecke und Mittelklick in der unteren Mitte

pinch-to-zoom = Zwei-Finger-Zoom
    .desc = Mit zwei Fingern in den Inhalt zoomen, wenn die Anwendung den Zoom unterstützt.

tap-to-click = Tippen zum Klicken
    .desc = Ermöglicht das Tippen mit einem Finger für den ersten Klick, mit zwei Fingern für den zweiten Klick und mit drei Fingern für den mittleren Klick.

touchpad = Touchpad
    .acceleration = Touchpad-Beschleunigung aktivieren
    .desc = Touchpad-Geschwindigkeit, Klickoptionen, Gesten.
    .speed = Touchpad-Geschwindigkeit

## Eingabe: Gesten

gestures = Gesten
    .four-finger-down = Mit vier Fingern nach unten wischen
    .four-finger-left = Mit vier Fingern nach links wischen
    .four-finger-right = Mit vier Fingern nach rechts wischen
    .four-finger-up = Mit vier Fingern nach oben wischen
    .three-finger-any = Mit drei Fingern in eine beliebige Richtung wischen

switch-workspaces = Arbeitsflächen wechseln
    .horizontal = Mit vier Fingern nach links/rechts wischen
    .vertical = Mit vier Fingern nach oben/unten wischen

switch-between-windows = Zwischen Fenstern wechseln
open-application-library = Anwendungsbibliothek öffnen
open-workspaces-view = Arbeitsflächenübersicht öffnen

## Uhrzeit & Sprache

time = Uhrzeit & Sprache
    .desc = N/V

time-date = Datum & Uhrzeit
    .desc = Zeitzone, automatische Uhreinstellungen und einige Zeitformatierungen.
    .auto = Automatisch festlegen
    .auto-ntp = Datum & Uhrzeit werden automatisch aktualisiert, wenn die Zeitzone eingestellt ist.

time-zone = Zeitzone
    .auto = Automatische Zeitzone
    .auto-info = Erfordert Ortungsdienste und Internetzugang

time-format = Datum- & Zeitformat
    .twenty-four = 24-Stunden-Uhrzeit
    .show-seconds = Sekunden anzeigen
    .first = Erster Tag der Woche
    .show-date = Datum im oberen Panel anzeigen
    .friday = Freitag
    .saturday = Samstag
    .sunday = Sonntag
    .monday = Montag

time-region = Region & Sprache
    .desc = Datum, Uhrzeiten und Zahlen gemäß deiner Region formatieren

## System

system = System & Konten

## System: Über

about = Über
    .desc = Gerätename, Hardwareinfo, Voreinstellungen des Betriebssystems.

about-device = Gerätename
    .desc = Dieser Name erscheint bei anderen Netzwerk- oder Bluetooth-Geräten.

about-hardware = Hardware
    .model = Hardwaremodell
    .memory = Speicher
    .processor = Prozessor
    .graphics = Grafik
    .disk-capacity = Festplattenkapazität

about-os = Betriebssystem
    .os = Betriebssystem
    .os-architecture = Betriebssystemarchitektur
    .desktop-environment = Desktopumgebung
    .windowing-system = Fenstersystem

about-related = Zugehörige Einstellungen
    .support = Unterstützung erhalten

## System: Firmware

firmware = Firmware
    .desc = Firmwaredetails.

## System: Benutzer

users = Benutzer
    .desc = Authentifizierung und Benutzerkonten.
