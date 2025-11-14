//! Button input task with non-blocking debouncing
//!
//! Based on Lesson 02 button handling.

use crate::DEBOUNCE_CALLS;
use esp_hal::gpio::Input;

// ============================================================================
// DEBOUNCE STATE
// ============================================================================

static mut BUTTON_WAS_PRESSED: bool = false;
static mut DEBOUNCE_COUNTER: u32 = 0;

// ============================================================================
// BUTTON TASK
// ============================================================================

/// Button task with edge detection and debouncing
///
/// Returns `true` if a button press was detected (rising edge)
pub fn button_task(button: &Input) -> bool {
    let button_pressed = button.is_low();

    unsafe {
        // If in debounce period, just decrement counter and return
        if DEBOUNCE_COUNTER > 0 {
            DEBOUNCE_COUNTER -= 1;
            BUTTON_WAS_PRESSED = button_pressed;
            return false;
        }

        // Detect button press (transition to LOW, since button is active LOW)
        if button_pressed && !BUTTON_WAS_PRESSED {
            DEBOUNCE_COUNTER = DEBOUNCE_CALLS; // Start debounce period
            BUTTON_WAS_PRESSED = button_pressed;
            return true; // Button press detected!
        }

        BUTTON_WAS_PRESSED = button_pressed;
        false
    }
}
