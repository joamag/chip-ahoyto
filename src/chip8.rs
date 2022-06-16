use std::fmt::Display;

use getrandom::getrandom;

#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

/// The width of the screen in pixels.
pub const SCREEN_PIXEL_WIDTH: usize = 64;

/// The height of the screen in pixels.
pub const SCREEN_PIXEL_HEIGHT: usize = 32;

/// The number of keys to be allocated to the machine.
const NUM_KEYS: usize = 16;

/// The total number of CPU registers that are going to be used
/// this value should follow the typical convention.
const NUM_REGISTERS: usize = 16;

/// The size of the stack, this value is actually a convention
/// as no specific definition on its size was ever created.
const STACK_SIZE: usize = 16;

/// The size of the RAM memory in bytes.
const RAM_SIZE: usize = 4096;

/// The starting address for the ROM loading, should be
/// the initial PC position.
const ROM_START: usize = 0x200;

/// Buffer that contains the base CHIP-8 font set that
/// is going to be used to draw the font in the screen
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

#[cfg_attr(feature = "web", wasm_bindgen)]
pub struct Chip8 {
    vram: [u8; SCREEN_PIXEL_WIDTH * SCREEN_PIXEL_HEIGHT],
    ram: [u8; RAM_SIZE],
    registers: [u8; NUM_REGISTERS],
    stack: [u16; STACK_SIZE],
    i: u16,
    dt: u8,
    st: u8,
    pc: u16,
    sp: u8,
    beep: bool,
    last_key: u8,
    keys: [bool; NUM_KEYS],
}

#[cfg_attr(feature = "web", wasm_bindgen)]
impl Chip8 {
    #[cfg_attr(feature = "web", wasm_bindgen(constructor))]
    pub fn new() -> Chip8 {
        let mut chip8 = Chip8 {
            vram: [0u8; SCREEN_PIXEL_WIDTH * SCREEN_PIXEL_HEIGHT],
            ram: [0u8; RAM_SIZE],
            registers: [0u8; NUM_REGISTERS],
            stack: [0u16; STACK_SIZE],
            i: 0,
            dt: 0,
            st: 0,
            pc: ROM_START as u16,
            sp: 0,
            beep: false,
            last_key: 0x00,
            keys: [false; NUM_KEYS],
        };
        chip8.load_font(&FONT_SET);
        chip8
    }

    pub fn reset(&mut self) {
        self.vram = [0u8; SCREEN_PIXEL_WIDTH * SCREEN_PIXEL_HEIGHT];
        self.registers = [0u8; NUM_REGISTERS];
        self.stack = [0u16; STACK_SIZE];
        self.i = 0;
        self.dt = 0;
        self.st = 0;
        self.pc = ROM_START as u16;
        self.sp = 0;
        self.beep = false;
        self.last_key = 0x00;
        self.keys = [false; NUM_KEYS];
        self.load_font(&FONT_SET);
    }

    pub fn reset_hard(&mut self) {
        self.ram = [0u8; RAM_SIZE];
        self.reset();
    }

    pub fn pixels(&self) -> Vec<u8> {
        self.vram.to_vec()
    }

    pub fn beep(&self) -> bool {
        self.beep
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.ram[ROM_START..ROM_START + rom.len()].clone_from_slice(rom);
    }

    pub fn clock(&mut self) {
        let opcode = self.fetch_opcode();
        self.process_opcode(opcode);
    }

    pub fn clock_dt(&mut self) {
        self.dt = self.dt.saturating_sub(1);
    }

    pub fn clock_st(&mut self) {
        self.st = self.st.saturating_sub(1);
        self.beep = self.st > 0;
    }

    pub fn key_press(&mut self, key: u8) {
        if key < NUM_KEYS as u8 {
            self.last_key = key;
            self.keys[key as usize] = true;
        }
    }

    pub fn key_lift(&mut self, key: u8) {
        if key < NUM_KEYS as u8 {
            self.keys[key as usize] = false;
        }
    }
}

impl Chip8 {
    fn process_opcode(&mut self, opcode: u16) {
        let id = opcode & 0xf000;
        let addr = opcode & 0x0fff;
        let nibble = (opcode & 0x000f) as u8;
        let x = (opcode >> 8 & 0xf) as usize;
        let y = (opcode >> 4 & 0xf) as usize;
        let byte = (opcode & 0x00ff) as u8;

        match id {
            0x0000 => match byte {
                0xe0 => self.vram = [0u8; SCREEN_PIXEL_WIDTH * SCREEN_PIXEL_HEIGHT],
                0xee => self.return_subroutine(),
                _ => panic!("unknown opcode 0x{:04x}", opcode),
            },
            0x1000 => self.pc = addr,
            0x2000 => self.call_subroutine(addr),
            0x3000 => self.skip_if(self.registers[x] == byte),
            0x4000 => self.skip_if(self.registers[x] != byte),
            0x5000 => self.skip_if(self.registers[x] == self.registers[y]),
            0x6000 => self.registers[x] = byte,
            0x7000 => self.registers[x] = self.registers[x].wrapping_add(byte),
            0x8000 => match nibble {
                0x0 => self.registers[x] = self.registers[y],
                0x1 => self.registers[x] |= self.registers[y],
                0x2 => self.registers[x] &= self.registers[y],
                0x3 => self.registers[x] ^= self.registers[y],
                0x4 => self.add(x, y),
                0x5 => self.registers[x] = self.sub(x, y),
                0x6 => self.shift_right(x),
                0x7 => self.registers[x] = self.sub(y, x),
                0xe => self.shift_left(x),
                _ => panic!("unknown opcode 0x{:04x}", opcode),
            },
            0x9000 => self.skip_if(self.registers[x] != self.registers[y]),
            0xa000 => self.i = addr,
            0xb000 => self.pc = addr + self.registers[0] as u16,
            0xc000 => self.registers[x] = byte & Chip8::rand(),
            0xd000 => self.draw_sprite(
                self.registers[x] as usize,
                self.registers[y] as usize,
                nibble as usize,
            ),
            0xe000 => match byte {
                0x9e => self.skip_if(self.keys[self.registers[x] as usize]),
                0xa1 => self.skip_if(!self.keys[self.registers[x] as usize]),
                _ => panic!("unknown opcode 0x{:04x}", opcode),
            },
            0xf000 => match byte {
                0x07 => self.registers[x] = self.dt,
                0x0a => self.wait_for_key(x),
                0x15 => self.dt = self.registers[x],
                0x18 => self.st = self.registers[x],
                0x1e => self.i += self.registers[x] as u16,
                0x29 => self.i = self.registers[x] as u16 * 5,
                0x33 => self.store_bcd(x),
                0x55 => self.ram[self.i as usize..=self.i as usize + x]
                    .clone_from_slice(&self.registers[0..=x]),
                0x65 => {
                    self.registers[0..=x]
                        .clone_from_slice(&self.ram[self.i as usize..=self.i as usize + x]);
                }
                _ => panic!("unknown opcode 0x{:04x}", opcode),
            },
            _ => panic!("unknown opcode 0x{:04x}", opcode),
        }
    }

    #[inline(always)]
    fn fetch_opcode(&mut self) -> u16 {
        let opcode =
            (self.ram[self.pc as usize] as u16) << 8 | self.ram[self.pc as usize + 1] as u16;
        self.pc += 2;
        opcode
    }

    #[inline(always)]
    fn add(&mut self, x: usize, y: usize) {
        let (sum, overflow) = self.registers[x].overflowing_add(self.registers[y]);
        self.registers[0xf] = overflow as u8;
        self.registers[x] = sum;
    }

    #[inline(always)]
    fn sub(&mut self, x: usize, y: usize) -> u8 {
        self.registers[0xf] = (self.registers[x] > self.registers[y]) as u8;
        self.registers[x].saturating_sub(self.registers[y])
    }
    
    #[inline(always)]
    fn call_subroutine(&mut self, addr: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = addr;
    }

    #[inline(always)]
    fn return_subroutine(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    #[inline(always)]
    fn shift_right(&mut self, x: usize) {
        self.registers[0xf] = self.registers[x] & 0x01;
        self.registers[x] >>= 1;
    }

    #[inline(always)]
    fn shift_left(&mut self, x: usize) {
        self.registers[0xf] = (self.registers[x] & 0x80) >> 7;
        self.registers[x] <<= 1;
    }

    #[inline(always)]
    fn store_bcd(&mut self, x: usize) {
        self.ram[self.i as usize] = self.registers[x] / 100;
        self.ram[self.i as usize + 1] = (self.registers[x] / 10) % 10;
        self.ram[self.i as usize + 2] = self.registers[x] % 10;
    }

    #[inline(always)]
    fn skip_if(&mut self, skip: bool) {
        self.pc += if skip { 2 } else { 0 };
    }

    #[inline(always)]
    fn wait_for_key(&mut self, x: usize) {
        if self.keys[self.last_key as usize] {
            self.registers[x] = self.last_key;
        } else {
            self.pc -= 2;
        }
    }

    #[inline(always)]
    fn draw_sprite(&mut self, x0: usize, y0: usize, height: usize) {
        self.registers[0xf] = 0;
        for y in 0..height {
            let sprite_line = self.ram[self.i as usize + y];
            for x in 0..8 {
                let xf = (x + x0) % SCREEN_PIXEL_WIDTH;
                let yf = (y + y0) % SCREEN_PIXEL_HEIGHT;
                let addr = yf * SCREEN_PIXEL_WIDTH + xf;
                if (sprite_line & (0x80 >> x)) != 0 {
                    if self.vram[addr] == 1 {
                        self.registers[0xf] = 1;
                    }
                    self.vram[addr] ^= 1
                }
            }
        }
    }

    fn load_font(&mut self, font_set: &[u8]) {
        self.ram[..font_set.len()].clone_from_slice(&font_set);
    }

    // rand crate does not compile to wasm32-unknown-unknown
    fn rand() -> u8 {
        let mut n = [0];
        getrandom(&mut n).unwrap();
        n[0]
    }
}

impl Default for Chip8 {
    fn default() -> Chip8 {
        Chip8::new()
    }
}

impl Display for Chip8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PC: {:#0x?}\nSP: {:#0x?}", self.pc, self.sp)
    }
}
