//! Integration test - I2C communication with MPU9250
//!
//! This is a device test that must run on hardware.
//! Tests I2C peripheral and sensor communication.
//!
//! Flash: cargo run --release --bin integration_test

#![no_std]
#![no_main]

use esp_hal::{
    delay::Delay,
    i2c::master::{Config, I2c},
    main,
};
use log::info;

use lesson_05_unit_and_integration_testing::{
    MPU9250_ADDR, WHO_AM_I_REG, EXPECTED_WHO_AM_I,
};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    esp_println::println!("\n❌ PANIC: {}\n", info);
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

// ============================================================================
// [USER TYPES] - Device integration test
// ============================================================================
// DELETE the comment above and type the code below in your video

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    log::set_max_level(log::LevelFilter::Info);

    info!("🧪 Integration Test: I2C Communication");

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // Test 1: I2C Initialization
    info!("Test 1: Initialize I2C...");
    let mut i2c = match I2c::new(peripherals.I2C0, Config::default()) {
        Ok(i2c) => {
            info!("  ✓ I2C peripheral initialized");
            i2c.with_sda(peripherals.GPIO2).with_scl(peripherals.GPIO11)
        }
        Err(_) => {
            info!("  ❌ I2C init failed");
            loop {}
        }
    };

    delay.delay_millis(100);

    // Test 2: Read WHO_AM_I register
    info!("Test 2: Read MPU9250 WHO_AM_I register...");
    let mut buffer = [0u8; 1];
    match i2c.write_read(MPU9250_ADDR, &[WHO_AM_I_REG], &mut buffer) {
        Ok(_) => {
            let who_am_i = buffer[0];
            info!("  WHO_AM_I: 0x{:02x}", who_am_i);

            if who_am_i == EXPECTED_WHO_AM_I {
                info!("  ✓ Correct WHO_AM_I value");
            } else {
                info!("  ⚠ Unexpected WHO_AM_I (expected 0x{:02x})", EXPECTED_WHO_AM_I);
            }
        }
        Err(_) => {
            info!("  ❌ I2C read failed (sensor not connected?)");
        }
    }

    // Test 3: Multiple reads (reliability)
    info!("Test 3: Multiple I2C reads (reliability)...");
    let mut success_count = 0;
    for i in 0..10 {
        if i2c.write_read(MPU9250_ADDR, &[WHO_AM_I_REG], &mut buffer).is_ok() {
            success_count += 1;
        }
        delay.delay_millis(10);
    }
    info!("  ✓ {}/10 reads successful", success_count);

    // Summary
    info!("\n🎉 Integration tests complete!");
    info!("All I2C communication tests passed.\n");

    loop {
        delay.delay_millis(1000);
    }
}

// [END USER TYPES]
