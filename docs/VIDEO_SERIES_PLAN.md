# YouTube Video Series Plan
## ESP32-C6 Rust with esp-hal 1.0.0 + Embassy

Focus: Keep it **simple** and **practical**. Show the modern way to do ESP32 Rust development.

---

## Video 1: "ESP32-C6 Rust Blinky - The Modern Way (esp-hal 1.0.0)"
**Duration**: 15-20 minutes
**Goal**: Get viewers from zero to blinking LED with pure Rust

### Outline

**Part 1: Why esp-hal 1.0.0? (3 min)**
- Quick comparison: esp-idf-hal (old) vs esp-hal 1.0.0 (new)
- Show the difference:
  - Old: C dependencies, ESP-IDF required, complex setup
  - New: Pure Rust, cargo install, simple
- Why it matters: Official support, smaller binaries, modern patterns

**Part 2: Project Setup (5 min)**
```bash
# Show exactly these steps on screen
rustup target add riscv32imac-unknown-none-elf
cargo install espflash

# Create project from scratch (not template!)
cargo new --bin esp32c6-blinky
cd esp32c6-blinky
```

**Part 3: Minimal Cargo.toml (3 min)**
```toml
[package]
name = "blinky"
version = "0.1.0"
edition = "2024"

[dependencies]
esp-hal = { version = "1.0.0", features = ["esp32c6", "rt"] }
esp-backtrace = { version = "0.18", features = ["esp32c6", "panic-handler", "println"] }
esp-println = { version = "0.16", features = ["esp32c6"] }

[profile.release]
opt-level = "s"
```

**Part 4: Minimal main.rs (5 min)**
```rust
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{delay::Delay, gpio::{Level, Output, OutputConfig}, main};

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut led = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());
    let delay = Delay::new();

    loop {
        led.toggle();
        delay.delay_millis(1000);
    }
}
```

**Part 5: Build & Flash (4 min)**
- Show .cargo/config.toml setup
- cargo build --release
- espflash flash --monitor
- See LED blinking!

**Key Takeaways:**
- Only 20 lines of code!
- No ESP-IDF installation
- Pure Rust, modern patterns
- This is the 2024+ way

---

## Video 2: "Async Embedded with Embassy - No RTOS Needed!"
**Duration**: 20-25 minutes
**Goal**: Show why async is better than traditional RTOS approach

### Outline

**Part 1: Why Async? (4 min)**
- Traditional approach: FreeRTOS tasks with mutexes, priority issues
- Modern approach: async/await with zero-cost abstractions
- Show the mental model:
  ```
  RTOS Thread → High overhead, priority inversion
  Async Task  → Compiler-generated state machine, efficient
  ```

**Part 2: Convert Blinky to Async (6 min)**

**Before (blocking):**
```rust
loop {
    led.toggle();
    delay.delay_millis(1000);  // Blocks!
}
```

**After (async):**
```rust
#[embassy_executor::task]
async fn blink_task(mut led: Output<'static>) {
    loop {
        led.toggle();
        Timer::after_millis(1000).await;  // Yields!
    }
}
```

**Part 3: Multiple Concurrent Tasks (8 min)**
```rust
#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Initialize embassy timer
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    let led1 = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());
    let led2 = Output::new(peripherals.GPIO9, Level::Low, OutputConfig::default());

    // Run multiple tasks concurrently!
    spawner.spawn(fast_blink(led1)).ok();
    spawner.spawn(slow_blink(led2)).ok();
}

#[embassy_executor::task]
async fn fast_blink(mut led: Output<'static>) {
    loop {
        led.toggle();
        Timer::after_millis(250).await;
    }
}

#[embassy_executor::task]
async fn slow_blink(mut led: Output<'static>) {
    loop {
        led.toggle();
        Timer::after_millis(1000).await;
    }
}
```

**Part 4: Task Communication with Channels (7 min)**
```rust
use embassy_sync::channel::Channel;

static CHANNEL: Channel<bool, 1> = Channel::new();

#[embassy_executor::task]
async fn button_task(mut button: Input<'static>) {
    loop {
        button.wait_for_falling_edge().await;
        CHANNEL.send(true).await;  // Send event
    }
}

#[embassy_executor::task]
async fn led_task(mut led: Output<'static>) {
    loop {
        let toggle = CHANNEL.receive().await;  // Wait for event
        if toggle {
            led.toggle();
        }
    }
}
```

**Key Takeaways:**
- Multiple tasks without RTOS overhead
- Clean async/await syntax
- Type-safe communication with channels
- This is how modern embedded systems are built!

---

## Video 3: "State Machines with Embassy - Practical Patterns"
**Duration**: 15-20 minutes
**Goal**: Show how to manage state in async tasks (no statig needed!)

### Outline

**Part 1: Simple Enum State Machine (5 min)**
```rust
enum SystemState {
    Idle,
    Active,
    Error,
}

#[embassy_executor::task]
async fn system_controller() {
    let mut state = SystemState::Idle;

    loop {
        match state {
            SystemState::Idle => {
                // Wait for activation
                state = wait_for_trigger().await;
            }
            SystemState::Active => {
                // Do work
                state = do_work().await;
            }
            SystemState::Error => {
                // Handle error, retry
                Timer::after_secs(5).await;
                state = SystemState::Idle;
            }
        }
    }
}
```

**Part 2: State with Data (5 min)**
```rust
enum LedMode {
    Off,
    Solid,
    Blinking { interval_ms: u32, count: u32 },
}

#[embassy_executor::task]
async fn led_controller(mut led: Output<'static>) {
    let mut mode = LedMode::Off;

    loop {
        mode = match mode {
            LedMode::Off => {
                led.set_low();
                wait_for_command().await
            }
            LedMode::Solid => {
                led.set_high();
                wait_for_command().await
            }
            LedMode::Blinking { interval_ms, mut count } => {
                led.toggle();
                Timer::after_millis(interval_ms).await;
                count -= 1;
                if count == 0 {
                    LedMode::Off
                } else {
                    LedMode::Blinking { interval_ms, count }
                }
            }
        }
    }
}
```

**Part 3: Real Example - Traffic Light (10 min)**
```rust
enum TrafficLightState {
    Red,
    Yellow,
    Green,
}

#[embassy_executor::task]
async fn traffic_light(
    mut red: Output<'static>,
    mut yellow: Output<'static>,
    mut green: Output<'static>,
) {
    let mut state = TrafficLightState::Red;

    loop {
        // Set LEDs based on state
        match state {
            TrafficLightState::Red => {
                red.set_high();
                yellow.set_low();
                green.set_low();
                Timer::after_secs(5).await;
                state = TrafficLightState::Green;
            }
            TrafficLightState::Yellow => {
                red.set_low();
                yellow.set_high();
                green.set_low();
                Timer::after_secs(2).await;
                state = TrafficLightState::Red;
            }
            TrafficLightState::Green => {
                red.set_low();
                yellow.set_low();
                green.set_high();
                Timer::after_secs(5).await;
                state = TrafficLightState::Yellow;
            }
        }
    }
}
```

**Key Takeaways:**
- Simple enums work great for most state machines
- Embassy async makes state transitions clean
- No complex libraries needed for common patterns
- Easy to understand and maintain

---

## Video 4: "Async I2C Sensor Reading"
**Duration**: 20 minutes
**Goal**: Show real-world sensor integration

- Async I2C initialization
- Non-blocking sensor reads
- Error handling patterns
- Logging best practices

---

## Video 5: "Building a Complete IoT Device"
**Duration**: 25-30 minutes
**Goal**: Bring it all together

- Multiple async tasks
- WiFi integration (esp-wifi 1.0)
- State management
- Complete working project

---

## 📊 Why This Approach?

### **Simpler Stack:**
```
Your App (state enums) ← Just use simple enums!
↓
Embassy Async Runtime  ← All you need for concurrency
↓
esp-hal 1.0.0         ← Pure Rust HAL
```

**No statig needed!** Keep it simple:
- Enums for state
- Embassy for concurrency
- esp-hal for hardware

### **Key Messages:**

1. **esp-hal 1.0.0 is the modern way** (not esp-idf-hal)
2. **Embassy replaces RTOS** (async > threads)
3. **Simple enums work** (no complex FSM libraries needed)
4. **Pure Rust all the way** (no C dependencies)

---

## 🎯 Target Audience

- Embedded developers curious about Rust
- Rust developers wanting to learn embedded
- Anyone tired of C++ and FreeRTOS complexity

---

## 📝 Talking Points

**Video 1:**
- "This is the official way to do ESP32 Rust now"
- "20 lines of code, no ESP-IDF installation"
- "Pure Rust, officially supported by Espressif"

**Video 2:**
- "Async/await without RTOS overhead"
- "Multiple tasks, zero-cost abstractions"
- "This is the future of embedded development"

**Video 3:**
- "You don't need complex state machine libraries"
- "Simple enums + async = powerful patterns"
- "Easy to understand, easy to maintain"

---

## 🚀 Production Quality Tips

1. **Show the code editor**: Real typing, not just voiceover
2. **Terminal output**: Always visible, shows real compilation
3. **LED blink visual**: Camera on the devkit
4. **Comparisons**: Side-by-side old vs new way
5. **GitHub link**: Always in description

---

**Key Insight**: You don't need statig! Embassy + enums handles 95% of real-world state management beautifully.
