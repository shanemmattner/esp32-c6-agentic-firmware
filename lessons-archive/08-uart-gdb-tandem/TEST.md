# Lesson 08 Test Specification

**Lesson:** UART + GDB Tandem Debugging
**Type:** Hardware validation required
**Duration:** ~15-20 minutes

---

## Hardware Setup

### Required Equipment
- ESP32-C6 development board
- USB-C cable (for GDB via USB-JTAG)
- FTDI USB-to-UART adapter
- 3x jumper wires

### Wiring

| ESP32-C6 | FTDI Adapter |
|----------|--------------|
| GPIO23 (TX) | RX |
| GPIO15 (RX) | TX |
| GND | GND |

**Verification:** Run `../../scripts/test-uart-pins.sh 23 15 5` to confirm wiring

---

## Automated Tests

### Test 1: Build Verification

```bash
cargo build --release
```

**Expected:** Clean build with no errors
**Pass Criteria:** Exit code 0, binary created in `target/riscv32imac-unknown-none-elf/release/main`

**Result:** ☐ PASS ☐ FAIL

---

### Test 2: Flash to Hardware

```bash
source ../../scripts/find-esp32-ports.sh
espflash flash --port $USB_CDC_PORT target/riscv32imac-unknown-none-elf/release/main
```

**Expected:** Successful flash
**Pass Criteria:** "Flashing has completed!" message, no errors

**Result:** ☐ PASS ☐ FAIL

---

### Test 3: UART Stream Verification

```bash
# Run for 10 seconds
python3 ../../.claude/templates/read_uart.py $FTDI_PORT 10
```

**Expected Output:**
```
=== Variable Streaming System ===
Slot 0: &sensor_1 = 0x3FC8ABCD -> 100
Slot 1: &sensor_2 = 0x3FC8ABD0 -> 200
Slot 2: &counter  = 0x3FC8ABD4 -> 0
Slot 3: &state    = 0x3FC8ABD8 -> 1

Stream: s0=100 s1=200 s2=0 s3=1
Stream: s0=101 s1=200 s2=1 s3=1
Stream: s0=102 s1=200 s2=2 s3=1
```

**Pass Criteria:**
- ✅ Header appears once
- ✅ Stream lines appear every ~100ms
- ✅ Counter values increment (s2)
- ✅ No garbled output

**Result:** ☐ PASS ☐ FAIL

---

## Interactive Tests

### Test 4: GDB Connection

**Steps:**
1. Flash firmware (if not already done)
2. In new terminal:
```bash
riscv32-esp-elf-gdb target/riscv32imac-unknown-none-elf/release/main
(gdb) target remote :3333
(gdb) continue
```

**Expected:**
- GDB connects without errors
- Firmware continues running
- UART stream continues in background

**Pass Criteria:**
- ✅ GDB shows "Continuing"
- ✅ No connection errors
- ✅ UART terminal still receiving data

**Result:** ☐ PASS ☐ FAIL

---

### Test 5: Variable Injection

**Steps:**
```gdb
(gdb) interrupt
(gdb) print sensor_1
(gdb) set sensor_1 = 9999
(gdb) continue
```

**Watch UART terminal**

**Expected:**
- UART shows `s0=9999` in next stream update
- Stream continues normally

**Pass Criteria:**
- ✅ Injected value appears in UART within 1 second
- ✅ No crashes or errors

**Result:** ☐ PASS ☐ FAIL

---

### Test 6: Pointer Redirection (Advanced)

**Steps:**
```gdb
(gdb) interrupt
(gdb) print &counter
$1 = (u32 *) 0x3FC8XXXX  # Note the address

(gdb) set SLOTS[0].ptr = $1
(gdb) continue
```

**Watch UART terminal**

**Expected:**
- Slot 0 now streams counter value
- Counter increments visible in s0

**Pass Criteria:**
- ✅ Slot redirection works
- ✅ Values are correct
- ✅ No crashes

**Result:** ☐ PASS ☐ FAIL

---

### Test 7: Memory Safety Validation

**Steps:**
```gdb
(gdb) interrupt
# Try invalid address (outside RAM)
(gdb) set SLOTS[0].ptr = 0x00000000
(gdb) continue
```

**Watch UART terminal**

**Expected:**
- Firmware detects invalid pointer
- Error message or safe fallback
- Firmware does NOT crash

**Pass Criteria:**
- ✅ No panic or reset
- ✅ Graceful error handling

**Result:** ☐ PASS ☐ FAIL

---

### Test 8: Hardware Watchpoint

**Steps:**
```gdb
(gdb) interrupt
(gdb) watch counter
(gdb) continue
```

**Expected:**
- GDB stops when counter changes
- Shows old and new values

**Pass Criteria:**
- ✅ Watchpoint triggers correctly
- ✅ Can resume with `continue`

**Result:** ☐ PASS ☐ FAIL

---

## Performance Tests

### Test 9: UART Throughput

**Monitor UART for 60 seconds, count messages:**

```bash
python3 ../../.claude/templates/read_uart.py $FTDI_PORT 60 | grep "Stream:" | wc -l
```

**Expected:** ~600 lines (10 Hz × 60 seconds)

**Pass Criteria:**
- ✅ Message rate 8-12 Hz (allows jitter)
- ✅ No dropped frames
- ✅ Consistent timing

**Result:** ☐ PASS ☐ FAIL

---

### Test 10: GDB Interrupt/Resume Cycle

**Stress test: Interrupt and resume 10 times**

```gdb
(gdb) interrupt
(gdb) continue
# Repeat 10x
```

**Expected:**
- Each interrupt succeeds
- Each resume restores UART stream
- No degradation over time

**Pass Criteria:**
- ✅ All 10 cycles complete
- ✅ UART stream remains stable
- ✅ No memory leaks or crashes

**Result:** ☐ PASS ☐ FAIL

---

## Troubleshooting

### No UART Output

**Check:**
1. Wiring (use `test-uart-pins.sh`)
2. Correct FTDI port: `ls $FTDI_PORT`
3. Baud rate: 115200 (default)
4. Firmware actually flashed

### GDB Connection Fails

**Check:**
1. USB-JTAG cable connected
2. Port not in use: `ps aux | grep openocd`
3. espflash can connect: `espflash board-info --port $USB_CDC_PORT`

### Garbled UART

**Check:**
1. Only one process reading port
2. Baud rate matches (115200)
3. UART adapter drivers installed

### Variable Injection Not Working

**Check:**
1. Using debug build or release with symbols
2. Variable names match code exactly
3. GDB actually interrupted firmware

---

## Test Results Summary

| Test | Expected | Status |
|------|----------|--------|
| 1. Build | Compiles | ☐ PASS ☐ FAIL |
| 2. Flash | Success | ☐ PASS ☐ FAIL |
| 3. UART Stream | Visible output | ☐ PASS ☐ FAIL |
| 4. GDB Connection | Connects | ☐ PASS ☐ FAIL |
| 5. Variable Injection | Works | ☐ PASS ☐ FAIL |
| 6. Pointer Redirect | Works | ☐ PASS ☐ FAIL |
| 7. Memory Safety | Enforced | ☐ PASS ☐ FAIL |
| 8. Watchpoint | Triggers | ☐ PASS ☐ FAIL ☐ SKIP |
| 9. Throughput | 10 Hz | ☐ PASS ☐ FAIL ☐ SKIP |
| 10. Stress Test | Stable | ☐ PASS ☐ FAIL ☐ SKIP |

## Pass Criteria

**Mandatory tests (must pass):**
- Tests 1-7

**Optional tests:**
- Tests 8-10 (advanced features)

**Overall status:** PASS if all mandatory tests pass

---

## Expected Test Duration

| Test | Duration |
|------|----------|
| Build & Flash | 2-3 min |
| UART Verification | 1-2 min |
| GDB Connection | 1-2 min |
| Variable Injection | 2-3 min |
| Pointer Redirection | 3-5 min |
| Memory Safety | 2-3 min |
| Performance | 3-5 min |
| **Total** | **15-20 min** |

---

## Notes

{Add any observations, issues, or deviations from expected behavior}

---

**Tested by:** ________________
**Date:** ________________
**Hardware:** ESP32-C6 DevKit
**Software:** esp-hal 1.0.0
