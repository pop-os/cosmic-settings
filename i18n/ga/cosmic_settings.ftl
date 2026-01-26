app = Socruithe COSMIC
dbus-connection-error = Theip ar nascadh le DBus
ok = Ceart go leor
unknown = Anaithnid
number = { $number }

## Network & Wireless

connections-and-profiles =
    { $variant ->
        [wired] Sreangnasctha
        [wifi] Wi-Fi
        [vpn] VPN
       *[other] anaithnid
    } ceangail agus próifílí ceangailte.
add-network = Cuir líonra leis
    .profile = Cuir próifíl leis
add-vpn = Cuir VPN leis
airplane-on = Tá mód eitleáin ar siúl.
cable-unplugged = Cábla díphlugáilte
connect = Ceangail
connected = Ceangailte
connecting = Ag nascadh…
disconnect = Dícheangail
forget = Déan dearmad
known-networks = Líonraí aitheanta
network-and-wireless = Líonra & gan sreang
no-networks = Ní bhfuarthas aon líonraí.
no-vpn = Níl aon naisc VPN ar fáil.
password = Pasfhocal
password-confirm = Deimhnigh an pasfhocal
remove = Bain
settings = Socruithe
username = Ainm úsáideora
visible-networks = Líonraí infheicthe
identity = Céannacht
auth-dialog = Fíordheimhniú de dhíth
    .vpn-description = Cuir isteach an t-ainm úsáideora agus pasfhocal a theastaíonn ón tseirbhís VPN.
    .wifi-description = Cuir isteach an focal faire nó eochair criptithe. Is féidir leat ceangal a dhéanamh freisin ach an cnaipe “WPS” a bhrú ar an ródaire.
forget-dialog = Déan dearmad ar an líonra Wi-Fi seo?
    .description = Beidh ort pasfhocal a iontráil arís chun an líonra Wi-Fi seo a úsáid amach anseo.
network-device-state =
    .activated = Ceangailte
    .config = Ag ceangal
    .deactivating = Ag dícheangal
    .disconnected = Dícheangailte
    .failed = Theip ar cheangal
    .ip-check = Ag seiceáil an cheangail
    .ip-config = Ag iarraidh eolais IP agus ródaithe
    .need-auth = Teastaíonn fíordheimhniú
    .prepare = Ag ullmhú le ceangal
    .secondaries = Ag fanacht le nasc tánaisteach
    .unavailable = Níl ar fáil
    .unknown = Stát anaithnid
    .unmanaged = Gan bhainistiú
    .unplugged = Cábla díphlugáilte
remove-connection-dialog = Bain próifíl ceangail?
    .vpn-description = Beidh ort pasfhocal a chur isteach arís chun an líonra seo a úsáid amach anseo.
    .wired-description = Beidh ort an phróifíl seo a athchruthú chun í a úsáid amach anseo.
vpn = VPN
    .connections = Naisc VPN
    .error = Theip ar chumraíocht VPN a chur leis
    .remove = Bain próifíl naisc
    .select-file = Roghnaigh comhad cumraíochta VPN
vpn-error = Earráid VPN
    .config = Theip ar chumraíocht VPN a chur leis
    .connect = Theip ar nascadh le VPN
    .connection-editor = Theip ar an eagarthóir ceangail
    .connection-settings = Theip ar na socruithe a fháil le haghaidh naisc ghníomhacha
    .updating-state = Theip ar staid an bhainisteora líonra a nuashonrú
    .wireguard-config-path = Cosán comhaid neamhbhailí le haghaidh cumraíochta WireGuard
    .wireguard-config-path-desc = Caithfidh an comhad roghnaithe a bheith ar chóras comhad áitiúil.
    .wireguard-device = Theip ar chruthú gléas WireGuard
    .with-password =
        Theip ar VPN a shocrú { $field ->
           *[username] ainm úsáideora
            [password] pasfhocal
            [password-flags] pasfhocail-bratacha
        } le nmcli
wired = Sreangnasctha
    .adapter = Adaptóir sreangnasctha { $id }
    .connections = Ceangail sreangnasctha
    .devices = Gléasanna sreangnasctha
    .remove = Bain an próifíl ceangailte
wifi = Wi-Fi
    .adapter = Adaptóir Wi-Fi { $id }
    .forget = Déan dearmad ar an líonra seo
wireguard-dialog = Cuir gléas WireGuard leis
    .description = Roghnaigh ainm gléis don chumraíocht WireGuard.

## Networking: Online Accounts

online-accounts = Cuntais ar líne
    .desc = Cuir cuntais, IMAP agus SMTP, logáil isteach fiontair leis

# Bluetooth

activate = Gníomhachtaigh
confirm = Deimhnigh
enable = Cumasaigh
bluetooth = Bluetooth
    .desc = Bainistigh gléasanna Bluetooth
    .status = Tá an córas seo le feiceáil mar { $aliases } agus socruithe Bluetooth oscailte.
    .connected = Ceangailte
    .connecting = Ag ceangal
    .disconnecting = Ag dícheangal
    .connect = Ceangail
    .disconnect = Dícheangail
    .forget = Déan dearmad
    .dbus-error = Tharla earráid agus tú ag idirghníomhú le DBus: { $why }
    .disabled = Tá an tseirbhís bluetooth díchumasaithe
    .inactive = Níl an tseirbhís bluetooth gníomhach
    .unknown = Níorbh fhéidir an tseirbhís bluetooth a ghníomhachtú. An bhfuil blueZ suiteáilte?
bluetooth-paired = Gléasanna ceangailte roimhe seo
    .connect = Ceangail
    .battery = { $percentage }% cadhnra
bluetooth-confirm-pin = Deimhnigh PIN Bluetooth
    .description = Deimhnigh le do thoil go bhfuil an PIN seo a leanas ag teacht leis an gceann atá ar taispeáint { $device }
bluetooth-available = Gléasanna in aice láimhe
bluetooth-adapters = Cuibheoirí Bluetooth

## Accessibility

accessibility = Inrochtaineacht
    .vision = Fís
    .on = Ar
    .off = As
    .unavailable = Níl ar fáil
    .screen-reader = Léitheoir scáileáin
    .high-contrast = Mód codarsnachta ard
    .invert-colors = Inbhéartaigh dathanna
    .color-filters = Scagairí dathanna
hearing = Éisteacht
    .mono = Seinn fuaim steireo mar mhonó
default = Réamhshocrú
magnifier = Formhéadaitheoir
    .controls =
        Nó bain úsáid as na haicearraí seo: { $zoom_in ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_in } chun súmáil isteach,
        }{ $zoom_out ->
            [zero] { "" }
           *[other]
                { "" }
                { $zoom_out } chun súmáil amach,
        }
        Super + scrollaigh le do luch
    .scroll_controls = Cumasaigh súmáil luiche nó tadhaill le Super + Scrollaigh
    .show_overlay = Taispeáin an forleagan formhéadaitheoir
    .increment = Incrimint súmáil
    .signin = Tosaigh an formhéadaitheoir nuair a shíníonn tú isteach
    .applet = Cas an formhéadaitheoir ar/as san fheidhmchláirín ar an bpainéal
    .movement = Bogann an radharc súmáilte
    .continuous = Go leanúnach leis an pointeoir
    .onedge = Nuair a shroicheann an pointeoir an imeall
    .centered = Chun an pointeoir a choinneáil lárnach
color-filter = Cineál scagaire datha
    .unknown = Scagaire anaithnid gníomhach
    .greyscale = Liathscála
    .deuteranopia = Glas/Dearg (laige glas, Deuteranopia)
    .protanopia = Dearg/Glas (laige dearg, Protanopia)
    .tritanopia = Gorm/Buí (laige gorm, Tritanopia)

## Desktop

desktop = Deasc

## Desktop: Wallpaper

wallpaper = Cúlbhrat
    .change = Athraigh íomhá gach
    .desc = Íomhánna cúlbhrat, dathanna, agus roghanna taispeántas sleamhnán.
    .fit = Cúlbhrat oiriúnach
    .folder-dialog = Roghnaigh fillteán cúlbhrat
    .image-dialog = Roghnaigh íomhá cúlbhrat
    .plural = Cúlbhrat
    .same = Cúlbhrat céanna ar gach taispeáint
    .slide = Taispeántas sleamhnán
add-color = Cuir dath leis
add-image = Cuir íomhá leis
all-displays = Gach taispeántas
colors = Dathanna
dialog-add = Cuir leis
fill = Líon
fit-to-screen = Oiriúnach don scáileán
open-new-folder = Oscail fillteán nua
recent-folders = Fillteáin le déanaí
x-minutes =
    { $number } { $number ->
        [one] nóiméad
       *[other] nóiméid
    }
x-hours =
    { $number ->
        [1] 1 uair
       *[other] { $number } uair an chloig
    }
never = Riamh

## Desktop: Appearance

appearance = Dealramh
    .desc = Dathanna béime agus téamaíocht
accent-color = Dath béime
app-background = Cúlra fuinneoige
auto = Uath
close = Dún
color-picker = Roghnóir dathanna
copied-to-clipboard = Cóipeáilte chuig an ghearrthaisce
copy-to-clipboard = Cóipeáil chuig an ghearrthaisce
dark = Dorcha
export = Easpórtáil
hex = Heics
import = Iompórtáil
light = Solas
mode-and-colors = Mód agus dathanna
recent-colors = Dathanna le déanaí
reset-to-default = Athshocraigh go réamhshocraithe
rgb = RGB
window-hint-accent = Dath leid fuinneoige gníomhach
window-hint-accent-toggle = Úsáid dath béime téama mar leid fuinneoige gníomhach
auto-switch = Athraigh go huathoibríoch idir modhanna solas agus dorcha
    .sunrise = Aistríonn sé go mód solais ag éirí gréine
    .sunset = Aistríonn sé go mód dorcha ag luí na gréine
    .next-sunrise = Aistríonn sé go mód solais ag an gcéad éirí gréine eile
    .next-sunset = Aistríonn sé go mód dorcha ag luí na gréine seo chugainn
container-background = Cúlra coimeádán
    .desc-detail = Úsáidtear dath cúlra an choimeádáin le haghaidh barra taoibh nascleanúna, tarraiceán taoibh, dialóga agus giuirléidí chomhchosúla. De réir réamhshocraithe, díorthaítear dath cúlra an choimeádáin go huathoibríoch ó chúlra na fuinneoige.
    .reset = Athshocraigh go huathoibríoch
    .desc = Úsáidte le haghaidh barra taoibh nascleanúna, tarraiceán taobh, dialóga agus giuirléidí chomhchosúla
control-tint = Dath scátha comhpháirte rialaithe
    .desc = Úsáidtear é le haghaidh cúlra de chnaipí caighdeánacha, ionchuir chuardaigh, ionchuir téacs, agus comhpháirteanna comhchosúla
frosted = Éifeacht gloine reatha ar chóras an úsáideora
    .desc = Cuireann sé doiléiriú cúlra i bhfeidhm ar an bpainéal, an duga, na feidhmchláiríní, an lainseálaí, agus an leabharlann aip
enable-export = Cuir an téama seo i bhfeidhm ar aipeanna GNOME.
    .desc = Ní thacaíonn gach tacar uirlisí le huath-athrú. B’fhéidir go mbeadh gá aipeanna nach bhfuil i COSMIC a atosú tar éis athrú téama.
icon-theme = Téama deilbhíní
    .desc = Cuireann sé sraith éagsúil deilbhíní i bhfeidhm ar fheidhmchláir
text-tint = Tint téacs comhéadan
    .desc = Úsáidtear é chun dathanna téacs comhéadain a dhíorthú a bhfuil codarsnacht leordhóthanach acu ar dhromchlaí éagsúla
style = Stíl
    .round = Ciorclach
    .slightly-round = Ciorclach beagán
    .square = Cearnógach
interface-density = Dlús comhéadan
    .comfortable = Compordach
    .compact = Dlúth
    .spacious = Leathnaithe
window-management-appearance = Bainistíocht fuinneoga
    .active-hint = Méid leid na fuinneoige gníomhaí
    .gaps = Bearnaí timpeall fuinneoga tílithe

### Experimental

experimental-settings = Socruithe turgnamhach
icons-and-toolkit = Deilbhíní agus téamaíocht uirlisí
interface-font = Cló an chóras
monospace-font = Cló monospás

## Desktop: Notifications

notifications = Fógraí
    .desc = Ná cuir isteach, fógraí glasála scáileáin, agus socruithe in aghaidh an fheidhmchláir

## Desktop: Panel

panel = Painéal
    .desc = Barra córais phríomhúil do roghchláir agus feidhmchláiríní
add = Cuir leis
add-applet = Cuir feidhmchláirín leis
all = Gach
applets = Feidhmchláiríní
center-segment = Deighleog lárnach
end-segment = Deighleog deireadh
large = Mór
no-applets-found = Níor aimsíodh feidhmchláiríní...
panel-bottom = Bun
panel-left = Clé
panel-right = Deas
panel-top = Barr
search-applets = Cuardach feidhmchláiríní...
small = Beag
start-segment = Deighleog tosaigh
panel-appearance = Dealramh
    .match = Meaitseáil deasc
    .light = Solas
    .dark = Dorcha
panel-behavior-and-position = Iompar agus suíomhanna
    .autohide = Folaigh an painéal go huathoibríoch
    .dock-autohide = Folaigh an duga go huathoibríoch
    .position = Suíomh ar an scáileán
    .display = Taispeáin ar taispeáint
panel-style = Stíl
    .anchor-gap = Bearna idir imill an phainéil agus imill an scáileáin
    .dock-anchor-gap = Bearna idir imill an duga agus imill an scáileáin
    .extend = Síneadh an painéal go himill an scáileáin
    .dock-extend = Síneadh duga go himill an scáileáin
    .appearance = Dealramh
    .size = Méid
    .background-opacity = Teimhneacht cúlra
panel-applets = Cumraíocht
    .dock-desc = Cumraigh feidhmchláiríní duga
    .desc = Cumraigh feidhmchláiríní painéil
panel-missing = Tá cumraíocht an phainéil ar iarraidh
    .desc = Tá an comhad cumraíochta painéil ar iarraidh mar gheall ar úsáid chumraíochta saincheaptha nó tá sé truaillithe.
    .fix = Athshocraigh go dtí an réamhshocrú

## Desktop: Dock

dock = Duga
    .desc = Barra roghnach le haghaidh aipeanna agus feidhmchláiríní

## Desktop: Window management

window-management = Bainistíocht fuinneoige
    .desc = Gníomh eochair Super, roghanna rialaithe fuinneog, agus roghanna breise tíliú fuinneog
super-key = Gníomh na heochrach Super
    .launcher = Oscail lainseálaí
    .workspaces = Oscail spásanna oibre
    .applications = Oscail feidhmchláir
    .disable = Díchumasaigh
edge-gravity = Imtharraingítear fuinneoga snámhacha chuig imill in aice láimhe
window-controls = Rialuithe fuinneoige
    .maximize = Taispeáin an cnaipe uasmhéadaithe
    .minimize = Taispeáin an cnaipe íoslaghdaithe
    .active-window-hint = Taispeáin leid fuinneoige gníomhach
focus-navigation = Nascleanúint fócais
    .focus-follows-cursor = Leanann an fócas cúrsóir
    .focus-follows-cursor-delay = Leanann fócas moill cúrsóra in ms
    .cursor-follows-focus = Leanann an cúrsóir fócas

## Desktop: Workspaces

workspaces = Spásanna oibre
    .desc = Treoshuíomh agus iompar spás oibre
workspaces-behavior = Iompar spáis oibre
    .dynamic = Spásanna oibre dinimiciúla
    .dynamic-desc = Baintear spásanna oibre folamh go huathoibríoch.
    .fixed = Líon seasta spásanna oibre
    .fixed-desc = Cuir leis nó bain spásanna oibre san fhorbhreathnú.
workspaces-multi-behavior = Iompar Il-mhonatóireachta
    .span = Spásanna oibre thar thaispeántais
    .separate = Tá spásanna oibre ar leithligh ag taispeántais
workspaces-overview-thumbnails = Mionsamhlacha forbhreathnaithe spás oibre
    .show-number = Taispeáin uimhir an spás oibre
    .show-name = Taispeáin ainm an spás oibre
workspaces-orientation = Treoshuíomh spásanna oibre
    .vertical = Ingearach
    .horizontal = Cothrománach
hot-corner = Cúinne te
    .top-left-corner = Cumasaigh cúinne te barr-chlé do spásanna oibre

## Displays

-requires-restart = Teastaíonn atosú
color = Dath
    .depth = Doimhneacht datha
    .profile = Próifíl datha
    .sidebar = Próifílí dathanna
    .temperature = Teocht datha
display = Taispeántais
    .desc = Bainistigh taispeántais agus solas oíche
    .arrangement = Socrú taispeántais
    .arrangement-desc = Tarraing taispeántais chun iad a athshocrú
    .enable = Cumasaigh an taispeáint
    .external = { $size } { $output } taispeáint sheachtrach
    .laptop = { $size } taispeáint ríomhaire glúine
    .options = Roghanna taispeána
    .refresh-rate = Ráta athnuachana
    .resolution = Taifeach
    .scale = Scála
    .additional-scale-options = Roghanna scála breise
mirroring = Scáthánú
    .id = Scáthánú { $id }
    .dont = Ná scáthánaigh
    .mirror = Scáthánaigh { $display }
    .project =
        Tionscnaigh chuig { $display ->
            [all] gach taispeáint
           *[other] { $display }
        }
    .project-count =
        Ag taispeáint ar { $count } eile { $count ->
            [1] taispeáint
           *[other] taispeántais
        }
night-light = Solas oíche
    .auto = Uathoibríoch (luí na gréine go héirí na gréine)
    .desc = Laghdaigh solas gorm le dathanna níos teo
orientation = Treoshuíomh
    .standard = Caighdeán
    .rotate-90 = Rothlaigh 90
    .rotate-180 = Rothlaigh 180
    .rotate-270 = Rothlaigh 270
vrr = Ráta athnuachana athraitheach
    .enabled = Cumasaithe
    .force = I gcónaí
    .auto = Uathoibríoch
    .disabled = Díchumasaithe
scheduling = Sceidealú
    .manual = Sceideal láimhe
dialog = Dialóg
    .title = Coinnigh na socruithe taispeána seo?
    .keep-changes = Coinnigh athruithe
    .change-prompt = Athrófar athruithe socruithe go huathoibríoch i { $time } soicindí.
    .revert-settings = Athraigh na socruithe

## Sound

sound = Fuaim
    .desc = N/A
sound-output = Aschur
    .volume = Toirt aschuir
    .device = Gléas aschuir
    .level = Leibhéal aschuir
    .config = Cumraíocht
    .balance = Iarmhéid
    .left = Clé
    .right = Deas
sound-input = Ionchur
    .volume = Toirt ionchuir
    .device = Gléas ionchuir
    .level = Leibhéal ionchuir
sound-alerts = Foláirimh
    .volume = Toirt foláirimh
    .sound = Fuaime foláirimh
sound-applications = Feidhmchláir
    .desc = Toirt agus socruithe feidhmchlár

## Power

power = Cumhacht & ceallraí
    .desc = Bainistigh socruithe cumhachta
battery = Ceallraí
    .minute =
        { $value } { $value ->
            [one] nóiméad
           *[other] nóiméid
        }
    .hour =
        { $value } { $value ->
            [one] uair
           *[other] uair an chloig
        }
    .day =
        { $value } { $value ->
            [one] lá
           *[other] laethanta
        }
    .less-than-minute = Níos lú ná nóiméad
    .and = agus
    .remaining-time =
        { $time } go dtí { $action ->
            [full] lán
           *[other] folamh
        }
connected-devices = Gléasanna ceangailte
    .unknown = Gléas anaithnid
power-mode = Mód cumhachta
    .battery = Saolré ceallraí sínte
    .battery-desc = Úsáid chumhachta laghdaithe agus feidhmíocht chiúin
    .balanced = Cothrom
    .balanced-desc = Feidhmíocht chiúin agus úsáid chumhachta mheasartha
    .performance = Ardfheidhmíocht
    .performance-desc = Feidhmíocht agus úsáid chumhachta buaic
    .no-backend = Níor aimsíodh an cúl-deireadh. Suiteáil system76-power nó power-profiles-daemon.
power-saving = Roghanna coigilte cumhachta
    .turn-off-screen-after = Múch an scáileán ina dhiaidh sin
    .auto-suspend = Fionraí uathoibríoch
    .auto-suspend-ac = Fionraí uathoibríoch nuair a bhíonn sé plugáilte isteach
    .auto-suspend-battery = Fionraí uathoibríoch ar chumhacht ceallraí

## Input

acceleration-desc = Coigeartaíonn sé íogaireacht rianaithe go huathoibríoch bunaithe ar luas
disable-while-typing = Díchumasaigh agus tú ag clóscríobh
input-devices = Gléasanna ionchuir
    .desc = Gléasanna ionchuir
primary-button = Cnaipe príomhúil
    .desc = Socraíonn sé ord na gcnaipí fisiciúla
    .left = Clé
    .right = Deas
scrolling = Scrollaigh
    .two-finger = Scrollaigh le dhá mhéar
    .edge = Scrollaigh feadh an imeall le méar amháin
    .speed = Luas scrollaithe
    .natural = Scrollaigh nádúrtha
    .natural-desc = Scrollaigh an t-ábhar, in ionad an radhairc

## Input: Keyboard

slow = Mall
fast = Tapa
short = Gairid
long = Fada
keyboard = Méarchlár
    .desc = Foinsí ionchuir, athrú, iontráil carachtar speisialta, aicearraí
keyboard-sources = Foinsí ionchuir
    .desc = Is féidir foinsí ionchuir a athrú trí úsáid a bhaint as teaglaim eochair Super+Spás. Is féidir é seo a shaincheapadh sna socruithe aicearra méarchláir.
    .move-up = Bog suas
    .move-down = Bog síos
    .settings = Socruithe
    .view-layout = Féach ar leagan amach an mhéarchláir
    .remove = Bain
    .add = Cuir foinse ionchuir leis
keyboard-special-char = Iontráil Carachtar Speisialta
    .alternate = Eochair charachtar malartach
    .compose = Eochair chumadóireachta
    .compose-desc = Ligeann an eochair chumadóireachta raon leathan carachtar a iontráil. Chun í a úsáid, brúigh an eochair chumadóireachta agus ansin sraith carachtar. Mar shampla, má bhrúnn tú an eochair chumadóireachta agus C agus o ina dhiaidh sin, cuirfear © isteach; agus má bhrúnn tú an eochair chumadóireachta agus a agus ‘ ina dhiaidh sin, cuirfear á isteach.
    .caps = Eochair Caps Lock
keyboard-typing-assist = Ag clóscríobh
    .repeat-rate = Ráta athuair
    .repeat-delay = Moill athrá
keyboard-numlock-boot = Glas Uimhreach
    .boot-state = Staid ag tosú
    .last-boot = An tosú deireanach
    .on = Ar
    .off = As
    .set = Socraigh staid tosaithe Ghlas Uimhreach
added = Curtha leis
type-to-search = Clóscríobh le cuardach...
show-extended-input-sources = Taispeáin foinsí ionchuir sínte

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Aicearraí méarchláir
    .desc = Féach ar aicearraí agus saincheap iad
cancel = Cealaigh
command = Ordú
custom = Saincheaptha
debug = Dífhabhtaigh
disabled = Díchumasaithe
input-source-switch = Athraigh foinse ionchuir teanga an mhéarchláir
migrate-workspace-prev = Aistrigh spás oibre go dtí an t-aschur roimhe seo
migrate-workspace-next = Aistrigh spás oibre go dtí an chéad aschur eile
migrate-workspace =
    Imirce spás oibre chuig aschur { $direction ->
       *[down] síos
        [left] clé
        [right] deas
        [up] suas
    }
navigate = Nascleanúint
replace = Ionadaigh
shortcut-name = Ainm aicearra
system-controls = Rialuithe córais
terminate = Foirceann
toggle-stacking = Scoránaigh cruachta fuinneoige
type-key-combination = Cineál teaglaim eochair
custom-shortcuts = Aicearraí saincheaptha
    .add = Cuir aicearra leis
    .context = Cuir aicearra saincheaptha leis
    .none = Gan aon aicearraí saincheaptha
modified = { $count } modhnaithe
nav-shortcuts = Nascleanúint
    .prev-output = Fócas ar aschur roimhe seo
    .next-output = Fócas ar an gcéad aschur eile
    .last-workspace = Fócas ar an spás oibre deireanach
    .prev-workspace = Fócas ar spás oibre roimhe seo
    .next-workspace = Fócas ar an gcéad spás oibre eile
    .focus =
        Fuinneog fócais { $direction ->
           *[down] síos
            [in] isteach
            [left] clé
            [out] amach
            [right] deas
            [up] suas
        }
    .output =
        Athraigh chuig aschur { $direction ->
           *[down] síos
            [left] clé
            [right] deas
            [up] suas
        }
    .workspace = Athraigh go spás oibre { $num }
manage-windows = Bainistigh fuinneoga
    .close = Dún an fhuinneog
    .maximize = Uasmhéadaigh an fhuinneog
    .fullscreen = Fuinneog lánscáileáin
    .minimize = Íoslaghdaigh an fhuinneog
    .resize-inwards = Athraigh méid na fuinneoige isteach
    .resize-outwards = Athraigh méid na fuinneoige amach
    .toggle-sticky = Scoránaigh an fhuinneog greamaitheach
move-windows = Bog fuinneoga
    .direction =
        Bog Fuinneog { $direction ->
           *[down] síos
            [left] clé
            [right] deas
            [up] suas
        }
    .display =
        Bog fuinneog monatóir amháin { $direction ->
           *[down] síos
            [left] clé
            [right] deas
            [up] suas
        }
    .workspace =
        Bog fuinneog spás oibre amháin { $direction ->
           *[below] thíos
            [left] clé
            [right] deas
            [above] thuas
        }
    .workspace-num = Bog an fhuinneog go spás oibre { $num }
    .prev-workspace = Bog an fhuinneog go dtí an spás oibre roimhe seo
    .next-workspace = Bog an fhuinneog go dtí an chéad spás oibre eile
    .last-workspace = Bog an fhuinneog go dtí an spás oibre deireanach
    .next-display = Bog an fhuinneog go dtí an chéad taispeáint eile
    .prev-display = Bog an fhuinneog go dtí an taispeáint roimhe seo
    .send-to-prev-workspace = Bog an fhuinneog go dtí an spás oibre roimhe seo
    .send-to-next-workspace = Bog an fhuinneog go dtí an chéad spás oibre eile
system-shortcut = Córas
    .app-library = Oscail an leabharlann aip
    .brightness-down = Laghdú gile taispeáint
    .brightness-up = Méadú gile taispeáint
    .display-toggle = Athraigh an taispeáint inmheánach
    .home-folder = Oscail fillteán baile
    .keyboard-brightness-down = Laghdú ar ghile an mhéarchláir
    .keyboard-brightness-up = Méadú ar ghile an mhéarchláir
    .launcher = Oscail an tosaitheoir
    .log-out = Logáil Amach
    .lock-screen = Glas an scáileán
    .mute = Balbhaigh aschur fuaime
    .mute-mic = Balbhaíonn ionchur micreafón
    .play-pause = Seinn/Sos
    .play-next = An chéad rian eile
    .play-prev = Rian roimhe seo
    .poweroff = Múch
    .screenshot = Tóg seat scáileáin
    .suspend = Cuir ar fionraí
    .terminal = Oscail teirminéal
    .touchpad-toggle = Scoránaigh an ceap tadhaill
    .volume-lower = Laghdú ar an méid aschuir fuaime
    .volume-raise = Méadú ar an aschur fuaime
    .web-browser = Osclaíonn sé brabhsálaí gréasáin
    .window-switcher = Athraigh idir fuinneoga oscailte
    .window-switcher-previous = Athraigh idir fuinneoga oscailte cúlaithe
    .workspace-overview = Oscail an forbhreathnú spás oibre
window-tiling = Tíleáil fuinneoige
    .horizontal = Socraigh treoshuíomh cothrománach
    .vertical = Socraigh treoshuíomh ingearach
    .swap-window = Malartaigh fuinneog
    .toggle-tiling = Athraigh tíleáil fuinneoige
    .toggle-stacking = Athraigh cruacháil fuinneoige
    .toggle-floating = Athraigh snámh na fuinneoige
    .toggle-orientation = Athraigh treoshuíomh
replace-shortcut-dialog = Ionadaigh aicearra?
    .desc = { $shortcut } in úsáid ag { $name }. Má tá tú in ionad é, { $name } beidh sé díchumasaithe.
zoom-in = Súmáil isteach
zoom-out = Súmáil amach

## Input: Mouse

mouse = Luch
    .desc = Luas luch, luasghéarú, scrollú nádúrtha
    .speed = Luas luch
    .acceleration = Cumasaigh luasghéarú na luiche

## Input: Touchpad

click-behavior = Iompar cliceála
    .click-finger = Cliceáil thánaisteach le dhá mhéara agus cliceáil lár le trí mhéara
    .button-areas = Cliceáil thánaisteach sa chúinne íochtarach deas agus cliceáil lár sa lár íochtarach
pinch-to-zoom = Piorraigh chun súmáil
    .desc = Úsáid dhá mhéar chun súmáil isteach san ábhar, i gcás feidhmchlár a thacaíonn le súmáil
tap-to-click = Tapáil le cliceáil
    .desc = Cumasaíonn sé tapáil le méar amháin le haghaidh cliceáil phríomhúil, tapáil le dhá mhéar le haghaidh cliceáil thánaisteach, agus tapáil le trí mhéar le haghaidh cliceáil lár
touchpad = Ceap tadhaill
    .acceleration = Cumasaigh luasghéarú an ceap tadhaill
    .desc = Luas an ceap tadhaill, roghanna cliceáil, gothaí
    .speed = Luas an ceap tadhaill

## Input: Gestures

gestures = Gothaí
    .four-finger-down = Sciob ceithre mhéara anuas
    .four-finger-left = Sciob ceithre mhéara ar chlé
    .four-finger-right = Sciob ceithre mhéara ar dheis
    .four-finger-up = Sciob ceithre mhéara suas
    .three-finger-any = Sciob trí mhéara i dtreo ar bith
switch-workspaces = Athraigh spásanna oibre
    .horizontal = Sciob ceithre mhéara ar chlé/ar dheis
    .vertical = Sciob ceithre mhéara suas/anuas
switch-between-windows = Athraigh idir fuinneoga
open-application-library = Oscail Leabharlann Feidhmchlár
open-workspaces-view = Oscail radharc spásanna oibre

## Time & Language

time = Am & teanga
    .desc = N/A
time-date = Dáta & am
    .desc = Crios ama, socruithe uathoibríocha cloig, agus formáidiú ama
    .auto = Socraigh go huathoibríoch
    .auto-ntp = Nuashonrófar an dáta agus an t-am go huathoibríoch nuair a shocrófar an crios ama
time-zone = Crios ama
    .auto = Crios ama uathoibríoch
    .auto-info = Teastaíonn seirbhísí suímh agus rochtain idirlín
time-format = Formáid dáta & ama
    .twenty-four = am 24 uair an chloig
    .show-seconds = Taispeáin soicindí
    .first = An chéad lá den tseachtain
    .show-date = Taispeáin an dáta san feidhmchláirín ama
    .friday = Aoine
    .saturday = Satharn
    .sunday = Domhnach
    .monday = Luan
time-region = Réigiún & teanga
    .desc = Formáidigh dátaí, amanna agus uimhreacha bunaithe ar do réigiún
formatting = Formáidiú
    .dates = Dátaí
    .time = Am
    .date-and-time = Dáta & am
    .numbers = Uimhreacha
    .measurement = Tomhas
    .paper = Páipéar
preferred-languages = Teangacha is fearr leat
    .desc = Cinneann ord na dteangacha cén teanga a úsáidtear don chomhéadan úsáideora. Cuirtear na hathruithe i bhfeidhm an chéad logáil isteach eile.
add-language = Cuir teanga leis
    .context = Cuir teanga leis
install-additional-languages = Suiteáil teangacha breise
region = Réigiún

## Applications

applications = Feidhmchláir

## Applications: Default Applications

default-apps = Feidhmchláir réamhshocraithe
    .desc = Brabhsálaí gréasáin réamhshocraithe, cliant ríomhphoist, brabhsálaí comhad, agus feidhmchláir eile
    .web-browser = Brabhsálaí gréasáin
    .file-manager = Bainisteoir comhad
    .mail-client = Cliant ríomhphoist
    .music = Ceol
    .video = Físeán
    .photos = Grianghraif
    .calendar = Féilire
    .terminal = Teirminéal
    .other-associations = Cumainn eile
    .text-editor = Eagarthóir téacs

## Applications: Startup Applications

startup-apps = Feidhmchláir tosaithe
    .desc = Cumraigh feidhmchláir a ritheann ar logáil isteach.
    .add = Cuir feidhmchlár leis
    .user = Lainseáil feidhmchláir nuair a logálann tú isteach
    .none = Níor cuireadh aon fheidhmchláir tosaithe leis
    .remove-dialog-title = Bain { $name }?
    .remove-dialog-description = An bhfuil tú cinnte gur mian leat é seo a bhaint mar fheidhmchlár tosaithe?
    .add-startup-app = Cuir feidhmchlár tosaithe leis

## Applications: Legacy Applications

legacy-applications = Comhoiriúnacht feidhmchláir X11
    .desc = Scálú feidhmchlár córais fuinneoige X11 agus aicearraí domhanda
legacy-app-global-shortcuts = Aicearraí domhanda i feidhmchláir X11
    .desc = Le haicearraí domhanda, is féidir le feidhmchláir eile aitheantas a thabhairt do bhuillí eochracha agus d’imeachtaí cnaipe luiche a dhéantar in aipeanna le haghaidh gnéithe cosúil le brúigh-chun-caint nó brúigh-chun-balbhú. De réir réamhshocraithe, tá sé seo díchumasaithe in aipeanna X11 lena chinntiú nach féidir le feidhmchláir eile monatóireacht a dhéanamh ar imeachtaí méarchláir agus luiche ina bhfuil faisnéis íogair.
    .none = Gan eochracha
    .modifiers = Mionathraitheoirí (Super, Shift, Control, Alt)
    .combination = Gach eochair agus na modhnóirí Super, Control nó Alt á mbrú
    .all = Gach eochair
    .mouse = Imeachtaí cnaipe luiche in fheidhmchláir X11
legacy-app-scaling = Scálú feidhmchlár córas fuinneoige X11
    .scaled-gaming = Optamaigh le haghaidh cearrbhachais agus feidhmchláir lánscáileáin
    .gaming-description = D’fhéadfadh feidhmchláir X11 a bheith beagán níos mó/níos lú i gcomparáid le feidhmchláir Wayland.
    .scaled-applications = Optamaigh le haghaidh feidhmchláir
    .applications-description = B’fhéidir nach mbeidh cluichí agus feidhmchláir lán-scáileáin X11 ag teacht le réiteach do thaispeána
    .scaled-compatibility = Mód comhoiriúnachta uasta
    .compatibility-description = D’fhéadfadh feidhmchláir X11 a bheith doiléir ar scáileáin HiDPI
    .preferred-display = Taispeántas is fearr le haghaidh cluichí agus feidhmchláir X11 lánscáileáin
    .no-display = Gan aon cheann

## System

system = Córas & cuntais

## System: About

about = Maidir
    .desc = Ainm gléis, faisnéis crua-earraí, réamhshocruithe córais oibriúcháin
about-device = Ainm an ghléis
    .desc = Feictear an t-ainm seo ar líonraí nó ar ghléasanna Bluetooth eile
about-hardware = Crua-earraí
    .model = Múnla crua-earraí
    .memory = Cuimhne
    .processor = Próiseálaí
    .graphics = Grafaicí
    .disk-capacity = Toilleadh diosca
about-os = Córas oibriúcháin
    .os = Córas oibriúcháin
    .os-architecture = Ailtireacht an chórais oibriúcháin
    .kernel = Leagan an eithne
    .desktop-environment = Timpeallacht deisce
    .windowing-system = Córas fuinneoige
about-related = Socruithe gaolmhara
    .support = Faigh tacaíocht

## System: Firmware

firmware = Dochtearraí
    .desc = Sonraí maidir le dochtearraí

## System: Users

users = Úsáideoirí
    .desc = Fíordheimhniú agus cuntais úsáideora
    .admin = Riarthóir
    .standard = Caighdeán
    .profile-add = Roghnaigh íomhá phróifíl
administrator = Riarthóir
    .desc = Is féidir le riarthóirí socruithe a athrú do gach úsáideoir, úsáideoirí eile a chur leis agus a bhaint
add-user = Cuir úsáideoir leis
change-password = Athraigh pasfhocal
remove-user = Bain úsáideoir
full-name = Ainm iomlán
invalid-username = Ainm úsáideora neamhbhailí
password-mismatch = Ní mór don phasfhocal agus don dearbhú a bheith mar an gcéanna
save = Sábháil
amplification = Aimpliú
    .desc = Ceadaíonn sé seo an toirt a ardú go 150%
add-another-keybinding = Cuir ceangal eochrach eile leis
qr-code-unavailable = Níl an cód QR ar fáil
network-name = Ainm an líonra
share = Comhroinn líonra
scan-to-connect-description = Scan an cód QR chun ceangal leis an líonra seo.
place-here = Leag feidhmchláiríní anseo
sound-device-port-unplugged = Díphlugáilte
sound-hd-audio = Fuaim HD
sound-usb-audio = Fuaim USB
sound-device-profiles = Próifílí gléasanna
shadows-floating = Fuinneoga ar snámh
    .clip = Meaitseáil coirnéil an chórais agus cuir scáthanna i bhfeidhm
shadows-tiling = Fuinneoga tílithe
    .clip = Meaitseáil coirnéil an chórais
    .shadow = Cuir scáthanna i bhfeidhm
shadow-and-corners = Scáth agus coirnéil fuinneoige
