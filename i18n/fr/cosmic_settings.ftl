app = Paramètres COSMIC

unknown = Inconnu

number = { $number }

## Desktop

desktop = Bureau

## Desktop: Appearance

appearance = Apparence
    .desc = Couleur d'accentuation et personalisation COSMIC.

accent-color = Couleur d'accentuation
app-background = Application ou arrière plan de fenêtre
auto = Auto
close = Fermer
color-picker = Pipette à couleurs
copied-to-clipboard = Copié dans le presse-pappier
copy-to-clipboard = Copier dans le presse-pappier
dark = Sombre
export = Exporter
hex = Hex
import = Importer
light = Clair
mode-and-colors = Mode et Couleur
recent-colors = Couleurs récentes
reset-default = Réinitialiser
reset-to-default = Réinitialiser
rgb = RVB
window-hint-accent = Couleur de l'indice de la fenêtre active Active window hint color
window-hint-accent-toggle = Utiliser la couleur d'accentuation comme indice de fenêtre active

auto-switch = Passer automatiquement du mode clair au mode sombre
    .desc = Passe en mode Lumière au lever du soleil

container-background = Fond de conteneur
    .desc-detail = La couleur d’arrière-plan du conteneur est utilisée pour la barre latérale de navigation, le tiroir latéral, les boîtes de dialogue et les widgets similaires. Par défaut, il est automatiquement dérivé de l'arrière-plan de l'application ou de la fenêtre.
    .reset = Réinitialiser en mode automatique
    .desc = La couleur principale du conteneur est utilisée pour la barre latérale de navigation, le tiroir latéral, les boîtes de dialogue et les widgets similaires.

control-tint = Teinte des composants de contrôle
    .desc = Utilisé pour les arrière-plans des boutons standard, les entrées de recherche, les entrées de texte et les composants similaires.

frosted = Effet verre dépoli sur l'interface système
    .desc = Applique un flou d'arrière-plan au panneau, au dock, aux applets, au lanceur et à la bibliothèque d'applications.

text-tint = Teinte du texte de l'interface
    .desc = Couleur utilisée pour dériver des couleurs de texte d'interface présentant un contraste suffisant sur diverses surfaces.

style = Style
    .round = Rond
    .slightly-round = Légèrement rond
    .square = Carré

# interface density left out for now
window-management = Gestion des fenêtres
    .active-hint = Taille de l'indice de la fenêtre active
    .gaps = Gaps around tiled windows

## Desktop: Display

-requires-restart = Nécessite un redémarrage

color = Couleur
    .depth = Profondeur de la couleur
    .profile = Profil de couleur
    .sidebar = Profils de couleurs
    .temperature = Température de la couleur

display = Displays
    .desc = Manage displays, graphics switching, and night light
    .arrangement = Display Arrangement
    .arrangement-desc = Drag displays to rearrange them.
    .enable = Enable display
    .external = { $size } { $output } External Display
    .laptop = { $size } Laptop Display
    .options = Display Options
    .refresh-rate = Refresh rate
    .resolution = Resolution
    .scale = Scale

graphics-mode = Graphics mode
    .mode = { $mode ->
        [compute] Compute
        *[hybrid] Hybrid
        [integrated] Integrated
        [nvidia] NVIDIA
    } graphics
    .enable = Enable { $mode ->
        [compute] compute
        *[hybrid] hybrid
        [integrated] integrated
        [nvidia] NVIDIA
    } graphics
    .desc = { $mode ->
        [compute] Uses dedicated graphics for computational workloads only. Disables external displays. { -requires-restart }.
        *[hybrid] Applications use integrated graphics unless explicitly requested to use dedicated graphics. { -requires-restart }.
        [integrated] Turns off dedicated graphics for a longer battery life and less fan noise.
        [nvidia] Better graphical experience and highest power usage. { -requires-restart }.
    }
    .restart = Restart and switch to { $mode }?
    .restart-desc = Switching to { $mode } will close all open applications

mirroring = Mirroring
    .id = Mirroring { $id }
    .dont = Don't mirror
    .mirror = Mirror { $display }
    .project = Project to { $display ->
        [all] all displays
        *[other] { $display }
    }
    .project-count = Projecting to { $count} other { $count ->
        [1] display
        *[other] displays
    }

night-light = Night Light
    .auto = Automatic (sunset to sunrise)
    .desc = Reduce blue light with warmer colors.

scheduling = Scheduling
    .manual = Manual schedule

## Desktop: Notifications

notifications = Notifications
    .desc = Ne pas déranger, notifications sur l'écran de vérouillage, et paramètres par application.

## Desktop: Options

desktop-panel-options = Options Bureau
    .desc = Action de la Touche Super, coins actifs, options du contrôle des fenêtres.

desktop-panels-and-applets = Barres du bureau et Applets

dock = Dock
    .desc = Barre avec des applications épinglées.

hot-corner = Coins Actifs
    .top-left-corner = Activer le coin actif supérieur gauche pour les Espaces de Travail

super-key-action = Action Touche Super
    .launcher = Lanceur
    .workspaces = Espaces de Travail
    .applications = Applications

top-panel = Barre d'Applets
    .workspaces = Afficher le button Espace de Travail
    .applications = Afficher le button Application

window-controls = Contrôles Fenêtre
    .minimize = Afficher le bouton Minimiser
    .maximize = Afficher le Bouton Maximiser

## Desktop: Panel

panel = Barre d'Applets
    .desc = Barre customizable avec différent applets

add = Ajouter
add-applet = Ajouter Applet
all = Tous
applets = Applets
center-segment = Center Segment
drop-here = Drop applets here
end-segment = End Segment
large = Large
no-applets-found = Aucun applets trouvé...
panel-bottom = Bas
panel-left = Gauche
panel-right = Droite
panel-top = Haut
search-applets = Rechercher des applets...
small = Petit
start-segment = Start Segment

panel-appearance = Apparence
    .match = Match desktop
    .light = Clair
    .dark = Sombre

panel-behavior-and-position = Comportement et Positions
    .autohide = Cacher automatiquement
    .dock-autohide = Cacher automatiquement
    .position = Position sur l'écran
    .display = Afficher sur l'écran

panel-style = Style
    .anchor-gap = Espace entre la Barre d'Applets et les bords de l'écran
    .dock-anchor-gap = Espace entre le Dock et les bords de l'écran
    .extend = Étendre la Barre d'Applets jusqu'aux bords de l'écran
    .dock-extend = Étendre le dock jusqu'aux bords de l'écran
    .appearance = Apparence
    .size = Taille
    .background-opacity = Opacité de l'arrière-plan

panel-applets = Configuration
    .dock-desc = Configuration des applets de la barre d'Applets.
    .desc = Configuration des applets du Dock.

panel-missing = Panel Configuration is Missing
    .desc = The panel configuration file is missing due to use of a custom configuration or it is corrupted.
    .fix = Reset to default

## Desktop: Wallpaper

wallpaper = Wallpaper
    .change = Change image every
    .desc = Wallpaper images, colors, and slideshow options.
    .fit = Wallpaper fit
    .folder-dialog = Choose wallpaper folder
    .image-dialog = Choose wallpaper image
    .plural = Wallpapers
    .same = Same wallpaper on all displays
    .slide = Slideshow

add-color = Add color
add-image = Add image
all-displays = All Displays
colors = Colors
dialog-add = _Add
fill = Fill
fit-to-screen = Fit to Screen
open-new-folder = Open new folder
recent-folders = Recent Folders

x-minutes = { $number } minutes
x-hours = { $number ->
    [1] 1 hour
    *[other] { $number } hours
}

## Desktop: Workspaces

workspaces = Espaces de travail
    .desc = Définir le nombre, le comportement et le placement d'espaces de travail.

workspaces-behavior = Comportement des espaces de travail
    .dynamic = Espaces de travail dynamique
    .dynamic-desc = Supprime automatiquement les espaces de travail vides.
    .fixed = Nombre fixe d'espaces de travail
    .fixed-desc = Supprimer ou ajouter des espaces de travail dans l'aperçu.

workspaces-multi-behavior = Comportement Multi-écran
    .span = Workspaces Span Displays
    .separate = Displays Have Separate Workspaces

workspaces-overview-thumbnails = Workspace Overview Thumbnails
    .show-number = Show Workspace Number
    .show-name = Show Workspace Name

workspaces-orientation = Orientation des Espaces de Travail
    .vertical = Vertical
    .horizontal = Horizontal

## Networking: Wired

wired = Filaire
    .desc = Connexion filaire, profil de connexion

## Networking: Online Accounts

online-accounts = Comptes en lignes
    .desc = Ajouter des comptes, IMAP et SMTP, connexion d'entreprise

## Time & Language

time = Heure et Language
    .desc = N/A

time-date = Date et Heure
    .desc = Fuseau horaire, réglage de l'heure automatique, et format de l'heure.
    .auto = Régler automatiquement

time-zone = Fuseau Horaire
    .auto = Fuseau horaire automatique
    .auto-info = Nécessite les services de localisation et l'accès à internet

time-format = Format date et heure
    .twenty-four = 24 heures
    .first = Premiers jour de la semaine

time-region = Région et Language
    .desc = Format de dates, heures, et nombres basés sur votre région

## Sound

sound = Son
    .desc = N/A

sound-output = Sortie
    .volume = Volume de sortie
    .device = Périphérique de sortie
    .level = Niveau de sortie
    .config = Configuration
    .balance = Balance

sound-input = Entrée
    .volume = Volume d'entrée
    .device = Périphérique d'entrée
    .level = Niveau d'entrée

sound-alerts = Alertes
    .volume = Volumes des alertes
    .sound = Alertes sonores

sound-applications = Applications
    .desc = Volumes et paramètres d'application

## System

system = Système et Comptes

## System: About

about = À propos
    .desc = Nom de l'appareil, information matériel, paramètres par défaut du système d'exploitation.

about-device = Nom de l'appareil
    .desc = Ce nom apparaît aux autres périphériques réseau ou bluetooth.

about-hardware = Matériel
    .model = Nom du matériel
    .memory = Mémoire vive
    .processor = Processeur
    .graphics = Carte graphique
    .disk-capacity = Capacité du disque

about-os = Système d'exploitation
    .os = Système d'exploitation
    .os-architecture = Architecture du système d'exploitation
    .desktop-environment = Environnement de bureau
    .windowing-system = Système de fenêtrage

about-related = Paramètres associés
    .support = Obtenir de l'aide

## System: Firmware

firmware = Micrologiciel
    .desc = Détails micrologiciel.

## System: Users

users = Utilisateurs
    .desc = Authentication et connexion, écran de verrouillage.

## Input

acceleration-desc = Automatically adjusts tracking sensitivity based on speed.

disable-while-typing = Disable while typing

input-devices = Input Devices
    .desc = Input Devices

primary-button = Boutton principal
    .left = Gauche
    .right = Droite

scrolling = Défilement
    .speed = Vitesse de défilement
    .natural = Défilement naturel
    .natural-desc = Le défilement déplace le contenu, pas la vue

## Input: Keyboard

keyboard = Clavier
    .desc = Keyboard input

keyboard-sources = Input Sources
    .desc = Input sources can be switched using Super+Space key combination. This can be customized in the keyboard shortcut settings.
    .move-up = Remonter
    .move-down = Descendre
    .settings = Paramètre
    .view-layout = Afficher l'agencement du clavier
    .remove = Supprimer

keyboard-special-char = Special Character Entry
    .alternate = Alternate characters key
    .compose = Compose key

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Keyboard Shortcuts
    .desc = View and customize shortcuts

## Input: Mouse

mouse = Mouse
    .desc = Mouse speed, acceleration, natural scrolling.
    .speed = Mouse speed
    .acceleration = Enable mouse acceleration

## Input: Touchpad

## Input: Gestures

swiping = Swiping
    .four-finger-down = Four-finger swipe down
    .four-finger-left = Four-finger swipe left
    .four-finger-right = Four-finger swipe right
    .four-finger-up = Four-finger swipe up
    .three-finger-any = Three-finger swip any direction

switch-between-windows = Switch between windows
switch-to-next-workspace = Switch to next workspace
switch-to-prev-workspace = Switch to prev workspace
open-application-library = Open Application Library
open-workspaces-view = Open Workspaces Overview