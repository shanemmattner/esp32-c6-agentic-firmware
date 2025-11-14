# Lesson 02 Test Specification

**Lesson:** Task Scheduler with Atomics
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
- Binary size ~20-30KB

---

### Test 2: Flash to Hardware

```bash
cargo run --release
```

**Expected:** Successful flash
**Pass Criteria:**
- "Flashing has completed!" message

---

### Test 3: Task Scheduler Operation

**Steps:**
1. Flash firmware and observe behavior
2. Watch NeoPixel and button responsiveness

**Expected Behavior:**
- Button task runs independently (checks button every cycle)
- LED task runs independently (updates NeoPixel when state changes)
- Both tasks operate without blocking each other

**Pass Criteria:**
- Button presses are detected immediately
- NeoPixel responds to button state changes
- No visible lag or freezing

---

### Test 4: Button Responsiveness

**Steps:**
1. Press the BOOT button rapidly (5 presses in 2 seconds)
2. Observe NeoPixel color changes

**Expected Behavior:**
- Each press cycles through colors: Red → Green → Blue → Off
- Task scheduler ensures responsive button handling

**Pass Criteria:**
- No missed button presses
- Color changes match button presses
- Smooth operation

---

### Test 5: Atomic State Verification

**Concept Test:**
- The button task updates `LED_STATE` atomically
- The LED task reads `LED_STATE` atomically
- No race conditions or data corruption

**Expected Behavior:**
- LED always reflects the correct state
- No glitches or incorrect colors

**Pass Criteria:**
- Consistent LED behavior
- No flickering or wrong colors

---

## Expected Results

### Visual Indicators
- Same as Lesson 01:
  - **Initial state:** NeoPixel off
  - **Button press 1:** Red
  - **Button press 2:** Green
  - **Button press 3:** Blue
  - **Button press 4:** Off

### Timing
- Button polling: Every scheduler cycle
- LED update: Every scheduler cycle (when state changes)
- Total cycle time: < 10ms

---

## Comparison to Lesson 01

| Aspect | Lesson 01 | Lesson 02 |
|--------|-----------|-----------|
| Architecture | Monolithic loop | Task scheduler |
| Button handling | Inline | Separate task |
| LED update | Inline | Separate task |
| Shared state | Direct variable | Atomic types |
| Testability | Coupled | Independent tasks |

**Key Improvement:** Tasks are now modular and can be tested independently.

---

## Troubleshooting

### Issue: Behavior identical to Lesson 01
**Expected:** Yes! The external behavior should be the same. The improvement is in code organization.

**Verification:**
- Check source code structure
- Verify `button_task()` and `led_task()` functions exist
- Confirm atomic types used for shared state

### Issue: Button presses missed
**Possible Causes:**
- Task scheduler timing issue
- Atomic ordering incorrect

**Solution:**
- Verify atomic operations use `Ordering::Relaxed` or stronger
- Check task execution frequency

### Issue: Build errors
**Possible Causes:**
- Atomic types not imported correctly

**Solution:**
```rust
use core::sync::atomic::{AtomicU8, Ordering};
```

---

## Test Results Summary

| Test | Expected | Status |
|------|----------|--------|
| 1. Build | Compiles | ☐ PASS ☐ FAIL |
| 2. Flash | Success | ☐ PASS ☐ FAIL |
| 3. Task Scheduler | Operates smoothly | ☐ PASS ☐ FAIL |
| 4. Button Response | Immediate | ☐ PASS ☐ FAIL |
| 5. Atomic State | No glitches | ☐ PASS ☐ FAIL |

**Overall Status:** ☐ PASS ☐ FAIL

---

## Learning Objectives Verified

- ☐ Understand task scheduler concept
- ☐ Use atomic types for shared state
- ☐ Implement lock-free data sharing
- ☐ Organize code into testable tasks

---

**Tested by:** ________________
**Date:** ________________
**Hardware:** ESP32-C6-DevKitC-1
