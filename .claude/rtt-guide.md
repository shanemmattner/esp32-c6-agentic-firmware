# RTT Mastery: Real-Time Transfer for Embedded Debugging with LLMs

## Overview

Real-Time Transfer (RTT) is a high-performance debugging technology that enables real-time communication between embedded devices and development machines without disrupting the target's real-time behavior. When combined with structured logging (defmt) and an AI development assistant (Claude Code), RTT becomes a powerful tool for autonomous embedded development.

This guide covers:
- **Background**: What RTT is and why it matters
- **Best Practices**: General, ESP32-specific, and Rust-specific guidance
- **Creative Use Cases**: Production debugging, performance analysis, multi-channel monitoring
- **Integration with Claude Code**: How to leverage RTT for AI-driven development
- **Embedded Debugging Ecosystem**: Tools and techniques beyond RTT

---

## Part 1: Background and Fundamentals

### What is RTT?

Real-Time Transfer (RTT) is a protocol developed by SEGGER for bidirectional, non-blocking communication between a microcontroller and a host PC via a debug probe. Unlike traditional approaches:

- **UART/Serial**: Requires dedicated GPIO pins, blocks program execution if buffer is full, ~14 KB/sec max throughput
- **Semihosting**: Uses breakpoints to halt execution, extremely slow (~100 bytes/sec), breaks real-time guarantees
- **RTT**: Uses RAM ring buffers, non-blocking, no dedicated pins needed, 1-10 MB/sec throughput, zero real-time impact

### Why RTT is Special

```
Traditional Debugging:                RTT Debugging:
┌────────────────────┐               ┌────────────────────┐
│ Device Memory      │               │ Device Memory      │
│ ┌────────────────┐ │               │ ┌────────────────┐ │
│ │ Program Code   │ │               │ │ Program Code   │ │
│ └────────────────┘ │               │ └────────────────┘ │
│ ┌────────────────┐ │               │ ┌────────────────┐ │
│ │ UART TX buffer │ │──(GPIO pin)──>│ │ RTT Ring Buffer│ │
│ └────────────────┘ │                │ └────────────────┘ │
│                    │                │                    │
│   Blocking writes! │                │ Non-blocking reads!│
│   14 KB/sec max    │                │ 1-10 MB/sec        │
└────────────────────┘                └────────────────────┘
         │                                      │
         └──> USB Serial                        └──> USB JTAG/Debug Probe
              (takes GPIO pin)                       (no extra pins needed)
```

### Core Benefits for AI-Assisted Development

1. **No Real-Time Disruption**: RTT polling doesn't halt the MCU, so timing-sensitive code works normally
2. **High Throughput**: Stream sensor data, performance metrics, or debug events at MB/sec speeds
3. **Non-Blocking**: Program continues even if debugger isn't connected
4. **Bi-directional**: Send commands to device, receive responses in real-time
5. **Zero Resources**: Uses only RAM, no dedicated hardware resources

---

## Part 2: Best Practices

### General Embedded Systems Best Practices

#### 1. Buffer Sizing

The RTT buffer is a circular/ring buffer in target RAM. Size it appropriately:

```
Minimum size = (bytes written per millisecond) + overhead
Recommended = 2-4 KB for typical applications
Large data streaming = 8-16 KB for high-frequency logging
```

**Rule of thumb**: If you lose data, double the buffer size.

```rust
// In Cargo.toml or via environment variable
// DEFMT_RTT_BUFFER_SIZE=2048  (must be power of 2)
```

#### 2. Multiple Channels

RTT supports multiple independent channels (up to 16 each direction):

```
Channel 0: Log messages (printable, human readable)
Channel 1: Raw sensor data (binary, high frequency)
Channel 2: Performance metrics (timestamps, counters)
Channel 3: Commands from host (bi-directional control)
```

Use different channels for different purposes to avoid congestion.

#### 3. Initialization

RTT must be properly initialized before use:

```rust
// Automatic (handled by defmt-rtt)
use defmt_rtt as _;

// Or explicit (rtt-target)
use rtt_target::{rtt_init_print, rtt_init};
rtt_init_print!();
```

#### 4. Non-Blocking Mode

RTT defaults to non-blocking:
- If buffer is full, new writes are dropped (configurable)
- Program continues executing at full speed
- Never stalls waiting for debugger

This is critical for real-time systems.

#### 5. Probe Connection Optimization

RTT performance depends on several factors:

| Factor | Impact | Optimization |
|--------|--------|--------------|
| Debug probe hardware | Critical | Use EspJtag or J-Link for best performance |
| SWD/JTAG clock speed | High | Increase from 1 MHz to 4+ MHz if stable |
| USB connection | Medium | Use good quality USB cable, avoid hubs |
| Buffer size | High | Size appropriately (not too small, not huge) |
| Host-side polling | Medium | probe-rs optimizes this automatically |

---

### ESP32-Specific Best Practices

#### 1. Built-in USB-JTAG

The ESP32-C6 has integrated USB-JTAG on GPIO19/20 (internal):
- No external pins needed for debugging
- Very convenient for development
- Supports all RTT protocols

```toml
# Cargo.toml for ESP32-C6
[dependencies]
esp-hal = { version = "1.0.0", features = ["esp32c6", "unstable"] }
defmt = "0.3"
defmt-rtt = "0.4"
```

#### 2. probe-rs Configuration

Optimized probe-rs setup for ESP32:

```toml
# .cargo/config.toml
[target.riscv32imac-unknown-none-elf]
runner = "probe-rs run --chip esp32c6"
rustflags = [
    "-C", "force-frame-pointers",
    "-C", "link-arg=-Tdefmt.x",  # RTT linker script
]
```

#### 3. Boot Mode Considerations

ESP32-C6 can boot in different modes. For RTT debugging:

```rust
// Use direct-boot for faster RTT startup
// In esp-hal Config:
let config = esp_hal::Config::default()
    .with_uart_baudrate(115_200);
```

#### 4. RTT + UART Hybrid Approach

Use both for maximum visibility:

```rust
// High-frequency data via RTT (structured)
defmt::info!("Accel: x={=i16}, y={=i16}, z={=i16}", x, y, z);

// Status via UART (human-readable, informative)
println!("Device healthy: {}", is_healthy);
```

RTT for volume and structure, UART for occasional text logs.

#### 5. Performance Characteristics on ESP32-C6

Expected performance with probe-rs + EspJtag:

```
RTT Throughput @ different SWD clock speeds:
• 1 MHz SWD   → ~250-500 KB/sec
• 4 MHz SWD   → ~1-2 MB/sec
• 10 MHz SWD  → ~3-5 MB/sec (varies by probe)

Practical limits for structured logging:
• 1-5 variables @ 100 Hz   → 0.1-0.5 MB/sec  ✓ Safe
• 10-20 variables @ 100 Hz → 1-2 MB/sec      ✓ Good
• 50+ variables @ 100 Hz   → 5+ MB/sec       ✗ Saturation
```

---

### Rust/Embedded-Rust Specific Best Practices

#### 1. defmt Integration (Recommended)

defmt provides zero-overhead structured logging:

```rust
use defmt::{info, warn, error, debug};
use defmt_rtt as _;  // RTT transport

// Automatic serialization via defmt::Format derive
#[derive(defmt::Format)]
struct SensorReading {
    temperature: i16,
    humidity: u16,
    timestamp_us: u32,
}

// Structured, machine-parseable logging
let reading = SensorReading { temperature: 25, humidity: 45, timestamp_us: 1000 };
info!("Sensor: {}", reading);  // Automatic format strings on host
```

Benefits:
- Format strings stored on **host**, not device (massive Flash savings)
- Structured data automatically serialized/deserialized
- No runtime formatting overhead
- defmt handles RTT transport automatically

#### 2. rtt-target for Lower-Level Control

If you need more control than defmt:

```rust
use rtt_target::{rprintln, rtt_init_print};

fn main() {
    rtt_init_print!();
    rprintln!("Hello from RTT!");
}
```

Good for:
- Custom protocols
- Binary data streaming
- Multiple channel management

#### 3. Panic Handler Integration

Panic messages over RTT:

```rust
use defmt::error;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    error!("PANIC: {}", defmt::Debug2Format(info));
    loop {}
}
```

Ensures panic information reaches host even without UART.

#### 4. Timestamp Synchronization

RTT timestamps on MCU should sync with host time:

```rust
// In Cargo.toml or probe-rs config
defmt::timestamp!("{=u64:us}", {
    // Return microseconds since boot or RTC
    esp_hal::time::get_time().as_micros() as u64
});
```

This allows correlation between:
- MCU timestamps (from RTT logs)
- Host timestamps (from probe-rs)
- External events (sensor interrupts, GPIO changes)

#### 5. Cargo.toml Dependencies

Minimal, lean setup:

```toml
[dependencies]
defmt = "0.3"
defmt-rtt = "0.4"

[profile.dev]
opt-level = "s"
debug = 2  # Full debug symbols

[profile.release]
debug = 2  # Keep symbols even in release
opt-level = "s"
lto = "fat"
codegen-units = 1
```

Key point: Keep debug symbols even in release builds for meaningful RTT output.

---

## Part 3: Creative Use Cases

### 1. Autonomous Firmware Validation

**Use Case**: Claude Code validates its own generated firmware without human testing.

```rust
// Device sends test results over RTT
defmt::info!("TEST[blinky_led]: PASS - LED toggled {} times", count);
defmt::info!("TEST[i2c_read]: PASS - Bytes={=u16}", bytes_read);
defmt::error!("TEST[spi_write]: FAIL - Timeout on TX");

// Host-side: Claude parses RTT output
// If any TEST[*]: FAIL → ask Claude to fix code
// If all TEST[*]: PASS → validation complete
```

This enables:
- Automated firmware testing without manual hardware access
- Claude self-correcting failures
- Continuous integration for embedded systems

### 2. Performance Characterization Under Load

**Use Case**: Determine maximum safe RTT throughput for your application.

```rust
// Send N variables at different rates, measure data loss
loop {
    for i in 0..num_variables {
        defmt::info!("var{}: {=u32}", i, rand());  // Sequenced IDs
    }
    delay(1000 / sample_rate_hz);
}
```

Host analysis:
- Count missing sequence numbers → data loss
- Correlate with variable count and sample rate
- Build a performance envelope: "Safe to log 15 variables @ 200 Hz"

### 3. Production Remote Diagnostics

**Use Case**: Stream device state to cloud for analytics without custom firmware per customer.

```rust
// After error occurs, RTT streams diagnostic snapshot
defmt::warn!("ERROR[I2C]: Timeout on addr=0x{=u8:x}, retry={}",addr, retry_count);
defmt::info!("DIAGNOSTIC[voltage]: {=u16} mV", adc_reading);
defmt::info!("DIAGNOSTIC[uptime]: {=u32} seconds", uptime);
defmt::info!("DIAGNOSTIC[heap_free]: {=u32} bytes", heap_free);
```

Benefits:
- Rich diagnostic context automatically captured
- No predefined error codes needed
- Easy to extend with new diagnostics

### 4. Multi-Channel Command & Control

**Use Case**: Send commands to device and receive structured responses.

```rust
// Channel 0: Log output (device → host)
defmt::info!("Ready for commands");

// Channel 1: Command input (host → device)
// Host sends: "SET_SAMPLE_RATE 500"
// Device parses and confirms: defmt::info!("SAMPLE_RATE: 500 Hz")
```

Example implementation:
```rust
// Non-blocking command polling
if let Ok(cmd) = command_channel.read() {
    match cmd {
        0x01 => defmt::info!("Cmd: GET_STATUS -> OK"),
        0x02 => defmt::info!("Cmd: RESET -> DONE"),
        _ => defmt::warn!("Unknown command: 0x{=u8:x}", cmd),
    }
}
```

### 5. Continuous State Machine Monitoring

**Use Case**: Verify device state transitions for complex systems.

```rust
#[derive(defmt::Format)]
enum SystemState { Idle, Sampling, Processing, Error }

// Log every state transition
if next_state != current_state {
    defmt::info!("STATE_CHANGE: {} → {}", current_state, next_state);
    current_state = next_state;
}
```

Host-side analysis:
- Extract state sequences from RTT logs
- Verify expected state machines
- Detect unexpected transitions → flag as bugs

---

## Part 4: Integration with Claude Code (LLM Automation)

### The Vision: RTT-Aware Claude Code

Claude Code can leverage RTT logs to:
1. **Validate** generated code on actual hardware
2. **Detect** failures in real-time
3. **Iterate** on code based on observed behavior
4. **Characterize** device capabilities (performance envelopes)

### Workflow Example

```
User: "Make the LED blink at 1 Hz with debounced button control"

Claude Code:
  1. Generates firmware with structured RTT logging
  2. Builds and flashes to ESP32-C6
  3. Captures RTT output for 5 seconds:
     "BUTTON[0]: PRESS - stable_time=25ms"
     "BUTTON[0]: RELEASE - stable_time=30ms"
     "LED: toggle (period=1002ms)"  ← Close to 1000ms target

  4. Analyzes: "Debounce working, LED period within 0.2% - PASS"
  5. Returns: "✓ Firmware validated on hardware"
```

### Implementing RTT-Aware Validation

Key components:
1. **Device-side**: Add validation logging to firmware
2. **Host-side**: Parse RTT, extract metrics, report results
3. **Claude integration**: Make validation data available to AI analysis

Example validation framework:
```rust
// Device sends structured validation data
macro_rules! test_result {
    ($name:expr, $pass:expr) => {
        if $pass {
            defmt::info!("[TEST_PASS] {}", $name);
        } else {
            defmt::info!("[TEST_FAIL] {}", $name);
        }
    };
}

// Host parses "[TEST_PASS]" and "[TEST_FAIL]" markers
// Claude automatically detects failures and adjusts code
```

---

## Part 5: Embedded Debugging Ecosystem

Beyond RTT, understand the full embedded debugging toolkit available to AI-driven development.

### Tool Matrix

| Tool | Purpose | Best For | Overhead |
|------|---------|----------|----------|
| **RTT** | Structured logging | Continuous data, state tracking | Minimal (non-blocking) |
| **GDB** | Interactive debugging | Breakpoints, variable inspection | High (halts execution) |
| **OpenOCD** | Debug server/probe | Multi-probe support, standardization | Medium |
| **probe-rs** | Modern debugger | Rust, ease of use, RTT | Low-Medium |
| **LLDB** | Alternative debugger | Speed, multi-arch support | High (halts execution) |
| **TRACE32** | Real-time tracing | Multi-core, ETM trace analysis | Medium |
| **SystemView** | RTOS analysis | FreeRTOS, task timing, interrupts | Medium |
| **Renode** | Simulation/emulation | CI/CD, virtual testing | Medium (offline) |

### When to Use Each Tool

**RTT** → Continuous monitoring, logging, performance validation
- ✓ Check sensor readings continuously
- ✓ Validate state transitions
- ✓ Measure timing precision
- ✓ Stream performance metrics

**GDB/OpenOCD** → Crash debugging, register inspection
- ✓ Set breakpoints on crashes
- ✓ Inspect variables at crash point
- ✓ Examine peripheral registers
- ✗ Bad for real-time validation (halts MCU)

**TRACE32/SystemView** → Task scheduling, interrupt analysis
- ✓ See exactly when each task runs
- ✓ Measure interrupt latency
- ✓ Analyze multi-core behavior
- ✗ Complex setup, expensive commercial tools

**Renode** → Virtual testing, CI integration
- ✓ Test without hardware
- ✓ Reproduce exact timing
- ✓ Run in GitHub Actions
- ✗ Less realistic than hardware

### Recommended Stack for AI-Driven Embedded Dev

**Tier 1 (Essential)**:
- `probe-rs` - Flashing and command-line tools
- `defmt` + `defmt-rtt` - Structured logging
- `GDB` + `OpenOCD` - Interactive debugging for crashes

**Tier 2 (High Value)**:
- Python scripts to parse RTT, extract metrics, validate results
- Custom test frameworks that Claude can leverage
- Automated performance characterization tools

**Tier 3 (Advanced)**:
- SystemView for RTOS analysis
- Renode for simulation-based testing
- Custom instrumentation in device code

---

## Part 6: RTT Performance Sweep Tool

### Purpose

Characterize the maximum safe RTT throughput for your device and use case.

### What It Does

```
Input:
  - Device: ESP32-C6
  - Variable count: 1, 5, 10, 15, 20, 25
  - Sample rates: 10 Hz, 50 Hz, 100 Hz, 200 Hz, 500 Hz
  - Duration: 30 seconds per test

Output:
  - Data loss percentage for each combination
  - Throughput measurements (KB/sec)
  - Safe operating envelope chart
  - Recommendations for your use case
```

### Expected Results

```
RTT Throughput Envelope for ESP32-C6 + probe-rs:

Sample Rate (Hz)
              10    50   100   200   500
Vars:  1      ✓     ✓     ✓     ✓    ✓    (0% loss)
       5      ✓     ✓     ✓     ✓    ✓    (0% loss)
      10      ✓     ✓     ✓     ✓    ◐    (1-5% loss)
      15      ✓     ✓     ✓     ◐    ✗    (5-20% loss)
      20      ✓     ✓     ◐     ✗    ✗    (20%+ loss)

Legend: ✓ = Safe (0-2% loss), ◐ = Caution (2-10%), ✗ = Danger (>10%)
```

### Implementation Strategy

See next section: "Design RTT Sweep Tool Specification"

---

## Part 7: Slash Command Design

### Vision

A comprehensive RTT toolkit accessible from Claude Code:

```
/rtt-sweep            → Run performance characterization
/rtt-tutorial         → Interactive RTT learning
/rtt-validate [code]  → Test firmware on hardware
/rtt-analyze [logs]   → Parse and analyze RTT output
/rtt-tools            → List available RTT utilities
```

### Implementation Structure

```
.claude/commands/
├── rtt-suite.md              # Main slash command (delegates)
├── rtt-sweep.py              # Performance sweep tool
├── rtt-parser.py             # Log parsing utilities
├── rtt-validator.py          # Test framework
└── rtt-guide.md              # This comprehensive guide
```

---

## Appendix: Quick Reference

### Common RTT Patterns

**Log a struct**:
```rust
#[derive(defmt::Format)]
struct Data { x: i16, y: i16, z: i16 }

let d = Data { x: 10, y: 20, z: 30 };
defmt::info!("Reading: {}", d);  // Automatically formatted
```

**Variable throughput test**:
```rust
for i in 0..num_variables {
    defmt::info!("data[{}]={}", i, values[i]);
}
```

**Synchronize timestamps**:
```rust
let us = esp_hal::time::get_time().as_micros() as u64;
defmt::info!("@{=u64:us}: event occurred", us);
```

**Panic capture**:
```rust
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", defmt::Debug2Format(info));
    loop {}
}
```

### Troubleshooting

| Problem | Cause | Fix |
|---------|-------|-----|
| RTT shows no output | Buffer too small | Increase DEFMT_RTT_BUFFER_SIZE |
| Intermittent data loss | Buffer overflow | Increase size (2x) |
| Probe connection fails | Wrong probe selected | Use `probe-rs list` to verify |
| Very slow RTT | SWD clock too low | Increase SWD speed in probe config |
| Program hangs with RTT | Blocking write mode | Ensure non-blocking mode (default) |

---

## References

- [SEGGER RTT Documentation](https://kb.segger.com/RTT)
- [defmt Book](https://defmt.ferrous-systems.com)
- [probe-rs Documentation](https://probe.rs/docs/)
- [Embedded Rust Book](https://rust-embedded.github.io/book/)
- [rtt-target Crate](https://docs.rs/rtt-target/)

---

**Last Updated**: November 2025
**Target Platforms**: ESP32-C6, STM32 (general guidance)
**Intended Audience**: Embedded developers using Claude Code for AI-assisted firmware development
