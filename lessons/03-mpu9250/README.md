# Lesson 03: MPU9250 IMU Sensor (SPI Communication)

Basic SPI communication test with the MPU9250 9-DOF IMU (accelerometer + gyroscope + magnetometer).

## Learning Objectives

- Initialize SPI peripheral on ESP32-C6
- Configure GPIO pins for SPI (SCLK, MOSI, MISO, CS)
- Implement basic SPI read/write protocol
- Verify sensor communication via WHO_AM_I register

## Hardware Requirements

- ESP32-C6 development board
- MPU9250 module
- USB-C cable

### Pin Configuration

```
MPU9250          ESP32-C6
─────────────────────────
VCC       →      3.3V
GND       →      GND
SCLK      →      GPIO11
SDI (MOSI)→      GPIO2
SDO (MISO)→      GPIO3
NCS (CS)  →      GPIO10
```

**Note:** The MPU9250 can also run in I2C mode. This lesson uses SPI for higher speed communication.

## What You'll Learn

This lesson demonstrates:
- SPI peripheral initialization with esp-hal 1.0.0
- Pin configuration for SPI bus
- Chip select (CS) handling
- Register-based sensor communication
- Device identification via WHO_AM_I register

## Build & Flash

```bash
cd lessons/03-dht22-sensor

# Build
cargo build --release

# Flash to ESP32-C6
cargo run --release
```

## Expected Output

```
🚀 Starting Lesson 03: MPU9250 SPI Test

✓ SPI pins configured
  SCLK: GPIO11
  SDI (MOSI): GPIO2
  SDO (MISO): GPIO3
  CS:   GPIO10

✓ SPI initialized
Attempting to read WHO_AM_I register (0x75)...
✓ Sent read request for WHO_AM_I register
✓ SPI communication successful!
  WHO_AM_I register: 0x71
  ✓ Device ID matches MPU9250 (0x71)

✅ MPU9250 detected and responding!
```

## Troubleshooting

| Issue | Possible Cause | Solution |
|-------|---|---|
| WHO_AM_I = 0x00 | No device response | Check power, GND, SCLK/MOSI/MISO/CS wiring |
| WHO_AM_I = 0xFF | SPI pin shorted | Verify no crossed wires, check board layout |
| SPI error | Initialization failed | Verify GPIO11/2/3/10 are available |
| Device ID mismatch | Wrong sensor | Verify device is MPU9250 (not MPU6050, etc) |

## Code Structure

- `src/bin/main.rs` - Main SPI test (~100 lines)
  - SPI initialization
  - WHO_AM_I register read
  - Device identification
- `Cargo.toml` - Project manifest

## MPU9250 Pinout

```
    +-------+
VCC |1      | GND
    |       |
SCL |2      | SDA (I2C mode) / SDO (SPI mode)
    |       |
    |3  250 | INT
    |2      |
FSYN|4      | NCS (SPI mode chip select)
    |       |
    |5      | SCLK (SPI mode clock)
AD0 |6      | SDI (SPI mode MOSI)
    |       |
GND |7      | GND
    |8      | AUXDA (I2C aux)
    +-------+
```

## SPI Protocol Overview

The MPU9250 uses a standard SPI interface:
- **Mode:** 0 or 3 (CPOL=0/1, CPHA=0/1)
- **Clock:** Up to 20 MHz
- **Data Format:** 8-bit bytes
- **Register Format:**
  - Read: Address byte with MSB=1, followed by data bytes
  - Write: Address byte with MSB=0, followed by data bytes

**WHO_AM_I Register (0x75):**
- Read-only
- Default value: 0x71 for MPU9250
- Used to verify device is responding

## Next Steps

- **Lesson 04:** Full MPU9250 initialization and reading accelerometer data
- **Lesson 05:** IMU data processing and orientation calculation

## References

- [MPU9250 Product Specification](https://invensense.tdk.com/wp-content/uploads/2015/02/MPU-9250-Datasheet.pdf)
- [esp-hal SPI Documentation](https://docs.rs/esp-hal/latest/esp_hal/spi/index.html)
- [ESP32-C6 Technical Reference](https://www.espressif.com/sites/default/files/documentation/esp32-c6_technical_reference_manual_en.pdf)

---

*Fast SPI communication test - foundation for sensor integration!* 🚀
