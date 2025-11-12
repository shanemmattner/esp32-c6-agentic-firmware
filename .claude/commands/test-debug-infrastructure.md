---
description: Test debug infrastructure for ESP32-C6 lessons (build, flash, debug symbols, RTT logging)
argument-hint: [lesson: 07|08] (default: 07)
---

# Test Debug Infrastructure for ESP32-C6 Lessons

You are testing the debugging infrastructure for ESP32-C6 lessons to ensure they are properly configured for debugging.

**Supported Lessons:**
- **Lesson 07:** GDB debugging with probe-rs (register inspection, breakpoints, call stacks)
- **Lesson 08:** defmt RTT logging with probe-rs (real-time logging over debug probe)

## Lesson Selection

{{argument}} <!-- "07" or "08" - defaults to "07" if not specified -->

The argument specifies which lesson to test. Each lesson has different debugging capabilities:

- **07 (GDB debugging):** Tests infrastructure for interactive debugging (breakpoints, register inspection, call stacks)
- **08 (RTT logging):** Tests infrastructure for defmt RTT real-time logging over the debug probe

## ⚠️ IMPORTANT: Interactive Test Limitation

**probe-rs interactive mode (Tests 2-7) CANNOT be automated** in non-interactive bash contexts.

The Bash tool executes commands without a TTY, which causes `probe-rs attach` to block waiting for stdin. No piping mechanism (heredocs, echo pipes, or expect-style scripts) works reliably in this environment.

**What This Command Does:**
1. ✓ Automated: Build and flash firmware
2. ✓ Automated: Infrastructure tests (Tests 8-11) - verify debug symbols, code structure, configs
3. ⚠️ Manual: Interactive probe-rs tests (Tests 2-7) - documented in report for manual execution

**Expected Outcome:** You'll receive a test report with infrastructure validation results and manual test instructions for interactive debugging verification.

---

## Your Task

Execute a systematic test of debugging capabilities on real ESP32-C6 hardware using **probe-rs**.

### Prerequisites Check

Before starting tests, verify:

1. **Hardware connected:**
   - ESP32-C6 board connected via USB
   - JTAG debugger connected (TMS→GPIO4, TDI→GPIO5, TDO→GPIO6, TCK→GPIO7, GND)
   - MPU9250 IMU connected (SDA→GPIO2, SCL→GPIO11)
   - Button on GPIO9
   - NeoPixel on GPIO8
   - UART adapter on GPIO15 (TX) and GPIO23 (RX)

2. **Software ready:**
   - `probe-rs` installed (check with `which probe-rs`)
   - `espflash` installed (for flashing firmware)
   - Lesson 07 firmware built (`cargo build --release`)
   - Python 3 available for verifying firmware boot messages

### Best Practices: Shell Execution Strategy

**Default Approach: Use Temp Scripts**

For ANY test with more than 2 commands, use temp scripts to avoid shell parsing issues:

```bash
cat > /tmp/test_name.sh << 'SCRIPT'
#!/bin/bash
set -e  # Exit on error

source /tmp/probe_env.sh  # Load environment variables

echo "=== Test N: Description ==="

# Your test logic here
command1
command2

echo "✓ Test N PASSED"
SCRIPT

chmod +x /tmp/test_name.sh
/tmp/test_name.sh
```

**Why?** The Bash tool executes commands in an eval context that can have issues with complex syntax. Temp scripts avoid parse errors and provide clearer structure.

**Critical Flashing Strategy:**

ALWAYS use this workflow:
1. **Flash:** `espflash flash --port $USB_CDC_PORT target/.../main`
2. **Debug:** `probe-rs attach --chip esp32c6 $PROBE_ARG target/.../main`

**NEVER use `probe-rs run`** - it creates an exclusive lock that blocks subsequent attach commands.

### Test Structure

Your tests should be **deterministic and reproducible**. Follow this structure:

```
1. Automated Setup & Build
   ├─ Environment detection (Step 0)
   ├─ Cleanup (Step 1)
   ├─ Build firmware (Step 2)
   └─ Flash firmware (Step 3)

2. Automated Infrastructure Tests ✓ (ALWAYS run these)
   ├─ Test 8: Debug symbols present in binary
   ├─ Test 9: Source code structure verified
   ├─ Test 10: .gdbinit configuration exists (for future GDB use)
   ├─ Test 11: Python helper scripts syntax valid
   └─ Test 12: Cargo.toml debug configuration verified

3. Manual Interactive Tests (document in report for user)
   ├─ Test 2: probe-rs can attach to running firmware
   ├─ Test 3: Peripheral register inspection (I2C, GPIO)
   ├─ Test 4: Memory inspection (stack, heap, buffers)
   ├─ Test 5: Breakpoint at main()
   ├─ Test 6: Function breakpoint (handle_command)
   └─ Test 7: Call stack analysis (backtrace)

Note: Tests 2-7 require interactive terminal with TTY. These cannot be automated
in non-interactive bash contexts. The report will provide exact commands for manual testing.
```

### Execution Guidelines

All tests use **probe-rs** as the debugger. Follow these steps in order:

#### Step 0: Determine Lesson Directory

First, determine which lesson to test based on the argument:

```bash
# Parse argument (defaults to "07" if not specified)
LESSON_ARG="{{argument}}"
LESSON_ARG="${LESSON_ARG:-07}"  # Default to 07 if empty

# Map to lesson directory
case "$LESSON_ARG" in
    "07"|"7")
        LESSON_DIR="07-gdb-debugging"
        LESSON_NAME="GDB Debugging (Lesson 07)"
        ;;
    "08"|"8")
        LESSON_DIR="08-defmt-rtt-logging"
        LESSON_NAME="defmt RTT Logging (Lesson 08)"
        ;;
    *)
        echo "✗ ERROR: Invalid lesson argument: $LESSON_ARG"
        echo "  Valid options: 07, 08"
        exit 1
        ;;
esac

echo "=== Testing: $LESSON_NAME ==="
echo "Lesson directory: lessons/$LESSON_DIR"
echo ""
```

#### Step 1: Environment Detection and Persistent Configuration

**CRITICAL:** Variables do NOT persist across separate bash calls in Claude Code. We solve this by writing environment variables to a temp file that can be sourced in each test.

```bash
# Use the LESSON_DIR from Step 0
cd /Users/shanemattner/Desktop/esp32-c6-agentic-firmware/lessons/$LESSON_DIR

cat > /tmp/probe_env.sh << ENV_SCRIPT
#!/bin/bash
# Auto-generated environment configuration
# Created: \$(date)

# Lesson directory from Step 0
export LESSON_DIR="$LESSON_DIR"

echo "=== Step 1: Environment Setup ==="

# 1. Detect USB CDC port (required for flashing)
export USB_CDC_PORT=$(ls /dev/cu.usbmodem* 2>/dev/null | head -1)
if [ -z "$USB_CDC_PORT" ]; then
    echo "✗ ERROR: USB CDC port not found"
    echo "  Expected: /dev/cu.usbmodem* (macOS) or /dev/ttyACM* (Linux)"
    echo "  Action: Verify ESP32-C6 USB connection"
    exit 1
fi
echo "✓ USB CDC: $USB_CDC_PORT"

# 2. Detect ESP JTAG probe (VID:PID:Serial format for --probe flag)
export ESP_PROBE=$(probe-rs list 2>&1 | grep -i "esp.*jtag" | grep -oE '[0-9a-fA-F]{4}:[0-9a-fA-F]{4}(:[0-9A-F:]+)?' | head -1)
if [ -n "$ESP_PROBE" ]; then
    export PROBE_ARG="--probe $ESP_PROBE"
    echo "✓ ESP Probe: $ESP_PROBE"
else
    export PROBE_ARG=""
    echo "⚠ ESP Probe: Auto-detection failed - will try without --probe flag"
fi

# 3. Detect UART port (optional - only needed for interactive Test 6)
export UART_PORT=$(ls /dev/cu.usbserial* 2>/dev/null | head -1)
if [ -n "$UART_PORT" ]; then
    echo "✓ UART: $UART_PORT"
else
    echo "⚠ UART: not detected (optional)"
fi

# 4. Define timeout command wrapper
if command -v gtimeout > /dev/null 2>&1; then
    export TIMEOUT_CMD="gtimeout"
    echo "✓ Timeout: gtimeout (Homebrew)"
elif command -v timeout > /dev/null 2>&1; then
    export TIMEOUT_CMD="timeout"
    echo "✓ Timeout: timeout (GNU)"
else
    export TIMEOUT_CMD=""
    echo "⚠ Timeout: not available (will use background jobs with manual kill)"
fi

echo ""
echo "=== Environment Ready ==="
echo "USB CDC: $USB_CDC_PORT"
echo "ESP Probe: ${ESP_PROBE:-auto-detect}"
echo "UART: ${UART_PORT:-not configured}"
echo ""
ENV_SCRIPT

chmod +x /tmp/probe_env.sh
/tmp/probe_env.sh
```

**How to use in subsequent tests:**

Every test that needs hardware access should start with:
```bash
source /tmp/probe_env.sh  # Load detected hardware configuration

# Now use the variables:
espflash flash --port "$USB_CDC_PORT" target/.../main
probe-rs attach --chip esp32c6 $PROBE_ARG target/.../main
```

**Note:** The env file is recreated on each test run, so port changes are automatically detected


#### Step 2: Cleanup and Verify Tools

```bash
# Clean up any existing debug sessions (do this at START, not END)
# Reason: Previous test runs may have crashed, leaving orphaned processes
# Better to clean up proactively than to fail with "exclusive access" errors
echo "=== Step 1: Cleanup ==="
pkill -f "probe-rs" || true
pkill -f "openocd" || true
sleep 1

# Verify cleanup succeeded
REMAINING=$(ps aux | grep -E "(probe-rs|openocd)" | grep -v grep | wc -l)
if [ "$REMAINING" -gt 0 ]; then
    echo "⚠ Warning: $REMAINING debug processes still running - may cause 'exclusive access' errors"
    ps aux | grep -E "(probe-rs|openocd)" | grep -v grep
else
    echo "✓ Cleanup successful - no orphaned debug processes"
fi

# Verify probe-rs is available
if ! which probe-rs > /dev/null 2>&1; then
    echo "✗ ERROR: probe-rs not found - cannot run tests"
    exit 1
fi
echo "✓ probe-rs found: $(which probe-rs)"

# Show detected probes
echo ""
echo "Detecting probes..."
probe-rs list
```

#### Step 3: Build firmware with debug symbols

```bash
source /tmp/probe_env.sh  # Get LESSON_DIR

cd /Users/shanemattner/Desktop/esp32-c6-agentic-firmware/lessons/$LESSON_DIR

echo "=== Step 3: Build Firmware ==="
cargo build --release  # Has debug=2 in Cargo.toml for debug symbols
echo "✓ Build complete"
```

#### Step 4: Flash firmware using espflash

**IMPORTANT STRATEGY:** Use `espflash` for flashing, then `probe-rs attach` for debugging.

**Why not `probe-rs run`?**
- `probe-rs run` flashes AND attaches in one command
- This creates an exclusive lock that blocks subsequent `probe-rs attach` commands
- Separate flashing (espflash) and debugging (probe-rs attach) avoids this issue

```bash
source /tmp/probe_env.sh  # Load USB_CDC_PORT, LESSON_DIR and other variables

cd /Users/shanemattner/Desktop/esp32-c6-agentic-firmware/lessons/$LESSON_DIR

echo "=== Step 4: Flash Firmware ==="
echo "Using USB CDC port: $USB_CDC_PORT"

espflash flash --port "$USB_CDC_PORT" target/riscv32imac-unknown-none-elf/release/main

# Verify firmware is running by capturing boot messages
# IMPORTANT: If this fails (device stuck in download mode), SKIP and continue to Test 8
# Reason: Interactive tests (2-7) require manual execution anyway
sleep 1

# Create Python script with variable substitution
cat > /tmp/verify_boot.py << EOF
import serial
import time
import sys

port = sys.argv[1] if len(sys.argv) > 1 else '$USB_CDC_PORT'

try:
    ser = serial.Serial(port, 115200, timeout=3)

    # Strategy 1: RTS+DTR reset (ESP32 standard reset sequence)
    ser.setRTS(True)   # IO0 = HIGH (normal boot, not download mode)
    ser.setDTR(False)  # EN = HIGH (not in reset)
    time.sleep(0.05)
    ser.setDTR(True)   # EN = LOW (enter reset)
    time.sleep(0.05)
    ser.setDTR(False)  # EN = HIGH (exit reset, should boot firmware)
    time.sleep(1.5)

    output = ser.read(ser.in_waiting).decode('utf-8', errors='replace')

    # Check if we got actual boot messages (not just bootloader in download mode)
    if output and "waiting for download" not in output:
        print("✓ Firmware boot verified")
        if output.strip():
            print(output[:500])  # Print first 500 chars
    elif output:
        # Device in download mode - try closing/reopening serial port
        ser.close()
        time.sleep(0.3)
        ser = serial.Serial(port, 115200, timeout=2)
        time.sleep(1.0)
        output2 = ser.read(ser.in_waiting).decode('utf-8', errors='replace')
        if output2 and "waiting for download" not in output2:
            print("✓ Firmware boot verified (after port reset)")
            print(output2[:500])
        else:
            print("⚠ Device in download mode - SKIPPING boot verification")
            print("  → Infrastructure tests (8-11) will proceed")
    else:
        print("⚠ No output - SKIPPING boot verification")
        print("  → Infrastructure tests (8-11) will proceed")

    ser.close()
except Exception as e:
    print(f"⚠ Boot verification failed: {e}")
    print("  → SKIPPING - Infrastructure tests (8-11) will proceed")
EOF

python3 /tmp/verify_boot.py "$USB_CDC_PORT"
```

**Note:** Boot verification may fail (device stuck in download mode). This is non-blocking - infrastructure tests will proceed.

#### Step 5: Infrastructure Tests (Automated Static Analysis)

These tests verify the project is properly configured for debugging. Use temp scripts for complex logic:

```bash
cat > /tmp/run_infrastructure_tests.sh << 'SCRIPT'
#!/bin/bash
set -e

source /tmp/probe_env.sh  # Load LESSON_DIR

cd /Users/shanemattner/Desktop/esp32-c6-agentic-firmware/lessons/$LESSON_DIR

echo "=== Step 4: Infrastructure Tests ==="
echo ""

# Test 8: Debug Symbols
echo "=== Test 8: Debug Symbols ==="
if file target/riscv32imac-unknown-none-elf/release/main | grep -q "not stripped"; then
    echo "✓ PASS: Binary contains debug symbols (not stripped)"
else
    echo "✗ FAIL: Binary appears to be stripped"
fi
echo ""

# Test 9: Source Code Structure
echo "=== Test 9: Source Code Structure ==="
if [ -f src/bin/main.rs ] && [ -f src/lib.rs ] && [ -f src/mpu9250.rs ] && [ -f src/cli.rs ]; then
    echo "✓ PASS: All required source files present"
    echo "  - src/bin/main.rs: $(wc -l < src/bin/main.rs) lines"
    echo "  - src/lib.rs: $(wc -l < src/lib.rs) lines"
    echo "  - src/mpu9250.rs: $(wc -l < src/mpu9250.rs) lines"
    echo "  - src/cli.rs: $(wc -l < src/cli.rs) lines"
else
    echo "✗ FAIL: Missing source files"
fi
echo ""

# Test 10: GDB Configuration Files (for future GDB testing)
echo "=== Test 10: GDB Configuration Files ==="
echo "NOTE: This lesson supports both probe-rs and GDB workflows"
if [ -f .gdbinit ] && [ -f gdb_helpers.py ] && [ -f openocd.cfg ]; then
    echo "✓ PASS: All GDB configuration files present"
    echo "  - .gdbinit: $(wc -l < .gdbinit) lines"
    echo "  - gdb_helpers.py: $(wc -l < gdb_helpers.py) lines"
    echo "  - openocd.cfg: $(wc -l < openocd.cfg) lines"
else
    echo "✗ FAIL: Missing GDB configuration files"
fi
echo ""

# Test 11: Python Helper Script Syntax
echo "=== Test 11: Python Helper Script Syntax ==="
if python3 -m py_compile gdb_helpers.py 2>&1; then
    echo "✓ PASS: gdb_helpers.py has valid Python syntax"
else
    echo "✗ FAIL: gdb_helpers.py has syntax errors"
fi
echo ""

# Test 12: Cargo.toml Debug Configuration
echo "=== Test 12: Cargo.toml Debug Configuration ==="
if grep -q "^\[\[bin\]\]" Cargo.toml; then
    echo "✓ [[bin]] section found in Cargo.toml"
    grep -A 2 "^\[\[bin\]\]" Cargo.toml
else
    echo "✗ [[bin]] section NOT found"
fi
echo ""

if grep -q "debug = 2" Cargo.toml || grep -q "debug = true" Cargo.toml; then
    echo "✓ debug symbols enabled in release profile"
    grep -B 1 -A 3 "^\[profile.release\]" Cargo.toml | head -6
else
    echo "⚠ Warning: debug symbols may not be enabled in release build"
fi
echo ""

echo "=== Infrastructure Tests Complete ==="
SCRIPT

chmod +x /tmp/run_infrastructure_tests.sh
/tmp/run_infrastructure_tests.sh
```

### Expected Outputs

**Automated Tests (8-12):**

**Test 8: Debug Symbols**
- Expected: Binary contains debug information
- Success criteria: `file` command shows "not stripped"

**Test 9: Source Code Structure**
- Expected: All required source files present with reasonable line counts
- Success criteria: main.rs, lib.rs, mpu9250.rs, cli.rs all exist

**Test 10: GDB Configuration Files**
- Expected: .gdbinit, gdb_helpers.py, openocd.cfg exist
- Note: Lesson supports both probe-rs and GDB+OpenOCD workflows

**Test 11: Python Helper Script Syntax**
- Expected: gdb_helpers.py has valid Python syntax
- Success criteria: py_compile succeeds with no errors

**Test 12: Cargo.toml Debug Configuration**
- Expected: [[bin]] section and debug = 2 in release profile
- Success criteria: Proper binary configuration for debugging

**Manual Interactive Tests (2-7):**

These require an interactive terminal with TTY and cannot be automated:

- Test 2: probe-rs attach (interactive REPL)
- Test 3: Peripheral register reads (read32 commands)
- Test 4: Memory inspection (read command)
- Test 5: Breakpoint at main()
- Test 6: Function breakpoint (handle_command)
- Test 7: Call stack analysis (backtrace)

The report will provide exact commands for manual testing

### Report Format

Generate a markdown report with this structure:

```markdown
# GDB Lesson 07 Test Report (Infrastructure Validation)

**Date:** YYYY-MM-DD HH:MM
**Mode:** quick/full
**Debugger:** probe-rs (infrastructure only)
**Duration:** X minutes

## Summary
- Total Automated Tests: 5 (Tests 8-12)
- Passed: Y
- Failed: Z
- Success Rate: Y/5 (%)
- Manual Tests: 6 (Tests 2-7) - see Manual Test Instructions below

## Environment
- ESP32-C6: Connected ✓/✗ (USB port: /dev/cu.usbmodemXXXX)
- JTAG Probe: Detected ✓/✗ (VID:PID:Serial)
- probe-rs: Available ✓/✗ (version X.Y.Z)
- espflash: Available ✓/✗
- Firmware: Built ✓/✗ (with debug symbols)

## Automated Test Results

### Infrastructure Tests (Static Analysis)

#### ✓/✗ Test 8: Debug Symbols
- Command: `file target/.../main | grep "not stripped"`
- Expected: Binary has debug symbols
- Actual: [result]
- Status: PASS/FAIL

#### ✓/✗ Test 9: Source Code Structure
- Command: Verified all required source files exist
- Expected: main.rs, lib.rs, mpu9250.rs, cli.rs present
- Actual: [result with line counts]
- Status: PASS/FAIL

#### ✓/✗ Test 10: GDB Configuration Files
- Command: Verified .gdbinit, gdb_helpers.py, openocd.cfg exist
- Expected: All GDB config files present
- Actual: [result]
- Status: PASS/FAIL

#### ✓/✗ Test 11: Python Helper Script Syntax
- Command: `python3 -m py_compile gdb_helpers.py`
- Expected: Valid Python syntax
- Actual: [result]
- Status: PASS/FAIL

#### ✓/✗ Test 12: Cargo.toml Debug Configuration
- Command: Verified [[bin]] section and debug settings
- Expected: Proper binary config with debug = 2
- Actual: [result]
- Status: PASS/FAIL

## Manual Test Instructions

⚠️ **Tests 2-7 require an interactive terminal** (cannot be automated in non-TTY contexts)

To complete interactive debugging verification, run these commands in a terminal:

```bash
source /tmp/probe_env.sh  # Load detected hardware config
cd lessons/07-gdb-debugging

# Start probe-rs interactive session
probe-rs attach --chip esp32c6 $PROBE_ARG target/riscv32imac-unknown-none-elf/release/main

# In probe-rs prompt, run these commands:
> read32 0x60013004   # Test 3: I2C STATUS register
> read32 0x6000403C   # Test 3: GPIO IN register
> read 0x3FC88000 64  # Test 4: Memory inspection (64 bytes from RAM)
> break main          # Test 5: Set breakpoint at main
> reset               # Test 5: Reset and trigger breakpoint
> continue            # Test 5: Resume execution
> backtrace           # Test 7: Show call stack
> quit                # Exit probe-rs
```

**Expected Results:**
- register reads return valid 32-bit hex values
- breakpoint at main() halts execution
- backtrace shows function call hierarchy

## Issues Found

[List any infrastructure issues detected]

## Recommendations

[List any improvements to lesson configuration or documentation]

## Conclusion

**Infrastructure Status:** PASS/FAIL

All automated tests verify the project is properly configured for debugging.
Manual interactive tests should be performed to validate end-to-end debugging workflows.
```

### Error Handling

If you encounter errors:

1. **probe-rs not found:**
   - Install with: `cargo install probe-rs --features cli`
   - Verify installation: `which probe-rs`

2. **No probes detected (`probe-rs list` empty):**
   - Verify JTAG debugger USB is connected
   - Check USB permissions (may need `sudo`)
   - Try unplugging/replugging JTAG debugger

3. **"Exclusive access" error when attaching:**
   - Another process is using the probe
   - Check: `ps aux | grep probe-rs`
   - Kill existing sessions: `pkill -f probe-rs`

4. **espflash cannot find port:**
   - USB CDC port changed after replug
   - Re-detect: `ls /dev/cu.usbmodem*`
   - Update `$USB_CDC_PORT` variable

5. **Firmware doesn't boot (no boot messages):**
   - Check USB cable supports data (not just power)
   - Verify baud rate is 115200
   - Try different USB port on computer

6. **Breakpoint not hit:**
   - Verify firmware has debug symbols: `file target/.../main | grep "not stripped"`
   - Check function name exists: Try simpler breakpoint like `break main`
   - Reset device after setting breakpoint

7. **Peripheral register reads return unexpected values:**
   - Verify peripheral is initialized (check boot messages)
   - Check address is correct (refer to ESP32-C6 TRM)
   - Try reading right after firmware boots

### Important Notes

- **DO NOT guess or assume** - If a test cannot run due to missing hardware, mark as SKIPPED with reason
- **Capture actual output** - Include real test output in report, not generic examples
- **Be thorough** - Even if a test passes, note any unexpected behavior
- **Focus on automation** - Tests 8-12 are the priority; Tests 2-7 require manual execution

### Success Criteria

**Quick mode passes if:**
- At least 4/5 infrastructure tests pass (80%+)
- Firmware builds and flashes successfully
- Binary has debug symbols
- No critical configuration issues found

**Full mode passes if:**
- All 5 infrastructure tests pass (100%)
- Manual test instructions are clear and complete
- Report documents any lesson documentation gaps

---

**After testing, report your findings to the user with the markdown report.**
