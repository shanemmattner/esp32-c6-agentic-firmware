# Lesson 04 Test Specification

**Lesson:** Static Color Navigator (State Machine)
**Hardware:** ESP32-C6 + MPU9250 + Button + NeoPixel
**Duration:** ~10 minutes

---

## Hardware Setup

### Required Equipment
- ESP32-C6 development board
- MPU9250 9-DOF IMU module
- Jumper wires
- USB-C cable

### Wiring

| Component | ESP32-C6 Pin |
|-----------|--------------|
| Button | GPIO9 (onboard) |
| NeoPixel | GPIO8 (onboard) |
| MPU9250 SDA | GPIO2 |
| MPU9250 SCL | GPIO11 |
| MPU9250 VCC | 3.3V |
| MPU9250 GND | GND |

---

## Test Procedure

### Test 1: Build Verification

```bash
cargo build --release
```

**Expected:** Clean build
**Pass Criteria:**
- Exit code 0
- Binary size ~40-50KB (larger due to state machine + HSV conversion)

---

### Test 2: Flash to Hardware

```bash
cargo run --release
```

**Expected:** Successful flash
**Pass Criteria:**
- "Flashing has completed!" message

---

### Test 3: State Machine Mode Switching

**Steps:**
1. Observe initial state (NeoPixel behavior)
2. Press button to cycle through modes
3. Tilt board and observe color changes

**Expected Behavior:**

**Mode 1: Idle (Off)**
- NeoPixel: Off
- Button press: Switch to Mode 2

**Mode 2: Tilt-Controlled Hue**
- NeoPixel: Color changes based on board tilt
- Tilt left/right: Hue changes (color wheel)
- Tilt forward/back: Different hue range
- Button press: Switch to Mode 3

**Mode 3: Brightness Control**
- NeoPixel: Fixed hue, brightness varies with tilt
- Tilt angle: Brightness 0-100%
- Button press: Back to Mode 1 (Off)

**Pass Criteria:**
- ✅ Button cycles through modes
- ✅ Modes transition correctly
- ✅ State machine operates as expected

---

### Test 4: IMU Tilt Detection

**Steps:**
1. Enter Mode 2 (press button from idle)
2. Tilt board in different directions
3. Observe NeoPixel color changes

**Expected Behavior:**
- Tilt left: Red → Yellow range
- Tilt right: Cyan → Blue range
- Tilt forward: Green range
- Tilt back: Magenta range
- Flat (horizontal): Center color

**Pass Criteria:**
- ✅ Color responds to tilt
- ✅ Smooth color transitions
- ✅ Accelerometer data correctly interpreted

---

### Test 5: HSV to RGB Conversion

**Verification:**
- Colors should span the full spectrum
- No black/white flickering (indicates conversion bug)
- Smooth transitions (no jumps)

**Pass Criteria:**
- ✅ Full color range visible
- ✅ Smooth hue transitions
- ✅ No unexpected black or white

---

### Test 6: State Machine Transitions

**Test sequence:**
1. Power on → Mode 1 (Off)
2. Button press → Mode 2 (Hue)
3. Button press → Mode 3 (Brightness)
4. Button press → Mode 1 (Off)
5. Repeat cycle 3x

**Pass Criteria:**
- ✅ Consistent state transitions
- ✅ No stuck states
- ✅ Button always responds

---

## Expected Results

### Mode Descriptions

| Mode | Name | NeoPixel Behavior | Exit Condition |
|------|------|-------------------|----------------|
| 1 | Idle | Off | Button → Mode 2 |
| 2 | Hue Control | Color changes with tilt | Button → Mode 3 |
| 3 | Brightness | Brightness varies with tilt | Button → Mode 1 |

### Timing
- Button response: Immediate (< 50ms)
- IMU read rate: ~10 Hz
- NeoPixel update: ~10 Hz

---

## Troubleshooting

### Issue: NeoPixel doesn't respond to tilt
**Possible Causes:**
- MPU9250 not initialized
- I2C wiring incorrect
- Accelerometer not reading

**Solution:**
- Check I2C wiring (SDA/SCL)
- Verify MPU9250 WHO_AM_I (Lesson 03 test)
- Check accelerometer values in debugger

### Issue: Colors are wrong
**Possible Causes:**
- HSV to RGB conversion bug
- Rotation calculation incorrect

**Solution:**
- Verify HSV conversion with known values
- Test with board flat (should be center color)
- Check for integer overflow in conversion

### Issue: State machine doesn't transition
**Possible Causes:**
- Button not detected
- State machine logic error

**Solution:**
- Verify button works (Lesson 01 test)
- Add debug output for state transitions
- Check state machine event handling

### Issue: NeoPixel flickers
**Possible Causes:**
- IMU reads too fast
- RMT timing issue
- Power supply noise

**Solution:**
- Add delay between IMU reads
- Check power supply stability
- Verify RMT configuration

---

## Test Results Summary

| Test | Expected | Status |
|------|----------|--------|
| 1. Build | Compiles | ☐ PASS ☐ FAIL |
| 2. Flash | Success | ☐ PASS ☐ FAIL |
| 3. Mode Switching | 3 modes work | ☐ PASS ☐ FAIL |
| 4. Tilt Detection | Color responds | ☐ PASS ☐ FAIL |
| 5. HSV Conversion | Full spectrum | ☐ PASS ☐ FAIL |
| 6. State Transitions | Consistent | ☐ PASS ☐ FAIL |

**Overall Status:** ☐ PASS ☐ FAIL

---

## Learning Objectives Verified

- ☐ Statig state machine library usage
- ☐ Event-driven architecture
- ☐ HSV to RGB conversion
- ☐ Multi-peripheral coordination
- ☐ Rotation angle calculation from IMU

---

**Tested by:** ________________
**Date:** ________________
**Hardware:** ESP32-C6 + MPU9250
