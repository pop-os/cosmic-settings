app = Paramètres COSMIC
dbus-connection-error = Échec de la connexion à DBus
ok = OK
unknown = Inconnu
number = { $number }

## Networking & Wireless

connections-and-profiles =
    Connexions { $variant ->
        [wired] filaires
        [wifi] Wi-Fi
        [vpn] VPN
       *[other] inconnues
    } et profils.
add-network = Ajouter un réseau
    .profile = Ajouter un profil
add-vpn = Ajouter un VPN
airplane-on = Mode avion activé.
cable-unplugged = Câble débranché
connect = Connecter
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
    .vpn-description = Entrez l'identifiant et le mot de passe requis par le service VPN.
    .wifi-description = Entrez le mot de passe ou la clé de chiffrement. Vous pouvez aussi vous connecter en appuyant sur le bouton “WPS” de votre routeur.
forget-dialog = Oublier ce réseau Wi-Fi ?
    .description = Vous devrez à nouveau saisir un mot de passe pour utiliser ce réseau Wi-Fi à l'avenir.
network-device-state =
    .activated = Connecté
    .config = Connexion...
    .deactivating = Déconnexion...
    .disconnected = Déconnecté
    .failed = Connexion échouée
    .ip-check = Vérification de la connexion...
    .ip-config = En attente d'une IP et des informations de routage
    .need-auth = Requiert une authentification
    .prepare = Préparation de la connexion
    .secondaries = En attente d'une connexion secondaire
    .unavailable = Indisponible
    .unknown = État inconnu
    .unmanaged = Non pris en charge
    .unplugged = Câble débranché
remove-connection-dialog = Supprimer le profil de connexion ?
    .vpn-description = Vous aurez besoin d'entrer le mot de passe à nouveau pour réutiliser ce réseau.
    .wired-description = Vous aurez besoin de créer ce profil à nouveau pour le réutiliser.
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
    .wireguard-config-path = Chemin de fichier invalide pour la configuration WireGuard
    .wireguard-config-path-desc = Le fichier choisi doit se trouver sur un système de fichiers local.
    .wireguard-device = Échec de la création du périphérique WireGuard
    .with-password =
        Échec de la configuration du VPN { $field ->
           *[username] nom d'utilisateur
            [password] mot de passe
            [password-flags] indicateurs de mot de passe
        } avec nmcli
wired = Filaire
    .adapter = Adaptateur filaire { $id }
    .connections = Connexions filaires
    .devices = Périphériques filaires
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
    .desc = Gérer les périphériques Bluetooth
    .status = Ce système est visible en tant que { $aliases } tant que les paramètres Bluetooth sont ouverts.
    .connected = Connecté
    .connecting = Connexion
    .disconnecting = Déconnexion
    .connect = Connecter
    .disconnect = Déconnecter
    .forget = Oublier
    .dbus-error = Une erreur est survenue lors de l'interaction avec DBus : { $why }
    .disabled = Le service Bluetooth est désactivé
    .inactive = Le service Bluetooth est inactif
    .unknown = Le service Bluetooth n'a pas pu être activé. BlueZ est-il installé ?
bluetooth-paired = Périphériques précédemment connectés
    .connect = Connecter
    .battery = { $percentage }% de batterie
bluetooth-confirm-pin = Confirmer le code PIN Bluetooth
    .description = Veuillez confirmer que le code PIN suivant correspond à celui affiché sur { $device }
bluetooth-available = Périphériques à proximité
bluetooth-adapters = Adaptateurs Bluetooth

## Desktop

desktop = Bureau

## Desktop: Wallpaper

wallpaper = Fond d’écran
    .change = Changer l'image tous les
    .desc = Images de fond d’écran, couleurs et options de diaporama
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
x-minutes =
    { $number } { $number ->
        [one] minute
       *[other] minutes
    }
x-hours =
    { $number } { $number ->
        [one] heure
       *[other] heures
    }
never = Jamais

## Desktop: Appearance

appearance = Apparence
    .desc = Couleur d'accentuation et thème
accent-color = Couleur d'accentuation
app-background = Arrière-plan des fenêtres
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
    .sunset = Passe au mode sombre au coucher du soleil
    .next-sunrise = Passe au mode clair au prochain lever du soleil
    .next-sunset = Passe au mode sombre au prochain coucher du soleil
container-background = Arrière-plan
    .desc-detail = La couleur d’arrière-plan est utilisée pour la barre latérale de navigation, le panneau latéral, les boîtes de dialogue et les widgets similaires. Par défaut, elle est automatiquement dérivée de l'arrière-plan de l'application ou de la fenêtre.
    .reset = Réinitialiser en mode automatique
    .desc = La couleur principale de l'arrière-plan est utilisée pour la barre latérale de navigation, le panneau latéral, les boîtes de dialogue et les widgets similaires
control-tint = Teinte des composants de contrôle
    .desc = Utilisé pour les arrière-plans des boutons standards, les entrées de recherche, les entrées de texte et les composants similaires
frosted = Effet de verre sur l'interface système
    .desc = Applique un effet de flou d'arrière-plan au panneau, au dock, aux applets, au lanceur et à la bibliothèque d'applications
enable-export = Appliquer ce thème aux applications GNOME
    .desc = Certaines boîtes à outils ne supportent pas le changement automatique de thème. Les applications non-COSMIC peuvent nécessiter un redémarrage après un changement de thème.
icon-theme = Thème d'icônes
    .desc = Applique un set d'icônes différent aux applications
text-tint = Teinte du texte de l'interface
    .desc = Utilisée pour obtenir des couleurs de texte d'interface offrant un contraste suffisant sur différentes surfaces
style = Style
    .round = Rond
    .slightly-round = Légèrement rond
    .square = Carré
interface-density = Densité de l'interface
    .comfortable = Confortable
    .compact = Compact
    .spacious = Spacieux
window-management-appearance = Gestion des fenêtres
    .active-hint = Taille de l'indice de fenêtre active
    .gaps = Espaces entre les fenêtres agencées

### Experimental

experimental-settings = Paramètres expérimentaux
icons-and-toolkit = Thèmes des icônes et de la boîte à outils
interface-font = Police système
monospace-font = Police monospace

## Desktop: Notifications

notifications = Notifications
    .desc = Ne pas déranger, notifications sur l'écran de verrouillage, et paramètres par application

## Desktop: Panel

panel = Panneau
    .desc = Barre système principale pour les menus et les applets
add = Ajouter
add-applet = Ajouter applet
all = Tous
applets = Applets
center-segment = Segment central
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
panel-behavior-and-position = Comportement et positions
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
    .desc = Un panneau optionnel pour les applis et les applets

## Desktop: Window management

window-management = Gestion des fenêtres
    .desc = Actions de la touche Super, options de contrôle des fenêtres et options supplémentaires d'agencement des fenêtres
super-key = Action de la touche Super
    .launcher = Ouvrir le Lanceur
    .workspaces = Ouvrir les Espaces de travail
    .applications = Ouvrir les Applications
    .disable = Désactiver
window-controls = Contrôles des fenêtres
    .maximize = Afficher le bouton maximiser
    .minimize = Afficher le bouton minimiser
    .active-window-hint = Afficher l'indice de la fenêtre active
focus-navigation = Navigation par le focus
    .focus-follows-cursor = Le focus suit le curseur
    .focus-follows-cursor-delay = Délai de suivi du focus en ms
    .cursor-follows-focus = Le curseur suit le focus

## Desktop: Workspaces

workspaces = Espaces de travail
    .desc = Orientation et comportement des espaces de travail
workspaces-behavior = Comportement des espaces de travail
    .dynamic = Espaces de travail dynamiques
    .dynamic-desc = Supprime automatiquement les espaces de travail vides.
    .fixed = Nombre fixe d'espaces de travail
    .fixed-desc = Ajouter ou supprimer des espaces de travail dans l'aperçu.
workspaces-multi-behavior = Comportement multi-écrans
    .span = Les espaces de travail s'étendent sur les écrans
    .separate = Les écrans ont des espaces de travail séparés
workspaces-overview-thumbnails = Vignettes d'aperçu des espaces de travail
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
    .profile = Profil de couleurs
    .sidebar = Profils de couleurs
    .temperature = Température de couleur
display = Écrans
    .desc = Gérer les écrans, les modes graphiques et le mode nuit
    .arrangement = Disposition des écrans
    .arrangement-desc = Faire glisser les écrans pour les réorganiser
    .enable = Activer écran
    .external = { $size } { $output } écran externe
    .laptop = { $size } écran de l'ordinateur portable
    .options = Options d'écran
    .refresh-rate = Fréquence de rafraîchissement
    .resolution = Résolution
    .scale = Échelle
    .additional-scale-options = Options d'échelle supplémentaires
mirroring = Duplication de l'écran
    .id = Duplication de { $id }
    .dont = Ne pas dupliquer
    .mirror = Dupliquer { $display }
    .project =
        Projeter vers { $display ->
            [all] tous les écrans
           *[other] { $display }
        }
    .project-count =
        Projection sur { $count } { $count ->
            [1] autre écran
           *[other] autres écrans
        }
night-light = Mode nuit
    .auto = Automatique (du coucher au lever du soleil)
    .desc = Réduire la lumière bleue avec des couleurs plus chaudes
orientation = Orientation
    .standard = Standard
    .rotate-90 = Rotation à 90 degrés
    .rotate-180 = Rotation à 180 degrés
    .rotate-270 = Rotation à 270 degrés
scheduling = Programmation
    .manual = Programmation manuelle
dialog = Dialogue
    .title = Conserver ces paramètres d'affichage ?
    .keep-changes = Conserver les modifications
    .change-prompt = Les modifications seront annulées automatiquement dans { $time } secondes.
    .revert-settings = Annuler les modifications
legacy-app-scaling = Mise à l'échelle des applications système X11
    .scaled-gaming = Optimisé pour le gaming et les applications en plein écran
    .gaming-description = Les applications X11 peuvent apparaîtres légèrement plus grandes/plus petites comparées aux applications Wayland
    .scaled-applications = Optimisé pour les applications
    .applications-description = Les jeux et les applications X11 en plein écran peuvent ne pas correspondre avec la résolution de votre écran
    .scaled-compatibility = Mode compatibilité maximale
    .compatibility-description = Les applications X11 peuvent apparaître floues sur les écrans HiDPI
    .preferred-display = Écran préféré pour les jeux et les applications X11 en plein écran
    .no-display = Aucun

## Sound

sound = Son
    .desc = N/A
sound-output = Sortie
    .volume = Volume de sortie
    .device = Périphérique de sortie
    .level = Niveau de sortie
    .config = Configuration
    .balance = Équilibre
    .left = Gauche
    .right = Droite
sound-input = Entrée
    .volume = Volume d'entrée
    .device = Périphérique d'entrée
    .level = Niveau d'entrée
sound-alerts = Alertes
    .volume = Volume des alertes
    .sound = Alertes sonores
sound-applications = Applications
    .desc = Volumes et paramètres d'application

## Power

power = Alimentation
    .desc = Gérer les paramètres d'alimentation
battery = Batterie
    .minute =
        { $value } { $value ->
            [one] minute
           *[other] minutes
        }
    .hour =
        { $value } { $value ->
            [one] heure
           *[other] heures
        }
    .day =
        { $value } { $value ->
            [one] jour
           *[other] jours
        }
    .less-than-minute = Moins d'une minute
    .and = et
    .remaining-time =
        { $time } jusqu'à la { $action ->
            [full] charge
           *[other] décharge
        } complète
connected-devices = Périphériques connectés
    .unknown = Périphérique inconnu
power-mode = Modes d'énergie
    .battery = Économie d'énergie
    .battery-desc = Performances réduites et moins de consommation d'énergie
    .balanced = Équilibré
    .balanced-desc = Performances et consommation d'énergie équilibrées
    .performance = Performance
    .performance-desc = Performances maximales mais forte consommation d'énergie
    .no-backend = Backend non trouvé. Installez system76-power ou power-profiles-daemon.
power-saving = Options d'économie d'énergie
    .turn-off-screen-after = Éteindre l'écran après
    .auto-suspend = Suspension automatique
    .auto-suspend-ac = Suspension automatique lorsque le PC est branché
    .auto-suspend-battery = Suspension automatique sur batterie

## Input

acceleration-desc = Ajuste automatiquement la sensibilité du suivi en fonction de la vitesse
disable-while-typing = Désactiver pendant la saisie
input-devices = Périphériques d'entrée
    .desc = Périphériques d'entrée
primary-button = Bouton principal
    .desc = Définit l'ordre des boutons physiques
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
    .desc = Sources de saisie, changement de source, saisie de caractères spéciaux, raccourcis
keyboard-sources = Sources de saisie
    .desc = Les sources d'entrée peuvent être changées en utilisant la combinaison de touches Super+Espace. Cette combinaison peut être modifiée dans les paramètres des raccourcis clavier.
    .move-up = Remonter
    .move-down = Descendre
    .settings = Paramètres
    .view-layout = Afficher la disposition du clavier
    .remove = Supprimer
    .add = Ajouter une source de saisie
keyboard-special-char = Saisie de caractères spéciaux
    .alternate = Touche de caractères alternatifs
    .compose = Touche de composition
    .compose-desc = La touche de composition permet d'entrer une grande variété de caractères. Pour l'utiliser, appuyez sur composition puis une séquence de caractères. Par exemple, la touche composition suivie de C et o entrera ©, tandis que la touche composition suivie de a et ‘ entrera á.
    .caps = Touche Verr Màj
keyboard-typing-assist = Saisie
    .repeat-rate = Taux de répétition
    .repeat-delay = Délai de répétition
added = Ajouté
type-to-search = Tapez pour rechercher...
show-extended-input-sources = Afficher les sources d'entrée étendues

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Raccourcis clavier
    .desc = Voir et personnaliser les raccourcis
cancel = Annuler
command = Commande
custom = Personnalisé
debug = Déboguer
disabled = Désactivé
migrate-workspace-prev = Migrer l’espace de travail vers la sortie précédente
migrate-workspace-next = Migrer l’espace de travail vers la sortie suivante
migrate-workspace =
    Migrer l’espace de travail vers la sortie { $direction ->
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
toggle-stacking = Activer/désactiver l'empilement des fenêtres
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
    .focus =
        Focus sur la fenêtre { $direction ->
           *[down] bas
            [in] dedans
            [left] gauche
            [out] dehors
            [right] droite
            [up] haut
        }
    .output =
        Basculer à la sortie { $direction ->
           *[down] bas
            [left] gauche
            [right] droite
            [up] haut
        }
    .workspace = Basculer à l’espace de travail { $num }
manage-windows = Gérer les fenêtres
    .close = Fermer la fenêtre
    .maximize = Maximiser la fenêtre
    .fullscreen = Mettre la fenêtre en plein écran
    .minimize = Minimiser la fenêtre
    .resize-inwards = Redimensionner la fenêtre vers l’intérieur
    .resize-outwards = Redimensionner la fenêtre vers l’extérieur
    .toggle-sticky = Activer/désactiver l'ancrage de la fenêtre
move-windows = Déplacer les fenêtres
    .direction =
        Déplacer la fenêtre vers { $direction ->
           *[down] le bas
            [left] la gauche
            [right] la droite
            [up] le haut
        }
    .display =
        Déplacer la fenêtre d'un écran vers { $direction ->
           *[down] le bas
            [left] la gauche
            [right] la droite
            [up] le haut
        }
    .workspace =
        Déplacer la fenêtre d’un espace de travail { $direction ->
           *[below] en dessous
            [left] à gauche
            [right] à droite
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
    .display-toggle = Activer/désactiver l'écran interne
    .home-folder = Ouvrir le dossier personnel
    .keyboard-brightness-down = Réduire la luminosité du clavier
    .keyboard-brightness-up = Augmenter la luminosité du clavier
    .launcher = Ouvrir le lanceur
    .log-out = Se déconnecter
    .lock-screen = Verrouiller l'écran
    .mute = Couper le son de la sortie audio
    .mute-mic = Couper le son du microphone
    .play-pause = Lecture/Pause
    .play-next = Piste suivante
    .play-prev = Piste précédente
    .poweroff = Éteindre
    .screenshot = Prendre une capture d'écran
    .suspend = Suspendre
    .terminal = Ouvrir un terminal
    .touchpad-toggle = Activer/désactiver le pavé tactile
    .volume-lower = Diminuer le volume de sortie audio
    .volume-raise = Augmenter le volume de sortie audio
    .web-browser = Ouvrir un navigateur web
    .window-switcher = Basculer entre les fenêtres ouvertes
    .window-switcher-previous = Basculer entre les fenêtres ouvertes dans le sens inversé
    .workspace-overview = Ouvrir l’aperçu des espaces de travail
window-tiling = Agencement automatique des fenêtres
    .horizontal = Définir l'orientation horizontale
    .vertical = Définir l'orientation verticale
    .swap-window = Échanger la fenêtre
    .toggle-tiling = Activer/désactiver l'agencement automatique des fenêtres
    .toggle-stacking = Activer/désactiver l'empilement des fenêtres
    .toggle-floating = Activer/désactiver les fenêtres flottantes
    .toggle-orientation = Activer/désactiver l'orientation
replace-shortcut-dialog = Remplacer le raccourci ?
    .desc = { $shortcut } est utilisé par { $name }. Si vous le remplacez, { $name } sera désactivé.

## Input: Mouse

mouse = Souris
    .desc = Vitesse de la souris, accélération, défilement naturel
    .speed = Vitesse de la souris
    .acceleration = Activer l'accélération de la souris

## Input: Touchpad

click-behavior = Comportement de clic
    .click-finger = Clic secondaire avec deux doigts et clic du milieu avec trois doigts
    .button-areas = Clic secondaire dans le coin inférieur droit et clic du milieu dans la partie inférieure centrale
pinch-to-zoom = Pincer pour zoomer
    .desc = Utiliser deux doigts pour zoomer dans le contenu, pour les applications qui prennent en charge le zoom
tap-to-click = Tapoter pour cliquer
    .desc = Active le tapotement à un doigt pour le clic principal, le tapotement à deux doigts pour le clic secondaire, et le tapotement à trois doigts pour le clic du milieu
touchpad = Pavé tactile
    .acceleration = Activer l'accélération du pavé tactile
    .desc = Vitesse du pavé tactile, options de clic, gestes
    .speed = Vitesse du pavé tactile

## Input: Gestures

switch-workspaces = Basculer entre les espaces de travail
    .horizontal = Glissement de quatre doigts vers la gauche/droite
    .vertical = Glissement de quatre doigts vers le haut/bas
switch-between-windows = Basculer entre les fenêtres
open-application-library = Ouvrir la bibliothèque d'applications
open-workspaces-view = Ouvrir l'aperçu des espaces de travail

## Time & Language

time = Heure et langue
    .desc = N/A
time-date = Date et heure
    .desc = Fuseau horaire, réglage de l'heure automatique et format de l'heure
    .auto = Régler automatiquement
    .auto-ntp = La date et l'heure seront mises à jour automatiquement lorsque le fuseau horaire sera défini
time-zone = Fuseau Horaire
    .auto = Fuseau horaire automatique
    .auto-info = Nécessite les services de localisation et un accès à internet
time-format = Format date et heure
    .twenty-four = Format 24 heures
    .show-seconds = Afficher les secondes
    .first = Premier jour de la semaine
    .show-date = Afficher la date dans l'applet du panneau
    .friday = Vendredi
    .saturday = Samedi
    .sunday = Dimanche
    .monday = Lundi
time-region = Région et langue
    .desc = Formater les dates, les heures et les nombres en fonction de votre région
formatting = Format
    .dates = Dates
    .time = Heure
    .date-and-time = Date et heure
    .numbers = Nombres
    .measurement = Mesure
    .paper = Papier
preferred-languages = Langues préférées
    .desc = L'ordre des langues détermine la langue utilisée pour l'interface. Les changements prendront effet à la prochaine connexion.
add-language = Ajouter une langue
    .context = Ajouter une Langue
install-additional-languages = Installer des langues additionnelles
region = Région

## System

system = Système et comptes

## System: About

about = À propos
    .desc = Nom de l'appareil, informations sur le matériel et paramètres par défaut du système d'exploitation
about-device = Nom de l'appareil
    .desc = Ce nom est visible par les autres appareils réseau ou Bluetooth
about-hardware = Matériel
    .model = Modèle du matériel
    .memory = Mémoire vive
    .processor = Processeur
    .graphics = Carte graphique
    .disk-capacity = Capacité du disque
about-os = Système d'exploitation
    .os = Système d'exploitation
    .os-architecture = Architecture du système d'exploitation
    .kernel = Version du noyau
    .desktop-environment = Environnement de bureau
    .windowing-system = Système de fenêtrage
about-related = Paramètres associés
    .support = Obtenir de l'aide

## System: Firmware

firmware = Micrologiciel
    .desc = Détails micrologiciel

## System: Users

users = Utilisateurs
    .desc = Authentification et comptes utilisateurs
    .admin = Admin
    .standard = Standard
    .profile-add = Choisir l'image de profil
administrator = Administrateur
    .desc = Les administrateurs peuvent changer les paramètres de tous les utilisateurs, en ajouter des nouveaux et en supprimer
add-user = Ajouter l'utilisateur
remove-user = Supprimer l'utilisateur
full-name = Nom complet

## System: Default Applications

default-apps = Applications par défaut
    .desc = Navigateur web, client mail, explorateur de fichiers, et autres applications
    .web-browser = Navigateur web
    .file-manager = Explorateur de fichiers
    .mail-client = Client mail
    .music = Musique
    .video = Vidéo
    .photos = Photos
    .calendar = Calendrier
    .terminal = Terminal
    .other-associations = Autres associations
    .text-editor = Éditeur de texte
identity = Identité
save = Enregistrer
password-confirm = Confirmer le mot de passe
network-name = Nom du réseau
qr-code-unavailable = QR code non disponible
scan-to-connect-description = Scannez le QR code pour vous connecter à ce réseau.
share = Partager ce réseau
activate = Activer
enable = Activer
hearing = Audition
    .mono = Jouer l'audio stéréo en mono
default = Défaut
accessibility = Accessibilité
    .vision = Vue
    .on = Activé
    .off = Désactivé
    .unavailable = Indisponible
    .screen-reader = Lecteur d'écran
    .high-contrast = Mode contraste élevé
    .invert-colors = Inverser les couleurs
    .color-filters = Filtres de couleurs
color-filter = Type de filtre de couleurs
    .unknown = Filtre inconnu actif
    .greyscale = Niveaux de gris
    .deuteranopia = Vert/Rouge (faiblesse du vert, Deutéranopie)
    .protanopia = Rouge/Vert (faiblesse du rouge, Protanopie)
    .tritanopia = Bleu/Jaune (faiblesse du bleu, Tritanopie)
magnifier = Loupe
    .controls =
        Utilisez ces raccourcis : { $zoom_in ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_in } pour zoomer,
        }{ $zoom_out ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_out } pour dézoomer,
        }
        Super + défiler avec la souris
    .scroll_controls = Activer le zoom avec la souris ou le pavé tactile avec Super + défilement
    .show_overlay = Afficher l'interface en superposition de la loupe
    .increment = Puissance du zoom
    .signin = Démarrer la loupe à la connexion
    .applet = Activer le bouton on/off de l'applet de la loupe dans le panneau
    .movement = L'affichage zoomé se déplace
    .continuous = Continuellement avec le curseur
    .onedge = Lorsque le curseur atteint le bord
    .centered = Pour maintenir le curseur centré
vrr = Taux de rafraîchissement variable
    .enabled = Activé
    .force = Toujours
    .auto = Automatique
    .disabled = Désactivé
place-here = Placer les applets ici
amplification = Amplification
    .desc = Permet d'augmenter le volume jusqu'à 150%
edge-gravity = Les fenêtres flottantes se fixent aux bords proches
add-another-keybinding = Ajouter un autre raccourci clavier
password-mismatch = Le mot de passe et la confirmation doivent être identiques
keyboard-numlock-boot = Verrouillage numérique
    .boot-state = État au démarrage
    .last-boot = Dernier démarrage
    .on = Activé
    .off = Désactivé
    .set = Définir l'état du Verr Num au démarrage
gestures = Gestes
    .four-finger-down = Glissement de quatre doigts vers le bas
    .four-finger-left = Glissement de quatre doigts vers la gauche
    .four-finger-right = Glissement de quatre doigts vers la droite
    .four-finger-up = Glissement de quatre doigts vers le haut
    .three-finger-any = Glissement de trois doigts dans n'importe quelle direction
legacy-applications = Compatibilité des applications X11
    .desc = Échelle des fenêtres d'applications systèmes X11 et raccourcis globaux
input-source-switch = Changer la langue de saisie du clavier
startup-apps = Applications de démarrage
    .desc = Configurer les applications qui se lancent au démarrage
    .add = Ajouter appli
    .user = Applications lancées lors de la connexion
    .none = Aucune application de démarrage ajoutée
    .remove-dialog-title = Supprimer { $name } ?
    .remove-dialog-description = Supprimer cette application de démarrage ?
    .add-startup-app = Ajouter application de démarrage
invalid-username = Nom d'utilisateur invalide
legacy-app-global-shortcuts = Raccourcis globaux dans les applications X11
    .desc = Les raccourcis globaux permettent aux frappes clavier et aux clics de souris effectués dans les applications d’être reconnus par d’autres applications, pour des fonctionnalités telles que le push-to-talk ou le push-to-mute. Par défaut, les raccourcis globaux sont désactivés dans les applications X11 afin de s’assurer que d’autres applications ne puissent pas surveiller les événements clavier ou souris contenant des informations sensibles.
    .none = Aucune touche
    .modifiers = Modificateurs (Super, Maj, Contrôle, Alt)
    .combination = Toutes les touches lorsque les modificateurs Super, Contrôle ou Alt sont appuyés
    .all = Toutes les touches
    .mouse = Clics de souris dans les applications X11
zoom-in = Zoomer
zoom-out = Dézoomer
applications = Applications
change-password = Modifier le mot de passe
sound-device-port-unplugged = Débranché
sound-hd-audio = Audio HD
sound-usb-audio = Audio USB
sound-device-profiles = Profils des périphériques
