//! # Simple UART-based Variable Streamer
//!
//! Uses UART (GPIO15=TX, GPIO23=RX) for both input and output.
//! No USB CDC complexity - just simple UART that we know works.

#![no_std]
#![no_main]

use core::fmt::Write;
use esp_backtrace as _;
use esp_hal::{delay::Delay, main, uart::{Config, Uart}, Blocking};

esp_bootloader_esp_idf::esp_app_desc!();

/// Test variables
static mut COUNTER: u32 = 0;
static mut TIMESTAMP: u64 = 0;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // LED on GPIO8 to show we're running
    use esp_hal::gpio::{Output, Level, OutputConfig};
    let mut led = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());

    // UART on GPIO15=TX, GPIO23=RX
    let mut uart = Uart::new(peripherals.UART1, Config::default())
        .expect("Failed to init UART")
        .with_rx(peripherals.GPIO23)
        .with_tx(peripherals.GPIO15);

    writeln!(uart, "BOOT|version=1.0.0|chip=ESP32-C6").ok();
    writeln!(uart, "READY").ok();

    let mut timestamp_ms: u64 = 0;

    loop {
        unsafe {
            COUNTER = COUNTER.wrapping_add(1);
            TIMESTAMP = timestamp_ms;
        }

        // Blink LED every 500ms
        if timestamp_ms % 500 == 0 {
            led.toggle();
        }

        // Heartbeat every second
        if timestamp_ms % 1000 == 0 {
            writeln!(uart, "HEARTBEAT|ts={}|counter={}", timestamp_ms, unsafe { COUNTER }).ok();
        }

        timestamp_ms += 10;
        delay.delay_millis(10);
    }
}
