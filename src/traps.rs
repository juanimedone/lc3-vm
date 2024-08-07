use crate::{memory::Memory, registers::*};
use std::io::Read;

pub enum Trapcode {
    GETC = 0x20,  // Get character from keyboard, not echoed onto the terminal
    OUT = 0x21,   // Output a character
    PUTS = 0x22,  // Output a word string
    IN = 0x23,    // Get character from keyboard, echoed onto the terminal
    PUTSP = 0x24, // Output a byte string
    HALT = 0x25,  // Halt the program
}

pub fn execute(registers: &mut Registers, memory: &Memory, instr: u16) {
    let pc = registers.read(Register::PC);
    registers.write(Register::R7, pc);

    match instr & 0xFF {
        x if x == Trapcode::GETC as u16 => {
            let ch = std::io::stdin().bytes().next().unwrap().unwrap() as u16;
            registers.write(Register::R0, ch);
        }
        x if x == Trapcode::OUT as u16 => {
            let ch = registers.read(Register::R0);
            print!("{}", ch as u8 as char);
        }
        x if x == Trapcode::PUTS as u16 => {
            let mut address = registers.read(Register::R0);
            loop {
                let word = memory.read(address);
                let high_byte = (word >> 8) as u8;
                let low_byte = (word & 0xFF) as u8;

                if high_byte == 0 {
                    if low_byte == 0 {
                        break;
                    }
                    print!("{}", low_byte as char);
                } else {
                    print!("{}", high_byte as char);
                    if low_byte != 0 {
                        print!("{}", low_byte as char);
                    }
                }
                address += 1;
            }
        }
        x if x == Trapcode::IN as u16 => {
            let ch = std::io::stdin().bytes().next().unwrap().unwrap() as u16;
            registers.write(Register::R0, ch);
        }
        x if x == Trapcode::PUTSP as u16 => {
            let mut address = registers.read(Register::R0);
            loop {
                let word = memory.read(address);
                let byte1 = (word & 0xFF) as u8;
                let byte2 = ((word >> 8) & 0xFF) as u8;

                if byte1 == 0 && byte2 == 0 {
                    break;
                }
                if byte1 != 0 {
                    print!("{}", byte1 as char);
                }
                if byte2 != 0 {
                    print!("{}", byte2 as char);
                }

                address += 1;
            }
        }
        x if x == Trapcode::HALT as u16 => {
            println!("Program halted");
            std::process::exit(0);
        }
        _ => {
            panic!("Unknown trap code: {:#04X}", instr & 0xFF);
        }
    }
}
