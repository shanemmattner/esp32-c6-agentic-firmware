# Create Embassy Async Task

Generate an async task using Embassy executor for concurrent embedded operations.

## Task

Create a new async task with the following:

1. **Task Name**: {Ask user for task name}
2. **Purpose**: {Ask what the task should do}
3. **Timing**: {Ask about timing requirements: periodic, event-driven, etc.}
4. **Dependencies**: {Ask what peripherals/resources are needed}

## Template

```rust
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::gpio::{Output, Level, OutputConfig};
use log::{info, debug, error};

/// {Task description}
#[embassy_executor::task]
async fn {task_name}_task(/* parameters */) {
    info!("Starting {task_name} task");

    loop {
        // TODO: Task implementation

        // For periodic tasks:
        Timer::after(Duration::from_millis(1000)).await;

        // For event-driven tasks:
        // signal.wait().await;
    }
}

/// Main entry point with Embassy
#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    info!("Initializing firmware with Embassy");

    let peripherals = esp_hal::init(esp_hal::Config::default());

    // Initialize embassy timer
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);

    // Spawn tasks
    spawner.spawn({task_name}_task(/* args */)).ok();

    info!("All tasks spawned successfully");
}
```

## Patterns to Include

### Periodic Task
```rust
#[embassy_executor::task]
async fn periodic_task() {
    loop {
        // Do work
        debug!("Periodic task tick");
        Timer::after(Duration::from_secs(1)).await;
    }
}
```

### Event-Driven Task
```rust
use embassy_sync::signal::Signal;

static SIGNAL: Signal<u32> = Signal::new();

#[embassy_executor::task]
async fn event_task() {
    loop {
        let value = SIGNAL.wait().await;
        info!("Received event: {}", value);
    }
}
```

### Communication Between Tasks (Channel)
```rust
use embassy_sync::channel::Channel;

static CHANNEL: Channel<SensorData, 10> = Channel::new();

#[embassy_executor::task]
async fn producer_task() {
    loop {
        let data = read_sensor();
        CHANNEL.send(data).await;
    }
}

#[embassy_executor::task]
async fn consumer_task() {
    loop {
        let data = CHANNEL.receive().await;
        process_data(data);
    }
}
```

### Shared State (Mutex)
```rust
use embassy_sync::mutex::Mutex;

static SHARED_STATE: Mutex<State> = Mutex::new(State::default());

#[embassy_executor::task]
async fn modifier_task() {
    loop {
        let mut state = SHARED_STATE.lock().await;
        state.update();
    }
}
```

## Dependencies to Add

```toml
[dependencies]
embassy-executor = "0.7"
embassy-time = "0.4"
embassy-sync = "0.6"
esp-hal-embassy = { version = "1.0", features = ["esp32c6"] }
```

## Testing

```rust
#[embassy_executor::test]
async fn test_{task_name}() {
    // Setup
    let mock_peripheral = MockPeripheral::new();

    // Run task for some iterations
    for _ in 0..10 {
        // Task logic
        Timer::after_millis(100).await;
    }

    // Assert expectations
}
```

## Next Steps

1. Implement task logic
2. Add error handling
3. Add logging at appropriate levels
4. Test with other tasks
5. Monitor with `cargo run --release`
