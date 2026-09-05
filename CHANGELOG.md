# Changelog

All notable changes to this project are documented here. Format: [Keep a
Changelog](https://keepachangelog.com/) — versions follow [semver](https://semver.org).

## [Unreleased]

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
