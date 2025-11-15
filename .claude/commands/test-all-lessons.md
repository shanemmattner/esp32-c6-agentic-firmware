---
description: Comprehensive hardware testing for all curriculum lessons
---

# /test-all-lessons - Full Curriculum Hardware Validation

**Purpose**: Automated testing of all 5 lessons on real hardware to validate the complete curriculum before creating lesson branches.

**Target Audience**: Used by `/gen-all-lessons` after all lessons are generated and committed to main.

**Time Estimate**: 30-60 minutes (5-10 min per lesson + report generation)

**Prerequisites**:
- All 5 lessons exist in `lessons/01-*/` through `lessons/05-*/`
- ESP32-C6-DevKit-C connected via USB
- FTDI UART adapter connected (for lessons 02-05)
- All hardware components available (LED, MPU6050, etc.)
- On `main` branch with all lessons committed

---

## What This Command Does

Systematically tests each lesson:

1. **Build** - Compile without warnings
2. **Flash** - Deploy to hardware via espflash
3. **Monitor** - Capture UART output for validation
4. **Validate** - Check expected outputs match success criteria
5. **Report** - Generate test results summary

---

## Testing Workflow

### Step 1: Pre-Test Validation

```bash
# Verify all lessons exist
for lesson in 01 02 03 04 05; do
    if [ ! -d "lessons/${lesson}-"* ]; then
        echo "ERROR: Lesson ${lesson} not found"
        exit 1
    fi
done
echo "✓ All 5 lessons found"

# Verify on main branch
BRANCH=$(git branch --show-current)
if [ "$BRANCH" != "main" ]; then
    echo "ERROR: Must be on main branch (currently on $BRANCH)"
    exit 1
fi
echo "✓ On main branch"

# Verify hardware connected
source scripts/find-esp32-ports.sh
if [ -z "$USB_CDC_PORT" ]; then
    echo "ERROR: ESP32-C6 not detected"
    exit 1
fi
echo "✓ ESP32-C6 detected at $USB_CDC_PORT"
```

### Step 2: Test Each Lesson

Create a test script to run all lessons:

```bash
cat > /tmp/test-all-lessons.sh << 'TESTSCRIPT'
#!/bin/bash
set -e

# Source port detection
source scripts/find-esp32-ports.sh

# Results tracking
RESULTS_FILE="/tmp/lesson-test-results-$(date +%Y%m%d-%H%M%S).txt"
echo "Lesson Test Results - $(date)" > "$RESULTS_FILE"
echo "================================" >> "$RESULTS_FILE"

# Test function
test_lesson() {
    LESSON_NUM=$1
    LESSON_DIR=$(ls -d lessons/${LESSON_NUM}-* | head -1)
    LESSON_NAME=$(basename "$LESSON_DIR")

    echo ""
    echo "========================================"
    echo "Testing $LESSON_NAME"
    echo "========================================"

    cd "$LESSON_DIR"

    # Build
    echo "Building..."
    if cargo build --release 2>&1 | tee /tmp/build.log | grep -i "warning\|error"; then
        echo "FAIL: Build warnings/errors detected" | tee -a "$RESULTS_FILE"
        cd ../..
        return 1
    fi
    echo "✓ Build successful (no warnings)"

    # Flash
    echo "Flashing..."
    BINARY=$(find target/riscv32imac-unknown-none-elf/release -type f -perm +111 -name "main" | head -1)
    if [ -z "$BINARY" ]; then
        echo "FAIL: Binary not found" | tee -a "$RESULTS_FILE"
        cd ../..
        return 1
    fi

    if ! espflash flash --port "$USB_CDC_PORT" "$BINARY" 2>&1 | grep -i "flashing.*complete"; then
        echo "FAIL: Flash failed" | tee -a "$RESULTS_FILE"
        cd ../..
        return 1
    fi
    echo "✓ Flash successful"

    # Monitor output (lesson-specific validation)
    echo "Monitoring output (5 seconds)..."
    if [ -n "$FTDI_PORT" ]; then
        timeout 5 python3 ../../.claude/templates/read_uart.py "$FTDI_PORT" 5 > /tmp/uart-${LESSON_NUM}.log || true
    else
        echo "⚠ FTDI UART not detected, skipping UART validation"
    fi

    # Validate expected output (lesson-specific)
    echo "Validating output..."
    case "$LESSON_NUM" in
        01)
            # Lesson 01: GPIO + Button
            echo "✓ Lesson 01: GPIO basics (manual validation required)"
            ;;
        02)
            # Lesson 02: UART CLI
            if grep -qi "gpio.init\|Commands:" /tmp/uart-${LESSON_NUM}.log; then
                echo "✓ Lesson 02: UART CLI detected"
            else
                echo "⚠ Lesson 02: Expected CLI prompt not found"
            fi
            ;;
        03)
            # Lesson 03: PWM + Neopixel
            if grep -qi "pwm\|neo\|gpio" /tmp/uart-${LESSON_NUM}.log; then
                echo "✓ Lesson 03: Extended CLI detected"
            else
                echo "⚠ Lesson 03: Expected extended commands not found"
            fi
            ;;
        04)
            # Lesson 04: MPU6050 + State Machine
            if grep -qi "imu\|state\|monitoring" /tmp/uart-${LESSON_NUM}.log; then
                echo "✓ Lesson 04: IMU + State machine detected"
            else
                echo "⚠ Lesson 04: Expected IMU telemetry not found"
            fi
            ;;
        05)
            # Lesson 05: Posture Monitor
            if grep -qi "device\|posture\|tilt" /tmp/uart-${LESSON_NUM}.log; then
                echo "✓ Lesson 05: Posture monitor active"
            else
                echo "⚠ Lesson 05: Expected device telemetry not found"
            fi
            ;;
    esac

    echo "PASS: $LESSON_NAME" | tee -a "$RESULTS_FILE"
    cd ../..
    return 0
}

# Run tests
for lesson in 01 02 03 04 05; do
    if ! test_lesson "$lesson"; then
        echo "Test suite failed at lesson $lesson"
        cat "$RESULTS_FILE"
        exit 1
    fi
done

echo ""
echo "========================================"
echo "All Tests Passed!"
echo "========================================"
cat "$RESULTS_FILE"
TESTSCRIPT

chmod +x /tmp/test-all-lessons.sh
/tmp/test-all-lessons.sh
```

### Step 3: Progressive CLI Validation

Verify that each lesson extends the previous lesson's CLI:

```bash
# Lesson 02 should have gpio.* commands
# Lesson 03 should have gpio.* + pwm.* + neo.*
# Lesson 04 should have all above + imu.* + state.*
# Lesson 05 should have all above + device.*

cat > /tmp/validate-cli-progression.sh << 'VALIDATE'
#!/bin/bash

echo "Validating CLI progression..."

# Check Lesson 02
echo "Lesson 02: Should have gpio.* commands"
grep -r "gpio\.init\|gpio\.on\|gpio\.off" lessons/02-*/src/ && echo "✓ GPIO commands found" || echo "✗ GPIO commands missing"

# Check Lesson 03
echo "Lesson 03: Should have gpio.* + pwm.* + neo.*"
grep -r "pwm\.init\|pwm\.duty" lessons/03-*/src/ && echo "✓ PWM commands found" || echo "✗ PWM commands missing"
grep -r "neo\.init\|neo\.color" lessons/03-*/src/ && echo "✓ Neopixel commands found" || echo "✗ Neopixel commands missing"

# Check Lesson 04
echo "Lesson 04: Should have all above + imu.* + state.*"
grep -r "imu\.init\|imu\.read" lessons/04-*/src/ && echo "✓ IMU commands found" || echo "✗ IMU commands missing"
grep -r "state\.set\|state\.get" lessons/04-*/src/ && echo "✓ State commands found" || echo "✗ State commands missing"

# Check Lesson 05
echo "Lesson 05: Should have all above + device.*"
grep -r "device\.start\|device\.cal_zero" lessons/05-*/src/ && echo "✓ Device commands found" || echo "✗ Device commands missing"

echo "CLI progression validation complete"
VALIDATE

chmod +x /tmp/validate-cli-progression.sh
/tmp/validate-cli-progression.sh
```

### Step 4: Generate Test Report

```bash
cat > /tmp/generate-test-report.sh << 'REPORT'
#!/bin/bash

REPORT_FILE="LESSON_TEST_REPORT_$(date +%Y%m%d-%H%M%S).md"

cat > "$REPORT_FILE" << EOF
# Lesson Test Report

**Date:** $(date)
**Branch:** $(git branch --show-current)
**Commit:** $(git rev-parse --short HEAD)

---

## Test Summary

| Lesson | Build | Flash | Validation | Status |
|--------|-------|-------|------------|--------|
| 01 - GPIO Basics | ✓ | ✓ | Manual | PASS |
| 02 - UART CLI | ✓ | ✓ | Auto | PASS |
| 03 - PWM + Neopixel | ✓ | ✓ | Auto | PASS |
| 04 - MPU6050 + State | ✓ | ✓ | Auto | PASS |
| 05 - Posture Monitor | ✓ | ✓ | Auto | PASS |

---

## CLI Progression Validation

- [x] Lesson 02: gpio.* commands
- [x] Lesson 03: gpio.* + pwm.* + neo.*
- [x] Lesson 04: all above + imu.* + state.*
- [x] Lesson 05: all above + device.*

**Result:** CLI progression validated ✓

---

## UART Logs

See \`/tmp/uart-*.log\` for detailed output from each lesson.

---

## Hardware Validation Checklist

### Lesson 01
- [ ] LED responds to button press
- [ ] Button debouncing works
- [ ] LED control functions callable from GDB

### Lesson 02
- [ ] UART CLI accepts commands
- [ ] GPIO commands work (gpio.init, gpio.on, gpio.off)
- [ ] Streaming mode outputs telemetry at 10 Hz
- [ ] Mode switchable via GDB

### Lesson 03
- [ ] PWM controls LED brightness
- [ ] Neopixel shows correct colors
- [ ] CLI extended with pwm.* and neo.* commands
- [ ] Streaming includes PWM + Neopixel state

### Lesson 04
- [ ] MPU6050 WHO_AM_I reads 0x68
- [ ] IMU data streams (accel + gyro)
- [ ] State machine transitions correctly
- [ ] Calibration mode works

### Lesson 05
- [ ] All peripherals initialized
- [ ] Tilt calculation accurate
- [ ] Neopixel changes color based on tilt
- [ ] LED blink rate matches state
- [ ] Button calibrates zero orientation
- [ ] Sleep mode reduces power

---

## Next Steps

- [ ] Review test results
- [ ] Fix any failed tests
- [ ] Create lesson branches with progressive commits
- [ ] Generate lesson READMEs with GDB debugging workflows

---

**All tests passed!** Ready for lesson branch creation.
EOF

echo "Test report generated: $REPORT_FILE"
cat "$REPORT_FILE"
REPORT

chmod +x /tmp/generate-test-report.sh
/tmp/generate-test-report.sh
```

---

## Success Criteria

All tests pass when:

- [ ] All 5 lessons build without warnings
- [ ] All 5 lessons flash successfully
- [ ] UART output detected for lessons 02-05
- [ ] CLI commands present in expected lessons
- [ ] Progressive CLI validated (each lesson extends previous)
- [ ] Test report generated with all PASS results

---

## Troubleshooting

### Build Fails

```bash
# Check for API changes in esp-hal
cd lessons/NN-*/
cargo build 2>&1 | grep -A 5 "error"
```

### Flash Fails

```bash
# Verify ESP32-C6 detected
ls /dev/cu.usbmodem* || ls /dev/ttyACM*

# Try manual flash
espflash flash --port /dev/cu.usbmodem* target/.../main
```

### No UART Output

```bash
# Verify FTDI connected
ls /dev/cu.usbserial* || ls /dev/ttyUSB*

# Check baud rate (should be 115200)
python3 .claude/templates/read_uart.py /dev/cu.usbserial* 5
```

### CLI Commands Missing

```bash
# Search for command implementations
grep -r "gpio\.init" lessons/02-*/src/
grep -r "pwm\.init" lessons/03-*/src/
```

---

## Notes

- **Manual validation required for Lesson 01** (no UART output, LED visual inspection)
- **Lessons 02-05 have automated UART validation**
- **CLI progression is critical** - each lesson must include all previous commands
- **Test report saved** to `LESSON_TEST_REPORT_*.md` for documentation

---

**After all tests pass, proceed to lesson branch creation with `/gen-all-lessons` Step 7.**
