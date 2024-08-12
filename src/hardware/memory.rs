use crate::utils::{check_key, getchar};

/// The size of the memory in the LC-3 VM.
/// 2^16 = 65536 locations of 16 bits each = 128 KB of memory.
pub const MEMORY_SIZE: usize = 1 << 16;

/// Enum representing memory-mapped registers.
pub enum MemoryMappedRegister {
    /// Keyboard status register
    KBSR = 0xFE00,
    /// Keyboard data register
    KBDR = 0xFE02,
}

/// Struct representing the memory of the LC-3 VM.
#[derive(Default)]
pub struct Memory {
    /// Vector storing the memory contents.
    memory: Vec<u16>,
}

impl Memory {
    /// Creates a new `Memory` instance with all locations initialized to zero.
    ///
    /// # Returns
    ///
    /// A new instance of `Memory`.
    pub fn new() -> Self {
        Self {
            memory: vec![0; MEMORY_SIZE],
        }
    }

    /// Reads a value from the specified memory address.
    ///
    /// If the address corresponds to a memory-mapped register, the appropriate
    /// behavior (e.g., checking the keyboard status) is executed.
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
