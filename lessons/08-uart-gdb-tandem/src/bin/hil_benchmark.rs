#![no_std]
#![no_main]

//! HIL Benchmark Firmware
//!
//! Sprint 1-3: Minimal UART command interface + GDB-inspectable variables
//! Hardware: ESP32-C6
//! UART: GPIO15=TX, GPIO23=RX @ 115200 baud
//! Port: /dev/cu.usbserial-FT58PFX4

use core::fmt::Write;
use esp_backtrace as _;
use esp_hal::{delay::Delay, main, uart::{Config, Uart}, Blocking};

esp_bootloader_esp_idf::esp_app_desc!();

// GDB-inspectable variables (Sprint 3)
#[no_mangle]
#[used]
static mut HIL_MODE: bool = false;

#[no_mangle]
#[used]
static mut TEST_COUNTER: u32 = 0;

#[no_mangle]
#[used]
static mut COMMAND_COUNT: u32 = 0;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // LED on GPIO8
    use esp_hal::gpio::{Output, Level, OutputConfig};
    let mut led = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());

    // UART on GPIO15=TX, GPIO23=RX @ 115200 baud
    let mut uart = Uart::new(peripherals.UART1, Config::default())
        .expect("Failed to init UART")
        .with_rx(peripherals.GPIO23)
        .with_tx(peripherals.GPIO15);

    // Startup banner
    writeln!(uart, "\r\n=== HIL Benchmark Firmware ===").ok();
    writeln!(uart, "Sprint 1-3: UART commands + GDB variables").ok();
    writeln!(uart, "GPIO15=TX, GPIO23=RX @ 115200 baud\r\n").ok();
    writeln!(uart, "Commands:").ok();
    writeln!(uart, "  PING       - Reply with PONG").ok();
    writeln!(uart, "  COUNTER    - Reply with counter value + increment").ok();
    writeln!(uart, "  STATUS     - Show GDB variables\r\n").ok();
    writeln!(uart, "> ").ok();

    let mut rx_buffer = heapless::Vec::<u8, 128>::new();
    let mut heartbeat_counter = 0u32;

    loop {
        // Heartbeat LED (blink every 500ms)
        if heartbeat_counter % 500 == 0 {
            led.toggle();
        }

        // Try to read UART (will return immediately if no data)
        let mut read_buf = [0u8; 1];
        if uart.read(&mut read_buf).is_ok() && read_buf[0] != 0 {
            let byte = read_buf[0];

            // Echo character back
            uart.write(&[byte]).ok();

            match byte {
                b'\r' | b'\n' => {
                    // Command complete
                    writeln!(uart, "").ok();

                    // Parse command
                    if let Ok(cmd_str) = core::str::from_utf8(&rx_buffer) {
                        let cmd_upper = cmd_str.trim();

                        // Manual uppercase (no_std doesn't have to_uppercase)
                        let mut cmd_buf = heapless::Vec::<u8, 128>::new();
                        for ch in cmd_upper.bytes() {
                            cmd_buf.push(ch.to_ascii_uppercase()).ok();
                        }
                        let cmd = core::str::from_utf8(&cmd_buf).unwrap_or("");

                        unsafe {
                            COMMAND_COUNT += 1;
                        }

                        match cmd {
                            "PING" => {
                                writeln!(uart, "PONG").ok();
                            }
                            "COUNTER" => {
                                unsafe {
                                    writeln!(uart, "COUNTER={}", TEST_COUNTER).ok();
                                    TEST_COUNTER += 1;
                                }
                            }
                            "STATUS" => {
                                unsafe {
                                    writeln!(uart, "HIL_MODE={}", HIL_MODE).ok();
                                    writeln!(uart, "TEST_COUNTER={}", TEST_COUNTER).ok();
                                    writeln!(uart, "COMMAND_COUNT={}", COMMAND_COUNT).ok();
                                }
                            }
                            "" => {
                                // Empty command, just show prompt
                            }
                            _ => {
                                writeln!(uart, "Unknown: {}", cmd_str).ok();
                            }
                        }
                    }

                    rx_buffer.clear();
                    write!(uart, "> ").ok();
                }
                b'\x7F' | b'\x08' => {
                    // Backspace
                    if rx_buffer.pop().is_some() {
                        uart.write(b"\x08 \x08").ok();
                    }
                }
                0x20..=0x7E => {
                    // Printable ASCII
                    if rx_buffer.push(byte).is_err() {
                        writeln!(uart, "\r\n[Buffer full]").ok();
                        rx_buffer.clear();
                        write!(uart, "> ").ok();
                    }
                }
                _ => {
                    // Ignore other control characters
                }
            }
        }

        delay.delay_millis(1);
        heartbeat_counter += 1;
    }
}
