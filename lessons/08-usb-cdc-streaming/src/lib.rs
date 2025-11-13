#![no_std]

use core::fmt;

/// I2C transaction event for logging
#[derive(Debug, Clone, Copy)]
pub struct I2cTransaction {
    pub addr: u8,
    pub operation: I2cOperation,
    pub bytes_transferred: usize,
    pub status: I2cStatus,
    pub timestamp_ms: u64,
}

impl fmt::Display for I2cTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "I2C|addr=0x{:02x}|op={:?}|bytes={}|status={:?}|ts={}",
            self.addr, self.operation, self.bytes_transferred, self.status, self.timestamp_ms
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum I2cOperation {
    Read,
    Write,
}

#[derive(Debug, Clone, Copy)]
pub enum I2cStatus {
    Success,
    Nack,
    Timeout,
    Error,
}

/// GPIO event for logging
#[derive(Debug, Clone, Copy)]
pub struct GpioEvent {
    pub pin: u8,
    pub state: GpioState,
    pub timestamp_ms: u64,
}

impl fmt::Display for GpioEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "GPIO|pin={}|state={:?}|ts={}",
            self.pin, self.state, self.timestamp_ms
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GpioState {
    Low,
    High,
}

/// Generic sensor reading
#[derive(Debug, Clone, Copy)]
pub struct SensorReading {
    pub sensor_id: u8,
    pub value: i32,
    pub unit: &'static str,
    pub timestamp_ms: u64,
}

impl fmt::Display for SensorReading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SENSOR|id={}|value={}|unit={}|ts={}",
            self.sensor_id, self.value, self.unit, self.timestamp_ms
        )
    }
}

/// Boot information
#[derive(Debug)]
pub struct BootInfo {
    pub version: &'static str,
    pub chip: &'static str,
}

impl fmt::Display for BootInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BOOT|version={}|chip={}", self.version, self.chip)
    }
}
