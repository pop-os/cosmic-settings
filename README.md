# COSMIC Settings

> Prototype of a proof of concept that is an active work in progress.

The settings application for the [COSMIC desktop environment](https://github.com/pop-os/cosmic-epoch). Developed with [libcosmic](https://github.com/pop-os/libcosmic) in the [iced](https://iced.rs/) GUI library.

## Build

To compile, a stable Rust compiler and [just](https://github.com/casey/just) are required.

- cargo
- just
- lld

Some C libraries are also required for font support at the moment.

- cmake
- libexpat1-dev
- libfontconfig-dev
- libfreetype-dev
- pkg-config

Then it can be compiled and installed like so.

```sh
just build-release
sudo just prefix=/usr install
```

If you are packaging for Linux distribution, you can use the `rootdir` variable to change the root path, in addition to the prefix.

```sh
just rootdir=debian/cosmic-settings prefix=/usr install
```

## Translators

Translation files may be found in the [i18n directory](./i18n). New translations may copy the [English (en) localization](./i18n/en) of the project and rename `en` to the desired [ISO 639-1 language code](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes). Translations may be submitted through GitHub as an issue or pull request. Submissions by email or other means are also acceptable; with the preferred name and email to associate with the changes.

## License

Licensed under the [GNU Public License 3.0](https://choosealicense.com/licenses/gpl-3.0).

### Contribution

Any contribution intentionally submitted for inclusion in the work by you shall be licensed under the GNU Public License 3.0 (GPL-3.0). Each source file should have a SPDX copyright notice at the top of the file:

```
// SPDX-License-Identifier: GPL-3.0-only
```
