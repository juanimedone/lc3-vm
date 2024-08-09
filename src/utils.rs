use libc::{select, FD_ISSET, FD_SET, FD_ZERO};
use std::io::{self, Read};
use std::os::unix::io::RawFd;
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

const STDIN_FD: RawFd = 0; // File descriptor for standard input (stdin)

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
    let mut read_fds = unsafe { std::mem::zeroed() };
    unsafe { FD_ZERO(&mut read_fds) };
    unsafe { FD_SET(STDIN_FD, &mut read_fds) };

    let mut timeout = libc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };

    let result = unsafe {
        select(
            STDIN_FD + 1,
            &mut read_fds as *mut _,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut timeout,
        )
    };

    result > 0 && unsafe { FD_ISSET(STDIN_FD, &read_fds) }
}

// Reads a single character from stdin
pub fn getchar() -> io::Result<u16> {
    let mut buffer = [0u8; 1];
    io::stdin().read_exact(&mut buffer)?;
    Ok(buffer[0] as u16)
}

pub fn sign_extend(x: u16, bit_count: usize) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x | (0xFFFF << bit_count)
    } else {
        x
    }
}
