// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

mod path {
    use std::path::Path;

    #[must_use]

    pub struct Manager {
        base: String,
    }

    impl Manager {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            let mut base = dirs::config_dir()
                .expect("XDG config directory missing")
                .join("cosmic-settings")
                .into_os_string()
                .into_string()
                .expect("XDG config path is not UTF-8");

            base.push('/');

            if !Path::new(&base).exists() {
                let _res = std::fs::create_dir_all(&base);
            }

            Self { base }
        }

        pub fn config<T>(&mut self, page: &str, with: impl Fn(&Path) -> T) -> T {
            let truncate_length = self.base.len();

            self.base.push_str(page);
            self.base.push_str(".config");

            let output = with(Path::new(&self.base));
            self.base.truncate(truncate_length);
            output
        }
    }
}

pub use path::Manager as PathManager;

use bytecheck::CheckBytes;
use rkyv::{ser::Serializer, Archive, Deserialize, Serialize};
use std::{io::Read, path::Path};

#[must_use]
#[repr(C)]
#[derive(Archive, Debug, Deserialize, Serialize)]
#[archive_attr(derive(CheckBytes, Debug))]
pub struct Config {
    pub active_page: Box<str>,
}

impl Config {
    pub fn deserialize(path: &Path) -> Self {
        let Ok(mut file) = std::fs::File::open(path) else {
            return Self::default();
        };

        let mut buffer = Vec::with_capacity(128);
        if file.read_to_end(&mut buffer).is_err() {
            return Self::default();
        }

        buffer.shrink_to_fit();

        let Ok(archived) = rkyv::check_archived_root::<Self>(buffer.as_slice()) else {
            return Self::default();
        };

        archived
            .deserialize(&mut rkyv::Infallible)
            .unwrap_or_default()
    }

    pub fn serialize(&self, path: &Path) {
        let mut serializer = rkyv::ser::serializers::AllocSerializer::<0>::default();

        if serializer.serialize_value(self).is_err() {
            return;
        };

        let bytes = serializer.into_serializer().into_inner();

        let _res = std::fs::write(path, &bytes);
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            active_page: Box::from("desktop"),
        }
    }
}
