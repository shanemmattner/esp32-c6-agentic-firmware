//! # Lesson 08: defmt + RTT Structured Logging
//!
//! Learn structured logging with defmt and Real-Time Transfer (RTT).
//!
//! **What You'll Learn:**
//! - Replace esp-println with defmt for 9x Flash savings
//! - Structured logging (machine-parseable)
//! - Real-Time Transfer (RTT) for high-speed logging
//! - Zero-overhead logging

#![no_std]
#![no_main]

use defmt::{info, warn, error, debug};  // Import only what we need, NOT *
use defmt_rtt as _;

use esp_hal::{
    delay::Delay,
    main,
};

// Import structured logging types from lib
use lesson_08_defmt_rtt_logging::{
    I2cTransaction, I2cOperation, I2cStatus,
    GpioEvent, GpioState,
    ImuReading,
    SensorStatus,
};

// defmt timestamp
defmt::timestamp!("{=u64:us}", {
    0  // Placeholder
});

// Custom panic handler for defmt (uses built-in panic_handler attribute)
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    error!("PANIC: {}", defmt::Debug2Format(info));
    loop {}
}

// ============================================================================
// MAIN
// ============================================================================

#[main]
fn main() -> ! {
    info!("🚀 Starting Lesson 08: defmt + RTT Structured Logging");

    let _peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    info!("✓ Initialization complete");
    info!("Demonstrating structured logging with defmt + custom types...");

    let mut loop_count: u32 = 0;

    loop {
        // === Log basic counters (machine-parseable) ===
        if loop_count % 100 == 0 {
            info!("Loop iteration: count={=u32}", loop_count);
        }

        // === Demonstrate I2C transaction logging ===
        if loop_count % 200 == 0 {
            let i2c_tx = I2cTransaction {
                addr: 0x68,
                operation: I2cOperation::Read,
                bytes_transferred: 6,
                status: I2cStatus::Success,
            };
            info!("I2C transaction: {}", i2c_tx);
        }

        // === Demonstrate GPIO event logging ===
        if loop_count % 300 == 0 {
            let gpio_evt = GpioEvent {
                pin: 9,
                state: GpioState::High,
                timestamp_us: loop_count * 10,
            };
            info!("GPIO event: {}", gpio_evt);
        }

        // === Demonstrate IMU reading logging ===
        if loop_count % 400 == 0 {
            let imu = ImuReading::with_values(
                256,   // accel_x
                512,   // accel_y
                -128,  // accel_z
                10,    // gyro_x
                20,    // gyro_y
                -5,    // gyro_z
                25,    // temp (°C)
                loop_count * 10,
            );
            info!("IMU reading: {}", imu);
        }

        // === Demonstrate sensor status logging ===
        if loop_count % 500 == 0 {
            let sensor = SensorStatus {
                device_id: 0x68,
                is_healthy: true,
                error_count: 0,
                sample_count: loop_count / 5,
            };
            warn!("Sensor status: {}", sensor);
        }

        // === Demonstrate debug logging ===
        if loop_count % 1000 == 0 {
            debug!("Checkpoint reached at iteration {=u32}", loop_count);
        }

        loop_count += 1;
        delay.delay_millis(10);
    }
}
