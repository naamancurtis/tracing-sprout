//! # Tracing Sprout
//!
//! A tokio-rs/tracing structured JSON formatting layer for the fledgling logger
//!
//! This subscriber layer is derived from [Tracing Bunyan Formatter](https://github.com/LukeMathWalker/tracing-bunyan-formatter),
//! with a few tweaks to internals and some of the formatting (and rules surrounding it).
//!
//! ## Features
//! - All traces will receive their parent's attributes as well as their own, child attributes will
//! take precedence if there are collisions
//! - There is a very minimal timing capability that adds elapsed time to `EVENT` and `EXIT` traces
//! - `TRACE`, `DEBUG` and `ERROR` logs get slighly more metadata (file name, line number, module path & target) attached
//! to them
//! - Avoids panics - as much as possible it opts to handle failure by `eprintln`ing to `stdout`.
//! These scenarios should be few and far between, but it's better that a failure in your tracing
//! implementation doesn't poison your main application.
//!
//! ## Example
//!
//! ```no_run
//! use tracing::{subscriber::set_global_default, Subscriber};
//! use tracing_sprout::TrunkLayer;
//! use tracing_subscriber::prelude::*;
//! use tracing_subscriber::{EnvFilter, Registry};
//!
//! let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
//! let formatting_layer = TrunkLayer::new("My Application".to_string(), std::io::stdout);
//! let subscriber = Registry::default()
//!      .with(env_filter)
//!      .with(formatting_layer);
//!
//! set_global_default(subscriber).expect("failed to set up global tracing subscriber")
//! ```
//!
//! ## Developing on an application using this
//!
//! Generally structured JSON logs are great for machines to read, but not so kind for humans. If
//! you're developing an application that uses this layer it would be advisable to download some
//! kind of CLI tool that outputs these logs in a slightly nicer format, a good example would be
//! the node.js's [pino-pretty](https://github.com/pinojs/pino-pretty), from there you can just pipe
//! your logs into it for a nicer development experience
//!
//! ```sh
//! cargo run | pino-pretty
//! ```
//!
//! ## Log output and a more detailed example
//!
//! See the [examples](https://github.com/naamancurtis/tracing-sprout/tree/main/examples) for a basic demonstration of how this can be used. The **basic.rs** example in there would output logs like the following:
//!
//! ```txt
//! {"app_name":"I'm Groot","pid":2735,"id":"1","time":"Sat, 13 Feb 2021 14:16:15 +0000","timestamp":1613225775,"msg":"[EPIC MONTAGE | START]","level":"info","span.type":"enter"}
//! {"app_name":"I'm Groot","pid":2735,"id":"1","group":"[\"Peter Quill\", \"Gamora\", \"Drax\", \"Rocket\"]","time":"Sat, 13 Feb 2021 14:16:15 +0000","timestamp":1613225775,"msg":"[EVENT] Trying to plug in the power","level":"trace","file":"examples/basic.rs","line":32,"module":"basic","target":"basic","thread_id":"ThreadId(1)","thread_name":"main","span.type":"event"}
//! {"app_name":"I'm Groot","pid":2735,"id":"2","info":"I'm overwriting my parents ID","time":"Sat, 13 Feb 2021 14:16:15 +0000","timestamp":1613225775,"msg":"[MUSIC IS PLAYING | START]","level":"debug","file":"examples/basic.rs","line":34,"module":"basic","target":"basic","thread_id":"ThreadId(1)","thread_name":"main","span.type":"enter"}
//! ...
//! ```
//!
//! However _(excuse the verbosity here)_, by piping it through a tool like the one mentioned above
//! _(pino-pretty)_ you get the following output
//!
//! ```txt
//! [Sat, 13 Feb 2021 14:14:54 +0000] INFO (1331): [EPIC MONTAGE | START]
//!     app_name: "I'm Groot"
//!     id: "1"
//!     span.type: "enter"
//! [Sat, 13 Feb 2021 14:14:54 +0000] TRACE (1331): [EVENT] Trying to plug in the power
//!     app_name: "I'm Groot"
//!     id: "1"
//!     group: "[\"Peter Quill\", \"Gamora\", \"Drax\", \"Rocket\"]"
//!     file: "examples/basic.rs"
//!     line: 32
//!     module: "basic"
//!     target: "basic"
//!     thread_id: "ThreadId(1)"
//!     thread_name: "main"
//!     span.type: "event"
//! [Sat, 13 Feb 2021 14:14:54 +0000] DEBUG (1331): [MUSIC IS PLAYING | START]
//!     app_name: "I'm Groot"
//!     id: "2"
//!     info: "I'm overwriting my parents ID"
//!     file: "examples/basic.rs"
//!     line: 34
//!     module: "basic"
//!     target: "basic"
//!     thread_id: "ThreadId(1)"
//!     thread_name: "main"
//!     span.type: "enter"
//!     ...
//! ```

pub(crate) mod constants;
mod error;
mod formatting;
mod storage;
pub(crate) mod util;

pub(crate) use error::SproutError;
pub(crate) type Result<T> = std::result::Result<T, SproutError>;

pub use formatting::TrunkLayer;
