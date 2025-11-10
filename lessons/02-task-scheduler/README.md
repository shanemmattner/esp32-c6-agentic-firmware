# Lesson 02: Task Scheduler with Atomics

This lesson builds on Lesson 01 by splitting the monolithic main loop into **separate tasks** running at independent intervals. You'll learn how to:
- Organize code into reusable task functions
- Use atomic types for lock-free shared state between tasks
- Implement a simple cooperative scheduler
- Keep tasks independent and testable

## Hardware Requirements

- **ESP32-C6-DevKitC-1** development board
- USB-C cable

Same as Lesson 01:
- **Onboard NeoPixel (WS2812)** on GPIO8
- **BOOT button** on GPIO9

---

## Architecture Overview

### Lesson 01 vs Lesson 02

**Lesson 01** (Monolithic):
```
Main Loop
  ├─ Check button
  ├─ Toggle LED state
  ├─ Update NeoPixel
  └─ Debounce delay
```

**Lesson 02** (Task-Based):
```
Scheduler Loop
  ├─ button_task() [every 10ms]
  │  └─ Reads GPIO, detects press, updates LED_ENABLED atomic
  └─ led_task() [every 50ms]
     └─ Reads LED_ENABLED atomic, updates NeoPixel
```

### Key Concepts

**Tasks**: Separate functions that handle independent concerns
- `button_task()` - reads input, updates shared state
- `led_task()` - reads shared state, updates output

**Atomics**: Lock-free shared memory for inter-task communication
- `static LED_ENABLED: AtomicBool` - shared between tasks
- No mutexes, no critical sections, no allocation

**Scheduler**: Simple time-based dispatcher running tasks at different intervals
- Button checked every 10ms (fast, responsive)
- LED updated every 50ms (slower, okay for visual feedback)

---

## Code Walkthrough

### 1. Atomic Shared State

```rust
use core::sync::atomic::{AtomicBool, Ordering};

/// LED state shared between button_task and led_task
static LED_ENABLED: AtomicBool = AtomicBool::new(false);
```

**Why atomics?** We need to safely share state between two tasks without locks. Atomic operations are:
- Lock-free (no mutexes)
- Fast (single CPU instruction on most platforms)
- Safe (Rust prevents data races at compile time)

### 2. Button Task

```rust
fn button_task(button: &Input, delay: &Delay) {
    static mut BUTTON_WAS_PRESSED: bool = false;
    let button_pressed = button.is_low();

    unsafe {
        if button_pressed && !BUTTON_WAS_PRESSED {
            info!("Button press detected!");
            let current = LED_ENABLED.load(Ordering::Relaxed);
            LED_ENABLED.store(!current, Ordering::Relaxed);
        }
        BUTTON_WAS_PRESSED = button_pressed;
    }
}
```

**What's happening:**
1. Store previous button state in static variable
2. Detect press as edge (LOW && was HIGH)
3. Read current LED state with atomic `load()`
4. Toggle and store new state with atomic `store()`
5. Runs every 10ms

**Why `Ordering::Relaxed`?** Single-threaded firmware has no race conditions - relaxed is fastest.

### 3. LED Task

```rust
fn led_task(led: &mut SmartLedsAdapter<..>) {
    let should_be_on = LED_ENABLED.load(Ordering::Relaxed);

    if should_be_on {
        let _ = led.write([RGB8::new(0, 0, 30)].into_iter());
        info!("LED ON");
    } else {
        let _ = led.write([RGB8::new(0, 0, 0)].into_iter());
        info!("LED OFF");
    }
}
```

**What's happening:**
1. Read LED_ENABLED atomic state
2. Update NeoPixel based on state
3. Runs every 50ms

### 4. Scheduler Loop

```rust
let mut button_next_run_ms: u64 = 0;
let mut led_next_run_ms: u64 = 0;
let mut current_time_ms: u64 = 0;

const BUTTON_PERIOD_MS: u64 = 10;
const LED_PERIOD_MS: u64 = 50;
const TICK_MS: u64 = 10;

loop {
    current_time_ms += TICK_MS;
    delay.delay_millis(TICK_MS as u32);

    if current_time_ms >= button_next_run_ms {
        button_task(&button, &delay);
        button_next_run_ms = current_time_ms + BUTTON_PERIOD_MS;
    }

    if current_time_ms >= led_next_run_ms {
        led_task(&mut led);
        led_next_run_ms = current_time_ms + LED_PERIOD_MS;
    }
}
```

**How it works:**
1. Advance virtual time by TICK_MS (10ms per loop)
2. Run button_task if it's due (every 10ms = every loop)
3. Run led_task if it's due (every 50ms = every 5 loops)
4. Simple cooperative scheduler - no interrupts needed

---

## Why This Pattern Matters

**Problem with Lesson 01**: Button debounce blocks everything for 200ms - makes code hard to extend with more features

**Solution with Lesson 02**: Tasks run independently at their own rates
- Button task is fast (10ms) for responsive input
- LED task is slower (50ms) because humans can't see 50ms flicker
- Easy to add more tasks: sensor reading, wireless comms, etc.

**Lock-free communication**: Atomics let tasks share data without blocking
- No `Mutex` (would require `std::sync`)
- No `unsafe` global state corruption (Rust compiler ensures safety)
- Perfect for embedded systems with tight timing constraints

---

## Building and Flashing

### Build the project

```bash
cd lessons/02-task-scheduler
cargo build --release
```

### Flash to ESP32-C6

```bash
cargo run --release
```

Monitor serial output:
```bash
python3 read_serial.py
```

### Expected Behavior

1. Flash completes
2. Device boots and scheduler starts
3. Serial output shows scheduler ticks with periodic task execution
4. Press BOOT button → `Button press detected!` appears in logs
5. LED turns ON (blue) and log shows `LED ON`
6. Press BOOT button again → LED turns OFF and log shows `LED OFF`

---

## Troubleshooting

**Button press not detected:**
- Button uses GPIO9 (BOOT button on devkit)
- Check serial output for task execution logs
- Button task runs every 10ms - responsiveness depends on polling rate

**LED not updating when button pressed:**
- LED task runs every 50ms (slower than button task)
- Check that `LED_ENABLED` atomic is being read/written correctly
- Verify NeoPixel still works (it's the same hardware as Lesson 01)

**No serial output:**
- Run `python3 read_serial.py` from project root
- Check device is flashed: `cargo run --release`
- Board may need manual reset after flashing

**Build fails:**
- Same requirements as Lesson 01 - check dependencies in Cargo.toml
- Ensure `critical-section = "1.2.0"` matches esp-hal 1.0.0

---

## Extending This Pattern

To add a new task (e.g., sensor reading):

1. **Create the task function:**
   ```rust
   fn sensor_task(sensor: &SensorDriver) {
       let reading = sensor.read();
       // Use atomics to share data
       SENSOR_READING.store(reading, Ordering::Relaxed);
   }
   ```

2. **Create a shared atomic:**
   ```rust
   static SENSOR_READING: AtomicU16 = AtomicU16::new(0);
   ```

3. **Add to scheduler:**
   ```rust
   let mut sensor_next_run_ms: u64 = 0;
   const SENSOR_PERIOD_MS: u64 = 100;  // Every 100ms

   if current_time_ms >= sensor_next_run_ms {
       sensor_task(&sensor);
       sensor_next_run_ms = current_time_ms + SENSOR_PERIOD_MS;
   }
   ```

---

## Key Takeaways

✅ **Task-based architecture** scales better than monolithic loops
✅ **Atomic types** provide lock-free inter-task communication
✅ **Cooperative scheduler** is simple and predictable (no interrupts)
✅ **Independent task rates** optimize for different responsiveness needs
✅ **Each task is testable** in isolation

Great job completing Lesson 02! 🎉
