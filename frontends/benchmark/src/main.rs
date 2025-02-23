use std::env;

use chip_ahoyto::{
    chip8::Chip8, chip8_classic::Chip8Classic, chip8_neo::Chip8Neo, util::read_file,
};
use time::Instant;

const CYCLE_COUNT: u64 = 5_000_000_000;

fn main() {
    let mut cycles = CYCLE_COUNT;
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        cycles = args[1].parse::<u64>().unwrap();
    }

    let chips: [Box<dyn Chip8>; 2] = [Box::new(Chip8Classic::new()), Box::new(Chip8Neo::new())];

    let rom_path = "../../res/roms/pong.ch8";
    let rom = read_file(rom_path);

    for mut chip8 in chips {
        chip8.reset_hard();
        chip8.load_rom(&rom);

        let instant = Instant::now();

        println!(
            "[{}] Running {} cycles for {}",
            chip8.name(),
            cycles,
            rom_path
        );

        for _ in 0..cycles {
            chip8.clock();
        }

        let duration_s = instant.elapsed().as_seconds_f32();
        let cycles_second = cycles as f32 / duration_s;
        let mega_second = cycles_second / 1000.0 / 1000.0;

        println!(
            "[{}] Took {} seconds or {:.2} MHz CPU",
            chip8.name(),
            duration_s,
            mega_second
        );
    }
}
