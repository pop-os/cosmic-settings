import "common.just"

# Return an error if the system does not contain the minimum required Rust version.
min-rust-version := '>=' + `grep rust-version Cargo.toml | cut -d' ' -f3 | cut -c 2-7`
rust-version-check := if semver_matches(`rustc --version | cut -c 7-12`, min-rust-version) == "false" {
    error('Requires rustc version ' + min-rust-version)
} else {
    "passed"
}

cargo-target-dir := env('CARGO_TARGET_DIR', 'target')

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

# Remove Cargo build artifacts
[no-cd]
clean:
    cargo clean

# Also remove .cargo and vendored dependencies
[no-cd]
clean-dist: clean
    rm -rf .cargo vendor vendor.tar target

# Run the application for testing purposes
run *args:
    env RUST_LOG=debug RUST_BACKTRACE=full cargo run {{args}}

# Run `cargo test`
test *args:
    cargo test {{args}}

# Vendor Cargo dependencies locally
[no-cd]
vendor:
    mkdir -p .cargo
    cargo vendor | head -n -1 > .cargo/config
    echo 'directory = "vendor"' >> .cargo/config
    tar pcf vendor.tar vendor
    rm -rf vendor

# Extracts vendored dependencies
[no-cd, private]
vendor-extract:
    rm -rf vendor
    tar pxf vendor.tar
