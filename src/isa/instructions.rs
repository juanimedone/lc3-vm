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

    if imm_flag != 0 {
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
