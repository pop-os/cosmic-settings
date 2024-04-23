app = Definições COSMIC

unknown = Desconhecido

number = { $number }

## Desktop

desktop = Ambiente de trabalho

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
    .desc = Aplica uma desfocagem de fundo ao painel, à doca, aos applets, ao lançador e à biblioteca de aplicações.

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

# interface density left out for now
window-management = Gestão de janelas
    .active-hint = Tamanho da sugestão da janela ativa
    .gaps = Espaços à volta das janelas em mosaico

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

graphics-mode = Modo gráfico
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
        [compute] Utiliza gráficos dedicados apenas para cargas de trabalho computacionais. Desativa os ecrãs externos. { -requires-restart }.
        *[hybrid] As aplicações utilizam gráficos integrados, exceto se for explicitamente solicitada a utilização de gráficos dedicados. { -requires-restart }.
        [integrated] Desliga os gráficos dedicados para uma maior duração da bateria e menos ruído da ventoinha.
        [nvidia] Melhor experiência gráfica e maior consumo de energia. { -requires-restart }.
    }
    .restart = Reiniciar e mudar para { $mode }?
    .restart-desc = Mudar para { $mode } fechará todas as aplicações abertas

mirroring = Espelhar
    .id = Espelhar { $id }
    .dont = Não espelhar
    .mirror = Espelhar { $display }
    .project = Projeto para { $display ->
        [all] todos os ecrãs
        *[other] { $display }
    }
    .project-count = Projetar para { $count} outros { $count ->
        [1] ecrã
        *[other] ecrãs
    }

night-light = Luz noturna
    .auto = Automático (do pôr ao nascer do sol)
    .desc = Reduz a luz azul com cores mais quentes.

orientation = Orientação
    .standard = Padrão
    .rotate-90 = Rodar 90
    .rotate-180 = Rodar 180
    .rotate-270 = Rodar 270

scheduling = Agendamento
    .manual = Agendamento manual

## Desktop: Notifications

notifications = Notificações
    .desc = Não incomodar, notificações no ecrã de bloqueio e definições por aplicação.

## Desktop: Options

desktop-panel-options = Opções do ambiente de trabalho
    .desc = Ação da tecla Super, cantos ativos, opções de controlo de janelas.

desktop-panels-and-applets = Painéis e Applets

dock = Doca
    .desc = Painel com aplicações afixadas.

hot-corner = Canto ativo
    .top-left-corner = Ativar o canto superior esquerdo para as áreas de trabalho

super-key-action = Ação da tecla Super
    .launcher = Lançador
    .workspaces = Áreas de trabalho
    .applications = Aplicações

top-panel = Painel superior
    .workspaces = Mostrar o botão das áreas de trabalho
    .applications = Mostrar o botão das aplicações

window-controls = Controlos de janela
    .minimize = Mostrar o botão de minimizar
    .maximize = Mostrar o botão de maximizar

## Desktop: Panel

panel = Painel
    .desc = Barra superior com controlos e menus do ambiente de trabalho.

add = Adicionar
add-applet = Adicionar Applet
all = Todos
applets = Applets
center-segment = Segmento central
drop-here = Largar applets aqui
end-segment = Segmento final
large = Grande
no-applets-found = Nenhum applet encontrado...
panel-bottom = Inferior
panel-left = Esquerda
panel-right = Direita
panel-top = Superior
search-applets = Procurar applets...
small = Pequeno
start-segment = Segmento inicial

panel-appearance = Aspeto
    .match = Combinar com o ambiente de trabalho
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
    .appearance = Aspeto
    .size = Tamanho
    .background-opacity = Opacidade do fundo

panel-applets = Configuração
    .dock-desc = Configura os applets da doca.
    .desc = Configura os applets do painel.

panel-missing = A configuração do painel está em falta
    .desc = O ficheiro de configuração do painel está em falta devido à utilização de uma configuração personalizada ou está corrompido.
    .fix = Repor a predefinição

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
dialog-add = _Adicionar
fill = Preencher
fit-to-screen = Ajustar ao ecrã
open-new-folder = Abrir nova pasta
recent-folders = Pastas recentes

x-minutes = { $number } minutos
x-hours = { $number ->
    [1] 1 hora
    *[other] { $number } horas
}

## Desktop: Workspaces

workspaces = Áreas de trabalho
    .desc = Definir número, comportamento e posição da área de rabalho.

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

## Networking: Wired

wired = Cabo de rede
    .desc = Ligações por cabo de rede, perfis de ligação

## Networking: Online Accounts

online-accounts = Contas online
    .desc = Adicionar contas, IMAP e SMTP, acessos empresariais

## Time & Language

time = Hora e Idioma
    .desc = N/D

time-date = Data e Hora
    .desc = Fuso horário, definições automáticas de relógio e alguma formatação de hora.
    .auto = Definir automaticamente

time-zone = Fuso horário
    .auto = Fuso horário automático
    .auto-info = Requer serviços de localização e acesso à Internet

time-format = Formato de Data e Hora
    .twenty-four = Formato de 24 horas
    .first = Primeiro dia da semana
    .show-date = Mostrar a data no painel superior
    .friday = Sexta-feira
    .saturday = Sábado
    .sunday = Domingo
    .monday = Segunda-feira

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

sound-alerts = Alertas
    .volume = Volume dos alertas
    .sound = Som dos alertas

sound-applications = Aplicações
    .desc = Volumes e definições das aplicações

## System

system = Sistema e Contas

## System: About

about = Acerca
    .desc = Nome do dispositivo, informação do equipamento, padrões do sistema operativo

about-device = Nome do dispositivo
    .desc = Este nome aparece a outros dispositivos de rede ou bluetooth

about-hardware = Hardware
    .model = Modelo do equipamento
    .memory = Memória
    .processor = Processador
    .graphics = Gráficos
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
    .desc = Autenticação e acesso, ecrã de bloqueio.

## Input

acceleration-desc = Ajusta automaticamente a sensibilidade do seguimento com base na velocidade.

disable-while-typing = Desativar enquanto escreve

input-devices = Dispositivos de entrada
    .desc = Dispositivos de entrada

primary-button = Primary button
    .left = Left
    .right = Right

scrolling = Deslocação
    .two-finger = Deslocar com dois dedos
    .edge = Deslocar ao longo da margem com um dedo
    .speed = Velocidade de deslocação
    .natural = Deslocação natural
    .natural-desc = Deslocar o conteúdo, em vez da visualização

## Input: Keyboard

keyboard = Teclado
    .desc = Entrada do teclado

keyboard-sources = Fontes de entrada
    .desc = As fontes de entrada podem ser comutadas utilizando a combinação de teclas Super+Espaço. Isto pode ser personalizado nas definições de teclas de atalho.
    .move-up = Subir
    .move-down = Descer
    .settings = Definições
    .view-layout = Ver esquema do teclado
    .remove = Remover
    .add = Adicionar fonte de entrada

keyboard-special-char = Entrada de carateres especiais
    .alternate = Tecla de caracteres alternativos
    .compose = Tecla de composição

added = Adicionado
type-to-search = Escreva para procurar...

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Teclas de atalho
    .desc = Ver e personalizar atalhos

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

swiping = Deslizar
    .four-finger-down = Deslizar quatro dedos para baixo
    .four-finger-left = Deslizar quatro dedos para a esquerda
    .four-finger-right = Deslizar quatro dedos para a direita
    .four-finger-up = Deslizar quatro dedos para cima
    .three-finger-any = Passar com três dedos em qualquer direção

switch-between-windows = Alternar entre janelas
switch-to-next-workspace = Mudar para a área de trabalho seguinte
switch-to-prev-workspace = Mudar para a área de trabalho anterior
open-application-library = Abrir biblioteca de aplicações
open-workspaces-view = Abrir visão geral das áreas de trabalho
