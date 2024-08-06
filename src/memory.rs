const MEMORY_SIZE: usize = 1 << 16; // 128 KB of memory (2^16 = 65536 locations of 16 bits each)

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

    pub fn read(&self, address: u16) -> u16 {
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u16) {
        self.memory[address as usize] = value;
    }
}
