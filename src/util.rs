use std::{fs::File, io::Read};

use getrandom::getrandom;

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
