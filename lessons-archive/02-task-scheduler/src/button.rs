//! Button input handling with edge detection and debouncing.
//!
//! This module reads button state and detects press events (LOW → HIGH transition).
//! When a press is detected, it toggles the shared LED_ENABLED atomic.

use crate::{toggle_led_enabled, BUTTON_PERIOD_MS, DEBOUNCE_MS};
use esp_hal::gpio::Input;
use log::info;

/// Button state for edge detection
static mut BUTTON_WAS_PRESSED: bool = false;

/// Debounce counter - tracks calls since last press
static mut DEBOUNCE_COUNTER: u32 = 0;

/// Calculate how many task calls equal the debounce period
const DEBOUNCE_CALLS: u32 = (DEBOUNCE_MS as u64 / BUTTON_PERIOD_MS) as u32;

/// Button task: Read button state and update shared LED state
///
/// This function should be called periodically by the scheduler (every 10ms).
/// It detects button press events (transition from released to pressed)
/// and toggles the LED state atomically.
///
/// Uses time-based debouncing that doesn't block the scheduler.
///
/// # Arguments
/// * `button` - Reference to the GPIO input pin
pub fn button_task(button: &Input) {
    let button_pressed = button.is_low();

    unsafe {
        // Decrement debounce counter if active
        if DEBOUNCE_COUNTER > 0 {
            DEBOUNCE_COUNTER -= 1;
            // Update button state but don't process press
            BUTTON_WAS_PRESSED = button_pressed;
            return;
        }

        // Detect button press (transition to LOW, since button is active LOW)
        if button_pressed && !BUTTON_WAS_PRESSED {
            info!("📍 [button_task] Button press detected!");

            // Toggle LED state using atomic operation
            toggle_led_enabled();

            info!("📍 [button_task] LED toggled");

            // Start debounce period (non-blocking)
            DEBOUNCE_COUNTER = DEBOUNCE_CALLS;
        }

        // Update previous state for next edge detection
        BUTTON_WAS_PRESSED = button_pressed;
    }
}
