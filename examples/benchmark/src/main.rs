use chip_ahoyto::chip8::Chip8;
use std::{fs::File, io::Read};

const CYCLE_COUNT: u32 = 1_000_000_000;

fn main() {
    let rom = read_file("./resources/pong.ch8");

    let mut chip8 = Chip8::new();
    chip8.reset_hard();
    chip8.load_rom(&rom);

    println!("Running {} cycles", CYCLE_COUNT);

    for _ in 0..CYCLE_COUNT {
        chip8.tick();
    }
}

fn read_file(path: &str) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    data
}
