// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

use std::collections::HashMap;
use zbus::{OwnedObjectPath, OwnedVariant, Variant};

pub(crate) trait Properties {
    fn value(&self, name: &str) -> Option<&Variant<'_>>;

    fn string(&self, name: &str) -> Option<String> {
        match self.value(name)? {
            Variant::Str(value) => Some(value.to_string()),
            _ => None,
        }
    }

    fn boolean(&self, name: &str) -> Option<bool> {
        match self.value(name)? {
            Variant::Bool(value) => Some(*value),
            _ => None,
        }
    }

    fn byte(&self, name: &str) -> Option<u8> {
        match self.value(name)? {
            Variant::U8(value) => Some(*value),
            _ => None,
        }
    }

    fn object_path(&self, name: &str) -> Option<OwnedObjectPath> {
        match self.value(name)? {
            Variant::ObjectPath(value) => Some(value.clone().into()),
            _ => None,
        }
    }
}

impl Properties for HashMap<String, OwnedVariant> {
    fn value(&self, name: &str) -> Option<&Variant<'_>> {
        self.get(name).map(|value| &**value)
    }
}

impl Properties for HashMap<&str, Variant<'_>> {
    fn value(&self, name: &str) -> Option<&Variant<'_>> {
        self.get(name)
    }
}
