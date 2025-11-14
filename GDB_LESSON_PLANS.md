# GDB Lesson Plans: Claude Code + GDB + UART Workflow for ESP32-C6

**ESP32-C6 + esp-hal 1.0.0 - From Blinky to Production HIL Testing**

**Primary Goals:**
1. **Learn the Claude Code + GDB + UART workflow** for effective embedded development
2. **Master esp-hal 1.0.0** Rust embedded patterns for ESP32-C6
3. **Build production-quality embedded systems** with professional testing practices

**Philosophy**: Start where every engineer starts (blinky!), then progressively build a complete embedded system. At each stage, learn how to use **Claude Code to drive development**, **GDB for precise debugging**, and **UART for continuous observability**.

**Not just learning GDB** - learning the **complete workflow**:
- Claude Code writes code, suggests debugging strategies
- UART streams continuous telemetry (big picture)
- GDB catches exact moments (precise inspection)
- Together: extremely effective embedded development

**Lesson progression:**
0. **Blinky** - Get comfortable with ESP32-C6 + esp-hal 1.0.0 (no tools yet)
1. **Claude + GDB fundamentals** - Learn the collaborative debugging workflow
2. **Add UART streaming** - Combine UART observability with GDB precision
3. **State machines + I2C** - Apply workflow to complex systems
4. **Task scheduler + Atomics** - Debug concurrent embedded systems
5. **Virtual HIL testing** - Professional testing without hardware

**Hardware**: ESP32-C6-DevKitC-1 (GPIO9 button + GPIO8 NeoPixel onboard) + MPU9250 IMU

---

## Lesson 00: LED Blinky - The Classic Introduction

### Overview
**Focus:** Get started with ESP32-C6, esp-hal 1.0.0, and embedded Rust
**Duration:** 30-45 minutes
**Complexity:** ⭐☆☆☆☆
**Hardware:** ESP32-C6-DevKitC-1 (onboard NeoPixel GPIO8)

### What You'll Build
The classic LED blinky - the "Hello World" of embedded systems. Just like every other engineer before you!

Simple NeoPixel blink:
- Turn LED red for 500ms
- Turn LED off for 500ms
- Repeat forever

### Embedded Practices
1. **Cargo project setup** - esp-hal 1.0.0 project structure
2. **RMT peripheral** - Timing-critical WS2812 control
3. **Delay** - Basic timing without timers
4. **No interrupts, no complexity** - Just blink!

### Why No GDB Yet?
This lesson is intentionally simple - just get something working! You'll add GDB in Lesson 01 when you have something interesting to debug.

**The irony**: Claude Code (an AI coding assistant) starts the same way every engineer has started for decades - with a blinking LED!

### Commit Structure (1 commit)

**Commit 1: Working LED blinky**
- Complete working implementation
- Simple, clean, ~50 lines of code
- Red blink → off → repeat

No progressive commits yet - just get it working!

### Learning Objectives
- Set up ESP32-C6 development environment
- Understand esp-hal 1.0.0 project structure
- Control NeoPixel RGB LED via RMT peripheral
- Basic timing with `delay`
- Get familiar with flashing workflow

### Success Criteria
- [ ] Project builds without errors
- [ ] Firmware flashes to ESP32-C6
- [ ] Onboard NeoPixel blinks red
- [ ] 500ms on, 500ms off, repeating
- [ ] UART shows startup messages

### What's Next?
In **Lesson 01**, we'll add:
- Button input (GPIO9)
- GDB debugging
- Interactive control (button changes color)
- Claude teaches you GDB while developing

But for now - just enjoy the blinky! Every embedded engineer remembers their first LED. 🎉

---

## Lesson 01: GDB Fundamentals with Button + NeoPixel

### Overview
**Focus:** Claude-driven development with basic GDB, event-driven programming
**Duration:** 60-90 minutes
**Complexity:** ⭐⭐☆☆☆
**Hardware:** ESP32-C6-DevKitC-1 (all onboard - GPIO9 button, GPIO8 NeoPixel)

### What You'll Build
Simple button-controlled NeoPixel color cycler. Press button → cycle through red, green, blue.

### Embedded Practices
1. **Event-driven architecture** - Edge detection for button presses
2. **Debouncing** - Clean input handling
3. **RMT peripheral** - Timing-critical WS2812 control
4. **Memory-mapped I/O** - Understanding GPIO registers

### GDB Techniques (3) - Claude Teaches You
1. **Memory inspection/writes** - Read/write GPIO registers to see button state
2. **GDB variables** - Bit math calculator (`set $gpio = 9; set $mask = 1 << $gpio`)
3. **Function calls** - Live hardware control: `call neopixel_set_color(255, 0, 0)`

### Commit Structure (3 commits)

**Commit 1: Broken button handler**
- Button press doesn't reliably change color
- Missing debouncing → bounces register multiple presses
- Claude guides: "Let's use GDB to inspect GPIO register during button press"

**Commit 2: GDB-controlled NeoPixel**
- Remove all button code, empty main loop
- Claude teaches bit manipulation via GDB:
  ```gdb
  set $neopixel_gpio = 8
  set $mask = 1 << $neopixel_gpio
  # Manually control LED from GDB
  call neopixel_set_color(255, 0, 0)  # Red
  call neopixel_set_color(0, 255, 0)  # Green
  ```
- "Wow moment": Control hardware without firmware changes!

**Commit 3: Add proper debouncing**
- Claude explains: "Let's add edge detection and debouncing"
- Implement clean button handler
- Full working solution

### Claude's Teaching Approach
- **Collaborative investigation**: "What value do you see in the GPIO_IN register?"
- **Guided discovery**: "Try writing to address 0x60091008. What happens?"
- **Bit math practice**: "Calculate the GPIO mask for pin 9 using GDB variables"

### Learning Objectives
- Understand memory-mapped I/O through hands-on GDB inspection
- Master basic GDB commands (examine, set, call)
- Learn proper button debouncing patterns
- Claude drives development, you learn by doing

---

## Lesson 02: UART Terminal + Arbitrary Memory Streamer

### Overview
**Focus:** Add UART observability for complex debugging, stream any memory location
**Duration:** 90-120 minutes
**Complexity:** ⭐⭐⭐☆☆
**Hardware:** ESP32-C6-DevKitC-1 + FTDI UART adapter (GPIO16 TX, GPIO17 RX)

### What You'll Build
UART terminal that can stream any memory address to host for analysis:
- Stream GPIO register values
- Monitor button state changes
- Watch NeoPixel color values
- Arbitrary memory window streaming (choose start address + length)

### Embedded Practices
1. **UART DMA** - Non-blocking high-speed streaming
2. **Circular buffers** - Handle streaming without losing data
3. **Memory-safe streaming** - Validate addresses before reading
4. **Structured output** - Machine-parseable format for analysis

### GDB Techniques (3)
1. **Watchpoints** - Break when UART TX buffer overflows
2. **Conditional breakpoints** - Only break on specific memory addresses
3. **Memory compare** - Verify streamed data matches actual memory

### Commit Structure (5 commits)

**Commit 1: Basic UART init**
- Simple UART TX: `uart.write_str("Hello\n")`
- Blocking writes only

**Commit 2: Add DMA for non-blocking streaming**
- Configure UART with DMA
- Stream at high speed without blocking main loop
- Claude guides: "Let's add watchpoint to catch buffer overflow"

**Commit 3: Arbitrary memory streamer**
- Command protocol: `STREAM <addr> <len> <interval_ms>`
- Example: `STREAM 0x60091000 64 100` → Stream 64 bytes from GPIO base every 100ms
- Claude teaches: "Use conditional breakpoints to debug specific addresses"

**Commit 4: Structured output format**
- Output: `[timestamp_ms] ADDR:0x60091000 DATA:0x12345678`
- Machine-parseable for host-side analysis
- Claude shows memory compare: Verify streamed data is correct

**Commit 5: Add safety checks**
- Validate memory address ranges (prevent reading unmapped memory)
- Handle UART errors gracefully
- Add buffer overflow protection

### Demo: Debugging Button State with UART + GDB

**UART streaming:**
```
STREAM 0x6009103C 4 50  # Stream GPIO_IN register every 50ms
[0] ADDR:0x6009103C DATA:0x00000200  # Button not pressed (bit 9 = 1)
[50] ADDR:0x6009103C DATA:0x00000000  # Button pressed (bit 9 = 0)
[100] ADDR:0x6009103C DATA:0x00000200  # Button released
```

**GDB watchpoint:**
```gdb
# Break when GPIO_IN changes
watch *(uint32_t*)0x6009103C

continue
# Hardware watchpoint 1: *(uint32_t*)0x6009103C
# Old value = 512
# New value = 0
```

**Power combo**: UART streams continuously, GDB catches exact moment of change!

### Learning Objectives
- Build production-quality UART terminal for debugging
- Use UART + GDB together (UART for streaming, GDB for precise breakpoints)
- Develop arbitrary memory inspection capability
- Claude teaches watchpoints and memory compare techniques

---

## Lesson 03: State Machine + IMU Sensor Integration

### Overview
**Focus:** Statig state machine library, I2C sensor drivers, event-driven architecture
**Duration:** 120-150 minutes
**Complexity:** ⭐⭐⭐⭐☆
**Hardware:** ESP32-C6 + MPU9250 IMU (I2C on GPIO2 SDA, GPIO11 SCL)

### What You'll Build
Color navigator state machine controlled by button + IMU tilt:
- **State 1 (Idle)**: NeoPixel off
- **State 2 (ColorSelect)**: Button cycles hue (0°→120°→240°→0°)
- **State 3 (BrightnessAdjust)**: IMU tilt controls brightness (tilt left = dim, right = bright)
- UART streams: current state, hue, brightness, IMU readings

### Embedded Practices
1. **Statig state machines** - Macro-based hierarchical FSM
2. **I2C sensor drivers** - MPU9250 accelerometer integration
3. **Event-driven architecture** - Button and IMU generate events
4. **Fixed-point math** - No floats, HSV→RGB conversion with integers

### GDB Techniques (3)
1. **Register diff** - Compare I2C peripheral registers before/after transactions
2. **Tracepoints** - Log all state transitions without stopping
3. **Python GDB script** - Auto-visualize state machine diagram

### Commit Structure (6 commits)

**Commit 1: Add I2C + MPU9250 driver**
- Read WHO_AM_I register (should return 0x71)
- Read accelerometer X/Y/Z values
- Stream to UART: `IMU: x=-512 y=1024 z=16384`

**Commit 2: Define statig state machine**
```rust
#[derive(Debug)]
enum State {
    Idle,
    ColorSelect { hue: u16 },      // hue: 0-359
    BrightnessAdjust { hue: u16, brightness: u8 },  // brightness: 0-255
}

#[derive(Debug)]
enum Event {
    ButtonPress,
    ImuTilt(i16),  // -32768 to 32767 (X-axis)
}
```
- Minimal transitions, no logic yet
- Claude guides: "Let's use tracepoints to log every state transition"

**Commit 3: Implement state transitions**
- Button press: `Idle → ColorSelect → BrightnessAdjust → Idle`
- In ColorSelect: Button cycles hue (0° → 120° → 240° → 0°)
- In BrightnessAdjust: IMU tilt controls brightness

**Commit 4: Add UART state streaming**
- Stream current state, hue, brightness every 100ms
- Example: `[1000] STATE:ColorSelect HUE:120 BRIGHT:0`
- Combined with memory streaming: See state machine + registers

**Commit 5: GDB register diff for I2C debugging**
- Claude teaches: "Let's snapshot I2C registers before/after transaction"
```gdb
# Before I2C read
set $i2c_status_before = *(uint32_t*)0x60013008

# Step through transaction
next

# After
set $i2c_status_after = *(uint32_t*)0x60013008
print/x $i2c_status_before ^ $i2c_status_after  # See what changed
```

**Commit 6: Python GDB state visualizer**
```python
# state_viz.py
import gdb

class StateMonitor(gdb.Breakpoint):
    def __init__(self):
        super().__init__("state_transition")

    def stop(self):
        state = str(gdb.parse_and_eval("current_state"))
        print(f"┌─ STATE CHANGE ─┐")
        print(f"│ {state:^15} │")
        print(f"└────────────────┘")
        return False  # Don't stop execution

StateMonitor()
```
- Auto-prints ASCII diagram on every state change
- "Wow moment": Live state machine visualization!

### UART + GDB Combined Debugging

**UART output (continuous streaming):**
```
[0] STATE:Idle HUE:0 BRIGHT:0 IMU:x=0 y=0 z=16384
[100] STATE:Idle HUE:0 BRIGHT:0 IMU:x=-200 y=50 z=16300
[200] STATE:ColorSelect HUE:0 BRIGHT:0 IMU:x=-250 y=60 z=16280
[300] STATE:ColorSelect HUE:120 BRIGHT:0 IMU:x=-300 y=70 z=16260
[400] STATE:BrightnessAdjust HUE:120 BRIGHT:128 IMU:x=-5000 y=100 z=14000
```

**GDB tracepoint (precise event capture):**
```gdb
# Set tracepoint on state transitions
trace state_transition
actions
  collect current_state
  collect event
  collect hue
  collect brightness
end

tstart
continue

# Later, analyze trace
tstop
tfind start
# Replay all state transitions offline
```

### Learning Objectives
- Build production state machine with statig
- Integrate I2C sensor (MPU9250) with proper error handling
- Combine UART streaming (big picture) + GDB precision (exact moments)
- Claude teaches register diff and Python scripting

---

## Lesson 04: Task Scheduler + Atomics - Splitting the Monolith

### Overview
**Focus:** Refactor monolithic loop into concurrent tasks with lock-free atomics
**Duration:** 120-150 minutes
**Complexity:** ⭐⭐⭐⭐☆
**Hardware:** ESP32-C6 + MPU9250 + UART

### What You'll Build
Split Lesson 03's monolithic state machine into independent tasks:
- **button_task** (10ms) - Read GPIO, generate ButtonPress events
- **imu_task** (50ms) - Read MPU9250, generate ImuTilt events
- **state_machine_task** (20ms) - Process events, update state
- **led_task** (100ms) - Read state, update NeoPixel
- **uart_task** (100ms) - Stream telemetry

All tasks communicate via **lock-free atomics** (no mutexes, no critical sections).

### Embedded Practices
1. **Cooperative task scheduler** - Fixed-interval task execution
2. **Lock-free concurrency** - AtomicU16, AtomicBool for shared state
3. **Task independence** - No task directly calls another
4. **Bounded execution time** - Each task completes in known time

### GDB Techniques (3)
1. **Watchpoints on atomics** - Break when shared state changes
2. **Call stack analysis** - Understand task execution order
3. **Performance profiling** - Measure task execution time

### Commit Structure (7 commits)

**Commit 1: Start with monolithic Lesson 03 code**
- All code in main loop
- State machine, I2C, GPIO, NeoPixel all mixed together
- Hard to reason about timing, hard to test

**Commit 2: Extract task functions (but still call directly)**
```rust
fn button_task(button: &Input, event_queue: &mut Vec<Event>) { }
fn imu_task(i2c: &mut I2C, event_queue: &mut Vec<Event>) { }
fn state_machine_task(events: &[Event], state: &mut State) { }
fn led_task(state: &State, neopixel: &mut NeoPixel) { }
```
- Functions exist, but main loop still calls them directly
- No concurrency yet

**Commit 3: Add simple scheduler**
```rust
struct Task {
    last_run: u64,
    interval_ms: u64,
    func: fn(),
}

fn scheduler_run(tasks: &mut [Task]) {
    for task in tasks {
        if elapsed_since(task.last_run) >= task.interval_ms {
            (task.func)();
            task.last_run = current_time_ms();
        }
    }
}
```
- Tasks run at fixed intervals
- But: shared state via mutable globals → race conditions!

**Commit 4: Replace globals with atomics**
```rust
// Shared state (lock-free!)
static BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);
static IMU_X_AXIS: AtomicI16 = AtomicI16::new(0);
static CURRENT_HUE: AtomicU16 = AtomicU16::new(0);
static CURRENT_BRIGHTNESS: AtomicU8 = AtomicU8::new(0);

// button_task writes
BUTTON_PRESSED.store(true, Ordering::Relaxed);

// state_machine_task reads
if BUTTON_PRESSED.swap(false, Ordering::Relaxed) {
    // Handle button press event
}
```
- No locks, no critical sections, no allocation!
- Claude teaches: "Let's use watchpoints to see atomic changes in real-time"

**Commit 5: GDB watchpoint on atomics**
```gdb
# Find address of CURRENT_HUE
info variables CURRENT_HUE
# Let's say it's at 0x3FC90000

# Set watchpoint
watch *(uint16_t*)0x3FC90000

continue
# Watchpoint 1: *(uint16_t*)0x3FC90000
# Old value = 0
# New value = 120  ← Caught exact moment hue changed!
```

**Commit 6: Call stack analysis**
```gdb
# Break in led_task
break led_task

continue

# Show call stack
backtrace
# #0 led_task()
# #1 scheduler_run()
# #2 main()

# Step up to scheduler
up
# Now at scheduler_run()

# Inspect which task is next
print tasks[1]  # imu_task
```

**Commit 7: Performance profiling**
```rust
// Add timing instrumentation
static TASK_CYCLES: [AtomicU64; 5] = [/* ... */];

fn button_task() {
    let start = read_cycle_counter();

    // Task logic...

    let end = read_cycle_counter();
    TASK_CYCLES[0].store(end - start, Ordering::Relaxed);
}
```

**GDB script to monitor:**
```gdb
define show_task_times
  printf "Task execution times (cycles):\n"
  printf "  button_task:  %lu\n", TASK_CYCLES[0]
  printf "  imu_task:     %lu\n", TASK_CYCLES[1]
  printf "  state_task:   %lu\n", TASK_CYCLES[2]
  printf "  led_task:     %lu\n", TASK_CYCLES[3]
  printf "  uart_task:    %lu\n", TASK_CYCLES[4]
end

# Run periodically
break scheduler_run
commands
  silent
  show_task_times
  continue
end
```

### UART Streaming for Task Monitoring

**Add task telemetry to UART:**
```
[0] TASKS: btn=50us imu=250us state=100us led=300us uart=500us
[100] TASKS: btn=52us imu=248us state=98us led=305us uart=490us
```

**GDB for precise breakpoints:**
- UART shows big picture (all task times)
- GDB catches exact moment a task exceeds budget

### Learning Objectives
- Refactor monolithic code into task-based architecture
- Use atomics for lock-free inter-task communication
- Profile task execution times
- Combine UART (continuous monitoring) + GDB (precise analysis)
- Claude teaches watchpoints, call stack, and profiling

---

## Lesson 05: Virtual HIL Testing Without Hardware

### Overview
**Focus:** Test complete system (all tasks, state machine, "sensors") without ESP32-C6 hardware
**Duration:** 150-240 minutes
**Complexity:** ⭐⭐⭐⭐⭐
**Hardware:** None! Runs on host (macOS/Linux/Windows)

### What You'll Build
Hardware-in-the-Loop (HIL) test framework that simulates:
- **Virtual button** - Inject button press events
- **Virtual IMU** - Provide fake accelerometer readings
- **Virtual NeoPixel** - Capture color commands, verify correctness
- **Virtual UART** - Capture and analyze output
- **Time control** - Speed up/slow down/pause time for deterministic testing

Run full integration tests on host, catch bugs before flashing to hardware!

### Embedded Practices
1. **Hardware abstraction layer (HAL)** - Separate business logic from hardware
2. **Dependency injection** - Pass traits instead of concrete types
3. **Mocking** - Virtual peripherals implement same traits as real hardware
4. **Deterministic testing** - Control time for reproducible tests
5. **CI/CD integration** - Run tests on every commit

### GDB Techniques (3)
1. **Automated test harness** - GDB script runs tests, checks assertions
2. **Reverse debugging** - Step backward to find bug root cause
3. **Record/replay** - Record test execution, replay to find race conditions

### Architecture: HAL Abstraction

**Before (Lesson 04 - tightly coupled to hardware):**
```rust
fn button_task(peripherals: &Peripherals) {
    let state = peripherals.GPIO9.is_low();  // Directly reads hardware
    // ...
}
```

**After (Lesson 05 - abstracted):**
```rust
// Define traits
trait ButtonInput {
    fn is_pressed(&self) -> bool;
}

trait ImuSensor {
    fn read_accel(&mut self) -> (i16, i16, i16);
}

trait LedOutput {
    fn set_color(&mut self, r: u8, g: u8, b: u8);
}

// Task now accepts any type implementing trait
fn button_task<B: ButtonInput>(button: &B, events: &mut EventQueue) {
    if button.is_pressed() {
        events.push(Event::ButtonPress);
    }
}
```

**Real hardware implementation:**
```rust
struct HardwareButton {
    pin: Input<'static>,
}

impl ButtonInput for HardwareButton {
    fn is_pressed(&self) -> bool {
        self.pin.is_low()
    }
}
```

**Virtual implementation (for testing):**
```rust
struct VirtualButton {
    pressed: bool,
}

impl ButtonInput for VirtualButton {
    fn is_pressed(&self) -> bool {
        self.pressed
    }
}
```

### Commit Structure (8 commits)

**Commit 1: Extract HAL traits**
- Define ButtonInput, ImuSensor, LedOutput, UartOutput traits
- Refactor tasks to accept trait objects instead of concrete types

**Commit 2: Implement real hardware wrappers**
```rust
struct HardwareButton { pin: Input<'static> }
struct HardwareImu { i2c: I2C<'static> }
struct HardwareNeoPixel { rmt: RMT }
```
- Firmware still works exactly as before
- But now abstracted!

**Commit 3: Implement virtual hardware (mocks)**
```rust
struct VirtualButton {
    pressed: bool,
}

struct VirtualImu {
    accel_x: i16,
    accel_y: i16,
    accel_z: i16,
}

struct VirtualNeoPixel {
    last_color: (u8, u8, u8),  // Capture what color was set
}
```
- Can run tasks on host!

**Commit 4: Add virtual time control**
```rust
struct VirtualClock {
    current_time_ms: u64,
}

impl VirtualClock {
    fn advance(&mut self, ms: u64) {
        self.current_time_ms += ms;
    }

    fn now(&self) -> u64 {
        self.current_time_ms
    }
}
```
- Deterministic testing: control exactly when tasks run

**Commit 5: Write integration tests**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_press_cycles_hue() {
        // Setup virtual hardware
        let mut button = VirtualButton { pressed: false };
        let mut imu = VirtualImu { accel_x: 0, accel_y: 0, accel_z: 16384 };
        let mut led = VirtualNeoPixel { last_color: (0, 0, 0) };
        let mut clock = VirtualClock { current_time_ms: 0 };

        let mut state = State::Idle;

        // Simulate button press
        button.pressed = true;
        button_task(&button, &mut events);
        state_machine_task(&events, &mut state);

        // Check state changed
        assert_eq!(state, State::ColorSelect { hue: 0 });

        // Press again → hue advances
        button.pressed = false;  // Release
        clock.advance(20);
        button.pressed = true;
        button_task(&button, &mut events);
        state_machine_task(&events, &mut state);

        assert_eq!(state, State::ColorSelect { hue: 120 });
    }

    #[test]
    fn test_imu_tilt_controls_brightness() {
        // Setup
        let mut imu = VirtualImu { accel_x: -16384, accel_y: 0, accel_z: 0 };  // Full left tilt
        let mut led = VirtualNeoPixel { last_color: (0, 0, 0) };

        let mut state = State::BrightnessAdjust { hue: 120, brightness: 128 };

        // Run IMU task
        imu_task(&mut imu, &mut events);
        state_machine_task(&events, &mut state);

        // Brightness should decrease
        if let State::BrightnessAdjust { brightness, .. } = state {
            assert!(brightness < 128);
        }
    }
}
```

**Run on host:**
```bash
cargo test --lib
# All tests run on host, no hardware needed!
```

**Commit 6: GDB automated test harness**
```gdb
# test_harness.gdb
define run_all_tests
  # Run test 1
  break test_button_press_cycles_hue
  run

  # Check result
  if test_failed == 1
    printf "❌ FAILED: test_button_press_cycles_hue\n"
  else
    printf "✅ PASSED: test_button_press_cycles_hue\n"
  end

  # Run test 2
  delete breakpoints  # Clear old breakpoints
  break test_imu_tilt_controls_brightness
  run

  if test_failed == 1
    printf "❌ FAILED: test_imu_tilt_controls_brightness\n"
  else
    printf "✅ PASSED: test_imu_tilt_controls_brightness\n"
  end
end
```

**Run:**
```bash
gdb -batch -x test_harness.gdb target/debug/lesson-05-hil-testing
```

**Commit 7: Reverse debugging with rr**
```bash
# Record test execution
rr record cargo test test_button_press_cycles_hue

# Replay in reverse
rr replay

(gdb) break test_button_press_cycles_hue
(gdb) continue
# Test fails at assertion

(gdb) reverse-continue  # Go backward!
# Now at previous state transition

(gdb) reverse-step
# Step backward through code to find where bug was introduced
```

**Commit 8: CI/CD integration**
```yaml
# .github/workflows/test.yml
name: HIL Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Run HIL tests
        run: cargo test --lib

      - name: Run GDB test harness
        run: |
          gdb -batch -x test_harness.gdb target/debug/lesson-05-hil-testing
```

**Every commit now auto-tested!**

### Virtual HIL + Real Hardware Workflow

**Development cycle:**
1. **Write test first** (TDD) - Define expected behavior with virtual hardware
2. **Run on host** - Fast iteration, no flashing
3. **Fix until test passes** - All logic correct
4. **Flash to real hardware** - Should work first try!
5. **Use UART + GDB** for any remaining hardware-specific issues

**Example:**
```bash
# 1. Write test on host
cargo test test_new_feature  # FAIL - feature not implemented

# 2. Implement feature
vim src/lib.rs

# 3. Test again
cargo test test_new_feature  # PASS

# 4. Flash to hardware
cd lessons/05-hil-testing
cargo run --release --bin real_hardware

# 5. Verify with UART
# UART output matches virtual test expectations

# 6. Done!
```

### Learning Objectives
- Build complete HIL test framework
- Separate business logic from hardware (HAL abstraction)
- Write integration tests that run on host
- Use GDB for automated testing
- Master reverse debugging with rr
- Set up CI/CD pipeline
- Claude teaches: "Test on host first, hardware second"

---

## Curriculum Summary

| Lesson | Focus | What You Build | GDB Techniques | Complexity | Duration |
|--------|-------|----------------|----------------|------------|----------|
| 00 | **Blinky** | Simple LED blink | None (just get it working!) | ⭐☆☆☆☆ | 30-45 min |
| 01 | GDB Basics | Button + NeoPixel | Memory ops, variables, function calls | ⭐⭐☆☆☆ | 60-90 min |
| 02 | UART + Observability | Memory streamer + terminal | Watchpoints, conditional breaks, memory compare | ⭐⭐⭐☆☆ | 90-120 min |
| 03 | State Machine + I2C | Color navigator + IMU | Register diff, tracepoints, Python scripting | ⭐⭐⭐⭐☆ | 120-150 min |
| 04 | Task Scheduler + Atomics | Split into concurrent tasks | Watchpoints on atomics, call stack, profiling | ⭐⭐⭐⭐☆ | 120-150 min |
| 05 | Virtual HIL Testing | Complete test framework | Automated testing, reverse debugging, CI/CD | ⭐⭐⭐⭐⭐ | 150-240 min |

**Total:** 9.5-14 hours (including blinky warm-up)

**Progressive Build:**
- Lesson 00: Blinky (the classic start)
- Lesson 01: Foundation (button + LED + GDB)
- Lesson 02: Add UART observability
- Lesson 03: Add state machine + sensor
- Lesson 04: Refactor into tasks + atomics
- Lesson 05: Test everything without hardware

**Each lesson builds on the previous**, creating a complete production-ready embedded system with professional testing practices!

---

## Key Learning Path

### Lesson 01: GDB Fundamentals
**Claude drives development**:
- "Let's use GDB to inspect the GPIO register"
- "Try this command: `x/1xw 0x6009103C`"
- "Now calculate the bit mask with: `set $mask = 1 << 9`"

**You learn**: Basic GDB, memory-mapped I/O, bit manipulation

---

### Lesson 02: Add UART for More Capability
**Build on Lesson 01**:
- Keep button + NeoPixel working
- Add UART streaming of GPIO register
- Develop arbitrary memory streamer

**Claude teaches**:
- "UART gives you continuous monitoring"
- "GDB gives you precise breakpoints"
- "Together they're powerful!"

**You learn**: UART + GDB combined debugging, DMA, watchpoints

---

### Lesson 03: State Machine + IMU Sensor
**Build on Lesson 02**:
- Keep UART streaming
- Add state machine for color navigation
- Add IMU for tilt sensing
- Stream state + sensor data via UART

**Claude teaches**:
- "Let's use register diff to debug I2C"
- "Tracepoints log without stopping"
- "Python script visualizes state machine"

**You learn**: Statig, I2C drivers, event-driven architecture, GDB scripting

---

### Lesson 04: Split into Tasks + Atomics
**Build on Lesson 03**:
- Same functionality
- But: refactored into independent tasks
- Lock-free communication via atomics

**Claude teaches**:
- "Watch the atomic change with watchpoint"
- "Profile each task's execution time"
- "Call stack shows scheduler flow"

**You learn**: Cooperative scheduling, atomics, performance profiling

---

### Lesson 05: Virtual HIL Testing
**Build on Lesson 04**:
- Extract HAL traits
- Implement virtual hardware
- Write integration tests
- Run on host!

**Claude teaches**:
- "Test business logic without hardware"
- "Reverse debugging finds root cause"
- "Automate tests in CI/CD"

**You learn**: HAL abstraction, mocking, TDD, reverse debugging, CI/CD

---

## Hardware Requirements

| Lesson | Hardware | Notes |
|--------|----------|-------|
| 01 | ESP32-C6-DevKitC-1 | All onboard (GPIO9 button + GPIO8 NeoPixel) |
| 02 | + FTDI UART adapter | GPIO16 TX, GPIO17 RX |
| 03 | + MPU9250 IMU | I2C: GPIO2 SDA, GPIO11 SCL |
| 04 | Same as Lesson 03 | No new hardware |
| 05 | **None!** | Runs on host |

**Total cost**: ~$25 (ESP32-C6 + FTDI + MPU9250)

---

## Why This Flow Works

1. **Lesson 01**: Learn GDB basics with simple hardware (button + LED)
2. **Lesson 02**: Add UART for complex debugging (memory streaming)
3. **Lesson 03**: Add state machine + sensor (real embedded system)
4. **Lesson 04**: Refactor for maintainability (tasks + atomics)
5. **Lesson 05**: Test without hardware (professional workflow)

**Each lesson adds one major concept**, building toward a complete production-ready system with professional testing!

**Claude drives learning**: Teaches GDB techniques as needed, when they're useful for the current problem.

**Real embedded practices**: Atomics, state machines, task scheduling, HAL abstraction, TDD, CI/CD.

**Hardware you have**: ESP32-C6-DevKitC-1, MPU9250 IMU. No servo needed!
