use chip_ahoyto::{chip8::Chip8, util::read_file};
use time::Instant;

const CYCLE_COUNT: u64 = 1_000_000_000;

fn main() {
    let rom_path = "./resources/pong.ch8";
    let rom = read_file(rom_path);

    let mut chip8 = Chip8::new();
    chip8.reset_hard();
    chip8.load_rom(&rom);

    let instant = Instant::now();

    let cycles = CYCLE_COUNT;

    println!("Running {} cycles for {}", cycles, rom_path);

    for _ in 0..CYCLE_COUNT {
        chip8.tick();
    }

    let duration_s = instant.elapsed().as_seconds_f32();
    let cycles_second = cycles as f32 / duration_s;
    let mega_second = cycles_second / 1000.0 / 1000.0;

    println!("Took {} seconds or {:.2} MHz CPU", duration_s, mega_second);
}
