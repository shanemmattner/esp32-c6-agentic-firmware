# Modern Rust Embedded Ecosystem
## esp-hal 1.0.0 + Embassy + State Machines

This document explores the **cutting-edge Rust embedded libraries** that work with esp-hal 1.0.0, focusing on async/await with Embassy and modern state machine patterns.

---

## 🚀 The Modern Stack

```
┌─────────────────────────────────────┐
│      Your Firmware Application      │
├─────────────────────────────────────┤
│  Embassy Async Runtime (optional)   │
├─────────────────────────────────────┤
│     State Machine Libraries         │
│  (statig, smlang, enum-state, etc.) │
├─────────────────────────────────────┤
│   embedded-hal 1.0 Trait Drivers    │
│  (sensor drivers, display, etc.)    │
├─────────────────────────────────────┤
│         esp-hal 1.0.0               │
│    (bare-metal HAL for ESP32)       │
├─────────────────────────────────────┤
│      ESP32-C6 Hardware (RISC-V)     │
└─────────────────────────────────────┘
```

---

## 🌟 Embassy: Async/Await for Embedded

Embassy is a **modern async runtime** for embedded systems. It brings async/await to bare-metal!

### Why Embassy?

- ✅ **No RTOS needed** - Async without FreeRTOS overhead
- ✅ **Zero-cost abstractions** - Compiles to efficient state machines
- ✅ **First-class esp-hal support** - Official integration
- ✅ **Multiple concurrent tasks** - Without threads!
- ✅ **Type-safe timers** - Compile-time timer validation

### Basic Embassy Example
```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::{gpio::{Output, Level, OutputConfig}, timer::timg::TimerGroup};

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Initialize embassy timer
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    let led = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());

    // Spawn async tasks
    spawner.spawn(blink_task(led)).ok();
    spawner.spawn(sensor_task()).ok();
}

#[embassy_executor::task]
async fn blink_task(mut led: Output<'static>) {
    loop {
        led.toggle();
        Timer::after(Duration::from_millis(1000)).await;  // async!
    }
}

#[embassy_executor::task]
async fn sensor_task() {
    loop {
        let temp = read_sensor().await;
        log::info!("Temperature: {}°C", temp);
        Timer::after(Duration::from_secs(5)).await;
    }
}
```

**Key Benefits:**
- Multiple tasks run concurrently without threads
- No priority inversion issues
- Efficient: Tasks only run when needed
- Clean: No callbacks, no mutexes for simple cases

### Embassy Features We'll Explore

1. **Async I2C/SPI**
```rust
// Non-blocking sensor reads!
let data = i2c.read_async(addr, &mut buffer).await?;
```

2. **Async Channels** (message passing between tasks)
```rust
use embassy_sync::channel::Channel;

static CHANNEL: Channel<u32, 10> = Channel::new();

// Task 1: Send
CHANNEL.send(sensor_value).await;

// Task 2: Receive
let value = CHANNEL.receive().await;
```

3. **Async Mutexes** (shared state)
```rust
use embassy_sync::mutex::Mutex;

static SHARED: Mutex<u32> = Mutex::new(0);

async fn task() {
    let mut value = SHARED.lock().await;
    *value += 1;
}
```

4. **Select** (wait on multiple futures)
```rust
use embassy_futures::select::{select, Either};

match select(button.wait_for_press(), Timer::after_secs(10)).await {
    Either::First(_) => info!("Button pressed!"),
    Either::Second(_) => info!("Timeout!"),
}
```

---

## 🔄 State Machine Libraries

State machines are essential for embedded systems. Rust has several excellent libraries:

### 1. **statig** (Type-State Pattern)

You mentioned using this before! statig uses Rust's type system for compile-time state validation.

**Key Features:**
- ✅ Zero-cost abstractions
- ✅ Compile-time state validation
- ✅ Type-safe transitions
- ✅ No runtime overhead

```rust
use statig::prelude::*;

#[derive(Debug, Default)]
pub struct Led;

pub enum Event {
    ButtonPress,
    Timeout,
}

#[state_machine(initial = "State::off()")]
impl Led {
    #[state]
    fn off(event: &Event) -> Response<State> {
        match event {
            Event::ButtonPress => Transition(State::on()),
            _ => Super,
        }
    }

    #[state]
    fn on(event: &Event) -> Response<State> {
        match event {
            Event::ButtonPress => Transition(State::off()),
            Event::Timeout => Transition(State::blinking()),
            _ => Super,
        }
    }

    #[state]
    fn blinking(event: &Event) -> Response<State> {
        match event {
            Event::ButtonPress => Transition(State::off()),
            _ => Super,
        }
    }
}

// Usage
let mut led = Led::default().state_machine().init();
led.handle(&Event::ButtonPress);  // off -> on
```

### 2. **smlang** (Macro-Based DSL)

Clean syntax with macro-based state machine definition.

```rust
use smlang::statemachine;

statemachine! {
    transitions: {
        *Idle + ButtonPress = Blinking,
        Blinking + ButtonPress = Solid,
        Solid + Timeout = Idle,
    }
}

// Auto-generated state machine
let mut sm = StateMachine::new();
assert!(sm.state() == States::Idle);
sm.process_event(Events::ButtonPress).unwrap();
assert!(sm.state() == States::Blinking);
```

### 3. **enum-state** (Enum-Based)

Simple enum-based approach, good for smaller state machines.

```rust
use enum_state::EnumState;

#[derive(EnumState)]
enum LedState {
    Off,
    On,
    Blinking { counter: u32 },
}

impl LedState {
    fn handle_button(&mut self) {
        *self = match self {
            LedState::Off => LedState::On,
            LedState::On => LedState::Blinking { counter: 0 },
            LedState::Blinking { .. } => LedState::Off,
        };
    }
}
```

### 4. **Custom Async State Machines with Embassy**

Combine Embassy async with state machines:

```rust
use embassy_time::{Duration, Timer};

enum State {
    Idle,
    Active,
    Cooldown,
}

#[embassy_executor::task]
async fn state_machine_task() {
    let mut state = State::Idle;

    loop {
        state = match state {
            State::Idle => {
                wait_for_trigger().await;
                log::info!("Idle -> Active");
                State::Active
            }
            State::Active => {
                process_active().await;
                log::info!("Active -> Cooldown");
                State::Cooldown
            }
            State::Cooldown => {
                Timer::after(Duration::from_secs(5)).await;
                log::info!("Cooldown -> Idle");
                State::Idle
            }
        }
    }
}
```

---

## 📋 Planned Lessons

### Lesson 02: Button Input + Simple State Machine
**Focus**: GPIO input, interrupts, basic enum-based state machine
```rust
enum LedMode {
    Off,
    On,
    Blinking,
}
// Toggle between modes with button
```

### Lesson 03: Embassy Async Blinky
**Focus**: Introduction to Embassy async runtime
```rust
#[embassy_executor::task]
async fn blink() {
    loop {
        led.toggle();
        Timer::after_millis(1000).await;
    }
}
```

### Lesson 04: Traffic Light with statig
**Focus**: Type-safe state machines with statig
```rust
#[state_machine]
impl TrafficLight {
    #[state] fn red() -> Response<State>
    #[state] fn yellow() -> Response<State>
    #[state] fn green() -> Response<State>
}
```

### Lesson 05: Multi-Task Embassy
**Focus**: Multiple async tasks, channels, mutexes
```rust
// Task 1: Read sensor
// Task 2: Control LED
// Task 3: Handle button
// All communicating via channels!
```

### Lesson 06: Async I2C Sensor
**Focus**: Async I2C with embedded-hal-async
```rust
#[embassy_executor::task]
async fn sensor_task(mut i2c: I2c<'static, I2C0, Async>) {
    let mut buffer = [0u8; 2];
    loop {
        i2c.read(ADDR, &mut buffer).await.ok();
        Timer::after_secs(1).await;
    }
}
```

---

## 🛠️ Library Comparison

| Library | Type | Best For | Overhead | Learning Curve |
|---------|------|----------|----------|----------------|
| **statig** | Compile-time | Complex FSMs | Zero | Medium |
| **smlang** | Macro DSL | Clear transitions | Minimal | Low |
| **enum-state** | Runtime | Simple states | Low | Very Low |
| **Embassy** | Async runtime | Concurrent tasks | Very Low | Medium |

---

## 🎯 Our Approach

For this repository, we'll explore **all of them**:

1. **Start Simple**: Enum-based state machines
2. **Add Type Safety**: statig for complex logic
3. **Go Async**: Embassy for concurrent tasks
4. **Combine**: Async state machines with Embassy

### Example Progression

**Lesson 01**: Basic GPIO (no state machine)
```rust
loop {
    led.toggle();
    delay.delay_millis(1000);
}
```

**Lesson 02**: Simple state machine
```rust
enum State { Off, On, Blinking }
match state {
    State::Off => { /* ... */ }
    State::On => { /* ... */ }
    State::Blinking => { /* ... */ }
}
```

**Lesson 03**: Type-safe state machine (statig)
```rust
#[state_machine(initial = "State::off()")]
impl Led { /* ... */ }
```

**Lesson 04**: Async state machine (Embassy)
```rust
#[embassy_executor::task]
async fn led_controller() {
    // Async state machine with .await
}
```

---

## 🔬 Testing Patterns

### Testing State Machines
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_state_transitions() {
        let mut sm = LedStateMachine::new();
        assert_eq!(sm.state(), State::Off);

        sm.handle_event(Event::ButtonPress);
        assert_eq!(sm.state(), State::On);

        sm.handle_event(Event::Timeout);
        assert_eq!(sm.state(), State::Blinking);
    }
}
```

### Testing Async Tasks
```rust
#[embassy_executor::test]
async fn test_async_sensor() {
    let mut sensor = MockSensor::new();
    let value = sensor.read().await;
    assert_eq!(value, 25.0);
}
```

---

## 📚 Resources

### Embassy
- **Docs**: https://embassy.dev/
- **Book**: https://embassy.dev/book/
- **Examples**: https://github.com/embassy-rs/embassy/tree/main/examples
- **ESP Integration**: https://github.com/esp-rs/esp-hal/tree/main/examples/async

### State Machines
- **statig**: https://github.com/mdeloof/statig
- **smlang**: https://github.com/korken89/smlang-rs
- **enum-state**: https://crates.io/crates/enum-state

### embedded-hal 1.0
- **Traits**: https://docs.rs/embedded-hal/1.0.0/
- **Async Traits**: https://docs.rs/embedded-hal-async/1.0.0/

---

## 🎬 Next Steps

1. ✅ **Lesson 01 Complete**: Basic blinky with esp-hal 1.0.0
2. 🔨 **Lesson 02**: Button input + enum state machine
3. 🔨 **Lesson 03**: Embassy async introduction
4. 🔨 **Lesson 04**: statig type-safe FSM
5. 🔨 **Lesson 05**: Multi-task async with channels

**Goal**: Demonstrate the full modern Rust embedded stack!

---

**Last Updated**: 2025-11-09
**Focus**: esp-hal 1.0.0 + Embassy + Modern State Machines
