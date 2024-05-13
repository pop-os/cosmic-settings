app = Paramètres COSMIC

unknown = Inconnu

number = { $number }

## Desktop

desktop = Bureau

## Desktop: Appearance

appearance = Apparence
    .desc = Couleur d'accentuation et personnalisation COSMIC.

accent-color = Couleur d'accentuation
app-background = Application ou arrière plan de fenêtre
auto = Auto
close = Fermer
color-picker = Pipette à couleurs
copied-to-clipboard = Copié dans le presse-papier
copy-to-clipboard = Copier dans le presse-papier
dark = Sombre
export = Exporter
hex = Hex
import = Importer
light = Clair
mode-and-colors = Mode et Couleur
recent-colors = Couleurs récentes
reset-to-default = Réinitialiser aux paramètres par défaut
rgb = RVB
window-hint-accent = Couleur d'indice de fenêtre active
window-hint-accent-toggle = Utiliser la couleur d'accentuation du thème comme indice de fenêtre active

auto-switch = Passer automatiquement du mode clair au mode sombre
    .sunrise = Passe au mode clair au lever du soleil
    .sunset = Passe au mode clair au coucher du soleil
    .next-sunrise = Passe au mode clair au prochain lever du soleil
    .next-sunset = Passe au mode clair au prochain coucher du soleil

container-background = Fond de conteneur
    .desc-detail = La couleur d’arrière-plan du conteneur est utilisée pour la barre latérale de navigation, le tiroir latéral, les boîtes de dialogue et les widgets similaires. Par défaut, il est automatiquement dérivé de l'arrière-plan de l'application ou de la fenêtre.
    .reset = Réinitialiser en mode automatique
    .desc = La couleur principale du conteneur est utilisée pour la barre latérale de navigation, le tiroir latéral, les boîtes de dialogue et les widgets similaires.

control-tint = Teinte des composants de contrôle
    .desc = Utilisé pour les arrière-plans des boutons standard, les entrées de recherche, les entrées de texte et les composants similaires.

frosted = Effet verre dépoli sur l'interface système
    .desc = Applique un flou d'arrière-plan au panneau, au dock, aux applets, au lanceur et à la bibliothèque d'applications.

enable-export = Appliquer ce thème aux applications GNOME.
    .desc = Pas tous les toolkits ne supportent le changement automatique de thème. Les applications non-COSMIC peuvent nécessiter un redémarrage après un changement de thème.

icon-theme = Thème d'icônes
    .desc = Applique un set d'icônes différent aux applications.

text-tint = Teinte du texte de l'interface
    .desc = Couleur utilisée pour dériver les couleurs de texte d'interface présentant un contraste suffisant sur différentes surfaces.

style = Style
    .round = Rond
    .slightly-round = Légèrement rond
    .square = Carré

# interface density left out for now
window-management = Gestion des fenêtres
    .active-hint = Taille de l'indice de fenêtre active
    .gaps = Espaces entre les fenêtres en mosaïque

## Desktop: Display

-requires-restart = Nécessite un redémarrage

color = Couleur
    .depth = Profondeur de couleur
    .profile = Profil de couleur
    .sidebar = Profils de couleurs
    .temperature = Température de couleur

display = Écrans
    .desc = Gérer les écrans, les modes graphiques, et le mode nuit
    .arrangement = Disposition des écrans
    .arrangement-desc = Faire glisser les écrans pour les réorganiser.
    .enable = Activer écran
    .external = { $size } { $output } Écran externe
    .laptop = { $size } Écran de l'ordinateur portable
    .options = Options d'écran
    .refresh-rate = Fréquence de rafraîchissement
    .resolution = Résolution
    .scale = Échelle

mirroring = Duplication de l'écran
    .id = Duplication de { $id }
    .dont = Ne pas dupliquer
    .mirror = Dupliquer { $display }
    .project = Projeter vers { $display ->
        [all] tous les écrans
        *[other] { $display }
    }
    .project-count = Projection sur { $count} { $count ->
        [1] autre écran
        *[other] autres écrans
    }

night-light = Mode nuit
    .auto = Automatique (du coucher au lever du soleil)
    .desc = Réduire la lumière bleue avec des couleurs plus chaudes.

orientation = Orientation
    .standard = Standard
    .rotate-90 = Rotation à 90 degrés
    .rotate-180 = Rotation à 180 degrés
    .rotate-270 = Rotation à 270 degrés

scheduling = Programmation
    .manual = Programmation manuelle

## Desktop: Notifications

notifications = Notifications
    .desc = Ne pas déranger, notifications sur l'écran de vérouillage, et paramètres par application.

## Desktop: Options

desktop-panel-options = Bureau et Panneau
    .desc = Action de la touche Super, coin actif, options du contrôle des fenêtres.

desktop-panels-and-applets = Barres du bureau et Applets

dock = Dock
    .desc = Barre avec des applications épinglées.

hot-corner = Coin Actif
    .top-left-corner = Activer le coin actif supérieur gauche pour les Espaces de Travail

super-key-action = Action de la touche Super
    .launcher = Lanceur
    .workspaces = Espaces de Travail
    .applications = Applications

top-panel = Panneau supérieur
    .workspaces = Afficher le bouton Espace de Travail
    .applications = Afficher le bouton Applications

window-controls = Contrôles Fenêtre
    .minimize = Afficher le bouton Minimiser
    .maximize = Afficher le bouton Maximiser

## Desktop: Panel

panel = Panneau
    .desc = Barre supérieure avec commandes du bureau et menus.

add = Ajouter
add-applet = Ajouter Applet
all = Tous
applets = Applets
center-segment = Segment central
drop-here = Déposer des applets ici
end-segment = Segment de fin
large = Large
no-applets-found = Aucun applet trouvé...
panel-bottom = Bas
panel-left = Gauche
panel-right = Droite
panel-top = Haut
search-applets = Rechercher des applets...
small = Petit
start-segment = Segment de début

panel-appearance = Apparence
    .match = Assortir au bureau
    .light = Clair
    .dark = Sombre

panel-behavior-and-position = Comportement et Positions
    .autohide = Masquer automatiquement le panneau
    .dock-autohide = Masquer automatiquement le dock
    .position = Position sur l'écran
    .display = Afficher à l'écran

panel-style = Style
    .anchor-gap = Espace entre le panneau et les bords de l'écran
    .dock-anchor-gap = Espace entre le dock et les bords de l'écran
    .extend = Étendre le panneau jusqu'aux bords de l'écran
    .dock-extend = Étendre le dock jusqu'aux bords de l'écran
    .appearance = Apparence
    .size = Taille
    .background-opacity = Opacité de l'arrière-plan

panel-applets = Configuration
    .dock-desc = Configuration des applets du dock.
    .desc = Configuration des applets du panneau.

panel-missing = La configuration du panneau est manquante
    .desc = Le fichier de configuration du panneau est manquant à cause d'une configuration personnalisée ou il est corrompu.
    .fix = Réinitialiser aux paramètres par défaut

## Desktop: Wallpaper

wallpaper = Fond d’écran
    .change = Changer l'image tous les
    .desc = Images de fond d’écran, couleurs et options de diaporama.
    .fit = Adapter le fond d’écran
    .folder-dialog = Choisir le dossier de fond d’écran
    .image-dialog = Choisir l'image de fond d’écran
    .plural = Fonds d’écran
    .same = Même fond d'écran sur tous les écrans
    .slide = Diaporama

add-color = Ajouter couleur
add-image = Ajouter image
all-displays = Tous les écrans
colors = Couleurs
dialog-add = _Ajouter
fill = Remplir
fit-to-screen = Adapter à l'écran
open-new-folder = Ouvrir nouveau dossier
recent-folders = Dossiers récents

x-minutes = { $number } minutes
x-hours = { $number ->
    [1] 1 heure
    *[other] { $number } heures
}

## Desktop: Workspaces

workspaces = Espaces de travail
    .desc = Définir le nombre, le comportement et l'emplacement d'espaces de travail.

workspaces-behavior = Comportement des espaces de travail
    .dynamic = Espaces de travail dynamiques
    .dynamic-desc = Supprime automatiquement les espaces de travail vides.
    .fixed = Nombre fixe d'espaces de travail
    .fixed-desc = Ajouter ou supprimer des espaces de travail dans l'aperçu.

workspaces-multi-behavior = Comportement multi-écrans
    .span = Les espaces de travail s'étendent sur tous les écrans
    .separate = Les écrans ont des espaces de travail séparés

workspaces-overview-thumbnails = Vignettes d'aperçu de l'espace de travail
    .show-number = Afficher le numéro de l'espace de travail
    .show-name = Afficher le nom de l'espace de travail

workspaces-orientation = Orientation des espaces de travail
    .vertical = Vertical
    .horizontal = Horizontal

## Networking: Wired

wired = Filaire
    .desc = Connexion filaire, profils de connexion

## Networking: Online Accounts

online-accounts = Comptes en lignes
    .desc = Ajouter des comptes, IMAP et SMTP, connexion d'entreprise

## Time & Language

time = Heure et Langue
    .desc = N/A

time-date = Date et Heure
    .desc = Fuseau horaire, réglage de l'heure automatique et format de l'heure.
    .auto = Régler automatiquement

time-zone = Fuseau Horaire
    .auto = Fuseau horaire automatique
    .auto-info = Nécessite les services de localisation et l'accès à internet

time-format = Format date et heure
    .twenty-four = Format 24 heures
    .first = Premier jour de la semaine
    .show-date = Afficher la date sur le panneau supérieur
    .friday = Vendredi
    .saturday = Samedi
    .sunday = Dimanche
    .monday = Lundi

time-region = Région et Langue
    .desc = Formater les dates, les heures et les nombres en fonction de votre région

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
    .model = Modèle du matériel
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
    .desc = Authentification et connexion, écran de verrouillage.

## Input

acceleration-desc = Ajuste automatiquement la sensibilité du suivi en fonction de la vitesse.

disable-while-typing = Désactiver pendant la saisie

input-devices = Périphériques d'entrée
    .desc = Périphériques d'entrée

primary-button = Boutton principal
    .left = Gauche
    .right = Droite

scrolling = Défilement
    .two-finger = Défiler avec deux doigts
    .edge = Défiler au bord avec un doigt
    .speed = Vitesse de défilement
    .natural = Défilement naturel
    .natural-desc = Le défilement déplace le contenu, pas la vue

## Input: Keyboard

keyboard = Clavier
    .desc = Saisie du clavier

keyboard-sources = Sources de saisie
    .desc = Les sources d'entrée peuvent être changées en utilisant la combinaison de touches Super+Espace. Cette combinaison peut être modifiée dans les paramètres de raccourci clavier.
    .move-up = Remonter
    .move-down = Descendre
    .settings = Paramètres
    .view-layout = Afficher l'agencement du clavier
    .remove = Supprimer

keyboard-special-char = Saisie de caractères spéciaux
    .alternate = Touche de caractères alternatifs
    .compose = Touche de composition

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Raccourcis clavier
    .desc = Voir et personnaliser les raccourcis

## Input: Mouse

mouse = Souris
    .desc = Vitesse de la souris, accélération, défilement naturel.
    .speed = Vitesse de la souris
    .acceleration = Activer l'accélération de la souris

## Input: Touchpad

click-behavior = Comportement de clic
    .click-finger = Clic secondaire avec deux doigts et clic du milieu avec trois doigts
    .button-areas = Clic secondaire dans le coin inférieur secondaire et clic du milieu dans la partie inférieure centrale

pinch-to-zoom = Pincer pour zoomer
    .desc = Utiliser deux doigts pour zoomer dans le contenu, pour les applications qui prennent en charge le zoom.

tap-to-click = Taper pour cliquer
    .desc = Active le tapotement à un doigt pour le clic principal, le tapotement à deux doigts pour le clic secondaire et le tapotement à trois doigts pour le clic du milieu.

touchpad = Pavé tactile
    .acceleration = Activer l'accélération du pavé tactile
    .desc = Vitesse du pavé tactile, options de clic, gestes.
    .speed = Vitesse du pavé tactile

## Input: Gestures

swiping = Gestes
    .four-finger-down = Balayage à quatre doigts vers le bas
    .four-finger-left = Balayage à quatre doigts vers la gauche
    .four-finger-right = Balayage à quatre doigts vers la droite
    .four-finger-up = Balayage à quatre doigts vers le haut
    .three-finger-any = Balayage à trois doigts dans n'importe quelle direction

switch-between-windows = Basculer entre les fenêtres
switch-to-next-workspace = Basculer vers le prochain espace de travail
switch-to-prev-workspace = Basculer vers l'espace de travail précédent
open-application-library = Ouvrir la bibliothèque d'applications
open-workspaces-view = Ouvrir l'aperçu des espaces de travail