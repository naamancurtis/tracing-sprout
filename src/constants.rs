// Metadata we're enhancing the spans with
pub(crate) const TIME: &str = "time";
pub(crate) const TIMESTAMP: &str = "timestamp";
pub(crate) const MESSAGE: &str = "msg";
pub(crate) const ELAPSED_MILLIS: &str = "elapsed_time_ms";
pub(crate) const TIME_SINCE_START: &str = "time_since_span_entered_ms";

/// Type of the span
pub(crate) const TYPE: &str = "span.type";

// Span Metadata
pub(crate) const LEVEL: &str = "level";
pub(crate) const FILE: &str = "file";
pub(crate) const LINE: &str = "line";
pub(crate) const TARGET: &str = "target";
pub(crate) const MODULE: &str = "module";
pub(crate) const THREAD_ID: &str = "thread_id";
pub(crate) const THREAD_NAME: &str = "thread_name";
