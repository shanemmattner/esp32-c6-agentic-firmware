//! # Lesson 01: Button + NeoPixel
//!
//! Press the button to toggle the NeoPixel on and off.
//!
//! **Hardware:**
//! - ESP32-C6 development board with onboard NeoPixel (WS2812)
//! - Onboard button
//!
//! **Pins:**
//! - GPIO9: Button input (active LOW - pressed = LOW)
//! - GPIO8: NeoPixel data line
//!
//! **What You'll Learn:**
//! - Digital input (button reading)
//! - NeoPixel/WS2812 control via RMT
//! - Simple button debouncing
//! - Basic state toggling

#![no_std]
#![no_main]

use esp_hal::{
    delay::Delay,
    gpio::{Input, InputConfig, Pull},
    main,
    rmt::Rmt,
    time::Rate,
    Blocking,
};
use esp_hal_smartled::{buffer_size, color_order, SmartLedsAdapter, Ws2812Timing};
use log::info;
use smart_leds::{SmartLedsWrite, RGB8};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    esp_println::println!("\n\n*** PANIC: {} ***\n", info);
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

// ============================================================================
// [USER TYPES] - Main application logic
// ============================================================================
// DELETE the above comment line and type the code below in your video

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    log::set_max_level(log::LevelFilter::Info);

    info!("Starting Lesson 01: Button + NeoPixel");

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // Configure button (GPIO9) as input with pull-up
    let button = Input::new(peripherals.GPIO9, InputConfig::default().with_pull(Pull::Up));
    info!("Button configured on GPIO9");

    // Initialize RMT for NeoPixel control
    let rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(80)).expect("Failed to init RMT");
    let mut led = SmartLedsAdapter::<{ buffer_size(1) }, Blocking, color_order::Rgb, Ws2812Timing>::new_with_memsize(
        rmt.channel0,
        peripherals.GPIO8,
        2,
    ).expect("Failed to create SmartLedsAdapter");
    info!("NeoPixel configured on GPIO8");

    info!("Press button to toggle LED!\n");

    // LED state
    let mut led_on = false;
    let mut button_was_pressed = false;

    loop {
        let button_pressed = button.is_low();

        // Detect button press (rising edge)
        if button_pressed && !button_was_pressed {
            // Toggle LED state
            led_on = !led_on;

            if led_on {
                // Turn on (blue, dimmed)
                led.write([RGB8::new(0, 0, 30)].into_iter()).unwrap();
                info!("LED ON (blue)");
            } else {
                // Turn off
                led.write([RGB8::new(0, 0, 0)].into_iter()).unwrap();
                info!("LED OFF");
            }

            // Simple debounce delay
            delay.delay_millis(200);
        }

        button_was_pressed = button_pressed;

        // Small delay to avoid busy-waiting
        delay.delay_millis(10);
    }
}

// [END USER TYPES]
