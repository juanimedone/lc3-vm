//! Main module for the LC-3 Virtual Machine.
//!
//! This module handles the initialization and execution of the LC-3 VM, including
//! command-line argument parsing, input buffering, and error handling.

use lc3_vm::utils::*;
use lc3_vm::vm::VM;
use std::env;
use std::process::exit;

/// Entry point for the LC-3 Virtual Machine.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: lc3-vm [object-file1] ...");
        exit(2);
    }
    let mut vm = VM::new();

    // Disable input buffering for immediate input processing
    let original_tio = match disable_input_buffering() {
        Ok(tio) => tio,
        Err(e) => {
            eprintln!("Error disabling input buffering: {}", e);
            exit(1);
        }
    };

    for path in &args[1..] {
        if let Err(msg) = vm.read_image_file(path) {
            eprintln!("Error: failed to load image file '{}': {}", path, msg);
            restore_input_buffering(&original_tio).unwrap_or_else(|e| {
                eprintln!("Error restoring input buffering: {}", e);
                exit(1);
            });
            exit(1);
        }
    }
    if let Err(e) = vm.run() {
        eprintln!("Error while running the VM: {}", e)
    }

    if let Err(e) = restore_input_buffering(&original_tio) {
        eprintln!("Error restoring input buffering: {}", e);
        exit(1);
    }
}
