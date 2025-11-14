# High-Speed USB CDC Streaming with defmt - Optimization Plan

**Goal:** Maximize USB CDC throughput using defmt for efficient structured logging, testing limits of USB Full Speed (1.5 MB/s theoretical max).

**Date:** 2025-11-13
**Status:** Planning
**Expected Time:** 1-2 hours implementation + testing

---

## Current Baseline Performance

**Existing Implementation:**
- Loop rate: 100 Hz (10ms delay)
- Message rate: ~20 msg/s
- Format: String-based pipe-delimited (`TYPE|field=val|...`)
- Throughput: ~1-3 KB/s
- Utilization: < 0.2% of USB CDC capacity

**Target Performance:**
- Loop rate: 1000 Hz (1ms delay) or faster
- Message rate: 200-1000 msg/s
- Format: defmt binary + text hybrid
- Throughput: 100-500 KB/s
- Utilization: 7-33% of USB CDC capacity

---

## Why defmt for USB CDC Streaming?

### defmt Advantages

1. **Compact binary encoding** - Format strings stored in flash, not sent over wire
2. **Zero-cost abstractions** - Compile-time format string optimization
3. **Type safety** - Strongly typed logging with compile-time checks
4. **Timestamp support** - Built-in microsecond-precision timestamps
5. **Minimal overhead** - ~10-20 bytes per message vs 50-100 bytes for strings

### defmt over USB CDC (vs RTT)

**Why this works:**
- defmt output can route through esp-println to USB CDC
- Still get defmt efficiency (binary encoding in memory)
- Converted to text at USB layer (small overhead)
- Avoids RTT reliability issues on macOS
- Works with standard serial terminals

**Trade-off:**
- Not pure binary (defmt → text → USB)
- But still more efficient than manual string formatting
- Text output is human-readable (easy debugging)

---

## Implementation Plan

### Phase 1: Add defmt Dependencies (15 min)

**Cargo.toml updates:**

```toml
[dependencies]
# Add defmt ecosystem
defmt = "0.3"
esp-println = { version = "0.13", features = ["esp32c6", "log", "defmt-espflash"] }

# Keep existing
esp-hal = { version = "1.0.0", features = ["esp32c6", "unstable"] }
esp-backtrace = { version = "0.15", features = ["esp32c6", "panic-handler", "println"] }
log = "0.4"
esp-bootloader-esp-idf = { version = "0.4.0", features = ["esp32c6"] }
```

**Key addition:** `defmt-espflash` feature enables defmt → USB CDC routing

**Verify:**
```bash
cargo check
```

---

### Phase 2: Convert to defmt Logging (30 min)

**Update src/lib.rs - Add defmt derives:**

```rust
use defmt::Format;

#[derive(Debug, Clone, Copy, Format)]  // Add Format
pub struct I2cTransaction {
    pub addr: u8,
    pub operation: I2cOperation,
    pub bytes_transferred: usize,
    pub status: I2cStatus,
    pub timestamp_ms: u64,
}

// Remove manual Display trait - defmt handles formatting
// Keep Display if you want both options

#[derive(Debug, Clone, Copy, Format)]
pub enum I2cOperation {
    Read,
    Write,
}

#[derive(Debug, Clone, Copy, Format)]
pub enum I2cStatus {
    Success,
    Nack,
    Timeout,
    Error,
}

// Same for all other types: GpioEvent, SensorReading, BootInfo
```

**Update src/bin/main.rs - Use defmt macros:**

```rust
use defmt::{info, debug, warn, error};
use defmt_rtt as _; // Import for linking

#[main]
fn main() -> ! {
    // Boot message
    info!("BOOT version={} chip={}", "1.0.0", "ESP32-C6");

    let _peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    info!("STATUS msg={} ready={}", "Initialization complete", true);

    let mut loop_count: u32 = 0;
    let mut timestamp_ms: u64 = 0;

    loop {
        // I2C transaction every 100ms (keep current rate initially)
        if loop_count % 10 == 0 {
            let i2c_tx = I2cTransaction {
                addr: 0x68,
                operation: I2cOperation::Read,
                bytes_transferred: 6,
                status: I2cStatus::Success,
                timestamp_ms,
            };
            info!("I2C {}", i2c_tx);  // defmt will format efficiently
        }

        // ... other events

        loop_count += 1;
        timestamp_ms += 10;
        delay.delay_millis(10);
    }
}
```

**Benefits:**
- `info!()` macro is compile-time optimized
- Format strings stored in flash (not sent over USB)
- Type information embedded (parser can reconstruct)

---

### Phase 3: High-Speed Loop Optimization (20 min)

**Goal:** Increase from 100 Hz to 1000 Hz (10x throughput)

**Changes in main.rs:**

```rust
const LOOP_DELAY_MS: u32 = 1;  // 1ms = 1000 Hz
const LOOP_RATE_HZ: u32 = 1000 / LOOP_DELAY_MS;

// Adjust event frequencies
loop {
    // I2C every 10ms (100 Hz)
    if loop_count % 10 == 0 {
        info!("I2C addr=0x{:02x} op={:?} bytes={} status={:?} ts={}",
              0x68, I2cOperation::Read, 6, I2cStatus::Success, timestamp_ms);
    }

    // GPIO every 25ms (40 Hz)
    if loop_count % 25 == 0 {
        info!("GPIO pin={} state={:?} ts={}", 8, GpioState::Low, timestamp_ms);
    }

    // Sensor every 50ms (20 Hz)
    if loop_count % 50 == 0 {
        info!("SENSOR id={} value={} unit={} ts={}",
              1, 2530 + (loop_count % 100), "centi-C", timestamp_ms);
    }

    // Heartbeat every 1000ms (1 Hz)
    if loop_count % 1000 == 0 {
        info!("HEARTBEAT count={} ts={}", loop_count / 1000, timestamp_ms);
    }

    loop_count += 1;
    timestamp_ms += LOOP_DELAY_MS as u64;
    delay.delay_millis(LOOP_DELAY_MS);
}
```

**Expected message rate:**
- I2C: 100 msg/s
- GPIO: 40 msg/s
- Sensor: 20 msg/s
- Heartbeat: 1 msg/s
- **Total: ~161 msg/s**

**Expected throughput:**
- Avg message size with defmt: ~30-40 bytes
- 161 msg/s × 35 bytes = ~5.6 KB/s
- Still well under USB capacity (< 0.4%)

---

### Phase 4: Stress Test Mode (15 min)

**Add high-frequency test mode:**

```rust
// Add at top of main()
const STRESS_TEST: bool = false;  // Toggle for stress testing

if STRESS_TEST {
    info!("🔥 STRESS TEST MODE - High frequency logging enabled");

    loop {
        // Log EVERY iteration at 1000 Hz
        info!("STRESS count={} ts={} data={}",
              loop_count, timestamp_ms, loop_count % 256);

        loop_count += 1;
        timestamp_ms += 1;
        delay.delay_millis(1);
    }
}
```

**Stress test metrics:**
- Message rate: 1000 msg/s
- Avg message size: ~35 bytes
- Throughput: ~35 KB/s (2.3% of USB CDC)

---

### Phase 5: Batched Logging for Maximum Throughput (20 min)

**Concept:** Log multiple values in single message to reduce overhead

```rust
// Instead of:
info!("I2C addr=0x{:02x}", 0x68);
info!("GPIO pin={}", 8);
info!("SENSOR value={}", 2530);

// Batch into single message:
info!("BATCH i2c_addr=0x{:02x} gpio_pin={} sensor_val={} ts={}",
      0x68, 8, 2530, timestamp_ms);
```

**Benefits:**
- Fewer message headers/footers
- Higher data density
- Better throughput efficiency

**Implementation:**

```rust
#[derive(Format)]
struct TelemetryBatch {
    i2c_addr: u8,
    i2c_bytes: usize,
    gpio_pin: u8,
    gpio_state: GpioState,
    sensor_value: i32,
    timestamp_ms: u64,
}

// Every 10ms, send batched telemetry
if loop_count % 10 == 0 {
    let batch = TelemetryBatch {
        i2c_addr: 0x68,
        i2c_bytes: 6,
        gpio_pin: 8,
        gpio_state: GpioState::Low,
        sensor_value: 2530,
        timestamp_ms,
    };
    info!("BATCH {}", batch);
}
```

**Expected throughput:**
- Batch rate: 100 Hz
- Batch size: ~50-60 bytes
- Throughput: ~5-6 KB/s

---

### Phase 6: Update Python Parser for defmt Output (30 min)

**Challenge:** defmt output format is different from our pipe-delimited format

**Option A: Keep both parsers**
- Detect format automatically
- Parse defmt output with regex
- Fallback to original pipe parser

**Option B: Unified parser**
- Convert defmt to structured format
- Use same display logic

**Implementation (Option A - Recommended):**

```python
class StreamParser:
    def parse_line(self, line: str):
        """Parse both defmt and pipe-delimited formats"""

        # Detect defmt format: "INFO I2C addr=0x68 ..."
        if line.startswith(("INFO", "DEBUG", "WARN", "ERROR")):
            self.parse_defmt_line(line)
        # Detect pipe format: "I2C|addr=0x68|..."
        elif "|" in line:
            self.parse_pipe_line(line)
        else:
            print(f"Unknown: {line}")

    def parse_defmt_line(self, line: str):
        """Parse defmt formatted output"""
        # Example: "INFO I2C addr=0x68 op=Read bytes=6 status=Success ts=1234"
        parts = line.split(maxsplit=2)  # ["INFO", "I2C", "addr=0x68 ..."]

        if len(parts) < 3:
            return

        level = parts[0]
        msg_type = parts[1]
        fields_str = parts[2]

        # Parse field=value pairs
        fields = {}
        for match in re.finditer(r'(\w+)=([^\s]+)', fields_str):
            fields[match.group(1)] = match.group(2)

        # Dispatch based on message type
        if msg_type == "I2C":
            self.handle_i2c(fields)
        elif msg_type == "GPIO":
            self.handle_gpio(fields)
        # ... etc
```

---

### Phase 7: Performance Testing & Benchmarking (20 min)

**Test Suite:**

1. **Baseline Test (100 Hz)**
   ```bash
   # Normal mode
   cargo run --release
   python3 stream_parser.py /dev/cu.usbmodem2101 --csv baseline.csv --stats
   ```
   - Run for 60 seconds
   - Measure: msg/s, KB/s, dropped frames

2. **High-Speed Test (1000 Hz)**
   ```bash
   # High-speed mode (set LOOP_DELAY_MS = 1)
   cargo run --release
   python3 stream_parser.py /dev/cu.usbmodem2101 --csv highspeed.csv --stats
   ```
   - Run for 60 seconds
   - Measure: msg/s, KB/s, CPU usage, dropped frames

3. **Stress Test (1000 Hz all messages)**
   ```bash
   # Stress mode (set STRESS_TEST = true)
   cargo run --release
   python3 stream_parser.py /dev/cu.usbmodem2101 --csv stress.csv --stats
   ```
   - Run for 60 seconds
   - Find breaking point
   - Measure max sustainable rate

**Benchmark Metrics:**

| Test | Loop Rate | Msg Rate | Throughput | CPU | Dropped | Status |
|------|-----------|----------|------------|-----|---------|--------|
| Baseline | 100 Hz | ~20 msg/s | ~1 KB/s | Low | 0 | ✅ |
| High-Speed | 1000 Hz | ~161 msg/s | ~6 KB/s | Med | 0 | ? |
| Stress | 1000 Hz | ~1000 msg/s | ~35 KB/s | High | ? | ? |
| Max | ? Hz | ? msg/s | ? KB/s | ? | <1% | Target |

**Success Criteria:**
- ✅ Sustain 500+ msg/s for 60 seconds
- ✅ Throughput > 15 KB/s
- ✅ Dropped frames < 1%
- ✅ CPU usage < 50% on host

---

### Phase 8: Optimization Strategies (If Needed)

**If we hit bottlenecks:**

**Firmware optimizations:**
1. **Reduce timestamp precision** - Use ms instead of us
2. **Compress enum values** - Use u8 instead of strings
3. **Skip redundant fields** - Only send changed values
4. **Use static buffers** - Avoid allocations

**USB CDC optimizations:**
5. **Increase baud rate** - Try 230400 or 460800 (may not help for USB CDC)
6. **Use DMA** - If esp-hal supports it for USB serial
7. **Batch writes** - Buffer multiple messages before flushing

**Parser optimizations:**
8. **Use Rust parser** - Rewrite in Rust for speed
9. **Multithreaded parsing** - Separate read/parse threads
10. **Binary mode** - Skip text conversion entirely

---

## Expected Performance Gains

### Message Size Reduction with defmt

**Before (pipe-delimited string):**
```
I2C|addr=0x68|op=Read|bytes=6|status=Success|ts=1234\n
```
= ~60 bytes

**After (defmt optimized):**
```
INFO I2C addr=0x68 op=Read bytes=6 status=Success ts=1234\n
```
= ~55 bytes (but defmt encodes more efficiently)

**Actual defmt binary (in memory before USB conversion):**
```
[tag:8][addr:8][op:8][bytes:16][status:8][ts:64] = ~13 bytes
```

**After USB text conversion:**
```
Same as above (~55 bytes) but more efficient processing
```

**Net improvement:** ~10-20% size reduction, ~50% processing efficiency

### Throughput Projection

| Scenario | Rate | Msg Size | Throughput | % of USB |
|----------|------|----------|------------|----------|
| Current | 20 msg/s | 60 B | 1.2 KB/s | 0.08% |
| High-Speed | 161 msg/s | 55 B | 8.6 KB/s | 0.57% |
| Stress | 1000 msg/s | 55 B | 53.7 KB/s | 3.5% |
| Batched | 100 msg/s | 100 B | 9.8 KB/s | 0.65% |
| **Target** | **500 msg/s** | **50 B** | **24.4 KB/s** | **1.6%** |

---

## Implementation Checklist

### Prerequisites
- [ ] Existing Lesson 08 working
- [ ] ESP32-C6 connected and responding
- [ ] Python parser tested and working

### Phase 1: Dependencies
- [ ] Add defmt to Cargo.toml
- [ ] Add defmt-espflash feature
- [ ] Verify compilation

### Phase 2: defmt Conversion
- [ ] Add Format derives to all structs
- [ ] Replace println! with defmt::info!
- [ ] Test basic defmt output
- [ ] Verify USB CDC still works

### Phase 3: Speed Increase
- [ ] Change LOOP_DELAY_MS to 1
- [ ] Adjust event frequencies
- [ ] Test 1000 Hz operation
- [ ] Monitor for stability issues

### Phase 4: Stress Test
- [ ] Add STRESS_TEST mode
- [ ] Test maximum throughput
- [ ] Find breaking point
- [ ] Document limits

### Phase 5: Batching
- [ ] Create TelemetryBatch struct
- [ ] Implement batched logging
- [ ] Measure efficiency gain
- [ ] Compare to individual messages

### Phase 6: Parser Update
- [ ] Add defmt format detection
- [ ] Implement defmt parser
- [ ] Test with real output
- [ ] Verify all message types work

### Phase 7: Benchmarking
- [ ] Run baseline test
- [ ] Run high-speed test
- [ ] Run stress test
- [ ] Document results

### Phase 8: Optimization
- [ ] Identify bottlenecks
- [ ] Apply optimizations
- [ ] Re-test performance
- [ ] Document final metrics

---

## Risk Assessment

### High Risk
- **USB CDC saturation** - May drop frames at very high rates
  - Mitigation: Start conservative, increase gradually
- **Parser can't keep up** - Python may be too slow
  - Mitigation: Monitor CPU usage, optimize parser

### Medium Risk
- **ESP32-C6 CPU overhead** - Logging may slow main loop
  - Mitigation: Measure actual timing, reduce if needed
- **defmt compatibility issues** - May not work with esp-println
  - Mitigation: Test early, fallback to println if needed

### Low Risk
- **Build issues** - defmt dependencies may conflict
  - Mitigation: Use known-good versions
- **Data corruption** - USB CDC may garble at high speeds
  - Mitigation: Add checksums, test thoroughly

---

## Timeline

**Total Estimated Time: 1.5-2 hours**

| Phase | Time | Description |
|-------|------|-------------|
| 1 | 15 min | Add defmt dependencies |
| 2 | 30 min | Convert to defmt logging |
| 3 | 20 min | Increase to 1000 Hz |
| 4 | 15 min | Add stress test mode |
| 5 | 20 min | Implement batching |
| 6 | 30 min | Update Python parser |
| 7 | 20 min | Performance testing |
| 8 | 20 min | Optimizations (if needed) |

**Fastest path (skip batching):** ~1 hour
**Full implementation:** ~2 hours

---

## Success Metrics

**Minimum Acceptable:**
- ✅ 100 msg/s sustained
- ✅ 5 KB/s throughput
- ✅ defmt logging works

**Target:**
- ✅ 500 msg/s sustained
- ✅ 25 KB/s throughput
- ✅ < 1% dropped frames

**Stretch:**
- 🎯 1000 msg/s sustained
- 🎯 50 KB/s throughput
- 🎯 0% dropped frames

---

## Next Steps After Implementation

1. **Document findings** - Add PERFORMANCE.md with benchmark results
2. **Create visualization** - Plot throughput vs time
3. **Compare to RTT** - When RTT works, benchmark against it
4. **Optimize further** - If we hit limits, apply Phase 8 optimizations
5. **Real sensor integration** - Replace simulated data with actual I2C/SPI

---

**Status:** Ready to implement
**Priority:** High (requested feature)
**Complexity:** Medium (well-defined scope)
