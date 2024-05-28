app = Configuración de COSMIC

unknown = Desconocido

number = { $number }

## Desktop

desktop = Escritorio

## Desktop: Appearance

appearance = Apariencia
    .desc = Colores de énfasis y creación de temas COSMIC.

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
    .desc = Aplica un efecto de vidrio esmerilado al panel, dock, applets, lanzador y biblioteca de aplicaciones.

experimental-settings = Configuraciones experimentales

enable-export = Aplicar este tema a las aplicaciones GNOME.
    .desc = No todos los toolkits soportan el cambio automático. Las aplicaciones que no sean COSMIC pueden necesitar ser reiniciadas después de un cambio de tema.

icon-theme = Tema de iconos
    .desc = Aplica un conjunto diferente de iconos a las aplicaciones.

text-tint = Tonalidad del texto de la interfaz
    .desc = Color utilizado para generar colores de texto de la interfaz con buen contraste en diferentes fondos.

style = Estilo
    .round = Redondo
    .slightly-round = Ligeramente redondo
    .square = Cuadrado

# interface density left out for now
window-management = Gestión de ventanas
    .active-hint = Tamaño de la indicación de ventana activa
    .gaps = Espacios alrededor de las ventanas en mosaico

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

scheduling = Programación
    .manual = Programación manual

## Desktop: Notifications

notifications = Notificaciones
    .desc = No molestar, notificaciones en la pantalla de bloqueo y configuración por aplicación.

## Desktop: Options

desktop-panel-options = Escritorio y Panel
    .desc = Acción de la tecla Super, esquinas activas, opciones de control de ventanas.

desktop-panels-and-applets = Paneles y Applets del Escritorio

dock = Dock
    .desc = Panel con aplicaciones ancladas.

hot-corner = Esquina Activa
    .top-left-corner = Habilitar esquina superior izquierda para Espacios de Trabajo

super-key-action = Acción de la tecla Super
    .launcher = Lanzador
    .workspaces = Espacios de Trabajo
    .applications = Aplicaciones

top-panel = Panel Superior
    .workspaces = Mostrar botón de Espacios de Trabajo
    .applications = Mostrar botón de Aplicaciones

window-controls = Controles de ventana
    .minimize = Mostrar botón de minimizar
    .maximize = Mostrar botón de maximizar

## Desktop: Panel

panel = Panel
    .desc = Barra superior con controles y menús del escritorio.

add = Añadir
add-applet = Añadir Applet
all = Todos
applets = Applets
center-segment = Centrar segmento 
drop-here = Soltar applets aquí
end-segment = Terminar segmento
large = Grande
no-applets-found = No se encontraron applets...
panel-bottom = Inferior
panel-left = Izquierdo
panel-right = Derecho
panel-top = Superior
search-applets = Buscar applets...
small = Pequeño
start-segment = Iniciar segmento

panel-appearance = Apariencia
    .match = Igualar con el escritorio
    .light = Claro
    .dark = Oscuro

panel-behavior-and-position = Comportamiento y posiciones
    .autohide = Ocultar panel automáticamente
    .dock-autohide = Ocultar dock automáticamente
    .position = Posición en la pantalla
    .display = Mostrar en pantalla

panel-style = Estilo
    .anchor-gap = Espacio entre el panel y los bordes de la pantalla
    .dock-anchor-gap = Espacio entre el dock y los bordes de la pantalla
    .extend = Extender panel hasta los bordes de la pantalla
    .dock-extend = Extender dock hasta los bordes de la pantalla
    .appearance = Apariencia
    .size = Tamaño
    .background-opacity = Opacidad del fondo

panel-applets = Configuración
    .dock-desc = Configurar applets del dock.
    .desc = Configurar applets del panel.

panel-missing = No se ha encontrado una configuración del panel
    .desc = El archivo de configuración del panel se ha eliminado debido al uso de una configuración personalizada o está dañado.
    .fix = Restablecer a configuración por defecto

## Desktop: Wallpaper

wallpaper = Fondo de Pantalla
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
dialog-add = _Añadir
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
    .desc = Configurar número de espacios de trabajo, comportamiento y ubicación.

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

workspaces-orientation = Orientación de espacios de trabajo
    .vertical = Vertical
    .horizontal = Horizontal

## Networking: Wired

wired = Conexión cableada
    .desc = Conexión cableada, perfiles de conexión

## Networking: Online Accounts

online-accounts = Cuentas en línea
    .desc = Añadir cuentas, IMAP y SMTP, inicios de sesión de empresa

## Time & Language

time = Hora e Idioma
    .desc = N/A

time-date = Fecha y Hora
    .desc = Zona horaria, configuración automática del reloj y formatos de hora
    .auto = Establecer automáticamente

time-zone = Zona horaria
    .auto = Zona horaria automática
    .auto-info = Requiere servicios de ubicación y acceso a internet

time-format = Formato de fecha y hora
    .twenty-four = Formato de 24 horas
    .first = Primer día de la semana
    .show-date = Mostrar fecha en el Panel Superior
    .friday = Viernes
    .saturday = Sábado
    .sunday = Domingo
    .monday = Lunes

time-region = Región e Idioma
    .desc = Cambiar formato de fechas, horas y números según tu región

## Sound

sound = Sonido
    .desc = N/A

sound-output = Salida de sonido
    .volume = Volumen de salida
    .device = Dispositivo de salida
    .level = Nivel de salida
    .config = Configuración
    .balance = Balance

sound-input = Entrada de sonido
    .volume = Volumen de entrada
    .device = Dispositivo de entrada
    .level = Nivel de entrada

sound-alerts = Alertas 
    .volume = Volumen de alertas
    .sound = Sonido de alertas

sound-applications = Aplicaciones
    .desc = Volúmen y configuración de sonido de aplicaciones

## System

system = Sistema y Cuentas

## System: About

about = Acerca de
    .desc = Nombre del dispositivo, información del hardware, valores por defecto del sistema operativo.

about-device = Nombre del dispositivo
    .desc = Este nombre aparece en otros dispositivos de red o bluetooth.

about-hardware = Hardware
    .model = Modelo de hardware
    .memory = Memoria
    .processor = Procesador
    .graphics = Gráficos
    .disk-capacity = Capacidad del disco

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

## Input

acceleration-desc = Ajusta automáticamente la sensibilidad de seguimiento según la velocidad

disable-while-typing = Desactivar mientras se escribe

input-devices = Dispositivos de entrada
    .desc = Dispositivos de entrada

primary-button = Botón primario
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
    .desc = Entrada de teclado

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

keyboard-typing-assist = Escritura
    .repeat-rate = Tasa de repetición
    .repeat-delay = Retardo de repetición

added = Añadido
type-to-search = Escriba para buscar...

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Atajos de teclado
    .desc = Ver y personalizar atajos

## Input: Mouse

mouse = Ratón
    .desc = Velocidad del ratón, aceleración, desplazamiento natural.
    .speed = Velocidad del ratón
    .acceleration = Habilitar aceleración del ratón

## Input: Touchpad

click-behavior = Comportamiento del Clic
    .click-finger = Clic secundario con dos dedos y clic medio con tres dedos
    .button-areas = Clic secundario en la esquina inferior derecha y clic medio en el centro inferior

pinch-to-zoom = Pellizcar para acercar
    .desc = Usar dos dedos para acercar el contenido, para aplicaciones que soportan zoom

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

switch-between-windows = Cambiar entre ventanas
switch-to-next-workspace = Cambiar al siguiente espacio de trabajo
switch-to-prev-workspace = Cambiar al espacio de trabajo anterior
open-application-library = Abrir Biblioteca de aplicaciones
open-workspaces-view = Abrir Vista de espacios de trabajo

## Power 

power = Energía
  .desc = Gestionar configuraciones de energía 

power-profiles = Perfiles de Energía
  .performance = Modo de Rendimiento
  .balanced = Modo Balanceado
  .battery = Modo de Ahorro de energía
  .performance-desc = Máximo rendimiento pero alto consumo de energía
  .balanced-desc = Rendimiento y consumo de energía equilibrados
  .battery-desc = Bajo rendimiento pero bajo consumo de energía
