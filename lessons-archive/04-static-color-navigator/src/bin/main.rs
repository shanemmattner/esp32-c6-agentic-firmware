//! # Lesson 04: Statig State Machine - Color Navigator
//!
//! Interactive color control using statig state machine library.
//!
//! **Hardware:**
//! - ESP32-C6 development board
//! - MPU9250 9-DOF IMU module (I2C)
//! - WS2812 NeoPixel LED
//! - Push button (active LOW with pull-up)
//!
//! **Pins:**
//! - GPIO9: Button input (active LOW)
//! - GPIO8: NeoPixel data (RMT)
//! - GPIO2: I2C SDA (MPU9250)
//! - GPIO11: I2C SCL (MPU9250)
//!
//! **What You'll Learn:**
//! - Using statig state machine library in no_std embedded Rust
//! - Event-driven architecture with button and IMU events
//! - HSV to RGB color conversion
//! - Combining multiple peripherals through state machine coordination
//!
//! **Interaction:**
//! - Button press: Cycle through base colors (Red → Green → Blue → Red)
//! - Tilt left/right: Adjust hue ±15° from base color
//! - Tilt forward/back: Adjust brightness 50-100%

#![no_std]
#![no_main]

use esp_hal::{
    delay::Delay,
    gpio::{Input, InputConfig, Pull},
    i2c::master::{Config as I2cConfig, I2c},
    main,
    rmt::Rmt,
    time::Rate,
    Blocking,
};
use esp_hal_smartled::{buffer_size, color_order, SmartLedsAdapter, Ws2812Timing};
use log::info;
use smart_leds::{SmartLedsWrite, RGB8};
use statig::prelude::*;

use lesson_04_static_color_navigator::{
    button, get_led_color, mpu9250,
    state_machine::{ColorNavigator, Event},
    BUTTON_GPIO, I2C_SCL_GPIO, I2C_SDA_GPIO, NEOPIXEL_GPIO,
};

// ============================================================================
// PANIC HANDLER
// ============================================================================

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    esp_println::println!("\n\n*** PANIC: {} ***\n", info);
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

// ============================================================================
// [USER TYPES] - Initialization and state machine loop
// ============================================================================
// DELETE the comment above and type the code below in your video

#[main]
fn main() -> ! {
    // Initialize logging
    esp_println::logger::init_logger_from_env();
    log::set_max_level(log::LevelFilter::Info);

    info!("🚀 Starting Lesson 04: Statig Color Navigator\n");

    // Initialize hardware
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // ========================================================================
    // Initialize I2C for MPU9250
    // ========================================================================

    let mut i2c = I2c::new(peripherals.I2C0, I2cConfig::default())
        .expect("I2C init failed")
        .with_sda(peripherals.GPIO2)
        .with_scl(peripherals.GPIO11);

    info!("✓ I2C initialized (GPIO{}=SDA, GPIO{}=SCL)", I2C_SDA_GPIO, I2C_SCL_GPIO);

    // Wake up MPU9250
    if mpu9250::wake_sensor(&mut i2c).is_ok() {
        info!("✓ MPU9250 awake");
    }

    delay.delay_millis(100);

    // Read WHO_AM_I to verify sensor
    match mpu9250::read_who_am_i(&mut i2c) {
        Ok(id) => {
            info!("✓ WHO_AM_I: 0x{:02x}", id);
        }
        Err(_) => {
            info!("⚠ Failed to read WHO_AM_I (continuing anyway)");
        }
    }

    // ========================================================================
    // Initialize Button (GPIO9, active LOW with pull-up)
    // ========================================================================

    let button = Input::new(peripherals.GPIO9, InputConfig::default().with_pull(Pull::Up));
    info!("✓ Button configured (GPIO{}, active LOW)", BUTTON_GPIO);

    // ========================================================================
    // Initialize NeoPixel (GPIO8, RMT)
    // ========================================================================

    let rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(80))
        .expect("Failed to init RMT");
    let mut led = SmartLedsAdapter::<
        { buffer_size(1) },
        Blocking,
        color_order::Rgb,
        Ws2812Timing,
    >::new_with_memsize(rmt.channel0, peripherals.GPIO8, 2)
    .expect("Failed to create SmartLedsAdapter");

    info!("✓ NeoPixel initialized (GPIO{})", NEOPIXEL_GPIO);

    // ========================================================================
    // Initialize State Machine
    // ========================================================================

    let mut state_machine = ColorNavigator::default().state_machine();
    info!("✓ State machine initialized\n");

    // Set initial color (Red base)
    state_machine.handle(&Event::ImuUpdate { accel_x: 0, accel_y: 0 });

    // ========================================================================
    // Initialize Manual Scheduler
    // ========================================================================

    info!("🔄 Starting interactive loop...\n");

    // Scheduler state
    let mut current_time_ms: u64 = 0;
    let mut button_next_run_ms: u64 = 0;
    let mut imu_next_run_ms: u64 = 0;
    let mut led_next_run_ms: u64 = 0;

    // IMU reading throttle counter
    let mut imu_log_counter = 0u32;

    const TICK_MS: u64 = 10;
    const BUTTON_PERIOD_MS: u64 = 10;
    const IMU_PERIOD_MS: u64 = 100;
    const LED_PERIOD_MS: u64 = 50;

    // ========================================================================
    // Main Loop
    // ========================================================================

    loop {
        // Tick
        delay.delay_millis(TICK_MS as u32);
        current_time_ms += TICK_MS;

        // Button task
        if current_time_ms >= button_next_run_ms {
            if button::button_task(&button) {
                info!("🔘 Event: ButtonPressed");
                state_machine.handle(&Event::ButtonPressed);
            }
            button_next_run_ms = current_time_ms + BUTTON_PERIOD_MS;
        }

        // IMU task
        if current_time_ms >= imu_next_run_ms {
            if let Ok(accel) = mpu9250::read_accel(&mut i2c) {
                // Log every 10th reading to reduce spam (once per second)
                if imu_log_counter % 10 == 0 {
                    info!("📊 IMU: accel_x={}, accel_y={}", accel.x, accel.y);
                }
                imu_log_counter = imu_log_counter.wrapping_add(1);

                // Send event to state machine
                state_machine.handle(&Event::ImuUpdate {
                    accel_x: accel.x,
                    accel_y: accel.y
                });
            }
            imu_next_run_ms = current_time_ms + IMU_PERIOD_MS;
        }

        // LED task
        if current_time_ms >= led_next_run_ms {
            let (r, g, b) = get_led_color();
            let _ = led.write([RGB8::new(r, g, b)].into_iter());
            led_next_run_ms = current_time_ms + LED_PERIOD_MS;
        }
    }
}

// [END USER TYPES]
