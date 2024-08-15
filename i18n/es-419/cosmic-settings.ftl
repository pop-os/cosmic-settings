app = Configuración de COSMIC

unknown = Desconocido

number = { $number }

## Desktop

desktop = Escritorio

## Desktop: Apariencia

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
    .desc-detail = El color de fondo del contenedor se utiliza para la barra lateral de navegación, el cajón lateral, los diálogos y «widgets» similares. Por defecto, se deriva automáticamente del fondo de la aplicación o ventana.
    .reset = Restablecer a automático
    .desc = El color principal del contenedor se utiliza para la barra lateral de navegación, el cajón lateral, los diálogos y «widgets» similares.

control-tint = Tono del componente de control
    .desc = Se utiliza para los fondos de los botones estándar, entradas de búsqueda, entradas de texto y componentes similares.

frosted = Efecto de cristal translúcido en la interfaz del sistema
    .desc = Aplica desenfoque de fondo a paneles, «docks», «applets», lanzador y biblioteca de aplicaciones.

experimental-settings = Configuraciones experimentales

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

# interface density left out for now
window-management = Gestión de ventanas
    .active-hint = Tamaño del contorno destacado de la ventana en uso
    .gaps = Margenes de las ventanas organizadas

## Desktop: Display

-requires-restart = Requiere reinicio

color = Color
    .depth = Profundidad de color
    .profile = Perfil de color
    .sidebar = Perfiles de color
    .temperature = Temperatura de color

display = Pantallas
    .desc = Gestionar pantallas, cambio de gráficos y luz nocturna
    .arrangement = Disposición de pantalla
    .arrangement-desc = Arrastra las pantallas para reorganizarlas.
    .enable = Habilitar pantalla
    .external = Pantalla externa de { $size } { $output }
    .laptop = Pantalla de «laptop» de { $size }
    .options = Opciones de pantalla
    .refresh-rate = Frecuencia de actualización
    .resolution = Resolución
    .scale = Escala

mirroring = Duplicar pantalla
    .id = Duplicado { $id }
    .dont = No duplicar
    .mirror = Duplicar { $display }
    .project = Proyectar a { $display ->
        [all] todas las pantallas
        *[other] { $display }
    }
    .project-count = Proyectando a { $count} otra(s) { $count ->
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

## Desktop: Notifications

notifications = Notifications
    .desc = No molestar, notificaciones en la pantalla de bloqueo y configuraciones por aplicación.

## Desktop: Options

desktop-panel-options = Escritorio y panel
    .desc = Acción de la tecla Súper, esquinas activas, opciones de control de ventanas.

desktop-panels-and-applets = Paneles de escritorio y «applets»

dock = «Dock»
    .desc = Panel con aplicaciones fijadas.

hot-corner = Esquina activa
    .top-left-corner = Habilitar esquina activa superior izquierda para espacios de trabajo

super-key = Tecla Súper
    .launcher = Abrir lanzador
    .workspaces = Abrir espacios de trabajo
    .applications = Abrir aplicaciones

top-panel = Panel superior
    .workspaces = Mostrar botón de espacios de trabajo
    .applications = Mostrar botón de aplicaciones

window-controls = Controles de ventana
    .minimize = Mostrar botón de minimizar
    .maximize = Mostrar botón de maximizar

## Desktop: Panel

panel = Panel
    .desc = Barra superior con controles y menús del escritorio.

add = Añadir
add-applet = Añadir «applet»
all = Todos
applets = «Applets»
center-segment = Segmento central
drop-here = Soltar «applets» aquí
end-segment = Segmento final
large = Grande
no-applets-found = No se encontraron «applets»...
panel-bottom = Abajo
panel-left = Izquierda
panel-right = Derecha
panel-top = Arriba
search-applets = Buscar «applets»...
small = Pequeño
start-segment = Segmento inicial

panel-appearance = Apariencia
    .match = Igual que el escritorio
    .light = Claro
    .dark = Oscuro

panel-behavior-and-position = Comportamiento y posiciones
    .autohide = Ocultar panel automáticamente
    .dock-autohide = Ocultar «dock» automáticamente
    .position = Posición en la pantalla
    .display = Mostrar en pantalla

panel-style = Estilo
    .anchor-gap = Espacio entre el panel y los bordes de la pantalla
    .dock-anchor-gap = Espacio entre el «dock» y los bordes de la pantalla
    .extend = Extender panel hasta los bordes de la pantalla
    .dock-extend = Extender «dock» hasta los bordes de la pantalla
    .appearance = Apariencia
    .size = Tamaño
    .background-opacity = Opacidad del fondo

panel-applets = Configuración
    .dock-desc = Configurar «applets» del «dock».
    .desc = Configurar «applets» del panel.

panel-missing = Falta la configuración del panel
    .desc = El archivo de configuración del panel falta debido al uso de una configuración personalizada o está dañado.
    .fix = Restablecer a predeterminado

## Desktop: Wallpaper

wallpaper = Fondo de pantalla
    .change = Cambiar imagen cada
    .desc = Imágenes de fondo de pantalla, colores y opciones de carrusel de imágenes.
    .fit = Ajuste del fondo de pantalla
    .folder-dialog = Elegir carpeta de fondos de pantalla
    .image-dialog = Elegir imagen de fondo de pantalla
    .plural = Fondos de pantalla
    .same = Mismo fondo de pantalla en todas las pantallas
    .slide = Carrusel de imágenes

add-color = Añadir color
add-image = Añadir imagen
all-displays = Todas las pantallas
colors = Colores
dialog-add = Añadir
fill = Rellenar
fit-to-screen = Ajustar a la pantalla
open-new-folder = Abrir nueva carpeta
recent-folders = Carpetas recientes

x-minutes = { $number } minutos
x-hours = { $number ->
    [1] 1 hora
    *[other] { $number } horas
}

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

## Networking: Wired

wired = Conexión cableada
    .desc = Conexión cableada, perfiles de conexión

## Networking: Online Accounts

online-accounts = Cuentas en línea
    .desc = Añade cuentas, IMAP y SMTP, inicio de sesión empresarial

## Time & Language

time = Hora y idioma
    .desc = N/A

time-date = Fecha y hora
    .desc = Zona horaria, configuraciones automáticas del reloj y algunas configuraciones de hora.
    .auto = Establecer automáticamente

time-zone = Zona horaria
    .auto = Zona horaria automática
    .auto-info = Requiere servicios de ubicación y acceso a internet

time-format = Formato de fecha y hora
    .twenty-four = Formato de 24 horas
    .first = Primer día de la semana
    .show-date = Mostrar fecha en el panel superior
    .friday = Viernes
    .saturday = Sábado
    .sunday = Domingo
    .monday = Lunes

time-region = Región e idioma
    .desc = Formato de fechas, horas y números según tu región

## Sound

sound = Sonido
    .desc = N/A

sound-output = Salida
    .volume = Volumen de salida
    .device = Dispositivo de salida
    .level = Nivel de salida
    .config = Configuración
    .balance = Balance

sound-input = Entrada
    .volume = Volumen de entrada
    .device = Dispositivo de entrada
    .level = Nivel de entrada

sound-alerts = Alertas
    .volume = Volumen de alertas
    .sound = Sonido de alertas

sound-applications = Aplicaciones
    .desc = Volúmen y configuración de audio de aplicaciones

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
    .desc = Autenticación e inicio de sesión, pantalla de bloqueo.

## Input

acceleration-desc = Ajusta automáticamente la sensibilidad de seguimiento según la velocidad.

disable-while-typing = Desactivar mientras se escribe

input-devices = Dispositivos de entrada
    .desc = Dispositivos de entrada

primary-button = Botón primario
    .left = Izquierdo
    .right = Derecho

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
    .desc = Entrada del teclado

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

keyboard-typing-assist = Escritura
    .repeat-rate = Tasa de repetición
    .repeat-delay = Retraso de repetición

added = Añadido
type-to-search = Escriba para buscar...

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Atajos de teclado
    .desc = Ver y personalizar atajos

add-keybinding = Agregar atajos de teclas
cancel = Cancelar
command = Comando
custom = Personalizado
debug = Depuración
disabled = Desactivado
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
    .lock-screen = Bloquear la pantalla
    .mute = Silenciar salida de audio
    .mute-mic = Silenciar entrada de micrófono
    .screenshot = Tomar una captura de pantalla
    .terminal = Abrir una terminal
    .volume-lower = Disminuir el volumen de la salida de audio
    .volume-raise = Aumentar el volumen de la salida de audio
    .web-browser = Abrir un navegador web
    .window-switcher = Cambiar entre ventanas abiertas
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
    .desc = Activa el toque con un dedo para clic primario, toque con dos dedos para clic secundario y toque con tres dedos para clic medio.

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

switch-between-windows = Cambiar entre ventanas
switch-to-next-workspace = Cambiar al espacio de trabajo siguiente
switch-to-prev-workspace = Cambiar al espacio de trabajo anterior
open-application-library = Abrir biblioteca de aplicaciones
open-workspaces-view = Abrir vista de espacios de trabajo

## Power

power = Energía
    .desc = Gestionar ajustes de energía

power-mode = Modo de energía
    .performance = Alto rendimiento
    .balanced = Balanceado
    .battery = Vida extendida de la batería
    .performance-desc = Rendimiento y uso de energía alto.
    .balanced-desc = Rendimiento y uso de energía estándar.
    .battery-desc = Rendimiento y uso de energía reducido.
    .nobackend = «Backend» no encontrado. Instalar system76-power o power-profiles-daemon.
