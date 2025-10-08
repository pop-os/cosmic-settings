# COSMIC Settings

The settings application for the [COSMIC desktop environment][cosmic-epoch].

## Translators

Translations must go through Weblate at https://hosted.weblate.org/projects/pop-os/cosmic-settings.

## Distributors

We will accept pull requests for distro-specific features and pages.
Make them compile conditionally with a [cargo feature][cargo-feature].

The accent palettes on the Appearance settings page are configurable through the cosmic-config directory at `/usr/share/cosmic/com.system76.CosmicSettings/v1/`. One at `accent_palette_dark`, and another at `accent_palette_light`. Examples can be found at [resources/accent_palette_dark.ron](./resources/accent_palette_dark.ron) and [resources/accent_palette_light.ron](./resources/accent_palette_light.ron). This can be copied locally to `~/.config/cosmic/com.system76.CosmicSettings/v1/` for testing, and then move to `/usr/share/cosmic` for packaging.

## Build

### Dependencies

See the `Build-Depends` section of the [debian control file](./debian/control).

### Install

COSMIC uses [just][just] as its preferred build tool.

```sh
just
sudo just install
```

### Packaging

If packaging for a Linux distribution, vendor dependencies locally with the `vendor` rule, and build with the vendored sources using the `build-vendored` rule. When installing files, use the `rootdir` and `prefix` variables to change installation paths.

```sh
just vendor
just build-vendored
just rootdir=debian/cosmic-settings prefix=/usr install
```

It is recommended to build a source tarball with the vendored dependencies, which can typically be done by running `just vendor` on the host system before it enters the build environment. Reference [debian/rules](./debian/rules) to see how we generate debian packages with `sbuild`.

## Developers

Developers should install [rustup][rustup] and configure their editor to use [rust-analyzer][rust-analyzer]. Run `just check` to ensure that the changes you make are free of linter warnings. You may configure your editor to run `just check-json` as the rust-analyzer check command.

Run the cosmic-settings binary with `just run` so that logs will be emitted to stderr, and crashes will generate detailed backtraces. Applications shouldn't crash, so when writing code, avoid use of `unwrap()` and `expect()`. Instead, log errors with `tracing::error!()` or `tracing::warn!()`.

To improve compilation times, use Rust >= 1.90.0 and configure [sccache][sccache] for use with Rust.

## License

Licensed under the [GNU Public License 3.0](https://choosealicense.com/licenses/gpl-3.0).

### Contribution

Any contribution intentionally submitted for inclusion in the work by you shall be licensed under the GNU Public License 3.0 (GPL-3.0). Each source file should have a SPDX copyright notice at the top of the file:

```
// SPDX-License-Identifier: GPL-3.0-only
```

[cargo-feature]: https://doc.rust-lang.org/cargo/reference/features.html
[cosmic-epoch]: https://github.com/pop-os/cosmic-epoch
[just]: https://github.com/casey/just
[rustup]: https://rustup.rs/
[rust-analyzer]: https://rust-analyzer.github.io/
[sccache]: https://github.com/mozilla/sccache
