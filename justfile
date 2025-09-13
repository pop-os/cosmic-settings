import 'scripts/cargo.just'

# Project metadata
name := 'cosmic-settings'
appid := 'com.system76.CosmicSettings'

# File paths
rootdir := ''
prefix := '/usr'

appdir := clean(rootdir / prefix) / 'share' / 'applications'
default-schema-target := clean(rootdir / prefix) / 'share' / 'cosmic'

bin-src := cargo-target-dir / 'release' / name
bin-dest := clean(rootdir / prefix) / 'bin' / name

iconsdir := clean(rootdir / prefix) / 'share' / 'icons' / 'hicolor'

metainfo := appid + '.metainfo.xml'
metainfo-src := 'resources' / metainfo
metainfo-dst := clean(rootdir / prefix) / 'share' / 'metainfo' / metainfo

polkit-actions-src := 'resources' / 'polkit-1' / 'actions'
polkit-actions-dst := clean(rootdir / prefix) / 'share' / 'polkit-1' / 'actions'

policy-users-src := polkit-actions-src / appid + '.Users.policy'
policy-users-dst := polkit-actions-dst / appid + '.Users.policy'

polkit-rules-src := 'resources' / 'polkit-1' / 'rules.d' / 'cosmic-settings.rules'
polkit-rules-dst := clean(rootdir / prefix) / 'share' / 'polkit-1' / 'rules.d' / 'cosmic-settings.rules'

# Desktop entries
entry-settings := appid + '.desktop'
entry-about := appid + '.About.desktop'
entry-accessibility := appid + '.Accessibility.desktop'
entry-appear := appid + '.Appearance.desktop'
entry-apps := appid + '.Applications.desktop'
entry-bluetooth := appid + '.Bluetooth.desktop'
entry-date-time := appid + '.DateTime.desktop'
entry-default-apps := appid + '.DefaultApps.desktop'
entry-desktop := appid + '.Desktop.desktop'
entry-displays := appid + '.Displays.desktop'
entry-dock := appid + '.Dock.desktop'
entry-firmware := appid + '.Firmware.desktop'
entry-input := appid + '.Input.desktop'
entry-keyboard := appid + '.Keyboard.desktop'
entry-legacy-apps := appid + '.LegacyApplications.desktop'
entry-mouse := appid + '.Mouse.desktop'
entry-network := appid + '.Network.desktop'
entry-notifications := appid + '.Notifications.desktop'
entry-panel := appid + '.Panel.desktop'
entry-power := appid + '.Power.desktop'
entry-region-language := appid + '.RegionLanguage.desktop'
entry-sound := appid + '.Sound.desktop'
entry-startup-apps := appid + '.StartupApps.desktop'
entry-system := appid + '.System.desktop'
entry-time := appid + '.Time.desktop'
entry-touchpad := appid + '.Touchpad.desktop'
entry-users := appid + '.Users.desktop'
entry-vpn := appid + '.Vpn.desktop'
entry-wallpaper := appid + '.Wallpaper.desktop'
entry-window-management := appid + '.WindowManagement.desktop'
entry-wired := appid + '.Wired.desktop'
entry-wireless := appid + '.Wireless.desktop'
entry-workspaces := appid + '.Workspaces.desktop'

# Build recipes
[private]
default: build-release

[private]
install-desktop-entries:
    install -Dm0644 'resources/{{entry-settings}}' '{{appdir}}/{{entry-settings}}'
    install -Dm0644 'resources/{{entry-about}}' '{{appdir}}/{{entry-about}}'
    install -Dm0644 'resources/{{entry-accessibility}}' '{{appdir}}/{{entry-accessibility}}'
    install -Dm0644 'resources/{{entry-appear}}' '{{appdir}}/{{entry-appear}}'
    install -Dm0644 'resources/{{entry-apps}}' '{{appdir}}/{{entry-apps}}'
    install -Dm0644 'resources/{{entry-bluetooth}}' '{{appdir}}/{{entry-bluetooth}}'
    install -Dm0644 'resources/{{entry-date-time}}' '{{appdir}}/{{entry-date-time}}'
    install -Dm0644 'resources/{{entry-default-apps}}' '{{appdir}}/{{entry-default-apps}}'
    install -Dm0644 'resources/{{entry-desktop}}' '{{appdir}}/{{entry-desktop}}'
    install -Dm0644 'resources/{{entry-displays}}' '{{appdir}}/{{entry-displays}}'
    install -Dm0644 'resources/{{entry-dock}}' '{{appdir}}/{{entry-dock}}'
    install -Dm0644 'resources/{{entry-firmware}}' '{{appdir}}/{{entry-firmware}}'
    install -Dm0644 'resources/{{entry-input}}' '{{appdir}}/{{entry-input}}'
    install -Dm0644 'resources/{{entry-keyboard}}' '{{appdir}}/{{entry-keyboard}}'
    install -Dm0644 'resources/{{entry-legacy-apps}}' '{{appdir}}/{{entry-legacy-apps}}'
    install -Dm0644 'resources/{{entry-mouse}}' '{{appdir}}/{{entry-mouse}}'
    install -Dm0644 'resources/{{entry-network}}' '{{appdir}}/{{entry-network}}'
    install -Dm0644 'resources/{{entry-notifications}}' '{{appdir}}/{{entry-notifications}}'
    install -Dm0644 'resources/{{entry-panel}}' '{{appdir}}/{{entry-panel}}'
    install -Dm0644 'resources/{{entry-power}}' '{{appdir}}/{{entry-power}}'
    install -Dm0644 'resources/{{entry-region-language}}' '{{appdir}}/{{entry-region-language}}'
    install -Dm0644 'resources/{{entry-sound}}' '{{appdir}}/{{entry-sound}}'
    install -Dm0644 'resources/{{entry-startup-apps}}' '{{appdir}}/{{entry-startup-apps}}'
    install -Dm0644 'resources/{{entry-system}}' '{{appdir}}/{{entry-system}}'
    install -Dm0644 'resources/{{entry-time}}' '{{appdir}}/{{entry-time}}'
    install -Dm0644 'resources/{{entry-touchpad}}' '{{appdir}}/{{entry-touchpad}}'
    install -Dm0644 'resources/{{entry-users}}' '{{appdir}}/{{entry-users}}'
    install -Dm0644 'resources/{{entry-vpn}}' '{{appdir}}/{{entry-vpn}}'
    install -Dm0644 'resources/{{entry-wallpaper}}' '{{appdir}}/{{entry-wallpaper}}'
    install -Dm0644 'resources/{{entry-window-management}}' '{{appdir}}/{{entry-window-management}}'
    install -Dm0644 'resources/{{entry-wired}}' '{{appdir}}/{{entry-wired}}'
    install -Dm0644 'resources/{{entry-wireless}}' '{{appdir}}/{{entry-wireless}}'
    install -Dm0644 'resources/{{entry-workspaces}}' '{{appdir}}/{{entry-workspaces}}'

# Install everything
install: install-desktop-entries (install-bin bin-src bin-dest) (install-file metainfo-src metainfo-dst) install-polkit-files
    find 'resources'/'default_schema' -type f -exec echo {} \; | rev | cut -d'/' -f-3 | rev | xargs -d '\n' -I {} install -Dm0644 'resources'/'default_schema'/{} {{default-schema-target}}/{}
    find 'resources'/'icons' -type f -exec echo {} \; | rev | cut -d'/' -f-3 | rev | xargs -d '\n' -I {} install -Dm0644 'resources'/'icons'/{} {{iconsdir}}/{}

install-polkit-files: (install-file polkit-rules-src polkit-rules-dst) (install-file policy-users-src policy-users-dst)

[private]
install-cmd options src dest:
    install {{options}} {{src}} {{dest}}

[private]
install-bin src dest: (install-cmd '-Dm0755' src dest)

[private]
install-file src dest: (install-cmd '-Dm0644' src dest)

# Uninstalls everything (requires same arguments as given to install)
uninstall:
    rm -rf {{bin-dest}} \
        '{{appdir}}/{{entry-settings}}' \
        '{{appdir}}/{{entry-about}}' \
        '{{appdir}}/{{entry-accessibility}}' \
        '{{appdir}}/{{entry-appear}}' \
        '{{appdir}}/{{entry-apps}}' \
        '{{appdir}}/{{entry-bluetooth}}' \
        '{{appdir}}/{{entry-date-time}}' \
        '{{appdir}}/{{entry-default-apps}}' \
        '{{appdir}}/{{entry-desktop}}' \
        '{{appdir}}/{{entry-displays}}' \
        '{{appdir}}/{{entry-dock}}' \
        '{{appdir}}/{{entry-firmware}}' \
        '{{appdir}}/{{entry-input}}' \
        '{{appdir}}/{{entry-keyboard}}' \
        '{{appdir}}/{{entry-legacy-apps}}' \
        '{{appdir}}/{{entry-mouse}}' \
        '{{appdir}}/{{entry-network}}' \
        '{{appdir}}/{{entry-notifications}}' \
        '{{appdir}}/{{entry-panel}}' \
        '{{appdir}}/{{entry-power}}' \
        '{{appdir}}/{{entry-region-language}}' \
        '{{appdir}}/{{entry-sound}}' \
        '{{appdir}}/{{entry-startup-apps}}' \
        '{{appdir}}/{{entry-system}}' \
        '{{appdir}}/{{entry-time}}' \
        '{{appdir}}/{{entry-touchpad}}' \
        '{{appdir}}/{{entry-users}}' \
        '{{appdir}}/{{entry-vpn}}' \
        '{{appdir}}/{{entry-wallpaper}}' \
        '{{appdir}}/{{entry-window-management}}' \
        '{{appdir}}/{{entry-wired}}' \
        '{{appdir}}/{{entry-wireless}}' \
        '{{appdir}}/{{entry-workspaces}}'
    find 'resources'/'default_schema' -type f -exec echo {} \; | rev | cut -d'/' -f-3 | rev | xargs -d '\n' -I {} rm -rf {{default-schema-target}}/{}
    find 'resources'/'icons' -type f -exec echo {} \; | rev | cut -d'/' -f-3 | rev | xargs -d '\n' -I {} rm {{iconsdir}}/{}

heaptrack *args:
    #!/usr/bin/env bash
    set -ex
    rm -fv heaptrack.cosmic-settings.*
    cargo heaptrack --profile release-with-debug --bin cosmic-settings -- {{args}}
    zstd -dc < heaptrack.cosmic-settings.*.raw.zst + /usr/lib/heaptrack/libexec/heaptrack_env | zstd -c > heaptrack_env.cosmic-settings.zst
    heaptrack_gui heaptrack.cosmic-settings.zst

check-features:
    #!/usr/bin/env bash
    set -ex
    cargo check --no-default-features
    cargo check
    for feature in \
        "page-accessibility" \
        "page-about" \
        "page-bluetooth" \
        "page-date" \
        "page-default-apps" \
        "page-display" \
        "page-input" \
        "page-legacy-applications" \
        "page-networking" \
        "page-power" \
        "page-region" \
        "page-sound" \
        "page-users" \
        "page-window-management" \
        "page-workspaces"
    do
        cargo check --no-default-features --features "${feature}"
    done

# Dependencies
cmd-depends := "
cargo
cmake
"

lib-depends := "
expat
fontconfig
freetype2
libinput
libudev
wayland-client=1.20
xkbcommon
"
