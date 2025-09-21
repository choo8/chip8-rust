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
            Instruction::Jump(addr) => {
                self.program_counter = addr;
            }
            Instruction::Load(vx, kk) => {
                self.registers.set(vx, kk);
            }
            Instruction::Add(vx, kk) => {
                let current = self.registers.get(vx);
                self.registers.set(vx, current.wrapping_add(kk));
            }
            Instruction::LoadIndexRegister(addr) => {
                self.index_register = addr;
            }
            Instruction::Display(x_reg, y_reg, nibble) => {
                let x = self.registers.get(x_reg) as usize;
                let y = self.registers.get(y_reg) as usize;
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
