# Lesson 07: GDB Debugging with Rust on ESP32-C6

Master professional embedded debugging with GDB, OpenOCD, and AI-assisted iterative development.

## The Philosophy: Feedback Loops for LLM Development

### Why Feedback is Critical

When developing embedded systems with AI assistance (like Claude Code), the **feedback loop is everything**. Without real feedback from the hardware, an LLM is essentially blind - it can write code that looks correct but has no way to verify it actually works.

**The Problem:**
- LLMs can generate syntactically correct code
- But they can't know if the LED actually lights up
- Or if the I2C communication is working
- Or why the system crashes after 30 seconds

**The Solution:**
Two complementary feedback mechanisms working together:

1. **UART/USB Serial Output** - High-level behavioral feedback
2. **GDB Debugger** - Deep state inspection and root cause analysis

### The Iterative Development Process

With proper feedback, Claude Code can iteratively develop and self-correct:

```
┌─────────────────────────────────────────────────────┐
│  1. GENERATE CODE                                   │
│     Claude writes firmware based on requirements    │
└─────────────────┬───────────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────────┐
│  2. BUILD & FLASH                                   │
│     Compile and upload to ESP32-C6                  │
└─────────────────┬───────────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────────┐
│  3. OBSERVE BEHAVIOR (UART)                         │
│     Watch serial output: Does it boot? Log messages?│
│     Status: "IMU init failed", "Button working"     │
└─────────────────┬───────────────────────────────────┘
                  │
                  ▼
           ┌──────┴──────┐
           │   Working?   │
           └──────┬──────┘
                  │
        ┌─────────┼─────────┐
        │ YES               │ NO
        ▼                   ▼
    ┌───────┐    ┌──────────────────────────────────┐
    │ DONE  │    │  4. DEBUG WITH GDB                │
    └───────┘    │     Attach debugger, set breaks   │
                 │     Inspect variables, registers  │
                 │     Find ROOT CAUSE               │
                 └──────────┬───────────────────────┘
                            │
                            ▼
                 ┌──────────────────────────────────┐
                 │  5. ANALYZE & FIX                 │
                 │     Claude reads GDB output       │
                 │     Understands what went wrong   │
                 │     Generates targeted fix        │
                 └──────────┬───────────────────────┘
                            │
                            └──────> BACK TO STEP 1
```

### UART vs GDB: When to Use What

Both mechanisms provide feedback, but they serve different purposes:

#### UART/USB Serial Output

**Best For:**
- ✅ High-level system behavior monitoring
- ✅ Continuous data logging (sensor readings, timing)
- ✅ Application-level status ("Ready", "Connecting...", "Error 42")
- ✅ Performance metrics (loop rate, latency)
- ✅ Long-running observations (hours/days)
- ✅ Production debugging (field deployed systems)
- ✅ User-facing diagnostics

**Characteristics:**
- **Asynchronous** - doesn't stop program execution
- **High-throughput** - can stream megabytes of data
- **Non-invasive** - minimal impact on timing
- **Limited depth** - only shows what you explicitly print

**Example Use Cases:**
```
[INFO] System boot complete
[INFO] IMU initialized: WHO_AM_I = 0x71
[DATA] Accel: x=245, y=-102, z=16384
[WARN] Button debounce triggered
[ERROR] I2C timeout on address 0x68
```

#### GDB Debugger

**Best For:**
- ✅ Understanding **why** something failed
- ✅ Inspecting internal state (variables, memory, registers)
- ✅ Peripheral hardware analysis (I2C registers, GPIO state)
- ✅ Call stack analysis (trace execution path)
- ✅ Memory corruption detection
- ✅ Race conditions and timing issues
- ✅ Precise execution control (step-by-step)
- ✅ Hardware-level debugging (fault registers, exceptions)

**Characteristics:**
- **Synchronous** - stops execution at breakpoints
- **Deep inspection** - see everything in memory
- **Invasive** - changes timing behavior
- **Precise** - exact program counter, exact state

**Example Use Cases:**
```
(gdb) break mpu9250::read_accel
Breakpoint 1, mpu9250::read_accel (i2c=0x3fcab234) at mpu9250.rs:45
45          let mut buffer = [0u8; 6];
(gdb) print i2c->status
$1 = 0x00000421  // Bit 0: BUSY, Bit 5: TIMEOUT
(gdb) x/6xb buffer
0x3fcab240: 0x00 0x00 0x00 0x00 0x00 0x00  // Buffer is empty!
```

### Using Both Together: The Complete Picture

The most powerful workflow uses **both mechanisms simultaneously**:

**Scenario: IMU readings are all zero**

1. **UART says:**
   ```
   [INFO] IMU initialized successfully
   [DATA] Accel: x=0, y=0, z=0
   [DATA] Accel: x=0, y=0, z=0
   ```
   → We know the symptom but not the cause

2. **GDB shows:**
   ```
   (gdb) break mpu9250::read_accel
   (gdb) print *i2c_status
   $1 = 0x00000420  // Bit 5 set = TIMEOUT
   (gdb) x/1xw 0x60013004  // I2C0 status register
   0x60013004: 0x00000020  // Hardware timeout!
   (gdb) print scl_pin
   $2 = 11
   (gdb) print sda_pin
   $3 = 2
   ```
   → Ah! I2C hardware is timing out. Pins are correct.
   → Check physical wiring or pull-ups

**Claude Code can now:**
1. See the zero readings from UART (knows there's a problem)
2. Read GDB state (understands it's an I2C timeout)
3. Suggest: "Check SCL/SDA pull-up resistors on GPIO2 and GPIO11"
4. Or propose: "Increase I2C timeout in config"

### Teaching Claude to Debug Itself

With GDB access, Claude Code can **iteratively test and fix its own code**:

**Example Conversation:**

**User:** "The LED doesn't turn on when I press the button"

**Claude:** "Let me debug this. I'll set breakpoints and check the state."

```
(gdb) break button::button_task
Breakpoint 1 at 0x42001234: file button.rs, line 45.
(gdb) continue
Breakpoint 1, button::button_task (button=0x3fcab100) at button.rs:45
45          let current_state = button.is_low();
(gdb) print current_state
$1 = true  // Button IS pressed
(gdb) print LAST_STATE
$2 = true  // But previous state was also pressed!
(gdb) print LED_ON
$3 = false  // LED never toggled
```

**Claude:** "I found the bug! The button debouncing logic doesn't detect edges properly. It only checks current state, not transitions. Let me fix this..."

```rust
// OLD CODE (broken)
if button.is_low() {
    LED_ON = !LED_ON;  // Toggles continuously while held!
}

// NEW CODE (fixed)
let current = button.is_low();
if current && !LAST_STATE {  // Edge detection
    LED_ON = !LED_ON;
}
LAST_STATE = current;
```

**Claude:** "Let me rebuild and test..."
[Builds, flashes, tests with GDB]
**Claude:** "Fixed! The LED now toggles once per button press."

### Development Speed & Accuracy

This feedback-driven approach dramatically improves:

**Speed:**
- No guessing - GDB shows exact state
- Fewer iterations - targeted fixes instead of trial-and-error
- Automated testing - Claude can verify fixes immediately

**Accuracy:**
- Root cause analysis - not just symptom treatment
- Hardware-level visibility - see peripheral registers
- Memory-safe - detect buffer overflows before they cause crashes

### What This Lesson Teaches

1. **How to use GDB effectively** on ESP32-C6 with Rust
2. **What GDB can inspect** (variables, memory, peripherals, registers)
3. **How to combine UART + GDB** for complete visibility
4. **How to teach Claude to use GDB** for self-debugging
5. **Agentic workflows** for iterative development

With these tools, you're not just writing firmware - you're building a **self-improving development system** where the AI assistant can observe, analyze, and correct its own code.

---

## Hardware Setup

### Components
- ESP32-C6-WROOM DevKit
- MPU9250 9-DOF IMU module (I2C)
- WS2812 NeoPixel LED
- Push button (active LOW with pull-up)
- USB-to-serial adapter (for UART terminal)
- **Optional:** External JTAG debugger (or use built-in USB-JTAG)

### Wiring

| Component | ESP32-C6 GPIO | Notes |
|-----------|---------------|-------|
| Button    | GPIO9         | Active LOW with pull-up |
| NeoPixel  | GPIO8         | Onboard WS2812 |
| MPU9250 SDA | GPIO2       | I2C data |
| MPU9250 SCL | GPIO11      | I2C clock |
| UART TX   | GPIO15        | To RX on USB-serial |
| UART RX   | GPIO23        | To TX on USB-serial |
| JTAG TMS  | GPIO4         | Optional external |
| JTAG TDI  | GPIO5         | Optional external |
| JTAG TDO  | GPIO6         | Optional external |
| JTAG TCK  | GPIO7         | Optional external |

## Prerequisites

### 1. Install RISC-V GDB

```bash
# macOS
brew install riscv-gnu-toolchain

# Linux
sudo apt install gdb-multiarch

# Or Espressif's toolchain
# https://github.com/espressif/crosstool-NG/releases
```

Verify:
```bash
riscv32-esp-elf-gdb --version
```

### 2. Install OpenOCD

```bash
# macOS
brew install openocd

# Linux
sudo apt install openocd

# Or Espressif's OpenOCD (recommended for ESP32-C6)
# https://github.com/espressif/openocd-esp32/releases
```

Verify:
```bash
openocd --version
```

### 3. Build Firmware

```bash
cd lessons/07-gdb-debugging
cargo build  # Use debug build for full symbols
```

---

## Part 1: GDB Core Capabilities

[... rest of the comprehensive guide continues ...]

### 1.1 Basic Debugging

**Start OpenOCD (Terminal 1):**
```bash
cd lessons/07-gdb-debugging
openocd -f board/esp32c6.cfg
```

**Start GDB (Terminal 2):**
```bash
riscv32-esp-elf-gdb target/riscv32imc-unknown-none-elf/debug/main
```

The `.gdbinit` file will automatically:
- Set RISC-V 32-bit architecture
- Connect to OpenOCD at :3333
- Load the firmware
- Enable Rust pretty-printing

**Basic Commands:**
```gdb
(gdb) break main              # Set breakpoint at main
(gdb) break handle_command    # Set breakpoint at function
(gdb) break src/bin/main.rs:123  # Set breakpoint at line
(gdb) info breakpoints        # List all breakpoints
(gdb) delete 1                # Delete breakpoint #1
(gdb) continue                # Resume execution
(gdb) step                    # Step into function
(gdb) next                    # Step over function
(gdb) finish                  # Run until function returns
(gdb) quit                    # Exit GDB
```

### 1.2 Variable Inspection

```gdb
(gdb) print led_on            # Print variable value
(gdb) print/x led_on          # Print in hexadecimal
(gdb) print/t led_on          # Print in binary
(gdb) print/d led_on          # Print in decimal
(gdb) ptype led_color         # Print type information
(gdb) info locals             # Show all local variables
(gdb) info args               # Show function arguments
(gdb) whatis led_on           # Show variable type
```

**Rust-specific:**
```gdb
(gdb) set print asm-demangle on  # Demangle Rust names
(gdb) set demangle-style rust    # Use Rust demangling
(gdb) set print pretty on        # Pretty-print structures
```

### 1.3 Call Stack Analysis

```gdb
(gdb) backtrace              # Show full call stack (bt)
(gdb) backtrace full         # Show stack with local vars
(gdb) frame 2                # Switch to frame #2
(gdb) up                     # Move up one frame
(gdb) down                   # Move down one frame
(gdb) info frame             # Show current frame info
(gdb) list                   # Show source code at current location
```

### 1.4 Memory Inspection

```gdb
# Examine memory at address
(gdb) x/10x 0x3FC88000       # 10 words in hex
(gdb) x/20b 0x3FC88000       # 20 bytes
(gdb) x/10i $pc              # 10 instructions at PC

# Format specifiers:
#   x = hex, d = decimal, u = unsigned, t = binary
#   o = octal, a = address, c = char, s = string
#   i = instruction

# Size specifiers:
#   b = byte (8-bit), h = halfword (16-bit)
#   w = word (32-bit), g = giant (64-bit)

# Display memory in various formats
(gdb) x/4xw $sp              # 4 words from stack pointer
(gdb) x/s 0x3FC88000         # String at address
(gdb) x/100x &cmd_buffer     # Examine command buffer
```

### 1.5 Peripheral Register Inspection

ESP32-C6 peripheral base addresses (from Technical Reference Manual):

| Peripheral | Base Address | Key Registers |
|------------|--------------|---------------|
| I2C0       | 0x60013000   | STATUS (0x04), FIFO_DATA (0x14) |
| GPIO       | 0x60004000   | OUT (0x04), IN (0x3C), ENABLE (0x20) |
| UART1      | 0x60010000   | STATUS (0x1C), FIFO (0x00) |
| RMT        | 0x60006000   | CHnDATA (0x00-0x1C) |

```gdb
# Check I2C0 status
(gdb) x/1xw 0x60013004       # I2C0 STATUS register
(gdb) x/1xw 0x60013014       # I2C0 FIFO_DATA register

# Check GPIO output and input
(gdb) x/1xw 0x60004004       # GPIO OUT register
(gdb) x/1xw 0x6000403C       # GPIO IN register
(gdb) x/1xw 0x60004020       # GPIO ENABLE register

# Check UART1 status
(gdb) x/1xw 0x6001001C       # UART1 STATUS register

# Use custom command (from .gdbinit)
(gdb) show-peripherals       # Display I2C and GPIO registers
```

### 1.6 Watchpoints (Hardware)

ESP32-C6 supports hardware watchpoints (typically 2-4):

```gdb
(gdb) watch led_on           # Break when variable changes
(gdb) watch -l led_on        # Watch memory location
(gdb) rwatch led_on          # Break on read
(gdb) awatch led_on          # Break on read or write
(gdb) info watchpoints       # List watchpoints
(gdb) delete 2               # Delete watchpoint #2

# Watch memory address directly
(gdb) watch *0x3FC88100      # Watch 4 bytes at address
(gdb) watch *(char*)0x3FC88100  # Watch 1 byte
```

**Note:** Hardware watchpoints don't slow execution, but you're limited to 2-4 depending on the CPU.

### 1.7 Conditional Breakpoints

Break only when specific conditions are true:

```gdb
# Break when variable meets condition
(gdb) break handle_command if cmd_len > 10

# Break on specific loop iteration
(gdb) break main.rs:200 if loop_count == 50

# Break when pointer is null
(gdb) break mpu9250.rs:45 if sensor_data == 0

# Complex conditions
(gdb) break read_sensors if accel_x > 1000 && accel_y < -500

# Modify existing breakpoint
(gdb) condition 1 loop_count == 100  # Add condition to breakpoint 1
(gdb) condition 1                    # Remove condition
```

### 1.8 Breakpoint Commands (Automation)

Auto-execute commands when breakpoint hits:

```gdb
# Set breakpoint with commands
(gdb) break read_sensors
(gdb) commands
  > silent              # Don't print breakpoint hit message
  > print accel_x
  > print accel_y
  > print accel_z
  > continue            # Auto-resume execution
  > end

# More complex example
(gdb) break handle_command
(gdb) commands
  > printf "Command: %s (len=%d)\n", cmd_buffer, cmd_len
  > if cmd_len > 20
    > print "Command too long!"
    > backtrace
  > else
    > continue
  > end
  > end
```

This creates a "tracepoint" that logs data without stopping.

### 1.9 Display Expressions (Auto-Show)

Automatically display values on every stop:

```gdb
(gdb) display led_on         # Show led_on every time we stop
(gdb) display loop_count
(gdb) display /x *0x60013004  # Display I2C status in hex
(gdb) info display           # List all displays
(gdb) delete display 1       # Remove display #1
(gdb) disable display 2      # Temporarily disable
(gdb) enable display 2       # Re-enable
```

### 1.10 Register Analysis

```gdb
(gdb) info registers         # Show all general-purpose registers
(gdb) info all-registers     # Include special/system registers
(gdb) print $pc              # Program counter
(gdb) print $sp              # Stack pointer
(gdb) print $ra              # Return address (RISC-V)
(gdb) print $a0              # Argument register 0
(gdb) print $t0              # Temporary register 0

# Modify registers (use with caution!)
(gdb) set $pc = 0x42000100   # Jump to address
(gdb) set $a0 = 42           # Change function argument
```

---

## Part 2: Advanced GDB Features

### 2.1 Custom GDB Commands

Create reusable command sequences with `define`:

```gdb
# Define a custom command
define show-imu
    printf "IMU Data:\n"
    printf "  accel_x: %d\n", accel_x
    printf "  accel_y: %d\n", accel_y
    printf "  accel_z: %d\n", accel_z
end

# Use it:
(gdb) show-imu
IMU Data:
  accel_x: 245
  accel_y: -102
  accel_z: 16384

# Command with parameters
define show-mem
    if $argc != 1
        printf "Usage: show-mem <address>\n"
    else
        x/16xw $arg0
    end
end

(gdb) show-mem 0x3FC88000
```

Add these to `.gdbinit` to make them permanent.

### 2.2 GDB Python Scripts

Python provides more powerful scripting. Create `gdb_helpers.py`:

```python
import gdb

class ShowPeripherals(gdb.Command):
    """Show ESP32-C6 peripheral register status"""

    def __init__(self):
        super(ShowPeripherals, self).__init__("show-periph", gdb.COMMAND_USER)

    def invoke(self, arg, from_tty):
        # Read I2C0 registers
        i2c_status = int(gdb.parse_and_eval("*(unsigned int*)0x60013004"))
        print(f"I2C0 STATUS: 0x{i2c_status:08x}")
        
        if i2c_status & 0x01:
            print("  [BUSY] I2C transaction in progress")
        if i2c_status & 0x20:
            print("  [TIMEOUT] I2C timeout occurred")
        
        # Read GPIO registers
        gpio_out = int(gdb.parse_and_eval("*(unsigned int*)0x60004004"))
        gpio_in = int(gdb.parse_and_eval("*(unsigned int*)0x6000403C"))
        
        print(f"\nGPIO OUT: 0x{gpio_out:08x}")
        print(f"  GPIO8 (NeoPixel): {'HIGH' if gpio_out & (1<<8) else 'LOW'}")
        
        print(f"\nGPIO IN:  0x{gpio_in:08x}")
        print(f"  GPIO9 (Button): {'HIGH' if gpio_in & (1<<9) else 'LOW (pressed)'}")

class DumpIMU(gdb.Command):
    """Read and decode MPU9250 registers via I2C"""
    
    def __init__(self):
        super(DumpIMU, self).__init__("dump-imu", gdb.COMMAND_USER)
    
    def invoke(self, arg, from_tty):
        # This would interact with I2C to read IMU registers
        print("Reading MPU9250 registers...")
        print("(Implementation would use I2C peripheral registers)")

# Register commands
ShowPeripherals()
DumpIMU()

# Rust pretty-printers
class RustStringPrinter:
    """Pretty-print Rust String"""
    def __init__(self, val):
        self.val = val
    
    def to_string(self):
        data = self.val['vec']['buf']['ptr']['pointer']
        length = int(self.val['vec']['len'])
        return data.string(length=length)

# Register pretty-printers
# (More complex, see GDB Python API docs)
```

Load in GDB:
```gdb
(gdb) source gdb_helpers.py
(gdb) show-periph
I2C0 STATUS: 0x00000001
  [BUSY] I2C transaction in progress
GPIO OUT: 0x00000100
  GPIO8 (NeoPixel): HIGH
GPIO IN:  0x00000000
  GPIO9 (Button): LOW (pressed)
```

### 2.3 Memory Dumps

```gdb
# Dump memory region to file
(gdb) dump binary memory ram.bin 0x3FC88000 0x3FC90000
(gdb) dump ihex memory flash.hex 0x42000000 0x42010000
(gdb) dump srec memory code.srec 0x42000000 0x42001000

# Restore memory from file
(gdb) restore ram.bin binary 0x3FC88000

# Append to existing file
(gdb) append binary memory ram.bin 0x3FC90000 0x3FC98000
```

### 2.4 Core Dumps & Post-Mortem Analysis

```gdb
# Generate core dump
(gdb) generate-core-file esp32_crash.core

# Later, analyze the core dump
$ riscv32-esp-elf-gdb target/riscv32imc-unknown-none-elf/debug/main esp32_crash.core
(gdb) backtrace
(gdb) info locals
(gdb) print fault_register
```

### 2.5 Reverse Execution (Limited Support)

**Note:** Reverse execution is typically not supported on embedded targets, but worth trying:

```gdb
(gdb) record              # Start recording execution
(gdb) reverse-continue    # Run backwards to previous breakpoint
(gdb) reverse-step        # Step backwards
(gdb) reverse-next        # Step over backwards
```

If not supported, you'll see: "Target doesn't support this command."

### 2.6 Flash Programming via GDB

```gdb
# Reload firmware
(gdb) load

# Reset target
(gdb) monitor reset halt
(gdb) monitor reset init

# Continue execution
(gdb) continue
```

### 2.7 Logging GDB Sessions

```gdb
# Enable logging
(gdb) set logging file gdb_session.log
(gdb) set logging on
Copying output to gdb_session.log.

# All output now goes to both terminal and file

# Disable logging
(gdb) set logging off
Done logging to gdb_session.log.
```

This is **critical for AI analysis** - share the log with Claude!

---

## Part 3: What LLMs Can Help With

Based on research (ChatDBG, GDB MCP servers), here's what LLMs excel at:

### 3.1 Explaining Current State

**Human asks:** "I'm stopped at line 234 in handle_command. What's happening?"

**LLM analyzes GDB output:**
```gdb
(gdb) info locals
led_on = true
cmd_len = 15
cmd_buffer = "led_color 255 0 0"
```

**LLM explains:**
> "You're in the handle_command function, which just parsed the command `led_color 255 0 0`. The LED is currently on (led_on=true). The code is about to call the LED color setting function with RGB values (255, 0, 0) - this will set the LED to red."

### 3.2 Root Cause Analysis

**Human:** "The program crashed at 0x420012A4"

**LLM reads:**
```gdb
(gdb) backtrace
#0  0x420012a4 in mpu9250::read_accel (i2c=0x0) at mpu9250.rs:67
#1  0x42001580 in handle_command () at main.rs:234
#2  0x420015c0 in main () at main.rs:156

(gdb) print i2c
$1 = (I2c *) 0x0
```

**LLM diagnoses:**
> "The crash is a null pointer dereference in mpu9250::read_accel. The i2c parameter is 0x0 (null). Looking at the call chain, handle_command called read_accel without passing a valid I2C handle. Check line 234 in main.rs - you're likely passing None or an uninitialized variable."

### 3.3 Suggesting Fixes

**Human:** "I have a buffer overflow at line 156. cmd_len is 130 but buffer is 128 bytes."

**LLM sees:**
```rust
let mut cmd_buffer: [u8; 128] = [0; 128];
let cmd_len = uart.read(&mut cmd_buffer)?; // Returns 130!
```

**LLM suggests:**
```rust
// Fix 1: Limit read size
let cmd_len = uart.read(&mut cmd_buffer[..127])?;
cmd_buffer[cmd_len] = 0; // Null terminate

// Fix 2: Check return value
let bytes_read = uart.read(&mut cmd_buffer)?;
let cmd_len = bytes_read.min(127);

// Fix 3: Use larger buffer
let mut cmd_buffer: [u8; 256] = [0; 256];
```

### 3.4 Writing Custom GDB Scripts

**Human:** "Write a GDB Python script that checks if the MPU9250 is responding."

**LLM generates:**
```python
import gdb

class CheckIMU(gdb.Command):
    """Check if MPU9250 is responding via I2C"""
    
    def __init__(self):
        super(CheckIMU, self).__init__("check-imu", gdb.COMMAND_USER)
    
    def invoke(self, arg, from_tty):
        # MPU9250 I2C address: 0x68
        # WHO_AM_I register: 0x75
        # Expected value: 0x71
        
        print("Checking MPU9250 via I2C...")
        
        # Read I2C status
        i2c_status = int(gdb.parse_and_eval("*(unsigned int*)0x60013004"))
        
        if i2c_status & 0x01:
            print("❌ I2C is BUSY")
            return
        
        if i2c_status & 0x20:
            print("❌ I2C TIMEOUT detected")
            return
        
        print("✓ I2C peripheral is idle and ready")
        
        # In a real implementation, you'd:
        # 1. Write device address + register to I2C FIFO
        # 2. Trigger read transaction
        # 3. Wait for completion
        # 4. Read result from FIFO
        
        print("To fully test: Set breakpoint in mpu9250::read_who_am_i")
        print("  (gdb) break mpu9250::read_who_am_i")
        print("  (gdb) continue")
        print("  (gdb) finish")
        print("  (gdb) print $_return")

CheckIMU()
```

### 3.5 Interpreting Register Values

**Human:** "The I2C0 STATUS register shows 0x00000421. What does it mean?"

**LLM decodes (from ESP32-C6 TRM):**
> "I2C0 STATUS (0x00000421):
> - Bit 0: BUSY = 1 (transaction in progress)
> - Bit 5: TIMEOUT = 1 (timeout occurred)
> - Bit 10: ACK_ERR = 1 (slave didn't acknowledge)
> 
> This means: The I2C bus is busy with a transaction that timed out, and the slave device (probably MPU9250 at 0x68) didn't acknowledge. Check:
> 1. Is the MPU9250 powered?
> 2. Are SDA/SCL pins correct (GPIO2/GPIO11)?
> 3. Are pull-up resistors present (4.7kΩ recommended)?
> 4. Is the I2C address correct (0x68 or 0x69)?"

### 3.6 Memory Pattern Analysis

**Human:** "Dump memory at 0x3FC88000 and analyze."

```gdb
(gdb) x/64xb 0x3FC88000
0x3fc88000: 0x48 0x65 0x6c 0x6c 0x6f 0x00 0x00 0x00
0x3fc88008: 0x00 0x00 0x00 0x00 0xa5 0xa5 0xa5 0xa5
0x3fc88010: 0xa5 0xa5 0xa5 0xa5 0xff 0xff 0xff 0xff
...
```

**LLM analyzes:**
> "Memory pattern at 0x3FC88000:
> - Bytes 0-5: ASCII string 'Hello' (0x48='H', 0x65='e', etc.)
> - Bytes 6-11: Zero-filled (unused/cleared)
> - Bytes 12-19: Pattern 0xA5 (common debug fill pattern)
> - Bytes 20+: All 0xFF (uninitialized or erased flash)
> 
> This looks like a string buffer followed by guard bytes (0xA5), which is good - no buffer overflow detected. The 0xFF pattern suggests memory beyond the buffer hasn't been written to."

### 3.7 Performance Profiling

**Human:** "The main loop runs at 8 Hz instead of 10 Hz. Find the bottleneck."

**LLM guides:**
```gdb
# Step 1: Measure time at key points
(gdb) break main_loop_start
(gdb) commands
  > silent
  > print $start_time = *(unsigned int*)0x60008004  # Read timer
  > continue
  > end

(gdb) break main_loop_end
(gdb) commands
  > silent
  > print $end_time = *(unsigned int*)0x60008004
  > print $end_time - $start_time
  > continue
  > end

# Step 2: Profile each function
(gdb) break read_sensors
(gdb) commands
  > silent
  > print "read_sensors start"
  > finish
  > print "read_sensors took: XXX cycles"
  > continue
  > end
```

**LLM suggests optimizations:**
> "Based on timing data, read_sensors takes 80ms instead of expected 10ms. This is the bottleneck. Check:
> - I2C clock speed (increase from 100kHz to 400kHz)
> - I2C timeout settings (may be waiting unnecessarily)
> - Remove debug logging inside read_sensors (adds overhead)"

---

## Part 4: Test Scenarios

Verify GDB capabilities with these hands-on tests:

### Test 1: Basic Breakpoint ✓
```gdb
(gdb) break main
(gdb) run
# Should stop at main entry
(gdb) continue
```

### Test 2: Function Breakpoint ✓
```gdb
(gdb) break handle_command
(gdb) continue
# Send UART command: "status"
# Should break in handle_command
(gdb) print cmd_buffer
(gdb) continue
```

### Test 3: Watchpoint on Variable ✓
```gdb
(gdb) watch LED_ON
(gdb) continue
# Press button or send "led_on"
# Should break when LED_ON changes
(gdb) print LED_ON
```

### Test 4: Peripheral Register Read ✓
```gdb
(gdb) x/1xw 0x60013004  # I2C0 STATUS
(gdb) x/1xw 0x6000403C  # GPIO IN
(gdb) show-peripherals  # Custom command
```

### Test 5: Memory Inspection ✓
```gdb
(gdb) x/4xw $sp         # Stack
(gdb) x/128xb &cmd_buffer  # Command buffer
(gdb) x/10i $pc         # Instructions
```

### Test 6: Call Stack ✓
```gdb
(gdb) break mpu9250::read_accel
(gdb) continue
# Send "imu_read" via UART
(gdb) backtrace
(gdb) frame 1
(gdb) info locals
```

### Test 7: Conditional Breakpoint ✓
```gdb
(gdb) break handle_command if cmd_len > 5
(gdb) continue
# Send short command: "help" (shouldn't break)
# Send long command: "led_color 255 0 0" (should break)
```

### Test 8: Breakpoint Commands ✓
```gdb
(gdb) break read_sensors
(gdb) commands
  > silent
  > print accel_x
  > print accel_y
  > print accel_z
  > continue
  > end
(gdb) continue
# Should print values without stopping
```

### Test 9: Custom GDB Command ✓
```gdb
define show-status
    printf "LED: %s\n", LED_ON ? "ON" : "OFF"
    printf "Loop count: %u\n", loop_count
end

(gdb) show-status
```

### Test 10: Python Script ✓
```gdb
(gdb) source gdb_helpers.py
(gdb) show-periph
# Should display I2C and GPIO register states
```

---

## Part 5: GDB Python Helper Scripts

We'll create these scripts based on testing results:

1. **esp32c6_peripherals.py** - Show all peripheral registers
2. **imu_decoder.py** - Decode MPU9250 register values
3. **memory_analyzer.py** - Detect buffer overflows, stack issues
4. **trace_logger.py** - Log function calls without stopping
5. **uart_monitor.py** - Display UART buffer contents

*Scripts will be created after hands-on testing in Step 3*

---

## Part 6: .claude/ Commands for Agentic Debugging

After testing, we'll create these slash commands:

- `/debug-attach` - Start OpenOCD + GDB session
- `/debug-status` - Show current GDB state
- `/debug-inspect <var>` - Deep variable inspection with AI analysis
- `/debug-peripheral <name>` - Check I2C, GPIO, UART status
- `/debug-breakpoint <function>` - Set smart breakpoints
- `/debug-watch <var>` - Set watchpoint and explain changes
- `/debug-crash` - Analyze crash/fault registers
- `/debug-explain` - Explain current execution state
- `/debug-fix` - AI suggests fixes based on GDB state
- `/debug-memory <addr>` - Analyze memory region
- `/debug-trace <function>` - Trace execution flow
- `/debug-script <task>` - Generate custom GDB Python script

*Commands will be implemented after testing in Step 4*

---

## Quick Start

1. **Wire up hardware** (IMU, LED, button, UART)
2. **Build firmware:** `cargo build`
3. **Terminal 1:** `openocd -f board/esp32c6.cfg`
4. **Terminal 2:** `riscv32-esp-elf-gdb target/riscv32imc-unknown-none-elf/debug/main`
5. **Terminal 3:** `screen /dev/ttyUSB0 115200` (UART)
6. **Start debugging!**

## Resources

- [ESP32-C6 Technical Reference Manual](https://www.espressif.com/sites/default/files/documentation/esp32-c6_technical_reference_manual_en.pdf) - Peripheral registers
- [OpenOCD Documentation](http://openocd.org/doc/html/index.html)
- [GDB Documentation](https://sourceware.org/gdb/documentation/)
- [GDB Python API](https://sourceware.org/gdb/current/onlinedocs/gdb.html/Python-API.html)
- [ChatDBG Paper](https://arxiv.org/html/2403.16354v1) - AI-powered debugging
- [GDB MCP Server](https://github.com/signal-slot/mcp-gdb) - Claude + GDB integration

---

Ready to build a self-debugging system.


---

## What We Built

This lesson includes:

### 1. Complete Firmware (from Lesson 06)
- ✅ I2C IMU (MPU9250)
- ✅ NeoPixel LED (GPIO8)
- ✅ Button input (GPIO9)
- ✅ UART terminal (GPIO15/23)
- All peripherals tested and working

### 2. GDB Configuration
- `openocd.cfg` - OpenOCD setup for ESP32-C6
- `.gdbinit` - Auto-initialization with custom commands
- `esp32c6-builtin.cfg` - USB-JTAG configuration

### 3. GDB Helper Script
`gdb_helpers.py` - Simple Python helpers:
```gdb
(gdb) source gdb_helpers.py
(gdb) show-i2c        # I2C status with decoded flags
(gdb) show-gpio       # GPIO state for key pins
(gdb) show-all        # All peripherals at once
(gdb) inspect led_on  # Smart variable inspection
```

### 4. Debugging Slash Command
`.claude/commands/esp32-debug.md` - AI-assisted debugging workflow

Use it with: `/esp32-debug` in Claude Code

## Quick Test

```bash
# Build and flash
cd lessons/07-gdb-debugging
cargo build
espflash flash --port /dev/cu.usbmodem2101 target/riscv32imac-unknown-none-elf/debug/main

# Monitor boot (USB CDC)
python3 << 'EOF'
import serial, time
ser = serial.Serial('/dev/cu.usbmodem2101', 115200, timeout=5)
ser.setDTR(False); time.sleep(0.1); ser.setDTR(True); time.sleep(2)
print(ser.read(ser.in_waiting).decode('utf-8', errors='replace'))
EOF

# Expected output:
# INFO - ✓ I2C initialized (GPIO2=SDA, GPIO11=SCL)
# INFO - ✓ MPU9250 awake
# INFO - ✓ Button configured (GPIO9, active LOW)
# INFO - ✓ NeoPixel initialized (GPIO8)
# INFO - ✓ UART initialized (GPIO15=TX, GPIO23=RX, 115200 baud)
# INFO - ✓ All peripherals initialized
```

## What's Next?

This lesson focused on **GDB fundamentals** and the **feedback loop philosophy**.

For advanced topics (save for future lessons):
- Complex GDB Python automation
- Performance profiling with sampling
- Reverse debugging (time-travel)
- RTOS-aware debugging
- Memory analysis tools

---

## Claude Code Slash Commands

This lesson includes custom slash commands for automated workflows. These commands encapsulate manual debugging steps into consistent, repeatable processes.

### Available Commands

#### `/test-gdb-lesson [quick|full]`

**Purpose:** Comprehensive test suite for this lesson's debugging capabilities.

**What it does:**
1. Auto-detects USB ports (handles port changes on replug)
2. Auto-detects ESP JTAG probe number (multi-probe support)
3. Cleans up existing debug sessions
4. Builds firmware with debug symbols
5. Flashes using espflash (avoids probe-rs exclusive locks)
6. Tests 11 debugging capabilities:
   - Firmware boot verification
   - probe-rs attach and register reads
   - Breakpoints (main, functions, call stack)
   - Memory inspection
   - Debug symbols verification
   - Source code structure
   - GDB config files (for future use)
7. Generates comprehensive markdown test report

**Usage:**
```bash
/test-gdb-lesson quick    # 5-10 minute test (11 core tests)
/test-gdb-lesson full     # 15-20 minute test (15 tests with scenarios)
```

**Success criteria:**
- Quick mode: 9/11 tests pass (81%+)
- Full mode: 12/15 tests pass (80%+)

**Why it exists:**
- Originally created because the test agent struggled with:
  - Probe selection (tried `--probe 1`, then `--probe 303a:1001` before finding auto-detection)
  - Port detection after replug (hardcoded paths failed)
  - Exclusive lock conflicts (probe-rs run vs attach)
  - Process cleanup (existing sessions blocking new ones)
- These learnings are now encoded into the command

#### `/esp32-debug`

**Purpose:** AI-assisted debugging workflow for ESP32-C6 firmware.

**What it does:**
1. Captures system state (boot messages, crash logs)
2. Uses probe-rs or GDB to inspect program state
3. Reads peripheral registers to understand hardware state
4. Provides root cause analysis and fixes
5. Iteratively tests fixes using feedback loop

**Usage:**
```bash
/esp32-debug
# Then describe the issue you're seeing
```

**Common use cases:**
- Firmware crashes on boot
- Peripheral not working despite initialization
- No serial output
- Unexpected behavior

**How it works:**
- Captures USB CDC output with DTR reset
- Attaches probe-rs for live debugging
- Inspects peripheral registers (I2C, GPIO, UART, RMT)
- Suggests fixes based on register state
- Helps rebuild → flash → test → iterate

---

## How We Use These Commands

### Development Workflow

1. **During lesson creation:**
   - Manual steps are performed and recorded
   - Repeated patterns identified
   - Common errors noted
   - Successful approaches documented

2. **Command creation:**
   - Encapsulate proven workflows into slash commands
   - Add error handling for common issues
   - Include auto-detection for changing hardware state
   - Document assumptions and prerequisites

3. **Continuous improvement:**
   - Analyze conversation logs from command usage
   - Look for struggle patterns (LLM trying multiple approaches)
   - Look for knowledge discoveries (new techniques found)
   - Look for anti-patterns (repeated errors)
   - Update commands based on learnings

### Improvement Cycle

```
Manual Task → Document Steps → Create Slash Command → Use Command →
Analyze Logs → Find Improvements → Update Command → Repeat
```

**Example:** The `/test-gdb-lesson` command evolved through:
1. Initial manual testing revealed probe selection issues
2. Created command with `--probe 1` hardcoded
3. Testing showed this failed with multiple probes
4. Added auto-detection from `probe-rs list | grep "esp.*jtag"`
5. Testing showed port changes after replug
6. Added dynamic port detection with `ls /dev/cu.usbmodem*`
7. Now robust to hardware changes

### Adding New Commands

When you find yourself doing the same manual steps repeatedly:

1. **Document the successful sequence** in your conversation
2. **Identify parameters** that change (ports, probe numbers, file paths)
3. **Extract the core workflow** (what stays the same)
4. **Create `.claude/commands/<name>.md`** with:
   - Clear description
   - Argument hints
   - Prerequisites check
   - Step-by-step execution
   - Expected outputs
   - Error handling
   - Success criteria

5. **Test the command** on real hardware
6. **Analyze the results** and improve
7. **Document learnings** in IDEAS.md

---

You now have a complete debugging workflow for embedded Rust development.

*Note: For ideas on future slash commands and workflow improvements, see `IDEAS.md` in the project root.*
