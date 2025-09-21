mod display;
mod instruction;
mod keypad;
mod memory;
mod register;
mod types;

use display::Display;
use instruction::Instruction;
use keypad::Keypad;
use memory::Memory;
use register::{RegisterIndex, RegisterFile};

use crate::chip8::memory::FONTSET_START_ADDRESS;

pub struct Chip8 {
    memory: Memory,
    registers: RegisterFile,
    index_register: u16,
    program_counter: u16,
    stack: [u16; 16],
    stack_pointer: u8,
    delay_timer: u8,
    sound_timer: u8,
    pub display: Display,
    keypad: Keypad,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            memory: Memory::new(),
            registers: RegisterFile::new(),
            index_register: 0,
            program_counter: 0x200, // Programs start at memory location 0x200
            stack: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            display: Display::new(),
            keypad: Keypad::new(),
        }
    }

    pub fn load_rom(&mut self, rom_data: &[u8]) {
        self.memory.load_rom(rom_data);
    }

    pub fn emulate_cycle(&mut self) {
        let raw_instruction = self.memory.read_instruction(self.program_counter);

        self.program_counter += 2;

        let instruction = Instruction::from(raw_instruction);

        match instruction {
            Instruction::Clear => {
                self.display.clear();
            }
            Instruction::Return => {
                self.stack_pointer -= 1;
                self.program_counter = self.stack[self.stack_pointer as usize];
            }
            Instruction::Jump(addr) => {
                self.program_counter = addr;
            }
            Instruction::Call(addr) => {
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.stack_pointer += 1;
                self.program_counter = addr;
            }
            Instruction::SkipEqual(x, kk, ) => {
                if self.registers.get(x) == kk {
                    self.program_counter += 2;
                }
            }
            Instruction::SkipNotEqual(x, kk) => {
                if self.registers.get(x) != kk {
                    self.program_counter += 2;
                }
            }
            Instruction::SkipEqualRegister(x, y, ) => {
                if self.registers.get(x) == self.registers.get(y) {
                    self.program_counter += 2;
                }
            }
            Instruction::Load(x, kk) => {
                self.registers.set(x, kk);
            }
            Instruction::Add(x, kk) => {
                let current = self.registers.get(x);
                self.registers.set(x, current.wrapping_add(kk));
            }
            Instruction::LoadRegister(x, y, ) => {
                let value = self.registers.get(y);
                self.registers.set(x, value);
            }
            Instruction::LoadOr(x, y) => {
                let value = self.registers.get(x) | self.registers.get(y);
                self.registers.set(x, value);
            }
            Instruction::LoadAnd(x, y) => {
                let value = self.registers.get(x) & self.registers.get(y);
                self.registers.set(x, value);
            }
            Instruction::LoadXor(x, y) => {
                let value = self.registers.get(x) ^ self.registers.get(y);
                self.registers.set(x, value);
            }
            Instruction::LoadAdd(x, y) => {
                let (result, carry) = self.registers.get(x).overflowing_add(self.registers.get(y));
                self.registers.set(x, result);
                self.registers.set(RegisterIndex::try_from(0xF).unwrap(), if carry { 1 } else { 0 });
            }
            Instruction::LoadSub(x, y) => {
                let (result, borrow) = self.registers.get(x).overflowing_sub(self.registers.get(y));
                self.registers.set(x, result);
                self.registers.set(RegisterIndex::try_from(0xF).unwrap(), if borrow { 0 } else { 1 });
            }
            Instruction::LoadShiftRight(x) => {
                let lsb = self.registers.get(x) & 0x1;
                self.registers.set(RegisterIndex::try_from(0xF).unwrap(), lsb);
                self.registers.set(x, self.registers.get(x) >> 1);
            }
            Instruction::LoadSubNegative(x, y) => {
                let (result, borrow) = self.registers.get(y).overflowing_sub(self.registers.get(x));
                self.registers.set(x, result);
                self.registers.set(RegisterIndex::try_from(0xF).unwrap(), if borrow { 0 } else { 1 });
            }
            Instruction::LoadShiftLeft(x) => {
                let msb = (self.registers.get(x) & 0x80) >> 7;
                self.registers.set(RegisterIndex::try_from(0xF).unwrap(), msb);
                self.registers.set(x, self.registers.get(x) << 1);
            }
            Instruction::SkipNotEqualRegister(x, y) => {
                if self.registers.get(x) != self.registers.get(y) {
                    self.program_counter += 2;
                }
            }
            Instruction::LoadIndexRegister(addr) => {
                self.index_register = addr;
            }
            Instruction::JumpWithOffset(addr) => {
                let offset = self.registers.get(RegisterIndex::try_from(0).unwrap()) as u16;
                self.program_counter = addr + offset;
            }
            Instruction::Random(x, kk) => {
                let random_byte: u8 = rand::random();
                self.registers.set(x, random_byte & kk);
            }
            Instruction::Display(x, y, nibble) => {
                let x = self.registers.get(x) as usize;
                let y = self.registers.get(y) as usize;
                let height = nibble.value() as usize;

                self.registers.set(RegisterIndex::try_from(0xF).unwrap(), 0);

                for row in 0..height {
                    let sprite_byte = self.memory.read_byte(self.index_register + row as u16);
                    for col in 0..8 {
                        if (sprite_byte & (0x80 >> col)) != 0 {
                            let pixel_x = (x + col) % 64;
                            let pixel_y = (y + row) % 32;
                            if self.display.toggle_pixel(pixel_x, pixel_y) {
                                self.registers.set(RegisterIndex::try_from(0xF).unwrap(), 1);
                            }
                        }
                    }
                }
            }
            Instruction::SkipKeyPress(x) => {
                let key = self.registers.get(x);
                if self.keypad.is_key_pressed(key) {
                    self.program_counter += 2;
                }
            }
            Instruction::SkipKeyNotPress(x) => {
                let key = self.registers.get(x);
                if !self.keypad.is_key_pressed(key) {
                    self.program_counter += 2;
                }
            }
            Instruction::LoadDelayTimer(x) => {
                let value = self.delay_timer;
                self.registers.set(x, value);
            }
            Instruction::LoadKeyPress(x) => {
                if let Some(key) = self.keypad.get_pressed_key() {
                    self.registers.set(x, key);
                } else {
                    self.program_counter -= 2; // Repeat this instruction
                }
            }
            Instruction::StoreDelayTimer(x) => {
                let value = self.registers.get(x);
                self.delay_timer = value;
            }
            Instruction::StoreSoundTimer(x) => {
                let value = self.registers.get(x);
                self.sound_timer = value;
            }
            Instruction::AddIndexRegister(x) => {
                let value = self.registers.get(x) as u16;
                self.index_register = self.index_register.wrapping_add(value);
            }
            Instruction::LoadFontCharacter(x) => {
                let value = self.registers.get(x);
                self.index_register = FONTSET_START_ADDRESS as u16 + (value as u16 * 5);
            }
            Instruction::LoadBinaryCodedDecimal(x) => {
                let value = self.registers.get(x);
                self.memory.write_byte(self.index_register, (value / 100) % 10);
                self.memory.write_byte(self.index_register + 1, (value % 100) / 10);
                self.memory.write_byte(self.index_register + 2, value % 10);
            }
            Instruction::StoreRegisters(x) => {
                for i in 0..=x.value() {
                    let value = self.registers.get(RegisterIndex::try_from(i as u8).unwrap());
                    self.memory.write_byte(self.index_register + i as u16, value);
                }
            }
            Instruction::LoadRegisters(x) => {
                for i in 0..=x.value() {
                    let value = self.memory.read_byte(self.index_register + i as u16);
                    self.registers.set(RegisterIndex::try_from(i as u8).unwrap(), value);
                }
            }
        }
    }

    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
            if self.sound_timer == 0 {
                // Beep sound
            }
        }
    }
}
