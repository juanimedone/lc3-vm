//! LC-3 Virtual Machine Library
//!
//! This library provides an implementation of the LC-3 (Little Computer 3) virtual machine.
//! It includes modules for handling the hardware components, instruction set architecture (ISA),
//! utility functions, and the virtual machine itself.

/// Hardware module for the LC-3 Virtual Machine.
///
/// This module contains the submodules for different hardware components
/// of the LC-3 VM, including flags, memory, and registers.
pub mod hardware;

/// Module for handling the instruction set architecture (ISA) of the LC-3 VM.
pub mod isa;

/// Module containing utility functions for handling input and bit manipulations used by the LC-3 VM.
pub mod utils;

/// Module implementing the main LC-3 VM functionalities.
pub mod vm;
