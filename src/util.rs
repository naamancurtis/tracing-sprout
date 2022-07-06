use chrono::prelude::*;
use json::JsonValue;
use tracing_core::metadata::{Level, Metadata};

use std::thread;

use crate::constants::*;
use crate::Result;

#[derive(Debug, Copy, Clone)]
pub(crate) enum Type {
    Enter,
    Event,
    Exit,
}

impl Type {
    pub(crate) fn as_str(&self) -> &str {
        match *self {
            Self::Enter => "enter",
            Self::Event => "event",
            Self::Exit => "exit",
        }
    }

    pub(crate) fn as_msg(&self) -> &str {
        match *self {
            Self::Enter => "START",
            Self::Event => "EVENT",
            Self::Exit => "END",
        }
    }
}

pub(crate) fn level_as_str(level: &Level) -> &str {
    match *level {
        Level::TRACE => "trace",
        Level::DEBUG => "debug",
        Level::INFO => "info",
        Level::WARN => "warn",
        Level::ERROR => "error",
    }
}

pub(crate) fn insert_core_fields(
    obj: &mut JsonValue,
    metadata: &Metadata,
    msg: &str,
) -> Result<()> {
    let now = Local::now();
    let level = metadata.level();

    obj.insert(TIME, now.to_rfc2822())?;
    obj.insert(MESSAGE, msg)?;
    obj.insert(LEVEL, level_as_str(level))?;
    obj.insert(TARGET, metadata.target())?;

    if matches!(*level, Level::TRACE | Level::DEBUG | Level::ERROR) {
        if let Some(file) = metadata.file() {
            obj.insert(FILE, file)?;
        }
        if let Some(line) = metadata.line() {
            obj.insert(LINE, line)?;
        }
        let t = thread::current();
        obj.insert(THREAD_ID, format!("{:?}", t.id()))?;
        obj.insert(THREAD_NAME, t.name().unwrap_or(""))?;
    }
    Ok(())
}

pub(crate) fn serialize_span(
    mut attributes: JsonValue,
    metadata: &Metadata,
    span_type: Type,
) -> Result<Vec<u8>> {
    let msg = match span_type {
        Type::Event => format_event_message(metadata, &attributes),
        Type::Enter | Type::Exit => format_span_context(metadata, span_type),
    };
    insert_core_fields(&mut attributes, metadata, &msg)?;
    attributes.insert(TYPE, span_type.as_str())?;
    // We remove the message, because we have added in our custom `msg` property
    attributes.remove("message");

    // I'd like to shrink this down, but it seems better to be safe than sorry?
    let mut buffer = Vec::with_capacity(1024);
    attributes.write(&mut buffer)?;

    Ok(buffer)
}

/// # Example
///
/// `[LOGIN_HANDLER | START]`
pub(crate) fn format_span_context(metadata: &Metadata, span_type: Type) -> String {
    match span_type {
        Type::Enter | Type::Exit => format!(
            "[{} | {}]",
            metadata.name().to_uppercase(),
            span_type.as_msg()
        ),
        Type::Event => format!("[{}]", span_type.as_msg()),
    }
}

/// # Example
///
/// `[EVENT] Received request to log in.`
pub(crate) fn format_event_message(metadata: &Metadata, attributes: &JsonValue) -> String {
    // Extract the "message" field, if provided. Fallback to the target, if missing.
    let message = if let Some(message) = attributes["message"].as_str() {
        message
    } else {
        metadata.target()
    };

    // If the event is in the context of a span, prepend the span name to the message.
    return format!(
        "{} {}",
        &format_span_context(metadata, Type::Event),
        message
    );
}
