//! # Lesson 08: USB CDC High-Speed Data Streaming
//!
//! Demonstrates high-speed structured logging using USB CDC (virtual serial port)
//! instead of RTT. Achieves 1.5 MB/s bandwidth with machine-parseable format.
//!
//! **Hardware:**
//! - ESP32-C6 development board
//! - USB-C cable (for power and data)
//!
//! **What You'll Learn:**
//! - USB CDC streaming for high-speed data transmission
//! - Structured logging with machine-parseable format
//! - Real-time data visualization with Python parser
//! - Performance analysis and bandwidth optimization

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, main};
use esp_println::println;
use lesson_08_usb_cdc_streaming::{
    BootInfo, GpioEvent, GpioState, I2cOperation, I2cStatus, I2cTransaction, SensorReading,
};

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // Print boot information
    let boot_info = BootInfo {
        version: "1.0.0",
        chip: "ESP32-C6",
    };
    println!("{}", boot_info);

    // Initialize hardware
    let _peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    println!("STATUS|msg=Initialization complete|ready=true");

    let mut loop_count: u32 = 0;
    let mut timestamp_ms: u64 = 0;

    loop {
        // Simulate different event types at different rates

        // I2C transaction every 100ms
        if loop_count % 10 == 0 {
            let i2c_tx = I2cTransaction {
                addr: 0x68,
                operation: I2cOperation::Read,
                bytes_transferred: 6,
                status: I2cStatus::Success,
                timestamp_ms,
            };
            println!("{}", i2c_tx);
        }

        // GPIO event every 250ms
        if loop_count % 25 == 0 {
            let gpio_event = GpioEvent {
                pin: 8,
                state: if loop_count % 50 == 0 {
                    GpioState::High
                } else {
                    GpioState::Low
                },
                timestamp_ms,
            };
            println!("{}", gpio_event);
        }

        // Sensor reading every 500ms
        if loop_count % 50 == 0 {
            let sensor = SensorReading {
                sensor_id: 1,
                value: 2530 + ((loop_count / 10) % 100) as i32, // Simulated temperature
                unit: "centi-C",
                timestamp_ms,
            };
            println!("{}", sensor);
        }

        // Heartbeat every 1 second
        if loop_count % 100 == 0 {
            println!("HEARTBEAT|count={}|ts={}", loop_count / 100, timestamp_ms);
        }

        loop_count += 1;
        timestamp_ms += 10; // Increment by 10ms
        delay.delay_millis(10); // 100 Hz loop rate
    }
}
