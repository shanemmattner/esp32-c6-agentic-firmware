//! # Lesson 01: GDB-Only LED Blinky
//!
//! **The Challenge:** Make an LED blink using ONLY GDB commands.
//!
//! This firmware is intentionally minimal - just an infinite loop with delays.
//! You will control the LED entirely through GDB register manipulation.
//!
//! **What You'll Learn:**
//! - Memory-mapped I/O fundamentals
//! - GDB register read/write capabilities
//! - ESP32-C6 GPIO peripheral control
//! - Hardware abstraction vs. direct control
//! - Agentic development with Claude Code
//!
//! **Hardware:**
//! - ESP32-C6 DevKit with onboard LED (GPIO8)
//! - USB cable for debugging via USB-JTAG
//!
//! **No LED control code here - that's the point!**

#![no_std]
#![no_main]

use esp_hal::{delay::Delay, main};
use esp_backtrace as _;
use log::info;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // Initialize logging
    esp_println::logger::init_logger_from_env();
    log::set_max_level(log::LevelFilter::Info);

    // Initialize peripherals (but don't configure GPIO yet)
    let _peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    info!("==============================================");
    info!("  Lesson 01: GDB-Only Blinky");
    info!("==============================================");
    info!("");
    info!("Firmware is running.");
    info!("No LED control code in this firmware!");
    info!("");
    info!("Now attach GDB and control GPIO8 directly:");
    info!("  1. Enable GPIO8 output: set *(uint32_t*)0x60091024 = 0x100");
    info!("  2. Turn ON:  set *(uint32_t*)0x60091008 = 0x100");
    info!("  3. Turn OFF: set *(uint32_t*)0x6009100C = 0x100");
    info!("");
    info!("See GPIO_REGISTERS.md for full register map.");
    info!("==============================================");

    // Infinite loop - provides timing for GDB breakpoint automation
    let mut loop_count: u32 = 0;
    loop {
        delay.delay_millis(500);
        loop_count = loop_count.wrapping_add(1);

        // This log helps you verify GDB breakpoints are working
        if loop_count % 10 == 0 {
            info!("Loop iteration: {}", loop_count);
        }
    }
}
