use crate::utils::{check_key, getchar};

pub const MEMORY_SIZE: usize = 1 << 16; // 128 KB of memory (2^16 = 65536 locations of 16 bits each)

pub enum MemoryMappedRegister {
    KBSR = 0xFE00, // Keyboard status register
    KBDR = 0xFE02, // Keyboard data register
}

#[derive(Default)]
pub struct Memory {
    memory: Vec<u16>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            memory: vec![0; MEMORY_SIZE],
        }
    }

    pub fn read(&mut self, address: u16) -> Result<u16, String> {
        if address == MemoryMappedRegister::KBSR as u16 {
            if check_key() {
                self.memory[address as usize] = 1 << 15; // Set the ready bit
                self.memory[MemoryMappedRegister::KBDR as usize] = getchar()?;
            } else {
                self.memory[address as usize] = 0; // Clear the ready bit
            }
            Ok(self.memory[address as usize])
        } else {
            Ok(self.memory[address as usize])
        }
    }

    pub fn write(&mut self, address: u16, value: u16) {
        self.memory[address as usize] = value;
    }
}
