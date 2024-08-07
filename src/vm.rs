use crate::memory::Memory;
use crate::registers::*;
use crate::{instructions::*, traps};
use std::fs::File;
use std::io::Read;

pub struct VM {
    memory: Memory,
    registers: Registers,
}

#[allow(clippy::new_without_default)]
impl VM {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            registers: Registers::new(),
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
                x if x == Opcode::BR as u16 => branch(&mut self.registers, instr),
                x if x == Opcode::ADD as u16 => add(&mut self.registers, instr),
                x if x == Opcode::LD as u16 => load(&mut self.registers, &self.memory, instr),
                x if x == Opcode::ST as u16 => store(&mut self.registers, &mut self.memory, instr),
                x if x == Opcode::JSR as u16 => jump_to_subroutine(&mut self.registers, instr),
                x if x == Opcode::AND as u16 => and(&mut self.registers, instr),
                x if x == Opcode::LDR as u16 => {
                    load_register(&mut self.registers, &self.memory, instr)
                }
                x if x == Opcode::STR as u16 => {
                    store_register(&mut self.registers, &mut self.memory, instr)
                }
                x if x == Opcode::NOT as u16 => not(&mut self.registers, instr),
                x if x == Opcode::LDI as u16 => {
                    load_indirect(&mut self.registers, &self.memory, instr)
                }
                x if x == Opcode::STI as u16 => {
                    store_indirect(&mut self.registers, &mut self.memory, instr)
                }
                x if x == Opcode::JMP as u16 => jump(&mut self.registers, instr),
                x if x == Opcode::LEA as u16 => load_effective_address(&mut self.registers, instr),
                x if x == Opcode::TRAP as u16 => {
                    traps::execute(&mut self.registers, &self.memory, instr)
                }
                x if x == Opcode::RTI as u16 || x == Opcode::RES as u16 => {
                    println!("RTI and RES are not implemented")
                }
                _ => {
                    eprintln!("Error: Invalid opcode");
                    running = false;
                }
            }
        }
    }
}
