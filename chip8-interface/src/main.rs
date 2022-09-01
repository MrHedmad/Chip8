use std::env;
use std::fs::File;
use std::io::Read;

use chip8_emulator::*;
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (DISPLAY_MEM_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (DISPLAY_MEM_HEIGHT as u32) * SCALE;
const CYCLES_PER_FRAME: usize = 10;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 { // Remember that the first item is the path to the binary
        println!("Invalid number of args\nUsage: cargo run <path>");
        return ;
    }

    // Setup SDL window
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Chip8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut processor = Chip8Processor::new();

    let mut rom = File::open(&args[1]).expect("Unable to open file.");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();

    processor.load_rom(&buffer);

    // This is a loop label that we can use to break out of tiered loops.
    'gameloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'gameloop;
                },
                Event::KeyDown { keycode: Some(key), .. } => {
                    if let Some(chip_key) = key_to_chip8_key(key) {
                        processor.press_key(chip_key);
                    }
                },
                Event::KeyUp { keycode: Some(key), .. } => {
                    if let Some(chip_key) = key_to_chip8_key(key) {
                        processor.release_key(chip_key);
                    }
                }

                _ => ()
            }
        }

        for _ in 0..CYCLES_PER_FRAME {
            processor.cycle();
        }
        processor.tick_timers();
        draw_screen(&processor, &mut canvas);
    }

}


fn draw_screen(processor: &Chip8Processor, canvas: &mut Canvas<Window>) {
    // Clear the canvas
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buffer = processor.get_display();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, pixel) in screen_buffer.iter().enumerate() {
        if *pixel {
            // Make the 1D array 2D. We get the coordinates of the pixel we are
            // iterating upon.
            let x = (i % DISPLAY_MEM_HEIGHT) as u32;
            let y = (i / DISPLAY_MEM_WIDTH) as u32;

            let rectangle = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rectangle).unwrap();
        }
    }

    canvas.present();
}

fn key_to_chip8_key(key: Keycode) -> Option<Chip8Key> {
    match key {
        Keycode::Num1 => Some(Chip8Key::K1),
        Keycode::Num2 => Some(Chip8Key::K2),
        Keycode::Num3 => Some(Chip8Key::K3),
        Keycode::Num4 => Some(Chip8Key::KC),
        Keycode::Q => Some(Chip8Key::K4),
        Keycode::W => Some(Chip8Key::K5),
        Keycode::E => Some(Chip8Key::K6),
        Keycode::R => Some(Chip8Key::KD),
        Keycode::A => Some(Chip8Key::K7),
        Keycode::S => Some(Chip8Key::K8),
        Keycode::D => Some(Chip8Key::K9),
        Keycode::F => Some(Chip8Key::KE),
        Keycode::Z => Some(Chip8Key::KA),
        Keycode::X => Some(Chip8Key::K0),
        Keycode::C => Some(Chip8Key::KB),
        Keycode::V => Some(Chip8Key::KF),
        _ => None,
    }
}