app = Configuración de COSMIC

dbus-connection-error = Error al conectar con DBus
ok = OK
unknown = Desconocido

number = { $number }

## Network & Wireless

connections-and-profiles = { $variant ->
    [wired] Cableada,
    [wifi] Wi-Fi,
    [vpn] VPN,
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
password-confirm = Confirme la contraseña
remove = Eliminar
settings = Configuración
username = Nombre de usuario
visible-networks = Redes visibles
identity = Identidad

auth-dialog = Autenticación requerida
    .vpn-description = Ingresa el nombre de usuario y la contraseña requeridos por el servicio de VPN.
    .wifi-description = Ingresa la contraseña o clave de encriptación. También puedes conectarte presionando el botón "WPS" en el router.

forget-dialog = ¿Olvidar esta red Wi-Fi?
    .description = Necesitarás ingresar una contraseña nuevamente para utilizar esta red Wi-Fi en el futuro.

network-device-state =
    .activated = Conectado
    .config = Conectándose
    .deactivating = Desconectando
    .disconnected = Desconectado
    .failed = No se pudo establecer la conexión
    .ip-check = Verificando la conexión
    .ip-config = Solicitando información de IP y enrutamiento
    .need-auth = Necesita autenticación
    .prepare = Preparándose para conectarse
    .secondaries = Esperando conexión secundaria
    .unavailable = No disponible
    .unknown = Estado desconocido
    .unmanaged = No administrado
    .unplugged = Cable desconectado

remove-connection-dialog = ¿Eliminar este perfil de conexión?
    .vpn-description = Necesitarás ingresar una contraseña nuevamente para usar esta red en el futuro.
    .wired-description = Necesitarás recrear este perfil para usarlo en el futuro.

vpn = VPN
    .connections = Conexiones VPN
    .error = Error al añadir la configuración de la VPN
    .remove = Eliminar perfil de conexión
    .select-file = Seleccionar un archivo de configuración de VPN

vpn-error = Error de VPN
    .config = Error al agregar la configuración de la VPN
    .connect = Error al intentar conectarse con la VPN
    .connection-editor = Error del editor de conexiones
    .connection-settings = Error al obtener la configuración de las conexiones activas
    .updating-state = Error al actualizar el estado del administrador de red
    .wireguard-config-path = Ruta de archivo no válida para la configuración de WireGuard
    .wireguard-config-path-desc = El archivo elegido debe estar en un sistema de archivos local.
    .wireguard-device = Error al crear un dispositivo para WireGuard
    .with-password = Error al configurar la { $field ->
        *[username] Usuario
        [password] Contraseña
        [password-flags] Indicadores de contraseña
    } con nmcli

wired = Conectado
    .adapter = Adaptador por cable { $id }
    .connections = Conexiones por cable
    .devices = Dispositivos por cable
    .remove = Eliminar perfil de conexión

wifi = Wi-Fi
    .adapter = Adaptador Wi-Fi { $id }
    .forget = Olvidar esta red

wireguard-dialog = Añadir un dispositivo para WireGuard
    .description = Elige un nombre de dispositivo para la configuración de WireGuard.

## Networking: Online Accounts

online-accounts = Cuentas en línea
    .desc = Añadir cuentas, IMAP y SMTP, inicios de sesión de empresa

# Bluetooth

activate = Activar
confirm = Confirmar
enable = Habilitar

bluetooth = Bluetooth
    .desc = Administrar dispositivos Bluetooth
    .status = Este sistema es visible como { $aliases } mientras la configuracion de Bluetooth está abierta.
    .connected = Conectado
    .connecting = Conectando
    .disconnecting = Desconectando
    .connect = Conectar
    .disconnect = Desconectar
    .forget = Olvidar
    .dbus-error = Ocurrió un error al interactuar con DBus: { $why }
    .disabled = El servicio de bluetooth está deshabilitado
    .inactive = El servicio de bluetooth está inactivo
    .unknown = No se pudo activar el servicio de bluetooth. Está bluez instalado?

bluetooth-paired = Dispositivos conectados previamente
    .connect = Conectar
    .battery = { $percentage }% de batería

bluetooth-confirm-pin = Confirmar PIN de Bluetooth
    .description = Por favor confirma que el siguiente PIN coincide con el que se muestra en { $device }

bluetooth-available = Dispositivos cercanos

bluetooth-adapters = Adaptadores Bluetooth

## Accessibility

accessibility = Accesibilidad
    .vision = Visión
    .on = Activada
    .off = Desactivada
    .unavailable = No disponible
    .screen-reader = Lector de pantalla
    .high-contrast = Modo de contraste alto
    .invert-colors = Invertir colores
    .color-filters = Filtros de color

hearing = Audición
    .mono = Reproducir audio estéreo como mono

default = Predeterminado
magnifier = Lupa
    .controls = También puedes usar estos atajos: { $zoom_in ->
             [zero] {""}
            *[other] {""}
                {$zoom_in} para ampliar,
        }{ $zoom_out ->
             [zero] {""}
            *[other] {""}
                {$zoom_out} para disminuir,
        }
        Super + rueda del ratón
    .scroll_controls = Habilita ampliar y disminuir usando el ratón o el panel táctil con Super + Scroll
    .show_overlay = Mostrar los valores de la lupa
    .increment = Incrementos de acercamiento o alejamiento
    .signin = Habilitar la lupa al iniciar sesión
    .applet = Habilitar o deshabilitar la lupa desde el panel
    .movement = Control de la pantalla mientras usa la lupa
    .continuous = Continuamente siguiendo el movimiento del puntero
    .onedge = Cuando el puntero alcanza un borde
    .centered = Mantiene el puntero centrado
color-filter = Tipos de filtro de color
    .unknown = Unknown Filter active
    .greyscale = Escala de grises
    .deuteranopia = Verde/Rojo (dicromacia verde, Deuteranopia)
    .protanopia = Rojo/Verde (dicromacia roja, Protanopia)
    .tritanopia = Azul/Amarillo (dicromacia azul, Tritanopia)

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
    .same = Mismo fondo en todas las pantallas
    .slide = Carrusel de imágenes.

add-color = Añadir color
add-image = Añadir imagen
all-displays = Todas las pantallas
colors = Colores
dialog-add = Añadir
fill = Llenar
fit-to-screen = Ajustar a la pantalla
open-new-folder = Abrir nueva carpeta
recent-folders = Carpetas recientes

x-minutes = { $number } minutos
x-hours = { $number ->
    [1] 1 hora
    *[other] { $number } horas
}
never = Nunca

## Desktop: Appearance

appearance = Apariencia
    .desc = Colores de énfasis y creación de temas.

accent-color = Color de énfasis
app-background = Fondo de la aplicación o ventana
auto = Automático
close = Cerrar
color-picker = Selector de color
copied-to-clipboard = Copiado al portapapeles
copy-to-clipboard = Copiar al portapapeles
dark = Oscuro
export = Exportar
hex = Hexadecimal
import = Importar
light = Claro
mode-and-colors = Modo y Colores
recent-colors = Colores recientes
reset-to-default = Restablecer a predeterminado
rgb = RGB
window-hint-accent = Color de énfasis de la ventana activa
window-hint-accent-toggle = Usar color de énfasis del tema como indicación de ventana activa

auto-switch = Cambiar automáticamente de modo Claro a Oscuro
    .sunrise = Cambia a modo Claro al amanecer
    .sunset = Cambia a modo Oscuro al atardecer
    .next-sunrise = Cambia a modo Claro en el próximo amanecer
    .next-sunset = Cambia a modo Oscuro en el próximo atardecer

container-background = Fondo del contenedor
    .desc-detail = El color de fondo del contenedor se utiliza para la barra de navegación lateral, el panel lateral, los diálogos y widgets similares. Por defecto, se deriva automáticamente del fondo de la aplicación o ventana.
    .reset = Restablecer a automático
    .desc = El color primario del contenedor se utiliza para la barra de navegación lateral, el panel lateral, los diálogos y widgets similares.

control-tint = Tonalidad de los componentes de control
    .desc = Se utiliza para los fondos de botones estándar, campos de búsqueda, campos de texto y componentes similares.

frosted = Efecto de vidrio esmerilado en la interfaz del sistema
    .desc = Aplica un efecto de vidrio esmerilado al panel, barra, subprogramas, lanzador y biblioteca de aplicaciones.

enable-export = Aplicar este tema a las aplicaciones GNOME.
    .desc = No todos los conjuntos de herramientas soportan el cambio automático. Las aplicaciones que no sean COSMIC pueden necesitar ser reiniciadas después de un cambio de tema.

icon-theme = Tema de iconos
    .desc = Aplica un conjunto diferente de iconos a las aplicaciones.

text-tint = Tonalidad del texto de la interfaz
    .desc = Color utilizado para generar colores de texto de la interfaz con buen contraste en diferentes fondos.

style = Estilo
    .round = Redondo
    .slightly-round = Ligeramente redondo
    .square = Cuadrado

interface-density = Densidad de la Interfaz
    .comfortable = Confortable
    .compact = Compacta
    .spacious = Espaciosa

window-management-appearance = Gestión de ventanas
    .active-hint = Tamaño de la indicación de ventana activa
    .gaps = Espacios alrededor de las ventanas en mosaico

### Experimental

experimental-settings = Configuraciones experimentales
icons-and-toolkit = Iconos y temas de los conjuntos de herramientas
interface-font = Fuente del Sistema
monospace-font = Fuente monoespaciada

## Desktop: Notifications

notifications = Notificaciones
    .desc = No molestar, notificaciones en la pantalla de bloqueo y configuraciones por aplicación.

## Desktop: Panel

panel = Barra
    .desc = Barra principal del sistema para menús y subprogramas.

add = Añadir
add-applet = Añadir subprograma
all = Todos
applets = Subprogramas
center-segment = Centrar segmento
drop-here = Soltar los subprogramas aquí
end-segment = Terminar segmento
large = Grande
no-applets-found = No se encontraron subprogramas...
panel-bottom = Inferior
panel-left = Izquierdo
panel-right = Derecho
panel-top = Superior
search-applets = Buscar subprogramas...
small = Pequeño
start-segment = Iniciar segmento

panel-appearance = Apariencia
    .match = Igualar con el escritorio
    .light = Claro
    .dark = Oscuro

panel-behavior-and-position = Comportamiento y posiciones
    .autohide = Ocultar la barra automáticamente
    .dock-autohide = Ocultar el panel automáticamente
    .position = Posición en la pantalla
    .display = Mostrar en pantalla

panel-style = Estilo
    .anchor-gap = Espacio entre la barra y los bordes de la pantalla
    .dock-anchor-gap = Espacio entre el panel y los bordes de la pantalla
    .extend = Extender la barra hasta los bordes de la pantalla
    .dock-extend = Extender el panel hasta los bordes de la pantalla
    .appearance = Apariencia
    .size = Tamaño
    .background-opacity = Opacidad del fondo

panel-applets = Configuración
    .dock-desc = Configurar los subprogramas del panel.
    .desc = Configurar los subprogramas de la barra.

panel-missing = No se ha encontrado la configuración de la barra
    .desc = El archivo de configuración de la barra se ha eliminado debido al uso de una configuración personalizada o está dañado.
    .fix = Restablecer a la configuración predeterminada.

## Desktop: Dock

dock = Panel
    .desc = Panel con aplicaciones ancladas.

## Desktop: Window Management

window-management = Gestión de ventanas
    .desc = Acción de la tecla Super, opciones de control de ventanas y opciones adicionales de mosaico de ventanas.

super-key = Tecla Súper
    .launcher = Abrir lanzador
    .workspaces = Abrir espacios de trabajo
    .applications = Abrir aplicaciones
    .disable = Deshabilitar

edge-gravity = Las ventanas flotantes gravitan hacia los bordes cercanos

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
    .desc = Configurar el número de espacios de trabajo, comportamiento y ubicación.

workspaces-behavior = Comportamiento de los espacios de trabajo
    .dynamic = Espacios de trabajo dinámicos
    .dynamic-desc = Elimina automáticamente los espacios de trabajo vacíos.
    .fixed = Número fijo de espacios de trabajo
    .fixed-desc = Añadir o eliminar espacios de trabajo en la vista general.

workspaces-multi-behavior = Comportamiento con múltiples pantallas
    .span = Un mismo espacio de trabajo se extiende a múltiples pantallas
    .separate = Cada pantalla tiene su propio espacio de trabajo

workspaces-overview-thumbnails = Miniaturas de la vista general de espacios de trabajo
    .show-number = Mostrar el número de los espacios de trabajo
    .show-name = Mostrar el nombre de los espacios de trabajo

workspaces-orientation = Orientación de los espacios de trabajo
    .vertical = Vertical
    .horizontal = Horizontal

hot-corner = Esquina activa
    .top-left-corner = Habilitar esquina activa superior izquierda para espacios de trabajo

## Desktop: Display

-requires-restart = Requiere reinicio

color = Color
    .depth = Profundidad de color
    .profile = Perfil de color
    .sidebar = Perfiles de color
    .temperature = Temperatura de color

display = Pantallas
    .desc = Gestionar pantallas, cambio de gráficos y luz nocturna
    .arrangement = Disposición de la pantalla
    .arrangement-desc = Arrastra las pantallas para reorganizarlas.
    .enable = Habilitar pantalla
    .external = Pantalla externa de { $size } { $output }
    .laptop = Pantalla de portátil de { $size }
    .options = Opciones de pantalla
    .refresh-rate = Frecuencia de actualización
    .resolution = Resolución
    .scale = Escala
    .additional-scale-options = Opciones de escalado adicionales

mirroring = Duplicar pantalla
    .id = Duplicado { $id }
    .dont = No duplicar
    .mirror = Duplicar { $display }
    .project = Proyectar a { $display ->
        [all] todas las pantallas
        *[other] { $display }
    }
    .project-count = Proyectando a { $count } otra(s) { $count ->
        [1] pantalla
        *[other] pantallas
    }

night-light = Luz nocturna
    .auto = Automático (de atardecer a amanecer)
    .desc = Reduce la luz azul usando colores más cálidos.

orientation = Orientación
    .standard = Estándar
    .rotate-90 = Rotar 90
    .rotate-180 = Rotar 180
    .rotate-270 = Rotar 270

vrr = Tasa de refresco variable
    .enabled = Habilitada
    .force = Siempre
    .auto = Automático
    .disabled = Deshabilitada

scheduling = Programación
    .manual = Programación manual

dialog = Diálogo
    .title = ¿Mantener estas configuraciones de pantalla?
    .keep-changes = Mantener cambios
    .change-prompt = Los cambios en la configuración se revertirán automáticamente en { $time } segundos.
    .revert-settings = Revertir configuraciones

## Sound

sound = Sonido
    .desc = N/A

sound-output = Salida de sonido
    .volume = Volumen de salida
    .device = Dispositivo de salida
    .level = Nivel de salida
    .config = Configuración
    .balance = Balance
    .left = Izquierda
    .right = Derecha

sound-input = Entrada de sonido
    .volume = Volumen de entrada
    .device = Dispositivo de entrada
    .level = Nivel de entrada

sound-alerts = Alertas
    .volume = Volumen de alertas
    .sound = Sonido de alertas

sound-applications = Aplicaciones
    .desc = Volúmen y configuración de sonido de aplicaciones

profile = Perfil

## Power

power = Energía
  .desc = Gestionar configuraciones de energía

battery = Batería
  .minute = { $value } { $value ->
        [one] minuto
       *[other] minutos
  }
  .hour = { $value } { $value ->
        [one] hora
       *[other] horas
  }
  .day = { $value } { $value ->
        [one] día
       *[other] días
  }
  .less-than-minute = Menos de un minuto
  .and = y
  .remaining-time = { $time } hasta { $action ->
        [full] completo
       *[other] vacío
   }

connected-devices = Dispositivos Conectados
  .unknown = Dispositivo desconocido

power-mode = Perfiles de Energía
    .battery = Modo Ahorro de Energía
    .battery-desc = Menor rendimiento pero inferior consumo de energía
    .balanced = Modo Balanceado
    .balanced-desc = Rendimiento y consumo de energía equilibrados
    .performance = Modo de Rendimiento
    .performance-desc = Máximo rendimiento pero mayor consumo de energía
    .no-backend = Backend no encontrado. Instala system76-power o power-profiles-daemon.

power-saving = Opciones de ahorro de energía
    .turn-off-screen-after = Apagar la pantalla después de
    .auto-suspend = Suspensión automática
    .auto-suspend-ac = Suspensión automática cuando el ordenador esté enchufado
    .auto-suspend-battery = Suspensión automática cuando esté usando la batería

## Input

acceleration-desc = Ajusta automáticamente la sensibilidad según la velocidad

disable-while-typing = Desactivar mientras se escribe

input-devices = Dispositivos de entrada
    .desc = Dispositivos de entrada

primary-button = Botón primario
    .desc = Establece el orden de los botones físicos.
    .left = Izquierdo
    .right = Derecho

scrolling = Desplazamiento
    .two-finger = Desplazar con dos dedos
    .edge = Desplazar a lo largo del borde con un dedo
    .speed = Velocidad de desplazamiento
    .natural = Desplazamiento natural
    .natural-desc = Desplazar el contenido, en lugar de la vista

## Input: Keyboard

slow = Lento
fast = Rápido
short = Corto
long = Largo
keyboard = Teclado
    .desc = Entrada de teclado, conmutación, caracteres especiales, atajos.

keyboard-sources = Fuentes de entrada de teclado
    .desc = Las fuentes de entrada se pueden cambiar usando la combinación de teclas Super+Espacio. Esto se puede personalizar en la configuración de atajos de teclado.
    .move-up = Mover arriba
    .move-down = Mover abajo
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
    .repeat-delay = Retardo de repetición

keyboard-numlock-boot = Bloq Num
    .boot-state = Estado al arrancar
    .last-boot = Último arranque
    .on = Activado
    .off = Desactivado
    .set = Establece el estado de Bloq Num al arrancar

added = Añadido
type-to-search = Escriba para buscar...
show-extended-input-sources = Mostrar fuentes de entrada extendidas

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Atajos de teclado
    .desc = Ver o personalizar los atajos

add-another-keybinding = Agregar atajos de teclas
cancel = Cancelar
command = Comando
custom = Personalizado
debug = Depuración
disabled = Desactivado
input-source-switch = Conmuta el idioma para la fuente de entrada del teclado
migrate-workspace-prev = Migrar espacio de trabajo a la salida anterior
migrate-workspace-next = Migrar espacio de trabajo a la salida siguiente
migrate-workspace = Migrar espacio de trabajo a la salida { $direction ->
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
    .focus = Enfocar ventana { $direction ->
        *[down] abajo
        [in] dentro
        [left] izquierda
        [out] fuera
        [right] derecha
        [up] arriba
    }
    .output = Cambiar a la salida { $direction ->
        *[down] abajo
        [left] izquierda
        [right] derecha
        [up] arriba
    }
    .workspace = Cambiar al espacio de trabajo { $num }

manage-windows = Gestionar ventanas
    .close = Cerrar ventana
    .maximize = Maximizar ventana
    .minimize = Minimizar ventana
    .resize-inwards = Redimensionar ventana hacia adentro
    .resize-outwards = Redimensionar ventana hacia afuera
    .toggle-sticky = Cambiar a ventana siempre visible

move-windows = Mover ventanas
    .direction = Mover ventana { $direction ->
        *[down] abajo
        [left] izquierda
        [right] derecha
        [up] arriba
    }
    .display = Mover ventana una pantalla { $direction ->
        *[down] abajo
        [left] izquierda
        [right] derecha
        [up] arriba
    }
    .workspace = Mover ventana un espacio de trabajo { $direction ->
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
    .home-folder = Abrir la carpeta de inicio
    .keyboard-brightness-down = Disminuir el brillo del teclado
    .keyboard-brightness-up = Aumentar el brillo del teclado
    .launcher = Abrir el lanzador
    .log-out = Cerrar sessión
    .lock-screen = Bloquear la pantalla
    .mute = Silenciar salida de audio
    .mute-mic = Silenciar entrada de micrófono
    .play-pause = Reproducir/Pausar
    .play-next = Siguiente pista
    .play-prev = Pista anterior
    .screenshot = Tomar una captura de pantalla
    .terminal = Abrir la terminal
    .volume-lower = Disminuir el volumen de la salida de audio
    .volume-raise = Aumentar el volumen de la salida de audio
    .web-browser = Abrir el navegador web
    .window-switcher = Cambiar entre las ventanas abiertas
    .window-switcher-previous = Cambiar entre las ventanas abiertas al revés
    .workspace-overview = Abrir la vista general de espacios de trabajo

window-tiling = Organización de ventanas
    .horizontal = Establecer orientación horizontal
    .vertical = Establecer orientación vertical
    .swap-window = Intercambiar ventana
    .toggle-tiling = Ordenar ventanas
    .toggle-stacking = Cambiar a ventanas apiladas
    .toggle-floating = Cambiar a ventanas flotantes
    .toggle-orientation = Activar orientación

replace-shortcut-dialog = ¿Reemplazar acceso directo?
    .desc = { $shortcut } está en uso por { $name }. Si lo reemplazas, { $name } será desactivado.

zoom-in = Ampliar
zoom-out = Disminuir

## Input: Mouse

mouse = Ratón
    .desc = Velocidad del ratón, aceleración, desplazamiento natural.
    .speed = Velocidad del ratón
    .acceleration = Habilitar la aceleración del ratón

## Input: Touchpad

click-behavior = Comportamiento del Clic
    .click-finger = Clic secundario con dos dedos y clic medio con tres dedos
    .button-areas = Clic secundario en la esquina inferior derecha y clic medio en el centro inferior

pinch-to-zoom = Pellizcar para ampliar
    .desc = Usar dos dedos para ampliar el contenido, para aplicaciones que soportan zoom

tap-to-click = Tocar para hacer clic
    .desc = Habilita el toque con un solo dedo para el clic primario, el toque con dos dedos para el clic secundario y el toque con tres dedos para el clic medio

touchpad = Panel táctil
    .acceleration = Habilitar aceleración del panel táctil
    .desc = Velocidad del panel táctil, opciones de clic, gestos.
    .speed = Velocidad del panel táctil

## Input: Gestures

swiping = Deslizar
    .four-finger-down = Deslizar con cuatro dedos hacia abajo
    .four-finger-left = Deslizar con cuatro dedos hacia la izquierda
    .four-finger-right = Deslizar con cuatro dedos hacia la derecha
    .four-finger-up = Deslizar con cuatro dedos hacia arriba
    .three-finger-any = Deslizar con tres dedos en cualquier dirección

switch-workspaces = Cambiar espacios de trabajo
    .horizontal = Deslizar cuatro dedos a la izquierda/derecha
    .vertical = Deslizar cuatro dedos hacia arriba/abajo

switch-between-windows = Cambiar entre ventanas
open-application-library = Abrir Biblioteca de aplicaciones
open-workspaces-view = Abrir Vista de espacios de trabajo

## Time & Language

time = Hora e Idioma
    .desc = N/A

time-date = Fecha y Hora
    .desc = Zona horaria, configuración automática del reloj y formatos de hora
    .auto = Establecer automáticamente
    .auto-ntp = La fecha y la hora se actualizarán automáticamente cuando se establezca la zona horaria.

time-zone = Zona horaria
    .auto = Zona horaria automática
    .auto-info = Requiere servicios de ubicación y acceso a internet

time-format = Formato de fecha y hora
    .twenty-four = Formato de 24 horas
    .show-seconds = Mostrar segundos
    .first = Primer día de la semana
    .show-date = Mostrar fecha en la barra
    .friday = Viernes
    .saturday = Sábado
    .sunday = Domingo
    .monday = Lunes

time-region = Región e Idioma
    .desc = Cambiar formato de fechas, horas y números según tu región

formatting = Formato
    .dates = Fechas
    .time = Horarios
    .date-and-time = Fecha & Horario
    .numbers = Números
    .measurement = Medidas
    .paper = Papel

preferred-languages = Idiomas preferidos
    .desc = El orden de los idiomas determina que idioma será usado para la traducción del escritorio. Los cambios se aplicarán en el siguiente inicio de sesión.

add-language = Añadir idioma
    .context = Añadir idioma
install-additional-languages = Instalar idiomas adicionales
region = Región

## Applications

applications = Aplicaciones

## Applications: Default Applications

default-apps = Aplicaciones predeterminadas
    .desc = Navegador web, cliente de correo, explorador de archivos y otras aplicaciones predeterminadas.
    .web-browser = Navegador web
    .file-manager = Explorador de archivos
    .mail-client = Cliente de correo
    .music = Música
    .video = Vídeo
    .photos = Fotografías
    .calendar = Calendario
    .terminal = Consola
    .other-associations = Otras asociaciones
    .text-editor = Editor de texto

## Applications: Startup Applications

startup-apps = Aplicaciones de inicio
    .desc = Configure aplicaciones que se ejecutan al iniciar sesión.
    .add = Añadir aplicación
    .user = Aplicaciones específicas del usuario
    .user-description = Estas aplicaciones se ejecutarán cuando inicies sesión con tu usuario actual.
    .none = No se han añadido aplicaciones de inicio
    .remove-dialog-title = Eliminar { $name }?
    .remove-dialog-description = ¿Estás seguro que quieres eliminarla como aplicación de inicio?
    .search-for-application = Buscar una aplicación

## Applications: Legacy Applications

legacy-applications = Compatibilidad para aplicaciones X11
    .desc = Escalado de las aplicaciones y atajos globales para el Sistema de Ventanas X11.

legacy-app-global-shortcuts = Atajos globales para aplicaciones X11
    .desc = Los atajos globales permiten que las pulsaciones de teclas y los eventos de los botones del ratón que ocurren en las aplicaciones sean reconocidos por otras aplicaciones para poder llevar a cabo funcionalidades como pulsar-para-hablar o pulsar-para-silenciar. Por defecto, esta opción se encuentra deshabilitada para asegurar que otras aplicaciones no pueden monitorear los eventos del teclado y el ratón que contienen información sensible.
    .none = Ninguna tecla
    .modifiers = Modificadores (Super, Shift, Control, Alt)
    .combination = Todas las teclas mientras los modificadores Super, Control o Alt son pulsados.
    .all = Todas las teclas
    .mouse = Eventos de los botones del ratón en aplicaciones X11

legacy-app-scaling = Escalado de las aplicaciones del sistema de ventanas X11
    .scaled-gaming = Optimizar para videojuegos y aplicaciones en pantalla completa.
    .gaming-description = Las aplicaciones X11 quizás aparezcan ligeramente más pequeñas o grandes en comparación a las aplicaciones de Wayland.
    .scaled-applications = Optimiza para aplicaciones
    .applications-description = Los videojuegos y las aplicaciones X11 en pantalla completa quizás no coincidan con la resolución del monitor.
    .scaled-compatibility = Modo de compatibilidad máxima
    .compatibility-description = Las aplicaciones X11 podrían aparecer difuminadas en monitores HiDPI.
    .preferred-display = Monitor preferido para videojuegos y aplicaciones X11 en pantalla completa
    .no-display = Ninguno

## System

system = Sistema y Cuentas

## System: About

about = Acerca de
    .desc = Nombre del dispositivo, información de los componentes, valores predeterminados del sistema operativo.

about-device = Nombre del dispositivo
    .desc = Este nombre identifica a este equipo en otros dispositivos de red o bluetooth.

about-hardware = Componentes físicos
    .model = Placa Base
    .memory = Memoria
    .processor = Procesador
    .graphics = Gráficos
    .disk-capacity = Capacidad del disco duro

about-os = Sistema Operativo
    .os = Sistema operativo
    .os-architecture = Arquitectura del sistema operativo
    .desktop-environment = Entorno de escritorio
    .windowing-system = Sistema de ventanas

about-related = Configuraciones relacionadas
    .support = Obtener soporte

## System: Firmware

firmware = Firmware
    .desc = Detalles del firmware

## System: Users

users = Usuarios
    .desc = Autenticación e inicio de sesión, pantalla de bloqueo
    .admin = Administrador
    .standard = Estándar
    .profile-add = Elige una imagen de perfil

administrator = Administrador
    .desc = Los administradores pueden cambiar las opciones para todos los usuarios, añadir y eliminar otros usuarios.

add-user = Añadir usuario
change-password = Cambiar la contraseña
remove-user = Eliminar usuario
full-name = Nombre completo
invalid-username = Nombre de usuario inválido
password-mismatch = Las contraseñas no coinciden.
save = Guardar
