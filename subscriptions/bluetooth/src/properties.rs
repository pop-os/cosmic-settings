// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

//! The properties of BlueZ objects, as they come in the initial read and in signals.

use std::collections::HashMap;
use zbus::zvariant::{OwnedObjectPath, OwnedValue, Value};

/// The properties of one interface of an object.
pub(crate) trait Properties {
    fn value(&self, name: &str) -> Option<&Value<'_>>;

    fn string(&self, name: &str) -> Option<String> {
        match self.value(name)? {
            Value::Str(value) => Some(value.to_string()),
            _ => None,
        }
    }

    fn boolean(&self, name: &str) -> Option<bool> {
        match self.value(name)? {
            Value::Bool(value) => Some(*value),
            _ => None,
        }
    }

    fn byte(&self, name: &str) -> Option<u8> {
        match self.value(name)? {
            Value::U8(value) => Some(*value),
            _ => None,
        }
    }

    fn object_path(&self, name: &str) -> Option<OwnedObjectPath> {
        match self.value(name)? {
            Value::ObjectPath(value) => Some(value.clone().into()),
            _ => None,
        }
    }
}

impl Properties for HashMap<String, OwnedValue> {
    fn value(&self, name: &str) -> Option<&Value<'_>> {
        self.get(name).map(|value| &**value)
    }
}

impl Properties for HashMap<&str, Value<'_>> {
    fn value(&self, name: &str) -> Option<&Value<'_>> {
        self.get(name)
    }
}
