# Changelog

All notable changes to this project are documented here. Format: [Keep a
Changelog](https://keepachangelog.com/) — versions follow [semver](https://semver.org).

## [Unreleased]

### Added
- `no_std` support: build with `--no-default-features` for core-only use
  (`MockClock` + `Clock` trait). `SystemClock`/`system_clock()` now require
  the new `std` feature (enabled by default, so default builds are unchanged).
  Targets without 64-bit atomics (e.g. thumbv7em) get a critical-section-backed
  `MockClock` fallback; the critical-section implementation is supplied by the
  final binary per critical-section convention.

### Changed
- `chrono` dependency is now `default-features = false` (no implicit
  `std`/`clock` pull-in when using chrono types in `no_std` builds).

## [0.2.0]

### Added
- Trait-based time abstraction with mock clock for testing.
- Not yet published to crates.io (crate name unavailable).

## [0.3.0] - 2026-09-05

### Changed
- BREAKING: `wasm` module now requires `target_arch = "wasm32"` in addition to the `wasm` feature (js-sys calls panic on native targets — the module was never usable there)

## [1.0.0] - 2026-09-05

### Added
- API declared stable; semver contract enforced via cargo-semver-checks CI gate
