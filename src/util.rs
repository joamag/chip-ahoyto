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
pub fn save_snapshot(chip8: &Box<dyn Chip8>) {
    let mut file = File::create("out.snp").unwrap();
    file.write(&chip8.pc().to_le_bytes()).unwrap();
    //file.write(&chip8.ram().tr().).unwrap();
    //@todo need to serialize the rest of the machine
}
