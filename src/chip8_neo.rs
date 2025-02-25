use std::io::{Cursor, Read};

use crate::{
    chip8::{Chip8, Quirk, DISPLAY_HEIGHT, DISPLAY_WIDTH, FONT_SET},
    clipping, display_blank, jumping, memory, shifting,
    util::random,
    vf_reset,
};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use crate::chip8::Registers;

const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const REGISTERS_SIZE: usize = 16;
const KEYS_SIZE: usize = 16;

/// The starting address for the ROM loading, should be
/// the initial PC position for execution.
const ROM_START: usize = 0x200;

#[derive(PartialEq)]
enum WaitVblank {
    NotWaiting,
    Waiting,
    Vblank,
}

pub struct QuirkFlags {
    vf_reset: bool,
    memory: bool,
    display_blank: bool,
    clipping: bool,
    shifting: bool,
    jumping: bool,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
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
    keys: [bool; KEYS_SIZE],
    last_key: u8,
    paused: bool,
    wait_vblank: WaitVblank,
    quirks: QuirkFlags,
}

impl Chip8 for Chip8Neo {
    fn name(&self) -> &str {
        "neo"
    }

    fn reset(&mut self) {
        self.vram = [0u8; DISPLAY_WIDTH * DISPLAY_HEIGHT];
        self.stack = [0u16; STACK_SIZE];
        self.regs = [0u8; REGISTERS_SIZE];
        self.pc = ROM_START as u16;
        self.i = 0x0;
        self.sp = 0x0;
        self.dt = 0x0;
        self.st = 0x0;
        self.keys = [false; KEYS_SIZE];
        self.last_key = 0x0;
        self.paused = false;
        self.wait_vblank = WaitVblank::Waiting;
        self.load_default_font();
    }

    fn reset_hard(&mut self) {
        self.ram = [0u8; RAM_SIZE];
        self.reset();
    }

    fn pause(&mut self) {
        if cfg!(feature = "quirks") {
            self.paused = true;
        } else {
            panic!("no pause support available");
        }
    }

    fn paused(&self) -> bool {
        self.paused
    }

    fn beep(&self) -> bool {
        self.st > 0
    }

    fn pc(&self) -> u16 {
        self.pc
    }

    fn sp(&self) -> u8 {
        self.sp
    }

    fn ram(&self) -> Vec<u8> {
        self.ram.to_vec()
    }

    fn vram(&self) -> Vec<u8> {
        self.vram.to_vec()
    }

    fn set_quirk(&mut self, quirk: Quirk, value: bool) {
        match quirk {
            Quirk::VfReset => self.quirks.vf_reset = value,
            Quirk::Memory => self.quirks.memory = value,
            Quirk::DisplayBlank => self.quirks.display_blank = value,
            Quirk::Clipping => self.quirks.clipping = value,
            Quirk::Shifting => self.quirks.shifting = value,
            Quirk::Jumping => self.quirks.jumping = value,
        }
    }

    fn get_state(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend(self.ram.iter());
        buffer.extend(self.vram.iter());
        buffer.extend(self.stack.map(|v| v.to_le_bytes()).iter().flatten());
        buffer.extend(self.regs.iter());
        buffer.extend(self.pc.to_le_bytes().iter());
        buffer.extend(self.i.to_le_bytes().iter());
        buffer.extend(self.sp.to_le_bytes().iter());
        buffer.extend(self.dt.to_le_bytes().iter());
        buffer.extend(self.st.to_le_bytes().iter());
        buffer.extend(self.keys.map(|v| v as u8).iter());
        buffer.extend(self.last_key.to_le_bytes().iter());
        buffer
    }

    fn set_state(&mut self, state: &[u8]) {
        let mut u8_buffer = [0u8; 1];
        let mut u16_buffer = [0u8; 2];
        let mut regs_buffer = [0u8; REGISTERS_SIZE * 2];
        let mut keys_buffer = [0u8; KEYS_SIZE];

        let mut cursor = Cursor::new(state.to_vec());

        cursor.read_exact(&mut self.ram).unwrap();
        cursor.read_exact(&mut self.vram).unwrap();
        cursor.read_exact(&mut regs_buffer).unwrap();
        self.stack.clone_from_slice(
            regs_buffer
                .chunks(2)
                .map(|v| {
                    u16_buffer.clone_from_slice(&v[0..2]);
                    u16::from_le_bytes(u16_buffer)
                })
                .collect::<Vec<u16>>()
                .as_slice(),
        );
        cursor.read_exact(&mut self.regs).unwrap();
        cursor.read_exact(&mut u16_buffer).unwrap();
        self.pc = u16::from_le_bytes(u16_buffer);
        cursor.read_exact(&mut u16_buffer).unwrap();
        self.i = u16::from_le_bytes(u16_buffer);
        cursor.read_exact(&mut u8_buffer).unwrap();
        self.sp = u8::from_le_bytes(u8_buffer);
        cursor.read_exact(&mut u8_buffer).unwrap();
        self.dt = u8::from_le_bytes(u8_buffer);
        cursor.read_exact(&mut u8_buffer).unwrap();
        self.st = u8::from_le_bytes(u8_buffer);
        cursor.read_exact(&mut keys_buffer).unwrap();
        self.keys
            .clone_from_slice(keys_buffer.map(|v| v == 1).iter().as_slice());
        cursor.read_exact(&mut u8_buffer).unwrap();
        self.last_key = u8::from_le_bytes(u8_buffer);
    }

    fn load_rom(&mut self, rom: &[u8]) {
        self.ram[ROM_START..ROM_START + rom.len()].clone_from_slice(rom);
    }

    fn clock(&mut self) {
        if cfg!(feature = "quirks") {
            // in case the CPU is currently in the paused state
            // the control flow is immediately returned as there's
            // nothing pending to be done
            if self.paused {
                return;
            }
        }

        // fetches the current instruction and increments
        // the PC (program counter) accordingly
        let instruction =
            ((self.ram[self.pc as usize] as u16) << 8) | self.ram[self.pc as usize + 1] as u16;
        self.pc += 2;

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
                _ => panic!("unimplemented instruction 0x0000, instruction 0x{instruction:04x}"),
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
                0x1 => {
                    self.regs[x] |= self.regs[y];
                    vf_reset!(self);
                }
                0x2 => {
                    self.regs[x] &= self.regs[y];
                    vf_reset!(self);
                }
                0x3 => {
                    self.regs[x] ^= self.regs[y];
                    vf_reset!(self);
                }
                0x4 => {
                    let (result, overflow) = self.regs[x].overflowing_add(self.regs[y]);
                    self.regs[x] = result;
                    self.regs[0xf] = overflow as u8;
                }
                0x5 => {
                    self.regs[0xf] = (self.regs[x] > self.regs[y]) as u8;
                    self.regs[x] = self.regs[x].wrapping_sub(self.regs[y]);
                }
                0x6 => {
                    self.regs[0xf] = self.regs[x] & 0x01;
                    shifting!(self, x, y, >>);
                }
                0x7 => {
                    self.regs[0xf] = (self.regs[y] > self.regs[x]) as u8;
                    self.regs[x] = self.regs[y].wrapping_sub(self.regs[x]);
                }
                0xe => {
                    self.regs[0xf] = (self.regs[x] & 0x80) >> 7;
                    shifting!(self, x, y, <<);
                }
                _ => panic!("unimplemented instruction 0x8000, instruction 0x{instruction:04x}"),
            },
            0x9000 => self.pc += if self.regs[x] != self.regs[y] { 2 } else { 0 },
            0xa000 => self.i = address,
            0xb000 => {
                jumping!(self, address, x);
            }
            0xc000 => self.regs[x] = byte & random(),
            0xd000 => {
                self.draw_sprite(
                    self.i as usize,
                    self.regs[x] as usize,
                    self.regs[y] as usize,
                    nibble as usize,
                );
            }
            0xe000 => match byte {
                0x9e => {
                    let key = self.regs[x] as usize;
                    self.pc += if self.keys[key] { 2 } else { 0 }
                }
                0xa1 => {
                    let key = self.regs[x] as usize;
                    self.pc += if !self.keys[key] { 2 } else { 0 }
                }
                _ => panic!("unimplemented instruction 0xe000, instruction 0x{instruction:04x}"),
            },
            0xf000 => match byte {
                0x07 => self.regs[x] = self.dt,
                0x0a => {
                    if self.keys[self.last_key as usize] {
                        self.regs[x] = self.last_key;
                    } else {
                        self.pc -= 2
                    }
                }
                0x15 => self.dt = self.regs[x],
                0x18 => self.st = self.regs[x],
                0x1e => self.i = self.i.saturating_add(self.regs[x] as u16),
                0x29 => self.i = self.regs[x] as u16 * 5,
                0x33 => {
                    self.ram[self.i as usize] = self.regs[x] / 100;
                    self.ram[self.i as usize + 1] = (self.regs[x] / 10) % 10;
                    self.ram[self.i as usize + 2] = self.regs[x] % 10;
                }
                0x55 => {
                    self.ram[self.i as usize..self.i as usize + x + 1]
                        .clone_from_slice(&self.regs[0..x + 1]);
                    memory!(self);
                }
                0x65 => {
                    self.regs[0..x + 1]
                        .clone_from_slice(&self.ram[self.i as usize..self.i as usize + x + 1]);
                    memory!(self);
                }
                _ => panic!("unimplemented instruction 0xf000, instruction 0x{instruction:04x}",),
            },
            _ => panic!("unimplemented opcode 0x{opcode:04x}, instruction 0x{instruction:04x}",),
        }
    }

    fn clock_dt(&mut self) {
        self.dt = self.dt.saturating_sub(1)
    }

    fn clock_st(&mut self) {
        self.st = self.st.saturating_sub(1)
    }

    fn key_press(&mut self, key: u8) {
        if key >= KEYS_SIZE as u8 {
            return;
        }
        self.keys[key as usize] = true;
        self.last_key = key;
    }

    fn key_lift(&mut self, key: u8) {
        if key >= KEYS_SIZE as u8 {
            return;
        }
        self.keys[key as usize] = false;
    }

    fn vblank(&mut self) {
        if self.wait_vblank == WaitVblank::Waiting {
            self.wait_vblank = WaitVblank::Vblank;
            self.paused = false;
        }
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Chip8Neo {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new() -> Self {
        let mut chip8 = Self {
            ram: [0u8; RAM_SIZE],
            vram: [0u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            stack: [0u16; STACK_SIZE],
            regs: [0u8; REGISTERS_SIZE],
            pc: ROM_START as u16,
            i: 0x0,
            sp: 0x0,
            dt: 0x0,
            st: 0x0,
            keys: [false; KEYS_SIZE],
            last_key: 0x0,
            paused: false,
            wait_vblank: WaitVblank::NotWaiting,
            quirks: QuirkFlags {
                vf_reset: true,
                memory: true,
                display_blank: false,
                clipping: true,
                shifting: false,
                jumping: false,
            },
        };
        chip8.load_default_font();
        chip8
    }

    fn load_font(&mut self, position: usize, font_set: &[u8]) {
        self.ram[position..position + font_set.len()].clone_from_slice(font_set);
    }

    fn load_default_font(&mut self) {
        self.load_font(0, &FONT_SET);
    }

    #[inline(always)]
    fn clear_screen(&mut self) {
        self.vram = [0u8; DISPLAY_WIDTH * DISPLAY_HEIGHT];
    }

    #[inline(always)]
    fn draw_sprite(&mut self, addr: usize, x0: usize, y0: usize, height: usize) {
        let mut yf;
        display_blank!(self);
        self.regs[0xf] = 0;
        for y in 0..height {
            let line_byte = self.ram[addr + y];
            for x in 0..8 {
                if line_byte & (0x80 >> x) == 0 {
                    continue;
                }
                clipping!(self, y, y0, yf);
                let xf = (x0 + x) % DISPLAY_WIDTH;
                let addr = yf * DISPLAY_WIDTH + xf;
                if self.vram[addr] == 1 {
                    self.regs[0xf] = 1;
                }
                self.vram[addr] ^= 1
            }
        }
    }

    #[cfg(feature = "quirks")]
    fn pause_vblank(&mut self) {
        self.paused = true;
        self.wait_vblank = WaitVblank::Waiting;
        self.pc -= 2;
    }
}

#[cfg(feature = "wasm")]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Chip8Neo {
    pub fn registers_wa(&mut self) -> Registers {
        Registers {
            pc: self.pc,
            i: self.i,
            sp: self.sp,
            dt: self.dt,
            st: self.st,
        }
    }

    pub fn get_state_wa(&self) -> Vec<u8> {
        self.get_state()
    }

    pub fn set_state_wa(&mut self, state: &[u8]) {
        self.set_state(state)
    }

    pub fn load_rom_wa(&mut self, rom: &[u8]) {
        self.load_rom(rom)
    }

    pub fn reset_wa(&mut self) {
        self.reset()
    }

    pub fn reset_hard_wa(&mut self) {
        self.reset_hard()
    }

    pub fn pause_wa(&mut self) {
        self.pause()
    }

    pub fn paused_wa(&mut self) -> bool {
        self.paused()
    }

    pub fn beep_wa(&self) -> bool {
        self.beep()
    }

    pub fn vram_wa(&self) -> Vec<u8> {
        self.vram()
    }

    pub fn clock_wa(&mut self) {
        self.clock()
    }

    pub fn clock_dt_wa(&mut self) {
        self.clock_dt()
    }

    pub fn clock_st_wa(&mut self) {
        self.clock_st()
    }

    pub fn key_press_wa(&mut self, key: u8) {
        self.key_press(key)
    }

    pub fn key_lift_wa(&mut self, key: u8) {
        self.key_lift(key)
    }

    pub fn vblank_wa(&mut self) {
        self.vblank()
    }
}

impl Default for Chip8Neo {
    fn default() -> Self {
        Self::new()
    }
}
