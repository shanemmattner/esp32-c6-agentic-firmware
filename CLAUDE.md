# CLAUDE.md - Guidelines for Claude Code Assistant

## Model Selection

**DEFAULT: Use Haiku Model**
- Unless explicitly told otherwise, use Claude Haiku (fastest, most cost-effective)
- Only use Sonnet or Opus if the task requires complex reasoning
- Current model: claude-haiku-4-5-20251001

**How to specify model:**
```
/model sonnet    # Switch to Sonnet
/model opus      # Switch to Opus
/model haiku     # Back to Haiku (default)
```

---

## File Operations

### ❌ Task() CANNOT Create Files
- `Task()` launches a subprocess agent for complex work
- **Agents cannot create files** - they can only read and report back
- **Don't use Task()** for file generation

### ✅ Use These Tools Instead
- `Write()` - Create new files or overwrite existing
- `Edit()` - Modify specific parts of existing files
- `Bash` - Create files via shell commands
- `Read()` - Read files before editing

### Rule of Thumb
**If you need to create/modify files → Use Write/Edit/Bash directly, NOT Task()**

---

## When to Use Task()

Task() is useful for:
- ✅ Complex research/exploration (general-purpose agent)
- ✅ Finding patterns in large codebases (Explore agent)
- ✅ Multi-step analysis and reporting
- ❌ **NOT for file creation/modification**

---

## Lean Lessons Approach

These lessons should be **simple and practical**:
- Focus on working code, not massive documentation
- Minimal READMEs (just basics)
- One main .rs file per lesson (~100-150 lines)
- Test on hardware immediately
- Keep it type-able for YouTube videos

---

## Project Conventions

### Directory Structure
```
lessons/{NN}-{name}/
├── src/
│   ├── bin/
│   │   └── main.rs          # Main firmware
│   └── lib.rs               # (optional library code)
├── .cargo/
│   └── config.toml          # Build config
├── Cargo.toml               # Dependencies
├── rust-toolchain.toml      # Toolchain
├── build.rs                 # Build script
└── README.md                # Simple docs (keep short!)
```

### Cargo.toml
- Always include `[[bin]]` section pointing to `src/bin/main.rs`
- Keep dependencies minimal
- Use esp-hal 1.0.0

### Documentation
- README.md: Keep it short (< 300 lines)
- Focus on: wiring, expected output, troubleshooting
- Skip lengthy theory - put that in code comments

---

## Hardware Testing Approach

### CRITICAL: Port Detection is NOT the Problem

**The ESP32-C6 USB-JTAG port is `/dev/cu.usbmodem1101` (or similar usbmodem device).**

If you can't see it with `ls /dev/cu.*`:
1. It's physically unplugged
2. You're looking at the wrong USB port (use the ESP32's built-in USB, not external adapters)
3. The device is in a bad state (try hard reset: unplug, hold BOOT, plug in, release BOOT)

**DO NOT** waste time trying different ports or debugging USB - if probe-rs can see it (`probe-rs list`), the port is working!

### Hardware Testing Workflow

1. **Detect device:**
```bash
ls /dev/cu.usbmodem*  # Should show /dev/cu.usbmodem1101 or similar
probe-rs list         # Should show "ESP JTAG -- 303a:1001..."
```

2. **Build firmware:**
```bash
cargo build --release
```

3. **Flash using probe-rs (RECOMMENDED - works without TTY):**
```bash
probe-rs run --chip esp32c6 target/riscv32imac-unknown-none-elf/release/main
```

4. **Alternative - Flash using espflash (requires manual port selection):**
```bash
cargo run --release  # You'll be prompted to select port interactively
```

5. **Test GPIO with probe-rs (no firmware needed!):**
```bash
# Enable GPIO12 as output
probe-rs write b32 --chip esp32c6 0x60091024 0x1000

# LED ON
probe-rs write b32 --chip esp32c6 0x60091008 0x1000

# LED OFF
probe-rs write b32 --chip esp32c6 0x6009100C 0x1000
```

### Common Pitfalls (AVOID THESE)

❌ **Don't assume espflash will work without TTY** - It requires interactive terminal for port selection
❌ **Don't try to automate espflash port selection** - Use probe-rs instead
❌ **Don't waste time with USB port detection** - If probe-rs sees it, it works
❌ **Don't flash via UART adapter** - Use the ESP32's built-in USB-JTAG (probe-rs compatible)

### When Testing Fails

If you can't get hardware working:
1. **Check probe-rs:** `probe-rs list` should show the device
2. **Try manual flashing:** Run `cargo run --release` in your actual terminal (not through me)
3. **Test with probe-rs GPIO:** Use the direct register writes above to verify LED works
4. **Check git history:** Working examples are on branch `lesson-01` (proven GDB-only blinky)

---

## Git Workflow

- Commit after each working lesson
- Keep commit messages clear and concise
- Format: `feat(lesson-{NN}): {brief description}`

---

## Bash Execution Best Practices

### Shell Limitations in LLM Context

The LLM's bash execution environment has important limitations:

**❌ Complex conditionals fail inline:**
```bash
# This will cause parse errors:
if [ $EXIT_CODE -eq 0 ]; then
    echo "success"
fi
```

**✅ Use temp scripts for complex logic:**
```bash
# This works reliably:
cat > /tmp/script.sh << 'SCRIPT'
#!/bin/bash
if [ $EXIT_CODE -eq 0 ]; then
    echo "success"
fi
SCRIPT
chmod +x /tmp/script.sh
/tmp/script.sh
```

### Variable Persistence

**Variables DON'T persist across Bash() tool calls:**
```bash
# Step 1:
export MY_VAR="value"

# Step 2 (different Bash call):
echo $MY_VAR  # Empty! Variable is gone
```

This applies to **all** variables, including those from `source`:

```bash
# Step 1:
source scripts/find-esp32-ports.sh  # Exports $USB_CDC_PORT

# Step 2 (different Bash call):
espflash flash --port $USB_CDC_PORT  # Variable is empty!
```

### Solutions for Variable Persistence

#### Option 1: Consolidate into Single Bash Call (Recommended)

```bash
# GOOD: All in one call
USB_PORT=$(ls /dev/cu.usbmodem* | head -1) && \
espflash flash --port $USB_PORT target/.../main && \
python3 read_uart.py $(ls /dev/cu.usbserial* | head -1) 5
```

#### Option 2: Re-detect in Each Step

```bash
# Step 1: Flash (detect port inline)
espflash flash --port $(ls /dev/cu.usbmodem* | head -1) target/.../main

# Step 2: Monitor (re-detect port)
python3 read_uart.py $(ls /dev/cu.usbserial* | head -1) 5
```

#### Option 3: Use Files for State (Last Resort)

```bash
# Step 1: Save to file
ls /dev/cu.usbmodem* | head -1 > /tmp/usb_port.txt

# Step 2: Read from file
python3 read_uart.py $(cat /tmp/usb_port.txt) 5
```

**Don't rely on `export` or `source`** - they don't work across tool invocations.

### When to Use Temp Scripts

Use temp scripts (`/tmp/*.sh`) for:
- Commands with if/then/fi conditionals
- Loops (for, while)
- Complex variable manipulation
- Multi-step operations with error checking

Use inline bash for:
- Simple single commands
- Command chains with `&&` or `||`
- Quick reads/writes without conditionals

---

### CRITICAL: Commands That Freeze Conversations

**NEVER use these commands - they will hang indefinitely and freeze the conversation:**

```bash
# ❌ DANGEROUS: sleep commands (blocks for N seconds)
sleep 5
sleep 10 && other_command

# ❌ DANGEROUS: timeout command (doesn't exist on macOS by default)
timeout 5 cat /dev/cu.usbserial*
timeout 10 head -20 /dev/ttyUSB0

# ❌ DANGEROUS: Reading from serial ports without timeout
cat /dev/cu.usbmodem*
head -20 /dev/cu.usbserial*
tail -f /dev/ttyUSB0

# ❌ DANGEROUS: Background processes without auto-termination
cat /dev/cu.usbserial* &
python script.py &  # Unless script has built-in timeout

# ❌ DANGEROUS: Interactive monitoring tools
espflash monitor
screen /dev/ttyUSB0
minicom
```

**✅ SAFE alternatives:**

```bash
# ✅ GOOD: Use Python script with guaranteed timeout
python3 read_uart.py /dev/cu.usbserial-FT58PFX4 5

# ✅ GOOD: Poll for completion instead of sleep
while [ ! -f /tmp/done ]; do echo "waiting..."; done

# ✅ GOOD: Use built-in command timeouts (when available)
cargo build --release  # Has implicit timeout

# ✅ GOOD: Non-blocking status checks
ps aux | grep process_name
ls -la /tmp/output.txt
```

**Why these commands are dangerous:**
- `sleep` blocks the conversation for the entire duration
- `timeout` doesn't exist on macOS (not in default PATH)
- Reading from serial devices without timeout waits forever if no data arrives
- Background processes (`&`) don't auto-terminate and can leak resources
- Interactive tools require TTY and user input, which isn't available

**Rule of thumb:** If a command might wait indefinitely for data/events, it will freeze the conversation. Always use time-bounded operations with guaranteed termination.

---

## Common Mistakes to Avoid

1. ❌ Using Task() to generate files
2. ❌ Over-engineering lessons (keep them simple!)
3. ❌ Massive documentation before working code
4. ❌ Not testing on hardware
5. ❌ Using expensive models (Sonnet/Opus) by default
6. ❌ Using complex conditionals inline in Bash (use temp scripts!)
7. ❌ Expecting variables to persist across Bash() calls (use files!)
8. ❌ **Using `sleep`, `timeout`, `cat <serial>`, `head <serial>`, or any blocking command**
9. ❌ **Reading from serial ports without guaranteed timeout**

---

## Embedded Debugging Philosophy: Virtual Debug Instrumentation

**The core insight:** Instead of being blind while firmware runs, instrument the entire system with **continuous telemetry** to get **real-time visibility into register values, state changes, and hardware behavior without stopping execution**.

### Traditional vs Data-Driven Debugging

**Traditional embedded debugging:**
- Breakpoints freeze execution (destroy timing)
- Minimal logging to "avoid overhead"
- You guess what's happening based on symptoms
- Hypothesis test each subsystem (slow, repetitive)

**Data-driven debugging with UART + GDB:**
- **Eyes (UART streaming):** See sensor values, state machines, event counters in real-time
- **Ears (GDB inspection):** Inspect register values, memory contents, peripheral state without stopping
- Everything runs live (firmware never stops, timing stays accurate)
- Patterns jump out immediately (no guessing, just observe)

### Virtual Instrumentation Mindset

Think of debugging infrastructure as placing sensors throughout your firmware:

```
Physical Hardware              Virtual Instrumentation
────────────────────          ──────────────────────────

I2C Bus           ────────→   UART: "i2c: wr=5/5 rd=5/5 errs=0"
Config Register   ────────→   GDB: (gdb) print ads_cfg_reg
ADC Output        ────────→   UART: "adc: raw=0x0ABC volts=1.234"
State FSM         ────────→   UART: "fsm: state=Reading time_ms=45"
Error Flags       ────────→   UART: "errors: i2c_timeout=0 crc=0"
```

Instead of stopping to inspect a value (breakpoint), you let the firmware run and **stream high-level state to UART** while using **GDB for on-demand deep inspection** of registers and memory.

### Data-Driven Analysis Strategy

**The core insight:** In complex embedded systems, you don't debug by hypothesis testing - you debug by **collecting all data and finding patterns**.

### Why Traditional Debugging Fails in Embedded

```
Old approach: "Button doesn't work → check button pin → check interrupt → check state machine"
Problem: You're guessing at what's wrong. What if it's actually an I2C timeout that cascades?
         Or a race condition between ISR and main loop? Or corrupted state from previous operation?
```

### Data-Driven Debugging with UART + GDB

```
New approach: "Button doesn't work → stream ALL variables (GPIO, I2C, ISR state, FSM, timers)
              via UART at 10 Hz → analyze patterns → see: 'button press → i2c_errors spike →
              sensor stops responding → LED never updates'"
Result: Root cause visible instantly. Fix is obvious: add I2C timeout recovery.
```

**GDB for deep inspection:**
```
(gdb) break i2c_error_handler
(gdb) continue
(gdb) print i2c_peripheral->status_reg
(gdb) print *button_state
(gdb) x/16x 0x60004000  # Inspect GPIO registers directly
```

### Why This Works with Claude Code

1. **Pattern matching at scale** - Claude excels at analyzing streaming telemetry
2. **Correlations reveal causality** - When variables spike together, something connects them
3. **No hypothesis needed** - Just collect data and analyze. The relationships appear naturally
4. **UART is non-blocking** - With proper buffering, timing stays accurate
5. **Structured logs** - Machine-parseable format enables automated pattern detection
6. **GDB on-demand** - Deep register/memory inspection without modifying code

### Logging Bandwidth Strategy

Instead of thinking "add minimal debug code," think in **data throughput budgets**:

```
Available UART bandwidth @ 115200 baud: ~14 KB/s (safe estimate)
Available UART bandwidth @ 921600 baud: ~100 KB/s (high-speed)

Typical variable sizes @ 10 Hz logging:
- Simple counter: ~20 bytes ("counter=1234\n")
- Multi-value log: ~60 bytes ("i2c: rd=5 wr=3 err=0 state=idle\n")

Example @ 115200 baud:
  20 variables × 30 bytes/line × 10 Hz = 6 KB/s
  This is ~40% of bandwidth, plenty of headroom

Example @ 921600 baud:
  100 variables × 30 bytes/line × 10 Hz = 30 KB/s
  This is ~30% of bandwidth, still safe
```

### When to Use This Strategy

✅ **Use maximum observability when:**
- System behavior is complex or non-obvious
- Multiple subsystems interact (I2C + GPIO + ISR + main loop)
- You're unfamiliar with the code
- Timing-sensitive bugs (UART buffering critical)
- Quick iteration needed (Claude analyzing logs is fast)

❌ **Minimize logging only when:**
- UART bandwidth saturated (switch to event counters + periodic dumps)
- Production deployment (then use minimal counters for telemetry)
- Proven simple bugs (single-subsystem issues)

### The Shift: From "Minimal Overhead" to "Maximum Insight"

Traditional embedded development: "We need to log carefully to avoid overhead"
Modern development: "We have 14-100 KB/s available via UART, let's instrument everything"

## Practical Debugging Strategies with UART + GDB

### Event Counters for High-Frequency Debugging

Track events without blocking the main firmware loop using atomic operations:

```rust
use core::sync::atomic::{AtomicU32, Ordering};

// Global event counters
static I2C_ERRORS: AtomicU32 = AtomicU32::new(0);
static GPIO_INTERRUPTS: AtomicU32 = AtomicU32::new(0);
static SENSOR_READS: AtomicU32 = AtomicU32::new(0);

// In interrupt handler or hot path:
I2C_ERRORS.fetch_add(1, Ordering::Relaxed);  // 5-10 CPU cycles, non-blocking

// Log periodically via UART (e.g., every 100ms):
println!("stats: i2c_err={} gpio_int={} sensor_rd={}",
    I2C_ERRORS.load(Ordering::Relaxed),
    GPIO_INTERRUPTS.load(Ordering::Relaxed),
    SENSOR_READS.load(Ordering::Relaxed)
);
```

**Why this works:**
- Atomic operations use hardware compare-and-swap, not locks
- `Relaxed` ordering = no synchronization overhead
- Periodic logging prevents UART buffer overflow
- Counters survive firmware resets

### GDB Memory/Register Inspection

Use GDB to inspect and modify memory at runtime without adding debug code:

```bash
# Start GDB session with ESP32-C6
espflash monitor --chip esp32c6

# While running, attach GDB in another terminal:
riscv32-esp-elf-gdb target/riscv32imac-unknown-none-elf/debug/main
(gdb) target remote :3333  # Connect to OpenOCD
(gdb) print my_global_var
(gdb) set my_global_var = 42
(gdb) continue
```

**Best practices:**

1. **Use ELF symbols** to find variable addresses:
   ```bash
   riscv32-esp-elf-nm -n target/riscv32imac-unknown-none-elf/debug/main | grep my_var
   ```

2. **Read peripheral registers directly:**
   ```bash
   # Query GPIO state without adding logging code:
   (gdb) x/1xw 0x60004000  # Read GPIO register
   (gdb) x/16x 0x60013000  # Read UART peripheral registers
   ```

3. **Set conditional breakpoints on hardware state:**
   ```bash
   (gdb) break main.rs:42 if sensor_value > 1000
   (gdb) watch i2c_error_count  # Break when variable changes
   ```

### UART Logging Patterns

**Structured logging for pattern detection:**

```rust
use esp_println::println;

// Simple counter
println!("counter={}", iteration);

// Multi-value state
println!("i2c: rd={} wr={} err={} state={}", reads, writes, errors, state);

// Sensor data
println!("sensors: temp={} humidity={} pressure={}", temp, hum, press);

// Error tracking
println!("errors: i2c_timeout={} crc={} overflow={}", i2c_to, crc_err, ovf);
```

**Why structured format matters:**
- Easy to parse with regex or scripts
- Claude can analyze patterns quickly
- Correlations jump out (e.g., "i2c_timeout spikes when temp > 80°C")

### Maximum Observability Workflow

**Step-by-step debugging approach:**

1. **Instrument everything** - Add UART logging for all key variables
2. **Run and capture** - Use `python3 read_uart.py <port> 30` to capture 30 seconds
3. **Analyze patterns** - Look for correlations, spikes, state changes
4. **Deep dive with GDB** - Inspect specific registers/memory when needed
5. **Iterate** - Fix issues, re-test with same instrumentation

**Example: Debugging I2C driver autonomously**

```rust
// UART logging for high-level state (every 100ms)
println!("i2c: status=0x{:04x} wr={} rd={} err={} scl={} sda={} state={}",
    i2c_status_reg, writes, reads, errors, scl_state, sda_state, fsm_state
);

println!("sensor: x={} y={} z={} temp={} ready={}",
    accel_x, accel_y, accel_z, temperature, sensor_ready
);
```

**GDB for deep inspection when issue found:**
```bash
# Break when I2C error detected
(gdb) break i2c_error_handler
(gdb) continue
# Now inspect peripheral state
(gdb) print i2c->status
(gdb) x/16x 0x60013000  # Dump I2C peripheral registers
(gdb) print sensor_state
```

**Why this works for autonomous debugging:**
- Claude analyzes UART logs to find patterns
- Correlations appear naturally (e.g., "button press → i2c_error → sensor_fail")
- No guessing which variable to inspect
- UART buffering keeps timing accurate
- GDB provides on-demand deep inspection without code changes

### Memory Budget Guidelines

ESP32-C6 has 512 KB SRAM total, ~400-450 KB available to user code:

| Usage | Allocation | Notes |
|-------|-----------|-------|
| **UART TX Buffer** | 1-4 KB | esp-println default buffer |
| **Event Counters** | 1-2 KB | Atomic counters for ISRs |
| **State Arrays** | 5-20 KB | Bit arrays for GPIO states, etc. |
| **Available for App** | 350-440 KB | Remaining SRAM for firmware |

**Keep it simple:** UART logging uses minimal memory compared to complex ring buffers

---

## Quick Reference

| Task | Tool | Time |
|------|------|------|
| Create lesson code | Write() + Bash | 5-10 min |
| Modify file | Edit() | 2-5 min |
| Create README | Write() | 3-5 min |
| Test on hardware | Manual | 10-20 min |
| **Avoid: Massive planning** | ~~Task()~~ | ⏱️ Don't |

---

## Slash Commands & Tools

Custom slash commands are stored in `.claude/commands/`:

### Hardware Testing Commands

- **`/test-uart-pins <tx> <rx> [duration]`** - Test UART GPIO pins on hardware
  - Examples: `/test-uart-pins 23 15 5`, `/test-uart-pins 16 17 3`
  - Creates minimal test firmware, builds, flashes, monitors output
  - Reports success/failure with troubleshooting hints
  - Auto-cleanup of temporary files

- **`/setup-hardware-lesson <number> <name>`** - Create new hardware lesson
  - Examples: `/setup-hardware-lesson 10 i2c-sensors`
  - Generates complete lesson structure with templates
  - Includes UART test binary, build files, minimal README
  - Pre-configured for esp-hal 1.0.0

- **`/test-lesson <number> [mode]`** - Unified hardware testing for lessons
  - Examples: `/test-lesson 07`, `/test-lesson 08 full`
  - Modes: `quick` (default, 3-5 min) or `full` (10-20 min)
  - Auto-detects hardware (USB ports, JTAG probes)
  - Reads lesson-specific `TEST.md` for test procedures
  - Generates comprehensive test reports

---

## Hardware Lesson Development

**CRITICAL: Always test on actual hardware as you develop. Don't write untested code.**

### Incremental Development Workflow

When creating hardware-interacting firmware, ALWAYS follow this sequence:

#### Phase 1: Minimal Viable Test (5-10 minutes)
1. **Start with template:** Copy `.claude/templates/uart_test_minimal.rs` to `src/bin/`
2. **Configure pins:** Update GPIO pin numbers for your hardware
3. **Build and flash:** `cargo build --bin uart_test_minimal`
4. **Verify output:** Use `python3 read_uart.py <port> 5` to confirm UART works
5. **Document working config:** Save pin numbers in uart-config.toml or README

#### Phase 2: Core Functionality (15-30 minutes)
1. Build on proven minimal test
2. Add one feature at a time
3. Test after each addition
4. Don't proceed if tests fail

#### Phase 3: Error Handling & Polish (10-15 minutes)
1. Add bounds checking, safety features
2. Improve error messages
3. Write comprehensive tests

### Why This Matters

❌ **BAD (Don't do this):**
```
1. Write 300-line complex firmware with DMA, slots, safety checks
2. Try to compile → API errors
3. Fix API errors
4. Flash → No output
5. Debug for 30+ minutes to find GPIO pins were wrong
```

✅ **GOOD (Do this):**
```
1. Copy uart_test_minimal.rs (47 lines)
2. Update GPIO pins
3. Flash and verify output → 5 minutes
4. Add one feature (e.g., single variable streaming) → 10 minutes
5. Add multi-variable support → 10 minutes
6. Add safety checks → 10 minutes
Total: ~35 minutes with validated checkpoints
```

### Hardware Verification Checklist

Before writing any complex firmware:

- [ ] Know which GPIO pins are connected (ask user if unsure)
- [ ] Have minimal test binary ready
- [ ] Can see output on serial port
- [ ] Verified correct TX/RX orientation (ESP TX → Adapter RX)
- [ ] Documented working configuration

### Pin Discovery Process

If you don't know which pins to use:

1. **Check documentation:**
   - Lesson-specific README
   - Previous working configurations
   - ESP32-C6 datasheet

2. **Ask user:**
   - "Which GPIO pins are connected to your UART adapter?"
   - "Is this the same setup as Lesson X?"

3. **Systematic testing:**
   - Try common pairs: GPIO16/17, GPIO23/15, GPIO4/5
   - Test both orientations (TX/RX swapped)
   - Use `scripts/test-uart-pins.sh` helper

---

## esp-hal 1.0.0 API and Documentation

**IMPORTANT: esp-hal 1.0.0 has breaking changes from pre-1.0 versions.**

### Always Fetch Latest Documentation

Instead of relying on hardcoded examples in this file, **always fetch the latest API documentation** when working with esp-hal:

```
Use WebFetch tool to get current documentation:
- https://docs.esp-rs.org/esp-hal/esp-hal/
- https://github.com/esp-rs/esp-hal/blob/main/CHANGELOG.md (for breaking changes)
- https://github.com/esp-rs/esp-hal/tree/main/examples (for working examples)
```

### Quick Reference: Common Patterns

**UART initialization (esp-hal 1.0.0+):**
```rust
// Fetch latest syntax from:
// https://docs.esp-rs.org/esp-hal/esp-hal/uart/index.html

// Basic pattern (verify against docs):
use esp_hal::uart::{Config as UartConfig, Uart};

let mut uart = Uart::new(peripherals.UART1, UartConfig::default())
    .with_tx(peripherals.GPIO23)
    .with_rx(peripherals.GPIO15);
```

**When in doubt:**
1. Check existing working code in `lessons/08-uart-gdb-tandem/src/bin/`
2. Use WebFetch to read latest esp-hal docs
3. Search esp-hal examples: https://github.com/esp-rs/esp-hal/tree/main/examples

### Migration from Pre-1.0

If you encounter old code patterns:
- `Io::new()` → No longer needed, use `peripherals.GPION` directly
- `io.pins.gpioN` → `peripherals.GPION`
- `Uart::new_with_config()` → `Uart::new().with_tx().with_rx()`

**Don't trust pre-1.0 examples** - Always verify against latest docs

---

## Serial Port Operations

**CRITICAL: Never use blocking serial operations that can freeze the conversation.**

### ✅ Recommended: Python Reader Script

Always use the provided Python script for serial monitoring:

```bash
# Auto-terminates after 5 seconds
python3 .claude/templates/read_uart.py /dev/cu.usbserial-FT58PFX4 5

# Or use device discovery
source scripts/find-esp32-ports.sh
python3 .claude/templates/read_uart.py $FTDI_PORT 5
```

**Why this works:**
- Guaranteed termination (no hanging)
- Cross-platform (macOS, Linux, Windows)
- Proper error handling
- Clean resource cleanup

### ❌ Anti-Patterns (Don't use these)

```bash
# BAD: Blocks forever, no timeout
cat /dev/cu.usbmodem1101

# BAD: Background process doesn't auto-terminate
cat /dev/cu.usbmodem1101 &

# BAD: Requires interactive TTY, fails in automation
espflash monitor /dev/cu.usbmodem1101

# BAD: macOS doesn't have GNU timeout by default
timeout 3 cat /dev/cu.usbserial-FT58PFX4
```

### Serial Port Cleanup

If you need to clean up stuck processes:

```bash
# Kill any hanging cat/espflash processes
pkill -f "cat /dev/cu\." || true
pkill -f "espflash monitor" || true
pkill -f "screen /dev/cu\." || true
```

---

## Device Discovery

**Don't hardcode serial port paths** - they change when devices are unplugged/replugged.

### Automatic Port Detection

```bash
# Use the discovery script
source scripts/find-esp32-ports.sh

# Variables are now exported:
# $USB_CDC_PORT - ESP32 USB-JTAG (for flashing/debugging)
# $FTDI_PORT    - FTDI UART (for data streaming)

# Use in commands:
espflash flash --port $USB_CDC_PORT target/.../main
python3 read_uart.py $FTDI_PORT 5
```

### Manual Detection

If discovery script doesn't work:

```bash
# macOS:
ls /dev/cu.usbmodem*    # ESP32 USB-JTAG
ls /dev/cu.usbserial*   # FTDI UART

# Linux:
ls /dev/ttyACM*         # ESP32 USB-JTAG
ls /dev/ttyUSB*         # FTDI UART
```

---

## Hardware Debugging Infrastructure

This project includes comprehensive hardware testing tools to prevent conversation freezing and streamline development:

### Templates (.claude/templates/)

- **`uart_test_minimal.rs`** - Minimal 73-line UART test firmware
  - Single-purpose: verify GPIO pins work
  - Clear pin configuration section
  - esp-hal 1.0.0 API
  - Use this FIRST before writing complex firmware

- **`read_uart.py`** - Safe UART reader with guaranteed timeout
  - Time-bounded execution (default 3-5 seconds)
  - No hanging, clean termination
  - Cross-platform (macOS, Linux, Windows)
  - Usage: `python3 read_uart.py /dev/cu.usbserial* 5`

### Scripts (scripts/)

- **`find-esp32-ports.sh`** - Automatic device discovery
  - Detects ESP32 USB-JTAG and FTDI UART ports
  - Exports `$USB_CDC_PORT` and `$FTDI_PORT`
  - Cross-platform (macOS/Linux)
  - Usage: `source scripts/find-esp32-ports.sh`

- **`test-uart-pins.sh`** - Automated GPIO pin verification
  - Creates temp project, builds, flashes, monitors
  - Reports success/failure with troubleshooting
  - Auto-cleanup
  - Usage: `./scripts/test-uart-pins.sh 23 15 5`

### Workflow Integration

**Before writing any hardware-interfacing code:**

1. Verify pins: `./scripts/test-uart-pins.sh 23 15 5`
2. If successful, copy template: `cp .claude/templates/uart_test_minimal.rs src/bin/`
3. Build on proven minimal test
4. Test after each feature addition

**Never:**
- Write 200+ lines before testing on hardware
- Assume GPIO pins without verification
- Use blocking serial commands (`cat`, `espflash monitor`)
- Hardcode serial port paths

---

**Last Updated:** 2025-11-14
**Current Work:** Lesson 08 Complete (UART + GDB Tandem Debugging)
**Infrastructure:** Hardware testing toolkit with templates, scripts, slash commands
