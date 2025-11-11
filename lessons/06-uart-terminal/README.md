# Lesson 06: UART Terminal

Interactive serial terminal for debugging and controlling ESP32-C6 firmware over UART.

## Learning Objectives

- UART serial communication with ESP32-C6
- Building command-line interfaces for embedded systems
- Integrating multiple peripherals (Button, LED, IMU, UART)
- Non-blocking command parsing and execution
- Streaming sensor data over serial

## Hardware Requirements

- ESP32-C6 development board
- MPU9250 9-DOF IMU module (I2C)
- USB-to-serial adapter (FTDI, CP2102, CH340, etc.)
- Jumper wires
- (Optional) Breadboard

## Pin Connections

| Component | ESP32-C6 Pin | Notes |
|-----------|--------------|-------|
| **UART TX** | GPIO15 | Connect to RX on USB-serial adapter |
| **UART RX** | GPIO23 | Connect to TX on USB-serial adapter |
| **UART GND** | GND | Common ground with USB-serial adapter |
| **Button** | GPIO9 | Active LOW (internal pull-up) |
| **NeoPixel** | GPIO8 | Onboard WS2812 LED |
| **I2C SDA** | GPIO2 | MPU9250 SDA |
| **I2C SCL** | GPIO11 | MPU9250 SCL |

**IMPORTANT:**
- GPIO15 (TX) on ESP32 → RX on USB-serial adapter
- GPIO23 (RX) on ESP32 → TX on USB-serial adapter
- Don't forget common GND connection!

## Building and Flashing

```bash
cd lessons/06-uart-terminal
cargo build --release
cargo run --release
```

## Connecting to the Terminal

Once flashed, connect via serial terminal at **115200 baud**:

### Option 1: screen (macOS/Linux)
```bash
screen /dev/ttyUSB0 115200
# or on macOS:
screen /dev/cu.usbserial-* 115200
```

Exit screen: `Ctrl-A` then `K`, then `Y`

### Option 2: minicom (Linux)
```bash
minicom -D /dev/ttyUSB0 -b 115200
```

### Option 3: PuTTY (Windows)
- Serial line: COM3 (or your port)
- Speed: 115200
- Connection type: Serial

## Expected Output

### Startup
```
==============================================
  ESP32-C6 Interactive Terminal
  Lesson 06: UART Terminal
==============================================

Type 'help' for available commands.

>
```

### Available Commands

Type `help` to see all commands:

```
Available Commands:
  help                    - Show this help
  status                  - Show system status
  reset                   - Reset system

  IMU Commands:
  imu_read                - Read accelerometer once
  imu_stream <hz>         - Stream IMU data (10, 50, 100 Hz)
  imu_stop                - Stop IMU streaming
  imu_range <g>           - Set accel range (2, 4, 8, 16)
  imu_filter <hz>         - Set filter bandwidth
  imu_status              - Show IMU configuration

  LED Commands:
  led_on                  - Turn on LED (blue)
  led_off                 - Turn off LED
  led_color <r> <g> <b>   - Set LED color (0-255)
```

## Command Examples

### Read IMU Once
```
> imu_read
📊 Accel: x=245, y=-102, z=16384
```

### Stream IMU Data
```
> imu_stream 50
✓ IMU streaming at 50 Hz
📊 245,-102,16384
📊 248,-99,16380
📊 242,-105,16388
...
> imu_stop
✓ IMU streaming stopped
```

### Control LED
```
> led_on
✓ LED ON

> led_color 255 0 128
✓ LED color set to R=255 G=0 B=128

> led_off
✓ LED OFF
```

### System Status
```
> status
System Status:
  LED: OFF
  LED Color: R=0 G=0 B=30
  IMU Streaming: DISABLED
```

### Check IMU
```
> imu_status
IMU Status:
  WHO_AM_I: 0x71
  Expected: 0x71
  Status: ✓ OK
```

## Interactive Features

- **Echo**: Characters are echoed back as you type
- **Backspace**: Delete characters with backspace/delete key
- **Line editing**: Edit your command before pressing Enter
- **Prompt**: `> ` indicates ready for input
- **Button integration**: Physical button press toggles LED and prints message

## Code Structure

This lesson demonstrates video production code organization:

```rust
// [SECTION 1/2: COPY-PASTE - Peripheral initialization]
// Boilerplate code - copy from starter code

// Global state and helper functions
static LED_ON: AtomicBool = ...
static IMU_STREAM_ENABLED: AtomicBool = ...

// [SECTION 2/2: USER TYPES - Main application]
// This is the part you type live in the video

#[main]
fn main() -> ! {
    // Initialize peripherals
    // Setup UART terminal
    // Main loop with command parsing
}

fn handle_command(...) {
    // Command dispatcher
}
```

The **USER TYPES** section (~140 lines) is designed to be typed live during video recording, while the global state section can be copy-pasted.

## Modules

- **uart.rs** - UART terminal with line buffering and echo
- **cli.rs** - Command parser with unit tests
- **button.rs** - Button debouncing (from Lesson 02)
- **mpu9250.rs** - IMU driver (from Lesson 03)

## Troubleshooting

| Issue | Solution |
|-------|----------|
| No output in terminal | Check baud rate (115200), TX/RX crossed correctly |
| Garbled characters | Wrong baud rate or bad connections |
| `Integration test not found` | This lesson uses `main.rs`, not `integration_test.rs` |
| IMU commands fail | Check I2C wiring (GPIO2=SDA, GPIO11=SCL) |
| LED doesn't change | Check GPIO8 NeoPixel connection |
| Can't type in terminal | Check RX connection (GPIO23 to adapter TX) |
| Terminal freezes during streaming | Use `imu_stop` to stop streaming, or Ctrl-C and reconnect |

### Finding Your Serial Port

**macOS:**
```bash
ls /dev/cu.*
```

**Linux:**
```bash
ls /dev/ttyUSB* /dev/ttyACM*
```

**Windows:**
- Device Manager → Ports (COM & LPT)

## Advanced: Baud Rate

Default: **115200 baud** (8N1 - 8 data bits, no parity, 1 stop bit)

To change, modify in `main.rs`:
```rust
let uart = Uart::new(peripherals.UART1, UartConfig::default())
    // Add custom config for different baud rate
```

## Testing Commands Without Hardware

The CLI parser has unit tests:
```bash
cargo test --lib
```

Tests verify command parsing logic without hardware.

## Integration with Previous Lessons

This lesson combines:
- **Lesson 01**: Button + NeoPixel control
- **Lesson 03**: MPU9250 I2C communication
- **Lesson 05**: Testing patterns (CLI parser has tests)
- **New**: UART terminal interface

## Key Concepts Demonstrated

1. **Non-blocking I/O**: UART reads don't block the main loop
2. **Command parsing**: String splitting and argument parsing
3. **Atomic state**: LED and streaming state shared across tasks
4. **Macro usage**: `uwriteln!` macro for formatted UART output
5. **Peripheral integration**: Multiple peripherals working together

## Performance

- **CPU overhead**: <5% when idle, <10% during 100 Hz IMU streaming
- **Command latency**: <10ms response time
- **Streaming rates**: 10, 50, or 100 Hz IMU data

## References

- [ESP-HAL UART Documentation](https://docs.esp-rs.org/)
- [ESP Embedded Rust CLI Tutorial](https://blog.theembeddedrustacean.com/esp-embedded-rust-command-line-interface)
- [heapless crate](https://docs.rs/heapless/)

---

*Build interactive debugging tools for your embedded projects!* 🖥️
