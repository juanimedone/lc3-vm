use std::io::Read;

pub const MEMORY_SIZE: usize = 1 << 16; // 128 KB of memory (2^16 = 65536 locations of 16 bits each)

pub enum MemoryMappedRegister {
    KBSR = 0xFE00, // Keyboard status register
    KBDR = 0xFE02, // Keyboard data register
}

pub struct Memory {
    memory: Vec<u16>,
}

#[allow(clippy::new_without_default)]
impl Memory {
    pub fn new() -> Self {
        Self {
            memory: vec![0; MEMORY_SIZE],
        }
    }

    pub fn read(&mut self, address: u16) -> u16 {
        if address == MemoryMappedRegister::KBSR as u16 {
            if check_key() {
                self.memory[address as usize] = 1 << 15; // Set the ready bit
                self.memory[MemoryMappedRegister::KBDR as usize] = getchar();
            } else {
                self.memory[address as usize] = 0; // Clear the ready bit
            }
            self.memory[address as usize]
        } else {
            self.memory[address as usize]
        }
    }

    pub fn write(&mut self, address: u16, value: u16) {
        self.memory[address as usize] = value;
    }
}

// Function to check if a key is pressed (mock implementation)
fn check_key() -> bool {
    todo!()
}

// Function to get a character from the keyboard
fn getchar() -> u16 {
    let mut buffer = [0u8; 1];
    std::io::stdin()
        .read_exact(&mut buffer)
        .expect("Failed to read input");
    buffer[0] as u16
}
