# Lesson 01: GDB-Only LED Blinky

**The Challenge:** Make an LED blink using ONLY GDB commands - no firmware code!

This lesson teaches **memory-mapped I/O fundamentals** and **register-level debugging** - essential skills for embedded development.

---

## What You'll Learn

1. **Register Discovery** - How to find hardware registers from PAC crate source code
2. **Memory-Mapped I/O** - How peripherals are controlled via memory addresses
3. **GDB Capabilities** - Read/write any memory, automated breakpoint commands
4. **Agentic Development** - Let Claude Code discover and control hardware for you

**Key Insight:** You don't need firmware code to control hardware - GDB can poke registers directly!

---

## Prerequisites

- ESP32-C6 DevKit with onboard LED (GPIO8)
- USB cable for programming and debugging
- GDB toolchain (`riscv32-esp-elf-gdb`)
- Debug server (`probe-rs` or `openocd`)

---

## Quick Start

###

 1. Build and Flash Blank Firmware

```bash
cd lessons/01-gdb-blinky
cargo build --release
espflash flash --port /dev/cu.usbmodem* target/riscv32imac-unknown-none-elf/release/main
```

The firmware just loops - no LED control code!

### 2. Start Debug Server

**Option A: Using probe-rs (recommended)**
```bash
# In a separate terminal
probe-rs attach --chip esp32c6 --protocol jtag target/riscv32imac-unknown-none-elf/release/main
```

**Option B: Using OpenOCD**
```bash
openocd -f board/esp32c6-builtin.cfg
```

### 3. Connect GDB and Blink

```bash
riscv32-esp-elf-gdb target/riscv32imac-unknown-none-elf/release/main

# Load automation script
(gdb) source gdb_scripts/blinky.gdb

# Start blinking!
(gdb) continue
```

The LED should blink every 500ms. Press Ctrl-C to pause.

---

## The Learning Path

### Method 1: Automated (Quick Win)

Just run `blinky.gdb` and watch it work. This proves the concept.

### Method 2: Interactive Discovery (Recommended)

Use the step-by-step guide:

```bash
(gdb) source gdb_scripts/manual_control.gdb
(gdb) step1  # Enable GPIO output
(gdb) step2  # Turn LED ON
(gdb) step3  # Turn LED OFF
(gdb) step4  # Read register state
```

### Method 3: Guided with Claude Code (Best Learning)

```
/gdb-blinky
```

Claude will guide you through:
1. Finding registers in the PAC crate
2. Calculating addresses
3. Writing GDB commands
4. Understanding what's happening

---

## How It Works

### The Register Discovery Process

**Instead of reading the 1000+ page datasheet**, we search the PAC crate:

```bash
# Find GPIO registers
python3 ../../scripts/find-registers.py GPIO
```

**Output:**
```
GPIO Peripheral
Base Address: 0x60091000

Register          Offset    Address      Description
-------------------------------------------------------------------------
OUT              0x0004    0x60091004   GPIO output register for GPIO0-31
OUT_W1TS         0x0008    0x60091008   Write 1 to set (turn ON)
OUT_W1TC         0x000C    0x6009100C   Write 1 to clear (turn OFF)
ENABLE_W1TS      0x0024    0x60091024   Enable GPIO as output
```

**For GPIO8 LED:** Bit 8 = `0x100` (or `1 << 8`)

### The GDB Magic

```gdb
# 1. Enable GPIO8 as output
(gdb) set *(uint32_t*)0x60091024 = 0x100

# 2. Turn LED ON
(gdb) set *(uint32_t*)0x60091008 = 0x100

# 3. Turn LED OFF
(gdb) set *(uint32_t*)0x6009100C = 0x100
```

**What's happening:**
- `0x60091024` = GPIO ENABLE_W1TS register (write 1 to set bits)
- `0x60091008` = GPIO OUT_W1TS register (write 1 to set output high)
- `0x6009100C` = GPIO OUT_W1TC register (write 1 to clear output low)
- `0x100` = Bit 8 mask for GPIO8

**No firmware code involved - pure register manipulation!**

---

## Automated Blinking

GDB breakpoints can run commands automatically:

```gdb
set $led_state = 0

break main.rs:54  # Breakpoint on delay loop

commands
  silent
  if $led_state == 0
    set *(uint32_t*)0x60091008 = 0x100  # ON
    set $led_state = 1
  else
    set *(uint32_t*)0x6009100C = 0x100  # OFF
    set $led_state = 0
  end
  continue
end

continue  # Start blinking
```

The LED now blinks automatically every 500ms!

---

## File Structure

```
lessons/01-gdb-blinky/
├── src/bin/main.rs           # Minimal firmware (just loops)
├── Cargo.toml                # Dependencies (minimal HAL)
├── gdb_scripts/
│   ├── blinky.gdb           # Automated blinking script
│   └── manual_control.gdb   # Step-by-step interactive guide
└── README.md                 # This file

Root:
├── GPIO_REGISTERS.md         # Complete GPIO register map
├── scripts/
│   └── find-registers.py     # Register discovery tool
└── .claude/commands/
    ├── gdb-blinky.md        # Claude Code interactive lesson
    └── find-registers.md    # Register search command
```

---

## Understanding the Firmware

```rust
#[main]
fn main() -> ! {
    let _peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    let mut loop_count: u32 = 0;
    loop {
        delay.delay_millis(500);  # Breakpoint target
        loop_count = loop_count.wrapping_add(1);
    }
}
```

**What it does:**
- Initializes the chip (clocks, peripherals)
- Provides 500ms timing loop
- That's it!

**What it doesn't do:**
- Configure GPIO
- Control the LED
- Any peripheral setup

**Why?** Because GDB will do all of that!

---

## Key Concepts

### 1. Memory-Mapped I/O

Peripherals are controlled by reading/writing specific memory addresses:

| Address | Peripheral | Register |
|---------|------------|----------|
| 0x60091000 | GPIO | Base |
| 0x60091024 | GPIO | ENABLE_W1TS |
| 0x60091008 | GPIO | OUT_W1TS |
| 0x6009100C | GPIO | OUT_W1TC |

**No special instructions needed** - just memory access!

### 2. W1TS and W1TC Registers

**W1TS** = Write 1 to Set
- Writing `0x100` sets bit 8 to 1
- Other bits unchanged

**W1TC** = Write 1 to Clear
- Writing `0x100` clears bit 8 to 0
- Other bits unchanged

**Why?** Atomic operations without read-modify-write!

### 3. GDB Can Do Anything

```gdb
# Read memory
(gdb) x/1xw 0x60091004

# Write memory
(gdb) set *(uint32_t*)0x60091004 = 0x100

# Call functions
(gdb) call some_function()

# Inject values
(gdb) set temperature = 150

# Automated commands
(gdb) break <location>
(gdb) commands ... end
```

GDB is **more powerful than most developers realize**.

---

## Troubleshooting

### LED doesn't turn on

**Check GPIO pin:**
```gdb
(gdb) x/1xw 0x60091004  # Read GPIO OUT register
```

Bit 8 should be set (0x100) when LED is on.

**Try toggling:**
```gdb
(gdb) set *(uint32_t*)0x60091008 = 0x100  # ON
# Wait a moment, check LED
(gdb) set *(uint32_t*)0x6009100C = 0x100  # OFF
```

**Verify GPIO8 is the LED pin on your board** (most ESP32-C6 DevKits use GPIO8)

### GDB can't connect

```bash
# Is debug server running?
ps aux | grep -E "probe-rs|openocd"

# Restart debug server
killall probe-rs
probe-rs attach --chip esp32c6 target/.../main
```

### Breakpoint doesn't trigger

```gdb
# Check breakpoint location
(gdb) info break

# Try different line number in main loop
(gdb) clear
(gdb) break main.rs:55
```

---

## Challenges

### Challenge 1: Multi-Speed Blinking

Can you make the LED blink at different speeds without changing firmware?

**Hint:** Adjust the breakpoint interval or add delay in GDB commands.

### Challenge 2: Button Reading

Read the button state (GPIO9) via GDB:

```gdb
# GPIO input register
(gdb) x/1xw 0x6009103C

# Check bit 9 (button is active LOW)
(gdb) print (*(uint32_t*)0x6009103C >> 9) & 1
```

Result: 0 = pressed, 1 = not pressed

### Challenge 3: Pattern Blinking

Create a pattern (3 fast blinks, 1 slow blink, repeat) using GDB commands.

### Challenge 4: Multiple GPIOs

If your board has multiple LEDs, control them simultaneously via GDB.

---

## What You've Learned

✓ Hardware is just memory-mapped registers
✓ PAC crates contain all register definitions
✓ GDB can read/write any memory address
✓ Firmware provides timing, GDB provides logic
✓ Claude Code can discover and automate this workflow

**Key Takeaway:** You can control hardware without writing control code - GDB provides a powerful direct interface.

---

## Next Lesson

**Lesson 02: High-Speed UART with DMA**

Build a data streaming peripheral using:
- GDB to develop it rapidly
- Register inspection in real-time
- Parameter tuning without reflashing
- Introduction to `/improve-command` for self-learning AI

We'll use the GDB skills from this lesson to watch UART registers as we configure them!

---

## Resources

- [ESP32-C6 Technical Reference Manual](https://www.espressif.com/sites/default/files/documentation/esp32-c6_technical_reference_manual_en.pdf)
- [esp32c6 PAC Crate](https://docs.rs/esp32c6/)
- [GDB Documentation](https://sourceware.org/gdb/documentation/)
- [probe-rs Book](https://probe.rs/)

---

## Credits

This lesson demonstrates agentic embedded development using:
- Claude Code for guided learning
- PAC-based register discovery
- GDB as a hardware control interface
- Minimal firmware for maximum learning

**You just controlled hardware with zero firmware code. That's powerful.**
