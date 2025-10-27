app = COSMIC Instellings

dbus-connection-error = Koppeling aan DBus het misluk
ok = OK
unknown = Onbekend

number = { $number }

## Network & Wireless

connections-and-profiles = { $variant ->
    [wired] Kabelverbindings
    [wifi] Wifi-verbindings
    [vpn] VPN-verbindings
    *[other] Onbekende verbindings
} en verbindingsprofiele.

add-network = Voeg netwerk toe
    .profile = Voeg profiel toe
add-vpn = Voeg VPN toe
airplane-on = Vliegtuigmodus is aan.
cable-unplugged = Kabel is losgekoppel
connect = Verbind
connected = Gekoppel
connecting = Verbind, wag…
disconnect = Beëindig verbinding
forget = Vergeet
known-networks = Bekende netwerke
network-and-wireless = Netwerk en wifi
no-networks = Geen netwerke gevind nie.
no-vpn = Geen VPN-verbindings beskikbaar nie.
password = Wagwoord
password-confirm = Bevestig wagwoord
remove = Verwyder
settings = Instellings
username = Gebruikersnaam
visible-networks = Sigbare netwerke
identity = Identiteit

auth-dialog = Verifikasie vereis
    .vpn-description = Voer die gebruikersnaam en wagwoord in wat deur die VPN-diens vereis word.
    .wifi-description = Voer die wagwoord of enkripsiesleutel in. U kan ook verbind deur op die “WPS”-knoppie op die roeteerder te druk.

forget-dialog = Vergeet hierdie wifi-netwerk?
    .description = U sal weer 'n wagwoord moet invoer as u hierdie wifi-netwerk in die toekoms gebruik wil.

network-device-state =
    .activated = Gekoppel
    .config = Verbind, wag…
    .deactivating = Verbinding beëindig…
    .disconnected = Het ontkoppel
    .failed = Kon nie verbind nie
    .ip-check = Verbinding nagaan…
    .ip-config =  IP- en roeteringsinligting opvra…
    .need-auth = Dit benodig verifikasie
    .prepare = Verbinding word voorberei…
    .secondaries = Wag op sekondêre verbinding…
    .unavailable = Nie beskikbaar nie
    .unknown = Status onbekend
    .unmanaged = Onbeheerd
    .unplugged = Kabel is losgekoppel

remove-connection-dialog = Hierdie verbindingsprofiel verwyder?
    .vpn-description = U sal weer 'n wagwoord moet invoer as u hierdie netwerk in die toekoms gaan gebruik.
    .wired-description = U sal hierdie profiel weer moet skep om dit in die toekoms te kan gebruik.

vpn = VPN
    .connections = VPN-verbindings
    .error = Kon nie VPN-konfigurasie byvoeg nie
    .remove = Verwyder verbindingsprofiel
    .select-file = Kies 'n VPN-konfigurasielêer
    
vpn-error = VPN-fout
    .config = Kon nie VPN-konfigurasie byvoeg nie
    .connect = Kon nie aan VPN koppel nie
    .connection-editor = Die verbindingsprofielkonfigurasie het misluk
    .connection-settings = Kon nie instellings vir aktiewe verbindings kry nie
    .updating-state = Kon netwerkbestuurderstoestand nie opdateer nie
    .wireguard-config-path = Ongeldige lêerpad vir WireGuard-konfigurasie
    .wireguard-config-path-desc = Gekose lêer moet op 'n plaaslike lêerstelsel wees.
    .wireguard-device = Die skepping van WireGuard-toestel het misluk
    .with-password = Kon nie { $field ->
        *[username] VPN-gebruikersnaam
        [password] VPN-wagwoord
        [password-flags] VPN-wagwoordvlae
    } met nmcli stel nie
    
wired = Bedraad
    .adapter = Bedrade adapter { $id }
    .connections = Bedrade verbindings
    .devices = Bedrade toestelle
    .remove = Verwyder verbindingsprofiel
    
wifi = Wifi
    .adapter = Wifi-adapter { $id }
    .forget = Vergeet hierdie netwerk
    
wireguard-dialog = Voeg WireGuard-toestel toe
    .description = Kies 'n toestelnaam vir die WireGuard-konfigurasie.

## Networking: Online Accounts

online-accounts = Aanlyn rekeninge
    .desc = Om aanlyn rekeninge toe te voeg, IMAP en SMTP, en ondernemingsaanmeldings

# Bluetooth

activate = Aktiveer
confirm = Bevestig
enable = Skakel in

bluetooth = Bluetooth
    .desc = Om bluetooth-apparate te bestuur
    .status = Hierdie stelsel is sigbaar as { $aliases } wanneer u die bluetooth-instellings open hou. 
    .connected = Gekoppel
    .connecting = Verbinding maak…
    .disconnecting = Verbinding beëindig…
    .connect = Verbind
    .disconnect = Ontkoppel
    .forget = Vergeet
    .dbus-error = 'n Fout het voorgekom tydens interaksie met DBus: { $why }
    .disabled = Die bluetooth-diens is gedeaktiveer
    .inactive = Die bluetooth-diens is nie aktief nie
    .unknown = Die Bluetooth-diens kon nie geaktiveer word nie. Het u BlueZ geïnstalleer?

bluetooth-paired = Voorheen gekoppelde apparate
    .connect = Verbind
    .battery = { $percentage }% battery

bluetooth-confirm-pin = Bevestig bluetooth-pinkode
    .description = Bevestig dat die volgende pinkode ooreenstem met dié wat op { $device } vertoon word

bluetooth-available = Apparate in die omgewing

bluetooth-adapters = Bluetooth-adapters

## Accessibility

accessibility = Toeganklikheid
    .vision = Visie
    .on = Aan
    .off = Af
    .unavailable = Nie beskikbaar nie
    .screen-reader = Skermleser
    .high-contrast = Hoë kontrastmodus
    .invert-colors = Keer kleure om
    .color-filters = Kleurfilters

hearing = Gehoor
    .mono = Speel stereo-klank as mono

default = Standaard
magnifier = Vergrootglas
    .controls = Of gebruik hierdie kortpaaie: { $zoom_in ->
             [zero] {""}
            *[other] {""}
                {$zoom_in} om in te zoem,
        }{ $zoom_out ->
             [zero] {""}
            *[other] {""}
                {$zoom_out} om uit te zoem,
        }
        Super + Scroll met uw muis
    .scroll_controls = Aktiveer muis- of raakvlakzoem met Super + Scroll
    .show_overlay = Toon die vergrootglasbedienings
    .increment = Gestapte zoemverhoging
    .signin = Laat die vergrootglas begin by aanmelding
    .applet = Skakel die vergrootglas aan/af in die applet op die paneel.
    .movement = Die vergrote aansig beweeg
    .continuous = Deurlopend met die wyser
    .onedge = Wanneer die wyser die rand bereik
    .centered = Om die wyser in die middel to hou
color-filter = Kleurfiltertipe
    .unknown = Onbekende filter aktief
    .greyscale = Grysskaal
    .deuteranopia = Groen/Rooi (groenswakheid, Deuteranopie)
    .protanopia = Rooi/Groen (rooiswakheid, Protanopie)
    .tritanopia = Blou/Geel (bluoswakheid, Tritanopie)


