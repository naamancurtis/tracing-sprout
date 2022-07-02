<div align="center">
  <h1>Tracing Sprout</h1>
  <p>
    <strong>A tokio-rs/tracing structured JSON formatting layer for the fledgling logger</strong>
  </p>
  <p>

[![crates.io](https://img.shields.io/crates/v/tracing_sprout?label=latest)](https://crates.io/crates/tracing_sprout)
[![docs](https://docs.rs/tracing_sprout/badge.svg)](https://docs.rs/tracing_sprout/latest/tracing_sprout/)
[![repo](https://img.shields.io/badge/github-code-black)](https://github.com/naamancurtis/tracing-sprout)
[![MIT](https://img.shields.io/github/license/naamancurtis/tracing-sprout)](https://github.com/naamancurtis/tracing-sprout/blob/main/LICENSE)

</div>

Heavily inspired by [Tracing Bunyan Formatter](https://github.com/LukeMathWalker/tracing-bunyan-formatter), just with some slight tweaks to the internals and the formatting. The actual formatting doesn't follow any defined specification, it's just something I view as readable and useful - I'm open to suggestions if people would like it aligned to a specific format.

## Features

- All traces will receive their parent's attributes as well as their own, child attributes will take precedence if there are collisions
- There is a very minimal timing capability that adds elapsed time to `EVENT` and `EXIT` traces
- `TRACE`, `DEBUG` and `ERROR` logs get slightly more metadata _(file name, line number, module path & target)_ attached to them
- Avoids panics - as much as possible it opts to handle failure by `eprintln`ing to `stdout`. These scenarios should be few and far between, but it's better that a failure in your tracing implementation doesn't poison your main application. _(although ideally it shouldn't fail silently)_

All traces will receive their parent's attributes as well as their own, there is also a very minimal timing capability that adds elapsed time to `Event` and `Exit` traces

## Basic Example

See `/examples` for a slightly more complex example

```rust
use tracing::{subscriber::set_global_default, Subscriber};
use tracing_sprout::TrunkLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{EnvFilter, Registry};

let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
let formatting_layer = TrunkLayer::new("My Application".to_string(), std::io::stdout);
let subscriber = Registry::default()
     .with(env_filter)
     .with(formatting_layer);

set_global_default(subscriber).expect("failed to set up global tracing subscriber")
```

## Example Output

### Raw JSON

```txt
{"name":"I'm Groot","version":"0.1.0-alpha.1","id":"1","time":"Sat, 02 Jul 2022 09:33:59 -0600","msg":"[EPIC MONTAGE | START]","level":"info","span_type":"enter"}
{"name":"I'm Groot","version":"0.1.0-alpha.1","id":"1","group":["Peter Quill","Gamora","Drax","Rocket"],"time":"Sat, 02 Jul 2022 09:33:59 -0600","msg":"[EVENT] Trying to plug in the power","level":"trace","file":"examples/basic.rs","line":32,"target":"basic","thread_id":"ThreadId(1)","thread_name":"main","span_type":"event"}
{"name":"I'm Groot","version":"0.1.0-alpha.1","id":"2","info":"I'm overwriting my parents ID","time":"Sat, 02 Jul 2022 09:33:59 -0600","msg":"[MUSIC IS PLAYING | START]","level":"debug","file":"examples/basic.rs","line":34,"target":"basic","thread_id":"ThreadId(1)","thread_name":"main","span_type":"enter"}
```

### Piped through CLI tool

In this case the CLI tool used was [pino-pretty](https://github.com/pinojs/pino-pretty)

```txt
[Sat, 02 Jul 2022 09:34:55 -0600] INFO (I'm Groot): [EPIC MONTAGE | STA
    version: "0.1.0-alpha.1"
    id: "1"
    span_type: "enter"
[Sat, 02 Jul 2022 09:34:55 -0600] TRACE (I'm Groot): [EVENT] Trying to
    version: "0.1.0-alpha.1"
    id: "1"
    group: [
      "Peter Quill",
      "Gamora",
      "Drax",
      "Rocket"
    ]
    file: "examples/basic.rs"
    line: 32
    target: "basic"
    thread_id: "ThreadId(1)"
    thread_name: "main"
    span_type: "event"
[Sat, 02 Jul 2022 09:34:55 -0600] DEBUG (I'm Groot): [MUSIC IS PLAYING | START]
    version: "0.1.0-alpha.1"
    id: "2"
    info: "I'm overwriting my parents ID"
    file: "examples/basic.rs"
    line: 34
    target: "basic"
    thread_id: "ThreadId(1)"
    thread_name: "main"
    span_type: "enter"
[Sat, 02 Jul 2022 09:34:55 -0600] INFO (I'm Groot): [DANCE | START]
    version: "0.1.0-alpha.1"
    id: "2"
    info: "I'm overwriting my parents ID"
    span_type: "enter"
```
