use crate::{flags::Flag, memory::Memory, registers::*};

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

fn sign_extend(x: u16, bit_count: usize) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x | (0xFFFF << bit_count)
    } else {
        x
    }
}

pub fn update_flags(registers: &mut Registers, reg: Register) {
    let value = registers.read(reg);
    if value == 0 {
        registers.write(Register::COND, Flag::ZRO as u16);
    } else if value >> 15 == 1 {
        // a 1 in the left-most bit indicates negative
        registers.write(Register::COND, Flag::NEG as u16);
    } else {
        registers.write(Register::COND, Flag::POS as u16);
    }
}

pub fn branch(registers: &mut Registers, instr: u16) {
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let instr_cond = (instr >> 9) & 0x7;
    let reg_cond = registers.read(Register::COND);

    if (instr_cond & reg_cond) != 0 {
        let pc = registers.read(Register::PC);
        registers.write(Register::PC, pc.wrapping_add(pc_offset));
    }
}

pub fn add(registers: &mut Registers, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        let result = registers.read(Register::from(r1)) + imm5;
        registers.write(Register::from(r0), result);
    } else {
        let r2 = instr & 0x7;
        let result = registers.read(Register::from(r1)) + registers.read(Register::from(r2));
        registers.write(Register::from(r0), result);
    }

    update_flags(registers, Register::from(r0));
}

pub fn load(registers: &mut Registers, memory: &mut Memory, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let pc = registers.read(Register::PC);

    let address = pc.wrapping_add(pc_offset);
    let value = memory.read(address);
    registers.write(Register::from(r0), value);

    update_flags(registers, Register::from(r0));
}

pub fn store(registers: &mut Registers, memory: &mut Memory, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let pc = registers.read(Register::PC);

    let address = pc.wrapping_add(pc_offset);
    let value = registers.read(Register::from(r0));
    memory.write(address, value);
}

pub fn jump_to_subroutine(registers: &mut Registers, instr: u16) {
    let long_flag = (instr >> 11) & 0x1;
    let current_pc = registers.read(Register::PC);

    // Save the return address (current PC) into R7
    registers.write(Register::R7, current_pc);

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

    update_flags(registers, Register::from(r0));
}

pub fn load_register(registers: &mut Registers, memory: &mut Memory, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);

    let base_address = registers.read(Register::from(r1));
    let final_address = base_address.wrapping_add(offset);
    let value = memory.read(final_address);
    registers.write(Register::from(r0), value);

    update_flags(registers, Register::from(r0));
}

pub fn store_register(registers: &mut Registers, memory: &mut Memory, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;
    let offset = sign_extend(instr & 0x3F, 6);

    let base_address = registers.read(Register::from(r1));
    let final_address = base_address.wrapping_add(offset);
    let value = registers.read(Register::from(r0));
    memory.write(final_address, value);
}

pub fn not(registers: &mut Registers, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let r1 = (instr >> 6) & 0x7;

    registers.write(Register::from(r0), !registers.read(Register::from(r1)));

    update_flags(registers, Register::from(r0));
}

pub fn load_indirect(registers: &mut Registers, memory: &mut Memory, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let pc = registers.read(Register::PC);

    let address = memory.read(pc.wrapping_add(pc_offset));
    let value = memory.read(address);
    registers.write(Register::from(r0), value);

    update_flags(registers, Register::from(r0));
}

pub fn store_indirect(registers: &mut Registers, memory: &mut Memory, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let pc = registers.read(Register::PC);

    let intermediate_address = pc.wrapping_add(pc_offset);
    let final_address = memory.read(intermediate_address);
    let value = registers.read(Register::from(r0));
    memory.write(final_address, value);
}

pub fn jump(registers: &mut Registers, instr: u16) {
    let r1 = (instr >> 6) & 0x7;
    registers.write(Register::PC, registers.read(Register::from(r1)));
}

pub fn load_effective_address(registers: &mut Registers, instr: u16) {
    let r0 = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);
    let pc = registers.read(Register::PC);

    let effective_address = pc.wrapping_add(pc_offset);
    registers.write(Register::from(r0), effective_address);

    update_flags(registers, Register::from(r0));
}
