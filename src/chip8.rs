pub trait Chip8 {
    fn name(&self) -> &str;
    fn reset(&mut self);
    fn reset_hard(&mut self);
    fn beep(&self) -> bool;
    fn pc(&self) -> u16;
    fn sp(&self) -> u8;
    fn ram(&self) -> Vec<u8>;
    fn vram(&self) -> Vec<u8>;
    fn get_state(&self) -> Vec<u8>;
    fn set_state(&mut self, state: &[u8]);
    fn load_rom(&mut self, rom: &[u8]);
    fn clock(&mut self);
    fn clock_dt(&mut self);
    fn clock_st(&mut self);
    fn key_press(&mut self, key: u8);
    fn key_lift(&mut self, key: u8);
}
