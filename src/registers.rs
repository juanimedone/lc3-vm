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

impl From<u16> for Register {
    fn from(value: u16) -> Self {
        match value {
            0 => Register::R0,
            1 => Register::R1,
            2 => Register::R2,
            3 => Register::R3,
            4 => Register::R4,
            5 => Register::R5,
            6 => Register::R6,
            7 => Register::R7,
            8 => Register::PC,
            9 => Register::COND,
            _ => panic!("Invalid register value"),
        }
    }
}

#[derive(Default)]
pub struct Registers {
    registers: Vec<u16>,
}

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

    pub fn update_flags(&mut self, reg: Register) {
        let value = self.read(reg);
        if value == 0 {
            self.write(Register::COND, Flag::ZRO as u16);
        } else if value >> 15 == 1 {
            // a 1 in the left-most bit indicates negative
            self.write(Register::COND, Flag::NEG as u16);
        } else {
            self.write(Register::COND, Flag::POS as u16);
        }
    }
}
