# Lesson 06: Production UART Serial Terminal - PRD

## Overview
- **Lesson Number**: 06
- **Feature**: Production-grade UART terminal with DMA, ring buffers, and full firmware control
- **Duration**: 2-3 hours
- **Difficulty**: Advanced
- **Prerequisites**: Lessons 01-05 (especially Lesson 03 for IMU, Lesson 04 for state machine)

## Learning Objectives

1. Configure UART peripheral with DMA and interrupt-driven I/O
2. Implement ring buffers for efficient serial communication
3. Build a menu-driven CLI using the `menu` crate
4. Create testable command parsing logic (pure functions)
5. Integrate serial terminal control with running firmware (state machine)
6. Minimize CPU overhead with ISR-based architecture

## Hardware Requirements

- ESP32-C6 development board
- USB-to-UART adapter (or use built-in USB if available)
- 3 wires: TX, RX, GND

**Pin Configuration**:
- GPIO15: UART1 TX (output from ESP32-C6)
- GPIO23: UART1 RX (input to ESP32-C6)
- GND: Common ground

## Software Requirements

- esp-hal 1.0.0 with UART + DMA features
- `menu` crate (no_std CLI framework)
- `heapless` crate (fixed-size data structures)
- No dynamic allocation (production-grade embedded)

## Expected Behavior

### Serial Output Patterns (Reference Behavior)

When you connect and send commands:

```
🚀 Starting Lesson 06: Production UART Terminal
✓ UART1 initialized on GPIO15/23 (115200 baud)
✓ I2C initialized on GPIO2/11 (MPU9250)
✓ State machine ready (initial: warm_palette)
✓ NeoPixel LED ready

Ready for commands. Type 'help' for available commands.

> help
Available commands:
  help                      - Show this message
  status                    - Show system status
  imu_read                  - Read MPU9250 accelerometer once (X, Y, Z)
  imu_stream <rate_hz>      - Start continuous IMU stream (10, 50, 100 Hz)
  imu_stream stop           - Stop IMU streaming
  imu_range <g>             - Set accel range (2, 4, 8, 16 g)
  imu_filter <hz>           - Set low-pass filter (5, 10, 20, 50 Hz)
  imu_status                - Show IMU info and current settings
  state_get                 - Get current state machine state
  state_set <name>          - Set state (warm_palette, cool_palette)
  led_on                    - Turn LED on
  led_off                   - Turn LED off
  led_color <r> <g> <b>    - Set LED color (0-255 each)
  echo <text>               - Echo text back
  reset                     - Soft reset

> status
System Status:
  Uptime: 1234ms
  State: warm_palette
  LED: ON (RGB: 100, 50, 200)
  IMU: accel_x=1200, accel_y=-500
  Free memory: 156KB

> imu_status
📋 IMU Status:
  Connected: YES
  Range: ±2g
  Filter: 20 Hz
  Sample rate: 100 Hz

> imu_range 16
✓ Accel range set to ±16g

> imu_filter 50
✓ Low-pass filter set to 50 Hz

> imu_read
📊 IMU Reading:
  accel_x: 1200 mg
  accel_y: -500 mg
  accel_z: 9800 mg

> imu_stream 50
✓ IMU streaming started (50 Hz)
📊 1200,-500,9800
📊 1210,-490,9801
📊 1195,-510,9799
📊 1220,-475,9802
📊 1205,-505,9800
> imu_stream stop
✓ IMU streaming stopped

> state_set cool_palette
✓ State changed: warm_palette → cool_palette
💡 LED updated to cool colors

> led_color 255 0 0
✓ LED set to RED

> echo hello from firmware
> hello from firmware

> reset
Resetting...
🚀 Starting Lesson 06: Production UART Terminal
...
```

**Critical patterns to verify**:
- ✅ Terminal prompt (`> `) after each command
- ✅ Command echo (show what user typed)
- ✅ Success/error messages with emoji indicators
- ✅ Interactive and responsive (no blocking)
- ✅ UART0 logging still works independently (debug output)
- ✅ Commands integrate with Lesson 04 state machine
- ✅ No lag or dropped characters (DMA working)

## Functional Requirements

1. **REQ-1: DMA-Driven UART**
   - UART1 configured with RX DMA on GPIO15/23
   - Ring buffer for efficient data handling
   - ISR minimizes main loop overhead

2. **REQ-2: Interactive Shell**
   - Prompt-based interface (`> `)
   - Echo user input as typed
   - Responsive even with active IMU + LED updates

3. **REQ-3: Extensible Command System**
   - Menu-based commands with help text
   - Commands can have parameters
   - Testable command parsing (pure functions in lib.rs)

4. **REQ-4: IMU Integration (Lesson 03)**
   - Read current accelerometer once: `imu_read`
   - Continuous IMU streaming: `imu_stream <rate>`, `imu_stream stop`
     - Configurable stream rates: 10, 50, 100 Hz
     - Compact format: `📊 x,y,z` (minimal overhead for high-frequency data)
   - Configure IMU parameters:
     - `imu_range <g>` - Set accelerometer range (2, 4, 8, 16 g)
     - `imu_filter <hz>` - Set low-pass filter (5, 10, 20, 50 Hz)
   - Show IMU status: `imu_status` (connection, range, filter, sample rate)
   - Real-time monitoring without blocking main loop

5. **REQ-5: State Machine Control (Lesson 04)**
   - Read current state: `state_get`
   - Transition to state: `state_set <name>`
   - Full integration with Lesson 04 color navigator

6. **REQ-6: LED Control (Lesson 01)**
   - Turn on/off: `led_on`, `led_off`
   - Set color: `led_color <r> <g> <b>`
   - Show current color in status

7. **REQ-7: System Monitoring**
   - `status` command shows: uptime, state, LED color, IMU values, memory
   - Real-time data without blocking

8. **REQ-8: Unobtrusive Operation**
   - Commands handled in background (ISR-driven)
   - Main firmware runs without interruption
   - State machine + IMU reading unaffected by terminal I/O
   - Minimal CPU impact even with active terminal

9. **REQ-9: Production Quality**
   - All commands have timeout protection
   - No panics on invalid input
   - Graceful error messages
   - Memory-safe (heapless, no_std)

## Technical Specifications

- **UART Baudrate**: 115200 bps
- **RX Buffer Size**: 256 bytes (ring buffer)
- **TX Buffer Size**: 256 bytes (ring buffer)
- **DMA**: Enabled for RX
- **ISR**: UART RX ISR + optional timeout ISR
- **Blocking**: Non-blocking everywhere (async-style without async keyword)
- **Memory**: Zero heap allocation

## Code Organization for Video Production

This lesson uses a **hybrid approach** for YouTube sustainability:

- **~50 lines USER TYPES** - Core logic, the interesting parts
- **~200 lines COPY-PASTE** - Drivers, boilerplate, utilities

Each code file has **comments marking sections**:
```rust
// ============================================================================
// [SECTION 1/3: USER TYPES - Main loop and IMU streaming]
// ============================================================================
// DELETE THIS COMMENT and type from here until [END SECTION 1/3]

fn main() -> ! {
    // ... your code here ...
}

// [END SECTION 1/3]

// ============================================================================
// [SECTION 2/3: COPY-PASTE - UART driver (boilerplate)]
// ============================================================================
// Keep this entire section, copy it from starter code if needed

// ... copy-pasted code ...

// [END SECTION 2/3]
```

During video recording:
1. Delete the `[SECTION N/M]` comment lines
2. Type the "USER TYPES" section live
3. Copy-paste the "COPY-PASTE" sections
4. Result: Clean final code with no section markers

## Implementation Plan

### Code Structure

```
lessons/06-uart-terminal/
├── src/
│   ├── lib.rs                      # [COPY-PASTE] Pure types + exports
│   ├── command.rs                  # [COPY-PASTE] Command enum + parser
│   ├── uart_driver.rs              # [COPY-PASTE] UART1 + DMA + ring buffers
│   └── bin/main.rs                 # [USER TYPES + COPY-PASTE]
│                                   #   - Main loop (USER TYPES)
│                                   #   - IMU streaming (USER TYPES)
│                                   #   - Initialization (COPY-PASTE)
│
├── tests/
│   ├── command_parsing.rs          # [COPY-PASTE] Unit tests
│   └── integration_test.rs         # [COPY-PASTE] Device test
│
├── STARTER_CODE.md                 # Copy-paste sections with explanation
├── Cargo.toml
└── README.md
```

### Video Breakdown

| Phase | Duration | What Happens |
|-------|----------|--------------|
| Intro | 2 min | Explain architecture + expected output |
| Setup | 3 min | Copy-paste uart_driver.rs, command.rs, lib.rs |
| Core | 10 min | **USER TYPES** main.rs sections (live coding) |
| Test | 5 min | Build, flash, test on hardware |

**Total video: ~20 minutes** (reasonable for complex lesson)

### Key Implementation Points

**Phase A: UART Driver (uart_driver.rs)**
- Configure UART1 on GPIO15/23
- Initialize RX DMA with ring buffer
- ISR handler for DMA complete interrupt
- TX via polling or ISR
- Non-blocking read/write functions

**Phase B: Command Parser (lib.rs + command.rs)**
- Pure `parse_command()` function (testable)
- Command enum with variants:
  - `Help`
  - `Status`
  - `ImuRead`
  - `ImuStream(rate_hz)` or `ImuStreamStop`
  - `ImuRange(g)` - Set range in grams (2, 4, 8, 16)
  - `ImuFilter(hz)` - Set filter frequency (5, 10, 20, 50)
  - `ImuStatus`
  - `StateGet`, `StateSet(palette_name)`
  - `LedOn`, `LedOff`, `LedColor(r, g, b)`
  - `Echo(text)`
  - `Reset`
- Parameter parsing logic (state names, RGB values, numeric ranges)
- Validation and error types
- NO UART I/O in parser (dependency injection)

**Phase C: Menu Integration (main.rs)**
- Combine Lessons 01, 03, 04 code:
  - Button + NeoPixel (Lesson 01)
  - MPU9250 I2C + reading (Lesson 03)
  - State machine (Lesson 04)
- `menu` crate with command callbacks
- Wire UART input to menu runner
- Menu callbacks execute commands:
  - `ImuRead` → call `mpu9250::read_accel()` from Lesson 03
  - `ImuStream` → set global flag + rate, main loop streams data
  - `ImuRange`, `ImuFilter` → reconfigure MPU9250 registers
  - `ImuStatus` → query sensor configuration
  - `StateGet` → query state machine state
  - `StateSet` → handle state transition via state machine
  - `LedColor` → update LED via shared state
  - `Status` → gather all sensor/state data
- Streaming handler in main loop:
  - Check if streaming enabled
  - Read IMU at configured rate (using timer/tick)
  - Send compact format `📊 x,y,z` to UART
  - Non-blocking (use timer interrupt or main loop tick)

**Phase D: Testing**
- Unit tests for command parser (host)
- Integration tests with UART (device)

### Logging Strategy

```rust
info!("🚀 Starting Lesson 06: UART Terminal");     // Startup on UART0
info!("✓ UART1 initialized on GPIO15/23");        // Driver init
// ... main loop ...
// Terminal output goes to UART1 (separate from logging)
```

## Testing Requirements

### Unit Tests (Host Tests)
- Test command parsing for all command types
- Test parameter extraction
- Test error handling (invalid commands, bad parameters)
- Test with various input formats

### Integration Tests (Device)
- Test UART initialization
- Test RX DMA and ring buffer
- Test full command flow: input → parse → execute → response
- Test multiple rapid commands (stress test)
- Verify Lesson 04 state machine control works

### Manual Testing
- Connect USB-to-UART adapter to GPIO15/23
- Open terminal (minicom, picocom, Miniterm)
- Test each command
- Verify echo and prompts work
- Test while other firmware running (Lesson 04 color changes)

## Success Criteria (All Mandatory)

- [x] Code builds without warnings
- [x] Unit tests pass (28+ tests from Lesson 05 + new command tests)
- [x] Integration test passes on hardware
- [x] All commands respond correctly
- [x] No dropped characters with rapid input
- [x] Main loop runs uninterrupted by terminal I/O
- [x] State machine control works (state_get, state_set)
- [x] User validates on real hardware

## Edge Cases

1. **Rapid input**: Send many characters quickly → verify ring buffer handles it
2. **Long commands**: Type 200+ characters → verify buffer boundaries
3. **Invalid commands**: Type gibberish → graceful error message
4. **Bad parameters**:
   - `state_set invalid_palette` → error message
   - `led_color 300 50 50` → clamp to 255
   - `led_color 50` → missing parameters error
   - `imu_stream 200` → reject invalid rate, suggest valid options
   - `imu_range 32` → reject invalid range
5. **Concurrent operations**:
   - Change state via terminal while IMU updating → safe
   - Read IMU while state machine active → accurate data
   - LED commanded via terminal while state machine changing it → last command wins (acceptable)
6. **IMU streaming edge cases**:
   - Start streaming → output begins, prompt hidden until stopped
   - Type while streaming → accumulate in RX buffer, process after stop
   - High stream rate (100 Hz) with other commands → verify no drops
   - `imu_stream stop` while not streaming → handle gracefully
   - Switch stream rates → smooth transition without data loss
7. **IMU configuration during streaming**:
   - `imu_range` while streaming → apply change, resume streaming
   - `imu_filter` while streaming → apply change, resume streaming
8. **IMU errors**: MPU9250 not connected → `imu_read` returns error, doesn't crash
9. **Terminal silent during initialization**: Startup messages print before terminal ready

## Building on Previous Lessons

This lesson integrates **all previous lessons** into one cohesive system:

| Lesson | Component | How It's Used |
|--------|-----------|---------------|
| **01** | Button + NeoPixel | Terminal can control LED, button still works |
| **02** | Task Scheduler | Terminal commands run without blocking main loop |
| **03** | MPU9250 I2C | `imu_read`, `imu_status` commands read sensor |
| **04** | State Machine | `state_get`, `state_set` control color navigator |
| **05** | Testing | Command parser tested with unit tests (host) |

**Before Lesson 06:**
- Firmware runs autonomously with little visibility
- Limited control (just button)

**After Lesson 06:**
- Full visibility into system state (status command)
- Remote control of everything (LED, state machine)
- Real-time monitoring (IMU readings)
- Debug interface for development

## Architecture Decisions

### Why DMA + ISR + Ring Buffers?

Traditional approach (polling):
```rust
loop {
    if uart.is_data_available() {
        byte = uart.read();
        process(byte);
    }
}
```
❌ Blocks main loop, wastes CPU

Our approach (DMA + ISR):
```rust
// ISR (background, automatic)
uart_rx_isr() {
    // DMA already filled buffer, just handle interrupt
    ring_buffer.advance_head();
}

// Main loop (continues running)
loop {
    // Do real work
    if let Some(byte) = ring_buffer.read() {
        process(byte);
    }
}
```
✅ Non-blocking, minimal CPU impact

### Why Pure Command Parser?

Command parser is **pure logic** (no I/O):
```rust
// Testable on host! No UART required
fn parse_command(input: &str) -> Result<Command, ParseError> {
    // Deterministic, no side effects
}
```

✅ Fast unit tests (milliseconds)
✅ Easy to test edge cases
✅ Reusable in different contexts

UART I/O is separate → tested on device

## Comparison: This vs Naive Approach

| Aspect | Naive | This Lesson |
|--------|-------|------------|
| **CPU Impact** | 10-20% (polling) | <1% (ISR-driven) |
| **Testability** | Coupled to UART | Pure logic testable on host |
| **Responsiveness** | Variable | Deterministic (interrupt-driven) |
| **Code Quality** | Ad-hoc | Structured (menu crate) |
| **Production Ready** | No | Yes |

## References

- [ESP32-C6 UART Module](https://docs.esp-rs.org/esp-hal/main/esp_hal/uart/)
- [menu crate documentation](https://docs.rs/menu/latest/menu/)
- [heapless crate](https://docs.rs/heapless/latest/heapless/)
- [DMA Programming Guide](https://docs.esp-rs.org/esp-hal/main/esp_hal/dma/)
- [Ring Buffer Pattern](https://en.wikipedia.org/wiki/Circular_buffer)

---

**Status**: Ready for Phase 1 Review
**Next Steps**: User approval → Phase 2 Setup → Phase 3 Implementation
