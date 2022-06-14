use chip_ahoyto::chip8::{Chip8, SCREEN_PIXEL_HEIGHT, SCREEN_PIXEL_WIDTH};
use sdl2::{
    event::Event, image::LoadSurface, keyboard::Keycode, pixels::PixelFormatEnum, surface::Surface,
};
use std::{fs::File, io::Read};

const PIXEL_SET: [u8; 3] = [80, 203, 147];
const SYSTEM_HZ: u32 = 240;
const SCREEN_SCALE: f32 = 15.0;

// The base title to be used in the window.
const TITLE: &str = "CHIP-Ahoyto";

// The title that is going to be presented initially to the user.
const TITLE_INITIAL: &str = "CHIP-Ahoyto [Drag and drop the ROM file to play]";

fn main() {
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

    // builds the CHIP-8 machine, this is the instance that
    // is going to logically represent the CHIP-8
    let mut chip8 = Chip8::new();
    let mut game_loaded = false;

    let tick_interval = 1000 / SYSTEM_HZ;
    let mut last_update_time = 0;
    let mut event_pump = sdl.event_pump().unwrap();

    'main: loop {
        while let Some(event) = event_pump.poll_event() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,

                Event::DropFile { filename, .. } => {
                    let rom = read_file(&filename);
                    chip8 = Chip8::new();
                    chip8.load_rom(&rom);
                    chip8.reset();
                    game_loaded = true;
                    canvas
                        .window_mut()
                        .set_title(&format!("{} [Currently playing: {}]", TITLE, filename))
                        .unwrap();
                    None
                }

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } if game_loaded => key_to_btn(keycode).map(|btn| chip8.key_press(btn)),

                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } if game_loaded => key_to_btn(keycode).map(|btn| chip8.key_lift(btn)),

                _ => None,
            };
        }

        // in case the game is not loaded we must delay next execution
        // a little bit to avoid extreme CPU usage
        if !game_loaded {
            timer_subsystem.delay(17);
            continue;
        }

        let current_time = timer_subsystem.ticks();
        let delta_t = current_time - last_update_time;
        if tick_interval > delta_t {
            // runs the tick operation in the CHIP-8 system,
            // effectively changing the logic state of the machine
            chip8.clock();
            chip8.clock_dt();
            chip8.clock_st();

            // @todo not sure if the delay should come here
            timer_subsystem.delay(tick_interval - delta_t);

            // @todo this looks to be very slow!
            // we should use a callback on pixel buffer change
            // to make this a faster thing
            let mut rgb_pixels = vec![];
            for p in chip8.pixels() {
                rgb_pixels.extend_from_slice(&[
                    p * PIXEL_SET[0],
                    p * PIXEL_SET[1],
                    p * PIXEL_SET[2],
                ])
            }

            // creates a texture based on the RGB pixel buffer
            // and copies that to the canvas for presentation
            texture
                .update(None, &rgb_pixels, SCREEN_PIXEL_WIDTH as usize * 3)
                .unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }

        // updates the last update time reference to the current
        // time so that it can be used from game loop control
        last_update_time = current_time;
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
