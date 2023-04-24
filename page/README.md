# cosmic-settings-page

This module contains the APIs for creating and managing settings pages.

- A [Page](./src/lib.rs) implements the `Page` and `AutoBind` traits.
- A [Section](./src/section.rs) is a subset of a page, with a view function to generate the UI.
- The [Binder](./src/binder.rs) holds all of the pages, their sections, and additional metadata associated with them
