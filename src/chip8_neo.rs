pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const RAM_SIZE: usize = 4096;

pub struct Chip8Neo {
    ram: [u8; RAM_SIZE],
    vram: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],
}
