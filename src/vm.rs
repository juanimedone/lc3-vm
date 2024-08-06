use crate::flags::Flag;
use crate::instructions::Opcode;
use crate::memory::Memory;
use crate::registers::{Register, Registers};
use std::fs::File;
use std::io::Read;

pub const PC_START: u16 = 0x3000; // default starting position

pub struct VM {
    memory: Memory,
    registers: Registers,
}

impl VM {
    pub fn new() -> Self {
        let mut registers = Registers::new();
        registers.write(Register::COND, Flag::ZRO as u16);
        registers.write(Register::PC, PC_START);
        Self {
            memory: Memory::new(),
            registers,
        }
    }

    pub fn read_image(&mut self, file_path: &str) -> Result<(), String> {
        let mut file = File::open(file_path).map_err(|e| e.to_string())?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
        for (i, chunk) in buffer.chunks(2).enumerate() {
            if chunk.len() == 2 {
                let value = u16::from_be_bytes([chunk[0], chunk[1]]);
                self.memory.write(i as u16, value);
            }
        }
        Ok(())
    }

    pub fn run(&mut self) {
        let mut running = true;
        while running {
            let pc = self.registers.read(Register::PC);
            let instr = self.memory.read(pc);
            self.registers.write(Register::PC, pc.wrapping_add(1));
            let op = instr >> 12;

            match op {
                x if x == Opcode::ADD as u16 => {
                    // Handle ADD
                }
                x if x == Opcode::AND as u16 => {
                    // Handle AND
                }
                x if x == Opcode::NOT as u16 => {
                    // Handle NOT
                }
                x if x == Opcode::BR as u16 => {
                    // Handle BR
                }
                x if x == Opcode::JMP as u16 => {
                    // Handle JMP
                }
                x if x == Opcode::JSR as u16 => {
                    // Handle JSR
                }
                x if x == Opcode::LD as u16 => {
                    // Handle LD
                }
                x if x == Opcode::LDI as u16 => {
                    // Handle LDI
                }
                x if x == Opcode::LDR as u16 => {
                    // Handle LDR
                }
                x if x == Opcode::LEA as u16 => {
                    // Handle LEA
                }
                x if x == Opcode::ST as u16 => {
                    // Handle ST
                }
                x if x == Opcode::STI as u16 => {
                    // Handle STI
                }
                x if x == Opcode::STR as u16 => {
                    // Handle STR
                }
                x if x == Opcode::TRAP as u16 => {
                    // Handle TRAP
                }
                x if x == Opcode::RES as u16 || x == Opcode::RTI as u16 => {
                    // Handle reserved and RTI
                }
                _ => {
                    eprintln!("Error: Invalid opcode");
                    running = false;
                }
            }
        }
    }
}
