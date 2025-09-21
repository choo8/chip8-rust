use super::register::RegisterIndex;
use super::types::Nibble;

#[derive(Debug)]
pub enum Instruction {
    // 00E0 - CLS
    Clear,
    // 00EE - RET
    Return,
    // 1nnn - JP addr
    Jump(u16),
    // 2nnn - CALL addr
    Call(u16),
    // 3xkk - SE Vx, byte
    SkipEqual(RegisterIndex, u8),
    // 4xkk - SNE Vx, byte
    SkipNotEqual(RegisterIndex, u8),
    // 5xy0 - SE Vx, Vy
    SkipEqualRegister(RegisterIndex, RegisterIndex),
    // 6xkk - LD Vx, byte
    Load(RegisterIndex, u8),
    // 7xkk - ADD Vx, byte
    Add(RegisterIndex, u8),
    // 8xy0 - LD Vx, Vy
    LoadRegister(RegisterIndex, RegisterIndex),
    // 8xy1 - OR Vx, Vy
    LoadOr(RegisterIndex, RegisterIndex),
    // 8xy2 - AND Vx, Vy
    LoadAnd(RegisterIndex, RegisterIndex),
    // 8xy3 - XOR Vx, Vy
    LoadXor(RegisterIndex, RegisterIndex),
    // 8xy4 - ADD Vx, Vy
    LoadAdd(RegisterIndex, RegisterIndex),
    // 8xy5 - SUB Vx, Vy
    LoadSub(RegisterIndex, RegisterIndex),
    // 8xy6 - SHR Vx
    LoadShiftRight(RegisterIndex),
    // 8xy7 - SUBN Vx, Vy
    LoadSubNegative(RegisterIndex, RegisterIndex),
    // 8xyE - SHL Vx
    LoadShiftLeft(RegisterIndex),
    // 9xy0 - SNE Vx, Vy
    SkipNotEqualRegister(RegisterIndex, RegisterIndex),
    // Annn - LD I, addr
    LoadIndexRegister(u16),
    // Bnnn - JP V0, addr
    JumpWithOffset(u16),
    // Cxkk - RND Vx, byte
    Random(RegisterIndex, u8),
    // Dxyn - DRW Vx, Vy, nibble
    Display(RegisterIndex, RegisterIndex, Nibble),
    // Ex9E - SKP Vx
    SkipKeyPress(RegisterIndex),
    // ExA1 - SKNP Vx
    SkipKeyNotPress(RegisterIndex),
    // Fx07 - LD Vx, DT
    LoadDelayTimer(RegisterIndex),
    // Fx0A - LD Vx, K
    LoadKeyPress(RegisterIndex),
    // Fx15 - LD DT, Vx
    StoreDelayTimer(RegisterIndex),
    // Fx18 - LD ST, Vx
    StoreSoundTimer(RegisterIndex),
    // Fx1E - ADD I, Vx
    AddIndexRegister(RegisterIndex),
    // Fx29 - LD F, Vx
    LoadFontCharacter(RegisterIndex),
    // Fx33 - LD B, Vx
    LoadBinaryCodedDecimal(RegisterIndex),
    // Fx55 - LD [I], Vx
    StoreRegisters(RegisterIndex),
    // Fx65 - LD Vx, [I]
    LoadRegisters(RegisterIndex),
}

impl From<u16> for Instruction {
    fn from(raw_instruction: u16) -> Self {
        let opcode = (raw_instruction & 0xF000) >> 12;

        match opcode {
            0x0 => {
                if raw_instruction == 0x00E0 {
                    Instruction::Clear
                } else {
                    Instruction::Return
                }
            }
            0x1 => {
                let nnn = raw_instruction & 0x0FFF;
                Instruction::Jump(nnn)
            }
            0x2 => {
                let nnn = raw_instruction & 0x0FFF;
                Instruction::Call(nnn)
            }
            0x3 => {
                let x = ((raw_instruction & 0x0F00) >> 8) as u8;
                let kk = (raw_instruction & 0x00FF) as u8;
                Instruction::SkipEqual(RegisterIndex::try_from(x).unwrap(), kk)
            }
            0x4 => {
                let x = ((raw_instruction & 0x0F00) >> 8) as u8;
                let kk = (raw_instruction & 0x00FF) as u8;
                Instruction::SkipNotEqual(RegisterIndex::try_from(x).unwrap(), kk)
            }
            0x5 => {
                let x = ((raw_instruction & 0x0F00) >> 8) as u8;
                let y = ((raw_instruction & 0x00F0) >> 4) as u8;
                Instruction::SkipEqualRegister(RegisterIndex::try_from(x).unwrap(), RegisterIndex::try_from(y).unwrap())
            }
            0x6 => {
                let x = ((raw_instruction & 0x0F00) >> 8) as u8;
                let kk = (raw_instruction & 0x00FF) as u8;
                Instruction::Load(RegisterIndex::try_from(x).unwrap(), kk)
            }
            0x7 => {
                let x = ((raw_instruction & 0x0F00) >> 8) as u8;
                let kk = (raw_instruction & 0x00FF) as u8;
                Instruction::Add(RegisterIndex::try_from(x).unwrap(), kk)
            }
            0x8 => {
                let x = ((raw_instruction & 0x0F00) >> 8) as u8;
                let y = ((raw_instruction & 0x00F0) >> 4) as u8;
                let subcode = raw_instruction & 0x000F;
                match subcode {
                    0x0 => Instruction::LoadRegister(RegisterIndex::try_from(x).unwrap(), RegisterIndex::try_from(y).unwrap()),
                    0x1 => Instruction::LoadOr(RegisterIndex::try_from(x).unwrap(), RegisterIndex::try_from(y).unwrap()),
                    0x2 => Instruction::LoadAnd(RegisterIndex::try_from(x).unwrap(), RegisterIndex::try_from(y).unwrap()),
                    0x3 => Instruction::LoadXor(RegisterIndex::try_from(x).unwrap(), RegisterIndex::try_from(y).unwrap()),
                    0x4 => Instruction::LoadAdd(RegisterIndex::try_from(x).unwrap(), RegisterIndex::try_from(y).unwrap()),
                    0x5 => Instruction::LoadSub(RegisterIndex::try_from(x).unwrap(), RegisterIndex::try_from(y).unwrap()),
                    0x6 => Instruction::LoadShiftRight(RegisterIndex::try_from(x).unwrap()),
                    0x7 => Instruction::LoadSubNegative(RegisterIndex::try_from(x).unwrap(), RegisterIndex::try_from(y).unwrap()),
                    0xE => Instruction::LoadShiftLeft(RegisterIndex::try_from(x).unwrap()),
                    _ => panic!("Unknown instruction: {:#X}", raw_instruction)
                }
            }
            0x9 => {
                let x = ((raw_instruction & 0x0F00) >> 8) as u8;
                let y = ((raw_instruction & 0x00F0) >> 4) as u8;
                Instruction::SkipNotEqualRegister(RegisterIndex::try_from(x).unwrap(), RegisterIndex::try_from(y).unwrap())
            }
            0xA => {
                let nnn = raw_instruction & 0x0FFF;
                Instruction::LoadIndexRegister(nnn)
            }
            0xB => {
                let nnn = raw_instruction & 0x0FFF;
                Instruction::JumpWithOffset(nnn)
            }
            0xC => {
                let x = ((raw_instruction & 0x0F00) >> 8) as u8;
                let kk = (raw_instruction & 0x00FF) as u8;
                Instruction::Random(RegisterIndex::try_from(x).unwrap(), kk)
            }
            0xD => {
                let x = ((raw_instruction & 0x0F00) >> 8) as u8;
                let y = ((raw_instruction & 0x00F0) >> 4) as u8;
                let n = (raw_instruction & 0x000F) as u8;
                Instruction::Display(RegisterIndex::try_from(x).unwrap(), RegisterIndex::try_from(y).unwrap(), Nibble::try_from(n).unwrap())
            }
            0xE => {
                let x = ((raw_instruction & 0x0F00) >> 8) as u8;
                let subcode = raw_instruction & 0x00FF;
                match subcode {
                    0x9E => Instruction::SkipKeyPress(RegisterIndex::try_from(x).unwrap()),
                    0xA1 => Instruction::SkipKeyNotPress(RegisterIndex::try_from(x).unwrap()),
                    _ => panic!("Unknown instruction: {:#X}", raw_instruction)
                }
            }
            0xF => {
                let x = ((raw_instruction & 0x0F00) >> 8) as u8;
                let subcode = raw_instruction & 0x00FF;
                match subcode {
                    0x7 => Instruction::LoadDelayTimer(RegisterIndex::try_from(x).unwrap()),
                    0x0A => Instruction::LoadKeyPress(RegisterIndex::try_from(x).unwrap()),
                    0x15 => Instruction::StoreDelayTimer(RegisterIndex::try_from(x).unwrap()),
                    0x18 => Instruction::StoreSoundTimer(RegisterIndex::try_from(x).unwrap()),
                    0x1E => Instruction::AddIndexRegister(RegisterIndex::try_from(x).unwrap()),
                    0x29 => Instruction::LoadFontCharacter(RegisterIndex::try_from(x).unwrap()),
                    0x33 => Instruction::LoadBinaryCodedDecimal(RegisterIndex::try_from(x).unwrap()),
                    0x55 => Instruction::StoreRegisters(RegisterIndex::try_from(x).unwrap()),
                    0x65 => Instruction::LoadRegisters(RegisterIndex::try_from(x).unwrap()),
                    _ => panic!("Unknown instruction: {:#X}", raw_instruction)
                }
            }
            _ => panic!("Unknown instruction: {:#X}", raw_instruction),
        }
    }
}
