use crate::hardware::{memory::Memory, registers::*};
use std::io::{self, Read, Write};

pub enum Trapcode {
    GETC = 0x20,  // Get character from keyboard, not echoed onto the terminal
    OUT = 0x21,   // Output a character
    PUTS = 0x22,  // Output a word string
    IN = 0x23,    // Get character from keyboard, echoed onto the terminal
    PUTSP = 0x24, // Output a byte string
    HALT = 0x25,  // Halt the program
}

impl From<u16> for Trapcode {
    fn from(value: u16) -> Self {
        match value {
            0x20 => Trapcode::GETC,
            0x21 => Trapcode::OUT,
            0x22 => Trapcode::PUTS,
            0x23 => Trapcode::IN,
            0x24 => Trapcode::PUTSP,
            0x25 => Trapcode::HALT,
            _ => panic!("Invalid trap code value"),
        }
    }
}

pub fn execute(
    registers: &mut Registers,
    memory: &mut Memory,
    instr: u16,
    running: &mut bool,
) -> Result<(), String> {
    let pc = registers.read(Register::PC);
    registers.write(Register::R7, pc);

    match Trapcode::from(instr & 0xFF) {
        Trapcode::GETC => getc(registers)?,
        Trapcode::OUT => out(registers)?,
        Trapcode::PUTS => puts(registers, memory)?,
        Trapcode::IN => in_(registers)?,
        Trapcode::PUTSP => putsp(registers, memory)?,
        Trapcode::HALT => halt(running)?,
    }
    Ok(())
}

fn getc(registers: &mut Registers) -> Result<(), String> {
    let ch = io::stdin()
        .bytes()
        .next()
        .ok_or("Failed to read byte")?
        .map_err(|e| e.to_string())? as u16;
    registers.write(Register::R0, ch);
    registers.update_flags(Register::R0);
    Ok(())
}

fn out(registers: &Registers) -> Result<(), String> {
    let ch = registers.read(Register::R0);
    print!("{}", ch as u8 as char);
    io::stdout().flush().map_err(|e| e.to_string())
}

fn puts(registers: &Registers, memory: &mut Memory) -> Result<(), String> {
    let mut address = registers.read(Register::R0);
    loop {
        let word = memory.read(address)?;
        if word == 0 {
            break;
        }
        print!("{}", word as u8 as char);
        address += 1;
    }
    io::stdout().flush().map_err(|e| e.to_string())
}

fn in_(registers: &mut Registers) -> Result<(), String> {
    print!("Enter a character: ");
    io::stdout().flush().map_err(|e| e.to_string())?;
    let ch = io::stdin()
        .bytes()
        .next()
        .ok_or("Failed to read byte")?
        .map_err(|e| e.to_string())? as u16;
    print!("{}", ch as u8 as char);
    io::stdout().flush().map_err(|e| e.to_string())?;
    registers.write(Register::R0, ch);
    registers.update_flags(Register::R0);
    Ok(())
}

fn putsp(registers: &Registers, memory: &mut Memory) -> Result<(), String> {
    let mut address = registers.read(Register::R0);
    loop {
        let word = memory.read(address)?;
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
    io::stdout().flush().map_err(|e| e.to_string())
}

fn halt(running: &mut bool) -> Result<(), String> {
    println!("Program halted");
    io::stdout().flush().map_err(|e| e.to_string())?;
    *running = false;
    Ok(())
}
