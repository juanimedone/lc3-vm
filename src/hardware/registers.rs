use crate::hardware::flags::Flag;

/// Default starting position for the program counter (PC).
pub const PC_START: u16 = 0x3000;

/// Enumeration of the 10 LC-3 registers.
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
    /// Converts a `u16` value to a `Register` enum variant.
    ///
    /// # Panics
    ///
    /// Panics if the value does not correspond to a valid register.
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

/// Structure representing the registers of the LC-3 VM.
#[derive(Default)]
pub struct Registers {
    /// Vector storing the registers contents.
    registers: Vec<u16>,
}

impl Registers {
    /// Creates a new `Registers` instance with default values.
    ///
    /// The program counter (PC) is initialized to `PC_START`,
    /// and the condition register (COND) is set to `Flag::ZRO`.
    ///
    /// # Returns
    ///
    /// A new instance of `Registers`.
    pub fn new() -> Self {
        let mut registers = vec![0; Register::COUNT as usize];
        registers[Register::PC as usize] = PC_START;
        registers[Register::COND as usize] = Flag::ZRO as u16;

        Self { registers }
    }

    /// Reads the value from the specified register.
    ///
    /// # Arguments
    ///
    /// * `reg` - The register to read from.
    ///
    /// # Returns
    ///
    /// The value of the specified register.
    pub fn read(&self, reg: Register) -> u16 {
        self.registers[reg as usize]
    }

    /// Writes a value to the specified register.
    ///
    /// # Arguments
    ///
    /// * `reg` - The register to write to.
    /// * `value` - The value to write.
    pub fn write(&mut self, reg: Register, value: u16) {
        self.registers[reg as usize] = value;
    }

    /// Updates the condition flags based on the value of the specified register.
    ///
    /// # Arguments
    ///
    /// * `reg` - The register to use for updating the condition flags.
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
