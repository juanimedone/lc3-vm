use std::io::{self, Read};
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

const STDIN_FD: i32 = 0; // File descriptor for standard input

/// Disables input buffering to allow immediate reading of input.
///
/// This function modifies the terminal settings to disable canonical mode and echoing.
/// It returns the original terminal settings, which should be used to restore the buffering later.
pub fn disable_input_buffering() -> io::Result<Termios> {
    let original_tio = Termios::from_fd(STDIN_FD)?;
    let mut new_tio = original_tio;
    new_tio.c_lflag &= !(ICANON | ECHO);
    tcsetattr(STDIN_FD, TCSANOW, &new_tio)?;
    Ok(original_tio)
}

/// Restores the original input buffering settings.
///
/// This function restores the terminal settings to their original state, as obtained from `disable_input_buffering`.
pub fn restore_input_buffering(original_tio: &Termios) -> io::Result<()> {
    tcsetattr(STDIN_FD, TCSANOW, original_tio)
}

/// Reads a single character from standard input.
///
/// This function reads exactly one byte from standard input and returns it as a `u16` value.
/// The function returns an error if reading from stdin fails.
pub fn getchar() -> Result<u16, String> {
    let mut buffer = [0u8; 1];
    io::stdin()
        .read_exact(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer[0] as u16)
}

/// Sign-extends a value to 16 bits based on its original bit count.
///
/// This function takes a value and extends it to 16 bits, preserving the sign based on the original bit count.
/// For example, if the original bit count is 8, this function will extend an 8-bit value to a 16-bit value.
pub fn sign_extend(x: u16, bit_count: usize) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        x | (0xFFFF << bit_count)
    } else {
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_extend_bit_is_0() {
        let value = 0b0000_0000_0011_1111;
        let bit_count = 7;
        let expected = 0b0000_0000_0011_1111;
        assert_eq!(sign_extend(value, bit_count), expected);
    }

    #[test]
    fn sign_extend_bit_is_1() {
        let value = 0b0000_0000_0011_1111;
        let bit_count = 6;
        let expected = 0b1111_1111_1111_1111;
        assert_eq!(sign_extend(value, bit_count), expected);
    }

    #[test]
    fn sign_extend_zero() {
        let value = 0b0000_0000_0000_0000;
        let bit_count = 1;
        let expected = 0b0000_0000_0000_0000;
        assert_eq!(sign_extend(value, bit_count), expected);
    }

    #[test]
    fn sign_extend_edge_case() {
        let value = 0b0000_0000_0000_0001;
        let bit_count = 1;
        let expected = 0b1111_1111_1111_1111;
        assert_eq!(sign_extend(value, bit_count), expected);
    }
}
