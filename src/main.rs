use lc3_vm::vm::VM;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run [image-file1] ...");
        exit(2);
    }

    let mut vm = VM::new();

    for path in &args[1..] {
        if let Err(msg) = vm.read_image_file(path) {
            eprintln!("Error: failed to load image '{}': {}", path, msg);
            exit(1);
        }
    }

    vm.run();
}
