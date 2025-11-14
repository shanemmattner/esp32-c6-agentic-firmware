# Archived Lessons Documentation

These lessons are preserved in `lessons-archive/` for reference. They contain valuable peripheral driver code that can be referenced when building the new lesson sequence.

---

## Overview

| Lesson | Topic | Key Peripherals | Status | Reusability |
|--------|-------|----------------|---------|-------------|
| 01 | Button + NeoPixel | GPIO Input, RMT (WS2812) | Complete | Reference for GPIO/RMT |
| 02 | Task Scheduler | Atomics, Cooperative scheduling | Complete | Reference for scheduling |
| 03 | MPU9250 IMU | I2C, Sensor fusion | Complete | Reference for I2C drivers |
| 04 | State Machine | Button, LED, State patterns | Complete | Reference for FSM patterns |
| 05 | Unit Testing | Test organization, Mocking | Complete | Reference for testing strategy |
| 06 | UART Terminal | UART, CLI parsing | Complete | **Use for Lesson 02** |
| 07 | GDB Debugging | GDB, OpenOCD, probe-rs | Complete | **Merge into Lesson 01** |
| 08 | UART + GDB Tandem | Variable streaming, DMA | Complete | **Use for Lesson 03** |

---

## Lesson 01: Button + NeoPixel (Archive)

**Location:** `lessons-archive/01-button-neopixel/`

**What it teaches:**
- GPIO input with pull-ups (button on GPIO9)
- RMT peripheral for NeoPixel control (GPIO8)
- Edge detection and debouncing
- SmartLED driver usage

**Key code to reference:**
```rust
// GPIO input configuration
let button = Input::new(peripherals.GPIO9, InputConfig::default().with_pull(Pull::Up));

// RMT + NeoPixel setup
let rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(80))?;
let led = SmartLedsAdapter::<{ buffer_size(1) }, Blocking, color_order::Rgb, Ws2812Timing>::new_with_memsize(
    rmt.channel0,
    peripherals.GPIO8,
    2,
)?;

// Edge detection
if button_pressed && !button_was_pressed {
    led_on = !led_on;  // Toggle on press
}
```

**Reusable for:**
- New lessons needing NeoPixel control
- Button input examples
- RMT peripheral reference

---

## Lesson 02: Task Scheduler (Archive)

**Location:** `lessons-archive/02-task-scheduler/`

**What it teaches:**
- Cooperative task scheduling
- Atomic state management
- Modular code organization (button.rs, neopixel.rs, scheduler.rs)
- Periodic task execution

**Key code to reference:**
```rust
// Atomic state for cross-task communication
static LED_ON: AtomicBool = AtomicBool::new(false);
static BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

// Task scheduler
struct Scheduler {
    tasks: [Task; MAX_TASKS],
    task_count: usize,
}

impl Scheduler {
    fn add_task(&mut self, task: Task) { ... }
    fn run(&mut self, delay: &mut Delay) { ... }
}

// Task definition
struct Task {
    execute: fn(&mut Delay),
    interval_ms: u64,
    last_run: u64,
}
```

**Reusable for:**
- Multi-peripheral coordination
- Real-time task scheduling patterns
- Modular firmware architecture

---

## Lesson 03: MPU9250 IMU (Archive)

**Location:** `lessons-archive/03-mpu9250/`

**What it teaches:**
- I2C peripheral setup and communication
- Sensor driver implementation
- Register-level hardware control
- Accelerometer/gyroscope data reading

**Key code to reference:**
```rust
// I2C initialization
let i2c = I2c::new(
    peripherals.I2C0,
    Config {
        frequency: 400.kHz(),
        ..Default::default()
    },
)
.with_sda(peripherals.GPIO2)
.with_scl(peripherals.GPIO11);

// Sensor driver
pub struct Mpu9250<'a> {
    i2c: &'a mut I2c<'a, Blocking>,
    address: u8,
}

impl Mpu9250<'_> {
    pub fn read_accel(&mut self) -> Result<(i16, i16, i16), I2cError> {
        let mut buffer = [0u8; 6];
        self.i2c.write_read(self.address, &[REG_ACCEL_XOUT_H], &mut buffer)?;

        let x = i16::from_be_bytes([buffer[0], buffer[1]]);
        let y = i16::from_be_bytes([buffer[2], buffer[3]]);
        let z = i16::from_be_bytes([buffer[4], buffer[5]]);

        Ok((x, y, z))
    }
}
```

**Reusable for:**
- **Lesson 03+**: I2C driver development with GDB inspection
- Sensor integration examples
- I2C debugging techniques

---

## Lesson 04: State Machine Navigator (Archive)

**Location:** `lessons-archive/04-static-color-navigator/`

**What it teaches:**
- Statig state machine pattern
- Event-driven programming
- State transitions and guards
- Color navigation FSM

**Key code to reference:**
```rust
use statig::prelude::*;

#[derive(Default)]
pub struct ColorNavigator;

#[state_machine(
    initial = "State::idle()",
    state(derive(Debug, Clone)),
    on_transition = "Self::on_transition"
)]
impl ColorNavigator {
    #[state]
    fn idle(event: &Event) -> Response<State> {
        match event {
            Event::ButtonPress => Transition(State::navigating()),
            _ => Super,
        }
    }

    #[state]
    fn navigating(event: &Event) -> Response<State> {
        match event {
            Event::ButtonPress => Transition(State::idle()),
            Event::NextColor => Transition(State::navigating()),
            _ => Super,
        }
    }
}
```

**Reusable for:**
- Complex control flow patterns
- UI state management
- Event-driven embedded systems

---

## Lesson 05: Unit and Integration Testing (Archive)

**Location:** `lessons-archive/05-unit-and-integration-testing/`

**What it teaches:**
- Unit testing with `#[cfg(test)]`
- Integration testing on hardware
- Test organization patterns
- Mocking peripheral behavior

**Key code to reference:**
```rust
// Unit tests (no hardware needed)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_rotation() {
        let color = Color::Red;
        assert_eq!(color.next(), Color::Green);
        assert_eq!(color.next().next(), Color::Blue);
    }
}

// Integration test (runs on hardware)
#[main]
fn main() -> ! {
    run_all_tests();
    loop {}
}

fn test_button_debounce() {
    // Press button rapidly
    // Verify single toggle
}
```

**Reusable for:**
- Testing strategies for new lessons
- Hardware-in-the-loop test patterns
- CI/CD integration examples

---

## Lesson 06: UART Terminal (Archive) ⭐ **High Priority**

**Location:** `lessons-archive/06-uart-terminal/`

**What it teaches:**
- UART peripheral configuration
- Interactive command parsing
- Buffer management
- Real-time sensor streaming

**Key code to reference:**
```rust
// UART setup
let uart = Uart::new(peripherals.UART1, UartConfig::default())
    .with_tx(peripherals.GPIO15)
    .with_rx(peripherals.GPIO23);

// Command parser
fn parse_command(buffer: &[u8]) -> Option<Command> {
    let cmd = core::str::from_utf8(buffer).ok()?;

    match cmd.trim() {
        "led_on" => Some(Command::LedOn),
        "led_off" => Some(Command::LedOff),
        "imu_read" => Some(Command::ImuRead),
        _ => None,
    }
}

// Streaming
loop {
    if let Some(data) = uart.read_byte() {
        buffer.push(data);
        if data == b'\n' {
            handle_command(&buffer);
            buffer.clear();
        }
    }
}
```

**USE THIS FOR LESSON 02:**
- UART configuration with DMA
- High-speed data streaming
- `/improve-command` development workflow
- GDB + UART tandem preview

**Reusable components:**
- `src/uart.rs` - UART abstraction
- `src/cli.rs` - Command parser
- Buffer management patterns

---

## Lesson 07: GDB Debugging (Archive) ⭐ **Partially Integrated**

**Location:** `lessons-archive/07-gdb-debugging/`

**What it teaches:**
- GDB + OpenOCD setup
- Breakpoints and watchpoints
- Peripheral register inspection
- Python GDB helpers
- AI-assisted debugging workflow

**Key components:**
```python
# gdb_helpers.py
class ShowPeripherals(gdb.Command):
    def invoke(self, arg, from_tty):
        i2c_status = int(gdb.parse_and_eval("*(unsigned int*)0x60013004"))
        print(f"I2C0 STATUS: 0x{i2c_status:08x}")
```

```gdb
# .gdbinit
target remote :3333
set print pretty on
set demangle-style rust

define show-i2c
    x/16xw 0x60013000
end
```

**MERGED INTO LESSON 01:**
- GDB register manipulation techniques
- Register discovery workflow
- Automated debugging scripts

**Still useful:**
- OpenOCD configuration examples
- Complex GDB Python helpers
- Multi-peripheral debugging scenarios

---

## Lesson 08: UART + GDB Tandem (Archive) ⭐ **High Priority**

**Location:** `lessons-archive/08-uart-gdb-tandem/`

**What it teaches:**
- Bidirectional hardware control
- Variable streaming over UART
- GDB slot redirection
- Memory-safe pointer system
- DMA for high-throughput streaming

**Key code to reference:**
```rust
// Variable streaming system
#[repr(C)]
struct StreamSlot {
    ptr: *const u8,
    type_id: VarType,
    name: &'static str,
}

static mut SLOTS: [StreamSlot; 4] = [
    StreamSlot {
        ptr: &SENSOR_1 as *const _ as *const u8,
        type_id: VarType::I32,
        name: "sensor_1",
    },
    // ...
];

// Memory safety checks
fn validate_pointer(ptr: *const u8, var_type: VarType) -> Result<(), MemoryError> {
    if ptr < RAM_START || ptr >= RAM_END {
        return Err(MemoryError::OutOfBounds);
    }

    if ptr as usize % var_type.alignment() != 0 {
        return Err(MemoryError::Misaligned);
    }

    Ok(())
}

// GDB redirection
// (gdb) set SLOTS[0].ptr = 0x3FC8AC00
// UART now streams new variable
```

**USE THIS FOR LESSON 03:**
- Complete tandem debugging system
- DMA UART implementation
- Type-safe variable slots
- GDB + UART integration

**Advanced features:**
- Slot-based streaming (4-8 variables)
- Runtime variable redirection via GDB
- Bounds checking and alignment validation
- Structured logging for AI analysis

---

## Reusability Matrix

| Component | Lesson 01 (New) | Lesson 02 (Planned) | Lesson 03 (Planned) |
|-----------|----------------|---------------------|---------------------|
| GPIO Input/Output | Archive-01 | - | - |
| RMT (NeoPixel) | Archive-01 | - | - |
| UART Basic | - | Archive-06 ⭐ | Archive-06 |
| UART + DMA | - | Archive-06 ⭐ | Archive-08 ⭐ |
| I2C Driver | - | - | Archive-03 |
| GDB Scripts | Archive-07 ✓ | Archive-07 | Archive-07 |
| Variable Streaming | - | - | Archive-08 ⭐ |
| State Machine | - | - | Archive-04 |
| Task Scheduler | - | Archive-02 | Archive-02 |
| Testing Patterns | - | Archive-05 | Archive-05 |

**Legend:**
- ⭐ = High priority for next lesson
- ✓ = Already integrated
- - = Not needed yet

---

## Migration Strategy

### For Lesson 02: UART + DMA

**Copy from Archive-06:**
1. `src/uart.rs` - UART abstraction layer
2. UART configuration patterns
3. Buffer management code

**Copy from Archive-08:**
1. DMA setup code
2. High-speed streaming patterns
3. Performance optimization techniques

**Enhance with:**
- `/improve-command` meta-learning
- GDB register inspection during development
- Baud rate tuning experiments

### For Lesson 03: GDB + UART Tandem

**Copy from Archive-08:**
1. `src/bin/main.rs` - Variable streaming system
2. `src/lib.rs` - Type definitions
3. Memory safety validation code

**Copy from Archive-07:**
1. GDB Python helpers
2. Advanced debugging workflows
3. Multi-peripheral inspection scripts

**Enhance with:**
- RTT comparison research
- Autonomous debugging examples
- Claude Code integration for pattern detection

---

## Code Extraction Guide

**To extract I2C driver from Archive-03:**
```bash
cp lessons-archive/03-mpu9250/src/mpu9250.rs lessons/0X-i2c-lesson/src/
# Edit to match new lesson structure
```

**To extract UART from Archive-06:**
```bash
cp lessons-archive/06-uart-terminal/src/uart.rs lessons/02-uart-dma/src/
cp lessons-archive/06-uart-terminal/src/cli.rs lessons/02-uart-dma/src/
# Update for DMA and high-speed operation
```

**To extract variable streaming from Archive-08:**
```bash
cp lessons-archive/08-uart-gdb-tandem/src/bin/main.rs lessons/03-tandem-debug/src/bin/
# Simplify and enhance with Claude Code integration
```

---

## Preservation Notes

**Why archived:**
- Old lesson sequence didn't prioritize debugging tools first
- New sequence: Debug infrastructure → Build features
- Pedagogically better to learn GDB/UART before complex peripherals

**What's preserved:**
- All working driver code (I2C, UART, RMT, etc.)
- Testing patterns and strategies
- State machine implementations
- Scheduling algorithms
- Complete git history

**What's improved in new sequence:**
- Lesson 01: GDB-first approach (vs old Lesson 07)
- Lesson 02: UART + DMA early (vs old Lesson 06)
- Lesson 03: Tandem debugging (vs old Lesson 08)
- Claude Code integration throughout (new)

---

## Quick Reference

**Need GPIO input?** → Archive-01 (button.rs)
**Need I2C driver?** → Archive-03 (mpu9250.rs)
**Need UART?** → Archive-06 (uart.rs) ⭐
**Need DMA UART?** → Archive-08 (main.rs) ⭐
**Need state machine?** → Archive-04 (state_machine.rs)
**Need scheduling?** → Archive-02 (scheduler.rs)
**Need testing?** → Archive-05 (tests)
**Need GDB scripts?** → Archive-07 (gdb_helpers.py) ✓ Already used

---

## Summary

**Total preserved code:** ~8 complete lessons
**Lines of working Rust:** ~5000+
**Tested peripherals:** GPIO, RMT, I2C, UART, MPU9250
**Ready to reuse:** Lessons 06, 07, 08 for new sequence

All archived lessons are fully functional and hardware-tested. They provide a rich library of peripheral drivers and patterns for the new debugging-first curriculum.
