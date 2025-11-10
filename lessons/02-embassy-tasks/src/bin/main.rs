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

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};
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

// ============================================================================
// SHARED STATE - Atomic for lock-free communication
// ============================================================================

/// LED state shared between button_task and led_task
static LED_ENABLED: AtomicBool = AtomicBool::new(false);

/// Button press detected flag
static BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    esp_println::println!("\n\n*** PANIC: {} ***\n", info);
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

// ============================================================================
// TASK FUNCTIONS
// ============================================================================

/// Button task: Read button state and update shared state
fn button_task(button: &Input, delay: &Delay) {
    static mut BUTTON_WAS_PRESSED: bool = false;

    let button_pressed = button.is_low();

    // Detect button press (LOW → HIGH transition)
    unsafe {
        if button_pressed && !BUTTON_WAS_PRESSED {
            info!("📍 [button_task] Button press detected!");

            // Toggle LED state using atomic
            let current = LED_ENABLED.load(Ordering::Relaxed);
            LED_ENABLED.store(!current, Ordering::Relaxed);

            info!("📍 [button_task] LED_ENABLED set to: {}", !current);

            // Debounce
            delay.delay_millis(200);
        }

        BUTTON_WAS_PRESSED = button_pressed;
    }
}

/// LED task: Read shared state and update NeoPixel
fn led_task(led: &mut SmartLedsAdapter<{ buffer_size(1) }, Blocking, color_order::Rgb, Ws2812Timing>) {
    let should_be_on = LED_ENABLED.load(Ordering::Relaxed);

    if should_be_on {
        let _ = led.write([RGB8::new(0, 0, 30)].into_iter());
        info!("💡 [led_task] LED ON");
    } else {
        let _ = led.write([RGB8::new(0, 0, 0)].into_iter());
        info!("⚫ [led_task] LED OFF");
    }
}

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    log::set_max_level(log::LevelFilter::Info);

    info!("🚀 Starting Lesson 02: Task Scheduler with Atomics");

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // Configure button (GPIO9) as input with pull-up
    let button = Input::new(peripherals.GPIO9, InputConfig::default().with_pull(Pull::Up));
    info!("✓ Button configured on GPIO9");

    // Initialize RMT for NeoPixel control
    let rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(80)).expect("Failed to init RMT");
    let mut led = SmartLedsAdapter::<{ buffer_size(1) }, Blocking, color_order::Rgb, Ws2812Timing>::new_with_memsize(
        rmt.channel0,
        peripherals.GPIO8,
        2,
    ).expect("Failed to create SmartLedsAdapter");
    info!("✓ NeoPixel configured on GPIO8");

    info!("✓ Scheduler initialized\n");
    info!("Press button to toggle LED!\n");

    // ========================================================================
    // SIMPLE COOPERATIVE SCHEDULER
    // ========================================================================

    let mut button_next_run_ms: u64 = 0;
    let mut led_next_run_ms: u64 = 0;
    let mut current_time_ms: u64 = 0;

    const BUTTON_PERIOD_MS: u64 = 10;   // Check button every 10ms
    const LED_PERIOD_MS: u64 = 50;      // Update LED every 50ms
    const TICK_MS: u64 = 10;            // Scheduler tick

    loop {
        current_time_ms += TICK_MS;
        delay.delay_millis(TICK_MS as u32);

        // Run button task if period elapsed
        if current_time_ms >= button_next_run_ms {
            button_task(&button, &delay);
            button_next_run_ms = current_time_ms + BUTTON_PERIOD_MS;
        }

        // Run LED task if period elapsed
        if current_time_ms >= led_next_run_ms {
            led_task(&mut led);
            led_next_run_ms = current_time_ms + LED_PERIOD_MS;
        }
    }
}
