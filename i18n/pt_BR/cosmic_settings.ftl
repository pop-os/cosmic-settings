app = Configurações do COSMIC

unknown = Desconhecido

number = { $number }

## Desktop

desktop = Desktop

## Desktop: Appearance

appearance = Aparência
    .desc = Cores de destaque e temas COSMIC.

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
mode-and-colors = Modo e cores
recent-colors = Cores recentes
reset-to-default = Restaurar padrão
rgb = RGB
window-hint-accent = Cor de destaque da janela ativa
window-hint-accent-toggle = Utilizar a cor de destaque do tema como destaque de janela ativa

auto-switch = Alternar automaticamente do modo Claro para o modo Escuro
    .sunrise = Altera para o modo Claro ao nascer do sol
    .sunset = Altera para o modo Claro ao pôr do sol
    .next-sunrise = Altera para o modo Claro no próximo nascer do sol
    .next-sunset = Altera para o modo Claro no próximo pôr do sol

container-background = Fundo do contêiner
    .desc-detail = A cor de fundo do contêiner é usada para a barra lateral de navegação, gaveta lateral, nas caixas de diálogos e em widgets similares. Por padrão, ela é automaticamente derivada do fundo da aplicação ou da janela. 
    .reset = Restaurar para automático
    .desc = A cor primária do contêiner é usada para a barra lateral de navegação, gaveta lateral, caixas de diálogos, e widgets similares.

control-tint = Tonalidade dos componentes de controle
    .desc = Usado para os fundos dos botões padrão, entradas de busca, entradas de texto e componentes similares.

frosted = Efeito de vidro fosco na interface do sistema
    .desc = Ativa desfoque de fundo ao painel, dock, applets, lançador e biblioteca de aplicativos.

enable-export = Aplicar este tema aos aplicativos GNOME.
    .desc = Nem todos os conjuntos de ferramenta suportam troca automática. Aplicativos não-COSMIC podem precisar ser reiniciados após uma mudança de tema.

icon-theme = Tema de ícones
    .desc = Aplica um conjunto diferente de ícones para aplicativos.

text-tint = Tonalidade do texto da interface
    .desc = Cor usada para derivar cores de texto da interface que têm contraste suficiente em várias superfícies.

style = Estilo
    .round = Arredondado
    .slightly-round = Levemente arredondado
    .square = Quadrado

# interface density left out for now
window-management = Gerenciamento de Janelas
    .active-hint = Tamanho da cor destaque da janela ativa
    .gaps = Espaçamentos ao redor de janelas organizadas em mosaico

## Desktop: Display

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

graphics-mode = Graphics mode
    .mode = { $mode ->
        [compute] Calcular
        *[hybrid] Híbrido
        [integrated] Integrado
        [nvidia] NVIDIA
    } graphics
    .enable = Ativar { $mode ->
        [compute] calcular
        *[hybrid] híbrido
        [integrated] integrado
        [nvidia] NVIDIA
    } graphics
    .desc = { $mode ->
        [compute] Utiliza gráficos dedicados apenas para cargas  de trabalho computacionais. Desativa monitores externos. { -requires-restart }.
        *[hybrid] Aplicativos usam os gráficos integrados, a menos que solicitado explicitamente para usar os gráficos dedicados. { -requires-restart }.
        [integrated] Desativa os gráficos integrados, exceto se for explicitamente solicitada a utilização de gráficos dedicados.
        [nvidia] Melhor experiência gráfica e maior consumo de energia. { -requires-restart }.
    }
    .restart = Reiniciar e alterar para { $mode }?
    .restart-desc = Alterar para { $mode } fechará todos as aplicações abertas

mirroring = Espelhar
    .id = Espelhar { $id }
    .dont = Não espelhar
    .mirror = Espelhar { $display }
    .project = Projetar para { $display ->
        [all] todos os monitores
        *[other] { $display }
    }
    .project-count = Projetar para { $count} outros { $count ->
        [1] monitores
        *[other] monitores
    }

night-light = Luz noturna
    .auto = Automático (do pôr ao nascer do sol)
    .desc = Reduz a luz azul com cores mais quentes.

orientation = Orientação
    .standard = Padrão
    .rotate-90 = Rotacionar 90
    .rotate-180 = Rotacionar 180
    .rotate-270 = Rotacionar 270

scheduling = Agendamento
    .manual = Agendamento manual

## Desktop: Notifications

notifications = Notificações
    .desc = Não perturbe, notificações da tela de bloqueio, e configurações por aplicação.

## Desktop: Options

desktop-panel-options = Área de Trabalho e Painel
    .desc = Ações da tecla Super, cantos ativos, opções de controle de janela.

desktop-panels-and-applets = Painés e Aplicativos de Área de Trabalho

dock = Dock
    .desc = Painel com aplicativos fixados.

hot-corner = Canto ativo
    .top-left-corner = Ativar o canto superior esquerdo para áreas de trabalho

super-key-action = Ações da tecla Super
    .launcher = Inicializador
    .workspaces = Áreas de Trabalho
    .applications = Aplicações

top-panel = Painel superior
    .workspaces = Mostrar Botão de Áreas de Trabalho
    .applications = Mostrar Botão de Aplicações

window-controls = Controles da Janela
    .minimize = Mostrar Botão de Minimizar
    .maximize = Mostrar Botão de Maximizar

## Desktop: Panel

panel = Painel
    .desc = Barra superior com controles e menus da área de trabalho.

add = Adicionar
add-applet = Add Applet
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
search-applets = Procurando applets...
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

panel-missing = Configuração do Painel Ausente
    .desc = O arquivo de configuração do painel está ausente devido ao uso de uma configuração personalizada ou encontra-se corrompido.
    .fix = Redefinir para padrão

## Desktop: Wallpaper

wallpaper = Papel de Parede
    .change = Mudar imagem a cada
    .desc = Imagens de papel de parede, cores e opções de apresentação de slide.
    .fit = Ajuste do Papel de Parede
    .folder-dialog = Escolher pasta do papel de parede
    .image-dialog = Escolher imagem do papel de parede
    .plural = Papéis de Parede
    .same = Mesmo papel de parede em todos os monitores
    .slide = Apresentação de slide

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

## Networking: Wired

wired = Cabo de rede
    .desc = Conexões por cabo de rede, perfis de ligação

## Networking: Online Accounts

online-accounts = Contas online
    .desc = Adicionar contas, IMAP e SMTP, acessos empresariais

## Time & Language

time = Hora & Idioma
    .desc = N/D

time-date = Data e Hora
    .desc = Fuso horário, definições automáticas de relógio e algumas formatação de hora.
    .auto = Definir automáticamente

time-zone = Fuso horário
    .auto = Fuso horário automático
    .auto-info = Requer serviços de localização e acesso à Internet

time-format = Formato de Data e Hora
    .twenty-four = Formato de 24 horas
    .first = Primeiro dia da semana
    .show-date = Mostrar Data no Painel Superior
    .friday = Sexta-feira
    .saturday = Sábado
    .sunday = Domingo
    .monday = Segunda

time-region = Região e Idioma
    .desc = Formatar datas, horas e números baseado na sua região

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

sound-alerts = Aletras
    .volume = Volume dos alertas
    .sound = Som dos alertas

sound-applications = Aplicações
    .desc = Volumes e definições das aplicações

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

about-related = Definições relacionadas
    .support = Obter ajuda

## System: Firmware

firmware = Firmware
    .desc = Detalhes do firmware.

## System: Users

users = Users
    .desc = Autenticação e acesso, tela de bloqueio.

## Input

acceleration-desc = Ajusta automaticamente a sensibilidade do seguimento com base na velocidade.

disable-while-typing = Desabilitar enquanto escreve

input-devices = Dispositivos de entrada
    .desc = Dispositivos de entrada

primary-button = Botão Primário
    .left = Esquerda
    .right = Direita

scrolling = Rolagem
    .two-finger = Rolagem com dois dedos
    .edge = Rolagem ao longo da borda com um dedo
    .speed = Velocidade de rolagem
    .natural = Rolagem natural
    .natural-desc = Rolar o conteúdo, em vez da visualização

## Input: Keyboard

keyboard = Teclado
    .desc = Entrada do teclado

keyboard-sources = Fontes de Entrada
    .desc = As fontes de entrada podem ser alternadas usando a combinação de teclas Super+Espaço. Isso pode ser personalizado nas configurações de atalho de teclado.
    .move-up = Mover para cima
    .move-down = Mover para baixo
    .settings = Configurações
    .view-layout = Ver layout do teclado
    .remove = Remover

keyboard-special-char = Entrada de Caracteres Especiais
    .alternate = Tecla de caracteres alternativos
    .compose = Tecla de composição

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Atalhos do teclado
    .desc = Ver e customizar atalhos

## Input: Mouse

mouse = Mouse
    .desc = Velocidade do mouse, aceleração e rolagem natural.
    .speed = Velocidade do mouse
    .acceleration = Ativar aceleração do mouse

## Input: Touchpad

click-behavior = Comportamento de Cliques
    .click-finger = Clique secundário com dois dedos e clique do meio com três dedos
    .button-areas = Clique secundário no canto inferior direito e clique do meio no centro inferior

pinch-to-zoom = Pinçar para ampliar
    .desc = Use dois dedos para ampliar o conteúdo, para aplciativos que suportam zoom.

tap-to-click = Toque para clicar
    .desc = Habilita o toque de um dedo para clique primário, toque de dois dedos para clique secundário e toque de três dedos para o clique do meio.

touchpad = Touchpad
    .acceleration = Habilitar aceleração do touchpad
    .desc = Velocidade do touchpad, opções de clique e gestos.
    .speed = Velocidade do touchpad

## Input: Gestures

swiping = Deslizando
    .four-finger-down = Deslizar quatros dedos para baixo
    .four-finger-left = Deslizar quatro dedos para esquerda
    .four-finger-right = Deslizar quatro dedos para direita
    .four-finger-up = Deslizar quatro dedos para cima
    .three-finger-any = Deslizar três dedos para qualquer direção

switch-between-windows = Alternar entre janelas
switch-to-next-workspace = Alternar para próxima área de trabalho
switch-to-prev-workspace = Alternar para área de trabalho anterior
open-application-library = Abrir Biblioteca de Aplicativos
open-workspaces-view = Abrir Visão Geral das Áreas de Trabalho