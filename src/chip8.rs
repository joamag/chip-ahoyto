/// The width of the screen in pixels.
pub const DISPLAY_WIDTH: usize = 64;

/// The height of the screen in pixels.
pub const DISPLAY_HEIGHT: usize = 32;

/// Buffer that contains the base CHIP-8 font set that
/// is going to be used to draw the font in the screen.
pub static FONT_SET: [u8; 80] = [
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

pub enum Quirk {
    VfReset,
    Memory,
    DisplayBlank,
    Clipping,
    Shifting,
    Jumping,
}

pub trait Chip8 {
    fn name(&self) -> &str;
    fn reset(&mut self);
    fn reset_hard(&mut self);
    fn pause(&mut self);
    fn paused(&self) -> bool;
    fn beep(&self) -> bool;
    fn pc(&self) -> u16;
    fn sp(&self) -> u8;
    fn ram(&self) -> Vec<u8>;
    fn vram(&self) -> Vec<u8>;
    fn set_quirk(&mut self, quirk: Quirk, value: bool);
    fn get_state(&self) -> Vec<u8>;
    fn set_state(&mut self, state: &[u8]);
    fn load_rom(&mut self, rom: &[u8]);
    fn clock(&mut self);
    fn clock_dt(&mut self);
    fn clock_st(&mut self);
    fn key_press(&mut self, key: u8);
    fn key_lift(&mut self, key: u8);
    fn vblank(&mut self);
}
