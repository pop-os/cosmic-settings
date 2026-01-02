app = Configurações
dbus-connection-error = Falha de conexão no DBus
ok = OK
unknown = Desconhecido
number = { $number }

## Network & Wireless

connections-and-profiles =
    { $variant ->
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
network-and-wireless = Rede e conexões sem fio
no-networks = Nenhuma rede foi encontrada.
no-vpn = Nenhuma conexão VPN está disponível.
password = Senha
password-confirm = Confirmar senha
remove = Remover
settings = Configurações
username = Usuário
visible-networks = Redes visíveis
identity = Identidade
auth-dialog = Autenticação necessária
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
    .with-password =
        Falha ao definir { $field ->
           *[username] o usuário
            [password] a senha
            [password-flags] os sinalizadores da senha
        } da VPN com nmcli
wired = Rede Cabeada
    .adapter = Adaptador de rede cabeada { $id }
    .connections = Conexões de rede cabeada
    .devices = Dispositivos de rede cabeada
    .remove = Remover perfil de conexão
wifi = Wi-Fi
    .adapter = Adaptador Wi-Fi { $id }
    .forget = Esquecer esta rede
wireguard-dialog = Adicionar Dispositivo WireGuard
    .description = Escolha um nome de dispositivo para a configuração do WireGuard.

## Networking: Online Accounts

online-accounts = Contas Online
    .desc = Adicionar contas, IMAP e SMTP, logins empresariais

# Bluetooth

activate = Ativar
confirm = Confirmar
enable = Habilitar
bluetooth = Bluetooth
    .desc = Gerenciar dispositivos Bluetooth
    .status = Este sistema está visível como { $aliases } enquanto a configuração do Bluetooth estiver aberta.
    .connected = Conectado
    .connecting = Conectando
    .disconnecting = Desconectando
    .connect = Conectar
    .disconnect = Desconectar
    .forget = Esquecer
    .dbus-error = Ocorreu um erro ao interagir com o DBus: { $why }
    .disabled = O serviço de Bluetooth está desativado
    .inactive = O serviço de Bluetooth não está ativo
    .unknown = O serviço de Bluetooth não pôde ser ativado. O Bluez está instalado?
bluetooth-paired = Dispositivos Conectados Anteriormente
    .connect = Conectar
    .battery = { $percentage }% bateria
bluetooth-confirm-pin = Confirmar o PIN do Bluetooth
    .description = Por favor, confirme se o seguinte PIN corresponde ao exibido em { $device }
bluetooth-available = Dispositivos próximos
bluetooth-adapters = Adaptadores Bluetooth

## Accessibility

accessibility = Acessibilidade
    .vision = Visão
    .on = Ligado
    .off = Desligado
    .unavailable = Indisponível
    .screen-reader = Leitor de tela
    .high-contrast = Modo de alto contraste
    .invert-colors = Inverter cores
    .color-filters = Filtros de cor
hearing = Ouvindo
    .mono = Reproduzir áudio estéreo como mono
default = Padrão
magnifier = Ampliador de tela
    .controls =
        Ou use teclas de atalho: { $zoom_in ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_in } para aumentar o zoom,
        }{ $zoom_out ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_out } para diminuir o zoom,
        }
        Super + roda de rolagem para rolar com o mouse
    .scroll_controls = Ativar zoom com o mouse ou o touchpad com Super + roda de rolagem
    .show_overlay = Mostrar a sobreposição do ampliador
    .increment = Incremento de zoom
    .signin = Iniciar o ampliador ao fazer login
    .applet = Ativar/desativar a lupa no miniaplicativo do painel
    .movement = A visualização ampliada se move
    .continuous = Continuamente com ponteiro
    .onedge = Quando o ponteiro atinge a borda
    .centered = Para manter o ponteiro centralizado
color-filter = Tipo de filtro de cores
    .unknown = Filtro ativo desconhecido
    .greyscale = Escala de cinza
    .deuteranopia = Verde/vermelho (fraqueza do verde, deuteranopia)
    .protanopia = Vermelho/verde (fraqueza do vermelho, protanopia)
    .tritanopia = Azul/amarelo (fraqueza do azul, tritanopia)

## Desktop

desktop = Área de Trabalho

## Desktop: Wallpaper

wallpaper = Plano de fundo
    .change = Mudar imagem a cada
    .desc = Imagens de plano de fundo, cores e opções de apresentação de slides
    .fit = Ajuste do plano de fundo
    .folder-dialog = Escolher pasta do plano de fundo
    .image-dialog = Escolher imagem do plano de fundo
    .plural = Planos de fundo
    .same = Mesmo plano de fundo em todas as telas
    .slide = Apresentação de slides
add-color = Adicionar cor
add-image = Adicionar imagem
all-displays = Todos as telas
colors = Cores
dialog-add = Adicionar
fill = Preencher
fit-to-screen = Ajustar à tela
open-new-folder = Abrir Nova Pasta
recent-folders = Pastas recentes
x-minutes =
    { $number } { $number ->
        [one] minuto
       *[other] minutos
    }
x-hours =
    { $number } { $number ->
        [one] hora
       *[other] horas
    }
never = Nunca

## Desktop: Appearance

appearance = Aparência
    .desc = Cores de destaque e temas
accent-color = Cor de destaque
app-background = Fundo de janela
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
mode-and-colors = Modo e cores
recent-colors = Cores recentes
reset-to-default = Restaurar padrão
rgb = RGB
window-hint-accent = Cor de realce da janela ativa
window-hint-accent-toggle = Utilizar a cor de destaque do tema como realce de janela ativa
auto-switch = Alternar automaticamente entre os modos claro e escuro
    .sunrise = Altera para o modo claro ao nascer do sol
    .sunset = Altera para o modo escuro ao pôr do sol
    .next-sunrise = Altera para o modo claro no próximo nascer do sol
    .next-sunset = Altera para o modo escuro no próximo pôr do sol
container-background = Fundo do contêiner
    .desc-detail = A cor de fundo do contêiner é usada para a barra lateral de navegação, a gaveta lateral, as caixas de diálogos e widgets similares. Por padrão, a cor de fundo do contêiner é automaticamente derivada do fundo da janela.
    .reset = Restaurar para automático
    .desc = Usada para a barra lateral de navegação, a gaveta lateral, as caixas de diálogos e widgets similares
control-tint = Tonalidade dos componentes de controle
    .desc = Usado para os fundos dos botões padrão, entradas de busca, entradas de texto e componentes similares
frosted = Efeito de vidro fosco na interface do sistema
    .desc = Ativa desfoque de fundo ao painel, à dock, aos miniaplicativos, ao lançador e à biblioteca de aplicativos
enable-export = Aplicar o tema atual para aplicativos GNOME
    .desc = Nem todos os toolkits suportam trocar automaticamente. Aplicativos que não são de COSMIC podem precisar ser reiniciados após uma mudança de tema.
icon-theme = Tema de ícones
    .desc = Aplica um conjunto diferente de ícones para os aplicativos
text-tint = Tonalidade do texto da interface
    .desc = Cor usada para derivar cores de texto da interface que possuem contraste suficiente em várias superfícies
style = Estilo
    .round = Arredondado
    .slightly-round = Levemente arredondado
    .square = Quadrado
interface-density = Densidade da interface
    .comfortable = Confortável
    .compact = Compacta
    .spacious = Espaçosa
window-management-appearance = Gerenciamento de Janelas
    .active-hint = Tamanho da borda de destaque da janela ativa
    .gaps = Espaçamento ao redor de janelas organizadas lado a lado

### Experimental

experimental-settings = Configuraçães Experimentais
icons-and-toolkit = Tema de ícones e toolkit
interface-font = Fonte do sistema
monospace-font = Fonte monoespaçada

## Desktop: Notifications

notifications = Notificações
    .desc = Não Perturbe, notificações da tela de bloqueio e configurações por aplicação

## Desktop: Panel

panel = Painel
    .desc = Barra principal do sistema para menus e miniaplicativos
add = Adicionar
add-applet = Adicionar miniaplicativo
all = Todos
applets = Miniaplicativos
center-segment = Segmento central
end-segment = Segmento final
large = Grande
no-applets-found = Nenhum miniaplicativo encontrado...
panel-bottom = Lado inferior
panel-left = Lado esquerdo
panel-right = Lado direito
panel-top = Lado superior
search-applets = Procurar por miniaplicativos...
small = Pequeno
start-segment = Segmento inicial
panel-appearance = Aparência
    .match = Estilo do sistema
    .light = Estilo claro
    .dark = Estilo escuro
panel-behavior-and-position = Comportamento e posições
    .autohide = Ocultar o painel automaticamente
    .dock-autohide = Ocultar a dock automaticamente
    .position = Posição na tela
    .display = Mostrar no monitor
panel-style = Estilo
    .anchor-gap = Espaço entre o painel e as bordas da tela
    .dock-anchor-gap = Espaço entre a dock e as bordas da tela
    .extend = Estender o painel até as bordas da tela
    .dock-extend = Estender a dock até as bordas da tela
    .appearance = Aparência
    .size = Tamanho
    .background-opacity = Opacidade do fundo
panel-applets = Configuração
    .dock-desc = Configurar miniaplicativos da dock
    .desc = Configurar miniaplicativos do painel
panel-missing = Configuração do painel está ausente
    .desc = O arquivo de configuração do painel está ausente devido ao uso de uma configuração personalizada ou porque o arquivo está corrompido.
    .fix = Redefinir para padrão

## Desktop: Dock

dock = Dock
    .desc = Uma barra opcional com aplicativos e miniaplicativos

## Desktop: Window management

window-management = Gerenciamento de janelas
    .desc = Ação da tecla Super, opções de controle de janelas e opções adicionais de janelas lado a lado
super-key = Ação da tecla Super (Windows)
    .launcher = Abrir o lançador
    .workspaces = Abrir a visão geral das áreas de trabalho
    .applications = Abrir o menu de aplicativos
    .disable = Desabilitar
edge-gravity = Janelas flutuantes gravitam em direção às bordas próximas
window-controls = Controles de janela
    .maximize = Mostrar botão de maximizar
    .minimize = Mostrar botão de minimizar
    .active-window-hint = Mostrar realce da janela ativa
focus-navigation = Navegação de foco
    .focus-follows-cursor = O foco segue o cursor
    .focus-follows-cursor-delay = Atraso do foco que segue o cursor em ms
    .cursor-follows-focus = O cursor segue o foco

## Desktop: Workspaces

workspaces = Áreas de trabalho
    .desc = Orientação e comportamento das áreas de trabalho
workspaces-behavior = Comportamento das áreas de trabalho
    .dynamic = Áreas de trabalho dinâmicas
    .dynamic-desc = Remove automaticamente áreas de trabalho vazias.
    .fixed = Número fixo de áreas de trabalho
    .fixed-desc = Adicione ou remova áreas de trabalho na visão geral.
workspaces-multi-behavior = Comportamento de vários monitores
    .span = Áreas de trabalho se expandem a todas as telas
    .separate = Telas têm áreas de trabalho separadas
workspaces-overview-thumbnails = Miniaturas da visão geral das áreas de trabalho
    .show-number = Mostrar o número da área de trabalho
    .show-name = Mostrar o nome da área de trabalho
workspaces-orientation = Orientação das áreas de trabalho
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
display = Telas
    .desc = Gerenciar telas e luz noturna
    .arrangement = Organização de telas
    .arrangement-desc = Arraste os as telas para rearranjá-las
    .enable = Ativar tela
    .external = Tela externa de { $size } { $output }
    .laptop = Tela de notebook de { $size }
    .options = Opções de tela
    .refresh-rate = Taxa de atualização
    .resolution = Resolução
    .scale = Escala
    .additional-scale-options = Opções adicionais da escala
mirroring = Espelhar
    .id = Espelhando { $id }
    .dont = Não espelhar
    .mirror = Espelhar { $display }
    .project =
        Projetar para { $display ->
            [all] todos os monitores
           *[other] { $display }
        }
    .project-count =
        Projetar para { $count } outros { $count ->
            [1] monitor
           *[other] monitores
        }
night-light = Luz noturna
    .auto = Automático (do pôr ao nascer do sol)
    .desc = Reduz a luz azul com cores mais quentes
orientation = Orientação
    .standard = Padrão
    .rotate-90 = Rotacionar 90°
    .rotate-180 = Rotacionar 180°
    .rotate-270 = Rotacionar 270°
vrr = Taxa de atualização variável
    .enabled = Habilitado
    .force = Sempre
    .auto = Automático
    .disabled = Desabilitado
scheduling = Agendamento
    .manual = Agendamento manual
dialog = Diálogo
    .title = Manter estas configurações de tela?
    .keep-changes = Manter alterações
    .change-prompt = As mudanças nas configurações serão revertidas automaticamente em { $time } segundos.
    .revert-settings = Reverter configurações

## Sound

sound = Som
    .desc = N/D
sound-output = Saída
    .volume = Volume de saída
    .device = Dispositivo de saída
    .level = Nível de saída
    .config = Configuração
    .balance = Balanço
    .left = Esquerda
    .right = Direita
sound-input = Entrada
    .volume = Volume de entrada
    .device = Dispositivo de entrada
    .level = Nível de entrada
amplification = Amplificação
    .desc = Permite aumentar o volume até 150%
sound-alerts = Alertas
    .volume = Volume dos alertas
    .sound = Som dos alertas
sound-applications = Aplicações
    .desc = Volume e configurações das aplicações

## Power

power = Energia e Bateria
    .desc = Gerenciar as configurações de energia
battery = Bateria
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
            [one] dia
           *[other] dias
        }
    .less-than-minute = Menos de 1 minuto
    .and = e
    .remaining-time =
        { $time } até estar { $action ->
            [full] cheia
           *[other] vazia
        }
connected-devices = Dispositivos conectados
    .unknown = Dispositivo desconhecido
power-mode = Modo de energia
    .battery = Economia de energia
    .battery-desc = Baixo consumo de energia e desempenho silencioso
    .balanced = Balanceado
    .balanced-desc = Desempenho padrão e consumo de energia moderado
    .performance = Alto desempenho
    .performance-desc = Máximo desempenho e consumo de energia elevado..
    .no-backend = Gerenciador de energia não encontrado. Instale o pacote system76-power ou power-profiles-daemon.
power-saving = Opções de economia de energia
    .turn-off-screen-after = Desligar a tela após
    .auto-suspend = Suspensão automática
    .auto-suspend-ac = Suspensão automática enquanto o computador estiver ligado na tomada
    .auto-suspend-battery = Suspensão automática enquanto o computador estiver usando a bateria

## Input

acceleration-desc = Ajustar automaticamente a sensibilidade com base na velocidade
disable-while-typing = Desabilitar enquanto digita
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

slow = Lento
fast = Rápido
short = Curto
long = Longo
keyboard = Teclado
    .desc = Layout, entrada de caracteres especiais, atalhos
keyboard-sources = Fontes de Entrada
    .desc = As fontes de entrada podem ser alternadas usando a combinação de teclas Super+Espaço. Isso pode ser personalizado nas configurações de atalho de teclado.
    .move-up = Mover para cima
    .move-down = Mover para baixo
    .settings = Configurações
    .view-layout = Ver layout do teclado
    .remove = Remover
    .add = Adicionar fonte de entrada
keyboard-special-char = Entrada de Caracteres Especiais
    .alternate = Tecla de caracteres especiais
    .compose = Tecla de composição
    .compose-desc = A tecla de composição permite a inserção de uma grande variedade de caracteres. Para usá-la, pressione a composição seguida pela sequência de caracteres. Por exemplo, a tecla de composição seguida de "C" e "o" irá inserir "©", enquanto a tecla de composição seguida de "a" e "‘" irá inserir um "á".
    .caps = Tecla Caps Lock
keyboard-typing-assist = Digitação
    .repeat-rate = Taxa de repetição
    .repeat-delay = Taxa de atraso
keyboard-numlock-boot = Numlock
    .boot-state = Estado na inicialização
    .last-boot = Última inicialização
    .on = Ligado
    .off = Desligado
    .set = Definir estado de inicialização da tecla NumLock
added = Adicionado
type-to-search = Digite para pesquisar...
show-extended-input-sources = Exibir fontes de entrada estendidas

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Atalhos do Teclado
    .desc = Ver e personalizar atalhos
cancel = Cancelar
command = Comando
custom = Personalizado
debug = Debug
disabled = Desativado
input-source-switch = Alterar fonte de entrada de idioma do teclado
migrate-workspace-prev = Migrar a área de trabalho para a saída anterior
migrate-workspace-next = Migrar a área de trabalho para a próxima saída
migrate-workspace =
    Migrar a área de trabalho para a saída { $direction ->
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
custom-shortcuts = Atalhos Personalizados
    .add = Adicionar atalho
    .context = Adicionar atalho personalizado
    .none = Sem atalhos personalizados
modified = { $count } modificado(s)
nav-shortcuts = Navegação
    .prev-output = Focar a saída anterior
    .next-output = Focus a próxima saída
    .last-workspace = Focar a última área de trabalho
    .prev-workspace = Focar a área de trabalho anterior
    .next-workspace = Focar a próxima área de trabalho
    .focus =
        Focar a janela  { $direction ->
           *[down] abaixo
            [in] interna
            [left] à esquerda
            [out] externa
            [right] à direita
            [up] acima
        }
    .output =
        Alternar para a saída { $direction ->
           *[down] abaixo
            [left] à esquerda
            [right] à direita
            [up] acima
        }
    .workspace = Alternar para a área de trabalho { $num }
manage-windows = Gerenciamento de Janelas
    .close = Fechar janela
    .maximize = Maximizar janela
    .fullscreen = Janela em tela cheia
    .minimize = Minimizar janela
    .resize-inwards = Redimensionar janela para dentro
    .resize-outwards = Redimensionar janela para fora
    .toggle-sticky = Ativar/Desativar janelas fixadas
move-windows = Mover Janelas
    .direction =
        Mover janela { $direction ->
           *[down] para baixo
            [left] para a esquerda
            [right] para a direita
            [up] para cima
        }
    .display =
        Mover a janela um monitor { $direction ->
           *[down] abaixo
            [left] à esquerda
            [right] à direita
            [up] acima
        }
    .workspace =
        Mover a janela uma área de trabalho { $direction ->
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
    .app-library = Abrir a biblioteca de aplicativos
    .brightness-down = Reduzir o brilho da tela
    .brightness-up = Aumentar o brilho da tela
    .display-toggle = Ativar/desativar a tela interna
    .home-folder = Abrir a pasta pessoal
    .keyboard-brightness-down = Reduzir o brilho do teclado
    .keyboard-brightness-up = Aumentar o brilho do teclado
    .launcher = Abrir o lançador
    .log-out = Encerrar sessão
    .lock-screen = Bloquear a tela
    .mute = Silenciar saída de áudio
    .mute-mic = Silenciar entrada do microfone
    .play-pause = Iniciar/pausar
    .play-next = Próxima faixa
    .play-prev = Faixa anterior
    .poweroff = Desligar
    .screenshot = Fazer uma captura de tela
    .suspend = Suspender
    .terminal = Abrir um terminal
    .touchpad-toggle = Ativar/desativar o touchpad
    .volume-lower = Reduzir o volume da saída de áudio
    .volume-raise = Aumentar o volume da saída de áudio
    .web-browser = Abrir um navegador web
    .window-switcher = Alternar entre as janelas abertas
    .window-switcher-previous = Alternar entre as janelas abertas na ordem inversa
    .workspace-overview = Abrir a visão geral das áreas de trabalho
window-tiling = Janelas lado a lado
    .horizontal = Definir orientação horizontal
    .vertical = Definir orientação vertical
    .swap-window = Trocar de janelas
    .toggle-tiling = Ativar/desativar janelas lado a lado
    .toggle-stacking = Ativas/desativar janelas empilháveis
    .toggle-floating = Ativar/desativar janelas flutuantes
    .toggle-orientation = Ativar/desativar orientação
replace-shortcut-dialog = Substituir atalho?
    .desc = { $shortcut } está sendo usado por { $name }. Se você substituí-lo, { $name } será desativado.
zoom-in = Aumentar o zoom
zoom-out = Diminuir o zoom

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
    .desc = Use dois dedos para ampliar o conteúdo, para aplciativos que suportam zoom
tap-to-click = Toque para clicar
    .desc = Habilita o toque de um dedo para clique primário, toque de dois dedos para clique secundário e toque de três dedos para o clique do meio
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
    .desc = Fuso horário, definições automáticas de relógio e algumas formatações de hora.
    .auto = Definir automaticamente
    .auto-ntp = A data e a hora serão atualizadas automaticamente quando o fuso horário for definido
time-zone = Fuso Horário
    .auto = Fuso horário automático
    .auto-info = Requer um serviço de localização e acesso à Internet
time-format = Formato de Data e Hora
    .twenty-four = Formato de 24 horas
    .show-seconds = Mostrar segundos
    .first = Primeiro dia da semana
    .show-date = Mostrar a data no painel
    .friday = Sexta-feira
    .saturday = Sábado
    .sunday = Domingo
    .monday = Segunda-feira
time-region = Região e Idioma
    .desc = Formatar datas, horas e números com base na sua região
formatting = Formatação
    .dates = Datas
    .time = Horas
    .date-and-time = Data e Hora
    .numbers = Números
    .measurement = Medidas
    .paper = Papel
preferred-languages = Idiomas Preferidos
    .desc = A ordem dos idiomas determina qual idioma será usado para a tradução do desktop. As alterações entram em vigor no próximo login.
add-language = Adicionar idioma
    .context = Adicionar Idioma
install-additional-languages = Instalar idiomas adicionais
region = Região

## Applications

applications = Aplicativos

## Applications: Default Applications

default-apps = Aplicativos Padrão
    .desc = Estabelecer os aplicativos padrões para o navegador web, o cliente de correio eletrônico, o gestor de arquivos e outros aplicativos
    .web-browser = Navegador web
    .file-manager = Gerenciador de arquivos
    .mail-client = Cliente de e-mail
    .music = Música
    .video = Vídeo
    .photos = Fotos
    .calendar = Calendário
    .terminal = Terminal
    .other-associations = Outras associações
    .text-editor = Editor de texto

## Applications: Startup Applications

startup-apps = Aplicativos de Inicialização
    .desc = Configurar aplicativos que são iniciados automaticamente ao fazer login no sistema.
    .add = Adicionar aplicativo
    .user = Aplicativos que serão iniciados ao fazer login no sistema
    .none = Não há aplicativos de inicialização adicionados
    .remove-dialog-title = Remover { $name }?
    .remove-dialog-description = Remover esse aplicativo de inicialização?
    .add-startup-app = Adicionar aplicativos de inicialização

## Applications: Legacy Applications

legacy-applications = Compatibilidade com Aplicativos X11
    .desc = Dimensionamento de aplicativos de sistema de janelas X11 e atalhos globais
legacy-app-global-shortcuts = Atalhos globais em aplicativos X11
    .desc = Atalhos globais permitem que pressionamentos de teclas e eventos de botão do mouse realizados em aplicativos sejam reconhecidos por outros aplicativos para recursos como push-to-talk ou push-to-mute. Por padrão, isso é desabilitado em aplicativos X11 para garantir que outros aplicativos não possam monitorar eventos de teclado e mouse contendo informações confidenciais.
    .none = Nenhuma tecla
    .modifiers = Modificadores (Super, Shift, Control, Alt)
    .combination = Todas as teclas enquanto modificadores Super, Control ou Alt estão sendo pressionados
    .all = Todas as teclas
    .mouse = Eventos de botão de mouse em aplicativos X11
legacy-app-scaling = Dimensionamento de Aplicativos X11
    .scaled-gaming = Otimizar para jogos e aplicativos em tela cheia
    .gaming-description = Os aplicativos X11 podem parecer um pouco maiores/menores em comparação aos aplicativos Wayland.
    .scaled-applications = Otimizar para aplicativos
    .applications-description = Jogos e aplicativos X11 em tela cheia podem não corresponder com a resolução da tela.
    .scaled-compatibility = Modo de máxima compatibilidade
    .compatibility-description = Os aplicativos X11 podem aparecer desfocados em telas HiDPI.
    .preferred-display = Preferência para jogos e aplicativos X11 em tela cheia
    .no-display = Nenhum

## System

system = Sistema e Contas

## System: About

about = Sobre
    .desc = Nome do dispositivo, informações de hardware, configurações padrão do sistema
about-device = Nome do dispositivo
    .desc = Aparece para outros dispositivos de rede ou Bluetooth
about-hardware = Hardware
    .model = Modelo do hardware
    .memory = Memória
    .processor = Processador
    .graphics = Gráficos
    .disk-capacity = Capacidade do disco
about-os = Sistema operacional
    .os = Sistema operacional
    .os-architecture = Arquitetura do sistema operacional
    .kernel = Versão do kernel
    .desktop-environment = Ambiente de trabalho
    .windowing-system = Sistema de janelas
about-related = Configurações relacionadas
    .support = Obter ajuda

## System: Firmware

firmware = Firmware
    .desc = Detalhes do firmware

## System: Users

users = Usuários
    .desc = Autenticação e contas de usuário.
    .admin = Administrador
    .standard = Padrão
    .profile-add = Escolher uma imagem de perfil
administrator = Administrador
    .desc = Administradores podem alterar configurações para todos os usuários, além de adicionar e remover outros usuários
add-user = Adicionar usuário
change-password = Alterar senha
remove-user = Remover usuário
full-name = Nome completo
invalid-username = Nome de usuário inválido
password-mismatch = A senha e a confirmação devem ser iguais
save = Salvar
add-another-keybinding = Adicionar outra tecla de atalho
qr-code-unavailable = QR code inválido
network-name = Nome da rede
share = Compartilhar rede
scan-to-connect-description = Escaneie o QR code para se conectar a esta rede.
place-here = Colocar miniaplicativos aqui
sound-device-port-unplugged = Desplugado
sound-hd-audio = Áudio HD
sound-usb-audio = Áudio USB
sound-device-profiles = Perfis de dispositivo
