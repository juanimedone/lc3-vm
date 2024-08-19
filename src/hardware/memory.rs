use crate::utils::getchar;

/// The size of the memory in the LC-3 VM.
/// 2^16 = 65536 locations of 16 bits each = 128 KB of memory.
pub const MEMORY_SIZE: usize = 1 << 16;

/// Enum representing memory-mapped registers.
pub enum MemoryMappedRegister {
    /// Keyboard status register.
    KBSR = 0xFE00,
    /// Keyboard data register.
    KBDR = 0xFE02,
}

/// Struct representing the memory of the LC-3 VM.
pub struct Memory {
    /// Array storing the memory contents.
    memory: [u16; MEMORY_SIZE],
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    /// Creates a new `Memory` instance with all locations initialized to zero.
    ///
    /// # Returns
    ///
    /// A new instance of `Memory`.
    pub fn new() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
        }
    }

    /// Reads a value from the specified memory address.
    ///
    /// If the address corresponds to a memory-mapped register, the appropriate
    /// behavior (e.g., reading from standard input) is executed.
    ///
    /// # Parameters
    ///
    /// - `address`: The memory address to read from.
    ///
    /// # Returns
    ///
    /// A `Result` containing the value read from memory or an error message.
    pub fn read(&mut self, address: u16) -> Result<u16, String> {
        if address == MemoryMappedRegister::KBSR as u16 {
            let char = getchar()?;
            if char != 0 {
                self.memory[address as usize] = 1 << 15; // Set the ready bit
                self.memory[MemoryMappedRegister::KBDR as usize] = char;
            } else {
                self.memory[address as usize] = 0; // Clear the ready bit
            }
        }
        Ok(self.memory[address as usize])
    }

    /// Writes a value to the specified memory address.
    ///
    /// # Parameters
    ///
    /// - `address`: The memory address to write to.
    /// - `value`: The value to write to memory.
    pub fn write(&mut self, address: u16, value: u16) {
        self.memory[address as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_memory() {
        let memory = Memory::new();
        assert_eq!(memory.memory.len(), MEMORY_SIZE);
        for &value in &memory.memory {
            assert_eq!(value, 0);
        }
    }

    #[test]
    fn read_regular_address() {
        let mut memory = Memory::new();
        memory.memory[100] = 1234;
        assert_eq!(memory.read(100).unwrap(), 1234);
    }

    #[test]
    fn test_memory_write() {
        let mut memory = Memory::new();
        memory.write(200, 5678);
        assert_eq!(memory.read(200).unwrap(), 5678);
    }
}
