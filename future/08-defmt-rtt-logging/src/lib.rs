//! Lesson 08: Structured Logging with defmt + RTT
//!
//! Custom defmt format implementations for machine-parseable structured logging.

#![no_std]

use defmt::Format;

// ============================================================================
// I2C Transaction Types
// ============================================================================

/// I2C Transaction - structured format for I2C communication events
#[derive(Format)]
pub struct I2cTransaction {
    pub addr: u8,
    pub operation: I2cOperation,
    pub bytes_transferred: u16,
    pub status: I2cStatus,
}

/// I2C Operation types
#[derive(Format)]
pub enum I2cOperation {
    Write,
    Read,
    WriteRead,
}

/// I2C Status codes
#[derive(Format)]
pub enum I2cStatus {
    Success,
    Timeout,
    Nack,
    BusError,
}

// ============================================================================
// GPIO State Types
// ============================================================================

/// GPIO State Change - structured format for GPIO events
#[derive(Format)]
pub struct GpioEvent {
    pub pin: u8,
    pub state: GpioState,
    pub timestamp_us: u32,
}

/// GPIO pin states
#[derive(Format)]
pub enum GpioState {
    High,
    Low,
    Interrupt,
}

// ============================================================================
// IMU Reading Types
// ============================================================================

/// IMU Reading - structured format for inertial measurement unit data
#[derive(Format)]
pub struct ImuReading {
    pub accel_x: i16,
    pub accel_y: i16,
    pub accel_z: i16,
    pub gyro_x: i16,
    pub gyro_y: i16,
    pub gyro_z: i16,
    pub temp: i16,
    pub timestamp_us: u32,
}

impl ImuReading {
    /// Create a new IMU reading with all zero values
    pub fn new() -> Self {
        Self {
            accel_x: 0,
            accel_y: 0,
            accel_z: 0,
            gyro_x: 0,
            gyro_y: 0,
            gyro_z: 0,
            temp: 0,
            timestamp_us: 0,
        }
    }

    /// Create IMU reading with specific values
    pub fn with_values(
        accel_x: i16,
        accel_y: i16,
        accel_z: i16,
        gyro_x: i16,
        gyro_y: i16,
        gyro_z: i16,
        temp: i16,
        timestamp_us: u32,
    ) -> Self {
        Self {
            accel_x,
            accel_y,
            accel_z,
            gyro_x,
            gyro_y,
            gyro_z,
            temp,
            timestamp_us,
        }
    }
}

// ============================================================================
// Sensor Status Types
// ============================================================================

/// Sensor Status - structured format for device health monitoring
#[derive(Format)]
pub struct SensorStatus {
    pub device_id: u8,
    pub is_healthy: bool,
    pub error_count: u16,
    pub sample_count: u32,
}
