use crate::{instructions::update_flags, memory::Memory, registers::*};
use std::io::{Read, Write};

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
            update_flags(registers, Register::R0);
        }
        x if x == Trapcode::OUT as u16 => {
            let ch = registers.read(Register::R0);
            print!("{}", ch as u8 as char);
            std::io::stdout().flush().unwrap();
        }
        x if x == Trapcode::PUTS as u16 => {
            let mut address = registers.read(Register::R0);
            loop {
                let word = memory.read(address);
                if word == 0 {
                    break;
                }
                print!("{}", word as u8 as char);
                address += 1;
            }
            std::io::stdout().flush().unwrap();
        }
        x if x == Trapcode::IN as u16 => {
            print!("Enter a character: ");
            std::io::stdout().flush().unwrap();
            let ch = std::io::stdin().bytes().next().unwrap().unwrap() as u16;
            print!("{}", ch as u8 as char);
            std::io::stdout().flush().unwrap();
            registers.write(Register::R0, ch);
            update_flags(registers, Register::R0);
        }
        x if x == Trapcode::PUTSP as u16 => {
            let mut address = registers.read(Register::R0);
            loop {
                let word = memory.read(address);
                let char1 = (word & 0xFF) as u8 as char;
                let char2 = (word >> 8) as u8 as char;

                if char1 == '\0' {
                    break;
                }
                print!("{}", char1);

                if char2 != '\0' {
                    print!("{}", char2);
                }

                address += 1;
            }
            std::io::stdout().flush().unwrap();
        }
        x if x == Trapcode::HALT as u16 => {
            println!("Program halted");
            std::io::stdout().flush().unwrap();
            std::process::exit(0);
        }
        _ => {
            panic!("Unknown trap code: {:#04X}", instr & 0xFF);
        }
    }
}
