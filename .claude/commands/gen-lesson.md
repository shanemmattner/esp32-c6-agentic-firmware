---
description: Complete ESP32-C6 lesson development workflow - from PRD to hardware-validated PR
---

# /gen-lesson - ESP32-C6 Lesson Development Workflow

**Purpose**: End-to-end lesson development for ESP32-C6 firmware using esp-hal 1.0.0. Creates high-quality, progressively challenging lessons for engineers learning Rust embedded development on ESP32-C6 with Claude Code.

**Target Audience**: Engineers familiar with embedded systems who want to learn esp-hal 1.0.0 Rust development on ESP32-C6 using Claude Code. Not for beginners.

**Use when**: Creating new lessons, implementing firmware features, or developing HAL peripheral drivers that require systematic development with PRD, tests, and iterative hardware validation.

---

## Workflow Overview

```
/gen-lesson "Create Lesson 03: I2C sensor driver"
  ├─ Phase 1: Generate PRD (research → ask questions → document expected behavior)
  │   └─ STOP: User reviews PRD
  ├─ Phase 2: Project Setup (copy from previous lesson or create new)
  ├─ Phase 3: Implementation (collaborative iterative development)
  │   ├─ Write code with strategic logging
  │   ├─ Build → Flash → Monitor → Compare → Fix (loop as needed)
  │   ├─ Use probe-rs debugging when needed
  │   └─ STOP if stuck and collaborate with user
  ├─ Phase 4: Testing (mandatory - unit tests + on-device tests)
  ├─ Phase 5: Hardware Validation (mandatory - user confirms behavior)
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
   - Check `lessons/01-blinky/` for patterns
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
- "Should we use async or blocking I2C?"

**Wait for user responses** before proceeding.

### Step 1.3: Generate PRD

**PRD Structure** (save to `docs/prd/lesson-XX-{feature-name}-prd.md`):

```markdown
# Lesson XX: {Feature Name} - PRD

## Overview
- **Lesson Number**: XX
- **Feature**: {Feature Name}
- **Duration**: {Estimated time}
- **Difficulty**: Intermediate/Advanced
- **Prerequisites**: {Previous lessons}

## Learning Objectives
What engineers will learn:
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
- probe-rs for debugging (optional but recommended)

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

## Testing Requirements (Mandatory)

### Unit Tests (`src/lib.rs` or `tests/`)
- Test calculation logic (no hardware needed)
- Test state machines
- Test data parsing/formatting
- Mock peripheral responses

### On-Device Tests (`tests/on_device.rs` using defmt-test - if applicable)
- Test peripheral initialization
- Test read/write operations
- Test error handling
- Test timing requirements

**Note**: On-device tests are optional for simple lessons but recommended for complex peripheral interactions.

## Success Criteria (All Mandatory)
- [x] Code builds without warnings
- [x] All unit tests pass
- [x] All on-device tests pass (if applicable)
- [x] Serial output matches expected patterns
- [x] probe-rs inspection shows correct register states (if used)
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

**Strategy Options**:
1. **Copy from previous lesson** (fastest for similar lessons)
2. **Create from scratch** (for significantly different lessons)

### Step 2.1: Create Directory Structure

**Option A: Copy from previous lesson (Recommended)**:
```bash
cp -r lessons/{previous-lesson}/ lessons/{XX-lesson-name}/
# Then update:
# - Cargo.toml package name
# - src/bin/main.rs lesson number and description
```

**Option B: Generate with esp-generate (Best for new structure)**:
```bash
esp-generate --chip esp32c6 lesson-{XX}-{name}
cd lesson-{XX}-{name}
# Follow steps in gen-lesson Phase 2 to configure
```

**Option C: Create from scratch**:
```bash
mkdir -p lessons/{XX-lesson-name}/{src/bin,tests,.cargo}
```

### Step 2.2: Create Cargo.toml

**Standard template for ESP32-C6 lessons**:

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
# On-device testing (optional, add if needed)
# defmt = "0.3"
# defmt-rtt = "0.4"
# defmt-test = "0.3"
# panic-probe = { version = "0.3", features = ["print-defmt"] }

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
runner = "espflash flash --chip esp32c6"

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
```

### Step 2.4: Create rust-toolchain.toml

```toml
[toolchain]
channel    = "stable"
components = ["rust-src"]
targets = ["riscv32imac-unknown-none-elf"]
```

### Step 2.5: Create build.rs

Copy from previous lesson or create:

```rust
fn main() {
    linker_be_nice();
    // make sure linkall.x is the last linker script (otherwise might cause problems with flip-link)
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}

fn linker_be_nice() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let kind = &args[1];
        let what = &args[2];

        match kind.as_str() {
            "undefined-symbol" => match what.as_str() {
                "_defmt_timestamp" => {
                    eprintln!();
                    eprintln!("💡 `defmt` not found - make sure `defmt.x` is added as a linker script and you have included `use defmt_rtt as _;`");
                    eprintln!();
                }
                "_stack_start" => {
                    eprintln!();
                    eprintln!("💡 Is the linker script `linkall.x` missing?");
                    eprintln!();
                }
                "esp_rtos_initialized"
                | "esp_rtos_yield_task"
                | "esp_rtos_task_create" => {
                    eprintln!();
                    eprintln!("💡 `esp-radio` has no scheduler enabled. Make sure you have initialized `esp-rtos` or provided an external scheduler.");
                    eprintln!();
                }
                "embedded_test_linker_file_not_added_to_rustflags" => {
                    eprintln!();
                    eprintln!("💡 `embedded-test` not found - make sure `embedded-test.x` is added as a linker script for tests");
                    eprintln!();
                }
                _ => (),
            },
            // we don't have anything helpful for "missing-lib" yet
            _ => {
                std::process::exit(1);
            }
        }

        std::process::exit(0);
    }

    println!(
        "cargo:rustc-link-arg=--error-handling-script={}",
        std::env::current_exe().unwrap().display()
    );
}
```

### Step 2.6: Create Skeleton Files

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
    delay::Delay,
    main,
};
use log::info;

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
    log::set_max_level(log::LevelFilter::Info);

    info!("🚀 Starting Lesson {XX}: {Feature}");

    // Initialize hardware
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    // TODO: Peripheral initialization

    info!("✓ Initialization complete");

    loop {
        // TODO: Main loop
        delay.delay_millis(1000);
    }
}
```

**tests/on_device.rs** (optional - skeleton if needed):
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

### Step 2.7: Summary (Informational)

> ✅ Project structure created:
> - `lessons/{XX-lesson-name}/`
> - Cargo.toml configured
> - .cargo/config.toml with espflash runner
> - rust-toolchain.toml
> - build.rs for helpful linker errors
> - Skeleton main.rs
>
> Proceeding to implementation...

---

## Phase 3: Implementation (Collaborative Iterative Development)

**Goal**: Implement solution through collaborative build-flash-monitor iteration with probe-rs debugging when needed

**Note**: This phase is collaborative - I'll communicate progress and ask for help when stuck. Not a silent autonomous loop.

### Step 3.1: Add Strategic Logging

**Before writing implementation**, plan logging points:

```rust
// Example logging strategy
info!("🚀 Starting Lesson {XX}: {Feature}");          // Startup
info!("✓ {Peripheral} initialized");                  // Post-config
info!("📊 {Data}: {value}");                          // Data acquired

// Mark breakpoint locations for debugging
// 📍 BREAKPOINT #1: Inspect peripheral registers here
```

**Logging Principles**:
- ✅ Use info!() for user-visible milestones
- ✅ Use debug!() for development insights (enable with RUST_LOG=debug)
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
   - Log operations at appropriate levels
   - Keep code simple and readable (for YouTube format)

3. **Error handling**
   - Handle peripheral errors gracefully
   - Log errors with context
   - Retry when appropriate
   - Never panic silently

### Step 3.3: Iterative Development Loop (Collaborative)

**IMPORTANT**: This is a collaborative process. I'll communicate progress and ask for help when needed.

**At start, explain approach**:
> Starting implementation for Lesson {XX}: {Feature}
>
> Implementation approach based on PRD:
> 1. Initialize {peripheral} with {configuration}
> 2. Configure GPIO{X} as {function}
> 3. {Main functionality}
>
> Beginning build-flash-monitor cycle...

**Iteration cycle**:

```
Iteration N:
├─ Build code
├─ Flash with espflash
├─ Monitor serial output
├─ Compare output against expected patterns from PRD
├─ Analyze differences
├─ Decide: Fix code OR use probe-rs debugging OR ask for help
└─ Continue or stop
```

**Detailed steps**:

**1. Build the code**:
```bash
cd lessons/{XX-lesson-name}
cargo build --release
```

**2. Flash and monitor**:
```bash
cargo run --release
# This uses espflash from .cargo/config.toml
# Monitor output with espflash monitor or similar
```

**3. Compare output against expected patterns** (from PRD):
- Extract actual serial output
- Compare against "Expected Behavior → Serial Output Patterns"
- Identify missing patterns
- Identify unexpected patterns

**4. Analyze and form hypothesis**:

**Progress indicators**:
- ✅ More expected patterns appearing
- ✅ Fewer errors/panics
- ✅ Output getting closer to expected format
- ✅ New debug information revealed

**Stuck indicators**:
- ❌ Same errors for 2+ iterations
- ❌ No serial output at all
- ❌ Unexpected hardware behavior
- ❌ Unclear what to fix

**5. Decision point**:

**If making progress**: Fix code based on output and explain the fix

**If stuck**: Ask user for help with clear explanation of issue

**If registers need inspection**: Suggest probe-rs debugging

**6. Communicate progress** after each iteration:

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

### Step 3.4: probe-rs Debugging (When Needed)

**Triggered when**:
- Serial output doesn't reveal root cause
- Suspect hardware/register issues
- Need to verify peripheral state
- After multiple iterations with unclear issues

**Debugging workflow**:

**1. Identify inspection point** in code

**2. Suggest debugging approach to user**:
> 🔍 **probe-rs Debugging May Help**
>
> Serial output shows: "{actual output}"
> Expected: "{expected output}"
>
> To diagnose, we could inspect registers with probe-rs.
> Are you set up to use probe-rs for debugging?

**3. Collaborate with user** on debugging findings

### Step 3.5: Implementation Complete

**When serial output matches expected patterns**:
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

## Phase 4: Testing (Mandatory)

**Goal**: Create and run tests to verify functionality

**Note**: Testing is mandatory for all lessons. Adapt testing approach to lesson complexity.

### Step 4.1: Write Unit Tests

**Create unit tests** for logic that doesn't require hardware (if applicable):

**src/lib.rs** (if needed for testable logic):
```rust
// Move testable logic to lib.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculation_logic() {
        // Test data conversion, calculations, etc.
        let raw_value = 0x1234;
        let converted = convert_value(raw_value);
        assert_eq!(converted, expected);
    }

    #[test]
    fn test_data_parsing() {
        // Test parsing
        let data = [0x12, 0x34];
        let result = parse_data(&data);
        assert!(result.is_ok());
    }
}
```

**Run unit tests**:
```bash
cargo test --lib
```

**Note**: For simple lessons with mostly hardware interaction, unit tests may be minimal or skipped. Document why.

### Step 4.2: Hardware Testing

**Manual testing** (always required):
- Flash firmware
- Verify serial output matches expected patterns
- Verify hardware behavior (LEDs, sensors, etc.)
- Document observed behavior

**On-device tests** (optional for simple lessons):
- Use defmt-test for complex peripheral interactions
- Test initialization, read/write, error handling

### Step 4.3: Test Summary

> ✅ **Testing complete**:
> - Manual testing: Verified on hardware
> - Serial output: Matches expected patterns
> - Hardware behavior: Confirmed
> - Unit tests: {N} passed (if applicable)
>
> Proceeding to hardware validation...

---

## Phase 5: Hardware Validation (Mandatory)

**Goal**: User confirms hardware behavior matches expected behavior from PRD

**This phase is MANDATORY for all lessons.**

### Step 5.1: Prepare Validation Instructions

> 📋 **Hardware Validation Required**
>
> The firmware is ready for final validation. Please verify the following:
>
> **Setup**:
> 1. Hardware connections:
>    - {Component 1} connected to GPIO{X}
>    - {Component 2} connected to GPIO{Y}
>    - Power: USB-C cable
>
> **Flash the firmware**:
> ```bash
> cd lessons/{XX-lesson-name}
> cargo run --release
> ```
>
> **Expected behavior**:
> - [ ] Serial output shows: "🚀 Starting Lesson {XX}"
> - [ ] {Observable behavior 1}
> - [ ] {Observable behavior 2}
> - [ ] Serial output matches expected pattern
> - [ ] No ERROR or panic messages
>
> **Serial output should match**:
> ```
> {Expected serial output from PRD}
> ```

### Step 5.2: Validation Checkpoint (USER APPROVAL REQUIRED)

**Ask for confirmation**:
> **Validation Results**
>
> Did the hardware behave as expected?
>
> - Type **"yes"** if everything worked correctly
> - Type **"issue"** and describe what went wrong

**Wait for user response**

**If approved**: Proceed to cleanup

**If issues**: Collaborate to fix and re-test

---

## Phase 6: Cleanup & Documentation

**Goal**: Production-ready code with documentation and commit

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
```

## Expected Output

When you flash and run this lesson, you should see:

```
{Expected serial output from PRD}
```

## Code Structure

- `src/bin/main.rs` - Main firmware implementation
- `src/lib.rs` - Library code (empty, not used)
- `Cargo.toml` - Project manifest with `[[bin]]` section pointing to `src/bin/main.rs`
- `.cargo/config.toml` - Build configuration with espflash runner
- `rust-toolchain.toml` - Rust toolchain and RISC-V target

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

**Clean up code**:
```bash
# Format code
cargo fmt

# Check for warnings
cargo clippy

# Final build
cargo build --release
```

**Remove excessive debug logging** (keep info-level logs)

### Step 6.3: Commit Changes

**Commit with descriptive message**:

```bash
git add lessons/{XX-lesson-name}/ docs/prd/lesson-{XX}*.md

git commit -m "feat(lesson-{XX}): Add {feature name} lesson

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
- Hardware validated
- {Test summary}

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

### Step 6.4: Completion Summary

> ✅ **Lesson development complete!**
>
> **Summary**:
> - Lesson {XX}: {Feature Name}
> - Hardware validated on ESP32-C6
> - Documentation complete
> - Committed to repository
>
> **Files Created**:
> - `lessons/{XX-lesson-name}/src/main.rs`
> - `lessons/{XX-lesson-name}/README.md`
> - `docs/prd/lesson-{XX}-{name}-prd.md`
>
> **Validation**:
> - ✅ Builds without warnings
> - ✅ Serial output matches expected patterns
> - ✅ Hardware behavior confirmed
>
> Lesson is ready!

---

## Key Principles

### 1. Collaborative Development
- Communicate progress clearly
- Ask for help when stuck (don't iterate forever)
- Explain hypotheses and fixes
- User can provide input at any time

### 2. Serial Output as "Oracle"
- PRD defines expected serial output patterns
- Compare actual vs expected output
- Serial patterns verify correct behavior
- Missing patterns indicate issues to fix

### 3. Strategic Logging
- **info!()** for user-visible milestones
- Keep logging simple and readable
- Include emojis for visual scanning
- Mark potential breakpoint locations

### 4. Testing is Mandatory
- All lessons must be tested
- At minimum: manual hardware validation
- Unit tests for complex logic
- Document testing approach

### 5. Hardware Validation is Mandatory
- Cannot skip hardware validation
- User must confirm expected behavior
- Checkpoint before proceeding to cleanup

### 6. Code Structure for Video Production

**For simple lessons (01-05):**
- **Target: ~100-150 lines user-typed** (entire lesson type-able in 5-10 minutes)
- Mark sections with comments showing what user types:
  ```rust
  // [USER TYPES] - Main application logic
  // ============================================================================
  fn main() -> ! { /* ... */ }
  // [END USER TYPES]
  ```

**For complex lessons (06+):**
- **Split code into sections** with clear markers:
  ```rust
  // [SECTION 1/3: USER TYPES - Main loop and streaming]
  // DELETE THIS COMMENT and type from here...
  fn main() -> ! { /* ... */ }
  // [END SECTION 1/3]

  // [SECTION 2/3: COPY-PASTE - UART driver]
  // Keep this, copy from starter code
  // ... boilerplate code ...
  // [END SECTION 2/3]
  ```

- **~50-100 lines USER TYPES** - Core logic (type live)
- **~100-300 lines COPY-PASTE** - Drivers, utilities, boilerplate
- **Total video: 15-20 minutes** (intro + copy + live coding + test)

**Guidelines:**
- Mark sections clearly for easy find-and-replace during video editing
- Keep USER TYPES sections interesting and understandable
- Put repetitive/boilerplate code in COPY-PASTE sections
- Provide STARTER_CODE.md with copy-paste blocks clearly marked
- No edge case exhaustion
- Simple, readable code
- Clear section comments

### 7. Test-Driven Development (TDD) for Future Lessons
- **Write tests BEFORE implementation** when appropriate (for pure functions)
- Think about testability first - separate hardware from logic
- Host tests for algorithms, data transformations, state machines
- Device tests for I2C/SPI/GPIO hardware validation
- **Keep tests simple** - focus on main use cases, not edge case exhaustion
- Test-first approach helps design better APIs and cleaner code

**Why test "obvious" logic?**
1. **Regression prevention** - Tests catch bugs when you refactor 6 months later
2. **Forces better design** - Thinking "how do I test this?" naturally leads to isolation, pure functions, and loose coupling
3. **Living documentation** - Tests show how code should work
4. **Confidence** - Change code without fear

Even simple tests like "state transitions from Off to On" are valuable because they catch regressions when you add features later.

### 8. Copy Previous Lessons When Appropriate
- Fastest way to start similar lessons
- Ensures consistency
- Update names, numbers, and functionality
- Maintain project structure

---

## Output Artifacts

At completion, you'll have:

1. **PRD** (`docs/prd/lesson-{XX}-{name}-prd.md`)
   - Expected behavior patterns
   - Hardware requirements
   - Success criteria

2. **Lesson Code** (`lessons/{XX-lesson-name}/`)
   - `src/bin/main.rs` - Main firmware
   - `src/lib.rs` - Library code (empty)
   - `Cargo.toml` - Project manifest with `[[bin]]` section
   - `.cargo/config.toml` - espflash runner configuration
   - `rust-toolchain.toml` - Toolchain config with RISC-V target
   - `build.rs` - Build script with helpful linker errors

3. **Documentation**
   - `lessons/{XX-lesson-name}/README.md` - Lesson docs
   - Code comments for learning

4. **Validation**
   - Hardware tested and confirmed
   - Serial output verified
   - All expected patterns present

5. **Commit**
   - Conventional commit format
   - Detailed description
   - Ready for repository

---

## When to Use This Workflow

| Use Case | Use `/gen-lesson` | Notes |
|----------|------------------|-------|
| New lesson (I2C, SPI, UART) | ✅ Yes | Full workflow with PRD |
| New peripheral driver | ✅ Yes | Hardware validation critical |
| Task scheduler implementation | ✅ Yes | Test and validate |
| State machine | ✅ Yes | Full workflow |
| Bug fix in existing lesson | ⚠️ Maybe | Skip PRD, go to Phase 3 |
| Documentation update | ❌ No | Just edit directly |

---

## Tips for Best Results

**Good lesson requests**:
- ✅ "Create Lesson 03: I2C sensor driver"
- ✅ "Add PWM motor control lesson"
- ✅ "Implement UART serial communication"

**Poor lesson requests**:
- ❌ "Make I2C work" (what device? what goal?)
- ❌ "Fix the code" (which code? what issue?)

**Prepare for success**:
- Have ESP32-C6 connected and ready
- Have external components available
- Know which GPIO pins to use
- Be ready to validate on hardware

---

**This is your complete ESP32-C6 lesson development workflow. Use it for systematic firmware development with hardware validation and comprehensive testing for engineers learning esp-hal 1.0.0 Rust development.**
