# LC-3 Virtual Machine

This project is an implementation of a simple LC-3 (Little Computer 3) virtual machine in Rust. It can load and execute LC-3 object files.

## Execution
### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

### How to run

1. Clone the repository.

2. Install the VM.
    ```bash
    cargo install --path .
    ```

3. Run the VM.

    You can run the VM with one or more LC-3 object files as arguments. 
    ```bash
    lc3-vm object-file1 object-file2 ...
    ```
    Replace `object-file1` and `object-file2` with the **paths** to your LC-3 object files.

    Example:
    ```bash
    lc3-vm assembly/rogue.obj
    ```
