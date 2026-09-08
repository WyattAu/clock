# clock

Trait-based time abstraction for Rust (`chronoshift`) — injectable clock for
production code, `MockClock` for deterministic tests. `#![forbid(unsafe_code)]`.

## Features

- `Clock` trait: `now_ns()` plus derived `now_ms`/`now_us`/`elapsed_*`
- `SystemClock`: wall-clock production implementation (`std`)
- `MockClock`: manually advanced fake clock (`advance_ns`/`ms`/`secs`, `set_*`)
- `no_std` support (`--no-default-features`, critical-section fallback on
  targets without 64-bit atomics)
- WASM backend via js-sys (`wasm` feature, `wasm32` targets)
- Optional chrono interop (`ClockExt::now_chrono`)

## Quick Start

```rust
use chronoshift::{Clock, MockClock};

// Production: `chronoshift::system_clock()`.
// Tests: inject a mock and drive time explicitly.
let clock = MockClock::new(0);
assert_eq!(clock.now_ns(), 0);
clock.advance_secs(30);
assert_eq!(clock.now_ms(), 30_000);
```

## Performance

Cost of the abstraction layer vs `std::time::SystemTime::now()` and
`tokio::time::Instant::now()`, plus an honest comparison against tokio's
`pause`/`advance` test-util: [COMPARISON.md](COMPARISON.md).
