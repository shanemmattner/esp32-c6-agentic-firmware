# Lesson 01: Blinky

Basic GPIO output and input with serial logging.

## Learning Objectives

- Configure GPIO pins as output and input
- Toggle GPIO output (blink LED)
- Read GPIO input state
- Use structured logging with `log` crate
- Understand basic embedded Rust project structure

## Hardware Requirements

- ESP32-C6 development board
- USB-C cable
- Optional: LED + resistor connected to GPIO13

### Pin Configuration

```
ESP32-C6
--------
GPIO13  -->  LED (or test with GPIO9 reading the state)
GPIO9   -->  Input (reads GPIO13 state for testing)
```

**Note**: No external hardware needed! GPIO9 can read GPIO13's state, allowing you to test without any wiring.

## What You'll Learn

This lesson demonstrates:
- GPIO output control (HIGH/LOW)
- GPIO input reading
- Structured logging with `info!()` macro
- Basic timing with `Delay`
- State detection between pins

## Build & Flash

```bash
cd lessons/01-blinky

# Build
cargo build --release

# Flash to ESP32-C6
cargo run --release
```

### Using Cargo Aliases (Faster)

```bash
cargo br   # build release
cargo ck   # check syntax only
cargo ff   # flash firmware (build + flash + monitor)
```

## Expected Output

When you flash and run this lesson, you should see:

```
🚀 Starting Lesson 01: Blinky
✓ GPIO13 configured as output
✓ GPIO9 configured as input
Starting GPIO demonstration...

--- GPIO Output Test ---
Set GPIO13 HIGH
  GPIO9 reads: HIGH
Set GPIO13 LOW
  GPIO9 reads: LOW

--- Blinking Loop ---

🔴 LED ON  → GPIO9: HIGH
⚫ LED OFF → GPIO9: LOW
🔴 LED ON  → GPIO9: HIGH
⚫ LED OFF → GPIO9: LOW
  └─ 10 cycles completed
```

## Code Structure

- `src/main.rs` - Main firmware implementation
- `Cargo.toml` - Project dependencies
- `.cargo/config.toml` - Build configuration with espflash runner
- `rust-toolchain.toml` - Rust toolchain specification
- `build.rs` - Build script for linker configuration

## Key Concepts

### GPIO Output

```rust
let mut led = Output::new(
    peripherals.GPIO13,
    Level::Low,           // Start with LED off
    OutputConfig::default(),
);
```

Control a pin's state (HIGH or LOW) to drive LEDs, relays, or other digital outputs.

### GPIO Input

```rust
let input = Input::new(peripherals.GPIO9, InputConfig::default());
let state = input.is_high();  // Returns true if HIGH, false if LOW
```

Read the state of a pin without needing external buttons or sensors.

### Structured Logging

```rust
info!("🚀 Starting Lesson 01: Blinky");     // Major milestones
info!("✓ GPIO{} configured as output", LED_PIN);  // Configuration steps
```

Using the `log` crate provides consistent, filterable logging across your firmware.

### Delays

```rust
delay.delay_millis(500);  // Wait 500 milliseconds
```

Simple blocking delays using CPU cycle counter. Good for basic timing, but blocks execution.

## Experiments

### Easy
1. Change `BLINK_DELAY_MS` to `250` for faster blinking
2. Add a counter to show how many blinks have occurred

### Medium
3. Blink 5 times, then pause for 2 seconds
4. Create an SOS pattern (morse code: · · · − − − · · ·)

### Advanced
5. Read GPIO9 state and change blink speed based on it
6. Add a third GPIO pin with different blink pattern

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Build fails | Ensure you're in `lessons/01-blinky/` directory |
| Can't find device | Check USB connection: `ls /dev/cu.*` or `ls /dev/ttyUSB*` |
| No serial output | Serial port may be different, check connection |
| LED doesn't blink | Verify GPIO13 wiring (or check GPIO9 reads state changes) |
| Permission denied | On Linux: `sudo usermod -a -G dialout $USER` (then logout/login) |

## Next Steps

- **Lesson 02**: Simple task scheduler - Run multiple tasks at different rates
- Experiment with different GPIO pins
- Try connecting an external LED to GPIO13

## References

- [esp-hal GPIO Module](https://docs.esp-rs.org/esp-hal/esp-hal/0.20.1/esp32c6/esp_hal/gpio/index.html)
- [ESP32-C6 Technical Reference](https://www.espressif.com/sites/default/files/documentation/esp32-c6_technical_reference_manual_en.pdf)
- [Rust Embedded Book](https://rust-embedded.github.io/book/)

---

*Your first ESP32-C6 embedded Rust firmware!* 🚀
