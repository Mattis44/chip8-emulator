mod instructions;
mod chip8;

use std::fs;
use crate::chip8::Chip8;

fn main() { 
    let mut chip = Chip8::new();
    let rom = read_rom_from("1-chip8-logo.ch8");
    
    chip.load_rom(&*rom).expect("TODO: panic message");
    
    for _ in 0..50 {
        chip.step();
    }
    
    chip.screen().print();
}

pub fn read_rom_from(path: &str) -> Vec<u8> {
    fs::read(path).expect("Can't read ROM")
}