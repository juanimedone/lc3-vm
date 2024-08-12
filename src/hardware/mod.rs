/// Module for defining the condition flags used in the LC-3 Virtual Machine.
///
/// The LC-3 VM uses three condition flags to indicate the result of the last operation:
/// - Positive (POS)
/// - Zero (ZRO)
/// - Negative (NEG)
pub mod flags;

/// Module for memory management in the LC-3 Virtual Machine.
///
/// This module provides the `Memory` struct and related functionality
/// for reading from and writing to memory. It also includes support for
/// memory-mapped registers such as the keyboard status and data registers.
pub mod memory;

/// Module for managing the registers in the LC-3 Virtual Machine.
pub mod registers;
