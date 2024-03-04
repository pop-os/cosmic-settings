app = Impostazioni di COSMIC

unknown = Sconosciuto

number = { $number }

## Desktop

desktop = Desktop

## Desktop: Appearance

appearance = Aspetto
<<<<<<< HEAD
    .desc = Colore d'accento e tema COSMIC.
=======
    .desc = Colori d'accento e tema Cosmic.
>>>>>>> 0039464a6ba60a0461b8c4aa2713bf4556366c0a

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
reset-default = Ripristina predefinito
reset-to-default = Ripristina predefinito
rgb = RGB
window-hint-accent = Colore d'accento per la finestra attiva
window-hint-accent-toggle = Usa il colore del tema come colore d'accento

auto-switch = Cambia automaticamente del tema schiaro al tema scuro
    .desc = All'alba, cambia al tema Chiaro

container-background = Sfondo container
    .desc-detail = Il colore di sfondo del container viene usato per la barra laterale di navigazione, finestre di dialogo e widget simili. Il valore predefinito deriva dallo sfondo dell'applicazione o finestra.
    .reset = Ripristina in "Automatico"
    .desc = Il colore primario del container usato per la barra laterale, finestre di dialogo e vari widget.

control-tint = Tinta per componenti di controllo
    .desc = Usata come sfondo per i pulsanti, caselle di ricerca, caselle di testo e componenti simili.

frosted = Effetto "vetro smerigliato" per l'interfaccia
    .desc = Applica un effetto di sfocatura al pannello, alla barra delle applicazioni, gli applet, il launcher e la libreria della applicazioni.

text-tint = Tinta per il testo dell'interfaccia
    .desc = Colore su cui si basano i colori del testo con sufficiente contrasto in diverse superfici.

style = Stile
    .round = Arrotondato
    .slightly-round = Leggermente arrotondato
    .square = Spigoloso

# interface density left out for now
window-management = Gestione finestre
    .active-hint = Dimensione bordo finestra attiva
    .gaps = Spaziatura finestre in modalità tiling

## Desktop: Display

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

graphics-mode = Modalità GPU
    .mode = { $mode ->
        [compute] Compute
        *[hybrid] Ibrida
        [integrated] Integrata
        [nvidia] NVIDIA
    }
    .enable = Abilita modalità { $mode ->
        [compute] compute
        *[hybrid] ibrida
        [integrated] integrata
        [nvidia] NVIDIA
    }
    .desc = { $mode ->
        [compute] Usa la GPU dedicata solo per l'elaborazione. Disabilita gli schermi esterni. { -requires-restart }.
        *[hybrid] Le applicazioni useranno la GPU integrata a meno che non venga richiesto esplicitamente l'uso della GPU dedicata. { -requires-restart }.
        [integrated] Disattiva la GPU dedicata per una maggior durata della batteria e meno rumore proveniente dalle ventole.
        [nvidia] Miglior esperienza grafica ma maggior consumo energetico. { -requires-restart }.
    }
    .restart = Riavviare in modalità { $mode }?
    .restart-desc = Il riavvio in modalità { $mode } chiuderà tutte le applicazioni aperte.

mirroring = Duplicazione
    .id = Duplicazione { $id }
    .dont = Non duplicare
    .mirror = Duplica { $display }
    .project = Proietta su { $display ->
        [all] tutti gli schermi
        *[other] { $display }
    }
    .project-count = Proiettamento su { $count} { $count ->
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
<<<<<<< HEAD
    .desc = Non disturbare, notifiche nel blocco schermo e impostazioni per le applicazioni

## Desktop: Options

desktop-panel-options = Desktop e Pannello
    .desc = Azione tasto Super, bordi reattivi e opzioni controllo finestre.
=======
    .desc = Non disturbare, notifiche blocco schermo, e impostazioni specifiche per applicazione.

## Desktop: Options

desktop-panel-options = Scrivania e Pannello
    .desc = Azione per il Tasto Super, angoli caldi, opzioni di controllo finestra.
>>>>>>> 0039464a6ba60a0461b8c4aa2713bf4556366c0a

desktop-panels-and-applets = Pannello e applet

<<<<<<< HEAD
dock = Barra delle applicazioni
    .desc = Pannello contenente le applicazioni fissate.

hot-corner = Bordi reattivi
    .top-left-corner = Abilita bordo reattivo in alto a sinistra per gli spazi di lavoro
=======
dock = Barra applicazioni
    .desc = Pannello con le applicazioni preferite.

hot-corner = Angolo Caldo
    .top-left-corner = Abilita angolo caldo in alto a sinistra per spazi di lavoro
>>>>>>> 0039464a6ba60a0461b8c4aa2713bf4556366c0a

super-key-action = Azione tasto Super
    .launcher = Launcher
    .workspaces = Spazi di lavoro
    .applications = Applicazioni

<<<<<<< HEAD
top-panel = Pannello superiore
    .workspaces = Pulsante "mostra spazi di lavoro"
    .applications = Pulsante "mostra applicazioni"
=======
top-panel = Pannello Superiore
    .workspaces = Tasto Mostra Spazi di Lavoro
    .applications = Tasto Mostra Applicazioni
>>>>>>> 0039464a6ba60a0461b8c4aa2713bf4556366c0a

window-controls = Controlli finestra
    .minimize = Mostra pulsante "minimizza"
    .maximize = Mostra pulsante "massimizza"

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

<<<<<<< HEAD
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
    .dock-extend = Estendi la barra delle applicazioni ai bordi dello schermo
=======
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
>>>>>>> 0039464a6ba60a0461b8c4aa2713bf4556366c0a
    .appearance = Aspetto
    .size = Grandezza
    .background-opacity = Opacità sfondo

panel-applets = Configurazione
<<<<<<< HEAD
    .dock-desc = Configura gli applet della barra delle applicazioni
    .desc = Configura gli applet del pannello

panel-missing = La configurazione del pannello è mancante
    .desc = Il file di configurazione è corrotto o non è stato trovato per via di una configurazione non prevista
    .fix = Ripristina configurazione predefinita
=======
    .dock-desc = Configura gli applet della barra applicazioni.
    .desc = Configura applets del pannello.

panel-missing = Configurazione Pannello non trovata
    .desc = Il file di configurazione del pannello non è stato trovato a causa dell'utilizzo di una configurazione personalizzata oppure si è corrotto.
    .fix = Reimposta predefinito
>>>>>>> 0039464a6ba60a0461b8c4aa2713bf4556366c0a

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
dialog-add = _Aggiungi
<<<<<<< HEAD
fill = Riempi
fit-to-screen = Adatta allo schermo
=======
fill = Fill
fit-to-screen = Adatta allo Schermo
>>>>>>> 0039464a6ba60a0461b8c4aa2713bf4556366c0a
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

## Desktop: Workspaces

workspaces = Spazi di lavoro
    .desc = Imposta numero spazi di lavoro, il loro comportamento e la loro posizione.

workspaces-behavior = Comportamento spazi di lavoro
    .dynamic = Spazi di lavoro dinamici
<<<<<<< HEAD
    .dynamic-desc = Rimuovi automaticamente gli spazi di lavoro vuoti
    .fixed = Spazi di lavoro limitati
    .fixed-desc = Aggiungi o rimuovi gli spazi di lavoro
=======
    .dynamic-desc = Rimuovi automaticamente spazi di lavoro vuoti.
    .fixed = Numero fisso di spazi di lavoro
    .fixed-desc = Aggiungi o rimuovi spazi di lavoro nell'anteprima.
>>>>>>> 0039464a6ba60a0461b8c4aa2713bf4556366c0a

workspaces-multi-behavior = Comportamento multi-schermo
    .span = Gli spazi di lavoro si espandono in tutti gli schermi
    .separate = Ogni schermo ha gli spazi di lavoro separati

<<<<<<< HEAD
workspaces-overview-thumbnails = Anteprima spazi di lavoro
    .show-number = Mostra il numero dello spazio di lavoro
    .show-name = Mostra il nome dello spazio di lavoro
=======
workspaces-overview-thumbnails = Anteprime Spazio di Lavoro
    .show-number = Mostra Numero Spazio di Lavoro
    .show-name = Mostra Nome Spazio di Lavoro
>>>>>>> 0039464a6ba60a0461b8c4aa2713bf4556366c0a

workspaces-orientation = Orientamento spazi di lavoro
    .vertical = Verticale
    .horizontal = Orizzontale

## Networking: Wired

wired = Via cavo
    .desc = Connessione via cavo, profili di connessione

## Networking: Online Accounts

online-accounts = Account online
    .desc = Aggiungi account IMAP, SMTP e enterprise

## Time & Language

time = Orario e linguaggio
    .desc = N/A

time-date = Data e ora
    .desc = Fuso orario, impostazioni orologio automatico e formattazione orario
    .auto = Imposta automaticamente

time-zone = Fuso orario
    .auto = Fuso orario automatico
    .auto-info = Richiede i servizi di posizione e l'accesso a internet

time-format = Formato data e ora
    .twenty-four = Formato 24 ore
    .first = Primo giorno della settimana

time-region = Lingua e località
    .desc = Formattazione data, ora e numeri basata sulla località

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

## System

system = Sistema e account

## System: About

<<<<<<< HEAD
about = Informazioni
    .desc = Nome dispositivo, informazioni hardware e impostazioni predefinite di sistema

about-device = Nome dispositivo
    .desc = Ai dispositivi bluetooth, o a una rete, apparirà questo nome
=======
about = Descrizione
    .desc = Nome dispositivo, informazioni hardware, predefiniti sistema operativo.

about-device = Nome dispositivo
    .desc = Questo nome appare a altri dispositivi in rete o via bluetooth.
>>>>>>> 0039464a6ba60a0461b8c4aa2713bf4556366c0a

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

## Input

acceleration-desc = La velocità del puntatore viene basata sulla sua velocità

<<<<<<< HEAD
disable-while-typing = Disabilita durante la digitazione

input-devices = Dispositivi di immissione
    .desc = Dispositivi di immissione (come mouse e tastiera)

primary-button = Pulsante principale
=======
disable-while-typing = Disabilita mentre si scrive

input-devices = Dispositivi di Input
    .desc = Dispositivi di Input

primary-button = Tasto principale
>>>>>>> 0039464a6ba60a0461b8c4aa2713bf4556366c0a
    .left = Sinistro
    .right = Destro

scrolling = Scorrimento
<<<<<<< HEAD
    .two-finger = Scorrimento con due dita
    .edge = Scorrimento nel bordo con un dito
=======
    .two-finger = Scorri con due dita
    .edge = Scorri lungo il bordo con un dito
>>>>>>> 0039464a6ba60a0461b8c4aa2713bf4556366c0a
    .speed = Velocità di scorrimento
    .natural = Scorrimento naturale
    .natural-desc = Fa scorrere il contenuto invece che la visualizzazione

## Input: Keyboard

keyboard = Tastiera
    .desc = Immissione con la tastiera

keyboard-sources = Sorgenti di immissione
    .desc = Le sorgenti di immissione possono essere cambiate usando la combinazione Super + Spazio. Ciò può essere personalizzato nelle impostazioni delle scorciatorie da tastiera.
    .move-up = Sposta in alto
    .move-down = Sposta in basso
    .settings = Impostazioni
    .view-layout = Visualizza il layout della tastiera
    .remove = Rimuovi

keyboard-special-char = Immissione caratteri speciali
    .alternate = Tasto per i caratteri speciali
    .compose = Tasto di composizione

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Scorciatoie da tastiera
    .desc = Visualizza e modifica le scorciatoie

## Input: Mouse

mouse = Mouse
    .desc = Velocità mouse, accelerazione e scorrimento naturale
    .speed = Velocità mouse
    .acceleration = Abilita accelerazione mouse

## Input: Touchpad

<<<<<<< HEAD
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

swiping = Scorrimento
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
=======
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
>>>>>>> 0039464a6ba60a0461b8c4aa2713bf4556366c0a
