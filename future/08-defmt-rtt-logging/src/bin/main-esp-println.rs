//! # Lesson 08: esp-println Baseline for Flash Size Comparison
//!
//! This is the **baseline version** using esp-println for logging.
//! Compare with main.rs (defmt) to see Flash size savings.
//!
//! **Expected Results:**
//! - esp-println version: ~150+ KB (includes format strings)
//! - defmt version: ~100 KB (format strings on host)
//! - Savings: ~30-40%

#![no_std]
#![no_main]

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

// Custom panic handler
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    esp_println::println!("PANIC!");
    loop {}
}

// ============================================================================
// MAIN
// ============================================================================

#[main]
fn main() -> ! {
    esp_println::println!("🚀 Starting Lesson 08: esp-println Baseline");

    let _peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    esp_println::println!("✓ Initialization complete");
    esp_println::println!("Demonstrating logging with esp-println...");

    let mut loop_count: u32 = 0;

    loop {
        // === Log basic counters ===
        if loop_count % 100 == 0 {
            esp_println::println!("Loop iteration: count={}", loop_count);
        }

        // === Demonstrate I2C transaction logging ===
        if loop_count % 200 == 0 {
            let i2c_tx = I2cTransaction {
                addr: 0x68,
                operation: I2cOperation::Read,
                bytes_transferred: 6,
                status: I2cStatus::Success,
            };
            esp_println::println!("I2C transaction: addr=0x{:02x}, bytes={}, status=success",
                                 i2c_tx.addr, i2c_tx.bytes_transferred);
        }

        // === Demonstrate GPIO event logging ===
        if loop_count % 300 == 0 {
            let gpio_evt = GpioEvent {
                pin: 9,
                state: GpioState::High,
                timestamp_us: loop_count * 10,
            };
            esp_println::println!("GPIO event: pin={}, state=high, timestamp={}",
                                 gpio_evt.pin, gpio_evt.timestamp_us);
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
            esp_println::println!("IMU reading: ax={}, ay={}, az={}, gx={}, gy={}, gz={}, temp={}",
                                 imu.accel_x, imu.accel_y, imu.accel_z,
                                 imu.gyro_x, imu.gyro_y, imu.gyro_z, imu.temp);
        }

        // === Demonstrate sensor status logging ===
        if loop_count % 500 == 0 {
            let sensor = SensorStatus {
                device_id: 0x68,
                is_healthy: true,
                error_count: 0,
                sample_count: loop_count / 5,
            };
            esp_println::println!("Sensor status: id=0x{:02x}, healthy=true, errors={}, samples={}",
                                 sensor.device_id, sensor.error_count, sensor.sample_count);
        }

        // === Demonstrate debug logging ===
        if loop_count % 1000 == 0 {
            esp_println::println!("Checkpoint reached at iteration {}", loop_count);
        }

        loop_count += 1;
        delay.delay_millis(10);
    }
}
