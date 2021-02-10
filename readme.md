# Tracing Sprout

Full disclosure, this is essentially a straight copy of [Tracing Bunyan Formatter](https://github.com/LukeMathWalker/tracing-bunyan-formatter), however I've changed some internals and some of the formatting (and rules surrounding it) to suit my own needs.

All traces will receive their parent's attributes as well as their own, there is also a very minimal timing capability that adds elapsed time to `Event` and `Exit` traces

## Changes

- The formatting is largely arbitrary (although it is consistent) and something I view as both machine
  and semi-human friendly.
- Extra metadata (like `file`, `line` etc) will only be added to `WARN`, `ERROR`
  and `TRACE`
  traces , this is simply to reduce the noise (and cost of storing them)
- The underlying JSON implementation uses [json-rust](https://github.com/maciejhirsz/json-rust), looking at the benchmarks, this seemed like a reasonable shift, however I haven't done any testing of my own yet to verify that.
- With some tracing libraries recently I've noticed the occasional bit of
  undefined behavior when
  used in an async multi-threaded context under immense load. Occasionally some of
  the `unwrap` or `expects` are actually panicking and
  poisoning the application instance _(my assumption being the load is high
  enough that it's possibly processing the tracing `lifecycle` events out of order)_ however the issues are intermittent and not easily reproducable. As such I've tried to go down a route with this crate
  where failure is allowed (if it finds a `None` where it expects a `Some` it just doesn't proceed). I've added standard `eprintln` statements when these errors occur
  (hopefully not at all) so they aren't silent, but let me know if they do.
  After all, there's something ironic about the thing you use to try and keep
  your applications running being the thing that's actually causing them to
  crash
