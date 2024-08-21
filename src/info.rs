//! General information about the crate and the emulator.

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Info;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Info {
    /// Obtains the name of the emulator.
    pub fn name() -> String {
        String::from("CHIP-Ahoyto")
    }

    /// Obtains the name of the emulator in lowercase.
    /// Useful for file paths and other cases where.
    pub fn name_lower() -> String {
        String::from("chip-ahoyto")
    }

    /// Obtains the version of the emulator.
    pub fn version() -> String {
        String::from("0.4.2")
    }

    /// Obtains the system this emulator is emulating.
    pub fn system() -> String {
        String::from("CHIP-8")
    }
}
