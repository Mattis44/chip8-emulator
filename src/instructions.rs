#[derive(Debug)]
pub enum Instruction {
    Clear,                                          // 00E0
    Return,                                         // 00EE
    Jump(u16),                                      // 1NNN
    Call(u16),                                      // 2nnn => 2202
    SkipIfVxEqualKK { x: u8, kk: u8 },              // 3xkk => 362b
    SkipIfVxNotEqualKK { x: u8, kk: u8 },           // 4xkk => 452a
    SkipIfVxEqualVy { x: u8, y: u8 },               // 5xy0 => 5560
    PutVx { x: u8, kk: u8 },                        // 6xkk => 6101
    AddVx { x: u8, kk: u8 },                        // 7xkk => 7009
    SetVxToVy { x: u8, y: u8 },                     // 8xy0 => 8750
    OrVxVy { x: u8, y: u8 },                        // 8XY1 : Vx |= Vy
    AndVxVy { x: u8, y: u8 },                       // 8XY2 : Vx &= Vy
    XorVxVy { x: u8, y: u8 },                       // 8XY3 : Vx ^= Vy
    AddVxVy { x: u8, y: u8 },                       // 8XY4 : Vx += Vy
    SubVxVy { x: u8, y: u8 },                       // 8XY5 : Vx -= Vy
    ShiftRight { x: u8, y: u8 },                    // 8XY6 : Vx >>= 1
    SubnVyVx { x: u8, y: u8 },                      // 8XY7 : Vx = Vy - Vx
    ShiftLeft { x: u8, y: u8 },                     // 8XYE : Vx <<= 1
    SkipIfVxNotEqualVy { x: u8, y: u8 },            // 9xy0 => 9560
    SetI(u16),                                      // Annn => A250
    JumpPlusV0(u16),                                // Bnnn => BE00
    Random { x: u8, kk: u8 },                       // CXNN => Random
    Display { x: u8, y: u8, n: u8 },                // Dxyn - Display n bytes at Vx, Vy
    SkipIfKeyPressed(u8),                           // Ex9E => E49E
    SkipIfKeyNotPressed(u8),                        // EXA1
    WaitForKeyPressed(u8),                          // FX0A
    AddToI(u8),                                     // FX1E : I += Vx
    SetVxToDelay(u8),                               // Fx07 => F407
    SetDelayToVx(u8),                               // Fx15 => F315
    SetSoundToVx(u8),                               // FX18
    SetIToFont(u8),                                 // FX29
    StoreBcd(u8),                                   // FX33
    StoreRegisters(u8),                             // FX55 : memory[I..] = V0..=Vx
    LoadRegisters(u8),                              // FX65 : V0..=Vx = memory[I..]
    Unknown(u16),                                   // Not registered
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
            0x0 => match value {
                0x00E0 => Instruction::Clear,
                0x00EE => Instruction::Return,
                _ => Instruction::Unknown(value),
            },
            0x1 => Instruction::Jump(nnn),
            0x2 => Instruction::Call(nnn),
            0x3 => Instruction::SkipIfVxEqualKK { x, kk },
            0x4 => Instruction::SkipIfVxNotEqualKK { x, kk },
            0x5 => Instruction::SkipIfVxEqualVy { x, y },
            0x6 => Instruction::PutVx { x, kk },
            0x7 => Instruction::AddVx {x, kk },
            0x8 => match n {
                0x0 => Instruction::SetVxToVy { x, y },
                0x1 => Instruction::OrVxVy { x, y },
                0x2 => Instruction::AndVxVy { x, y },
                0x3 => Instruction::XorVxVy { x, y },
                0x4 => Instruction::AddVxVy { x, y },
                0x5 => Instruction::SubVxVy { x, y },
                0x6 => Instruction::ShiftRight { x, y },
                0x7 => Instruction::SubnVyVx { x, y },
                0xE => Instruction::ShiftLeft { x, y },
                _ => Instruction::Unknown(value),
            },
            0x9 => Instruction::SkipIfVxNotEqualVy { x, y },
            0xA => Instruction::SetI(nnn),
            0xB => Instruction::JumpPlusV0(nnn),
            0xC => Instruction::Random { x, kk },
            0xD => Instruction::Display { x, n, y },
            0xE => match kk {
                0x9E => Instruction::SkipIfKeyPressed(x),
                0xA1 => Instruction::SkipIfKeyNotPressed(x),
                _ => Instruction::Unknown(value),
            }
            0xF => match kk {
                0x0A => Instruction::WaitForKeyPressed(x),
                0x1E => Instruction::AddToI(x),
                0x07 => Instruction::SetVxToDelay(x),
                0x15 => Instruction::SetDelayToVx(x),
                0x18 => Instruction::SetSoundToVx(x),
                0x29 => Instruction::SetIToFont(x),
                0x33 => Instruction::StoreBcd(x),
                0x55 => Instruction::StoreRegisters(x),
                0x65 => Instruction::LoadRegisters(x),
                _ => Instruction::Unknown(value),
            },
            _ => Instruction::Unknown(value),
        }
    }
}
