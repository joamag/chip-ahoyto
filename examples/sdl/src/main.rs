use chip_ahoyto::{
    chip8::Chip8, chip8::SCREEN_PIXEL_HEIGHT, chip8::SCREEN_PIXEL_WIDTH, chip8_neo::Chip8Neo,
    util::read_file,
};
use sdl2::{
    audio::AudioCallback, audio::AudioSpecDesired, event::Event, image::LoadSurface,
    keyboard::Keycode, pixels::Color, pixels::PixelFormatEnum, rect::Rect, render::TextureQuery,
    surface::Surface, ttf::Hinting,
};
use std::path::Path;

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

const COLORS: [[u8; 3]; 6] = [
    [255, 255, 255],
    [80, 203, 147],
    [74, 246, 38],
    [255, 0, 0],
    [0, 255, 0],
    [0, 0, 255],
];

const LOGIC_HZ: u32 = 480;
const VISUAL_HZ: u32 = 60;
const IDLE_HZ: u32 = 60;
const TIMER_HZ: u32 = 60;

const BEEP_DURATION: f32 = 0.1;

const LOGIC_DELTA: u32 = VISUAL_HZ;

const SCREEN_SCALE: f32 = 10.0;

/// The name of the font file to be used in the diagnostics.
static FONT_NAME: &'static str = "RobotoMono-Bold.ttf";

/// The size of the font in pixels to be used in the render.
const FONT_SIZE: u16 = 13;

/// The base title to be used in the window.
static TITLE: &'static str = "CHIP-Ahoyto";

/// The title that is going to be presented initially to the user.
static TITLE_INITIAL: &'static str = "CHIP-Ahoyto [Drag and drop the ROM file to play]";

pub struct BeepCallback {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for BeepCallback {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            if self.phase >= 0.0 && self.phase <= 0.5 {
                *x = self.volume;
            } else {
                *x = -self.volume;
            }
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

impl BeepCallback {
    pub fn set_phase_inc(&mut self, phase_inc: f32) {
        self.phase_inc = phase_inc;
    }

    pub fn set_phase(&mut self, phase: f32) {
        self.phase = phase;
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }
}

pub struct State {
    system: Chip8Neo,
    logic_frequency: u32,
    visual_frequency: u32,
    idle_frequency: u32,
    timer_frequency: u32,
    screen_scale: f32,
    beep_duration: f32,
    next_tick_time: u32,
    beep_ticks: u32,
    pixel_color: [u8; 3],
    diag_color: [u8; 3],
    pixel_color_index: u32,
    title: String,
    rom_name: String,
    rom_loaded: bool,
    diag: bool,
}

impl State {
    pub fn set_title(&mut self, title: &String) {
        self.title = title.to_string();
    }
}

fn main() {
    // builds the CHIP-8 machine, this is the instance that
    // is going to logically represent the CHIP-8
    let mut state = State {
        system: Chip8Neo::new(),
        logic_frequency: LOGIC_HZ,
        visual_frequency: VISUAL_HZ,
        idle_frequency: IDLE_HZ,
        timer_frequency: TIMER_HZ,
        screen_scale: SCREEN_SCALE,
        beep_duration: BEEP_DURATION,
        next_tick_time: 0,
        beep_ticks: 0,
        pixel_color: COLORS[0],
        diag_color: COLORS[1],
        pixel_color_index: 0,
        title: String::from(TITLE_INITIAL),
        rom_name: String::from("unloaded"),
        rom_loaded: false,
        diag: false,
    };

    // initializes the SDL sub-system
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let mut timer_subsystem = sdl.timer().unwrap();
    let audio_subsystem = sdl.audio().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    // initialized the fonts context to be used
    // in the loading of fonts
    let ttf_context = sdl2::ttf::init().unwrap();

    // loads the font that is going to be used in the drawing
    // process cycle if necessary
    let mut font = ttf_context
        .load_font(format!("./resources/{}", FONT_NAME), FONT_SIZE)
        .unwrap();
    font.set_style(sdl2::ttf::FontStyle::BOLD);
    font.set_hinting(Hinting::Light);

    // creates the system window that is going to be used to
    // show the emulator and sets it to the central are o screen
    let mut window = video_subsystem
        .window(
            TITLE,
            state.screen_scale as u32 * SCREEN_PIXEL_WIDTH as u32,
            state.screen_scale as u32 * SCREEN_PIXEL_HEIGHT as u32,
        )
        .resizable()
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    // updates the icon of the window to reflect the image
    // and style of the emulator
    let surface = Surface::from_file("./resources/icon.png").unwrap();
    window.set_icon(&surface);

    let mut canvas = window.into_canvas().accelerated().build().unwrap();
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();

    // creates the texture streaming that is going to be used
    // as the target for the pixel buffer
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

    // creates a new audio device and prints the specs for it
    // making sure that a proper beep callback is set
    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None,
    };
    let device = audio_subsystem
        .open_playback(None, &desired_spec, |spec| BeepCallback {
            phase_inc: 440.0 / spec.freq as f32,
            phase: 0.0,
            volume: 0.5,
        })
        .unwrap();

    'main: loop {
        if state.timer_frequency < state.visual_frequency {
            panic!("timer frequency must be higher or equal to visual frequency")
        }

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
                    state.logic_frequency = state.logic_frequency.saturating_add(LOGIC_DELTA);
                    None
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Minus),
                    ..
                } => {
                    state.logic_frequency = state.logic_frequency.saturating_sub(LOGIC_DELTA);
                    None
                }

                Event::KeyDown {
                    keycode: Some(Keycode::O),
                    ..
                } => {
                    state.system.reset();
                    None
                }

                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    state.pixel_color_index = (state.pixel_color_index + 1) % COLORS.len() as u32;
                    state.pixel_color = COLORS[state.pixel_color_index as usize];
                    let diag_color_index = (state.pixel_color_index + 2) % COLORS.len() as u32;
                    state.diag_color = COLORS[diag_color_index as usize];
                    None
                }

                Event::KeyDown {
                    keycode: Some(Keycode::T),
                    ..
                } => {
                    state.diag = !state.diag;
                    None
                }

                Event::DropFile { filename, .. } => {
                    let rom = read_file(&filename);
                    let rom_name = Path::new(&filename).file_name().unwrap().to_str().unwrap();

                    state.system.reset_hard();
                    state.system.load_rom(&rom);

                    state.rom_name = String::from(rom_name);
                    state.rom_loaded = true;

                    state.set_title(&format!("{} [Currently playing: {}]", TITLE, rom_name));

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

        // updates the window tittle according to the specs of the machine
        // and the provided base title
        canvas
            .window_mut()
            .set_title(&format!(
                "{} [{} hz, {} fps]",
                state.title, state.logic_frequency, state.visual_frequency
            ))
            .unwrap();

        // in case the ROM is not loaded we must delay next execution
        // a little bit to avoid extreme CPU usage, at the same the
        // background must be copied to allow resizing of window to
        // be properly handled
        if !state.rom_loaded {
            canvas.copy(&background, None, None).unwrap();
            canvas.present();

            timer_subsystem.delay(1000 / state.idle_frequency);

            continue;
        }

        let current_time = timer_subsystem.ticks();

        if current_time >= state.next_tick_time {
            // allocates space for the variable that is going to control
            // if a new beep was requested by the CHIP-8 logic cycles
            let mut beep = false;

            // calculates the ratio between the logic and the visual frequency
            // to make sure that the proper number of updates are performed
            let logic_visual_ratio = state.logic_frequency / state.visual_frequency;
            for _ in 0..logic_visual_ratio {
                // runs the clock operation in the CHIP-8 system,
                // effectively changing the logic state of the machine
                state.system.clock();
            }

            // calculates the ration between the timer and the visual frequency
            // so that the proper timer updates are rune
            let timer_visual_ratio = state.timer_frequency / state.visual_frequency;
            for _ in 0..timer_visual_ratio {
                // runs the clock for the timers (both sound and delay),
                // after that tries to determine if a beep should be sounded
                state.system.clock_dt();
                state.system.clock_st();
                beep |= state.system.beep();
            }

            // in case a beep has been requested in the logical loop
            // then the audio device is activated for the number of
            // visual ticks associated with the beep duration (in seconds)
            if beep {
                device.resume();
                state.beep_ticks = (state.visual_frequency as f32 * state.beep_duration) as u32;
            }

            // decrements the number of pending beep ticks and checks
            // if the value has reached zero in that case pauses the
            // beep issuing device
            state.beep_ticks = state.beep_ticks.saturating_sub(1);
            if state.beep_ticks == 0 {
                device.pause();
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

            // draws the diagnostics information to the canvas in case the
            // current state is requesting the display of it
            if state.diag {
                let x = 12;
                let mut y = 12;
                let padding = 2;
                let text = format!(
                    "Engine: {}\nROM: {}\nFrequency: {} Hz\nDisplay: {} fps\nPC: 0x{:04x}\nSP: 0x{:04x}",
                    state.system.name(),
                    state.rom_name,
                    state.logic_frequency,
                    state.visual_frequency,
                    state.system.pc(),
                    state.system.sp()
                );

                let text_sequence = text.split("\n");
                for part in text_sequence {
                    let surface = font
                        .render(part)
                        .blended(Color::RGBA(
                            state.diag_color[0],
                            state.diag_color[1],
                            state.diag_color[2],
                            255,
                        ))
                        .unwrap();
                    let texture = texture_creator
                        .create_texture_from_surface(&surface)
                        .unwrap();
                    let TextureQuery { width, height, .. } = texture.query();
                    canvas
                        .copy(&texture, None, Some(rect!(x, y, width, height)))
                        .unwrap();
                    y += height + padding;
                }
            }

            // presents the canvas effectively updating the screen
            // information presented to the user
            canvas.present();

            // updates the next update time reference to the current
            // time so that it can be used from game loop control
            state.next_tick_time = current_time + (1000 / state.visual_frequency);
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
