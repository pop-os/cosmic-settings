name := 'cosmic-settings'
appid := 'com.system76.CosmicSettings'
rootdir := ''
prefix := '/usr'

usrdir := absolute_path(clean(rootdir / prefix))
appdir := usrdir / 'share' / 'applications'
default-schema-target := usrdir / 'share' / 'cosmic'
iconsdir := usrdir / 'share' / 'icons' / 'hicolor'

# Installation target paths
bin-dst := usrdir / 'bin' / name
metainfo-dst := usrdir / 'share' / 'metainfo' / appid + '.metainfo.xml'
polkit-actions-dst := usrdir / 'share' / 'polkit-1' / 'actions'
policy-users-dst := polkit-actions-dst / appid + '.Users.policy'
polkit-rules-dst := usrdir / 'share' / 'polkit-1' / 'rules.d' / 'cosmic-settings.rules'

import 'cargo.just'

# Build recipes
[private]
default: build-release

# Build a debian package locally without a schroot or vendoring
build-deb:
    dpkg-buildpackage -d -nc

# Install everything
install:
    install -Dm0644 {{'resources' / appid + '.metainfo.xml'}} {{metainfo-dst}}
    install -Dm0644 {{'resources' / 'polkit-1' / 'rules.d' / 'cosmic-settings.rules'}} {{polkit-rules-dst}}
    install -Dm0644 {{'resources' / 'polkit-1' / 'actions' / appid + '.Users.policy'}} {{policy-users-dst}}
    install -Dm0755 {{cargo-target-dir / 'release' / name}} {{bin-dst}}
    cd resources/applications && find * -type f -exec install -Dm0644 '{}' '{{appdir}}/{}' \;
    cd resources/default_schema && find * -type f -exec install -Dm0644 '{}' '{{default-schema-target}}/{}' \;
    cd resources/icons && find * -type f -exec install -Dm0644 '{}' '{{iconsdir}}/{}' \;

# Uninstalls everything (requires same arguments as given to install)
uninstall:
    rm {{bin-dst}} {{metainfo-dst}} {{polkit-rules-dst}} {{policy-users-dst}}
    cd resources/applications && find * -type f -exec rm '{{appdir}}/{}' \;
    cd resources/default_schema && find * -type f -exec rm '{{default-schema-target}}/{}' \;
    cd resources/icons && find * -type f -exec rm '{{iconsdir}}/{}' \;

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

# Bump cargo version, create git commit, and create tag
tag version:
    find -type f -name Cargo.toml -exec sed -i '0,/^version/s/^version.*/version = "{{version}}"/' '{}' \; -exec git add '{}' \;
    cargo check
    cargo clean
    dch -D noble -v {{version}}
    git add Cargo.lock debian/changelog
    git commit -m 'release: {{version}}'
    git commit --amend
    git tag -a {{version}} -m ''
