# Lesson 03: MPU9250 IMU Sensor (I2C Communication)

Basic I2C communication with the MPU9250 9-DOF IMU (accelerometer + gyroscope + magnetometer).

## Learning Objectives

- Initialize I2C peripheral on ESP32-C6
- Configure GPIO pins for I2C (SDA, SCL)
- Implement basic I2C read/write protocol
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
SDA       →      GPIO2
SCL       →      GPIO11
```

**Note:** The MPU9250 supports both I2C and SPI modes. This lesson uses I2C for simplicity (2 wires vs 4).

## What You'll Learn

This lesson demonstrates:
- I2C peripheral initialization with esp-hal 1.0.0
- Pin configuration for I2C bus
- Register-based sensor communication
- Device identification via WHO_AM_I register

## Build & Flash

```bash
cd lessons/03-mpu9250

# Build
cargo build --release

# Flash to ESP32-C6
cargo run --release
```

## Expected Output

```
Starting Lesson 03: MPU9250 Sensor

✓ I2C initialized (GPIO2=SDA, GPIO11=SCL)
✓ MPU9250 awake
✓ WHO_AM_I: 0x71
✓ Task scheduler ready

Starting sensor readings...

[Accel] X= 16052 Y= -5744 Z=  1576
[Gyro]  X=  -111 Y=   -49 Z=   -54

[Accel] X= 15992 Y= -5852 Z=  1564
[Gyro]  X=   -61 Y=  -151 Z=   -44
...
```

## Troubleshooting

| Issue | Possible Cause | Solution |
|-------|---|---|
| WHO_AM_I = 0x00 | No device response | Check power, GND, SDA/SCL wiring |
| WHO_AM_I = 0xFF | I2C pin issue | Verify SDA/SCL not swapped, check pull-ups |
| I2C error | Initialization failed | Verify GPIO2/11 are available |
| Device ID mismatch | Wrong sensor | Verify device is MPU9250 (not MPU6050, etc) |

## Code Structure

- `src/bin/main.rs` - Main I2C sensor reading (~100 lines)
  - I2C initialization
  - WHO_AM_I register read
  - Accelerometer and gyroscope data reading
- `src/mpu9250.rs` - MPU9250 driver functions
- `src/scheduler.rs` - Simple task scheduler
- `src/tasks.rs` - IMU reading task
- `Cargo.toml` - Project manifest

## I2C Protocol Overview

The MPU9250 I2C interface:
- **I2C Address:** 0x68 (or 0x69 if AD0 pin is high)
- **Clock Speed:** Standard 100kHz or Fast 400kHz
- **Data Format:** 8-bit registers
- **Register Access:** Write address byte, then read/write data

**WHO_AM_I Register (0x75):**
- Read-only
- Default value: 0x71 for MPU9250
- Used to verify device is responding

## Next Steps

- **Lesson 04:** Statig state machine with IMU-controlled color navigator
- **Future:** IMU data processing and orientation calculation

## References

- [MPU9250 Product Specification](https://invensense.tdk.com/wp-content/uploads/2015/02/MPU-9250-Datasheet.pdf)
- [esp-hal I2C Documentation](https://docs.rs/esp-hal/latest/esp_hal/i2c/index.html)
- [ESP32-C6 Technical Reference](https://www.espressif.com/sites/default/files/documentation/esp32-c6_technical_reference_manual_en.pdf)

---

I2C sensor communication - foundation for motion-based applications.
