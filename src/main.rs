use lc3_vm::vm::*;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: lc3 [image-file1] ...");
        exit(2);
    }

    let mut memory = initialize_memory();
    let mut reg = initialize_registers();
    reg[Register::COND as usize] = Flag::ZRO as u16;
    reg[Register::PC as usize] = PC_START;

    for path in &args[1..] {
        if let Err(_) = read_image(path, &mut memory) {
            eprintln!("Error: failed to load image: {}", path);
            exit(1);
        }
    }

    let mut running = true;
    while running {
        let pc = reg[Register::PC as usize];
        let instr = mem_read(pc, &memory);
        reg[Register::PC as usize] = pc.wrapping_add(1);
        let op = instr >> 12;

        match op {
            x if x == Opcode::ADD as u16 => {
                // Handle ADD
            }
            x if x == Opcode::AND as u16 => {
                // Handle AND
            }
            x if x == Opcode::NOT as u16 => {
                // Handle NOT
            }
            x if x == Opcode::BR as u16 => {
                // Handle BR
            }
            x if x == Opcode::JMP as u16 => {
                // Handle JMP
            }
            x if x == Opcode::JSR as u16 => {
                // Handle JSR
            }
            x if x == Opcode::LD as u16 => {
                // Handle LD
            }
            x if x == Opcode::LDI as u16 => {
                // Handle LDI
            }
            x if x == Opcode::LDR as u16 => {
                // Handle LDR
            }
            x if x == Opcode::LEA as u16 => {
                // Handle LEA
            }
            x if x == Opcode::ST as u16 => {
                // Handle ST
            }
            x if x == Opcode::STI as u16 => {
                // Handle STI
            }
            x if x == Opcode::STR as u16 => {
                // Handle STR
            }
            x if x == Opcode::TRAP as u16 => {
                // Handle TRAP
            }
            x if x == Opcode::RES as u16 || x == Opcode::RTI as u16 => {
                // Handle reserved and RTI
            }
            _ => {
                eprintln!("BAD OPCODE");
                running = false;
            }
        }
    }
}
