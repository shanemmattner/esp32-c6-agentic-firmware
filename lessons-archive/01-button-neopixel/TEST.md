# Lesson 01 Test Specification

**Lesson:** Button + NeoPixel Control
**Hardware:** ESP32-C6-DevKitC-1 (onboard button + NeoPixel)
**Duration:** ~5 minutes

---

## Hardware Setup

### Required Equipment
- ESP32-C6-DevKitC-1 development board
- USB-C cable

### Onboard Components
- **NeoPixel (WS2812):** GPIO8
- **BOOT button:** GPIO9

No external wiring required!

---

## Test Procedure

### Test 1: Build Verification

```bash
cargo build --release
```

**Expected:** Clean build with no errors
**Pass Criteria:**
- Exit code 0
- Binary created in `target/riscv32imac-unknown-none-elf/release/main`
- Binary size ~20-30KB

---

### Test 2: Flash to Hardware

```bash
cargo run --release
```

**Expected:** Successful flash
**Pass Criteria:**
- "Flashing has completed!" message
- No errors

---

### Test 3: Button Press Detection

**Steps:**
1. Observe the onboard NeoPixel (should be off initially)
2. Press the BOOT button (GPIO9)
3. Release the button

**Expected Behavior:**
- NeoPixel cycles through colors on each button press:
  - Press 1: Red
  - Press 2: Green
  - Press 3: Blue
  - Press 4: Off
  - Press 5: Red (cycle repeats)

**Pass Criteria:**
- NeoPixel responds to button press
- Color changes are visible
- No bouncing or flickering
- Pattern repeats correctly

---

### Test 4: Debouncing Verification

**Steps:**
1. Rapidly press the button multiple times (5+ presses in 2 seconds)
2. Count the number of color changes

**Expected Behavior:**
- Each press registers as exactly one color change
- No double-triggers
- Consistent response

**Pass Criteria:**
- Clean transitions (no spurious triggers)
- One color change per press

---

## Expected Results

### Visual Indicators
- **Initial state:** NeoPixel off
- **Button press 1:** NeoPixel = Red
- **Button press 2:** NeoPixel = Green
- **Button press 3:** NeoPixel = Blue
- **Button press 4:** NeoPixel = Off

### Timing
- Button response: Immediate (< 50ms)
- Debounce delay: ~50ms (prevents double-triggers)

---

## Troubleshooting

### Issue: NeoPixel doesn't light up
**Possible Causes:**
- Board variant without onboard NeoPixel
- GPIO8 not connected to NeoPixel
- Insufficient power

**Solution:**
- Verify board model (should be ESP32-C6-DevKitC-1)
- Check board schematic for NeoPixel location
- Try different USB cable (must support data + power)

### Issue: Button presses not detected
**Possible Causes:**
- Wrong button (use BOOT button, not RESET)
- Button not connected to GPIO9

**Solution:**
- Verify using BOOT button (usually labeled "BOOT")
- Check board schematic

### Issue: Multiple color changes per press
**Possible Causes:**
- Debounce timing too short
- Noisy button signal

**Solution:**
- Check button wiring (should have pull-up)
- Increase debounce delay in code if needed

---

## Test Results Summary

| Test | Expected | Status |
|------|----------|--------|
| 1. Build | Compiles | ☐ PASS ☐ FAIL |
| 2. Flash | Success | ☐ PASS ☐ FAIL |
| 3. Button Press | Color cycles | ☐ PASS ☐ FAIL |
| 4. Debouncing | Clean triggers | ☐ PASS ☐ FAIL |

**Overall Status:** ☐ PASS ☐ FAIL

---

**Tested by:** ________________
**Date:** ________________
**Hardware:** ESP32-C6-DevKitC-1
