# Lesson 02: Debugger with probe-rs - PRD

## Overview

**Title:** Debugger Essentials with probe-rs
**Duration:** 30 minutes
**Difficulty:** Intermediate
**Prerequisites:** Lesson 01 (Blinky) completed

Build on Lesson 01's GPIO fundamentals by adding professional-grade debugging capabilities. Learn to use probe-rs with the ESP32-C6's built-in USB-JTAG to inspect memory, set breakpoints, and understand code execution in real-time.

---

## 🎯 Goal

Demonstrate that the ESP32-C6's built-in USB-JTAG enables simultaneous serial communication and hardware debugging—no external equipment needed.

**Key Insight:** Understanding code execution through a debugger is as important as understanding the hardware it controls.

---

## 📚 Learning Objectives

By the end of this lesson, students will:

1. **Set up probe-rs** - Install and configure probe-rs with the ESP32-C6
2. **Use breakpoints** - Pause execution at specific lines to inspect state
3. **Inspect memory** - Read registers and memory locations in real-time
4. **View call stacks** - Understand the function call chain during execution
5. **Watch variables** - Track variable changes during program execution
6. **Debug GPIO state** - Use breakpoints to see GPIO pin states during transitions
7. **Understand timing** - Use the debugger to verify delay timing accuracy

---

## 🛠️ Hardware Requirements

Same as Lesson 01:
- ESP32-C6 development board
- USB-C cable (uses GPIO 12/13 for JTAG, not an external pin)
- Optional: LED + resistor on GPIO13 (from Lesson 01)

**USB Connection:** Standard USB cable to USB-C port
- GPIO 13 = D+ (JTAG)
- GPIO 12 = D- (JTAG)
- GND = Ground
- All integrated—no external hardware needed

---

## 💻 Software Requirements

### Prerequisites to Install

```bash
# probe-rs (already in Lesson 01 prereqs)
cargo install probe-rs --locked

# VSCode (optional, but recommended for GUI debugging)
# Or use CLI-based debugging

# ESP32-C6 debugging support (auto-handled by esp-generate with -o probe-rs)
```

### Project Setup

Use Lesson 01's `esp-generate` command:
```bash
esp-generate --chip esp32c6 lesson-02-debugger -o probe-rs
```

The `-o probe-rs` flag automatically configures:
- Proper linker scripts for debug symbols
- `.cargo/config.toml` with debugging runner
- `Cargo.toml` with necessary debug dependencies

---

## 📖 What Students Will Learn

### Part 1: probe-rs Basics (5 min)

**Concepts:**
- What is JTAG and why it's useful for debugging
- How ESP32-C6's built-in USB-JTAG works
- probe-rs as the Rust-native debugging tool
- Difference between UART serial and JTAG debugging

**Activities:**
- Verify probe-rs installation
- List connected ESP32-C6 devices
- Understand probe-rs vs OpenOCD comparison

### Part 2: Setting Breakpoints (8 min)

**Concepts:**
- What breakpoints are and why they're useful
- Soft breakpoints (BKPT instruction)
- Breakpoint persistence vs one-shot
- Resume and continue execution

**Activities:**
- Set a breakpoint in the blink loop
- Run firmware, hit breakpoint, inspect state
- Set conditional breakpoints
- Resume execution and hit again

### Part 3: Memory & Register Inspection (10 min)

**Concepts:**
- Reading memory by address
- Register layout and meaning
- Peripheral control registers (GPIO_OUT, GPIO_IN)
- Using debugger to verify GPIO pin states

**Activities:**
- Inspect GPIO peripheral registers while LED is ON
- Inspect GPIO peripheral registers while LED is OFF
- Compare serial logging vs actual register values
- Verify that register values match expected state

### Part 4: Call Stack & Variables (5 min)

**Concepts:**
- Stack unwinding and function call trace
- Local variables and their storage location
- Register allocation by the compiler
- Using debugger to understand optimization

**Activities:**
- Pause at a breakpoint and view call stack
- Watch the `cycle` counter variable change
- Inspect `delay_millis` value
- See how compiler optimizes loop counter

### Part 5: Putting It Together (2 min)

**Concepts:**
- Debugging workflow: breakpoint → inspect → resume
- When to use debugger vs logging
- Combining serial logging with JTAG debugging

**Activities:**
- Run Lesson 01 code with breakpoints
- Verify logging output matches actual state
- Debug a simple bug (e.g., off-by-one in cycle counter)

---

## ✅ Success Criteria

Students successfully complete this lesson when they can:

- [ ] Install and verify probe-rs works with their ESP32-C6
- [ ] Set and hit a breakpoint in their firmware
- [ ] Pause execution and inspect the GPIO peripheral registers
- [ ] Read the `cycle` counter variable from the debugger
- [ ] View the call stack at a breakpoint
- [ ] Resume from a breakpoint and let the program continue
- [ ] Understand the relationship between source code and memory state
- [ ] Explain when to use the debugger vs logging

---

## 🔬 Key Concepts

### Built-in USB-JTAG

ESP32-C6 includes dedicated hardware for JTAG debugging:
- No external programming/debugging hardware needed
- JTAG and serial work simultaneously (different endpoints)
- Same USB cable powers the chip and provides JTAG
- Automatic detection—probe-rs finds it instantly

### probe-rs

Modern Rust-native debugging tool:
- Works with arm-based and riscv-based embedded systems
- Full breakpoint and watchpoint support
- Real-time memory inspection
- No external server (like OpenOCD) needed
- Integrates with VSCode and IDE plugins

### Debugging vs Logging

| Aspect | Debugging (JTAG) | Logging (Serial) |
|--------|------------------|-----------------|
| Speed | Real-time, pauses execution | Real-time, non-blocking |
| Overhead | Minimal | Can impact timing |
| State inspection | Automatic at any point | Must be explicitly logged |
| Performance impact | None when not paused | Always active |
| Best for | Understanding code flow | Performance monitoring |
| Combined use | Excellent—use both together | Excellent—use both together |

---

## 📋 Lesson Outline

### Introduction (2 min)
- Show what we did in Lesson 01 (GPIO blinking + serial logging)
- Introduce the limitation: "How do we know the GPIO register is actually changing?"
- Show how the debugger answers this

### Part 1: Setup (3 min)
```bash
# Verify probe-rs installation
probe-rs list

# Output should show your ESP32-C6
# Example: "ESP32-C6 (riscv) @ 10:e6:d4:10:e0:b0"
```

### Part 2: Prepare Lesson 01 Code (2 min)
- Use Lesson 01's code (or simplify it)
- Add a few strategic comments marking breakpoint locations
- No code changes needed—same Blinky + GPIO9 input

### Part 3: Setting First Breakpoint (8 min)

**Code:**
```rust
loop {
    led.set_high();  // ← Breakpoint here
    info!("🔴 LED ON");
    delay.delay_millis(500);

    led.set_low();   // ← Breakpoint here
    info!("⚫ LED OFF");
    delay.delay_millis(500);

    cycle += 1;
}
```

**Using probe-rs:**
```bash
# Method 1: CLI
cargo run --release
# When it pauses at breakpoint, use `monitor` command to inspect

# Method 2: VSCode (with probe-rs extension)
# Set breakpoint in editor, press F5 to debug
```

### Part 4: Inspect GPIO Registers (8 min)

**Show students:**
- ESP32-C6 GPIO peripheral base address
- `GPIO_OUT_REG` offset (reads output pin states)
- `GPIO_IN_REG` offset (reads input pin states)
- How to read memory: `monitor read 0x600a4xxx`

**At breakpoint after `led.set_high()`:**
- GPIO_OUT register should show GPIO13 bit set
- GPIO_IN register should show GPIO9 bit set (same as output)
- Demonstrates hardware and software are in sync

**At breakpoint after `led.set_low()`:**
- GPIO_OUT register should show GPIO13 bit cleared
- GPIO_IN register should show GPIO9 bit cleared
- Proves the register actually changed

### Part 5: Inspect Variables (5 min)

**Show students:**
- Local variable `cycle` in the main loop
- Watch it increment: 0 → 1 → 2 ... → 10 (with special log)
- Stack layout and calling convention
- How compiler optimizes the loop

### Part 6: Call Stack (5 min)

**At any breakpoint:**
- Show the call stack: `loop → main → entry`
- Explain what each frame means
- Show return addresses and function entry points

---

## 🎓 Code Example

**lesson-02-debugger/src/bin/main.rs**

```rust
#![no_std]
#![no_main]

use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig},
    main,
};
use log::info;

esp_bootloader_esp_idf::esp_app_desc!();

const LED_PIN: u8 = 13;
const INPUT_PIN: u8 = 9;

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    log::set_max_level(log::LevelFilter::Info);

    info!("🚀 Starting Lesson 02: Debugger with probe-rs");

    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));
    let delay = Delay::new();

    let mut led = Output::new(peripherals.GPIO13, Level::Low, OutputConfig::default());
    let input = Input::new(peripherals.GPIO9, InputConfig::default());

    info!("✓ GPIO13 configured as output");
    info!("✓ GPIO9 configured as input");
    info!("📍 Set breakpoints in the loop below to inspect GPIO state\n");

    let mut cycle = 0;
    loop {
        led.set_high();  // 📍 BREAKPOINT #1: Inspect after this
        info!("🔴 LED ON  → GPIO9: {}", if input.is_high() { "HIGH" } else { "LOW" });
        delay.delay_millis(500);

        led.set_low();   // 📍 BREAKPOINT #2: Inspect after this
        info!("⚫ LED OFF → GPIO9: {}", if input.is_high() { "HIGH" } else { "LOW" });
        delay.delay_millis(500);

        cycle += 1;
        if cycle % 10 == 0 {
            info!("  └─ {} cycles completed", cycle);
        }
    }
}
```

---

## 🧪 Debugging Exercises

### Exercise 1: Hit a Breakpoint
1. Set breakpoint at `led.set_high()`
2. Run `cargo run --release`
3. Should pause immediately
4. Resume with debugger command

### Exercise 2: Inspect GPIO Registers
1. Set breakpoint after `led.set_high()`
2. Hit breakpoint
3. Read GPIO_OUT register: `monitor read 0x600a4xxx` (GPIO13 should be 1)
4. Read GPIO_IN register: `monitor read 0x600a4xxx` (GPIO9 should be 1)
5. Resume and hit breakpoint after `led.set_low()`
6. Read same registers (should both be 0)

### Exercise 3: Watch Variable
1. Set breakpoint in loop
2. Watch the `cycle` variable
3. Each time you resume, it should increment
4. At cycle = 10, check if log appears
5. Continue until cycle = 20

### Exercise 4: Understand Call Stack
1. Hit any breakpoint
2. View call stack: should show loop → main → _start
3. Explain what each frame represents
4. Look at return addresses (point to instruction after CALL)

---

## 📚 Related Documentation

- [probe-rs Docs](https://probe.rs/)
- [ESP32-C6 USB-JTAG Guide](https://docs.espressif.com/projects/esp-idf/en/stable/esp32c6/api-guides/usb-serial-jtag-console.html)
- [ESP32-C6 JTAG Debugging](https://docs.espressif.com/projects/esp-idf/en/stable/esp32c6/api-guides/jtag-debugging/index.html)
- [ESP32-C6 Built-in JTAG Configuration](https://docs.espressif.com/projects/esp-idf/en/stable/esp32c6/api-guides/jtag-debugging/configure-builtin-jtag.html)

---

## 🎯 Key Takeaways

1. **ESP32-C6 has free debugging** - USB-JTAG is built in, no external hardware needed
2. **probe-rs is Rust-native** - No OpenOCD or complex setup required
3. **JTAG and serial coexist** - Debug and see logs simultaneously
4. **Breakpoints are powerful** - Pause execution and inspect real-time state
5. **Registers tell the truth** - Hardware state is what matters; use debugger to see it
6. **Debugging != Logging** - Each has its place; use both together
7. **GPIO13/12 are reserved** - For USB-JTAG; don't reconfigure them for I/O

---

## 🚀 Next Lessons

- **Lesson 03:** Async/await with Embassy (use debugger for async debugging)
- **Lesson 04:** I2C Sensor Driver (debug I2C protocol on the wire)
- **Lesson 05:** SPI Display (debug display output with breakpoints)

---

## 📊 Success Metrics

- [ ] Lesson passes build checks
- [ ] All exercises completed successfully
- [ ] Students can set and hit breakpoints independently
- [ ] Students can explain GPIO register state at any point
- [ ] Integration with next lessons (async debugging)

---

*PRD for Lesson 02: Debugger with probe-rs*
*Based on Lesson 01: Blinky with GPIO Input*
*Target Audience: Embedded Rust developers, intermediate level*
