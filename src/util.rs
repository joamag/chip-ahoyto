use std::{
    fs::File,
    io::{Read, Write},
};

use getrandom::getrandom;

use crate::chip8::Chip8;

pub fn read_file(path: &str) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    data
}

pub fn random() -> u8 {
    let mut n = [0];
    getrandom(&mut n).unwrap();
    n[0]
}

/// Saves a snapshot image of the provided machine
/// so that it can be latter loaded and RAM restored.
pub fn save_snapshot(name: &str, chip8: &Box<dyn Chip8>) {
    let mut file = File::create(name).unwrap();
    let state = chip8.get_state();
    let buffer = state.as_slice();
    file.write_all(buffer).unwrap();
}

pub fn take_snapshot(chip8: &Box<dyn Chip8>) {
    save_snapshot("out.sv8", chip8);
}
