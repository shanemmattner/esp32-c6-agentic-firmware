# Lesson 04: Statig State Machine - Color Navigator

Interactive color control using statig state machine library, combining button input, IMU tilt sensing, and NeoPixel LED output.

## Learning Objectives

- Using statig state machine library in no_std embedded Rust
- Event-driven architecture with button and IMU events
- HSV to RGB color conversion without floating point
- Combining multiple peripherals through state machine coordination
- Rotation angle calculation from accelerometer data

## Hardware Requirements

- ESP32-C6 development board
- MPU9250 9-DOF IMU module (I2C)
- WS2812 NeoPixel LED (onboard or external)
- Push button (onboard or external)
- USB-C cable

### Pin Configuration

```
Component        ESP32-C6
─────────────────────────
Button      →    GPIO9 (active LOW with pull-up)
NeoPixel    →    GPIO8 (RMT)
MPU9250 SDA →    GPIO2 (I2C)
MPU9250 SCL →    GPIO11 (I2C)
```

## What You'll Learn

This lesson demonstrates:
- Statig hierarchical state machine with macro-based definition
- Event-driven architecture: Button and IMU generate events
- State transitions on button press
- Rotation-based color calculation using accelerometer X/Y axes
- Manual HSV→RGB conversion for embedded systems
- Lock-free atomic shared state for inter-task communication
- Non-blocking cooperative scheduler

## Build & Flash

```bash
cd lessons/04-statig-color-navigator

# Build
cargo build --release

# Flash to ESP32-C6
cargo run --release
```

## Expected Output

```
Starting Lesson 04: Static Color Navigator

✓ I2C initialized (GPIO2=SDA, GPIO11=SCL)
✓ MPU9250 awake
✓ WHO_AM_I: 0x71
✓ Button configured (GPIO9, active LOW)
✓ NeoPixel initialized (GPIO8)
✓ State machine initialized

Starting interactive loop...

IMU: accel_x=15976, accel_y=4968
LED: HSV(14°, 100%, 35%) → RGB(89, 21, 0)

[User presses button]
Event: ButtonPressed
Transition: WarmPalette → CoolPalette

IMU: accel_x=11824, accel_y=11396
LED: HSV(203°, 100%, 35%) → RGB(0, 13, 89)

[Continuous updates as board rotates...]
```

## Interaction

**How to use:**
- **Rotate the board** in any direction → Color smoothly changes through palette
- **Press button** → Switch between Warm and Cool color palettes

**Color Palettes:**
- **Warm Palette**: Red → Orange → Yellow → Yellow-Green (0-120° hue)
- **Cool Palette**: Cyan → Blue → Purple → Magenta (180-300° hue)

**Brightness**: Fixed at 35% for comfortable viewing

## Code Structure

- `src/bin/main.rs` - Main application with manual scheduler
- `src/state_machine.rs` - Statig state machine with Warm/Cool palettes
- `src/color.rs` - Manual HSV→RGB conversion with unit tests
- `src/button.rs` - Non-blocking button debouncing (from Lesson 02)
- `src/mpu9250.rs` - MPU9250 I2C driver (from Lesson 03)
- `src/lib.rs` - Constants and atomic shared state
- `Cargo.toml` - Dependencies including statig 0.3 with macro feature

## Key Concepts

### Statig State Machine

```rust
#[state_machine(
    initial = "State::warm_palette()",
    state(derive(Debug)),
    on_transition = "Self::on_transition"
)]
impl ColorNavigator {
    #[state]
    fn warm_palette(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::ButtonPressed => Transition(State::cool_palette()),
            Event::ImuUpdate { accel_x, accel_y } => {
                update_warm_palette(*accel_x, *accel_y);
                Handled
            }
        }
    }

    #[state]
    fn cool_palette(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::ButtonPressed => Transition(State::warm_palette()),
            Event::ImuUpdate { accel_x, accel_y } => {
                update_cool_palette(*accel_x, *accel_y);
                Handled
            }
        }
    }
}
```

### Rotation Angle Calculation

Calculates rotation angle from X/Y accelerometer values using integer-only approximation of atan2:

```rust
fn calculate_rotation_angle(accel_x: i16, accel_y: i16) -> u32 {
    // Maps X-Y plane to 0-360 degrees without floating point
    // Uses quadrant detection and ratio approximation
    // Returns smooth rotation tracking for color control
}
```

### HSV to RGB Conversion

Manual conversion without floating point, suitable for embedded systems:
- Input: Hue (0-360°), Saturation (0-100%), Value (0-100%)
- Output: RGB values (0-255)
- Algorithm: Sector-based calculation with integer math
- Unit tested for correctness

## Troubleshooting

| Issue | Possible Cause | Solution |
|-------|---|---|
| LED not changing color | MPU9250 not responding | Check I2C wiring (GPIO2/11) |
| Button not working | Pull-up missing | Verify GPIO9 configured with Pull::Up |
| LED too bright/dim | Brightness hardcoded | Adjust brightness value in state_machine.rs |
| Jerky color changes | Sensor noise | Add filtering/smoothing to IMU readings |
| No state transitions | Button debouncing issue | Check DEBOUNCE_MS constant |

## Task Scheduling

The main loop uses a simple cooperative scheduler:
- **Tick period**: 10ms
- **Button task**: Runs every 10ms (debounced)
- **IMU task**: Runs every 100ms (throttled logging)
- **LED task**: Runs every 50ms (updates NeoPixel)

All tasks are non-blocking to maintain responsive control.

## Next Steps

- Add more color palettes (Rainbow, Fire, Ocean)
- Implement Z-axis tilt for saturation control
- Add gyroscope data for gesture recognition
- Create animations triggered by motion patterns

## References

- [statig crate documentation](https://docs.rs/statig/latest/statig/)
- [statig GitHub](https://github.com/mdeloof/statig)
- [MPU9250 Datasheet](https://invensense.tdk.com/wp-content/uploads/2015/02/MPU-9250-Datasheet.pdf)
- [HSV color model](https://en.wikipedia.org/wiki/HSL_and_HSV)
- [esp-hal I2C Documentation](https://docs.rs/esp-hal/latest/esp_hal/i2c/index.html)

---

*State machines meet motion sensing - interactive embedded control!* 🎨
