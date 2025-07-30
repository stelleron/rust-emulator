use crate::chip8::Chip8;
use crate::chip8::Chip8::VIDEO_HEIGHT;
use crate::chip8::Chip8::VIDEO_WIDTH;

extern crate sdl2;

mod chip8;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::env;
use std::time::SystemTime;

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
                                    VIDEO_HEIGHT as u32 * scale)
                                .position_centered()
                                .build()
                                .unwrap();
    let mut canvas = window.into_canvas().accelerated().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0,0));
    canvas.clear();
    canvas.present();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(
                                                Some(sdl2::pixels::PixelFormatEnum::RGBA8888),
                                                VIDEO_WIDTH as u32,
                                                VIDEO_HEIGHT as u32
                                            ).unwrap();
    let mut event_pump = context.event_pump().unwrap();

    // Intiialize Chip8
    let mut chip8 = Chip8::Chip8::new();
    chip8.load_rom(rom);

    // Initialize some variables
    let video_pitch = size_of::<u32>() * VIDEO_WIDTH as usize;
    let mut last_time = SystemTime::now();

    // Loop
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'running,
                Event::KeyDown { keycode, .. } => {
                    if keycode.unwrap() == Keycode::Escape {
                        break 'running
                    } else {
                        chip8.process_input(keycode.unwrap(), true);
                    }
                },
                Event::KeyUp { keycode, .. } => {
                    chip8.process_input(keycode.unwrap(), false);
                },
                _ => {}
            }

            let curr_time = SystemTime::now();
            let dt = curr_time.duration_since(last_time).unwrap().as_millis();

            if dt > delay as u128 {
                last_time = curr_time;
                chip8.cycle();

                let video_bytes: &[u8] = bytemuck::cast_slice(&chip8.video);
                texture.update(None, video_bytes, video_pitch).unwrap();
                canvas.clear();
                canvas.copy(&texture, None,None).unwrap();
                canvas.present();
            }
        }
    }
}
