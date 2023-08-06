use getrandom::getrandom;
use std::{
    fs::File,
    io::{Read, Write},
};

use crate::chip8::Chip8;

/// Reads the contents of the file at the given path into
/// a vector of bytes.
pub fn read_file(path: &str) -> Vec<u8> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("Failed to open file: {}", path),
    };
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    data
}

/// Generates a random byte, to be used as PRNG function
/// in the emulator.
pub fn random() -> u8 {
    let mut n = [0];
    getrandom(&mut n).unwrap();
    n[0]
}

/// Saves a snapshot image of the provided machine
/// so that it can be latter loaded and RAM restored.
pub fn save_snapshot(name: &str, chip8: &dyn Chip8) {
    let mut file = File::create(name).unwrap();
    let state = chip8.get_state();
    let buffer = state.as_slice();
    file.write_all(buffer).unwrap();
}

pub fn take_snapshot(chip8: &dyn Chip8) {
    save_snapshot("out.sv8", chip8);
}

/// Capitalizes the first character in the provided string.
pub fn capitalize(string: &str) -> String {
    let mut chars = string.chars();
    match chars.next() {
        None => String::new(),
        Some(chr) => chr.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
