extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::{
    env, fs,
    time::{Duration, Instant},
};

use chip8_rust::chip8::Chip8;
use chip8_rust::chip8::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

const SCALE_FACTOR: u32 = 10; // Increase this for a larger window
const WINDOW_WIDTH: u32 = DISPLAY_WIDTH as u32 * SCALE_FACTOR;
const WINDOW_HEIGHT: u32 = DISPLAY_HEIGHT as u32 * SCALE_FACTOR;

const CYCLES_PER_SECOND: u32 = 5400;
const TARGET_FPS: u64 = 60;
const MICROSECONDS_PER_FRAME: u64 = 1_000_000 / TARGET_FPS;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <rom_file>", args[0]);
        std::process::exit(1);
    }
    let rom_path = &args[1];
    let rom_data = fs::read(rom_path).expect("Failed to read ROM file");

    let mut chip8 = Chip8::new();
    chip8.load_rom(&rom_data);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("CHIP-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let cycles_per_frame = CYCLES_PER_SECOND / TARGET_FPS as u32;
    let mut last_frame_time = Instant::now();

    'running: loop {
        // --- Event Handling ---
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown { keycode, .. } => {
                    if let Some(chip8_key) = map_key(keycode) {
                        if chip8.is_waiting_for_key() {
                            chip8.resolve_key_wait(chip8_key);
                        } else {
                            chip8.keypad.set_key_pressed(chip8_key, true);
                        }
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(chip8_key) = map_key(keycode) {
                        if !chip8.is_waiting_for_key() && chip8.is_waiting_for_release() {
                            chip8.resolve_key_release();
                        }
                        chip8.keypad.set_key_pressed(chip8_key, false);
                    }
                }
                _ => {}
            }
        }

        // --- Frame Rate Control ---
        let elapsed = last_frame_time.elapsed();
        if elapsed < Duration::from_micros(MICROSECONDS_PER_FRAME) {
            std::thread::sleep(Duration::from_micros(MICROSECONDS_PER_FRAME) - elapsed);
        }
        last_frame_time = Instant::now();

        // --- CPU Emulation ---
        if !chip8.is_waiting_for_key() && !chip8.is_waiting_for_release() {
            for cycle_idx in 0..cycles_per_frame {
                chip8.emulate_cycle(cycle_idx == 0);
                if chip8.is_waiting_for_key() {
                    break;
                }
            }
        }

        // Update Timers
        if chip8.delay_timer > 0 {
            chip8.delay_timer -= 1;
        }

        if chip8.sound_timer > 0 {
            chip8.sound_timer -= 1;
        }

        // --- Drawing ---
        draw_screen(&chip8, &mut canvas)?;
    }

    Ok(())
}

fn draw_screen(
    chip8: &Chip8,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
) -> Result<(), String> {
    let display_buffer = chip8.display.get_buffer();

    // Clear the screen with a background color
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // Set the drawing color for the "on" pixels
    canvas.set_draw_color(Color::RGB(0, 255, 0));

    // Iterate through the CHIP-8 buffer and draw rectangles for each "on" pixel
    for (y, row) in display_buffer.iter().enumerate() {
        for (x, &pixel) in row.iter().enumerate() {
            if pixel {
                let rect = Rect::new(
                    (x as u32 * SCALE_FACTOR) as i32,
                    (y as u32 * SCALE_FACTOR) as i32,
                    SCALE_FACTOR,
                    SCALE_FACTOR,
                );
                canvas.fill_rect(rect)?;
            }
        }
    }

    canvas.present();
    Ok(())
}

fn map_key(keycode: Option<Keycode>) -> Option<u8> {
    match keycode {
        Some(Keycode::Num1) => Some(0x1),
        Some(Keycode::Num2) => Some(0x2),
        Some(Keycode::Num3) => Some(0x3),
        Some(Keycode::Num4) => Some(0xC),

        Some(Keycode::Q) => Some(0x4),
        Some(Keycode::W) => Some(0x5),
        Some(Keycode::E) => Some(0x6),
        Some(Keycode::R) => Some(0xD),

        Some(Keycode::A) => Some(0x7),
        Some(Keycode::S) => Some(0x8),
        Some(Keycode::D) => Some(0x9),
        Some(Keycode::F) => Some(0xE),

        Some(Keycode::Z) => Some(0xA),
        Some(Keycode::X) => Some(0x0),
        Some(Keycode::C) => Some(0xB),
        Some(Keycode::V) => Some(0xF),

        _ => None, // Any other key is not mapped
    }
}
