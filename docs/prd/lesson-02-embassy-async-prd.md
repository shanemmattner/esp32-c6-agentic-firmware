# Lesson 02: Embassy + Async Tasks - PRD

## Overview
- **Lesson Number**: 02
- **Feature**: Async/await with Embassy
- **Duration**: 30 min
- **Difficulty**: Beginner
- **Prerequisites**: Lesson 01 (Blinky)

## Learning Objectives
1. Replace blocking code with async/await
2. Spawn concurrent tasks with Embassy
3. Use Embassy timers instead of blocking delays
4. Understand task-based concurrency

## Hardware Requirements
- ESP32-C6 development board
- Same as Lesson 01: GPIO13 and GPIO9 (no additional hardware)

**Pin Configuration**:
- GPIO13: Output (LED)
- GPIO9: Input (reads GPIO13 state)

## Software Requirements
- esp-hal 1.0.0 with features: ["esp32c6", "unstable", "embassy"]
- embassy-executor for task spawning
- embassy-time for async delays
- Standard logging (esp-println + log)

## Expected Behavior

### Serial Output Patterns
```
🚀 Starting Lesson 02: Embassy Async Tasks
✓ GPIO13 configured as output
✓ GPIO9 configured as input
✓ Spawning async tasks...
✓ Tasks running

[Blink Task] LED ON
[Monitor Task] GPIO9: HIGH
[Blink Task] LED OFF
[Monitor Task] GPIO9: LOW
[Blink Task] LED ON
[Monitor Task] GPIO9: HIGH
```

**Critical patterns**:
- ✅ Both tasks print messages (proves concurrency)
- ✅ Blink task toggles every 500ms
- ✅ Monitor task samples every 100ms (faster than blink)
- ✅ No blocking - both tasks run simultaneously
- ❌ No panics or errors

## Functional Requirements

**REQ-1**: Convert Lesson 01 blocking code to async
- Replace `delay.delay_millis()` with `Timer::after().await`
- Replace main loop with executor + tasks

**REQ-2**: Two concurrent tasks
- **Blink task**: Toggle GPIO13 every 500ms
- **Monitor task**: Read GPIO9 every 100ms

**REQ-3**: Both tasks run independently
- No blocking delays
- Each task has own async loop
- Embassy executor schedules both

## Technical Specifications

**Timing**:
- Blink rate: 500ms per state (1Hz toggle)
- Monitor rate: 100ms sampling (10Hz)

**Memory**:
- Minimal heap usage (Embassy uses static memory)
- Two task stacks

**Error Handling**:
- Tasks never return (infinite loops)
- Panic handler for unrecoverable errors

## Implementation Plan

### Code Structure
```rust
// Pin configuration
const LED_PIN: u8 = 13;
const INPUT_PIN: u8 = 9;

// Async tasks
#[embassy_executor::task]
async fn blink_task(mut led: Output<'static>) { ... }

#[embassy_executor::task]
async fn monitor_task(input: Input<'static>) { ... }

// Main with embassy executor
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Init peripherals
    // Spawn tasks
    // Executor runs forever
}
```

### Key Implementation Points
1. Add embassy dependencies to Cargo.toml
2. Use `#[embassy_executor::main]` macro
3. Convert peripherals to `'static` lifetime for tasks
4. Use `Timer::after(Duration::from_millis(ms)).await` for delays
5. Spawn tasks with `spawner.spawn(task).unwrap()`

### Logging Strategy
- **info!()**: Startup, task spawn
- **debug!()**: Each task iteration (blink/monitor actions)

## Testing Requirements

### Unit Tests
- Not applicable (async tasks require executor)

### On-Device Tests
- Test 1: Both tasks run concurrently
- Test 2: GPIO state changes detected
- Test 3: Tasks don't panic under load

## Success Criteria
- [x] Code builds with embassy features
- [x] Both tasks spawn successfully
- [x] Serial output shows interleaved task messages
- [x] GPIO13 toggles at 500ms intervals
- [x] GPIO9 readings match GPIO13 state
- [x] No blocking code (no `delay.delay_millis()`)

## Edge Cases
1. **Task spawn failure**: Should panic immediately (unwrap is fine)
2. **GPIO initialization failure**: Handled by esp-hal
3. **Timer drift**: Embassy handles precise timing

## References
- [Embassy Book](https://embassy.dev/book/)
- [esp-hal Embassy Examples](https://github.com/esp-rs/esp-hal/tree/main/examples)
- [Embassy Executor](https://docs.embassy.dev/embassy-executor/)

---

**Status**: Ready for implementation
**Next Steps**: Create simple project structure and implement
