//! # Lesson 02: Task Scheduler with Atomics
//!
//! Split button and LED control into separate tasks using a simple scheduler.
//! Tasks communicate via atomic shared state (no locks needed!).
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
//! - Split monolithic code into separate tasks
//! - Use atomic types for lock-free shared state
//! - Implement a simple cooperative scheduler
//! - Task communication without allocations
//! - Organize code into modules for maintainability

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
use lesson_02_task_scheduler::{
    button, neopixel, scheduler::Scheduler, BUTTON_GPIO, NEOPIXEL_GPIO, RMT_CLOCK_MHZ,
};
use log::info;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    esp_println::println!("\n\n*** PANIC: {} ***\n", info);
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

// ============================================================================
// [USER TYPES] - Main application and scheduler loop
// ============================================================================
// DELETE the above comment line and type the code below in your video

#[main]
fn main() -> ! {
    // Initialize logging
    esp_println::logger::init_logger_from_env();
    log::set_max_level(log::LevelFilter::Info);

    info!("🚀 Starting Lesson 02: Task Scheduler with Atomics");

    // Initialize ESP32 peripherals
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // Configure button GPIO (GPIO9) as input with pull-up
    let button = Input::new(peripherals.GPIO9, InputConfig::default().with_pull(Pull::Up));
    info!("✓ Button configured on GPIO{}", BUTTON_GPIO);

    // Initialize RMT for NeoPixel control
    let rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(RMT_CLOCK_MHZ))
        .expect("Failed to init RMT");
    let mut led = SmartLedsAdapter::<
        { buffer_size(1) },
        Blocking,
        color_order::Rgb,
        Ws2812Timing,
    >::new_with_memsize(rmt.channel0, peripherals.GPIO8, 2)
    .expect("Failed to create SmartLedsAdapter");
    info!("✓ NeoPixel configured on GPIO{}", NEOPIXEL_GPIO);

    // Create scheduler
    let mut scheduler = Scheduler::new();

    info!("✓ Scheduler initialized\n");
    info!("Press button to toggle LED!\n");

    // ========================================================================
    // MAIN SCHEDULER LOOP
    // ========================================================================

    loop {
        scheduler.tick(
            &delay,
            || button::button_task(&button),
            || neopixel::led_task(&mut led),
        );
    }
}

// [END USER TYPES]
