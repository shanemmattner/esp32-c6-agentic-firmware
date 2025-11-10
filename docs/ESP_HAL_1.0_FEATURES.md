# esp-hal 1.0.0 Feature Guide
## Modern Bare-Metal Rust for ESP32

This document highlights the **new patterns and features** introduced in esp-hal 1.0.0 (October 2024), the first officially supported bare-metal Rust HAL for ESP32 chips.

---

## 🆕 What's Different?

### The Old Way (esp-idf-hal)
```rust
// Requires ESP-IDF C toolchain installation
use esp_idf_hal::prelude::*;
use esp_idf_hal::peripherals::Peripherals;

let peripherals = Peripherals::take().unwrap();
let mut led = PinDriver::output(peripherals.pins.gpio8)?;
// Wraps C ESP-IDF functions
```

### The New Way (esp-hal 1.0.0)
```rust
// Pure Rust, no C dependencies!
use esp_hal::{main, gpio::{Output, Level, OutputConfig}};

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut led = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());
    // Direct hardware access, bare-metal
}
```

**Key Difference**: No ESP-IDF installation required, pure Rust all the way down!

---

## 🔥 Core Features

### 1. **Unified Init Pattern**
```rust
#[main]
fn main() -> ! {
    // Single init call configures everything
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // All peripherals available immediately
    let gpio8 = peripherals.GPIO8;
    let i2c0 = peripherals.I2C0;
    let spi2 = peripherals.SPI2;
}
```

**vs. Old**: Required separate initialization for each peripheral subsystem.

### 2. **Type-Safe GPIO**
```rust
// Compile-time pin validation
let mut led = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());

// Can't accidentally use wrong pin type - won't compile!
// let input = Output::new(input_only_pin, ...); // ERROR!
```

**Benefit**: Catch pin configuration errors at compile time, not runtime.

### 3. **embedded-hal 1.0 Traits**
```rust
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

// Standard traits work across all embedded platforms
impl OutputPin for Output<'_> {
    fn set_high(&mut self) -> Result<(), Self::Error> { ... }
    fn set_low(&mut self) -> Result<(), Self::Error> { ... }
}
```

**Benefit**: Write portable drivers that work on any embedded platform.

### 4. **Direct Peripheral Access**
```rust
// Access peripherals directly, no intermediate objects
let peripherals = esp_hal::init(esp_hal::Config::default());

let led = Output::new(peripherals.GPIO8, ...);  // Direct!
// vs old: io.pins.gpio8 (extra layer)
```

**Benefit**: Clearer code, less indirection.

---

## 🚀 New Capabilities

### Embassy Async Integration
```rust
use embassy_executor::Spawner;

#[embassy_executor::task]
async fn blink_task(mut led: Output<'static>) {
    loop {
        led.toggle();
        Timer::after_millis(1000).await;  // async!
    }
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let led = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());

    spawner.spawn(blink_task(led)).ok();
}
```

**Benefit**: Modern async/await for concurrent tasks without RTOS.

### DMA Everywhere
```rust
use esp_hal::dma::{Dma, DmaPriority};

let dma = Dma::new(peripherals.DMA);
let dma_channel = dma.channel0.configure(false, DmaPriority::Priority0);

// SPI with DMA
let spi = Spi::new(peripherals.SPI2, 1.MHz(), SpiMode::Mode0)
    .with_dma(dma_channel);

// I2C with DMA
let i2c = I2c::new(peripherals.I2C0, sda, scl, Config::default())
    .with_dma(dma_channel);
```

**Benefit**: Zero-copy transfers for all peripherals, better performance.

### RMT Peripheral Support
```rust
use esp_hal::rmt::{Rmt, TxChannelConfig, TxChannel};

let rmt = Rmt::new(peripherals.RMT, 80.MHz()).unwrap();
let channel = rmt.channel0.configure(
    peripherals.GPIO8,
    TxChannelConfig::default()
).unwrap();

// Control WS2812 addressable LEDs, IR transmitters, etc.
```

**Benefit**: Precise timing control for protocols like WS2812, IR.

---

## 📋 Common Patterns

### GPIO Output
```rust
use esp_hal::gpio::{Output, Level, OutputConfig};

let mut led = Output::new(
    peripherals.GPIO8,
    Level::Low,              // Initial state
    OutputConfig::default()  // Push-pull, normal drive
);

led.set_high();
led.set_low();
led.toggle();
```

### GPIO Input with Interrupt
```rust
use esp_hal::gpio::{Input, InputConfig, Pull, Event};

let mut button = Input::new(
    peripherals.GPIO9,
    InputConfig::default().with_pull(Pull::Up)
);

button.listen(Event::FallingEdge);
```

### I2C Communication
```rust
use esp_hal::i2c::{I2c, Config};

let i2c = I2c::new(
    peripherals.I2C0,
    sda_pin,
    scl_pin,
    Config::default()  // 100 kHz default
);

// Use embedded-hal traits
i2c.write(DEVICE_ADDR, &[0x12, 0x34])?;
let mut buf = [0u8; 2];
i2c.read(DEVICE_ADDR, &mut buf)?;
```

### SPI Communication
```rust
use esp_hal::spi::{Spi, SpiMode};

let spi = Spi::new(
    peripherals.SPI2,
    1.MHz(),
    SpiMode::Mode0
).with_pins(sck, mosi, miso, cs);

// Transfer data
let rx_data = spi.transfer(&[0x01, 0x02, 0x03])?;
```

### UART Serial
```rust
use esp_hal::uart::{Uart, Config};

let mut uart = Uart::new(
    peripherals.UART0,
    Config::default()
).with_pins(tx, rx);

// Read/write bytes
uart.write_bytes(b"Hello!\n")?;
let mut buf = [0u8; 64];
let len = uart.read_bytes(&mut buf)?;
```

### Delay
```rust
use esp_hal::delay::Delay;

let delay = Delay::new();

delay.delay_millis(1000);  // 1 second
delay.delay_micros(500);   // 500 microseconds
```

---

## 🎯 Best Practices

### 1. **Use Type-Safe Pins**
```rust
// Good: Type-safe, won't compile if wrong
let led = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());

// Bad: Runtime errors possible
// let pin = 8;
// gpio.set(pin, true); // Which GPIO? Runtime check needed
```

### 2. **Leverage embedded-hal Traits**
```rust
// Write generic drivers using traits
fn init_sensor<I2C>(i2c: I2C) -> Result<Sensor, Error>
where
    I2C: embedded_hal::i2c::I2c
{
    // Works with ANY I2C implementation!
}
```

### 3. **Use Logging Extensively**
```rust
use log::{info, debug, warn, error};

info!("Starting initialization...");
debug!("GPIO8 state: {}", led.is_set_high());
warn!("Temperature high: {}°C", temp);
error!("I2C communication failed!");
```

### 4. **Handle Errors Properly**
```rust
// Good: Handle all error cases
match i2c.write(addr, &data) {
    Ok(_) => info!("I2C write successful"),
    Err(e) => error!("I2C error: {:?}", e),
}

// Better: Use ? operator with proper error types
fn init() -> Result<(), esp_hal::i2c::Error> {
    i2c.write(addr, &data)?;
    Ok(())
}
```

---

## 🔬 Testing Patterns

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensor_calculation() {
        let raw = 1024;
        let temp = calculate_temperature(raw);
        assert_eq!(temp, 25.0);
    }
}
```

### Integration Tests (on-device)
```rust
#[cfg(feature = "integration-test")]
#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Test GPIO
    let mut led = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());
    led.set_high();
    assert!(led.is_set_high());

    info!("All tests passed!");
    loop {}
}
```

---

## 📊 Performance Characteristics

| Operation | esp-idf-hal (C) | esp-hal 1.0 (Rust) | Notes |
|-----------|-----------------|---------------------|-------|
| Binary Size | ~200KB | ~35KB | Bare-metal is smaller |
| GPIO Toggle | ~1μs | ~0.2μs | Direct register access |
| I2C Transfer | ESP-IDF overhead | Direct DMA | Faster with DMA |
| Startup Time | ESP-IDF boot | Immediate | No OS initialization |

---

## 🌟 Unique Features

### 1. **LP Core Support**
```rust
// Ultra-low-power core for background tasks
use esp_hal::lp_core::{LpCore, LpCoreConfig};

let lp_core = LpCore::new(peripherals.LP_CORE);
// Run code while main core sleeps
```

### 2. **TWAI (CAN Bus)**
```rust
use esp_hal::twai::{Twai, TwaiConfig};

let can = Twai::new(peripherals.TWAI0, tx, rx, TwaiConfig::default());
// Automotive CAN bus support
```

### 3. **HMAC/SHA/AES Hardware Acceleration**
```rust
use esp_hal::sha::Sha256;

let mut sha = Sha256::new(peripherals.SHA);
sha.update(&data);
let hash = sha.finish();
// Hardware-accelerated crypto
```

---

## 🚧 Migration from esp-idf-hal

| esp-idf-hal | esp-hal 1.0.0 |
|-------------|---------------|
| `Peripherals::take()` | `esp_hal::init()` |
| `PinDriver::output()` | `Output::new()` |
| `PinDriver::input()` | `Input::new()` |
| `I2cDriver::new()` | `I2c::new()` |
| `SpiDriver::new()` | `Spi::new()` |
| `FreeRtos::delay_ms()` | `Delay::new().delay_millis()` |

---

## 📚 Resources

- **Official Docs**: https://docs.esp-rs.org/esp-hal/
- **Examples**: https://github.com/esp-rs/esp-hal/tree/main/examples
- **Release Notes**: https://github.com/esp-rs/esp-hal/releases/tag/esp-hal-v1.0.0
- **Migration Guide**: https://docs.esp-rs.org/book/

---

**Last Updated**: 2025-11-09
**esp-hal Version**: 1.0.0
**Target**: ESP32-C6 (RISC-V)
