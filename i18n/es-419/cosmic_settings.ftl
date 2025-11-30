app = Configuración de COSMIC
unknown = Desconocido
dbus-connection-error = Error al conectar con DBus
ok = Aceptar
number = { $number }

## Network & Wireless

connections-and-profiles =
    { $variant ->
        [wired] Cableada
        [wifi] Wi-Fi
        [vpn] VPN
       *[other] Desconocida
    } conexiones y perfiles de conexión.
add-network = Añadir red
    .profile = Añadir perfil
add-vpn = Añadir VPN
airplane-on = El modo avión está activado.
cable-unplugged = Cable desconectado
connect = Conectar
connected = Conectado
connecting = Conectando…
disconnect = Desconectar
forget = Olvidar
known-networks = Redes conocidas
network-and-wireless = Red e inalámbrico
no-networks = No se han encontrado redes.
no-vpn = No hay conexiones VPN disponibles.
password = Contraseña
remove = Eliminar
settings = Configuración
username = Nombre de usuario
visible-networks = Redes visibles
auth-dialog = Autenticación requerida
    .vpn-description = Ingresa el nombre de usuario y la contraseña requeridos por el servicio de VPN.
    .wifi-description = Ingresa la contraseña o clave de encriptación. También puedes conectarte presionando el botón "WPS" en el router.
forget-dialog = ¿Olvidar esta red Wi-Fi?
    .description = Necesitarás ingresar una contraseña nuevamente para usar esta red Wi-Fi en el futuro.
network-device-state =
    .activated = Conectado
    .config = Conectando
    .deactivating = Desconectando
    .disconnected = Desconectado
    .failed = Falló la conexión
    .ip-check = Verificando conexión
    .ip-config = Solicitando información de IP y enrutamiento
    .need-auth = Necesita autenticación
    .prepare = Preparándose para conectar
    .secondaries = Esperando conexión secundaria
    .unavailable = No disponible
    .unknown = Estado desconocido
    .unmanaged = No administrado
    .unplugged = Cable desconectado
remove-connection-dialog = ¿Eliminar el perfil de conexión?
    .vpn-description = Necesitarás ingresar una contraseña nuevamente para usar esta red en el futuro.
    .wired-description = Necesitarás recrear este perfil para usarlo en el futuro.
vpn = VPN
    .connections = Conexiones VPN
    .error = Error al agregar la configuración de VPN
    .remove = Eliminar perfil de conexión
    .select-file = Seleccionar un archivo de configuración de VPN
vpn-error = Error de VPN
    .config = Error al agregar la configuración de VPN
    .connect = Error al conectar a la VPN
    .connection-editor = Error del editor de conexiones
    .connection-settings = Error al obtener la configuración de las conexiones activas
    .updating-state = Error al actualizar el estado del administrador de red
    .wireguard-config-path = Ruta de archivo no válida para la configuración de WireGuard
    .wireguard-config-path-desc = El archivo elegido debe estar en un sistema de archivos local.
    .wireguard-device = Error al crear el dispositivo WireGuard
    .with-password =
        Error al configurar la { $field ->
           *[username] nombre de usuario
            [password] contraseña
            [password-flags] banderas de la contraseña
        } con nmcli
wired = Conectado
    .adapter = Adaptador por cable { $id }
    .connections = Conexiones por cable
    .devices = Dispositivos por cable
    .remove = Eliminar perfil de conexión
wifi = Wi-Fi
    .adapter = Adaptador Wi-Fi { $id }
    .forget = Olvidar esta red
wireguard-dialog = Agregar dispositivo WireGuard
    .description = Elige un nombre de dispositivo para la configuración de WireGuard.

## Networking: Online Accounts

online-accounts = Cuentas en línea
    .desc = Agregar cuentas, IMAP y SMTP, inicios de sesión empresariales

# Bluetooth

confirm = Confirmar
bluetooth = Bluetooth
    .desc = Administrar dispositivos de Bluetooth
    .status = Este sistema es visible como { $aliases } mientras las configuraciones de Bluetooth estén abiertas.
    .connected = Conectado
    .connecting = Conectando
    .disconnecting = Desconectando
    .connect = Conectar
    .disconnect = Desconectar
    .forget = Olvidar
    .dbus-error = Ocurrió un error al interactuar con DBus: { $why }
    .disabled = El servicio de Bluetooth está deshabilitado
    .inactive = El servicio de Bluetooth no está activo
    .unknown = El servicio de Bluetooth no se pudo activar. ¿Está instalado BlueZ?
bluetooth-paired = Dispositivos conectados previamente
    .connect = Conectar
    .battery = { $percentage }% de batería
bluetooth-confirm-pin = Confirmar PIN de Bluetooth
    .description = Por favor confirma que el siguiente PIN coincide con el que se muestra en { $device }
bluetooth-available = Dispositivos cercanos
bluetooth-adapters = Adaptadores Bluetooth

## Desktop

desktop = Escritorio

## Desktop: Wallpaper

wallpaper = Fondo de Pantalla
    .change = Cambiar imagen cada
    .desc = Imágenes de fondo, colores y opciones de carrusel de imágenes.
    .fit = Ajuste del fondo de pantalla
    .folder-dialog = Elegir carpeta de fondos de pantalla
    .image-dialog = Elegir imagen de fondo de pantalla
    .plural = Fondos de Pantalla
    .same = Mismo fondo de pantalla en todas las pantallas
    .slide = Carrusel de imágenes
add-color = Añadir color
add-image = Añadir imagen
all-displays = Todas las pantallas
colors = Colores
dialog-add = Añadir
fill = Llenar
fit-to-screen = Ajustar a la pantalla
open-new-folder = Abrir nueva carpeta
recent-folders = Carpetas recientes
x-minutes =
    { $number } { $number ->
        [one] minuto
       *[other] minutos
    }
x-hours =
    { $number ->
        [1] 1 hora
       *[other] { $number } horas
    }

## Desktop: Appearance

appearance = Apariencia
    .desc = Colores de acento y tema COSMIC.
accent-color = Color de acento
app-background = Fondo de la aplicación o ventana
auto = Automático
close = Cerrar
color-picker = Selector de color
copied-to-clipboard = Copiado al portapapeles
copy-to-clipboard = Copiar al portapapeles
dark = Oscuro
export = Exportar
hex = Hex
import = Importar
light = Claro
mode-and-colors = Modo y colores
recent-colors = Colores recientes
reset-to-default = Restablecer a predeterminado
rgb = RGB
window-hint-accent = Color que se muestra al destacar la ventana en uso
window-hint-accent-toggle = Usar color de acento del tema como color de la ventana en uso
auto-switch = Cambia automáticamente entre los modos claro y oscuro
    .sunrise = Cambia a modo claro al amanecer
    .sunset = Cambia a modo oscuro al atardecer
    .next-sunrise = Cambia a modo claro en el próximo amanecer
    .next-sunset = Cambia a modo oscuro en el próximo atardecer
container-background = Fondo del contenedor
    .desc-detail = El color de fondo del contenedor se utiliza para la barra lateral de navegación, el cajón lateral, los diálogos y widgets similares. Por omisión, se deriva automáticamente del fondo de la aplicación o ventana.
    .reset = Restablecer a automático
    .desc = El color principal del contenedor se utiliza para la barra lateral de navegación, el cajón lateral, los diálogos y widgets similares.
control-tint = Tono del componente de control
    .desc = Se utiliza para los fondos de los botones estándar, entradas de búsqueda, entradas de texto y componentes similares.
frosted = Efecto de cristal translúcido en la interfaz del sistema
    .desc = Aplica desenfoque de fondo al panel, el dock, las miniaplicaciones, el lanzador y la biblioteca de aplicaciones.
enable-export = Aplicar este tema a aplicaciones GNOME.
    .desc = No todas las herramientas admiten el cambio automático. Es posible que las aplicaciones que no son de COSMIC necesiten reiniciarse después de un cambio de tema.
icon-theme = Tema de iconos
    .desc = Aplica un conjunto diferente de iconos a las aplicaciones.
text-tint = Tono de texto de interfaz
    .desc = Color utilizado para derivar colores de texto de interfaz que tengan suficiente contraste en diversas superficies.
style = Estilo
    .round = Redondeado
    .slightly-round = Ligeramente redondeado
    .square = Cuadrado
interface-density = Densidad de la interfaz
    .comfortable = Cómoda
    .compact = Compacta
    .spacious = Espaciosa
window-management-appearance = Gestión de ventanas
    .active-hint = Tamaño del indicador de la ventana activa
    .gaps = Espacios alrededor de las ventanas en mosaico

### Experimental

experimental-settings = Configuraciones experimentales
icons-and-toolkit = Iconos y tematización del toolkit
interface-font = Fuente del sistema
monospace-font = Fuente monoespaciada

## Desktop: Notifications

notifications = Notifications
    .desc = No molestar, notificaciones en la pantalla de bloqueo y configuraciones por aplicación.

## Desktop: Panel

panel = Panel
    .desc = Barra superior con controles y miniaplicaciones.
add = Añadir
add-applet = Añadir miniaplicación
all = Todos
applets = Miniaplicaciones
center-segment = Segmento central
end-segment = Segmento final
large = Grande
no-applets-found = No se encontraron miniaplicaciones...
panel-bottom = Abajo
panel-left = Izquierda
panel-right = Derecha
panel-top = Arriba
search-applets = Buscar miniaplicaciones...
small = Pequeño
start-segment = Segmento inicial
panel-appearance = Apariencia
    .match = Igual que el escritorio
    .light = Claro
    .dark = Oscuro
panel-behavior-and-position = Comportamiento y posiciones
    .autohide = Ocultar panel automáticamente
    .dock-autohide = Ocultar el dock automáticamente
    .position = Posición en la pantalla
    .display = Mostrar en pantalla
panel-style = Estilo
    .anchor-gap = Espacio entre el panel y los bordes de la pantalla
    .dock-anchor-gap = Espacio entre el dock y los bordes de la pantalla
    .extend = Extender panel hasta los bordes de la pantalla
    .dock-extend = Extender el dock hasta los bordes de la pantalla
    .appearance = Apariencia
    .size = Tamaño
    .background-opacity = Opacidad del fondo
panel-applets = Configuración
    .dock-desc = Configurar miniaplicaciones del dock
    .desc = Configurar miniaplicaciones del panel
panel-missing = Falta la configuración del panel
    .desc = El archivo de configuración del panel falta debido al uso de una configuración personalizada o está dañado.
    .fix = Restablecer a predeterminado

## Desktop: Dock

dock = Dock
    .desc = Un panel opcional con aplicaciones y miniaplicaciones.

## Desktop: Window management

window-management = Gestión de ventanas
    .desc = Acción de la tecla Súper, opciones de control de ventanas y opciones adicionales de ventanas en mosaico.
super-key = Acción de la tecla Súper
    .launcher = Abrir lanzador
    .workspaces = Abrir espacios de trabajo
    .applications = Abrir aplicaciones
    .disable = Deshabilitar
window-controls = Controles de ventana
    .minimize = Mostrar botón de minimizar
    .maximize = Mostrar botón de maximizar
    .active-window-hint = Mostrar indicación de ventana activa
focus-navigation = Navegación de enfoque
    .focus-follows-cursor = El enfoque sigue al cursor
    .focus-follows-cursor-delay = Retraso del enfoque que sigue al cursor en ms
    .cursor-follows-focus = El cursor sigue al enfoque

## Desktop: Workspaces

workspaces = Espacios de trabajo
    .desc = Configura el número de espacios de trabajo, comportamiento y ubicación.
workspaces-behavior = Comportamiento de los espacios de trabajo
    .dynamic = Espacios de trabajo dinámicos
    .dynamic-desc = Elimina automáticamente los espacios de trabajo vacíos.
    .fixed = Número fijo de espacios de trabajo
    .fixed-desc = Añadir o eliminar espacios de trabajo en la vista general.
workspaces-multi-behavior = Comportamiento con múltiples pantallas
    .span = Los espacios de trabajo se extienden a todas las pantallas
    .separate = Las pantallas tienen espacios de trabajo separados
workspaces-overview-thumbnails = Miniaturas de vista general de espacios de trabajo
    .show-number = Mostrar número de espacio de trabajo
    .show-name = Mostrar nombre de espacio de trabajo
workspaces-orientation = Orientación de los espacios de trabajo
    .vertical = Vertical
    .horizontal = Horizontal
hot-corner = Esquina activa
    .top-left-corner = Habilitar esquina activa superior izquierda para espacios de trabajo

## Displays

-requires-restart = Requiere reinicio
color = Color
    .depth = Profundidad de color
    .profile = Perfil de color
    .sidebar = Perfiles de color
    .temperature = Temperatura de color
display = Pantallas
    .desc = Gestionar pantallas y luz nocturna
    .arrangement = Disposición de pantallas
    .arrangement-desc = Arrastra las pantallas para reorganizarlas.
    .enable = Habilitar pantalla
    .external = Pantalla externa de { $size } { $output }
    .laptop = Pantalla de laptop de { $size }
    .options = Opciones de pantalla
    .refresh-rate = Frecuencia de actualización
    .resolution = Resolución
    .scale = Escala
    .additional-scale-options = Opciones de escala adicionales
mirroring = Duplicar pantalla
    .id = Duplicado { $id }
    .dont = No duplicar
    .mirror = Duplicar { $display }
    .project =
        Proyectar a { $display ->
            [all] todas las pantallas
           *[other] { $display }
        }
    .project-count =
        Proyectando a { $count } otra(s) { $count ->
            [1] pantalla
           *[other] pantallas
        }
night-light = Luz nocturna
    .auto = Automático (del atardecer al amanecer)
    .desc = Reduce la luz azul con colores más cálidos.
orientation = Orientación
    .standard = Estándar
    .rotate-90 = Rotar 90
    .rotate-180 = Rotar 180
    .rotate-270 = Rotar 270
scheduling = Programación
    .manual = Programación manual
dialog = Diálogo
    .title = ¿Mantener estas configuraciones de pantalla?
    .keep-changes = Mantener cambios
    .change-prompt = Los cambios en la configuración se revertirán automáticamente en { $time } segundos.
    .revert-settings = Revertir configuraciones
legacy-app-scaling = Escalado de aplicaciones del sistema de ventanas X11
    .scaled-gaming = Optimizar para juegos y aplicaciones en pantalla completa
    .gaming-description = Puede que las aplicaciones X11 parezcan un poco más grandes o pequeñas comparadas a las aplicaciones Wayland.
    .scaled-applications = Optimizar para las aplicaciones
    .applications-description = Puede que los juegos y las aplicaciones X11 en pantalla completa no coincidan con la resolución de tu pantalla
    .scaled-compatibility = Modo de compatibilidad máxima
    .compatibility-description = Puede que las aplicaciones X11 parezcan borrosas en pantallas HiDPI.
    .preferred-display = Pantalla preferida para juegos y aplicaciones X11 en pantalla completa
    .no-display = Ninguna

## Sound

sound = Sonido
    .desc = N/A
sound-output = Salida
    .volume = Volumen de salida
    .device = Dispositivo de salida
    .level = Nivel de salida
    .config = Configuración
    .balance = Balance
    .left = Izquierda
    .right = Derecha
sound-input = Entrada
    .volume = Volumen de entrada
    .device = Dispositivo de entrada
    .level = Nivel de entrada
sound-alerts = Alertas
    .volume = Volumen de alertas
    .sound = Sonido de alertas
sound-applications = Aplicaciones
    .desc = Volúmen y configuración de audio de aplicaciones

## Power

power = Energía y batería
    .desc = Administrar configuraciones de energía
battery = Batería
    .minute =
        { $value } { $value ->
            [one] minuto
           *[other] minutos
        }
    .hour =
        { $value } { $value ->
            [one] hora
           *[other] horas
        }
    .day =
        { $value } { $value ->
            [one] día
           *[other] días
        }
    .less-than-minute = Menos de un minuto
    .and = y
    .remaining-time =
        { $time } hasta { $action ->
            [full] completo
           *[other] vacío
        }
connected-devices = Dispositivos Conectados
    .unknown = Dispositivo desconocido
power-mode = Modo de Energía
    .battery = Vida de batería extendida
    .battery-desc = Uso reducido de energía y rendimiento silencioso.
    .balanced = Equilibrado
    .balanced-desc = Rendimiento silencioso y uso moderado de energía.
    .performance = Alto rendimiento
    .performance-desc = Rendimiento y uso de energía máximos.
    .no-backend = Backend no encontrado. Instala system76-power o power-profiles-daemon.

## Input

acceleration-desc = Ajusta automáticamente la sensibilidad de seguimiento según la velocidad.
disable-while-typing = Desactivar mientras se escribe
input-devices = Dispositivos de entrada
    .desc = Dispositivos de entrada
primary-button = Botón Primario
    .desc = Establece el orden de los botones físicos.
    .left = Izquierda
    .right = Derecha
scrolling = Desplazamiento
    .two-finger = Desplazamiento con dos dedos
    .edge = Desplazamiento a lo largo del borde con un dedo
    .speed = Velocidad de desplazamiento
    .natural = Desplazamiento natural
    .natural-desc = Desplaza el contenido en lugar de la vista

## Input: Keyboard

slow = Lento
fast = Rápido
short = Corto
long = Largo
keyboard = Teclado
    .desc = Entrada del teclado, cambio de entrada, caracteres especiales, atajos.
keyboard-sources = Fuentes de entrada
    .desc = Las fuentes de entrada se pueden cambiar usando la combinación de teclas Súper + Espacio. Esto se puede personalizar en la configuración de los atajos de teclado.
    .move-up = Mover hacia arriba
    .move-down = Mover hacia abajo
    .settings = Configuración
    .view-layout = Ver distribución del teclado
    .remove = Eliminar
    .add = Añadir fuente de entrada
keyboard-special-char = Entrada de caracteres especiales
    .alternate = Tecla de caracteres alternativos
    .compose = Tecla de composición
    .caps = Tecla de Bloq Mayús
keyboard-typing-assist = Escritura
    .repeat-rate = Tasa de repetición
    .repeat-delay = Retraso de repetición
added = Añadido
type-to-search = Escribir para buscar...
show-extended-input-sources = Mostrar fuentes de entrada extendidas

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Atajos de teclado
    .desc = Ver y personalizar atajos
cancel = Cancelar
command = Comando
custom = Personalizado
debug = Depuración
disabled = Desactivado
migrate-workspace-prev = Migrar espacio de trabajo a la salida anterior
migrate-workspace-next = Migrar espacio de trabajo a la salida siguiente
migrate-workspace =
    Migrar espacio de trabajo a la salida { $direction ->
       *[down] inferior
        [left] izquierda
        [right] derecha
        [up] superior
    }
navigate = Navegar
replace = Remplazar
shortcut-name = Nombre del atajo
system-controls = Controles del sistema
terminate = Terminar
toggle-stacking = Cambiar a apilamiento de ventanas
type-key-combination = Escribir combinación de teclas
custom-shortcuts = Atajos personalizados
    .add = Añadir atajo
    .context = Añadir atajo personalizado
    .none = No hay atajos personalizados
modified = { $count } modificado
nav-shortcuts = Navegación
    .prev-output = Enfocar salida anterior
    .next-output = Enfocar salida siguiente
    .last-workspace = Enfocar el último espacio de trabajo
    .prev-workspace = Enfocar el espacio de trabajo anterior
    .next-workspace = Enfocar el espacio de trabajo siguiente
    .focus =
        Enfocar ventana { $direction ->
           *[down] abajo
            [in] dentro
            [left] izquierda
            [out] fuera
            [right] derecha
            [up] arriba
        }
    .output =
        Cambiar a la salida { $direction ->
           *[down] abajo
            [left] izquierda
            [right] derecha
            [up] arriba
        }
    .workspace = Cambiar al espacio de trabajo { $num }
manage-windows = Gestionar ventanas
    .close = Cerrar ventana
    .maximize = Maximizar ventana
    .fullscreen = Mostrar ventana en pantalla completa
    .minimize = Minimizar ventana
    .resize-inwards = Redimensionar ventana hacia adentro
    .resize-outwards = Redimensionar ventana hacia afuera
    .toggle-sticky = Cambiar a ventana siempre visible
move-windows = Mover ventanas
    .direction =
        Mover ventana { $direction ->
           *[down] abajo
            [left] izquierda
            [right] derecha
            [up] arriba
        }
    .display =
        Mover ventana una pantalla { $direction ->
           *[down] abajo
            [left] izquierda
            [right] derecha
            [up] arriba
        }
    .workspace =
        Mover ventana un espacio de trabajo { $direction ->
           *[below] abajo
            [left] izquierda
            [right] derecha
            [above] arriba
        }
    .workspace-num = Mover ventana al espacio de trabajo { $num }
    .prev-workspace = Mover ventana al espacio de trabajo anterior
    .next-workspace = Mover ventana al espacio de trabajo siguiente
    .last-workspace = Mover ventana al último espacio de trabajo
    .next-display = Mover ventana a la pantalla siguiente
    .prev-display = Mover ventana a la pantalla anterior
    .send-to-prev-workspace = Mover ventana al espacio de trabajo anterior
    .send-to-next-workspace = Mover ventana al espacio de trabajo siguiente
system-shortcut = Sistema
    .app-library = Abrir la biblioteca de aplicaciones
    .brightness-down = Disminuir el brillo de la pantalla
    .brightness-up = Aumentar el brillo de la pantalla
    .display-toggle = Habilitar/deshabilitar la pantalla interna
    .home-folder = Abrir la carpeta personal
    .keyboard-brightness-down = Disminuir el brillo del teclado
    .keyboard-brightness-up = Aumentar el brillo del teclado
    .launcher = Abrir el lanzador
    .log-out = Cerrar sesión
    .lock-screen = Bloquear la pantalla
    .mute = Silenciar salida de audio
    .mute-mic = Silenciar entrada de micrófono
    .play-pause = Reproducir/pausar
    .play-next = Siguiente pista
    .play-prev = Pista anterior
    .poweroff = Apagar
    .screenshot = Tomar una cainicioptura de pantalla
    .terminal = Abrir un terminal
    .touchpad-toggle = Habilitar/deshabilitar panel táctil
    .volume-lower = Disminuir el volumen de la salida de audio
    .volume-raise = Aumentar el volumen de la salida de audio
    .web-browser = Abrir un navegador web
    .window-switcher = Cambiar entre ventanas abiertas
    .window-switcher-previous = Cambiar entre ventanas abiertas en sentido contrario
    .workspace-overview = Abrir la vista general de espacios de trabajo
window-tiling = Organización de ventanas
    .horizontal = Establecer orientación horizontal
    .vertical = Establecer orientación vertical
    .swap-window = Intercambiar ventana
    .toggle-tiling = Organizar ventanas en mosaico
    .toggle-stacking = Cambiar a ventanas apiladas
    .toggle-floating = Cambiar a ventanas flotantes
    .toggle-orientation = Activar orientación
replace-shortcut-dialog = ¿Reemplazar acceso directo?
    .desc = { $shortcut } está en uso por { $name }. Si lo reemplazas, { $name } será desactivado.

## Input: Mouse

mouse = Mouse
    .desc = Velocidad del mouse, aceleración, desplazamiento natural.
    .speed = Velocidad del mouse
    .acceleration = Activar aceleración del mouse

## Input: Touchpad

click-behavior = Comportamiento de clic
    .click-finger = Clic secundario con dos dedos y clic medio con tres dedos
    .button-areas = Clic secundario en la esquina inferior derecha y clic medio en el centro inferior
pinch-to-zoom = Pellizcar para hacer zum
    .desc = Usa dos dedos para hacer zum en el contenido, para aplicaciones que lo soportan.
tap-to-click = Tocar para hacer clic
    .desc = Activa el toque con un dedo para el clic primario, toque con dos dedos para el clic secundario y toque con tres dedos para el clic medio.
touchpad = Panel táctil
    .acceleration = Activar aceleración del panel táctil
    .desc = Velocidad del panel táctil, opciones de clic, gestos.
    .speed = Velocidad del panel táctil

## Input: Gestures

gestures = Gestos
    .four-finger-down = Deslizar cuatro dedos hacia abajo
    .four-finger-left = Deslizar cuatro dedos hacia la izquierda
    .four-finger-right = Deslizar cuatro dedos hacia la derecha
    .four-finger-up = Deslizar cuatro dedos hacia arriba
    .three-finger-any = Deslizar tres dedos en cualquier dirección
switch-workspaces = Cambiar espacios de trabajo
    .horizontal = Deslizar cuatro dedos a la izquierda/derecha
    .vertical = Deslizar cuatro dedos hacia arriba/abajo
switch-between-windows = Cambiar entre ventanas
open-application-library = Abrir Biblioteca de Aplicaciones
open-workspaces-view = Abrir Vista de Espacios de Trabajo

## Time & Language

time = Hora e idioma
    .desc = N/A
time-date = Fecha y Hora
    .desc = Zona horaria, configuración automática del reloj y algunos formatos de hora.
    .auto = Configurar automáticamente
    .auto-ntp = La fecha y la hora se actualizarán automáticamente cuando se establezca la zona horaria.
time-zone = Zona horaria
    .auto = Zona horaria automática
    .auto-info = Requiere servicios de ubicación y acceso a internet
time-format = Formato de Fecha y Hora
    .twenty-four = Formato de 24 horas
    .show-seconds = Mostrar segundos
    .first = Primer día de la semana
    .show-date = Mostrar Fecha en el Panel Superior
    .friday = Viernes
    .saturday = Sábado
    .sunday = Domingo
    .monday = Lunes
time-region = Región e idioma
    .desc = Formato de fechas, horas y números según tu región.

## System

system = Sistema y cuentas

## System: About

about = Acerca de
    .desc = Nombre del dispositivo, información de hardware, configuraciones predeterminadas del sistema operativo.
about-device = Nombre del dispositivo
    .desc = Este nombre aparece para otros dispositivos de red o Bluetooth.
about-hardware = Hardware
    .model = Modelo de hardware
    .memory = Memoria
    .processor = Procesador
    .graphics = Gráficos
    .disk-capacity = Capacidad del disco
about-os = Sistema operativo
    .os = Sistema operativo
    .os-architecture = Arquitectura del sistema operativo
    .desktop-environment = Entorno de escritorio
    .windowing-system = Sistema de ventanas
about-related = Configuraciones relacionadas
    .support = Obtener soporte

## System: Firmware

firmware = Firmware
    .desc = Detalles del firmware.

## System: Users

users = Usuarios
    .desc = Autenticación y cuentas de usuarios
    .admin = Administrador
    .standard = Estándar
    .profile-add = Elegir imagen de perfil
password-confirm = Confirme la contraseña
identity = Identidad
activate = Activar
enable = Habilitar
accessibility = Accesibilidad
    .vision = Visión
    .on = Habilitado
    .off = Deshabilitado
    .unavailable = No disponible
    .screen-reader = Lector de pantalla
    .high-contrast = Modo de alto contraste
    .invert-colors = Invertir color
    .color-filters = Filtros de color
hearing = Audición
    .mono = Reproducir audio estéreo como monoaural
default = Predeterminado
magnifier = Ampliación de pantalla
    .controls =
        O use estos atajos: { $zoom_in ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_in } para ampliar,
        }{ $zoom_out ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_out } para disminuir,
        }
        Súper + rueda del ratón
    .scroll_controls = Habilitar la ampliación con el mouse or el panel táctil con la tecla Súper + la rueda del mouse
    .show_overlay = Mostrar las opciones superpuestas de la ampliación de pantalla
    .increment = Incremento de ampliación
    .signin = Habilitar la ampliación al iniciar sesión
    .applet = Habilitar/deshabilitar ampliación en una miniaplicación en el panel
    .movement = Movimiento de la ampliación de pantalla
    .continuous = Moverse continuamente con el puntero
    .onedge = Moverse cuando el puntero alcanza el borde
    .centered = Moverse para mantener el puntero centrado
color-filter = Tipo de filtro de color
    .unknown = Filtro desconocido activo
    .greyscale = Escala de grises
    .deuteranopia = Verde/rojo (falta de percepción del verde, deuteranopia)
    .protanopia = Rojo/verde (falta de percepción del rojo, protanopia)
    .tritanopia = Azul/amarillo (falta de percepción del azul, tritanopia)
never = Nunca
edge-gravity = Las ventanas flotantes se adhieren a los bordes cercanos
vrr = Frecuencia de actualización variable
    .enabled = Habilitada
    .force = Siempre
    .auto = Automático
    .disabled = Deshabilitada
amplification = Amplificación
    .desc = Permite que el volumen se aumente al 150 %.
power-saving = Opciones de ahorro de energía
    .turn-off-screen-after = Apagar la pantalla después de
    .auto-suspend = Suspensión automática
    .auto-suspend-ac = Suspensión automática con la computadora enchufada
    .auto-suspend-battery = Suspensión automática mientras se esté usando la batería
keyboard-numlock-boot = Bloqueo numérico
    .boot-state = Estado durante el arranque
    .last-boot = Último arranque
    .on = Habilitado
    .off = Deshabilitado
    .set = Establecer el estado de arranque del bloqueo numérico
input-source-switch = Cambiar la fuente de entrada del idioma del teclado
add-another-keybinding = Añadir otro atajo
zoom-in = Ampliar
zoom-out = Disminuir
formatting = Formato
    .dates = Fechas
    .time = Hora
    .date-and-time = Fecha y hora
    .numbers = Números
    .measurement = Medidas
    .paper = Papel
preferred-languages = Idiomas preferidos
    .desc = El orden determina en qué idioma se mostrará la interfaz de usuario. Los cambios se aplicarán en el próximo inicio de sesión.
add-language = Añadir idioma
    .context = Añadir idioma
install-additional-languages = Instalar idiomas adicionales
region = Región
applications = Aplicaciones
default-apps = Aplicaciones predeterminadas
    .desc = Establecer las aplicaciones predeterminadas para el navegador web, el cliente de correo, el gestor de archivos y otras aplicaciones
    .web-browser = Navegador web
    .file-manager = Gestor de archivos
    .mail-client = Cliente de correo
    .music = Música
    .video = Video
    .photos = Fotos
    .calendar = Calendario
    .terminal = Terminal
    .other-associations = Otras asociaciones
    .text-editor = Editor de texto
startup-apps = Aplicaciones de inicio
    .desc = Configurar aplicaciones lanzadas al iniciar sesión.
    .add = Añadir aplicación
    .user = Aplicaciones lanzadas al iniciar sesión
    .none = No hay aplicaciones de inicio añadidas
    .remove-dialog-title = Quitar { $name }?
    .remove-dialog-description = ¿Estás seguro de que quieres quitar esta aplicación de inicio?
    .search-for-application = Buscar aplicación
legacy-applications = Compatibilidad con aplicaciones X11
    .desc = Escalado de aplicaciones y atajos globales para el sistema de ventanas X11.
legacy-app-global-shortcuts = Atajos globales en aplicaciones X11
    .desc = Los atajos globales permiten que las aplicaciones reconozcan las pulsaciones de teclas y eventos de botones de mouse cuando estos se realizan en otras aplicaciones para propósitos de funcionalidades como pulsar para hablar o pulsar para silenciar. Por omisión, esta característica está deshabilitada en las aplicaciones X11 para asegurar que otras aplicaciones no puedan monitorear los eventos de teclado y mouse que contengan información sensible.
    .none = Ninguna tecla
    .modifiers = Teclas modificadoras (Súper, Mayús, Control, Alt)
    .combination = Todas las teclas mientras se pulse una de las teclas modificadoras: Súper, Control o Alt
    .all = Todas las teclas
    .mouse = Eventos de mouse en las aplicaciones
administrator = Administrador
    .desc = Los administradores pueden cambiar las configuraciones para todos los usuarios, y pueden añadir o eliminar otros usuarios.
add-user = Añadir usuario
change-password = Cambiar la contraseña
remove-user = Eliminar usuario
full-name = Nombre completo
invalid-username = Nombre de usuario inválido.
password-mismatch = Las contraseñas no coinciden.
save = Guardar
