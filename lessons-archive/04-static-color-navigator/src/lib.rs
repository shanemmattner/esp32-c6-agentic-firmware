//! Lesson 04: Statig Color Navigator
//!
//! Interactive color control using statig state machine, combining button input,
//! IMU tilt sensing, and NeoPixel LED output.

#![no_std]

use core::sync::atomic::{AtomicU32, Ordering};

// ============================================================================
// MODULE EXPORTS
// ============================================================================

pub mod button;
pub mod color;
pub mod mpu9250;
pub mod scheduler;
pub mod state_machine;

// ============================================================================
// PIN CONFIGURATION
// ============================================================================

pub const BUTTON_GPIO: u8 = 9;
pub const NEOPIXEL_GPIO: u8 = 8;
pub const I2C_SDA_GPIO: u8 = 2;
pub const I2C_SCL_GPIO: u8 = 11;

// ============================================================================
// TASK TIMING
// ============================================================================

pub const BUTTON_PERIOD_MS: u64 = 10;
pub const IMU_PERIOD_MS: u64 = 100;
pub const LED_PERIOD_MS: u64 = 50;
pub const TICK_MS: u64 = 10;
pub const DEBOUNCE_MS: u32 = 200;
pub const DEBOUNCE_CALLS: u32 = (DEBOUNCE_MS as u64 / BUTTON_PERIOD_MS) as u32;

// ============================================================================
// SHARED STATE (Atomic)
// ============================================================================

/// Current LED color as packed RGB (0x00RRGGBB)
pub static CURRENT_COLOR: AtomicU32 = AtomicU32::new(0x00000000);

/// Get current LED color as (R, G, B)
pub fn get_led_color() -> (u8, u8, u8) {
    let packed = CURRENT_COLOR.load(Ordering::Relaxed);
    let r = ((packed >> 16) & 0xFF) as u8;
    let g = ((packed >> 8) & 0xFF) as u8;
    let b = (packed & 0xFF) as u8;
    (r, g, b)
}

/// Set LED color from (R, G, B)
pub fn set_led_color(r: u8, g: u8, b: u8) {
    let packed = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
    CURRENT_COLOR.store(packed, Ordering::Relaxed);
}
