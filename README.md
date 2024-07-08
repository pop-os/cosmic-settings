# COSMIC Settings

The settings application for the [COSMIC desktop environment][cosmic-epoch].

## Translators

Translation files may be found in the [i18n directory](./i18n). New translations may copy the [English (en) localization](./i18n/en) of the project and rename `en` to the desired [ISO 639-1 language code][iso-codes]. Translations may be submitted through GitHub as an issue or pull request. Submissions by email or other means are also acceptable; with the preferred name and email to associate with the changes.

## Build

### Dependencies

- rust (>= 1.71.0)
- [just][just]
- cmake
- libexpat1-dev
- libfontconfig-dev
- libfreetype-dev
- libinput-dev
- pkg-config

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

To improve compilation times, disable LTO in the release profile, install the [mold][mold] linker, and configure [sccache][sccache] for use with Rust. The [mold][mold] linker will only improve link times if LTO is disabled.

Run the cosmic-settings binary with `just run` so that logs will be emitted to stderr, and crashes will generate detailed backtraces. Applications shouldn't crash, so when writing code, avoid use of `unwrap()` and `expect()`. Instead, log errors with `tracing::error!()` or `tracing::warn!()`.

## License

Licensed under the [GNU Public License 3.0](https://choosealicense.com/licenses/gpl-3.0).

### Contribution

Any contribution intentionally submitted for inclusion in the work by you shall be licensed under the GNU Public License 3.0 (GPL-3.0). Each source file should have a SPDX copyright notice at the top of the file:

```
// SPDX-License-Identifier: GPL-3.0-only
```

[cosmic-epoch]: https://github.com/pop-os/cosmic-epoch
[iso-codes]: https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes
[just]: https://github.com/casey/just
[rustup]: https://rustup.rs/
[rust-analyzer]: https://rust-analyzer.github.io/
[mold]: https://github.com/rui314/mold
[sccache]: https://github.com/mozilla/sccache
