app = Configurações do COSMIC

dbus-connection-error = Falha de conexão no DBus
ok = OK
unknown = Desconhecido

number = { $number }

## Network & Wireless

connections-and-profiles = { $variant ->
    [wired] Conexões cabeadas
    [wifi] Conexões Wi-Fi
    [vpn] Conexões VPN
    *[other] Conexões desconhecidas
} e perfis de conexão.

add-network = Adicionar rede
    .profile = Adicionar perfil
add-vpn = Adicionar VPN
airplane-on = Modo avião está ativo.
cable-unplugged = Cabo desconectado
connect = Conectar
connected = Conectado
connecting = Conectando…
disconnect = Desconectar
forget = Esquecer
known-networks = Redes conhecidas
network-and-wireless = Rede e Wireless
no-networks = Nenhuma rede foi encontrada.
no-vpn = Nenhuma conexão VPN está disponível.
password = Senha
remove = Remover
settings = Configurações
username = Usuário
visible-networks = Redes visíveis

auth-dialog = Autenticação
    .vpn-description = Digite o usuário e a senha exigidos pelo serviço de VPN.
    .wifi-description = Digite a senha ou chave de criptografia. Você também pode conectar pressionando o botão “WPS” no roteador.

forget-dialog = Esquecer esta rede Wi-Fi?
    .description = Você precisará informar uma senha novamente para usar esta rede no futuro.

network-device-state =
    .activated = Conectado
    .config = Conectando
    .deactivating = Desconectando
    .disconnected = Desconectado
    .failed = Falha ao conectar
    .ip-check = Checando conexão
    .ip-config = Solicitando IP e informações de roteamento
    .need-auth = Necessita de autenticação
    .prepare = Preparando para conectar
    .secondaries = Aguardando conexão secundária
    .unavailable = Indisponível
    .unknown = Estado desconhecido
    .unmanaged = Não gerenciado
    .unplugged = Cabo desconectado

remove-connection-dialog = Remover perfil de conexão?
    .vpn-description = Você precisará informar uma senha novamente para usar esta rede no futuro.
    .wired-description = Você precisará recriar este perfil para usá-lo no futuro.

vpn = VPN
    .connections = Conexões VPN
    .error = Falha ao adicionar a configuração da VPN
    .remove = Remover perfil de conexão
    .select-file = Selecione um arquivo de conexão VPN

vpn-error = Erro de VPN
    .config = Falha ao adicionar a configuração da VPN
    .connect = Falha ao conectar na VPN
    .connection-editor = Falha de edição da conexão
    .connection-settings = Falha ao obter as configurações das conexões ativas
    .updating-state = Falha ao atualizar o status do gerenciador de rede
    .wireguard-config-path = Caminho do arquivo inválido para a configuração do WireGuard
    .wireguard-config-path-desc = O arquivo selecionado deve estar em um sistema de arquivos local.
    .wireguard-device = Falha ao criar um dispositivo WireGuard
    .with-password = Falha ao definir VPN { $field ->
        *[username] usuário
        [password] senha
        [password-flags] sinalizadores de senha
    } com nmcli

wired = Rede Cabeada
    .adapter = Adaptador de rede cabeada { $id }
    .connections = Conexões de rede cabeada
    .devices = Dispositivos de rede cabeada
    .remove = Remover perfil de conexão

wifi = Wi-Fi
    .adapter = Adaptador Wi-Fi { $id }
    .forget = Esquecer esta rede

wireguard-dialog = Adicionar dispositivo WireGuard
    .description = Escolha um nome de dispositivo para a configuração do WireGuard.

## Networking: Online Accounts

online-accounts = Contas online
    .desc = Adicionar contas, IMAP e SMTP, logins empresariais

# Bluetooth

confirm = Confirmar

bluetooth = Bluetooth
    .desc = Gerenciamento de dispositivos Bluetooth
    .status = Este sistema está visível como { $aliases } enquanto a configuração do Bluetooth estiver aberta.
    .connected = Conectado
    .connecting = Conectando
    .disconnecting = Desconectando
    .connect = Conectar
    .disconnect = Desconectar
    .forget = Esquecer
    .dbus-error = Ocorreu um erro ao interagir com o DBus: { $why }
    .show-device-without-name = Mostrar dispositivo sem nome

bluetooth-paired = Dispositivos conectados anteriormente
    .connect = Conectar
    .battery = { $percentage }% bateria

bluetooth-confirm-pin = Confirmar o PIN do Bluetooth
    .description = Por favor, confirme se o seguinte PIN corresponde ao exibido em { $device }

bluetooth-available = Dispositivos próximos

bluetooth-adapters = Adaptadores de Bluetooth

## Desktop

desktop = Desktop

## Desktop: Wallpaper

wallpaper = Papel de Parede
    .change = Mudar imagem a cada
    .desc = Imagens de papel de parede, cores e opções de apresentação de slides.
    .fit = Ajuste do Papel de Parede
    .folder-dialog = Escolher pasta do papel de parede
    .image-dialog = Escolher imagem do papel de parede
    .plural = Papéis de Parede
    .same = Mesmo papel de parede em todos os monitores
    .slide = Apresentação de slides

add-color = Adicionar cor
add-image = Adicionar imagem
all-displays = Todos os Monitores
colors = Cores
dialog-add = Adicionar
fill = Preencher
fit-to-screen = Ajustar à Tela
open-new-folder = Abrir Nova Pasta
recent-folders = Pastas Recentes

x-minutes = { $number } minutos
x-hours = { $number ->
    [1] 1 hora
    *[other] { $number } horas
}
never = Nunca

## Desktop: Appearance

appearance = Aparência
    .desc = Cores de destaque e temas.

accent-color = Cor de destaque
app-background = Fundo da aplicação ou da janela
auto = Automático
close = Fechar
color-picker = Seletor de cores
copied-to-clipboard = Copiado para a área de transferência
copy-to-clipboard = Copiar para a área de transferência
dark = Escuro
export = Exportar
hex = Hex
import = Importar
light = Claro
mode-and-colors = Modo e Cores
recent-colors = Cores recentes
reset-to-default = Restaurar padrão
rgb = RGB
window-hint-accent = Cor de destaque da janela ativa
window-hint-accent-toggle = Utilizar a cor de destaque do tema como destaque de janela ativa

auto-switch = Alternar automaticamente do modo Claro para o modo Escuro
    .sunrise = Altera para o modo Claro ao nascer do sol
    .sunset = Altera para o modo Escuro ao pôr do sol
    .next-sunrise = Altera para o modo Claro no próximo nascer do sol
    .next-sunset = Altera para o modo Escuro no próximo pôr do sol

container-background = Fundo do contêiner
    .desc-detail = A cor de fundo do contêiner é usada para a barra lateral de navegação, gaveta lateral, nas caixas de diálogos e em widgets similares. Por padrão, ela é automaticamente derivada do fundo da aplicação ou da janela.
    .reset = Restaurar para automático
    .desc = A cor primária do contêiner é usada para a barra lateral de navegação, gaveta lateral, caixas de diálogos, e widgets similares.

control-tint = Tonalidade dos componentes de controle
    .desc = Usado para os fundos dos botões padrão, entradas de busca, entradas de texto e componentes similares.

frosted = Efeito de vidro fosco na interface do sistema
    .desc = Ativa desfoque de fundo ao painel, dock, applets, lançador e biblioteca de aplicativos.

enable-export = Aplicar este tema para aplicativos GNOME.
    .desc = Nem todos os toolkits suportam troca automática. Aplicativos não COSMIC podem precisar ser reiniciados após uma mudança de tema.

icon-theme = Tema de ícones
    .desc = Aplica um conjunto diferente de ícones para aplicativos.

text-tint = Tonalidade do texto da interface
    .desc = Cor usada para derivar cores de texto da interface que têm contraste suficiente em várias superfícies.

style = Estilo
    .round = Arredondado
    .slightly-round = Levemente arredondado
    .square = Quadrado

interface-density = Densidade da interface
    .comfortable = Confortável
    .compact = Compacto
    .spacious = Espaçoso

window-management = Gerenciamento de Janelas
    .active-hint = Tamanho da cor destaque da janela ativa
    .gaps = Espaçamentos ao redor de janelas organizadas em mosaico

### Experimental

experimental-settings = Configuraçães experimentais
icons-and-toolkit = Tema de ícones e toolkit
interface-font = Fonte do sistema
monospace-font = Fonte monoespaçada

## Desktop: Notifications

notifications = Notificações
    .desc = Não perturbe, notificações da tela de bloqueio e configurações por aplicação.

## Desktop: Panel

panel = Painel
    .desc = Barra superior com controles e menus da área de trabalho.

add = Adicionar
add-applet = Adicionar Applet
all = Todos
applets = Applets
center-segment = Segmento Central
drop-here = Solte os applets aqui
end-segment = Segmento final
large = Grande
no-applets-found = Nenhum applet encontrado...
panel-bottom = Inferior
panel-left = Esquerda
panel-right = Direita
panel-top = Superior
search-applets = Procurar por applets...
small = Pequeno
start-segment = Segmento inicial

panel-appearance = Aparência
    .match = Corresponder à área de trabalho
    .light = Claro
    .dark = Escuro

panel-behavior-and-position = Comportamento e Posições do Painel
    .autohide = Ocultar o painel automaticamente
    .dock-autohide = Ocultar dock automaticamente
    .position = Posição na tela
    .display = Mostrar no monitor

panel-style = Estilo
    .anchor-gap = Espaço entre o painel e as bordas da tela
    .dock-anchor-gap = Espaço entre o dock e as bordas da tela
    .extend = Estender o painel até as bordas da tela
    .dock-extend = Estender o dock até as bordas da tela
    .appearance = Aparência
    .size = Tamanho
    .background-opacity = Opacidade do fundo

panel-applets = Configuração
    .dock-desc = Configurar applets do dock.
    .desc = Configurar applets do painel.

panel-missing = Configuração do painel está ausente
    .desc = O arquivo de configuração do painel está ausente devido ao uso de uma configuração personalizada ou encontra-se corrompido.
    .fix = Redefinir para padrão

## Desktop: Dock

dock = Dock
    .desc = Painel com aplicativos fixados na bandeja de apps e outros applets.

## Desktop: Window Management

window-management = Gerenciamento de Janelas
    .desc = Ação da tecla Super, opções de controle e de alinhamento de janelas.

super-key = Ação da tecla Super (Windows)
    .launcher = Abrir Lançador
    .workspaces = Abrir Áreas de Trabalho
    .applications = Abrir Aplicativos
    .disable = Desabilitar

window-controls = Controle de Janelas
    .minimize = Mostrar o botão de minimizar
    .maximize = Mostrar o botão de maximizar
    .active-window-hint = Mostrar dica de janela ativa

focus-navigation = Navegação em Foco
    .focus-follows-cursor = O foco segue o cursor
    .focus-follows-cursor-delay = O foco segue o atraso do cursor em ms
    .cursor-follows-focus = O cursor segue o foco

## Desktop: Workspaces

workspaces = Áreas de Trabalho
    .desc = Configurar número de áreas de trabalho, comportamento e posicionamento.

workspaces-behavior = Comportamento das Áreas de Trabalho
    .dynamic = Áreas de trabalho dinâmicas
    .dynamic-desc = Remove automaticamente áreas de trabalho vazias.
    .fixed = Número fixo de Áreas de Trabalho
    .fixed-desc = Adicione ou remova áreas de trabalho na visão geral.

workspaces-multi-behavior = Comportamento de vários monitores
    .span = Áreas de trabalho expandem monitores
    .separate = Monitores têm áreas de trabalho separadas

workspaces-overview-thumbnails = Miniaturas da Visão Geral das Áreas de Trabalho
    .show-number = Mostrar Número da Área de Trabalho
    .show-name = Mostrar Nome da Área de Trabalho

workspaces-orientation = Orientação das Áreas de Trabalho
    .vertical = Vertical
    .horizontal = Horizontal

hot-corner = Hot Corner
    .top-left-corner = Habilitar o canto superior esquerdo para exibir as áreas de trabalho

## Displays

-requires-restart = Requer reinicialização

color = Cor
    .depth = Profundidade de cor
    .profile = Perfil de cor
    .sidebar = Perfis de cor
    .temperature = Temperatura de cor

display = Monitores
    .desc = Gerencie monitores, altere gráficos, e luz noturna
    .arrangement = Organização dos Monitores
    .arrangement-desc = Arraste os monitores para rearranjá-los.
    .enable = Ativar monitor
    .external = { $size } { $output } Monitor Externo
    .laptop = { $size } Tela do notebook
    .options = Opções do Monitor
    .refresh-rate = Taxa de atualização
    .resolution = Resolução
    .scale = Escala

mirroring = Espelhar
    .id = Espelhar { $id }
    .dont = Não espelhar
    .mirror = Espelhar { $display }
    .project = Projetar para { $display ->
        [all] todos os monitores
        *[other] { $display }
    }
    .project-count = Projetar para { $count} outros { $count ->
        [1] monitor
        *[other] monitores
    }

night-light = Luz noturna
    .auto = Automático (do pôr ao nascer do sol)
    .desc = Reduz a luz azul com cores mais quentes.

orientation = Orientação
    .standard = Padrão
    .rotate-90 = Rotacionar 90°
    .rotate-180 = Rotacionar 180°
    .rotate-270 = Rotacionar 270°

scheduling = Agendamento
    .manual = Agendamento manual

dialog = Dialog
    .title = Manter estas configurações de telas?
    .keep-changes = Manter alterações
    .change-prompt = As mudanças nas configurações serão revertidas automaticamente em { $time } segundos.
    .revert-settings = Reverter configurações

legacy-applications = Dimensionamento de aplicativos X11
    .scaled-by-system = Dimensionar todos os aplicativos X11
    .system-description = Aplicativos X11 aparecerão desfocados em telas HiDPI.
    .scaled-natively = Renderizar aplicativos X11 em resolução nativa
    .native-description = Aplicativos X11 que não suportam dimensionamento ficarão menores quando monitores HiDPI estiverem em uso. Habilitar para que os jogos utilizem a resolução total do monitor.

## Sound

sound = Som
    .desc = N/D

sound-output = Saída
    .volume = Volume de saída
    .device = Dispositivo de saída
    .level = Nível de saída
    .config = Configuração
    .balance = Equilíbrio

sound-input = Entrada
    .volume = Volume de entrada
    .device = Dispositivo de entrada
    .level = Nível de entrada

sound-alerts = Alertas
    .volume = Volume dos alertas
    .sound = Som dos alertas

sound-applications = Aplicações
    .desc = Volume e configurações das aplicações

profile = Perfil

## Power

power = Energia e Bateria
    .desc = Gerencie as configurações de energia

battery = Bateria
  .minute = { $value } { $value ->
        [one] minuto
       *[other] minutos
  }
  .hour = { $value } { $value ->
        [one] hora
       *[other] horas
  }
  .day = { $value } { $value ->
        [one] dia
       *[other] dias
  }
  .less-than-minute = Menos de 1 minuto
  .and = e
  .remaining-time = { $time } até { $action ->
        [full] cheia
       *[other] vazia
   }

connected-devices = Dispositivos conectados
  .unknown = Dispositivo desconhecido

power-mode = Modo de Energia
  .battery = Economia de bateria
  .battery-desc = Baixo consumo de energia e desempenho limitado.
  .balanced = Balanceado
  .balanced-desc = Desempenho padrão e consumo de energia moderado.
  .performance = Alto desempenho
  .performance-desc = Desempenho e consumo de energia elevados.
  .no-backend = Gestor de energia não encontrado. Instale o pacote "system76-power" ou "power-profiles-daemon".

power-saving = Opções de Economia de Energia
    .turn-off-screen-after = Desligar a tela após
    .auto-suspend = Suspensão automática
    .auto-suspend-ac = Suspender automaticamente quando plugado na tomada
    .auto-suspend-battery = Suspender automaticamente no modo bateria

## Input

acceleration-desc = Ajusta automaticamente a sensibilidade com base na velocidade.

disable-while-typing = Desabilitar enquanto escreve

input-devices = Dispositivos de entrada
    .desc = Dispositivos de entrada

primary-button = Botão Primário
    .desc = Define a ordem dos botões físicos.
    .left = Esquerda
    .right = Direita

scrolling = Rolagem
    .two-finger = Rolagem com dois dedos
    .edge = Rolagem ao longo da borda com um dedo
    .speed = Velocidade de rolagem
    .natural = Rolagem natural
    .natural-desc = Rolar o conteúdo, ao invés da visualização

## Input: Keyboard

slow = Devagar
fast = Rápido
short = Curto
long = Longo
keyboard = Teclado
    .desc = Layout, entrada de caracteres especiais, atalhos.

keyboard-sources = Fontes de Entrada
    .desc = As fontes de entrada podem ser alternadas usando a combinação de teclas Super+Espaço. Isso pode ser personalizado nas configurações de atalho de teclado.
    .move-up = Mover para cima
    .move-down = Mover para baixo
    .settings = Configurações
    .view-layout = Ver layout do teclado
    .remove = Remover
    .add = Adicionar fonte de entrada

keyboard-special-char = Entrada de Caracteres Especiais
    .alternate = Tecla de caracteres alternativos
    .compose = Tecla de composição
    .caps = Tecla Caps Lock

keyboard-typing-assist = Digitação
    .repeat-rate = Taxa de repetição
    .repeat-delay = Taxa de atraso

added = Adicionado
type-to-search = Digite para pesquisar...
show-extended-input-sources = Exibir fontes de entrada estendidas

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Atalhos do teclado
    .desc = Ver e personalizar atalhos

add-keybinding = Adicionar atalho
cancel = Cancelar
command = Comando
custom = Personalizado
debug = Debug
disabled = Desativado
migrate-workspace-prev = Migrar a área de trabalho para a saída anterior
migrate-workspace-next = Migrar a área de trabalho para a próxima saída
migrate-workspace = Migrar a área de trabalho para a saída { $direction ->
    *[down] abaixo
    [left] à esquerda
    [right] à direita
    [up] acima
}
navigate = Navegar
replace = Substituir
shortcut-name = Nome do atalho
system-controls = Controles de sistema
terminate = Encerrar
toggle-stacking = Ativar/Desativar empilhamento de janelas
type-key-combination = Digite a combinação de teclas

custom-shortcuts = Atalhos personalizados
    .add = Adicionar atalho
    .context = Adiciona um atalho personalizado
    .none = Sem atalhos personalizados

modified = { $count } modificados

nav-shortcuts = Navegação
    .prev-output = Focar a saída anterior
    .next-output = Focus a próxima saída
    .last-workspace = Focar a última área de trabalho
    .prev-workspace = Focar a área de trabalho anterior
    .next-workspace = Focar a próxima área de trabalho
    .focus = Focar a janela  { $direction ->
        *[down] abaixo
        [in] interna
        [left] à esquerda
        [out] externa
        [right] à direita
        [up] acima
    }
    .output = Alternar para a saída { $direction ->
        *[down] abaixo
        [left] à esquerda
        [right] à direita
        [up] acima
    }
    .workspace = Alternar para a área de trabalho { $num }

manage-windows = Gerenciamento de janelas
    .close = Fechar janela
    .maximize = Maximizar janela
    .minimize = Minimizar janela
    .resize-inwards = Redimensionar janela para dentro
    .resize-outwards = Redimensionar janela para fora
    .toggle-sticky = Ativar/Desativar janelas fixadas

move-windows = Mover janelas
    .direction = Mover janela { $direction ->
        *[down] para baixo
        [left] para a esquerda
        [right] para a direita
        [up] para cima
    }
    .display = Mover a janela um monitor { $direction ->
        *[down] abaixo
        [left] à esquerda
        [right] à direita
        [up] acima
    }
    .workspace = Mover a janela uma área de trabalho { $direction ->
        *[below] abaixo
        [left] à esquerda
        [right] à direita
        [above] acima
    }
    .workspace-num = Mover a janela para a área de trabalho { $num }
    .prev-workspace = Mover a janela para a área de trabalho anterior
    .next-workspace = Mover a janela para a próxima área de trabalho
    .last-workspace = Mover a janela para a última área de trabalho
    .next-display = Mover a janela para a próxima tela
    .prev-display = Mover a janela para a tela anterior
    .send-to-prev-workspace = Mover a janela para a área de trabalho anterior
    .send-to-next-workspace = Mover a janela para a próxima área de trabalho

system-shortcut = Sistema
    .app-library = Abrir o menu de aplicativos
    .brightness-down = Reduzir o brilho do monitor
    .brightness-up = Aumentar o brilho do monitor
    .home-folder = Abrir a pasta pessoal
    .keyboard-brightness-down = Reduzir o brilho do teclado
    .keyboard-brightness-up = Aumentar o brilho do teclado
    .launcher = Abrir o lançador
    .lock-screen = Bloquear a tela
    .mute = Silenciar saída de áudio
    .mute-mic = Silencias entrada do microfone
    .play-pause = Iniciar/Pausar
    .play-next = Próxima faixa
    .play-prev = Faixa anterior
    .screenshot = Tirar um print da tela
    .terminal = Abrir o terminal
    .volume-lower = Reduzir o volume da saída de áudio
    .volume-raise = Aumentar o volume da saída de áudio
    .web-browser = Abrir o navegador web
    .window-switcher = Alternar entre as janelas abertas
    .workspace-overview = Abrir a visão geral das Áreas de Trabalho

window-tiling = Janelas lado-a-lado (Window tiling)
    .horizontal = Definir orientação horizontal
    .vertical = Definir orientação vertical
    .swap-window = Troca de janelas
    .toggle-tiling = Ativar/Desativar janelas lado-a-lado (tiling)
    .toggle-stacking = Ativas/Desativar janelas empilháveis
    .toggle-floating = Ativar/Desativar janelas flutuantes
    .toggle-orientation = Ativar/Desativar orientação

replace-shortcut-dialog = Substituir atalho?
    .desc = { $shortcut } está sendo usado por { $name }. se você substituí-lo, { $name } será desativado.

## Input: Mouse

mouse = Mouse
    .desc = Velocidade do mouse, aceleração e rolagem natural.
    .speed = Velocidade do mouse
    .acceleration = Ativar aceleração do mouse

## Input: Touchpad

click-behavior = Comportamento de Cliques
    .click-finger = Clique secundário com dois dedos e clique do meio com três dedos
    .button-areas = Clique secundário no canto inferior direito e clique do meio no centro inferior

pinch-to-zoom = Movimento de pinça para ampliar
    .desc = Use dois dedos para ampliar o conteúdo, para aplciativos que suportam zoom.

tap-to-click = Toque para clicar
    .desc = Habilita o toque de um dedo para clique primário, toque de dois dedos para clique secundário e toque de três dedos para o clique do meio.

touchpad = Touchpad
    .acceleration = Habilitar aceleração do touchpad
    .desc = Velocidade do touchpad, opções de clique e gestos.
    .speed = Velocidade do touchpad

## Input: Gestures

gestures = Gestos
    .four-finger-down = Deslizar quatros dedos para baixo
    .four-finger-left = Deslizar quatro dedos para a esquerda
    .four-finger-right = Deslizar quatro dedos para a direita
    .four-finger-up = Deslizar quatro dedos para cima
    .three-finger-any = Deslizar três dedos para qualquer direção

switch-workspaces = Alternar entre áreas de trabalho
    .horizontal = Deslizar quatro dedos para a direita ou esquerda
    .vertical = Deslizar quatro dedos para cima ou para baixo

switch-between-windows = Alternar entre janelas
open-application-library = Abrir o menu de aplicativos
open-workspaces-view = Abrir a visão geral das áreas de trabalho

## Time & Language

time = Hora e Idioma
    .desc = N/D

time-date = Data e Hora
    .desc = Fuso horário, definições automáticas de relógio e algumas formatação de hora.
    .auto = Definir automaticamente
    .auto-ntp = A data e a hora serão atualizadas automaticamente quando o fuso horário for definido.

time-zone = Fuso horário
    .auto = Fuso horário automático
    .auto-info = Requer um serviço de localização e acesso à Internet

time-format = Formato de Data e Hora
    .twenty-four = Formato de 24 horas
    .show-seconds = Mostrar segundos
    .first = Primeiro dia da semana
    .show-date = Mostrar data no painel superior
    .friday = Sexta-feira
    .saturday = Sábado
    .sunday = Domingo
    .monday = Segunda-feira

time-region = Região e Idioma
    .desc = Formatar datas, horas e números com base na sua região

## System

system = Sistema e Contas

## System: About

about = Sobre
    .desc = Nome do dispositivo, informações de hardware, configurações padrão do sistema operacional.

about-device = Nome do dispositivo
    .desc = Este nome aparece para outros dispositivos de rede ou bluetooth.

about-hardware = Hardware
    .model = Modelo do hardware
    .memory = Memória
    .processor = Processador
    .graphics = Gráficos
    .disk-capacity = Capacidade do disco

about-os = Sistema Operacional
    .os = Sistema operacional
    .os-architecture = Arquitetura do sistema operacional
    .desktop-environment = Ambiente de trabalho
    .windowing-system = Sistema de janelas

about-related = Configurações relacionadas
    .support = Obter ajuda

## System: Firmware

firmware = Firmware
    .desc = Detalhes do firmware.

## System: Users

users = Usuários
    .desc = Autenticação e contas de usuário.
