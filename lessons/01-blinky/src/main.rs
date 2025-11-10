//! # Lesson 01: Blinky - Basic GPIO Output & Input
//!
//! The simplest ESP32-C6 firmware - demonstrates GPIO output and input.
//!
//! **Hardware:**
//! - ESP32-C6 development board
//! - Optional: LED connected to GPIO13
//!
//! **What You'll Learn:**
//! - Initialize esp-hal
//! - Configure GPIO as output
//! - Configure GPIO as input
//! - Control GPIO state
//! - Print logs to serial

#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig},
    main,
};
use log::info;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

// ============================================================================
// PIN CONFIGURATION
// ============================================================================

const LED_PIN: u8 = 13;        // GPIO13 - LED output
const INPUT_PIN: u8 = 9;       // GPIO9 - Input (detects LED state)
const BLINK_DELAY_MS: u32 = 500;

// ============================================================================
// MAIN
// ============================================================================

#[main]
fn main() -> ! {
    // Initialize logging
    esp_println::logger::init_logger_from_env();
    log::set_max_level(log::LevelFilter::Info);

    info!("🚀 Starting Lesson 01: Blinky");

    // Initialize hardware
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let delay = Delay::new();

    // Configure GPIO13 as output (LED)
    let mut led = Output::new(
        peripherals.GPIO13,
        Level::Low,
        OutputConfig::default(),
    );
    info!("✓ GPIO{} configured as output", LED_PIN);

    // Configure GPIO9 as input (detector)
    let input = Input::new(peripherals.GPIO9, InputConfig::default());
    info!("✓ GPIO{} configured as input", INPUT_PIN);

    info!("Starting GPIO demonstration...\n");

    // Demo: Turn LED ON and read input
    info!("--- GPIO Output Test ---");
    led.set_high();
    info!("Set GPIO13 HIGH");
    info!("  GPIO9 reads: {}", if input.is_high() { "HIGH" } else { "LOW" });
    delay.delay_millis(1000);

    // Demo: Turn LED OFF and read input
    led.set_low();
    info!("Set GPIO13 LOW");
    info!("  GPIO9 reads: {}", if input.is_high() { "HIGH" } else { "LOW" });
    delay.delay_millis(1000);

    info!("\n--- Blinking Loop ---");
    info!("(Check GPIO9 input state as GPIO13 toggles)\n");

    // Main loop: blink and show GPIO state
    let mut cycle = 0;
    loop {
        led.set_high();
        info!("🔴 LED ON  → GPIO9: {}", if input.is_high() { "HIGH" } else { "LOW" });
        delay.delay_millis(BLINK_DELAY_MS);

        led.set_low();
        info!("⚫ LED OFF → GPIO9: {}", if input.is_high() { "HIGH" } else { "LOW" });
        delay.delay_millis(BLINK_DELAY_MS);

        cycle += 1;
        if cycle % 10 == 0 {
            info!("  └─ {} cycles completed", cycle);
        }
    }
}
