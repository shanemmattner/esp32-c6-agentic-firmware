//! # Lesson 01: GDB-Only LED Blinky (Debugging Journey Edition)
//!
//! **The Reality:** Making an LED blink with GDB-only register writes is harder than it looks!
//!
//! This file contains BOTH versions:
//! 1. Working Rust HAL code (commented out) - to verify hardware
//! 2. Blank firmware - for GDB-only control experiments
//!
//! **The Debugging Journey:**
//! - We tried GDB register writes → didn't work
//! - We used Rust HAL to verify hardware → worked!
//! - We compared register states → found missing config
//! - We fixed GDB approach → finally worked!
//!
//! See DEBUGGING_JOURNEY.md for the full story.

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, gpio::{Level, Output, OutputConfig}, main};
use log::info;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // Initialize logging
    esp_println::logger::init_logger_from_env();
    log::set_max_level(log::LevelFilter::Info);

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // ========================================================================
    // WORKING LED BLINK CODE (Rust HAL) - COMMENTED OUT
    // ========================================================================
    // Uncomment this section to verify your hardware works!
    // This proves the LED circuit is correct before trying GDB-only control.

    // let mut led = Output::new(peripherals.GPIO12, Level::Low, OutputConfig::default());

    // info!("==============================================");
    // info!("  LED Blink Test (Rust HAL)");
    // info!("==============================================");
    // info!("Starting LED blink on GPIO12...");
    // info!("If LED blinks, hardware is good!");

    // loop {
    //     led.set_high();
    //     info!("LED ON (GPIO12 HIGH)");
    //     delay.delay_millis(500);

    //     led.set_low();
    //     info!("LED OFF (GPIO12 LOW)");
    //     delay.delay_millis(500);
    // }
    // ========================================================================

    // ========================================================================
    // BLANK FIRMWARE FOR GDB-ONLY CONTROL
    // ========================================================================
    // This version does NOT configure GPIO12.
    // We'll control it entirely through GDB register writes.
    //
    // **The Challenge:** Can you make the LED blink using only GDB?
    //
    // Hint: You'll need more than just ENABLE and OUT registers!
    // See DEBUGGING_JOURNEY.md for the solution.

    info!("==============================================");
    info!("  Lesson 01: GDB-Only Blinky");
    info!("==============================================");
    info!("");
    info!("Blank firmware running - GPIO12 NOT configured");
    info!("Use GDB to control LED via register writes");
    info!("");
    info!("Quick test commands:");
    info!("  (gdb) set *(uint32_t*)0x60091024 = 0x1000  # Enable");
    info!("  (gdb) set *(uint32_t*)0x60091008 = 0x1000  # LED ON");
    info!("  (gdb) set *(uint32_t*)0x6009100C = 0x1000  # LED OFF");
    info!("");
    info!("If LED doesn't work, see DEBUGGING_JOURNEY.md!");
    info!("==============================================");

    let mut loop_count: u32 = 0;
    loop {
        delay.delay_millis(500);
        loop_count = loop_count.wrapping_add(1);

        // Periodic status (helps verify GDB breakpoints are hitting)
        if loop_count % 10 == 0 {
            info!("Loop iteration: {}", loop_count);
        }
    }
}
