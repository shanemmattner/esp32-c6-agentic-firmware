# Lesson 02: Simple Task Scheduler

Cooperative task scheduler using manual time tracking and function pointers.

## Learning Objectives

- Build a simple bare-metal task scheduler
- Use function pointers for task definitions
- Track time manually with delays
- Run multiple tasks at different periods
- Organize firmware code for scalability
- Separate hardware context from task logic

## Hardware Requirements

- ESP32-C6 development board
- USB-C cable
- Optional: LED + resistor connected to GPIO13

### Pin Configuration

```
ESP32-C6
--------
GPIO13  -->  LED (output - blinks)
GPIO9   -->  Input (monitors GPIO13 state)
```

**Note**: Same wiring as Lesson 01, but now with structured task system!

## What You'll Learn

This lesson demonstrates:
- Cooperative task scheduling (polling-based)
- Manual time tracking with tick counter
- Task struct pattern with period and last_run tracking
- Context pattern for sharing hardware between tasks
- Function pointers for task callbacks
- Foundation for future firmware architecture

## Build & Flash

```bash
cd lessons/02-simple-tasks

# Build
cargo build --release

# Flash to ESP32-C6
cargo run --release
```

### Using Cargo Aliases

```bash
cargo br   # build release
cargo ck   # check syntax only
cargo ff   # flash firmware (build + flash + monitor)
```

## Expected Output

When you flash and run this lesson, you should see:

```
🚀 Starting Lesson 02: Simple Task Scheduler

✓ GPIO13 configured as output
✓ GPIO9 configured as input
✓ Task scheduler ready

🔄 Starting task scheduler loop...

[Blink] LED OFF
[Monitor] GPIO9: LOW
[Monitor] GPIO9: LOW
[Monitor] GPIO9: LOW
[Monitor] GPIO9: LOW
[Blink] LED ON
[Monitor] GPIO9: HIGH
[Monitor] GPIO9: HIGH
[Monitor] GPIO9: HIGH
[Monitor] GPIO9: HIGH
[Blink] LED OFF
...
```

**Note the timing**: Monitor task runs 5x for every blink (100ms vs 500ms periods).

## Code Structure

- `src/main.rs` - Main firmware with task system (~150 lines)
  - Pin configuration constants
  - Task struct and Context struct
  - Task functions (blink_task, monitor_task)
  - Main loop with scheduler
- `Cargo.toml` - Project dependencies
- `.cargo/config.toml` - Build configuration with espflash runner
- `rust-toolchain.toml` - Rust toolchain specification
- `build.rs` - Build script for linker configuration

## Key Concepts

### Task Structure

```rust
struct Task {
    run: fn(&mut Context),  // Function pointer to task
    period_ms: u64,         // How often to run (milliseconds)
    last_run: u64,          // Last execution timestamp
}
```

Each task tracks when it should run next based on its period.

### Context Pattern

```rust
struct Context {
    led: Output<'static>,
    input: Input<'static>,
}
```

Hardware references are stored in a Context struct and passed to each task. This keeps tasks pure and testable.

### Manual Time Tracking

```rust
let mut current_time_ms: u64 = 0;
const TICK_MS: u64 = 10;

loop {
    delay.delay_millis(TICK_MS as u32);
    current_time_ms += TICK_MS;
    // Check tasks...
}
```

Simple 10ms tick system. Not perfect timing, but good enough for many applications and very simple.

### Cooperative Scheduling

```rust
for task in &mut tasks {
    if task.should_run(current_time_ms) {
        task.execute(current_time_ms, &mut ctx);
    }
}
```

Tasks run one after another (cooperative). No interrupts or preemption. Simple and predictable.

### Task Functions

```rust
fn blink_task(ctx: &mut Context) {
    ctx.led.toggle();
    let state = if ctx.led.is_set_high() { "ON" } else { "OFF" };
    info!("[Blink] LED {}", state);
}
```

Tasks are just functions that take a Context reference. Easy to write and test.

## Why This Architecture?

**Pros**:
- ✅ Simple and understandable (~150 lines)
- ✅ Easy to add new tasks (just add to array)
- ✅ No async/await complexity
- ✅ Predictable execution order
- ✅ Good foundation for learning

**Cons**:
- ❌ Not suitable for hard real-time requirements
- ❌ Tasks must complete quickly (no blocking)
- ❌ Timing accuracy depends on task execution time
- ❌ No priority system

**When to use**:
- Learning embedded Rust
- Simple periodic tasks
- Prototyping
- Non-critical timing requirements

**When to upgrade**:
- Need precise timing → Use hardware timers with interrupts
- Need concurrency → Use Embassy async runtime
- Need preemption → Use RTOS (FreeRTOS, etc.)

## Experiments

### Easy
1. Add a third task that prints uptime every 2 seconds
2. Change the blink period to 1000ms
3. Add a counter to monitor_task showing execution count

### Medium
4. Add a task that counts cycles and prints every 100 cycles
5. Modify tasks to run at 100ms, 500ms, and 1000ms periods
6. Create a pattern where LED blinks 3 times, then pauses

### Advanced
7. Add task execution time tracking (measure how long each task takes)
8. Implement task priority (some tasks run before others)
9. Add ability to enable/disable tasks at runtime

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Build fails | Ensure you're in `lessons/02-simple-tasks/` directory |
| Tasks not running | Check that task periods are > 0 |
| Timing seems off | This is normal - timing accuracy depends on task execution time |
| GPIO9 reads wrong | Normal if tasks take time - cooperative scheduling means delays |

## Comparison with Lesson 01

**Lesson 01** (Simple):
- Single main loop
- Direct hardware control
- ~80 lines of code
- Good for learning basics

**Lesson 02** (Task System):
- Multiple tasks with different periods
- Hardware abstracted into Context
- ~150 lines of code
- Good foundation for complex firmware

## Next Steps

- **Lesson 03**: Hardware timer interrupts for precise timing
- **Lesson 04**: Embassy async/await for concurrent tasks
- **Lesson 05**: I2C sensor integration with task system
- Experiment: Add your own tasks (UART communication, ADC reading, etc.)

## References

- [esp-hal Delay Module](https://docs.esp-rs.org/esp-hal/esp-hal/0.20.1/esp32c6/esp_hal/delay/index.html)
- [Function Pointers in Rust](https://doc.rust-lang.org/book/ch19-05-advanced-functions-and-closures.html)
- [ESP32-C6 Technical Reference](https://www.espressif.com/sites/default/files/documentation/esp32-c6_technical_reference_manual_en.pdf)

---

*A simple task scheduler - foundation for scalable firmware!* 🚀
