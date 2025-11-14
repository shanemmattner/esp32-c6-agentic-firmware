# Lesson 08: UART + GDB Tandem Debugging

Stream variables in real-time over UART while simultaneously controlling firmware behavior with GDB - no recompilation needed.

## Learning Objectives

- **Pointer-based variable streaming** - GDB redirects stream slots to any memory address
- **Memory safety in embedded** - Bounds checking and alignment validation
- **GDB + UART tandem workflow** - Continuous logging meets interactive control
- **Zero-recompilation debugging** - Change what's streamed without rebuilding firmware

## Hardware Requirements

- ESP32-C6 development board
- USB cable (for GDB via USB-JTAG)
- FTDI USB-to-UART adapter (for streaming)
- Jumper wires

## Wiring

Connect UART1 to FTDI adapter:

| ESP32-C6 | FTDI Adapter |
|----------|--------------|
| GPIO23 (TX) | RX |
| GPIO15 (RX) | TX |
| GND | GND |

**Note:** GPIO pins can be changed in `src/bin/main.rs` (lines 24-25)

## What You'll Learn

### The Traditional Debugging Dilemma

**Option A: UART Logging**
- ✅ Continuous output
- ✅ See real-time behavior
- ❌ No interaction
- ❌ Limited to hardcoded logs

**Option B: GDB Debugging**
- ✅ Full control
- ✅ Inspect any variable
- ❌ Firmware paused
- ❌ Can't see continuous behavior

### Our Solution: Use Both!

**UART Stream (Continuous) + GDB Control (Interactive)**

- Watch 4 configurable variable slots streaming over UART
- GDB redirects slots to any variable in memory
- Inject test values and see immediate UART response
- Never stop firmware execution

## Building and Flashing

```bash
# Auto-detect ports
source ../../scripts/find-esp32-ports.sh

# Build and flash
cargo build --release
espflash flash --port $USB_CDC_PORT target/riscv32imac-unknown-none-elf/release/main

# In another terminal, monitor UART output
python3 ../../.claude/templates/read_uart.py $FTDI_PORT 30
```

## Expected Output

```
=== Variable Streaming System ===
Slot 0: &sensor_1 = 0x3FC8ABCD -> 100
Slot 1: &sensor_2 = 0x3FC8ABD0 -> 200
Slot 2: &counter  = 0x3FC8ABD4 -> 0
Slot 3: &state    = 0x3FC8ABD8 -> 1

Stream: s0=100 s1=200 s2=0 s3=1
Stream: s0=101 s1=200 s2=1 s3=1
Stream: s0=102 s1=200 s2=2 s3=1
...
```

## GDB Tandem Debugging Workflow

### 1. Start GDB Session

```bash
# Flash firmware first (see above)

# Attach GDB
riscv32-esp-elf-gdb target/riscv32imac-unknown-none-elf/release/main
(gdb) target remote :3333
(gdb) continue
```

**UART stream continues in background!**

### 2. Redirect Stream Slots

```gdb
# Stop firmware temporarily
(gdb) interrupt

# Find address of a variable you want to stream
(gdb) print &button_count
$1 = (u32 *) 0x3FC8AC00

# Redirect slot 0 to stream button_count
(gdb) set SLOTS[0].ptr = 0x3FC8AC00

# Resume - UART now shows button_count in slot 0!
(gdb) continue
```

### 3. Inject Test Values

```gdb
# Test edge case without hardware
(gdb) interrupt
(gdb) set temperature = 150
(gdb) continue
# UART instantly shows overheat response
```

### 4. Hardware Watchpoints

```gdb
# Break when variable changes
(gdb) watch button_count
(gdb) continue
# Stops automatically when button_count changes
```

## Real-World Use Cases

### 1. Testing Edge Cases

**Scenario:** Test temperature sensor at 150°C

**Traditional:** Heat gun + fire hazard + slow iteration
**Our Approach:**
```gdb
(gdb) set TEMPERATURE = 150
(gdb) continue
# UART shows response immediately
```

### 2. Debugging State Machines

**Scenario:** Bug appears after 10,000 button presses

**Traditional:** Click 10,000 times
**Our Approach:**
```gdb
(gdb) set button_count = 9999
(gdb) continue
# UART shows bug condition
```

### 3. Performance Tuning

**Scenario:** Find optimal PID gains

**Traditional:** Recompile 20 times with different constants
**Our Approach:**
```gdb
(gdb) set kp = 0.5
(gdb) set ki = 0.1
(gdb) set kd = 0.05
(gdb) continue
# Watch UART for stability
```

## Memory Safety Features

This lesson demonstrates production-grade memory safety:

```rust
// Bounds checking
if ptr < RAM_START || ptr >= RAM_END {
    return Err(MemoryError::OutOfBounds);
}

// Alignment validation
if ptr % align_of::<T>() != 0 {
    return Err(MemoryError::Misaligned);
}

// Type safety
match slot.var_type {
    VarType::I32 => { /* safe cast to i32 */ }
    VarType::U32 => { /* safe cast to u32 */ }
    // ...
}
```

## Code Structure

### Firmware (`src/bin/main.rs`)

- **Memory Safety** (lines 38-42): RAM bounds and alignment constants
- **VarType Enum** (lines 46-53): Type-safe variable types
- **VariableSlot** (lines 55-68): Pointer + type + metadata
- **Validation** (lines 70-115): Bounds and alignment checking
- **Streaming Loop** (lines 200+): Continuous UART output

### Key Components

1. **Variable Slots** - 4 configurable pointers to any variable
2. **Type System** - I32, U32, F32, BOOL with safe casting
3. **Memory Validation** - Bounds checking, alignment, null detection
4. **UART Streaming** - Continuous output at 10 Hz
5. **GDB Interface** - Direct memory access to slots

## Troubleshooting

### No UART output

1. Verify wiring: ESP32 TX → FTDI RX, GND → GND
2. Test pins: `../../scripts/test-uart-pins.sh 23 15 5`
3. Check baud rate: 115200 (default)

### GDB can't connect

1. Flash firmware first
2. Check USB-JTAG cable connected
3. Try: `espflash monitor --chip esp32c6`

### Garbled UART output

1. Verify baud rate matches (115200)
2. Check only one program is reading the port
3. Try lower streaming frequency in code

### Can't redirect slots in GDB

1. Ensure firmware is built with debug symbols: `--release` still includes them
2. Use correct slot index: `SLOTS[0]`, `SLOTS[1]`, `SLOTS[2]`, `SLOTS[3]`
3. Check address is in valid RAM range: `0x3FC80000 - 0x3FD00000`

## Performance

- **UART Bandwidth:** 115200 baud = ~14 KB/s
- **Stream Rate:** 10 Hz (configurable)
- **Message Size:** ~60 bytes per update
- **Throughput:** ~600 bytes/sec (4% of capacity)

**Plenty of headroom for additional variables and higher rates.**

## Extending This Lesson

**Add more variable types:**
```rust
enum VarType {
    I32, U32, F32, BOOL,
    I16, U16, I8, U8,  // Add these
}
```

**Increase slot count:**
```rust
const MAX_SLOTS: usize = 8;  // Was 4
```

**Add DMA for higher throughput:**
- See `LESSON_08_REDESIGN.md` for DMA implementation plan

**Create GDB Python scripts:**
```python
# custom_commands.py
class StreamVariable(gdb.Command):
    def invoke(self, arg, from_tty):
        # Automatically redirect slot to variable
```

## Next Steps

- **Lesson 09:** (TBD)
- **Challenge:** Stream 10 variables at 100 Hz without dropping frames
- **Advanced:** Implement circular buffer with DMA for zero-copy streaming

## References

- [GDB Manual](https://sourceware.org/gdb/current/onlinedocs/gdb/)
- [esp-hal UART](https://docs.esp-rs.org/esp-hal/esp-hal/uart/)
- [ESP32-C6 Memory Map](https://www.espressif.com/sites/default/files/documentation/esp32-c6_technical_reference_manual_en.pdf)
- [RISC-V GDB Guide](https://github.com/riscv-collab/riscv-gnu-toolchain)
