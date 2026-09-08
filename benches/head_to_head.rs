// Benchmarks run on fixed, known-good inputs; unwrap failures abort the
// bench run visibly, which is the desired behavior here.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

//! Head-to-head comparison: chronoshift's `SystemClock` against the two
//! things a Rust service would otherwise call to read time:
//!
//! 1. `std::time::SystemTime::now()` — the baseline every unabstracted
//!    codebase uses.
//! 2. `tokio::time::Instant::now()` — what tokio-runtime code typically
//!    uses, and the same clock tokio's `pause`/`advance` test-util controls.
//!
//! Each iteration reads the clock and black-boxes the result; the difference
//! between rows is the measured cost of the abstraction layer. `SystemClock`
//! is returned from `system_clock()` as a concrete type (`impl Clock`), so
//! `now_ns` dispatches statically — there is no dyn/vtable hop to pay.

use std::hint::black_box;

use chronoshift::{system_clock, Clock};
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_now_ns(c: &mut Criterion) {
    let clock = system_clock();

    let mut group = c.benchmark_group("comparison");
    group.bench_function("chronoshift_system_clock_now_ns", |b| {
        b.iter(|| black_box(clock.now_ns()));
    });
    group.bench_function("std_system_time_now", |b| {
        b.iter(|| black_box(std::time::SystemTime::now()));
    });
    group.bench_function("tokio_time_instant_now", |b| {
        b.iter(|| black_box(tokio::time::Instant::now()));
    });
    group.finish();
}

criterion_group!(comparison, bench_now_ns);
criterion_main!(comparison);
