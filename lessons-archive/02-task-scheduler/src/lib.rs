//! # Lesson 02: Task Scheduler Library
//!
//! Modular task-based architecture with atomic shared state.

#![no_std]

pub mod button;
pub mod neopixel;
pub mod scheduler;

use core::sync::atomic::{AtomicBool, Ordering};

// ============================================================================
// HARDWARE CONFIGURATION
// ============================================================================

/// GPIO pin for button input (BOOT button on ESP32-C6-DevKitC-1)
pub const BUTTON_GPIO: u8 = 9;

/// GPIO pin for NeoPixel data line
pub const NEOPIXEL_GPIO: u8 = 8;

/// RMT peripheral clock frequency (80 MHz)
pub const RMT_CLOCK_MHZ: u32 = 80;

// ============================================================================
// TASK TIMING CONFIGURATION
// ============================================================================

/// Button task period - how often to check button state (10ms for responsive input)
pub const BUTTON_PERIOD_MS: u64 = 10;

/// LED task period - how often to update NeoPixel (50ms, humans can't see flicker)
pub const LED_PERIOD_MS: u64 = 50;

/// Scheduler tick period (10ms base tick)
pub const TICK_MS: u64 = 10;

/// Debounce delay after button press detection (200ms)
pub const DEBOUNCE_MS: u32 = 200;

// ============================================================================
// LED COLOR CONFIGURATION
// ============================================================================

/// LED color when ON (dim blue)
pub const LED_COLOR_ON: (u8, u8, u8) = (0, 0, 30);

/// LED color when OFF (black)
pub const LED_COLOR_OFF: (u8, u8, u8) = (0, 0, 0);

// ============================================================================
// SHARED STATE - Atomic for lock-free communication
// ============================================================================

/// LED state shared between button_task and led_task
pub static LED_ENABLED: AtomicBool = AtomicBool::new(false);

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/// Read LED enabled state (atomic load)
#[inline]
pub fn is_led_enabled() -> bool {
    LED_ENABLED.load(Ordering::Relaxed)
}

/// Set LED enabled state (atomic store)
#[inline]
pub fn set_led_enabled(enabled: bool) {
    LED_ENABLED.store(enabled, Ordering::Relaxed)
}

/// Toggle LED enabled state (atomic read-modify-write)
#[inline]
pub fn toggle_led_enabled() {
    let current = LED_ENABLED.load(Ordering::Relaxed);
    LED_ENABLED.store(!current, Ordering::Relaxed);
}
