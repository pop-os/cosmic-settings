app = Impostazioni COSMIC

unknown = Sconosciuto

number = { $number }

## Desktop

desktop = Scrivania

## Desktop: Appearance

appearance = Aspetto
    .desc = Colori d'accento e tema Cosmic.

accent-color = Colore d'accento
app-background = Sfondo applicazione o finestra
auto = Automatico
close = Chiudi
color-picker = Selezione Colore
copied-to-clipboard = Copiato negli appunti
copy-to-clipboard = Copia negli appunti
dark = Scuro
export = Esporta
hex = Esadecimale
import = Importa
light = Chiaro
mode-and-colors = Modalità e colori
recent-colors = Colori recenti
reset-default = Reimposta predefinito
reset-to-default = Reimposta predefinito
rgb = RGB
window-hint-accent = Colore di suggerimento della finestra attiva
window-hint-accent-toggle = Usa il colore d'accento del tema come suggerimento della finestra attiva

auto-switch = Passa automaticamente dalla modalità Chiara a quella Scura
    .desc = All'alba passa automaticamente alla modalità Chiara

container-background = Sfondo del contenitore
    .desc-detail = Lo sfondo del contenitore è utilizzato per la barra laterale di navigazione, per il cassetto laterale, per le finestre di dialogo e cose simili.
    .reset = Reimpostazione a quello automatico
    .desc = Il colore primario del contenitore è utilizzato per la barra di navigazione laterale, per il cassetto laterale, per le finestre di dialogo e cose simili.

control-tint =  Tinta del componente di controllo
    .desc = Utilizzata come sfondo per tasti, input di ricerca, input di testo e componenti simili.

frosted = Effetto vetro ghiacciato sull'interfaccia di sistema
    .desc = Applica una sfocatura agli sfondi di pannello, barra applicazioni, applet, avviatore e libreria applicazioni.

text-tint = Tinta dell'interfaccia di testo
    .desc = Colore usato per generare i colori dell'interfaccia di testo che abbiano un contrasto sufficiente con le varie superfici.

style = Stile
    .round = Arrotondato
    .slightly-round = Leggermente arrotondato
    .square = Squadrato

# interface density left out for now
window-management = Gestione delle finestre
    .active-hint = Dimensioni dei suggerimenti sulla finestra
    .gaps = Spazio attorno tra le finestre a piastrelle

    ## Desktop: Display

-requires-restart = Richiede un riavvio

color = Color
    .depth = Profondità colore
    .profile = Profilo colore
    .sidebar = Profili colore
    .temperature = Temperatura colore

display = Schermi
    .desc = Gestisci schermi, cambio scheda grafica e luce notturna
    .arrangement = Disposizione schermi
    .arrangement-desc = Trascina schermi per riorganizzarli.
    .enable = abilita schermo
    .external = { $size } { $output } Schermo esterno
    .laptop = { $size } Schermo del portatile
    .options = Opzioni schermo
    .refresh-rate = Frequenza di aggiornamento
    .resolution = Risoluzione
    .scale = Scalemento

graphics-mode = Modalità grafica
    .mode = { $mode ->
        [compute] Calcolo
        *[hybrid] Ibrida
        [integrated] Integrata
        [nvidia] NVIDIA
    } graphics
    .enable = Abilita modalità grafica { $mode ->
        [compute] calcolo
        *[hybrid] ibrida
        [integrated] integrata
        [nvidia] NVIDIA
    }
    .desc = { $mode ->
        [compute] Utilizza la scheda grafica dedicata solo per carichi di lavoro computazionali. Disabilita schermi esterni { -requires-restart }.
        *[hybrid] Le applicazioni utilizzano la scheda grafica integrata, a meno che non richiedano esplicitamente di utilizzare quella dedicata. { -requires-restart }.
        [integrated] Speche la scheda grafica dedicata pere una maggiore durata della batteria e minore rumore delle ventole.
        [nvidia] La migliore esperienza grafica e il maggiore utilizzo di energia. { -requires-restart }.
    }
    .restart = Riavvia e passa alla modalità { $mode }?
    .restart-desc = Passare alla modalità { $mode } chiuderà tutte le applicazioni aperte

mirroring = Duplicazione schermo
    .id = Duplicazione { $id }
    .dont = Non duplicare
    .mirror = Duplica { $display }
    .project = Proietta verso { $display ->
        [all] tutti gli schermi
        *[other] { $display }
    }
    .project-count = Proiettare verso { $count} { $count ->
        [1] schermo
        *[other] schermi
    }

night-light = Luce Notturna
    .auto = Automatica (dall'alba al tramonto)
    .desc = Riduce la luce blu tramite colori più caldi.

orientation = Orientamento
    .landscape = Normale
    .landscape-flipped = Ruotato di 90°
    .portrait = Ruotato di 180°
    .portrait-flipped = Ruotato di 270°

scheduling = Pianificazione
    .manual = Pianificazione manuale

## Desktop: Notifications

notifications = Notifiche
    .desc = Non disturbare, notifiche blocco schermo, e impostazioni specifiche per applicazione.

## Desktop: Options

desktop-panel-options = Scrivania e Pannello
    .desc = Azione per il Tasto Super, angoli caldi, opzioni di controllo finestra.

desktop-panels-and-applets = Pannelli Scrivania e Applet

dock = Barra applicazioni
    .desc = Pannello con le applicazioni preferite.

hot-corner = Angolo Caldo
    .top-left-corner = Abilita angolo caldo in alto a sinistra per spazi di lavoro

super-key-action = Azione per il Tasto Super
    .launcher = Avviatore
    .workspaces = Spazi di lavoro
    .applications = Applicazioni

top-panel = Pannello Superiore
    .workspaces = Tasto Mostra Spazi di Lavoro
    .applications = Tasto Mostra Applicazioni

window-controls = Controlli Finestra
    .minimize = Mostra Tasto Minimizza
    .maximize = Mostra tasto Massimizza

## Desktop: Panel
panel = Pannello
    .desc = Barra superiore con controlli scrivania e menu.

add = Aggiungi
add-applet = Aggiungi Applet
all = Tutte
applets = Applet
center-segment = Segmento Centrale
drop-here = Rilascia applet qui
end-segment = Segmento Finale
large = Grande
no-applets-found = Nessun applet trovato...
panel-bottom = Basso
panel-left = Sinistra
panel-right = Destra
panel-top = Alto
search-applets = Cerca applet...
small = Piccolo
start-segment = Segmento Iniziale

panel-appearance = Aspetto
    .match = Abbina alla scrivania
    .light = Chiaro
    .dark = Scuro

panel-behavior-and-position = Comportamento e Posizioni
    .autohide = Nascondi pannello automaticamente
    .dock-autohide = Nascondi automaticamente barra applicazioni
    .position = Posizione sullo schermo
    .display = Mostra su schermo

panel-style = Stile
    .anchor-gap = Spazio tra il pannello e bordi dello schermo
    .dock-anchor-gap = Spazio tra la barra applicazioni e i bordi dello schermo
    .extend = Estendi pannello fino ai bordi dello schermo
    .dock-extend = Espandi la barra applicazioni fino ai bordi dello schermo
    .appearance = Aspetto
    .size = Dimensioni
    .background-opacity = Opacità dello sfondo

panel-applets = Configurazione
    .dock-desc = Configura gli applet della barra applicazioni.
    .desc = Configura applets del pannello.

panel-missing = Configurazione Pannello non trovata
    .desc = Il file di configurazione del pannello non è stato trovato a causa dell'utilizzo di una configurazione personalizzata oppure si è corrotto.
    .fix = Reimposta predefinito

## Desktop: Wallpaper

wallpaper = Sfondo
    .change = Cambia immagine ogni
    .desc = Immagini di sfondo, colori e opzioni presentazione.
    .fit = Adatta sfondo
    .folder-dialog = Cambia la cartella degli sfondi
    .image-dialog = Scegli l'immagine di sfondo
    .plural = Sfondi
    .same = Stesso sfondo su tutti gli schermi
    .slide = Presentazione

add-color = Aggiungi colore
add-image = Aggiungi immagine
all-displays = Tutti gli Schermi
colors = Colori
dialog-add = _Aggiungi
fill = Fill
fit-to-screen = Adatta allo Schermo
open-new-folder = Apri nuova cartella
recent-folders = Cartelle recenti

x-minutes = { $number } minuti
x-hours = { $number ->
    [1] 1 ora
    *[other] { $number } ore
}

## Desktop: Workspaces

workspaces = Spazi di lavoro
    .desc = Imposta numero, comportamento e posizione degli spazi di lavoro.

workspaces-behavior = Comportamento spazi di lavoro
    .dynamic = Spazi di lavoro dinamici
    .dynamic-desc = Rimuovi automaticamente spazi di lavoro vuoti.
    .fixed = Numero fisso di spazi di lavoro
    .fixed-desc = Aggiungi o rimuovi spazi di lavoro nell'anteprima.

workspaces-multi-behavior = Comportamento con più schermi
    .span = Gli spazi di lavoro comprendono più schermi
    .separate = Ogni schermo ha il suo spazio di lavoro

workspaces-overview-thumbnails = Anteprime Spazio di Lavoro
    .show-number = Mostra Numero Spazio di Lavoro
    .show-name = Mostra Nome Spazio di Lavoro

workspaces-orientation = Orientamento Spazi di Lavoro
    .vertical = Verticale
    .horizontal = Orizzontale

## Networking: Wired

wired = Cablata
    .desc = Connessione cablata, profili di connessione

## Networking: Online Accounts

online-accounts = Account Online
    .desc = Aggiungi account, IMAP e SMTP, login aziendali

## Time & Language

time = Ora e Lingua
    .desc = N/A

time-date = Data e ora
    .desc = Fuso orario, impostazioni orologio automatiche, e alcuni formati ora.
    .auto = Imposta automaticamente

time-zone = Fuso orario
    .auto = Fuso orario automatico
    .auto-info = Richiede geolocalizzazione e accesso a internet

time-format = Formato data e ora
    .twenty-four = formato 24 ore
    .first = Primo giorno della settimana

time-region = Regione e Lingua
    .desc = Formato date, ora e numeri in base alla tua regione

## Sound

sound = Suono
    .desc = N/A

sound-output = Uscita
    .volume = Volume in uscita
    .device = Dispositivo in uscita
    .level = Livello in uscita
    .config = Configurazione
    .balance = Bilanciamento

sound-input = Ingresso
    .volume = Volume in ingresso
    .device = Dispositivo in ingresso
    .level = Livello in ingresso

sound-alerts = Notifiche
    .volume = Volume notifiche
    .sound = Suono notifiche

sound-applications = Applicazioni
    .desc = Volume applicazioni e impostazioni

## System

system = Sistema e Account

## System: About

about = Descrizione
    .desc = Nome dispositivo, informazioni hardware, predefiniti sistema operativo.

about-device = Nome dispositivo
    .desc = Questo nome appare a altri dispositivi in rete o via bluetooth.

about-hardware = Hardware
    .model = Modello hardware
    .memory = RAM
    .processor = Processore
    .graphics = Scheda grafica
    .disk-capacity = Memoria di massa

about-os = Sistema Operativo
    .os = Sistema Operativo
    .os-architecture = Architettura del sistema operativo
    .desktop-environment = Ambiente grafico
    .windowing-system = Gestore grafico

about-related = Impostazioni correlate
    .support = Chiedi aiuto

## System: Firmware

firmware = Firmware
    .desc = Dettagli firmware.

## System: Users

users = Utenti
    .desc = Autenticazione e login, blocco schermo.

## Input

acceleration-desc = Aggiusta automaticamente la sensibilità di tracciamento in base alla velocità.

disable-while-typing = Disabilita mentre si scrive

input-devices = Dispositivi di Input
    .desc = Dispositivi di Input

primary-button = Tasto principale
    .left = Sinistro
    .right = Destro

scrolling = Scorrimento
    .two-finger = Scorri con due dita
    .edge = Scorri lungo il bordo con un dito
    .speed = Velocità di scorrimento
    .natural = Scorrimento naturale
    .natural-desc = Scorri il contenuto invece che la visualizzazione

## Input: Keyboard

keyboard = Tastiera
    .desc = Input da tastiera

keyboard-sources = Sorgenti Input
    .desc = Si può saltare tra le varie sorgenti input utilizzando la combinazione di tasti Super+Spazio. Questa può essere configurata nelle impostazioni sulle scorciatoie di sistema.
    .move-up = Muovi su
    .move-down = Muovi giù
    .settings = Impostazioni
    .view-layout = Visualizza la disposizione della tastiera
    .remove = Rimuovi

keyboard-special-char = Inserimento Caratteri Speciali
    .alternate = Tasto per i caratteri alternativi
    .compose = Tasto di composizione

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Scorciatoie da Tastiera
    .desc = Vedi e personalizza le scorciatoie

## Input: Mouse

mouse = Mouse
    .desc = Velocità, accelerazione, scorrimento naturale del mouse.
    .speed = Velocità mouse
    .acceleration = Abilita accelerazione mouse

## Input: Touchpad

click-behavior = Comportamento sul Click
    .click-finger = Click secondario con due dita e tasto centrale con tre dita
    .button-areas = Click secondario nell'angolo in basso a destra e tasto centrale in basso al centro

pinch-to-zoom = Pizzica per Ingrandire
    .desc = Utilizza due dita per ingrandire il contenuto, per applicazione che supportano l'ingrandimento.

tap-to-click = Tocca per fare click
    .desc = Abilita tocco con un dito per il click principale, due dita per il click secondario e tre dita per la pressione del tasto centrale.

touchpad = Touchpad
    .acceleration = Abilita accelerazione sul touchpad
    .desc = Velocità, opzioni click e gesti sul touchpad.
    .speed = Velocità sul touchpad

## Input: Gestures

swiping = Trascinamenti
    .four-finger-down = Trascina verso il basso con quattro dita
    .four-finger-left = Trascina verso sinistra con quattro dita
    .four-finger-right = Trascina verso destra con quattro dita
    .four-finger-up = Trascina verso l'altro con quattro dita
    .three-finger-any = Trascina verso qualsiasi direzione con tre dita

switch-between-windows = Scorri tra le finestre
switch-to-next-workspace = Passa al prossimo spazio di lavoro
switch-to-prev-workspace = Passa al precedente spazio di lavoro
open-application-library = Apri Libreria Applicazioni
open-workspaces-view = Apri Panoramica Spazi di Lavoro
