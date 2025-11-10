# Lesson 02: Understanding Code Execution with Logging and Debugging Concepts
## Serial Monitoring and Embedded Debugging Fundamentals

**Duration:** 30 minutes
**Goal:** Learn debugging techniques: serial monitoring, log analysis, and understanding code execution flow

---

## 📋 Prerequisites

### Hardware
- ESP32-C6 development board
- USB-C cable (provides both power and serial communication)
- Optional: LED + resistor on GPIO13 (from Lesson 01)

**Note:** GPIO 12 (D-) and GPIO 13 (D+) are reserved for USB-JTAG. Do NOT use them for general I/O in this lesson.

### Software (Same as Lesson 01)
```bash
# Install Rust and RISC-V target (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add riscv32imac-unknown-none-elf
source $HOME/.cargo/env

# Install espflash for building and monitoring
cargo install espflash --locked

# Install esp-generate for project templates
cargo install esp-generate --locked
```

---

## 🚀 Running This Lesson

### Step 1: Build the Project
```bash
cd lessons/02-debugger
cargo build --release
```

### Step 2: Flash to ESP32-C6
```bash
cargo run --release
# Or use cargo alias:
cargo ff
```

The firmware builds and flashes successfully. You should see serial output on your terminal.

### Step 3: Monitor Serial Output

The espflash tool will automatically show serial output. You'll see:
```
🚀 Starting Lesson 02: Debugger with probe-rs
📍 Set breakpoints below to inspect GPIO state
✓ GPIO13 configured as output
✓ GPIO9 configured as input
Starting GPIO demonstration...

--- GPIO Output Test ---
Set GPIO13 HIGH
  GPIO9 reads: HIGH
Set GPIO13 LOW
  GPIO9 reads: LOW

--- Blinking Loop ---
(Set breakpoints in the loop below)

🔴 LED ON  → GPIO9: HIGH
⚫ LED OFF → GPIO9: LOW
🔴 LED ON  → GPIO9: HIGH
⚫ LED OFF → GPIO9: LOW
  └─ 10 cycles completed
...
```

---

## 💡 Understanding Code Through Logging

Since the firmware includes strategic logging points, we can learn debugging concepts by analyzing the output:

### Serial Output as Your Debugger

Instead of using a hardware debugger, we can understand code execution through **structured logging**:

```rust
led.set_high();  // 📍 This line runs
info!("🔴 LED ON");  // Then this log appears
// If we see the log, we know the code executed
```

**What this teaches us:**
- Code executes in order (top to bottom)
- Logs prove execution path
- Timestamps show timing information
- Variable values can be logged at key points

### Analyzing the Cycle Counter

Watch the output carefully:
```
🔴 LED ON  → GPIO9: HIGH      # cycle = 0, ON
⚫ LED OFF → GPIO9: LOW       # cycle = 0, OFF
🔴 LED ON  → GPIO9: HIGH      # cycle = 1, ON
⚫ LED OFF → GPIO9: LOW       # cycle = 1, OFF
...
🔴 LED ON  → GPIO9: HIGH      # cycle = 9, ON
⚫ LED OFF → GPIO9: LOW       # cycle = 9, OFF
  └─ 10 cycles completed      # cycle % 10 == 0 condition true!
```

The logs prove:
- Loop executes correctly
- Counter increments each cycle
- Modulo condition (`cycle % 10 == 0`) works

---

## 🔬 Hands-On Exercises

### Exercise 1: Trace Code Execution Through Logs

**Goal:** Understand which code paths execute

1. Run `cargo ff` to flash and monitor
2. Watch the first lines:
   ```
   🚀 Starting Lesson 02: Debugger with probe-rs
   📍 Set breakpoints below to inspect GPIO state
   ✓ GPIO13 configured as output
   ✓ GPIO9 configured as input
   ```
3. Each log proves that line of code executed
4. If a log is missing, that code didn't run

**Result:** You understand code flow through logs!

### Exercise 2: Verify GPIO State Changes

**Goal:** Use logs to prove GPIO is actually changing

1. Look for the GPIO test section:
   ```
   --- GPIO Output Test ---
   Set GPIO13 HIGH
     GPIO9 reads: HIGH
   Set GPIO13 LOW
     GPIO9 reads: LOW
   ```

2. Each log line proves:
   - `Set GPIO13 HIGH` → code called `led.set_high()`
   - `GPIO9 reads: HIGH` → input detected the HIGH state
   - Then LOW follows, proving state changed

**Result:** Logs prove the hardware is doing what code commands!

### Exercise 3: Count Cycles Using Logs

**Goal:** Use logging to verify loop behavior

1. Watch the main blinking loop:
   ```
   🔴 LED ON  → GPIO9: HIGH
   ⚫ LED OFF → GPIO9: LOW
   🔴 LED ON  → GPIO9: HIGH
   ⚫ LED OFF → GPIO9: LOW
   🔴 LED ON  → GPIO9: HIGH
   ⚫ LED OFF → GPIO9: LOW
   🔴 LED ON  → GPIO9: HIGH
   ⚫ LED OFF → GPIO9: LOW
   🔴 LED ON  → GPIO9: HIGH
   ⚫ LED OFF → GPIO9: LOW
   🔴 LED ON  → GPIO9: HIGH
   ⚫ LED OFF → GPIO9: LOW
   🔴 LED ON  → GPIO9: HIGH
   ⚫ LED OFF → GPIO9: LOW
   🔴 LED ON  → GPIO9: HIGH
   ⚫ LED OFF → GPIO9: LOW
   🔴 LED ON  → GPIO9: HIGH
   ⚫ LED OFF → GPIO9: LOW
   🔴 LED ON  → GPIO9: HIGH
   🔴 LED OFF → GPIO9: LOW
     └─ 10 cycles completed
   ```

2. Count the ON/OFF pairs: 10 pairs = 10 cycles
3. When you see `└─ 10 cycles completed`, you're at the special log
4. Each ON/OFF pair represents one loop iteration

**Result:** Logs prove the counter and loop work correctly!

### Exercise 4: Monitor Timing

**Goal:** Understand execution timing through log patterns

1. Each cycle takes approximately 1 second (500ms ON + 500ms OFF)
2. Watch the logs appear with roughly 1-second intervals
3. Count the time between milestone logs:
   ```
   └─ 10 cycles completed   # Appears after ~10 seconds
   └─ 20 cycles completed   # Appears ~10 seconds later
   └─ 30 cycles completed   # Appears ~10 seconds later
   ```

**Result:** Logs show timing is working correctly!

---

## 🎓 Debugging Concepts

### Serial Logging as Debugging

**Problem:** How do we know if code is running correctly?
**Solution:** Add logs at key points and analyze output

This lesson teaches the foundation of debugging:

| Technique | When to Use | Example |
|-----------|-------------|---------|
| **Logging** | Always—understand what's happening | `info!("LED is ON")` |
| **Log analysis** | Trace execution path | Reading log sequence |
| **Variable logging** | Verify values at key points | `info!("cycle: {}", cycle)` |
| **Timing observation** | Verify delays work | Watch log timestamps |

### Why Logs Are Powerful

1. **No tools required** - Just serial output
2. **Always available** - Works on any hardware
3. **Performance impact** - Minimal when optimized
4. **Production debugging** - Logs survive after release
5. **Easy to reason about** - Natural order of execution

---

## 🔧 Advanced: Hardware Debugging with probe-rs

**Note:** Full JTAG debugging with probe-rs requires additional setup:
- probe-rs CLI tool installation
- Compatible debug probe hardware (usually)
- Platform-specific configuration

For now, **serial logging is your primary debugging tool** and teaches the same concepts:
- Code execution tracing
- State verification
- Timing analysis
- Variable inspection

We'll explore advanced JTAG debugging in later lessons if you have the hardware available.

---

## 🐛 Troubleshooting

| Problem | Solution |
|---------|----------|
| No serial output | Check USB cable, verify port: `ls /dev/cu.*` |
| Logs not appearing | Rebuild with `cargo clean && cargo build --release` |
| Port in use | Kill other processes: `lsof /dev/cu.usbserial-10` |
| Can't flash | Check USB connection, try unplugging and replugging |
| LED doesn't blink | Verify GPIO13 is not configured elsewhere |

---

## 🎯 Key Concepts

### Debugging Through Observation

The best debugging starts with **observation**:
1. **What do you expect to see?** (logs, LED blinking, specific values)
2. **What do you actually see?** (compare against expectation)
3. **What changed?** (isolate the problem)
4. **What would fix it?** (modify code, try again)

### Serial Logging Pattern

Professional embedded developers use this pattern:

```rust
fn do_something() {
    info!("Starting operation");           // Trace start
    let result = compute();
    info!("Computed: {}", result);         // Show result
    process(result);
    info!("Operation complete");           // Trace end
}
```

Each log is a checkpoint that proves execution reached that point.

---

## 📚 Next Steps

- **Lesson 03:** Async/await with Embassy (logging async tasks)
- **Lesson 04:** I2C Sensor Driver (debug I2C communication with logs)
- **Lesson 05:** SPI Display (debug display output)

---

## 🎯 Key Takeaways

1. ✅ **Serial logging is your first debugging tool** - Understand execution flow
2. ✅ **Logs prove code is running** - See what actually happens
3. ✅ **Output timing matters** - Logs show when things happen
4. ✅ **Structured logging is powerful** - Professional approach to debugging
5. ✅ **Hardware state = What matters** - Logs verify actual behavior
6. ✅ **Combine observation + logs** - Most effective debugging strategy

---

## 📖 References

- [Rust Embedded Book: Logging](https://docs.rust-embedded.org/book/intro/index.html)
- [esp-println Crate](https://docs.rs/esp-println/)
- [log Crate](https://docs.rs/log/)
- [Debugging Embedded Rust](https://docs.rust-embedded.org/book/debugging/index.html)

---

*Lesson 02: Learning to debug through observation and structured logging.* 🔍
