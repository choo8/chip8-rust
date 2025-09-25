pub const NUM_REGISTERS: usize = 16;

#[derive(Clone, Copy, Debug)]
pub struct RegisterIndex(u8);

#[derive(Debug)]
pub struct InvalidRegisterIndex;

impl RegisterIndex {
    pub fn value(&self) -> usize {
        self.0 as usize
    }
}

impl TryFrom<u8> for RegisterIndex {
    type Error = InvalidRegisterIndex;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < NUM_REGISTERS as u8 {
            Ok(Self(value))
        } else {
            Err(InvalidRegisterIndex)
        }
    }
}

pub struct RegisterFile {
    registers: [u8; NUM_REGISTERS],
}

impl RegisterFile {
    pub fn new() -> Self {
        Self {
            registers: [0; NUM_REGISTERS],
        }
    }

    pub fn get(&self, reg: RegisterIndex) -> u8 {
        self.registers[reg.value()]
    }

    pub fn set(&mut self, reg: RegisterIndex, val: u8) {
        self.registers[reg.value()] = val;
    }
}
