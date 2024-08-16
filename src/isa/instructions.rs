use crate::{hardware::memory::Memory, hardware::registers::*, utils::sign_extend};

/// Represents the LC-3 opcodes.
pub enum Opcode {
    BR = 0, // branch
    ADD,    // add
    LD,     // load
    ST,     // store
    JSR,    // jump to subroutine
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

impl From<u16> for Opcode {
    /// Converts a 16-bit unsigned integer into an `Opcode` enum variant.
    ///
    /// # Panics
    ///
    /// Panics if the value is not a valid opcode (0 to 15).
    fn from(value: u16) -> Self {
        match value {
            0 => Opcode::BR,
            1 => Opcode::ADD,
            2 => Opcode::LD,
            3 => Opcode::ST,
            4 => Opcode::JSR,
            5 => Opcode::AND,
            6 => Opcode::LDR,
            7 => Opcode::STR,
            8 => Opcode::RTI,
            9 => Opcode::NOT,
            10 => Opcode::LDI,
            11 => Opcode::STI,
            12 => Opcode::JMP,
            13 => Opcode::RES,
            14 => Opcode::LEA,
            15 => Opcode::TRAP,
            _ => panic!("Invalid opcode value"),
        }
    }
}

/// Executes the BR (branch) instruction.
///
/// This function computes the branch target address based on the instruction's offset and condition codes
/// and updates the program counter if the condition codes are met.
///
/// # Parameters
/// - `registers`: A mutable reference to the `Registers` object.
/// - `instr`: A 16-bit unsigned integer representing the full instruction including the opcode and operands.
///
/// # Instruction Format
/// - **Condition Codes**: Bits 9-11
/// - **PC Offset**: Bits 0-8
pub fn branch(registers: &mut Registers, instr: u16) {
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let instr_cond = (instr >> 9) & 0x7;
    let reg_cond = registers.read(Register::COND);

    if (instr_cond & reg_cond) != 0 {
        let pc = registers.read(Register::PC);
        registers.write(Register::PC, pc.wrapping_add(pc_offset));
    }
}

/// Executes the ADD instruction.
///
/// This function performs integer addition. It can either add an immediate value or the value of a register.
///
/// # Parameters
/// - `registers`: A mutable reference to the `Registers` object.
/// - `instr`: A 16-bit unsigned integer representing the full instruction including the opcode and operands.
///
/// # Instruction Format
/// - **Destination Register**: Bits 9-11
/// - **Source Register 1**: Bits 6-8
/// - **Immediate Flag**: Bit 5
/// - **Immediate Value**: Bits 0-4 (if `imm_flag` is 1)
/// - **Source Register 2**: Bits 0-2 (if `imm_flag` is 0)
pub fn add(registers: &mut Registers, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        let result = registers.read(Register::from(r1)).wrapping_add(imm5);
        registers.write(Register::from(r0), result);
    } else {
        let r2 = instr & 0x7;
        let result = registers
            .read(Register::from(r1))
            .wrapping_add(registers.read(Register::from(r2)));
        registers.write(Register::from(r0), result);
    }

    registers.update_flags(Register::from(r0));
}

/// Executes the LD (load) instruction.
///
/// This function loads a value from memory into a register using a PC-relative address.
///
/// # Parameters
/// - `registers`: A mutable reference to the `Registers` object.
/// - `memory`: A mutable reference to the `Memory` object.
/// - `instr`: A 16-bit unsigned integer representing the full instruction including the opcode and operands.
///
/// # Instruction Format
/// - **Destination Register**: Bits 9-11
/// - **PC Offset**: Bits 0-8
pub fn load(registers: &mut Registers, memory: &mut Memory, instr: u16) -> Result<(), String> {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let pc = registers.read(Register::PC);

    let address = pc.wrapping_add(pc_offset);
    let value = memory.read(address)?;
    registers.write(Register::from(r0), value);

    registers.update_flags(Register::from(r0));
    Ok(())
}

/// Executes the ST (store) instruction.
///
/// This function stores a value from a register into memory using a PC-relative address.
///
/// # Parameters
/// - `registers`: A mutable reference to the `Registers` object.
/// - `memory`: A mutable reference to the `Memory` object.
/// - `instr`: A 16-bit unsigned integer representing the full instruction including the opcode and operands.
///
/// # Instruction Format
/// - **Source Register**: Bits 9-11
/// - **PC Offset**: Bits 0-8
pub fn store(registers: &mut Registers, memory: &mut Memory, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let pc = registers.read(Register::PC);

    let address = pc.wrapping_add(pc_offset);
    let value = registers.read(Register::from(r0));
    memory.write(address, value);
}

/// Executes the JSR (jump to subroutine) instruction.
///
/// This function saves the current PC to R7 and then jumps to a new address. It can either use a PC-relative
/// offset or a register-indirect jump.
///
/// # Parameters
/// - `registers`: A mutable reference to the `Registers` object.
/// - `instr`: A 16-bit unsigned integer representing the full instruction including the opcode and operands.
///
/// # Instruction Format
/// - **Long Flag**: Bit 11
/// - **PC Offset (JSR)**: Bits 0-10 (if `long_flag` is 1)
/// - **Base Register (JSRR)**: Bits 6-8 (if `long_flag` is 0)
pub fn jump_to_subroutine(registers: &mut Registers, instr: u16) {
    let current_pc = registers.read(Register::PC);
    registers.write(Register::R7, current_pc);

    let long_flag = (instr >> 11) & 0x1;
    if long_flag == 1 {
        // JSR: Use PC-relative offset
        let long_pc_offset = sign_extend(instr & 0x7FF, 11);
        let new_pc = current_pc.wrapping_add(long_pc_offset);
        registers.write(Register::PC, new_pc);
    } else {
        // JSRR: Use register-indirect jump
        let r1 = (instr >> 6) & 0x7;
        let new_pc = registers.read(Register::from(r1));
        registers.write(Register::PC, new_pc);
    }
}

/// Executes the AND instruction.
///
/// This function performs a bitwise AND operation. It can either use an immediate value or another register.
///
/// # Parameters
/// - `registers`: A mutable reference to the `Registers` object.
/// - `instr`: A 16-bit unsigned integer representing the full instruction including the opcode and operands.
///
/// # Instruction Format
/// - **Destination Register**: Bits 9-11
/// - **Source Register 1**: Bits 6-8
/// - **Immediate Flag**: Bit 5
/// - **Immediate Value**: Bits 0-4 (if `imm_flag` is 1)
/// - **Source Register 2**: Bits 0-2 (if `imm_flag` is 0)
pub fn and(registers: &mut Registers, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        registers.write(
            Register::from(r0),
            registers.read(Register::from(r1)) & imm5,
        );
    } else {
        let r2 = instr & 0x7;
        registers.write(
            Register::from(r0),
            registers.read(Register::from(r1)) & registers.read(Register::from(r2)),
        );
    }

    registers.update_flags(Register::from(r0));
}

/// Executes the LDR (load register) instruction.
///
/// This function loads a value from memory into a register using a base register and an offset.
///
/// # Parameters
/// - `registers`: A mutable reference to the `Registers` object.
/// - `memory`: A mutable reference to the `Memory` object.
/// - `instr`: A 16-bit unsigned integer representing the full instruction including the opcode and operands.
///
/// # Instruction Format
/// - **Destination Register**: Bits 9-11
/// - **Base Register**: Bits 6-8
/// - **Offset**: Bits 0-5
pub fn load_register(
    registers: &mut Registers,
    memory: &mut Memory,
    instr: u16,
) -> Result<(), String> {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);

    let base_address = registers.read(Register::from(r1));
    let final_address = base_address.wrapping_add(offset);
    let value = memory.read(final_address)?;
    registers.write(Register::from(r0), value);

    registers.update_flags(Register::from(r0));
    Ok(())
}

/// Executes the STR (store register) instruction.
///
/// This function stores a value from a register into memory using a base register and an offset.
///
/// # Parameters
/// - `registers`: A mutable reference to the `Registers` object.
/// - `memory`: A mutable reference to the `Memory` object.
/// - `instr`: A 16-bit unsigned integer representing the full instruction including the opcode and operands.
///
/// # Instruction Format
/// - **Source Register**: Bits 9-11
/// - **Base Register**: Bits 6-8
/// - **Offset**: Bits 0-5
pub fn store_register(registers: &mut Registers, memory: &mut Memory, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);

    let base_address = registers.read(Register::from(r1));
    let final_address = base_address.wrapping_add(offset);
    let value = registers.read(Register::from(r0));
    memory.write(final_address, value);
}

/// Executes the NOT instruction.
///
/// This function performs a bitwise NOT operation on the value of a register and stores the result in another register.
///
/// # Parameters
/// - `registers`: A mutable reference to the `Registers` object.
/// - `instr`: A 16-bit unsigned integer representing the full instruction including the opcode and operands.
///
/// # Instruction Format
/// - **Destination Register**: Bits 9-11
/// - **Source Register**: Bits 6-8
pub fn not(registers: &mut Registers, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;

    registers.write(Register::from(r0), !registers.read(Register::from(r1)));

    registers.update_flags(Register::from(r0));
}

/// Executes the LDI (load indirect) instruction.
///
/// This function loads a value from memory into a register using an address obtained indirectly.
///
/// # Parameters
/// - `registers`: A mutable reference to the `Registers` object.
/// - `memory`: A mutable reference to the `Memory` object.
/// - `instr`: A 16-bit unsigned integer representing the full instruction including the opcode and operands.
///
/// # Instruction Format
/// - **Destination Register**: Bits 9-11
/// - **PC Offset**: Bits 0-8
pub fn load_indirect(
    registers: &mut Registers,
    memory: &mut Memory,
    instr: u16,
) -> Result<(), String> {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let pc = registers.read(Register::PC);

    let address = memory.read(pc.wrapping_add(pc_offset))?;
    let value = memory.read(address)?;
    registers.write(Register::from(r0), value);

    registers.update_flags(Register::from(r0));
    Ok(())
}

/// Executes the STI (store indirect) instruction.
///
/// This function stores a value from a register into memory using an address obtained indirectly.
///
/// # Parameters
/// - `registers`: A mutable reference to the `Registers` object.
/// - `memory`: A mutable reference to the `Memory` object.
/// - `instr`: A 16-bit unsigned integer representing the full instruction including the opcode and operands.
///
/// # Instruction Format
/// - **Source Register**: Bits 9-11
/// - **PC Offset**: Bits 0-8
pub fn store_indirect(
    registers: &mut Registers,
    memory: &mut Memory,
    instr: u16,
) -> Result<(), String> {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let pc = registers.read(Register::PC);

    let intermediate_address = pc.wrapping_add(pc_offset);
    let final_address = memory.read(intermediate_address)?;
    let value = registers.read(Register::from(r0));
    memory.write(final_address, value);
    Ok(())
}

/// Executes the JMP (jump) instruction.
///
/// This function sets the program counter to the value in a register.
///
/// # Parameters
/// - `registers`: A mutable reference to the `Registers` object.
/// - `instr`: A 16-bit unsigned integer representing the full instruction including the opcode and operands.
///
/// # Instruction Format
/// - **Base Register**: Bits 6-8
pub fn jump(registers: &mut Registers, instr: u16) {
    let r1 = (instr >> 6) & 0x7;
    registers.write(Register::PC, registers.read(Register::from(r1)));
}

/// Executes the LEA (load effective address) instruction.
///
/// This function computes the effective address using a PC-relative offset and stores it in a register.
///
/// # Parameters
/// - `registers`: A mutable reference to the `Registers` object.
/// - `instr`: A 16-bit unsigned integer representing the full instruction including the opcode and operands.
///
/// # Instruction Format
/// - **Destination Register**: Bits 9-11
/// - **PC Offset**: Bits 0-8
pub fn load_effective_address(registers: &mut Registers, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let pc = registers.read(Register::PC);

    let effective_address = pc.wrapping_add(pc_offset);
    registers.write(Register::from(r0), effective_address);

    registers.update_flags(Register::from(r0));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::flags::Flag;
    use std::u16;

    #[test]
    fn branch_matching_positive_condition() {
        let mut registers = Registers::new();
        registers.write(Register::COND, Flag::POS as u16);

        // Full instruction: BR (Opcode = 0b0000), Condition = 0b001 (positive), Offset = 5
        let instr = 0b0000_001_000000101;
        branch(&mut registers, instr);

        // Check if the PC is updated to PC_START + 5
        assert_eq!(registers.read(Register::PC), PC_START + 5);
    }

    #[test]
    fn branch_matching_zero_condition() {
        let mut registers = Registers::new();
        registers.write(Register::COND, Flag::ZRO as u16);

        // Full instruction: BR (Opcode = 0b0000), Condition = 0b010 (zero), Offset = 5
        let instr = 0b0000_010_000000101;
        branch(&mut registers, instr);

        // Check if the PC is updated to PC_START + 5
        assert_eq!(registers.read(Register::PC), PC_START + 5);
    }

    #[test]
    fn branch_matching_negative_condition() {
        let mut registers = Registers::new();
        registers.write(Register::COND, Flag::NEG as u16);

        // Full instruction: BR (Opcode = 0b0000), Condition = 0b100 (negative), Offset = 5
        let instr = 0b0000_100_000000101;
        branch(&mut registers, instr);

        // Check if the PC is updated to PC_START + 5
        assert_eq!(registers.read(Register::PC), PC_START + 5);
    }

    #[test]
    fn branch_with_no_matching_condition() {
        let mut registers = Registers::new();
        registers.write(Register::COND, Flag::ZRO as u16);

        // Full instruction: BR (Opcode = 0b0000), Condition = 0b001 (positive), Offset = 5
        let instr = 0b0000_001_000000101;
        branch(&mut registers, instr);

        // Check if the PC is not updated, it should still be PC_START
        assert_eq!(registers.read(Register::PC), PC_START);
    }

    #[test]
    fn add_immediate_positive_value() {
        let mut registers = Registers::new();
        registers.write(Register::R1, 5);

        // Full instruction: ADD (Opcode = 0b0001), Destination = R0, Source = R1, Immediate Flag = 1, Immediate Value = 3
        let instr = 0b0001_000_001_1_00011;
        add(&mut registers, instr);

        // R0 = R1 + 3 = 5 + 3 = 8
        assert_eq!(registers.read(Register::R0), 8);
    }

    #[test]
    fn add_immediate_negative_value() {
        let mut registers = Registers::new();
        registers.write(Register::R1, 5);

        // Full instruction: ADD (Opcode = 0b0001), Destination = R0, Source = R1, Immediate Flag = 1, Immediate Value = -3
        let instr = 0b0001_000_001_1_11101;
        add(&mut registers, instr);

        // R0 = R1 + (-3) = 5 - 3 = 2
        assert_eq!(registers.read(Register::R0), 2);
    }

    #[test]
    fn add_immediate_value_with_overflow() {
        let mut registers = Registers::new();
        registers.write(Register::R1, u16::MAX);

        // Full instruction: ADD (Opcode = 0b0001), Destination = R0, Source = R1, Immediate Flag = 1, Immediate Value = 2
        let instr = 0b0001_000_001_1_00010;
        add(&mut registers, instr);

        // R0 = R1 + 2 = 65535 + 2 = 1
        assert_eq!(registers.read(Register::R0), 1);
    }

    #[test]
    fn add_positive_registers_values() {
        let mut registers = Registers::new();
        registers.write(Register::R1, 5);
        registers.write(Register::R2, 3);

        // Full instruction: ADD (Opcode = 0b0001), Destination = R0, Source1 = R1, Immediate Flag = 0, Source2 = R2
        let instr = 0b0001_000_001_0_00_010;
        add(&mut registers, instr);

        // R0 = R1 + R2 = 5 + 3 = 8
        assert_eq!(registers.read(Register::R0), 8);
    }

    #[test]
    fn add_positive_and_negative_registers_values() {
        let mut registers = Registers::new();
        registers.write(Register::R1, 5);
        registers.write(Register::R2, -3i16 as u16);

        // Full instruction: ADD (Opcode = 0b0001), Destination = R0, Source1 = R1, Immediate Flag = 0, Source2 = R2
        let instr = 0b0001_000_001_0_00_010;
        add(&mut registers, instr);

        // R0 = R1 + R2 = 5 + (-3) = 2
        assert_eq!(registers.read(Register::R0), 2);
    }

    #[test]
    fn add_negative_registers_values() {
        let mut registers = Registers::new();
        registers.write(Register::R1, -5i16 as u16);
        registers.write(Register::R2, -3i16 as u16);

        // Full instruction: ADD (Opcode = 0b0001), Destination = R0, Source1 = R1, Immediate Flag = 0, Source2 = R2
        let instr = 0b0001_000_001_0_00_010;
        add(&mut registers, instr);

        // R0 = R1 + R2 = -5 + (-3) = -8
        assert_eq!(registers.read(Register::R0), -8i16 as u16);
    }

    #[test]
    fn add_registers_values_with_overflow() {
        let mut registers = Registers::new();
        registers.write(Register::R1, u16::MAX);
        registers.write(Register::R2, 2);

        // Full instruction: ADD (Opcode = 0b0001), Destination = R0, Source1 = R1, Immediate Flag = 0, Source2 = R2
        let instr = 0b0001_000_001_0_00_010;
        add(&mut registers, instr);

        // R0 = R1 + R2 = 65535 + 2 = 1
        assert_eq!(registers.read(Register::R0), 1);
    }

    #[test]
    fn load_updates_register() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();
        memory.write(PC_START + 5, 42);

        // Full instruction: LD (Opcode = 0b0010), Destination = R0, PC Offset = 5
        let instr = 0b0010_000_000000101;
        load(&mut registers, &mut memory, instr).unwrap();

        // R0 = memory[PC_START + 5] = 42
        assert_eq!(registers.read(Register::R0), 42);
    }

    #[test]
    fn store_updates_memory() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();
        registers.write(Register::R0, 99);

        // Full instruction: ST (Opcode = 0b0011), Source = R0, PC Offset = 5
        let instr = 0b0011_000_000000101;
        store(&mut registers, &mut memory, instr);

        // Check if memory at address PC_START + 5 contains the value 99
        assert_eq!(memory.read(PC_START + 5).unwrap(), 99);
    }

    #[test]
    fn jump_to_subroutine_with_long_flag_and_positive_offset() {
        let mut registers = Registers::new();

        // Full instruction: JSR (Opcode = 0b0100), Long Flag = 1, PC Offset = 5
        let instr = 0b0100_100000000101;
        jump_to_subroutine(&mut registers, instr);

        // PC should be updated to PC_START + 5
        assert_eq!(registers.read(Register::PC), PC_START + 5);
        // R7 should contain the old PC value, which is PC_START
        assert_eq!(registers.read(Register::R7), PC_START);
    }

    #[test]
    fn jump_to_subroutine_with_long_flag_and_negative_offset() {
        let mut registers = Registers::new();

        // Full instruction: JSR (Opcode = 0b0100), Long Flag = 1, PC Offset = -5
        let instr = 0b0100_111111111011;
        jump_to_subroutine(&mut registers, instr);

        // PC should be updated to PC_START - 5 = 0x2FFB
        assert_eq!(registers.read(Register::PC), 0x2FFB);
        // R7 should contain the old PC value, which is PC_START
        assert_eq!(registers.read(Register::R7), PC_START);
    }

    #[test]
    fn jump_to_subroutine_register_indirect() {
        let mut registers = Registers::new();
        registers.write(Register::R1, 0x4000);

        // Full instruction: JSRR (Opcode = 0b0100), Long Flag = 0, Base Register = R1
        let instr = 0b0100_000_001_000000;
        jump_to_subroutine(&mut registers, instr);

        // PC should be updated to the value in R1, which is 0x4000
        assert_eq!(registers.read(Register::PC), 0x4000);
        // R7 should contain the old PC value, which is PC_START
        assert_eq!(registers.read(Register::R7), PC_START);
    }

    #[test]
    fn and_with_immediate_value() {
        let mut registers = Registers::new();
        registers.write(Register::R1, 0b1010);

        // Full instruction: AND (Opcode = 0b0101), Destination = R0, Source = R1, Immediate Flag = 1, Immediate Value = 0b0101
        let instr = 0b0101_000_001_1_00101;
        and(&mut registers, instr);

        // R0 = R1 & 0b0101 = 0b1010 & 0b0101 = 0b0000
        assert_eq!(registers.read(Register::R0), 0b0000);
    }

    #[test]
    fn and_with_register() {
        let mut registers = Registers::new();
        registers.write(Register::R1, 0b1010);
        registers.write(Register::R2, 0b0110);

        // Full instruction: AND (Opcode = 0b0101), Destination = R0, Source1 = R1, Immediate Flag = 0, Source2 = R2
        let instr = 0b0101_000_001_0_00010;
        and(&mut registers, instr);

        // R0 = R1 & R2 = 0b1010 & 0b0110 = 0b0010
        assert_eq!(registers.read(Register::R0), 0b0010);
    }

    #[test]
    fn load_register_updates_register() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();
        memory.write(0x3008, 77);
        registers.write(Register::R1, PC_START);

        // Full instruction: LDR (Opcode = 0b0110), Destination = R0, Base Register = R1, Offset = 8
        let instr = 0b0110_000_001_001000;
        load_register(&mut registers, &mut memory, instr).unwrap();

        // R0 = memory[R1 + 8] = memory[PC_START + 8] = 77
        assert_eq!(registers.read(Register::R0), 77);
    }

    #[test]
    fn store_register_updates_memory() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();
        registers.write(Register::R0, 55);
        registers.write(Register::R1, 8);

        // Full instruction: STR (Opcode = 0b0111), Source = R0, Base Register = R1, Offset = 8
        let instr = 0b0111_000_001_001000;
        store_register(&mut registers, &mut memory, instr);

        // Check if memory at address R1 + 8 = 16 contains the value 55
        assert_eq!(memory.read(16).unwrap(), 55);
    }

    #[test]
    fn not_instruction_inverts_bits_correctly() {
        let mut registers = Registers::new();
        registers.write(Register::R1, 0b0000_1111);

        // Full instruction: NOT (Opcode = 0b1001), Destination = R0, Source = R1
        let instr = 0b1001_000_001_000000;
        not(&mut registers, instr);

        // R0 = ~R1 = ~0b0000_0000_0000_1111 = 0b1111_1111_1111_0000
        assert_eq!(registers.read(Register::R0), 0b1111_1111_1111_0000);
    }

    #[test]
    fn load_indirect_updates_register() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();
        memory.write(PC_START + 5, 0x4000);
        memory.write(0x4000, 88);

        // Full instruction: LDI (Opcode = 0b1010), Destination = R0, PC Offset = 5
        let instr = 0b1010_000_000000101;
        load_indirect(&mut registers, &mut memory, instr).unwrap();

        // R0 = memory[memory[PC_START + 5]] = memory[0x4000] = 88
        assert_eq!(registers.read(Register::R0), 88);
    }

    #[test]
    fn store_indirect_updates_memory() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();
        registers.write(Register::R0, 99);
        memory.write(PC_START + 5, 0x4000);

        // Full instruction: STI (Opcode = 0b1011), Source = R0, PC Offset = 5
        let instr = 0b1011_000_000000101;
        store_indirect(&mut registers, &mut memory, instr).unwrap();

        // Check if memory at address 0x4000 contains the value 99
        assert_eq!(memory.read(0x4000).unwrap(), 99);
    }

    #[test]
    fn jump_updates_pc() {
        let mut registers = Registers::new();
        registers.write(Register::R0, 0x4000);

        // Full instruction: JMP (Opcode = 0b1100), Base Register = R0
        let instr = 0b1100_000_000000000;
        jump(&mut registers, instr);

        // PC should be updated to 0x4000
        assert_eq!(registers.read(Register::PC), 0x4000);
    }

    #[test]
    fn load_effective_address_updates_register() {
        let mut registers = Registers::new();

        // Full instruction: LEA (Opcode = 0b1110), Destination = R0, PC Offset = 5
        let instr = 0b1110_000_000000101;
        load_effective_address(&mut registers, instr);

        // R0 should be updated to PC_START + 5
        assert_eq!(registers.read(Register::R0), PC_START + 5);
    }
}
