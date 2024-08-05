use std::fs::File;
use std::io::Read;

pub const MEMORY_SIZE: usize = 65536; // 64 KB of memory (65536 locations)
pub const PC_START: u16 = 0x3000; // default starting position

#[derive(Debug)]
pub enum Register {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    COND,
    COUNT,
}

#[derive(Debug)]
pub enum Flag {
    POS = 1 << 0, // P
    ZRO = 1 << 1, // Z
    NEG = 1 << 2, // N
}

#[derive(Debug)]
pub enum Opcode {
    BR = 0, // branch
    ADD,    // add
    LD,     // load
    ST,     // store
    JSR,    // jump register
    AND,    // bitwise and
    LDR,    // load register
    STR,    // store register
    RTI,    // unused
    NOT,    // bitwise not
    LDI,    // load indirect
    STI,    // store indirect
    JMP,    // jump
    RES,    // reserved (unused)
    LEA,    // load effective address
    TRAP,   // execute trap
}

pub fn initialize_memory() -> Vec<u16> {
    vec![0; MEMORY_SIZE]
}

pub fn initialize_registers() -> Vec<u16> {
    vec![0; Register::COUNT as usize]
}

pub fn read_image(file_path: &str, memory: &mut Vec<u16>) -> Result<(), String> {
    let mut file = File::open(file_path).map_err(|e| e.to_string())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
    for (i, chunk) in buffer.chunks(2).enumerate() {
        if chunk.len() == 2 {
            let value = u16::from_be_bytes([chunk[0], chunk[1]]);
            memory[i] = value;
        }
    }
    Ok(())
}

pub fn mem_read(address: u16, memory: &Vec<u16>) -> u16 {
    memory[address as usize]
}

pub fn mem_write(address: u16, value: u16, memory: &mut Vec<u16>) {
    memory[address as usize] = value;
}
