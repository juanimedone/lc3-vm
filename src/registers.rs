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

pub struct Registers {
    reg: Vec<u16>,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            reg: vec![0; Register::COUNT as usize],
        }
    }

    pub fn read(&self, reg: Register) -> u16 {
        self.reg[reg as usize]
    }

    pub fn write(&mut self, reg: Register, value: u16) {
        self.reg[reg as usize] = value;
    }
}
