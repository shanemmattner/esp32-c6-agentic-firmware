# Lesson 08: USB CDC Streaming - Test Specification

## Hardware Setup

**Requirements:**
- ESP32-C6 development board
- USB-C cable (data-capable)
- Computer with Python 3.7+

**Wiring:**
- USB-C cable from computer to ESP32-C6 USB port
- No external components needed

## Software Setup

```bash
# Install Python dependencies
pip install pyserial matplotlib

# Build firmware
cd lessons/08-usb-cdc-streaming
cargo build --release
```

## Test Procedures

### Test 1: Build Verification

**Goal:** Verify firmware compiles without errors

```bash
cargo build --release
```

**Expected:**
- ✅ Build succeeds
- ✅ No warnings or errors
- ✅ Binary created

**Result:** ☐ PASS ☐ FAIL

---

### Test 2: Flash and Boot

**Goal:** Verify firmware flashes and boots

```bash
cargo run --release
```

**Expected output:**
```
BOOT|version=1.0.0|chip=ESP32-C6
STATUS|msg=Initialization complete|ready=true
```

**Verification:**
- ✅ Firmware uploads successfully
- ✅ BOOT message appears immediately
- ✅ STATUS message confirms initialization

**Result:** ☐ PASS ☐ FAIL

---

### Test 3: Structured Output Format

**Goal:** Verify all message types are correctly formatted

**Monitor output for 10 seconds and verify:**

- ✅ I2C messages: `I2C|addr=0xXX|op=Read|bytes=N|status=Success|ts=NNNN`
- ✅ GPIO messages: `GPIO|pin=N|state=Low|ts=NNNN`
- ✅ SENSOR messages: `SENSOR|id=N|value=NNNN|unit=centi-C|ts=NNNN`
- ✅ HEARTBEAT messages: `HEARTBEAT|count=N|ts=NNNN`

**Verification:**
- ✅ All message types present
- ✅ Pipe-delimited format correct
- ✅ Field names and values present
- ✅ Timestamps increment

**Result:** ☐ PASS ☐ FAIL

---

### Test 4: Python Parser

**Goal:** Verify Python parser can decode all message types

```bash
# Find USB port
ls /dev/cu.usbmodem*

# Run parser (replace with your port)
python3 stream_parser.py /dev/cu.usbmodem2101
```

**Expected output:**
```
📡 Listening on /dev/cu.usbmodem2101 @ 115200 baud
Press Ctrl+C to stop

🚀 BOOT: ESP32-C6 v1.0.0
✓ STATUS: Initialization complete (ready=true)
I2C: addr=0x68 op=Read bytes=6 status=Success
GPIO: pin=8 ⚪ Low
📊 SENSOR 1: 2530 centi-C
💓 Heartbeat #1
```

**Verification:**
- ✅ Parser connects successfully
- ✅ All message types parsed and displayed
- ✅ Emojis render correctly
- ✅ No parsing errors

**Result:** ☐ PASS ☐ FAIL

---

### Test 5: Statistics Mode

**Goal:** Verify statistics tracking works

```bash
python3 stream_parser.py /dev/cu.usbmodem2101 --stats
```

**Run for 10 seconds, then Ctrl+C**

**Expected:**
- ✅ Statistics displayed on each heartbeat
- ✅ Message counts increment
- ✅ Rate calculation reasonable (~10-20 msg/s)
- ✅ Throughput calculation present
- ✅ Final statistics printed on exit

**Result:** ☐ PASS ☐ FAIL

---

### Test 6: CSV Logging

**Goal:** Verify CSV export works

```bash
python3 stream_parser.py /dev/cu.usbmodem2101 --csv test_output.csv
```

**Run for 10 seconds, then Ctrl+C**

**Verification:**
```bash
head -20 test_output.csv
wc -l test_output.csv
```

**Expected:**
- ✅ CSV file created
- ✅ Contains header: `timestamp,type,data`
- ✅ Data rows present
- ✅ Timestamps in ISO format
- ✅ Line count > 100 (for 10 seconds)

**Result:** ☐ PASS ☐ FAIL

---

### Test 7: Performance Test (60 seconds)

**Goal:** Verify sustained throughput and stability

```bash
# Run for 60 seconds
timeout 60 python3 stream_parser.py /dev/cu.usbmodem2101 --csv perf_test.csv --stats

# Analyze results
wc -l perf_test.csv
ls -lh perf_test.csv
```

**Expected results:**
- ✅ Runs for 60 seconds without errors
- ✅ No disconnections
- ✅ Consistent message rate (~10-20 msg/s)
- ✅ Throughput 1-3 KB/s
- ✅ Final message count > 600

**Result:** ☐ PASS ☐ FAIL

---

### Test 8: Real-Time Plotting (Optional)

**Goal:** Verify matplotlib visualization works

```bash
python3 plot_sensor_data.py /dev/cu.usbmodem2101
```

**Expected:**
- ✅ Plot window opens
- ✅ Data appears in real-time
- ✅ X-axis: time, Y-axis: sensor value
- ✅ Plot updates smoothly
- ✅ No lag or freezing

**Result:** ☐ PASS ☐ FAIL ☐ SKIPPED

---

## Test Results Summary

| Test | Expected | Status |
|------|----------|--------|
| 1. Build | Compiles | ☐ PASS ☐ FAIL |
| 2. Flash & Boot | BOOT message | ☐ PASS ☐ FAIL |
| 3. Format | All types present | ☐ PASS ☐ FAIL |
| 4. Parser | Decodes all | ☐ PASS ☐ FAIL |
| 5. Statistics | Tracking works | ☐ PASS ☐ FAIL |
| 6. CSV | Export works | ☐ PASS ☐ FAIL |
| 7. Performance | 60s sustained | ☐ PASS ☐ FAIL |
| 8. Plotting | Visualization | ☐ PASS ☐ FAIL ☐ SKIP |

## Pass Criteria

**Mandatory tests (must pass):**
- Tests 1-7

**Optional tests:**
- Test 8 (plotting)

**Overall status:** PASS if all mandatory tests pass

## Notes

{Add any observations, issues, or deviations from expected behavior}

---

**Tested by:** ________________
**Date:** ________________
**Hardware:** ESP32-C6 DevKit
**Software:** esp-hal 1.0.0, Python 3.X
