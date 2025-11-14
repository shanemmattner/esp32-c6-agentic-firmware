# Lesson 05: Unit and Integration Testing

Testing embedded Rust code: host-based unit tests for pure functions and device integration tests for hardware.

## Learning Objectives

- Understand unit tests (host) vs integration tests (device)
- Write host-based unit tests for pure functions
- Create device integration tests for hardware
- Use `#[cfg_attr(not(test), no_std)]` for testable code
- Separate testable logic from hardware dependencies

## Project Structure

```
lessons/05-unit-and-integration-testing/
├── src/
│   ├── lib.rs              # Library entry point
│   ├── color.rs            # HSV→RGB with tests
│   ├── rotation.rs         # Rotation algorithm with tests
│   └── state_machine.rs    # State machine with tests
├── src/bin/
│   └── integration_test.rs # Device I2C test
├── Cargo.toml
└── README.md
```

## Running Tests

### Host-Based Unit Tests (Fast!)

```bash
cd lessons/05-unit-and-integration-testing
mv .cargo .cargo.device && mv rust-toolchain.toml rust-toolchain.toml.device
cargo test --lib
mv .cargo.device .cargo && mv rust-toolchain.toml.device rust-toolchain.toml
```

**Result:** 28 tests pass (color, rotation, state machine)

### Device Integration Tests (Hardware Required)

```bash
cd lessons/05-unit-and-integration-testing
cargo run --release --bin integration_test
```

**Expected output:**
```
🧪 Integration Test: I2C Communication
Test 1: Initialize I2C...
  ✓ I2C peripheral initialized
Test 2: Read MPU9250 WHO_AM_I register...
  WHO_AM_I: 0x71
  ✓ Correct WHO_AM_I value
Test 3: Multiple I2C reads (reliability)...
  ✓ 10/10 reads successful
Integration tests complete!
```

## Code Examples

### Pure Function Testing (color.rs)

```rust
pub fn hsv_to_rgb(hsv: HsvColor) -> (u8, u8, u8) {
    // Pure function - no hardware dependencies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pure_red() {
        assert_eq!(hsv_to_rgb(HsvColor::new(0, 100, 100)), (255, 0, 0));
    }
}
```

### Algorithm Testing (rotation.rs)

```rust
pub fn calculate_rotation_angle(accel_x: i16, accel_y: i16) -> u32 {
    // Integer-only atan2 approximation
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_quadrant_1() {
        let angle = calculate_rotation_angle(1000, 1000);
        assert!((40..=50).contains(&angle));
    }
}
```

### State Machine Testing (state_machine.rs)

```rust
#[state_machine(initial = "State::off()")]
impl SimpleMachine {
    #[state]
    fn off(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::ButtonPressed => Transition(State::on()),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_toggle_on() {
        let mut sm = SimpleMachine::default().state_machine();
        sm.handle(&Event::ButtonPressed);
        assert_eq!(sm.state(), &State::on());
    }
}
```

### Integration Test (integration_test.rs)

```rust
// Test I2C communication on real hardware
let mut i2c = I2c::new(peripherals.I2C0, Config::default())
    .with_sda(peripherals.GPIO2)
    .with_scl(peripherals.GPIO11);

// Read WHO_AM_I from MPU9250
let mut buffer = [0u8; 1];
i2c.write_read(MPU9250_ADDR, &[WHO_AM_I_REG], &mut buffer)?;
assert_eq!(buffer[0], EXPECTED_WHO_AM_I);
```

## Key Concepts

### What's Testable?

| Code Type | Test Method | Speed | Hardware |
|-----------|-------------|-------|----------|
| Pure functions | Host tests | Milliseconds | No |
| Algorithms | Host tests | Milliseconds | No |
| State machines | Host tests | Milliseconds | No |
| Hardware I/O | Device tests | Seconds | Yes |
| I2C/SPI | Device tests | Seconds | Yes |

### Good Architecture (Testable)

```rust
// ✅ Pure logic - testable
fn calculate_color(angle: u32) -> (u8, u8, u8) { ... }

// ✅ Hardware - separate
fn read_sensor_and_update_led(i2c: &mut I2c, led: &mut Led) {
    let angle = read_angle_from_sensor(i2c);  // Hardware
    let (r, g, b) = calculate_color(angle);   // Tested!
    led.set_color(r, g, b);                   // Hardware
}
```

## Test-Driven Development (TDD)

### Why Test "Obvious" Logic?

**Regression Prevention** 🛡️
- You write perfect logic today
- Six months later you refactor and break it
- Tests catch the bug before shipping

**Forces Better Design** 🏗️
- "How do I test this?" leads to:
  - Separation of concerns (logic vs hardware)
  - Pure functions (deterministic)
  - Clear interfaces
  - Loose coupling

**Living Documentation**
- Tests show how code should work
- Always up-to-date (or they fail)

### TDD Workflow (Lesson 06+)

1. **Think about tests first** - What behavior do we want?
2. **Write the test** - Define expected behavior
3. **Watch it fail** - Confirm test catches missing functionality
4. **Implement** - Write minimum code to pass
5. **Verify** - Tests pass
6. **Refactor** - Improve while keeping tests green

### Keep It Simple

- **3-5 tests per module** (not 10-20!)
- Test main use cases, not every edge case
- Code must be type-able for live videos

## Test Coverage

**28 Total Tests:**
- Color: 12 tests (primary/secondary colors, grayscale, edge cases)
- Rotation: 12 tests (quadrants, cardinal directions, ranges)
- State Machine: 4 tests (transitions, multiple toggles)

## Benefits

- **Fast Feedback** - Tests run in milliseconds
- **Easy Debugging** - Standard Rust debuggers
- **CI/CD Ready** - No hardware needed for unit tests
- **Cost Effective** - No expensive test equipment
- **Confidence** - Refactor without fear

## Limitations (Host Testing)

Can't test on host:
- Hardware timing/delays
- GPIO electrical characteristics
- I2C/SPI protocols (only logic)
- Interrupts
- Real sensor data

**Solution:** Device integration tests (like integration_test.rs)

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Tests won't compile | Rename `.cargo/` and `rust-toolchain.toml` |
| `can't find crate for std` | Check `#![cfg_attr(not(test), no_std)]` in lib.rs |
| Integration test build fails | Ensure esp-hal features enabled |
| I2C test fails | Check hardware connections (GPIO2/11, MPU9250) |

## References

- [The Rust Book: Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Rust Embedded Book](https://docs.rust-embedded.org/book/)
- [Testing no_std code](https://blog.dbrgn.ch/2019/12/24/testing-for-no-std-compatibility/)

---

*Fast, reliable testing - the foundation of quality embedded code!* ✅
