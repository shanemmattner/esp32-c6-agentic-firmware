# RTT Suite - Comprehensive Embedded Debugging with Real-Time Transfer

## Overview

The RTT Suite provides Claude Code with a complete toolkit for embedded systems debugging and development using Real-Time Transfer (RTT). It includes:

1. **RTT Mastery Guide** - Comprehensive documentation
2. **RTT Sweep Tool** - Performance characterization
3. **RTT Parser** - Log analysis utilities
4. **RTT Validator** - Automated firmware testing
5. **Interactive tutorials** - Learn RTT best practices

## Quick Start

### What You Can Do Now

```bash
# Learn RTT best practices
/rtt-tutorial "I'm new to RTT, teach me"

# Run performance characterization
/rtt-sweep --device esp32c6

# Validate your firmware
/rtt-validate "my_firmware.rs"

# Parse and analyze RTT logs
/rtt-analyze logs.txt

# Get tool reference
/rtt-tools
```

---

## Command Reference

### 1. `/rtt-tutorial [topic]`

**Purpose**: Interactive learning about RTT and embedded debugging

**Usage**:
```
/rtt-tutorial                           # Start from beginning
/rtt-tutorial "RTT vs UART"            # Specific comparison
/rtt-tutorial "defmt best practices"    # Topic search
/rtt-tutorial "ESP32-C6 setup"         # Device-specific
/rtt-tutorial "production debugging"    # Use case
```

**What it does**:
- Explains RTT concepts step-by-step
- Provides working code examples
- Compares alternatives (UART, GDB, SWO)
- Links to relevant sections of the guide
- Interactive Q&A format

**Example Output**:
```
📚 RTT Tutorial: Getting Started with Real-Time Transfer

RTT (Real-Time Transfer) is a debugging technology that allows your
embedded device to send and receive data in real-time without blocking
the main program.

Think of it like this:

  Traditional Logging:      RTT Logging:
  ┌──────────────┐         ┌──────────────┐
  │ Device       │         │ Device       │
  │ (blocked!)   │         │ (running!)   │
  │              │         │              │
  │  USB UART → PC          │ USB JTAG → PC
  │   14 KB/s              │  1-10 MB/s
  └──────────────┘         └──────────────┘

Key advantages:
✓ No dedicated GPIO pins needed (uses debug interface)
✓ 50-100x faster than UART
✓ Non-blocking (doesn't halt your code)
✓ Zero real-time impact

Would you like to learn about:
- How to set up RTT on ESP32-C6?
- RTT vs other debugging methods?
- Performance optimization?
- Production debugging with RTT?

Answer: "esp32-c6"
```

---

### 2. `/rtt-sweep [options]`

**Purpose**: Characterize RTT throughput for your device/probe configuration

**Usage**:
```
/rtt-sweep                              # Full sweep (20-30 min)
/rtt-sweep --fast                       # Quick test (5 min)
/rtt-sweep --device esp32c6             # Specific device
/rtt-sweep --probe-id 303a:1001:...    # Specific probe
/rtt-sweep --max-vars 15                # Max variable count
/rtt-sweep --max-rate 200               # Max sample rate
/rtt-sweep --output-format html         # HTML report
/rtt-sweep --compare previous_run       # Compare to baseline
```

**What it does**:
1. Compiles test firmware variants
2. Systematically tests different configs:
   - Variable counts: 1, 5, 10, 15, 20, 25
   - Sample rates: 10, 50, 100, 200, 500 Hz
3. Measures data loss % and throughput
4. Generates performance envelope
5. Produces recommendations for your setup

**Output**:
- Console table showing results
- CSV file (spreadsheet-ready)
- HTML dashboard (interactive visualization)
- JSON (machine-readable)
- Text report with recommendations

**Example Output**:
```
🔬 RTT Performance Sweep - ESP32-C6 (EspJtag)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Test Configuration:
  Device: ESP32-C6
  Probe: EspJtag (303a:1001:F0:F5:BD:01:88:2C)
  SWD Clock: 4 MHz
  Duration per test: 30 seconds

Results:
┌─────────┬──────────┬──────────────┬──────────────────┬──────┐
│ Vars    │ Rate(Hz) │ Loss(%)      │ Throughput(KB/s) │ Safe │
├─────────┼──────────┼──────────────┼──────────────────┼──────┤
│ 1       │ 10       │ 0.00         │ 0.1              │ ✓    │
│ 1       │ 100      │ 0.00         │ 1.0              │ ✓    │
│ 10      │ 100      │ 0.00         │ 10.0             │ ✓    │
│ 10      │ 200      │ 0.10         │ 20.0             │ ✓    │
│ 20      │ 100      │ 0.00         │ 20.0             │ ✓    │
│ 20      │ 200      │ 3.5          │ 40.0             │ ◐    │
│ 25      │ 100      │ 2.1          │ 25.0             │ ✓    │
│ 25      │ 200      │ 12.4         │ 50.0             │ ✗    │
└─────────┴──────────┴──────────────┴──────────────────┴──────┘

📊 PERFORMANCE ENVELOPE:

Safe Zone (0-2% loss):
  ✓ 20 variables @ 100 Hz → 20 KB/s
  ✓ 15 variables @ 200 Hz → 30 KB/s
  ✓ 10 variables @ 500 Hz → 50 KB/s

Caution Zone (2-10% loss):
  ◐ 25 variables @ 100 Hz → 25 KB/s
  ◐ 20 variables @ 200 Hz → 40 KB/s

RECOMMENDATIONS:
✓ For real-time logs: Use ≤20 variables @ 100 Hz
✓ For high-speed data: Use ≤10 variables @ 500 Hz
✗ Avoid: >25 variables @ any rate

📁 Full report: rtt-sweep-report.html
📊 Data export: rtt-sweep-results.csv
```

---

### 3. `/rtt-validate [firmware_file] [options]`

**Purpose**: Automatically test firmware on hardware using RTT

**Usage**:
```
/rtt-validate src/bin/main.rs           # Validate firmware
/rtt-validate . --test blinky           # Named test
/rtt-validate --help                    # Test framework help
/rtt-validate --list-tests              # Show available tests
```

**What it does**:
1. Compiles firmware
2. Flashes to device
3. Captures RTT output for N seconds
4. Analyzes against test expectations
5. Reports PASS/FAIL with evidence

**Test Framework**:
```rust
// In firmware, mark tests with defmt::info!("[TEST_*]")

defmt::info!("[TEST_PASS] LED blink test");
defmt::info!("[TEST_FAIL] Button debounce failed");
defmt::error!("[TEST_CRASH] Stack overflow detected");

// Validator parses these automatically
// Reports: 3 PASS, 1 FAIL, 1 CRASH
```

**Example**:
```
/rtt-validate src/bin/main.rs

📋 Firmware Validation Report
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Compiling: src/bin/main.rs ✓
Flashing to ESP32-C6 ✓
Capturing RTT output (10s)...

RESULTS:
✓ [TEST_PASS] Initialization
✓ [TEST_PASS] LED toggle (10 times)
✓ [TEST_PASS] Button debounce (25ms threshold)
✗ [TEST_FAIL] I2C communication timeout

SUMMARY: 3 PASS, 1 FAIL

Analysis:
- I2C test failing suggests pull-up resistors may be missing
- Try: Check SDA/SCL connections on MPU9250

Suggestion: Debug with /rtt-analyze to see I2C timing details
```

---

### 4. `/rtt-analyze [log_file] [options]`

**Purpose**: Parse and analyze RTT log data

**Usage**:
```
/rtt-analyze log.txt                    # Basic analysis
/rtt-analyze log.txt --format json      # Parse to JSON
/rtt-analyze log.txt --extract events   # Filter specific events
/rtt-analyze log.txt --timings          # Latency analysis
/rtt-analyze log.txt --compare ref.txt  # Compare to baseline
```

**What it does**:
1. Parses structured defmt logs
2. Extracts events and metrics
3. Detects patterns (anomalies, trends)
4. Produces analysis report

**Output Format**:
```json
{
  "duration_seconds": 10.5,
  "total_events": 1050,
  "events_per_second": 100,

  "event_types": {
    "info": 900,
    "warn": 100,
    "error": 50
  },

  "extracted_data": {
    "temperature_readings": [24.3, 24.5, 24.4, ...],
    "button_presses": [{"time": 0.5, "type": "press"}, ...],
    "errors": [{"time": 1.2, "msg": "I2C timeout"}]
  },

  "analysis": {
    "data_loss_percent": 0.0,
    "avg_latency_us": 150,
    "anomalies": ["Spike in temperature at T=5.2s"],
    "recommendations": [...]
  }
}
```

---

### 5. `/rtt-tools`

**Purpose**: Reference for available RTT utilities and tools

**Usage**:
```
/rtt-tools                              # List all tools
/rtt-tools --help                       # Detailed help
/rtt-tools --install                    # Install dependencies
/rtt-tools --status                     # Check system setup
```

**Output**:
```
🔧 RTT Suite Tools
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📚 Documentation:
  /rtt-tutorial              Interactive RTT learning
  /rtt-guide                 Full RTT Mastery guide

⚡ Practical Tools:
  /rtt-sweep                 Performance characterization
  /rtt-validate              Automated firmware testing
  /rtt-analyze               Log analysis and parsing

🔍 Utilities:
  rtt-capture.py             Raw RTT capture script
  rtt-parser.py              Log file parser
  rtt-formatter.py           Format RTT output

📊 Example Workflows:
  1. New to RTT? → /rtt-tutorial
  2. Optimize your device? → /rtt-sweep
  3. Validate firmware? → /rtt-validate
  4. Understand logs? → /rtt-analyze

🛠️ System Status:
  ✓ probe-rs installed
  ✓ defmt-rtt available
  ✓ Python 3.8+ available
  ✓ RTT tools ready to use
```

---

## Advanced Features

### Performance Envelope Visualization

```
/rtt-sweep --output-format html --visualize
→ Opens interactive dashboard showing:
  - Heat map (vars vs rate → loss %)
  - Throughput curves
  - Safe/caution/danger zones
  - Historical comparisons
```

### Batch Characterization

```
/rtt-sweep --batch device1,device2,device3
→ Test multiple devices/probes in sequence
→ Compare results
→ Generate comparison report
```

### Continuous Monitoring

```
/rtt-monitor --duration 1h --alert-on error
→ Run firmware continuously
→ Alert if error count exceeds threshold
→ Save statistics for analysis
```

---

## Integration with Claude Code Workflow

### Example: Autonomous Firmware Development

```
1. User: "Write a button debounce handler"

2. Claude generates code + RTT test markers:
   defmt::info!("[TEST_DEBOUNCE] state={}", state);

3. Claude runs: /rtt-validate src/bin/main.rs
   → Gets back: ✓ Debounce test PASS

4. Claude summarizes: "Button debouncer complete,
                       validated on hardware,
                       debounce threshold 25ms"

5. User: "Add I2C sensor reading"

6. Claude generates + tests:
   /rtt-validate src/bin/main.rs
   → Gets: ✗ I2C test FAIL

7. Claude analyzes with: /rtt-analyze log.txt
   → Detects: "Timeout after 100ms, no ACK"

8. Claude adjusts pull-ups and retry logic,
   validates again → ✓ PASS

Loop continues until all features working...
```

---

## Tips & Tricks

### 1. Quick Performance Check

```bash
/rtt-sweep --fast --device esp32c6
# 5 minute sweep with reduced configs
# Good for "is my setup healthy?"
```

### 2. Compare Probe Performance

```bash
# Test with EspJtag
/rtt-sweep --probe "303a:1001:..." --output probe1.json

# Test with different probe
/rtt-sweep --probe "1366:0101:..." --output probe2.json

# Compare
/rtt-sweep --compare probe1.json probe2.json
```

### 3. Validate After Changes

```bash
# Before optimization
/rtt-sweep --output before.json

# Make changes (larger RTT buffer, faster SWD clock)

# After optimization
/rtt-sweep --output after.json --compare before.json
```

### 4. Extract Specific Data

```
/rtt-analyze log.txt --extract "temperature" --format csv
→ Extracts all temperature readings → CSV for plotting
```

---

## Troubleshooting

### "RTT output not appearing"

```
/rtt-tools --status
→ Checks: probe connection, defmt-rtt in dependencies, etc.
```

### "Data loss is high"

```
/rtt-sweep --diagnose --device esp32c6
→ Checks buffer size, SWD clock, USB connection health
→ Recommends fixes
```

### "Need to debug RTT performance"

```
/rtt-analyze log.txt --timings --verbose
→ Shows latency patterns and jitter
→ Identifies bottlenecks
```

---

## Further Reading

- Full guide: `.claude/rtt-guide.md`
- Sweep spec: `.claude/rtt-sweep-spec.md`
- Embedded Rust book: https://rust-embedded.github.io/book/
- defmt docs: https://defmt.ferrous-systems.com

---

**Last Updated**: November 2025
**Status**: Complete reference suite
**Next**: Implement `/rtt-sweep` tool as first concrete deliverable
