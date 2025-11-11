//! # Lesson 03: MPU9250 IMU Sensor
//!
//! Reads accelerometer and gyroscope data from MPU9250 via I2C.
//!
//! **Hardware:**
//! - ESP32-C6 development board
//! - MPU9250 IMU module
//!
//! **Pins:**
//! - GPIO2: SDA (I2C data)
//! - GPIO11: SCL (I2C clock)
//!
//! **What You'll Learn:**
//! - I2C communication
//! - Reading IMU sensor data
//! - Task-based sensor polling
//! - Building on Lesson 02's task scheduler

#![no_std]
#![no_main]

use esp_hal::{
    delay::Delay,
    i2c::master::{I2c, Config as I2cConfig},
    main,
    Blocking,
};
use log::info;

use lesson_03_mpu9250::mpu9250;
use lesson_03_mpu9250::scheduler::{Context, Task};
use lesson_03_mpu9250::tasks::imu_task;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    esp_println::println!("\n\n*** PANIC: {} ***\n", info);
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

// ============================================================================
// [USER TYPES] - I2C initialization and scheduler loop
// ============================================================================
// DELETE the above comment line and type the code below in your video

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    log::set_max_level(log::LevelFilter::Info);

    info!("🚀 Starting Lesson 03: MPU9250 Sensor\n");

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // Initialize I2C (GPIO2=SDA, GPIO11=SCL)
    let mut i2c = I2c::new(peripherals.I2C0, I2cConfig::default())
        .expect("I2C init failed")
        .with_sda(peripherals.GPIO2)
        .with_scl(peripherals.GPIO11);

    info!("✓ I2C initialized (GPIO2=SDA, GPIO11=SCL)");

    // Wake up sensor
    if mpu9250::wake_sensor(&mut i2c).is_ok() {
        info!("✓ MPU9250 awake");
    }

    delay.delay_millis(100);

    // Read WHO_AM_I
    match mpu9250::read_who_am_i(&mut i2c) {
        Ok(id) => {
            info!("✓ WHO_AM_I: 0x{:02x}", id);
        }
        Err(_) => {
            info!("✗ Failed to read WHO_AM_I");
        }
    }

    info!("✓ Task scheduler ready\n");

    // Create task list
    let mut tasks: [Task<Blocking>; 1] = [Task {
        run: imu_task,
        period_ms: 500,
        last_run: 0,
    }];

    let mut ctx = Context { i2c: &mut i2c };

    info!("🔄 Starting sensor readings...\n");

    // Simple cooperative scheduler
    let mut current_time_ms: u64 = 0;
    const TICK_MS: u64 = 10;

    loop {
        delay.delay_millis(TICK_MS as u32);
        current_time_ms += TICK_MS;

        for task in &mut tasks {
            if task.should_run(current_time_ms) {
                task.execute(current_time_ms, &mut ctx);
            }
        }
    }
}

// [END USER TYPES]
