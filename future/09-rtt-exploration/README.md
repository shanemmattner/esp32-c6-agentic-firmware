# Lesson 09: Data-Driven Debugging with RTT

Building an ADS1015 ADC driver while exploring RTT's variable transmission budget for real-time firmware telemetry.

## Overview

This lesson teaches two critical embedded Rust skills:

1. **ADS1015 ADC Driver (RMT Protocol)** - Interfacing with 12-bit analog I2C sensor
2. **Data-Driven Debugging** - Logging 50-500+ variables @ 100 Hz via RTT (non-blocking)

RTT (Real-Time Transfer) is the ideal telemetry pipe because it's **non-invasive and fast** - firmware never blocks, and you get 1-10 MB/s throughput vs UART's 14 KB/s.

## Why This Approach?

**Traditional embedded debugging:**
- Minimal logging to avoid overhead
- Hypothesis-test each subsystem
- Risk missing complex interactions

**Data-driven debugging with RTT:**
- Log everything (variable bandwidth budgets, not minimal overhead)
- Correlations reveal root cause instantly
- Claude Code analyzes patterns from massive telemetry streams
- RTT non-blocking means timing is accurate

**Example:** ADS1015 reading is stuck at 0. With massive logging:
```
button_pressed=0 i2c_errors=5 adc_ready=0 adc_value=0
→ I2C is timing out, not ADC problem
→ Check I2C pull-ups
```

Without massive logging, you'd guess each subsystem independently.

## Hardware Setup

### Components
- ESP32-C6-WROOM DevKit
- ADS1015 12-bit ADC (I2C address 0x48)
- MPU9250 9-DOF IMU (I2C, for sensor fusion testing)
- WS2812 NeoPixel LED
- Push button
- USB cable (JTAG access via built-in)

### Wiring

| Component | GPIO | Notes |
|-----------|------|-------|
| ADS1015 SDA | GPIO2 | I2C data |
| ADS1015 SCL | GPIO11 | I2C clock |
| MPU9250 SDA | GPIO2 | I2C (shared bus) |
| MPU9250 SCL | GPIO11 | I2C (shared bus) |
| Button | GPIO9 | Active LOW |
| NeoPixel | GPIO8 | Data line |

## Quick Start

```bash
cd lessons/09-rtt-exploration
cargo build --release
probe-rs run --chip esp32c6 target/riscv32imac-unknown-none-elf/release/main
```

Watch RTT output - you'll see all variables streaming live.

## What You'll Learn

1. **I2C ADC Driver (ADS1015)** - Rust driver for 12-bit ADC with configuration
2. **RTT Variable Budgets** - How many variables can we log sustainably?
3. **RMT Protocol** - Exploring alternatives to I2C for sensor interfaces
4. **Data-Driven Debugging** - Logging strategies that reveal root causes
5. **Autonomous Patterns** - How Claude Code uses RTT for self-correction

## Checkpoints

- **C4:** ADS1015 driver basic (read single-shot, RTT-observable state)
- **C5:** ADS1015 continuous mode + multi-variable logging
- **C6:** Sweep testing - variable count vs sample rate vs frame drops
- **C7:** Profiling RTT limits - find max sustainable variables @ 100 Hz

## Variable Bandwidth Budget

Instead of "minimal logging," think in **data throughput**:

```
RTT throughput: 1-10 MB/s (JTAG clock dependent)

Example: 100 variables @ 100 Hz
- Per variable: ~20 bytes/message
- Total: 100 vars × 20 bytes × 100 Hz = 200 KB/s
- Headroom: 200 KB/s out of 1-10 MB/s = safe

100 variables @ 1000 Hz = 2 MB/s = still safe on 4+ MHz JTAG
```

We'll benchmark actual limits in checkpoint 7.

## Key Philosophy

**RTT is just the pipe.** The real topic is **data-driven debugging** - the idea that with sufficient telemetry bandwidth, root causes become obvious without hypothesis-testing.

This is perfect for autonomous Claude Code development because:
- Non-blocking (timing stays accurate)
- High throughput (can observe everything)
- Structured defmt logs (machine-parseable patterns)
- Claude excels at correlating massive datasets

## References

- [ADS1015 Datasheet](https://www.ti.com/lit/ds/symlink/ads1015.pdf)
- [esp-hal I2C](https://docs.esp-rs.org/esp-hal/latest/esp_hal/i2c/index.html)
- [defmt Docs](https://defmt.ferrous-systems.com/)
- [probe-rs Guide](https://probe.rs/)
- See `CLAUDE.md` for embedded debugging philosophy

---

**Build a practical ADC driver while exploring RTT's power for debugging** 🔍
