pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const RAM_SIZE: usize = 4096;
pub const STACK_SIZE: usize = 16;
pub const REGISTERS_SIZE: usize = 16;

pub struct Chip8Neo {
    ram: [u8; RAM_SIZE],
    vram: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    stack: [u16; STACK_SIZE],
    registers: [u8; REGISTERS_SIZE],
    pc: u16,
    i: u16,
    sp: u8,
    dt: u8,
    st: u8,
}
