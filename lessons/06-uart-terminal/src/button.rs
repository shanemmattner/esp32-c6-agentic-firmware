//! Button input handling with edge detection and debouncing.
//!
//! Simple button press detection for Lesson 06.

use esp_hal::gpio::Input;

/// Button state for edge detection
static mut BUTTON_WAS_PRESSED: bool = false;

/// Debounce counter - tracks calls since last press
static mut DEBOUNCE_COUNTER: u32 = 0;

/// Debounce period in task calls (10ms per call, 200ms debounce)
const DEBOUNCE_CALLS: u32 = 20;

/// Button task: Read button state and detect press events
///
/// Returns true if a button press was detected (transition from released to pressed).
/// This function should be called periodically by the scheduler (every 10ms).
///
/// # Arguments
/// * `button` - Reference to the GPIO input pin
///
/// # Returns
/// * `true` if button press detected, `false` otherwise
pub fn button_task(button: &Input) -> bool {
    let button_pressed = button.is_low();

    unsafe {
        // Decrement debounce counter if active
        if DEBOUNCE_COUNTER > 0 {
            DEBOUNCE_COUNTER -= 1;
            // Update button state but don't process press
            BUTTON_WAS_PRESSED = button_pressed;
            return false;
        }

        // Detect button press (transition to LOW, since button is active LOW)
        if button_pressed && !BUTTON_WAS_PRESSED {
            // Start debounce period (non-blocking)
            DEBOUNCE_COUNTER = DEBOUNCE_CALLS;

            // Update previous state for next edge detection
            BUTTON_WAS_PRESSED = button_pressed;

            return true;
        }

        // Update previous state for next edge detection
        BUTTON_WAS_PRESSED = button_pressed;
        false
    }
}
