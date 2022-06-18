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
        let mut chip8 = Chip8Neo {
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
        chip8.load_default_font();
        chip8
    }

    pub fn reset(&mut self) {
        self.vram = [0u8; DISPLAY_WIDTH * DISPLAY_HEIGHT];
        self.stack = [0u16; STACK_SIZE];
        self.registers = [0u8; REGISTERS_SIZE];
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
                _ => println!("unimplemented instruction"),
            },
            0x1000 => self.pc = address,
            0x6000 => self.registers[x] = byte,
            0x7000 => self.registers[x] += byte,
            0x8000 => match nibble {
                0x0 => self.registers[x] = self.registers[y],
                0x1 => self.registers[x] |= self.registers[y],
                0x2 => self.registers[x] &= self.registers[y],
                0x3 => self.registers[x] ^= self.registers[y],
                0x4 => self.registers[x] += self.registers[y],
                0x5 => self.registers[x] -= self.registers[y],
                0x7 => self.registers[x] = self.registers[y] - self.registers[x],
                _ => println!("unimplemented instruction"),
            },
            0xa000 => self.i = address,
            0xd000 => {
                self.draw_sprite(
                    self.registers[x] as usize,
                    self.registers[y] as usize,
                    nibble as usize,
                );
            }
            _ => println!(
                "unimplemented opcode {}, instruction {}",
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
        self.registers[0xf] = 0;
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
                    self.registers[0xf] = 1;
                }
                self.vram[addr] ^= 1
            }
        }
    }
}
