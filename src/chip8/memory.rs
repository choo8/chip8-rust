const MEMORY_SIZE: usize = 4096;
const ROM_START_ADDRESS: usize = 0x200;
const FONTSET_START_ADDRESS: usize = 0x50;

const FONT_SET: [u8; 80] = [
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

pub struct Memory {
    ram: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        let mut ram = [0; MEMORY_SIZE];

        let font_end = FONTSET_START_ADDRESS + FONT_SET.len();
        ram[FONTSET_START_ADDRESS..font_end].copy_from_slice(&FONT_SET);

        Self { ram }
    }

    pub fn load_rom(&mut self, rom_data: &[u8]) {
        let rom_start = ROM_START_ADDRESS;
        let rom_end = rom_start + rom_data.len();

        if rom_end > MEMORY_SIZE {
            panic!("ROM size exceeds available memory");
        }

        self.ram[rom_start..rom_end].copy_from_slice(rom_data);
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    pub fn read_instruction(&self, address: u16) -> u16 {
        let addr = address as usize;
        let high_byte = self.ram[addr] as u16;
        let low_byte = self.ram[addr + 1] as u16;
        (high_byte << 8) | low_byte
    }
}
