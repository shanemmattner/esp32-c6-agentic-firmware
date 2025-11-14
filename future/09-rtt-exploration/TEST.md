# Lesson 08 Test Specification

**Lesson:** 08 - Structured Logging with defmt + RTT
**Hardware:** ESP32-C6 + JTAG debugger
**Test Duration:** ~3 minutes (quick), ~10 minutes (full)

---

## Hardware Setup

### Required Connections

```
ESP32-C6 Pin      →  Peripheral
─────────────────────────────────
GPIO4             →  JTAG TMS
GPIO5             →  JTAG TDI
GPIO6             →  JTAG TDO
GPIO7             →  JTAG TCK
GND               →  GND (shared)
USB-C             →  Computer (for flashing and USB CDC)
```

### Verification Checklist

- [ ] ESP32-C6 USB-C cable connected
- [ ] JTAG debugger connected (ESP-Prog or compatible)
- [ ] probe-rs installed and working

---

## Test Modes

### Quick Mode (default)
- Build and flash firmware
- Infrastructure validation (debug symbols, defmt configuration)
- RTT buffer verification
- Basic log message capture
- ~3 minutes

### Full Mode
- All quick mode tests
- Comprehensive RTT performance testing
- Multi-level logging verification
- Timestamp validation
- Format string optimization checks
- ~10 minutes

---

## Automated Tests

These tests run automatically via `/test-lesson 08`:

### Test 1: Build Verification
**Purpose:** Verify firmware compiles with defmt and RTT
**Command:** `cargo build --release`
**Success Criteria:**
- Exit code 0
- Binary size reasonable (~40-60KB)
- No compilation errors
- defmt metadata generated

### Test 2: Debug Symbols
**Purpose:** Verify debugging is possible
**Command:** `file target/riscv32imac-unknown-none-elf/release/main | grep "not stripped"`
**Success Criteria:** Binary contains debug information

### Test 3: Source Code Structure
**Purpose:** Verify all required files present
**Command:** Check for required source files
**Success Criteria:**
- `src/bin/main.rs` exists
- `Cargo.toml` has defmt and defmt-rtt dependencies
- `Cargo.toml` has [[bin]] section

### Test 4: defmt Configuration
**Purpose:** Verify defmt is properly configured
**Command:** Check Cargo.toml for defmt dependencies and features
**Success Criteria:**
- `defmt = "0.3"` present
- `defmt-rtt = "0.4"` present
- No conflicting logging backends (esp-println)

### Test 5: Flash Firmware
**Purpose:** Verify firmware can be flashed to hardware
**Command:** `espflash flash --port $USB_CDC_PORT target/.../main`
**Success Criteria:**
- Flash succeeds
- Firmware size appropriate for defmt binary
- No "exclusive access" errors

### Test 6: RTT Buffer Configuration
**Purpose:** Verify RTT buffers are configured
**Method:** Check binary for RTT control block
**Success Criteria:**
- RTT control block present in binary
- Up buffer configured
- Down buffer configured (if bidirectional)

### Test 7: Log Message Capture (probe-rs)
**Purpose:** Verify defmt logs work over RTT
**Command:** `probe-rs run --chip esp32c6 target/.../main`
**Success Criteria:**
- Firmware boots
- Log messages appear in real-time
- No RTT buffer overflows
- Timestamps visible (if enabled)

**Expected Output:**
```
INFO  - System boot
INFO  - defmt RTT logging initialized
DEBUG - Example debug message
TRACE - Low-level trace: value=42
```

### Test 8: Log Level Filtering
**Purpose:** Verify log levels work correctly
**Command:** Build with different DEFMT_LOG environment variable
**Success Criteria:**
- `DEFMT_LOG=info` suppresses DEBUG and TRACE
- `DEFMT_LOG=debug` shows INFO and DEBUG
- `DEFMT_LOG=trace` shows all messages

---

## Interactive Tests (Manual)

⚠️ **These tests require manual execution**

### Interactive Test 1: Real-Time Log Streaming
**Purpose:** Verify real-time log output with probe-rs

**Setup:**
```bash
source /tmp/test_env.sh
cd "$LESSON_DIR"
```

**Commands:**
```bash
# Start probe-rs with RTT logging
probe-rs run --chip esp32c6 --probe $ESP_PROBE target/riscv32imac-unknown-none-elf/release/main
```

**Expected Output:**
```
INFO  - System booting...
INFO  - Peripherals initialized
DEBUG - Button state: released
DEBUG - LED state: off
TRACE - Loop iteration: 0
TRACE - Loop iteration: 1
```

**Success Criteria:**
- Messages appear in real-time
- No lag or buffering issues
- Timestamps increment correctly
- Log levels respected

### Interactive Test 2: Format String Optimization
**Purpose:** Verify defmt format string optimization

**Commands:**
```bash
# Check binary size with different logging levels
DEFMT_LOG=off cargo build --release
ls -lh target/.../main

DEFMT_LOG=info cargo build --release
ls -lh target/.../main

DEFMT_LOG=trace cargo build --release
ls -lh target/.../main
```

**Expected Results:**
- Binary size increases with more verbose logging
- defmt overhead is minimal (few KB)
- Format strings not duplicated in binary

**Success Criteria:**
- Binary size difference < 5KB between log levels
- No string duplication in binary

### Interactive Test 3: RTT Performance Under Load
**Purpose:** Verify RTT doesn't drop messages under high throughput

**Commands:**
```bash
# Run firmware with high-frequency logging
probe-rs run --chip esp32c6 target/.../main
```

**Expected Behavior:**
- Firmware logs at high rate (e.g., 1000 messages/sec)
- No "RTT buffer overflow" errors
- Messages arrive in order
- Timestamps are continuous

**Success Criteria:**
- No dropped messages
- RTT keeps up with firmware logging rate
- System remains responsive

---

## Expected Outputs

### Serial Console Output (via probe-rs)

When running with `probe-rs run`, you should see:

```
      probe-rs v0.X.Y
Erasing ✓
Programming [#####] 100% done
Finished in 2.3s
INFO  - ESP32-C6 defmt RTT Logging Demo
INFO  - System initialized
DEBUG - Button: GPIO9 (active LOW)
DEBUG - LED: GPIO8 (WS2812)
INFO  - Entering main loop
TRACE - Loop 0: button=released
TRACE - Loop 1: button=released
DEBUG - Button pressed!
INFO  - LED: red
TRACE - Loop 2: button=pressed
```

### Log Format

defmt provides structured logging:

```
LEVEL - Message
INFO  - Simple message
DEBUG - Value: {=u32}   ← type-safe formatting
TRACE - Multiple: x={=i16}, y={=i16}, z={=i16}
```

---

## Troubleshooting

### Issue: No RTT output
**Symptoms:** probe-rs runs but no log messages
**Cause:** RTT not initialized or JTAG not connected
**Solution:**
```bash
# Verify JTAG connection
probe-rs list

# Check binary has RTT control block
nm target/.../main | grep _SEGGER_RTT

# Rebuild with correct features
cargo clean
cargo build --release
```

### Issue: "RTT buffer overflow"
**Symptoms:** Messages show buffer overflow warning
**Cause:** Logging rate exceeds RTT throughput
**Solution:**
- Reduce logging frequency
- Increase RTT buffer size in firmware
- Use `DEFMT_LOG=info` to reduce volume

### Issue: Garbled log messages
**Symptoms:** Messages corrupted or incomplete
**Cause:** Probe disconnected during logging
**Solution:**
- Keep JTAG probe connected
- Don't unplug during flashing
- Use `espflash` then `probe-rs attach` (not `probe-rs run`)

### Issue: Build fails with "defmt not found"
**Symptoms:** Compilation error about defmt macros
**Cause:** Missing defmt dependency
**Solution:**
```bash
# Verify Cargo.toml has:
# defmt = "0.3"
# defmt-rtt = "0.4"

cargo clean
cargo build --release
```

### Issue: Binary too large
**Symptoms:** Flash fails due to size
**Cause:** Too many log messages or debug level
**Solution:**
```bash
# Build with info level only
DEFMT_LOG=info cargo build --release

# Or disable logging entirely for production
DEFMT_LOG=off cargo build --release
```

---

## Performance Benchmarks (Full Mode)

### Timing Measurements
- Boot time: ~30ms
- First log message: <5ms after boot
- Log throughput: >10,000 messages/sec (typical)
- RTT latency: <1ms

### Memory Usage
- defmt overhead: ~3-5KB (format strings)
- RTT buffer: 1KB (default)
- Stack usage: minimal (<100 bytes per log call)
- Total binary: ~45KB

### RTT Performance
- Throughput: Limited by JTAG speed (typically 1 MB/s)
- Latency: Sub-millisecond
- Overhead: Minimal (non-blocking writes)

---

## Comparison: defmt+RTT vs esp-println

**Advantages of defmt + RTT:**
- ✅ Format strings stored once (not duplicated)
- ✅ Type-safe formatting (compile-time checked)
- ✅ Real-time streaming over JTAG (no UART needed)
- ✅ Zero-cost when disabled
- ✅ Timestamps built-in
- ✅ Smaller binary size

**Advantages of esp-println:**
- ✅ Works without JTAG debugger
- ✅ Simpler setup (just USB)
- ✅ Familiar printf-style syntax

**When to use defmt + RTT:**
- Development with JTAG debugger available
- High-frequency logging
- Binary size critical
- Type safety important

**When to use esp-println:**
- Production debugging (no debugger)
- Simple prototyping
- UART-only connectivity

---

## Notes

- This lesson requires JTAG debugger (USB-JTAG or ESP-Prog)
- probe-rs is the recommended tool (faster than OpenOCD)
- defmt is optimized for embedded systems (minimal overhead)
- RTT (Real-Time Transfer) is faster than UART for debugging
- Expected completion time: 3 minutes for basic validation, 10 minutes for full testing
- Known issues: None

---

**Last Updated:** 2025-11-12
**Verified On:** probe-rs 0.24+, ESP32-C6 rev 0.1, macOS
