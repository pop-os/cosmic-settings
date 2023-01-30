name := 'cosmic-settings'
appid := 'com.system76.CosmicSettings'

arch := `uname -m`

# Choose x86-64-v2, v3, or v4 for packaging
x86-64-target := 'native'

export RUSTFLAGS := if arch == 'x86_64' {
    '-C target-cpu=' + x86-64-target
} else {
    ''
}

rootdir := ''
prefix := '/usr'

# File paths
bin-src := 'target/release/' + name
bin-dest := rootdir + prefix + '/bin/' + name

desktop := appid + '.desktop'
desktop-src := 'resources/' + desktop
desktop-dest := rootdir + prefix + '/share/applications/' + desktop

help:
    @echo '{{name}} ({{appid}})'
    @echo 'RUSTFLAGS={{RUSTFLAGS}}'
    @echo 'prefix={{prefix}}'
    @just -l

# Remove Cargo build artifacts
clean:
    cargo clean

# Also remove .cargo and vendored dependencies
clean-dist: clean
    rm -rf .cargo vendor vendor.tar target

# Run the application for testing purposes
run *args:
    cargo run {{args}}

# Compile debug build of cosmic-settings
build-debug *args:
    cargo build {{args}}

# Compile release build of cosmic-settings
build-release *args: (build-debug '--release' args)

# Vendored release build of cosmic-settings
build-vendored *args: _vendor-extract (build-release '--frozen --offline' args)

# Run `cargo clippy`
check *args:
    cargo clippy --all-features {{args}} -- -W clippy::pedantic

# Run `cargo clippy` with json message format
check-json: (check '--message-format=json')

# Run `cargo test`
test:
    cargo test --all-features

# Installation command
_install options src dest:
    install {{options}} {{src}} {{dest}}

_install-bin src dest: (_install '-Dm0755' src dest)
_install-file src dest: (_install '-Dm0644' src dest)

# Install everything
install: (_install-bin bin-src bin-dest) (_install-file desktop-src desktop-dest)

# Uninstalls everything (requires same arguments as given to install)
uninstall:
    rm -rf {{bin-dest}} {{desktop-dest}}

# Vendor Cargo dependencies locally
vendor:
    mkdir -p .cargo
    cargo vendor --sync Cargo.toml \
        | head -n -1 > .cargo/config
    echo 'directory = "vendor"' >> .cargo/config
    tar pcf vendor.tar vendor
    rm -rf vendor

# Extracts vendored dependencies if vendor=1
_vendor-extract:
    #!/usr/bin/env sh
    rm -rf vendor
    tar pxf vendor.tar

# Show the name of the project
name:
    @cargo pkgid | sed -e 's:.*/::' -e 's:[#^].*::'

# Show the current version
version:
    @cargo pkgid | sed -e 's:.*/::' -e 's:.*#::'

# Show the current git commit
git-rev:
    @git rev-parse --short HEAD
