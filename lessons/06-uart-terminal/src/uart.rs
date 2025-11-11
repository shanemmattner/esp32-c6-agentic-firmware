//! UART module for serial communication
//!
//! Simple blocking UART for terminal I/O.
//! Handles reading commands and writing responses.

use core::fmt;
use esp_hal::uart::Uart;
use esp_hal::Blocking;
use heapless::Vec;

pub const RX_BUFFER_SIZE: usize = 128;

/// Terminal state for line buffering
pub struct Terminal {
    rx_buffer: Vec<u8, RX_BUFFER_SIZE>,
}

impl Terminal {
    /// Create new terminal
    pub fn new() -> Self {
        Self {
            rx_buffer: Vec::new(),
        }
    }

    /// Write a string to UART
    pub fn write_str(&mut self, uart: &mut Uart<Blocking>, s: &str) -> Result<(), ()> {
        uart.write(s.as_bytes()).map(|_| ()).map_err(|_| ())
    }

    /// Write bytes to UART
    pub fn write_bytes(&mut self, uart: &mut Uart<Blocking>, data: &[u8]) -> Result<(), ()> {
        uart.write(data).map(|_| ()).map_err(|_| ())
    }

    /// Read a single byte (non-blocking)
    /// Returns None if no data available
    fn read_byte(&mut self, uart: &mut Uart<Blocking>) -> Option<u8> {
        let mut buf = [0u8; 1];
        match uart.read(&mut buf) {
            Ok(n) if n > 0 => Some(buf[0]),
            _ => None,
        }
    }

    /// Read until newline or buffer full
    /// Returns Some(line) when complete line received
    /// Returns None if line not yet complete
    pub fn read_line(&mut self, uart: &mut Uart<Blocking>) -> Option<Vec<u8, RX_BUFFER_SIZE>> {
        // Try to read bytes
        while let Some(byte) = self.read_byte(uart) {
            // Echo character back (for interactive terminal)
            let _ = self.write_bytes(uart, &[byte]);

            // Handle special characters
            match byte {
                b'\r' | b'\n' => {
                    // Newline - command complete
                    let _ = self.write_str(uart, "\r\n");
                    let line = self.rx_buffer.clone();
                    self.rx_buffer.clear();
                    return Some(line);
                }
                b'\x7F' | b'\x08' => {
                    // Backspace or DEL
                    if self.rx_buffer.pop().is_some() {
                        // Erase character on terminal
                        let _ = self.write_str(uart, "\x08 \x08");
                    }
                }
                0x20..=0x7E => {
                    // Printable ASCII
                    if self.rx_buffer.push(byte).is_err() {
                        // Buffer full
                        let _ = self.write_str(uart, "\r\n[Buffer full]\r\n");
                        let line = self.rx_buffer.clone();
                        self.rx_buffer.clear();
                        return Some(line);
                    }
                }
                _ => {
                    // Ignore other control characters
                }
            }
        }

        None
    }

    /// Show prompt
    pub fn prompt(&mut self, uart: &mut Uart<Blocking>) {
        let _ = self.write_str(uart, "> ");
    }
}

/// Helper: Convert Vec<u8> to &str
pub fn bytes_to_str(bytes: &[u8]) -> Result<&str, ()> {
    core::str::from_utf8(bytes).map_err(|_| ())
}

/// Helper function: write formatted string to UART
/// Returns Ok(()) if successful
pub fn uart_write_fmt(uart: &mut Uart<Blocking>, args: fmt::Arguments) -> Result<(), ()> {
    struct UartWriterTemp<'a, 'b> {
        uart: &'a mut Uart<'b, Blocking>,
    }

    impl<'a, 'b> fmt::Write for UartWriterTemp<'a, 'b> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.uart.write(s.as_bytes()).map(|_| ()).map_err(|_| fmt::Error)
        }
    }

    let mut writer = UartWriterTemp { uart };
    fmt::Write::write_fmt(&mut writer, args).map_err(|_| ())
}

/// Macro to write formatted text to UART
#[macro_export]
macro_rules! uwrite {
    ($uart:expr, $($arg:tt)*) => {
        $crate::uart::uart_write_fmt($uart, format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! uwriteln {
    ($uart:expr) => {
        $crate::uart::uart_write_fmt($uart, format_args!("\r\n"))
    };
    ($uart:expr, $($arg:tt)*) => {{
        let _ = $crate::uart::uart_write_fmt($uart, format_args!($($arg)*));
        $crate::uart::uart_write_fmt($uart, format_args!("\r\n"))
    }};
}
