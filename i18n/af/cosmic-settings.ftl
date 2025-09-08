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

add-network = Voeg netwerk by
    .profile = Voeg profiel by
add-vpn = Voeg VPN by
airplane-on = Vliegtuigmodus is aan.
cable-unplugged = Kabel is losgekoppel
connect = Verbind
connected = Gekoppel
connecting = Verbinding maak…
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

forget-dialog = 
    .description = U sal weer 'n wagwoord moet invoer as u hierdie wifi-netwerk in die toekoms gaan gebruik.

network-device-state =
    .activated = Gekoppel
    .config = Verbinding maak…
    .deactivating = Verbinding beëindig…
    .disconnected = Verbinding is beëindig
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
    .wireguard-device = Skep van WireGuard-toestel het misluk
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
    
wireguard-dialog = Voeg WireGuard-toestel by
    .description = Kies 'n toestelnaam vir die WireGuard-konfigurasie.
