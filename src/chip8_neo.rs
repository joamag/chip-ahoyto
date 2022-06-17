pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const RAM_SIZE: usize = 4096;
pub const STACK_SIZE: usize = 16;
pub const REGISTERS_SIZE: usize = 16;

/// The starting address for the ROM loading, should be
/// the initial PC position.
const ROM_START: usize = 0x200;

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

#[cfg_attr(feature = "web", wasm_bindgen)]
impl Chip8Neo {
    #[cfg_attr(feature = "web", wasm_bindgen(constructor))]
    pub fn new() -> Chip8Neo {
        let chip8 = Chip8Neo {
            ram: [0u8; RAM_SIZE],
            vram: [0u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            stack: [0u16; STACK_SIZE],
            registers: [0u8; REGISTERS_SIZE],
            pc: ROM_START as u16,
            i: 0x0,
            sp: 0x0,
            dt: 0x0,
            st: 0x0,
        };
        chip8
    }

    pub fn tick(&mut self) {
        self.pc += 0x2;
    }
}
