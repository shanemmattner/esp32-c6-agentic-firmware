//! # Blinky - Lesson 01
//!
//! The simplest possible ESP32-C6 firmware using esp-hal 1.0.0.
//! Blinks an LED every second to demonstrate basic GPIO control.
//!
//! ## Hardware
//! - ESP32-C6 development board
//! - External LED on GPIO13
//! - Input detector on GPIO9 (optional: connect to GPIO13 to verify toggling)
//!
//! ## What You'll Learn
//! - Basic esp-hal 1.0.0 initialization
//! - GPIO output configuration
//! - Delay timing
//! - Logging for debugging

#![no_std]
#![no_main]

// ============================================================================
// IMPORTS
// ============================================================================

use esp_backtrace as _;  // Panic handler - prints backtrace on crash
use esp_hal::{
    delay::Delay,                           // Blocking delay provider
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},    // GPIO types
    main,                                    // Entry point macro
};
use log::info;  // Logging macros

// Required for proper linker symbols - don't worry about this for now
esp_bootloader_esp_idf::esp_app_desc!();

// ============================================================================
// MAIN ENTRY POINT
// ============================================================================

/// Main firmware entry point
///
/// The #[main] attribute marks this as the entry point.
/// The `-> !` means this function never returns (runs forever).
#[main]
fn main() -> ! {
    // Step 1: Initialize logging
    // This must be done first so we can see debug output
    esp_println::logger::init_logger_from_env();
    // Ensure INFO level logging is enabled (default if env var not set)
    log::set_max_level(log::LevelFilter::Info);

    info!("🚀 Starting Blinky (Lesson 01)");

    // Step 2: Initialize the hardware abstraction layer
    // This gives us access to all ESP32-C6 peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default());
    info!("✓ HAL initialized");

    // Step 3: Configure GPIO13 as an output
    // - peripherals.GPIO13: The specific pin we want to use
    // - Level::Low: Start with LED off
    // - OutputConfig::default(): Standard push-pull output
    let mut led = Output::new(
        peripherals.GPIO13,
        Level::Low,
        OutputConfig::default()
    );
    info!("✓ GPIO13 configured as output");

    // Step 4: Configure GPIO9 as an input
    // - peripherals.GPIO9: The pin we want to read
    // - Pull::Up: Enable internal pull-up resistor
    // This pin can be used to detect toggling (e.g., connect to GPIO13 to verify output)
    let input = Input::new(peripherals.GPIO9, InputConfig::default());
    info!("✓ GPIO9 configured as input");

    // Step 5: Create a delay provider
    // Used for blocking delays (simple but blocks other code)
    let delay = Delay::new();

    // Step 6: Main loop - blink forever!
    info!("💡 Entering blink loop...");
    let mut count = 0;
    loop {
        led.set_high();              // Turn LED ON
        info!("🔴 LED ON  | GPIO9 input: {}", if input.is_high() { "HIGH" } else { "LOW " });
        delay.delay_millis(500);
        count += 1;
        info!("    ..continuing ON | GPIO9: {}", if input.is_high() { "HIGH" } else { "LOW " });
        delay.delay_millis(500);

        led.set_low();               // Turn LED OFF
        info!("⚫ LED OFF | GPIO9 input: {}", if input.is_high() { "HIGH" } else { "LOW " });
        delay.delay_millis(500);
        info!("    ..continuing OFF | GPIO9: {}", if input.is_high() { "HIGH" } else { "LOW " });
        delay.delay_millis(500);

        if count % 10 == 0 {
            info!("━━━━━━━━━━━━━━ Cycle #{} ━━━━━━━━━━━━━━", count / 2);
        }
    }

    // Note: We never reach here because of the infinite loop
}

// ============================================================================
// LEARNING NOTES
// ============================================================================

// 🎓 Key Concepts:
//
// 1. #![no_std] - We don't use Rust's standard library (too big for embedded)
// 2. #![no_main] - We provide our own entry point, not the standard one
// 3. -> ! - The "never" type - function runs forever
// 4. loop {} - Infinite loop (required in embedded - can't return from main)
//
// 🔍 What's happening:
//
// 1. Panic handler (esp_backtrace) catches crashes and prints debug info
// 2. HAL init gives us safe access to hardware
// 3. GPIO Output controls a single pin
// 4. Delay blocks execution for specified time
// 5. Loop toggles LED state forever
//
// 💡 Best Practices Used:
//
// - Comprehensive comments for learning
// - Logging at key points
// - Descriptive variable names
// - Clear step-by-step progression
// - Error handling through types (no unwrap() needed!)
//
// 🎯 Next Steps:
//
// - Try changing the delay time
// - Try a different GPIO pin
// - Try using led.toggle() instead of set_high/set_low
// - Move on to Lesson 02 (async with Embassy)
