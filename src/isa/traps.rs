use crate::hardware::{memory::Memory, registers::*};
use std::io::{self, Read, Write};

/// Represents LC-3 trap codes.
pub enum Trapcode {
    /// Get character from keyboard, not echoed onto the terminal.
    GETC = 0x20,

    /// Output a character.
    OUT = 0x21,

    /// Output a word string.
    PUTS = 0x22,

    /// Get character from keyboard, echoed onto the terminal.
    IN = 0x23,

    /// Output a byte string.
    PUTSP = 0x24,

    /// Halt the program.
    HALT = 0x25,
}

impl From<u16> for Trapcode {
    /// Converts a 16-bit unsigned integer into a `Trapcode` enum variant.
    ///
    /// # Panics
    ///
    /// Panics if the value is not a valid trap code (0x20 to 0x25).
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

/// Executes the trap instruction.
///
/// This function processes a trap instruction, calling the appropriate function based on the trap code.
///
/// # Parameters
/// - `registers`: A mutable reference to the `Registers` object.
/// - `memory`: A mutable reference to the `Memory` object.
/// - `instr`: The full instruction including the trap code.
/// - `running`: A mutable reference to the running status of the program.
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

/// Executes the GETC trap code.
///
/// This function reads a character from the keyboard (not echoed) and stores it in register R0.
///
/// # Parameters
/// - `registers`: A mutable reference to the `Registers` object.
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

/// Executes the OUT trap code.
///
/// This function outputs a character from register R0 to the terminal.
///
/// # Parameters
/// - `registers`: A reference to the `Registers` object.
fn out(registers: &Registers) -> Result<(), String> {
    let ch = registers.read(Register::R0);
    print!("{}", ch as u8 as char);
    io::stdout().flush().map_err(|e| e.to_string())
}

/// Executes the PUTS trap code.
///
/// This function outputs a null-terminated string starting from the address in register R0.
///
/// # Parameters
/// - `registers`: A reference to the `Registers` object.
/// - `memory`: A mutable reference to the `Memory` object.
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

/// Executes the IN trap code.
///
/// This function prompts the user to enter a character, echoes it, and stores it in register R0.
///
/// # Parameters
/// - `registers`: A mutable reference to the `Registers` object.
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

/// Executes the PUTSP trap code.
///
/// This function outputs a string of bytes starting from the address in register R0,
/// where each word contains two ASCII characters.
///
/// # Parameters
/// - `registers`: A reference to the `Registers` object.
/// - `memory`: A mutable reference to the `Memory` object.
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

/// Executes the HALT trap code.
///
/// This function halts the execution of the program and prints a message.
///
/// # Parameters
/// - `running`: A mutable reference to the running status of the program.
fn halt(running: &mut bool) -> Result<(), String> {
    println!("Program halted");
    io::stdout().flush().map_err(|e| e.to_string())?;
    *running = false;
    Ok(())
}
