//! # Lesson 03: MPU9250 IMU Sensor
//!
//! Reads accelerometer and gyroscope data from MPU9250 via I2C.
//!
//! **Hardware:**
//! - ESP32-C6 development board
//! - MPU9250 IMU module
//!
//! **Pins:**
//! - GPIO11: SDA (I2C data)
//! - GPIO2: SCL (I2C clock)
//!
//! **What You'll Learn:**
//! - I2C communication
//! - Reading IMU sensor data
//! - Task-based sensor polling
//! - Clean code organization

#![no_std]
#![no_main]

use esp_hal::{
    delay::Delay,
    i2c::master::{I2c, Config as I2cConfig},
    main,
};
use log::info;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    esp_println::println!("\n\n*** PANIC: {} ***\n", info);
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    log::set_max_level(log::LevelFilter::Info);

    info!("Starting Lesson 03: MPU9250 Sensor");

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // Initialize I2C (GPIO2=SDA, GPIO11=SCL)
    let mut i2c = I2c::new(peripherals.I2C0, I2cConfig::default())
        .expect("I2C init failed")
        .with_sda(peripherals.GPIO2)
        .with_scl(peripherals.GPIO11);

    info!("I2C initialized (GPIO2=SDA, GPIO11=SCL)");

    // MPU9250 constants
    const MPU_ADDR: u8 = 0x68;
    const WHO_AM_I: u8 = 0x75;
    const PWR_MGMT_1: u8 = 0x6B;
    const ACCEL_XOUT_H: u8 = 0x3B;
    const GYRO_XOUT_H: u8 = 0x43;

    // Wake up sensor
    match i2c.write(MPU_ADDR, &[PWR_MGMT_1, 0x00]) {
        Ok(_) => info!("MPU9250 awake"),
        Err(e) => info!("Failed to wake MPU: {:?}", e),
    }

    delay.delay_millis(100);

    // Read WHO_AM_I
    let mut buf = [0u8; 1];
    match i2c.write_read(MPU_ADDR, &[WHO_AM_I], &mut buf) {
        Ok(_) => info!("WHO_AM_I: 0x{:02x}", buf[0]),
        Err(e) => info!("Failed WHO_AM_I: {:?}", e),
    }

    info!("Reading sensor data...\n");

    loop {
        // Read accelerometer
        let mut accel = [0u8; 6];
        match i2c.write_read(MPU_ADDR, &[ACCEL_XOUT_H], &mut accel) {
            Ok(_) => {
                let x = i16::from_be_bytes([accel[0], accel[1]]);
                let y = i16::from_be_bytes([accel[2], accel[3]]);
                let z = i16::from_be_bytes([accel[4], accel[5]]);
                info!("[Accel] X={:6} Y={:6} Z={:6}", x, y, z);
            }
            Err(e) => info!("[Accel] Error: {:?}", e),
        }

        // Read gyroscope
        let mut gyro = [0u8; 6];
        match i2c.write_read(MPU_ADDR, &[GYRO_XOUT_H], &mut gyro) {
            Ok(_) => {
                let x = i16::from_be_bytes([gyro[0], gyro[1]]);
                let y = i16::from_be_bytes([gyro[2], gyro[3]]);
                let z = i16::from_be_bytes([gyro[4], gyro[5]]);
                info!("[Gyro]  X={:6} Y={:6} Z={:6}", x, y, z);
            }
            Err(e) => info!("[Gyro] Error: {:?}", e),
        }

        info!("");
        delay.delay_millis(500);
    }
}
