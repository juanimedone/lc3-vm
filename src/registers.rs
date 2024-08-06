use crate::flags::Flag;

pub const PC_START: u16 = 0x3000; // default starting position

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

pub struct Registers {
    registers: Vec<u16>,
}

#[allow(clippy::new_without_default)]
impl Registers {
    pub fn new() -> Self {
        let mut registers = vec![0; Register::COUNT as usize];
        registers[Register::PC as usize] = PC_START;
        registers[Register::COND as usize] = Flag::ZRO as u16;

        Self { registers }
    }

    pub fn read(&self, reg: Register) -> u16 {
        self.registers[reg as usize]
    }

    pub fn write(&mut self, reg: Register, value: u16) {
        self.registers[reg as usize] = value;
    }
}
