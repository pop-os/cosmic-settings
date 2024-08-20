app = Impostazioni di COSMIC

unknown = Sconosciuto

number = { $number }

## Networking: Wired

wired = Via cavo
    .desc = Connessione via cavo, profili di connessione

## Networking: Online Accounts

online-accounts = Account online
    .desc = Aggiungi account IMAP, SMTP e enterprise

## Desktop

desktop = Desktop

## Desktop: Wallpaper

wallpaper = Sfondo
    .change = Cambia immagine ogni
    .desc = Wallpaper images, colors, and slideshow options.
    .fit = Wallpaper fit
    .folder-dialog = Choose wallpaper folder
    .image-dialog = Choose wallpaper image
    .plural = Wallpapers
    .same = Same wallpaper on all displays
    .slide = Slideshow

add-color = Aggiungi colore
add-image = Aggiungi immagine
all-displays = Tutti i display
colors = Colori
dialog-add = Aggiungi
fill = Riempi
fit-to-screen = Adatta allo schermo
open-new-folder = Apri nuova cartella
recent-folders = Cartelle recenti

x-minutes = { $number } minutes
x-hours = { $number ->
    [1] 1 hour
    *[other] { $number } hours
}

## Desktop: Appearance

appearance = Aspetto
    .desc = Colore d'accento e tema COSMIC.

accent-color = Colore d'accento
app-background = Sfondo applicazione o finestra
auto = Automatico
close = Chiudi
color-picker = Selezione colore
copied-to-clipboard = Copiato negli appunti
copy-to-clipboard = Copia negli appunti
dark = Scuro
export = Esporta
hex = Esadecimale
import = Importa
light = Chiaro
mode-and-colors = Modalità e colori
recent-colors = Colori recenti
reset-to-default = Ripristina predefinito
rgb = RGB
window-hint-accent = Colore d'accento per la finestra attiva
window-hint-accent-toggle = Usa il colore del tema come colore d'accento

auto-switch = Cambia automaticamente dal tema chiaro al tema scuro
    .sunrise = Switches to Light mode at sunrise
    .sunset = Switches to Dark mode at sunset
    .next-sunrise = Switches to Light mode at next sunrise
    .next-sunset = Switches to Dark mode at next sunset

container-background = Sfondo contenitore
    .desc-detail = Il colore di sfondo del contenitore viene usato per la barra laterale di navigazione, finestre di dialogo e widget simili. Il valore predefinito deriva dallo sfondo dell'applicazione o finestra.
    .reset = Ripristina in "Automatico"
    .desc = Il colore primario del contenitore usato per la barra laterale, finestre di dialogo e vari widget.

control-tint = Tinta per componenti di controllo
    .desc = Usata come sfondo per i pulsanti, caselle di ricerca, caselle di testo e componenti simili.

frosted = Effetto "vetro smerigliato" per l'interfaccia
    .desc = Applica un effetto di sfocatura al pannello, la barra delle applicazioni, gli applet, il launcher e la libreria della applicazioni.

experimental-settings = Experimental settings

enable-export = Apply this theme to GNOME apps.
    .desc = Not all toolkits support auto-switching. Non-COSMIC apps may need to be restarted after a theme change.

icon-theme = Icon theme
    .desc = Applies a different set of icons to applications.

text-tint = Tinta per il testo dell'interfaccia
    .desc = Color used to derive interface text colors that have sufficient contrast on various surfaces.

style = Stile
    .round = Round
    .slightly-round = Slightly round
    .square = Square

# interface density left out for now
window-management-appearance = Window Management
    .active-hint = Active window hint size
    .gaps = Gaps around tiled windows

## Desktop: Notifications

notifications = Notifiche
    .desc = Non disturbare, notifiche nel blocco schermo e impostazioni per le applicazioni

## Desktop: Panel

panel = Pannello
    .desc = Top bar with desktop controls and menus.

add = Aggiungi
add-applet = Aggungi applet
all = Tutti
applets = Applet
center-segment = Segmento centrale
drop-here = Trascina gli applet quì
end-segment = Segmento finale
large = Grande
no-applets-found = Nessun applet trovato
panel-bottom = In basso
panel-left = A sinistra
panel-right = A destra
panel-top = In alto
search-applets = Ricerca applet
small = Piccolo
start-segment = Segmento iniziale

panel-appearance = Tema
    .match = Match desktop
    .light = Light
    .dark = Dark

panel-behavior-and-position = Comportamento e posizione
    .autohide = Automatically hide panel
    .dock-autohide = Automatically hide dock
    .position = Position on screen
    .display = Show on display

panel-style = Stile
    .anchor-gap = Gap between panel and screen edges
    .dock-anchor-gap = Gap between dock and screen edges
    .extend = Extend panel to screen edges
    .dock-extend = Extend dock to screen edges
    .appearance = Appearance
    .size = Size
    .background-opacity = Background opacity

panel-applets = Configurazione
    .dock-desc = Configure dock applets.
    .desc = Configure panel applets.

panel-missing = La configurazione del pannello è mancante
    .desc = The panel configuration file is missing due to use of a custom configuration or it is corrupted.
    .fix = Reset to default

## Desktop: Dock

dock = Barra delle applicazioni
    .desc = Panel with pinned applications in the app tray and other applets.

## Desktop: Window management

window-management = Gestione finestre
    .desc = Super key action, window control options, and additional window tiling options.

super-key = Super key
    .launcher = Open Launcher
    .workspaces = Open Workspaces
    .applications = Open Applications

window-controls = Controlli finestra
    .minimize = Show minimize button
    .maximize = Show maximize button

## Desktop: Workspaces

workspaces = Spazi di lavoro
    .desc = Workspace orientation and behavior.

workspaces-behavior = Comportamento spazi di lavoro
    .dynamic = Dynamic workspaces
    .dynamic-desc = Automatically removes empty workspaces.
    .fixed = Fixed Number of Workspaces
    .fixed-desc = Add or remove workspaces in the overview.

workspaces-multi-behavior = Comportamento multi-schermo
    .span = Workspaces Span Displays
    .separate = Displays Have Separate Workspaces

workspaces-overview-thumbnails = Anteprima spazi di lavoro
    .show-number = Show Workspace Number
    .show-name = Show Workspace Name

workspaces-orientation = Orientamento spazi di lavoro
    .vertical = Verticale
    .horizontal = Orizzontale

hot-corner = Bordi reattivi
    .top-left-corner = Enable top-left hot corner for Workspaces

## Displays

-requires-restart = Richiede il riavvio

color = Colore
    .depth = Profondità colore
    .profile = Profilo colore
    .sidebar = Profilo colore
    .temperature = Temperatura colore

display = Schermi
    .desc = Gestione schermi, modalità GPU e modalità notturna
    .arrangement = Ordinamento schermi
    .arrangement-desc = Trascina gli schermi per ordinarli.
    .enable = Attiva schermo
    .external = { $size } { $output } Schermo esterno
    .laptop = { $size } Schermo laptop
    .options = Opzioni schermo
    .refresh-rate = Frequenza d'aggiornamento
    .resolution = Risoluzione
    .scale = Scala

mirroring = Duplicazione
    .id = Duplicazione { $id }
    .dont = Non duplicare
    .mirror = Duplica { $display }
    .project = Proietta su { $display ->
        [all] tutti gli schermi
        *[other] { $display }
    }
    .project-count = Proiettamento su { $count} other { $count ->
        [1] altro schermo
        *[other] altri schermi
    }

night-light = Modalità notturna
    .auto = Automatica (dal tramonto all'alba)
    .desc = Riduci la luce blu usando colori più caldi.

orientation = Orientamento
    .standard = Standard
    .rotate-90 = 90 gradi in senso orario
    .rotate-180 = 180 gradi
    .rotate-270 = 180 gradi

scheduling = Pianificazione
    .manual = Pianificazione manuale

dialog = Dialog
    .title = Keep These Display Settings?
    .keep-changes = Keep Changes
    .change-prompt = Settings changes will automatically revert in { $time } seconds.
    .revert-settings = Revert Settings

## Sound

sound = Suono
    .desc = N/A

sound-output = Output
    .volume = Volume di output
    .device = Dispositivo di output
    .level = Livello di output
    .config = Configurazione
    .balance = Bilanciamento

sound-input = Input
    .volume = Volume di input
    .device = Dispositivo di input
    .level = Livello di input

sound-alerts = Suoni di sistema
    .volume = Volume dei suoni di sistema
    .sound = Tipo suono di sistema

sound-applications = Applicazioni
    .desc = Volumi delle applicazioni

profile = Profile

## Power

power = Power & Battery
    .desc = Manage power settings

power-mode = Power Mode
    .battery = Extended battery life
    .battery-desc = Reduced power usage and silent performance.
    .balanced = Balanced
    .balanced-desc = Quiet performance and moderate power usage.
    .performance = High performance
    .performance-desc = Peak performance and power usage.
    .nobackend = Backend not found. Install system76-power or power-profiles-daemon.

## Input

acceleration-desc = La velocità del puntatore viene basata sulla sua velocità

disable-while-typing = Disabilita durante la digitazione

input-devices = Dispositivi di immissione
    .desc = Dispositivi di immissione (come mouse e tastiera)

primary-button = Pulsante principale
    .left = Sinistro
    .right = Destro

scrolling = Scorrimento
    .two-finger = Scorrimento con due dita
    .edge = Scorrimento nel bordo con un dito
    .speed = Velocità di scorrimento
    .natural = Scorrimento naturale
    .natural-desc = Fa scorrere il contenuto invece che la visualizzazione

## Input: Keyboard

slow = Slow
fast = Fast
short = Short
long = Long
keyboard = Tastiera
    .desc = Immissione con la tastiera

keyboard-sources = Sorgenti di immissione
    .desc = Le sorgenti di immissione possono essere cambiate usando la combinazione Super + Spazio. Ciò può essere personalizzato nelle impostazioni delle scorciatorie da tastiera.
    .move-up = Sposta in alto
    .move-down = Sposta in basso
    .settings = Impostazioni
    .view-layout = Visualizza il layout della tastiera
    .remove = Rimuovi
    .add = Add input source

keyboard-special-char = Immissione caratteri speciali
    .alternate = Tasto per i caratteri speciali
    .compose = Tasto di composizione

keyboard-typing-assist = Typing
    .repeat-rate = Repeat rate
    .repeat-delay = Repeat delay

added = Added
type-to-search = Type to search...

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Scorciatoie da tastiera
    .desc = Visualizza e modifica le scorciatoie

add-keybinding = Add keybinding
cancel = Cancel
command = Command
custom = Custom
debug = Debug
disabled = Disabled
migrate-workspace-prev = Migrate workspace to previous output
migrate-workspace-next = Migrate workspace to next output
migrate-workspace = Migrate workspace to output { $direction ->
    *[down] down
    [left] left
    [right] right
    [up] up
}
navigate = Navigate
replace = Replace
shortcut-name = Shortcut name
system-controls = System controls
terminate = Terminate
toggle-stacking = Toggle window stacking
type-key-combination = Type key combination

custom-shortcuts = Custom Shortcuts
    .add = Add shortcut
    .context = Add Custom Shortcut
    .none = No custom shortcuts

modified = { $count } modified

nav-shortcuts = Navigation
    .prev-output = Focus previous output
    .next-output = Focus next output
    .last-workspace = Focus last workspace
    .prev-workspace = Focus previous workspace
    .next-workspace = Focus next workspace
    .focus = Focus window { $direction ->
        *[down] down
        [in] in
        [left] left
        [out] out
        [right] right
        [up] up
    }
    .output = Switch to output { $direction ->
        *[down] down
        [left] left
        [right] right
        [up] up
    }
    .workspace = Switch to workspace { $num }

manage-windows = Manage windows
    .close = Close window
    .maximize = Maximize window
    .minimize = Minimize window
    .resize-inwards = Resize window inwards
    .resize-outwards = Resize window outwards
    .toggle-sticky = Toggle sticky window

move-windows = Move Windows
    .direction = Move window { $direction ->
        *[down] down
        [left] left
        [right] right
        [up] up
    }
    .display = Move window one monitor { $direction ->
        *[down] down
        [left] left
        [right] right
        [up] up
    }
    .workspace = Move window one workspace { $direction ->
        *[below] below
        [left] left
        [right] right
        [above] above
    }
    .workspace-num = Move window to workspace { $num }
    .prev-workspace = Move window to prev workspace
    .next-workspace = Move window to next workspace
    .last-workspace = Move window to last workspace
    .next-display = Move window to next display
    .prev-display = Move window to prev display
    .send-to-prev-workspace = Move window to previous workspace
    .send-to-next-workspace = Move window to next workspace

system-shortcut = System
    .app-library = Open the app library
    .brightness-down = Decrease display brightness
    .brightness-up = Increase display brightness
    .home-folder = Open home folder
    .keyboard-brightness-down = Decrease keyboard brightness
    .keyboard-brightness-up = Increase keyboard brightness
    .launcher = Open the launcher
    .lock-screen = Lock the screen
    .mute = Mute audio output
    .mute-mic = Mutes microphone input
    .play-pause = Play/Pause
    .play-next = Next track
    .play-prev = Previous track
    .screenshot = Take a screenshot
    .terminal = Open a terminal
    .volume-lower = Decrease audio output volume
    .volume-raise = Increase audio output volume
    .web-browser = Opens a web browser
    .window-switcher = Switch between open windows
    .workspace-overview = Open the workspace overview

window-tiling = Window tiling
    .horizontal = Set horizontal orientation
    .vertical = Set vertical orientation
    .swap-window = Swap window
    .toggle-tiling = Toggle window tiling
    .toggle-stacking = Toggle window stacking
    .toggle-floating = Toggle window floating
    .toggle-orientation = Toggle orientation

replace-shortcut-dialog = Replace Shortcut?
    .desc = { $shortcut } is used by { $name }. If you replace it, { $name } will be disabled.

## Input: Mouse

mouse = Mouse
    .desc = Velocità mouse, accelerazione e scorrimento naturale
    .speed = Velocità mouse
    .acceleration = Abilita accelerazione mouse

## Input: Touchpad

click-behavior = Comportamento click
    .click-finger = Click secondario con due dita e click centrale con tre dita
    .button-areas = Click secondario nel bordo in basso a destra e click centrale nel bordo centrale in basso

pinch-to-zoom = Pizzica per lo zoom
    .desc = Usa due dita per ingrandire i contenuti nelle applicazioni che supportano l'ingrandimento.

tap-to-click = Tocca per fare click
    .desc = Enables single-finger tap for primary click, two-finger tap for secondary click, and three-finger tap for middle click.

touchpad = Touchpad
    .acceleration = Abilita l'accelerazione del touchpad
    .desc = Velocità del touchpad, opzioni click e gestures.
    .speed = Velocità del touchpad

## Input: Gestures

gestures = Gestures
    .four-finger-down = Scorri in basso usando quattro dita
    .four-finger-left = Scorri a sinistra usando quattro dita
    .four-finger-right = Scorri a destra usando quattro dita
    .four-finger-up = Scorri in alto usando quattro dita
    .three-finger-any = Scorri in qualsiasi direzione usando tre dita

switch-between-windows = Scorri tra le varie finestre
switch-to-next-workspace = Vai al prossimo spazio di lavoro
switch-to-prev-workspace = Vai allo spazio di lavoro precedente
open-application-library = Apri la libreria delle applicazioni
open-workspaces-view = Riepilogo degli spazi di lavoro

## Time & Language

time = Orario e linguaggio
    .desc = N/A

time-date = Data e ora
    .desc = Fuso orario, impostazioni orologio automatico e formattazione orario.
    .auto = Imposta automaticamente
    .auto-ntp = Date & time will update automatically when the time zone is set.

time-zone = Fuso orario
    .auto = Fuso orario automatico
    .auto-info = Richiede i servizi di posizione e l'accesso a internet

time-format = Formato data e ora
    .twenty-four = Formato 24 ore
    .first = Primo giorno della settimana
    .show-date = Mostra data sul Top Panel
    .friday = Venerdì
    .saturday = Sabato
    .sunday = Domenica
    .monday = Lunedì

time-region = Lingua e località
    .desc = Formattazione data, ora e numeri basata sulla località

## System

system = Sistema e account

## System: About

about = Informazioni
    .desc = Nome dispositivo, informazioni hardware e impostazioni predefinite di sistema.

about-device = Nome dispositivo
    .desc = Ai dispositivi bluetooth, o a una rete, apparirà questo nome.

about-hardware = Hardware
    .model = Modello hardware
    .memory = Memoria
    .processor = Processore
    .graphics = GPU
    .disk-capacity = Capacità archiviazione

about-os = Sistema Operativo
    .os = Sistema Operativo
    .os-architecture = Architettura Sistema Operativo
    .desktop-environment = Ambiente Desktop
    .windowing-system = Gestore finestre

about-related = Impostazioni correlate
    .support = Ottieni supporto

## System: Firmware

firmware = Firmware
    .desc = Dettagli del firmware

## System: Users

users = Utenti
    .desc = Autenticazione, login e blocco schermo
