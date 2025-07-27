use crate::chip8::Chip8;
use crate::chip8::Chip8::VIDEO_HEIGHT;
use crate::chip8::Chip8::VIDEO_WIDTH;
use std::env;
extern crate sdl2;

mod chip8;

use sdl2::libc::exit;
use sdl2::{Sdl, VideoSubsystem};
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator, Texture};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;


fn main() {
    // Get the arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <Scale> <Delay> <ROM>", args[0]);
        std::process::exit(1);
    }

    let scale = args[1].parse::<u32>().unwrap();
    let delay = args[2].parse::<u32>().unwrap();
    let rom = &args[3];

    // Initialize SDL2
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();
    let window = video.window("Chip-8 Emulator",
                                    VIDEO_WIDTH as u32 * scale,
                                    VIDEO_WIDTH as u32 * scale)
                                .position_centered()
                                .build()
                                .unwrap();
    let mut canvas = window.into_canvas().accelerated().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_streaming(
                                                Some(sdl2::pixels::PixelFormatEnum::RGBA8888),
                                                VIDEO_WIDTH as u32,
                                                VIDEO_HEIGHT as u32
                                            ).unwrap();
    let mut event_pump = context.event_pump().unwrap();

    // Intiialize Chip8
    let mut chip8 = Chip8::Chip8::new();
    chip8.load_rom(rom);

    // Loop
    'running: loop {
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
