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

## Project Structure

The code is organized into modules for maintainability and reusability:

```
lessons/02-task-scheduler/
├── src/
│   ├── lib.rs              # Library root: constants, atomics, module declarations
│   ├── scheduler.rs        # Scheduler implementation
│   ├── button.rs          # Button task and logic
│   ├── neopixel.rs        # NeoPixel/LED task and logic
│   └── bin/
│       └── main.rs        # Main entry point
├── Cargo.toml
└── README.md
```

### Module Breakdown

**`lib.rs`** - Configuration and shared state
- Hardware pin definitions (`BUTTON_GPIO = 9`, `NEOPIXEL_GPIO = 8`)
- Timing constants (`BUTTON_PERIOD_MS`, `LED_PERIOD_MS`, `TICK_MS`)
- LED color constants (`LED_COLOR_ON`, `LED_COLOR_OFF`)
- Atomic shared state (`LED_ENABLED: AtomicBool`)
- Helper functions (`is_led_enabled()`, `set_led_enabled()`, `toggle_led_enabled()`)

**`scheduler.rs`** - Cooperative scheduler
- `Scheduler` struct tracking virtual time and task intervals
- `tick()` method running tasks at their configured periods

**`button.rs`** - Button input handling
- `button_task()` - Reads GPIO, detects press edges, toggles LED state

**`neopixel.rs`** - NeoPixel output
- `led_task()` - Reads LED state from atomic, updates NeoPixel color

**`main.rs`** - Application entry point
- Peripheral initialization
- Creates scheduler instance
- Main loop calling `scheduler.tick()`

### Benefits of This Structure

✅ **Modularity** - Each module has a single responsibility
✅ **Testability** - Tasks can be tested independently
✅ **Reusability** - Modules can be reused in other lessons
✅ **Scalability** - Easy to add new tasks and modules
✅ **Constants in one place** - Hardware config centralized in `lib.rs`

---

## Code Walkthrough

### 1. Configuration Constants (lib.rs)

All hardware and timing configuration is centralized in `lib.rs`:

```rust
// Hardware pins
pub const BUTTON_GPIO: u8 = 9;
pub const NEOPIXEL_GPIO: u8 = 8;
pub const RMT_CLOCK_MHZ: u32 = 80;

// Task timing
pub const BUTTON_PERIOD_MS: u64 = 10;   // Check button every 10ms
pub const LED_PERIOD_MS: u64 = 50;      // Update LED every 50ms
pub const TICK_MS: u64 = 10;            // Scheduler tick
pub const DEBOUNCE_MS: u32 = 200;       // Debounce delay

// LED colors
pub const LED_COLOR_ON: (u8, u8, u8) = (0, 0, 30);   // Dim blue
pub const LED_COLOR_OFF: (u8, u8, u8) = (0, 0, 0);   // Off
```

**Why constants?** Centralizing configuration makes it easy to:
- Change pin assignments without touching task code
- Tune timing parameters in one place
- Adjust LED colors without rebuilding modules

### 2. Atomic Shared State (lib.rs)

```rust
use core::sync::atomic::{AtomicBool, Ordering};

/// LED state shared between button_task and led_task
pub static LED_ENABLED: AtomicBool = AtomicBool::new(false);

// Helper functions for cleaner atomic access
pub fn is_led_enabled() -> bool {
    LED_ENABLED.load(Ordering::Relaxed)
}

pub fn toggle_led_enabled() {
    let current = LED_ENABLED.load(Ordering::Relaxed);
    LED_ENABLED.store(!current, Ordering::Relaxed);
}
```

#### What is Lock-Free Shared State?

**Lock-free** means multiple tasks can access shared data **without blocking each other** using locks/mutexes. Atomic operations guarantee that reads and writes happen as single, indivisible operations.

**Example: What happens without atomics?**

```rust
// ❌ UNSAFE - Data race possible
static mut LED_STATE: bool = false;

// Task 1: Button task
LED_STATE = !LED_STATE;  // Read, invert, write (3 steps)

// Task 2: LED task
if LED_STATE { ... }     // Read (might see partial update!)
```

**Problem:** If task 2 reads `LED_STATE` while task 1 is writing it, you get undefined behavior (data race).

**Example: With atomics (our solution):**

```rust
// ✅ SAFE - Lock-free atomic operations
static LED_ENABLED: AtomicBool = AtomicBool::new(false);

// Task 1: Button task
let current = LED_ENABLED.load(Ordering::Relaxed);  // Atomic read
LED_ENABLED.store(!current, Ordering::Relaxed);     // Atomic write

// Task 2: LED task
let state = LED_ENABLED.load(Ordering::Relaxed);    // Atomic read
if state { ... }
```

**Why This is Safe:**

1. **Atomic guarantee:** Each `load()` and `store()` is a single CPU instruction - cannot be interrupted mid-operation
2. **No partial updates:** Task 2 will always read either the old value OR the new value, never something in-between
3. **No blocking:** Tasks never wait for locks - they just read/write immediately
4. **Rust compile-time safety:** The compiler prevents direct access to the bool inside `AtomicBool`, forcing you to use safe atomic operations

**Why Perfect for Embedded:**
- **Fast:** Single instruction (vs mutex = interrupts disabled + context switching)
- **No allocation:** Works in `no_std` environments (no heap needed)
- **Predictable:** No deadlocks, no priority inversion
- **Simple:** Just `load()` and `store()` - no lock/unlock dance

**Memory Ordering (`Ordering::Relaxed`):**

We use `Relaxed` ordering because:
- Single-threaded scheduler (no true parallelism on single-core ESP32-C6)
- No dependencies between atomic operations
- Fastest option - no memory barriers needed

For multi-core systems with complex dependencies, you'd use stricter orderings like `Acquire`/`Release`.

### 3. Button Task (button.rs) - Non-Blocking Debounce

```rust
static mut BUTTON_WAS_PRESSED: bool = false;
static mut DEBOUNCE_COUNTER: u32 = 0;

const DEBOUNCE_CALLS: u32 = (DEBOUNCE_MS as u64 / BUTTON_PERIOD_MS) as u32;
// 200ms / 10ms = 20 calls

pub fn button_task(button: &Input) {
    let button_pressed = button.is_low();

    unsafe {
        // If in debounce period, just decrement counter and return
        if DEBOUNCE_COUNTER > 0 {
            DEBOUNCE_COUNTER -= 1;
            BUTTON_WAS_PRESSED = button_pressed;
            return;  // Don't process button press
        }

        // Detect press as edge (LOW && was HIGH)
        if button_pressed && !BUTTON_WAS_PRESSED {
            info!("Button press detected!");
            toggle_led_enabled();
            DEBOUNCE_COUNTER = DEBOUNCE_CALLS;  // Start debounce period
        }

        BUTTON_WAS_PRESSED = button_pressed;
    }
}
```

**What's happening:**
1. Check if in debounce period - if yes, decrement counter and return early
2. Detect press as edge (LOW && was HIGH)
3. Call `toggle_led_enabled()` helper function
4. Start debounce period by setting counter (20 calls = 200ms)
5. Update previous state for next iteration

**Why non-blocking debounce?**
- Old approach: `delay.delay_millis(200)` blocks the entire scheduler
- New approach: Use a counter that decrements each call (every 10ms)
- Scheduler keeps running other tasks during debounce period
- LED task continues updating every 50ms
- More responsive system overall

### 4. LED Task (neopixel.rs)

```rust
pub fn led_task<'a>(led: &mut SmartLedsAdapter<...>) {
    let should_be_on = is_led_enabled();  // Use helper function

    if should_be_on {
        let (r, g, b) = LED_COLOR_ON;  // Use constant
        let _ = led.write([RGB8::new(r, g, b)].into_iter());
        info!("LED ON");
    } else {
        let (r, g, b) = LED_COLOR_OFF;  // Use constant
        let _ = led.write([RGB8::new(r, g, b)].into_iter());
        info!("LED OFF");
    }
}
```

**What's happening:**
1. Read LED state using `is_led_enabled()` helper
2. Use color constants from `lib.rs`
3. Update NeoPixel accordingly

### 5. Scheduler (scheduler.rs)

```rust
pub struct Scheduler {
    current_time_ms: u64,
    button_next_run_ms: u64,
    led_next_run_ms: u64,
}

impl Scheduler {
    pub fn tick<F1, F2>(&mut self, delay: &Delay, mut button_task: F1, mut led_task: F2)
    where
        F1: FnMut(),
        F2: FnMut(),
    {
        self.current_time_ms += TICK_MS;
        delay.delay_millis(TICK_MS as u32);

        if self.current_time_ms >= self.button_next_run_ms {
            button_task();
            self.button_next_run_ms = self.current_time_ms + BUTTON_PERIOD_MS;
        }

        if self.current_time_ms >= self.led_next_run_ms {
            led_task();
            self.led_next_run_ms = self.current_time_ms + LED_PERIOD_MS;
        }
    }
}
```

**How it works:**
1. Advance virtual time by `TICK_MS`
2. Run tasks when their period elapses
3. Uses `FnMut` closures to allow mutable borrows (needed for LED driver)
4. Timing constants imported from `lib.rs`

### 6. Main Loop (main.rs)

```rust
let mut scheduler = Scheduler::new();

loop {
    scheduler.tick(
        &delay,
        || button::button_task(&button, &delay),
        || neopixel::led_task(&mut led),
    );
}
```

**Clean and simple** - just pass closures to the scheduler!

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

Lesson 02 complete.
