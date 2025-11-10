# Generate esp-hal 1.0.0 Driver Template

Generate a driver template for a peripheral using modern esp-hal 1.0.0 patterns.

## Task

Create a new driver implementation with the following requirements:

1. **Peripheral**: {Ask user which peripheral: I2C, SPI, UART, GPIO, etc.}
2. **Device**: {Ask for specific device if applicable, e.g., "BME280 sensor"}
3. **Features**: {Ask what functionality is needed}

## Template Structure

```rust
//! {Device} Driver using esp-hal 1.0.0
//!
//! This driver implements {peripheral} communication with {device}.

#![no_std]

use esp_hal::{peripheral}::{Peripheral, Config};
use log::{info, debug, warn, error};

pub struct {Device}<'d, P: Peripheral> {
    peripheral: P,
    config: Config,
}

impl<'d, P: Peripheral> {Device}<'d, P> {
    /// Create a new {device} driver
    pub fn new(peripheral: P, config: Config) -> Self {
        info!("Initializing {device} driver");

        Self {
            peripheral,
            config,
        }
    }

    /// Initialize the device
    pub fn init(&mut self) -> Result<(), Error> {
        debug!("Configuring {device}...");
        // TODO: Initialization logic
        info!("{device} initialized successfully");
        Ok(())
    }

    /// Read data from device
    pub fn read(&mut self) -> Result<Data, Error> {
        debug!("Reading from {device}");
        // TODO: Read implementation
        Ok(Data::default())
    }

    /// Write data to device
    pub fn write(&mut self, data: &Data) -> Result<(), Error> {
        debug!("Writing to {device}: {:?}", data);
        // TODO: Write implementation
        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    BusError,
    DeviceError,
    ConfigError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        // TODO: Add initialization test
    }

    #[test]
    fn test_read_write() {
        // TODO: Add read/write test
    }
}
```

## Requirements

1. Use **esp-hal 1.0.0** patterns (not esp-idf-hal)
2. Implement **embedded-hal 1.0** traits where applicable
3. Add **comprehensive logging** at INFO, DEBUG levels
4. Include **proper error handling** with Result types
5. Add **unit tests** for key functionality
6. Document with **rustdoc comments**
7. Use **type-safe** peripheral configuration

## Example Usage

Generate example usage code that shows:
- Initialization
- Basic operations
- Error handling
- Logging output

## Next Steps

After generating the template:
1. Implement TODO sections
2. Add integration tests
3. Test on hardware
4. Document in lesson README
