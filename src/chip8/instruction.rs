use super::register::RegisterIndex;
use super::types::Nibble;

#[derive(Debug)]
pub enum Instruction {
    // 00E0 - CLS
    Clear,
    // 1nnn - JP addr
    Jump(u16),
    // 6xkk - LD Vx, byte
    Load(RegisterIndex, u8),
    // 7xkk - ADD Vx, byte
    Add(RegisterIndex, u8),
    // Annn - LD I, addr
    LoadIndexRegister(u16),
    // Dxyn - DRW Vx, Vy, nibble
    Display(RegisterIndex, RegisterIndex, Nibble),
}

impl From<u16> for Instruction {
    fn from(raw_instruction: u16) -> Self {
        let opcode = (raw_instruction & 0xF000) >> 12;

        match opcode {
            0x0 => Instruction::Clear,
            0x1 => {
                let nnn = raw_instruction & 0x0FFF;
                Instruction::Jump(nnn)
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
            0xA => {
                let nnn = raw_instruction & 0x0FFF;
                Instruction::LoadIndexRegister(nnn)
            }
            0xD => {
                let x = ((raw_instruction & 0x0F00) >> 8) as u8;
                let y = ((raw_instruction & 0x00F0) >> 4) as u8;
                let n = (raw_instruction & 0x000F) as u8;
                Instruction::Display(RegisterIndex::try_from(x).unwrap(), RegisterIndex::try_from(y).unwrap(), Nibble::try_from(n).unwrap())
            }
            _ => panic!("Unknown instruction: {:#X}", raw_instruction),
        }
    }
}
