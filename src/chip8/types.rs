#[derive(Debug, Clone, Copy)]
pub struct Nibble(u8);

#[derive(Debug)]
pub struct InvalidNibble;

impl Nibble {
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for Nibble {
    type Error = InvalidNibble;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 0xF {
            Ok(Self(value))
        } else {
            Err(InvalidNibble)
        }
    }
}
