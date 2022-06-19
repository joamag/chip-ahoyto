use chip_ahoyto::{chip8::Chip8, chip8_neo::Chip8Neo, util::read_file};
use time::Instant;

const CYCLE_COUNT: u64 = 5_000_000_000;

fn benchmark_chip8() {
    let rom_path = "./resources/pong.ch8";
    let rom = read_file(rom_path);

    let mut chip8 = Chip8::new();
    chip8.reset_hard();
    chip8.load_rom(&rom);

    let instant = Instant::now();

    let cycles = CYCLE_COUNT;

    println!("[Chip8] Running {} cycles for {}", cycles, rom_path);

    for _ in 0..CYCLE_COUNT {
        chip8.clock();
    }

    let duration_s = instant.elapsed().as_seconds_f32();
    let cycles_second = cycles as f32 / duration_s;
    let mega_second = cycles_second / 1000.0 / 1000.0;

    println!(
        "[Chip8] Took {} seconds or {:.2} MHz CPU",
        duration_s, mega_second
    );
}

fn benchmark_chip8_neo() {
    let rom_path = "./resources/pong.ch8";
    let rom = read_file(rom_path);

    let mut chip8 = Chip8Neo::new();
    chip8.reset_hard();
    chip8.load_rom(&rom);

    let instant = Instant::now();

    let cycles = CYCLE_COUNT;

    println!("[Chip8Neo] Running {} cycles for {}", cycles, rom_path);

    for _ in 0..CYCLE_COUNT {
        chip8.clock();
    }

    let duration_s = instant.elapsed().as_seconds_f32();
    let cycles_second = cycles as f32 / duration_s;
    let mega_second = cycles_second / 1000.0 / 1000.0;

    println!(
        "[Chip8Neo] Took {} seconds or {:.2} MHz CPU",
        duration_s, mega_second
    );
}

fn main() {
    benchmark_chip8();
    benchmark_chip8_neo();
}
