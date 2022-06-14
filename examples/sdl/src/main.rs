use chip_ahoyto::chip8::{Chip8, SCREEN_PIXEL_HEIGHT, SCREEN_PIXEL_WIDTH};
use sdl2::{
    event::Event, image::LoadSurface, keyboard::Keycode, pixels::PixelFormatEnum, surface::Surface,
};
use std::{fs::File, io::Read};

const COLORS: [[u8; 3]; 2] = [[255, 255, 255], [80, 203, 147]];

const LOGIC_HZ: u32 = 240;
const VISUAL_HZ: u32 = 60;
const IDLE_HZ: u32 = 60;

const LOGIC_DELTA: u32 = 60;

const SCREEN_SCALE: f32 = 15.0;

// The base title to be used in the window.
const TITLE: &str = "CHIP-Ahoyto";

// The title that is going to be presented initially to the user.
const TITLE_INITIAL: &str = "CHIP-Ahoyto [Drag and drop the ROM file to play]";

pub struct State {
    system: Chip8,
    rom_loaded: bool,
    logic_frequency: u32,
    visual_frequency: u32,
    idle_frequency: u32,
    next_tick_time: u32,
    pixel_color: [u8; 3],
}

fn main() {
    // builds the CHIP-8 machine, this is the instance that
    // is going to logically represent the CHIP-8
    let mut state = State {
        system: Chip8::new(),
        rom_loaded: false,
        logic_frequency: LOGIC_HZ,
        visual_frequency: VISUAL_HZ,
        idle_frequency: IDLE_HZ,
        next_tick_time: 0,
        pixel_color: COLORS[0],
    };

    // initializes the SDL sub-system
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let mut timer_subsystem = sdl.timer().unwrap();

    // creates the system window that is going to be used to
    // show the emulator and sets it to the central are o screen
    let mut window = video_subsystem
        .window(
            TITLE_INITIAL,
            SCREEN_SCALE as u32 * SCREEN_PIXEL_WIDTH as u32,
            SCREEN_SCALE as u32 * SCREEN_PIXEL_HEIGHT as u32,
        )
        .resizable()
        .position_centered()
        .build()
        .unwrap();

    // updates the icon of the window to reflect the image
    // and style of the emulator
    let surface = Surface::from_file("./resources/icon.png").unwrap();
    window.set_icon(&surface);

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_scale(SCREEN_SCALE, SCREEN_SCALE).unwrap();
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(
            PixelFormatEnum::RGB24,
            SCREEN_PIXEL_WIDTH as u32,
            SCREEN_PIXEL_HEIGHT as u32,
        )
        .unwrap();

    // creates a texture for the surface and presents it to
    // to the screen creating a call to action to drag and
    // drop the image into the screen
    let background = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();
    canvas.copy(&background, None, None).unwrap();
    canvas.present();

    let mut event_pump = sdl.event_pump().unwrap();

    'main: loop {
        while let Some(event) = event_pump.poll_event() {
            match event {
                Event::Quit { .. } => break 'main,

                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,

                Event::KeyDown {
                    keycode: Some(Keycode::Plus),
                    ..
                } => {
                    state.logic_frequency += LOGIC_DELTA;
                    None
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Minus),
                    ..
                } => {
                    state.logic_frequency -= LOGIC_DELTA;
                    None
                }

                Event::DropFile { filename, .. } => {
                    let rom = read_file(&filename);

                    state.system.reset();
                    state.system.load_rom(&rom);

                    state.rom_loaded = true;

                    canvas
                        .window_mut()
                        .set_title(&format!("{} [Currently playing: {}]", TITLE, filename))
                        .unwrap();
                    None
                }

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } if state.rom_loaded => key_to_btn(keycode).map(|btn| state.system.key_press(btn)),

                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } if state.rom_loaded => key_to_btn(keycode).map(|btn| state.system.key_lift(btn)),

                _ => None,
            };
        }

        // in case the ROM is not loaded we must delay next execution
        // a little bit to avoid extreme CPU usage
        if !state.rom_loaded {
            timer_subsystem.delay(1000 / state.idle_frequency);
            continue;
        }

        if timer_subsystem.ticks() >= state.next_tick_time {
            // calculates the ratio between the logic and the visual frequency
            // to make sure that the proper number of updates are performed
            let logic_visual_ratio = state.logic_frequency / state.visual_frequency;
            for _ in 0..logic_visual_ratio {
                // runs the tick operation in the CHIP-8 system,
                // effectively changing the logic state of the machine
                state.system.clock();
                state.system.clock_dt();
                state.system.clock_st();
            }

            // re-creates a vector of pixels from the system pixels
            // buffer, this is considered a pretty expensive operation
            let mut rgb_pixels = vec![];
            for p in state.system.pixels() {
                rgb_pixels.extend_from_slice(&[
                    p * state.pixel_color[0],
                    p * state.pixel_color[1],
                    p * state.pixel_color[2],
                ])
            }

            // creates a texture based on the RGB pixel buffer
            // and copies that to the canvas for presentation
            texture
                .update(None, &rgb_pixels, SCREEN_PIXEL_WIDTH as usize * 3)
                .unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();

            // updates the next update time reference to the current
            // time so that it can be used from game loop control
            state.next_tick_time = timer_subsystem.ticks() + (1000 / state.visual_frequency);
        }

        let current_time = timer_subsystem.ticks();
        let pending_time = state.next_tick_time.saturating_sub(current_time);
        timer_subsystem.delay(pending_time);
    }
}

fn key_to_btn(keycode: Keycode) -> Option<u8> {
    match keycode {
        Keycode::Num1 => Some(0x01),
        Keycode::Num2 => Some(0x02),
        Keycode::Num3 => Some(0x03),
        Keycode::Num4 => Some(0x0C),
        Keycode::Q => Some(0x04),
        Keycode::W => Some(0x05),
        Keycode::E => Some(0x06),
        Keycode::R => Some(0x0D),
        Keycode::A => Some(0x07),
        Keycode::S => Some(0x08),
        Keycode::D => Some(0x09),
        Keycode::F => Some(0x0E),
        Keycode::Z => Some(0x0A),
        Keycode::X => Some(0x00),
        Keycode::C => Some(0x0B),
        Keycode::V => Some(0x0F),
        _ => None,
    }
}

fn read_file(path: &str) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut rom = Vec::new();
    file.read_to_end(&mut rom).unwrap();
    rom
}
