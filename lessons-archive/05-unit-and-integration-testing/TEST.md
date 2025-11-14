# Lesson 05 Test Specification

**Lesson:** Unit and Integration Testing
**Hardware:** ESP32-C6 (for integration tests only)
**Duration:** ~5 minutes

---

## Overview

This lesson focuses on **testing methodology** rather than hardware functionality. Tests are divided into:
1. **Unit tests** - Run on host machine (fast, no hardware)
2. **Integration tests** - Run on device (slow, requires hardware)

---

## Test Procedure

### Test 1: Host-Based Unit Tests

**Purpose:** Verify pure functions work correctly on host

**Steps:**
```bash
cd lessons/05-unit-and-integration-testing

# Temporarily disable embedded toolchain
mv .cargo .cargo.device
mv rust-toolchain.toml rust-toolchain.toml.device

# Run host tests
cargo test --lib

# Restore embedded toolchain
mv .cargo.device .cargo
mv rust-toolchain.toml.device rust-toolchain.toml
```

**Expected Output:**
```
running 28 tests
test color::tests::test_hsv_to_rgb_red ... ok
test color::tests::test_hsv_to_rgb_green ... ok
test color::tests::test_hsv_to_rgb_blue ... ok
test rotation::tests::test_atan2_approximation ... ok
test state_machine::tests::test_idle_to_hue ... ok
...

test result: ok. 28 passed; 0 failed; 0 ignored
```

**Pass Criteria:**
- ✅ All 28 tests pass
- ✅ No test failures
- ✅ Tests run in < 5 seconds

---

### Test 2: Build Integration Test Binary

**Purpose:** Verify device integration test compiles

```bash
cargo build --release --bin integration_test
```

**Expected:** Clean build
**Pass Criteria:**
- Exit code 0
- Binary created

---

### Test 3: Run Integration Test on Device (Optional)

**Purpose:** Verify I2C communication on real hardware

**Hardware Required:**
- ESP32-C6
- MPU9250 connected (GPIO2=SDA, GPIO11=SCL)

**Steps:**
```bash
cargo run --release --bin integration_test
```

**Expected Output (via serial):**
```
=== Integration Test: I2C ===
Initializing I2C...
Reading MPU9250 WHO_AM_I...
✓ WHO_AM_I: 0x71 (Expected: 0x71)
Reading accelerometer...
✓ Accelerometer: X=0, Y=0, Z=16384
Test PASSED
```

**Pass Criteria:**
- ✅ WHO_AM_I reads correctly (0x71 or 0x73)
- ✅ Accelerometer data is non-zero
- ✅ Test reports PASSED

---

## Unit Test Coverage

### Module: `color.rs` (HSV → RGB Conversion)

**Tests:**
- `test_hsv_to_rgb_red` - H=0° → Red
- `test_hsv_to_rgb_green` - H=120° → Green
- `test_hsv_to_rgb_blue` - H=240° → Blue
- `test_hsv_to_rgb_yellow` - H=60° → Yellow
- `test_hsv_to_rgb_cyan` - H=180° → Cyan
- `test_hsv_to_rgb_magenta` - H=300° → Magenta
- `test_hsv_saturation_zero` - S=0 → White
- `test_hsv_value_zero` - V=0 → Black

**Purpose:** Verify integer-based HSV conversion is correct

---

### Module: `rotation.rs` (IMU Calculations)

**Tests:**
- `test_atan2_approximation` - Verify angle calculation
- `test_rotation_flat` - Board horizontal → 0°
- `test_rotation_tilted` - Board tilted → angle changes
- `test_rotation_magnitude` - Calculate vector length

**Purpose:** Verify rotation math is correct (no floating point)

---

### Module: `state_machine.rs` (State Logic)

**Tests:**
- `test_idle_to_hue` - Button press transitions states
- `test_hue_to_brightness` - Mode cycling
- `test_brightness_to_idle` - Return to idle
- `test_button_debounce` - No double-triggers
- `test_event_handling` - Events processed correctly

**Purpose:** Verify state machine logic without hardware

---

## Comparison: Unit vs Integration Tests

| Aspect | Unit Tests | Integration Tests |
|--------|------------|-------------------|
| **Speed** | Fast (< 5s) | Slow (~1 min) |
| **Hardware** | None | Required |
| **Coverage** | Pure functions | Hardware I/O |
| **Debugging** | Easy | Hard |
| **CI/CD** | Yes | No (needs device) |

**Key Insight:** Write unit tests for algorithms, integration tests for hardware.

---

## Troubleshooting

### Issue: Host tests fail with "no_std" errors
**Symptoms:** `std::vec::Vec` not found, etc.

**Cause:** Need to enable `std` for host tests

**Solution:** Verify `lib.rs` has:
```rust
#![cfg_attr(not(test), no_std)]
```

### Issue: Integration test can't communicate with MPU9250
**Symptoms:** WHO_AM_I reads 0xFF or times out

**Solution:**
- Check I2C wiring
- Verify MPU9250 powered
- Use Lesson 03 test procedure

### Issue: Cargo test runs device toolchain
**Symptoms:** Tests fail to link or use wrong target

**Solution:**
- Temporarily move `.cargo/` and `rust-toolchain.toml`
- Run tests
- Restore files

---

## Test Results Summary

| Test | Expected | Status |
|------|----------|--------|
| 1. Host Unit Tests | 28 pass | ☐ PASS ☐ FAIL |
| 2. Integration Build | Compiles | ☐ PASS ☐ FAIL |
| 3. Device Integration | I2C works | ☐ PASS ☐ FAIL ☐ SKIP |

**Overall Status:** ☐ PASS ☐ FAIL

**Note:** Test 3 is optional if hardware not available

---

## Learning Objectives Verified

- ☐ Understand unit vs integration tests
- ☐ Write host-based unit tests
- ☐ Create device integration tests
- ☐ Use `#[cfg_attr(not(test), no_std)]` pattern
- ☐ Separate testable logic from hardware

---

**Tested by:** ________________
**Date:** ________________
**Hardware:** Host machine + ESP32-C6 (optional)
