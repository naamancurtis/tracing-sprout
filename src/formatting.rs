use std::io::Write;
use tracing::{Event, Id, Subscriber};
use tracing_core::span::{Attributes, Record};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

use std::time::Instant;

use crate::constants::*;
use crate::storage::SproutStorage;
use crate::util::{serialize_span, Type};
use crate::Result;

/// The subscriber layer, add this to your application's tracing regisitry to initialize it
///
///
/// # Example
///
/// ```no_run
/// use tracing::{subscriber::set_global_default, Subscriber};
/// use tracing_sprout::TrunkLayer;
/// use tracing_subscriber::prelude::*;
/// use tracing_subscriber::{EnvFilter, Registry};
///
/// let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
/// let formatting_layer = TrunkLayer::new("My Application".to_string(), std::io::stdout);
/// let subscriber = Registry::default()
///      .with(env_filter)
///      .with(formatting_layer);
///
/// set_global_default(subscriber).expect("failed to set up global tracing subscriber")
/// ```
pub struct TrunkLayer<W: MakeWriter + 'static> {
    writer: W,
    pid: u32,
    name: String,
}

impl<W> TrunkLayer<W>
where
    W: MakeWriter + 'static,
{
    /// Construct a new TrunkLayer to add to the Tracing Registry
    ///
    /// # Examples
    ///
    /// ```
    /// use tracing_sprout::TrunkLayer;
    ///
    /// let layer = TrunkLayer::new("I am Groot".to_string(), std::io::stdout);
    /// ```
    pub fn new(name: String, writer: W) -> Self {
        Self {
            writer,
            pid: std::process::id(),
            name,
        }
    }
    /// Given a serialized byte array, this function will add a `\n` byte to the end and
    /// then flush the bytes into the writer that was provided when initializing the layer.
    ///
    /// As it's more than likely this will be used in concurrent/multi-threaded systems partial
    /// writes directly to the writer would result in scrambled data, therefore each span has to be
    /// written entirely in one go
    fn emit(&self, mut buffer: Vec<u8>) -> Result<()> {
        buffer.push(b'\n');
        self.writer.make_writer().write_all(&buffer)?;
        Ok(())
    }
}

impl<S, W> Layer<S> for TrunkLayer<W>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    W: MakeWriter + 'static,
{
    fn new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        if let Some(span) = ctx.span(id) {
            // We want to inherit the fields from the parent span, if there is one.
            let mut visitor = if let Some(parent_span) = span.parent() {
                let mut extensions = parent_span.extensions_mut();
                extensions
                    .get_mut::<SproutStorage>()
                    .cloned()
                    .unwrap_or_else(|| SproutStorage::new(&self.name, self.pid))
            } else {
                SproutStorage::new(&self.name, self.pid)
            };

            let mut extensions = span.extensions_mut();

            // Register all fields.
            // Fields on the new span should override fields on the parent span if there is a conflict.
            attrs.record(&mut visitor);
            let attributes = visitor.clone_attributes();
            // Associate the visitor with the Span for future usage via the Span's extensions
            extensions.insert(visitor);

            if let Ok(serialized) = serialize_span(attributes, &span.metadata(), Type::Enter) {
                let _ = self.emit(serialized);
            }
        } else {
            eprintln!("[SPROUT]: Expected to find Span ID when creating a new span. \n\tThis is likely a bug");
        }
    }

    fn on_record(&self, span: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        if let Some(span) = ctx.span(span) {
            let mut extensions = span.extensions_mut();
            if let Some(visitor) = extensions.get_mut::<SproutStorage>() {
                values.record(visitor);
            } else {
                eprintln!("[SPROUT]: Expected to find Sprout Storage located in the span when recording new attributes.\n\tThis is likely a bug");
            }
        } else {
            eprintln!("[SPROUT]: Expected to find Span ID when recording span attributes.\n\tThis is likely a bug");
        }
    }

    fn on_enter(&self, span: &Id, ctx: Context<'_, S>) {
        if let Some(span) = ctx.span(span) {
            let mut extensions = span.extensions_mut();
            if let Some(visitor) = extensions.get_mut::<SproutStorage>() {
                visitor.entered_at = Some(Instant::now());
            } else {
                eprintln!("[SPROUT]: Expected to find Sprout Storage located in the span when entering it.\n\tThis is likely a bug");
            }
        } else {
            eprintln!(
                "[SPROUT]: Expected to find Span ID when entering span.\n\tThis is likely a bug"
            );
        }
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let mut visitor = ctx
            .lookup_current()
            .map(|span| span.extensions_mut().get_mut::<SproutStorage>().cloned())
            .flatten()
            .unwrap_or_else(|| SproutStorage::new(&self.name, self.pid));

        event.record(&mut visitor);
        let elapsed: Option<u64> = visitor
            .entered_at
            .clone()
            .map(|t| t.elapsed().as_millis() as u64);
        // It would be nice for it to have this value, but if it fails, it fails
        visitor.add_attribute_opt(TIME_SINCE_START, elapsed).ok();
        let metadata = event.metadata();
        if let Ok(bytes) = serialize_span(visitor.attributes, metadata, Type::Event) {
            let _ = self.emit(bytes);
        } else {
            eprintln!(
                "[SPROUT]: Was unable to write event bytes to the writer.\n\tThis is likely a bug"
            );
        }
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        if let Some(span) = ctx.span(&id) {
            let mut extensions = span.extensions_mut();
            if let Some(visitor) = extensions.get_mut::<SproutStorage>() {
                let elapsed: Option<u64> = visitor
                    .entered_at
                    .take()
                    .map(|t| t.elapsed().as_millis() as u64);
                // It would be nice for it to have this value, but if it fails, it fails
                visitor.add_attribute_opt(ELAPSED_MILLIS, elapsed).ok();
                if let Ok(serialized) =
                    serialize_span(visitor.clone_attributes(), &span.metadata(), Type::Exit)
                {
                    let _ = self.emit(serialized);
                }
            };
        } else {
            eprintln!(
                "[SPROUT]: Expected to find Span ID when closing span.\n\tThis is likely a bug"
            );
        }
    }
}
