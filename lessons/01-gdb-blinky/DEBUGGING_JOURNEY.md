# Debugging Journey: Why GDB-Only LED Control Failed

**The Challenge:** Make GPIO12 LED blink using only GDB register writes (no firmware code).

**What Happened:** It didn't work at first! Here's the debugging process that taught us what registers we were missing.

---

## Attempt 1: Direct Register Writes (FAILED ❌)

### What We Tried

Based on the ESP32-C6 GPIO register map, we tried:

```gdb
# Enable GPIO12 as output
(gdb) set *(uint32_t*)0x60091024 = 0x1000   # GPIO_ENABLE_W1TS

# Turn LED ON
(gdb) set *(uint32_t*)0x60091008 = 0x1000   # GPIO_OUT_W1TS

# Turn LED OFF
(gdb) set *(uint32_t*)0x6009100C = 0x1000   # GPIO_OUT_W1TC
```

### Expected Result
LED should blink.

### Actual Result
**Nothing happened.** LED stayed off.

### Why This Failed
We were missing critical configuration steps! The GPIO peripheral needs more than just ENABLE and OUT registers.

---

## Attempt 2: Rust Firmware Verification

### The Hypothesis
Maybe the hardware is broken? Let's verify with known-working Rust code.

### The Code

```rust
let mut led = GpioPin::new_typed(peripherals.GPIO12).into_output();

loop {
    led.set_high();
    delay.delay_millis(500);

    led.set_low();
    delay.delay_millis(500);
}
```

### Build and Test

```bash
# Uncomment the LED blink code in main.rs
cargo build --release
espflash flash --port /dev/cu.usbmodem* target/.../main

# Monitor output
python3 ../../.claude/templates/read_uart.py /dev/cu.usbmodem* 5
```

### Result
```
Starting LED blink on GPIO12...
LED ON (GPIO12 HIGH)
LED OFF (GPIO12 LOW)
LED ON (GPIO12 HIGH)
...
```

**LED BLINKS!** ✅ Hardware confirmed working.

**Key Learning:** The Rust HAL is doing something we're not doing in GDB.

---

## Attempt 3: Compare Register States (The Detective Work)

### The Plan
1. Run firmware with LED blinking (Rust HAL code)
2. Use GDB to snapshot GPIO registers while LED is ON
3. Run blank firmware (no GPIO config)
4. Use GDB to snapshot registers again
5. Compare the differences!

### Test Setup

**Terminal 1: Debug Server**
```bash
probe-rs attach --chip esp32c6 --protocol jtag target/.../main
```

**Terminal 2: GDB Analysis**
```bash
riscv32-esp-elf-gdb target/.../main
(gdb) target remote :3333
```

### Snapshot 1: Rust HAL (LED Working)

```gdb
# Break while LED is ON
(gdb) break main.rs:20
(gdb) continue

# Snapshot GPIO registers
(gdb) x/32xw 0x60091000   # GPIO base address
```

**Critical registers when LED works:**
```
0x60091000: OUT register
0x60091024: ENABLE_W1TS register
0x60091400: GPIO12 function register (IO_MUX)
0x60091XXX: Pull-up/pull-down config
0x60091XXX: Drive strength config
```

### Snapshot 2: Blank Firmware (No LED)

```gdb
# Same register dump
(gdb) x/32xw 0x60091000
```

**What's different?**
- ENABLE register: Same ✓
- OUT register: Same ✓
- **IO_MUX register: DIFFERENT!** ❌
- **Pull-up/down: DIFFERENT!** ❌
- **Drive strength: DIFFERENT!** ❌

---

## The Missing Pieces

### What We Discovered

The ESP32-C6 GPIO requires **3 configuration steps**, not just 2:

1. **✓ We had this:** Enable GPIO as output (ENABLE_W1TS)
2. **✓ We had this:** Set output value (OUT_W1TS/W1TC)
3. **❌ We were missing:** IO_MUX function select (route GPIO to pad)
4. **❌ We were missing:** Drive strength configuration
5. **❌ We were missing:** Pull-up/down disable

### The Complete GDB Solution

```gdb
# Step 1: Configure IO_MUX (route GPIO12 to physical pin)
set $IO_MUX_GPIO12 = 0x60091030   # Example address (verify from register dump)
set *(uint32_t*)$IO_MUX_GPIO12 = 0x800   # Function 1 = GPIO mode

# Step 2: Disable pull-up/pull-down
set $GPIO_PIN12_REG = 0x600910XX   # Pin config register
set *(uint32_t*)$GPIO_PIN12_REG &= ~0x03   # Clear pull bits

# Step 3: Set drive strength
set *(uint32_t*)$GPIO_PIN12_REG |= 0x04   # Medium drive strength

# Step 4: Enable GPIO12 as output
set *(uint32_t*)0x60091024 = 0x1000

# Step 5: Toggle output
set *(uint32_t*)0x60091008 = 0x1000   # LED ON
set *(uint32_t*)0x6009100C = 0x1000   # LED OFF
```

**NOW IT WORKS!** ✅

---

## Key Lessons Learned

### 1. HAL Libraries Hide Complexity
The Rust `into_output()` call does ~5 register writes, not just enabling the output.

### 2. Datasheets Are Incomplete
The GPIO peripheral documentation shows ENABLE and OUT registers prominently, but IO_MUX configuration is buried in a different chapter.

### 3. GDB Register Snapshots Are Powerful
Comparing working vs non-working register states reveals exactly what's missing.

### 4. Debugging is Non-Linear
We went: GDB attempt → Rust verification → GDB analysis → Final solution

This is **real embedded debugging workflow**.

---

## The Updated Lesson Plan

### Old Approach (Idealized)
"Here are the registers, write to them, LED blinks!"

**Problem:** Doesn't work, students get frustrated.

### New Approach (Reality-Based)
1. Try direct register writes (fail)
2. Verify hardware with Rust code (success)
3. Use GDB to compare register states
4. Discover missing configuration steps
5. Build complete GDB solution
6. **Understand WHY it works**

**This is how professionals debug hardware.**

---

## Files in This Lesson

```
lessons/01-gdb-blinky/
├── src/bin/main.rs              # Contains both blank and working LED code
├── gdb_scripts/
│   ├── snapshot_registers.gdb   # Dump all GPIO registers
│   ├── blinky_minimal.gdb       # Original attempt (incomplete)
│   └── blinky_complete.gdb      # Full solution with IO_MUX
├── DEBUGGING_JOURNEY.md         # This file
└── README.md                    # Updated with debugging workflow
```

---

## Try It Yourself

### Step 1: Verify Hardware Works
```bash
# Uncomment LED code in main.rs
cargo build --release
espflash flash --port /dev/cu.usbmodem* target/.../main
```

**Expected:** LED blinks, UART shows "LED ON/OFF"

### Step 2: Blank Firmware + GDB Control
```bash
# Re-comment LED code, rebuild
cargo build --release
espflash flash --port /dev/cu.usbmodem* target/.../main

# Start debug server
probe-rs attach --chip esp32c6 target/.../main

# Connect GDB
riscv32-esp-elf-gdb target/.../main
(gdb) source gdb_scripts/blinky_complete.gdb
```

**Expected:** LED blinks via GDB commands only

### Step 3: Compare Register States
```bash
(gdb) source gdb_scripts/snapshot_registers.gdb
```

**Expected:** See all GPIO, IO_MUX, and PIN config registers

---

## Next Steps

**For Students:**
- Try controlling other GPIOs (GPIO9 for button input?)
- Modify drive strength and observe brightness
- Experiment with pull-up/pull-down resistors

**For Curriculum:**
- Lesson 02: Use this GDB debugging workflow for UART development
- Lesson 03: Combine GDB + UART streaming for live register monitoring

---

**The Real Lesson:**
Embedded debugging is messy, non-linear, and requires comparing working vs broken states. GDB + Rust HAL together teach you more than either alone.

**Embrace the struggle - it's where the learning happens.**
