use crate::hardware::memory::{Memory, MEMORY_SIZE};
use crate::hardware::registers::*;
use crate::isa::{instructions::*, traps};
use std::fs::File;
use std::io::Read;

#[derive(Default)]
pub struct VM {
    memory: Memory,
    registers: Registers,
}

impl VM {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            registers: Registers::new(),
        }
    }

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
                x if x == Opcode::LD as u16 => load(&mut self.registers, &mut self.memory, instr),
                x if x == Opcode::ST as u16 => store(&mut self.registers, &mut self.memory, instr),
                x if x == Opcode::JSR as u16 => jump_to_subroutine(&mut self.registers, instr),
                x if x == Opcode::AND as u16 => and(&mut self.registers, instr),
                x if x == Opcode::LDR as u16 => {
                    load_register(&mut self.registers, &mut self.memory, instr)
                }
                x if x == Opcode::STR as u16 => {
                    store_register(&mut self.registers, &mut self.memory, instr)
                }
                x if x == Opcode::NOT as u16 => not(&mut self.registers, instr),
                x if x == Opcode::LDI as u16 => {
                    load_indirect(&mut self.registers, &mut self.memory, instr)
                }
                x if x == Opcode::STI as u16 => {
                    store_indirect(&mut self.registers, &mut self.memory, instr)
                }
                x if x == Opcode::JMP as u16 => jump(&mut self.registers, instr),
                x if x == Opcode::LEA as u16 => load_effective_address(&mut self.registers, instr),
                x if x == Opcode::TRAP as u16 => {
                    traps::execute(&mut self.registers, &mut self.memory, instr)
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
