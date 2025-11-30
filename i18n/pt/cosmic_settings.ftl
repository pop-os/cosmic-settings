app = Definições COSMIC
unknown = Desconhecido
number = { $number }

## Networking: Wired

wired = Por cabo
    .adapter = Adaptador com cabo { $id }
    .connections = Ligações com cabo
    .devices = Dispositivos com cabo
    .remove = Remover perfil de ligação

## Networking: Online Accounts

online-accounts = Contas online
    .desc = Adicionar contas, IMAP e SMTP, acessos empresariais

## Desktop

desktop = Ambiente de trabalho

## Desktop: Wallpaper

wallpaper = Papel de parede
    .change = Mudar imagem a cada
    .desc = Imagens de papel de parede, cores e opções de apresentação de diapositivos.
    .fit = Ajuste do papel de parede
    .folder-dialog = Escolher a pasta do papel de parede
    .image-dialog = Escolher a imagem do papel de parede
    .plural = Papéis de parede
    .same = O mesmo papel de parede em todos os ecrãs
    .slide = Apresentação de diapositivos
add-color = Adicionar cor
add-image = Adicionar imagem
all-displays = Todos os ecrãs
colors = Cores
dialog-add = Adicionar
fill = Preencher
fit-to-screen = Ajustar ao ecrã
open-new-folder = Abrir nova pasta
recent-folders = Pastas recentes
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

appearance = Aparência
    .desc = Cores de destaque e temas COSMIC.
accent-color = Cor de destaque
app-background = Fundo da aplicação ou janela
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
reset-to-default = Repor a predefinição
rgb = RGB
window-hint-accent = Cor da sugestão da janela ativa
window-hint-accent-toggle = Usar a cor de destaque do tema como sugestão de janela ativa
auto-switch = Mudar automaticamente do modo Claro para Escuro
    .sunrise = Muda para o modo Claro ao nascer do sol
    .sunset = Muda para o modo Claro ao pôr do sol
    .next-sunrise = Muda para o modo Claro no próximo nascer do sol
    .next-sunset = Muda para o modo Claro no próximo pôr do sol
container-background = Fundo do contentor
    .desc-detail = A cor de fundo do contentor é usada na barra lateral de navegação, gaveta lateral, nas caixas de diálogo e em widgets semelhantes. Por predefinição, é automaticamente derivada do fundo da aplicação ou da janela.
    .reset = Repor para automático
    .desc = A cor principal do contentor é usada para a barra lateral de navegação, gaveta lateral, caixas de diálogo e widgets semelhantes.
control-tint = Controlar a tonalidade dos componentes
    .desc = Usado para fundos de botões normais, entradas de pesquisa, entradas de texto e componentes semelhantes.
frosted = Efeito de vidro fosco na interface do sistema
    .desc = Aplica uma desfocagem de fundo ao painel, à doca, aos miniaplicativos, ao lançador e à biblioteca de aplicações.
experimental-settings = Definições experimentais
enable-export = Aplicar este tema a aplicações GNOME.
    .desc = Nem todos os toolkits suportam a troca automática. As aplicações não-COSMIC poderão ter de ser reiniciadas após uma mudança de tema.
icon-theme = Tema de ícones
    .desc = Aplica um conjunto diferente de ícones às aplicações.
text-tint = Tonalidade do texto da interface
    .desc = Cor utilizada para derivar cores de texto de interface que tenham contraste suficiente em várias superfícies.
style = Estilo
    .round = Redondo
    .slightly-round = Ligeiramente redondo
    .square = Quadrado
window-management-appearance = Gestão de janelas
    .active-hint = Tamanho da sugestão da janela ativa
    .gaps = Espaços à volta das janelas em mosaico

## Desktop: Notifications

notifications = Notificações
    .desc = Não incomodar, notificações no ecrã de bloqueio e definições por aplicação.

## Desktop: Panel

panel = Painel
    .desc = Barra do sistema principal para menus e miniaplicativos.
add = Adicionar
add-applet = Adicionar miniaplicativo
all = Todos
applets = Miniaplicativos
center-segment = Segmento central
end-segment = Segmento final
large = Grande
no-applets-found = Nenhum miniaplicativo encontrado...
panel-bottom = Inferior
panel-left = Esquerda
panel-right = Direita
panel-top = Superior
search-applets = Procurar miniaplicativos...
small = Pequeno
start-segment = Segmento inicial
panel-appearance = Aspeto
    .match = Estilo do sistema
    .light = Claro
    .dark = Escuro
panel-behavior-and-position = Comportamento e Posições
    .autohide = Ocultar automaticamente o painel
    .dock-autohide = Ocultar automaticamente a doca
    .position = Posição no ecrã
    .display = Mostrar no ecrã
panel-style = Estilo
    .anchor-gap = Espaço entre o painel e as margens do ecrã
    .dock-anchor-gap = Espaço entre a doca e as margens do ecrã
    .extend = Estender o painel até às margens do ecrã
    .dock-extend = Estender a doca até às margens do ecrã
    .appearance = Aparência
    .size = Tamanho
    .background-opacity = Opacidade do fundo
panel-applets = Configuração
    .dock-desc = Configura os miniaplicativos da doca
    .desc = Configura os miniaplicativos do painel
panel-missing = A configuração do painel está em falta
    .desc = O ficheiro de configuração do painel está em falta devido à utilização de uma configuração personalizada ou está corrompido.
    .fix = Repor a predefinição

## Desktop: Dock

dock = Doca
    .desc = Uma barra opcional para aplicações e miniaplicativos.

## Desktop: Window management

window-management = Gestão de Janelas
    .desc = Ação da tecla Super, opções de controlo de janelas, e opções adicionais de alinhamento de janelas.
super-key = Tecla Super
    .launcher = Abrir Lançador
    .workspaces = Abrir Áreas de trabalho
    .applications = Abrir Aplicações
    .disable = Desativar
window-controls = Controlos de janela
    .maximize = Mostrar o botão de maximizar
    .minimize = Mostrar o botão de minimizar
    .active-window-hint = Mostrar dica da janela ativa

## Desktop: Workspaces

workspaces = Áreas de trabalho
    .desc = Definir número, comportamento e posição da área de trabalho.
workspaces-behavior = Comportamento da área de trabalho
    .dynamic = Áreas de trabalho dinâmicas
    .dynamic-desc = Remove automaticamente áreas de trabalho vazias.
    .fixed = Número fixo de áreas de trabalho
    .fixed-desc = Adiciona ou remove áreas de trabalho na visão geral.
workspaces-multi-behavior = Comportamento de vários monitores
    .span = Áreas de trabalho expandem os ecrãs
    .separate = Ecrãs têm áreas de trabalho separadas
workspaces-overview-thumbnails = Miniaturas da visão geral da área de trabalho
    .show-number = Mostrar número da área de trabalho
    .show-name = Mostrar nome da área de trabalho
workspaces-orientation = Orientação das áreas de trabalho
    .vertical = Vertical
    .horizontal = Horizontal
hot-corner = Canto ativo
    .top-left-corner = Ativar o canto superior esquerdo para as áreas de trabalho

## Desktop: Display

-requires-restart = Requer reinício
color = Cor
    .depth = Profundidade da cor
    .profile = Perfil da cor
    .sidebar = Perfis de cor
    .temperature = Temperatura da cor
display = Ecrãs
    .desc = Gerir ecrãs, comutação de gráficos e luz noturna
    .arrangement = Disposição do ecrã
    .arrangement-desc = Arrastar ecrãs para os reorganizar.
    .enable = Ativar ecrã
    .external = { $size } { $output } Ecrã externo
    .laptop = { $size } Ecrã portátil
    .options = Opções de ecrã
    .refresh-rate = Taxa de atualização
    .resolution = Resolução
    .scale = Escala
    .additional-scale-options = Opções adicionais de escala
mirroring = Espelhar
    .id = Espelhar { $id }
    .dont = Não espelhar
    .mirror = Espelhar { $display }
    .project =
        Projeto para { $display ->
            [all] todos os ecrãs
           *[other] { $display }
        }
    .project-count =
        Projetar para { $count } outros { $count ->
            [1] ecrã
           *[other] ecrãs
        }
night-light = Luz noturna
    .auto = Automático (do pôr ao nascer do sol)
    .desc = Reduz a luz azul com cores mais quentes.
orientation = Orientação
    .standard = Padrão
    .rotate-90 = Rodar 90°
    .rotate-180 = Rodar 180°
    .rotate-270 = Rodar 270°
scheduling = Agendamento
    .manual = Agendamento manual
dialog = Diálogo
    .title = Manter estas definições de ecrã?
    .keep-changes = Manter alterações
    .change-prompt = As alterações nas definições serão automaticamente revertidas em { $time } segundos.
    .revert-settings = Reverter definições

## Sound

sound = Som
    .desc = N/D
sound-output = Saída
    .volume = Volume de saída
    .device = Dispositivo de saída
    .level = Nível de saída
    .config = Configuração
    .balance = Equilíbrio
    .left = Esquerdo
    .right = Direito
sound-input = Entrada
    .volume = Volume de entrada
    .device = Dispositivo de entrada
    .level = Nível de entrada
sound-alerts = Alertas
    .volume = Volume dos alertas
    .sound = Som dos alertas
sound-applications = Aplicações
    .desc = Volumes e definições das aplicações

## Power

power = Energia e Bateria
    .desc = Gere as definições da energia
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
    .less-than-minute = Menos de um minuto
    .and = e
    .remaining-time =
        { $time } até { $action ->
            [full] Carregada
           *[other] Descarregada
        }
power-mode = Modo de Energia
    .battery = Expande a vida da bateria
    .battery-desc = Consumo de energia reduzido e desempenho limitado.
    .balanced = Balanceado
    .balanced-desc = Desempenho balanceado e consumo de energia moderado.
    .performance = Alto desempenho
    .performance-desc = Alto desempenho e consumo de energia.
    .no-backend = Backend não encontrado. Instale o system76-power ou o power-profiles-daemon.

## Input

acceleration-desc = Ajusta automaticamente a sensibilidade do seguimento com base na velocidade.
disable-while-typing = Desativar enquanto escreve
input-devices = Dispositivos de entrada
    .desc = Dispositivos de entrada
primary-button = Botão primário
    .desc = Define a ordem dos botões físicos.
    .left = Esquerda
    .right = Direita
scrolling = Deslocação
    .two-finger = Deslocar com dois dedos
    .edge = Deslocar ao longo da margem com um dedo
    .speed = Velocidade de deslocação
    .natural = Deslocação natural
    .natural-desc = Deslocar o conteúdo, em vez da visualização

## Input: Keyboard

slow = Lenta
fast = Rápida
short = Curto
long = Longo
keyboard = Teclado
    .desc = Fontes de entrada, entrada de caracteres especiais, atalhos.
keyboard-sources = Fontes de entrada
    .desc = As fontes de entrada podem ser comutadas utilizando a combinação de teclas Super+Espaço. Isto pode ser personalizado nas definições de teclas de atalho.
    .move-up = Mover para cima
    .move-down = Mover para baixo
    .settings = Definições
    .view-layout = Ver esquema do teclado
    .remove = Remover
    .add = Adicionar fonte de entrada
keyboard-special-char = Entrada de especiais
    .alternate = Tecla de caráteres alternativos
    .compose = Tecla de composição
    .caps = tecla Caps Lock
keyboard-typing-assist = Escrita
    .repeat-rate = Taxa de repetição
    .repeat-delay = Taxa de atraso
added = Adicionado
type-to-search = Escreva para procurar...

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Teclas de atalho
    .desc = Ver e personalizar atalhos
cancel = Cancelar
command = Comando
custom = Personalizado
debug = Depuração
disabled = Desativado
migrate-workspace-prev = Migrar área de trabalho para a saída anterior
migrate-workspace-next = Migrar área de trabalho para a saída seguinte
migrate-workspace =
    Migrar área de trabalho para a saída { $direction ->
       *[down] de baixo
        [left] da esquerda
        [right] da direita
        [up] de cima
    }
navigate = Navegar
replace = Substituir
shortcut-name = Nome do atalho
system-controls = Comandos do sistema
terminate = Terminar
toggle-stacking = Ativar empilhamento de janelas
type-key-combination = Escreva combinação de teclas
custom-shortcuts = Atalhos personalizados
    .add = Adicionar atalho
    .context = Adicionar atalho personalizado
    .none = Sem atalhos personalizados
modified = { $count } modificado(s)
nav-shortcuts = Navegação
    .prev-output = Focar na saída anterior
    .next-output = Focar na saída seguinte
    .last-workspace = Focar na última área de trabalho
    .prev-workspace = Focar na área de trabalho anterior
    .next-workspace = Focar na área de trabalho seguinte
    .focus =
        Focar na janela { $direction ->
           *[down] em baixo
            [in] de dentro
            [left] da esquerda
            [out] de fora
            [right] da direita
            [up] em cima
        }
    .output =
        Trocar para a saída { $direction ->
           *[down] de baixo
            [left] da esquerda
            [right] da direita
            [up] de cima
        }
    .workspace = Mudar para a área de trabalho { $num }
manage-windows = Gerir janelas
    .close = Fechar janela
    .maximize = Maximizar janela
    .fullscreen = Ecrã inteiro
    .minimize = Minimizar janela
    .resize-inwards = Redimensionar janela para dentro
    .resize-outwards = Redimensionar janela para fora
    .toggle-sticky = Tornar janela fixa
move-windows = Mover Janelas
    .direction =
        Mover janela { $direction ->
           *[down] para baixo
            [left] para a esquerda
            [right] para a direita
            [up] para cima
        }
    .display =
        Mover janela um monitor { $direction ->
           *[down] para baixo
            [left] para a esquerda
            [right] para a direita
            [up] para cima
        }
    .workspace =
        Mover janela uma área de trabalho { $direction ->
           *[below] para baixo
            [left] para a esquerda
            [right] para a direita
            [above] para cima
        }
    .workspace-num = Mover janela para a área de trabalho { $num }
    .prev-workspace = Mover janela para a área de trabalho anterior
    .next-workspace = Mover janela para a área de trabalho seguinte
    .last-workspace = Mover janela para a última área de trabalho
    .next-display = Mover janela para o ecrã seguinte
    .prev-display = Mover janela para o ecrã anterior
    .send-to-prev-workspace = Mover janela para a área de trabalho anterior
    .send-to-next-workspace = Mover janela para a área de trabalho seguinte
system-shortcut = Sistema
    .app-library = Abrir a biblioteca de aplicações
    .brightness-down = Diminuir o brilho do ecrã
    .brightness-up = Aumentar o brilho do ecrã
    .display-toggle = Ativar o ecrã interno
    .home-folder = Abrir pasta pessoal
    .keyboard-brightness-down = Diminuir o brilho do teclado
    .keyboard-brightness-up = Aumentar o brilho do teclado
    .launcher = Abrir o lançador
    .log-out = Sair
    .lock-screen = Bloquear o ecrã
    .mute = Silenciar saída de áudio
    .mute-mic = Silenciar entrada do microfone
    .play-pause = Reproduzir/Pausar
    .play-next = Faixa seguinte
    .play-prev = Faixa anterior
    .poweroff = Desligar
    .screenshot = Tirar uma captura de ecrã
    .terminal = Abrir o terminal
    .touchpad-toggle = Ativar painel tátil
    .volume-lower = Diminuir o volume da saída do áudio
    .volume-raise = Aumentar o volume da saída do áudio
    .web-browser = Abrir um navegador de Internet
    .window-switcher = Alternar entre janelas abertas
    .window-switcher-previous = Alternar entre janelas abertas invertidas
    .workspace-overview = Abrir a visão geral das áreas de trabalho
window-tiling = Janelas em mosaico (tiling)
    .horizontal = Definir orientação horizontal
    .vertical = Definir orientação vertical
    .swap-window = Trocar janela
    .toggle-tiling = Ativar janelas em mosaico (tiling)
    .toggle-stacking = Ativar janelas empilháveis
    .toggle-floating = Ativar janelas flutuantes
    .toggle-orientation = Ativar orientação
replace-shortcut-dialog = Substituir Atalho?
    .desc = { $shortcut } está a ser usado por { $name }. Se tu o substituíres, { $name } vai ser desativado.

## Input: Mouse

mouse = Rato
    .desc = Velocidade do rato, aceleração, deslocação natural.
    .speed = Velocidade do rato
    .acceleration = Ativar a aceleração do rato

## Input: Touchpad

click-behavior = Comportamento do clique
    .click-finger = Clique secundário com dois dedos e clique do meio com três dedos
    .button-areas = Clique secundário no canto inferior direito e clique do meio no centro inferior
pinch-to-zoom = Apertar para ampliar
    .desc = Utilize dois dedos para fazer zoom no conteúdo, para aplicações que suportem ampliação.
tap-to-click = Tocar para clicar
    .desc = Permite o toque com um dedo para o clique principal, o toque com dois dedos para o clique secundário e o toque com três dedos para o clique no botão do meio.
touchpad = Painel tátil
    .acceleration = Ativar a aceleração do painel tátil
    .desc = Velocidade do painel tátil, opções de clique, gestos.
    .speed = Velocidade do painel tátil

## Input: Gestures

gestures = Gestos
    .four-finger-down = Deslizar quatro dedos para baixo
    .four-finger-left = Deslizar quatro dedos para a esquerda
    .four-finger-right = Deslizar quatro dedos para a direita
    .four-finger-up = Deslizar quatro dedos para cima
    .three-finger-any = Passar com três dedos em qualquer direção
switch-workspaces = Alterar entre áreas de trabalho
    .horizontal = Deslizar quatro dedos para a esquerda/direita
    .vertical = Deslizar quatro dedos para cima/baixo
switch-between-windows = Alternar entre janelas
open-application-library = Abrir biblioteca de aplicações
open-workspaces-view = Abrir visão geral das áreas de trabalho

## Time & Language

time = Hora e Idioma
    .desc = N/D
time-date = Data e Hora
    .desc = Fuso horário, definições automáticas de relógio e alguma formatação de hora.
    .auto = Definir automaticamente
    .auto-ntp = A data e a hora serão atualizadas automaticamente quando o fuso horário for definido.
time-zone = Fuso horário
    .auto = Fuso horário automático
    .auto-info = Requer serviços de localização e acesso à Internet
time-format = Formato de Data e Hora
    .twenty-four = Formato de 24 horas
    .show-seconds = Mostrar segundos
    .first = Primeiro dia da semana
    .show-date = Mostrar a data no miniaplicativo da hora
    .friday = Sexta-feira
    .saturday = Sábado
    .sunday = Domingo
    .monday = Segunda-feira
time-region = Região e Idioma
    .desc = Formatar datas, horas e números baseado na sua região.

## System

system = Sistema e Contas

## System: About

about = Acerca
    .desc = Nome do dispositivo, informações do equipamento, predefinições do sistema operativo.
about-device = Nome do dispositivo
    .desc = Este nome aparece a outros dispositivos de rede ou Bluetooth.
about-hardware = Hardware
    .model = Modelo do equipamento
    .memory = Memória
    .processor = Processador
    .graphics = Placa gráfica
    .disk-capacity = Capacidade do disco
about-os = Sistema Operativo
    .os = Sistema operativo
    .os-architecture = Arquitetura do sistema operativo
    .desktop-environment = Ambiente de trabalho
    .windowing-system = Sistema de janelas
about-related = Definições relacionadas
    .support = Obter suporte

## System: Firmware

firmware = Firmware
    .desc = Detalhes do firmware.

## System: Users

users = Utilizadores
    .desc = Autenticação e contas do utilizador.
    .admin = Admin
    .standard = Padrão
    .profile-add = Escolher imagem do perfil
remove = Remover
connect = Ligar
password = Palavra-passe
username = Nome de utilizador
dbus-connection-error = Falha de ligação no DBus
ok = OK
connections-and-profiles =
    { $variant ->
        [wired] Ligações por cabo
        [wifi] Ligações Wi-Fi
        [vpn] Ligações VPN
       *[other] Ligações desconhecidas
    } e perfis de ligação.
add-network = Adicionar rede
    .profile = Adicionar perfil
add-vpn = Adicionar VPN
airplane-on = O modo avião está ligado.
cable-unplugged = Cabo desconectado
connected = Ligado
connecting = A ligar…
disconnect = Desligar
forget = Esquecer
known-networks = Redes conhecidas
network-and-wireless = Rede e Wireless
no-networks = Nenhuma rede foi encontrada.
no-vpn = Nenhuma ligação VPN está disponível.
password-confirm = Confirmar palavra-passe
settings = Definições
visible-networks = Redes visíveis
identity = Identidade
auth-dialog = Autenticação necessária
    .vpn-description = Introduza o nome de utilizador e a palavra-passe exigidos pelo serviço VPN.
    .wifi-description = Introduza a palavra-passe ou a chave de encriptação. Também pode ligar-se premindo o botão «WPS» no router.
forget-dialog = Esquecer esta rede Wi-Fi?
    .description = Terá de introduzir novamente uma palavra-passe para utilizar esta rede Wi-Fi no futuro.
network-device-state =
    .activated = Ligado
    .config = A ligar
    .deactivating = A desligar
    .disconnected = Desligado
    .failed = Falha na ligação
    .ip-check = A verificar ligação
    .ip-config = A solicitar informações de IP e encaminhamento
    .need-auth = Necessita de autenticação
    .prepare = A preparar a ligação
    .secondaries = A aguardar ligação secundária
    .unavailable = Indisponível
    .unknown = Estado desconhecido
    .unmanaged = Não gerido
    .unplugged = Cabo desligado
remove-connection-dialog = Remover perfil de ligação?
    .vpn-description = Terá de introduzir novamente uma palavra-passe para utilizar esta rede no futuro.
    .wired-description = Terá de recriar este perfil para o utilizar no futuro.
vpn = VPN
    .connections = Ligações VPN
    .error = Falha ao adicionar configuração VPN
    .remove = Remover perfil de ligação
    .select-file = Selecionar um ficheiro de configuração VPN
vpn-error = Erro de VPN
    .config = Falha ao adicionar configuração de VPN
    .connect = Falha ao ligar à VPN
    .connection-editor = Falha no editor de ligação
    .connection-settings = Falha ao obter configurações para ligações ativas
    .updating-state = Falha ao atualizar o estado do gestor de rede
    .wireguard-config-path = Caminho de ficheiro inválido para configuração do WireGuard
    .wireguard-config-path-desc = O ficheiro escolhido deve estar num sistema de ficheiros local.
    .wireguard-device = Falha ao criar dispositivo WireGuard
    .with-password =
        Falha ao definir VPN { $field ->
           *[username] nome de utilizador
            [password] palavra-passe
            [password-flags] sinalizadores de palavra-passe
        } com nmcli
wifi = Wi-Fi
    .adapter = Adaptador Wi-Fi { $id }
    .forget = Esquecer esta rede
wireguard-dialog = AAdicionar dispositivo WireGuard
    .description = Escolha um nome de dispositivo para a configuração do WireGuard.
activate = Ativar
confirm = Confirmar
enable = Ativar
bluetooth = Bluetooth
    .desc = Gerir dispositivos Bluetooth
    .status = Este sistema é visível como { $aliases } enquanto as definições Bluetooth estão abertas.
    .connected = Ligado
    .connecting = A ligar
    .disconnecting = A desligar
    .connect = Ligar
    .disconnect = Desligar
    .forget = Esquecer
    .dbus-error = Ocorreu um erro ao interagir com o DBus: { $why }
    .disabled = O serviço Bluetooth está desativado
    .inactive = O serviço Bluetooth não está ativo
    .unknown = Não foi possível ativar o serviço Bluetooth. O BlueZ está instalado?
bluetooth-paired = Dispositivos conectados anteriormente
    .connect = Conectar
    .battery = { $percentage }% de bateria
bluetooth-confirm-pin = Confirmar PIN Bluetooth
    .description = Confirme se o seguinte PIN corresponde ao exibido no { $device }
bluetooth-available = Dispositivos próximos
bluetooth-adapters = Adaptadores de Bluetooth
accessibility = Acessibilidade
    .vision = Visão
    .on = Ligado
    .off = Desligado
    .unavailable = Indisponível
    .screen-reader = Leitor de ecrã
    .high-contrast = Modo de alto contraste
    .invert-colors = Inverter cores
    .color-filters = Filtros de cor
hearing = Audição
    .mono = Reproduzir áudio estéreo como mono
default = Predefinição
magnifier = Lupa
    .controls =
        Ou utilize estes atalhos: { $zoom_in ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_in } para aumentar,
        }{ $zoom_out ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_out } para diminuir,
        }
        Super + Scroll para rolar com o rato
    .scroll_controls = Ativar atalho "Super + Scroll" (via rato ou touchpad)
    .show_overlay = Mostrar a sobreposição da lupa
    .increment = Incremento de zoom
    .signin = Iniciar a lupa ao iniciar sessão
    .applet = Alternar a lupa entre ligado/desligado no miniaplicativo no painel
    .movement = A vista ampliada move-se
    .continuous = Continuamente com o ponteiro
    .onedge = Quando o ponteiro atinge a borda
    .centered = Para manter o ponteiro centrado
color-filter = Tipo de filtro de cores
    .unknown = Filtro ativo desconhecido
    .greyscale = Escala de cinzentos
    .deuteranopia = Verde/Vermelho (deficiência na percepção do verde, Deuteranopia)
    .protanopia = Vermelho/Verde (deficiência na percepção do vermelho, Protanopia)
    .tritanopia = Azul/Amarelo (deficiência na percepção do azul, Tritanopia)
never = Nunca
interface-density = Densidade da interface
    .comfortable = Confortável
    .compact = Compacto
    .spacious = Espaçoso
icons-and-toolkit = Tema de ícones e toolkit
interface-font = Tipo de letra do sistema
monospace-font = Tipo de letra monoespaçada
edge-gravity = Janelas flutuantes gravitam em direção às bordas próximas
focus-navigation = Navegação em Foco
    .focus-follows-cursor = O foco segue o cursor
    .focus-follows-cursor-delay = O foco segue o atraso do cursor em ms
    .cursor-follows-focus = O cursor segue o foco
vrr = Taxa de atualização variável
    .enabled = Ativado
    .force = Sempre
    .auto = Automático
    .disabled = Desativado
amplification = Amplificação
    .desc = Permite aumentar o volume até 150%.
connected-devices = Dispositivos conectados
    .unknown = Dispositivo desconhecido
power-saving = Opções de poupança de energia
    .turn-off-screen-after = Desligar o ecrã após
    .auto-suspend = Suspensão automática
    .auto-suspend-ac = Suspender automaticamente quando ligado à corrente
    .auto-suspend-battery = Suspender automaticamente quando alimentado por bateria
keyboard-numlock-boot = Numlock
    .boot-state = Estado no arranque
    .last-boot = Último arranque
    .on = Ligado
    .off = Desligado
    .set = Definir estado de arranque do numlock
show-extended-input-sources = Mostrar fontes de entrada expandidas
add-another-keybinding = Adicionar outra tecla de atalho
input-source-switch = Alterar fonte de entrada de idioma do teclado
zoom-in = Aumentar
zoom-out = Diminuir
formatting = Formatação
    .dates = Datas
    .time = Horas
    .date-and-time = Data e Hora
    .numbers = Números
    .measurement = Medida
    .paper = Papel
preferred-languages = Idiomas Preferidos
    .desc = A ordem dos idiomas determina qual idioma é usado para a interface do utilizador. As alterações têm efeito no próximo início de sessão.
add-language = Adicionar idioma
    .context = Adicionar Idioma
install-additional-languages = Instalar idiomas adicionais
region = Região
applications = Aplicações
default-apps = Aplicações predefinidas
    .desc = Navegador web, cliente de e-mail, gestor de ficheiros e outras aplicações predefinidas.
    .web-browser = Navegador Web
    .file-manager = Gestor de Ficheiros
    .mail-client = Cliente de E-mail
    .music = Música
    .video = Vídeo
    .photos = Fotos
    .calendar = Calendário
    .terminal = Terminal
    .other-associations = Outras Associações
    .text-editor = Editor de Texto
startup-apps = Aplicações de arranque
    .desc = Configure as aplicações que são executadas no início de sessão.
    .add = Adicionar aplicação
    .user = Aplicações iniciadas quando inicia sessão
    .none = Nenhuma aplicação de arranque adicionada
    .remove-dialog-title = Remover { $name }?
    .remove-dialog-description = Tem a certeza de que pretende remover esta aplicação de arranque?
    .search-for-application = Procurar aplicação
legacy-applications = Compatibilidade com aplicações X11
    .desc = Dimensionamento de aplicações do sistema de janelas X11 e atalhos globais.
legacy-app-global-shortcuts = Atalhos globais em aplicações X11
    .desc = Os atalhos globais permitem que as teclas e os eventos do botão do rato executados em aplicações sejam reconhecidos por outras aplicações para funcionalidades como pressionar para falar ou pressionar para silenciar. Por predefinição, esta funcionalidade está desativada em aplicações X11 para garantir que outras aplicações não possam monitorizar eventos do teclado e do rato que contenham informações confidenciais.
    .none = Sem teclas
    .modifiers = Modificadores (Super, Shift, Control, Alt)
    .combination = Todas as teclas enquanto os modificadores Super, Control ou Alt estão a ser pressionados
    .all = Todas as teclas
    .mouse = Eventos do botão do rato em aplicações X11
legacy-app-scaling = Dimensionamento de aplicações do sistema X11 Window
    .scaled-gaming = Otimizar para jogos e aplicações em ecrã inteiro
    .gaming-description = As aplicações X11 podem parecer ligeiramente maiores/menores em comparação com as aplicações Wayland.
    .scaled-applications = Otimizar para aplicações
    .applications-description = Os jogos e as aplicações X11 em ecrã inteiro podem não corresponder à resolução do seu ecrã.
    .scaled-compatibility = Modo de compatibilidade máxima
    .compatibility-description = As aplicações X11 podem parecer desfocadas em ecrãs HiDPI.
    .preferred-display = Ecrã preferencial para jogos e aplicações X11 em ecrã inteiro
    .no-display = Nenhum
administrator = Administrador
    .desc = Os administradores podem alterar as definições de todos os utilizadores, adicionar e remover outros utilizadores.
add-user = Adicionar utilizador
change-password = Alterar palavra-passe
remove-user = Remover utilizador
full-name = Nome completo
invalid-username = Nome de utilizador inválido.
password-mismatch = A palavra-passe e a confirmação devem ser iguais.
save = Guardar
