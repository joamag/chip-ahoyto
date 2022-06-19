pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const RAM_SIZE: usize = 4096;
pub const STACK_SIZE: usize = 16;
pub const REGISTERS_SIZE: usize = 16;

/// The starting address for the ROM loading, should be
/// the initial PC position for execution.
const ROM_START: usize = 0x200;

static FONT_SET: [u8; 80] = [
    0xf0, 0x90, 0x90, 0x90, 0xf0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xf0, 0x10, 0xf0, 0x80, 0xf0, // 2
    0xf0, 0x10, 0xf0, 0x10, 0xf0, // 3
    0x90, 0x90, 0xf0, 0x10, 0x10, // 4
    0xf0, 0x80, 0xf0, 0x10, 0xf0, // 5
    0xf0, 0x80, 0xf0, 0x90, 0xf0, // 6
    0xf0, 0x10, 0x20, 0x40, 0x40, // 7
    0xf0, 0x90, 0xf0, 0x90, 0xf0, // 8
    0xf0, 0x90, 0xf0, 0x10, 0xf0, // 9
    0xf0, 0x90, 0xf0, 0x90, 0x90, // A
    0xe0, 0x90, 0xe0, 0x90, 0xe0, // B
    0xf0, 0x80, 0x80, 0x80, 0xf0, // C
    0xe0, 0x90, 0x90, 0x90, 0xe0, // D
    0xf0, 0x80, 0xf0, 0x80, 0xf0, // E
    0xf0, 0x80, 0xf0, 0x80, 0x80, // F
];

pub struct Chip8Neo {
    ram: [u8; RAM_SIZE],
    vram: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    stack: [u16; STACK_SIZE],
    regs: [u8; REGISTERS_SIZE],
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
        let mut chip8 = Chip8Neo {
            ram: [0u8; RAM_SIZE],
            vram: [0u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            stack: [0u16; STACK_SIZE],
            regs: [0u8; REGISTERS_SIZE],
            pc: ROM_START as u16,
            i: 0x0,
            sp: 0x0,
            dt: 0x0,
            st: 0x0,
        };
        chip8.load_default_font();
        chip8
    }

    pub fn reset(&mut self) {
        self.vram = [0u8; DISPLAY_WIDTH * DISPLAY_HEIGHT];
        self.stack = [0u16; STACK_SIZE];
        self.regs = [0u8; REGISTERS_SIZE];
        self.pc = ROM_START as u16;
        self.i = 0x0;
        self.sp = 0x0;
        self.dt = 0x0;
        self.st = 0x0;
        self.load_default_font();
    }

    pub fn reset_hard(&mut self) {
        self.ram = [0u8; RAM_SIZE];
        self.reset();
    }

    pub fn pixels(&self) -> Vec<u8> {
        self.vram.to_vec()
    }

    pub fn clock(&mut self) {
        // fetches the current instruction and increments
        // the PC (program counter) accordingly
        let instruction =
            (self.ram[self.pc as usize] as u16) << 8 | self.ram[self.pc as usize + 1] as u16;
        self.pc += 0x2;

        let opcode = instruction & 0xf000;
        let address = instruction & 0x0fff;
        let x = ((instruction & 0x0f00) >> 8) as usize;
        let y = ((instruction & 0x00f0) >> 4) as usize;
        let nibble = (instruction & 0x000f) as u8;
        let byte = (instruction & 0x00ff) as u8;

        match opcode {
            0x0000 => match byte {
                0xe0 => self.clear_screen(),
                0xee => {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                }
                _ => println!("unimplemented instruction "),
            },
            0x1000 => self.pc = address,
            0x2000 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = address;
            }
            0x3000 => self.pc += if self.regs[x] == byte { 2 } else { 0 },
            0x4000 => self.pc += if self.regs[x] != byte { 2 } else { 0 },
            0x5000 => self.pc += if self.regs[x] == self.regs[y] { 2 } else { 0 },
            0x6000 => self.regs[x] = byte,
            0x7000 => self.regs[x] = self.regs[x].wrapping_add(byte),
            0x8000 => match nibble {
                0x0 => self.regs[x] = self.regs[y],
                0x1 => self.regs[x] |= self.regs[y],
                0x2 => self.regs[x] &= self.regs[y],
                0x3 => self.regs[x] ^= self.regs[y],
                0x4 => {
                    let (result, overflow) = self.regs[x].overflowing_add(self.regs[y]);
                    self.regs[x] = result;
                    self.regs[0xf] = overflow as u8;
                }
                0x5 => {
                    self.regs[0xf] = (self.regs[x] >= self.regs[y]) as u8;
                    self.regs[x] = self.regs[x].saturating_sub(self.regs[y]);
                }
                0x6 => {
                    self.regs[0xf] = self.regs[x] & 0x01;
                    self.regs[x] >>= 1;
                }
                0x7 => {
                    self.regs[0xf] = (self.regs[y] >= self.regs[x]) as u8;
                    self.regs[x] = self.regs[y].saturating_sub(self.regs[x]);
                }
                0xe => {
                    self.regs[0xf] = (self.regs[x] & 0x80) >> 7;
                    self.regs[x] <<= 1;
                }
                _ => println!("unimplemented instruction"),
            },
            0x9000 => self.pc += if self.regs[x] != self.regs[y] { 2 } else { 0 },
            0xa000 => self.i = address,
            0xb000 => self.pc = address + self.regs[0x0] as u16,
            0xc000 => self.regs[x] = byte, //@todo: generate random number
            0xd000 => {
                self.draw_sprite(
                    self.regs[x] as usize,
                    self.regs[y] as usize,
                    nibble as usize,
                );
            }
            0xf000 => match byte {
                0x07 => self.regs[x] = self.dt,
                0x15 => self.dt = self.regs[x],
                0x18 => self.st = self.regs[x],
                0x29 => self.i = self.regs[x] as u16 * 5,
                0x33 => {
                    self.ram[self.i as usize] = self.regs[x] / 100;
                    self.ram[self.i as usize + 1] = (self.regs[x] / 10) % 10;
                    self.ram[self.i as usize + 2] = self.regs[x] % 10;
                }
                0x55 => self.ram[self.i as usize..self.i as usize + x + 1]
                    .clone_from_slice(&self.regs[0..x + 1]),
                0x65 => self.regs[0..x + 1]
                    .clone_from_slice(&self.ram[self.i as usize..self.i as usize + x + 1]),
                _ => println!(
                    "unimplemented instruction 0xf000, instruction 0x{:04x}",
                    instruction
                ),
            },
            _ => println!(
                "unimplemented opcode 0x{:04x}, instruction 0x{:04x}",
                opcode, instruction
            ),
        }
    }

    pub fn clock_dt(&mut self) {}

    pub fn clock_st(&mut self) {}

    pub fn key_press(&mut self, key: u8) {}

    pub fn key_lift(&mut self, key: u8) {}

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.ram[ROM_START..ROM_START + rom.len()].clone_from_slice(&rom);
    }

    pub fn beep(&self) -> bool {
        false
    }

    pub fn pc(&self) -> u16 {
        self.pc
    }

    pub fn sp(&self) -> u8 {
        self.sp
    }

    fn load_font(&mut self, position: usize, font_set: &[u8]) {
        self.ram[position..position + font_set.len()].clone_from_slice(&font_set);
    }

    fn load_default_font(&mut self) {
        self.load_font(0, &FONT_SET);
    }

    fn clear_screen(&mut self) {
        self.vram = [0u8; DISPLAY_WIDTH * DISPLAY_HEIGHT];
    }

    fn draw_sprite(&mut self, x0: usize, y0: usize, height: usize) {
        self.regs[0xf] = 0;
        for y in 0..height {
            let line_byte = self.ram[(self.i as usize + y)];
            for x in 0..8 {
                let yf = (y0 + y) % DISPLAY_HEIGHT;
                let xf = (x0 + x) % DISPLAY_WIDTH;
                if line_byte & (0x80 >> x) == 0 {
                    continue;
                }
                let addr = yf * DISPLAY_WIDTH + xf;
                if self.vram[addr] == 1 {
                    self.regs[0xf] = 1;
                }
                self.vram[addr] ^= 1
            }
        }
    }
}
