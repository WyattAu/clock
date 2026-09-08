# chronoshift vs tokio pause/advance — head-to-head comparison

Rust has one dominant way to fake time in tests: tokio's `test-util`
feature (`start_paused = true`, `tokio::time::advance`). chronoshift
competes with it as a *time abstraction*, not a timer simulator. This page
compares both honestly: measured numbers first, then capabilities.

Reproduce:

```sh
cargo bench --bench head_to_head
```

## Benchmark: cost of reading the clock

What production code pays per clock read, criterion, single-threaded:

| benchmark                          | median time |
|------------------------------------|-------------|
| `std_system_time_now`              | ~33 ns      |
| `tokio_time_instant_now`           | ~40 ns      |
| `chronoshift_system_clock_now_ns`  | ~88 ns      |

Hardware: Intel Core i5-9400F @ 2.90GHz (6 cores), Linux x86_64,
rustc 1.94.1, criterion 0.5, chronoshift 1.0.0 (default features).

Reading the numbers honestly:

- The abstraction layer costs ~55 ns per read (~2.6x `std::time`).
  `SystemClock::now_ns()` returns nanoseconds-since-epoch, so it performs
  the `duration_since(UNIX_EPOCH)` conversion that bare `SystemTime::now()`
  defers; the gap is mostly that arithmetic, not dispatch — `system_clock()`
  returns a concrete type and `now_ns` is statically dispatched (no vtable).
- At ~88 ns this is noise for any service; code doing millions of clock
  reads per second in a tight loop should cache timestamps anyway.

## Feature comparison

|                                       | chronoshift 1.0                          | tokio `test-util` pause/advance          |
|---------------------------------------|------------------------------------------|------------------------------------------|
| Deterministic tests                   | Yes (`MockClock`, manual advance)        | Yes (paused clock, `advance`, auto-advance on idle) |
| Controls tokio timers (sleep/timeout) | No                                       | Yes                                      |
| Abstracts `std::time` in prod code    | Yes (inject `Clock`)                     | No (prod code must use `tokio::time`)    |
| Works outside tokio                   | Yes (any runtime / no runtime)           | No (requires tokio runtime)              |
| Runtime cost in production            | One trait call (~88 ns)                  | None (test-only cfg)                     |
| Feature-unification risk              | None                                     | `test-util` can leak into release builds via dep unification |
| Embedded / `no_std`                   | Yes (`--no-default-features`, critical-section fallback on 32-bit) | No |
| WASM                                  | Yes (js-sys backend, `wasm32` only)      | Partial (tokio on wasm via wasm-bindgen, no paused clock) |
| chrono interop                        | Yes (`ClockExt::now_chrono`, optional)   | No                                       |
| Unsafe code                           | Forbidden (`#![forbid(unsafe_code)]`)    | Not applicable                           |

## Positioning

tokio's pause/advance is the right tool when your time dependencies are
tokio timers (`sleep`, `timeout`, `interval`): it is compile-time free,
auto-advances, and needs no production-code changes. It cannot help with
`std::time`/`chrono` reads in your own logic, non-tokio runtimes, embedded,
or `no_std`.

chronoshift is the right tool when time-dependent *logic* (TTLs, rate
windows, cache expiry, deadlines computed from raw timestamps) should be
testable regardless of runtime or platform — at a measured cost of ~55 ns
per clock read and a manual, explicit mock (`advance_ns`/`set_ns`) rather
than an auto-driving fake clock.

The honest headline: they are complements, not substitutes — tokio's
test-util simulates the runtime's timer wheel, chronoshift abstracts the
clock your production code reads.
