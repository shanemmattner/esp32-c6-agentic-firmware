//! # Lesson 02: Simple Task Scheduler
//!
//! Demonstrates a basic cooperative task scheduler using hardware timers.
//!
//! **Hardware:**
//! - ESP32-C6 development board
//! - Same as Lesson 01 (no additional hardware)
//!
//! **Pins:**
//! - GPIO13: Output (LED)
//! - GPIO9: Input (reads GPIO13 state)
//!
//! **What You'll Learn:**
//! - Use ESP32-C6 SystemTimer for time tracking
//! - Build a simple cooperative task scheduler
//! - Run multiple "tasks" at different rates
//! - Organize code for future lessons

#![no_std]
#![no_main]

use esp_hal::{
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig},
    main,
};
use log::info;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

// ============================================================================
// PIN CONFIGURATION
// ============================================================================

const LED_PIN: u8 = 13;
const INPUT_PIN: u8 = 9;

// ============================================================================
// SIMPLE TASK SYSTEM
// ============================================================================

/// A task that runs at a fixed period
struct Task {
    run: fn(&mut Context),
    period_ms: u64,
    last_run: u64,
}

/// Context passed to all tasks - holds hardware references
struct Context {
    led: Output<'static>,
    input: Input<'static>,
}

impl Task {
    /// Check if this task should run based on current time
    fn should_run(&self, now: u64) -> bool {
        (now - self.last_run) >= self.period_ms
    }

    /// Execute the task and update last run time
    fn execute(&mut self, now: u64, ctx: &mut Context) {
        (self.run)(ctx);
        self.last_run = now;
    }
}

// ============================================================================
// TASK FUNCTIONS
// ============================================================================

/// Task 1: Blink the LED every 500ms
fn blink_task(ctx: &mut Context) {
    ctx.led.toggle();
    let state = if ctx.led.is_set_high() { "ON" } else { "OFF" };
    info!("[Blink] LED {}", state);
}

/// Task 2: Monitor GPIO9 state every 100ms
fn monitor_task(ctx: &mut Context) {
    let state = if ctx.input.is_high() { "HIGH" } else { "LOW" };
    info!("[Monitor] GPIO9: {}", state);
}

// ============================================================================
// MAIN
// ============================================================================

#[main]
fn main() -> ! {
    // Initialize logging
    esp_println::logger::init_logger_from_env();
    log::set_max_level(log::LevelFilter::Info);

    info!("🚀 Starting Lesson 02: Simple Task Scheduler\n");

    // Initialize hardware
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // Configure GPIO13 as output (LED)
    let led = Output::new(peripherals.GPIO13, Level::Low, OutputConfig::default());
    info!("✓ GPIO{} configured as output", LED_PIN);

    // Configure GPIO9 as input
    let input = Input::new(peripherals.GPIO9, InputConfig::default());
    info!("✓ GPIO{} configured as input", INPUT_PIN);

    info!("✓ Task scheduler ready\n");

    // Create task list - easy to add more tasks!
    let mut tasks = [
        Task {
            run: blink_task,
            period_ms: 500, // Run every 500ms
            last_run: 0,
        },
        Task {
            run: monitor_task,
            period_ms: 100, // Run every 100ms
            last_run: 0,
        },
    ];

    // Create context with hardware
    let mut ctx = Context { led, input };

    info!("🔄 Starting task scheduler loop...\n");

    // Manual time tracking - increments every 10ms
    let mut current_time_ms: u64 = 0;
    const TICK_MS: u64 = 10;

    // Simple cooperative scheduler - polls tasks in order
    loop {
        // Small delay to create 10ms ticks
        delay.delay_millis(TICK_MS as u32);
        current_time_ms += TICK_MS;

        // Check each task and run if period elapsed
        for task in &mut tasks {
            if task.should_run(current_time_ms) {
                task.execute(current_time_ms, &mut ctx);
            }
        }
    }
}
