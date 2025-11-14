//! Lesson 05: Unit and Integration Testing
//!
//! This library demonstrates testable embedded code:
//! - Pure functions with host-based unit tests
//! - Hardware code tested manually on device
//!
//! Run host tests: `cargo test --lib`

#![cfg_attr(not(test), no_std)]

pub mod color;
pub mod rotation;
pub mod state_machine;

// Re-export commonly used types
pub use color::{hsv_to_rgb, HsvColor};
pub use rotation::calculate_rotation_angle;
pub use state_machine::{Event, SimpleMachine};

// GPIO pin constants
pub const BUTTON_GPIO: u8 = 9;
pub const NEOPIXEL_GPIO: u8 = 8;
pub const I2C_SDA_GPIO: u8 = 2;
pub const I2C_SCL_GPIO: u8 = 11;

// MPU9250 I2C constants
pub const MPU9250_ADDR: u8 = 0x68;
pub const WHO_AM_I_REG: u8 = 0x75;
pub const EXPECTED_WHO_AM_I: u8 = 0x71;
