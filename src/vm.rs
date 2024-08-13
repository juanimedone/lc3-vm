use crate::hardware::memory::{Memory, MEMORY_SIZE};
use crate::hardware::registers::*;
use crate::isa::{instructions::*, traps};
use std::fs::File;
use std::io::Read;

/// The VM struct represents the LC-3 virtual machine, containing the memory and registers.
#[derive(Default)]
pub struct VM {
    memory: Memory,
    registers: Registers,
}

impl VM {
    /// Creates a new instance of the VM with initialized memory and registers.
    ///
    /// # Returns
    ///
    /// A new instance of `VM`.
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            registers: Registers::new(),
        }
    }

    /// Reads an image file and loads its contents into the VM's memory.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the path to the object file to be loaded.
    ///
    /// # Errors
    ///
    /// Returns a `String` error if the file cannot be opened or read.
    pub fn read_image_file(&mut self, path: &str) -> Result<(), String> {
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut origin_bytes = [0u8; 2];
        file.read_exact(&mut origin_bytes)
            .map_err(|e| e.to_string())?;
        let origin = u16::from_be_bytes(origin_bytes);

        let max_read = MEMORY_SIZE - origin as usize;
        let mut buffer = vec![0u16; max_read];
        let mut byte_buffer = vec![0u8; max_read * 2];
        let read = file.read(&mut byte_buffer).map_err(|e| e.to_string())?;
        let read_u16_count = read / 2;

        for i in 0..read_u16_count {
            let byte1 = byte_buffer[i * 2];
            let byte2 = byte_buffer[i * 2 + 1];
            let value = u16::from_be_bytes([byte1, byte2]);
            buffer[i] = value;
        }

        for (i, value) in buffer.iter().take(read_u16_count).enumerate() {
            self.memory.write(origin + i as u16, *value);
        }

        Ok(())
    }

    /// Runs the VM, executing instructions in a loop until the VM is halted.
    ///
    /// # Errors
    ///
    /// Returns a `String` error if there is an issue with reading memory or executing instructions.
    pub fn run(&mut self) -> Result<(), String> {
        let mut running = true;
        while running {
            let pc = self.registers.read(Register::PC);
            let instr = self.memory.read(pc)?;
            self.registers.write(Register::PC, pc.wrapping_add(1));
            let op = Opcode::from(instr >> 12);
            match op {
                Opcode::BR => branch(&mut self.registers, instr),
                Opcode::ADD => add(&mut self.registers, instr),
                Opcode::LD => load(&mut self.registers, &mut self.memory, instr)?,
                Opcode::ST => store(&mut self.registers, &mut self.memory, instr),
                Opcode::JSR => jump_to_subroutine(&mut self.registers, instr),
                Opcode::AND => and(&mut self.registers, instr),
                Opcode::LDR => load_register(&mut self.registers, &mut self.memory, instr)?,
                Opcode::STR => store_register(&mut self.registers, &mut self.memory, instr),
                Opcode::NOT => not(&mut self.registers, instr),
                Opcode::LDI => load_indirect(&mut self.registers, &mut self.memory, instr)?,
                Opcode::STI => store_indirect(&mut self.registers, &mut self.memory, instr)?,
                Opcode::JMP => jump(&mut self.registers, instr),
                Opcode::LEA => load_effective_address(&mut self.registers, instr),
                Opcode::TRAP => {
                    traps::execute(&mut self.registers, &mut self.memory, instr, &mut running)?
                }
                Opcode::RTI | Opcode::RES => {
                    println!("RTI and RES are not implemented");
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write};

    const TEST_FILES_PATH: &str = "tests/assembly/";

    // Helper function to create a test object file with predefined content
    fn create_test_file(path: &str, content: &[u8]) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(content)?;
        Ok(())
    }

    #[test]
    fn read_image_file_success() {
        let content = vec![
            0x30, 0x00, // Origin address: 0x3000
            0x12, 0x34, // Instruction 1
            0xAB, 0xCD, // Instruction 2
        ];
        let file_path = format!("{}{}", TEST_FILES_PATH, "test_success.obj");
        create_test_file(&file_path, &content).expect("Failed to create test file");

        let mut vm = VM::new();
        assert!(vm.read_image_file(&file_path).is_ok());

        assert_eq!(vm.memory.read(0x3000).unwrap(), 0x1234);
        assert_eq!(vm.memory.read(0x3001).unwrap(), 0xABCD);
    }

    #[test]
    fn read_image_file_nonexistent_path() {
        let mut vm = VM::new();
        assert!(vm.read_image_file("nonexistent_file.obj").is_err());
    }

    #[test]
    fn read_image_file_invalid_format() {
        // Create a file with invalid content (not enough bytes for origin address)
        let content = vec![0x30];
        let file_path = format!("{}{}", TEST_FILES_PATH, "test_err.obj");
        create_test_file(&file_path, &content).expect("Failed to create test file");

        let mut vm = VM::new();
        assert!(vm.read_image_file(&file_path).is_err());
    }
}
