use crate::instructions::Instruction;

const START_ADDR: usize = 0x200;
const MEMORY_SIZE: usize = 4096;
const MAX_ROM_SIZE: usize = MEMORY_SIZE - START_ADDR;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

const FONTSET_START_ADDRESS: usize = 0x50;

pub struct Screen {
    pixels: [bool; WIDTH * HEIGHT],
}

impl Screen {
    pub fn new() -> Self {
        Self {
            pixels: [false; WIDTH * HEIGHT],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.pixels[y * WIDTH + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        self.pixels[y * WIDTH + x] = value;
    }

    pub fn toggle_pixel(&mut self, x: usize, y: usize) -> bool {
        let idx = (y % HEIGHT) * WIDTH + (x % WIDTH);
        let prev = self.pixels[idx];
        self.pixels[idx] ^= true;

        prev
    }

    pub fn clear(&mut self) {
        self.pixels = [false; WIDTH * HEIGHT]
    }

    pub fn print(&self) {
        for y in 0..HEIGHT {
            let line: String = (0..WIDTH)
                .map(|x| if self.get(x, y) { '#' } else { ' ' })
                .collect();
            println!("{}", line);
        }
    }
}

pub struct Chip8 {
    memory: [u8; MEMORY_SIZE],
    register: [u8; 16],
    i: u16, // store memory addresses
    pc: u16, // program counter => currently executing address
    stack: [u16; 16],
    sp: u8, // stack pointer => point to the topmost level of the stack
    delay_timer: u8,
    sound_timer: u8,
    screen: Screen, // 64x32
}

#[derive(Debug)]
pub enum Chip8Error {
    RomTooLarge { size: usize, max_size: usize },
}

impl Chip8 {
    pub fn new() -> Self {
        let mut memory = [0u8; MEMORY_SIZE];
        memory[FONTSET_START_ADDRESS..FONTSET_START_ADDRESS + FONTSET.len()]
            .copy_from_slice(&FONTSET);
        Self {
            memory,
            register: [0; 16],
            i: 0,
            pc: START_ADDR as u16,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            screen: Screen::new(),
        }
    }
    
    pub fn screen(&self) -> &Screen {
        &self.screen
    }

    pub fn load_rom(&mut self, rom: &[u8]) -> Result<(), Chip8Error> {
        if rom.len() > MAX_ROM_SIZE {
            return Err(Chip8Error::RomTooLarge {
                size: rom.len(),
                max_size: MAX_ROM_SIZE,
            });
        }
        self.memory[START_ADDR..START_ADDR + rom.len()]
            .copy_from_slice(rom);
        Ok(())
    }

    pub fn draw_sprite(&mut self, x: u8, y: u8, n: u8) {
        let start_x = self.register[x as usize] as usize % WIDTH;
        let start_y = self.register[y as usize] as usize % HEIGHT;

        self.register[0xF] = 0;

        for row in 0..n as usize {
            let py = start_y + row;
            if py >= HEIGHT {
                break;
            }
            let sprite_byte = self.memory[self.i as usize + row];

            for col in 0..8 {
                let px = start_x + col;
                if px >= WIDTH {
                    break;
                }
                let bit = (sprite_byte >> (7 - col)) & 1;

                if bit == 1 && self.screen.toggle_pixel(px, py) {
                    self.register[0xF] = 1;
                }
            }
        }
    }

    pub fn fetch(&self) -> u16 {
        let pc_index = self.pc as usize;
        ((self.memory[pc_index] as u16) << 8) | self.memory[pc_index + 1] as u16
    }

    pub fn execute(&mut self, instr: Instruction) {
        match instr {
            Instruction::Clear => self.screen.clear(),
            Instruction::Jump(nnn) => self.pc = nnn,
            Instruction::PutVx {x, kk} => self.register[x as usize] = kk,
            Instruction::SetI(nnn) => self.i = nnn,
            Instruction::Display { x, y, n } => self.draw_sprite(x, y, n),
            Instruction::Unknown(value) => { panic!("unknown upcode : {:04x}", value) }
        }
    }
    
    pub fn step(&mut self) {
        let op_code = self.fetch();
        self.pc += 2;
        
        let instr = Instruction::decode(op_code);
        
        self.execute(instr);
    }
}