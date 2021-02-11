use json::{object, JsonValue};
use tracing::field::{Field, Visit};

use std::fmt;
use std::time::Instant;

use crate::Result;

#[derive(Debug)]
pub struct SproutStorage {
    pub(crate) attributes: JsonValue,
    pub(crate) entered_at: Option<Instant>,
}

impl Clone for SproutStorage {
    fn clone(&self) -> Self {
        Self {
            attributes: self.attributes.clone(),
            entered_at: None,
        }
    }
}

impl SproutStorage {
    pub fn new(name: &str, pid: u32) -> Self {
        let attributes = object! {
            "app_name": name,
            "pid": pid
        };
        Self {
            attributes,
            entered_at: None,
        }
    }

    pub fn clone_attributes(&self) -> JsonValue {
        self.attributes.clone()
    }

    pub fn add_attribute_opt<T>(&mut self, key: &str, value: Option<T>) -> Result<()>
    where
        T: Into<JsonValue>,
    {
        if let Some(v) = value {
            self.attributes.insert(key, v)?;
        }
        Ok(())
    }
}

/// Taken verbatim from tracing-subscriber
impl Visit for SproutStorage {
    /// Visit a signed 64-bit integer value.
    fn record_i64(&mut self, field: &Field, value: i64) {
        self.attributes
            .insert(&field.name(), value)
            .expect("Root should always be a json object");
    }

    /// Visit an unsigned 64-bit integer value.
    fn record_u64(&mut self, field: &Field, value: u64) {
        self.attributes
            .insert(&field.name(), value)
            .expect("Root should always be a json object");
    }

    /// Visit a boolean value.
    fn record_bool(&mut self, field: &Field, value: bool) {
        self.attributes
            .insert(&field.name(), value)
            .expect("Root should always be a json object");
    }

    /// Visit a string value.
    fn record_str(&mut self, field: &Field, value: &str) {
        self.attributes
            .insert(&field.name(), value)
            .expect("Root should always be a json object");
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        match field.name() {
            // Skip fields that are actually log metadata that have already been handled
            name if name.starts_with("log.") => (),
            name if name.starts_with("r#") => {
                self.attributes
                    .insert(&name[2..], format!("{:?}", value))
                    .expect("Root should always be a json object");
            }
            name => {
                self.attributes
                    .insert(name, format!("{:?}", value))
                    .expect("Root should always be a json object");
            }
        };
    }
}
