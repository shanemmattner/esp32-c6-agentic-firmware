# ESP32-C6 Debugging Daemons

Simple daemon architecture for high-speed data streaming and arbitrary memory inspection.

## Architecture

```
┌─────────────┐         ┌──────────────┐         ┌──────────────┐
│  ESP32-C6   │ UART    │ UART Daemon  │ JSON    │              │
│  Firmware   ├────────→│ (streaming)  │────────→│   LLM Tool   │
│             │         │              │  stdin/ │   Interface  │
│             │         └──────────────┘  stdout │              │
│             │                                  │              │
│             │  GDB/   ┌──────────────┐         │              │
│             │  JTAG   │ GDB Daemon   │────────→│              │
│             ├────────→│ (polling)    │  JSON   │              │
└─────────────┘         └──────────────┘  stdin/  └──────────────┘
                                          stdout
```

## Design Principles

1. **Simple daemons** - No automatic detection or analysis, LLM controls everything
2. **JSON command interface** - stdin/stdout for easy scripting
3. **Binary data recording** - Fast recording with on-demand CSV export
4. **Independent operation** - UART and GDB daemons run independently
5. **Version controlled** - All daemon code in this repo

## Directory Structure

```
daemons/
├── README.md           # This file
├── uart/
│   └── uart_daemon.py  # UART streaming daemon
├── gdb/
│   └── gdb_daemon.py   # GDB memory inspection daemon (TODO)
└── common/
    └── protocol.md     # JSON protocol specification (TODO)
```

## UART Daemon

**Purpose:** High-speed data streaming from ESP32-C6 firmware (100-1000 Hz)

**Features:**
- Streams binary packets from UART
- Parses 64-byte TestPacket format
- Records to binary file
- Exports to CSV on demand
- JSON command interface

**Commands:**
```bash
# Start streaming
echo '{"cmd":"start", "port":"/dev/cu.usbserial-111300", "baudrate":115200, "record":"data.bin"}' | python3 uart/uart_daemon.py

# Get status
echo '{"cmd":"status"}' | python3 uart/uart_daemon.py

# Stop streaming
echo '{"cmd":"stop"}' | python3 uart/uart_daemon.py

# Export to CSV
echo '{"cmd":"export", "format":"csv", "output":"data.csv"}' | python3 uart/uart_daemon.py

# Quit
echo '{"cmd":"quit"}' | python3 uart/uart_daemon.py
```

**Responses:**
```json
{"status":"ok", "msg":"Started streaming on /dev/cu.usbserial-111300 @ 115200 baud"}
{"status":"data", "running":true, "packets":1234, "bytes":78976, "errors":0, "rate":6400}
{"status":"error", "msg":"Port not found"}
```

## GDB Daemon

**Purpose:** Low-speed arbitrary memory inspection (1-10 Hz)

**Features:**
- Connects to ESP32-C6 via GDB Machine Interface
- Parses ELF file for variable→address mapping
- Polls memory addresses at configurable rates
- Records to same binary format as UART daemon
- JSON command interface

**Status:** TODO - Not yet implemented

**Planned Commands:**
```bash
# Start GDB session
echo '{"cmd":"start", "elf":"firmware.elf", "probe":"303a:1001"}' | python3 gdb/gdb_daemon.py

# Watch variable
echo '{"cmd":"watch", "var":"sensor_temp", "rate":10}' | python3 gdb/gdb_daemon.py

# List all variables
echo '{"cmd":"list"}' | python3 gdb/gdb_daemon.py

# Stop watching variable
echo '{"cmd":"unwatch", "var":"sensor_temp"}' | python3 gdb/gdb_daemon.py

# Quit
echo '{"cmd":"quit"}' | python3 gdb/gdb_daemon.py
```

## Firmware Integration

### UART Speed Test Firmware

**Location:** `../lessons/08-usb-cdc-streaming/src/bin/uart_speed_test.rs`

**Features:**
- Streams 64-byte TestPacket @ 100 Hz
- LED blink on GPIO8 (500ms)
- UART on GPIO15 (TX) and GPIO23 (RX)
- Structured data: timestamp, counters, simulated sensors
- Packet validation with magic number (0xDEADBEEF) and checksum

**Packet Format:**
```rust
#[repr(C, packed)]
struct TestPacket {
    magic: u32,        // 0xDEADBEEF
    seq: u32,          // Sequence number
    timestamp_ms: u64, // Timestamp
    counter: u32,      // Counter value
    sensor_temp: i32,  // Simulated temp (centi-celsius)
    accel_x: i16,      // Simulated accel X
    accel_y: i16,      // Simulated accel Y
    accel_z: i16,      // Simulated accel Z
    state: u8,         // FSM state
    padding: [u8; 29], // Pad to 64 bytes
    checksum: u32,     // Simple checksum
}
```

**Build:**
```bash
cd ../lessons/08-usb-cdc-streaming
cargo build --release --bin uart_speed_test
```

**Flash:**
```bash
# Via JTAG
probe-rs run --chip esp32c6 --probe 303a:1001 target/riscv32imac-unknown-none-elf/release/uart_speed_test
```

## Usage Example

```bash
# Terminal 1: Start UART daemon
python3 daemons/uart/uart_daemon.py

# Terminal 2: Send commands
echo '{"cmd":"start", "port":"/dev/cu.usbserial-111300", "baudrate":115200, "record":"test_run.bin"}' > /tmp/uart_cmd

cat /tmp/uart_cmd | nc localhost 5000  # If using network wrapper

# Later: Export data
echo '{"cmd":"export", "format":"csv", "output":"test_run.csv"}' | nc localhost 5000

# Analyze with pandas
python3 -c "
import pandas as pd
df = pd.read_csv('test_run.csv')
print(df.describe())
print(df['sensor_temp_C'].plot())
"
```

## Baudrate Sweep Plan

Test maximum UART throughput:
1. **115200 baud** - Baseline (11.5 KB/s theoretical)
2. **921600 baud** - Fast (92 KB/s theoretical)
3. **2000000 baud** - Very fast (200 KB/s theoretical)
4. **3000000 baud** - Extreme (300 KB/s theoretical)
5. **4000000 baud** - Maximum ESP32-C6 supports (400 KB/s theoretical)

**Expected Results:**
- 100 Hz × 64 bytes = 6.4 KB/s (well below 115200 baud)
- Should work reliably at all tested speeds
- Goal: Find maximum sustainable rate with zero packet loss

**Test Procedure:**
1. Flash firmware with specific baudrate in Config
2. Start UART daemon with matching baudrate
3. Record for 60 seconds
4. Check error count (should be 0)
5. Export to CSV and validate packet sequence numbers
6. Repeat for higher baudrates

## Known Issues

### ESP32-C6 Boot Mode Problem
**Status:** UNRESOLVED HARDWARE ISSUE

**Symptoms:**
- Board boots into download mode (boot:0x75) after JTAG flash
- USB CDC not accessible after JTAG flash
- Power cycling doesn't fix the issue
- Only occurs with this specific board

**Workarounds Attempted:**
- Probe-rs JTAG flash → Failed
- espflash USB bootloader flash → Failed
- Power cycling → Failed
- DTR reset → Failed

**Current Status:**
- Firmware compiles successfully
- Daemon architecture implemented
- Waiting for hardware resolution

## Future Extensions

1. **RTT Daemon** - Add third daemon for RTT streaming (same JSON interface)
2. **Network Mode** - Wrap daemons with TCP server for remote access
3. **Real-time Plotting** - Add matplotlib integration for live visualization
4. **Variable Write** - Extend GDB daemon to support memory writes
5. **Type Detection** - Parse DWARF debug info for accurate type conversion
6. **Ring Buffer Export** - Export last N seconds of recorded data

## Dependencies

**Python:**
- `pyserial` - UART communication
- `json` - Command protocol
- `struct` - Binary packet parsing
- `threading` - Background data collection

**Install:**
```bash
pip3 install pyserial
```

## Testing

**Unit Tests:** TODO
**Integration Tests:** TODO
**Hardware Tests:** Blocked on ESP32-C6 boot mode issue

## Contributing

When adding new features:
1. Keep daemons simple - no automatic behavior
2. Use JSON stdin/stdout for all commands
3. Binary recording with on-demand CSV export
4. Follow existing command structure
5. Document all commands in this README

---

**Last Updated:** 2025-11-12
**Status:** Infrastructure complete, hardware testing blocked
