#![allow(dead_code)]
use tracing::subscriber::set_global_default;
use tracing_sprout::TrunkLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{EnvFilter, Registry};

use tracing::{debug_span, error, info, info_span, trace, warn};

#[derive(Debug)]
enum Size {
    Baby,
    Normal,
    Giant,
}

#[tracing::instrument]
fn dance() {
    let size = Size::Baby;
    info!(dancing = true, growing = true, ?size, song = %"Mr Blue Sky", "Groot is dancing");
}

fn main() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("trace"));
    let formatting_layer = TrunkLayer::new("I'm Groot".to_string(), std::io::stdout);
    let subscriber = Registry::default().with(env_filter).with(formatting_layer);

    set_global_default(subscriber).expect("failed to set up global tracing subscriber");

    let parent_span = info_span!("Epic montage", id = %1);
    let _parent_guard = parent_span.enter();

    trace!(group = ?vec!["Peter Quill", "Gamora", "Drax", "Rocket"], "Trying to plug in the power");
    let child_span =
        debug_span!("Music is playing", id = %2, info = %"I'm overwriting my parents ID");

    {
        let _child_guard = child_span.enter();
        let warning = "Don't get hit Groot!";
        dance();
        warn!(%warning, goon_count = ?vec![5f64, 3.0, 24.32],  "There are lots of bad guys");
    }

    error!("Big explosions everywhere!");
}
