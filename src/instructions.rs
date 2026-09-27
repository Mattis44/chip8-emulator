#[derive(Debug)]
pub enum Instruction {
    Clear, // 00E0
    Jump(u16), // 1NNN
    PutVx{x: u8, kk: u8}, // 6xkk => 6101
    SetI(u16), // Annn => A250
    Display{x: u8, y: u8, n: u8}, // Dxyn - Display n bytes at Vx, Vy
    Unknown(u16), // Not registered
}

impl Instruction {
    pub fn decode(value: u16) -> Instruction {
        let family = value >> 12;

        let nnn = value & 0x0FFF;
        let x = ((value & 0x0F00) >> 8) as u8;
        let y = ((value & 0x00F0) >> 4) as u8;
        let n = (value & 0x000F) as u8;
        let kk = (value & 0x00FF) as u8;

        match family {
            0x0 if value == 0x00E0 => Instruction::Clear,
            0x1 => Instruction::Jump(nnn),
            0x6 => Instruction::PutVx { x, kk },
            0xA => Instruction::SetI(nnn),
            0xD => Instruction::Display { x, n, y },
            _ => Instruction::Unknown(value),
        }
    }
}