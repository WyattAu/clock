use crate::Clock;

/// WASM-compatible clock using js_sys::Date.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub struct WasmClock;

impl Clock for WasmClock {
    fn now_ns(&self) -> i64 {
        let ms = js_sys::Date::now();
        (ms * 1_000_000.0) as i64
    }
}

impl WasmClock {
    /// Create a new WASM clock.
    pub fn new() -> Self {
        Self
    }
}
