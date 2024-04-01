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

# Desktop entries
entry-settings := appid + '.desktop'
entry-about := appid + '.About.desktop'
entry-appear := appid + '.Appearance.desktop'
entry-date-time := appid + '.DateTime.desktop'
entry-desktop := appid + '.Desktop.desktop'
entry-displays := appid + '.Displays.desktop'
entry-firmware := appid + '.Firmware.desktop'
entry-keyboard := appid + '.Keyboard.desktop'
entry-mouse := appid + '.Mouse.desktop'
entry-notifications := appid + '.Notifications.desktop'
entry-region-language := appid + '.RegionLanguage.desktop'
entry-sound := appid + '.Sound.desktop'
entry-touchpad := appid + '.Touchpad.desktop'
entry-users := appid + '.Users.desktop'
entry-wallpaper := appid + '.Wallpaper.desktop'
entry-workspaces := appid + '.Workspaces.desktop'

# Build recipes
[private]
default: build-release

[private]
install-desktop-entries:
    install -Dm0644 'resources/{{entry-settings}}' '{{appdir}}/{{entry-settings}}'
    install -Dm0644 'resources/{{entry-about}}' '{{appdir}}/{{entry-about}}'
    install -Dm0644 'resources/{{entry-appear}}' '{{appdir}}/{{entry-appear}}'
    install -Dm0644 'resources/{{entry-date-time}}' '{{appdir}}/{{entry-date-time}}'
    install -Dm0644 'resources/{{entry-desktop}}' '{{appdir}}/{{entry-desktop}}'
    install -Dm0644 'resources/{{entry-displays}}' '{{appdir}}/{{entry-displays}}'
    install -Dm0644 'resources/{{entry-firmware}}' '{{appdir}}/{{entry-firmware}}'
    install -Dm0644 'resources/{{entry-keyboard}}' '{{appdir}}/{{entry-keyboard}}'
    install -Dm0644 'resources/{{entry-mouse}}' '{{appdir}}/{{entry-mouse}}'
    install -Dm0644 'resources/{{entry-notifications}}' '{{appdir}}/{{entry-notifications}}'
    install -Dm0644 'resources/{{entry-region-language}}' '{{appdir}}/{{entry-region-language}}'
    install -Dm0644 'resources/{{entry-sound}}' '{{appdir}}/{{entry-sound}}'
    install -Dm0644 'resources/{{entry-touchpad}}' '{{appdir}}/{{entry-touchpad}}'
    install -Dm0644 'resources/{{entry-users}}' '{{appdir}}/{{entry-users}}'
    install -Dm0644 'resources/{{entry-wallpaper}}' '{{appdir}}/{{entry-wallpaper}}'
    install -Dm0644 'resources/{{entry-workspaces}}' '{{appdir}}/{{entry-workspaces}}'

# Install everything
install: install-desktop-entries (install-bin bin-src bin-dest) (install-file metainfo-src metainfo-dst)
    find 'resources'/'default_schema' -type f -exec echo {} \; | rev | cut -d'/' -f-3 | rev | xargs -d '\n' -I {} install -Dm0644 'resources'/'default_schema'/{} {{default-schema-target}}/{}
    find 'resources'/'icons' -type f -exec echo {} \; | rev | cut -d'/' -f-3 | rev | xargs -d '\n' -I {} install -Dm0644 'resources'/'icons'/{} {{iconsdir}}/{}

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
        '{{appdir}}/{{entry-appear}}' \
        '{{appdir}}/{{entry-date-time}}' \
        '{{appdir}}/{{entry-desktop}}' \
        '{{appdir}}/{{entry-displays}}' \
        '{{appdir}}/{{entry-firmware}}' \
        '{{appdir}}/{{entry-keyboard}}' \
        '{{appdir}}/{{entry-mouse}}' \
        '{{appdir}}/{{entry-notifications}}' \
        '{{appdir}}/{{entry-region-language}}' \
        '{{appdir}}/{{entry-sound}}' \
        '{{appdir}}/{{entry-touchpad}}' \
        '{{appdir}}/{{entry-users}}' \
        '{{appdir}}/{{entry-wallpaper}}' \
        '{{appdir}}/{{entry-workspaces}}'
    find 'resources'/'default_schema' -type f -exec echo {} \; | rev | cut -d'/' -f-3 | rev | xargs -d '\n' -I {} rm -rf {{default-schema-target}}/{}
    find 'resources'/'icons' -type f -exec echo {} \; | rev | cut -d'/' -f-3 | rev | xargs -d '\n' -I {} rm {{iconsdir}}/{}

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