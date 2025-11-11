---
description: Comprehensive test suite for GDB debugging lesson (Lesson 07) using probe-rs
argument-hint: [mode: quick|full] (default: quick)
---

# Test GDB Debugging Lesson (Lesson 07)

You are testing the GDB debugging capabilities documented in Lesson 07 **using probe-rs** as the debugger.

## Test Mode

{{argument}} <!-- "quick" or "full" - defaults to "quick" if not specified -->

- **quick**: Core capabilities only (~5-10 minutes)
- **full**: All capabilities including advanced scenarios (~15-20 minutes)

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

### Test Structure

Your tests should be **deterministic and reproducible**. Follow this structure:

```
1. Environment Setup
   ├─ Clean up existing debug sessions
   ├─ Detect USB ports dynamically
   ├─ Auto-detect ESP JTAG probe
   ├─ Build firmware with debug symbols
   └─ Flash to ESP32-C6 using espflash

2. Core probe-rs Tests (ALWAYS run these)
   ├─ Test 1: Firmware boots successfully
   ├─ Test 2: probe-rs can attach to running firmware
   ├─ Test 3: Peripheral register inspection (I2C, GPIO)
   ├─ Test 4: Memory inspection (stack, heap, buffers)
   ├─ Test 5: Breakpoint at main()
   ├─ Test 6: Function breakpoint (handle_command)
   └─ Test 7: Call stack analysis (backtrace)

3. Infrastructure Tests (ALWAYS run these)
   ├─ Test 8: Debug symbols present in binary
   ├─ Test 9: Source code structure verified
   ├─ Test 10: .gdbinit configuration exists (for future GDB use)
   └─ Test 11: Python helper scripts syntax valid

4. Advanced Tests (ONLY in "full" mode)
   ├─ Test 12: Reset and re-attach
   ├─ Test 13: Multi-breakpoint workflow
   ├─ Test 14: Complex scenario - Debug "button not responding"
   └─ Test 15: Complex scenario - Debug "I2C timeout"

Note: GDB-specific features (Python scripts, custom commands, watchpoints) are
documented but not tested with probe-rs. They require riscv32-esp-elf-gdb.
```

### Execution Guidelines

All tests use **probe-rs** as the debugger. Follow these steps in order:

#### Step 1: Environment Setup and Detection

```bash
cd /Users/shanemattner/Desktop/esp32-c6-agentic-firmware/lessons/07-gdb-debugging

# Clean up any existing debug sessions
echo "Cleaning up existing debug sessions..."
pkill -f "probe-rs" || true
pkill -f "openocd" || true
sleep 1

# Detect USB ports dynamically
echo "Detecting ESP32-C6 USB ports..."
USB_CDC_PORT=$(ls /dev/cu.usbmodem* 2>/dev/null | head -1)
UART_PORT=$(ls /dev/cu.usbserial* 2>/dev/null | head -1)
echo "  USB CDC: ${USB_CDC_PORT:-not found}"
echo "  UART: ${UART_PORT:-not found}"

# Verify probe-rs is available
if ! which probe-rs > /dev/null 2>&1; then
    echo "✗ ERROR: probe-rs not found - cannot run tests"
    exit 1
fi
echo "✓ probe-rs found: $(which probe-rs)"

# Detect probes
echo "Detecting probes..."
probe-rs list

# Try to auto-detect ESP JTAG probe number
ESP_PROBE=$(probe-rs list 2>&1 | grep -i "esp.*jtag" | grep -oE '^\[([0-9]+)\]' | tr -d '[]' | head -1)
if [ -n "$ESP_PROBE" ]; then
    echo "✓ Auto-detected ESP JTAG probe: $ESP_PROBE"
    PROBE_ARG="--probe $ESP_PROBE"
else
    echo "⚠ Could not auto-detect ESP JTAG probe - will try without --probe flag"
    PROBE_ARG=""
fi
```

#### Step 2: Build firmware with debug symbols
```bash
cargo build --release  # Has debug=2 in Cargo.toml
```

#### Step 3: Flash firmware using espflash

**Strategy:** Use `espflash` for flashing (NOT `probe-rs run`). This avoids exclusive lock issues.

```bash
# Use the auto-detected USB CDC port from Step 1
espflash flash --port ${USB_CDC_PORT} target/riscv32imac-unknown-none-elf/release/main

# Verify firmware is running by capturing boot messages
sleep 2
python3 << EOF
import serial, time
ser = serial.Serial('${USB_CDC_PORT}', 115200, timeout=2)
ser.setDTR(False); time.sleep(0.1); ser.setDTR(True); time.sleep(1)
print(ser.read(ser.in_waiting).decode('utf-8', errors='replace'))
ser.close()
EOF
```

Expected output: Should see "✓ I2C initialized", "✓ NeoPixel initialized", etc.

#### Step 4: Run Core Tests with probe-rs

**Test 1: Firmware Boot Verification**
```bash
# Already done in Step 3 - firmware should show successful peripheral initialization
```

**Test 2: probe-rs Attach**
```bash
# Attach to running firmware (use $PROBE_ARG from Step 1)
probe-rs attach --chip esp32c6 $PROBE_ARG target/riscv32imac-unknown-none-elf/release/main
# Should enter interactive mode without errors
# Type 'quit' to exit
```

**Test 3: Peripheral Register Reads**
```bash
probe-rs attach --chip esp32c6 $PROBE_ARG target/riscv32imac-unknown-none-elf/release/main
# In probe-rs interactive mode:
> read32 0x60013004   # I2C STATUS register
> read32 0x6000403C   # GPIO IN register
> quit
```

**Test 4: Memory Inspection**
```bash
probe-rs attach --chip esp32c6 $PROBE_ARG target/riscv32imac-unknown-none-elf/release/main
> read 0x3FC88000 64   # Read 64 bytes from RAM base
> quit
```

**Test 5: Breakpoint at main()**
```bash
probe-rs attach --chip esp32c6 $PROBE_ARG target/riscv32imac-unknown-none-elf/release/main
> break main
> reset
# Should stop at main() entry point
> continue
> quit
```

**Test 6: Function Breakpoint**
```bash
probe-rs attach --chip esp32c6 $PROBE_ARG target/riscv32imac-unknown-none-elf/release/main
> break handle_command
> continue
# Send UART command via separate terminal to trigger breakpoint
# (This test may be skipped if interactive UART is not available)
> quit
```

**Test 7: Call Stack Analysis**
```bash
probe-rs attach --chip esp32c6 $PROBE_ARG target/riscv32imac-unknown-none-elf/release/main
> break main
> reset
> backtrace
# Should show call stack
> quit
```

#### Step 5: Infrastructure Tests (Static Analysis)

**Test 8: Debug Symbols**
```bash
file target/riscv32imac-unknown-none-elf/release/main | grep "not stripped"
# Should show "not stripped"
```

**Test 9: Source Code Structure**
```bash
test -f src/bin/main.rs && test -f src/lib.rs && test -f src/mpu9250.rs && test -f src/cli.rs
echo $?  # Should be 0 (success)
```

**Test 10: GDB Configuration Files**
```bash
test -f .gdbinit && test -f gdb_helpers.py && test -f openocd.cfg
echo $?  # Should be 0
```

**Test 11: Python Helper Script Syntax**
```bash
python3 -m py_compile gdb_helpers.py
echo $?  # Should be 0 (no syntax errors)
```

### Expected Outputs

**Test 1: Firmware Boot Verification**
- Expected: Boot messages show successful peripheral initialization
- Success criteria: See "✓ I2C initialized", "✓ NeoPixel initialized", "✓ Button configured", "✓ UART initialized"

**Test 2: probe-rs Attach**
- Expected: probe-rs enters interactive mode without errors
- Success criteria: No "exclusive access" errors, interactive prompt appears

**Test 3: Peripheral Register Reads**
- Expected: Can read I2C STATUS (0x60013004) and GPIO IN (0x6000403C)
- Success criteria: Returns valid 32-bit values (not error messages)

**Test 4: Memory Inspection**
- Expected: Can read RAM contents without errors
- Success criteria: Returns hex dump of memory contents

**Test 5: Breakpoint at main()**
- Expected: probe-rs stops at main() entry after reset
- Success criteria: Execution pauses, shows source location

**Test 6: Function Breakpoint**
- Expected: Stops when handle_command is called (if UART command sent)
- Success criteria: Break in handle_command function
- Note: May skip if interactive UART not available during test

**Test 7: Call Stack Analysis**
- Expected: Shows function call hierarchy
- Success criteria: `backtrace` displays multiple stack frames

**Test 8: Debug Symbols**
- Expected: Binary contains debug information
- Success criteria: `file` command shows "not stripped"

**Test 9: Source Code Structure**
- Expected: All required source files present
- Success criteria: All test commands return 0 (success)

**Test 10: GDB Configuration Files**
- Expected: .gdbinit, gdb_helpers.py, openocd.cfg exist
- Success criteria: All files present (for future GDB use)

**Test 11: Python Helper Script Syntax**
- Expected: gdb_helpers.py has valid Python syntax
- Success criteria: py_compile succeeds with no errors

### Report Format

Generate a markdown report with this structure:

```markdown
# GDB Lesson 07 Test Report (probe-rs)

**Date:** YYYY-MM-DD HH:MM
**Mode:** quick/full
**Debugger:** probe-rs
**Duration:** X minutes

## Summary
- Total Tests: 11
- Passed: Y
- Failed: Z
- Skipped: N
- Success Rate: Y/11 (%)

## Environment
- ESP32-C6: Connected ✓/✗ (USB port: /dev/cu.usbmodemXXXX)
- JTAG Probe: Connected ✓/✗ (probe #X)
- probe-rs: Available ✓/✗
- Firmware: Built ✓/✗

## Test Results

### Core Tests (probe-rs)

#### ✓/✗/⊘ Test 1: Firmware Boot Verification
- Command: Captured boot messages via USB CDC
- Expected: Peripheral initialization messages
- Actual: [describe what happened]
- Status: PASS/FAIL/SKIP
- Notes: [any observations]

[... repeat for Tests 2-7 ...]

### Infrastructure Tests (Static Analysis)

#### ✓/✗ Test 8: Debug Symbols
- Command: `file target/.../main | grep "not stripped"`
- Expected: Binary has debug symbols
- Actual: [result]
- Status: PASS/FAIL

[... repeat for Tests 9-11 ...]

## Issues Found

1. [Issue description]
   - Test: Test X
   - Severity: High/Medium/Low
   - Root cause: [analysis]
   - Suggested fix: [recommendation]

## Recommendations

- [List any improvements to lesson, docs, or scripts]

## Next Steps

- [What should be done to address failures]
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
- **Capture actual output** - Include real probe-rs output in report, not generic examples
- **Be thorough** - Even if a test passes, note any unexpected behavior
- **Time-box tests** - If a test hangs, wait max 30 seconds then FAIL and move on
- **Clean up processes** - probe-rs auto-cleans on exit, but check for orphaned processes

### Success Criteria

**Quick mode passes if:**
- At least 9/11 tests pass (81%+)
- Firmware boots successfully
- probe-rs can attach and read registers
- No critical issues found

**Full mode passes if:**
- At least 12/15 tests pass (80%+)
- At least one breakpoint test succeeds
- Report identifies any lesson documentation gaps

---

**After testing, report your findings to the user with the markdown report.**
