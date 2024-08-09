use nix::sys::select::{select, FdSet};
use nix::sys::time::{TimeVal, TimeValLike};
use std::io::{self, Read};
use std::os::unix::io::RawFd;
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

const STDIN_FD: RawFd = 0; // File descriptor for standard input

pub fn disable_input_buffering() -> io::Result<Termios> {
    let original_tio = Termios::from_fd(STDIN_FD)?;
    let mut new_tio = original_tio;
    new_tio.c_lflag &= !(ICANON | ECHO);
    tcsetattr(STDIN_FD, TCSANOW, &new_tio)?;
    Ok(original_tio)
}

pub fn restore_input_buffering(original_tio: &Termios) -> io::Result<()> {
    tcsetattr(STDIN_FD, TCSANOW, original_tio)
}

pub fn check_key() -> bool {
    let mut read_fds = FdSet::new();
    read_fds.insert(STDIN_FD);
    let mut timeout = TimeVal::zero();

    match select(None, Some(&mut read_fds), None, None, Some(&mut timeout)) {
        Ok(n) => n > 0 && read_fds.contains(STDIN_FD),
        Err(_) => false,
    }
}

// Reads a single character from stdin
pub fn getchar() -> Result<u16, String> {
    let mut buffer = [0u8; 1];
    io::stdin()
        .read_exact(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer[0] as u16)
}

pub fn sign_extend(x: u16, bit_count: usize) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x | (0xFFFF << bit_count)
    } else {
        x
    }
}
