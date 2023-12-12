name := 'cosmic-settings'
appid := 'com.system76.CosmicSettings'

# Use mold linker if clang and mold exists.
clang-path := `which clang || true`
mold-path := `which mold || true`

linker-arg := if clang-path != '' {
    if mold-path != '' {
        '-C linker=' + clang-path + ' -C link-arg=--ld-path=' + mold-path + ' '
    } else {
        ''
    }
} else {
    ''
}

export RUSTFLAGS := linker-arg + env_var_or_default('RUSTFLAGS', '')

rootdir := ''
prefix := '/usr'

default-schema-target := clean(rootdir / prefix) / 'share' / 'cosmic'

# File paths
bin-src := 'target' / 'release' / name
bin-dest := clean(rootdir / prefix) / 'bin' / name

desktop := appid + '.desktop'
desktop-src := 'resources' / desktop
desktop-dest := clean(rootdir / prefix) / 'share' / 'applications' / desktop

iconsdir := clean(rootdir / prefix) / 'share' / 'icons' / 'hicolor'

[private]
default: build-release

# Remove Cargo build artifacts
clean:
    cargo clean

# Also remove .cargo and vendored dependencies
clean-dist: clean
    rm -rf .cargo vendor vendor.tar target

# Compile with debug profile
build-debug *args:
    cargo build {{args}}

# Compile with release profile
build-release *args: (build-debug '--release' args)

# Compile with a vendored tarball
build-vendored *args: vendor-extract (build-release '--frozen --offline' args)

# Check for errors and linter warnings
check *args:
    cargo clippy --all-features {{args}} -- -W clippy::pedantic

# Runs a check with JSON message format for IDE integration
check-json: (check '--message-format=json')

# Installation command
[private]
install-cmd options src dest:
    install {{options}} {{src}} {{dest}}

[private]
install-bin src dest: (install-cmd '-Dm0755' src dest)

[private]
install-file src dest: (install-cmd '-Dm0644' src dest)

# Install everything
install: (install-bin bin-src bin-dest) (install-file desktop-src desktop-dest)
    find 'resources'/'default_schema' -type f -exec echo {} \; | rev | cut -d'/' -f-3 | rev | xargs -d '\n' -I {} install -Dm0644 'resources'/'default_schema'/{} {{default-schema-target}}/{}
    find 'resources'/'icons' -type f -exec echo {} \; | rev | cut -d'/' -f-3 | rev | xargs -d '\n' -I {} install -Dm0644 'resources'/'icons'/{} {{iconsdir}}/{}

# Run the application for testing purposes
run *args:
    env RUST_LOG=debug RUST_BACKTRACE=full cargo run --release {{args}}

# Run `cargo test`
test:
    cargo test

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

# Extracts vendored dependencies
[private]
vendor-extract:
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
