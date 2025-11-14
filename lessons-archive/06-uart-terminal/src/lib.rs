//! # Lesson 06: UART Terminal
//!
//! Production UART serial terminal for debugging and firmware control.
//!
//! **Hardware:**
//! - ESP32-C6 development board
//! - MPU9250 9-DOF IMU module (I2C)
//! - WS2812 NeoPixel LED
//! - Push button (active LOW with pull-up)
//! - UART connection via USB-to-serial adapter
//!
//! **Pins:**
//! - GPIO9: Button input (active LOW)
//! - GPIO8: NeoPixel data (RMT)
//! - GPIO2: I2C SDA (MPU9250)
//! - GPIO11: I2C SCL (MPU9250)
//! - GPIO15: UART TX (transmit to PC)
//! - GPIO23: UART RX (receive from PC)
//!
//! **What You'll Learn:**
//! - UART communication with DMA
//! - Building CLI interfaces with menu crate
//! - Ring buffers with heapless
//! - Integrating multiple peripherals
//! - Command-driven firmware control

#![no_std]

// ============================================================================
// GPIO Pin Definitions
// ============================================================================

pub const BUTTON_GPIO: u8 = 9;
pub const NEOPIXEL_GPIO: u8 = 8;
pub const I2C_SDA_GPIO: u8 = 2;
pub const I2C_SCL_GPIO: u8 = 11;
pub const UART_TX_GPIO: u8 = 15;
pub const UART_RX_GPIO: u8 = 23;

pub const RMT_CLOCK_MHZ: u32 = 80;

// ============================================================================
// MPU9250 Constants (from Lesson 03)
// ============================================================================

pub const MPU9250_ADDR: u8 = 0x68;
pub const WHO_AM_I_REG: u8 = 0x75;
pub const EXPECTED_WHO_AM_I: u8 = 0x71;
pub const PWR_MGMT_1_REG: u8 = 0x6B;
pub const ACCEL_XOUT_H: u8 = 0x3B;
pub const GYRO_XOUT_H: u8 = 0x43;

// ============================================================================
// Modules
// ============================================================================

pub mod mpu9250;
pub mod button;
pub mod uart;
pub mod cli;
