# Lesson 06 Test Specification

**Lesson:** UART Terminal
**Hardware:** ESP32-C6 + MPU9250 + UART adapter
**Duration:** ~15 minutes

---

## Hardware Setup

### Required Equipment
- ESP32-C6 development board
- MPU9250 9-DOF IMU module
- USB-to-UART adapter (FTDI, CP2102, CH340, etc.)
- Jumper wires
- USB-C cable (for flashing)

### Wiring

| Component | ESP32-C6 Pin | Connection |
|-----------|--------------|------------|
| **UART TX** | GPIO15 | → UART adapter RX |
| **UART RX** | GPIO23 | → UART adapter TX |
| **UART GND** | GND | → UART adapter GND |
| **Button** | GPIO9 | Onboard BOOT button |
| **NeoPixel** | GPIO8 | Onboard LED |
| **MPU9250 SDA** | GPIO2 | I2C Data |
| **MPU9250 SCL** | GPIO11 | I2C Clock |
| **MPU9250 VCC** | 3.3V | Power |
| **MPU9250 GND** | GND | Ground |

**CRITICAL:** TX on ESP32 connects to RX on adapter (and vice versa)!

---

## Test Procedure

### Test 1: Wiring Verification

**Use the UART pin test script:**
```bash
../../scripts/test-uart-pins.sh 15 23 5
```

**Expected Output:**
```
=== UART Pin Test (TX=15, RX=23) ===
...
✓ UART output detected
```

**Pass Criteria:**
- ✅ Script reports success
- ✅ UART adapter detected

If this fails, check wiring before proceeding!

---

### Test 2: Build Verification

```bash
cargo build --release
```

**Expected:** Clean build
**Pass Criteria:**
- Exit code 0
- Binary size ~60-80KB (includes CLI + sensors)

---

### Test 3: Flash to Hardware

```bash
source ../../scripts/find-esp32-ports.sh
espflash flash --port $USB_CDC_PORT target/riscv32imac-unknown-none-elf/release/main
```

**Expected:** Successful flash
**Pass Criteria:**
- "Flashing has completed!" message

---

### Test 4: UART Boot Messages

**Steps:**
1. Flash firmware
2. Monitor UART output:
```bash
python3 ../../.claude/templates/read_uart.py $FTDI_PORT 5
```

**Expected Output:**
```
ESP32-C6 UART Terminal
=====================

Initializing peripherals...
✓ I2C initialized
✓ MPU9250 found (WHO_AM_I: 0x71)
✓ GPIO initialized
✓ NeoPixel initialized

Type 'help' for available commands.
>
```

**Pass Criteria:**
- ✅ Boot messages appear
- ✅ All peripherals initialize successfully
- ✅ Prompt (`>`) appears

---

### Test 5: Help Command

**Steps:**
1. Open serial terminal (minicom, screen, or PuTTY)
2. Connect to UART port at 115200 baud
3. Type: `help` and press Enter

**Expected Output:**
```
> help
Available commands:
  read <addr>      - Read I2C register (hex)
  write <addr> <val> - Write I2C register (hex)
  accel            - Read accelerometer
  gyro             - Read gyroscope
  temp             - Read temperature
  button           - Read button state
  led <r> <g> <b>  - Set NeoPixel color
  help             - Show this help
>
```

**Pass Criteria:**
- ✅ All commands listed
- ✅ Prompt returns after command

---

### Test 6: Accelerometer Command

**Steps:**
1. Type: `accel` and press Enter
2. Repeat with board in different orientations

**Expected Output:**
```
> accel
Accel: X=-0.05g, Y=0.02g, Z=1.00g
>
```

**Pass Criteria:**
- ✅ X, Y, Z values displayed
- ✅ Z ≈ 1.0g when horizontal
- ✅ Values change with board orientation

---

### Test 7: LED Control Command

**Steps:**
1. Type: `led 255 0 0` (red)
2. Type: `led 0 255 0` (green)
3. Type: `led 0 0 255` (blue)
4. Type: `led 0 0 0` (off)

**Expected Behavior:**
- NeoPixel changes color accordingly

**Pass Criteria:**
- ✅ LED responds to commands
- ✅ Colors are correct
- ✅ LED turns off with 0 0 0

---

### Test 8: Gyroscope Command

**Steps:**
1. Type: `gyro` and press Enter
2. Rotate board while running command

**Expected Output:**
```
> gyro
Gyro: X=0 deg/s, Y=0 deg/s, Z=0 deg/s
>
```

**Pass Criteria:**
- ✅ X, Y, Z values displayed
- ✅ Near zero when stationary
- ✅ Non-zero when rotating

---

### Test 9: Temperature Command

**Steps:**
1. Type: `temp` and press Enter

**Expected Output:**
```
> temp
Temperature: 25.3 °C
>
```

**Pass Criteria:**
- ✅ Temperature value reasonable (20-40°C)
- ✅ Not reading 0 or error

---

### Test 10: Button State Command

**Steps:**
1. Type: `button` and press Enter (without pressing physical button)
2. Hold physical button and type: `button` again

**Expected Output:**
```
> button
Button: Released
>
> button
Button: Pressed
>
```

**Pass Criteria:**
- ✅ Reports "Released" when not pressed
- ✅ Reports "Pressed" when held

---

### Test 11: I2C Direct Read/Write

**Steps:**
1. Type: `read 75` (WHO_AM_I register)

**Expected Output:**
```
> read 75
I2C Read [0x75]: 0x71
>
```

**Pass Criteria:**
- ✅ Reads WHO_AM_I correctly (0x71)
- ✅ No I2C timeout errors

---

### Test 12: Invalid Command Handling

**Steps:**
1. Type: `invalid_command`
2. Type: `led` (missing arguments)
3. Type: `led 300 0 0` (out of range)

**Expected Output:**
```
> invalid_command
Unknown command. Type 'help' for available commands.
>
> led
Usage: led <r> <g> <b> (0-255)
>
> led 300 0 0
Error: RGB values must be 0-255
>
```

**Pass Criteria:**
- ✅ Unknown commands show error
- ✅ Missing arguments show usage
- ✅ Invalid values show error

---

## Troubleshooting

### Issue: No UART output
**Check:**
1. UART adapter connected and powered
2. TX/RX not swapped (ESP TX → Adapter RX)
3. GND connection present
4. Baud rate 115200
5. Use `test-uart-pins.sh` to verify wiring

### Issue: Garbled output
**Check:**
1. Baud rate matches (115200)
2. No other programs reading the port
3. UART adapter drivers installed
4. Try different USB port

### Issue: Commands don't execute
**Check:**
1. Pressing Enter after typing command
2. Carriage return/line feed settings
3. Echo disabled in terminal (avoid double typing)

### Issue: MPU9250 not found
**Check:**
1. I2C wiring (SDA=GPIO2, SCL=GPIO11)
2. MPU9250 powered (3.3V)
3. Use Lesson 03 test procedure

### Issue: LED command doesn't work
**Check:**
1. Onboard NeoPixel present on board
2. GPIO8 connected to NeoPixel
3. Try `led 255 255 255` (full white) to test

---

## Test Results Summary

| Test | Expected | Status |
|------|----------|--------|
| 1. Wiring | Script succeeds | ☐ PASS ☐ FAIL |
| 2. Build | Compiles | ☐ PASS ☐ FAIL |
| 3. Flash | Success | ☐ PASS ☐ FAIL |
| 4. Boot Messages | All peripherals init | ☐ PASS ☐ FAIL |
| 5. Help | Commands listed | ☐ PASS ☐ FAIL |
| 6. Accelerometer | Reads data | ☐ PASS ☐ FAIL |
| 7. LED Control | Colors work | ☐ PASS ☐ FAIL |
| 8. Gyroscope | Reads data | ☐ PASS ☐ FAIL |
| 9. Temperature | Reasonable value | ☐ PASS ☐ FAIL |
| 10. Button State | Detects press | ☐ PASS ☐ FAIL |
| 11. I2C Direct | WHO_AM_I correct | ☐ PASS ☐ FAIL |
| 12. Error Handling | Shows errors | ☐ PASS ☐ FAIL |

**Overall Status:** ☐ PASS ☐ FAIL

---

## Learning Objectives Verified

- ☐ UART serial communication
- ☐ Command-line interface implementation
- ☐ Multi-peripheral integration
- ☐ Non-blocking command parsing
- ☐ Sensor data streaming over serial

---

## Terminal Settings

**Recommended terminal emulator settings:**
- Baud rate: 115200
- Data bits: 8
- Stop bits: 1
- Parity: None
- Flow control: None
- Echo: Off (local echo disabled)
- Line ending: CR+LF or LF

---

**Tested by:** ________________
**Date:** ________________
**Hardware:** ESP32-C6 + MPU9250 + UART adapter
