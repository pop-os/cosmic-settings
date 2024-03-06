import 'scripts/cargo.just'
import 'scripts/cosmic.just'

name := 'cosmic-settings'
appid := 'com.system76.CosmicSettings'

default-schema-target := clean(rootdir / prefix) / 'share' / 'cosmic'

# File paths
bin-src := cargo-target-dir / 'release' / name
bin-dest := clean(rootdir / prefix) / 'bin' / name

desktop := appid + '.desktop'
desktop-src := 'resources' / desktop
desktop-dest := clean(rootdir / prefix) / 'share' / 'applications' / desktop

iconsdir := clean(rootdir / prefix) / 'share' / 'icons' / 'hicolor'

metainfo := appid + '.metainfo.xml'
metainfo-src := 'resources' / metainfo
metainfo-dst := clean(rootdir / prefix) / 'share' / 'metainfo' / metainfo

[private]
default: build-release

# Install everything
install: (install-bin bin-src bin-dest) (install-file desktop-src desktop-dest) (install-file metainfo-src metainfo-dst)
    find 'resources'/'default_schema' -type f -exec echo {} \; | rev | cut -d'/' -f-3 | rev | xargs -d '\n' -I {} install -Dm0644 'resources'/'default_schema'/{} {{default-schema-target}}/{}
    find 'resources'/'icons' -type f -exec echo {} \; | rev | cut -d'/' -f-3 | rev | xargs -d '\n' -I {} install -Dm0644 'resources'/'icons'/{} {{iconsdir}}/{}

# Uninstalls everything (requires same arguments as given to install)
uninstall:
    rm -rf {{bin-dest}} {{desktop-dest}}
    find 'resources'/'default_schema' -type f -exec echo {} \; | rev | cut -d'/' -f-3 | rev | xargs -d '\n' -I {} rm -rf {{default-schema-target}}/{}
    find 'resources'/'icons' -type f -exec echo {} \; | rev | cut -d'/' -f-3 | rev | xargs -d '\n' -I {} rm {{iconsdir}}/{}

# Vendor Cargo dependencies locally
vendor:
    mkdir -p .cargo
    cargo vendor --sync Cargo.toml \
        | head -n -1 > .cargo/config
    echo 'directory = "vendor"' >> .cargo/config
    tar pcf vendor.tar vendor
    rm -rf vendor
