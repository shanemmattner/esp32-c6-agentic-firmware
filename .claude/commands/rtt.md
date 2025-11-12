# RTT - Real-Time Transfer Debugging Suite

Comprehensive tools for embedded debugging, firmware validation, and performance characterization using Real-Time Transfer (RTT).

## Quick Start

```bash
/rtt tutorial                    # Learn RTT best practices
/rtt sweep --device esp32c6     # Performance characterization
/rtt validate src/bin/main.rs   # Automated firmware testing
/rtt analyze logs.txt           # Log analysis and parsing
/rtt tools                       # Reference and utilities
/rtt guide                       # Open comprehensive RTT guide
```

## Commands

### `/rtt tutorial [topic]`

Interactive learning about RTT and embedded debugging.

**Usage**:
```
/rtt tutorial                           # Start from beginning
/rtt tutorial "RTT vs UART"            # Specific comparison
/rtt tutorial "defmt best practices"    # Topic search
/rtt tutorial "ESP32-C6 setup"         # Device-specific
/rtt tutorial "production debugging"    # Use case
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
embedded device to send and receive data in real-time without blocking.

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

### `/rtt sweep [options]`

Characterize RTT throughput for your device/probe configuration.

**Usage**:
```
/rtt sweep                              # Full sweep (20-30 min)
/rtt sweep --fast                       # Quick test (5 min)
/rtt sweep --device esp32c6             # Specific device
/rtt sweep --probe-id 303a:1001:...    # Specific probe
/rtt sweep --max-vars 15                # Max variable count
/rtt sweep --max-rate 200               # Max sample rate
/rtt sweep --output-format html         # HTML report
/rtt sweep --compare previous_run       # Compare to baseline
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

PERFORMANCE ENVELOPE:
  Safe Zone (0-2% loss):
    ✓ 20 variables @ 100 Hz
    ✓ 15 variables @ 200 Hz
    ✓ 10 variables @ 500 Hz

  Caution Zone (2-10% loss):
    ◐ 25 variables @ 100 Hz
    ◐ 20 variables @ 200 Hz

RECOMMENDATIONS:
✓ For real-time logs: Use ≤20 variables @ 100 Hz
✓ For high-speed data: Use ≤10 variables @ 500 Hz
✗ Avoid: >25 variables @ any rate
```

---

### `/rtt validate [firmware_file] [options]`

Automatically test firmware on hardware using RTT.

**Usage**:
```
/rtt validate src/bin/main.rs           # Validate firmware
/rtt validate . --test blinky           # Named test
/rtt validate --help                    # Test framework help
/rtt validate --list-tests              # Show available tests
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

Suggestion: Debug with /rtt analyze to see I2C timing details
```

---

### `/rtt analyze [log_file] [options]`

Parse and analyze RTT log data.

**Usage**:
```
/rtt analyze log.txt                    # Basic analysis
/rtt analyze log.txt --format json      # Parse to JSON
/rtt analyze log.txt --extract events   # Filter specific events
/rtt analyze log.txt --timings          # Latency analysis
/rtt analyze log.txt --compare ref.txt  # Compare to baseline
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

### `/rtt tools`

Reference for available RTT utilities and system status.

**Usage**:
```
/rtt tools                              # List all tools
/rtt tools --help                       # Detailed help
/rtt tools --install                    # Install dependencies
/rtt tools --status                     # Check system setup
```

**Output**:
```
🔧 RTT Suite Tools
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📚 Documentation:
  /rtt tutorial              Interactive RTT learning
  /rtt guide                 Full RTT Mastery guide

⚡ Practical Tools:
  /rtt sweep                 Performance characterization
  /rtt validate              Automated firmware testing
  /rtt analyze               Log analysis and parsing

🛠️ System Status:
  ✓ probe-rs installed
  ✓ defmt-rtt available
  ✓ Python 3.8+ available
  ✓ RTT tools ready to use
```

---

### `/rtt guide`

Open the comprehensive RTT Mastery reference guide.

Shows the complete `.claude/rtt-guide.md` with:
- Fundamentals and background
- Best practices (general, ESP32, Rust-specific)
- Creative use cases
- Tool ecosystem comparison
- Quick reference and troubleshooting

---

## Example Workflows

### Validating New Firmware

```
User: "Write a button debounce handler"

Claude:
  1. Generates code with RTT test markers
  2. Runs: /rtt validate src/bin/main.rs
  3. Gets: ✓ Button debounce test PASS
  4. Reports: "Button handler complete, validated on hardware"
```

### Optimizing RTT Performance

```
/rtt sweep --device esp32c6
→ Shows: Safe to log ≤20 variables @ 100 Hz

Later, after SWD clock optimization:
/rtt sweep --device esp32c6 --compare previous_run
→ Shows: Now safe for ≤25 variables @ 100 Hz (+25% improvement)
```

### Understanding Device Behavior

```
/rtt analyze firmware_run.log --extract temperature
→ Extracts all temperature values into CSV
→ Can then plot trends, detect anomalies
```

---

## Tips & Tricks

### Quick Performance Check

```bash
/rtt sweep --fast --device esp32c6
# 5 minute sweep with reduced configs
# Good for "is my setup healthy?"
```

### Compare Probe Performance

```bash
# Test with EspJtag
/rtt sweep --probe "303a:1001:..." --output probe1.json

# Test with different probe
/rtt sweep --probe "1366:0101:..." --output probe2.json

# Compare
/rtt sweep --compare probe1.json probe2.json
```

### Extract Specific Data

```bash
/rtt analyze log.txt --extract "temperature" --format csv
→ Extracts all temperature readings → CSV for plotting
```

---

## Troubleshooting

### "RTT output not appearing"

```
/rtt tools --status
→ Checks: probe connection, defmt-rtt in dependencies, etc.
```

### "Data loss is high"

```
/rtt sweep --diagnose --device esp32c6
→ Checks buffer size, SWD clock, USB connection health
→ Recommends fixes
```

### "Need to debug RTT performance"

```
/rtt analyze log.txt --timings --verbose
→ Shows latency patterns and jitter
→ Identifies bottlenecks
```

---

## Further Reading

- Full guide: See `/rtt guide`
- Embedded Rust book: https://rust-embedded.github.io/book/
- defmt docs: https://defmt.ferrous-systems.com
- probe-rs Docs: https://probe.rs/docs/

---

## Quick Reference

### RTT Performance Envelope (ESP32-C6 + EspJtag)

| Variables | 100 Hz | 200 Hz | 500 Hz |
|-----------|--------|--------|--------|
| 10        | ✓ Safe | ✓ Safe | ◐ Caution |
| 15        | ✓ Safe | ◐ Caution | ✗ Danger |
| 20        | ✓ Safe | ✗ Danger | ✗ Danger |
| 25        | ◐ Caution | ✗ Danger | ✗ Danger |

### Tool Selection Decision Tree

```
Need to stream data continuously?
  → YES: Use /rtt validate and /rtt analyze

Need interactive debugging (breakpoints)?
  → YES: Use GDB (outside RTT suite)

Need real-time task analysis?
  → YES: Use SystemView or TRACE32

Need to test without hardware?
  → YES: Use Renode
```

---

**Documentation**: See `.claude/rtt-guide.md` for comprehensive reference.
