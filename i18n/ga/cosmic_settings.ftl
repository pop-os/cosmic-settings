app = Socruithe COSMIC

dbus-connection-error = Theip ar nascadh le DBus
ok = Ceart go leor
unknown = Anaithnid

number = { $number }

## Network & Wireless

connections-and-profiles = { $variant ->
    [wired] Nasctha
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
known-networks = Líonraí Aitheanta
network-and-wireless = Líonra & Gan Sreang
no-networks = Ní bhfuarthas aon líonraí.
no-vpn = Níl aon naisc VPN ar fáil.
password = Pasfhocal
password-confirm = Deimhnigh Pasfhocal
remove = Bain
settings = Socruithe
username = Ainm úsáideora
visible-networks = Líonraí Infheicthe
identity = Féiniúlacht

auth-dialog = Fíordheimhniú de dhíth
    .vpn-description = Cuir isteach an t-ainm úsáideora agus pasfhocal a theastaíonn ón tseirbhís VPN.
    .wifi-description = Cuir isteach an focal faire nó eochair criptithe. Is féidir leat ceangal a dhéanamh freisin ach an cnaipe “WPS” a bhrú ar an ródaire.

forget-dialog = Déan dearmad ar an líonra Wi-Fi seo?
    .description = Beidh ort pasfhocal a chur isteach arís chun an líonra Wi-Fi seo a úsáid amach anseo.

network-device-state =
    .activated = Ceangailte
    .config = Ag nascadh
    .deactivating = Ag dícheangal
    .disconnected = Dícheangailte
    .failed = Theip ar nascadh
    .ip-check = Ceangal á sheiceáil
    .ip-config = IP á iarraidh agus faisnéis ródaithe
    .need-auth = Tá fíordheimhniú de dhíth
    .prepare = Ag ullmhú chun nascadh
    .secondaries = Ag fanacht le nasc tánaisteach
    .unavailable = Níl ar fáil
    .unknown = Stát anaithnid
    .unmanaged = Gan bhainistiú
    .unplugged = Cábla díphlugáilte

remove-connection-dialog = Bain Próifíl Ceangail?
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
    .with-password = Theip ar VPN a shocrú { $field ->
        *[username] ainm úsáideora
        [password] pasfhocal
        [password-flags] pasfhocail-bratacha
    } le nmcli

wired = Sreangaithe
    .adapter = Adaptóir sreangaithe { $id }
    .connections = Ceangail Sreangaithe
    .devices = Gléasanna Sreangaithe
    .remove = Bain an próifíl ceangailte

wifi = Wi-Fi
    .adapter = Adaptóir Wi-Fi { $id }
    .forget = Déan dearmad ar an líonra seo

wireguard-dialog = Cuir gléas WireGuard leis
    .description = Roghnaigh ainm gléis don chumraíocht WireGuard.

## Networking: Online Accounts

online-accounts = Cuntais Ar Líne
    .desc = Cuir cuntais, IMAP agus SMTP, logáil isteach fiontair leis

# Bluetooth

activate = Gníomhachtaigh
confirm = Deimhnigh
enable = Cumasaigh

bluetooth = Bluetooth
    .desc = Bainistigh gléasanna Bluetooth
    .status = Tá an córas seo le feiceáil mar { $aliases } agus socruithe Bluetooth oscailte.
    .connected = Ceangailte
    .connecting = Ag nascadh
    .disconnecting = Ag dícheangal
    .connect = Ceangail
    .disconnect = Dícheangail
    .forget = Déan dearmad
    .dbus-error = Tharla earráid agus tú ag idirghníomhú le DBus: { $why }
    .disabled = Tá an tseirbhís bluetooth díchumasaithe
    .inactive = Níl an tseirbhís bluetooth gníomhach
    .unknown = Níorbh fhéidir an tseirbhís bluetooth a ghníomhachtú. An bhfuil bluez suiteáilte?

bluetooth-paired = Gléasanna Ceangailte Roimhe Seo
    .connect = Ceangail
    .battery = { $percentage }% cadhnra

bluetooth-confirm-pin = Deimhnigh PIN Bluetooth
    .description = Deimhnigh le do thoil go bhfuil an UAP seo a leanas ag teacht leis an gceann atá ar taispeáint { $device }

bluetooth-available = Gléasanna in aice láimhe

bluetooth-adapters = Cuibheoirí Bluetooth

## Accessibility

accessibility = Inrochtaineacht
    .vision = Fís
    .on = Ar
    .off = As
    .unavailable = Níl ar fáil
    .high-contrast = Mód codarsnachta ard
    .invert-colors = Inbhéartaigh Dathanna
    .color-filters = Scagairí dathanna

hearing = Éisteacht
    .mono =Seinn fuaim steireo mar mhonó

default = Réamhshocrú
magnifier = Formhéadaitheoir
    .controls = Nó bain úsáid as na haicearraí seo: { $zoom_in ->
             [zero] {""}
            *[other] {""}
                {$zoom_in} chun súmáil isteach,
        }{ $zoom_out ->
             [zero] {""}
            *[other] {""}
                {$zoom_out} chun súmáil amach,
        }
        Super + scrollaigh le do luch
    .scroll_controls = Cumasaigh súmáil luiche nó tadhaill le Super + Scrollaigh
    .show_overlay = Taispeáin an Forleagan Formhéadúcháin
    .increment = Incrimint súmála
    .signin = Tosaigh an formhéadaitheoir nuair a shíníonn tú isteach
    .applet = Cas an formhéadaitheoir air/as san aipléid ar an bpainéal
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
    .desc = Íomhánna Cúlbhrat, dathanna, agus roghanna taispeántas sleamhnán.
    .fit = Cúlbhrat oiriúnach
    .folder-dialog = Roghnaigh fillteán cúlbhrat
    .image-dialog = Roghnaigh íomhá cúlbhrat
    .plural = Cúlbhrat
    .same = Cúlbhrat céanna ar gach taispeáint
    .slide = Taispeántas sleamhnán

add-color = Cuir dath leis
add-image = Cuir íomhá leis
all-displays = Gach Taispeáint
colors = Dathanna
dialog-add = Cuir
fill = Líon
fit-to-screen = Oiriúnach don Scáileán
open-new-folder = Oscail fillteán nua
recent-folders = Fillteáin le Déanaí

x-minutes = { $number } nóiméad
x-hours = { $number ->
    [1] 1 uair
    *[other] { $number } uair an chloig
}
never = Riamh

## Desktop: Appearance

appearance = Dealramh
    .desc = Dathanna béime agus téamaí.

accent-color = Dath béime
app-background = Feidhmchlár nó cúlra fuinneoige
auto = Uath
close = Dún
color-picker = Roghnóir Dathanna
copied-to-clipboard = Cóipeáladh chuig an ngearrthaisce
copy-to-clipboard = Cóipeáil chuig an ngearrthaisce
dark = Dorcha
export = Easpórtáil
hex = Heics
import = Iompórtáil
light = Solas
mode-and-colors = Mód agus Dathanna
recent-colors = Dathanna le déanaí
reset-to-default = Athshocraigh go réamhshocrú
rgb = RGB
window-hint-accent = Dath leid fuinneoige gníomhach
window-hint-accent-toggle = Úsáid dath accent téama mar leid fuinneoige gníomhach

auto-switch = Athraigh go huathoibríoch idir modhanna Solas agus Dorcha
    .sunrise = Aistríonn sé go mód Solais ag éirí gréine
    .sunset = Aistríonn sé go mód Dorcha ag luí na gréine
    .next-sunrise = Aistríonn sé go mód an tSolais ag an gcéad éirí gréine eile
    .next-sunset = Aistríonn sé go mód Dorcha ag luí na gréine seo chugainn

container-background = Cúlra coimeádán
    .desc-detail = Úsáidtear dath cúlra an choimeádáin le haghaidh barra taoibh nascleanúna, tarraiceán taoibh, dialóga agus giuirléidí dá samhail. De réir réamhshocraithe, díorthaítear go huathoibríoch é ó chúlra Feidhmchláir nó fuinneoige.
    .reset = Athshocraigh go huathoibríoch
    .desc = Úsáidtear dath coimeádán príomhúil le haghaidh barra taoibh nascleanúna, tarraiceán taoibh, dialóga agus giuirléidí dá samhail.

control-tint = Dath scáth comhpháirt rialaithe
    .desc = Úsáidtear é le haghaidh cúlra de chnaipí caighdeánacha, ionchuir chuardaigh, ionchuir téacs, agus comhpháirteanna comhchosúla.

frosted = Éifeacht gloine reatha ar chóras an úsáideora
    .desc = Cuireann sé blur cúlra i bhfeidhm ar an bpainéal, an doca, na haipléid, an túsghabháil, agus an leabharlann aip.

enable-export = Cuir an téama seo i bhfeidhm ar aipí GNOME.
    .desc = Ní thacaíonn gach uirlis le féin-athrú. D'fhéadfadh go mbeidh gá le athosclaí a dhéanamh ar aipí neamh-COSMIC tar éis athrú téama.

icon-theme = Téama Deilbhín
    .desc = Cuireann sé sraith éagsúil deilbhíní i bhfeidhm ar fheidhmchláir.

text-tint = Tint téacs comhéadan
    .desc = Dath a úsáidtear chun dathanna téacs comhéadain a bhfuil codarsnacht leordhóthanach acu ar dhromchlaí éagsúla a dhíorthú.

style = Stíl
    .round = Ciorclach
    .slightly-round = Ciorclach beagán
    .square = Cearnógach

interface-density = Dlús Comhéadan
    .comfortable = Compordach
    .compact = Dlúth
    .spacious = Leathnaithe

window-management-appearance = Bainistíocht Fuinneoga
    .active-hint = Méid leideanna fuinneoga gníomhacha
    .gaps = Bearnaí timpeall na bhfuinneoganna tíleáilte

### Experimental

experimental-settings = Socruithe Turgnamhach
icons-and-toolkit = Témpleáil ícon agus uirlis
interface-font = Cló an chóras
monospace-font = Cló monospásach

## Desktop: Notifications

notifications = Fógraí
    .desc = Ná Dhoirteadh, fógraí scáileán glasála, agus socruithe de réir aipeanna.

## Desktop: Panel

panel = Panel
    .desc = Barr uachtarach le rialuithe deasc agus comhéadan.

add = Cuir
add-applet = Cuir Feidhmchláirín leis
all = Gach
applets = Feidhmchláiríní
center-segment = Deighleog an Ionaid
drop-here = Cuir feidhmchláiríní anseo
end-segment = Deighleog Críoch
large = Mór
no-applets-found = Níor aimsíodh feidhmchláiríní...
panel-bottom = Bun
panel-left = Ar chlé
panel-right = Ar dheis
panel-top = Barr
search-applets = Cuardach feidhmchláiríní...
small = Beaga
start-segment = Tosaigh Deighleog

panel-appearance = Dealramh
    .match = Meaitseáil deasc
    .light = Solas
    .dark = Dorcha

panel-behavior-and-position = Iompar agus Seasaimh
    .autohide = Folaigh an painéal go huathoibríoch
    .dock-autohide = Folaigh duga go huathoibríoch
    .position = An suíomh ar an scáileán
    .display = Taispeáin ar taispeáint

panel-style = Stíl
    .anchor-gap = Bearna idir imill an phainéil agus imill an scáileáin
    .dock-anchor-gap = Bearna idir imill an duga agus imill an scáileáin
    .extend = Síneadh an painéal go himill an scáileáin
    .dock-extend = Síneadh duga go himill an scáileáin
    .appearance = Dealramh
    .size = Méid
    .background-opacity = Doimhneacht cúlra

panel-applets = Cumraíocht
    .dock-desc = Cumraigh feidhmchláiríní duga
    .desc = Cumraigh feidhmchláiríní painéil

panel-missing = Tá Cumraíocht an Phainéil ar Iarraidh
    .desc = Tá comhad cumraíochta an phainéil in easnamh mar gheall ar chumraíocht shaincheaptha a úsáid nó go bhfuil sé truaillithe.
    .fix = Athshocraigh go réamhshocrú

## Desktop: Dock

dock = Duga
    .desc = Painéal le feidhmchláir pinn sa tráidire aip agus feidhmchláiríní eile.

## Desktop: Window management

window-management = Bainistíocht fuinneoige
    .desc = Sárghníomhaíocht eochair, roghanna rialaithe fuinneoige, agus roghanna breise tíliú fuinneoige.

super-key = Gníomh na heochrach Super
    .launcher = Oscail Tosaitheoir
    .workspaces = Oscail spásanna oibre
    .applications = Oscail Feidhmchláir
    .disable = Díchumasaigh

edge-gravity = Imíonn fuinneoga snámhacha go dtí imill in aice láimhe

window-controls = Rialuithe Fuinneog
    .maximize = Taispeáin an cnaipe uasmhéadú
    .minimize = Taispeáin cnaipe íoslaghdú
    .active-window-hint = Taispeáin leid fuinneoige gníomhach

focus-navigation = Nascleanúint Fócas
    .focus-follows-cursor = Leanann an fócas cúrsóir
    .focus-follows-cursor-delay = Leanann fócas moill cúrsóra in ms
    .cursor-follows-focus = Leanann an cúrsóir fócas

## Desktop: Workspaces

workspaces = Spásanna oibre
    .desc = Treoshuíomh agus iompar spás oibre.

workspaces-behavior = Iompar Spás Oibre
    .dynamic = Spásanna oibre dinimiciúla
    .dynamic-desc = Baintear spásanna oibre folamha go huathoibríoch.
    .fixed = Líon Seasta Spásanna Oibre
    .fixed-desc = Cuir leis nó bain spásanna oibre san fhorbhreathnú.

workspaces-multi-behavior = Iompar Il-mhonatóireachta
    .span = Taispeántais Réise Spásanna Oibre
    .separate = Bíodh Spásanna Oibre ar Leith ag Taispeántais

workspaces-overview-thumbnails = Mionsamhlacha Forbhreathnú Spás Oibre
    .show-number = Taispeáin Uimhir Spás Oibre
    .show-name = Taispeáin Ainm Spás Oibre

workspaces-orientation = Treoshuíomh Spásanna Oibre
    .vertical = Ingearach
    .horizontal = Cothrománach

hot-corner = Cúinne Te
    .top-left-corner = Cumasaigh cúinne te barr-chlé do Spásanna Oibre

## Displays

-requires-restart = Teastaíonn atosú

color = Dath
    .depth = Doimhneacht datha
    .profile = Próifíl datha
    .sidebar = Próifílí dathanna
    .temperature = Teocht datha

display = Taispeántais
    .desc = Bainistigh taispeántais, aistriú grafaicí, agus solas oíche
    .arrangement = Socrú Taispeána
    .arrangement-desc = Tarraing taispeántais chun iad a atheagrú.
    .enable = Cumasaigh taispeáint
    .external = { $size } { $output } Taispeáint Seachtrach
    .laptop = { $size } Taispeáint Glúine
    .options = Roghanna Taispeána
    .refresh-rate = Ráta athnuachana
    .resolution = Taifeach
    .scale = Scáil
    .additional-scale-options = Roghanna scála breise

mirroring = Scáthánú
    .id = Scáthánú { $id }
    .dont = Ná scáthán
    .mirror = Scáthán { $display }
    .project = Caith chuig { $display ->
        [all] gach taispeáint
        *[other] { $display }
    }
    .project-count = Ag caith chuig { $count} eile { $count ->
        [1] taispeáint
        *[other] taispeántais
    }

night-light = Solas Oíche
    .auto = Uathoibríoch (ó luí na gréine go éirí gréine)
    .desc = Laghdaigh solas gorm le dathanna níos teo.

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
    .title = Coinnigh na Socruithe Taispeána seo?
    .keep-changes = Coinnigh Athruithe
    .change-prompt = Rachaidh athruithe socruithe ar ais go huathoibríoch i gceann { $time } soicind.
    .revert-settings = Cuir Socruithe ar ais

## Sound

sound = Fuaim
    .desc = N/A

sound-output = Aschur
    .volume = Toirt aschuir
    .device = Gléas aschuir
    .level = Leibhéal aschuir
    .config = Cumraíocht
    .balance = Iarmhéid
    .left = Ar chlé
    .right = Ar dheis

sound-input = Ionchur
    .volume = Toirt ionchuir
    .device = Gléas ionchuir
    .level = Leibhéal ionchuir

sound-alerts = Foláirimh
    .volume = Toirt foláirimh
    .sound = Fuaime foláirimh

sound-applications = Feidhmchláir
    .desc = Toirt agus socruithe feidhmchlár

profile = Próifíl

## Power

power = Cumhacht & Cadhnra
    .desc = Bainistigh socruithe cumhachta

battery = Ceallraí
  .minute = { $value } { $value ->
        [one] nóiméad
       *[other] nóiméad
  }
  .hour = { $value } { $value ->
        [one] uair
       *[other] uair an chloig
  }
  .day = { $value } { $value ->
        [one] lá
       *[other] laethanta
  }
  .less-than-minute = Níos lú ná nóiméad
  .and = agus
  .remaining-time = { $time } go dtí { $action ->
        [full] lán
       *[other] folamh
   }

connected-devices = Gléasanna Nasctha
  .unknown = Gléasanna Nasctha

power-mode = Mód Cumhachta
    .battery = Saol ceallraí leathnaithe
    .battery-desc = Úsáid cumhachta laghdaithe agus feidhmíocht chiúin.
    .balanced = Cothromaithe
    .balanced-desc = Feidhmíocht chiúin agus úsáid chumhachta mheasartha.
    .performance = Feidhmíocht ard
    .performance-desc = Feidhmíocht buaic agus úsáid ard chumhachta.
    .no-backend = Níor aimsíodh an cúlra. Suiteáil system76-power nó power-profiles-daemon.

power-saving = Roghanna Sábháil Cumhachta
    .turn-off-screen-after = Tiontaigh an scáileán off tar éis
    .auto-suspend = Cosc uathoibríoch
    .auto-suspend-ac = Cosc uathoibríoch nuair atá sé ceangailte
    .auto-suspend-battery = Cosc uathoibríoch nuair atá sé ar chumas na ceallraí

## Input

acceleration-desc = Coigeartaíonn sé go huathoibríoch íogaireacht rianaithe bunaithe ar luas.

disable-while-typing = Díchumasaigh agus tú ag clóscríobh

input-devices = Gléasanna Ionchuir
    .desc = Gléasanna Ionchuir

primary-button = Cnaipe príomhúil
    .desc = Socraíonn sé ord na gcnaipí fisiciúla.
    .left = Ar chlé
    .right = Ar dheis

scrolling = Scrollaigh
    .two-finger = Scrollaigh le dhá mhéar
    .edge = Scrollaigh feadh an imeall le méar amháin
    .speed = Luas scrollaithe
    .natural = Scrollaigh nádúrtha
    .natural-desc = Scrollaigh an t-ábhar, in ionad an radhairc

## Input: Keyboard

slow = Mall
fast = Go tapa
short = Gearr
long = Fada
keyboard = Méarchlár
    .desc = Foinsí ionchuir, aistriú, iontráil carachtar speisialta, aicearraí.

keyboard-sources = Foinsí Ionchuir
    .desc = Is féidir foinsí ionchuir a athrú trí úsáid a bhaint as teaglaim eochair Super+Spás. Is féidir é seo a shaincheapadh sna socruithe aicearra méarchláir.
    .move-up = Bog suas
    .move-down = Bog síos
    .settings = Socruithe
    .view-layout = Féach ar leagan amach an mhéarchláir
    .remove = Bain
    .add = Cuir foinse ionchuir leis

keyboard-special-char = Iontráil Carachtair Speisialta
    .alternate = Eochair charachtair mhalartacha
    .compose = Cum eochair
    .caps = Caps Lock eochair

keyboard-typing-assist = Ag clóscríobh
    .repeat-rate = Ráta athuair
    .repeat-delay = Moill athrá

keyboard-numlock-boot = GlasUimhir
    .boot-state = Stát ar tosaithe
    .last-boot = An tosaithe deireanach
    .on = Ar
    .off = As
    .set = Socraigh staid tosaithe glas uimhir

added = Curtha leis
type-to-search = Clóscríobh le cuardach...
show-extended-input-sources = Taispeáin foinsí ionchuir sínte

## Input: Keyboard: Shortcuts

keyboard-shortcuts = Aicearra Méarchláir
    .desc = Féach ar agus saincheapaigh aicearraí

add-keybinding = Cuir eochaircheangal leis
cancel = Cealaigh
command = Ordú
custom = Saincheaptha
debug = Dífhabhtaigh
disabled = Díchumasaithe
input-source-switch = Athraigh foinse ionchuir teanga an mhéarchláir
migrate-workspace-prev = Aistrigh spás oibre go dtí an t-aschur roimhe seo
migrate-workspace-next = Aistrigh spás oibre go dtí an chéad aschur eile
migrate-workspace = Aistrigh spás oibre go dtí an t-aschur { $direction ->
    *[down] síos
    [left] ar chlé
    [right] ar dheis
    [up] suas
}
navigate = Nascleanúint
replace = Ionadaigh
shortcut-name = Ainm aicearra
system-controls = Rialuithe córais
terminate = Foirceann
toggle-stacking = Scoránaigh cruachta fuinneoige
type-key-combination = Cineál teaglaim eochair

custom-shortcuts = Aicearraí Saincheaptha
    .add = Cuir aicearra leis
    .context = Cuir Aicearra Saincheaptha leis
    .none = Gan aicearraí saincheaptha

modified = { $count } modhnaithe

nav-shortcuts = Nascleanúint
    .prev-output = Fócas ar aschur roimhe seo
    .next-output = Fócas ar an gcéad aschur eile
    .last-workspace = Fócas ar an spás oibre deireanach
    .prev-workspace = Fócas ar spás oibre roimhe seo
    .next-workspace = Fócas ar an gcéad spás oibre eile
    .focus = Fuinneog fócais { $direction ->
        *[down] síos
        [in] isteach
        [left] ar chlé
        [out] amach
        [right] ar dheis
        [up] suas
    }
    .output = Athraigh chuig aschur { $direction ->
        *[down] síos
        [left] ar chlé
        [right] ar dheis
        [up] suas
    }
    .workspace = Téigh go spás oibre { $num }

manage-windows = Bainistigh fuinneoga
    .close = Dún an fhuinneog
    .maximize = Uasmhéadaigh an fhuinneog
    .minimize = Íoslaghdaigh an fhuinneog
    .resize-inwards = Athraigh méid na fuinneoige isteach
    .resize-outwards = Athraigh méid na fuinneoige amach
    .toggle-sticky = Scoránaigh an fhuinneog greamaitheach

move-windows = Bog Fuinneoga
    .direction = Bog Fuinneog { $direction ->
        *[down] síos
        [left] ar chlé
        [right] ar dheis
        [up] suas
    }
    .display = Bog fhuinneog amháin monatóir { $direction ->
        *[down] síos
        [left] ar chlé
        [right] ar dheis
        [up] suas
    }
    .workspace = Bog fuinneog amháin spás oibre { $direction ->
        *[below] thíos
        [left] ar chlé
        [right] ar dheis
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
    .screenshot = Tóg seat scáileáin
    .terminal = Oscail teirminéal
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
    .toggle-floating = Athraigh snámh fuinneoige
    .toggle-orientation = Athraigh treoshuíomh

replace-shortcut-dialog = Ionadaigh Aicearra?
    .desc = { $shortcut } in úsáid ag { $name }. Má tá tú in ionad é, { $name } beidh sé faoi mhíchumas.

zoom-in = Súmáil Isteach
zoom-out = Súmáil Amach

## Input: Mouse

mouse = Luch
    .desc = Luas luch, luasghéarú, scrollú nádúrtha.
    .speed = Luas luch
    .acceleration = Cum luasghéarú luch

## Input: Touchpad

click-behavior = Cliceáil iompar
    .click-finger = Cliceáil dhúchais le dhá mhéara agus cliceáil lár le trí mhéara
    .button-areas = Cliceáil dhúchais sa chúinne íochtarach dheis agus cliceáil lár sa lár íochtarach

pinch-to-zoom = Piorraigh chun aonaid
    .desc = Úsáid dhá mhéar chun an ábhar a mhéadú, le haghaidh aipeanna a tacaíonn le méadóireacht.

tap-to-click = Cliceáil le bualadh
    .desc = Cumas tapáil le méar amháin chun cliceáil príomhúil, tapáil le dhá mhéar chun cliceáil dhúchais, agus tapáil le trí mhéar chun cliceáil lár.

touchpad = Clár láimhe
    .acceleration = Cum luasghéarú clár láimhe
    .desc = Luas clár láimhe, roghanna cliceáil, giotaí.
    .speed = Luas clár láimhe

## Input: Gestures

gestures = Giotaí
    .four-finger-down = Sciob ceithre mhéara anuas
    .four-finger-left = Sciob ceithre mhéara ar chlé
    .four-finger-right = Sciob ceithre mhéara ar dheis
    .four-finger-up = Sciob ceithre mhéara suas
    .three-finger-any = Sciob trí mhéara i dtreo ar bith

switch-workspaces = Athraigh spásanna oibre
    .horizontal = Sciob ceithre mhéara ar chlé/deis
    .vertical = Sciob ceithre mhéara suas/anuas

switch-between-windows = Athraigh idir fuinneoga
open-application-library = Oscail Leabharlann Feidhmchlár
open-workspaces-view = Oscail Radharc Spásanna Oibre

## Time & Language

time = Am & Teanga
    .desc = N/A

time-date = Dáta & Am
    .desc = Crios ama, socruithe clog uathoibríoch, agus roinnt formáidiú ama.
    .auto = Socraigh go huathoibríoch
    .auto-ntp = Déanfar dáta agus am a nuashonrú go huathoibríoch nuair a bheidh an crios ama socraithe.

time-zone = Crios Ama
    .auto = Crios ama uathoibríoch
    .auto-info = Teastaíonn seirbhísí suímh agus rochtain idirlín

time-format = Formáid Dáta & Am
    .twenty-four = Am 24-uair
    .show-seconds = Taispeáin soicindí
    .first = An chéad lá den tseachtain
    .show-date = Taispeáin Dáta ar an bPainéal Barr
    .friday = Aoine
    .saturday = Satharn
    .sunday = Domhnach
    .monday = Luan

time-region = Réigiún & Teanga
    .desc = Formáid dátaí, ama, agus uimhreacha bunaithe ar do réigiún

formatting = Formáidiú
    .dates = Dátaí
    .time = Am
    .date-and-time = Dáta & Am
    .numbers = Uimhreacha
    .measurement = Tomhas
    .paper = Páipéar

preferred-languages = Teangacha Rogha
    .desc = Cinneann ord na dteangacha cén teanga a úsáidtear chun an deasc a aistriú. Tagann athruithe i bhfeidhm ar an gcéad logáil isteach eile.

add-language = Cuir teanga leis
    .context = Cuir teanga leis
install-additional-languages = Suiteáil teangacha breise
region = Réigiún

## Applications

applications = Feidhmchláir

## Applications: Default Applications

default-apps = Feidhmchláir Réamhshocraithe
    .desc = Brabhsálaí gréasáin réamhshocraithe, cliant ríomhphoist, brabhsálaí comhad, agus feidhmchláir eile.
    .web-browser = Brabhsálaí gréasáin
    .file-manager = Bainisteoir comhad
    .mail-client = Cliant ríomhphoist
    .music = Ceol
    .video = Físeán
    .photos = Grianghraif
    .calendar = Féilire
    .terminal = Teirminéal
    .other-associations = Cumainn Eile
    .text-editor = Eagarthóir Téacs

## Applications: Startup Applications

startup-apps = Feidhmchláir Tosaithe
    .desc = Cumraigh feidhmchláir a ritheann ar logáil isteach.
    .add = Cuir feidhmchlár leis
    .user = Feidhmchláir shonracha don úsáideoir
    .user-description = Seoltar na feidhmchláir seo nuair a logálann tú isteach i d’úsáideoir reatha.
    .remove-dialog-title = Bain { $name }?
    .remove-dialog-description = An bhfuil tú cinnte gur mian leat é seo a bhaint mar fheidhmchlár tosaithe?
    .search-for-application = Cuardaigh feidhmchlár

## Applications: Legacy Applications

legacy-applications = Comhoiriúnacht Feidhmchláir X11
    .desc = Scálú feidhmchlár córais fuinneoige X11 agus aicearraí domhanda.

legacy-app-global-shortcuts = Aicearraí Domhanda in Feidhmchláir X11
    .desc = Le haicearraí domhanda, is féidir le feidhmchláir eile aitheantas a thabhairt do bhuillí eochracha agus d’imeachtaí cnaipe luiche a dhéantar in aipeanna le haghaidh gnéithe cosúil le brúigh-chun-caint nó brúigh-chun-balbhú. De réir réamhshocraithe, tá sé seo díchumasaithe in aipeanna X11 lena chinntiú nach féidir le feidhmchláir eile monatóireacht a dhéanamh ar imeachtaí méarchláir agus luiche ina bhfuil faisnéis íogair.
    .none = Gan eochracha
    .modifiers = Mionathraitheoirí (Super, Shift, Control, Alt)
    .combination = Gach eochair agus na modhnóirí Super, Control nó Alt á mbrú
    .all = Gach eochair
    .mouse = Imeachtaí cnaipe luiche in fheidhmchláir X11

legacy-app-scaling = Scálú Feidhmchláir Chóras Fuinneoige X11
    .scaled-gaming = Optamaigh le haghaidh cearrbhachais agus feidhmchláir lánscáileáin
    .gaming-description = D’fhéadfadh feidhmchláir X11 a bheith beagán níos mó/níos lú i gcomparáid le feidhmchláir Wayland.
    .scaled-applications = Optamaigh le haghaidh feidhmchláir
    .applications-description = B’fhéidir nach mbeidh cluichí agus feidhmchláir lán-scáileáin X11 ag teacht le réiteach do thaispeána.
    .scaled-compatibility = Mód comhoiriúnachta uasta
    .compatibility-description = D’fhéadfadh feidhmchláir X11 a bheith doiléir ar scáileáin HiDPI.
    .preferred-display = Taispeántas is fearr le haghaidh cluichí agus feidhmchláir X11 lánscáileáin
    .no-display = Gan aon cheann

## System

system = Córas & Cuntais

## System: About

about = Maidir
    .desc = Ainm gléis, faisnéis crua-earraí, mainneachtainí an chórais oibriúcháin.

about-device = Ainm an ghléis
    .desc = Feictear an t-ainm seo ar ghléasanna líonra nó bluetooth eile.

about-hardware = Crua-earraí
    .model = Múnla crua-earraí
    .memory = Cuimhne
    .processor = Próiseálaí
    .graphics = Grafaicí
    .disk-capacity = Toilleadh Diosca

about-os = Córas Oibriúcháin
    .os = Córas Oibriúcháin
    .os-architecture = Ailtireacht an chóras oibriúcháin
    .desktop-environment = Timpeallacht an deasc
    .windowing-system = Córas fuinneoige

about-related = Socruithe Gaolmhara
    .support = Faigh tacaíocht

## System: Firmware

firmware = Dochtearraí
    .desc = Sonraí maidir le dochtearraí.

## System: Users

users = Úsáideoirí
    .desc = Fíordheimhniú agus cuntais úsáideora.
    .admin = Riarachán
    .standard = Caighdeán
    .profile-add = Roghnaigh íomhá phróifíl

administrator = Riarthóir
    .desc = Is féidir le riarthóirí socruithe do gach úsáideoir a athrú, úsáideoirí eile a chur leis agus iad a bhaint.

add-user = Cuir úsáideoir leis
change-password = Change password
remove-user = Bain úsáideoir
full-name = Ainm iomlán
invalid-username = Ainm úsáideora neamhbhailí.
password-mismatch = Ní mór don phasfhocal agus don dearbhú a bheith mar a chéile.
save = Sábháil
