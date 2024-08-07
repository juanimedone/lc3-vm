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

fn update_flags(registers: &mut Registers, reg: Register) {
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

pub fn add(registers: &mut Registers, instr: u16) {
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    let imm_flag = (instr >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        let result = registers.read(Register::from(sr1)) + imm5;
        registers.write(Register::from(dr), result);
    } else {
        let r2 = instr & 0x7;
        let result = registers.read(Register::from(sr1)) + registers.read(Register::from(r2));
        registers.write(Register::from(dr), result);
    }

    update_flags(registers, Register::from(dr));
}

pub fn ldi(registers: &mut Registers, memory: &Memory, instr: u16) {
    let dr = (instr >> 9) & 0x7;
    let pc_offset = sign_extend(instr & 0x1FF, 9);

    let pc = registers.read(Register::PC);
    let final_address = memory.read(pc.wrapping_add(pc_offset));
    let value = memory.read(final_address);

    registers.write(Register::from(dr), value);

    update_flags(registers, Register::from(dr));
}
