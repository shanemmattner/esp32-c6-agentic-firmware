# Lesson 03 Test Specification

**Lesson:** MPU9250 IMU Sensor (I2C Communication)
**Hardware:** ESP32-C6 + MPU9250 module
**Duration:** ~10 minutes

---

## Hardware Setup

### Required Equipment
- ESP32-C6 development board
- MPU9250 9-DOF IMU module
- 4x jumper wires
- USB-C cable

### Wiring

| MPU9250 | ESP32-C6 |
|---------|----------|
| VCC | 3.3V |
| GND | GND |
| SDA | GPIO2 |
| SCL | GPIO11 |

**Important:** Use 3.3V, NOT 5V! MPU9250 is 3.3V logic.

---

## Test Procedure

### Test 1: Wiring Verification

**Before building, verify connections:**
- ☐ MPU9250 VCC → ESP32 3.3V
- ☐ MPU9250 GND → ESP32 GND
- ☐ MPU9250 SDA → GPIO2
- ☐ MPU9250 SCL → GPIO11
- ☐ All connections secure

---

### Test 2: Build Verification

```bash
cargo build --release
```

**Expected:** Clean build
**Pass Criteria:**
- Exit code 0
- Binary size ~25-35KB

---

### Test 3: Flash to Hardware

```bash
cargo run --release
```

**Expected:** Successful flash
**Pass Criteria:**
- "Flashing has completed!" message

---

### Test 4: WHO_AM_I Register Read

**Steps:**
1. Flash firmware
2. Monitor serial output (if enabled) or use debugger

**Expected Behavior:**
- I2C communication successful
- WHO_AM_I register reads as `0x71` (MPU9250) or `0x73` (MPU9255)

**Pass Criteria:**
- ✅ I2C initialization succeeds
- ✅ WHO_AM_I value is correct
- ✅ No I2C timeout errors

**Troubleshooting:**
If WHO_AM_I fails:
- Check wiring (especially SDA/SCL not swapped)
- Verify 3.3V power supply
- Try different I2C address (0x68 or 0x69)

---

### Test 5: Accelerometer Read

**Expected Behavior:**
- Accelerometer X, Y, Z values can be read
- When board is flat on table: Z ≈ 1.0g, X ≈ 0g, Y ≈ 0g
- When tilted, values change accordingly

**Pass Criteria:**
- ✅ Accelerometer data updates
- ✅ Z-axis shows ~1g when horizontal
- ✅ Values respond to board movement

---

### Test 6: Gyroscope Read

**Expected Behavior:**
- Gyroscope X, Y, Z values can be read
- When stationary: All values ≈ 0 deg/s
- When rotating, values change

**Pass Criteria:**
- ✅ Gyroscope data updates
- ✅ Values near zero when stationary
- ✅ Values respond to board rotation

---

### Test 7: Temperature Sensor Read

**Expected Behavior:**
- Temperature sensor returns reasonable value
- Typical range: 20-40°C (room temperature)

**Pass Criteria:**
- ✅ Temperature value is reasonable
- ✅ Not reading 0 or invalid values

---

## Expected I2C Operation

### I2C Transaction Flow
1. **START condition**
2. **Device address** (0x68 or 0x69)
3. **Register address** (e.g., 0x75 for WHO_AM_I)
4. **READ/WRITE data**
5. **STOP condition**

### Timing
- I2C clock frequency: 100-400 kHz
- WHO_AM_I read: < 1ms
- Full sensor read: < 5ms

---

## Troubleshooting

### Issue: I2C communication fails
**Symptoms:** Timeout errors, no response from sensor

**Check:**
1. **Wiring:**
   - SDA and SCL not swapped
   - Connections are secure
   - Using GPIO2 (SDA) and GPIO11 (SCL)

2. **Power:**
   - MPU9250 receiving 3.3V (NOT 5V!)
   - GND connection solid

3. **I2C Address:**
   - MPU9250 default: 0x68
   - If AD0 pin is HIGH: 0x69
   - Check code matches hardware

### Issue: WHO_AM_I returns wrong value
**Symptoms:** Value is not 0x71 or 0x73

**Possible Causes:**
- Reading wrong register
- Wrong I2C address
- Fake/counterfeit MPU9250 chip

**Solution:**
- Verify register address 0x75
- Try both addresses (0x68 and 0x69)
- Check chip markings

### Issue: Accelerometer reads zeros
**Symptoms:** All axes read 0

**Possible Causes:**
- Sensor not initialized
- Power-on reset needed
- Wrong register addresses

**Solution:**
- Add 100ms delay after power-on
- Send power management reset
- Verify register map

### Issue: Gyroscope values drift
**Symptoms:** Non-zero values when stationary

**Expected:** Some drift is normal! Gyroscopes have bias.

**Solution:**
- Implement calibration routine (zero offset)
- Average multiple readings
- Use sensor fusion (Kalman filter) for accuracy

---

## Test Results Summary

| Test | Expected | Status |
|------|----------|--------|
| 1. Wiring | Connections secure | ☐ PASS ☐ FAIL |
| 2. Build | Compiles | ☐ PASS ☐ FAIL |
| 3. Flash | Success | ☐ PASS ☐ FAIL |
| 4. WHO_AM_I | 0x71 or 0x73 | ☐ PASS ☐ FAIL |
| 5. Accelerometer | Reads data | ☐ PASS ☐ FAIL |
| 6. Gyroscope | Reads data | ☐ PASS ☐ FAIL |
| 7. Temperature | Reasonable value | ☐ PASS ☐ FAIL |

**Overall Status:** ☐ PASS ☐ FAIL

---

## Learning Objectives Verified

- ☐ I2C peripheral initialization
- ☐ I2C read/write protocol
- ☐ Sensor WHO_AM_I verification
- ☐ Accelerometer data acquisition
- ☐ Gyroscope data acquisition

---

## Notes

- The MPU9250 also has a magnetometer (AK8963), but it requires separate initialization on a different I2C bus
- For production code, add proper error handling and sensor calibration
- Consider adding FIFO buffer reading for higher sample rates

---

**Tested by:** ________________
**Date:** ________________
**Hardware:** ESP32-C6 + MPU9250
