# COSMIC Settings

The settings application for the [COSMIC desktop environment](https://github.com/pop-os/cosmic-epoch). Developed with [libcosmic](https://github.com/pop-os/libcosmic), using the [iced](https://iced.rs/) GUI library.

## Build & Install

To compile, a stable Rust compiler and [just](https://github.com/casey/just) are required.

- cargo
- just
- mold

Some C libraries are also required for font support at the moment.

- cmake
- libexpat1-dev
- libfontconfig-dev
- libfreetype-dev
- pkg-config

Then it can be compiled and installed like so.

```sh
just
sudo just install
```

## Packagers

If packaging for a Linux distribution, vendor dependencies locally with the `vendor` rule, and build with the vendored sources using the `build-vendored` rule.

```sh
just vendor
just build-vendored
```

When installing files, use the `rootdir` and `prefix` variables to change installation paths.

```sh
just rootdir=debian/cosmic-settings prefix=/usr install
```

## Translators

Translation files may be found in the [i18n directory](./i18n). New translations may copy the [English (en) localization](./i18n/en) of the project and rename `en` to the desired [ISO 639-1 language code](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes). Translations may be submitted through GitHub as an issue or pull request. Submissions by email or other means are also acceptable; with the preferred name and email to associate with the changes.

## Developers

Run the cosmic-settings binary with `just run` so that logs will be emitted to stderr, and crashes will generate detailed backtraces. Applications shouldn't crash, so when writing code, avoid use of `unwrap()` and `expect()`. Instead, log errors with `tracing::error!()` or `tracing::warn!()`.

This project is split across the following workspace members:

- [app](./app/): cosmic-settings GUI frontend and binary
- [page](./page/): library for creating and handling settings pages
- [pages](./pages/): libraries for page-specific logic

When creating a new page, UI-specific code will go directly into **app**, and page-specific logic will go into a crate under the **pages** directory. This is mainly to isolate page-specific crate dependencies and logic from the UI; so that the source code specific to the UI is easier to maintain and refactor.

Eventually, pages may be separated into plugins, and this will help with that migration.

## License

Licensed under the [GNU Public License 3.0](https://choosealicense.com/licenses/gpl-3.0).

### Contribution

Any contribution intentionally submitted for inclusion in the work by you shall be licensed under the GNU Public License 3.0 (GPL-3.0). Each source file should have a SPDX copyright notice at the top of the file:

```
// SPDX-License-Identifier: GPL-3.0-only
```
