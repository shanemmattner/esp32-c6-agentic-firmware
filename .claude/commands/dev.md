---
description: Complete ESP32-C6 lesson development workflow - from PRD to hardware-validated PR
---

# /dev - ESP32-C6 Lesson Development Workflow

**Purpose**: End-to-end lesson development for ESP32-C6 firmware using esp-hal 1.0.0, with autonomous iteration, probe-rs debugging, and hardware validation.

**Use when**: Creating new lessons, implementing firmware features, or developing HAL peripheral drivers that require systematic development with PRD, tests, and iterative hardware validation.

---

## Workflow Overview

```
/dev "Create Lesson 03: I2C sensor driver"
  ├─ Phase 1: Generate PRD (research → ask questions → document expected behavior)
  │   └─ STOP: User reviews PRD
  ├─ Phase 2: Project Setup (create directory, Cargo.toml, config)
  ├─ Phase 3: Implementation (autonomous iterative development)
  │   ├─ Write code with strategic logging
  │   ├─ Build → Flash → Monitor → Compare → Fix (loop up to 5x)
  │   ├─ Use probe-rs debugging when needed
  │   └─ STOP if stuck after 5 iterations
  ├─ Phase 4: Testing (unit tests + defmt-test on-device)
  ├─ Phase 5: Hardware Validation (interactive - user confirms behavior)
  │   └─ STOP: User validates on hardware
  └─ Phase 6: Cleanup & PR (format, document, PR)
```

**Time estimate**: 1-3 hours depending on complexity

---

## Phase 1: Generate PRD

**Goal**: Create comprehensive Product Requirements Document with expected behavior patterns

### Step 1.1: Research and Analyze (Silent)

**Before asking questions**, research the codebase and esp-hal APIs:

1. **Search existing lessons** for related functionality:
   - Check `lessons/01-blinky/`, `lessons/02-debugger/` for patterns
   - Review existing PRDs in `docs/prd/`
   - Look at esp-hal examples for the peripheral

2. **Understand esp-hal 1.0.0 APIs**:
   - What peripherals are involved? (GPIO, I2C, SPI, UART, etc.)
   - What esp-hal modules provide the functionality?
   - What configuration structs are needed?
   - What pins are available on ESP32-C6?

3. **Identify hardware requirements**:
   - What external components are needed?
   - What pins will be used?
   - What electrical characteristics matter?
   - What datasheets should be referenced?

4. **Identify knowledge gaps**:
   - What can't be determined from docs alone?
   - What requires user preference/decision?
   - What scope clarification is needed?

**DO NOT present research findings to user** - use this to formulate smart questions.

### Step 1.2: Ask Clarifying Questions

**After research**, ask targeted questions (max 5 unless user requests more):

**Question Guidelines**:
- **Be specific** - reference hardware, pins, APIs found during research
- **Offer options** when possible (makes answering easier)
- **Skip obvious** - if you can infer from similar lessons, don't ask
- **Focus on**: hardware setup, peripheral choice, sensor/device selection, use cases, success criteria

**Example questions**:
- "Which I2C sensor? (BME280, MPU6050, or other?)"
- "What readings should we display? (temperature, humidity, accel?)"
- "Which pins? Suggest: GPIO6 (SDA), GPIO7 (SCL)"
- "What I2C speed? (100kHz standard or 400kHz fast?)"
- "Should we use async (Embassy) or blocking I2C?"

**Wait for user responses** before proceeding.

### Step 1.3: Generate PRD

**PRD Structure** (save to `docs/prd/lesson-XX-{feature-name}-prd.md`):

```markdown
# Lesson XX: {Feature Name} - PRD

## Overview
- **Lesson Number**: XX
- **Feature**: {Feature Name}
- **Duration**: {Estimated time}
- **Difficulty**: Beginner/Intermediate/Advanced
- **Prerequisites**: {Previous lessons}

## Learning Objectives
What students will learn:
1. {Objective 1}
2. {Objective 2}
3. {Objective 3}

## Hardware Requirements
- ESP32-C6 development board
- {External components}
- {Wiring/connections}

**Pin Configuration**:
- GPIO{X}: {Function} - {Purpose}
- GPIO{Y}: {Function} - {Purpose}

## Software Requirements
- esp-hal 1.0.0 features: {list features}
- Additional crates: {list}
- probe-rs for debugging

## Expected Behavior

### Serial Output Patterns (Reference Behavior)
This is the "oracle" - what success looks like:

```
🚀 Starting Lesson XX: {Feature}
✓ {Peripheral} initialized
✓ GPIO{X} configured as {function}
✓ {Device} detected at address 0x{XX}
📊 Reading sensor...
  Temperature: 25.3°C
  Humidity: 45.2%
✓ Reading complete
{Loop pattern if applicable}
```

**Critical patterns to verify**:
- ✅ Initialization messages (with checkmarks)
- ✅ Peripheral detection/configuration
- ✅ Data readings with expected format
- ❌ No ERROR or panic messages
- ⚠️ Warnings are acceptable for {specific cases}

### Register States to Verify (probe-rs inspection points)
When using probe-rs debugger, these registers should show:

1. **After peripheral initialization**:
   - `{PERIPHERAL}_CONF_REG` (0x{address}): {expected value}
   - `GPIO_OUT_REG` (0x600A4004): GPIO{X} = {state}

2. **During operation**:
   - `{PERIPHERAL}_STATUS_REG` (0x{address}): {expected value}
   - `{PERIPHERAL}_DATA_REG` (0x{address}): {expected range}

## Functional Requirements
1. **REQ-1**: {Requirement}
2. **REQ-2**: {Requirement}
3. **REQ-3**: {Requirement}

## Technical Specifications
- **Timing**: {Response time, update rate, delays}
- **Power**: {Current consumption, sleep modes}
- **Memory**: {Flash/RAM usage estimates}
- **Error Handling**: {How to handle failures}

## Implementation Plan

### Code Structure
```rust
// Pin definitions
const PIN_X: u8 = {X};
const PIN_Y: u8 = {Y};

// Peripheral configuration
// Device driver (if external device)
// Main loop
```

### Key Implementation Points
1. Initialize peripheral with correct configuration
2. Add strategic logging at:
   - Initialization start/complete
   - Before/after peripheral operations
   - Data acquisition points
   - Error conditions
3. Mark potential breakpoint locations in comments
4. Handle errors gracefully with Result types

### Logging Strategy
- **info!()**: Major milestones (init, state changes, readings)
- **debug!()**: Detailed steps (register writes, calculations)
- **warn!()**: Recoverable issues (retries, degraded mode)
- **error!()**: Failures (peripheral not found, communication error)

## Testing Requirements

### Unit Tests (`src/lib.rs` or `tests/`)
- Test calculation logic (no hardware needed)
- Test state machines
- Test data parsing/formatting
- Mock peripheral responses

### On-Device Tests (`tests/on_device.rs` using defmt-test)
- Test peripheral initialization
- Test read/write operations
- Test error handling
- Test timing requirements

## Success Criteria
- [x] Code builds without warnings
- [x] All unit tests pass
- [x] All on-device tests pass (defmt-test)
- [x] Serial output matches expected patterns
- [x] probe-rs inspection shows correct register states
- [x] Hardware behavior matches specification
- [x] User validates on real hardware

## Edge Cases
1. **{Edge case 1}**: {How to handle}
2. **{Edge case 2}**: {How to handle}
3. **{Edge case 3}**: {How to handle}

## References
- [esp-hal {Peripheral} Module](https://docs.esp-rs.org/esp-hal/)
- [{Device} Datasheet](link)
- [ESP32-C6 Technical Reference](https://www.espressif.com/sites/default/files/documentation/esp32-c6_technical_reference_manual_en.pdf)

---

**Status**: Draft
**Next Steps**: Implementation Phase
```

### Step 1.4: User Checkpoint

**Present PRD and ask**:
> I've created a PRD for this lesson. Please review:
>
> [Link to PRD file]
>
> Key points:
> - Hardware: {summary}
> - Expected behavior: {summary of serial output}
> - Success criteria: {summary}
>
> Does this accurately capture what we're building? Any missing requirements or concerns?

**Wait for user approval** before proceeding to Phase 2.

---

## Phase 2: Project Setup

**Goal**: Create lesson directory structure with all necessary files

### Step 2.1: Create Directory Structure

```bash
mkdir -p lessons/{XX-lesson-name}/{src/bin,tests,.cargo}
```

### Step 2.2: Create Cargo.toml

**Copy from lesson 01 and adapt**:

```toml
[package]
name = "lesson-{XX}-{name}"
version = "0.1.0"
edition = "2021"
rust-version = "1.88"

[[bin]]
name = "lesson-{XX}-{name}"
path = "./src/bin/main.rs"

[dependencies]
# Hardware abstraction layer
esp-hal = { version = "1.0.0", features = ["esp32c6", "unstable"] }

# Panic handler with backtrace
esp-backtrace = { version = "0.15", features = ["esp32c6", "panic-handler", "println"] }

# Serial printing and logging
esp-println = { version = "0.13", features = ["esp32c6", "log"] }
log = "0.4"

# Bootloader app descriptor
esp-bootloader-esp-idf = { version = "0.4.0", features = ["esp32c6"] }

# Critical sections
critical-section = "1.2.0"

# Add lesson-specific dependencies here
# Example for I2C lesson:
# embedded-hal = "1.0"

[dev-dependencies]
# On-device testing
defmt = "0.3"
defmt-rtt = "0.4"
defmt-test = "0.3"
panic-probe = { version = "0.3", features = ["print-defmt"] }

[profile.dev]
opt-level = "s"

[profile.release]
codegen-units = 1
debug = 2
debug-assertions = false
incremental = false
lto = 'fat'
opt-level = 's'
overflow-checks = false
```

### Step 2.3: Create .cargo/config.toml

```toml
[target.riscv32imac-unknown-none-elf]
runner = "probe-rs run --chip esp32c6"

[env]

[build]
rustflags = [
  "-C", "force-frame-pointers",
]

target = "riscv32imac-unknown-none-elf"

[unstable]
build-std = ["core"]

[alias]
br = "build --release"
ck = "check"
ff = "run --release"
test-on-device = "test --test on_device --release"
```

### Step 2.4: Create Skeleton Files

**src/bin/main.rs** (skeleton):
```rust
//! # Lesson {XX}: {Feature Name}
//!
//! {Brief description}
//!
//! **Hardware:**
//! - ESP32-C6 development board
//! - {External components}
//!
//! **Pins:**
//! - GPIO{X}: {Function}
//!
//! **What You'll Learn:**
//! - {Learning objective 1}
//! - {Learning objective 2}

#![no_std]
#![no_main]

use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    main,
};
use log::{info, debug};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

// ============================================================================
// PIN CONFIGURATION
// ============================================================================

const PIN_X: u8 = {X};

// ============================================================================
// MAIN
// ============================================================================

#[main]
fn main() -> ! {
    // Initialize logging
    esp_println::logger::init_logger_from_env();
    log::set_max_level(log::LevelFilter::Debug);

    info!("🚀 Starting Lesson {XX}: {Feature}");

    // Initialize hardware
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let delay = Delay::new();

    // TODO: Peripheral initialization

    info!("✓ Initialization complete");

    loop {
        // TODO: Main loop
        delay.delay_millis(1000);
    }
}
```

**tests/on_device.rs** (skeleton):
```rust
//! On-device tests using defmt-test
#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[defmt_test::tests]
mod tests {
    use defmt::assert;

    #[init]
    fn init() -> () {
        // Initialize hardware for tests
        ()
    }

    #[test]
    fn test_peripheral_init(_state: &mut ()) {
        // TODO: Test peripheral initialization
        assert!(true);
    }
}
```

### Step 2.5: Summary (Informational)

> ✅ Project structure created:
> - `lessons/{XX-lesson-name}/`
> - Cargo.toml configured with defmt-test
> - .cargo/config.toml with probe-rs runner
> - Skeleton main.rs and test files
>
> Proceeding to implementation...

---

## Phase 3: Implementation (Autonomous Iterative Development)

**Goal**: Implement solution through autonomous build-flash-monitor iteration with probe-rs debugging when needed

### Step 3.1: Add Strategic Logging

**Before writing implementation**, plan logging points:

```rust
// Example logging strategy
info!("🚀 Starting Lesson {XX}: {Feature}");          // Startup
debug!("Configuring {peripheral}...");                // Pre-config
info!("✓ {Peripheral} initialized");                  // Post-config
debug!("  └─ Mode: {mode}, Speed: {speed}");         // Config details

// 📍 BREAKPOINT #1: Inspect peripheral registers here
debug!("Reading from {device}...");                   // Pre-operation
info!("📊 {Data}: {value}");                          // Data acquired
// 📍 BREAKPOINT #2: Inspect data registers here

debug!("Delay {ms}ms");                               // Timing info
```

**Logging Principles**:
- ✅ Use info!() for user-visible milestones
- ✅ Use debug!() for development insights
- ✅ Use warn!() for recoverable issues
- ✅ Use error!() for failures
- ✅ Mark breakpoint locations with `// 📍 BREAKPOINT #N` comments
- ✅ Include emojis for visual scanning (🚀 ✓ 📊 ⚠️ ❌)

### Step 3.2: Implement Core Functionality

**Write the implementation** based on PRD requirements:

1. **Peripheral initialization**
   - Configure pins
   - Initialize peripheral with correct settings
   - Verify initialization succeeded
   - Log success/failure

2. **Main functionality**
   - Implement core feature logic
   - Add error handling with Result types
   - Log operations at debug level
   - Log results at info level

3. **Error handling**
   - Handle peripheral errors gracefully
   - Log errors with context
   - Retry when appropriate
   - Never panic silently

### Step 3.3: Iterative Development Loop (Autonomous)

**IMPORTANT**: This loop runs autonomously up to 5 iterations. Be communicative so user can follow progress.

**At start, explain approach**:
> Starting implementation for Lesson {XX}: {Feature}
>
> Implementation approach based on PRD:
> 1. Initialize {peripheral} with {configuration}
> 2. Configure GPIO{X} as {function}
> 3. {Main functionality}
>
> Beginning build-flash-monitor cycle...

**Iteration cycle** (repeat until success or 5 iterations):

### Iteration Loop

```
Iteration N:
├─ Build code
├─ Auto-detect ESP32-C6
├─ Flash with probe-rs
├─ Monitor serial output (10-30 seconds)
├─ Compare output against expected patterns from PRD
├─ Analyze differences
├─ Decide: Fix code OR use probe-rs debugging
└─ Continue or stop
```

**Detailed steps**:

**1. Build the code**:
```bash
cd lessons/{XX-lesson-name}
cargo build --release
```

**2. Auto-detect device**:
```bash
probe-rs list
```

**3. Flash and monitor**:
```bash
cargo run --release
# This uses probe-rs runner from .cargo/config.toml
# Captures serial output automatically
```

**4. Compare output against expected patterns** (from PRD):
- Extract actual serial output
- Compare against "Expected Behavior → Serial Output Patterns"
- Identify missing patterns (e.g., expected "✓ I2C initialized" but got "ERROR")
- Identify unexpected patterns (e.g., panic, error messages)

**5. Analyze and form hypothesis**:

**Progress indicators** (making progress):
- ✅ More expected patterns appearing
- ✅ Fewer errors/panics
- ✅ Output getting closer to expected format
- ✅ New debug information revealed

**Stuck indicators** (not making progress):
- ❌ Same errors for 2+ iterations
- ❌ No serial output at all
- ❌ Unexpected hardware behavior
- ❌ Need to inspect registers

**6. Decision point**:

**If making progress**: Fix code based on output
```rust
// Example: Missing initialization message
// Add: info!("✓ I2C initialized");

// Example: Wrong peripheral configuration
// Fix: I2C::new(i2c, sda, scl, 400.kHz())  // was 100kHz
```

**If stuck or unclear**: Use probe-rs debugging
```
🔍 Serial output unclear, using probe-rs to inspect registers...

Hypothesis: I2C peripheral not initializing correctly
Plan: Set breakpoint after I2C::new(), inspect I2C_CONF_REG

Preparing probe-rs debugging session...
```

**7. Communicate progress** after each iteration:

> **Iteration {N}**:
> - Build: ✅ Success
> - Flash: ✅ Success
> - Serial output analysis:
>   - ✅ Found: "🚀 Starting Lesson {XX}"
>   - ✅ Found: "✓ GPIO{X} configured"
>   - ❌ Missing: "✓ {Peripheral} initialized"
>   - ❌ Found ERROR: "{error message}"
> - Hypothesis: {peripheral} initialization failing due to {reason}
> - Fix attempted: {description of fix}
> - Next: Rebuild and test...

**8. Make targeted fix and rebuild**

### Step 3.4: probe-rs Debugging (When Needed)

**Triggered when**:
- Serial output doesn't reveal root cause
- Suspect hardware/register issues
- Need to verify peripheral state
- After 2 failed iterations with same issue

**Debugging workflow**:

**1. Identify inspection point**:
```rust
// In code, mark with comment:
led.set_high();  // 📍 BREAKPOINT #1: Inspect GPIO_OUT_REG here

i2c.write(addr, &data);  // 📍 BREAKPOINT #2: Inspect I2C registers here
```

**2. Prepare debugging instructions for user**:
> 🔍 **probe-rs Debugging Needed**
>
> Serial output shows: "{actual output}"
> Expected: "{expected output}"
>
> To diagnose, we need to inspect registers:
>
> **Option A: CLI Debugging**
> ```bash
> cd lessons/{XX-lesson-name}
> probe-rs run --chip esp32c6 target/riscv32imac-unknown-none-elf/release/lesson-{XX}-{name}
>
> # When it breaks, inspect:
> # (gdb) x/x 0x{REGISTER_ADDRESS}
> ```
>
> **Option B: VS Code Debugging** (easier)
> 1. Open VS Code in `lessons/{XX-lesson-name}/`
> 2. Set breakpoint at line {XX}
> 3. Press F5 to debug
> 4. When paused, inspect variables panel
>
> What do you see?
> - `{variable_name}` = ?
> - `{register_name}` = ?

**3. Wait for user response**

**4. Analyze user's findings**:
- If register shows unexpected value → Fix configuration
- If variable is wrong → Fix calculation logic
- If peripheral not responding → Check wiring/hardware
- If still unclear → Ask for more inspection

**5. Make fix based on findings**:
```rust
// Example fix based on register inspection
// User reported: I2C_CONF_REG shows FIFO not enabled
// Fix: Enable FIFO in configuration
let config = Config::new()
    .with_fifo(true)  // ← Added based on register inspection
    .with_frequency(400.kHz());
```

**6. Return to iteration loop** (rebuild and test)

### Step 3.5: Stopping Conditions

**Stop and escalate after 5 iterations if**:
- Same error persists
- No progress on serial output matching
- Hardware behavior unexpected
- Unclear how to proceed

**Escalation message**:
> ⚠️ **Stuck after 5 iterations**
>
> **Current status**:
> - Builds: ✅ Success
> - Flashes: ✅ Success
> - Serial output:
>   ```
>   {actual output}
>   ```
> - Expected:
>   ```
>   {expected patterns from PRD}
>   ```
>
> **Hypotheses tried**:
> 1. {Hypothesis 1} → Result: {outcome}
> 2. {Hypothesis 2} → Result: {outcome}
> 3. {Hypothesis 3} → Result: {outcome}
>
> **Need your help**:
> - Can you inspect the hardware? Is {component} connected correctly?
> - Does the {LED/sensor/display} show any signs of life?
> - Can you run probe-rs and report the value of {register/variable}?
>
> **Alternatively**:
> - Try different approach: {alternative}
> - Simplify scope: {descoped version}
> - Need more information about: {what}
>
> What would you like me to do?

**Wait for user input** before continuing.

### Step 3.6: Implementation Complete

**When serial output matches expected patterns** (informational):
> ✅ **Implementation successful!**
>
> **Serial output** (actual):
> ```
> {captured serial output}
> ```
>
> **Matches expected patterns**:
> - ✅ Initialization messages
> - ✅ Peripheral configuration
> - ✅ Data readings
> - ✅ No errors or panics
>
> **Iterations**: {N} build-flash-monitor cycles
>
> Proceeding to testing phase...

---

## Phase 4: Testing

**Goal**: Create and run unit tests and on-device hardware tests

### Step 4.1: Write Unit Tests

**Create unit tests** for logic that doesn't require hardware:

**tests/unit_tests.rs** or **src/lib.rs**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculation_logic() {
        // Test data conversion, calculations, etc.
        let raw_value = 0x1234;
        let converted = convert_to_celsius(raw_value);
        assert_eq!(converted, 25.0);
    }

    #[test]
    fn test_state_machine() {
        // Test state transitions
        let mut state = State::Init;
        state = state.transition(Event::Start);
        assert_eq!(state, State::Running);
    }

    #[test]
    fn test_data_parsing() {
        // Test parsing sensor data
        let data = [0x12, 0x34];
        let result = parse_sensor_data(&data);
        assert!(result.is_ok());
    }
}
```

**Run unit tests**:
```bash
cargo test --lib
```

### Step 4.2: Write On-Device Tests (defmt-test)

**Create hardware integration tests** in `tests/on_device.rs`:

```rust
//! On-device hardware tests using defmt-test
#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    peripherals::Peripherals,
};

struct State {
    peripherals: Peripherals,
    delay: Delay,
}

#[defmt_test::tests]
mod tests {
    use super::*;

    #[init]
    fn init() -> State {
        info!("Initializing hardware for tests");
        let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
        let peripherals = esp_hal::init(config);
        let delay = Delay::new();

        State { peripherals, delay }
    }

    #[test]
    fn test_peripheral_initialization(state: &mut State) {
        info!("Testing peripheral initialization");

        // Initialize peripheral
        // TODO: Add actual peripheral init

        // Verify it initialized correctly
        assert!(true, "Peripheral should initialize successfully");
    }

    #[test]
    fn test_peripheral_read_write(state: &mut State) {
        info!("Testing peripheral read/write");

        // TODO: Write data to peripheral
        // TODO: Read data back
        // TODO: Verify data matches

        assert!(true, "Read/write should work");
    }

    #[test]
    fn test_error_handling(state: &mut State) {
        info!("Testing error handling");

        // TODO: Trigger error condition
        // TODO: Verify error is handled correctly

        assert!(true, "Errors should be handled gracefully");
    }
}
```

**Run on-device tests**:
```bash
cd lessons/{XX-lesson-name}
cargo test-on-device
# This uses probe-rs to flash tests to hardware and run them
```

### Step 4.3: Verify All Tests Pass

**Run both test suites**:
```bash
# Unit tests (host)
cargo test --lib

# On-device tests (hardware)
cargo test-on-device
```

**Expected output**:
```
running 3 tests
test tests::test_calculation_logic ... ok
test tests::test_state_machine ... ok
test tests::test_data_parsing ... ok

test result: ok. 3 passed; 0 failed
```

```
defmt-test: running 3 tests
test test_peripheral_initialization ... ok
test test_peripheral_read_write ... ok
test test_error_handling ... ok

test result: ok. 3 passed; 0 failed
```

### Step 4.4: Test Summary (Informational)

> ✅ **All tests passing**:
> - Unit tests: {N} passed
> - On-device tests: {M} passed
>
> **Coverage**:
> - [x] Peripheral initialization
> - [x] Read/write operations
> - [x] Error handling
> - [x] Data conversion logic
> - [x] {Other test categories}
>
> Proceeding to hardware validation...

---

## Phase 5: Hardware Validation (Interactive)

**Goal**: User confirms hardware behavior matches expected behavior from PRD

This phase guides you through manual verification that the firmware works correctly on real hardware.

### Step 5.1: Prepare Validation Instructions

**Based on the lesson**, prepare clear validation steps:

> 📋 **Hardware Validation Checklist**
>
> The firmware is now ready for final validation. Please verify the following on your ESP32-C6:
>
> **Setup**:
> 1. Ensure hardware is connected:
>    - {Component 1} connected to GPIO{X}
>    - {Component 2} connected to GPIO{Y}
>    - Power supply: USB-C cable
>
> **Flash the firmware**:
> ```bash
> cd lessons/{XX-lesson-name}
> cargo run --release
> ```
>
> **Expected behavior**:
> - [ ] Serial output shows: "🚀 Starting Lesson {XX}"
> - [ ] {Observable behavior 1} (e.g., LED blinks, sensor reading displayed)
> - [ ] {Observable behavior 2}
> - [ ] Serial output matches expected pattern from PRD
> - [ ] No ERROR or panic messages
>
> **Serial output should look like**:
> ```
> {Expected serial output from PRD}
> ```
>
> **Hardware behavior**:
> - {LED should blink / sensor should respond / display should show}
> - {Timing expectations}
> - {Any other observable characteristics}

### Step 5.2: User Performs Validation

**User actions**:
1. Flash firmware to hardware
2. Observe serial output
3. Observe hardware behavior (LEDs, sensors, etc.)
4. Compare against expected behavior
5. Report results

### Step 5.3: Validation Checkpoint (USER APPROVAL REQUIRED)

**Ask for confirmation**:
> **Validation Results**
>
> Did the hardware behave as expected?
>
> - Type **"yes"** if everything worked correctly
> - Type **"issue"** and describe what went wrong
> - Share any unexpected serial output or hardware behavior

**Wait for user response**

**If user reports "yes"**:
> ✅ Hardware validation successful! Proceeding to cleanup and documentation...

**If user reports issues**:
> Let me analyze the issue you reported:
> - {Analyze user's description}
> - {Hypothesis about root cause}
> - {Suggested fix}
>
> Should I:
> A) Make the fix and re-test
> B) Add more debug logging to investigate
> C) Guide you through probe-rs debugging

**Handle issues and return to Phase 3** if needed.

---

## Phase 6: Cleanup & Documentation

**Goal**: Production-ready code with documentation and PR

### Step 6.1: Create Lesson README

**Create `lessons/{XX-lesson-name}/README.md`**:

```markdown
# Lesson {XX}: {Feature Name}

{Brief description of what this lesson teaches}

## Learning Objectives

- {Objective 1}
- {Objective 2}
- {Objective 3}

## Hardware Requirements

- ESP32-C6 development board
- {External components}
- {Wiring details}

### Wiring Diagram

```
ESP32-C6        {Component}
--------        ----------
GPIO{X}    -->  {Pin}
GPIO{Y}    -->  {Pin}
GND        -->  GND
{VCC}      -->  {Power if needed}
```

## What You'll Learn

This lesson demonstrates:
- {Key concept 1}
- {Key concept 2}
- {Key concept 3}

## Build & Flash

```bash
cd lessons/{XX-lesson-name}

# Build
cargo build --release

# Flash to ESP32-C6
cargo run --release

# Run tests
cargo test --lib              # Unit tests
cargo test-on-device          # Hardware tests
```

## Expected Output

When you flash and run this lesson, you should see:

```
{Expected serial output from PRD}
```

## Code Structure

- `src/bin/main.rs` - Main firmware implementation
- `tests/on_device.rs` - On-device hardware tests using defmt-test
- `Cargo.toml` - Project dependencies
- `.cargo/config.toml` - Build configuration with probe-rs runner

## Key Concepts

### {Concept 1}
{Explanation}

### {Concept 2}
{Explanation}

## Troubleshooting

| Issue | Solution |
|-------|----------|
| {Common issue 1} | {Solution} |
| {Common issue 2} | {Solution} |

## Next Steps

- **Lesson {XX+1}**: {Next lesson topic}
- Experiment: {Suggested modifications}

## References

- [esp-hal {Peripheral} Docs](link)
- [{Component} Datasheet](link)
- [ESP32-C6 Technical Reference](link)
```

### Step 6.2: Code Cleanup

**Remove excessive debug logging**:
```rust
// Keep info!() for user-visible milestones
info!("🚀 Starting Lesson {XX}");
info!("✓ {Peripheral} initialized");
info!("📊 {Data reading}");

// Remove or comment out verbose debug!() logs
// debug!("Register value: 0x{:08x}", reg);  // ← Remove unless critical

// Keep important debug logs (can be enabled with RUST_LOG=debug)
debug!("Configured with: {:?}", config);
```

**Remove breakpoint comments** (or keep for educational purposes):
```rust
// Option A: Remove
led.set_high();  // ← Remove "📍 BREAKPOINT #1" comment

// Option B: Keep for educational value
led.set_high();  // 💡 TIP: Set breakpoint here to inspect GPIO_OUT_REG
```

**Format code**:
```bash
cargo fmt
```

**Check for warnings**:
```bash
cargo clippy -- -D warnings
```

**Verify final build**:
```bash
cargo build --release
```

### Step 6.3: Update Top-Level Documentation

**Update `README.md`** (top-level):
```markdown
## 📚 Lessons

- **[01-blinky](./lessons/01-blinky/)** ✅ - GPIO output & input with logging
- **[02-debugger](./lessons/02-debugger/)** ✅ - probe-rs debugging
- **[{XX-lesson-name}](./lessons/{XX-lesson-name}/)** ✅ - {Brief description}
```

**Update `docs/LESSON_PLAN.md`** if it exists:
```markdown
### Lesson {XX}: {Feature Name}
**Status**: ✅ Complete
**Duration**: {Duration}
**Prerequisites**: Lesson {XX-1}

{Brief description}

**Key Concepts**:
- {Concept 1}
- {Concept 2}
```

### Step 6.4: Commit Message Format

**Follow conventional commits**:

```
feat(lesson-{XX}): Add {feature name} lesson

Implements Lesson {XX} demonstrating {key concepts}.

Key features:
- {Feature 1}
- {Feature 2}
- {Feature 3}

Hardware:
- ESP32-C6 + {external components}
- GPIO{X}: {function}
- GPIO{Y}: {function}

Testing:
- Unit tests for {logic}
- defmt-test on-device tests for {hardware operations}
- Validated on hardware: {validation summary}

Closes #{issue_number}

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

### Step 6.5: Create Pull Request

**Commit and create PR**:

```bash
# Stage all changes
git add lessons/{XX-lesson-name}/ docs/prd/lesson-{XX}*.md README.md

# Commit with message from Step 6.4
git commit -m "feat(lesson-{XX}): Add {feature name} lesson

{Full commit message}"

# Push and create PR
git push origin HEAD
gh pr create --title "Lesson {XX}: {Feature Name}" --body "$(cat <<'EOF'
## Summary

Implements Lesson {XX} demonstrating {key concepts}.

## Hardware Setup

- ESP32-C6 development board
- {External components}
- Wiring: {Summary}

## Implementation Details

### Code Structure
- `src/bin/main.rs`: Main firmware with {peripheral} driver
- `tests/on_device.rs`: Hardware tests using defmt-test
- Strategic logging at key points

### Testing
- ✅ {N} unit tests (all passing)
- ✅ {M} on-device tests (all passing)
- ✅ Validated on hardware

### Expected Serial Output
```
{Expected output from PRD}
```

## Validation Results

- [x] Code builds without warnings
- [x] All unit tests pass
- [x] All on-device tests pass (defmt-test)
- [x] Serial output matches expected patterns
- [x] Hardware behavior verified on ESP32-C6
- [x] Documentation complete (README, PRD)

## Development Process

- Iterations: {N} build-flash-monitor cycles
- probe-rs debugging: {Used/Not used}
- Issues encountered: {Summary}

## Related

- PRD: docs/prd/lesson-{XX}-{name}-prd.md
- Closes #{issue_number}

---

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

### Step 6.6: PR Summary and Completion

**Present completed PR**:
> ✅ **Pull request created**: {PR URL}
>
> **Summary**:
> - Lesson {XX}: {Feature Name}
> - {N} unit tests + {M} on-device tests (all passing)
> - Hardware validated on ESP32-C6
> - Documentation complete
>
> **Files Added**:
> - `lessons/{XX-lesson-name}/src/bin/main.rs` - Main firmware
> - `lessons/{XX-lesson-name}/tests/on_device.rs` - defmt-test hardware tests
> - `lessons/{XX-lesson-name}/README.md` - Lesson documentation
> - `lessons/{XX-lesson-name}/Cargo.toml` - Dependencies
> - `docs/prd/lesson-{XX}-{name}-prd.md` - PRD
>
> **Validation**:
> - ✅ Builds without warnings
> - ✅ All tests pass
> - ✅ Serial output matches expected patterns
> - ✅ Hardware behavior confirmed
>
> **Development complete!** PR is ready for review.

---

## Key Principles

### 1. Autonomous Iteration (5 Cycles Max)
- **Build → Flash → Monitor → Compare → Fix** loop runs autonomously
- Maximum 5 iterations before asking for help
- Communicate progress after each iteration
- User can interrupt at any time

### 2. Serial Output as "Oracle" (Like Reference Schematic)
- PRD defines expected serial output patterns
- Compare actual vs expected output automatically
- Serial patterns verify correct behavior
- Missing patterns indicate issues to fix

### 3. Strategic Logging Throughout
- **info!()** for user-visible milestones
- **debug!()** for development insights (can be enabled with RUST_LOG=debug)
- **warn!()** for recoverable issues
- **error!()** for failures
- Mark potential breakpoint locations with comments

### 4. probe-rs Debugging When Needed
- Use when serial output doesn't reveal root cause
- Inspect peripheral registers to verify hardware state
- Guided interactive debugging sessions
- Return to iteration after findings

### 5. Testing at Multiple Levels
- **Unit tests**: Logic without hardware (fast)
- **defmt-test**: On-device hardware tests (real peripherals)
- **Manual validation**: User confirms on real hardware

### 6. Hardware-First Validation
- Can't trust tests alone - must validate on hardware
- User confirms expected behavior
- Checkpoint before proceeding to cleanup

### 7. Documentation as First-Class Citizen
- PRD captures expected behavior upfront
- Lesson README for students
- Code comments for learning
- All generated before PR

### 8. probe-rs for Everything
- Flash with `probe-rs run`
- Debug with `probe-rs`
- Test with `probe-rs` (defmt-test)
- Single tool for entire workflow

---

## Output Artifacts

At completion, you'll have:

1. **PRD** (`docs/prd/lesson-{XX}-{name}-prd.md`)
   - Expected behavior patterns (serial output)
   - Hardware requirements
   - Register states for verification
   - Success criteria

2. **Lesson Code** (`lessons/{XX-lesson-name}/`)
   - `src/bin/main.rs` - Main firmware with strategic logging
   - `tests/on_device.rs` - defmt-test hardware tests
   - `Cargo.toml` - Dependencies including defmt-test
   - `.cargo/config.toml` - probe-rs runner configuration

3. **Documentation**
   - `lessons/{XX-lesson-name}/README.md` - Student-facing docs
   - Code comments for learning
   - Wiring diagrams
   - Troubleshooting guide

4. **Tests**
   - Unit tests (logic validation)
   - On-device tests (hardware validation with defmt-test)
   - All tests passing

5. **Pull Request**
   - Conventional commit format
   - Detailed description
   - Hardware validation confirmed
   - Ready for review

---

## When to Use This Workflow

| Use Case | Use `/dev` | Notes |
|----------|-----------|-------|
| New lesson (I2C, SPI, UART) | ✅ Yes | Full workflow with PRD and tests |
| New peripheral driver | ✅ Yes | Hardware validation critical |
| Async task with Embassy | ✅ Yes | Can test on-device with defmt-test |
| State machine implementation | ✅ Yes | Unit tests + hardware validation |
| Bug fix in existing lesson | ⚠️ Maybe | Skip PRD, go to Phase 3 implementation |
| Documentation update | ❌ No | Just edit directly |
| Quick typo fix | ❌ No | Just fix directly |

---

## Tips for Best Results

**Good problem statements**:
- ✅ "Create Lesson 03: I2C sensor driver for BME280"
- ✅ "Add SPI display driver with double buffering"
- ✅ "Implement UART shell with command parsing"

**Poor problem statements**:
- ❌ "Make I2C work" (what device? what readings?)
- ❌ "Fix the code" (which code? what issue?)
- ❌ "Add stuff" (what stuff?)

**Prepare for success**:
- Have ESP32-C6 connected and ready
- Have external components wired up (or ready to wire)
- Know which GPIO pins are available
- Have probe-rs and espflash installed
- Be ready to confirm hardware behavior at validation checkpoint

**Trust the process**:
- Let autonomous iteration run (5 cycles)
- Serial output comparison catches most issues
- probe-rs debugging when needed
- Hardware validation is final confirmation
- Don't skip checkpoints

---

## Model Configuration

**Default model**: Uses your configured default (typically `claude-sonnet-4-5`)
- All phases use same model
- Uses Claude Code subscription
- No additional API costs

---

**This is your complete ESP32-C6 lesson development workflow. Use it for systematic firmware development with autonomous iteration, hardware validation, and comprehensive testing.**
