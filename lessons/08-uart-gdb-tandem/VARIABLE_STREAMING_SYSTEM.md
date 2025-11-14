# Arbitrary Runtime Variable Streaming System

## Overview

This system allows you to stream **arbitrary variables** from ESP32-C6 firmware at **runtime** without any compile-time registration. You can dynamically choose which variables to watch and at what rates.

## Architecture: Two Components Working as One

### Component 1: ESP32 Firmware (Generic Memory Streamer)
**Location:** `src/bin/memory_streamer.rs`

**What it does:**
- Receives commands over USB CDC like: `STREAM 0x3fc82040 4 1000`
  - `0x3fc82040` = memory address
  - `4` = number of bytes to read
  - `1000` = sampling rate in Hz
- Reads raw memory bytes and sends back: `DATA|addr=0x3fc82040|hex=AB12CD34`
- **Doesn't know or care** what variable names are
- Just a "dumb" memory reader

**Key features:**
- Supports up to 16 concurrent streams
- Non-blocking sampling (doesn't freeze firmware)
- Validates addresses are in SRAM range (prevents crashes)
- Streams at independent rates (10 Hz, 100 Hz, 1000 Hz, etc.)

### Component 2: Python Tool (Symbol Table Mapper)
**Location:** `var_stream.py`

**What it does:**
- Parses your compiled ELF file to extract all variable addresses
- Provides user-friendly CLI: `watch sensor_temp 100`
- Translates variable names → memory addresses
- Sends commands to ESP32 firmware
- Receives hex data and converts back to human-readable values

**How it works:**
```bash
$ python var_stream.py memory_streamer.elf /dev/cu.usbmodem2101

>>> list                           # Show all variables
Name                     Address      Size    Type
-----------------------------------------------------------
timestamp_ms             0x4080_1000  8       u64
heartbeat_counter        0x4080_1008  4       u32
loop_count               0x4080_100C  4       u32

>>> watch timestamp_ms 100         # Stream at 100 Hz
✓ Watching timestamp_ms @ 100 Hz
timestamp_ms: 1234567890

>>> watch heartbeat_counter 10     # Stream at 10 Hz
✓ Watching heartbeat_counter @ 10 Hz
heartbeat_counter: 42

>>> stop timestamp_ms              # Stop streaming
✓ Stopped watching timestamp_ms
```

## How They Work Together

```
┌─────────────────────────────────────────────────────────────┐
│                         YOUR COMPUTER                       │
│                                                             │
│  1. You type:  watch sensor_temp 100                       │
│                                                             │
│  2. Python parses ELF:                                     │
│     sensor_temp → 0x4080_2040 (4 bytes)                   │
│                                                             │
│  3. Python sends to ESP32:                                 │
│     "STREAM 0x4080_2040 4 100\n"                           │
│                                                             │
└─────────────────┬───────────────────────────────────────────┘
                  │ USB CDC
                  ↓
┌─────────────────────────────────────────────────────────────┐
│                       ESP32-C6 FIRMWARE                     │
│                                                             │
│  4. Firmware receives command                               │
│     → Adds stream config: {addr=0x4080_2040, size=4, 100Hz}│
│                                                             │
│  5. Every 10ms (100 Hz):                                    │
│     → Read 4 bytes from 0x4080_2040                        │
│     → Send: "DATA|addr=0x4080_2040|hex=12AB34CD\n"         │
│                                                             │
└─────────────────┬───────────────────────────────────────────┘
                  │ USB CDC
                  ↓
┌─────────────────────────────────────────────────────────────┐
│                         YOUR COMPUTER                       │
│                                                             │
│  6. Python receives hex data                                │
│     → Converts: 12AB34CD → 0x12AB34CD = 314,572,877       │
│     → Displays: "sensor_temp: 314572877"                   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## To the User: It Feels Like One Tool

From your perspective, you just run ONE command:
```bash
python var_stream.py memory_streamer.elf
```

Then you have a simple interface to watch any variable you want:
```
>>> watch my_variable 1000
```

Behind the scenes, the two components coordinate automatically.

## Example Use Cases

### Debug a sensor reading at high speed
```
>>> watch accel_x 1000        # 1kHz sampling
>>> watch accel_y 1000
>>> watch accel_z 1000
accel_x: 1234
accel_y: -5678
accel_z: 9012
...
```

### Monitor state machine at low rate
```
>>> watch fsm_state 10        # 10 Hz is enough
fsm_state: 2
fsm_state: 3
fsm_state: 3
fsm_state: 4
```

### Stream different variables at different rates
```
>>> watch timestamp_ms 100    # Fast timestamp
>>> watch button_state 50     # Medium button polling
>>> watch temp_celsius 1      # Slow temperature
```

## Safety Considerations

### Memory Safety Risks
1. **Torn reads**: Multi-byte variables might change mid-read
   - Solution: Use `read_volatile()` in firmware
2. **Invalid addresses**: Reading unmapped memory crashes the chip
   - Solution: Validate address is in SRAM range (0x4080_0000 - 0x4088_0000)
3. **MMIO side effects**: Reading peripheral registers can trigger hardware actions
   - Solution: Only stream SRAM addresses (not MMIO registers)

### Current Limitations
1. **Read-only**: Cannot write to variables (yet)
2. **No atomicity**: Variables can change between bytes
3. **No type safety**: Python guesses types from size
4. **USB CDC input not implemented**: Commands are planned but not yet working

## Next Steps

1. **Fix USB CDC input** - Firmware needs bidirectional capability
2. **Add test variables** - Create demo with sensor data, counters, etc.
3. **Improve type detection** - Use DWARF debug info for accurate types
4. **Add write capability** - Allow setting variables at runtime
5. **Real-time plotting** - Visualize streamed data with matplotlib

## Current Status

✅ ESP32 firmware compiles and flashes
✅ Python tool parses ELF and extracts variable addresses
✅ Architecture designed for arbitrary runtime streaming
⚠️ USB CDC bidirectional input not yet implemented
⚠️ Need test variables to demonstrate streaming
