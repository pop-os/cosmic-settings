name := 'cosmic-settings'
appid := 'com.system76.CosmicSettings'

# Use the x86-64-v2 target by default on x86-64 systems.
target-cpu := if arch() == 'x86_64' { 'x86-64-v2' } else { '' }
cpu-arg := if target-cpu != '' { '-C target-cpu=' + target-cpu + ' ' } else { '' }

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

export RUSTFLAGS := cpu-arg + linker-arg + env_var_or_default('RUSTFLAGS', '')

rootdir := ''
prefix := '/usr'

# File paths
bin-src := 'target' / 'release' / name
bin-dest := clean(rootdir / prefix) / 'bin' / name

desktop := appid + '.desktop'
desktop-src := 'resources' / desktop
desktop-dest := clean(rootdir / prefix) / 'share' / 'applications' / desktop

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
build-vendored *args: vendor-extract (build-release '--offline --locked' args)

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

# Run the application for testing purposes
run *args:
    env RUST_LOG=debug RUST_BACKTRACE=full cargo run --release {{args}}

# Run `cargo test`
test:
    cargo test

# Uninstalls everything (requires same arguments as given to install)
uninstall:
    rm -rf {{bin-dest}} {{desktop-dest}}

# Vendor Cargo dependencies locally
vendor:
    rm -rf .cargo vendor vendor.tar
    mkdir -p .cargo
    cargo vendor | head -n -1 > .cargo/config
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
