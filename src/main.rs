mod chip8;
mod instructions;

use crate::chip8::{Chip8, HEIGHT, WIDTH};
use minifb::{Key, Scale, Window, WindowOptions};
use std::fs;

const STEPS_PER_FRAME: usize = 10;

const KEYMAP: [Key; 16] = [
    Key::X,    // 0
    Key::Key1, // 1
    Key::Key2, // 2
    Key::Key3, // 3
    Key::A,    // 4
    Key::Z,    // 5
    Key::E,    // 6
    Key::Q,    // 7
    Key::S,    // 8
    Key::D,    // 9
    Key::W,    // A
    Key::C,    // B
    Key::Key4, // C
    Key::R,    // D
    Key::F,    // E
    Key::V,    // F
];

fn main() {
    let arg = match std::env::args().nth(1) {
        Some(a) => a,
        None => panic!("Missing ROM in arg")
    };

    let mut chip = Chip8::new();
    let rom = read_rom_from(&arg);
    chip.load_rom(&*rom).expect("TODO: panic message");

    let mut window = Window::new(
        "CHIP-8",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X16,
            ..WindowOptions::default()
        },
    )
    .expect("Can't create window");
    window.set_target_fps(60);

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];


    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        for (i, key) in KEYMAP.iter().enumerate() {
            chip.set_key(i, window.is_key_down(*key));
        }
        
        for _ in 0..STEPS_PER_FRAME {
            chip.step();
        }

        chip.tick_timers();

        for (color, &on) in buffer.iter_mut().zip(chip.screen().pixels()) {
            *color = if on { 0xFFFFFF } else { 0x000000 };
        }

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("Can't update window");
    }
}

pub fn read_rom_from(path: &str) -> Vec<u8> {
    fs::read(path).expect("Can't read ROM")
}
