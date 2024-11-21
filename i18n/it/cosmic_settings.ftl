app = Impostazioni di COSMIC

unknown = Sconosciuto

number = { $number }

## Network & Wireless

connections-and-profiles = Connessione { $variant ->
    [wired] Ethernet
    [wifi] Wi-Fi
    [vpn] VPN
    *[other] Sconosciuto
} e profili di connessione.

add-network = Aggiungi rete
    .profile = Aggiungi profilo
add-vpn = Aggiungi VPN
airplane-on = Modalità aereo attivata.
cable-unplugged = Cavo scollegato
connect = Connetti
connected = Connesso
connecting = Connessione in corso…
disconnect = Disconnetti
forget = Dimentica
known-networks = Reti conosciute
network-and-wireless = Rete e wireless
no-networks = Nessuna rete trovata.
no-vpn = Nessuna connessione VPN disponibile.
password = Password
remove = Rimuovi
settings = Impostazioni
username = Nome utente
visible-networks = Reti visibili

auth-dialog = Autenticazione richiesta
    .vpn-description = Inserisci il nome utente e la password richiesti dal servizio VPN.
    .wifi-description = Inserisci la password o la chiave di crittografia. Puoi anche connetterti premendo il pulsante “WPS” sul router.

forget-dialog = Dimenticare questa rete Wi-Fi?
    .description = Sarà necessario reinserire la password per utilizzare questa rete Wi-Fi in futuro.

network-device-state =
    .activated = Connesso alla rete
    .config = Connessione in corso
    .deactivating = Disconnessione in corso
    .disconnected = Disconnesso
    .failed = Connessione fallita
    .ip-check = Verifica della connessione in corso
    .ip-config = Richiesta delle informazioni IP e di routing in corso
    .need-auth = Autenticazione richiesta
    .prepare = Preparazione alla connessione alla rete
    .secondaries = In attesa della connessione secondaria
    .unavailable = Non disponibile
    .unknown = Stato sconosciuto
    .unmanaged = Non gestito
    .unplugged = Cavo scollegato

remove-connection-dialog = Rimuovere il profilo di connessione?
    .vpn-description = Sarà necessario reinserire la password per utilizzare questa rete in futuro.
    .wired-description = Sarà necessario ricreare questo profilo per utilizzarlo in futuro.

vpn = VPN
    .connections = Connessioni VPN
    .remove = Rimuovi profilo di connessione
    .select-file = Seleziona un file di configurazione VPN

wired = Ethernet
    .adapter = Adattatore ethernet { $id }
    .connections = Connessioni cablate
    .devices = Dispositivi cablati
    .remove = Rimuovi profilo di connessione

wifi = Wi-Fi
    .adapter = Adattatore Wi-Fi { $id }
    .forget = Dimentica questa rete

## Networking: Online Accounts

online-accounts = Account online
    .desc = Aggiungi account IMAP, SMTP e enterprise

# Bluetooth

confirm = Conferma

bluetooth = Bluetooth
    .desc = Gestisci dispositivi Bluetooth
    .status = Il sistema è visibile come { $aliases } quando le impostazioni Bluetooth sono aperte.
    .connected = Connesso
    .connecting = Connessione ...
    .disconnecting = Disconnessione ...
    .connect = Connetti
    .disconnect = Disconnetti
    .forget = Dimentica
    .dbus-error = Errore durante l'interazione con DBus: { $why }
    .show-device-without-name = Mostra dispositivi senza nome

bluetooth-paired = Dispositivi accoppiati
    .connect = Connetti
    .battery = { $percentage }% batteria rimanente

bluetooth-confirm-pin = Confirma PIN Bluetooth
    .description = Conferma che il PIN coincida con quello mostrato su { $device }

bluetooth-available = Dispositivi nelle vicinanze

bluetooth-adapters = Adattatori Bluetooth

## Desktop

desktop = Desktop

## Desktop: Wallpaper

wallpaper = Sfondo
    .change = Cambia immagine ogni
    .desc = Immagine di sfondo, colori e sfondo periodico.
    .fit = Adatta sfondo
    .folder-dialog = Scegliere la cartella in cui si trova lo sfondo
    .image-dialog = Scegliere l'immagine di sfondo
    .plural = Sfondi
    .same = Stessa immagine in tutti gli schermi
    .slide = Sfondo periodico

add-color = Aggiungi colore
add-image = Aggiungi immagine
all-displays = Tutti i display
colors = Colori
dialog-add = Aggiungi
fill = Riempi
fit-to-screen = Adatta allo schermo
open-new-folder = Apri nuova cartella
recent-folders = Cartelle recenti

x-minutes = { $number ->
    [1] 1 minuto
    *[other] { $number } minuti
}
x-hours = { $number ->
    [1] 1 ora
    *[other] { $number } ore
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
    .sunrise = All'alba, cambia al tema chiaro
    .sunset = Al tramonto, cambia al tema scuro
    .next-sunrise = Passa al tema chiato alla prossima alba
    .next-sunset = Passa al tema scuro al prossimo tramonto

container-background = Sfondo contenitore
    .desc-detail = Il colore di sfondo del contenitore viene usato per la barra laterale di navigazione, finestre di dialogo e widget simili. Il valore predefinito deriva dallo sfondo dell'applicazione o finestra.
    .reset = Ripristina in "Automatico"
    .desc = Il colore primario del contenitore usato per la barra laterale, finestre di dialogo e vari widget.

control-tint = Tinta per componenti di controllo
    .desc = Usata come sfondo per i pulsanti, caselle di ricerca, caselle di testo e componenti simili.

frosted = Effetto "vetro smerigliato" per l'interfaccia
    .desc = Applica un effetto di sfocatura al pannello, la barra delle applicazioni, gli applet, il launcher e la libreria della applicazioni.

enable-export = Applica questo tema alle app GNOME.
    .desc = Non tutte le applicazioni supportano l'auto-switch. Le app Non-COSMIC potrebbero dover essere riavviate dopo aver cambiato tema.

icon-theme = Tema icone
    .desc = Applica icone differenti alle applicazioni.

text-tint = Tinta per il testo dell'interfaccia
    .desc = Colore su cui si basano i colori del testo con sufficiente contrasto in diverse superfici.

style = Stile
    .round = Arrotondato
    .slightly-round = Leggermente arrotondato
    .square = Spigoloso

interface-density = Interface Density
    .comfortable = Comfort
    .compact = Compatta
    .spacious = Spaziosa

window-management = Gestione finestre
    .active-hint = Dimensione bordo finestra attiva
    .gaps = Spaziatura finestre in modalità tiling

### Experimental

experimental-settings = Impostazioni sperimentali
icons-and-toolkit = Temi delle icone e del toolkit
interface-font = Carattere di sistema
monospace-font = Carattere monospaziato

## Desktop: Notifications

notifications = Notifiche
    .desc = Non disturbare, notifiche nel blocco schermo e impostazioni per le applicazioni

## Desktop: Panel

panel = Pannello
    .desc = Barra superiore con controlli desktop e menu

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
    .match = Combacia col desktop
    .light = Chiaro
    .dark = Scuro

panel-behavior-and-position = Comportamento e posizione
    .autohide = Nascondi automaticamente il pannello
    .dock-autohide = Nascondi automaticamente la barra delle applicazioni
    .position = Posizione nello schermo
    .display = Mostra nel display

panel-style = Stile
    .anchor-gap = Spazio tra il pannello e i bordi dello schermo
    .dock-anchor-gap = Spazio tra la barra delle applicazioni e i bordi dello schermo
    .extend = Estendi pannello ai bordi dello schermo
    .dock-extend = Estendi dock ai bordi dello schermo
    .appearance = Aspetto
    .size = Grandezza
    .background-opacity = Opacità sfondo

panel-applets = Configurazione
    .dock-desc = Configura gli applet del dock
    .desc = Configura gli applet del pannello

panel-missing = La configurazione del pannello è mancante
    .desc = Il file di configurazione è corrotto o non è stato trovato per via di una configurazione non prevista
    .fix = Ripristina configurazione predefinita

## Desktop: Dock

dock = Barra delle applicazioni
    .desc = Pannello contenente le applicazioni fissate.

## Desktop: Window management

window-management = Gestione finestre
    .desc = Azione tasto Super, opzioni controllo delle finestre, allineamento e impostazioni aggiuntive.

super-key = Tasto Super
    .launcher = Apri Launcher
    .workspaces = Apri Spazi di Lavoro
    .applications = Apri Applicazioni
    .disable = Disabilita

window-controls = Controlli finestra
    .minimize = Mostra pulsante "minimizza"
    .maximize = Mostra pulsante "massimizza"

focus-navigation = Focus Navigation
    .focus-follows-cursor = Focus segue il cursore
    .focus-follows-cursor-delay = Focus segue il cursore con ritarto in ms
    .cursor-follows-focus = Cursore segue il focus

## Desktop: Workspaces

workspaces = Spazi di lavoro
    .desc = Imposta numero spazi di lavoro, il loro comportamento e la loro posizione.

workspaces-behavior = Comportamento spazi di lavoro
    .dynamic = Spazi di lavoro dinamici
    .dynamic-desc = Rimuovi automaticamente gli spazi di lavoro vuoti
    .fixed = Spazi di lavoro limitati
    .fixed-desc = Aggiungi o rimuovi gli spazi di lavoro

workspaces-multi-behavior = Comportamento multi-schermo
    .span = Gli spazi di lavoro si espandono in tutti gli schermi
    .separate = Ogni schermo ha gli spazi di lavoro separati

workspaces-overview-thumbnails = Anteprima spazi di lavoro
    .show-number = Mostra il numero dello spazio di lavoro
    .show-name = Mostra il nome dello spazio di lavoro

workspaces-orientation = Orientamento spazi di lavoro
    .vertical = Verticale
    .horizontal = Orizzontale

hot-corner = Bordi reattivi
    .top-left-corner = Abilita bordo reattivo in alto a sinistra per gli spazi di lavoro

## Displays

-requires-restart = Richiede il riavvio

color = Colore
    .depth = Profondità colore
    .profile = Profilo colore
    .sidebar = Profili colore
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
    .project-count = Proiezione su { $count} { $count ->
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
    .rotate-270 = 90 gradi in senso antiorario

scheduling = Pianificazione
    .manual = Pianificazione manuale

dialog = Dialog
    .title = Mantenere queste impostazioni per lo schermo?
    .keep-changes = Mantieni impostazioni
    .change-prompt = Le impostazioni verranno automaticamente ripristinate tra { $time } secondi.
    .revert-settings = Ripristina impostazioni

legacy-applications = Scala delle applicazioni del sistema X11
    .scaled-by-system = Scala tutte le applicazioni X11
    .system-description = Le applicazioni X11 appariranno sfocate su schermi HiDPI.
    .scaled-natively = Renderizza le applicazioni X11 alla risoluzione nativa
    .native-description = Le applicazioni X11 che non supportano la scalatura saranno piccole quando si utilizzano display HiDPI. Abilita questa opzione per i giochi per utilizzare la risoluzione completa del monitor.

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

profile = Profilo

## Power

power = Alimentazione e batteria
    .desc = Gestione impostazioni energetiche

battery = Batteria
  .minute = { $value } { $value ->
        [one] minuto
       *[other] minuti
  }
  .hour = { $value } { $value ->
        [one] ora
       *[other] ore
  }
  .day = { $value } { $value ->
        [one] giorno
       *[other] giorni
  }
  .less-than-minute = Meno di un minuto
  .and = e
  .remaining-time = { $time } fino a { $action ->
        [full] carica completa
       *[other] scarica
   }

connected-devices = Connected Devices
  .unknown = Unknown device

power-mode = Power Mode
    .battery = Estendi la vita della batteria
    .battery-desc = Ridotto consumo energetico e ventole silenziose.
    .balanced = Bilanciato
    .balanced-desc = Consumo energetico e ventole moderati.
    .performance = Alte prestazioni
    .performance-desc = Prestazioni e consumi energetici elevati.
    .nobackend = Requisito non trovato. Installa i pacchetti system76-power o power-profiles-daemon.

## Input

acceleration-desc = La velocità del puntatore viene basata sulla sua velocità

disable-while-typing = Disabilita durante la digitazione

input-devices = Dispositivi di immissione
    .desc = Dispositivi di immissione (come mouse e tastiera)

primary-button = Pulsante principale
    .desc =  Imposta l'ordine dei pulsanti fisici.
    .left = Sinistro
    .right = Destro

scrolling = Scorrimento
    .two-finger = Scorrimento con due dita
    .edge = Scorrimento nel bordo con un dito
    .speed = Velocità di scorrimento
    .natural = Scorrimento naturale
    .natural-desc = Fa scorrere il contenuto invece che la visualizzazione

## Input: Keyboard

slow = Lento
fast = Veloce
short = Corto
long = Lungo
keyboard = Tastiera
    .desc = Immissione con la tastiera

keyboard-sources = Sorgenti di immissione
    .desc = Le sorgenti di immissione possono essere cambiate usando la combinazione Super + Spazio. Ciò può essere personalizzato nelle impostazioni delle scorciatorie da tastiera.
    .move-up = Sposta in alto
    .move-down = Sposta in basso
    .settings = Impostazioni
    .view-layout = Visualizza il layout della tastiera
    .remove = Rimuovi
    .add = Aggiungi sorgente di immissione

keyboard-special-char = Immissione caratteri speciali
    .alternate = Tasto per i caratteri speciali
    .compose = Tasto di composizione
    .caps = Caps Lock

keyboard-typing-assist = Digitazione
    .repeat-rate = Ripeti ritmo
    .repeat-delay = Ripeti ritardo

added = Aggiunto
type-to-search = Digita per cercare...
show-extended-input-sources = Mostra

## Input: Keyboard: Scorciatoie

keyboard-shortcuts = Scorciatoie da tastiera
    .desc = Visualizza e modifica le scorciatoie

add-keybinding = Aggiungere un collegamento ai tasti
cancel = Annulla
command = Comando
custom = Personalizzato
debug = Debug
disabled = Disabilitato
migrate-workspace-prev = Sposta lo spazio di lavoro all'output precedente
migrate-workspace-next = Sposta lo spazio di lavoro all'output successivo
migrate-workspace = Sposta lo spazio di lavoro all'output { $direction ->
    *[down] basso
    [left] sinistra
    [right] destra
    [up] alto
}
navigate = Naviga
replace = Sostituisci
shortcut-name = Nome collegamento
system-controls = Controlli di sistema
terminate = Termina
toggle-stacking = Alterna la sovrapposizione delle finestre
type-key-combination = Digita combinazione di tasti

custom-shortcuts = Scorciatoie personalizzate
    .add = Aggiungi scorciatoia
    .context = Aggiungi scorciatoia personalizzata
    .none = Nessuna scorciatoia personalizzata

modified = { $count } modificata

nav-shortcuts = Navigazione
    .prev-output = Focus su output precedente
    .next-output = Focus su output successivo
    .last-workspace = Focus sull'ultimo spazio di lavoro
    .prev-workspace = Focus sullo spazio di lavoro precedente
    .next-workspace = Focus sullo spazio di lavoro successivo
    .focus = Focus sulla finestra { $direction ->
        *[down] giù
        [in] dentro
        [left] sinistra
        [out] fuori
        [right] destra
        [up] su
    }
    .output = Passa a output { $direction ->
        *[down] giù
        [left] sinistra
        [right] destra
        [up] su
    }
    .workspace = Passa allo spazio di lavoro { $num }

manage-windows = Gestione finestre
    .close = Chiudi finestra
    .maximize = Massimizza finestra
    .minimize = Minimizza finestra
    .resize-inwards = Ridimensiona finestra verso l'interno
    .resize-outwards = Ridimensiona finestra verso l'esterno
    .toggle-sticky = Attiva/disattiva finestra fissa

move-windows = Sposta finestre
    .direction = Sposta finestra { $direction ->
        *[down] giù
        [left] sinistra
        [right] destra
        [up] su
    }
    .display = Sposta finestra su un monitor { $direction ->
        *[down] giù
        [left] sinistra
        [right] destra
        [up] su
    }
    .workspace = Sposta finestra su uno spazio di lavoro { $direction ->
        *[below] sotto
        [left] sinistra
        [right] destra
        [above] sopra
    }
    .workspace-num = Sposta finestra allo spazio di lavoro { $num }
    .prev-workspace = Sposta finestra allo spazio di lavoro precedente
    .next-workspace = Sposta finestra allo spazio di lavoro successivo
    .last-workspace = Sposta finestra all'ultimo spazio di lavoro
    .next-display = Sposta finestra al monitor successivo
    .prev-display = Sposta finestra al monitor precedente
    .send-to-prev-workspace = Sposta finestra allo spazio di lavoro precedente
    .send-to-next-workspace = Sposta finestra allo spazio di lavoro successivo

system-shortcut = Sistema
    .app-library = Apri la libreria delle app
    .brightness-down = Riduci luminosità del display
    .brightness-up = Aumenta luminosità del display
    .home-folder = Apri cartella home
    .keyboard-brightness-down = Riduci luminosità della tastiera
    .keyboard-brightness-up = Aumenta luminosità della tastiera
    .launcher = Apri il launcher
    .lock-screen = Blocca lo schermo
    .mute = Disattiva audio
    .mute-mic = Disattiva microfono
    .play-pause = Riproduci/Pausa
    .play-next = Traccia successiva
    .play-prev = Traccia precedente
    .screenshot = Cattura schermata
    .terminal = Apri un terminale
    .volume-lower = Riduci volume dell'audio
    .volume-raise = Aumenta volume dell'audio
    .web-browser = Apri un browser web
    .window-switcher = Cambia finestra aperta
    .workspace-overview = Apri panoramica degli spazi di lavoro

window-tiling = Affiancamento finestre
    .horizontal = Imposta orientamento orizzontale
    .vertical = Imposta orientamento verticale
    .swap-window = Scambia finestra
    .toggle-tiling = Attiva/disattiva affiancamento finestre
    .toggle-stacking = Attiva/disattiva impilamento finestre
    .toggle-floating = Attiva/disattiva finestra flottante
    .toggle-orientation = Cambia orientamento

replace-shortcut-dialog = Sostituire scorciatoia?
    .desc = { $shortcut } è utilizzata da { $name }. Se la sostituisci, { $name } sarà disabilitato.

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
    .desc = Usa due dita per ingrandire i contenuti nelle applicazioni che supportano l'ingrandimento

tap-to-click = Tocca per fare click
    .desc = Abilita il tocco singolo per il click principale, il tocco a due dita per il click secondario e il tocco con tre dita per il click centrale

touchpad = Touchpad
    .acceleration = Abilita l'accelerazione del touchpad
    .desc = Velocità del touchpad, opzioni click e gestures.
    .speed = Velocità del touchpad

## Input: Gestures

gestures = Scorrimento
    .four-finger-down = Scorri in basso usando quattro dita
    .four-finger-left = Scorri a sinistra usando quattro dita
    .four-finger-right = Scorri a destra usando quattro dita
    .four-finger-up = Scorri in alto usando quattro dita
    .three-finger-any = Scorri in qualsiasi direzione usando tre dita

switch-workspaces = Switch workspaces
    .horizontal = Four-finger swipe left/right
    .vertical = Four-finger swipe up/down

switch-between-windows = Scorri tra le varie finestre
open-application-library = Apri la libreria delle applicazioni
open-workspaces-view = Riepilogo degli spazi di lavoro

## Time & Language

time = Orario e lingua
    .desc = N/A

time-date = Data e ora
    .desc = Fuso orario, impostazioni orologio automatico e formattazione orario
    .auto = Imposta automaticamente
    .auto-ntp = La data e l'ora si aggiornano automaticamente quando viene impostato il fuso orario.

time-zone = Fuso orario
    .auto = Fuso orario automatico
    .auto-info = Richiede i servizi di posizione e l'accesso a internet

time-format = Formato data e ora
    .twenty-four = Formato 24 ore
    .show-seconds = Mostra i secondi
    .first = Primo giorno della settimana
    .show-date = Mostra data sul pannello superiore
    .friday = Venerdì
    .saturday = Sabato
    .sunday = Domenica
    .monday = Lunedì

time-region = Area geografica & lingua
    .desc = Impostazioni regionali di data, ora e numeri

## System

system = Sistema e account

## System: About

about = Informazioni
    .desc = Nome dispositivo, informazioni hardware e impostazioni predefinite di sistema

about-device = Nome dispositivo
    .desc = Ai dispositivi bluetooth, o a una rete, apparirà questo nome

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
