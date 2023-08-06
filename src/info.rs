//! General information about the crate and the emulator.

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

/// Obtains the name of the emulator.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn name() -> String {
    String::from("CHIP-Ahoyto")
}

/// Obtains the version of the emulator.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn version() -> String {
    String::from("0.4.2")
}

/// Obtains the system this emulator is emulating.
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub fn system() -> String {
    String::from("CHIP-8")
}
