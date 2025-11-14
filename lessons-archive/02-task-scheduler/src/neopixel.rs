//! NeoPixel (WS2812) LED control using RMT peripheral.
//!
//! This module handles NeoPixel initialization and updates based on shared state.

use crate::{is_led_enabled, LED_COLOR_OFF, LED_COLOR_ON};
use esp_hal::Blocking;
use esp_hal_smartled::{buffer_size, color_order, SmartLedsAdapter, Ws2812Timing};
use log::info;
use smart_leds::{SmartLedsWrite, RGB8};

/// Type alias for the NeoPixel LED driver (requires lifetime parameter)
pub type NeoPixelDriver<'a> = SmartLedsAdapter<'a, { buffer_size(1) }, Blocking, color_order::Rgb, Ws2812Timing>;

/// LED task: Read shared state and update NeoPixel
///
/// This function should be called periodically by the scheduler.
/// It reads the LED_ENABLED atomic and updates the NeoPixel accordingly.
///
/// # Arguments
/// * `led` - Mutable reference to the NeoPixel driver
pub fn led_task<'a>(led: &mut SmartLedsAdapter<'a, { buffer_size(1) }, Blocking, color_order::Rgb, Ws2812Timing>) {
    let should_be_on = is_led_enabled();

    if should_be_on {
        let (r, g, b) = LED_COLOR_ON;
        let _ = led.write([RGB8::new(r, g, b)].into_iter());
        info!("💡 [led_task] LED ON");
    } else {
        let (r, g, b) = LED_COLOR_OFF;
        let _ = led.write([RGB8::new(r, g, b)].into_iter());
        info!("⚫ [led_task] LED OFF");
    }
}
