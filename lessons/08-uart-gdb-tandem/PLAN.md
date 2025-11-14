# Lesson 08: USB CDC High-Speed Data Streaming

**Goal:** Achieve high-speed structured logging and data streaming using USB CDC (virtual serial port) instead of RTT, with GDB debugging integration.

## Overview

Replace RTT-based logging with USB CDC streaming to achieve:
- ✅ High-speed data transmission (up to 12 Mbps)
- ✅ Structured logging with machine-parseable format
- ✅ Real-time data visualization
- ✅ Works with existing ESP32-C6 USB connection
- ✅ GDB-compatible debugging

## Technical Approach

### USB CDC vs RTT Comparison

| Feature | RTT (Lesson 08 Future) | USB CDC (This Lesson) |
|---------|------------------------|----------------------|
| **Speed** | 1-10 MB/s (theoretical) | 12 Mbps (1.5 MB/s USB Full Speed) |
| **Blocking** | Non-blocking | Blocking (manageable) |
| **Setup** | Requires JTAG + probe-rs | Uses built-in USB |
| **Status** | ❌ Not working on macOS | ✅ Works now |
| **Debugging** | Via JTAG | Via GDB + JTAG |

### Architecture

```
┌─────────────────────────────────────────────────────────┐
│  ESP32-C6 Firmware                                      │
│  ┌────────────────────────────────────────────────┐    │
│  │ Sensors / Peripherals                          │    │
│  │ (I2C, GPIO, ADC, IMU, etc.)                    │    │
│  └────────────────┬───────────────────────────────┘    │
│                   │                                      │
│                   ▼                                      │
│  ┌────────────────────────────────────────────────┐    │
│  │ Structured Data Types                          │    │
│  │ - I2cTransaction, GpioEvent, ImuReading        │    │
│  │ - Custom #[derive(Debug)] formatting           │    │
│  └────────────────┬───────────────────────────────┘    │
│                   │                                      │
│                   ▼                                      │
│  ┌────────────────────────────────────────────────┐    │
│  │ USB CDC Serial Output                          │    │
│  │ - esp_println! macro (non-blocking wrapper)    │    │
│  │ - Format: "TYPE|field1=val|field2=val|..."     │    │
│  └────────────────┬───────────────────────────────┘    │
└───────────────────┼──────────────────────────────────────┘
                    │ USB Cable
                    │
┌───────────────────▼──────────────────────────────────────┐
│  Host (macOS/Linux)                                      │
│  ┌────────────────────────────────────────────────┐    │
│  │ Python Parser (stream_parser.py)               │    │
│  │ - Read from /dev/cu.usbmodem*                  │    │
│  │ - Parse structured format                       │    │
│  │ - Real-time visualization                       │    │
│  │ - Data logging to file                          │    │
│  └────────────────┬───────────────────────────────┘    │
│                   │                                      │
│                   ▼                                      │
│  ┌────────────────────────────────────────────────┐    │
│  │ Analysis Tools                                  │    │
│  │ - matplotlib plots                              │    │
│  │ - CSV export                                    │    │
│  │ - Real-time dashboard                           │    │
│  └────────────────────────────────────────────────┘    │
└──────────────────────────────────────────────────────────┘
```

## Implementation Plan

### Phase 1: Basic USB CDC Streaming (Core)

**Files to create:**
```
lessons/08-usb-cdc-streaming/
├── Cargo.toml                  # Dependencies: esp-hal, esp-println
├── src/
│   ├── lib.rs                  # Structured data types
│   └── bin/
│       └── main.rs             # USB CDC streaming firmware
├── stream_parser.py            # Python parser for host
├── README.md                   # Documentation
└── TEST.md                     # Test specification
```

**Structured Data Types (src/lib.rs):**
```rust
#![no_std]

use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct I2cTransaction {
    pub addr: u8,
    pub operation: I2cOperation,
    pub bytes_transferred: usize,
    pub status: I2cStatus,
    pub timestamp_us: u64,
}

impl fmt::Display for I2cTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "I2C|addr=0x{:02x}|op={:?}|bytes={}|status={:?}|ts={}",
            self.addr, self.operation, self.bytes_transferred,
            self.status, self.timestamp_us)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum I2cOperation { Read, Write }

#[derive(Debug, Clone, Copy)]
pub enum I2cStatus { Success, Nack, Timeout, Error }

// Similar for GpioEvent, ImuReading, SensorStatus, etc.
```

**Firmware (src/bin/main.rs):**
```rust
#![no_std]
#![no_main]

use esp_println::println;
use esp_hal::{delay::Delay, main};
use lesson_08_usb_cdc_streaming::{I2cTransaction, I2cOperation, I2cStatus};

#[main]
fn main() -> ! {
    println!("BOOT|version=1.0|chip=ESP32-C6");

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    let mut timestamp_us: u64 = 0;
    let mut loop_count: u32 = 0;

    loop {
        // Simulate sensor readings
        if loop_count % 100 == 0 {
            let i2c_tx = I2cTransaction {
                addr: 0x68,
                operation: I2cOperation::Read,
                bytes_transferred: 6,
                status: I2cStatus::Success,
                timestamp_us,
            };
            println!("{}", i2c_tx);
        }

        loop_count += 1;
        timestamp_us += 10_000; // 10ms
        delay.delay_millis(10);
    }
}
```

**Python Parser (stream_parser.py):**
```python
#!/usr/bin/env python3
"""
USB CDC Stream Parser for ESP32-C6

Parses structured logging output from USB serial port.

Usage:
    python3 stream_parser.py /dev/cu.usbmodem2101
"""

import serial
import re
import sys
from dataclasses import dataclass
from typing import Dict

@dataclass
class I2cTransaction:
    addr: int
    operation: str
    bytes: int
    status: str
    timestamp_us: int

class StreamParser:
    def __init__(self, port: str, baudrate: int = 115200):
        self.ser = serial.Serial(port, baudrate, timeout=1)
        self.stats = {
            'i2c_count': 0,
            'gpio_count': 0,
            'imu_count': 0,
        }

    def parse_line(self, line: str):
        """Parse structured log line"""
        if not line:
            return

        parts = line.split('|')
        if len(parts) < 2:
            print(f"Raw: {line}")
            return

        msg_type = parts[0]
        fields = {}

        for field in parts[1:]:
            if '=' in field:
                key, value = field.split('=', 1)
                fields[key] = value

        if msg_type == "I2C":
            self.handle_i2c(fields)
        elif msg_type == "GPIO":
            self.handle_gpio(fields)
        elif msg_type == "IMU":
            self.handle_imu(fields)
        else:
            print(f"{msg_type}: {fields}")

    def handle_i2c(self, fields: Dict[str, str]):
        self.stats['i2c_count'] += 1
        print(f"I2C Transaction #{self.stats['i2c_count']}: "
              f"addr={fields.get('addr')}, "
              f"op={fields.get('op')}, "
              f"bytes={fields.get('bytes')}, "
              f"status={fields.get('status')}")

    def run(self):
        print("📡 Listening for USB CDC stream...")
        try:
            while True:
                if self.ser.in_waiting > 0:
                    line = self.ser.readline().decode('utf-8', errors='replace').strip()
                    self.parse_line(line)
        except KeyboardInterrupt:
            print("\n✓ Stream parser stopped")
            print(f"Statistics: {self.stats}")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python3 stream_parser.py /dev/cu.usbmodem2101")
        sys.exit(1)

    parser = StreamParser(sys.argv[1])
    parser.run()
```

### Phase 2: GDB Integration

**Add GDB debugging while streaming:**
- Use separate JTAG connection for GDB
- USB CDC continues streaming even when GDB halts
- Set breakpoints without interrupting stream

**GDB Helper (gdb_stream.py):**
```python
# Start streaming parser in background
# Attach GDB via JTAG
# Continue observing stream while debugging
```

### Phase 3: Advanced Features

1. **Ring Buffer for Burst Data**
   - Handle high-frequency events without dropping data
   - Circular buffer in firmware

2. **Data Compression**
   - Delta encoding for timestamps
   - Abbreviated field names
   - Binary protocol option

3. **Multi-Channel Streams**
   - Separate channels for different data types
   - Priority-based transmission

## Performance Analysis

### USB CDC Bandwidth Budget

**USB Full Speed:** 12 Mbps = 1.5 MB/s = 1,500 KB/s

**Structured log overhead:**
```
"I2C|addr=0x68|op=Read|bytes=6|status=Success|ts=123456\n"
= ~60 bytes per message
```

**Messages per second at different rates:**
- 10 Hz: 600 bytes/s (0.04% bandwidth)
- 100 Hz: 6 KB/s (0.4% bandwidth)
- 1000 Hz: 60 KB/s (4% bandwidth)
- 10,000 Hz: 600 KB/s (40% bandwidth)

**Conclusion:** Can easily stream 1000 Hz (1000 samples/second) with headroom.

### Comparison to RTT Goals

| Metric | RTT (Future) | USB CDC (This) |
|--------|--------------|----------------|
| Target throughput | 1-10 MB/s | 1.5 MB/s |
| Practical rate | Unknown (broken) | 600 KB/s @ 10kHz |
| Variables @ 100 Hz | 50-500 | 100-200 |
| Blocking | No | Yes (minimal impact) |
| **Works now** | ❌ No | ✅ Yes |

## Testing Strategy

### Automated Tests

1. **Build verification** - Firmware compiles
2. **Flash verification** - Can upload to ESP32-C6
3. **Stream capture** - Parser receives structured data
4. **Format validation** - All message types parse correctly
5. **Throughput test** - Sustain 1000 Hz for 60 seconds

### Manual Tests

1. Stream to terminal - Visual verification
2. Parse to CSV - Data export works
3. Real-time plotting - matplotlib visualization
4. GDB while streaming - Debug without interrupting stream

## Success Criteria

- [x] Plan documented
- [ ] Firmware streams structured data at 100 Hz
- [ ] Python parser successfully decodes all message types
- [ ] Sustained 1000 Hz streaming for 1 minute
- [ ] GDB debugging while stream continues
- [ ] Example sensor integration (I2C, GPIO, ADC)
- [ ] README with examples
- [ ] TEST.md with validation

## Timeline

**Estimated:** 2-3 hours total
- Phase 1 (Core): 1 hour
- Phase 2 (GDB): 30 min
- Phase 3 (Advanced): 1 hour (optional)
- Testing: 30 min

## Dependencies

**Cargo.toml:**
```toml
[dependencies]
esp-hal = { version = "1.0.0", features = ["esp32c6"] }
esp-println = { version = "0.13", features = ["esp32c6"] }
```

**Host requirements:**
- Python 3.7+
- pyserial library: `pip install pyserial`
- Optional: matplotlib for plotting

## Why This Is Better Than Future RTT

1. **Works immediately** - No tooling issues
2. **Simpler setup** - Just USB cable, no JTAG wiring
3. **Proven technology** - USB CDC is stable and mature
4. **GDB compatible** - Can debug simultaneously via JTAG
5. **Platform independent** - Works on macOS, Linux, Windows
6. **Good enough performance** - 1.5 MB/s meets most needs

## Migration Path from RTT

When RTT becomes available in the future:
1. Keep same structured data types
2. Swap `println!()` for `defmt::info!()`
3. Everything else stays the same
4. Can compare RTT vs USB CDC performance

---

**Status:** Ready to implement
**Next:** Create basic firmware and parser (Phase 1)
