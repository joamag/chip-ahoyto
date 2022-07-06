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

/// Enumerates all the quirks available in the
/// CHIP-8 machine.
pub enum Quirk {
    /// Resets the `v[f]` register of the machine
    /// whenever a logic operation is performed.
    VfReset,

    /// Increments the `i` register whenever a copy
    /// operation is performed.
    Memory,

    /// The display blank quirk which enforces
    /// waiting for a screen v-blank operation,
    /// effectively reducing the maximum frequency
    /// of the machine to the frequency of the display
    /// which is typically 60hz.
    DisplayBlank,

    /// Clips the sprite drawing operation around
    /// the Y axis upon overflowing.
    Clipping,

    /// Controls if the `x` register is going to be shifted
    /// by one bit (enabled) or if the `y` register is going
    /// to be shifted and then set in the `x` register.
    Shifting,

    /// If the jump operation is going to be done using the
    /// address plus the `x` register or only the address value.
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
