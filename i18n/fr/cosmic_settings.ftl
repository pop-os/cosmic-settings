app = Paramètres COSMIC

dbus-connection-error = Échec de la connexion à DBus
ok = OK
unknown = Inconnu

number = { $number }

## Networking & Wireless

connections-and-profiles = Connexions { $variant ->
    [wired] Filaires
    [wifi] Wi-Fi
    [vpn] VPN
    *[other] inconnues
} et profils.

add-network = Ajouter un réseau
    .profile = Ajouter un profil
add-vpn = Ajouter un VPN
airplane-on = Mode avion activé.
cable-unplugged = Câble débranché
connect = Se connecter
connected = Connecté
connecting = Connexion…
disconnect = Se déconnecter
forget = Oublier
known-networks = Réseaux connus
network-and-wireless = Réseau et sans fil
no-networks = Aucun réseau n'a été trouvé.
no-vpn = Pas de connexion VPN disponible.
password = Mot de passe
remove = Supprimer
settings = Paramètres
username = Nom d'utilisateur
visible-networks = Réseaux visibles

auth-dialog = Authentification requise
    .vpn-description = Entrez l'identifiant et le mot de passe demandé par le service de VPN.
    .wifi-description = Entrez le mot de passe ou la clé de chiffrement. Vous pouvez aussi vous connecter en pressant le bouton “WPS” sur votre box.

forget-dialog = Oublier ce réseau Wi-Fi ?
    .description = Vous aurez besoin de réentrer le mot de passe pour réutiliser ce réseau Wi-Fi.

network-device-state =
    .activated = Connecté au réseau
    .config = Connexion au réseau...
    .deactivating = Déconnexion du réseau...
    .disconnected = Déconnecté
    .failed = Connexion échouée
    .ip-check = Vérification de la connexion...
    .ip-config = En attente d'une IP et des informations de routage
    .need-auth = Requiert une authentification
    .prepare = Préparation de la connexion au réseau...
    .secondaries = En attente d'une connexion secondaire
    .unavailable = Indisponible
    .unknown = État inconnu
    .unmanaged = Non pris en charge
    .unplugged = Câble débranché

remove-connection-dialog = Supprimer le profil de connexion?
    .vpn-description = Vous aurez besoin de réentrer le mot de passe pour réutiliser ce réseau.
    .wired-description = Vous aurez besoin de recréer ce profil pour le réutiliser.

vpn = VPN
    .connections = Connexions VPN
    .error = Échec de l'ajout de la configuration VPN
    .remove = Supprimer le profil de connexion
    .select-file = Sélectionnez un fichier de configuration VPN

vpn-error = Erreur VPN
    .config = Échec de l'ajout de la configuration VPN
    .connect = Échec de la connexion au VPN
    .connection-editor = Échec de l'éditeur de connexion
    .connection-settings = Échec de l'obtention des paramètres des connexions actives
    .updating-state = Échec de la mise à jour de l'état du gestionnaire de réseau
    .wireguard-config-path = Chemin de fichier non valide pour la configuration WireGuard
    .wireguard-config-path-desc = Le fichier choisi doit se trouver sur un système de fichiers local.
    .wireguard-device = Échec de la création du périphérique WireGuard
    .with-password = Échec de la configuration du VPN { $field ->
        *[username] nom d'utilisateur
        [password] mot de passe
        [password-flags] indicateurs de mot de passe
    } avec nmcli

wired = Filaire
    .adapter = Adaptateur filaire { $id }
    .connections = Connexions filaires
    .devices = Périphériques câblés
    .remove = Supprimer un profil de connexion

wifi = Wi-Fi
    .adapter = Adaptateur Wi-Fi { $id }
    .forget = Oublier ce réseau

wireguard-dialog = Ajouter un périphérique WireGuard
    .description = Choisir un nom de périphérique pour la configuration WireGuard.

## Networking: Online Accounts

online-accounts = Comptes en lignes
    .desc = Ajouter des comptes, IMAP et SMTP, connexion d'entreprise

# Bluetooth

confirm = Confirmer

bluetooth = Bluetooth
    .desc = Gestion du Bluetooth.
    .status = Ce système est visible en tant que { $aliases } pendant que les paramètres Bluetooth sont ouvert.
    .connected = Connecté
    .connecting = Connexion
    .disconnecting = Déconnexion
    .connect = Connecter
    .disconnect = Déconnecter
    .forget = Oublier
    .dbus-error = Une erreur est survenue lors de l'interaction avec DBus: { $why }
    .show-device-without-name = Afficher les périphériques sans nom

bluetooth-paired = Périphériques précédemment connectés
    .connect = Connecter
    .battery = { $percentage }% de batterie

bluetooth-confirm-pin = Confirmer le code PIN Bluetooth
    .description = Veuillez confirmer que le code PIN suivant correspond à celui affiché sur { $device }

bluetooth-available = Périphériques à proximité

bluetooth-adapters = Adaptateur Bluetooth

## Desktop

desktop = Bureau

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
dialog-add = Ajouter
fill = Remplir
fit-to-screen = Adapter à l'écran
open-new-folder = Ouvrir nouveau dossier
recent-folders = Dossiers récents

x-minutes = { $number } minutes
x-hours = { $number ->
    [1] 1 heure
    *[other] { $number } heures
}
never = Jamais

## Desktop: Appearance

appearance = Apparence
    .desc = Couleur d'accentuation et personnalisation COSMIC.

accent-color = Couleur d'accentuation
app-background = Arrière-plan d'applications ou fenêtres
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
mode-and-colors = Mode et couleurs
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
    .desc-detail = La couleur d’arrière-plan du conteneur est utilisée pour la barre latérale de navigation, le panneau latéral, les boîtes de dialogue et les widgets similaires. Par défaut, il est automatiquement dérivé de l'arrière-plan de l'application ou de la fenêtre.
    .reset = Réinitialiser en mode automatique
    .desc = La couleur principale du conteneur est utilisée pour la barre latérale de navigation, le panneau latéral, les boîtes de dialogue et les widgets similaires.

control-tint = Teinte des composants de contrôle
    .desc = Utilisé pour les arrière-plans des boutons standard, les entrées de recherche, les entrées de texte et les composants similaires.

frosted = Effet de verre sur l'interface système
    .desc = Applique un effet de flou d'arrière-plan au panneau, au dock, aux applets, au lanceur et à la bibliothèque d'applications.

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

interface-density = Densité d'interface
    .comfortable = Confortable
    .compact = Compact
    .spacious = Spacieux

window-management-appearance = Gestion des fenêtres
    .active-hint = Taille de l'indice de fenêtre active
    .gaps = Espaces entre les fenêtres en mosaïque

### Experimental

experimental-settings = Paramètres expérimentaux
icons-and-toolkit = Thèmes des icônes et de la boîte à outils
interface-font = Police système
monospace-font = Police monospace

## Desktop: Notifications

notifications = Notifications
    .desc = Ne pas déranger, notifications sur l'écran de vérouillage, et paramètres par application.

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
    .dock-desc = Configuration des applets du dock
    .desc = Configuration des applets du panneau

panel-missing = La configuration du panneau est manquante
    .desc = Le fichier de configuration du panneau est manquant à cause d'une configuration personnalisée ou il est corrompu.
    .fix = Réinitialiser aux paramètres par défaut

## Desktop: Dock

dock = Dock
    .desc = Panneau avec des applications épinglées dans le plateau d'applications et d'autres applets.

## Desktop: Window management

window-management = Gestion des fenêtres
    .desc = Actions de la touche Super, options de contrôle des fenêtres et options supplémentaires de gestion des fenêtres.

super-key = Touche Super
    .launcher = Ouvrir le Lanceur
    .workspaces = Ouvrir les Espaces de travail
    .applications = Ouvrir les Applications
    .disable = Désactiver

window-controls = Contrôles des fenêtres
    .minimize = Afficher le bouton de réduction
    .maximize = Afficher le bouton de maximisation
    .active-window-hint = Afficher l'indice de la fenêtre active

focus-navigation = Focus Navigation
    .focus-follows-cursor = Le focus suit le curseur
    .focus-follows-cursor-delay = Délai de suivi du focus en ms
    .cursor-follows-focus = Le curseur suit le focus

## Desktop: Workspaces

workspaces = Espaces de travail
    .desc = Orientation et comportement des espaces de travail.

workspaces-behavior = Comportement des espaces de travail
    .dynamic = Espaces de travail dynamiques
    .dynamic-desc = Supprime automatiquement les espaces de travail vides.
    .fixed = Nombre fixe d'espaces de travail
    .fixed-desc = Ajouter ou supprimer des espaces de travail dans l'aperçu.

workspaces-multi-behavior = Comportement multi-écrans
    .span = Les espaces de travail s'étendent sur les écrans
    .separate = Les écrans ont des espaces de travail séparés

workspaces-overview-thumbnails = Vignettes de l'aperçu des espaces de travail
    .show-number = Afficher le numéro de l'espace de travail
    .show-name = Afficher le nom de l'espace de travail

workspaces-orientation = Orientation des espaces de travail
    .vertical = Verticale
    .horizontal = Horizontale

hot-corner = Coin actif
    .top-left-corner = Activer le coin actif en haut à gauche pour les espaces de travail

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

dialog = Dialogue
    .title = Conserver ces paramètres d'affichage ?
    .keep-changes = Conserver les modifications
    .change-prompt = Les modifications des paramètres reviendront automatiquement dans { $time } secondes.
    .revert-settings = Revenir aux paramètres

legacy-app-scaling = Mise à l'échelle des applications X11
    .scaled-by-system = Mettre à l'échelle toutes les applications X11
    .system-description = Les applications X11 seront floues sur les écrans HiDPI.
    .scaled-natively = Afficher toutes les applications X11 à la résolution native
    .native-description = Les applications X11 qui ne supportent pas la mise à l'échelle seront petites sur les écrans HiDPI. Activer pour que les jeux utilisent toute la résolution de l'écran.

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

profile = Profile

## Power

power = Énergie
  .desc = Gérer les paramètres d'alimentation

battery = Batterie
  .minute = { $value } { $value ->
        [one] minute
       *[other] minutes
  }
  .hour = { $value } { $value ->
        [one] heure
       *[other] heures
  }
  .day = { $value } { $value ->
        [one] jour
       *[other] jours
  }
  .less-than-minute = Moins d'une minute
  .and = et
  .remaining-time = { $time } jusqu'à la { $action ->
        [full] charge
       *[other] decharge
   } complète

connected-devices = Périphériques connectés
  .unknown = Périphériques inconnu

power-mode = Modes d'énergie
    .battery = Économie d'énergie
    .battery-desc = Performances réduites mais consommation d'énergie réduite.
    .balanced = Équilibré
    .balanced-desc = Performances et consommation d'énergie équilibré.
    .performance = Performance
    .performance-desc = Performances maximales mais force consommation d'énergie.
    .no-backend = Backend non trouvé. Installez system76-power ou power-profiles-daemon.

power-saving = Options d'économie d'énergie
    .turn-off-screen-after = Éteindre l'écran après
    .auto-suspend = Suspension automatique
    .auto-suspend-ac = Suspension automatique lors du branchement
    .auto-suspend-battery = Suspension automatique sur batterie

## Input

acceleration-desc = Ajuste automatiquement la sensibilité du suivi en fonction de la vitesse.

disable-while-typing = Désactiver pendant la saisie

input-devices = Périphériques d'entrée
    .desc = Périphériques d'entrée

primary-button = Bouton principal
    .desc = Définit l'ordre des boutons physiques.
    .left = Gauche
    .right = Droite

scrolling = Défilement
    .two-finger = Défiler avec deux doigts
    .edge = Défiler au bord avec un doigt
    .speed = Vitesse de défilement
    .natural = Défilement naturel
    .natural-desc = Le défilement déplace le contenu, pas la vue

## Input: Keyboard

slow = Lent
fast = Rapide
short = Court
long = Long
keyboard = Clavier
    .desc = Sources de saisie, commutation, saisie de caractères spéciaux, raccourcis.

keyboard-sources = Sources de saisie
    .desc = Les sources d'entrée peuvent être changées en utilisant la combinaison de touches Super+Espace. Cette combinaison peut être modifiée dans les paramètres de raccourci clavier.
    .move-up = Remonter
    .move-down = Descendre
    .settings = Paramètres
    .view-layout = Afficher l'agencement du clavier
    .remove = Supprimer
    .add = Ajouter une source de saisie

keyboard-special-char = Saisie de caractères spéciaux
    .alternate = Touche de caractères alternatifs
    .compose = Touche de composition
    .caps = Touche de verrouillage des majuscules

keyboard-typing-assist = Saisie
    .repeat-rate = Taux de répétition
    .repeat-delay = Délai de répétition

added = Ajouté
type-to-search = Tapez pour rechercher...
show-extended-input-sources = Afficher les sources d'entrée étendues

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Raccourcis clavier
    .desc = Voir et personnaliser les raccourcis

add-keybinding = Ajouter un raccourci
cancel = Annuler
command = Commande
custom = Personnalisé
debug = Déboguer
disabled = Désactivé
migrate-workspace-prev = Migrer l’espace de travail vers la sortie précédente
migrate-workspace-next = Migrer l’espace de travail vers la sortie suivante
migrate-workspace = Migrer l’espace de travail vers la sortie { $direction ->
    *[down] bas
    [left] gauche
    [right] droite
    [up] haut
}
navigate = Naviguer
replace = Remplacer
shortcut-name = Nom du raccourci
system-controls = Contrôles système
terminate = Terminer
toggle-stacking = Basculer l'empilement des fenêtres
type-key-combination = Taper la combinaison de touches

custom-shortcuts = Raccourcis personnalisés
    .add = Ajouter un raccourci
    .context = Ajouter un raccourci personnalisé
    .none = Aucun raccourci personnalisé

modified = { $count } modifiés

nav-shortcuts = Navigation
    .prev-output = Focus sur la sortie précédente
    .next-output = Focus sur la sortie suivante
    .last-workspace = Focus sur le dernier espace de travail
    .prev-workspace = Focus sur l’espace de travail précédent
    .next-workspace = Focus sur l’espace de travail suivant
    .focus = Focus sur la fenêtre { $direction ->
        *[down] bas
        [in] dedans
        [left] gauche
        [out] dehors
        [right] droite
        [up] haut
    }
    .output = Basculer à la sortie { $direction ->
        *[down] bas
        [left] gauche
        [right] droite
        [up] haut
    }
    .workspace = Basculer à l’espace de travail { $num }

manage-windows = Gérer les fenêtres
    .close = Fermer la fenêtre
    .maximize = Maximiser la fenêtre
    .minimize = Minimiser la fenêtre
    .resize-inwards = Redimensionner la fenêtre vers l’intérieur
    .resize-outwards = Redimensionner la fenêtre vers l’extérieur
    .toggle-sticky = Basculer la fenêtre collante

move-windows = Déplacer les fenêtres
    .direction = Déplacer la fenêtre { $direction ->
        *[down] bas
        [left] gauche
        [right] droite
        [up] haut
    }
    .display = Déplacer la fenêtre d'un écran { $direction ->
        *[down] bas
        [left] gauche
        [right] droite
        [up] haut
    }
    .workspace = Déplacer la fenêtre d’un espace de travail { $direction ->
        *[below] en dessous
        [left] gauche
        [right] droite
        [above] au-dessus
    }
    .workspace-num = Déplacer la fenêtre vers l’espace de travail { $num }
    .prev-workspace = Déplacer la fenêtre vers l’espace de travail précédent
    .next-workspace = Déplacer la fenêtre vers l’espace de travail suivant
    .last-workspace = Déplacer la fenêtre vers le dernier espace de travail
    .next-display = Déplacer la fenêtre vers l’écran suivant
    .prev-display = Déplacer la fenêtre vers l’écran précédent
    .send-to-prev-workspace = Déplacer la fenêtre vers l’espace de travail précédent
    .send-to-next-workspace = Déplacer la fenêtre vers l’espace de travail suivant

system-shortcut = Système
    .app-library = Ouvrir la bibliothèque d’applications
    .brightness-down = Réduire la luminosité de l'écran
    .brightness-up = Augmenter la luminosité de l'écran
    .home-folder = Ouvrir le dossier personnel
    .keyboard-brightness-down = Réduire la luminosité du clavier
    .keyboard-brightness-up = Augmenter la luminosité du clavier
    .launcher = Ouvrir le lanceur
    .lock-screen = Verrouiller l'écran
    .mute = Couper le son de la sortie audio
    .mute-mic = Couper le son du microphone
    .play-pause = Lecture/Pause
    .play-next = Piste suivante
    .play-prev = Piste précédente
    .screenshot = Prendre une capture d'écran
    .terminal = Ouvrir un terminal
    .volume-lower = Diminuer le volume de sortie audio
    .volume-raise = Augmenter le volume de sortie audio
    .web-browser = Ouvrir un navigateur web
    .window-switcher = Passer d'une fenêtre ouverte à l'autre
    .window-switcher-previous = Passer d'une fenêtre ouverte à l'autre inversée
    .workspace-overview = Ouvrir l’aperçu des espaces de travail

window-tiling = Tuilage des fenêtres
    .horizontal = Définir l'orientation horizontale
    .vertical = Définir l'orientation verticale
    .swap-window = Échanger la fenêtre
    .toggle-tiling = Basculer le tuilage des fenêtres
    .toggle-stacking = Basculer l'empilement des fenêtres
    .toggle-floating = Basculer la flottabilité des fenêtres
    .toggle-orientation = Basculer l'orientation

replace-shortcut-dialog = Remplacer le raccourci ?
    .desc = { $shortcut } est utilisé par { $name }. Si vous le remplacez, { $name } sera désactivé.

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

switch-workspaces = Basculer entre les espaces de travail
    .horizontal = Balayage à quatre doigts vers la gauche/droite
    .vertical = Balayage à quatre doigts vers le haut/bas

switch-between-windows = Basculer entre les fenêtres
open-application-library = Ouvrir la bibliothèque d'applications
open-workspaces-view = Ouvrir l'aperçu des espaces de travail

## Time & Language

time = Heure et Langue
    .desc = N/A

time-date = Date et Heure
    .desc = Fuseau horaire, réglage de l'heure automatique et format de l'heure.
    .auto = Régler automatiquement
    .auto-ntp = La date et l'heure seront mises à jour automatiquement lorsque le fuseau horaire sera défini.

time-zone = Fuseau Horaire
    .auto = Fuseau horaire automatique
    .auto-info = Nécessite les services de localisation et l'accès à internet

time-format = Format date et heure
    .twenty-four = Format 24 heures
    .show-seconds = Afficher les secondes
    .first = Premier jour de la semaine
    .show-date = Afficher la date sur le panneau supérieur
    .friday = Vendredi
    .saturday = Samedi
    .sunday = Dimanche
    .monday = Lundi

time-region = Région et Langue
    .desc = Formater les dates, les heures et les nombres en fonction de votre région.

formatting = Format
    .dates = Dates
    .time = Heure
    .date-and-time = Date et heure
    .numbers = Nombre
    .measurement = Mesure
    .paper = Papier

preferred-languages = Langues favorites
    .desc = L'ordre des langues détermine la langue utilisée. Les changements prendront effet à la prochaine connexion.

add-language = Ajouter une langue
    .context = Ajouter une Langue
install-additional-languages = Installer les langues ajoutées
region = Région

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
    .admin = Admin
    .standard = Standard
    .profile-add = Choisir l'image de profil

administrator = Administrateur
    .desc = Les administrateurs peuvent changer les paramètres de tous les utilisateurs, en ajouter de nouveaux et en supprimer.

add-user = Ajouter l'utilisateur
remove-user = Supprimer l'utilisateur
full-name = Nom complet

## System: Default Applications

default-apps = Applications par défaut
    .desc = Navigateur web, client mail, navigateur de fichiers, et autres applications.
    .web-browser = Navigateur web
    .file-manager = Navigateur de fichiers
    .mail-client = Client mail
    .music = Musique
    .video = Vidéo
    .photos = Photos
    .calendar = Calendrier
    .terminal = Terminal
    .other-associations = Autres associations
